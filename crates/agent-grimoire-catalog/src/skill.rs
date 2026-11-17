//! Native validation of Agent Skills directories (`SKILLS` in `docs/specs.md`).
//!
//! Grimoire applies the specification's rules, and is deliberately stricter
//! where the spec only recommends:
//! - unknown frontmatter keys are errors;
//! - `allowed-tools` is refused, because it pre-approves tools outside
//!   Grimoire's permission consent;
//! - every bundled script must be referenced from `SKILL.md`, and a skill with
//!   scripts must declare the `shell` permission;
//! - `SKILL.md` must stay under 500 lines.
//!
//! References are found in two ways:
//! - **Links** (Markdown links, images and HTML `src`/`href`) must name an
//!   existing file inside the skill that every client installs.
//! - **Mentions** are path-like words under `scripts/`, `references/` or
//!   `assets/` in prose or code, such as `scripts/extract.py` or
//!   `sh ./scripts/x.sh`. A mention of an existing file counts as documentation;
//!   a mention of a missing file is only a warning, because it may name an
//!   output the skill creates.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use agent_grimoire_model::{AssetManifest, ClientId, Diagnostic, RelPath};
use pulldown_cmark::{Event, Parser, Tag};
use serde::Deserialize;

use crate::asset::METADATA;
use crate::fs::{Tree, read_text};

/// Directories the spec defines inside a skill.
const PAYLOAD_DIRS: [&str; 3] = ["scripts", "references", "assets"];
/// "Keep your main `SKILL.md` under 500 lines."
const MAX_LINES: usize = 500;
/// About 5,000 tokens at roughly four characters per token ("< 5000 tokens recommended").
const WARN_CHARS: usize = 20_000;
const MAX_DESCRIPTION: usize = 1024;
const MAX_COMPATIBILITY: usize = 500;

/// The frontmatter fields the specification defines. Anything else is rejected.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Frontmatter {
    name: String,
    description: String,
    #[serde(default, deserialize_with = "crate::yaml::some_string")]
    license: Option<String>,
    #[serde(default, deserialize_with = "crate::yaml::some_string")]
    compatibility: Option<String>,
    #[serde(default, deserialize_with = "crate::yaml::some_map")]
    metadata: Option<BTreeMap<String, String>>,
    #[serde(
        default,
        rename = "allowed-tools",
        deserialize_with = "crate::yaml::some_string"
    )]
    allowed_tools: Option<String>,
}

pub(crate) fn check(
    dir: &Path,
    tree: &Tree,
    manifest: &AssetManifest,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) {
    let file = format!("{rel_dir}/SKILL.md");
    if !tree.is_file("SKILL.md") {
        out.push(
            Diagnostic::error("skill/missing", "a skill needs SKILL.md at its root")
                .in_file(rel_dir.to_owned()),
        );
        return;
    }
    let text = match read_text(&dir.join("SKILL.md")) {
        Ok(text) => text,
        Err(e) => {
            out.push(Diagnostic::error("skill/frontmatter", e).in_file(file));
            return;
        }
    };
    let Some((yaml, body)) = split_frontmatter(&text, &file, out) else {
        return;
    };
    match crate::yaml::from_str::<Frontmatter>(yaml) {
        Ok(front) => frontmatter(&front, manifest, &file, out),
        Err(e) => out.push(
            Diagnostic::error("skill/frontmatter", format!("invalid frontmatter: {e}"))
                .in_file(&file),
        ),
    }
    if body.trim().is_empty() {
        out.push(
            Diagnostic::error(
                "skill/body",
                "SKILL.md needs instructions after the frontmatter",
            )
            .in_file(&file),
        );
    }
    let lines = text.lines().count();
    if lines >= MAX_LINES {
        out.push(
            Diagnostic::error(
                "skill/length",
                format!(
                    "SKILL.md must stay under {MAX_LINES} lines (got {lines}); move detail into references/"
                ),
            )
            .in_file(&file),
        );
    }
    let chars = body.chars().count();
    if chars > WARN_CHARS {
        out.push(
            Diagnostic::warning(
                "skill/length",
                format!(
                    "SKILL.md instructions are about {} tokens; the spec recommends under 5000",
                    chars / 4
                ),
            )
            .in_file(&file),
        );
    }
    let payload = payload(tree);
    let installed = coverage(tree, manifest, &payload, rel_dir, out);
    let referenced = references(dir, tree, body, &installed, &file, out);
    scripts_documented(tree, manifest, &referenced, rel_dir, out);
}

