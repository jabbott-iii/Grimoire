//! `docs/diagnostics.md` lists every diagnostic code the CLI can print, and no
//! other code.
//!
//! The codes in the source are the string literals in the `src/` directories of
//! the five crates whose diagnostics reach a CLI user ([`CRATES`]). The
//! documented codes are the first column of the doc's tables. How the source is
//! read, and the limits of that:
//! - `Diagnostic::code` is a `&'static str`, and no crate builds one at run time
//!   (no `format!`, `concat!` or leaked `String` makes a code), so every code is
//!   a literal. A code is exactly `"<prefix>/<name>"`: a prefix of lowercase
//!   letters, digits and `-` that starts with a letter, and a name of lowercase
//!   letters, digits and `-`. Two kinds of literal are read: every literal of
//!   that shape, and every literal that starts with a known prefix (one in
//!   [`USER_FACING`] or [`NOT_USER_FACING`]) and `/`. The second kind must have
//!   the shape too, so a misspelt code such as `"plan/hook_missing"` or
//!   `"plan/codex/override"` fails this test rather than going unseen. A
//!   literal of either kind that isn't a code, such as a path, goes in
//!   [`NOT_CODES`].
//! - This is a text scan, not a parse. A quoted code in a comment counts too. A
//!   quote after an odd number of backslashes is escaped, so it neither opens
//!   nor closes a literal. A code built at run time would be missed, and so
//!   would a literal that has neither the shape nor a known prefix, such as
//!   `"Plan/x"` or `"plan_x/y"`.
//! - Test code is skipped: a line that is exactly `#[cfg(test)]`, the attribute
//!   lines after it, and the item it marks. A block item ends at the next line
//!   that is exactly `}` (rustfmt closes a top-level item at column 0), and a
//!   one-line item at its `;`. For `#[cfg(test)] mod name;`, the module's file
//!   and directory are skipped too: `name.rs` and `name/` beside a `lib.rs`,
//!   `main.rs` or `mod.rs`, and otherwise in the directory named after the
//!   declaring file. Any other `#[cfg(test)]` item, or one with `#[path]`, fails
//!   this test instead of being skipped. An indented `#[cfg(test)]` isn't
//!   skipped, so its literals count. These limits fail loudly rather than hide
//!   a code.
//! - Every code-shaped literal needs a known prefix: one in [`USER_FACING`],
//!   which must be documented, or one in [`NOT_USER_FACING`], with the reason the
//!   CLI never prints it. A new prefix fails this test until it is classified,
//!   and so does a code-shaped literal that isn't a code, such as a path, until
//!   it is listed in [`NOT_CODES`] or rewritten.
//! - The catalog crate isn't read. Its codes (`catalog/…` and the other
//!   catalog-author codes) come from catalog validation, and the CLI uses only
//!   its `Source`, whose errors carry no code.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

/// The crates whose diagnostics reach a CLI user, under `crates/`.
const CRATES: [&str; 5] = [
    "agent-grimoire",
    "agent-grimoire-adapters",
    "agent-grimoire-install",
    "agent-grimoire-model",
    "agent-grimoire-resolve",
];

/// Prefixes of the codes the CLI can print. Each must be documented.
const USER_FACING: [&str; 16] = [
    "adapter", "add", "apply", "cli", "config", "doctor", "fs", "init", "lock", "plan", "project",
    "remove", "resolve", "show", "update", "verify",
];

/// Prefixes in those crates whose codes the CLI never prints, and why.
const NOT_USER_FACING: [(&str, &str); 3] = [
    (
        "manifest",
        "only `validate::manifest` raises these; catalog validation calls it, and so does the \
         catalog pack loader, which reports a failure without the code",
    ),
    (
        "bundle",
        "only `validate::bundle` raises these, called as for `manifest`",
    ),
    (
        "plugin",
        "only `plugin::files` raises these, and only `cargo xtask gen-marketplace` calls it",
    ),
];

/// Literals in those crates that the scan reads but that aren't codes, such as a
/// path, each with what it is. None is needed yet.
const NOT_CODES: [(&str, &str); 0] = [];

/// This file, named in messages that ask for a literal to be classified.
const THIS_FILE: &str = "crates/agent-grimoire/tests/diagnostics_doc.rs";

/// Whether `text` has the shape of a code: `<prefix>/<name>`.
fn is_code(text: &str) -> bool {
    let word = |s: &str| {
        !s.is_empty()
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    };
    text.split_once('/').is_some_and(|(prefix, name)| {
        prefix.starts_with(|c: char| c.is_ascii_lowercase()) && word(prefix) && word(name)
    })
}

