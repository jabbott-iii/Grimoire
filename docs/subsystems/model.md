# Subsystem: `agent-grimoire-model`

`agent-grimoire-model` holds the strict Rust types for Grimoire's four TOML documents: an asset's
manifest (`asset.toml`), a bundle (`catalog/bundles/<id>.toml`), a project's configuration
(`grimoire.toml`) and the lockfile (`grimoire.lock`). It also holds the checks that need only one
of those documents, and it is the source of their JSON Schemas in `schemas/`. It guarantees that
a document that parses has no unknown field and no invalid value, that every path it carries is
confined to the project root as a string, and that a lock written by Grimoire serializes the same
way on every operating system. It is the bottom layer of the CLI: every other crate builds on it,
and it depends on none of them. Three decisions shaped it: [ADR-0001](../adr/0001-catalog-delivery.md)
(the catalog is built into the binary, so the lock's only source is `builtin`),
[ADR-0005](../adr/0005-client-priority.md) (the six clients and their order) and
[ADR-0006](../adr/0006-filesystem-confinement.md) (every write is confined to the project root).

How to read the references in this file:

- **Spec IDs.** Facts about external formats are cited by the ID of their entry in
  [`docs/specs.md`](../specs.md), the register of external formats, each checked against its
  primary source. For example, `CC-SKILL` is the entry for Claude Code's skills. An ID with a
  phase suffix, such as `CC-P3`, was added in that phase.
- **Diagnostic codes.** A code such as `config/url-missing` is the stable name of one of
  Grimoire's errors or warnings. The codes the `grimoire` CLI can print are listed in
  [`docs/diagnostics.md`](../diagnostics.md). The `manifest/` and `bundle/` codes are for catalog
  authors, and catalog validation reports them ([`catalog.md`](catalog.md#catalog-validation)).
- **Terms** such as *asset*, *client*, *adapter* and *output* are defined once, in the
  [glossary](../01-architecture.md#glossary).

The API reference is the crate docs, the `//!` lines at the top of
`crates/agent-grimoire-model/src/lib.rs`, which docs.rs shows. This spec gives the design and
the formats; the crate docs list the types.

## Why strict types, checked at the boundary

The four documents are written by people: catalog authors write `asset.toml`, and users write
`grimoire.toml`. A misspelled key in a format that ignores unknown keys does nothing, silently.
`maturty = "stable"` would leave an asset's maturity unset, and nobody would notice until it
mattered. So every document refuses unknown fields, and every constrained value, such as an
identifier, a path, a version or a license expression, is a newtype (a type that wraps one value
and accepts only valid ones) checked while the document is parsed. The error then carries the
TOML line and column, and code past the boundary never checks a value again.

The alternatives were a permissive parser with a separate lint pass, which reports a typo only if
someone runs the lint, and validating values where they are used, which repeats each check and
loses the location.

What it trades away: an older CLI refuses a file that uses a field added after it was released.
A project that uses a new field needs a CLI that has it. The `format` number at the top of each
document makes that refusal explicit: this version of Grimoire accepts only `format = 1`.

## Identifiers and versions

Built in M1.2 as described here.

- **Asset IDs are unique across the whole catalog**, not just per kind. That gives:
  - short commands, with no kind to type (`grimoire add rust-errors`);
  - unambiguous keys in the lock;
  - one-to-one plugin names in the generated marketplace.
- **Format:** the Agent Skills `name` rules for every kind: 1–64 characters of `[a-z0-9-]`, no
  leading, trailing or double hyphen, and equal to the directory name. Bundles use the same type
  (`AssetId`) and share the same namespace.
  - IDs starting with the plugin-reserved prefixes `claude-`, `anthropic-`, `anthropics-` and
    `cc-plugin-` are also rejected, because each asset and bundle ID becomes a plugin name in the
    generated marketplace, and Claude Code's plugin tools report such a name as an error
    (`CC-PLUGIN-P4`).
  - The name `grimoire` and the prefix `grimoire-` are kept for content Grimoire itself owns in a
    project, such as the `CLAUDE.md` block that imports `AGENTS.md` (`Lockfile::PROJECT_OWNER`;
    see [`grimoire.lock`](#grimoirelock)).
  - `synced` is a folder name that Claude Code reserves for skills, so it is refused too.
- **Asset versions use semantic versioning (SemVer), one version per asset.** A SemVer version
  has the form `MAJOR.MINOR.PATCH`, such as `1.0.0`.
  - CI requires a version bump and a CHANGELOG entry whenever an asset's files change
    ([`catalog.md`](catalog.md#catalog-validation)).
  - The asset version is independent of the CLI version.
  - A `Version` is SemVer 2.0.0, such as `1.2.3` or `1.2.3-rc.1`, **without build metadata** (the
    part after a `+`). Build metadata doesn't change SemVer precedence, so refusing it makes the
    type's ordering exactly SemVer precedence. `Version::parse` and deserialization refuse it;
    `From<semver::Version>` doesn't check, so only a version without build metadata may be
    converted that way.
  - A `VersionReq` is a Cargo-style requirement, such as `^1.2`, `>=1, <3` or `*`.
- **Manifests carry `min-cli-version`**, the oldest CLI that can install the asset. Resolution
  fails with a clear error if the running CLI is older ([`resolve.md`](resolve.md#resolution-errors)).
- **Every document must set `format = 1`.** `Format` accepts only `Format::CURRENT`, which is 1.

## Manifests and configuration

The three files below, `asset.toml`, `grimoire.toml` and `grimoire.lock`, are TOML, and so is a
bundle. They are parsed with unknown fields as errors. JSON Schemas for them are generated from
the Rust types into `schemas/`, by `cargo xtask schema` ([`xtask.md`](xtask.md#generated-artifacts)).

A document is checked in two steps, and each reports problems its own way:

1. **Parsing** (`from_toml`, such as `AssetManifest::from_toml`) checks the TOML syntax, every
   field's type, unknown fields and each constrained value. The first problem stops it: the
   `ParseError`'s message includes the line and column, and its span is the byte range of the
   offending input, when known.
2. **Validation** (the `validate` module: `validate::manifest`, `validate::bundle`,
   `validate::project_config` and `validate::lockfile`) takes a parsed document and reports every
   rule it breaks, as a list of `Diagnostic`s. For example, a `deprecated` maturity needs a
   `[deprecated]` table that names the replacement, and an asset can't be both selected and
   excluded in `grimoire.toml`. Rules that span files or assets belong to the catalog crate
   ([`catalog.md`](catalog.md#catalog-validation)).

A `Diagnostic` is an error or a warning. An error fails validation; a warning is reported, but
doesn't. Each has a stable code, so tests and tooling match on the code rather than on the
message. Its `file` and `field` say where the problem is, as in `clients.cursor.outputs[0]`. A
diagnostic's display output escapes control characters in its file, field and message (newlines
in the message are kept), so that an untrusted name can't inject terminal escape sequences or
workflow commands for continuous integration (CI).

## `asset.toml`

Built in M1.2 as described here, with the Phase 3 additions marked below.

```toml
format = 1                         # manifest format version
id = "rust-errors"
kind = "rule"                      # must match catalog/<kind>/
version = "1.0.0"
summary = "Idiomatic error handling for Rust CLIs"   # one line, ≤ 120 chars
description = """Longer Markdown description."""
license = "MIT OR Apache-2.0"     # SPDX expression, validated
maintainers = ["jabbott-iii"]      # GitHub logins
tags = ["rust", "errors"]
maturity = "stable"                # experimental | stable | deprecated
# deprecated = { replacement = "rust-errors-v2", reason = "…" }   # required iff maturity = "deprecated"
min-cli-version = "0.1.0"

[requires]                         # dependencies: id → semver requirement
# other-asset = "^1"
[conflicts]                        # id → requirement + reason
# legacy-style = { version = "*", reason = "contradicts error-type guidance" }

[permissions]                      # what the asset can cause at runtime; all default to none/false
filesystem = "none"                # none | read | write
network = false
shell = false
env = []                           # env var NAMES the asset reads (never values)
hooks = false                      # registers automatic client hooks

[rule]                             # at most one kind table, matching `kind`
paths = ["**/*.rs"]                # omit for always-on

[clients.claude-code]              # presence of a table = supported client
outputs = [{ src = "rule.md", to = "rules:rust-errors.md", strategy = "render" }]
[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
```

`license` is an SPDX expression (SPDX is the Software Package Data Exchange standard for license
names), checked as `LicenseExpr`.

- **Kind tables** (at most one per manifest, matching `kind`; `agent`, `tool` and `mcp-server`
  require theirs). What each kind is, and why, is in [`catalog.md`](catalog.md#asset-kinds).
  - `[skill]` (no required keys);
  - `[agent] tools = [...]`. A `model` hint is deferred until an example asset in the catalog
    needs one, so agents currently inherit the session's model;
  - `[tool] binary = "…"` plus optional `hooks = [...]` bindings: `event`, `tools`, `args`,
    `timeout-secs` and (Phase 3) `on-failure`. `on-failure = "block"` is allowed only on
    `pre-tool-use` and `user-prompt-submit`, the events whose failure can stop something
    (`manifest/hook-on-failure`);
  - `[mcp-server] binary = "…", transports = ["stdio","streamable-http"], env = [...]`, plus the
    declared tool list with annotations, which tests compare against the live server.
  - **Phase 3, environment variables:**
    - `permissions.env` lists *every* variable the asset reads, and each one needs consent
      ([`resolve.md`](resolve.md#permission-consent)).
    - `[mcp-server] env` lists the subset that the client configuration forwards from the user's
      environment.
    - A variable the client sets itself, such as Claude Code's `CLAUDE_PROJECT_DIR` for stdio
      servers (`CC-P3`), is in `permissions.env` only, so nothing is forwarded for it.
- **Explicit output map per client (refines the brief's "explicit file map").** Every client an
  asset supports lists its outputs explicitly: source file → destination → strategy. A reviewer
  can see exactly what each asset writes for each client. Each output is an `OutputSpec`.
  - The destination (a `Destination`) is written as an **anchor** (`rules:`, `skills:`, `agents:`,
    `instructions`, `mcp`, `hooks`) plus a relative name, rather than a literal path. An anchor
    (`Anchor`) is a named destination, and each adapter resolves its anchors to real paths. For
    example, `rules:rust-errors.md` becomes `.claude/rules/rust-errors.md` for `claude-code`
    ([`adapters.md`](adapters.md#per-client-mapping)).
  - Reason 1: a client changing its layout becomes one adapter change instead of an edit to every
    manifest.
  - Reason 2: the planner can choose a location that several selected clients share
    ([`adapters.md`](adapters.md#client-read-matrix-and-cross-client-planning)).
  - The resolved literal paths are shown by `plan` and recorded in the lock.
  - Validation rejects any destination outside the adapter's declared anchor set. An asset
    therefore *cannot* write, for example, `.github/workflows/x.yml`.
  - `cargo xtask new` scaffolds these tables ([`xtask.md`](xtask.md)).
  - The alternative, literal paths, would make every layout change a catalog-wide edit.
- **Strategies** (`Strategy`):
  - `copy`: a whole file, byte-for-byte.
  - `render`: a whole file generated by the adapter, for example with client frontmatter.
  - `block`: a managed block in a shared Markdown file.
  - `entry`: an owned entry in a JSON or TOML config file.

  How each strategy is written, updated and removed is in
  [`install.md`](install.md#ownership-units-and-merge-strategies).

## `grimoire.toml`

Built in M1.2 as described here. The `[allow]` table came in Phase 2 (M2.2) and the HTTP
transport options in Phase 3 (M3.4), as noted below.

This is the file in the user's repository that says which clients and assets they want
(`ProjectConfig`). Users edit it by hand or with `grimoire add` and `grimoire remove`. The user
reference for every field is [`docs/configuration.md`](../configuration.md).

```toml
format = 1
clients = ["claude-code", "agents-md"]
bundles = ["rust-cli"]
tags = []                          # select every asset carrying these tags
exclude = []                       # never select these, even via bundle/tag

[assets]
rust-errors = "^1"
changelog-writer = { version = "^0.3", clients = ["claude-code"], allow = ["shell"] }
github-insights = { version = "^1", allow = ["network", "env:GITHUB_TOKEN"], env = { GITHUB_TOKEN = "GH_TOKEN" } }
project-docs = { version = "^0.1", allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"], transport = "streamable-http", url = "http://127.0.0.1:8931/mcp" }
```

An `[assets]` entry is either a bare version requirement or a table (`AssetSelection`,
`AssetOptions`). Per-asset overrides in v1:

- a subset of clients;
- **`allow`**: accepted permissions ([`resolve.md`](resolve.md#permission-consent));
- for MCP servers:
  - a remapping of environment-variable *names*. In the example above, the server's
    `GITHUB_TOKEN` is read from the user's `GH_TOKEN`;
  - the transport. `transport = "streamable-http"` needs the server's `url` (Phase 3, below).
- Setting `transport`, `url` or `env` on any other kind is an error (`resolve/options-kind`).

ADR-0002 also proposed an override `runtime = "docker"`. It would write an MCP server's entry as a
`docker run` command for Grimoire's container image, instead of a command for a local binary. It
is deferred past v1 (the maintainer's choice in Phase 6, recorded in
[ADR-0011](../adr/0011-container-image.md), which designs the container image), for two reasons:

- no MCP client offers a common variable for the project directory;
- the CLI can't embed the digest of the image that contains it. (A digest is the SHA-256 of the
  image, and the image holds the CLI, so the digest doesn't exist yet when the CLI is built.)

`grimoire.toml` still rejects it as an unknown field. [`docs/container.md`](../container.md) shows
hand-written, digest-pinned `docker run` entries instead.

Nothing that could alter destinations outside the anchor rules is overridable.

**Consent for assets without an `[assets]` entry (Phase 2).** An asset brought in by a bundle, a
tag or another asset's `requires` has no `[assets]` table to hold `allow`. Its grants go in a
top-level table instead:

```toml
[allow]
secret-guard = ["filesystem-read", "hooks"]
```

Each asset's grants live in exactly one place.

- An asset listed under `[assets]` takes them there, and an `[allow]` entry for it is an error
  (`config/allow-direct`).
- `grimoire add` writes whichever applies. It moves the grant when an asset becomes direct, that
  is, listed under `[assets]`.
- An `[allow]` entry for an asset that is not installed is a warning (`resolve/allow-unused`).

**HTTP transport (Phase 3).** With `transport = "streamable-http"`, the user runs the server and
the client connects to it, so the entry needs the server's `url`:

- `url` is required with `streamable-http` and refused otherwise (`config/url-missing`,
  `config/url-transport`).
- `env` overrides are refused with it (`config/env-transport`): the client does not start the
  server, so it passes no environment.
- The URL is checked while parsing (`ServerUrl`):
  - It must be `https://`, or `http://` only for this machine (`localhost`, `127.0.0.0/8`,
    `[::1]`).
  - User information (as in `user:password@host`), a query or a fragment are refused, so no
    credential can be written into a configuration file. MCP forbids access tokens in URLs
    (`MCP-SPEC`).
  - So are `$` and `{`, which a client would expand as a variable reference.
  - It must be ASCII only, at most 2048 characters.
- `claude-code` writes `mcpServers.<id> = {"type": "http", "url": "<url>"}`.
- Authorization, when the server requires it, is negotiated by the client with the server's OAuth
  metadata ([ADR-0007](../adr/0007-mcp-server-authorization.md), which has servers check JSON Web
  Token access tokens against a local key set; see [`mcp.md`](mcp.md)). Grimoire writes no headers
  and no tokens.

## `grimoire.lock`

Built in M1.2 as described here. The Phase 2 additions came with the installer (M2.4).

The lockfile (`Lockfile`) records exactly what Grimoire installed: each asset's version and
source (`LockedAsset`), why it was selected (`Selector`), and a hash of every unit of content it
owns (`LockedOutput`, by `Unit`). Planning compares it with the disk to find drift
([`install.md`](install.md#drift-and-conflicts)).

```toml
format = 1                          # lockfile format version; never contains the CLI version

[[asset]]
id = "rust-errors"
kind = "rule"
version = "1.0.0"
source = "builtin"                  # future: "index+https://…" (ADR-0001)
manifest = "sha256:…"
selected-by = ["direct"]            # or "bundle:rust-cli", "tag:security", "requires:x"
allow = []

  [[asset.output]]
  clients = ["claude-code"]
  path = ".claude/rules/rust-errors.md"
  unit = "file"                     # file | block | entry
  hash = "sha256:…"                 # hash of the content Grimoire owns in that unit
```

The lock is sorted deterministically and uses normalized `/` paths, so it diffs cleanly and is
identical across operating systems.

- **It never holds the CLI version**, so an upgrade alone doesn't change the lock. It changes only
  when an asset's version changes, or when what Grimoire writes for a client changes, which
  `grimoire plan` shows before `apply` records it (ADR-0001). Each asset's `SourceId` is
  `builtin`, the catalog built into the binary, the only source today.
- **`Lockfile::canonicalize` sorts it**: assets by ID, outputs by path and then key, and each list
  of clients and selectors, with repeats removed. A canonical lock serializes the same way every
  time.
- **`validate::lockfile` checks ownership:** a path is owned either as one whole `file`, or as
  `block`s, at most one per owner, or as `entry` units with distinct keys, and never by a mix of
  these.

**Phase 2 additions to the lock:**

- A top-level `[[output]]` list (`Lockfile::outputs`) holds content Grimoire itself owns rather
  than an asset.
  - Today that is the `CLAUDE.md` block that imports `AGENTS.md`
    ([`adapters.md`](adapters.md#client-read-matrix-and-cross-client-planning)), and, since
    Phase 4, the `GEMINI.md` block that imports it for Gemini CLI.
  - Its block markers use the reserved name `grimoire` (`Lockfile::PROJECT_OWNER`). The asset id
    `grimoire` and the prefix `grimoire-` are reserved so that no asset can collide with it.
- Creation records:
  - `created-file = true` records that adding an output created its (shared) file.
  - `created-keys = ["/mcpServers"]` records the JSON containers it created.
  - Removal deletes exactly those, once empty. A file or container that existed before Grimoire
    is never deleted.
  - When the creating output is removed while others remain, the record moves to one of them.

The installer reads the lock as untrusted input: a lock path is limited to the locations Grimoire
writes, so a tampered lock can't make `plan` print or `apply` delete any other file
([`install.md`](install.md#filesystem-safety)).

## Paths are portable ASCII `RelPath`s

Built in M1.2 as described here.

Every path that is written or installed is a `RelPath`: relative, with `/` separators, at most
`MAX_REL_PATH` (200) characters, and safe on Linux, macOS and Windows. It refuses:

- an absolute path, a drive prefix such as `C:`, `\`, and empty, `.` or `..` components, so a
  destination string can't name anything outside the project root;
- any character Windows forbids in a file name (`<>:"|?*` and control characters), a component
  that ends in `.` or a space (Windows strips them), and the Windows device names, such as `CON`
  or `NUL.txt`;
- every non-ASCII character. That rules out invisible characters, bidirectional-text characters
  (which change the direction text is displayed in), superscript device names (Windows treats
  `COM¹` as a device), and names that macOS or Windows normalize or fold into each other.

A `RelPath` checks only the string. A symbolic link on the way to the path is a separate, runtime
concern of the installer, which refuses to write through one (ADR-0006;
[`install.md`](install.md#filesystem-safety)).

## The JSON Schemas

`schema::documents` returns the four schemas, in a fixed order, with the file name each one has
under `schemas/`. They are generated with schemars and pinned to JSON Schema draft 2020-12, so
that a schemars upgrade can't silently change the published draft. Each schema's descriptions
come from the types' doc comments. How `cargo xtask schema` writes them, and how CI checks that
they are current, is in [`xtask.md`](xtask.md#generated-artifacts).

## Invariants

1. Every document refuses an unknown field, with an error that points at the field's location.
2. Every constrained value (`AssetId`, `RelPath`, `Version`, `VersionReq`, `LicenseExpr`,
   `ServerUrl`, `EnvVarName`, `Sha256Digest` and the others) is checked while parsing, and an
   invalid one is a parse error, never a value inside a parsed document.
3. Every document sets `format = 1`, and any other number is refused.
4. An `AssetId` follows the Agent Skills `name` rules and refuses the reserved prefixes and names.
5. A `RelPath` is relative and confined: no absolute path, drive prefix, `\`, empty, `.` or `..`
   component, Windows-forbidden character or device name, trailing `.` or space, or non-ASCII
   character, and at most 200 characters.
6. A `Version` refuses build metadata, so its order is exactly SemVer precedence.
7. An output's anchor belongs to its client and fits its strategy and the asset's kind; any other
   destination fails validation.
8. A rule declares no permissions, and an agent declares the permissions its tools imply (for
   example, `execute` requires `shell`).
9. A manifest has at most one kind table, matching `kind`, and `agent`, `tool` and `mcp-server`
   have theirs.
10. `on-failure = "block"` appears only on `pre-tool-use` and `user-prompt-submit` hooks.
11. A grant lives in exactly one place: an asset under `[assets]` has no `[allow]` entry.
12. `url` is present exactly when `transport = "streamable-http"`, and `env` overrides never
    appear with it.
13. A canonical lock serializes to the same bytes every time, round-trips through parsing, and
    never holds the CLI version.
14. Each lock path is owned as one `file`, or as `block`s (at most one per owner), or as `entry`
    units with distinct keys, never as a mix.
15. A diagnostic's display output contains no raw control characters from its file, field or
    message, except newlines in the message.
16. The checked-in `schemas/` match the types.

## Tests

| Invariant | Test |
|---|---|
| 1 | `unknown_fields_are_errors_with_location` in `crates/agent-grimoire-model/tests/documents.rs`; the crate docs' example (a doc test) |
| 2 | `invalid_values_are_reported_while_parsing` and `server_urls_are_checked_while_parsing` in `tests/documents.rs`; `asset_ids`, `github_logins`, `env_vars`, `globs`, `binary_names`, `mcp_tool_names` and `server_urls` in `src/values.rs`; `kebab_rules` and `text_rules` in `src/constrained.rs`; `expressions` in `src/license.rs`; `digests` and `digest_of_bytes` in `src/scalars.rs` |
| 3 | `format_accepts_only_current` in `src/scalars.rs` |
| 4 | `asset_id_matches_the_rules`, a property test, in `tests/properties.rs`; `asset_ids` in `src/values.rs` |
| 5 | `accepted_paths_are_confined`, `traversal_is_always_rejected` and `safe_component_lists_are_accepted`, property tests, in `tests/properties.rs`; `accepts_ordinary_paths` and `rejects_unsafe_paths` in `src/path.rs` |
| 6 | `versions_and_requirements` in `src/version.rs` |
| 7 | `outputs_must_fit_client_anchor_kind_and_strategy` in `tests/documents.rs`; `strategy_anchor_matrix`, `destinations_parse` and `portable_target_has_no_agents_or_mcp` in `src/output.rs`; `destinations_round_trip`, a property test, in `tests/properties.rs` |
| 8 | `rules_must_not_declare_permissions` and `agents_declare_the_permissions_their_tools_imply` in `tests/documents.rs` |
| 9 | `kind_tables_must_match_kind` in `tests/documents.rs` |
| 10 | `fail_closed_hooks_are_allowed_where_events_can_be_blocked` and `tool_and_hook_rules` in `tests/documents.rs` |
| 11 | No test yet: nothing asserts `config/allow-direct`. Its place is beside `project_config_rules` in `tests/documents.rs` |
| 12 | `project_config_rules` in `tests/documents.rs` (`config/url-missing`, `config/url-transport`, `config/env-transport`) |
| 13 | `lockfile_round_trips_canonically` and `canonicalize_sorts_assets_outputs_and_selectors` in `tests/documents.rs`; `lockfile_format`, an insta snapshot of a real lock, and `crlf_lockfiles_are_not_changes`, in `crates/agent-grimoire/tests/cli.rs` |
| 14 | `lockfile_ownership` and `lockfile_shape_rules` in `tests/documents.rs` |
| 15 | `display_formats` in `src/diagnostic.rs` |
| 16 | `cargo xtask schema --check`, which CI's catalog job runs ([`xtask.md`](xtask.md#generated-artifacts)) |

`every_manifest_rule_has_a_mutation` in `tests/documents.rs` checks the validation rules as a
group: for each manifest rule it changes one line of a valid manifest and asserts that exactly
that rule's code is reported. Other cases are in `bundles_parse_and_validate`, `bundle_rules`,
`mcp_server_rules`, `skill_outputs_keep_the_layout`, `skills_must_install_skill_md`,
`deprecation_requires_replacement_and_vice_versa`, `rule_path_duplicates_are_warnings_only`,
`tool_hook_and_output_branches` and `project_config_parses_both_selection_forms`, all in
`tests/documents.rs`.
