//! The `grimoire` command-line interface (CLI) as a library, so that tests can
//! drive it with a fixture catalog and captured streams. It is not a stable
//! API: to install assets, use the `grimoire` binary.
//!
//! A fixture catalog is a small catalog written for a test, and captured
//! streams are standard input, output and error held in memory. [`run`]
//! parses a command line, runs one command and returns its exit status, an
//! [`Exit`]. Everything else the command takes from outside comes in a
//! [`Context`]: the catalog, the directory to start in, the three streams,
//! whether it may prompt, and the `PATH` value.
//!
//! # Where it fits
//!
//! This crate is the top of Grimoire's crates. It turns a command line into
//! calls to the five library crates, and reports what they found:
//!
//! - [`agent_grimoire_model`] has the types of `grimoire.toml`,
//!   `grimoire.lock` and the asset manifests, and
//!   [`Diagnostic`](agent_grimoire_model::Diagnostic), the type that commands
//!   report their errors and warnings as.
//! - [`agent_grimoire_catalog`] has [`Source`], the catalog that commands
//!   read and install from.
//! - [`agent_grimoire_resolve`] works out which assets and versions a
//!   selection means, for which clients, and which permissions were accepted.
//!   A client is one AI coding tool that Grimoire writes files for, such as
//!   Claude Code or Cursor.
//! - [`agent_grimoire_adapters`] turns each resolved asset into the content
//!   of the files each client reads.
//! - [`agent_grimoire_install`] plans those files against the disk, writes
//!   them only inside the project root, and checks them against
//!   `grimoire.lock`.
//!
//! The design is in
//! [`docs/subsystems/cli.md`, CLI](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/cli.md):
//! the commands (§9.1), machine use (§9.2) and exit codes (§9.3).
//!
//! ## How the binary calls it
//!
//! The `grimoire` binary, `src/main.rs`, is a thin wrapper around [`run`]. It
//! loads the catalog embedded in this build with
//! [`Source::builtin`](agent_grimoire_catalog::Source::builtin), then builds
//! the [`Context`] from the real process:
//!
//! - `version` is this package's version, parsed with
//!   [`Version::parse`](agent_grimoire_model::Version::parse);
//! - `cwd` is the current directory;
//! - `stdin`, `stdout` and `stderr` are the process's locked standard streams;
//! - `interactive` is true only when standard input is a terminal and the `CI`
//!   environment variable is unset or empty;
//! - `path` is the `PATH` environment variable.
//!
//! It calls `run` with the process's arguments and exits with the returned
//! status's [`Exit::code`]. If the embedded catalog doesn't load, the version
//! doesn't parse, or the current directory can't be read, it prints an error
//! on standard error and exits 1 ([`Exit::Failure`]) without calling `run`.
//! Under `--json`, these and a panic's message (below) are the only text on
//! standard error.
//!
//! The catalog is embedded because
//! [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md),
//! the architecture decision record on how the catalog reaches users, chose
//! to pack it into each release. Installing then needs no network code and
//! works offline, and the binary's checksums and signatures cover the catalog
//! too.
//!
//! ## How the tests call it
//!
//! The end-to-end tests in `tests/cli.rs` build the [`Context`] themselves:
//! a fixture catalog from `agent-grimoire-testkit` (a crate that is never
//! published), in-memory streams, a temporary project directory as `cwd`, and
//! `interactive` set by each test. They then check the exit status, the
//! output and the files in the project. `tests/builtin.rs` does the same with
//! the embedded catalog, so that every asset in it is installed, verified and
//! uninstalled by a test, and every bundle is installed and verified.
//!
//! ## Internal, not a stable API
//!
//! The library part of this package, [`run`], [`Context`] and [`Exit`], is
//! not part of Grimoire's public API
//! ([`docs/05-release.md`, The public API](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#the-public-api)).
//! It exists so that the CLI's tests can drive the CLI. Nothing else depends
//! on it, so a change to it isn't a breaking change. Don't depend on it.
//!
//! On crates.io, the Rust package registry, this package is how the binary
//! is installed: `cargo install --locked agent-grimoire` builds and installs
//! `grimoire`. What Grimoire keeps stable is the command line itself: its
//! commands and options, exit codes, `--json` output and diagnostic codes.
//! The user reference describes them:
//!
//! - [`docs/cli.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md)
//!   for the commands, options, exit codes, `--json` output and environment;
//! - [`docs/diagnostics.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md)
//!   for every diagnostic code, the stable name of an error or warning, such
//!   as `verify/mismatch`.
//!
//! # The parts
//!
//! - **Running a command:** [`run`] takes the whole command line, program
//!   name first, as in `["grimoire", "plan", "--check"]`, and a [`Context`].
//!   It runs one command and returns its [`Exit`]. It never ends the process
//!   itself.
//! - **What a command runs with:** [`Context`], whose fields are:
//!   - `source`, the catalog to read and install from;
//!   - `version`, the running CLI's version. Resolving checks it against each
//!     selected asset's `min-cli-version`, and an asset that needs a newer CLI
//!     fails with `resolve/cli-too-old`. `--version` doesn't print this field:
//!     it prints the version this crate was built as.
//!   - `cwd`, the directory the command starts in. It is the project root,
//!     where `grimoire.toml` lives, unless `-C DIR` names another, and a
//!     relative `DIR` is taken from it. `list` and `show` read only the
//!     catalog, so they ignore it.
//!   - `stdin`, read only for answers to prompts;
//!   - `stdout`, for data output: human output, or the one JSON document;
//!   - `stderr`, for diagnostics, notes and prompts;
//!   - `interactive`, whether prompts are possible. `--no-input` and `--json`
//!     turn prompts off for the run whatever it says.
//!   - `path`, the `PATH` value. `doctor` looks in its directories for each
//!     locked tool's and Model Context Protocol (MCP) server's binary, except a
//!     server configured over Streamable HTTP, which needs none here. `plan`,
//!     `apply`, `update`, `doctor`, `add --apply` and `remove --apply` look
//!     there for the binary of each selected tool whose hooks fail closed
//!     (block the event they cover when they can't run), and warn
//!     (`plan/hook-binary-missing`) when it isn't found. With `None`, no
//!     binary is found. Grimoire never runs the binaries it finds.
//! - **How the command ended:** [`Exit`], one variant for each exit status
//!   from 0 to 6. [`Exit::code`] gives the number the process exits with.
//!
//! # Example
//!
//! `list` reads only the catalog, so it needs no project. This runs
//! `grimoire list --kind rule --json` against the embedded catalog as the
//! binary would, but with the streams in memory:
//!
//! ```
//! use std::io::Cursor;
//!
//! use agent_grimoire::{Context, Exit, run};
//! use agent_grimoire_catalog::Source;
//! use agent_grimoire_model::Version;
//!
//! let source = Source::builtin()?;
//! let mut stdin = Cursor::new(Vec::new()); // no answers: nothing prompts here
//! let mut stdout = Vec::new();
//! let mut stderr = Vec::new();
//! let mut ctx = Context {
//!     source: &source,
//!     version: Version::parse(env!("CARGO_PKG_VERSION"))?,
//!     cwd: std::env::current_dir()?,
//!     stdin: &mut stdin,
//!     stdout: &mut stdout,
//!     stderr: &mut stderr,
//!     interactive: false,
//!     path: None,
//! };
//!
//! let exit = run(["grimoire", "list", "--kind", "rule", "--json"], &mut ctx);
//! assert_eq!(exit, Exit::Ok);
//! assert_eq!(exit.code(), 0);
//!
//! // With `--json`, stdout holds exactly one JSON document, and stderr nothing.
//! let envelope: serde_json::Value = serde_json::from_slice(&stdout)?;
//! assert_eq!(envelope["command"], "list");
//! assert_eq!(envelope["ok"], true);
//! let assets = envelope["result"]["assets"].as_array().expect("a list of assets");
//! assert!(!assets.is_empty());
//! assert!(assets.iter().all(|asset| asset["kind"] == "rule"));
//! assert!(stderr.is_empty());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Output: the streams and `--json`
//!
//! Without `--json`, human output goes to `stdout`, written as the command
//! runs. Diagnostics (errors and warnings, each with its code), notes and
//! prompts go to `stderr`. Control characters in a diagnostic are printed
//! escaped, so a hostile file name can't send escape sequences to a terminal.
//!
//! With `--json`, `stdout` carries exactly one JSON document and `stderr`
//! stays quiet. The document is the envelope: an object with the keys
//! `format` (1), `command`, `ok`, `exit`, `result` and `diagnostics`, written
//! once, when the command finishes.
//!
//! - A command-line parse error produces the envelope too, when `--json` is
//!   anywhere on the command line: `command` and `result` are `null`, `exit`
//!   is 2, and the one diagnostic has the code `cli/usage`. `init` finding no
//!   client (`init/no-client`) is also a usage error, but it reports
//!   `command` as `"init"`.
//! - `--help` and `--version` print plain text even with `--json`, and return
//!   [`Exit::Ok`].
//! - [`docs/cli.md`, JSON output](https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md#json-output)
//!   gives every key and each command's `result`.
//!
//! `run` ignores errors from writing to the streams: a write that fails is
//! not reported, and it doesn't change the exit status.
//!
//! # Prompts
//!
//! Only `add` asks a question: whether to accept the permissions an asset
//! needs, such as `shell` or `network`. It asks on `stderr` and reads one line
//! from `stdin`, and only when `interactive` is true and neither `--no-input`
//! nor `--json` is given.
//!
//! - `y`, `Y`, `yes`, `Yes` or `YES`, with any spaces around it, accepts. Any
//!   other answer, or a failed read, refuses.
//! - Without a prompt the answer is no. `add` then stops, and its error names
//!   the `--allow` value that accepts the permissions.
//!
//! A test that answers a prompt sets `interactive` to true and puts its
//! answers in `stdin`.
//!
//! # Exit statuses
//!
//! | Code | Variant | Meaning |
//! |---|---|---|
//! | 0 | [`Exit::Ok`] | Success, or nothing to do |
//! | 1 | [`Exit::Failure`] | Unexpected failure: an input/output (I/O) error, or an internal error |
//! | 2 | [`Exit::Usage`] | Usage error: an unknown command or option, a missing or invalid argument, or no command, or `init` finding no client |
//! | 3 | [`Exit::Invalid`] | Invalid configuration or lock, or a resolution failure |
//! | 4 | [`Exit::Conflict`] | Conflict: managed content was edited or removed, or a file Grimoire doesn't manage is in the way |
//! | 5 | [`Exit::Check`] | A check failed: `plan --check` found changes, `verify` found mismatches, or `doctor` found problems |
//! | 6 | [`Exit::Refused`] | Refused for safety: for example a symbolic link at a managed path, or a path that would leave the project root |
//!
//! After a conflict, nothing was written, except in these cases:
//!
//! - `add --apply` and `remove --apply` have already written `grimoire.toml`;
//! - `--write-new` has written the files without conflicts, recorded them in
//!   `grimoire.lock`, and written a `<path>.new` file beside each conflicting
//!   file that would still exist. It writes nothing when a file can't be
//!   edited at all, or when a `<path>.new` already holds something else
//!   (`apply/sidecar-exists`);
//! - a run that stopped partway, because a file changed under it
//!   (`apply/changed`) or something got in the way of a write
//!   (`fs/in-the-way`), has written some files, and `grimoire.lock` records
//!   them.
//!
//! [`docs/cli.md`, Exit codes](https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md#exit-codes)
//! gives every condition and the commands that return each status, and
//! [`docs/subsystems/cli.md`, Exit codes](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/cli.md#exit-codes)
//! is their design.
//!
//! `run` doesn't catch panics. A panic, the way a Rust program stops on an
//! internal error it can't recover from, passes up out of `run` to its
//! caller. No `--json` document is written then, and the binary exits with
//! Rust's default status for a panic, 101.

mod cli;
mod commands;
#[cfg(test)]
mod docs_check;
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
    /// The `PATH` value. `doctor` checks that each installed tool's or MCP
    /// server's binary is on it. Every command that plans (`plan`, `apply`,
    /// `update`, `add --apply`, `remove --apply` and `doctor`) also warns
    /// (`plan/hook-binary-missing`) when a selected tool's hooks fail closed
    /// and its binary is not on it.
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
