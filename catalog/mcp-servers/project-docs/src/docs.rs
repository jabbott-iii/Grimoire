//! Read-only, root-confined access to a project's documentation.
//!
//! All access goes through a capability handle on the root directory
//! (`cap-std`), so no path can leave it, even if the tree changes during a
//! request. On top of that, symbolic links are never followed, hidden files and
//! directories (names starting with `.`, such as `.env` or `.git`) are never
//! read, and only documentation files are served: Markdown, reStructuredText
//! and AsciiDoc.

use std::io::{self, Read as _};
use std::path::Path;

use cap_fs_ext::{DirExt as _, FollowSymlinks, OpenOptionsFollowExt as _};
use cap_std::fs::{Dir, OpenOptions};

/// File extensions served, compared case-insensitively.
pub(crate) const EXTENSIONS: [&str; 5] = ["md", "markdown", "mdx", "rst", "adoc"];
/// Directories never entered, besides hidden ones: dependency and build output.
const SKIPPED_DIRS: [&str; 5] = ["node_modules", "target", "vendor", "__pycache__", "venv"];
/// The largest document read.
pub(crate) const MAX_DOC_BYTES: u64 = 1024 * 1024;
/// The most documents a listing or search considers.
pub(crate) const MAX_DOCS: usize = 5000;
/// How deep a walk goes.
const MAX_DEPTH: usize = 16;
/// The longest path accepted, in bytes.
pub(crate) const MAX_PATH_BYTES: usize = 512;
/// Directory names that hold architecture decision records.
const ADR_DIRS: [&str; 3] = ["adr", "adrs", "decisions"];

/// Why a document could not be served. Every refusal that depends on what is on
/// disk reads the same from the outside as "not found", so a caller cannot probe
/// for files it may not read. Paths refused for their form never get this far.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum DocError {
    /// The path is not a document this server serves: invalid, outside the
    /// root, hidden, a symbolic link, not a documentation file, or missing.
    NotFound(String),
    /// The document is larger than [`MAX_DOC_BYTES`].
    TooLarge(String),
    /// The document is not UTF-8 text.
    NotText(String),
    /// Reading failed.
    Io(String),
}

impl std::fmt::Display for DocError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocError::NotFound(p) => write!(
                f,
                "`{p}` is not a document in this project; use list_docs to see the available paths"
            ),
            DocError::TooLarge(p) => write!(
                f,
                "`{p}` is larger than {} KiB, the most this server reads",
                MAX_DOC_BYTES / 1024
            ),
            DocError::NotText(p) => write!(f, "`{p}` is not UTF-8 text"),
            DocError::Io(p) => write!(f, "`{p}` could not be read"),
        }
    }
}

/// A validated document path: relative, `/`-separated, with no empty, `.`,
/// `..` or hidden components, no backslashes or control characters, and a
/// documentation extension.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct DocPath(String);

impl DocPath {
    /// Validates a directory prefix or a document path.
    fn check(value: &str) -> Result<(), String> {
        if value.is_empty() {
            return Err("the path is empty".to_owned());
        }
        if value.len() > MAX_PATH_BYTES {
            return Err(format!("the path is longer than {MAX_PATH_BYTES} bytes"));
        }
        if value.starts_with('/') {
            return Err("the path must be relative to the project root".to_owned());
        }
        if value.chars().any(|c| c.is_control() || c == '\\') {
            return Err("the path contains a backslash or a control character".to_owned());
        }
        for part in value.split('/') {
            if part.is_empty() || part == "." || part == ".." {
                return Err("the path has an empty, `.` or `..` component".to_owned());
            }
            if part.starts_with('.') {
                return Err("hidden files and directories are never served".to_owned());
            }
            if SKIPPED_DIRS.contains(&part) {
                return Err(format!(
                    "dependency and build directories ({}) are never served",
                    SKIPPED_DIRS.join(", ")
                ));
            }
        }
        Ok(())
    }

