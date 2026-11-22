//! Every crate under `crates/` documents itself in one shape ([ADR-0012], crate
//! documentation). This module's test fails when one doesn't.
//!
//! - **Crate docs.** The `//!` lines at the top of a crate's `src/lib.rs`, which
//!   `cargo doc` and docs.rs turn into the crate's front page, open with the
//!   sections `# Where it fits`, `# The parts` and `# Example`, in that order.
//!   More sections may follow.
//! - **A README of its own.** A published crate (one without `publish = false`)
//!   that sets `readme = "README.md"` gets that file as its crates.io page. The
//!   README:
//!   - starts with `# <package name>`;
//!   - has the sections `## Status`, `## How the CLI uses it`, `## Platforms`
//!     (optional), `## Documentation` and `## License`, in that order, and no
//!     other `##` section;
//!   - links to the crate's docs.rs page;
//!   - holds at least one Rust example, which `src/lib.rs` compiles and runs as a
//!     doc test, through a `#[cfg(doctest)]` item whose doc is the README. No
//!     example hides lines with `# `: only rustdoc hides them, and GitHub and
//!     crates.io show them as written;
//!   - links only to `https://` URLs, `mailto:` addresses, `#fragment`s and its
//!     own `LICENSE-APACHE` and `LICENSE-MIT`, because crates.io resolves a
//!     relative link against the crate's directory on GitHub.
//! - **The workspace README.** Only `agent-grimoire` uses the repository's
//!   README (`readme.workspace = true`): that file is the CLI's crates.io page,
//!   and `readme_links` checks it. Any other published crate needs its own.
//!
//! The checks are structural: they can't tell whether the words are true. The
//! rule in `AGENTS.md` (a change to a crate updates its docs in the same
//! change) covers that.
//!
//! [ADR-0012]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0012-crate-documentation.md

use std::fs;
use std::path::Path;

use pulldown_cmark::{CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag, TagEnd};

use crate::readme_links::{allowed, link_targets};

/// The crate-docs sections every crate opens with, in order.
const CRATE_DOCS_SECTIONS: [&str; 3] = ["Where it fits", "The parts", "Example"];

/// A crate README's `##` sections, in order, and whether each is required.
const README_SECTIONS: [(&str, bool); 5] = [
    ("Status", true),
    ("How the CLI uses it", true),
    ("Platforms", false),
    ("Documentation", true),
    ("License", true),
];

/// The only crate whose crates.io page is the repository's README.
const WORKSPACE_README_CRATE: &str = "agent-grimoire";

/// The item that makes `src/lib.rs` compile the README's examples as doc tests.
const README_DOCTESTS: &str = "#[cfg(doctest)]\n#[doc = include_str!(\"../README.md\")]\n";

/// The headings of one level in `markdown`, in order, outside code blocks.
fn headings(markdown: &str, level: HeadingLevel) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Option<String> = None;
    for event in Parser::new_ext(markdown, Options::ENABLE_GFM) {
        match event {
            Event::Start(Tag::Heading { level: l, .. }) if l == level => {
                current = Some(String::new());
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = current.as_mut() {
                    heading.push_str(&text);
                }
            }
            Event::End(TagEnd::Heading(l)) if l == level => {
                if let Some(heading) = current.take() {
                    found.push(heading.trim().to_owned());
                }
            }
            _ => {}
        }
    }
    found
}

/// The text of every fenced code block tagged `rust` in `markdown`.
fn rust_examples(markdown: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Option<String> = None;
    for event in Parser::new_ext(markdown, Options::ENABLE_GFM) {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(info)))
                if info.split([',', ' ']).next() == Some("rust") =>
            {
                current = Some(String::new());
            }
            Event::Text(text) => {
                if let Some(code) = current.as_mut() {
                    code.push_str(&text);
                }
            }
            Event::End(TagEnd::CodeBlock) => found.extend(current.take()),
            _ => {}
        }
    }
    found
}

/// Whether rustdoc hides `line` of a doc test: one that is `#`, or starts with
/// `# `, after its indentation.
fn is_hidden(line: &str) -> bool {
    let line = line.trim_start();
    line == "#" || line.starts_with("# ")
}