/// Splits `---\n<yaml>\n---\n<body>`. A UTF-8 byte-order mark is rejected.
fn split_frontmatter<'a>(
    text: &'a str,
    file: &str,
    out: &mut Vec<Diagnostic>,
) -> Option<(&'a str, &'a str)> {
    let fail = |out: &mut Vec<Diagnostic>, msg: &str| {
        out.push(Diagnostic::error("skill/frontmatter", msg.to_owned()).in_file(file.to_owned()));
        None
    };
    if text.starts_with('\u{feff}') {
        return fail(out, "SKILL.md must not start with a byte-order mark");
    }
    let Some(rest) = text
        .strip_prefix("---\n")
        .or_else(|| text.strip_prefix("---\r\n"))
    else {
        return fail(
            out,
            "SKILL.md must start with a `---` line opening the YAML frontmatter",
        );
    };
    let mut offset = 0;
    for line in rest.split_inclusive('\n') {
        if line.trim_end_matches(['\n', '\r']) == "---" {
            return Some((&rest[..offset], &rest[offset + line.len()..]));
        }
        offset += line.len();
    }
    fail(out, "the YAML frontmatter is not closed by a `---` line")
}

fn frontmatter(
    front: &Frontmatter,
    manifest: &AssetManifest,
    file: &str,
    out: &mut Vec<Diagnostic>,
) {
    let err = |code: &'static str, field: &str, msg: String| {
        Diagnostic::error(code, msg)
            .at(field)
            .in_file(file.to_owned())
    };
    if front.name != manifest.id.as_str() {
        out.push(err(
            "skill/name",
            "name",
            format!(
                "name must equal the directory name and asset id `{}`",
                manifest.id
            ),
        ));
    }
    let len = front.description.chars().count();
    if front.description.trim().is_empty() || len > MAX_DESCRIPTION {
        out.push(err(
            "skill/description",
            "description",
            format!("description must be 1-{MAX_DESCRIPTION} characters (got {len})"),
        ));
    }
    if let Some(compat) = &front.compatibility {
        let len = compat.chars().count();
        if compat.trim().is_empty() || len > MAX_COMPATIBILITY {
            out.push(err(
                "skill/compatibility",
                "compatibility",
                format!("compatibility must be 1-{MAX_COMPATIBILITY} characters (got {len})"),
            ));
        }
    }
    if front.license.as_ref().is_some_and(|l| l.trim().is_empty()) {
        out.push(err(
            "skill/license",
            "license",
            "license must not be empty".into(),
        ));
    }
    if let Some(metadata) = &front.metadata
        && metadata.keys().any(|k| k.trim().is_empty())
    {
        out.push(err(
            "skill/metadata",
            "metadata",
            "metadata keys must not be empty".into(),
        ));
    }
    if front.allowed_tools.is_some() {
        out.push(err(
            "skill/allowed-tools",
            "allowed-tools",
            "catalog skills must not use allowed-tools: it pre-approves tools outside Grimoire's permission consent".into(),
        ));
    }
}

/// `SKILL.md` plus every file under the spec's payload directories.
fn payload(tree: &Tree) -> BTreeSet<RelPath> {
    let mut files = BTreeSet::new();
    if let Ok(skill) = RelPath::new("SKILL.md") {
        files.insert(skill);
    }
    for dir in PAYLOAD_DIRS {
        if let Ok(prefix) = RelPath::new(dir) {
            files.extend(tree.files_under(&prefix).cloned());
        }
    }
    files
}