    /// Parses the path of a document.
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        Self::check(value)?;
        if !is_doc_name(value) {
            return Err(format!(
                "only documentation files are served ({})",
                EXTENSIONS.map(|e| format!(".{e}")).join(", ")
            ));
        }
        Ok(Self(value.to_owned()))
    }

    /// Parses a directory prefix such as `docs/adr`. A trailing `/` is allowed.
    pub(crate) fn prefix(value: &str) -> Result<String, String> {
        let trimmed = value.strip_suffix('/').unwrap_or(value);
        Self::check(trimmed)?;
        Ok(trimmed.to_owned())
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_doc_name(name: &str) -> bool {
    Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| EXTENSIONS.iter().any(|x| x.eq_ignore_ascii_case(e)))
}

/// One document found by a walk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Entry {
    /// Path relative to the root, `/`-separated.
    pub(crate) path: String,
    /// Size in bytes.
    pub(crate) bytes: u64,
}

/// The documents under a root.
pub(crate) struct Docs {
    root: Dir,
}

impl Docs {
    /// Opens `root` as the documentation root.
    pub(crate) fn open(root: &Path) -> io::Result<Self> {
        Ok(Self {
            root: Dir::open_ambient_dir(root, cap_std::ambient_authority())?,
        })
    }

    /// Every document under `prefix` (or the whole root), sorted by path, and
    /// whether the walk stopped at [`MAX_DOCS`].
    pub(crate) fn walk(&self, prefix: Option<&str>) -> (Vec<Entry>, bool) {
        let mut out = Vec::new();
        let mut truncated = false;
        let start = match prefix {
            None => self.root.try_clone().ok(),
            Some(prefix) => open_dirs(&self.root, prefix),
        };
        if let Some(dir) = start {
            walk(
                &dir,
                prefix.unwrap_or_default(),
                0,
                &mut out,
                &mut truncated,
            );
        }
        out.sort_by(|a, b| a.path.cmp(&b.path));
        (out, truncated)
    }

    /// Reads one document as text.
    pub(crate) fn read(&self, path: &DocPath) -> Result<String, DocError> {
        self.read_limited(path, None)
    }

    /// Reads the first `bytes` bytes of a document, decoded leniently: enough
    /// for its title or an ADR's status line.
    pub(crate) fn head(&self, path: &DocPath, bytes: u64) -> Result<String, DocError> {
        self.read_limited(path, Some(bytes))
    }

    fn read_limited(&self, path: &DocPath, head: Option<u64>) -> Result<String, DocError> {
        let shown = path.as_str();
        let not_found = || DocError::NotFound(shown.to_owned());
        let (parent, name) = match shown.rsplit_once('/') {
            Some((parent, name)) => (open_dirs(&self.root, parent).ok_or_else(not_found)?, name),
            None => (self.root.try_clone().map_err(|_| not_found())?, shown),
        };
        let meta = parent.symlink_metadata(name).map_err(|_| not_found())?;
        if !meta.is_file() {
            return Err(not_found());
        }
        if head.is_none() && meta.len() > MAX_DOC_BYTES {
            return Err(DocError::TooLarge(shown.to_owned()));
        }
        let mut options = OpenOptions::new();
        options.read(true).follow(FollowSymlinks::No);
        // Non-blocking, so a FIFO swapped in after the check above cannot hang us.
        #[cfg(unix)]
        {
            use cap_std::fs::OpenOptionsExt as _;
            options.custom_flags(libc::O_NONBLOCK);
        }
        let file = parent.open_with(name, &options).map_err(|_| not_found())?;
        // Check what was opened, not only what the path showed before.
        let opened = file
            .metadata()
            .map_err(|_| DocError::Io(shown.to_owned()))?;
        if !opened.is_file() {
            return Err(not_found());
        }
        let mut bytes = Vec::new();
        file.take(head.unwrap_or(MAX_DOC_BYTES + 1))
            .read_to_end(&mut bytes)
            .map_err(|_| DocError::Io(shown.to_owned()))?;
        if head.is_some() {
            return Ok(String::from_utf8_lossy(&bytes).into_owned());
        }
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_DOC_BYTES {
            return Err(DocError::TooLarge(shown.to_owned()));
        }
        String::from_utf8(bytes).map_err(|_| DocError::NotText(shown.to_owned()))
    }
}

