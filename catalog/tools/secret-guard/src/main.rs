//! `grimoire-tool-secret-guard`: finds likely secrets and blocks them.
//!
//! - `scan [--json] [PATH]...` scans files, directories (recursively, without
//!   following symbolic links or entering `.git`) and standard input (`-`, or no
//!   path at all).
//! - `hook` checks one Claude Code hook event on standard input and blocks
//!   tool calls and prompts that contain a secret.
//! - `rules [--json]` lists the detection rules.
//!
//! Reports never contain a matched value, only its rule, location and a short
//! SHA-256 fingerprint.

mod detect;
mod hook;
mod scan;

use std::fmt::Write as _;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use serde_json::json;

const EXIT_STATUS: &str = "\
Exit status:
  scan:  0 nothing found, 1 secrets found, 2 usage error, 3 an input could not be read
  hook:  0 allow, 2 block (a secret was found, or the event could not be checked)
  rules: 0";

#[derive(Debug, Parser)]
#[command(
    name = "grimoire-tool-secret-guard",
    version,
    about = "Find likely secrets in files, standard input and Claude Code hook events",
    after_help = EXIT_STATUS
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Scan files, directories or standard input.
    Scan {
        /// Print one JSON document on stdout instead of text.
        #[arg(long)]
        json: bool,
        /// Files or directories; `-` or none reads standard input.
        paths: Vec<PathBuf>,
    },
    /// Check a Claude Code `PreToolUse` or `UserPromptSubmit` event on standard input.
    Hook,
    /// List the detection rules.
    Rules {
        /// Print JSON instead of text.
        #[arg(long)]
        json: bool,
    },
}

/// Writes to a stream, ignoring a closed pipe: there is nobody left to tell.
fn emit(mut out: impl std::io::Write, text: &str) {
    let _ = out.write_all(text.as_bytes());
    let _ = out.flush();
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Scan { json, paths } => scan_command(&paths, json),
        Command::Hook => hook_command(),
        Command::Rules { json } => rules_command(json),
    }
}

fn scan_command(paths: &[PathBuf], json: bool) -> ExitCode {
    let report = scan::run(paths);
    if json {
        let doc = json!({
            "format": 1,
            "findings": report.findings.iter().map(|(path, f)| json!({
                "path": path,
                "line": f.line,
                "column": f.column,
                "rule": f.rule,
                "description": f.description,
                "fingerprint": f.fingerprint,
            })).collect::<Vec<_>>(),
            "skipped": report.skipped.iter().map(|(path, why)| json!({"path": path, "reason": why})).collect::<Vec<_>>(),
            "errors": report.errors.iter().map(|(path, why)| json!({"path": path, "message": why})).collect::<Vec<_>>(),
        });
        emit(std::io::stdout().lock(), &format!("{doc}\n"));
    } else {
        let mut text = String::new();
        for (path, f) in &report.findings {
            let _ = writeln!(
                text,
                "{path}:{}:{}: {}: {} [{}]",
                f.line, f.column, f.rule, f.description, f.fingerprint
            );
        }
        emit(std::io::stdout().lock(), &text);
        let mut notes = String::new();
        for (path, why) in &report.errors {
            let _ = writeln!(notes, "error: {path}: {why}");
        }
        let _ = writeln!(
            notes,
            "secret-guard: {} likely secret(s), {} skipped, {} error(s)",
            report.findings.len(),
            report.skipped.len(),
            report.errors.len()
        );
        emit(std::io::stderr().lock(), &notes);
    }
    if !report.errors.is_empty() {
        ExitCode::from(3)
    } else if !report.findings.is_empty() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn hook_command() -> ExitCode {
    let outcome = match scan::read_stdin(hook::MAX_EVENT_BYTES) {
        Ok(event) => hook::decide(&event),
        // Reading failed or the event is too large: refuse, so the guard fails closed.
        Err(e) => hook::decide_unreadable(&e),
    };
    match outcome {
        hook::Outcome::Allow => ExitCode::SUCCESS,
        hook::Outcome::Block { stderr, stdout } => {
            if let Some(stdout) = stdout {
                emit(std::io::stdout().lock(), &format!("{stdout}\n"));
            }
            emit(std::io::stderr().lock(), &format!("{stderr}\n"));
            ExitCode::from(2)
        }
    }
}

fn rules_command(json: bool) -> ExitCode {
    let text = if json {
        let rules: Vec<_> = detect::RULES
            .iter()
            .map(|r| json!({"id": r.id, "description": r.description}))
            .collect();
        format!("{}\n", json!({"format": 1, "rules": rules}))
    } else {
        detect::RULES.iter().fold(String::new(), |mut out, r| {
            let _ = writeln!(out, "{}: {}", r.id, r.description);
            out
        })
    };
    emit(std::io::stdout().lock(), &text);
    ExitCode::SUCCESS
}
