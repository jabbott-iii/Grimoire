//! `cargo xtask release channels` and `cargo xtask release verify`: every package
//! manager's files, generated from the CLI archives the build jobs made, so every
//! channel ships the same bytes (ADR-0009).
//!
//! Generation is a pure function of the archives and the repository files, so
//! `verify` regenerates everything in memory and compares it with what is in `dist/`.

mod homebrew;
mod installers;
mod npm;
mod scoop;
mod wheel;
mod winget;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use super::archive;
use super::checksums::{self, BUNDLE_SUFFIX, SUMS};
use super::meta::{self, Release};
use super::plan;
use super::sbom;
use super::targets::{self, BIN, PACKAGE, Target};

/// A shorter description for package managers that cap it (Homebrew: 80 characters).
pub(crate) const SHORT_DESCRIPTION: &str =
    "Install only the AI-agent assets you choose into your repository";
/// The SHA-256 written for a target that `--partial` (a pull request dry-run) didn't build.
pub(crate) const MISSING_SHA256: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

/// One built CLI archive.
#[derive(Debug, Clone)]
pub(crate) struct Built {
    pub(crate) target: &'static Target,
    pub(crate) archive: String,
    pub(crate) sha256: String,
    pub(crate) binary: Vec<u8>,
}

/// What generation reads.
#[derive(Debug, Clone)]
pub(crate) struct Input {
    pub(crate) version: String,
    pub(crate) description: String,
    /// Built targets, in [`targets::TARGETS`] order.
    pub(crate) built: Vec<Built>,
    pub(crate) readme: Vec<u8>,
    pub(crate) license_mit: Vec<u8>,
    pub(crate) license_apache: Vec<u8>,
    pub(crate) launcher_js: String,
    pub(crate) install_sh: String,
    pub(crate) install_ps1: String,
}

impl Input {
    /// The archive SHA-256 for `target`, or [`MISSING_SHA256`] for one not built.
    pub(crate) fn sha256(&self, triple: &str) -> &str {
        self.built
            .iter()
            .find(|b| b.target.triple == triple)
            .map_or(MISSING_SHA256, |b| b.sha256.as_str())
    }
}

/// A generated file and its Unix mode.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Out {
    pub(crate) bytes: Vec<u8>,
    pub(crate) mode: u32,
}

impl Out {
    fn text(s: String) -> Self {
        Out {
            bytes: s.into_bytes(),
            mode: 0o644,
        }
    }
}

/// Everything generated: release assets for `dist/`, and work files (npm package
/// directories to `npm pack`, JSON twins of the winget manifests to validate).
#[derive(Debug, Default)]
pub(crate) struct Generated {
    pub(crate) dist: BTreeMap<String, Out>,
    pub(crate) work: BTreeMap<String, Out>,
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn read_text(path: &Path) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|_| format!("{} is not UTF-8", path.display()))
}

/// Reads the archives in `dist` and the repository files generation needs.
pub(crate) fn load(
    root: &Path,
    release: &Release,
    dist: &Path,
    partial: bool,
) -> Result<Input, String> {
    let mut built = Vec::new();
    for target in targets::TARGETS {
        let name = target.archive(PACKAGE, &release.version);
        let path = dist.join(&name);
        if !path.is_file() {
            if partial {
                continue;
            }
            return Err(format!(
                "{name} is missing (pass --partial for a dry-run subset)"
            ));
        }
        let bytes = read(&path)?;
        let files =
            archive::read_canonical(target.format, &bytes).map_err(|e| format!("{name}: {e}"))?;
        let binary = archive::member(&files, &target.binary_file(BIN))
            .map_err(|e| format!("{name}: {e}"))?
            .data
            .clone();
        built.push(Built {
            target,
            archive: name,
            sha256: checksums::sha256_hex(&bytes),
            binary,
        });
    }
    if built.is_empty() {
        return Err(format!("no CLI archives in {}", dist.display()));
    }
    Ok(Input {
        version: release.version.clone(),
        description: release.description.clone(),
        built,
        readme: read(&root.join("README.md"))?,
        license_mit: read(&root.join("LICENSE-MIT"))?,
        license_apache: read(&root.join("LICENSE-APACHE"))?,
        launcher_js: read_text(&root.join("packaging/npm/grimoire.js"))?,
        install_sh: read_text(&root.join("packaging/install/install.sh"))?,
        install_ps1: read_text(&root.join("packaging/install/install.ps1"))?,
    })
}

