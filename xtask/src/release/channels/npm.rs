//! npm: the `agent-grimoire` launcher and one `@agent-grimoire/{os}-{cpu}` package per
//! target (NPM-TP and the npm Phase 5 research in docs/specs.md, ADR-0004).
//!
//! - Platform packages hold only the binary: `os` and `cpu` select them, there is no
//!   `libc` field (the Linux binaries are static musl, so they run on glibc too), no
//!   `bin`, and no lifecycle scripts (npm 12 blocks dependency scripts anyway).
//! - The launcher's `bin` is `packaging/npm/grimoire.js`, which runs the matching
//!   platform package's binary. Its optional dependencies name every platform.
//! - `repository.url` must match the GitHub repository exactly for provenance.

use std::fs;
use std::path::Path;

use serde_json::{Value, json};

use super::{Built, Generated, Input, Out};
use crate::release::archive;
use crate::release::targets::{self, BIN, PACKAGE, REPO_URL, Target};

const REPOSITORY: &str = "git+https://github.com/jabbott-iii/Grimoire.git";
const FILES: [&str; 3] = ["README.md", "LICENSE-MIT", "LICENSE-APACHE"];

fn pretty(value: &Value) -> Result<String, String> {
    serde_json::to_string_pretty(value)
        .map(|s| s + "\n")
        .map_err(|e| e.to_string())
}

fn common(input: &Input, g: &mut Generated, dir: &str, readme: String) {
    g.work.insert(format!("{dir}/README.md"), Out::text(readme));
    g.work.insert(
        format!("{dir}/LICENSE-MIT"),
        Out {
            bytes: input.license_mit.clone(),
            mode: 0o644,
        },
    );
    g.work.insert(
        format!("{dir}/LICENSE-APACHE"),
        Out {
            bytes: input.license_apache.clone(),
            mode: 0o644,
        },
    );
}

/// The binary's path inside a platform package.
fn binary_path(target: &Target) -> String {
    format!("bin/{}", target.binary_file(BIN))
}

pub(super) fn generate(input: &Input, g: &mut Generated) -> Result<(), String> {
    for t in targets::TARGETS {
        if !input.launcher_js.contains(&t.npm_package()) {
            return Err(format!(
                "packaging/npm/grimoire.js does not mention {}",
                t.npm_package()
            ));
        }
    }
    let optional: serde_json::Map<String, Value> = targets::TARGETS
        .iter()
        .map(|t| (t.npm_package(), Value::from(input.version.clone())))
        .collect();
    let mut files: Vec<&str> = vec!["bin/grimoire.js"];
    files.extend(FILES);
    let launcher = json!({
        "name": PACKAGE,
        "version": input.version,
        "description": input.description,
        "license": "MIT OR Apache-2.0",
        "homepage": REPO_URL,
        "repository": {"type": "git", "url": REPOSITORY},
        "bin": {BIN: "bin/grimoire.js"},
        "files": files,
        "engines": {"node": ">=18"},
        "optionalDependencies": optional,
    });
    let dir = format!("npm/{PACKAGE}");
    g.work
        .insert(format!("{dir}/package.json"), Out::text(pretty(&launcher)?));
    g.work.insert(
        format!("{dir}/bin/grimoire.js"),
        Out {
            bytes: input.launcher_js.clone().into_bytes(),
            mode: 0o755,
        },
    );
    common(
        input,
        g,
        &dir,
        format!(
            "# agent-grimoire\n\n{}.\n\n```sh\nnpm install --global agent-grimoire\ngrimoire --help\n```\n\n\
This package runs the `grimoire` binary from the `@agent-grimoire/<os>-<cpu>` package npm installs for your platform \
(an optional dependency; don't install with `--omit=optional`). It has no install scripts.\n\n\
Source, other ways to install, and how to verify a release: {REPO_URL}\n",
            input.description
        ),
    );

    for b in &input.built {
        let t = b.target;
        let mut files = vec![binary_path(t)];
        files.extend(FILES.iter().map(|f| (*f).to_owned()));
        let package = json!({
            "name": t.npm_package(),
            "version": input.version,
            "description": format!("The grimoire binary for {}-{} ({}). Install agent-grimoire instead.", t.npm_os, t.npm_cpu, t.triple),
            "license": "MIT OR Apache-2.0",
            "homepage": REPO_URL,
            "repository": {"type": "git", "url": REPOSITORY},
            "os": [t.npm_os],
            "cpu": [t.npm_cpu],
            "files": files,
        });
        let dir = format!("npm/{}", t.npm_dir());
        g.work
            .insert(format!("{dir}/package.json"), Out::text(pretty(&package)?));
        g.work.insert(
            format!("{dir}/{}", binary_path(t)),
            Out {
                bytes: b.binary.clone(),
                mode: 0o755,
            },
        );
        common(
            input,
            g,
            &dir,
            format!(
                "# {}\n\nThe `grimoire` binary for {}, built from {REPO_URL} at v{}. \
Install [`agent-grimoire`](https://www.npmjs.com/package/agent-grimoire) instead; it picks this package for you.\n",
                t.npm_package(),
                t.triple,
                input.version
            ),
        );
    }
    Ok(())
}

/// The tarballs `npm pack` makes from the generated directories.
pub(super) fn tarballs(input: &Input) -> Vec<String> {
    let mut names = vec![format!("{PACKAGE}-{}.tgz", input.version)];
    names.extend(
        input
            .built
            .iter()
            .map(|b| format!("{}-{}.tgz", b.target.npm_dir(), input.version)),
    );
    names
}

/// The packed platform tarball holds the same binary as the archive, executable.
pub(super) fn check_tarball(dist: &Path, version: &str, b: &Built) -> Result<(), String> {
    let name = format!("{}-{version}.tgz", b.target.npm_dir());
    let Ok(bytes) = fs::read(dist.join(&name)) else {
        return Ok(()); // reported as missing
    };
    let files =
        archive::read(targets::Format::TarGz, &bytes).map_err(|e| format!("{name}: {e}"))?;
    let bin = archive::member(&files, &format!("package/{}", binary_path(b.target)))
        .map_err(|e| format!("{name}: {e}"))?;
    if bin.data != b.binary {
        return Err(format!("{name}: its binary differs from {}", b.archive));
    }
    if bin.mode & 0o111 == 0 {
        return Err(format!("{name}: its binary is not executable"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::input;
    use super::*;

    #[test]
    fn launcher_and_platform_packages() {
        let mut g = Generated::default();
        generate(&input(false), &mut g).unwrap();
        let launcher =
            String::from_utf8(g.work["npm/agent-grimoire/package.json"].bytes.clone()).unwrap();
        insta::assert_snapshot!(launcher);
        let linux = String::from_utf8(
            g.work["npm/agent-grimoire-linux-x64/package.json"]
                .bytes
                .clone(),
        )
        .unwrap();
        insta::assert_snapshot!(linux);
        assert_eq!(
            g.work["npm/agent-grimoire-linux-x64/bin/grimoire"].mode,
            0o755
        );
        assert_eq!(
            tarballs(&input(true)),
            [
                "agent-grimoire-1.2.3.tgz",
                "agent-grimoire-linux-x64-1.2.3.tgz",
                "agent-grimoire-darwin-arm64-1.2.3.tgz",
                "agent-grimoire-win32-x64-1.2.3.tgz"
            ]
        );
    }

    #[test]
    fn the_launcher_must_know_every_platform() {
        let mut i = input(false);
        i.launcher_js = "nothing".to_owned();
        assert!(generate(&i, &mut Generated::default()).is_err());
    }
}