/// Whether `prefix` is in [`USER_FACING`] or [`NOT_USER_FACING`].
fn known(prefix: &str) -> bool {
    USER_FACING.contains(&prefix) || NOT_USER_FACING.iter().any(|(p, _)| *p == prefix)
}

/// Whether the scan reads `text`: it has the shape of a code, or it starts with
/// a known prefix and `/`.
fn is_candidate(text: &str) -> bool {
    is_code(text)
        || text
            .split_once('/')
            .is_some_and(|(prefix, _)| known(prefix))
}

/// What `literal`, a candidate from [`candidates`], is: `Ok(true)` for a code
/// the CLI can print, and `Ok(false)` for a code it never prints or a literal in
/// `not_codes`.
///
/// # Errors
///
/// Says what to decide about a literal with an unknown prefix, or with a known
/// prefix but not the shape of a code.
fn classify(literal: &str, not_codes: &[(&str, &str)]) -> Result<bool, String> {
    if not_codes.iter().any(|(l, _)| *l == literal) {
        return Ok(false);
    }
    let prefix = literal.split('/').next().unwrap_or_default();
    if !known(prefix) {
        return Err(format!(
            "`{literal}`: the prefix `{prefix}/` is in neither USER_FACING nor NOT_USER_FACING \
             in {THIS_FILE}. If it is a code, add the prefix to USER_FACING and document its \
             codes in docs/diagnostics.md, or add it to NOT_USER_FACING with the reason the CLI \
             never prints them. If it isn't a code (a path, say), add it to NOT_CODES there with \
             what it is, or rewrite it so it isn't shaped like `<prefix>/<name>`"
        ));
    }
    if !is_code(literal) {
        return Err(format!(
            "`{literal}` starts with the code prefix `{prefix}/`, but a code is \
             `<prefix>/<name>`, with a name of lowercase letters, digits and `-`. Rename the \
             code, or, if it isn't a code (a path, say), add it to NOT_CODES in {THIS_FILE} with \
             what it is"
        ));
    }
    Ok(USER_FACING.contains(&prefix))
}

/// What a source file holds outside its test code.
#[derive(Debug, Default, PartialEq, Eq)]
struct Production {
    /// The source without its `#[cfg(test)]` items.
    code: String,
    /// The modules it declares as `#[cfg(test)] mod name;`, whose files are test code.
    test_modules: Vec<String>,
}

/// `source` without its `#[cfg(test)]` items.
///
/// # Errors
///
/// Fails on a `#[cfg(test)]` item this can't skip safely.
fn without_tests(source: &str) -> Result<Production, String> {
    let mut kept = Production::default();
    let mut lines = source.lines().enumerate();
    while let Some((n, line)) = lines.next() {
        if line != "#[cfg(test)]" {
            kept.code.push_str(line);
            kept.code.push('\n');
            continue;
        }
        let at = n + 1;
        let mut item = None;
        for (_, l) in lines.by_ref() {
            if l.starts_with("#[path") {
                return Err(format!("line {at}: a `#[cfg(test)]` module has `#[path]`"));
            }
            if !l.starts_with("#[") {
                item = Some(l);
                break;
            }
        }
        match item {
            Some(l) if l.ends_with('{') => {
                if !lines.by_ref().any(|(_, l)| l == "}") {
                    return Err(format!(
                        "line {at}: the `#[cfg(test)]` block has no closing `}}` at column 0"
                    ));
                }
            }
            Some(l) if l.ends_with(';') => {
                if let Some(name) = module_name(l) {
                    kept.test_modules.push(name.to_owned());
                }
            }
            other => {
                return Err(format!(
                    "line {at}: can't tell where the `#[cfg(test)]` item {other:?} ends"
                ));
            }
        }
    }
    Ok(kept)
}

/// The name in `mod name;`, `pub mod name;` or `pub(crate) mod name;`.
fn module_name(line: &str) -> Option<&str> {
    let rest = line.strip_suffix(';')?;
    let rest = rest
        .strip_prefix("pub ")
        .or_else(|| rest.strip_prefix("pub(crate) "))
        .unwrap_or(rest);
    rest.strip_prefix("mod ")
}

/// The file and the directory of module `name`, declared in `file`.
fn module_paths(file: &Path, name: &str) -> [PathBuf; 2] {
    let dir = file.parent().unwrap();
    let stem = file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    let base = if matches!(stem, "lib" | "main" | "mod") {
        dir.to_path_buf()
    } else {
        dir.join(stem)
    };
    [base.join(format!("{name}.rs")), base.join(name)]
}

