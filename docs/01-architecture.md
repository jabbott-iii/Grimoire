# 01 — Architecture

This document describes how Grimoire is put together: the principles every part follows, the crates and which may depend on which, the repository's layout, how one command flows through the crates, how they use threads, where configuration lives, and the security model that ties the parts together. Each part's own design is in its spec in [`subsystems/`](subsystems/); this document links to them rather than repeating them.

How to read the references here:
- **Spec IDs.** Facts about external formats are cited by the ID of their entry in [`specs.md`](specs.md), the register of external formats and tools, each checked against its primary source. For example, `CC-MEM` is the entry for Claude Code's memory and rule files. An ID with a phase suffix, such as `CC-P2`, was added in that phase; Phase N is milestone MN in [`roadmap.md`](roadmap.md).
- **Diagnostic codes.** A code such as `plan/duplicate-rule` is the stable name of one of Grimoire's errors or warnings. The codes the `grimoire` CLI can print are listed in [`diagnostics.md`](diagnostics.md). Others, such as `crate/hardening`, come from catalog validation ([the catalog spec](subsystems/catalog.md#catalog-validation)).
- **ADRs.** Decisions with lasting consequences are recorded as architecture decision records (ADRs) in [`adr/`](adr/), for example ADR-0001.

## Principles

1. **Install only what the user selected, and own exactly what was written.** Grimoire records every unit it writes, with its hash, in `grimoire.lock`. That lets it update the unit later, detect drift (a change made outside Grimoire), and remove exactly that unit and nothing else ([the install spec](subsystems/install.md)).
2. **No network and no execution while installing.** The catalog is built into the binary, so the install path contains no network code, and installing never runs asset content ([ADR-0001](adr/0001-catalog-delivery.md), which embeds the catalog).
3. **Every write stays inside the project root.** All file access goes through one directory handle on the root, and a symbolic link at or above a path Grimoire manages is refused, even when it points inside the project ([ADR-0006](adr/0006-filesystem-confinement.md), which chose cap-std for this).
4. **Strict formats.** The model types reject unknown fields, and constrained values are newtypes (types that wrap one value and accept only valid ones), validated while parsing, so an error carries its TOML location ([the model spec](subsystems/model.md)).
5. **Stable diagnostics.** Every error and warning has a stable code, such as `manifest/kind-table-missing`. Tests assert on codes, not on message text, and [`diagnostics.md`](diagnostics.md) lists every code a user can see.
6. **Refuse rather than widen.** An adapter refuses an asset it can't render faithfully for a client, such as an agent whose tool list the client can't enforce, instead of installing something broader than the asset declares ([the adapters spec](subsystems/adapters.md)).
7. **Generated files are checked in, with a `--check` mode.** The JSON Schemas, the embedded catalog and the Claude Code marketplace are derived from the Rust types and the catalog, never edited by hand, and CI fails when a checked-in copy is stale ([the xtask spec](subsystems/xtask.md#generated-artifacts)).
8. **Adding an asset is a data-only change** under `catalog/` ([`catalog/README.md`](../catalog/README.md)). A tool or MCP server is a workspace crate inside its own asset directory, so it needs no change to the core crates.
9. **Rust only, with listed exceptions.** Code that isn't Rust needs an exception in [ADR-0010](adr/0010-language-policy-exceptions.md) ([`02-dependency-policy.md`](02-dependency-policy.md#language-policy)).

## Glossary

| Term | Meaning |
|---|---|
| **Asset** | One versioned catalog entry: a manifest (`asset.toml`) plus its content files. |
| **Kind** | One of `rule`, `skill`, `agent`, `tool`, `mcp-server` ([the catalog spec](subsystems/catalog.md#asset-kinds)). |
| **Bundle** | A named, versioned list of asset references. It is not an asset kind ([the catalog spec](subsystems/catalog.md#bundle-not-an-asset-kind)). |
| **Client** | An AI coding tool Grimoire renders for. Grimoire's six are `claude-code`, `codex`, `cursor`, `gemini-cli`, `copilot`, and the portable `agents-md` target. |
| **Adapter** | The code that knows one client's layout. It turns a resolved asset (one whose version has been chosen) into **outputs**. |
| **Output** | One unit of content Grimoire owns in the user's repository: a whole file, a managed block (a marked section that Grimoire owns) in a shared Markdown file, or an entry in a JSON or TOML configuration file ([the install spec](subsystems/install.md#ownership-units-and-merge-strategies)). TOML is a configuration file format. |
| **Source** | Where catalog data comes from. Version 1 (v1) has only `builtin`, the catalog embedded in the binary. [ADR-0001](adr/0001-catalog-delivery.md) chose to embed it, so installing needs no network and the catalog is covered by the binary's own checksums and signatures. |
| **Project root** | The directory containing `grimoire.toml`. Grimoire never writes outside it. |

## Repository layout

```text
Cargo.toml                 workspace; members = ["crates/*", "xtask", "catalog/tools/*", "catalog/mcp-servers/*"]
Cargo.lock
deny.toml                  cargo-deny policy
crates/                    the five library crates each have a README.md for crates.io (Phase 7)
  agent-grimoire-model/    types for asset.toml, bundles, grimoire.toml, grimoire.lock; strict serde + schemars
  agent-grimoire-catalog/  Source (the catalog index), the builtin (embedded) pack, catalog-wide lints
  agent-grimoire-resolve/  selection (assets/bundles/tags), dependency/conflict/client/CLI-version resolution
  agent-grimoire-install/  root-confined fs, atomic writes, outputs/ownership, planner, applier, drift, lockfile I/O
  agent-grimoire-adapters/ one module per client; insta snapshot tests
  agent-grimoire/          the `grimoire` CLI: a library (`run`) plus the binary
  agent-grimoire-mcp/      shared hardening for MCP server assets (Phase 3, publish = false)
  agent-grimoire-testkit/  fixture catalogs and projects for tests (Phase 2, publish = false)
xtask/                     `cargo xtask …`: new, validate, schema, pack-catalog, gen-marketplace, release (publish = false)
release-plz.toml           release-plz config: it opens the release PR only (Phase 5, ADR-0009)
packaging/                 hand-written release inputs that aren't Rust (Phase 5, ADR-0010):
  install/                 install.sh and install.ps1 templates, rendered per release
  npm/grimoire.js          the npm launcher
  homebrew-tap/            sync-formula.yml, a workflow template for jabbott-iii/homebrew-tap
  scoop-bucket/            sync-manifest.yml, a workflow template for jabbott-iii/scoop-bucket
catalog/
  rules/<id>/              asset.toml, rule.md, README.md, CHANGELOG.md, evals/
  skills/<id>/             asset.toml, SKILL.md, scripts/ references/ assets/, README.md, CHANGELOG.md, evals/
  agents/<id>/             asset.toml, agent.md, README.md, CHANGELOG.md, evals/
  tools/<id>/              asset.toml, Cargo.toml, src/, tests/, README.md, CHANGELOG.md
  mcp-servers/<id>/        asset.toml, Cargo.toml, src/, tests/, README.md, CHANGELOG.md
  bundles/<id>.toml
schemas/                   generated JSON Schemas (checked in; CI fails if stale)
.claude-plugin/marketplace.json + plugins/claude-code/…   generated (Phase 4; checked in; CI fails if stale)
docs/                      00-overview.md to 05-release.md, the core documents; roadmap.md;
                           subsystems/, one spec per part; adr/, the decision records;
                           specs.md, the external-format register; and the user reference:
                           cli.md, diagnostics.md, configuration.md, container.md
.github/                   workflows, scripts/ (image-tags.sh, Phase 6, ADR-0011), dependabot.yml;
                           ISSUE_TEMPLATE/ (issue forms), pull_request_template.md and CODEOWNERS (Phase 7)
Dockerfile                 the container image: COPY only, base pinned by digest (Phase 6, ADR-0011)
.grype.yaml                Grype config for the image scan gate (Phase 6, ADR-0011)
README.md                  also published on crates.io (agent-grimoire), on PyPI and in every CLI archive (Phase 7)
CONTRIBUTING.md  SECURITY.md  CODE_OF_CONDUCT.md   community files, at the root only (Phase 7)
LICENSE-MIT  LICENSE-APACHE
CLAUDE.md  AGENTS.md       conventions for this repo's own contributors and agents
```

Package and directory names follow [ADR-0004](adr/0004-package-and-binary-names.md). It chose `agent-grimoire` as the package name on crates.io (the Rust package registry), npm, PyPI, Homebrew and Scoop, because `grimoire` was already taken on crates.io, npm and PyPI. The installed command is still called `grimoire` on every channel except winget, where it is `agent-grimoire`.

The layout refines the project brief, the original statement of requirements, in three ways.

**Refinement 1: code-bearing assets live under `catalog/`, not under `crates/`.**

The brief asks for two things at once: "one crate per MCP server and per tool" in `crates/`, and that "a new asset can be added with files under `catalog/` only" with "no core code changes". Both can't hold if server crates sit in `crates/`.

Making each tool and MCP-server asset directory a workspace crate satisfies both. The member globs `catalog/tools/*` and `catalog/mcp-servers/*` (path patterns in the root `Cargo.toml` that list the workspace's crates) pick each one up. That gives two benefits:

- adding a server is one new directory;
- `crates/` stays "core".

Cargo rejects a member glob that matches nothing (`CARGO-LINTS` in `specs.md`), so the globs were added together with the first code-bearing asset, in Phase 3.

A shared hardening library for servers (transport setup, Origin validation, the authorization layer, limits) lives in `crates/agent-grimoire-mcp` (Phase 3; `publish = false`, so it is never published to crates.io).

- It is core code that new servers *use*, not code they modify.
- Catalog validation requires every MCP server to depend on it (`crate/hardening`).
- Catalog validation also forbids asset crates from depending on the installer's crates (`crate/core-dependency`).

**Refinement 2: adapters are one crate with a module per client.** They share the output model and test harness. Each crate published to crates.io needs its own one-time first publish with a token and its own publishing setup (ADR-0004). A crate per client would add that overhead for no isolation benefit.

**Refinement 3: generated files are checked in.** This covers schemas, the packed catalog and the marketplace. Each generator has a `--check` mode, which fails when the checked-in copy differs from what the generator would write. CI runs it, so "generated, never hand-maintained" is enforced mechanically.

The embedded catalog, `crates/agent-grimoire-catalog/catalog.pack`, sits inside the catalog crate because `cargo publish` packages only files under the crate's own directory ([the catalog spec](subsystems/catalog.md#the-embedded-pack-and-source)). The CLI is a library (`agent_grimoire::run`) plus a thin binary, so tests can drive it with fixture catalogs (small catalogs made for tests) from `crates/agent-grimoire-testkit` ([`04-quality.md`](04-quality.md#test-tools)).

## Layers

Crate names here drop the `agent-grimoire-` prefix, and `cli` is `agent-grimoire`. Each row lists the workspace crates that a crate depends on, as its `Cargo.toml` declares them. `testkit` is only ever a dev-dependency (a dependency used only to build and run tests).

| Crate | Depends on | Dev-dependency | Published |
|---|---|---|---|
| `model` | none of the workspace crates | | yes |
| `catalog` | `model` | | yes |
| `install` | `model` | | yes |
| `resolve` | `model`, `catalog` | `testkit` | yes |
| `adapters` | `model`, `catalog`, `resolve`, `install` | `testkit` | yes |
| `cli` | `model`, `catalog`, `resolve`, `install`, `adapters` | `testkit` | yes |
| `testkit` | `model`, `catalog` | | no |
| `mcp` | none of the workspace crates | | no |
| `xtask` | `model`, `catalog`, `resolve`, `adapters` | `testkit` | no |

The layers, from the bottom:
- `model` holds the file formats. `catalog` loads and validates assets against them, and `install` reads and writes the user's files in terms of them. The two don't know about each other.
- `resolve` decides what to install from the catalog and the user's configuration.
- `adapters` reads a resolution and builds `install`'s output values, one module per client.
- `cli` wires them together and is the only crate a user runs.

Asset crates under `catalog/tools/` and `catalog/mcp-servers/` depend on none of the core crates: catalog validation refuses that (`crate/core-dependency`). An MCP server must depend on `mcp`, the shared hardening (`crate/hardening`); today `grimoire-mcp-project-docs` does, and `grimoire-tool-secret-guard` depends on no workspace crate. Asset crates are never published (`publish = false`).

The published libraries are internal to the CLI: they are published only so that `cargo install agent-grimoire` can build it ([the public API](05-release.md#the-public-api)).

## Main control flow

Every command that changes a project runs the same pipeline, `crates/agent-grimoire/src/pipeline.rs`, shared by `plan`, `apply`, `update` and `uninstall`:

1. **Load the catalog.** The binary's `main` builds `Source::builtin()` from the embedded pack, parsing and validating it again. A pack that fails stops the binary with "this grimoire build is broken" ([the catalog spec](subsystems/catalog.md#the-embedded-pack-and-source)).
2. **Read the project.** `grimoire.toml` and `grimoire.lock` are read through the root handle and parsed strictly ([the model spec](subsystems/model.md)).
3. **Resolve.** `agent_grimoire_resolve::resolve` takes the configuration, the lock, the CLI's version, the update request and the clients that have adapters (`AVAILABLE`). It selects assets, chooses versions, and checks conflicts, client support, `min-cli-version` and permission consent ([the resolve spec](subsystems/resolve.md)).
4. **Render.** `agent_grimoire_adapters::render` turns each resolved asset into the outputs of each selected client, and writes content that several clients read only once ([the adapters spec](subsystems/adapters.md)).
5. **Plan.** The planner compares those outputs with the lock and with what is on disk, unit by unit, and gives each one an action (`Action` in `agent-grimoire-install`): create, update, delete, unchanged, adopt (already as desired on disk, so only the lock changes), gone (no longer wanted and already absent), or a conflict ([the install spec](subsystems/install.md#desired-state-plan-apply)). `plan` prints this and stops.
6. **Apply.** `apply` checks every precondition first and writes nothing if there is a conflict. It re-reads each file just before writing it, writes atomically, and writes the lock last.

`verify` hashes what is on disk and compares it with the lock without planning. `doctor` runs read-only checks of the whole setup. [The CLI spec](subsystems/cli.md) describes each command's place in this flow, and [`cli.md`](cli.md) is the user reference for every command.

## Threading model

The CLI and the core crates are single-threaded and synchronous: they start no threads and use no async runtime. One `grimoire` process works on one project at a time, and two processes in the same project aren't locked against each other ([known gaps](#known-gaps-and-residual-risks)).

MCP servers are the exception. `grimoire-mcp-project-docs` runs on a multi-threaded Tokio runtime (Tokio is Rust's most common async runtime). The shared hardening in `agent-grimoire-mcp` runs each tool call's blocking work on Tokio's blocking pool through `Limits::run`, within limits on concurrency, rate and time ([the MCP spec](subsystems/mcp.md)). The `secret-guard` tool is synchronous.

## Configuration

- **A user's project:** `grimoire.toml`, which the user writes, and `grimoire.lock`, which Grimoire writes. The user reference is [`configuration.md`](configuration.md); the formats are in [the model spec](subsystems/model.md#grimoiretoml).
- **The CLI's environment variables** are listed in [`cli.md`](cli.md#environment).
- **This repository:** `rust-toolchain.toml` pins the toolchain, and the root `Cargo.toml` holds the workspace, its lints and the `dist` profile ([build profiles](05-release.md#build-profiles)). `.cargo/config.toml` defines the `cargo xtask` alias. `deny.toml` is the dependency policy ([`02-dependency-policy.md`](02-dependency-policy.md)), `release-plz.toml` configures release-plz, and `.grype.yaml` is the image scan's gate ([the container image spec](subsystems/container-image.md)).

## Security model

This is Grimoire's threat model: what could go wrong, what Grimoire does about it, and what risk remains. Threats have IDs: F for the installer and the files it writes (below), M for MCP servers ([the MCP spec](subsystems/mcp.md)) and I for the container image ([the container image spec](subsystems/container-image.md)). UNVERIFIED marks a statement that couldn't be confirmed from its primary source. Each spec carries its part's threats and invariants; this section holds the ones that cross parts.

### What Grimoire does on a user's machine

Grimoire installs assets for clients. Assets come in five kinds ([the catalog spec](subsystems/catalog.md#asset-kinds)):
- a **rule** is a Markdown instruction document that a client puts into the model's context (the text the model reads before it acts) without being invoked, always or when matching files are in play;
- a **skill** is a folder of instructions, and optionally scripts, that the model loads when it needs it;
- an **agent** is a subagent definition: a delegated specialist with its own prompt and an explicit list of the tools it may use;
- a **tool** is a Rust program that an agent or a person runs, or that a client runs as a hook ([the assets spec](subsystems/assets.md#tool-hooks));
- an **MCP server** is a program that offers the model more tools over the Model Context Protocol ([the MCP spec](subsystems/mcp.md)).

Clients are the AI coding tools that read these assets: Claude Code, Codex, Cursor, Gemini CLI and GitHub Copilot, plus the portable `agents-md` target, which writes `AGENTS.md` and `.agents/skills/`.

`grimoire`, the command-line interface (CLI), reads a catalog of assets that is embedded in its own binary. [ADR-0001](adr/0001-catalog-delivery.md) chose that so the install path needs no network code, and so the catalog is covered by the same checksums and signatures as the binary. Every release signs the binary ([the release spec](subsystems/release.md)).

It writes files inside one directory, the **project root** (the directory that holds `grimoire.toml`). It keeps a record of everything it wrote in the lock file, `grimoire.lock`. Nothing else.

- **No execution.** Installing never executes asset content, and runs no install hooks (scripts that some package managers run while installing). It never runs the tool or MCP server binaries it configures. `doctor` and the `plan/hook-binary-missing` warning only check whether a file of that name is on `PATH`.
- **No network.** The install path contains no network code.
- **No secrets.** Grimoire never stores secret values. MCP server configuration refers to environment variables by name (`${VAR}`).

### Trust boundaries

A trust boundary is where input from a source Grimoire can't fully trust reaches code that acts on it. This table says how far Grimoire trusts each input, and why.

| Input | Trusted? | Why |
|---|---|---|
| A catalog change in a pull request | Untrusted until merged | `cargo xtask validate` (one of the repository's own check commands, run through `cargo xtask`) checks its form, not its intent. Review decides, and with one maintainer that is usually one person's review ([the repository and contributions](04-quality.md#the-repository-and-contributions)). |
| The embedded catalog | Yes, as part of the binary | It is validated in continuous integration (CI, the automated checks GitHub runs on each change) before packing (`cargo xtask pack-catalog --check`, not yet a required check: [the repository and contributions](04-quality.md#the-repository-and-contributions)). It is compared with a fresh pack of `catalog/` before a release (`cargo xtask release plan`). It is parsed and validated again whenever the binary loads it. And it is covered by the binary's checksums, signature and provenance, a signed record of how the binary was built ([the release spec](subsystems/release.md)). |
| `grimoire.toml`, `grimoire.lock` | Untrusted input | Parsed strictly: unknown fields are errors, and every path is a `RelPath` (a checked relative path, F1). The lock is also validated for consistent ownership: for example, no whole file has two owners, and no entry is owned twice. Both files are part of the repository and are reviewed like code. |
| Every other file in the project | Hostile | Grimoire treats it as if an attacker wrote it. It may hold symbolic links (F3), special files (F5), broken block markers (F9), invalid JSON, or edits to managed content (content Grimoire wrote and tracks in the lock). |
| The filesystem tree itself | Hostile and changing | It may change between Grimoire's checks and its writes (F4). |

### Threats and mitigations

The exit codes in the Result column are the CLI's own, 0 to 6 ([`cli.md`](cli.md#exit-codes), `crates/agent-grimoire/src/out.rs`). The ones used here: 3 is an invalid configuration or lock, or a resolution failure; 4 is a conflict; 6 is a refusal for safety.

| # | Threat | Mitigation | Result |
|---|---|---|---|
| F1 | A destination or lock path escapes the root: `..`, an absolute path, a drive prefix (`C:`) or UNC prefix (`\\server\share`, a Universal Naming Convention path to a Windows network share), or `:` (which names an alternate data stream, a hidden second stream of data inside a Windows file) | Every path is a `RelPath`, Grimoire's type for a checked path. It is ASCII, relative and `/`-separated, with no `.` or `..` components, no `<>:"\|?*`, no control characters, and no trailing `.` or space. ASCII alone rules out invisible and bidirectional-text characters, device names written with superscript digits (Windows treats `COM¹` as `COM1`), and names that collide after Unicode normalization. Asset destinations are anchors, such as `rules:` or `skills:` plus a relative name, which each adapter (the code for one client) turns into a real path. So an asset cannot name a literal path. | Rejected while parsing (exit 3) |
| F2 | Windows device names (`CON`, `aux.md`, `COM1`, …), which Windows treats as devices, not files, in every folder | Rejected by `RelPath` in every component, with or without an extension. | Exit 3 |
| F3 | A symlink (symbolic link: a file that points to another path) at or above a managed path (a path Grimoire writes and tracks), inside or outside the root | Such a link is refused even when it points inside the project, because writing through it would put content where the user didn't expect ([ADR-0006](adr/0006-filesystem-confinement.md)). Each directory is opened with `open_dir_nofollow` (an open that refuses a symlink), one component at a time from the root, after a `symlink_metadata` check (which describes a link itself, not its target). Files are opened no-follow and non-blocking, and the opened handle is checked again. `grimoire.toml`, `grimoire.lock` and `--write-new` sidecars (the `<path>.new` files of F7) are read the same way, and sidecars are checked before anything is written. The checks for client files (`init`, `doctor`) and for `AGENTS.override.md` walk the same way, but count a linked folder on the way as absent rather than failing. `uninstall --purge` checks `grimoire.toml` before it removes anything. | Refused (exit 6). Nothing is written. An existence check refuses nothing: a name under a linked folder counts as absent, so nothing outside the root is detected. |
| F4 | A race that swaps a directory for a symlink between check and use (a time-of-check to time-of-use, or TOCTOU, race) | All access goes through a cap-std handle on the root ([ADR-0006](adr/0006-filesystem-confinement.md)). cap-std is a Rust library whose directory handle reaches only paths beneath it, even if the tree changes during the operation. ADR-0006 chose it because checking each component and then writing by path leaves this race open. It uses `openat2(RESOLVE_BENEATH)` on Linux (a system call that refuses to resolve a path outside the directory), and a handle-relative resolver elsewhere, which opens one component at a time. | The path can't leave the root. The swapped symlink is refused at that component. |
| F5 | Special files at a managed path: a FIFO (first in, first out: a named pipe, whose open for reading waits until a writer opens it), a device or a socket | Checked with `symlink_metadata` before opening. The opened handle is checked again, and on Unix the open is non-blocking, so a FIFO swapped in after the check can't hang Grimoire. | Exit 6 |
| F6 | An oversized file exhausts memory | Reads are capped at 8 MiB, enforced while reading. | Exit 6 |
| F7 | Managed content the user edited is overwritten | Each unit (a whole file, a block or one configuration entry, F8) is compared three ways, by hash: what is on disk, what the lock recorded, and what Grimoire now wants to write ([drift and conflicts](subsystems/install.md#drift-and-conflicts)). Each of these is a **conflict**: content the user edited or deleted; unmanaged content in the way, at a destination the lock doesn't record; and a comment the user added inside Grimoire's entry in a TOML or JSONC file. (TOML is a configuration file format, and JSONC is JSON with comments.) Values are hashed without comments, so an update or removal would otherwise lose that comment. `apply` checks every precondition first, and writes nothing if there is a conflict (exit 4). Only `--force` overwrites. `--write-new` writes `<path>.new` beside the file and leaves the file alone. | Never silent |
| F8 | Removal deletes the user's content | Only owned units are removed: a whole file, a block between Grimoire's begin and end markers, or one JSON member or array element. A whole file in a managed folder is deleted with its asset. That includes a file Grimoire adopted because it already matched, or overwrote with `--force`, because adopting or overwriting makes it Grimoire's. A shared file (`AGENTS.md`, `.mcp.json` and the like) is deleted only if Grimoire created it and nothing else is left in it. The lock records that as `created-file`, which Grimoire also records when it adopts a shared file that holds only its own content. A JSON or TOML container (an object or table that holds entries) is removed only if Grimoire created it and it is empty (`created-keys`). Edited content that is no longer wanted is a conflict, not a deletion. | Only Grimoire's content is removed |
| F9 | A shared file is corrupted by an edit Grimoire can't do safely | Grimoire never guesses what it owns. It refuses the file, even with `--force`, in three cases. **Markdown blocks** have malformed, unterminated, nested or duplicate markers. **JSON** is not strict RFC 8259 (the JSON standard, which allows no comments or trailing commas; RFCs are the Internet's standards documents), or has duplicate keys on Grimoire's path; `.gemini/settings.json` may also hold comments, which Gemini CLI allows. **TOML** (`.codex/config.toml`) doesn't parse. JSON must be strict because Claude Code reads its settings as strict JSON, and a file a client can't parse shouldn't be "fixed" silently. Markers inside a fenced code block (code between two lines of backticks or tildes) are ignored, and a block is never appended inside an unclosed fence. | Exit 4, the file is untouched |
| F10 | A crash or interruption leaves a half-written file | Every file is written to a temporary file in the same directory, fsynced (flushed to the disk), then renamed over the target. The lock is written last. A rerun converges (reaches the same end state), because a unit already in its desired state is adopted rather than reported as drift (a change made outside Grimoire). | No torn files |
| F11 | A file changes between `plan` and the write | `apply` re-reads each file just before writing and aborts if it differs from what was planned. | Exit 4. Rerun. |
| F12 | Two paths that differ only in case | Rejected when planning, before any write. This covers two desired paths, and also a locked path that a new catalog version renames only in case, such as `Guide.md` to `guide.md`. On macOS and Windows, whose file systems ignore case by default, both names are one file, so deleting the old name would delete the new one. | Exit 3 |
| F13 | Line-ending conversion (`core.autocrlf`, the Git setting that can turn `\n` line endings into Windows-style `\r\n` endings, called CRLF for carriage return and line feed) is read as drift | Text hashes fold `\r\n` to `\n`, so both forms hash the same. Bytes on disk that already match are left as they are. A TOML or JSON file Grimoire doesn't change keeps its bytes, and a changed TOML file keeps its CRLF line endings and byte-order mark (an optional invisible character at the start of a file). | No false conflict |
| F14 | An asset update silently gains permissions | Declared permissions (what an asset says it needs, such as `shell` or `hooks`) must be accepted in `grimoire.toml`. Each environment variable is its own grant (`env:<NAME>`), so a version that reads one more variable needs consent for that name. A new version that needs more stops resolution (choosing the asset versions to install) until the user accepts (exit 3). | No silent escalation |
| F15 | Installing for one client grants something to another client | Every case is in [cross-client leaks](subsystems/adapters.md#cross-client-leaks). Every command that plans (`plan`, `apply`, `add --apply`, `remove --apply`, `update`, `doctor`) warns when a `.claude/agents/` agent, a `.claude/settings.json` hook or a `.mcp.json` server reaches a client it wasn't installed for. In the cases listed there, it also warns when Claude Code or Gemini CLI reads an `AGENTS.md` rule that wasn't installed for it. Other rules and skills that another client reads are not reported. | Visible for agents, hooks and MCP servers; not prevented |
| F16 | Hooks run without a trust prompt. (A hook is a command the client runs automatically on an event, such as before each tool call.) | Claude Code runs project hooks without a trust dialog (the prompt that asks the user to trust a project's settings) in non-interactive runs (`-p`) and under the SDK, its software development kit (`CC-SET`, the Claude Code settings and hooks entry). Hooks can therefore come only from tool assets with an explicit `hooks` grant. They are written in exec form: `command` plus `args`, started directly, with no shell. The command is the bare name of a Grimoire binary (`grimoire-tool-<id>`), which the client finds on `PATH`. Limiting hooks to reviewed, tested, CI-built, signed Rust binaries removes the risk of an arbitrary shell command in a JSON file. | Reviewed binaries only |
| F17 | A shared file's permissions are widened | A replaced file keeps its Unix mode bits (its read, write and execute permissions, such as `0644`), which are applied to the temporary file before any content is written. A `<path>.new` sidecar takes the original's mode. Windows ACLs (access control lists, Windows' file permissions) and Unix ownership are not copied. | |
| F18 | A tampered lock points Grimoire at an unrelated file (`.git/HEAD`, `.env`), to print it in a plan diff or delete it | Every lock path must be a location an adapter writes: the folders `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `.cursor/rules/`, `.cursor/agents/`, `.gemini/agents/`, `.github/instructions/` and `.github/agents/`, and the files `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.mcp.json`, `.claude/settings.json`, `.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`. Each record must also claim only what Grimoire owns there: an entry in a configuration file, a block in an instruction file, and a whole file only in a managed folder. Grimoire's own outputs, which belong to no asset (the blocks that make Claude Code and Gemini CLI read `AGENTS.md`, [cross-client leaks](subsystems/adapters.md#cross-client-leaks)), may only be blocks in `CLAUDE.md` or `GEMINI.md`. | Exit 3, before reading |
| F19 | `--force` or `--write-new` destroys work in progress | A `<path>.new` that differs from what Grimoire would write is never replaced (exit 4, nothing written). An edited hook element (one entry in a hook array) is a conflict, never silently forgotten. | |

### Known gaps and residual risks

- **The final step on Windows.** cap-std performs the final step of creating a missing directory, of a rename and of a removal path-based on Windows, after resolving the parents inside the sandbox. Confinement holds, but on Windows the last step isn't relative to a directory handle (ADR-0006, which chose cap-std).
- **Leftover temporary files.** After a crash, a temporary file with a UUID name (a random unique identifier) can be left in a managed directory. Grimoire does not clean these up automatically. On Linux, cap-tempfile 4.0.3 uses an unnamed `O_TMPFILE` file (a Linux file with no name in any directory) where the filesystem supports it, and names it only just before the rename, so a leftover is rare there.
- **Interrupts.** There is no signal handler (code that runs when the process receives a signal, such as Ctrl-C's SIGINT). Atomic writes keep files consistent, and rerunning converges. Grimoire's own exit codes are 0 to 6; a run stopped by Ctrl-C ends with the status the shell reports for a process killed by SIGINT, 130 in a POSIX (standard Unix) shell.
- **Empty directories.** After deleting a file it owns, Grimoire removes every directory above it that is now empty, including one the user created empty. An empty directory holds no content, and Git does not track one.
- **Concurrent runs.** Two `grimoire` processes in the same project at once are not locked against each other.
- **A tampered lock within Grimoire's locations.** Someone who can edit `grimoire.lock` could record the hash of a file Grimoire manages, so that a later removal deletes it. F18 keeps this to Grimoire's own locations. Treat lock diffs like code in review.
- **JSON layout.** Adding an entry to a container written on one line reflows it across lines. The values are unchanged.
- **Claude Code behavior that is not documented** (`specs.md`):
  - whether an `@AGENTS.md` import placed between two HTML-comment marker lines is honored;
  - what versions before 2.1.139 do with exec-form `args`, and before 2.1.295 with `onFailure`.
- **MCP over HTTP, not yet tested end to end** (`CC-P3`, the Phase 3 Claude Code facts, UNVERIFIED):
  - Claude Code's OAuth flow against a Grimoire server, with a real authorization server;
  - whether Claude Code sends an `Origin` header;
  - whether it sends the RFC 8707 `resource` parameter (which names the server a token is requested for).
- **Client behavior that is not documented** (Phase 4, `*-P4`):
  - whether Cursor and Copilot honor `args` and `onFailure` in Claude Code hooks;
  - whether Cursor expands `@AGENTS.md` in `CLAUDE.md`;
  - whether Copilot CLI deduplicates the `AGENTS.md` it reads twice;
  - what an unset `${env:NAME}` becomes in `.cursor/mcp.json`.

  None of the four new clients was run.
- **Trust lives in the client.** Codex loads `.codex/config.toml`, Gemini CLI loads everything from the project, and Copilot loads `.mcp.json` only in a project the user has trusted. Grimoire can't check or grant that; `doctor` says so.
- **Key rotation is manual.** The JWKS file must be replaced when the authorization server rotates its keys.
- **Revocation is per key, not per token.** Removing a key from the JWKS file, or deleting the file, stops its tokens within about 5 seconds. A single token cannot be revoked before it expires. A file that cannot be read or parsed keeps the previous keys.
- **No browser clients.** There is no CORS support (cross-origin resource sharing, the headers that let a web page call a server on another site).
- **Leaks that aren't warned about.** Rules and skills that a client reads without being selected for them are not reported ([cross-client leaks](subsystems/adapters.md#cross-client-leaks)).
- **Published READMEs are frozen per version.**
  - The root `README.md` is the `agent-grimoire` crate's page on crates.io, the wheels' long description on PyPI, and a file in every CLI release archive. Each library crate carries its own `README.md`, each asset archive its asset's, and `xtask/src/release/channels/npm.rs` writes the npm packages' READMEs.
  - Treat each of them as fixed for a version once it is published:
    - crates.io renders the text in the publish request;
    - PyPI keeps the uploaded text when it renders it again, and whether a release's description can be edited afterward is UNVERIFIED (README-RENDER-P7 in `specs.md`);
    - npm shows a new README only with a new version;
    - and an immutable release locks its archives, but only for a release published while the setting is on ([repository settings](05-release.md#repository-settings), and step 7 of "The trust chain" in [the release spec](subsystems/release.md)).
  - So a wrong or outdated security note or install step stays with that version. A correction reaches users only with the next release, or through an advisory ([reporting a vulnerability](#reporting-a-vulnerability)).
  - The text is frozen, but not how it renders: crates.io resolves relative links against the repository's default branch as it is now, not the published version, and PyPI renders stored descriptions again when it upgrades its renderer.
- **The process around the code:** one maintainer, one reviewer, and CI that doesn't yet gate a merge ([the repository and contributions](04-quality.md#the-repository-and-contributions)).
- **Releases and distribution:** the residual risks are listed in [the release spec](subsystems/release.md), and the container image's in [the container image spec](subsystems/container-image.md).

### Reporting a vulnerability

Report a vulnerability privately, never in a public issue or pull request. Private vulnerability reporting is enabled on the repository (read on 2026-11-17), and its form is at <https://github.com/jabbott-iii/Grimoire/security/advisories/new>; it needs a signed-in GitHub account. [`SECURITY.md`](../SECURITY.md) is the policy: what to report, what to include, and what happens next.

- **One person handles reports.** The repository has one collaborator, so only the maintainer can triage a report (assess it and decide what to do) and publish an advisory. GitHub notifies them only if they watch the repository for all activity or for security alerts ([repository settings](05-release.md#repository-settings)).
- **Most users get no alert.**
  - GitHub's Advisory Database, which Dependabot alerts are built on, covers crates.io, npm and PyPI packages, but an alert reaches only a repository whose dependency graph lists the package. A CLI installed with `cargo install`, `npm install --global` or `pipx` isn't such a dependency.
  - Homebrew, Scoop, winget, the install scripts and container images aren't among the database's ecosystems at all, and the tool and MCP server binaries ship only in the release archives and the image.
  - So users learn of a fix from the advisory and the release notes, which `SECURITY.md` asks them to watch.
- **A fix made in private skips CI** ([the repository and contributions](04-quality.md#the-repository-and-contributions), residual risks).
