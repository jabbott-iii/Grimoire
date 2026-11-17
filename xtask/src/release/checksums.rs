//! `cargo xtask release checksums --dist DIR [--check]`: `SHA256SUMS` in GNU
//! `sha256sum` format (`<hex>  <name>`), sorted by name, over every release asset
//! except cosign bundles (ADR-0009). `actions/attest` and `sha256sum -c` read it.

use std::fmt::Write as _;
use std::fs;
use std::path::Path;

use sha2::{Digest as _, Sha256};

pub(crate) const SUMS: &str = "SHA256SUMS";
pub(crate) const BUNDLE_SUFFIX: &str = ".sigstore.json";

/// The lowercase hex SHA-256 of `bytes`.
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(64);
    for b in Sha256::digest(bytes) {
        let _ = write!(hex, "{b:02x}");
    }
    hex
}

/// The regular files directly in `dir`, sorted. Symlinks, subdirectories and names
/// that aren't plain ASCII are refused: release assets are flat and portable.
pub(crate) fn asset_files(dir: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))? {
        let entry = entry.map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
        let name = entry
            .file_name()
            .into_string()
            .map_err(|n| format!("non-UTF-8 file name {}", n.to_string_lossy()))?;
        let kind = entry
            .file_type()
            .map_err(|e| format!("cannot stat {name}: {e}"))?;
        if !kind.is_file() {
            return Err(format!("{name}: release assets must be regular files"));
        }
        if !name
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b))
        {
            // GitHub renames release assets with any other character.
            return Err(format!(
                "{name}: asset names must be ASCII letters, digits, '.', '_' or '-'"
            ));
        }
        names.push(name);
    }
    names.sort();
    Ok(names)
}

/// The `SHA256SUMS` text for `dir`.
pub(crate) fn render(dir: &Path) -> Result<String, String> {
    let mut out = String::new();
    for name in asset_files(dir)? {
        if name == SUMS || name.ends_with(BUNDLE_SUFFIX) {
            continue;
        }
        let bytes = fs::read(dir.join(&name)).map_err(|e| format!("cannot read {name}: {e}"))?;
        out.push_str(&sha256_hex(&bytes));
        out.push_str("  ");
        out.push_str(&name);
        out.push('\n');
    }
    Ok(out)
}

#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
pub(crate) fn run(dir: &Path, check: bool) -> Result<bool, String> {
    let text = render(dir)?;
    let path = dir.join(SUMS);
    if check {
        let ok = fs::read_to_string(&path).ok().as_deref() == Some(text.as_str());
        if ok {
            eprintln!(
                "release checksums: {SUMS} matches {} files",
                text.lines().count()
            );
        } else {
            eprintln!(
                "error: {SUMS} does not match the files in {}",
                dir.display()
            );
        }
        Ok(ok)
    } else {
        fs::write(&path, &text).map_err(|e| format!("cannot write {SUMS}: {e}"))?;
        eprintln!(
            "release checksums: wrote {SUMS} ({} files)",
            text.lines().count()
        );
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sums_skip_bundles_and_themselves_and_detect_changes() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("b.zip"), b"b").unwrap();
        fs::write(dir.path().join("a.tar.gz"), b"a").unwrap();
        fs::write(dir.path().join("a.tar.gz.sigstore.json"), b"{}").unwrap();
        assert!(run(dir.path(), false).unwrap());
        let text = fs::read_to_string(dir.path().join(SUMS)).unwrap();
        assert_eq!(
            text,
            format!(
                "{}  a.tar.gz\n{}  b.zip\n",
                sha256_hex(b"a"),
                sha256_hex(b"b")
            )
        );
        assert!(run(dir.path(), true).unwrap());
        fs::write(dir.path().join("b.zip"), b"changed").unwrap();
        assert!(!run(dir.path(), true).unwrap());
        fs::create_dir(dir.path().join("sub")).unwrap();
        assert!(render(dir.path()).is_err());
    }
}
