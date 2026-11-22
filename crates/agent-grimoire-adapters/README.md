# agent-grimoire-adapters

Client adapters for Grimoire. A client is an AI coding tool that Grimoire installs assets for, and its adapter is the code that turns an asset into that client's own files, at the paths that client reads. This crate has one adapter per supported client (`claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli` and `copilot`), the planner that places shared content once across the selected clients, and the files of the Claude Code plugins in Grimoire's generated marketplace.

It is part of [Grimoire](https://github.com/jabbott-iii/Grimoire): a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the command-line interface (CLI) that installs the assets a project selects. An asset is one entry of that catalog. The CLI, `agent-grimoire`, calls this crate between two others: `agent-grimoire-resolve` works out which assets a project installs and for which clients, this crate turns them into the outputs that should be on disk, and `agent-grimoire-install` plans and makes the writes. It builds on `agent-grimoire-model` for the shared types and `agent-grimoire-catalog` for the assets' files. Grimoire's build tool, `cargo xtask`, uses it too: to render every catalog asset during validation, and to generate the plugin marketplace.

## Status

Before 1.0, and internal to the CLI. It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API changes whenever the CLI needs it to, so don't depend on it: to install assets, use the CLI ([Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README).

A change that breaks it is still marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

What it does:

- **`render`** takes the resolved assets (a `Resolution`) and returns a `Rendered`: `desired`, every output that should be on disk, sorted by path and then owner, and `warnings`. Each output names its owner (an asset, or Grimoire itself), the clients it serves, its path relative to the project root, and its content: a whole file, a managed block (a marked section that Grimoire owns inside a shared Markdown file), or an entry in a JSON or TOML file. If any asset can't be rendered, `render` returns every diagnostic instead, errors and warnings together, so that all the problems are reported at once.
- **Six adapters**, one module each: `claude_code`, `agents_md`, `codex`, `cursor`, `gemini_cli` and `copilot`. An adapter is pure: it reads no files and writes none, and the same input always gives the same output. It maps the destinations an asset declares, called anchors (such as `rules:`, `skills:`, `instructions` or `mcp`), to its client's literal paths, and refuses an output its client can't express (`adapter/unsupported`). Each module's documentation has a table of what it writes where.
- **Placement**, the planner. After the adapters run, one pass finds content that several selected clients can read from one place and writes it there once, for all of them. For example, when `agents-md` or `codex` puts a rule in `AGENTS.md`, the other selected clients that read that file get no copy of their own: `cursor` and `copilot` read `AGENTS.md` directly, and `claude-code` and `gemini-cli` read it through a one-line `@AGENTS.md` import, in a block that Grimoire owns in `CLAUDE.md` or `GEMINI.md`. Cursor's and Copilot's copy of a skill goes to `.agents/skills/` when another selected client needs that folder, else to `.claude/skills/` when `claude-code` writes it, else to `.agents/skills/`; both clients read both folders. And `copilot` shares the `claude-code` entry in `.mcp.json`. Outputs of one asset that land in the same place (the same file, or the same entry in it) are merged; if they would hold different content, that is an error (`adapter/output-clash`).
- **Warnings** for what placement can't avoid: a selected client that may still read a rule or skill twice (`plan/duplicate-rule`, `plan/duplicate-skill`), and content that another client reads too, even one the asset isn't installed for (`plan/cross-client-rule`, `plan/cross-client-agent`, `plan/cross-client-hooks`, `plan/cross-client-mcp`).
- **`AVAILABLE`**, the clients this build can render for: all six. The CLI passes it to the resolver and checks the clients given to `grimoire init` against it.
- **Lock checks.** `managed_unit` says what Grimoire may own at a path: a whole file in one of nine managed folders (such as `.claude/skills/` or `.cursor/rules/`), a block in `AGENTS.md`, `CLAUDE.md` or `GEMINI.md`, or an entry in one of five configuration files (`.mcp.json`, `.claude/settings.json`, `.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`). The CLI refuses a `grimoire.lock` that records anything else (`project/lock-path`): Grimoire never records an output anywhere else, so such a lock was edited by hand, and following it could make Grimoire read or delete a user's file. `is_project_location` names the two files where Grimoire owns a block for itself, `CLAUDE.md` and `GEMINI.md`.
- **`destination`**, the text `grimoire show` prints for where a declared output lands for one client. Placement can still move a rule or skill to a location another client also reads, and `grimoire plan` shows the final paths.
- **`plugin`**, the files of the Claude Code plugins that `cargo xtask gen-marketplace` generates, one per eligible asset. `plugin::ineligible` says why an asset can't be a plugin: it is a rule, a tool, or has no `[clients.claude-code]` table. `plugin::files` gives an eligible skill's, agent's or MCP server's plugin files, which are the files the `claude-code` adapter installs, except that an agent names MCP tools the way Claude Code names a plugin's (`mcp__plugin_<server>_<server>__<tool>`). `plugin::reserved_name` applies Claude Code's rules for reserved plugin names.

Not yet:

- Hooks for `codex`, `cursor`, `gemini-cli` and `copilot`. Those adapters have no `hooks` anchor, because none of those clients can guarantee that a failed hook blocks the action, and none could be tested here. Hooks for them come back once they can be tested against the real clients ([ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)).
- Tool assets as Claude Code plugins, until a plugin hook is known to fail closed (to block the action when the hook itself fails), because Grimoire never installs the tool's binary ([`docs/architecture.md` §10](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#10-generated-artifacts)).

## How the CLI uses it

Every command that plans writes, such as `grimoire plan`, `apply` and `update`, resolves the project's `grimoire.toml` and lock, calls `render`, and hands `Rendered::desired` to `agent-grimoire-install` to plan against the disk. Here the catalog built into the binary is resolved for a project that selects one rule for `claude-code` and `agents-md`, and the rule is written once, to `AGENTS.md`:

```rust
use std::collections::BTreeSet;

use agent_grimoire_adapters::{AVAILABLE, render};
use agent_grimoire_catalog::Source;
use agent_grimoire_install::plan::{Content, Owner};
use agent_grimoire_model::{AssetId, ClientId, ProjectConfig, Version};
use agent_grimoire_resolve::{Request, Update, resolve};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The catalog embedded in the CLI, and a project's grimoire.toml.
    let source = Source::builtin()?;
    let config = ProjectConfig::from_toml(
        r#"
        format = 1
        clients = ["claude-code", "agents-md"]

        [assets]
        conventional-commits = "*"
        "#,
    )?;

    // Resolve first, as the CLI does: which assets, versions and clients.
    let version = Version::parse(env!("CARGO_PKG_VERSION"))?;
    let resolution = resolve(
        &source,
        &Request {
            config: &config,
            lock: None,
            cli_version: &version,
            update: &Update::None,
            available_clients: &AVAILABLE,
        },
    )
    .map_err(|diagnostics| format!("{diagnostics:?}"))?;

    let rendered = render(&resolution).map_err(|diagnostics| format!("{diagnostics:?}"))?;
    let at = |path: &str| rendered.desired.iter().find(|d| d.path.as_str() == path);

    // `agents-md` puts the rule in AGENTS.md, and that one block serves
    // `claude-code` too, so there is no copy in .claude/rules/.
    let rule = at("AGENTS.md").expect("the rule's block");
    assert_eq!(rule.owner, Owner::Asset(AssetId::new("conventional-commits")?));
    assert_eq!(
        rule.clients,
        BTreeSet::from([ClientId::ClaudeCode, ClientId::AgentsMd])
    );
    assert!(at(".claude/rules/conventional-commits.md").is_none());

    // Claude Code reads AGENTS.md through Grimoire's own block in CLAUDE.md.
    let import = at("CLAUDE.md").expect("the import block");
    assert_eq!(import.owner, Owner::Project);
    assert_eq!(import.content, Content::Block("@AGENTS.md\n".to_owned()));
    Ok(())
}
```

## Documentation

- API reference: https://docs.rs/agent-grimoire-adapters
- Design: [`docs/architecture.md` §8, Client adapters](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#8-client-adapters):
  - [§8.1, Adapter contract](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#81-adapter-contract): an adapter is pure, and what each one declares;
  - [§8.2, Client read matrix and cross-client planning](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#82-client-read-matrix-and-cross-client-planning): which client reads which location, the placement rules, and the warnings `plan` reports;
  - [§8.3, Per-client mapping](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#83-per-client-mapping-client-order-per-adr-0005-amended-by-adr-0008): what each adapter writes, for each asset kind.
- [ADR-0005](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0005-client-priority.md), an architecture decision record (ADR), chose the six clients and the order their adapters were built in. [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md) amended it after the formats were checked again: hooks stay Claude Code only, Codex gets no agents, and Cursor and Copilot read shared rules from `AGENTS.md`.
- The plugin marketplace: [`docs/architecture.md` §10, Generated artifacts](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#10-generated-artifacts).
- Cross-client leaks, where content installed for one client reaches another: [`docs/security-model.md` §3a](https://github.com/jabbott-iii/Grimoire/blob/main/docs/security-model.md#3a-cross-client-leaks).
- Versioning: [`docs/versioning.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md). Grimoire versions its crates together.
- Grimoire: https://github.com/jabbott-iii/Grimoire

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
