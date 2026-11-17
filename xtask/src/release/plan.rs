//! `cargo xtask release plan`: decides what a run of `.github/workflows/release.yml`
//! does and hands the workflow its matrices (ADR-0009).
//!
//! - `dry-run` on pull requests and on any ref other than `refs/heads/main`, and on
//!   `main` until the maintainer sets the repository variable `GRIMOIRE_RELEASES` to
//!   `enabled` (a workflow that names a missing environment creates it unprotected).
//! - `release` on a push or manual run on `main` when `v{version}` isn't tagged yet.
//! - `skip` on `main` when it is.

use std::fmt::Write as _;
use std::path::Path;

use serde_json::{Value, json};

use super::meta::Release;
use super::targets::{self, PACKAGE, Target};

/// What the run does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Mode {
    DryRun,
    Release,
    Skip,
}

impl Mode {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Mode::DryRun => "dry-run",
            Mode::Release => "release",
            Mode::Skip => "skip",
        }
    }
}

/// The value of `GRIMOIRE_RELEASES` that turns releases on.
pub(crate) const ENABLED: &str = "enabled";

/// Decides the mode. `tag_exists` is asked only when it matters.
pub(crate) fn mode(
    event: &str,
    git_ref: &str,
    releases: &str,
    tag_exists: impl FnOnce() -> Result<bool, String>,
) -> Result<Mode, String> {
    match event {
        "pull_request" => Ok(Mode::DryRun),
        "push" | "workflow_dispatch" => {
            if git_ref != "refs/heads/main" || releases != ENABLED {
                Ok(Mode::DryRun)
            } else if tag_exists()? {
                Ok(Mode::Skip)
            } else {
                Ok(Mode::Release)
            }
        }
        other => Err(format!(
            "unexpected event `{other}`: release.yml runs on pull_request, push and workflow_dispatch"
        )),
    }
}

/// Every archive a mode builds: the CLI and each asset binary, per target.
pub(crate) fn archives(release: &Release, mode: Mode) -> Vec<String> {
    if mode == Mode::Skip {
        return Vec::new();
    }
    archives_for(release, targets::for_mode(mode == Mode::DryRun))
}

/// Every archive built for `targets`: the CLI and each asset binary, per target.
pub(crate) fn archives_for<'a>(
    release: &Release,
    targets: impl IntoIterator<Item = &'a Target>,
) -> Vec<String> {
    let mut names = Vec::new();
    for t in targets {
        names.push(t.archive(PACKAGE, &release.version));
        for a in &release.assets {
            names.push(t.archive(&a.bin, &a.version));
        }
    }
    names
}

/// GitHub's hard limits a release would hit as the catalog grows: a matrix has at most
/// 256 jobs (one SBOM attestation per archive), and a release at most 1000 assets
/// (each archive and SBOM, the channel files and `SHA256SUMS`, every one with a cosign
/// bundle). Failing here is earlier and clearer than failing halfway through a release.
pub(crate) fn check_limits(release: &Release) -> Result<(), String> {
    let archives = archives(release, Mode::Release).len();
    // npm (7), wheels (6), winget (3), formula, Scoop, two install scripts, SHA256SUMS.
    let others = 7 + targets::TARGETS.len() + 3 + 1 + 1 + 2 + 1;
    let assets = (2 * archives + others) * 2;
    if archives > 256 {
        return Err(format!(
            "{archives} release archives exceed GitHub's 256-job matrix limit (attest-sbom); see ADR-0009"
        ));
    }
    if assets > 1000 {
        return Err(format!(
            "a release would have {assets} assets, over GitHub's limit of 1000; see ADR-0009"
        ));
    }
    Ok(())
}

