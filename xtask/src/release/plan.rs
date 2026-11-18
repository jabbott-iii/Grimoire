//! `cargo xtask release plan`: decides what a run of `.github/workflows/release.yml`
//! does and hands the workflow its matrices (ADR-0009).
//!
//! - `dry-run` on pull requests and on any ref other than `refs/heads/main`, and on
//!   `main` until the maintainer sets the repository variable `GRIMOIRE_RELEASES` to
//!   `enabled` (a workflow that names a missing environment creates it unprotected).
//! - `release` on a push or manual run on `main` when `v{version}` isn't tagged yet.
//! - `skip` on `main` when it is.
//!
//! A run that would release, or that tests the release PR, fails while a public file
//! still has a `<!-- before-first-release` or `<!-- /before-first-release -->` marker
//! ([`check_markers`]). A run that releases also fails while the checked-in catalog pack
//! is out of date ([`check_pack_is_current`]), or holds an asset that needs a newer
//! grimoire than the version it releases ([`check_min_cli_versions`]).

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
            "{archives} release archives exceed GitHub's 256-job matrix limit (sign.yml's sboms job); see ADR-0009"
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

/// The public files that may hold text to change before the first release: the
/// README's "not released yet" note, the code of conduct's contact placeholder and
/// SECURITY.md's "No release exists yet" sentence. The README is published with
/// every release and can't change afterwards.
pub(crate) const MARKED_FILES: [&str; 4] = [
    "README.md",
    "CODE_OF_CONDUCT.md",
    "CONTRIBUTING.md",
    "SECURITY.md",
];

/// Such text sits between an opening marker line, `<!-- before-first-release: … -->`,
/// and a closing one, `<!-- /before-first-release -->`. Both must go: deleting only the
/// opening line leaves the closing one, which still fails the check.
pub(crate) const BEFORE_FIRST_RELEASE: &str = "<!-- before-first-release";

/// The closing marker.
pub(crate) const BEFORE_FIRST_RELEASE_END: &str = "<!-- /before-first-release -->";

/// Whether `line` is an opening or closing marker. The match is loose (any spacing
/// after `<!--`, any case), so a marker typed slightly differently still counts.
pub(crate) fn is_marker(line: &str) -> bool {
    let Some(rest) = line.trim_start().strip_prefix("<!--") else {
        return false;
    };
    let rest = rest.trim_start().to_ascii_lowercase();
    rest.starts_with("before-first-release") || rest.starts_with("/before-first-release")
}

/// Every `file:line` in `files` (name, text) that still carries a marker.
pub(crate) fn before_first_release(files: &[(&str, String)]) -> Vec<String> {
    files
        .iter()
        .flat_map(|(name, text)| {
            text.lines()
                .enumerate()
                .filter(|(_, line)| is_marker(line))
                .map(move |(i, _)| format!("{name}:{}", i + 1))
        })
        .collect()
}

/// Refuses a run that would release, or that tests the release PR, while a marker
/// remains. Both have a `CHANGELOG.md` section for the version: release-plz adds it
/// in the release PR, and release mode needs it.
pub(crate) fn check_markers(
    mode: Mode,
    has_changelog_section: bool,
    files: &[(&str, String)],
) -> Result<(), String> {
    if mode == Mode::Skip || !has_changelog_section {
        return Ok(());
    }
    let marked = before_first_release(files);
    if marked.is_empty() {
        return Ok(());
    }
    Err(format!(
        "{} still {} a `{BEFORE_FIRST_RELEASE}` or `{BEFORE_FIRST_RELEASE_END}` marker. Do what each opening marker says, and delete everything from it to its closing `{BEFORE_FIRST_RELEASE_END}` line, in a pull request merged to main before the release PR (docs/release.md §2 C): the README is published with the release and can't change afterwards",
        marked.join(", "),
        if marked.len() == 1 { "has" } else { "have" },
    ))
}

