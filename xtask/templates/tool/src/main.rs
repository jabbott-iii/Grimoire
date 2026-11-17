//! `@@BINARY@@`: one line saying what this tool does.
//!
//! Conventions every Grimoire tool follows: `--help` and `--version`, `--json`
//! for machine output, data on stdout, diagnostics on stderr, and documented
//! exit statuses. It never prompts.

use std::io::Write as _;
use std::process::ExitCode;

use clap::Parser;

const EXIT_STATUS: &str = "\
Exit status: 0 success, 1 the check failed, 2 usage error";

#[derive(Debug, Parser)]
#[command(name = "@@BINARY@@", version, about = "One line saying what this tool does", after_help = EXIT_STATUS)]
struct Cli {
    /// Print one JSON document on stdout instead of text.
    #[arg(long)]
    json: bool,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let text = if cli.json {
        format!("{}\n", serde_json::json!({ "format": 1, "ok": true }))
    } else {
        "ok\n".to_owned()
    };
    // A closed pipe leaves nobody to tell.
    let _ = std::io::stdout().lock().write_all(text.as_bytes());
    ExitCode::SUCCESS
}