/// Checks that every client installs the whole payload, flags top-level files no
/// client installs, and returns the files that *every* client installs.
fn coverage(
    tree: &Tree,
    manifest: &AssetManifest,
    payload: &BTreeSet<RelPath>,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) -> BTreeSet<RelPath> {
    let mut per_client: BTreeMap<ClientId, BTreeSet<RelPath>> = BTreeMap::new();
    for (client, outputs) in &manifest.clients {
        let mut covered = BTreeSet::new();
        for src in outputs.outputs.iter().filter_map(|o| o.src.as_ref()) {
            covered.extend(tree.files_under(src).cloned());
        }
        let missing: Vec<&str> = payload.difference(&covered).map(RelPath::as_str).collect();
        if !missing.is_empty() {
            let shown = missing
                .iter()
                .take(5)
                .copied()
                .collect::<Vec<_>>()
                .join(", ");
            let more = if missing.len() > 5 {
                format!(" and {} more", missing.len() - 5)
            } else {
                String::new()
            };
            out.push(
                Diagnostic::error(
                    "skill/payload",
                    format!("{client} outputs must install the whole skill; missing {shown}{more}"),
                )
                .at(format!("clients.{client}.outputs"))
                .in_file(format!("{rel_dir}/asset.toml")),
            );
        }
        per_client.insert(*client, covered);
    }
    let anywhere: BTreeSet<&RelPath> = per_client.values().flatten().collect();
    for path in tree.entries.keys() {
        let top = path.components().next().unwrap_or_default();
        let is_top_level = path.components().count() == 1;
        let known = METADATA.contains(&top) || PAYLOAD_DIRS.contains(&top) || top == "SKILL.md";
        if is_top_level && !known && !anywhere.iter().any(|p| p.starts_with(path)) {
            out.push(
                Diagnostic::warning(
                    "skill/extra-file",
                    format!("`{path}` is not part of the skill payload and is never installed"),
                )
                .in_file(format!("{rel_dir}/{path}")),
            );
        }
    }
    let mut clients = per_client.into_values();
    let first = clients.next().unwrap_or_default();
    clients.fold(first, |all, client| {
        all.intersection(&client).cloned().collect()
    })
}

/// How strongly a reference names a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Strength {
    /// A path-like word in prose or code; it may name an output the skill creates.
    Mention,
    /// A Markdown link or image, or an HTML `src`/`href`: it must resolve.
    Link,
}

/// Every local reference in `markdown`, keyed by target, keeping the strongest form.
fn local_references(markdown: &str) -> BTreeMap<String, Strength> {
    let mut found = BTreeMap::new();
    let mut add = |target: String, strength: Strength| {
        let slot = found.entry(target).or_insert(strength);
        *slot = (*slot).max(strength);
    };
    for event in Parser::new(markdown) {
        match event {
            Event::Start(Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. }) => {
                if let Some(target) = link_target(&dest_url) {
                    add(target, Strength::Link);
                }
            }
            Event::Html(html) | Event::InlineHtml(html) => {
                for value in html_targets(&html) {
                    if let Some(target) = link_target(&value) {
                        add(target, Strength::Link);
                    }
                }
            }
            Event::Code(text) | Event::Text(text) => {
                for target in mentions(&text) {
                    add(target, Strength::Mention);
                }
            }
            _ => {}
        }
    }
    found
}

