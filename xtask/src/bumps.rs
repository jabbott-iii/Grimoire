//! Version-bump enforcement against git history (`architecture.md` §6.4).
//!
//! An asset must get a higher version whenever a file that is *installed or
//! built* changes: `asset.toml` and anything else in the asset directory,
//! except `README.md`, `CHANGELOG.md`, `evals/` and `tests/`. The lockfile
//! records hashes of installed content, so a silent content change under an
//! unchanged version would break reproducibility. The catalog lint already
//! requires a CHANGELOG entry for the current version.
//!
//! Removing an asset is allowed only if it was deprecated at the base.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use agent_grimoire_catalog::Catalog;
use agent_grimoire_model::{Diagnostic, Kind, Version};

use crate::git;

/// Files in an asset directory whose changes never require a version bump.
const NOT_CONTENT: [&str; 4] = ["README.md", "CHANGELOG.md", "evals", "tests"];

/// A changed path, attributed to an asset or bundle.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Touched {
    Asset { kind_dir: String, id: String },
    Bundle { file: String },
}

pub(crate) fn check(
    repo: &Path,
    catalog_rel: &str,
    base: &str,
    catalog: &Catalog,
) -> Result<Vec<Diagnostic>, String> {
    // A revision that starts with '-' would be parsed by git as an option.
    if base.is_empty() || base.starts_with('-') {
        return Err(format!("invalid base revision {base:?}"));
    }
    let merge_base = String::from_utf8_lossy(&git::run(repo, &["merge-base", "--end-of-options", base, "HEAD"]).map_err(|e| {
        format!("{e}\nhint: the version-bump check needs history; in CI use actions/checkout with fetch-depth: 0")
    })?)
    .trim()
    .to_owned();

    let mut changed = git::nul_paths(&git::run(
        repo,
        &[
            "diff",
            "--no-renames",
            "--name-only",
            "-z",
            &merge_base,
            "--",
            catalog_rel,
        ],
    )?)?;
    changed.extend(git::nul_paths(&git::run(
        repo,
        &[
            "ls-files",
            "--others",
            "--exclude-standard",
            "-z",
            "--",
            catalog_rel,
        ],
    )?)?);

    let mut touched: BTreeMap<Touched, BTreeSet<String>> = BTreeMap::new();
    for path in &changed {
        let Some(rest) = path
            .strip_prefix(catalog_rel)
            .and_then(|r| r.strip_prefix('/'))
        else {
            continue;
        };
        let parts: Vec<&str> = rest.split('/').collect();
        match parts.as_slice() {
            ["bundles", file] => {
                touched
                    .entry(Touched::Bundle {
                        file: (*file).to_owned(),
                    })
                    .or_default();
            }
            [kind_dir, id, inner @ ..]
                if Kind::from_catalog_dir(kind_dir).is_some() && !inner.is_empty() =>
            {
                touched
                    .entry(Touched::Asset {
                        kind_dir: (*kind_dir).to_owned(),
                        id: (*id).to_owned(),
                    })
                    .or_default()
                    .insert(inner.join("/"));
            }
            _ => {}
        }
    }

    let mut out = Vec::new();
    for (item, files) in touched {
        match item {
            Touched::Asset { kind_dir, id } => {
                asset(
                    repo,
                    catalog_rel,
                    &merge_base,
                    base,
                    catalog,
                    &kind_dir,
                    &id,
                    &files,
                    &mut out,
                );
            }
            Touched::Bundle { file } => bundle(
                repo,
                catalog_rel,
                &merge_base,
                base,
                catalog,
                &file,
                &mut out,
            ),
        }
    }
    Ok(out)
}

fn old_value(text: &str, key: &str) -> Option<String> {
    let table: toml::Table = toml::from_str(text).ok()?;
    table.get(key)?.as_str().map(str::to_owned)
}

