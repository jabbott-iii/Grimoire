//! Diagnostics reported by validation.

use std::fmt;

use serde::Serialize;

/// How serious a diagnostic is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    /// Fails validation.
    Error,
    /// Reported, but does not fail validation.
    Warning,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
        })
    }
}

/// One validation finding.
///
/// `code` is a stable identifier such as `manifest/kind-table-missing`; tests
/// and tooling should match on it rather than on `message`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Diagnostic {
    /// Error or warning.
    pub severity: Severity,
    /// Stable identifier of the rule that fired.
    pub code: &'static str,
    /// Human-readable explanation.
    pub message: String,
    /// The file, relative to the validated root, with `/` separators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    /// The field within the file, e.g. `clients.cursor.outputs[0]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

impl Diagnostic {
    /// An error.
    #[must_use]
    pub fn error(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            file: None,
            field: None,
        }
    }

    /// A warning.
    #[must_use]
    pub fn warning(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            ..Self::error(code, message)
        }
    }

    /// Attaches the field the diagnostic is about.
    #[must_use]
    pub fn at(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    /// Attaches the file the diagnostic is about, unless one is already set.
    #[must_use]
    pub fn in_file(mut self, file: impl Into<String>) -> Self {
        if self.file.is_none() {
            self.file = Some(file.into());
        }
        self
    }

    /// Whether this is an error.
    #[must_use]
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}

/// Escapes control characters so untrusted names cannot inject terminal escape
/// sequences or CI workflow commands. Newlines in messages are kept.
fn escaped(text: &str, keep_newlines: bool) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        if c.is_control() && !(keep_newlines && c == '\n') {
            out.extend(c.escape_default());
        } else {
            out.push(c);
        }
    }
    out
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.severity, self.code)?;
        match (&self.file, &self.field) {
            (Some(file), Some(field)) => {
                write!(f, " {} ({})", escaped(file, false), escaped(field, false))?;
            }
            (Some(file), None) => write!(f, " {}", escaped(file, false))?,
            (None, Some(field)) => write!(f, " ({})", escaped(field, false))?,
            (None, None) => {}
        }
        write!(f, ": {}", escaped(&self.message, true))
    }
}

/// Whether any diagnostic is an error.
#[must_use]
pub fn has_errors(diagnostics: &[Diagnostic]) -> bool {
    diagnostics.iter().any(Diagnostic::is_error)
}

/// Sorts diagnostics into a stable order: by file, then field, then code.
pub fn sort(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|a, b| {
        (&a.file, &a.field, a.code, &a.message).cmp(&(&b.file, &b.field, b.code, &b.message))
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_formats() {
        let d = Diagnostic::error("manifest/x", "bad")
            .at("clients.codex")
            .in_file("rules/a/asset.toml");
        assert_eq!(
            d.to_string(),
            "error[manifest/x] rules/a/asset.toml (clients.codex): bad"
        );
        assert_eq!(
            Diagnostic::warning("w/y", "meh").to_string(),
            "warning[w/y]: meh"
        );
        let hostile = Diagnostic::error("x/y", "bad\u{1b}[31m\nnext").in_file("a\n::error::b");
        assert_eq!(
            hostile.to_string(),
            "error[x/y] a\\n::error::b: bad\\u{1b}[31m\nnext"
        );
    }
}