/// Normalizes a link destination. Returns `None` for anchors and non-file URLs.
/// `file:` URLs are returned unchanged so they are rejected as non-relative.
fn link_target(dest: &str) -> Option<String> {
    if dest.is_empty() || dest.starts_with('#') {
        return None;
    }
    if let Some((scheme, _)) = dest.split_once(':')
        && scheme.starts_with(|c: char| c.is_ascii_alphabetic())
        && scheme
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'))
    {
        return scheme.eq_ignore_ascii_case("file").then(|| dest.to_owned());
    }
    let path = dest.split(['#', '?']).next().unwrap_or_default();
    let path = path.strip_prefix("./").unwrap_or(path);
    // pulldown-cmark never percent-decodes destinations. An undecodable target
    // is kept as-is so it is reported as invalid.
    let decoded = percent_decode(path).unwrap_or_else(|| path.to_owned());
    let trimmed = decoded.trim_end_matches('/');
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

/// The values of `src` and `href` attributes in a fragment of raw HTML.
fn html_targets(html: &str) -> Vec<String> {
    let lower = html.to_ascii_lowercase();
    let mut out = Vec::new();
    for attr in ["src", "href"] {
        let mut from = 0;
        while let Some(pos) = lower[from..].find(attr) {
            let start = from + pos;
            from = start + attr.len();
            let at_boundary = start == 0 || {
                let before = lower.as_bytes()[start - 1];
                before.is_ascii_whitespace() || before == b'<'
            };
            let Some(rest) = html[from..].trim_start().strip_prefix('=') else {
                continue;
            };
            if !at_boundary {
                continue;
            }
            let rest = rest.trim_start();
            let value = match rest.chars().next() {
                Some(quote @ ('"' | '\'')) => rest[1..].split(quote).next().unwrap_or_default(),
                _ => rest
                    .split(|c: char| c.is_whitespace() || c == '>')
                    .next()
                    .unwrap_or_default(),
            };
            out.push(value.to_owned());
        }
    }
    out
}

/// Path-like words under the payload directories, e.g. `scripts/x.sh` in
/// `sh ./scripts/x.sh`. Globs and placeholders are not concrete paths and are skipped.
fn mentions(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || "\"'`()[]{},;|=".contains(c))
        .map(|w| w.trim_end_matches(['.', ':', '!', '?']))
        .map(|w| w.strip_prefix("./").unwrap_or(w))
        .filter(|w| PAYLOAD_DIRS.iter().any(|d| w.starts_with(&format!("{d}/"))))
        .filter(|w| {
            w.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | '/'))
        })
        .map(|w| w.trim_end_matches('/').to_owned())
        .collect()
}

fn is_markdown(path: &str) -> bool {
    Path::new(path)
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
}

/// Decodes `%XX` escapes. Returns `None` for malformed escapes or non-UTF-8 results.
fn percent_decode(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' {
            let hex = text.get(i + 1..i + 3)?;
            out.push(u8::from_str_radix(hex, 16).ok()?);
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).ok()
}

/// Checks every reference from `SKILL.md` and returns the referenced paths that exist.
fn references(
    dir: &Path,
    tree: &Tree,
    body: &str,
    installed: &BTreeSet<RelPath>,
    file: &str,
    out: &mut Vec<Diagnostic>,
) -> BTreeSet<RelPath> {
    let mut referenced = BTreeSet::new();
    for (target, strength) in local_references(body) {
        let Ok(path) = RelPath::new(&target) else {
            if strength == Strength::Link {
                out.push(
                    Diagnostic::error(
                        "skill/reference",
                        format!("`{target}` must be a relative path inside the skill"),
                    )
                    .in_file(file.to_owned()),
                );
            }
            continue;
        };
        if tree.get(&path).is_none() {
            let diagnostic = match strength {
                Strength::Link => Diagnostic::error(
                    "skill/reference-missing",
                    format!("`{target}` does not exist in the skill"),
                ),
                Strength::Mention => Diagnostic::warning(
                    "skill/reference-missing",
                    format!(
                        "`{target}` is mentioned but does not exist in the skill; ignore this if it names an output"
                    ),
                ),
            };
            out.push(diagnostic.in_file(file.to_owned()));
            continue;
        }
        if !installed.iter().any(|p| p.starts_with(&path)) {
            out.push(
                Diagnostic::error(
                    "skill/reference-not-installed",
                    format!("`{target}` is referenced but not installed for every client"),
                )
                .in_file(file.to_owned()),
            );
        }
        // "Keep file references one level deep from SKILL.md." Each target is read once.
        if strength == Strength::Link
            && is_markdown(path.as_str())
            && path.as_str() != "SKILL.md"
            && let Ok(nested) = read_text(&dir.join(path.as_str()))
        {
            let chained: Vec<String> = local_references(&nested)
                .into_iter()
                .filter(|(t, s)| *s == Strength::Link && is_markdown(t))
                .map(|(t, _)| t)
                .collect();
            if !chained.is_empty() {
                out.push(
                    Diagnostic::warning(
                        "skill/reference-depth",
                        format!(
                            "`{path}` links on to {}; keep references one level deep from SKILL.md",
                            chained.join(", ")
                        ),
                    )
                    .in_file(file.to_owned()),
                );
            }
        }
        referenced.insert(path);
    }
    referenced
}

