//! Loading and checking one asset directory.

use std::collections::BTreeMap;
use std::path::Path;

use agent_grimoire_model::{
    AssetId, AssetManifest, Diagnostic, Kind, RelPath, Strategy, Version, validate,
};

use crate::fs::{EntryKind, Tree, read_text, walk};
use crate::rules::RuleText;
use crate::{code, skill};

/// Files and directories that describe an asset but are never installed.
pub(crate) const METADATA: [&str; 5] =
    ["asset.toml", "README.md", "CHANGELOG.md", "evals", "tests"];

/// Rules must stay under this many lines; Cursor advises "under 500 lines".
const RULE_MAX_LINES: usize = 500;
/// Rules at or above this length get a warning; Claude Code advises "under 200 lines".
const RULE_WARN_LINES: usize = 200;
/// GitHub Copilot caps a custom agent's prompt at 30,000 characters (`COPILOT-AGENTS`).
const AGENT_MAX_CHARS: usize = 30_000;

/// An asset that parsed, with what later catalog-wide checks need.
pub(crate) struct LoadedAsset {
    pub(crate) dir: String,
    pub(crate) manifest: AssetManifest,
    pub(crate) rule_texts: Vec<RuleText>,
}

/// Loads `root/<kind dir>/<id>/` and checks everything local to it.
pub(crate) fn load(
    root: &Path,
    kind: Kind,
    id: &AssetId,
    out: &mut Vec<Diagnostic>,
) -> Option<LoadedAsset> {
    let rel_dir = format!("{}/{id}", kind.catalog_dir());
    let dir = root.join(kind.catalog_dir()).join(id.as_str());
    let tree = walk(&dir, &rel_dir, out);
    let manifest_file = format!("{rel_dir}/asset.toml");
    if !tree.is_file("asset.toml") {
        out.push(
            Diagnostic::error(
                "catalog/manifest-missing",
                "every asset needs an asset.toml",
            )
            .in_file(rel_dir),
        );
        return None;
    }
    let manifest = match read_text(&dir.join("asset.toml"))
        .and_then(|text| AssetManifest::from_toml(&text).map_err(|e| e.to_string()))
    {
        Ok(manifest) => manifest,
        Err(message) => {
            out.push(Diagnostic::error("catalog/manifest-parse", message).in_file(manifest_file));
            return None;
        }
    };
    if manifest.id != *id {
        out.push(
            Diagnostic::error(
                "catalog/id-mismatch",
                format!("id must equal the directory name `{id}`"),
            )
            .at("id")
            .in_file(&manifest_file),
        );
    }
    if manifest.kind != kind {
        out.push(
            Diagnostic::error(
                "catalog/kind-mismatch",
                format!(
                    "kind must be \"{kind}\" for assets under catalog/{}/",
                    kind.catalog_dir()
                ),
            )
            .at("kind")
            .in_file(&manifest_file),
        );
    }
    out.extend(
        validate::manifest(&manifest)
            .into_iter()
            .map(|d| d.in_file(&manifest_file)),
    );

    if tree.truncated {
        // The walk stopped early; checking file presence now would only add false errors.
        return Some(LoadedAsset {
            dir: rel_dir,
            manifest,
            rule_texts: Vec::new(),
        });
    }
    metadata_files(&dir, &tree, &manifest, &rel_dir, out);
    let texts = sources(&dir, &tree, &manifest, &rel_dir, out);

    let mut rule_texts = Vec::new();
    match manifest.kind {
        Kind::Rule => {
            for (src, text) in &texts {
                rule_length(text, &format!("{rel_dir}/{src}"), out);
                rule_markers(text, &format!("{rel_dir}/{src}"), out);
                rule_texts.push(RuleText {
                    id: manifest.id.clone(),
                    file: format!("{rel_dir}/{src}"),
                    text: text.clone(),
                });
            }
        }
        Kind::Agent => {
            for (src, text) in &texts {
                let chars = text.chars().count();
                if chars > AGENT_MAX_CHARS {
                    out.push(
                        Diagnostic::error(
                            "agent/length",
                            format!("agent prompts may be at most {AGENT_MAX_CHARS} characters (got {chars})"),
                        )
                        .in_file(format!("{rel_dir}/{src}")),
                    );
                }
            }
        }
        Kind::Skill => skill::check(&dir, &tree, &manifest, &rel_dir, out),
        Kind::Tool | Kind::McpServer => code::check(&dir, &tree, &manifest, &rel_dir, out),
    }
    Some(LoadedAsset {
        dir: rel_dir,
        manifest,
        rule_texts,
    })
}