/// The mode the run ends with, and whether release mode was turned into a dry-run
/// because `CHANGELOG.md` has no section for the version yet. A release needs that
/// section, which the merged release PR adds; without it, any other push to main
/// would release the version early. Only a run that still releases then checks
/// the embedded catalog ([`check_pack_is_current`], [`check_min_cli_versions`]), so
/// no other run reads or packs anything.
pub(crate) fn settle(
    root: &Path,
    mode: Mode,
    has_changelog_section: bool,
    version: &str,
) -> Result<(Mode, bool), String> {
    match mode {
        Mode::Release if !has_changelog_section => Ok((Mode::DryRun, true)),
        Mode::Release => {
            let pack = read_pack(root)?;
            check_pack_is_current(&root.join("catalog"), &pack)?;
            check_min_cli_versions(&pack, version)?;
            Ok((mode, false))
        }
        Mode::DryRun | Mode::Skip => Ok((mode, false)),
    }
}

/// The checked-in pack's path, as messages show it.
fn pack_shown() -> String {
    format!("{}/{}", crate::pack::PACK_DIR, crate::pack::PACK_FILE)
}

/// The checked-in pack under `root`: the catalog the release binaries embed
/// (`include_bytes!` in `crates/agent-grimoire-catalog/src/source.rs`), and that
/// `cargo publish` ships.
pub(crate) fn read_pack(root: &Path) -> Result<Vec<u8>, String> {
    let shown = pack_shown();
    std::fs::read(root.join(&shown)).map_err(|e| format!("cannot read {shown}: {e}"))
}

/// Refuses to release `pack`, the checked-in pack, unless it is exactly what
/// `cargo xtask pack-catalog` makes from `catalog` today. The CLI embeds the
/// checked-in pack while the asset binaries are built from `catalog/`, so a stale
/// pack would ship an older catalog beside newer binaries. CI's catalog job runs
/// `pack-catalog --check` on every pull request, but `release.yml` runs no such
/// check, and a merge can bypass the checks.
pub(crate) fn check_pack_is_current(catalog: &Path, pack: &[u8]) -> Result<(), String> {
    let shown = pack_shown();
    let fresh = agent_grimoire_catalog::pack(catalog).map_err(|diagnostics| {
        let errors: Vec<String> = diagnostics
            .iter()
            .filter(|d| d.is_error())
            .map(ToString::to_string)
            .collect();
        format!(
            "catalog/ is invalid, so {shown} can't be checked: {}. Fix it in a pull request merged to main; `cargo xtask validate` shows every problem",
            errors.join("; ")
        )
    })?;
    if fresh == pack {
        return Ok(());
    }
    Err(format!(
        "{shown} is out of date: it isn't what `cargo xtask pack-catalog` makes from catalog/, and the release would embed it. Run `cargo xtask pack-catalog` and merge the result to main in a pull request"
    ))
}