/// Opens each directory of `rel` in turn, refusing symbolic links and hidden
/// names. `None` if any step fails.
fn open_dirs(root: &Dir, rel: &str) -> Option<Dir> {
    let mut dir = root.try_clone().ok()?;
    for part in rel.split('/') {
        if part.is_empty() || part.starts_with('.') || SKIPPED_DIRS.contains(&part) {
            return None;
        }
        dir = dir.open_dir_nofollow(part).ok()?;
    }
    Some(dir)
}

fn walk(dir: &Dir, rel: &str, depth: usize, out: &mut Vec<Entry>, truncated: &mut bool) {
    if depth > MAX_DEPTH || *truncated {
        return;
    }
    let Ok(read) = dir.entries() else { return };
    let mut entries: Vec<(String, cap_std::fs::FileType)> = read
        .filter_map(Result::ok)
        .filter_map(|e| {
            let name = e.file_name().into_string().ok()?;
            let kind = e.file_type().ok()?;
            Some((name, kind))
        })
        .collect();
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, kind) in entries {
        if name.starts_with('.') || kind.is_symlink() {
            continue;
        }
        let path = if rel.is_empty() {
            name.clone()
        } else {
            format!("{rel}/{name}")
        };
        if path.len() > MAX_PATH_BYTES || path.chars().any(|c| c.is_control() || c == '\\') {
            continue;
        }
        if kind.is_dir() {
            if SKIPPED_DIRS.contains(&name.as_str()) {
                continue;
            }
            if let Ok(child) = dir.open_dir_nofollow(&name) {
                walk(&child, &path, depth + 1, out, truncated);
            }
        } else if kind.is_file() && is_doc_name(&name) {
            if out.len() >= MAX_DOCS {
                *truncated = true;
                return;
            }
            let bytes = dir.symlink_metadata(&name).map_or(0, |m| m.len());
            out.push(Entry { path, bytes });
        }
        if *truncated {
            return;
        }
    }
}

/// The document's title: its first level-1 heading (Markdown `# `, AsciiDoc
/// `= `, or a reStructuredText line underlined with `=`), after any front matter.
pub(crate) fn title(text: &str) -> Option<String> {
    let mut lines = text.lines().peekable();
    if lines.peek().is_some_and(|l| l.trim_end() == "---") {
        lines.next();
        for line in lines.by_ref() {
            if line.trim_end() == "---" {
                break;
            }
        }
    }
    let lines: Vec<&str> = lines.take(200).collect();
    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed
            .strip_prefix("# ")
            .or_else(|| trimmed.strip_prefix("= "))
        {
            return clean(rest);
        }
        if !trimmed.is_empty()
            && lines
                .get(i + 1)
                .is_some_and(|next| next.len() >= 3 && next.trim().chars().all(|c| c == '='))
        {
            return clean(trimmed);
        }
    }
    None
}

fn clean(text: &str) -> Option<String> {
    let text: String = text
        .trim()
        .trim_end_matches('#')
        .trim()
        .chars()
        .take(200)
        .collect();
    (!text.is_empty()).then_some(text)
}

