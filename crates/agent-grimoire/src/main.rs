//! The `grimoire` command-line interface.
//!
//! Phase 1 provides only `--help` and `--version`; the commands in
//! `docs/architecture.md` §9 arrive in Phase 2.

use std::process::ExitCode;

use clap::Parser;

/// Install only the AI-agent assets you choose into your repository.
#[derive(Debug, Parser)]
#[command(name = "grimoire", version, about, long_about = None)]
struct Cli {}

fn main() -> ExitCode {
    // `parse` prints help or version and exits with clap's codes (0, or 2 for usage errors).
    let Cli {} = Cli::parse();
    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::Cli;

    #[test]
    fn command_definition_is_valid() {
        Cli::command().debug_assert();
    }
}