fn metadata_files(
    dir: &Path,
    tree: &Tree,
    manifest: &AssetManifest,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) {
    for name in ["README.md", "CHANGELOG.md"] {
        let file = format!("{rel_dir}/{name}");
        if !tree.is_file(name) {
            out.push(
                Diagnostic::error("catalog/metadata", format!("every asset needs a {name}"))
                    .in_file(file),
            );
            continue;
        }
        match read_text(&dir.join(name)) {
            Err(e) => out.push(Diagnostic::error("catalog/metadata", e).in_file(file)),
            Ok(text) if text.trim().is_empty() => {
                out.push(Diagnostic::error("catalog/metadata", format!("{name} is empty")).in_file(file));
            }
            Ok(text) if name == "CHANGELOG.md" && !changelog_has(&text, &manifest.version) => out.push(
                Diagnostic::error(
                    "catalog/changelog",
                    format!(
                        "CHANGELOG.md needs an entry for the current version: a heading `## {0}` or `## [{0}]`",
                        manifest.version
                    ),
                )
                .in_file(file),
            ),
            Ok(_) => {}
        }
    }
    let has_files_in = |name: &str| {
        RelPath::new(name)
            .is_ok_and(|prefix| tree.is_dir(name) && tree.files_under(&prefix).next().is_some())
    };
    if manifest.kind.is_code_bearing() {
        if !has_files_in("tests") {
            out.push(
                Diagnostic::error(
                    "catalog/tests",
                    "tools and MCP servers need integration tests under tests/",
                )
                .in_file(rel_dir.to_owned()),
            );
        }
    } else if !has_files_in("evals") && !has_files_in("tests") {
        out.push(
            Diagnostic::error(
                "catalog/tests",
                "every asset needs evals/ or tests/ with at least one file",
            )
            .in_file(rel_dir.to_owned()),
        );
    }
}

#[must_use]
/// Whether `changelog` has a level-2 heading for `version`: `## 1.2.3`, `## [1.2.3]`, `## 1.2.3 - date`.
pub fn changelog_has(changelog: &str, version: &Version) -> bool {
    let version = version.to_string();
    changelog.lines().any(|line| {
        line.strip_prefix("## ")
            .map(|rest| rest.trim_start().trim_start_matches('['))
            .and_then(|rest| rest.strip_prefix(version.as_str()))
            .is_some_and(|after| after.is_empty() || after.starts_with([']', ' ', '\t']))
    })
}

/// Checks every output `src` and returns the text of each rendered or block source, keyed by path.
fn sources(
    dir: &Path,
    tree: &Tree,
    manifest: &AssetManifest,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) -> BTreeMap<RelPath, String> {
    let mut texts = BTreeMap::new();
    let file = format!("{rel_dir}/asset.toml");
    for (client, outputs) in &manifest.clients {
        for (i, output) in outputs.outputs.iter().enumerate() {
            let Some(src) = &output.src else { continue };
            let field = format!("clients.{client}.outputs[{i}].src");
            let first = src.components().next().unwrap_or_default();
            if METADATA.contains(&first) {
                out.push(
                    Diagnostic::error(
                        "catalog/src-metadata",
                        format!("`{src}` is asset metadata and is never installed"),
                    )
                    .at(&field)
                    .in_file(&file),
                );
                continue;
            }
            match (tree.get(src), output.strategy) {
                (None, _) => out.push(
                    Diagnostic::error(
                        "catalog/src-missing",
                        format!("`{src}` does not exist in {rel_dir}/"),
                    )
                    .at(&field)
                    .in_file(&file),
                ),
                (Some(EntryKind::Dir), Strategy::Render | Strategy::Block) => out.push(
                    Diagnostic::error(
                        "catalog/src-type",
                        format!("`{src}` must be a file for this strategy"),
                    )
                    .at(&field)
                    .in_file(&file),
                ),
                (Some(EntryKind::Dir), _) if tree.files_under(src).next().is_none() => out.push(
                    Diagnostic::error("catalog/src-type", format!("`{src}` is an empty directory"))
                        .at(&field)
                        .in_file(&file),
                ),
                (Some(EntryKind::File { .. }), Strategy::Render | Strategy::Block) => {
                    if texts.contains_key(src) {
                        continue;
                    }
                    match read_text(&dir.join(src.as_str())) {
                        Ok(text) if text.trim().is_empty() => out.push(
                            Diagnostic::error("catalog/src-text", format!("`{src}` is empty"))
                                .at(&field)
                                .in_file(&file),
                        ),
                        Ok(text) => {
                            texts.insert(src.clone(), text);
                        }
                        Err(e) => out.push(
                            Diagnostic::error("catalog/src-text", format!("`{src}` {e}"))
                                .at(&field)
                                .in_file(&file),
                        ),
                    }
                }
                _ => {}
            }
        }
    }
    texts
}