/// Refuses to release `version` while `pack`, the embedded catalog, holds an asset
/// whose `min-cli-version` is above it: that release could never install it
/// (`resolve/cli-too-old`). CI's tests refuse such an asset on every pull request
/// (`crates/agent-grimoire/tests/builtin.rs`), but `release.yml` runs no tests, and
/// a merge can bypass the checks.
pub(crate) fn check_min_cli_versions(pack: &[u8], version: &str) -> Result<(), String> {
    let version = agent_grimoire_model::Version::parse(version)
        .map_err(|e| format!("the workspace version `{version}`: {e}"))?;
    let source =
        agent_grimoire_catalog::Source::from_pack(agent_grimoire_model::SourceId::Builtin, pack)
            .map_err(|e| format!("{}: {e}", pack_shown()))?;
    let newer: Vec<String> = source
        .assets()
        .values()
        .map(|a| &a.manifest)
        .filter(|m| version < m.min_cli_version)
        .map(|m| format!("`{}` (min-cli-version {})", m.id, m.min_cli_version))
        .collect();
    if newer.is_empty() {
        return Ok(());
    }
    let one = newer.len() == 1;
    Err(format!(
        "{} {} a newer grimoire than {version}, the version this run releases, so that release could never install {} (`resolve/cli-too-old`). In a pull request merged to main, lower the asset's `min-cli-version` (with an asset version bump and an entry in the asset's CHANGELOG.md, then `cargo xtask pack-catalog`), or release a version at least that new through a release PR (docs/release.md §1)",
        newer.join(", "),
        if one { "needs" } else { "need" },
        if one { "it" } else { "them" },
    ))
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
    fn markers_block_the_release_pr_and_the_release() {
        let marked = vec![
            ("README.md", "# Grimoire\n\n<!-- before-first-release: delete this note -->\n> Not released yet.\n".to_owned()),
            ("CODE_OF_CONDUCT.md", "Report to:\n  <!-- before-first-release: fill in the contact -->\n".to_owned()),
        ];
        assert_eq!(
            before_first_release(&marked),
            ["README.md:3", "CODE_OF_CONDUCT.md:2"]
        );
        let err = check_markers(Mode::Release, true, &marked).unwrap_err();
        assert!(
            err.contains("README.md:3, CODE_OF_CONDUCT.md:2 still have"),
            "{err}"
        );
        // The release PR is a dry-run with the version's CHANGELOG section.
        assert!(check_markers(Mode::DryRun, true, &marked).is_err());
        // Other pull requests, and runs with nothing to release, are not affected.
        assert!(check_markers(Mode::DryRun, false, &marked).is_ok());
        assert!(check_markers(Mode::Skip, true, &marked).is_ok());

        // Only a line that starts with a marker counts.
        let prose = vec![(
            "README.md",
            "Mention `<!-- before-first-release` in prose.\n".to_owned(),
        )];
        assert!(check_markers(Mode::Release, true, &prose).is_ok());
        assert!(check_markers(Mode::Release, true, &[]).is_ok());
    }

    #[test]
    fn a_marked_span_fails_until_both_markers_are_gone() {
        let span = |text: &str| vec![("README.md", text.to_owned())];
        let whole = "<!-- before-first-release: delete to the closing line -->\n> Not released yet.\n<!-- /before-first-release -->\n";
        assert_eq!(
            before_first_release(&span(whole)),
            ["README.md:1", "README.md:3"]
        );
        // Deleting only the opening line leaves the closing one, and the note.
        let half = "> Not released yet.\n<!-- /before-first-release -->\n";
        assert_eq!(before_first_release(&span(half)), ["README.md:2"]);
        assert!(check_markers(Mode::Release, true, &span(half)).is_err());
        assert!(check_markers(Mode::Release, true, &span("> Released.\n")).is_ok());
        // Spacing and case don't hide a marker.
        for typo in [
            "<!--before-first-release: x -->",
            "  <!--   Before-First-Release -->",
            "<!--/before-first-release-->",
            "<!-- /BEFORE-FIRST-RELEASE -->",
        ] {
            assert!(is_marker(typo), "{typo}");
        }
        assert!(!is_marker("<!-- released -->"));
        assert!(!is_marker("before-first-release"));
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
        assert_eq!(archives(&release(), Mode::DryRun).len(), 8);
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

    /// A fixture catalog whose `future-rule` needs grimoire `min`.
    fn catalog(min: &str) -> agent_grimoire_testkit::FixtureCatalog {
        let catalog = agent_grimoire_testkit::FixtureCatalog::new();
        catalog
            .rule("plain-rule", "1.0.0", &[], "# Plain\n\nText.\n")
            .rule("future-rule", "1.0.0", &[], "# Future\n\nText.\n")
            .edit_manifest("rules/future-rule", |m| {
                m.replace(
                    "min-cli-version = \"0.1.0\"",
                    &format!("min-cli-version = \"{min}\""),
                )
            });
        catalog
    }

    fn copy_tree(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).unwrap();
        for entry in std::fs::read_dir(from).unwrap() {
            let entry = entry.unwrap();
            let target = to.join(entry.file_name());
            if entry.file_type().unwrap().is_dir() {
                copy_tree(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }

    /// A repository root holding `catalog` under `catalog/` and its pack, checked in.
    fn root_with(catalog: &agent_grimoire_testkit::FixtureCatalog) -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        copy_tree(catalog.root(), &root.path().join("catalog"));
        let dir = root.path().join(crate::pack::PACK_DIR);
        std::fs::create_dir_all(&dir).unwrap();
        let bytes = agent_grimoire_catalog::pack(catalog.root()).unwrap();
        std::fs::write(dir.join(crate::pack::PACK_FILE), bytes).unwrap();
        root
    }

    #[test]
    fn a_release_refuses_an_asset_that_needs_a_newer_grimoire() {
        let pack = |min: &str| agent_grimoire_catalog::pack(catalog(min).root()).unwrap();
        let future = pack("0.3.0");
        let err = check_min_cli_versions(&future, "0.2.0").unwrap_err();
        assert!(
            err.contains("`future-rule` (min-cli-version 0.3.0)"),
            "{err}"
        );
        assert!(!err.contains("plain-rule"), "{err}");
        assert!(check_min_cli_versions(&future, "0.3.0").is_ok());
        assert!(check_min_cli_versions(&future, "0.3.1").is_ok());
        // A pre-release sorts before its release.
        assert!(check_min_cli_versions(&future, "0.3.0-rc.1").is_err());
        assert!(check_min_cli_versions(&pack("0.3.0-rc.1"), "0.3.0-rc.1").is_ok());
        assert!(check_min_cli_versions(b"not a pack", "0.3.0").is_err());
    }

    #[test]
    fn a_release_refuses_a_stale_pack() {
        let fixture = catalog("0.1.0");
        let pack = agent_grimoire_catalog::pack(fixture.root()).unwrap();
        assert_eq!(check_pack_is_current(fixture.root(), &pack), Ok(()));
        // A changed asset file, a pack that isn't one, and a missing catalog.
        std::fs::write(
            fixture.root().join("rules/plain-rule/rule.md"),
            "# Plain\n\nChanged.\n",
        )
        .unwrap();
        let err = check_pack_is_current(fixture.root(), &pack).unwrap_err();
        assert!(err.contains("out of date"), "{err}");
        let fresh = agent_grimoire_catalog::pack(fixture.root()).unwrap();
        assert_eq!(check_pack_is_current(fixture.root(), &fresh), Ok(()));
        assert!(check_pack_is_current(fixture.root(), b"not a pack").is_err());
        let empty = tempfile::tempdir().unwrap();
        assert!(check_pack_is_current(&empty.path().join("catalog"), &fresh).is_err());
        // An invalid catalog can't be packed, so it can't be checked either.
        fixture.edit_manifest("rules/plain-rule", |m| {
            m.replace("version = \"1.0.0\"", "version = \"x\"")
        });
        let err = check_pack_is_current(fixture.root(), &fresh).unwrap_err();
        assert!(err.contains("catalog/ is invalid"), "{err}");
    }

    #[test]
    fn only_a_run_that_still_releases_checks_the_pack() {
        // With no pack to read, only a run that still releases fails.
        let empty = tempfile::tempdir().unwrap();
        assert_eq!(
            settle(empty.path(), Mode::Release, false, "0.2.0"),
            Ok((Mode::DryRun, true))
        );
        assert_eq!(
            settle(empty.path(), Mode::DryRun, true, "0.2.0"),
            Ok((Mode::DryRun, false))
        );
        assert_eq!(
            settle(empty.path(), Mode::Skip, true, "0.2.0"),
            Ok((Mode::Skip, false))
        );
        assert!(settle(empty.path(), Mode::Release, true, "0.2.0").is_err());

        let root = root_with(&catalog("0.3.0"));
        assert_eq!(
            settle(root.path(), Mode::Release, true, "0.3.0"),
            Ok((Mode::Release, false))
        );
        let err = settle(root.path(), Mode::Release, true, "0.2.0").unwrap_err();
        assert!(err.contains("min-cli-version 0.3.0"), "{err}");

        // A stale pack stops a release, before its `min-cli-version`s are read.
        std::fs::write(
            root.path().join("catalog/rules/plain-rule/rule.md"),
            "# Plain\n\nChanged.\n",
        )
        .unwrap();
        for version in ["0.2.0", "0.3.0"] {
            let err = settle(root.path(), Mode::Release, true, version).unwrap_err();
            assert!(err.contains("out of date"), "{err}");
        }
        assert_eq!(
            settle(root.path(), Mode::DryRun, true, "0.3.0"),
            Ok((Mode::DryRun, false))
        );
    }

    #[test]
    fn the_checks_read_the_checked_in_pack() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let pack = read_pack(root).unwrap();
        // 0.0.0 is below every asset's `min-cli-version`, so an error naming
        // them shows the pack was read and parsed.
        let err = check_min_cli_versions(&pack, "0.0.0").unwrap_err();
        assert!(err.contains("min-cli-version"), "{err}");
    }
}