/// Every string literal in `source` that the scan reads ([`is_candidate`]). Each
/// `"` that isn't escaped is tried as an opening quote, so the scan never depends
/// on pairing quotes up correctly; text between two literals never starts with a
/// prefix and `/` in Rust.
fn candidates(source: &str) -> BTreeSet<&str> {
    let quotes: Vec<usize> = source
        .match_indices('"')
        .map(|(i, _)| i)
        .filter(|&i| {
            source[..i]
                .bytes()
                .rev()
                .take_while(|&b| b == b'\\')
                .count()
                .is_multiple_of(2)
        })
        .collect();
    quotes
        .windows(2)
        .map(|pair| &source[pair[0] + 1..pair[1]])
        .filter(|text| is_candidate(text))
        .collect()
}

/// The first-column cells of the doc's tables that hold a backticked text with
/// a `/`: a row starting `` | `plan/conflict` ``. Cells such as `` `severity` ``
/// have no `/` and are skipped.
fn documented(doc: &str) -> Vec<&str> {
    doc.lines()
        .filter_map(|line| line.strip_prefix("| `")?.split_once('`'))
        .map(|(cell, _)| cell)
        .filter(|cell| cell.contains('/'))
        .collect()
}

/// The `.rs` files under `dir`, in a stable order. A symbolic link fails.
fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let kind = entry.file_type().unwrap();
        assert!(!kind.is_symlink(), "{} is a symbolic link", path.display());
        if kind.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
    out.sort();
}

#[test]
fn diagnostics_doc_lists_every_user_facing_code() {
    let crates = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let repo = crates.parent().unwrap();
    let mut problems = Vec::new();

    // Each user-facing code, with the files that raise it.
    let mut raised: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
    let mut sources = Vec::new();
    for name in CRATES {
        let mut files = Vec::new();
        rust_files(&crates.join(name).join("src"), &mut files);
        assert!(!files.is_empty(), "no source files in crates/{name}/src");
        for file in files {
            match without_tests(&fs::read_to_string(&file).unwrap()) {
                Ok(production) => sources.push((file, production)),
                Err(e) => problems.push(format!("{}: {e}", file.display())),
            }
        }
    }
    // The files and directories of `#[cfg(test)] mod name;` modules.
    let test_only: Vec<PathBuf> = sources
        .iter()
        .flat_map(|(file, p)| p.test_modules.iter().flat_map(|m| module_paths(file, m)))
        .collect();
    let mut read = BTreeSet::new();
    for (file, production) in &sources {
        if test_only.iter().any(|t| file.starts_with(t)) {
            continue;
        }
        let shown = file.strip_prefix(repo).unwrap().display().to_string();
        for literal in candidates(&production.code) {
            read.insert(literal);
            match classify(literal, &NOT_CODES) {
                Ok(true) => {
                    raised.entry(literal).or_default().insert(shown.clone());
                }
                Ok(false) => {}
                Err(e) => problems.push(format!("{shown}: {e}")),
            }
        }
    }
    for (literal, _) in NOT_CODES {
        if !read.contains(literal) {
            problems.push(format!(
                "the source has no literal `{literal}`; remove it from NOT_CODES"
            ));
        }
    }
    for prefix in USER_FACING {
        if !raised.keys().any(|c| c.split('/').next() == Some(prefix)) {
            problems.push(format!(
                "no code in the source has the prefix `{prefix}/`; remove it from USER_FACING"
            ));
        }
    }

    let doc = fs::read_to_string(repo.join("docs/diagnostics.md")).unwrap();
    let mut listed = BTreeSet::new();
    for code in documented(&doc) {
        if !is_code(code) {
            problems.push(format!(
                "docs/diagnostics.md: `{code}` in a table's first column is not a code"
            ));
        } else if !listed.insert(code) {
            problems.push(format!("docs/diagnostics.md lists `{code}` more than once"));
        }
    }
    for (code, files) in &raised {
        if !listed.contains(code) {
            let files: Vec<&str> = files.iter().map(String::as_str).collect();
            problems.push(format!(
                "`{code}` is missing from docs/diagnostics.md (raised in {})",
                files.join(", ")
            ));
        }
    }
    for code in &listed {
        if !raised.contains_key(code) {
            problems.push(format!(
                "docs/diagnostics.md lists `{code}`, but no user-facing code in the source has it"
            ));
        }
    }
    assert!(
        problems.is_empty(),
        "docs/diagnostics.md and the source disagree:\n- {}",
        problems.join("\n- ")
    );
}