/// The `key=value` lines for `$GITHUB_OUTPUT`.
pub(crate) fn outputs(release: &Release, mode: Mode) -> String {
    let matrix: Vec<Value> = if mode == Mode::Skip {
        Vec::new()
    } else {
        targets::for_mode(mode == Mode::DryRun)
            .map(|t| json!({"target": t.triple, "runner": t.runner}))
            .collect()
    };
    let mut out = String::new();
    let lines = [
        ("mode", mode.as_str().to_owned()),
        ("version", release.version.clone()),
        ("tag", release.tag()),
        ("targets", Value::from(matrix).to_string()),
        ("crates", json!(release.crates).to_string()),
        ("archives", json!(archives(release, mode)).to_string()),
    ];
    for (key, value) in lines {
        let _ = writeln!(out, "{key}={value}");
    }
    out
}

/// Whether `origin` has `refs/tags/{tag}`.
pub(crate) fn remote_tag_exists(root: &Path, tag: &str) -> Result<bool, String> {
    let out = crate::git::run(
        root,
        &["ls-remote", "--tags", "origin", &format!("refs/tags/{tag}")],
    )?;
    Ok(!out.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::release::meta::AssetBin;

    fn release() -> Release {
        Release {
            workspace_root: "/w".into(),
            target_dir: "/w/target".into(),
            version: "0.2.0".to_owned(),
            description: "d".to_owned(),
            member_dirs: Vec::new(),
            crates: vec!["agent-grimoire-model".to_owned(), PACKAGE.to_owned()],
            assets: vec![AssetBin {
                package: "grimoire-tool-x".to_owned(),
                bin: "grimoire-tool-x".to_owned(),
                version: "0.1.0".to_owned(),
                dir: "/w/catalog/tools/x".into(),
            }],
        }
    }

    #[test]
    fn modes() {
        let never = || -> Result<bool, String> { panic!("tag lookup not needed") };
        assert_eq!(
            mode("pull_request", "refs/pull/1/merge", ENABLED, never),
            Ok(Mode::DryRun)
        );
        assert_eq!(mode("push", "refs/heads/main", "", never), Ok(Mode::DryRun));
        assert_eq!(
            mode("push", "refs/heads/feature", ENABLED, never),
            Ok(Mode::DryRun)
        );
        assert_eq!(
            mode("push", "refs/heads/main", ENABLED, || Ok(false)),
            Ok(Mode::Release)
        );
        assert_eq!(
            mode("workflow_dispatch", "refs/heads/main", ENABLED, || Ok(true)),
            Ok(Mode::Skip)
        );
        assert!(mode("push", "refs/heads/main", ENABLED, || Err("offline".into())).is_err());
        assert!(mode("pull_request_target", "refs/heads/main", ENABLED, never).is_err());
    }

    #[test]
    fn growth_limits() {
        let mut r = release();
        assert!(check_limits(&r).is_ok());
        let asset = r.assets[0].clone();
        r.assets = (0..60)
            .map(|i| AssetBin {
                bin: format!("grimoire-tool-{i}"),
                ..asset.clone()
            })
            .collect();
        assert!(check_limits(&r).is_err());
    }

    #[test]
    fn outputs_for_each_mode() {
        let dry = outputs(&release(), Mode::DryRun);
        assert!(
            dry.starts_with("mode=dry-run\nversion=0.2.0\ntag=v0.2.0\n"),
            "{dry}"
        );
        assert!(
            dry.contains(
                r#"targets=[{"target":"x86_64-unknown-linux-musl","runner":"ubuntu-24.04"},"#
            ),
            "{dry}"
        );
        assert_eq!(archives(&release(), Mode::DryRun).len(), 6);
        assert_eq!(archives(&release(), Mode::Release).len(), 12);
        assert!(
            archives(&release(), Mode::Release)
                .contains(&"grimoire-tool-x-0.1.0-aarch64-pc-windows-msvc.zip".to_owned())
        );
        let skip = outputs(&release(), Mode::Skip);
        assert!(
            skip.contains("targets=[]\n") && skip.contains("archives=[]\n"),
            "{skip}"
        );
        assert!(
            skip.contains(r#"crates=["agent-grimoire-model","agent-grimoire"]"#),
            "{skip}"
        );
    }
}