/// The crate docs of a `src/lib.rs`: its leading `//!` lines, without the
/// marker and the one space after it.
fn crate_docs(lib_rs: &str) -> String {
    lib_rs
        .lines()
        .map_while(|line| line.strip_prefix("//!"))
        .map(|line| line.strip_prefix(' ').unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// What is wrong with a crate's docs, given its `src/lib.rs`.
fn check_crate_docs(lib_rs: &str) -> Vec<String> {
    let docs = crate_docs(lib_rs);
    if docs.trim().is_empty() {
        return vec!["src/lib.rs has no `//!` crate docs at its top".to_owned()];
    }
    let found = headings(&docs, HeadingLevel::H1);
    let opening: Vec<&str> = found
        .iter()
        .take(CRATE_DOCS_SECTIONS.len())
        .map(String::as_str)
        .collect();
    if opening == CRATE_DOCS_SECTIONS {
        Vec::new()
    } else {
        vec![format!(
            "src/lib.rs's crate docs must open with the sections {}, in that order; they have {found:?}",
            CRATE_DOCS_SECTIONS.map(|s| format!("`# {s}`")).join(", ")
        )]
    }
}

/// What is wrong with the README of the published crate `name`.
fn check_readme(name: &str, readme: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let titles = headings(readme, HeadingLevel::H1);
    if titles.first().map(String::as_str) != Some(name) {
        problems.push(format!(
            "README.md must start with `# {name}`; its first title is {:?}",
            titles.first()
        ));
    }
    let sections = headings(readme, HeadingLevel::H2);
    let expected: Vec<&str> = README_SECTIONS
        .iter()
        .filter(|(section, required)| *required || sections.iter().any(|s| s == section))
        .map(|(section, _)| *section)
        .collect();
    if sections
        .iter()
        .map(String::as_str)
        .ne(expected.iter().copied())
    {
        problems.push(format!(
            "README.md's `##` sections must be {} (Platforms optional), in that order; they are {sections:?}",
            README_SECTIONS.map(|(s, _)| format!("`{s}`")).join(", ")
        ));
    }
    if !readme.contains(&format!("https://docs.rs/{name}")) {
        problems.push(format!(
            "README.md must link to the API reference, https://docs.rs/{name}"
        ));
    }
    let examples = rust_examples(readme);
    if examples.is_empty() {
        problems.push("README.md must hold a Rust example (a ```rust block)".to_owned());
    }
    if examples.iter().flat_map(|code| code.lines()).any(is_hidden) {
        problems.push(
            "README.md: a Rust example hides lines with `# `. Only rustdoc hides them: \
             GitHub and crates.io show them as written, so write them as plain code"
                .to_owned(),
        );
    }
    for (line, target) in link_targets(readme) {
        if !allowed(&target) && target != "LICENSE-APACHE" && target != "LICENSE-MIT" {
            problems.push(format!(
                "README.md:{line}: `{target}` is relative: crates.io resolves it against the crate's \
                 directory. Use https://github.com/jabbott-iii/Grimoire/blob/main/<path>"
            ));
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    const DOCS: &str = "//! A crate.\n//!\n//! # Where it fits\n//!\n//! Here.\n//!\n\
                        //! # The parts\n//!\n//! - [`a`]\n//!\n//! # Example\n//!\n\
                        //! ```\n//! # fn main() {}\n//! ```\n//!\n//! # Errors\n\nmod a;\n";

    const README: &str = "# x\n\nWhat.\n\n## Status\n\nBefore 1.0.\n\n## How the CLI uses it\n\n\
                          ```rust\nlet a = 1;\n```\n\n## Documentation\n\n\
                          - API reference: https://docs.rs/x\n\n## License\n\n\
                          [MIT](LICENSE-MIT) or [Apache](LICENSE-APACHE).\n";

    #[test]
    fn accepts_the_shape() {
        assert_eq!(check_crate_docs(DOCS), Vec::<String>::new());
        assert_eq!(check_readme("x", README), Vec::<String>::new());
        let with_platforms = README.replace(
            "## Documentation",
            "## Platforms\n\nSame everywhere.\n\n## Documentation",
        );
        assert_eq!(check_readme("x", &with_platforms), Vec::<String>::new());
    }

    #[test]
    fn crate_docs_need_the_sections_in_order() {
        assert_eq!(check_crate_docs("mod a;\n").len(), 1);
        let swapped = DOCS
            .replace("# Where it fits", "# TMP")
            .replace("# The parts", "# Where it fits")
            .replace("# TMP", "# The parts");
        assert_eq!(check_crate_docs(&swapped).len(), 1);
        assert_eq!(
            check_crate_docs(&DOCS.replace("# Example", "# Examples")).len(),
            1
        );
    }

    #[test]
    fn a_heading_inside_a_code_block_is_not_a_section() {
        let docs = "//! A crate.\n//!\n//! ```\n//! # Where it fits\n//! ```\n";
        assert_eq!(check_crate_docs(docs).len(), 1);
    }

    #[test]
    fn readme_needs_its_title_sections_reference_and_example() {
        assert_eq!(
            check_readme("y", README).len(),
            2,
            "wrong title and docs.rs link"
        );
        let extra = README.replace("## License", "## Notes\n\nMore.\n\n## License");
        assert_eq!(check_readme("x", &extra).len(), 1);
        let reordered = README
            .replace("## Status", "## TMP")
            .replace("## How the CLI uses it", "## Status")
            .replace("## TMP", "## How the CLI uses it");
        assert_eq!(check_readme("x", &reordered).len(), 1);
        let no_example = README.replace("```rust", "```text");
        assert_eq!(check_readme("x", &no_example).len(), 1);
        let hidden = README.replace("let a = 1;", "# fn main() {\nlet a = 1;\n# }");
        assert_eq!(check_readme("x", &hidden).len(), 1);
        let attribute = README.replace("let a = 1;", "#[derive(Debug)]\nstruct A;");
        assert_eq!(check_readme("x", &attribute), Vec::<String>::new());
    }

    #[test]
    fn readme_links_must_be_absolute_except_the_licenses() {
        let relative = README.replace("Before 1.0.", "See [the docs](../../docs/cli.md).");
        let problems = check_readme("x", &relative);
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("README.md:7:"), "{problems:?}");
    }

    /// The `[package]` table of a crate's `Cargo.toml`.
    fn package(dir: &Path) -> toml::Table {
        let path = dir.join("Cargo.toml");
        let text = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let mut manifest: toml::Table = text
            .parse()
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        match manifest.remove("package") {
            Some(toml::Value::Table(package)) => package,
            _ => panic!("{} has no [package]", path.display()),
        }
    }

    /// Every crate under `crates/` follows ADR-0012's shapes (see the module docs).
    #[test]
    fn every_crate_is_documented_in_the_shape() {
        let crates = Path::new(env!("CARGO_MANIFEST_DIR")).join("../crates");
        let mut dirs: Vec<_> = fs::read_dir(&crates)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", crates.display()))
            .map(|entry| entry.expect("a directory entry").path())
            .filter(|path| path.join("Cargo.toml").is_file())
            .collect();
        dirs.sort();
        assert!(!dirs.is_empty(), "no crates under {}", crates.display());

        let mut problems = Vec::new();
        for dir in &dirs {
            let package = package(dir);
            let name = package["name"].as_str().expect("a package name").to_owned();
            let rel = format!("crates/{}", dir.file_name().unwrap().to_string_lossy());
            let lib_rs = fs::read_to_string(dir.join("src/lib.rs")).unwrap_or_default();
            problems.extend(
                check_crate_docs(&lib_rs)
                    .into_iter()
                    .map(|p| format!("{rel}: {p}")),
            );

            let published = package.get("publish").and_then(toml::Value::as_bool) != Some(false);
            match package.get("readme") {
                Some(toml::Value::String(readme)) if readme == "README.md" => {
                    let text = fs::read_to_string(dir.join("README.md")).unwrap_or_default();
                    problems.extend(
                        check_readme(&name, &text)
                            .into_iter()
                            .map(|p| format!("{rel}/{p}")),
                    );
                    if !lib_rs.contains(README_DOCTESTS) {
                        problems.push(format!(
                            "{rel}: src/lib.rs must run the README's examples as doc tests:\n\
                             #[cfg(doctest)]\n#[doc = include_str!(\"../README.md\")]\n\
                             struct ReadmeDoctests;"
                        ));
                    }
                }
                Some(toml::Value::Table(readme))
                    if readme.get("workspace").and_then(toml::Value::as_bool) == Some(true)
                        && name == WORKSPACE_README_CRATE => {}
                Some(other) => problems.push(format!(
                    "{rel}: `readme = {other}`: a crate other than {WORKSPACE_README_CRATE} sets \
                     `readme = \"README.md\"`, a README of its own"
                )),
                None if published => problems.push(format!(
                    "{rel}: a published crate needs `readme = \"README.md\"` and a README in the \
                     crate-docs shape (ADR-0012)"
                )),
                None => {}
            }
        }
        assert!(
            problems.is_empty(),
            "crate docs out of shape (ADR-0012, xtask/src/crate_docs.rs):\n{}",
            problems.join("\n")
        );
    }
}