#[test]
fn extractor_skips_tests_and_finds_every_code_shaped_literal() {
    let source = r#"
fn run(out: &mut Out<'_, '_>) -> Exit {
    let shown = format!("\"{id}\" and \"plan/quoted\"");
    let path = ".claude/rules/x.md";
    let words = "two words/x";
    // A quoted "comment/code" counts too.
    out.diag(Diagnostic::warning(
        "plan/multi-line",
        "a message with a / in it",
    ));
    out.fail(Exit::Invalid, "init/exists", "it exists")
}

#[cfg(test)]
mod docs_check;
#[cfg(test)]
use std::fmt::Write as _;

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    #[test]
    fn helper() {
        assert_eq!(code(), "plan/test-only");
    }
}

fn after_the_tests() -> &'static str {
    "fs/after-tests"
}
"#;
    let production = without_tests(source).unwrap();
    assert!(
        !production.code.contains("plan/test-only")
            && !production.code.contains("docs_check")
            && !production.code.contains("fmt::Write"),
        "{}",
        production.code
    );
    assert_eq!(production.test_modules, ["docs_check"]);
    // The escaped quotes in the `format!` don't end its literal.
    assert_eq!(
        candidates(&production.code).into_iter().collect::<Vec<_>>(),
        [
            "comment/code",
            "fs/after-tests",
            "init/exists",
            "plan/multi-line"
        ]
    );
    // Test items this can't skip safely fail instead of hiding code.
    for unsafe_to_skip in [
        "#[cfg(test)]\nmod tests {\n    fn f() {}\n",
        "#[cfg(test)]\n#[path = \"other.rs\"]\nmod tests;\n",
        "#[cfg(test)]\nconst CODES: &[&str] = &[\n    \"plan/x\",\n];\n",
        "#[cfg(test)]\n",
    ] {
        assert!(without_tests(unsafe_to_skip).is_err(), "{unsafe_to_skip}");
    }
    assert_eq!(module_name("pub(crate) mod helpers;"), Some("helpers"));
    assert_eq!(module_name("use std::fmt;"), None);
    let src = Path::new("crates/x/src");
    assert_eq!(
        module_paths(&src.join("lib.rs"), "docs_check"),
        [src.join("docs_check.rs"), src.join("docs_check")]
    );
    assert_eq!(
        module_paths(&src.join("entry.rs"), "toml"),
        [src.join("entry/toml.rs"), src.join("entry/toml")]
    );
}

#[test]
fn misshapen_codes_and_unknown_prefixes_need_a_decision() {
    let source = r#"
    out.diag(Diagnostic::warning("plan/hook_missing", "x"));
    out.diag(Diagnostic::warning("plan/codex/override", "x"));
    let raw = (r"\", "plan/after-a-raw-string");
    let kept = ("plan/conflict", "bundle/id-missing", "skills/x", "manifest/Bad");
    let ignored = ("plan_x/y", "Plan/x", ".claude/plan/x.md", "a / b");
"#;
    assert_eq!(
        candidates(source).into_iter().collect::<Vec<_>>(),
        [
            "bundle/id-missing",
            "manifest/Bad",
            // A raw string that ends in `\` doesn't hide the next literal.
            "plan/after-a-raw-string",
            "plan/codex/override",
            "plan/conflict",
            "plan/hook_missing",
            "skills/x"
        ]
    );
    assert_eq!(classify("plan/conflict", &[]), Ok(true));
    assert_eq!(classify("bundle/id-missing", &[]), Ok(false));
    for misshapen in ["plan/hook_missing", "plan/codex/override", "manifest/Bad"] {
        let e = classify(misshapen, &[]).unwrap_err();
        assert!(
            e.contains("Rename the code") && e.contains(THIS_FILE),
            "{e}"
        );
    }
    let e = classify("skills/x", &[]).unwrap_err();
    assert!(
        e.contains("USER_FACING") && e.contains("NOT_CODES") && e.contains(THIS_FILE),
        "{e}"
    );
    assert_eq!(classify("skills/x", &[("skills/x", "a path")]), Ok(false));
    assert_eq!(
        classify("plan/a_path.md", &[("plan/a_path.md", "a path")]),
        Ok(false)
    );
}

#[test]
fn documented_codes_come_from_the_first_column() {
    let doc = "\
| Code | Severity |
|---|---|
| `plan/conflict` | error |
| `Plan/Bad` | warning |
| `severity` | `\"error\"` or `\"warning\"` |
| project commands | `fs/io` |

Prose that names `fs/refused`.
";
    assert_eq!(documented(doc), ["plan/conflict", "Plan/Bad"]);
    assert!(is_code("plan/conflict") && is_code("resolve/cli-too-old"));
    for bad in [
        "Plan/Bad",
        "plan/",
        "/x",
        "a/b/c",
        "plan/x y",
        "1x/y",
        "two words/x",
    ] {
        assert!(!is_code(bad), "{bad}");
    }
}
