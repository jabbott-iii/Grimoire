//! `PyPI`: one binary-only wheel per target, written from the archived binary instead
//! of rebuilding it with maturin (the maintainer's choice, ADR-0009; PYPI-TP, MATURIN
//! and the pypi Phase 5 research in docs/specs.md).
//!
//! The layout is what maturin's `bindings = "bin"` produces and pip installs:
//! `{name}-{ver}.data/scripts/grimoire` (mode 0755), then `.dist-info/` with `METADATA`
//! (core metadata 2.4, `License-Expression`), `WHEEL`, `licenses/` and `RECORD` last.
//! Specs: <https://packaging.python.org/en/latest/specifications/binary-distribution-format/>,
//! `…/recording-installed-packages/` and `…/core-metadata/`.

use std::fmt::Write as _;

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use sha2::{Digest as _, Sha256};

use super::{Generated, Input, Out};
use crate::release::targets::{BIN, REPO_URL};
use crate::release::zip::{self, Member};

/// The distribution name in file names (PEP 427 escaping of `agent-grimoire`).
const DIST: &str = "agent_grimoire";

/// SemVer → PEP 440: `1.2.3` stays, `1.2.3-alpha.1` → `1.2.3a1`, `-beta.N` → `bN`,
/// `-rc.N` → `rcN`. Anything else can't be expressed and is refused.
pub(crate) fn pep440(version: &str) -> Result<String, String> {
    let (core, pre) = match version.split_once('-') {
        Some((core, pre)) => (core, Some(pre)),
        None => (version, None),
    };
    if version.contains('+')
        || core.split('.').count() != 3
        || !core
            .split('.')
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err(format!("{version} has no PEP 440 form"));
    }
    let Some(pre) = pre else {
        return Ok(core.to_owned());
    };
    let (label, n) = pre.split_once('.').ok_or_else(|| {
        format!("{version}: use -alpha.N, -beta.N or -rc.N for a PyPI prerelease")
    })?;
    let short = match label {
        "alpha" => "a",
        "beta" => "b",
        "rc" => "rc",
        _ => {
            return Err(format!(
                "{version}: use -alpha.N, -beta.N or -rc.N for a PyPI prerelease"
            ));
        }
    };
    if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("{version}: the prerelease number must be digits"));
    }
    Ok(format!("{core}{short}{n}"))
}

fn record_hash(data: &[u8]) -> String {
    format!("sha256={}", URL_SAFE_NO_PAD.encode(Sha256::digest(data)))
}

/// The wheel file name for a target. A compressed tag set is sorted (`PyPA`
/// platform-compatibility-tags spec, "Compressed tag sets").
pub(crate) fn file_name(version: &str, platforms: &[&str]) -> Result<String, String> {
    let mut sorted = platforms.to_vec();
    sorted.sort_unstable();
    Ok(format!(
        "{DIST}-{}-py3-none-{}.whl",
        pep440(version)?,
        sorted.join(".")
    ))
}

pub(super) fn generate(input: &Input, g: &mut Generated) -> Result<(), String> {
    let version = pep440(&input.version)?;
    let readme = String::from_utf8(input.readme.clone()).map_err(|_| "README.md is not UTF-8")?;
    let metadata = format!(
        "Metadata-Version: 2.4\n\
Name: agent-grimoire\n\
Version: {version}\n\
Summary: {summary}\n\
Keywords: ai,agents,mcp,skills,cli\n\
Project-URL: Repository, {REPO_URL}\n\
Project-URL: Changelog, {REPO_URL}/blob/v{semver}/CHANGELOG.md\n\
License-Expression: MIT OR Apache-2.0\n\
License-File: LICENSE-MIT\n\
License-File: LICENSE-APACHE\n\
Description-Content-Type: text/markdown\n\
\n\
{readme}",
        summary = input.description,
        semver = input.version,
    );
    let data = format!("{DIST}-{version}.data");
    let info = format!("{DIST}-{version}.dist-info");
    for b in &input.built {
        let t = b.target;
        let mut wheel = format!(
            "Wheel-Version: 1.0\nGenerator: grimoire-xtask {}\nRoot-Is-Purelib: false\n",
            input.version
        );
        for p in t.wheel_platforms {
            let _ = writeln!(wheel, "Tag: py3-none-{p}");
        }
        let mut members = vec![
            Member {
                name: format!("{data}/scripts/{}", t.binary_file(BIN)),
                mode: 0o755,
                data: b.binary.clone(),
            },
            Member {
                name: format!("{info}/METADATA"),
                mode: 0o644,
                data: metadata.clone().into_bytes(),
            },
            Member {
                name: format!("{info}/WHEEL"),
                mode: 0o644,
                data: wheel.into_bytes(),
            },
            Member {
                name: format!("{info}/licenses/LICENSE-MIT"),
                mode: 0o644,
                data: input.license_mit.clone(),
            },
            Member {
                name: format!("{info}/licenses/LICENSE-APACHE"),
                mode: 0o644,
                data: input.license_apache.clone(),
            },
        ];
        let mut record = String::new();
        for m in &members {
            let _ = writeln!(
                record,
                "{},{},{}",
                m.name,
                record_hash(&m.data),
                m.data.len()
            );
        }
        let _ = writeln!(record, "{info}/RECORD,,");
        members.push(Member {
            name: format!("{info}/RECORD"),
            mode: 0o644,
            data: record.into_bytes(),
        });
        g.dist.insert(
            file_name(&input.version, t.wheel_platforms)?,
            Out {
                bytes: zip::write(&members)?,
                mode: 0o644,
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::input;
    use super::*;

    #[test]
    fn versions_map_to_pep_440() {
        assert_eq!(pep440("1.2.3").unwrap(), "1.2.3");
        assert_eq!(pep440("1.2.3-alpha.1").unwrap(), "1.2.3a1");
        assert_eq!(pep440("1.2.3-beta.10").unwrap(), "1.2.3b10");
        assert_eq!(pep440("1.2.3-rc.2").unwrap(), "1.2.3rc2");
        for bad in ["1.2", "1.2.3-dev.1", "1.2.3-rc", "1.2.3+build", "1.2.x"] {
            assert!(pep440(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn wheels_have_the_binary_metadata_and_a_correct_record() {
        let mut g = Generated::default();
        generate(&input(false), &mut g).unwrap();
        let name = "agent_grimoire-1.2.3-py3-none-manylinux2014_x86_64.manylinux_2_17_x86_64.musllinux_1_2_x86_64.whl";
        let members = zip::read(&g.dist[name].bytes).unwrap();
        let names: Vec<&str> = members.iter().map(|m| m.name.as_str()).collect();
        insta::assert_debug_snapshot!(names);
        let script = &members[0];
        assert_eq!(script.mode, 0o755);
        assert_eq!(script.data, b"binary for x86_64-unknown-linux-musl");
        let record = String::from_utf8(members.last().unwrap().data.clone()).unwrap();
        for m in &members[..members.len() - 1] {
            assert!(
                record.contains(&format!(
                    "{},{},{}",
                    m.name,
                    record_hash(&m.data),
                    m.data.len()
                )),
                "{record}"
            );
        }
        let metadata = String::from_utf8(members[1].data.clone()).unwrap();
        insta::assert_snapshot!(metadata);
        let wheel = String::from_utf8(members[2].data.clone()).unwrap();
        assert!(
            wheel.contains("Tag: py3-none-musllinux_1_2_x86_64\n"),
            "{wheel}"
        );
        assert!(
            g.dist
                .contains_key("agent_grimoire-1.2.3-py3-none-win_arm64.whl")
        );
    }
}