/// Generates every channel's files.
pub(crate) fn generate(input: &Input) -> Result<Generated, String> {
    let mut g = Generated::default();
    npm::generate(input, &mut g)?;
    wheel::generate(input, &mut g)?;
    g.dist.insert(
        "agent-grimoire.rb".to_owned(),
        Out::text(homebrew::formula(input)),
    );
    g.dist.insert(
        "agent-grimoire.json".to_owned(),
        Out::text(scoop::manifest(input)?),
    );
    winget::generate(input, &mut g)?;
    installers::generate(input, &mut g)?;
    Ok(g)
}

fn write_tree(base: &Path, files: &BTreeMap<String, Out>) -> Result<(), String> {
    for (rel, out) in files {
        let path = base.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
        }
        fs::write(&path, &out.bytes)
            .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            fs::set_permissions(&path, fs::Permissions::from_mode(out.mode))
                .map_err(|e| format!("cannot set the mode of {}: {e}", path.display()))?;
        }
    }
    Ok(())
}

#[allow(clippy::print_stderr, reason = "xtask reports progress on stderr")]
pub(crate) fn run(root: &Path, dist: &Path, work: &Path, partial: bool) -> Result<(), String> {
    let release = meta::load(root)?;
    let input = load(root, &release, dist, partial)?;
    let generated = generate(&input)?;
    write_tree(dist, &generated.dist)?;
    write_tree(work, &generated.work)?;
    eprintln!(
        "release channels: wrote {} release assets to {} and {} work files to {}",
        generated.dist.len(),
        dist.display(),
        generated.work.len(),
        work.display()
    );
    Ok(())
}

/// The archives of every target whose CLI archive is in `dist/`: all six for a
/// release, whatever subset was built with `--partial`.
fn built_archives(release: &Release, input: &Input) -> Vec<String> {
    plan::archives_for(release, input.built.iter().map(|b| b.target))
}

/// Every file `dist/` must hold, apart from cosign bundles.
fn expected_assets(release: &Release, input: &Input, generated: &Generated) -> Vec<String> {
    let mut names: Vec<String> = built_archives(release, input);
    names.extend(names.clone().iter().map(|a| sbom::sbom_name(a)));
    names.extend(generated.dist.keys().cloned());
    names.extend(npm::tarballs(input));
    names.push(SUMS.to_owned());
    names.sort();
    names
}

/// Checks a finished `dist/`: exactly the expected assets, `SHA256SUMS` current,
/// every generated file as `channels` makes it, every SBOM well formed, and the
/// same binary inside every archive, wheel and npm tarball of a target.
#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
pub(crate) fn verify(root: &Path, dist: &Path, partial: bool) -> Result<(), String> {
    let release = meta::load(root)?;
    let input = load(root, &release, dist, partial)?;
    if !partial && input.built.len() != targets::TARGETS.len() {
        return Err("a release needs every target".to_owned());
    }
    let generated = generate(&input)?;
    let mut problems = Vec::new();

    let present: Vec<String> = checksums::asset_files(dist)?;
    let assets: Vec<&String> = present
        .iter()
        .filter(|n| !n.ends_with(BUNDLE_SUFFIX))
        .collect();
    let expected = expected_assets(&release, &input, &generated);
    for name in &expected {
        if !assets.contains(&name) {
            problems.push(format!("missing: {name}"));
        }
    }
    for name in &assets {
        if !expected.contains(name) {
            problems.push(format!("unexpected: {name}"));
        }
    }
    for bundle in present.iter().filter(|n| n.ends_with(BUNDLE_SUFFIX)) {
        let subject = &bundle[..bundle.len() - BUNDLE_SUFFIX.len()];
        if !assets.iter().any(|a| *a == subject) {
            problems.push(format!("bundle without its file: {bundle}"));
        }
    }
    if checksums::render(dist)? != fs::read_to_string(dist.join(SUMS)).unwrap_or_default() {
        problems.push(format!("{SUMS} does not match the files"));
    }
    for (name, out) in &generated.dist {
        match fs::read(dist.join(name)) {
            Ok(bytes) if bytes == out.bytes => {}
            Ok(_) => problems.push(format!("{name} differs from what `channels` generates")),
            Err(_) => {} // reported as missing above
        }
    }
    for archive in built_archives(&release, &input) {
        if let Ok(bytes) = fs::read(dist.join(sbom::sbom_name(&archive)))
            && let Err(e) = sbom::check(&bytes, &archive)
        {
            problems.push(e);
        }
    }
    for b in &input.built {
        if let Err(e) = npm::check_tarball(dist, &input.version, b) {
            problems.push(e);
        }
    }
    if problems.is_empty() {
        eprintln!("release verify: {} assets checked", assets.len());
        Ok(())
    } else {
        Err(format!(
            "dist/ failed verification:\n  {}",
            problems.join("\n  ")
        ))
    }
}

