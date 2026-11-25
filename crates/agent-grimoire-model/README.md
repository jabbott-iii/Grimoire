# agent-grimoire-model

Manifest, configuration and lockfile types for Grimoire: the strict Rust types for the four files that describe assets and projects, the checks that need only one of those files, and the source of their JSON Schemas. The files are written in TOML, a configuration file format. Strict means that a field the type doesn't know is an error rather than something to skip, and that each constrained value, such as an asset ID, a version or a path, is checked while the file is parsed, so an error points at its line and column.

It is part of [Grimoire](https://github.com/jabbott-iii/Grimoire): a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the command-line interface (CLI) that installs the assets a project selects. Every other crate of the CLI builds on this one, and it builds on none of them:

- the catalog crate, `agent-grimoire-catalog`, parses and checks every asset's `asset.toml` and every bundle with it;
- the resolver, `agent-grimoire-resolve`, reads a project's `grimoire.toml` and `grimoire.lock` as this crate's types;
- the client adapters, `agent-grimoire-adapters`, resolve each asset's outputs to paths for each client, and the installer, `agent-grimoire-install`, records what it writes as lockfile entries (`LockedOutput`);
- the CLI, `agent-grimoire`, reads and checks `grimoire.toml` and `grimoire.lock` with it;
- `cargo xtask schema`, one of the repository's build tasks, writes the JSON Schemas in `schemas/` from its types.

## Status

Before 1.0, and internal to the CLI. It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API changes whenever the CLI needs it to, so don't depend on it: to install assets, use the CLI ([Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README).

A change that breaks it is still marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

What it does:

- **The four files**, each read by its type's `from_toml`, which fails with a `ParseError` that carries the parser's message, including the line and column, and, when known, the byte range of the offending input:
  - `AssetManifest`, for `asset.toml`, the manifest that describes one asset: a rule, skill, agent, tool or MCP server that Grimoire can install;
  - `Bundle`, for a bundle (`catalog/bundles/<id>.toml`), a named list of assets that a project can select together;
  - `ProjectConfig`, for `grimoire.toml`, a project's selection of clients and assets;
  - `Lockfile`, for `grimoire.lock`, the lockfile, which records exactly what was installed. It never holds the CLI version, so an upgrade alone doesn't change it: it changes when an asset version changes, or when what Grimoire writes for a client changes. `Lockfile::canonicalize` sorts it, so that it is written the same way every time.

  Each file must set `format = 1`, the version of its format, and any other format number is refused.
- **Constrained values**, each checked when it is made and when it is parsed, with an `InvalidValue` error that says what was wrong:
  - `AssetId`: 1 to 64 characters of `a-z`, `0-9` and `-`, with no `-` at the start or end and no `--`. It refuses the prefixes `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-`, which Claude Code reserves for plugin names; the prefix `grimoire-` and the name `grimoire`, kept for content Grimoire itself owns in a project; and `synced`, a folder name Claude Code reserves for skills.
  - `Version`, a semantic version (SemVer) 2.0.0 such as `1.2.3`, without build metadata (the part after a `+`), and `VersionReq`, a Cargo-style requirement such as `^1.2` or `>=1, <3`.
  - `RelPath`, a relative path that is safe on Linux, macOS and Windows: ASCII only, `/` separators, no empty, `.` or `..` components, none of the characters or device names (such as `CON`) that Windows forbids, and at most 200 characters.
  - `ServerUrl`, the address of an MCP server reached over HTTP: `https://`, or `http://` only for this machine (`localhost`, `127.0.0.0/8`, `[::1]`), with no user information (`user@`), query or fragment, the parts of a URL where a credential usually goes, and at most 2048 characters.
  - `Permission`, a grant a user writes in `grimoire.toml`, such as `network` or `env:GITHUB_TOKEN`; `Capability`, a tool an agent may use, such as `read` or `mcp:<server>/<tool>`; and `LicenseExpr`, a license expression in the SPDX (Software Package Data Exchange) form, such as `MIT OR Apache-2.0`, written in canonical form.
- **`validate`**: `validate::manifest`, `validate::bundle`, `validate::project_config` and `validate::lockfile` check the rules that span several fields of one file. For example, a deprecated asset must name its replacement, and an asset can't be both selected and excluded. Each returns every problem it finds, as a list of `Diagnostic`s: errors or warnings, each with a stable code such as `manifest/kind-table-missing`. Rules that need other files or other assets are the catalog crate's.
- **`schema::documents`**: the JSON Schema of each of the four files, generated from these types with the schemars library and pinned to JSON Schema draft 2020-12, with the file name each one has under `schemas/`.

Not yet, as [`docs/roadmap.md`, Phase 1](https://github.com/jabbott-iii/Grimoire/blob/main/docs/roadmap.md#m1--a-validated-catalog-and-its-ci-gates-phase-1) records for the first three:

- an agent's `model` hint, deferred until an asset in the catalog needs one, so agents use the session's model ([`docs/subsystems/model.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#assettoml));
- permission allow-list entries for a tool, such as Claude Code's `Bash(<binary> *)`, which would let an agent run the tool's binary without asking. They arrive with the first tool that needs them ([`docs/subsystems/catalog.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/catalog.md#tool));
- `runtime = "docker"` for an MCP server in `grimoire.toml`, deferred past version 1 ([`docs/subsystems/model.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#grimoiretoml) and [ADR-0011](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0011-container-image.md), the architecture decision record (ADR) on the container image). It is still refused as an unknown field;
- any catalog source other than the one built into the binary: the lock's `source` is always `builtin` ([ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md), which embeds the catalog in the binary for version 1).

## How the CLI uses it

The CLI reads `grimoire.toml` in two steps. It parses the file, which checks every field's type and form and refuses unknown fields; a failure stops it with the diagnostic `project/config-parse` (exit code 3). Then it runs `validate::project_config`, names the file in each diagnostic it gets back, and stops if any of them is an error.

```rust
use agent_grimoire_model::{ClientId, ParseError, ProjectConfig, has_errors, validate};

const GRIMOIRE_TOML: &str = r#"
format = 1
clients = ["claude-code", "agents-md"]

[assets]
rust-errors = "^1"
"#;

fn main() -> Result<(), ParseError> {
    // Step 1: parse. Types, value forms and unknown fields are checked here.
    let config = ProjectConfig::from_toml(GRIMOIRE_TOML)?;
    assert_eq!(config.clients, [ClientId::ClaudeCode, ClientId::AgentsMd]);

    // Step 2: the rules that span several fields, reported as diagnostics.
    let problems: Vec<_> = validate::project_config(&config)
        .into_iter()
        .map(|diagnostic| diagnostic.in_file("grimoire.toml"))
        .collect();
    assert!(!has_errors(&problems));

    // A misspelled option is an unknown field: a parse error that points at it.
    let typo = GRIMOIRE_TOML.replace(
        r#"rust-errors = "^1""#,
        r#"rust-errors = { version = "^1", clinets = ["claude-code"] }"#,
    );
    let error = ProjectConfig::from_toml(&typo).unwrap_err();
    assert!(error.message().contains("unknown field `clinets`"));

    // An asset that is both selected and excluded parses, but fails validation.
    let both = format!("exclude = [\"rust-errors\"]\n{GRIMOIRE_TOML}");
    let problems = validate::project_config(&ProjectConfig::from_toml(&both)?);
    assert!(has_errors(&problems));
    assert_eq!(problems[0].code, "config/exclude");
    Ok(())
}
```

The `config/` and `lock/` codes reach CLI users and are listed in [`docs/diagnostics.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md). The `manifest/` and `bundle/` codes are for catalog authors, and catalog validation (`cargo xtask validate`) reports them.

## Documentation

- API reference: https://docs.rs/agent-grimoire-model
- Design: [`docs/subsystems/model.md`, Identifiers and versions](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#identifiers-and-versions), which makes asset IDs unique across the whole catalog and gives each asset its own SemVer version, independent of the CLI's; and [`docs/subsystems/model.md`, Manifests and configuration](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#manifests-and-configuration), which specifies `asset.toml`, `grimoire.toml` and `grimoire.lock`, each with an annotated example, as well as permission consent and catalog validation.
- Paths: [`docs/subsystems/install.md`, Filesystem safety](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#filesystem-safety), which explains why every path is ASCII, and [ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md), which confines every write to the project root.
- File formats for users: [`docs/configuration.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/configuration.md) (`grimoire.toml` and `grimoire.lock`) and [`catalog/README.md`](https://github.com/jabbott-iii/Grimoire/blob/main/catalog/README.md) (`asset.toml` and bundles).
- Versioning: [`docs/05-release.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#versioning). Grimoire versions its crates together.
- Grimoire: https://github.com/jabbott-iii/Grimoire

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
