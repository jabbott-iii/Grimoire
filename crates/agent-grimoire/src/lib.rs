//! The `grimoire` command-line interface, as a library so tests can drive it
//! with a fixture catalog and captured streams. Not a stable API: use the binary.
//!
//! [`run`] parses arguments, runs one command and returns its exit status
//! (`architecture.md` §9.3). Human output goes to stdout, diagnostics to stderr;
//! with `--json`, stdout carries exactly one JSON document and stderr stays quiet.

mod cli;
mod commands;
mod out;
mod pipeline;
mod project;

use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::PathBuf;

use agent_grimoire_catalog::Source;
use agent_grimoire_model::Version;
use clap::Parser as _;

pub use out::Exit;

/// The streams and environment a command runs with.
pub struct Context<'a> {
    /// The catalog to install from.
    pub source: &'a Source,
    /// The running CLI's version, checked against `min-cli-version`.
    pub version: Version,
    /// The directory the command starts in.
    pub cwd: PathBuf,
    /// Answers to prompts.
    pub stdin: &'a mut dyn BufRead,
    /// Data output.
    pub stdout: &'a mut dyn Write,
    /// Diagnostics and prompts.
    pub stderr: &'a mut dyn Write,
    /// Whether prompts are possible: stdin is a terminal and `CI` is not set.
    pub interactive: bool,
    /// The `PATH` value, for `doctor`.
    pub path: Option<OsString>,
}

/// Runs `grimoire` with `args` (including the program name).
pub fn run<I, T>(args: I, ctx: &mut Context<'_>) -> Exit
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
    let json = args.iter().skip(1).any(|a| a == "--json");
    let cli = match cli::Cli::try_parse_from(&args) {
        Ok(cli) => cli,
        Err(e) => return out::usage(&e, json, ctx),
    };
    let mut out = out::Out::new(cli.global.json, ctx);
    let code = commands::run(cli, &mut out);
    out.finish(code)
}