/// The repository root, for tests.
#[cfg(test)]
pub(crate) fn repo_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_default()
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// A fixed input: fake binaries for every target except Windows arm64.
    pub(crate) fn input(partial: bool) -> Input {
        let built = targets::TARGETS
            .iter()
            .filter(|t| !partial || t.dry_run)
            .map(|t| Built {
                target: t,
                archive: t.archive(PACKAGE, "1.2.3"),
                sha256: checksums::sha256_hex(t.triple.as_bytes()),
                binary: format!("binary for {}", t.triple).into_bytes(),
            })
            .collect();
        let root = repo_root();
        Input {
            version: "1.2.3".to_owned(),
            description: "Install only the AI-agent assets you choose (rules, skills, agents, tools, MCP servers) into your repository".to_owned(),
            built,
            readme: b"# Grimoire\n".to_vec(),
            license_mit: b"MIT licence text\n".to_vec(),
            license_apache: b"Apache licence text\n".to_vec(),
            launcher_js: read_text(&root.join("packaging/npm/grimoire.js")).unwrap_or_else(|_| {
                targets::TARGETS.iter().map(|t| t.npm_package() + "\n").collect()
            }),
            install_sh: "version='@@VERSION@@'\nlinux='@@SHA256:x86_64-unknown-linux-musl@@'\n".to_owned(),
            install_ps1: "$Version = '@@VERSION@@'\n$Win = '@@SHA256:aarch64-pc-windows-msvc@@'\n".to_owned(),
        }
    }

    #[test]
    fn generation_covers_every_channel_and_is_deterministic() {
        let a = generate(&input(false)).unwrap();
        let b = generate(&input(false)).unwrap();
        assert_eq!(a.dist, b.dist);
        let names: Vec<&str> = a.dist.keys().map(String::as_str).collect();
        insta::assert_debug_snapshot!(names);
        assert!(a.work.contains_key("npm/agent-grimoire/package.json"));
        assert!(
            a.work
                .contains_key("npm/agent-grimoire-win32-arm64/bin/grimoire.exe")
        );
        assert!(
            a.work
                .contains_key("winget-json/jabbott-iii.agent-grimoire.installer.json")
        );
    }

    /// npm pack's layout: every file of a package directory under `package/`.
    fn npm_pack(dir: &Path, out: &Path) {
        let gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut tar = tar::Builder::new(gz);
        let mut stack = vec![dir.to_path_buf()];
        while let Some(d) = stack.pop() {
            for entry in fs::read_dir(&d).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                let rel = path
                    .strip_prefix(dir)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/");
                let data = fs::read(&path).unwrap();
                let mut header = tar::Header::new_ustar();
                header.set_path(format!("package/{rel}")).unwrap();
                header.set_size(data.len() as u64);
                header.set_mode(if rel.starts_with("bin/") {
                    0o755
                } else {
                    0o644
                });
                header.set_cksum();
                tar.append(&header, data.as_slice()).unwrap();
            }
        }
        fs::write(out, tar.into_inner().unwrap().finish().unwrap()).unwrap();
    }

    /// A complete dry-run `dist/` from fake binaries passes `verify`; tampering with
    /// any generated file, an SBOM or an npm binary fails it.
    #[test]
    fn verify_accepts_a_complete_dry_run_and_rejects_tampering() {
        let root = repo_root();
        let release = meta::load(&root).unwrap();
        let tmp = tempfile::tempdir().unwrap();
        let (dist, work) = (tmp.path().join("dist"), tmp.path().join("work"));
        fs::create_dir_all(&dist).unwrap();
        for t in targets::for_mode(true) {
            let mut bins = vec![(PACKAGE.to_owned(), release.version.clone(), BIN.to_owned())];
            bins.extend(
                release
                    .assets
                    .iter()
                    .map(|a| (a.bin.clone(), a.version.clone(), a.bin.clone())),
            );
            for (name, version, bin) in bins {
                let files = vec![
                    archive::File {
                        name: t.binary_file(&bin),
                        mode: 0o755,
                        data: format!("{bin} {}", t.triple).into_bytes(),
                    },
                    archive::File {
                        name: "README.md".to_owned(),
                        mode: 0o644,
                        data: b"readme".to_vec(),
                    },
                ];
                let file = t.archive(&name, &version);
                fs::write(dist.join(&file), archive::write(t.format, &files).unwrap()).unwrap();
                fs::write(
                    dist.join(sbom::sbom_name(&file)),
                    br#"{"bomFormat":"CycloneDX","specVersion":"1.6","serialNumber":"urn:uuid:0"}"#,
                )
                .unwrap();
            }
        }
        run(&root, &dist, &work, true).unwrap();
        for entry in fs::read_dir(work.join("npm")).unwrap() {
            let dir = entry.unwrap().path();
            let name = format!(
                "{}-{}.tgz",
                dir.file_name().unwrap().to_str().unwrap(),
                release.version
            );
            npm_pack(&dir, &dist.join(name));
        }
        assert!(checksums::run(&dist, false).unwrap());
        verify(&root, &dist, true).unwrap();
        assert!(
            verify(&root, &dist, false).is_err(),
            "a release needs all six targets"
        );

        let tamper = |file: &str, bytes: &[u8]| {
            let path = dist.join(file);
            let old = fs::read(&path).unwrap();
            fs::write(&path, bytes).unwrap();
            checksums::run(&dist, false).unwrap();
            let result = verify(&root, &dist, true);
            fs::write(&path, old).unwrap();
            checksums::run(&dist, false).unwrap();
            result
        };
        assert!(
            tamper("install.sh", b"#!/bin/sh\n")
                .unwrap_err()
                .contains("install.sh differs")
        );
        let sbom = sbom::sbom_name(&targets::TARGETS[0].archive(PACKAGE, &release.version));
        assert!(tamper(&sbom, b"{}").is_err());
        let npm = format!("{}-{}.tgz", targets::TARGETS[0].npm_dir(), release.version);
        let other = tmp.path().join("other");
        fs::create_dir_all(other.join("bin")).unwrap();
        fs::write(other.join("bin/grimoire"), b"another binary").unwrap();
        npm_pack(&other, &tmp.path().join("other.tgz"));
        let bytes = fs::read(tmp.path().join("other.tgz")).unwrap();
        assert!(tamper(&npm, &bytes).unwrap_err().contains("differs"));
        verify(&root, &dist, true).unwrap();
    }

    #[test]
    fn partial_generation_marks_missing_targets() {
        let g = generate(&input(true)).unwrap();
        let rb = String::from_utf8(g.dist["agent-grimoire.rb"].bytes.clone()).unwrap();
        assert!(rb.contains(MISSING_SHA256), "{rb}");
        let ps1 = String::from_utf8(g.dist["install.ps1"].bytes.clone()).unwrap();
        assert!(ps1.contains("$Win = ''"), "{ps1}");
        assert!(
            !g.work
                .contains_key("npm/agent-grimoire-win32-arm64/package.json")
        );
        let full = generate(&input(false)).unwrap();
        let rb = String::from_utf8(full.dist["agent-grimoire.rb"].bytes.clone()).unwrap();
        assert!(!rb.contains(MISSING_SHA256));
    }
}