/// Whether `entry` is an ADR file: `<digits>-<name>.<ext>` in a directory named
/// `adr`, `adrs` or `decisions`. Returns its number.
pub(crate) fn adr_number(path: &str) -> Option<u32> {
    let (dir, file) = path.rsplit_once('/')?;
    let parent = dir.rsplit('/').next()?;
    if !ADR_DIRS.iter().any(|d| d.eq_ignore_ascii_case(parent)) {
        return None;
    }
    let digits: String = file.chars().take_while(char::is_ascii_digit).collect();
    let rest = &file[digits.len()..];
    if digits.is_empty() || digits.len() > 6 || !(rest.starts_with('-') || rest.starts_with('_')) {
        return None;
    }
    digits.parse().ok()
}

/// The value of a `Status:`-style field: a line such as `Status: Accepted`,
/// `- **Status:** Accepted`, or a `## Status` heading followed by the value.
pub(crate) fn field(text: &str, name: &str) -> Option<String> {
    let lines: Vec<&str> = text.lines().take(400).collect();
    for (i, line) in lines.iter().enumerate() {
        let plain: String = line
            .trim()
            .trim_start_matches(['-', '*', '+', '>', ' '])
            .replace("**", "")
            .replace("__", "");
        let plain = plain.trim();
        let lower = plain.to_ascii_lowercase();
        if let Some(rest) = lower.strip_prefix(name).and_then(|r| r.strip_prefix(':')) {
            let value = plain[plain.len() - rest.len()..].trim();
            if !value.is_empty() {
                return clean_value(value);
            }
        }
        let heading = lower.trim_start_matches('#').trim();
        if lower.starts_with('#') && heading == name {
            return lines[i + 1..]
                .iter()
                .map(|l| l.trim())
                .find(|l| !l.is_empty())
                .filter(|l| !l.starts_with('#'))
                .and_then(clean_value);
        }
    }
    None
}