/// Bundled scripts must be documented (each referenced from `SKILL.md` by its
/// own path), and a skill that bundles scripts declares the `shell` permission.
fn scripts_documented(
    tree: &Tree,
    manifest: &AssetManifest,
    referenced: &BTreeSet<RelPath>,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) {
    let Ok(scripts) = RelPath::new("scripts") else {
        return;
    };
    if tree.files_under(&scripts).next().is_some() && !manifest.permissions.shell {
        out.push(
            Diagnostic::error(
                "skill/script-permission",
                "a skill that bundles scripts must declare `shell = true` under [permissions]",
            )
            .at("permissions.shell")
            .in_file(format!("{rel_dir}/asset.toml")),
        );
    }
    for script in tree.files_under(&scripts) {
        if !referenced.contains(script) {
            out.push(
                Diagnostic::error(
                    "skill/script-undocumented",
                    format!(
                        "`{script}` is not referenced from SKILL.md; name it (for example `{script}` or a link) where you explain when and how to run it"
                    ),
                )
                .in_file(format!("{rel_dir}/SKILL.md")),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_links_and_mentions() {
        let md = "See [forms](references/forms.md#top) and ![logo](./assets/logo.png).\n\
                  Run `scripts/extract.py`, or [docs](https://example.com) or [here](#anchor).\n\
                  Then run:\n\n```sh\nsh ./scripts/build.sh --out assets/out.png\n```\n\n\
                  Plain prose: scripts/plain.py. Globs like `scripts/*.sh` and `references/<topic>.md` are skipped.\n\
                  <img src=\"assets/raw.png\"> and [dir](references/) and [f](file:///etc/passwd).";
        let found = local_references(md);
        let links: Vec<&str> = found
            .iter()
            .filter(|(_, s)| **s == Strength::Link)
            .map(|(t, _)| t.as_str())
            .collect();
        let mentions: Vec<&str> = found
            .iter()
            .filter(|(_, s)| **s == Strength::Mention)
            .map(|(t, _)| t.as_str())
            .collect();
        assert_eq!(
            links,
            [
                "assets/logo.png",
                "assets/raw.png",
                "file:///etc/passwd",
                "references",
                "references/forms.md"
            ]
        );
        assert_eq!(
            mentions,
            [
                "assets/out.png",
                "scripts/build.sh",
                "scripts/extract.py",
                "scripts/plain.py"
            ]
        );
    }

    #[test]
    fn html_attribute_values() {
        assert_eq!(
            html_targets(r#"<a HREF='x.md'>x</a><img data-src="no" src=y.png alt="src=z">"#),
            ["y.png", "x.md"]
        );
    }

    #[test]
    fn percent_decoding() {
        assert_eq!(
            percent_decode("references/a%20b.md").as_deref(),
            Some("references/a b.md")
        );
        assert_eq!(percent_decode("plain.md").as_deref(), Some("plain.md"));
        assert_eq!(percent_decode("bad%2"), None);
        assert_eq!(percent_decode("bad%zz"), None);
        assert_eq!(
            percent_decode("%2e%2e/escape.md").as_deref(),
            Some("../escape.md")
        );
    }

    #[test]
    fn splits_frontmatter() {
        let mut out = Vec::new();
        let (yaml, body) = split_frontmatter("---\nname: x\n---\nBody\n", "f", &mut out).unwrap();
        assert_eq!(yaml, "name: x\n");
        assert_eq!(body, "Body\n");
        let (yaml, _) = split_frontmatter("---\r\nname: x\r\n---\r\nBody", "f", &mut out).unwrap();
        assert_eq!(yaml, "name: x\r\n");
        assert_eq!(out, Vec::new());
        assert!(split_frontmatter("name: x\n", "f", &mut out).is_none());
        assert!(split_frontmatter("---\nname: x\n", "f", &mut out).is_none());
        assert!(split_frontmatter("\u{feff}---\nname: x\n---\n", "f", &mut out).is_none());
        assert_eq!(out.len(), 3);
    }
}