#[allow(clippy::too_many_arguments)]
fn asset(
    repo: &Path,
    catalog_rel: &str,
    merge_base: &str,
    base: &str,
    catalog: &Catalog,
    kind_dir: &str,
    id: &str,
    files: &BTreeSet<String>,
    out: &mut Vec<Diagnostic>,
) {
    let dir = format!("{catalog_rel}/{kind_dir}/{id}");
    let manifest_file = format!("{dir}/asset.toml");
    let old = git::show(repo, merge_base, &manifest_file);
    let current = catalog
        .assets()
        .values()
        .find(|a| a.dir == format!("{kind_dir}/{id}"));
    let Some(current) = current else {
        // Removed, or failed to parse (already reported by the catalog lint).
        if let Some(old) = old
            && !repo.join(&dir).exists()
            && old_value(&old, "maturity").as_deref() != Some("deprecated")
        {
            out.push(
                Diagnostic::error(
                    "version/removed-without-deprecation",
                    format!("`{id}` was removed but was not deprecated at {base}; deprecate it in a release first"),
                )
                .in_file(dir),
            );
        }
        return;
    };
    let Some(old_version) = old
        .as_deref()
        .and_then(|t| old_value(t, "version"))
        .and_then(|v| Version::parse(&v).ok())
    else {
        // New asset; the CHANGELOG entry is enforced by the catalog lint.
        return;
    };
    let new_version = &current.manifest.version;
    let content: Vec<&str> = files
        .iter()
        .map(String::as_str)
        .filter(|f| !NOT_CONTENT.contains(&f.split('/').next().unwrap_or_default()))
        .collect();
    if new_version < &old_version {
        out.push(
            Diagnostic::error(
                "version/decreased",
                format!("version went down from {old_version} to {new_version}"),
            )
            .at("version")
            .in_file(manifest_file),
        );
    } else if !content.is_empty() && new_version == &old_version {
        let shown: Vec<&str> = content.iter().take(5).copied().collect();
        out.push(
            Diagnostic::error(
                "version/not-bumped",
                format!(
                    "installed or built files changed since {base} ({}{}) but the version is still {new_version}; bump it and add a CHANGELOG entry",
                    shown.join(", "),
                    if content.len() > 5 { ", …" } else { "" }
                ),
            )
            .at("version")
            .in_file(manifest_file),
        );
    }
}