fn clean_value(value: &str) -> Option<String> {
    let value: String = value
        .trim_matches(|c: char| c == '*' || c == '_' || c.is_whitespace())
        .chars()
        .take(120)
        .collect();
    (!value.is_empty()).then_some(value)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn tree() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for (path, text) in [
            ("README.md", "# Project\n\nIntro.\n"),
            ("docs/guide.md", "---\ntitle: x\n---\n# The guide\n"),
            (
                "docs/adr/0001-record-decisions.md",
                "# 1. Record decisions\n\nDate: 2026-01-02\n\n## Status\n\nAccepted\n",
            ),
            (
                "docs/adr/0002-use-rust.md",
                "# ADR-0002: Use Rust\n\n- **Status:** Superseded by ADR-0003\n",
            ),
            ("docs/notes.txt", "not served\n"),
            (".env", "SECRET=1\n"),
            (".hidden/doc.md", "# hidden\n"),
            ("node_modules/pkg/README.md", "# dep\n"),
            ("src/lib.rs", "fn x() {}\n"),
        ] {
            let p = root.join(path);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, text).unwrap();
        }
        dir
    }

    #[test]
    fn walks_only_visible_documentation() {
        let dir = tree();
        let docs = Docs::open(dir.path()).unwrap();
        let (entries, truncated) = docs.walk(None);
        let paths: Vec<&str> = entries.iter().map(|e| e.path.as_str()).collect();
        assert_eq!(
            paths,
            [
                "README.md",
                "docs/adr/0001-record-decisions.md",
                "docs/adr/0002-use-rust.md",
                "docs/guide.md"
            ]
        );
        assert!(!truncated);
        let (under, _) = docs.walk(Some("docs/adr"));
        assert_eq!(under.len(), 2);
        assert_eq!(docs.walk(Some("missing")).0, Vec::new());
    }

    #[test]
    fn reads_are_confined() {
        let dir = tree();
        let docs = Docs::open(dir.path()).unwrap();
        let ok = DocPath::parse("docs/guide.md").unwrap();
        assert!(docs.read(&ok).unwrap().contains("The guide"));
        for bad in [
            "../x.md",
            "/etc/x.md",
            ".env",
            "docs/../README.md",
            ".hidden/doc.md",
            "src/lib.rs",
            "a\\b.md",
            "docs//guide.md",
            "docs/notes.txt",
        ] {
            assert!(DocPath::parse(bad).is_err(), "{bad}");
        }
        let missing = DocPath::parse("docs/missing.md").unwrap();
        assert_eq!(
            docs.read(&missing),
            Err(DocError::NotFound("docs/missing.md".into()))
        );
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_are_never_followed() {
        let dir = tree();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("secret.md"), "# outside\n").unwrap();
        std::os::unix::fs::symlink(
            outside.path().join("secret.md"),
            dir.path().join("docs/link.md"),
        )
        .unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join("docs/linkdir")).unwrap();
        let docs = Docs::open(dir.path()).unwrap();
        let paths: Vec<String> = docs.walk(None).0.into_iter().map(|e| e.path).collect();
        assert!(!paths.iter().any(|p| p.contains("link")), "{paths:?}");
        for path in ["docs/link.md", "docs/linkdir/secret.md"] {
            assert!(
                matches!(
                    docs.read(&DocPath::parse(path).unwrap()),
                    Err(DocError::NotFound(_))
                ),
                "{path}"
            );
        }
    }

    #[test]
    fn large_and_binary_documents_are_refused() {
        let dir = tree();
        fs::write(
            dir.path().join("big.md"),
            vec![b'a'; usize::try_from(MAX_DOC_BYTES).unwrap() + 1],
        )
        .unwrap();
        fs::write(dir.path().join("bin.md"), [0xff, 0xfe]).unwrap();
        let docs = Docs::open(dir.path()).unwrap();
        assert!(matches!(
            docs.read(&DocPath::parse("big.md").unwrap()),
            Err(DocError::TooLarge(_))
        ));
        assert!(matches!(
            docs.read(&DocPath::parse("bin.md").unwrap()),
            Err(DocError::NotText(_))
        ));
    }

    #[test]
    fn titles_and_adr_fields() {
        assert_eq!(
            title("---\na: b\n---\n\n# Hello #\n").as_deref(),
            Some("Hello")
        );
        assert_eq!(
            title("= AsciiDoc title\n").as_deref(),
            Some("AsciiDoc title")
        );
        assert_eq!(
            title("Restructured\n============\n").as_deref(),
            Some("Restructured")
        );
        assert_eq!(title("no heading\n"), None);
        assert_eq!(adr_number("docs/adr/0007-use-x.md"), Some(7));
        assert_eq!(adr_number("decisions/12_thing.md"), Some(12));
        assert_eq!(adr_number("docs/guide/0007-use-x.md"), None);
        assert_eq!(adr_number("docs/adr/template.md"), None);
        assert_eq!(
            field("## Status\n\nAccepted\n", "status").as_deref(),
            Some("Accepted")
        );
        assert_eq!(
            field("- **Status:** Proposed (2026)\n", "status").as_deref(),
            Some("Proposed (2026)")
        );
        assert_eq!(
            field("Date: 2026-01-02\n", "date").as_deref(),
            Some("2026-01-02")
        );
        assert_eq!(field("## Status\n\n## Next\n", "status"), None);
    }

    #[test]
    fn dependency_and_build_directories_are_never_served() {
        let dir = tree();
        fs::create_dir_all(dir.path().join("docs/target")).unwrap();
        fs::write(dir.path().join("docs/target/x.md"), "# built\n").unwrap();
        let docs = Docs::open(dir.path()).unwrap();
        for path in [
            "node_modules/pkg/README.md",
            "docs/target/x.md",
            "vendor/a.md",
        ] {
            assert!(DocPath::parse(path).is_err(), "{path}");
        }
        assert!(DocPath::prefix("node_modules").is_err());
        assert_eq!(docs.walk(Some("node_modules")).0, Vec::new());
        assert!(!docs.walk(None).0.iter().any(|e| e.path.contains("target")));
    }
}
