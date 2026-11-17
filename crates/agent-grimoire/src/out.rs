//! Output: human text or one JSON document, diagnostics, prompts and exit codes.

use agent_grimoire_model::{Diagnostic, Severity};
use serde_json::{Value, json};

use crate::Context;

/// Exit status (`architecture.md` §9.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Exit {
    /// Success, or nothing to do.
    Ok = 0,
    /// Unexpected failure (I/O, internal error).
    Failure = 1,
    /// Usage error.
    Usage = 2,
    /// Invalid configuration or manifest, or a resolution failure.
    Invalid = 3,
    /// Conflict: drift or an unmanaged file in the way. Nothing was written.
    Conflict = 4,
    /// A check failed: `plan --check`, `verify` or `doctor`.
    Check = 5,
    /// Refused for safety: a path escape or a symlink at a managed path.
    Refused = 6,
}

impl Exit {
    /// The process exit code.
    #[must_use]
    pub fn code(self) -> u8 {
        self as u8
    }
}

/// Collects a command's output.
pub(crate) struct Out<'c, 'a> {
    pub(crate) json: bool,
    pub(crate) ctx: &'c mut Context<'a>,
    command: &'static str,
    result: Value,
    diagnostics: Vec<Diagnostic>,
}

impl<'c, 'a> Out<'c, 'a> {
    pub(crate) fn new(json: bool, ctx: &'c mut Context<'a>) -> Self {
        Self {
            json,
            ctx,
            command: "",
            result: Value::Null,
            diagnostics: Vec::new(),
        }
    }

    pub(crate) fn command(&mut self, name: &'static str) {
        self.command = name;
    }

    /// The structured result, for `--json`.
    pub(crate) fn result(&mut self, value: Value) {
        self.result = value;
    }

    /// Adds `key` to the structured result (making it an object if needed).
    pub(crate) fn extend_result(&mut self, key: &str, value: Value) {
        if self.result.is_null() {
            self.result = json!({});
        } else if !self.result.is_object() {
            self.result = json!({ "value": self.result.take() });
        }
        if let Some(map) = self.result.as_object_mut() {
            map.insert(key.to_owned(), value);
        }
    }

    /// A line of human output on stdout.
    pub(crate) fn line(&mut self, text: impl AsRef<str>) {
        if !self.json {
            let _ = writeln!(self.ctx.stdout, "{}", text.as_ref());
        }
    }

    /// A human status message on stderr.
    pub(crate) fn note(&mut self, text: impl AsRef<str>) {
        if !self.json {
            let _ = writeln!(self.ctx.stderr, "{}", text.as_ref());
        }
    }

    /// Reports a diagnostic (stderr for humans, the envelope for `--json`).
    pub(crate) fn diag(&mut self, d: Diagnostic) {
        if !self.json {
            let _ = writeln!(self.ctx.stderr, "{d}");
        }
        self.diagnostics.push(d);
    }

    pub(crate) fn diags(&mut self, ds: impl IntoIterator<Item = Diagnostic>) {
        for d in ds {
            self.diag(d);
        }
    }

    /// Reports an error and returns `exit`.
    pub(crate) fn fail(
        &mut self,
        exit: Exit,
        code: &'static str,
        message: impl Into<String>,
    ) -> Exit {
        self.diag(Diagnostic::error(code, message));
        exit
    }

    /// Asks a yes/no question. Without a terminal, the answer is no.
    pub(crate) fn confirm(&mut self, question: &str) -> bool {
        if !self.ctx.interactive || self.json {
            return false;
        }
        let _ = write!(self.ctx.stderr, "{question} [y/N] ");
        let _ = self.ctx.stderr.flush();
        let mut answer = String::new();
        if self.ctx.stdin.read_line(&mut answer).is_err() {
            return false;
        }
        matches!(answer.trim(), "y" | "Y" | "yes" | "YES" | "Yes")
    }

    /// Writes the JSON envelope, if any, and returns the exit status.
    pub(crate) fn finish(self, exit: Exit) -> Exit {
        if self.json {
            let envelope = json!({
                "format": 1,
                "command": self.command,
                "ok": exit == Exit::Ok,
                "exit": exit.code(),
                "result": self.result,
                "diagnostics": self.diagnostics.iter().map(diagnostic_json).collect::<Vec<_>>(),
            });
            let _ = writeln!(self.ctx.stdout, "{envelope:#}");
        }
        exit
    }
}

fn diagnostic_json(d: &Diagnostic) -> Value {
    json!({
        "severity": match d.severity { Severity::Error => "error", Severity::Warning => "warning" },
        "code": d.code,
        "message": d.message,
        "file": d.file,
        "field": d.field,
    })
}

/// Reports a command-line parse error (or prints help or the version).
pub(crate) fn usage(e: &clap::Error, json: bool, ctx: &mut Context<'_>) -> Exit {
    use clap::error::ErrorKind;
    let informational = matches!(
        e.kind(),
        ErrorKind::DisplayHelp
            | ErrorKind::DisplayVersion
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
    if informational {
        let _ = write!(ctx.stdout, "{}", e.render());
        return if e.kind() == ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand {
            Exit::Usage
        } else {
            Exit::Ok
        };
    }
    if json {
        let envelope = json!({
            "format": 1,
            "command": Value::Null,
            "ok": false,
            "exit": Exit::Usage.code(),
            "result": Value::Null,
            "diagnostics": [{
                "severity": "error",
                "code": "cli/usage",
                "message": e.render().to_string().trim(),
                "file": Value::Null,
                "field": Value::Null,
            }],
        });
        let _ = writeln!(ctx.stdout, "{envelope:#}");
    } else {
        let _ = write!(ctx.stderr, "{}", e.render());
    }
    Exit::Usage
}
