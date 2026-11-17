//! Normalized, portable relative paths.

use crate::constrained::constrained_string;

/// Maximum length of a relative path, in characters.
pub const MAX_REL_PATH: usize = 200;

/// Device names Windows reserves in every directory, with or without an extension.
const WINDOWS_RESERVED: [&str; 22] = [
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

fn check_rel_path(value: &str) -> Result<(), String> {
    if value.is_empty() {
        return Err("must not be empty".into());
    }
    let len = value.chars().count();
    if len > MAX_REL_PATH {
        return Err(format!(
            "must be at most {MAX_REL_PATH} characters (got {len})"
        ));
    }
    if value.starts_with('/') {
        return Err("must be relative (no leading '/')".into());
    }
    if value.contains('\\') {
        return Err("must use '/' as the separator, not '\\'".into());
    }
    for component in value.split('/') {
        check_component(component)?;
    }
    Ok(())
}

fn check_component(component: &str) -> Result<(), String> {
    if component.is_empty() {
        return Err("must not contain empty components ('//' or a trailing '/')".into());
    }
    if component == "." || component == ".." {
        return Err("must not contain '.' or '..' components".into());
    }
    // ASCII only: rules out invisible and bidi characters, superscript device
    // names such as `COM¹`, and names that macOS or Windows normalize or fold
    // into each other.
    if let Some(c) = component.chars().find(|c| !c.is_ascii()) {
        return Err(format!("must use only ASCII characters (found {c:?})"));
    }
    if let Some(c) = component
        .chars()
        .find(|c| c.is_ascii_control() || matches!(c, '<' | '>' | ':' | '"' | '|' | '?' | '*'))
    {
        return Err(format!("must not contain {c:?}"));
    }
    if component.ends_with('.') || component.ends_with(' ') {
        return Err("components must not end with '.' or ' ' (Windows strips them)".into());
    }
    let stem = component.split('.').next().unwrap_or(component).trim_end();
    if WINDOWS_RESERVED
        .iter()
        .any(|r| r.eq_ignore_ascii_case(stem))
    {
        return Err(format!("must not use the Windows reserved name {stem:?}"));
    }
    Ok(())
}

constrained_string! {
    /// A relative path that is safe on Linux, macOS and Windows.
    ///
    /// ASCII only. Uses `/` separators, has no empty, `.` or `..` components, no absolute or
    /// drive prefix, no characters Windows forbids, no Windows reserved device
    /// names, and is at most [`MAX_REL_PATH`] characters long. Escapes through
    /// symlinks are a separate, runtime concern of the installer.
    pub struct RelPath {
        what: "relative path",
        check: check_rel_path,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 200,
            "description": "ASCII relative path with '/' separators. No leading '/', no '.', '..' or empty components, no '\\\\', ':', '<', '>', '\"', '|', '?', '*', no Windows reserved names, no component ending in '.' or ' '."
        },
    }
}

impl RelPath {
    /// The path's components, in order.
    pub fn components(&self) -> impl Iterator<Item = &str> {
        self.0.split('/')
    }

    /// The last component.
    #[must_use]
    pub fn file_name(&self) -> &str {
        self.0.rsplit('/').next().unwrap_or(&self.0)
    }

    /// The last component without its final extension.
    #[must_use]
    pub fn file_stem(&self) -> &str {
        let name = self.file_name();
        match name.rsplit_once('.') {
            Some((stem, _)) if !stem.is_empty() => stem,
            _ => name,
        }
    }

    /// Whether `prefix` equals this path or is one of its leading directories.
    #[must_use]
    pub fn starts_with(&self, prefix: &RelPath) -> bool {
        self.0 == prefix.0
            || self
                .0
                .strip_prefix(prefix.as_str())
                .is_some_and(|rest| rest.starts_with('/'))
    }

    /// Joins two relative paths.
    ///
    /// # Errors
    ///
    /// Fails if the joined path exceeds [`MAX_REL_PATH`].
    pub fn join(&self, other: &RelPath) -> Result<RelPath, crate::InvalidValue> {
        RelPath::new(format!("{}/{}", self.0, other.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_ordinary_paths() {
        for ok in [
            "SKILL.md",
            "references/forms.md",
            ".claude/rules/x.md",
            "a b/c.txt",
            "x.tar.gz",
        ] {
            assert!(RelPath::new(ok).is_ok(), "{ok}");
        }
    }

    #[test]
    fn rejects_unsafe_paths() {
        for bad in [
            "",
            "/etc/passwd",
            "../x",
            "a/../b",
            "a/./b",
            "./a",
            "a//b",
            "a/",
            "a\\b",
            "C:/x",
            "C:x",
            "a:stream",
            "con",
            "CON.txt",
            "dir/nul.md",
            "aux",
            "lpt1.x",
            "trailing.",
            "trailing ",
            "bad|pipe",
            "q?",
            "star*",
            "nul\0byte",
            "line\nbreak",
            "COM\u{b9}.md",
            "LPT\u{b2}",
            "a\u{202e}txt.exe",
            "\u{feff}bom.md",
            "x\u{200b}.md",
            "\u{fc}ber.md",
        ] {
            assert!(RelPath::new(bad).is_err(), "{bad:?} should be rejected");
        }
        assert!(RelPath::new("a/".repeat(100) + "b").is_err());
    }

    #[test]
    fn helpers() {
        let p = RelPath::new("references/forms.md").unwrap();
        assert_eq!(p.file_name(), "forms.md");
        assert_eq!(p.file_stem(), "forms");
        assert_eq!(
            p.components().collect::<Vec<_>>(),
            ["references", "forms.md"]
        );
        let dir = RelPath::new("references").unwrap();
        assert!(p.starts_with(&dir));
        assert!(!RelPath::new("referencesx/y").unwrap().starts_with(&dir));
        assert_eq!(
            dir.join(&RelPath::new("a.md").unwrap()).unwrap().as_str(),
            "references/a.md"
        );
        assert_eq!(RelPath::new(".hidden").unwrap().file_stem(), ".hidden");
    }
}
