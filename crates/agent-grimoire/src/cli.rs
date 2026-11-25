//! Command-line definitions.
//!
//! The design is in [`docs/subsystems/cli.md`, CLI](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/cli.md).

use std::path::PathBuf;

use agent_grimoire_model::{ClientId, Kind, Maturity, Permission};
use clap::{Args, Parser, Subcommand, ValueEnum};

/// Install only the AI-agent assets you choose into your repository.
#[derive(Debug, Parser)]
#[command(
    name = "grimoire",
    version,
    about,
    long_about = None,
    after_help = "Exit status: 0 success, 1 unexpected failure, 2 usage error, 3 invalid configuration \
or resolution failure, 4 conflict, 5 check failed, 6 refused for safety. docs/cli.md explains each one."
)]
pub(crate) struct Cli {
    #[command(flatten)]
    pub(crate) global: Global,
    #[command(subcommand)]
    pub(crate) command: Command,
}

/// Options every command accepts.
#[derive(Debug, Args)]
pub(crate) struct Global {
    /// Print exactly one JSON document on stdout instead of human output.
    #[arg(long, global = true)]
    pub(crate) json: bool,
    /// Never prompt. Implied by --json, when stdin is not a terminal, and when the CI
    /// environment variable is set to a non-empty value.
    #[arg(long, global = true)]
    pub(crate) no_input: bool,
    /// Run as if started in DIR (the project root).
    #[arg(short = 'C', value_name = "DIR", global = true)]
    pub(crate) directory: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Create grimoire.toml, detecting the clients already in use.
    Init {
        /// A client to write files for (repeatable). Detected from the project when omitted.
        #[arg(long = "client", value_name = "CLIENT")]
        clients: Vec<ClientArg>,
    },
    /// Browse or search the catalog.
    List {
        /// Show only entries whose id, summary, description or tags contain this text.
        query: Option<String>,
        /// Only assets of this kind.
        #[arg(long, value_enum)]
        kind: Option<KindArg>,
        /// Only assets with this tag.
        #[arg(long)]
        tag: Option<String>,
        /// Only assets that support this client.
        #[arg(long, value_enum)]
        client: Option<ClientArg>,
        /// Only assets of this maturity.
        #[arg(long, value_enum)]
        maturity: Option<MaturityArg>,
    },
    /// Show an asset or bundle: permissions, outputs, dependencies and latest changes.
    Show {
        /// Asset or bundle id.
        id: String,
    },
    /// Add assets, bundles (bundle:ID) or tags (tag:TAG) to grimoire.toml.
    Add {
        /// What to add.
        #[arg(required = true, value_name = "ID|bundle:ID|tag:TAG")]
        specs: Vec<String>,
        /// Accept these permissions for the assets being added (comma-separated).
        /// `env:NAME` accepts one environment variable; a bare `env` accepts every
        /// variable those assets read now, and grimoire.toml records each one by name.
        #[arg(long, value_delimiter = ',', value_parser = parse_allow)]
        allow: Vec<AllowArg>,
        /// Also run `apply`.
        #[arg(long)]
        apply: bool,
    },
    /// Remove assets, bundles (bundle:ID) or tags (tag:TAG) from grimoire.toml.
    Remove {
        /// What to remove.
        #[arg(required = true, value_name = "ID|bundle:ID|tag:TAG")]
        specs: Vec<String>,
        /// Also run `apply`.
        #[arg(long)]
        apply: bool,
    },
    /// Show what `apply` would change, with diffs.
    Plan {
        /// Exit 5 if applying would change anything.
        #[arg(long)]
        check: bool,
    },
    /// Make the content grimoire manages on disk match grimoire.toml and grimoire.lock.
    Apply(ApplyArgs),
    /// Move assets to the versions in this grimoire's catalog, then apply.
    Update {
        /// Assets to update; all when omitted.
        ids: Vec<String>,
        /// Update grimoire.lock only; leave files alone.
        #[arg(long)]
        no_apply: bool,
        #[command(flatten)]
        apply: ApplyArgs,
    },
    /// Check every file, block and entry recorded in grimoire.lock against the disk.
    Verify,
    /// Remove all content grimoire manages, then grimoire.lock.
    Uninstall {
        /// Also remove grimoire.toml, if there is one.
        #[arg(long)]
        purge: bool,
        /// Remove managed content even where it was edited.
        #[arg(long)]
        force: bool,
    },
    /// Report on the project and environment.
    Doctor,
}

#[derive(Debug, Args, Clone, Copy)]
pub(crate) struct ApplyArgs {
    /// Overwrite managed content that was edited, and files grimoire doesn't manage that are
    /// in the way.
    #[arg(long, conflicts_with = "write_new")]
    pub(crate) force: bool,
    /// Leave conflicting files alone and write what grimoire would write to <path>.new.
    #[arg(long)]
    pub(crate) write_new: bool,
}

/// One `--allow` value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AllowArg {
    /// A permission, including `env:NAME` for one variable.
    Grant(Permission),
    /// A bare `env`: every environment variable the named assets read now.
    /// It is written to grimoire.toml as one `env:NAME` grant per variable.
    AllEnv,
}

fn parse_allow(value: &str) -> Result<AllowArg, String> {
    if value == "env" {
        return Ok(AllowArg::AllEnv);
    }
    value
        .parse()
        .map(AllowArg::Grant)
        .map_err(|e: agent_grimoire_model::InvalidValue| e.reason().to_owned())
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum ClientArg {
    ClaudeCode,
    AgentsMd,
    Codex,
    Cursor,
    GeminiCli,
    Copilot,
}

impl From<ClientArg> for ClientId {
    fn from(c: ClientArg) -> Self {
        match c {
            ClientArg::ClaudeCode => ClientId::ClaudeCode,
            ClientArg::AgentsMd => ClientId::AgentsMd,
            ClientArg::Codex => ClientId::Codex,
            ClientArg::Cursor => ClientId::Cursor,
            ClientArg::GeminiCli => ClientId::GeminiCli,
            ClientArg::Copilot => ClientId::Copilot,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum KindArg {
    Rule,
    Skill,
    Agent,
    Tool,
    McpServer,
}

impl From<KindArg> for Kind {
    fn from(k: KindArg) -> Self {
        match k {
            KindArg::Rule => Kind::Rule,
            KindArg::Skill => Kind::Skill,
            KindArg::Agent => Kind::Agent,
            KindArg::Tool => Kind::Tool,
            KindArg::McpServer => Kind::McpServer,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub(crate) enum MaturityArg {
    Experimental,
    Stable,
    Deprecated,
}

impl From<MaturityArg> for Maturity {
    fn from(m: MaturityArg) -> Self {
        match m {
            MaturityArg::Experimental => Maturity::Experimental,
            MaturityArg::Stable => Maturity::Stable,
            MaturityArg::Deprecated => Maturity::Deprecated,
        }
    }
}
