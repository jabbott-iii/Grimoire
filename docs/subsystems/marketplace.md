# Subsystem: the Claude Code plugin marketplace

The marketplace lets Claude Code users install catalog assets as Claude Code plugins, without the
`grimoire` CLI. A plugin is a package a user adds to Claude Code from a marketplace, a catalog of
plugins. Grimoire's marketplace is generated from `catalog/` and checked in: the marketplace file
`.claude-plugin/marketplace.json` and one plugin directory per eligible asset and bundle under
`plugins/claude-code/`. It guarantees that every plugin holds exactly the files the `claude-code`
adapter would install, that no plugin carries a hook, that each plugin's README lists what
installing it grants, and that a plugin's version changes whenever its files do.
[ADR-0008](../adr/0008-phase-4-client-adapters.md) (hooks stay Claude Code only, and the Phase 4
formats) shaped it, and the formats it follows are
[`CC-PLUGIN-P4`](../specs.md#cc-plugin-p4-plugin-manifest-components-and-naming) and
[`CC-MKT-P4`](../specs.md#cc-mkt-p4-marketplace-file-sources-dependencies-and-adding-a-marketplace)
in `docs/specs.md`. How `cargo xtask gen-marketplace` writes and checks these files is in
[`xtask.md`](xtask.md#generated-artifacts).

## Why a generated marketplace

Some Claude Code users want an asset without adding a CLI and a `grimoire.toml` to their project.
A plugin gives them that. Writing plugins by hand would mean a second copy of every skill, agent
and MCP server entry, which drifts from the catalog. Generating them from the same renderers the
`claude-code` adapter uses keeps one source, and `--check` in CI keeps the checked-in files equal
to what the generator produces.

The trade-off is consent. Installing or updating a plugin grants whatever that version declares,
without the per-permission questions `grimoire add` asks. The marketplace makes up for that only
partly: by leaving out what can't be made safe (hooks), by listing each plugin's permissions in its
README, and by changing the version whenever the content changes, so an update is visible.

## Layout

`.claude-plugin/marketplace.json` at the repository root (name `grimoire`, owner `jabbott-iii`, no
email; the maintainer's choices on 2026-11-17) lists one plugin per eligible asset and per bundle.
Each plugin is under `plugins/claude-code/<id>/`, with its own `.claude-plugin/plugin.json` and a
generated `README.md`.

## Which assets are plugins

**Eligible assets** are skills, agents and MCP servers that support `claude-code`.

- Their plugin holds the files the `claude-code` adapter installs:

  | Kind | Plugin files |
  |---|---|
  | skill | `skills/<id>/…`, byte for byte |
  | agent | `agents/<id>.md`, with MCP tools named as a plugin's (`mcp__plugin_<server>_<server>__<tool>`) |
  | MCP server | `.mcp.json` with the stdio entry |

- Agent tools name MCP tools the way Claude Code names a plugin's
  (`mcp__plugin_<server>_<server>__<tool>`).
- An agent depends on the plugins of the MCP servers it uses.
- Per-project options, such as an HTTP transport or a renamed environment variable, don't apply.

**Left out:**

- rules, because a plugin's `CLAUDE.md` isn't loaded (Claude Code loads no instructions from a
  plugin);
- tools, because whether a plugin hook can fail closed is unverified while Grimoire never installs
  the binary (the maintainer's choice on 2026-11-17: tool assets with hooks are not plugins until
  fail-closed plugin hooks are tested);
- any asset with no `[clients.claude-code]` table.

Built in M4.7 as described here. The eligibility check, in
`crates/agent-grimoire-adapters/src/plugin.rs`:

```rust
/// Why an asset isn't published as a Claude Code plugin.
pub enum Ineligible {
    Rule,         // Claude Code loads no instructions from a plugin
    Tool,         // no hooks as plugins until a plugin hook is known to fail closed
    NoClaudeCode, // an asset without a `claude-code` table
}

/// Why an asset can't be a Claude Code plugin, or `None` if it can.
pub fn ineligible(manifest: &AssetManifest) -> Option<Ineligible>;

/// The component files of an eligible asset's plugin, and the MCP servers its agent uses.
pub fn files(asset: &SourceAsset) -> Result<PluginFiles, Diagnostic>;

/// Why `name` can't be a plugin name, by Claude Code's reserved-name rules.
pub fn reserved_name(name: &str) -> Option<String>;
```

`files` reuses the `claude-code` adapter's renderers. It fails with `plugin/asset` when the asset is
ineligible, a source file is missing or not UTF-8, an agent's tool can't be named for Claude Code,
or an output goes to an anchor a plugin can't carry.

## Bundles

Bundles become plugins that depend on their eligible members' plugins (the maintainer's choice on
2026-11-17). They use bare names, with no version ranges, because ranges need per-plugin Git tags.
The description names the members left out, with the real reason for each, such as "a rule" or
"a tool". A bundle with no eligible member gets no plugin.

## Names

Names must pass Claude Code's reserved-name rules, including the whole-word rule that
`claude plugin validate --strict` enforces; `gen-marketplace` fails otherwise. `reserved_name`
refuses the exact names `claude`, `anthropic`, `anthropics`, `claude-code` and `claude-mods`; the
prefixes `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-`; `official` next to a reserved
name; and `claude`, `anthropic` or `anthropics` anywhere as a word. The whole-word rule is only a
warning in `claude plugin validate`, but the marketplace is validated with `--strict`, so it
counts.

## Versions

Versions live only in each `plugin.json`, as the docs advise. Each is the asset or bundle version
plus build metadata (the part after `+` in a SemVer version) from a hash of the plugin's files:
`0.2.0+<12 hex>`. Claude Code keeps a user on a cached copy until the version changes, and plugin
content depends on the generator as well as the catalog. The hash makes the version change
whenever the files do.

## Dependencies

An agent's plugin depends on the plugins of the MCP servers whose tools it names, and generation
fails if such a server isn't a plugin. Any other `[requires]` entry that is a plugin becomes a
dependency too; one that isn't is named in the description.

## What each plugin's README says

- **Permissions:** each plugin's README lists what the catalog declares (files, network, shell,
  environment variables, an agent's tools). Installing a plugin grants these without the consent
  `grimoire add` asks for.
- **Binaries:** an MCP plugin names a binary on `PATH`, as the project entry does, and its README
  says to install it first. Plugin MCP servers still need the server binary on `PATH`.
- **Deprecated** assets say so, and name their replacement, at the start of the description.
- **Where it came from:** the README links the asset's catalog directory and says that the file is
  generated and not edited by hand.

## Adding the marketplace

Users add the marketplace as `owner/repo` or a GitHub repository URL. A raw `marketplace.json` URL
can't resolve relative plugin sources.

Codex also reads `.claude-plugin/marketplace.json`
([`CODEX-PLUGINS-P4`](../specs.md#codex-plugins-p4-plugin-and-marketplace-layouts)). Compatibility
is unverified and out of v1 scope.

## CI

`gen-marketplace --check`, then a pinned, checksum-verified Claude Code 2.1.311 runs
`claude plugin validate --strict` on the marketplace and on each plugin directory, with no login
and no secrets (the maintainer's condition on 2026-11-17). The job is `plugins` in
`.github/workflows/ci.yml` ([`../04-quality.md`](../04-quality.md#continuous-integration)).

## Security of the marketplace

A plugin a user installs can run code with their privileges, because its MCP server starts on
every session.

- **No hooks in plugins.** Tool assets are left out until a plugin hook is shown to fail closed, so
  no plugin can silently weaken a guard.
- **No binaries.** An MCP plugin names a binary on `PATH`, as the project entry does, and its
  README says to install it first.
- **No consent step.** Installing or updating a plugin grants whatever that version declares. The
  consent the CLI asks for when permissions grow (F14, in
  [`../01-architecture.md`](../01-architecture.md#threats-and-mitigations)) and when a new version
  forwards more environment variables (M11, in [`mcp.md`](mcp.md)) applies to the `grimoire` CLI
  only. Each plugin's README lists the declared permissions, environment variables and agent
  tools, and its version changes with its content, so an update is visible in `/plugin` (Claude
  Code's command for managing plugins).
- **Generated content.** The generator reuses the `claude-code` adapter's renderers, except that
  agents name MCP tools the way Claude Code names a plugin's, and per-project options (transport,
  variable remapping) don't apply. `--check` in CI keeps the checked-in plugins equal to what the
  generator produces. CI also runs `claude plugin validate --strict` on every plugin with a
  pinned, checksum-verified Claude Code and no secrets.
- **Double installs.** A user who installs an asset with `grimoire` and also enables its plugin
  gets two skills or two agents. An MCP server with the same command connects once, and per
  Claude Code's plugin documentation (`CC-PLUGIN-P4`) the project's definition wins, so a plugin
  agent's plugin-named MCP tools are then unavailable. Neither tool detects the other.

## Invariants

1. Every skill, agent and MCP server in the catalog that supports `claude-code` has a plugin, and
   no rule, tool, or asset without a `claude-code` table does.
2. A plugin's files are the files the `claude-code` adapter installs, except that an agent names
   MCP tools in the plugin form.
3. An agent's plugin depends on the plugin of every MCP server whose tools it names; generation
   fails if that server has no plugin.
4. No plugin name breaks Claude Code's reserved-name rules, the whole-word rule included.
5. A plugin's version changes whenever any of its files changes.
6. The checked-in marketplace equals what the generator produces, and checking it writes only the
   marketplace's own files.
7. The generator writes nothing through a symbolic link where the marketplace's directories go.
8. Every plugin passes `claude plugin validate --strict`.

## Tests

| Invariant | Tests |
|---|---|
| 1 | `the_builtin_catalog_generates` in `xtask/src/marketplace.rs` |
| 2 | `agents_depend_on_their_servers_and_name_plugin_tools` in `xtask/src/marketplace.rs`, and the snapshots of the `claude-code` adapter in `crates/agent-grimoire-adapters/tests/snapshots/`, whose renderers the plugins reuse |
| 3 | `agents_depend_on_their_servers_and_name_plugin_tools` and `an_agent_whose_server_is_no_plugin_is_refused` in `xtask/src/marketplace.rs` |
| 4 | `reserved_names` in `crates/agent-grimoire-adapters/src/plugin.rs` |
| 5 | `plugin_versions_change_with_content` in `xtask/src/marketplace.rs` |
| 6 | `check_and_write_touch_only_the_marketplace` in `xtask/src/marketplace.rs`; CI's `cargo xtask gen-marketplace --check` |
| 7 | `symbolic_links_are_refused` in `xtask/src/marketplace.rs` (Unix-only), which links `plugins/` elsewhere and checks that nothing is written there |
| 8 | CI's `plugins` job in `.github/workflows/ci.yml`, which runs only on GitHub |