/// Rules become managed blocks in shared files, delimited by `<!-- grimoire:… -->`
/// marker lines. Rule text holding such a line would break the block around it,
/// and leading frontmatter would be read as the client's own.
fn rule_markers(text: &str, file: &str, out: &mut Vec<Diagnostic>) {
    if let Some((n, _)) = text
        .lines()
        .enumerate()
        .find(|(_, line)| line.trim_start().starts_with("<!-- grimoire:"))
    {
        out.push(
            Diagnostic::error(
                "rule/marker",
                format!(
                    "line {} starts with `<!-- grimoire:`, which is reserved for Grimoire's managed-block markers",
                    n + 1
                ),
            )
            .in_file(file.to_owned()),
        );
    }
    if ends_in_open_fence(text) {
        out.push(
            Diagnostic::error(
                "rule/unclosed-fence",
                "the text ends inside an unclosed code fence, which would turn everything after its block in AGENTS.md into code",
            )
            .in_file(file.to_owned()),
        );
    }
    if text.starts_with("---") {
        out.push(
            Diagnostic::error(
                "rule/frontmatter",
                "rules carry no frontmatter; Grimoire generates each client's frontmatter from asset.toml",
            )
            .in_file(file.to_owned()),
        );
    }
}

/// Whether `text` ends inside a `CommonMark` fenced code block: an opening line of
/// three or more backticks or tildes (indented at most three spaces, and for
/// backticks with no backtick after the run) without a matching closing line.
pub(crate) fn ends_in_open_fence(text: &str) -> bool {
    let mut open: Option<(char, usize)> = None;
    for line in text.lines() {
        let indent = line.len() - line.trim_start_matches(' ').len();
        if indent > 3 {
            continue;
        }
        let rest = &line[indent..];
        let Some(c) = rest.chars().next().filter(|c| *c == '`' || *c == '~') else {
            continue;
        };
        let run = rest.chars().take_while(|x| *x == c).count();
        if run < 3 {
            continue;
        }
        let after = &rest[run..];
        match open {
            None if !(c == '`' && after.contains('`')) => open = Some((c, run)),
            Some((oc, on)) if oc == c && run >= on && after.trim().is_empty() => open = None,
            _ => {}
        }
    }
    open.is_some()
}

fn rule_length(text: &str, file: &str, out: &mut Vec<Diagnostic>) {
    let lines = text.lines().count();
    if lines >= RULE_MAX_LINES {
        out.push(
            Diagnostic::error(
                "rule/length",
                format!("rules must stay under {RULE_MAX_LINES} lines (got {lines})"),
            )
            .in_file(file.to_owned()),
        );
    } else if lines >= RULE_WARN_LINES {
        out.push(
            Diagnostic::warning(
                "rule/length",
                format!("rules should stay under {RULE_WARN_LINES} lines (got {lines}); consider a skill"),
            )
            .in_file(file.to_owned()),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_fences() {
        assert!(ends_in_open_fence("Run:\n\n```sh\ncargo test\n"));
        assert!(!ends_in_open_fence("Run:\n\n```sh\ncargo test\n```\n"));
        assert!(!ends_in_open_fence("A span: ```` ``` ```` here.\n"));
        assert!(!ends_in_open_fence("    ```\nindented code\n"));
        assert!(
            ends_in_open_fence("~~~~\n~~~\n"),
            "a shorter run does not close"
        );
    }

    #[test]
    fn changelog_headings() {
        let v = Version::parse("1.0.1").unwrap();
        assert!(changelog_has("# Changelog\n\n## 1.0.1\n- x", &v));
        assert!(changelog_has("## [1.0.1] - 2026-11-16", &v));
        assert!(changelog_has("## 1.0.1 - 2026-11-16", &v));
        assert!(!changelog_has("## 1.0.10", &v));
        assert!(!changelog_has("### 1.0.1", &v));
        assert!(!changelog_has("## 1.0.0", &v));
    }
}
