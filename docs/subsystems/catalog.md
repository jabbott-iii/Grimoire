# Subsystem: `agent-grimoire-catalog`

`agent-grimoire-catalog` defines what the catalog holds and keeps it valid. The catalog is the
collection of assets Grimoire can install: rules, skills, agents, tools and Model Context Protocol
(MCP) servers, one directory per asset under `catalog/`, plus bundles in `catalog/bundles/`. The
crate's `load` reads that directory without following symbolic links and runs every check that
needs more than one document; `pack` turns a valid catalog into one deterministic file,
`catalog.pack`; and `Source` loads such a file into the in-memory index that the CLI installs
from. It guarantees that an invalid catalog can never be packed, so it can never be built into a
release, and that a damaged pack fails when it is loaded, not halfway through an install.
[ADR-0001](../adr/0001-catalog-delivery.md) (the catalog is built into the binary, behind a
`Source`) and [ADR-0002](../adr/0002-mcp-server-and-tool-packaging.md) (each tool and MCP server is
its own Rust binary) shaped it.

Spec IDs such as `SKILLS` name entries in [`docs/specs.md`](../specs.md), the register of external
formats. Codes such as `catalog/unexpected-entry` are diagnostics for catalog authors: only
`cargo xtask` commands print them, and they aren't part of Grimoire's public API
([`docs/diagnostics.md`, "Not listed here"](../diagnostics.md#not-listed-here)). The API reference
is the crate docs in `crates/agent-grimoire-catalog/src/lib.rs`. The fields of `asset.toml` are in
[`model.md`](model.md#assettoml).

## Why a validated catalog, built into the binary

Adding an asset is a data-only change: a contributor adds a directory under `catalog/` and no
Rust code. Grimoire installs that data into other people's repositories, where a rule loads into
an AI model's context and a hook runs with the user's privileges. So the catalog is checked by
machine before anything reaches a user: strict manifests, a fixed directory layout, size limits,
the Agent Skills rules, and a render of every asset for every client it declares. A reviewer then
reads content, not structure.

The catalog is built into the CLI binary (ADR-0001), rather than fetched:

- installing needs no network code, and works offline;
- the catalog is covered by the binary's own checksums, signatures, software bill of materials
  (SBOM, the list of what the binary contains) and provenance (the signed record of how it was
  built), so users have no second key or identity to trust;
- the CLI version and the lockfile together determine what is installed.

The cost is that a new or changed asset reaches users only in a CLI release, and that a pack holds
one version of each asset. A newer CLI therefore can't write the files again for a lockfile that
pins an older version, though `grimoire verify`, which needs only the recorded hashes, still works.
The alternatives ADR-0001 weighed, such as a remote index, are in that record.

## Asset kinds

The model types for every kind were built in M1.2, and the first asset of each kind in M3.2 to
M3.5.

The kinds differ in **how the content reaches the model** and **what it can do**. The security
model ([`01-architecture.md`](../01-architecture.md#threats-and-mitigations)) reasons in the same
terms.

| Kind | Reaches the model… | Can execute? | Installed by Grimoire as… |
|---|---|---|---|
| Rule | automatically, always or when matching files are in play | no | rendered instruction files or managed blocks |
| Skill | on demand, when the model picks it by its description | the agent may run bundled scripts | a byte-for-byte copy of the skill directory |
| Agent | as a delegated sub-session with its own prompt and tools | through the tools it is granted | a rendered subagent definition |
| Tool | not at all; it is invoked as a program | yes, a Rust binary | client wiring only (hook bindings, permission entries). The binary is not installed |
| MCP server | through MCP tool, resource and prompt listings | yes, a Rust server process | a client MCP-config entry. The binary is not installed |

### Rule

A **rule** is a Markdown instruction document that a client loads into the model's context
**without being invoked**: nobody has to ask for it. It loads either always, or when files
matching its declared globs are in play. Globs are file patterns such as `**/*.rs`.

A rule carries conventions, constraints and procedures. It has no executable content and grants
no tools. It also carries no client-specific frontmatter, the block of YAML (a configuration file
format) between `---` lines at the top of a Markdown file. Grimoire generates each client's
frontmatter from the manifest.

- **Source:** `catalog/rules/<id>/rule.md`, plus `asset.toml` with `[rule] paths = [...]`.
  Without `paths`, the rule is always on.
- **Rendering:** a file of its own wherever the client has one for rules; otherwise a managed
  block (a marked section Grimoire owns) in the client's shared instruction file. See
  [`adapters.md`](adapters.md#per-client-mapping).
  - Per-rule files: Claude Code `.claude/rules/`, Cursor `.cursor/rules/*.mdc`, Copilot
    `.github/instructions/*.instructions.md`.
  - Shared files: `AGENTS.md`, `GEMINI.md`.
  - Phase 4 exception, decided in [ADR-0008](../adr/0008-phase-4-client-adapters.md) (the Phase 4
    adapters, which keep hooks Claude Code only and read shared rules from `AGENTS.md`): when a
    rule is in `AGENTS.md`, Claude Code, Cursor, Gemini CLI and Copilot read it from there and get
    no per-rule file or block of their own
    ([`adapters.md`](adapters.md#client-read-matrix-and-cross-client-planning)).
- **Clients without path scoping** (`AGENTS.md`, `GEMINI.md`) can't load a rule only for some
  files. There, a path-scoped rule is rendered with an explicit scope line in plain language
  ("Applies when working on files matching …").
- **Not a rule:** content the model should load only when relevant to a task (that is a skill),
  or a persona with its own tools (that is an agent).
- **Quality bar:** concise, scoped, actionable. [Catalog validation](#catalog-validation) rejects
  rules that are exact or near duplicates of another, and flags declared conflicts between rules
  that can be installed together.

### Skill

A **skill** is a directory that conforms to the Agent Skills specification (`SKILLS`), an open
format that many clients read. It contains:

- a `SKILL.md` file, with `name` and `description` frontmatter that the spec allows, and a
  Markdown body;
- optional `scripts/`, `references/` and `assets/` folders.

Clients show the model only the name and description until the model activates the skill. This is
called progressive disclosure: the full instructions take up context only once they are needed.

- **Source:** the asset directory *is* the skill directory: `catalog/skills/<id>/SKILL.md`, with
  `name: <id>`. The spec allows "any additional files", so `asset.toml`, `README.md`,
  `CHANGELOG.md` and `evals/` can live alongside. They are not installed.
- **Rendering:** Grimoire copies the skill payload (the files that are installed) byte-for-byte
  into a skills directory each selected client reads ([`adapters.md`](adapters.md#per-client-mapping)).
  It never transforms or executes it.
- **Portability:** catalog skills use only the frontmatter fields the open spec defines (`name`,
  `description`, `license`, `compatibility`, `metadata`). Catalog validation rejects every other
  key, including client extensions (fields that only one client defines).
  - Reason: Claude Code extensions make claude.ai and the Skills API reject the skill
    (`CC-SKILL`). Other clients ignore unknown keys inconsistently.
- **Permissions:** the spec's experimental `allowed-tools` field is refused as well. It
  pre-approves tools, so the agent can use them without asking, and that approval would sit
  outside Grimoire's [permission consent](resolve.md#permission-consent). A skill that bundles
  `scripts/` must declare `shell = true`.
- **Quality bar:**
  - The description says what the skill does and when to use it.
  - `SKILL.md` is under 500 lines and roughly 5,000 tokens.
  - Every relative file reference exists and stays one level deep from `SKILL.md`.
  - Bundled scripts are pinned, linted and documented. Their language follows the language
    policy ([ADR-0010](../adr/0010-language-policy-exceptions.md), which lists the code that may
    be written in a language other than Rust): Rust by default, and another language only as an
    exception that an ADR records.

### Agent

An **agent** is a subagent definition: a named specialist that the main agent can delegate a task
to, and that then works in a session of its own. It has:

1. a description saying when to delegate to it;
2. a system prompt (the Markdown body), the standing instructions it works under;
3. an **explicit, least-privilege tool allow-list**, which names only the tools the agent needs.
   The list uses Grimoire's client-neutral capability names (`read`, `search`, `edit`, `execute`,
   `web`, `mcp:<server>[/<tool>]`), and each adapter translates them to the client's own tool
   names;
4. an optional model hint (deferred; see [`model.md`](model.md#assettoml)).

- **Source:** `catalog/agents/<id>/agent.md`, plus `[agent] tools = [...]`. `tools` is
  **required**; `[]` (no tools) is legal.
  - Reason: several clients grant *all* tools when the list is omitted (Claude Code, Gemini CLI,
    Copilot: `CC-AGENT`, `GEMINI-AGENTS`, `COPILOT-AGENTS`). Grimoire always writes the list out.
- **Rendering:** each adapter maps capabilities to the client's tool names.
  - **Grimoire never silently widens the declared tools.**
  - Where a client can't express the restriction at all, no agent is written for it:
    - Codex has no `agents` anchor (an anchor is a named destination an asset can write to;
      [`model.md`](model.md#assettoml)). The reason is that a Codex agent can't override the
      sandbox of the session that started it, the limits that session runs under
      (`CODEX-SUBAGENTS`). An asset that declares a Codex agent fails validation
      (`manifest/anchor-client`), and a project that selects Codex and such an agent gets the
      warning `resolve/client-not-covered`.
    - The `cursor` adapter refuses an agent whose tools aren't exactly `read` and `search`,
      because Cursor offers only `readonly` (`adapter/unsupported`, `CURSOR-AGENTS`).
  - Where a client enforces the restriction only partly, the adapter writes the agent and warns:
    `adapter/agent-readonly` for Cursor, `adapter/agent-strict-tools` for Copilot in VS Code.
  - See [`adapters.md`](adapters.md#per-client-mapping) and
    [`assets.md`](assets.md#content-that-reaches-the-model).
- **Quality bar:** minimal tools, explicit scope, no secrets.

### Tool

A **tool** is a standalone, non-interactive **Rust executable** built from this workspace (the
Cargo workspace: the crates in this repository that are built together). Its source is in its
asset directory. It performs **one local task to completion**. An agent runs it through its shell
tool, a skill's instructions or a hook binding (below), or a human runs it.

- It speaks no protocol beyond its command-line arguments (argv), stdin, stdout and the exit
  status. It follows standard CLI conventions: `--help`, `--version`, `--json`, data on stdout,
  diagnostics on stderr, documented exit codes.
- **Not tools:**
  - MCP servers: long-running protocol servers, a separate kind.
  - Scripts inside a skill: part of that skill, executed by the agent.
  - Third-party binaries: not built or signed by us.
- **Installing a tool asset never installs the binary**, because installing uses no network and
  executes nothing. It does three things:
  1. records the tool in the lock (`grimoire.lock`, the file where Grimoire records what it
     installed; [`model.md`](model.md#grimoirelock));
  2. renders the client wiring the tool declares. Today that is hook bindings. Optional
     permission allow-list entries, such as Claude Code's `Bash(<binary> *)`, which let the agent
     run the binary without asking, are not modeled yet. They arrive with the first tool that
     needs them;
  3. lets `grimoire doctor` check that the binary is on `PATH`. It doesn't check the binary's
     version: `doctor` never runs a binary ([`cli.md`](cli.md#commands)).

  Binaries arrive through the release channels instead. ADR-0002 made each tool and MCP server a
  separate binary, released together with the CLI.

**Hooks are not a separate kind.** A hook is a command that a client runs automatically when an
event happens, for example before each tool call. In Grimoire, a hook is a **binding declared by a
tool asset**: a client event, a matcher (for tool events, which tools it applies to) and argv.

- Adapters render it into the client's hook configuration as owned entries
  ([`install.md`](install.md#ownership-units-and-merge-strategies)).
- The command is always that tool's binary in exec form: the program and its arguments are passed
  directly, with no shell to interpret them. It is never a shell string.

Rationale:

1. Hooks run automatically on matching events with the user's privileges, and some clients run
   them without a trust prompt in non-interactive modes (`CC-SET`). Hook commands are therefore
   restricted to reviewed and tested Rust binaries, built and signed by continuous integration
   (CI). That removes a whole class of risk: an arbitrary shell one-liner in a JSON file.
2. Hook logic becomes testable.
3. An asset with hook bindings must declare the `hooks` permission, which the user must accept
   explicitly ([`resolve.md`](resolve.md#permission-consent)).

Alternatives considered:

- A `hook` kind with inline commands: rejected for v1.
- Excluding hooks entirely: the fallback, not chosen.

**Phase 3: failure behavior.** A binding may set `on-failure = "block"`. Then, when the hook
cannot run (the binary is missing, crashes, times out or prints invalid output), the client blocks
the event instead of letting it through.

- Guards such as `secret-guard` use it so that they fail closed: if the guard can't run, the
  action is stopped rather than allowed ([`assets.md`](assets.md#tool-hooks)).
- For Claude Code it is written as `onFailure: "block"`, which needs Claude Code 2.1.295 or later
  (`CC-P3`).
- The default, `continue`, is every client's own default.
- Because a missing binary then blocks the session, `plan` and `apply` warn when a fail-closed
  hook's binary is not on `PATH` (`plan/hook-binary-missing`).

### MCP server

An **MCP server** is a Rust program built on `rmcp`, the official Rust library for MCP (`RMCP`).
Its source is in its asset directory. It runs as a separate process and serves MCP tools,
resources and prompts to a client over two transports, the ways the protocol is carried:

- **stdio:** the client starts the server and talks to it through the server's stdin and stdout;
- **Streamable HTTP:** the server runs on its own and the client sends it HTTP requests.

It targets protocol **2026-07-28** (`MCP-SPEC`), and declares its permissions in `asset.toml`.

- **Installing** registers the server in each selected client's MCP configuration as an owned
  entry ([`install.md`](install.md#ownership-units-and-merge-strategies)).
  - Secrets appear only as environment-variable references in each client's syntax, such as
    `${VAR}`, never as values.
  - **The binary is never installed** (ADR-0002). Installing downloads and runs nothing, so the
    binary comes through the release channels instead ([Tool](#tool)).
- **Production bar:**
  - stdio servers log to stderr only, because stdout may carry only MCP messages (`MCP-SPEC`).
  - Strict input validation, timeouts and size limits.
  - Accurate tool annotations: the hints a server gives about each tool, such as whether it only
    reads.
  - Origin validation enabled explicitly, because rmcp disables it by default (`RMCP`). The server
    checks the `Origin` header that browsers send, so that a web page open in the user's browser
    can't call a server on the user's machine.
  - HTTP authorization follows the spec's OAuth 2.1 resource-server profile when enabled: the
    server accepts only access tokens that a separate authorization server issued for it.
  - Protocol-level integration tests.
- **Phase 3:** the bar is implemented once, in `crates/agent-grimoire-mcp`, and every server must
  serve through it ([`mcp.md`](mcp.md)). [ADR-0007](../adr/0007-mcp-server-authorization.md)
  chose how servers check authorization and made this shared crate mandatory. It provides:
  - stdio with a clean stop on end of input, `SIGINT` (Ctrl-C) or `SIGTERM` (a request to
    terminate);
  - Streamable HTTP. It is stateless: no session is kept between requests. Without authorization,
    it accepts connections only on loopback, the addresses that only this machine can reach.
    `Host` and `Origin` are checked before anything else, so that a probe learns as little as
    possible;
  - OAuth resource-server checks of JSON Web Token (JWT) access tokens against a local JSON Web
    Key Set (JWKS) file, which holds the authorization server's public keys. Using a local file
    means the server makes no outbound network connection;
  - concurrency, rate and time limits for tool work;
  - a test helper that compares a server's live `tools/list` (the MCP request that lists its
    tools) with its `asset.toml`.

  Servers speak MCP 2026-07-28 over both transports. They also speak 2025-11-25, for clients that
  still use the earlier handshake. Over HTTP:
  - the version is checked after authorization, so the body of an unauthenticated request is
    never parsed;
  - each connection carries one request and is closed within 75 seconds, so a client that stops
    reading can't hold on to a connection.

### Bundle (not an asset kind)

A **bundle**, `catalog/bundles/<id>.toml`, is a named, versioned list of asset references with
version requirements. Selecting a bundle selects its members at resolve time, when Grimoire works
out the full set of assets and their versions ([`resolve.md`](resolve.md)). The lock records which
bundle brought in which asset, so removing the bundle removes only the assets nothing else
selected. Bundle IDs share the asset ID namespace ([`model.md`](model.md#identifiers-and-versions)).

## Catalog validation

Built in M1.4 and M1.5 as described here. Later phases added checks, each marked with its phase.

`cargo xtask validate` runs these checks, and CI runs it on every pull request
([`xtask.md`](xtask.md)). `load` in this crate runs the checks that need the files on disk or the
whole catalog, and the model's single-document checks on each manifest and bundle
([`model.md`](model.md#manifests-and-configuration)). The version-bump check needs git history, so
it lives in `xtask` (`cargo xtask validate --base`), not here.

**Per asset:**

- valid against the schema, with strict fields (no unknown ones);
- `id` equals the directory name and `kind` equals the parent directory;
- every `src` exists;
- every anchor is valid for its client;
- `README.md` and `CHANGELOG.md` exist, plus `evals/` or `tests/`;
- a SemVer bump and CHANGELOG entry when files changed, compared against the merge base (the last
  commit the change shares with the branch it will merge into);
- deprecated assets name an existing replacement.

**Per kind:**

- Skills:
  - Agent Skills rules (`SKILLS`), spec-only frontmatter, and referenced files exist
    ([Agent Skills, checked natively](#agent-skills-checked-natively));
  - no `.claude-plugin/` anywhere in the skill, which would make Claude Code load the installed
    copy as a plugin, with its own hooks and MCP servers (`skill/plugin-manifest`, Phase 4).
- Agents: `tools` present and consistent with `[permissions]` (for example, `execute` requires
  `shell`).
- Tools and MCP servers: the crate builds, `binary` matches a `[[bin]]` target in its
  `Cargo.toml`, and the declared tools match the server's `tools/list` in tests.
  - Phase 3, for every asset crate:
    - no dependency on Grimoire's core crates (`crate/core-dependency`);
    - no `path` or `git` dependency (one taken from a local folder or a Git repository) in the
      asset's own `Cargo.toml` (`crate/path-dependency`), because one could bring in crates this
      check never sees. That includes `agent-grimoire-mcp`, which a server reaches through the
      workspace instead (`agent-grimoire-mcp.workspace = true`);
    - no `Cargo.toml` below the asset's own (`crate/nested-manifest`).
  - Phase 3, for MCP servers:
    - a dependency on `agent-grimoire-mcp` (`crate/hardening`);
    - no direct use of rmcp's stdio or Streamable HTTP transports in `src/`
      (`crate/raw-transport`);
    - a test that calls `agent_grimoire_mcp::testing::declared_tools_match`
      (`crate/declared-tools-test`).

    The last two are text searches that catch the usual mistakes, not proofs; code review still
    guarantees them.
- Phase 4, for every asset: `cargo xtask validate` renders it for each client it declares, through
  the real adapters. So a refusal that only an adapter makes fails validation rather than a user's
  `apply`. Examples are a Cursor agent with other tools, a comma in a Cursor or Copilot glob, and
  an MCP tool name a client would change.
- Rules:
  - no permissions;
  - duplicate and near-duplicate detection across rules;
  - every client's output uses the same `src`, because clients read each other's rule files
    (`manifest/rule-src`). Phase 4 extends this from `claude-code` and `agents-md` to every
    client.

**Catalog-wide:**

- unique IDs, across assets and bundles;
- dependencies exist, match their version requirements and are acyclic;
- bundle members exist;
- no bundle contains assets that conflict.

### What `load` checks on disk

`load` never fails. Each problem is a `Diagnostic` with a stable code, its file path relative to
the catalog root, and the list sorted by file. The `Catalog` it returns may be partial: an asset
whose `asset.toml` is missing or doesn't parse is left out, and the others are still checked.

- **Directory layout and file hygiene.** The catalog root may hold only `README.md`, `bundles/`
  and one directory per kind (`rules`, `skills`, `agents`, `tools`, `mcp-servers`). Each asset's
  directory is walked without following symbolic links (symlinks), and:
  - symlinks and special files are refused: only regular files and directories are allowed. A
    file is checked before it is opened, because opening a FIFO (a named pipe) would block;
  - every path must be a portable `RelPath`, the same on Linux, macOS and Windows, and at most
    `MAX_REL_PATH` (200) characters counted from the catalog root, such as
    `skills/<id>/references/guide.md`, because the pack stores each file under that path;
  - no two paths may differ only in letter case, such as `skill.md` and `SKILL.md`. They collide
    on macOS and Windows, where one would silently replace the other on install;
  - size limits: at most `MAX_FILE_BYTES` (1 MiB) per file, because assets ship inside the binary,
    enforced while reading and not only reported; at most 2,000 entries per asset; at most 12
    levels of nesting.
- **Required metadata files and changelog entries.** Each asset needs an `asset.toml` whose `id`
  is its directory's name and whose `kind` matches its parent directory, a non-empty `README.md`
  and `CHANGELOG.md`, and `evals/` or `tests/` with at least one file. Tools and MCP servers need
  `tests/`. The changelog needs a level-2 heading for the current version (`changelog_has`).
- **Output sources.** Every `src` file or directory an output names exists.
- **Each kind's own rules.**
  - A rule stays under 500 lines, with a warning from 200. A rule can be installed as a managed
    block in a shared file such as `AGENTS.md`, so it may not start a line with Grimoire's
    `<!-- grimoire:` marker or end inside an open code fence, either of which would break the
    block around it. It may not start with frontmatter either, which a client would read as its
    own.
  - An agent's prompt is at most 30,000 characters, GitHub Copilot's limit.
  - Skills follow the Agent Skills rules, checked natively (below).
  - Tools and MCP servers are Rust crates (ADR-0002), and follow the crate rules: for example
    exactly one binary, the asset's own version, `publish = false`, no dependency on Grimoire's
    core crates, and, for MCP servers, a dependency on the shared hardening crate
    `agent-grimoire-mcp`.
- **Dependencies, cycles, deprecations and bundles.** Ids are unique across assets and bundles.
  Each dependency exists, matches its version requirement and forms no cycle
  (`catalog/requires-missing`, `catalog/requires-version`, `catalog/requires-cycle`). A deprecated
  asset names a replacement that exists. Bundle members exist, and no bundle holds assets that
  conflict.
- **Duplicate rules.** Two rules with the same text, or nearly the same, are reported. Rules that
  contradict each other can't be found this way: they declare `conflicts` instead.

### Agent Skills, checked natively

Skills follow the Agent Skills specification, the open format for skills (`SKILLS`). This crate
checks them in Rust rather than with the specification's Python reference validator,
`skills-ref`, which describes itself as a demonstration. That also keeps Python out of the build,
under the language policy (ADR-0010). Where the specification only recommends, the checks are
deliberately stricter:

- unknown frontmatter keys are errors, and the frontmatter's YAML is read strictly: duplicate keys
  are refused at every level, and an unquoted `123`, `true` or `~` is not taken as a string;
- `allowed-tools` is refused, because it pre-approves tools outside Grimoire's permission consent;
- every bundled script must be referenced from `SKILL.md`, and a skill with scripts must declare
  the `shell` permission;
- `SKILL.md` must stay under 500 lines.

## The embedded pack and `Source`

Built in M2.5 as described here. On 2026-11-22 `load` also began refusing a catalog path longer
than the pack can store (commit `24200d4`).

A pack is a deterministic, binary-safe list of files, each stored after its length so that a
reader knows where it ends. The same catalog always gives the same bytes, so CI can check that `catalog.pack` is up to date (`cargo xtask pack-catalog --check`;
[`xtask.md`](xtask.md#generated-artifacts)). It holds only these files: each asset's `asset.toml`,
`README.md` and `CHANGELOG.md`, the files its outputs install, and every bundle. Its format,
`grimoire catalog pack v1`, isn't part of Grimoire's public API.

`pack` validates a catalog with `load` and returns every diagnostic instead of a pack when there
is an error, so an invalid catalog can never be embedded. It also fails, with `pack/read` or
`pack/limits`, when a file can't be read again or the pack would break its limits, such as 100,000
files.

`Source::from_pack` reads a pack as untrusted input:

- at most 100,000 files, each at most `MAX_FILE_BYTES`;
- every path a portable relative path from the catalog root, of at most `MAX_REL_PATH` characters,
  in strictly sorted order, so no path appears twice. `load` applies the same path rule to every
  catalog file, so a pack that `pack` writes never fails it;
- every manifest and bundle parsed strictly and checked again on its own, so a damaged pack fails
  when it is loaded rather than at install time. Each manifest's `id` and `kind` must match its
  directory, and no id may be used twice. The checks across the catalog aren't run again: `pack`
  runs them before it packs.

A malformed pack, or an invalid document in it, is a `SourceError`.

`Source` is a struct, the in-memory index of manifests, digests and files, not the trait that
ADR-0001 proposed. Phase 2 settled this: a remote index would build the same type, and a trait
with a single implementation added nothing.

```rust
// The shape of the API, as the CLI uses it (the crate docs list every item).
let source: Source = Source::builtin()?;                    // the pack built into this crate
let asset: Option<&SourceAsset> = source.asset(&id);        // manifest, digest and files
let bundle: Option<&Bundle> = source.bundle(&bundle_id);    // a bundle, by id
let every: &BTreeMap<AssetId, SourceAsset> = source.assets();
assert_eq!(source.id(), SourceId::Builtin);                 // what grimoire.lock records as `source`
```

- The `grimoire` binary calls `Source::builtin` once when it starts and passes the `Source` to
  every command. If the pack doesn't load, the CLI reports that its build is broken and exits with
  code 1. `Source::builtin` fails only if the embedded pack is corrupt, which CI prevents.
- `agent-grimoire-resolve` looks up assets and bundles in it ([`resolve.md`](resolve.md)), and
  `agent-grimoire-adapters` reads each asset's files from its `SourceAsset`
  ([`adapters.md`](adapters.md)).
- A `SourceAsset` holds the asset's manifest, the SHA-256 digest of its manifest file, which
  `grimoire.lock` also records, and its files (`SourceAsset::file`, `SourceAsset::files_under`,
  `SourceAsset::readme`, `SourceAsset::changelog`). `grimoire show` prints the latest CHANGELOG
  entry from it.
- The CLI never calls `load`. When it loads its pack, it runs only the model's single-document
  checks again and reports a failure as a broken build, without a code.

The pack lives in this crate because `cargo publish` packages only files under the crate's own
directory, so a build script that read `../../catalog` would break `cargo install`. That is also
why the CLI should be installed with `cargo install agent-grimoire --locked`: on Cargo 1.96 to
1.99, a build without `--locked` can take a later version of this crate within the same 0.y, and
so newer assets than the CLI version it reports
([`05-release.md`](../05-release.md#the-minimum-supported-rust-version)). A release also refuses
to run while the checked-in pack differs from a fresh pack of `catalog/` ([`release.md`](release.md)).

`Source` is where a later remote catalog would plug in. ADR-0001 leaves that to a future decision.
Such a source would build the same index, so nothing that reads a `Source` would change. Today
its only identity is `SourceId::Builtin`, and the lock's `source` field reads `builtin`.

## Invariants

1. `load` never follows a symbolic link and refuses every symlink and special file in the catalog.
2. Every catalog path is a portable `RelPath` of at most 200 characters from the catalog root, and
   no two paths differ only in letter case.
3. No catalog file is larger than 1 MiB, no asset has more than 2,000 entries, and nothing nests
   deeper than 12 levels; each limit is enforced while reading.
4. The catalog root holds only `README.md`, `bundles/` and the five kind directories.
5. Each asset's `id` equals its directory name, its `kind` equals its parent directory, and it has
   a non-empty `README.md` and `CHANGELOG.md`, with a changelog heading for its version, and
   `evals/` or `tests/`.
6. Every output `src` exists.
7. No two rules are the same or nearly the same text, no rule carries a Grimoire marker, an open
   code fence or frontmatter, and no rule reaches 500 lines.
8. Every skill follows the Agent Skills rules with spec-only frontmatter read as strict YAML,
   references only files that exist, mentions every bundled script, and contains no
   `.claude-plugin/`.
9. Every asset crate follows the crate rules, and every MCP server serves through
   `agent-grimoire-mcp` and tests its declared tools.
10. Ids are unique across assets and bundles; dependencies exist, match and form no cycle;
    deprecated assets name an existing replacement; bundles hold no conflicting assets.
11. Every asset renders for each client it declares, through the real adapters.
12. A changed asset has a higher version and a CHANGELOG entry for it, and a removed asset was
    deprecated first.
13. `load`'s diagnostics are sorted and render the same way every time.
14. `pack` writes the same bytes for the same catalog, and refuses an invalid one; the checked-in
    `catalog.pack` equals a fresh pack of `catalog/`.
15. `Source::from_pack` refuses a malformed pack, and the embedded pack loads.
16. Every embedded asset installs for every client it declares, and none needs a newer CLI than the
    one that embeds it.

## Tests

| Invariant | Test |
|---|---|
| 1 | `symlinks_and_unportable_names_are_rejected`, `symlinks_anywhere_in_the_catalog_are_rejected` and `special_files_are_rejected` in `crates/agent-grimoire-catalog/tests/catalog.rs` (Unix only, as [`03-platforms.md`](../03-platforms.md#invariants-on-every-platform) says) |
| 2 | `paths_from_the_catalog_root_must_fit_in_the_pack`, `case_insensitive_collisions_are_rejected` and `reserved_and_unportable_ids_are_rejected` in `tests/catalog.rs` |
| 3 | `oversized_files_are_rejected` and `nesting_and_entry_limits` in `tests/catalog.rs` |
| 4 | `layout_errors`, `layout_entries_inside_kind_and_bundle_dirs`, `empty_catalog_is_valid` and `missing_root_is_reported` in `tests/catalog.rs` |
| 5 | `id_and_kind_must_match_directories`, `metadata_files_are_required` and `non_utf8_and_empty_metadata_are_rejected` in `tests/catalog.rs`; `changelog_headings` in `src/asset.rs` |
| 6 | `sources_must_exist_and_not_be_metadata` in `tests/catalog.rs` |
| 7 | `rules_must_not_duplicate_each_other`, `near_duplicate_rules_at_seventy_percent_warn`, `rules_that_run_long_are_reported` and `rules_cannot_carry_markers_or_frontmatter` in `tests/catalog.rs`; `exact_duplicates_ignore_formatting`, `near_duplicates_are_errors`, `different_rules_pass` and `same_asset_is_not_compared_with_itself` in `src/rules.rs`; `open_fences` in `src/asset.rs` |
| 8 | `skill_frontmatter_rules`, `more_skill_frontmatter_rules`, `skill_references_and_payload`, `script_documentation_and_mentions`, `skill_link_forms` and `skills_may_not_contain_a_plugin_manifest` in `tests/catalog.rs`; `strict_parsing` in `src/yaml.rs`; `finds_links_and_mentions`, `html_attribute_values`, `percent_decoding` and `splits_frontmatter` in `src/skill.rs` |
| 9 | `code_assets_are_checked_as_crates`, `crate_rules_for_code_assets`, `asset_crates_cannot_hide_dependencies`, `servers_use_the_hardening_crate_and_no_asset_uses_core_crates` and `servers_serve_through_the_hardening_crate_and_test_their_declared_tools` in `tests/catalog.rs` |
| 10 | `dependencies_are_resolved_catalog_wide`, `conflicts_across_dependency_closures`, `duplicate_ids_across_kinds_are_rejected`, `bundle_rules`, `bundles_parse_strictly_and_ids_are_unique`, `deprecations_need_an_existing_replacement`, `deprecated_replacements_are_warned_about`, `deprecation_and_unknown_conflict_warnings`, `deprecated_dependency_tables_are_read_too` and `agent_mcp_references_must_resolve` in `tests/catalog.rs` |
| 11 | `adapter_refusals_fail_validation` in `xtask/src/render.rs` |
| 12 | `content_change_requires_a_bump`, `untracked_content_counts_as_a_change`, `metadata_changes_need_no_bump`, `versions_must_not_go_down`, `removal_requires_prior_deprecation`, `renaming_an_asset_counts_as_removing_it`, `moving_a_file_between_assets_bumps_both`, `removing_a_deprecated_asset_is_allowed` and `bundle_changes_need_a_bump_and_removal_warns` in `xtask/src/bumps.rs`; `cargo xtask validate --base origin/main` in CI |
| 13 | `diagnostics_render_stably` and `diagnostics_are_sorted_across_phases` in `tests/catalog.rs` |
| 14 | `round_trip_and_rejections` and `encode_and_decode_agree_on_path_length` in `src/pack.rs`; `cargo xtask pack-catalog --check` in CI |
| 15 | `round_trip_and_rejections` in `src/pack.rs`; `the_embedded_pack_loads` in `src/source.rs`; the crate docs' examples (doc tests) |
| 16 | `every_exemplar_installs_for_every_client`, `every_exemplar_installs_verifies_and_uninstalls`, `no_embedded_asset_needs_a_newer_grimoire` and `the_catalog_holds_one_exemplar_per_kind_and_two_bundles` in `crates/agent-grimoire/tests/builtin.rs` |

`valid_catalog_has_no_diagnostics` and `manifest_rules_surface_through_the_catalog` in
`tests/catalog.rs` check that a valid fixture catalog passes and that the model's checks reach
catalog authors through `load`. The fixture catalogs come from `agent-grimoire-testkit`
([`04-quality.md`](../04-quality.md#test-tools)).