fn bundle(
    repo: &Path,
    catalog_rel: &str,
    merge_base: &str,
    base: &str,
    catalog: &Catalog,
    file: &str,
    out: &mut Vec<Diagnostic>,
) {
    let path = format!("{catalog_rel}/bundles/{file}");
    let Some(old) = git::show(repo, merge_base, &path) else {
        return;
    };
    let Some(old_version) = old_value(&old, "version").and_then(|v| Version::parse(&v).ok()) else {
        return;
    };
    let current = catalog
        .bundles()
        .values()
        .find(|b| b.file == format!("bundles/{file}"));
    match current {
        None if !repo.join(&path).exists() => out.push(
            Diagnostic::warning(
                "version/bundle-removed",
                format!(
                    "bundle `{file}` was removed; projects that select it will fail to resolve"
                ),
            )
            .in_file(path),
        ),
        Some(current) if current.bundle.version <= old_version => out.push(
            Diagnostic::error(
                "version/not-bumped",
                format!(
                    "the bundle changed since {base} but its version {} is not above {old_version}",
                    current.bundle.version
                ),
            )
            .at("version")
            .in_file(path),
        ),
        // Unchanged-on-disk parse failures are reported by the catalog lint.
        None | Some(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    const MANIFEST: &str = r#"format = 1
id = "rust-errors"
kind = "rule"
version = "1.0.0"
summary = "Error handling"
description = "Typed errors."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "stable"
min-cli-version = "0.1.0"

[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
"#;

    fn write(root: &Path, path: &str, text: &str) {
        let path = root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn git(root: &Path, args: &[&str]) {
        let mut full = vec![
            "-c",
            "user.name=test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
        ];
        full.extend_from_slice(args);
        git::run(root, &full).unwrap();
    }

    /// A repository whose only commit (on `main`) contains one valid rule at 1.0.0.
    fn repo() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        git(root, &["init", "-q", "-b", "main"]);
        let a = "catalog/rules/rust-errors";
        write(root, &format!("{a}/asset.toml"), MANIFEST);
        write(root, &format!("{a}/rule.md"), "Use typed errors.\n");
        write(root, &format!("{a}/README.md"), "# Rust errors\n");
        write(root, &format!("{a}/CHANGELOG.md"), "## 1.0.0\n- Initial.\n");
        write(root, &format!("{a}/evals/cases.md"), "- case\n");
        git(root, &["add", "-A"]);
        git(root, &["commit", "-q", "-m", "initial"]);
        dir
    }

    fn codes(root: &Path) -> Vec<&'static str> {
        let validation = agent_grimoire_catalog::load(&root.join("catalog"));
        assert!(!validation.has_errors(), "{:?}", validation.diagnostics);
        check(root, "catalog", "main", &validation.catalog)
            .unwrap()
            .into_iter()
            .map(|d| d.code)
            .collect()
    }

    fn bump(root: &Path, to: &str) {
        let a = root.join("catalog/rules/rust-errors");
        let manifest = fs::read_to_string(a.join("asset.toml")).unwrap();
        fs::write(
            a.join("asset.toml"),
            manifest.replace("version = \"1.0.0\"", &format!("version = \"{to}\"")),
        )
        .unwrap();
        fs::write(
            a.join("CHANGELOG.md"),
            format!("## {to}\n- Changed.\n\n## 1.0.0\n- Initial.\n"),
        )
        .unwrap();
    }

    #[test]
    fn unchanged_catalog_passes() {
        let dir = repo();
        assert_eq!(codes(dir.path()), Vec::<&str>::new());
    }

    #[test]
    fn content_change_requires_a_bump() {
        let dir = repo();
        write(
            dir.path(),
            "catalog/rules/rust-errors/rule.md",
            "Use typed errors everywhere.\n",
        );
        assert_eq!(codes(dir.path()), ["version/not-bumped"]);
        bump(dir.path(), "1.0.1");
        assert_eq!(codes(dir.path()), Vec::<&str>::new());
    }

    #[test]
    fn untracked_content_counts_as_a_change() {
        let dir = repo();
        write(
            dir.path(),
            "catalog/rules/rust-errors/extra.md",
            "new file\n",
        );
        assert_eq!(codes(dir.path()), ["version/not-bumped"]);
    }

    #[test]
    fn metadata_changes_need_no_bump() {
        let dir = repo();
        write(
            dir.path(),
            "catalog/rules/rust-errors/README.md",
            "# Rust errors\n\nMore docs.\n",
        );
        write(
            dir.path(),
            "catalog/rules/rust-errors/evals/more.md",
            "- another case\n",
        );
        assert_eq!(codes(dir.path()), Vec::<&str>::new());
    }

    #[test]
    fn versions_must_not_go_down() {
        let dir = repo();
        bump(dir.path(), "0.9.0");
        assert_eq!(codes(dir.path()), ["version/decreased"]);
    }

    #[test]
    fn removal_requires_prior_deprecation() {
        let dir = repo();
        fs::remove_dir_all(dir.path().join("catalog/rules/rust-errors")).unwrap();
        let validation = agent_grimoire_catalog::load(&dir.path().join("catalog"));
        let found: Vec<_> = check(dir.path(), "catalog", "main", &validation.catalog)
            .unwrap()
            .into_iter()
            .map(|d| d.code)
            .collect();
        assert_eq!(found, ["version/removed-without-deprecation"]);
    }

    fn commit(root: &Path, message: &str) {
        git(root, &["add", "-A"]);
        git(root, &["commit", "-q", "-m", message]);
    }

    fn all_codes(root: &Path, base: &str) -> Vec<&'static str> {
        let validation = agent_grimoire_catalog::load(&root.join("catalog"));
        check(root, "catalog", base, &validation.catalog)
            .unwrap()
            .into_iter()
            .map(|d| d.code)
            .collect()
    }

    /// Adds a second valid rule, `beta-rule`, with an extra file, and commits it to `main`.
    fn with_second_rule(root: &Path) {
        let b = "catalog/rules/beta-rule";
        let manifest = MANIFEST.replace("rust-errors", "beta-rule");
        write(root, &format!("{b}/asset.toml"), &manifest);
        write(
            root,
            &format!("{b}/rule.md"),
            "Prefer small modules with clear boundaries.\n",
        );
        write(root, &format!("{b}/README.md"), "# Beta\n");
        write(root, &format!("{b}/CHANGELOG.md"), "## 1.0.0\n- Initial.\n");
        write(root, &format!("{b}/evals/cases.md"), "- case\n");
        write(
            root,
            "catalog/rules/rust-errors/extra.md",
            "Extra content.\n",
        );
        commit(root, "second rule");
    }

    #[test]
    fn renaming_an_asset_counts_as_removing_it() {
        let dir = repo();
        let root = dir.path();
        git(root, &["checkout", "-q", "-b", "feature"]);
        git(
            root,
            &["mv", "catalog/rules/rust-errors", "catalog/rules/rust-errs"],
        );
        let m = root.join("catalog/rules/rust-errs/asset.toml");
        fs::write(
            &m,
            fs::read_to_string(&m)
                .unwrap()
                .replace("id = \"rust-errors\"", "id = \"rust-errs\""),
        )
        .unwrap();
        commit(root, "rename");
        assert_eq!(
            all_codes(root, "main"),
            ["version/removed-without-deprecation"]
        );
    }

    #[test]
    fn moving_a_file_between_assets_bumps_both() {
        let dir = repo();
        let root = dir.path();
        with_second_rule(root);
        git(root, &["checkout", "-q", "-b", "feature"]);
        git(
            root,
            &[
                "mv",
                "catalog/rules/rust-errors/extra.md",
                "catalog/rules/beta-rule/extra.md",
            ],
        );
        let b = root.join("catalog/rules/beta-rule");
        fs::write(
            b.join("asset.toml"),
            MANIFEST
                .replace("rust-errors", "beta-rule")
                .replace("1.0.0", "1.0.1"),
        )
        .unwrap();
        fs::write(
            b.join("CHANGELOG.md"),
            "## 1.0.1\n- Moved.\n\n## 1.0.0\n- Initial.\n",
        )
        .unwrap();
        commit(root, "move");
        // rust-errors lost a file but kept its version.
        assert_eq!(all_codes(root, "main"), ["version/not-bumped"]);
    }

    #[test]
    fn removing_a_deprecated_asset_is_allowed() {
        let dir = repo();
        let root = dir.path();
        with_second_rule(root);
        let m = root.join("catalog/rules/rust-errors/asset.toml");
        fs::write(
            &m,
            fs::read_to_string(&m).unwrap().replace("version = \"1.0.0\"", "version = \"1.1.0\"").replace(
                "maturity = \"stable\"",
                "maturity = \"deprecated\"\ndeprecated = { replacement = \"beta-rule\", reason = \"Merged.\" }",
            ),
        )
        .unwrap();
        write(
            root,
            "catalog/rules/rust-errors/CHANGELOG.md",
            "## 1.1.0\n- Deprecated.\n\n## 1.0.0\n- Initial.\n",
        );
        commit(root, "deprecate");
        fs::remove_dir_all(root.join("catalog/rules/rust-errors")).unwrap();
        assert_eq!(all_codes(root, "main"), Vec::<&str>::new());
    }

    const BUNDLE: &str = "format = 1\nid = \"essentials\"\nversion = \"1.0.0\"\nsummary = \"Essentials\"\ndescription = \"x\"\nmaintainers = [\"example-maintainer\"]\n\n[assets]\nrust-errors = \"^1\"\n";

    #[test]
    fn bundle_changes_need_a_bump_and_removal_warns() {
        let dir = repo();
        let root = dir.path();
        write(root, "catalog/bundles/essentials.toml", BUNDLE);
        commit(root, "bundle");

        write(
            root,
            "catalog/bundles/essentials.toml",
            &BUNDLE.replace("summary = \"Essentials\"", "summary = \"The essentials\""),
        );
        assert_eq!(all_codes(root, "main"), ["version/not-bumped"]);

        write(
            root,
            "catalog/bundles/essentials.toml",
            &BUNDLE
                .replace("summary = \"Essentials\"", "summary = \"The essentials\"")
                .replace("1.0.0", "1.1.0"),
        );
        assert_eq!(all_codes(root, "main"), Vec::<&str>::new());

        fs::remove_file(root.join("catalog/bundles/essentials.toml")).unwrap();
        assert_eq!(all_codes(root, "main"), ["version/bundle-removed"]);
    }

    #[test]
    fn option_like_bases_are_refused() {
        let dir = repo();
        let validation = agent_grimoire_catalog::load(&dir.path().join("catalog"));
        for base in ["--output=/tmp/x", "-h", ""] {
            assert!(
                check(dir.path(), "catalog", base, &validation.catalog).is_err(),
                "{base:?}"
            );
        }
    }

    #[test]
    fn a_missing_base_is_a_clear_error() {
        let dir = repo();
        let validation = agent_grimoire_catalog::load(&dir.path().join("catalog"));
        let err = check(dir.path(), "catalog", "no-such-branch", &validation.catalog).unwrap_err();
        assert!(err.contains("fetch-depth"), "{err}");
    }
}
