# agent-grimoire-resolve

Selection and dependency resolution for Grimoire. A project says in `grimoire.toml` which assets it wants (entries of Grimoire's catalog, such as rules and skills) and which clients to install them for (AI coding tools, such as Claude Code and Codex). This crate turns that selection, `grimoire.lock` (the record of what the project last installed) and the catalog into the exact set to install: which version of each asset, why it is selected, which clients it is installed for and the permissions it needs, all of which the project has accepted. It reports every problem it finds at once, each with its own diagnostic code.

It is part of [Grimoire](https://github.com/jabbott-iii/Grimoire): a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the command-line interface (CLI) that installs the assets a project selects. The CLI, in the `agent-grimoire` crate, resolves before `plan`, `apply`, `update` and `doctor` plan any change, and before `add` and `remove` write `grimoire.toml`. It hands the result to `agent-grimoire-adapters`, which writes each resolved asset's files for its clients. This crate builds on `agent-grimoire-model`, which parses `grimoire.toml` and `grimoire.lock`, and on `agent-grimoire-catalog`, whose `Source` holds the catalog built into the CLI.

## Status

Before 1.0, and internal to the CLI. It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API changes whenever the CLI needs it to, so don't depend on it: to install assets, use the CLI ([Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README).

A change that breaks it is still marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

What it does:

- `resolve`, the entry point, takes a `Request` and a catalog `Source`. It returns a `Resolution`, or, when it finds any error, every diagnostic it found, warnings included. A diagnostic is an error or a warning with a stable code, such as `resolve/permission`. A code keeps its meaning from release to release, while its message text may change, so tests and scripts match on the code.
- `Request` says what to resolve: the parsed `grimoire.toml` (already checked on its own), the project's `grimoire.lock` if it has one, the running CLI's version, an `Update`, and the clients this build has adapters for.
- `Update` says which assets may move away from the versions the lock pins: none (`Update::None`), all (`Update::All`) or only the ones listed (`Update::Only`).
- `Resolution` holds the assets to install, by id, and the warnings. Each asset is a `Resolved`, which holds:
  - the asset as the catalog has it: its manifest (`asset.toml`), the manifest's digest and its files;
  - `selected_by`, every reason it is selected: listed under `[assets]`, a bundle, a tag, or another asset's `requires`;
  - `allow`, the permissions it needs, all of which the project accepted;
  - `clients`, the clients it is installed for;
  - `options`, its table under `[assets]`, if it has one.
- `missing_permissions` gives the permissions each selected asset needs that `grimoire.toml` doesn't accept yet, so that the CLI can ask for them before it resolves.
- `selected_ids` gives the id of every asset the configuration selects from the catalog, including ones that would fail to resolve.

The checks `resolve` makes:

- **Selection.** An asset is selected when it is listed under `[assets]`, is in a bundle listed in `bundles` (a bundle is a named list of assets), carries a tag listed in `tags`, or is required by a selected asset (its manifest's `requires`, followed to every depth).
  - `exclude` drops an asset that a bundle or a tag would select. Excluding an asset that a selected asset requires is an error.
  - A tag never selects a deprecated asset, nor one that supports none of the project's clients. A warning names each asset that a tag skips for having no client.
  - An unknown asset or bundle is an error, and a tag that no asset carries is a warning.
- **Versions.** Each Grimoire release builds in one version of each asset, so resolution checks that version rather than choosing one.
  - Every version requirement, whether from `[assets]`, a bundle or a `requires`, must match it. A tag takes whatever version the catalog has, pre-releases (such as `1.0.0-rc.1`) included.
  - A version that `grimoire.lock` pins must be the catalog's version, with the same manifest, unless the `Update` lets that asset move.
  - The running CLI must be at least the asset's `min-cli-version`.
- **Conflicts.** Two selected assets conflict when one's manifest lists the other under `conflicts`, with a version requirement that the other's version matches. That is an error, reported once per pair, naming what selected each one.
- **Clients.** An asset is installed for every project client it supports. If its `[assets]` entry lists `clients`, it is installed for exactly those instead, and each must be one it supports.
  - Without `clients`, a project client that the asset doesn't support is skipped with a warning.
  - So is a client with no place at all for the asset's kind: Grimoire writes hooks only for Claude Code, and Codex gets no agents. `agents-md`, the portable `AGENTS.md` target, never gets this warning, because by design it holds no agents, MCP servers or hooks.
  - An asset left with no client is an error.
- **Permission consent.** Each permission an asset declares (`filesystem-read`, `filesystem-write`, `network`, `shell`, `hooks`, or `env:<NAME>` for one environment variable) must be accepted in `grimoire.toml`, or resolution fails. An asset listed under `[assets]` takes its grants in that entry's `allow`, and any other asset in the top-level `[allow]` table. A `filesystem-write` grant covers `filesystem-read`.
- **Options.** `transport`, `url` and `env` in an `[assets]` entry are only for MCP servers. On any other kind they are an error.

## How the CLI uses it

`grimoire add` first calls `missing_permissions` to find what the new selection needs. It asks the user to accept those permissions, or takes them from `--allow`, and adds them to the new `grimoire.toml` it is preparing. It then resolves that, without the lock, and writes the file only if it resolves. `plan`, `apply` and `update` resolve with the project's lock before they plan anything. On a resolution error, these commands print every diagnostic and exit with code 3. This example takes a new project through the same steps:

```rust
use agent_grimoire_catalog::Source;
use agent_grimoire_model::{AssetId, ClientId, Permission, ProjectConfig, Selector, Version, validate};
use agent_grimoire_resolve::{Request, Update, missing_permissions, resolve};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The catalog built into this build of Grimoire.
    let source = Source::builtin()?;
    let version = Version::parse(env!("CARGO_PKG_VERSION"))?;

    // A new project's grimoire.toml: the `security-baseline` bundle, for Claude Code.
    let config = ProjectConfig::from_toml(
        r#"
        format = 1
        clients = ["claude-code"]
        bundles = ["security-baseline"]
        "#,
    )?;
    // The CLI checks the file on its own before it resolves.
    assert!(validate::project_config(&config).is_empty());

    let request = Request {
        config: &config,
        lock: None, // no grimoire.lock yet
        cli_version: &version,
        update: &Update::None,
        // The clients the CLI has adapters for: all six in this build.
        available_clients: &ClientId::ALL,
    };

    // The bundle's assets need permissions that the project hasn't accepted.
    let diagnostics = resolve(&source, &request).unwrap_err();
    assert!(diagnostics.iter().any(|d| d.code == "resolve/permission"));

    // `grimoire add` shows these and asks; the user accepts them all, and the
    // CLI writes them to grimoire.toml's `[allow]` table.
    let mut accepted = config.clone();
    for (id, needed) in missing_permissions(&source, &config) {
        accepted.allow.insert(id, needed);
    }

    let resolution = resolve(&source, &Request { config: &accepted, ..request })
        .expect("the selection resolves once its permissions are accepted");

    // `secret-guard` registers hooks, so it needs the `hooks` permission.
    let guard = &resolution.assets[&AssetId::new("secret-guard")?];
    let bundle = Selector::Bundle(AssetId::new("security-baseline")?);
    assert!(guard.selected_by.contains(&bundle));
    assert!(guard.clients.contains(&ClientId::ClaudeCode));
    assert!(guard.allow.contains(&Permission::Hooks));
    Ok(())
}
```

## Documentation

- API reference: https://docs.rs/agent-grimoire-resolve
- Design: [`docs/architecture.md` §7.5, Resolution errors](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#75-resolution-errors): the errors resolution reports, and why a version pinned in the lock that differs from the catalog's stops planning even when no file would change. Also:
  - [§6.2, `grimoire.toml`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#62-grimoiretoml-user-repo): the selection this crate reads, including `exclude`, the `[assets]` options and the `[allow]` table;
  - [§6.3, Permission consent](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#63-permission-consent): the grant names, and that resolution fails on any permission the project hasn't accepted;
  - [§7.1, Desired state, plan, apply](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#71-desired-state-plan-apply): resolution is the first step of every install, and the lock's pins hold unless `update` moves them;
  - the architecture decision records (ADRs) [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md), which builds the catalog into the CLI with one version of each asset, behind the `Source` type; [ADR-0005](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0005-client-priority.md), which chose the six clients and their order; and [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md), which keeps hooks to Claude Code and gives Codex no agents.
- Diagnostics: [`docs/diagnostics.md`, Resolution](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md#resolution-resolve) lists every `resolve/` code, what raises it and what to do about it.
- Versioning: [`docs/versioning.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md). Grimoire versions its crates together.
- Grimoire: https://github.com/jabbott-iii/Grimoire

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
