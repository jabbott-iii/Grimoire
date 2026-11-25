//! `cargo xtask release notes --version V --image-digest D --out FILE`: the GitHub
//! Release text, taken from the `CHANGELOG.md` section release-plz wrote for that
//! version, plus the container image's digest.

use std::fs;
use std::path::Path;

use super::image::IMAGE;
use super::targets::REPO_URL;

/// The section of `changelog` for `version`: from its `## ` heading (release-plz
/// writes `## [V](compare link) - date`) up to the next `## ` heading.
pub(crate) fn section(changelog: &str, version: &str) -> Option<String> {
    let bracketed = format!("## [{version}]");
    let plain = format!("## {version}");
    let mut lines = changelog.lines();
    lines.by_ref().find(|l| {
        l.starts_with(&bracketed)
            || l.strip_prefix(&plain)
                .is_some_and(|rest| rest.is_empty() || rest.starts_with([' ', '\t']))
    })?;
    let body: Vec<&str> = lines.take_while(|l| !l.starts_with("## ")).collect();
    let text = body.join("\n").trim().to_owned();
    (!text.is_empty()).then_some(text)
}

/// The full release notes. A release always has its `CHANGELOG.md` section
/// (`release plan` waits for it), so a missing one is an error, not a fallback.
/// `image_digest` is the container image's index digest, which the image job
/// computed before the release; the registry job pushes exactly that index.
pub(crate) fn render(
    changelog: Option<&str>,
    version: &str,
    image_digest: &str,
) -> Result<String, String> {
    let changes = changelog.ok_or("CHANGELOG.md is missing")?;
    let changes = section(changes, version)
        .ok_or_else(|| format!("CHANGELOG.md has no section for {version}"))?;
    let hex = image_digest.strip_prefix("sha256:").unwrap_or_default();
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(format!("`{image_digest}` is not a sha256 digest"));
    }
    Ok(format!(
        "{changes}\n\n---\n\nInstall and verify: [docs/05-release.md]({REPO_URL}/blob/v{version}/docs/05-release.md). \
Every asset has a SHA-256 in `SHA256SUMS`, a build provenance attestation and a cosign bundle (`<asset>.sigstore.json`).\n\n\
Container image: `{IMAGE}@{image_digest}` (tags `{version}` and `latest`), for linux/amd64 and linux/arm64. \
It is pushed after this release, once the registry jobs are approved. Pin it by digest and verify it as \
[docs/container.md]({REPO_URL}/blob/v{version}/docs/container.md) shows.\n"
    ))
}

pub(crate) fn run(
    root: &Path,
    version: &str,
    image_digest: &str,
    out: &Path,
) -> Result<(), String> {
    let changelog = fs::read_to_string(root.join("CHANGELOG.md")).ok();
    fs::write(out, render(changelog.as_deref(), version, image_digest)?)
        .map_err(|e| format!("cannot write {}: {e}", out.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const LOG: &str = "# Changelog\n\n## [Unreleased]\n\n## [0.2.0](https://x/compare/v0.1.0...v0.2.0) - 2026-12-01\n\n### Added\n\n- a thing\n\n## [0.1.0] - 2026-11-20\n\n- first\n";

    #[test]
    fn takes_the_versions_section_only() {
        assert_eq!(
            section(LOG, "0.2.0").as_deref(),
            Some("### Added\n\n- a thing")
        );
        assert_eq!(section(LOG, "0.1.0").as_deref(), Some("- first"));
        assert_eq!(section(LOG, "0.1"), None);
        let digest = format!("sha256:{}", "ab".repeat(32));
        assert!(render(Some(LOG), "0.3.0", &digest).is_err());
        assert!(render(None, "0.2.0", &digest).is_err());
        assert!(render(Some(LOG), "0.2.0", "sha256:ABC").is_err());
        let notes = render(Some(LOG), "0.2.0", &digest).unwrap();
        assert!(notes.starts_with("### Added\n\n- a thing\n"), "{notes}");
        assert!(
            notes.contains(
                "(https://github.com/jabbott-iii/Grimoire/blob/v0.2.0/docs/05-release.md)"
            ),
            "{notes}"
        );
        assert!(
            notes.contains(&format!(
                "`ghcr.io/jabbott-iii/grimoire@{digest}` (tags `0.2.0` and `latest`)"
            )),
            "{notes}"
        );
    }
}
