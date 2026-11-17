//! `cargo xtask release sbom --dist DIR`: a CycloneDX 1.6 JSON SBOM for every
//! archive in DIR, named `{archive}.cdx.json` (SBOM in docs/specs.md, ADR-0009).
//!
//! cargo-cyclonedx 0.5.10 writes its files next to each workspace member's
//! `Cargo.toml` and has no `--package`, so this runs it once per target, copies out
//! the binaries' SBOMs and deletes every file the run created. `SOURCE_DATE_EPOCH` is
//! left unset: it drops `serialNumber`, which `actions/attest` requires.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::meta::{self, Release};
use super::targets::{self, BIN, PACKAGE};

/// The SBOM file for an archive.
pub(crate) fn sbom_name(archive: &str) -> String {
    format!("{archive}.cdx.json")
}

fn cdx_files(dirs: &[PathBuf]) -> Result<BTreeSet<PathBuf>, String> {
    let mut found = BTreeSet::new();
    for dir in dirs {
        for entry in fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".cdx.json"))
            {
                found.insert(path);
            }
        }
    }
    Ok(found)
}

/// Where cargo-cyclonedx puts the SBOM of `bin` built for `triple`
/// (`--describe binaries --target-in-filename`).
fn generated(dir: &Path, bin: &str, triple: &str) -> PathBuf {
    dir.join(format!("{bin}_bin_{triple}.cdx.json"))
}

#[allow(clippy::print_stderr, reason = "xtask reports progress on stderr")]
pub(crate) fn run(root: &Path, dist: &Path) -> Result<(), String> {
    let release = meta::load(root)?;
    let before = cdx_files(&release.member_dirs)?;
    let result = generate(root, dist, &release, &before);
    // Remove everything the runs created, whether they succeeded or not.
    for path in cdx_files(&release.member_dirs)?.difference(&before) {
        let _ = fs::remove_file(path);
    }
    result
}

#[allow(clippy::print_stderr, reason = "xtask reports progress on stderr")]
fn generate(
    root: &Path,
    dist: &Path,
    release: &Release,
    before: &BTreeSet<PathBuf>,
) -> Result<(), String> {
    if !before.is_empty() {
        return Err(format!(
            "remove the existing SBOM files first: {}",
            before
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    let cli_dir = root.join("crates").join(PACKAGE);
    let mut made = 0;
    for target in targets::TARGETS {
        let cli_archive = target.archive(PACKAGE, &release.version);
        if !dist.join(&cli_archive).is_file() {
            continue;
        }
        let status = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .current_dir(root)
            .args(["cyclonedx", "--spec-version", "1.6", "--format", "json"])
            .args([
                "--describe",
                "binaries",
                "--target",
                target.triple,
                "--target-in-filename",
            ])
            .status()
            .map_err(|e| {
                format!("cannot run `cargo cyclonedx` (install cargo-cyclonedx 0.5.10): {e}")
            })?;
        if !status.success() {
            return Err(format!("cargo cyclonedx failed for {}", target.triple));
        }
        let mut pairs = vec![(generated(&cli_dir, BIN, target.triple), cli_archive)];
        for a in &release.assets {
            pairs.push((
                generated(&a.dir, &a.bin, target.triple),
                target.archive(&a.bin, &a.version),
            ));
        }
        for (from, archive) in pairs {
            if !dist.join(&archive).is_file() {
                return Err(format!("{archive} is missing from {}", dist.display()));
            }
            let bytes = fs::read(&from).map_err(|e| {
                let found: Vec<String> = cdx_files(std::slice::from_ref(
                    &from.parent().unwrap_or(root).to_path_buf(),
                ))
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.display().to_string())
                .collect();
                format!(
                    "cannot read {} ({e}); found: {}",
                    from.display(),
                    found.join(", ")
                )
            })?;
            check(&bytes, &archive)?;
            fs::write(dist.join(sbom_name(&archive)), &bytes)
                .map_err(|e| format!("cannot write the SBOM for {archive}: {e}"))?;
            made += 1;
        }
    }
    if made == 0 {
        return Err(format!("no CLI archives in {}", dist.display()));
    }
    eprintln!("release sbom: wrote {made} SBOMs");
    Ok(())
}

/// The fields `actions/attest` needs (ATTEST): `bomFormat`, `specVersion` 1.6 and a
/// `serialNumber`.
pub(crate) fn check(bytes: &[u8], archive: &str) -> Result<(), String> {
    let json: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|e| format!("{archive}: SBOM is not JSON: {e}"))?;
    let field = |k: &str| json.get(k).and_then(serde_json::Value::as_str);
    if field("bomFormat") != Some("CycloneDX") || field("specVersion") != Some("1.6") {
        return Err(format!("{archive}: the SBOM is not CycloneDX 1.6"));
    }
    if !field("serialNumber").is_some_and(|s| s.starts_with("urn:uuid:")) {
        return Err(format!("{archive}: the SBOM has no serialNumber"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sboms_must_be_cyclonedx_1_6_with_a_serial_number() {
        let good = br#"{"bomFormat":"CycloneDX","specVersion":"1.6","serialNumber":"urn:uuid:1"}"#;
        assert!(check(good, "a").is_ok());
        assert!(
            check(
                br#"{"bomFormat":"CycloneDX","specVersion":"1.3","serialNumber":"urn:uuid:1"}"#,
                "a"
            )
            .is_err()
        );
        assert!(check(br#"{"bomFormat":"CycloneDX","specVersion":"1.6"}"#, "a").is_err());
        assert_eq!(sbom_name("x.zip"), "x.zip.cdx.json");
    }
}
