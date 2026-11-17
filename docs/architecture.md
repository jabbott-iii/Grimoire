# Grimoire architecture

- **Status:** Draft for Phase 0 review. Everything marked **Proposal** needs maintainer approval before Phase 1.
- **Date:** 2026-11-16

Spec facts are cited by their IDs in [`specs.md`](specs.md), e.g. `CC-MEM`. Decisions with lasting consequences are in [`adr/`](adr/).

## 1. What Grimoire is

Grimoire has three parts:

1. **A catalog** of AI-agent assets under `catalog/`. Each asset has a strict manifest.
2. **A Rust CLI, `grimoire`.** It installs only the assets a user selects into the user's repository, rendered in each chosen client's native layout. It tracks everything it writes so it can update, detect drift on, and remove exactly that content.
3. **Generated distribution artifacts.** These include JSON Schemas and a Claude Code plugin marketplace. All of them are derived from the catalog and the Rust types, never maintained by hand.

## 2. Glossary

| Term | Meaning |
|---|---|
| **Asset** | One versioned catalog entry: a manifest (`asset.toml`) plus its content files. |
| **Kind** | One of `rule`, `skill`, `agent`, `tool`, `mcp-server` (§3). |
| **Bundle** | A named, versioned list of asset references. It is not an asset kind (§3.6). |
| **Client** | An AI coding tool Grimoire renders for, e.g. `claude-code`, `codex`, `cursor`, `gemini-cli`, `copilot`, and the portable `agents-md` target. |
| **Adapter** | The code that knows one client's layout. It turns a resolved asset into **outputs**. |
| **Output** | One unit of content Grimoire owns in the user's repo: a whole file, a managed block in a shared Markdown file, or an entry in a JSON/TOML config file (§7.2). |
| **Source** | Where catalog data comes from. v1 has only `builtin`, the catalog embedded in the binary ([ADR-0001](adr/0001-catalog-delivery.md)). |
| **Project root** | The directory containing `grimoire.toml`. Grimoire never writes outside it. |

## 3. Asset kinds (Proposal: please confirm these definitions)

The kinds are separated by **how the content reaches the model** and **what it can do**. That separation is also what the security model reasons about.

| Kind | Reaches the model… | Can execute? | Installed by Grimoire as… |
|---|---|---|---|
| Rule | automatically, always or when matching files are in play | no | rendered instruction files or managed blocks |
| Skill | on demand, when the model picks it by its description | the agent may run bundled scripts | a byte-for-byte copy of the skill directory |
| Agent | as a delegated sub-session with its own prompt and tools | through the tools it is granted | a rendered subagent definition |
| Tool | not at all; it is invoked as a program | yes, a Rust binary | client wiring only (hook bindings, permission entries). The binary is not installed |
| MCP server | through MCP tool, resource and prompt listings | yes, a Rust server process | a client MCP-config entry. The binary is not installed |

### 3.1 Rule

A **rule** is a Markdown instruction document that a client loads into the model's context **without being invoked**. It loads either always or when files matching declared globs are in play. It carries conventions, constraints and procedures. It has no executable content, grants no tools, and carries no client-specific frontmatter. Grimoire generates each client's frontmatter from the manifest.

- **Source:** `catalog/rules/<id>/rule.md`, plus `asset.toml` with `[rule] paths = [...]`. An absent `paths` means always-on.
- **Rendering:** a per-rule file wherever the client has one; otherwise a managed block in the client's shared instruction file. See §8.
  - Per-rule files: Claude Code `.claude/rules/`, Cursor `.cursor/rules/*.mdc`, Copilot `.github/instructions/*.instructions.md`.
  - Shared files: `AGENTS.md`, `GEMINI.md`, `.github/copilot-instructions.md`.
- **Clients without path scoping** (`AGENTS.md`, `GEMINI.md`): a path-scoped rule is rendered with an explicit natural-language scope line ("Applies when working on files matching …").
- **Not a rule:** content the model should load only when relevant to a task (that is a skill), or a persona with its own tools (that is an agent).
- **Quality bar:** concise, scoped, actionable. Catalog lints reject exact or near-duplicate rules and flag declared conflicts between rules that can be installed together (§6.4).

### 3.2 Skill

A **skill** is a directory that conforms to the Agent Skills specification (`SKILLS`). It contains a `SKILL.md` with spec-valid `name`/`description` frontmatter and a Markdown body, plus optional `scripts/`, `references/` and `assets/`. Clients show only the name and description until the model activates the skill (progressive disclosure).

- **Source:** the asset directory *is* the skill directory: `catalog/skills/<id>/SKILL.md`, with `name: <id>`. The spec allows "any additional files", so `asset.toml`, `README.md`, `CHANGELOG.md` and `evals/` can live alongside. They are not installed.
- **Rendering:** Grimoire copies the skill payload byte-for-byte into a skills directory each selected client reads (§8). It never transforms or executes it.
- **Portability:** catalog skills use only open-spec frontmatter fields (`name`, `description`, `license`, `compatibility`, `metadata`). Client extensions are rejected by the catalog lint.
- **Permissions:** the spec's experimental `allowed-tools` is refused as well, because it pre-approves tools outside Grimoire's permission consent (§6.3). A skill that bundles `scripts/` must declare `shell = true`.
  - Reason: Claude Code extensions make claude.ai and the Skills API reject the skill (`CC-SKILL`). Other clients ignore unknown keys inconsistently.
- **Quality bar:**
  - The description says what the skill does and when to use it.
  - `SKILL.md` is under 500 lines and roughly 5,000 tokens.
  - Every relative file reference exists and stays one level deep.
  - Bundled scripts are pinned, linted and documented. Their language exceptions are governed by the language policy.

### 3.3 Agent

An **agent** is a subagent definition, a named and delegated specialist with:

1. a description saying when to delegate to it;
2. a system prompt (the Markdown body);
3. an **explicit, least-privilege tool allow-list** in Grimoire's client-neutral capability vocabulary (`read`, `search`, `edit`, `execute`, `web`, `mcp:<server>[/<tool>]`);
4. an optional model hint (deferred; see §6.1).

- **Source:** `catalog/agents/<id>/agent.md`, plus `[agent] tools = [...]`. `tools` is **required**; `[]` is legal.
  - Reason: several clients grant *all* tools when the list is omitted (Claude Code, Gemini CLI, Copilot: `CC-AGENT`, `GEMINI-AGENTS`, `COPILOT-AGENTS`). Grimoire always writes the list out.
- **Rendering:** each adapter maps capabilities to the client's tool names.
  - **If a client can't enforce the declared restriction, the adapter refuses.** It never silently widens. Examples: Cursor offers only `readonly`; Codex agents can't override the parent sandbox (`CURSOR-AGENTS`, `CODEX-SUBAGENTS`). See §8.3.
- **Quality bar:** minimal tools, explicit scope, no secrets.

### 3.4 Tool (Proposal: definition needs your approval)

A **tool** is a standalone, non-interactive **Rust executable** built from this workspace, with its source in its asset directory. An agent invokes it through its shell tool, a skill's instructions or a hook binding, or a human invokes it, to perform **one local task to completion**.

- It speaks no protocol beyond argv, stdin, stdout and the exit status. It follows standard CLI conventions: `--help`, `--version`, `--json`, data on stdout, diagnostics on stderr, documented exit codes.
- **Not tools:**
  - MCP servers: long-running protocol servers, a separate kind.
  - Scripts inside a skill: part of that skill, executed by the agent.
  - Third-party binaries: not built or signed by us.
- **Installing a tool asset never installs the binary**, because there is no network and no execution at install time. It does three things:
  1. records the tool in the lock;
  2. renders the client wiring the tool declares. Today that is hook bindings. Optional permission allow-list entries (such as Claude Code `Bash(<binary> *)`) are not modelled yet, and arrive with the first tool that needs them;
  3. lets `grimoire doctor` check that the binary is on `PATH` at a compatible version.

  Binaries arrive through the release channels ([ADR-0002](adr/0002-mcp-server-and-tool-packaging.md)).

**Hooks (Proposal): hooks are not a separate kind.** A hook is a **binding declared by a tool asset**: client event + matcher + argv. Adapters render it into the client's hook configuration as owned entries (§7.2). The command is always that tool's binary in exec form; it is never a shell string.

Rationale:
1. Hooks run automatically on matching events with the user's privileges, and some clients run them without a trust prompt in non-interactive modes (`CC-SET`). Restricting them to reviewed, tested, CI-built, signed Rust binaries removes the "arbitrary shell one-liner in a JSON file" risk class.
2. Hook logic becomes testable.
3. An asset with hook bindings must declare the `hooks` permission, which the user must accept explicitly (§6.3).

Alternatives considered:
- A `hook` kind with inline commands: rejected for v1.
- Excluding hooks entirely: viable fallback if you prefer.

### 3.5 MCP server

An **MCP server** is a Rust program built on `rmcp` (`RMCP`), with its source in its asset directory. It runs as a separate process and serves MCP tools, resources and prompts over **stdio** and **Streamable HTTP**, targeting protocol **2026-07-28** (`MCP-SPEC`). It declares its permissions in `asset.toml`.

- **Installing** registers the server in each selected client's MCP configuration as an owned entry (§7.2). Secrets appear only as env-var references in each client's syntax. **The binary is never installed** ([ADR-0002](adr/0002-mcp-server-and-tool-packaging.md)).
- **Production bar:**
  - stdio servers log to stderr only.
  - Strict input validation, timeouts and size limits.
  - Accurate tool annotations.
  - Origin validation enabled explicitly, because rmcp disables it by default (`RMCP`).
  - HTTP auth follows the spec's OAuth 2.1 resource-server profile when enabled.
  - Protocol-level integration tests.

### 3.6 Bundle (not an asset kind)

A **bundle**, `catalog/bundles/<id>.toml`, is a named, versioned list of asset references with version requirements. Selecting a bundle expands to its members at resolve time. The lock records which bundle brought in which asset, so removing the bundle removes only assets nothing else selected.

## 4. Repository layout (Proposal: refines the brief)

```text
Cargo.toml                 workspace; members = ["crates/*", "xtask"] (+ "catalog/tools/*", "catalog/mcp-servers/*" from Phase 3)
Cargo.lock
deny.toml                  cargo-deny policy
crates/
  grimoire-model/          types for asset.toml, bundles, grimoire.toml, grimoire.lock; strict serde + schemars
  grimoire-catalog/        Source trait, builtin (embedded) source, catalog-wide lints
  grimoire-resolve/        selection (assets/bundles/tags), dependency/conflict/client/CLI-version resolution
  grimoire-install/        root-confined fs, atomic writes, outputs/ownership, planner, applier, drift, lockfile I/O
  grimoire-adapters/       Adapter trait + one module per client; insta snapshot tests
  grimoire-cli/            the `grimoire` binary (published package name: see ADR-0004)
xtask/                     `cargo xtask …`: new, validate, schema, pack-catalog, gen-marketplace (publish = false)
catalog/
  rules/<id>/              asset.toml, rule.md, README.md, CHANGELOG.md, evals/
  skills/<id>/             asset.toml, SKILL.md, scripts/ references/ assets/, README.md, CHANGELOG.md, evals/
  agents/<id>/             asset.toml, agent.md, README.md, CHANGELOG.md, evals/
  tools/<id>/              asset.toml, Cargo.toml, src/, tests/, README.md, CHANGELOG.md
  mcp-servers/<id>/        asset.toml, Cargo.toml, src/, tests/, README.md, CHANGELOG.md
  bundles/<id>.toml
schemas/                   generated JSON Schemas (checked in; CI fails if stale)
.claude-plugin/marketplace.json + plugins/claude-code/…   generated (Phase 4; checked in; CI fails if stale)
docs/  docs/adr/
.github/                   workflows, templates, CODEOWNERS, dependabot.yml
Dockerfile
CLAUDE.md  AGENTS.md       conventions for this repo's own contributors and agents
```

Crate directory names above are placeholders. Final package names, and the matching directory names, follow [ADR-0004](adr/0004-package-and-binary-names.md), which proposes `agent-grimoire-*`.

**Refinement 1: code-bearing assets live under `catalog/`, not under `crates/`.**

The brief asks for two things at once: "one crate per MCP server and per tool" in `crates/`, and that "a new asset can be added with files under `catalog/` only" with "no core code changes". Both can't hold if server crates sit in `crates/`. Making each tool and MCP-server asset directory a workspace crate, picked up by the `catalog/*/*` member globs, satisfies both. Cargo rejects a member glob that matches nothing (`CARGO-LINTS` in `specs.md`), so the globs are added together with the first code-bearing asset, in Phase 3:

- adding a server is one new directory;
- `crates/` stays "core".

A shared hardening library for servers (transport setup, Origin validation, auth layer, limits) will be introduced in `crates/` with the first server in Phase 3. It is core code that new servers *use*, not code they modify.

**Refinement 2: adapters are one crate with a module per client.** They share the output model and test harness. A crate per client would add publish overhead (see ADR-0004) for no isolation benefit.

**Refinement 3: generated files are checked in.** This covers schemas, the packed catalog and the marketplace. Each generator has a `--check` mode that CI runs, so "generated, never hand-maintained" is enforced mechanically.

**Dependency direction.** `model` ← `catalog` ← `resolve`, then `model` + `install` ← `adapters` (adapters construct install's output values), then everything ← `cli`. Tool and server crates depend on none of the core crates.

## 5. Identifiers and versions

- **Asset IDs are unique across the whole catalog**, not just per kind. That gives simple CLI UX (`grimoire add rust-errors`), unambiguous lock keys, and one-to-one plugin names in the generated marketplace.
- **Format:** the Agent Skills `name` rules for every kind. 1–64 chars, `[a-z0-9-]`, no leading, trailing or double hyphen, and equal to the directory name.
  - IDs starting with the plugin-reserved prefixes `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-` are also rejected, because generated Claude Code plugins would fail to load (`CC-PLUGIN`).
- **Asset versions are SemVer, one per asset.** CI requires a version bump and a CHANGELOG entry whenever an asset's files change. The asset version is independent of the CLI version.
- **Manifests carry `min-cli-version`.** Resolution fails clearly if the running CLI is older.

## 6. Manifests and configuration (Proposal)

All three files are TOML, parsed with unknown fields as errors. JSON Schemas for them are generated from the Rust types into `schemas/`.

### 6.1 `asset.toml`

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

[rule]                             # exactly one kind table, matching `kind`
paths = ["**/*.rs"]                # omit for always-on

[clients.claude-code]              # presence of a table = supported client
outputs = [{ src = "rule.md", to = "rules:rust-errors.md", strategy = "render" }]
[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
```

- **Kind tables:**
  - `[skill]` (no required keys);
  - `[agent] tools = [...]`. A `model` hint is deferred until an exemplar needs one, so agents currently inherit the session model;
  - `[tool] binary = "…"` plus optional `hooks = [...]` bindings;
  - `[mcp-server] binary = "…", transports = ["stdio","streamable-http"], env = [...]`, plus the declared tool list with annotations, which tests compare against the live server.
- **Explicit output map per client (Proposal: refines "explicit file map").** Every client an asset supports lists its outputs explicitly: source file → destination → strategy. A reviewer can see exactly what each asset writes for each client.
  - The destination is written as an **anchor** (`rules:`, `skills:`, `agents:`, `instructions`, `mcp`, `hooks`) plus a relative name, rather than a literal path. Each adapter resolves its anchors.
  - Reason 1: a client changing its layout becomes one adapter change instead of an edit to every manifest.
  - Reason 2: the planner can choose a location that several selected clients share (§8.2).
  - The resolved literal paths are shown by `plan` and recorded in the lock.
  - Validation rejects any destination outside the adapter's declared anchor set. An asset therefore *cannot* write, for example, `.github/workflows/x.yml`.
  - `cargo xtask new` scaffolds these tables. If you prefer literal paths, the cost is that layout changes become catalog-wide edits.
- **Strategies:**
  - `copy`: a whole file, byte-for-byte.
  - `render`: a whole file generated by the adapter, e.g. with client frontmatter.
  - `block`: a managed block in a shared Markdown file.
  - `entry`: an owned entry in a JSON or TOML config file.

### 6.2 `grimoire.toml` (user repo)

```toml
format = 1
clients = ["claude-code", "agents-md"]
bundles = ["rust-cli"]
tags = []                          # select every asset carrying these tags
exclude = []                       # never select these, even via bundle/tag

[assets]
rust-errors = "^1"
changelog-writer = { version = "^0.3", clients = ["claude-code"], allow = ["shell"] }
github-insights = { version = "^1", allow = ["network", "env"], env = { GITHUB_TOKEN = "GH_TOKEN" } }
```

Per-asset overrides in v1:
- a subset of clients;
- **`allow`**: accepted permissions (§6.3);
- for MCP servers, an env-var *name* remapping and the transport.

The ADR-0002 override `runtime = "docker"` is added in Phase 6, together with the image it points at. Until then `grimoire.toml` rejects it as an unknown field.

Nothing that could alter destinations outside the anchor rules is overridable.

### 6.3 Permission consent

Declared `[permissions]` are not just documentation. Resolution fails if an asset needs a permission that is not in its `allow` list.

- `grimoire add` shows the permissions and asks interactively, or requires `--allow …` non-interactively.
- If a later version of an asset *expands* its permissions, `update`/`apply` stop and ask again. There is no silent escalation.
- `rule` assets are pure text and must declare no permissions.

### 6.4 Catalog validation (`cargo xtask validate`, also a CI gate)

**Per asset:**
- schema validity and strict fields;
- `id` equals the directory name and `kind` equals the parent directory;
- every `src` exists;
- every anchor is valid for its client;
- `README.md` and `CHANGELOG.md` exist, plus `evals/` or `tests/`;
- a SemVer bump and CHANGELOG entry when files changed (compared against the merge base);
- deprecated assets name an existing replacement.

**Per kind:**
- Skills: Agent Skills rules (`SKILLS`), spec-only frontmatter, referenced files exist.
- Agents: `tools` present and consistent with `[permissions]` (e.g. `execute` requires `shell`).
- Tools and MCP servers: the crate builds, `binary` matches a `[[bin]]`, and the declared tools match the server's `tools/list` in tests.
- Rules: no permissions, plus duplicate and near-duplicate detection across rules.

**Catalog-wide:**
- unique IDs;
- dependencies exist and are acyclic;
- bundle members exist;
- no bundle contains assets that conflict.

### 6.5 `grimoire.lock`

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

The lock is sorted deterministically and uses normalized `/` paths, so it diffs cleanly and is identical across operating systems.

## 7. Install model

### 7.1 Desired state, plan, apply

1. **Resolve:** `grimoire.toml` + lock + catalog become the resolved asset set. Lock pins are kept unless `update` asks otherwise.
2. **Render:** adapters turn the resolved set and the selected clients into the desired outputs.
3. **Plan:** compare the desired outputs, the lock and disk. The result is a list of `create | update | delete | unchanged | conflict` actions, each with a reason and a diff.
4. **Apply:**
   1. Check every precondition first. Any conflict means nothing is written (exit 4) unless `--force` or `--write-new` is given.
   2. Write each output atomically.
   3. Write the lock last.

**Idempotence:** a second `apply` produces an all-`unchanged` plan and writes nothing, not even the lock.

**Crash safety:** a unit whose on-disk hash already equals the *desired* hash counts as applied. Re-running after an interruption therefore converges, without false drift.

### 7.2 Ownership units and merge strategies

| Unit | Used for | What Grimoire owns | Drift check |
|---|---|---|---|
| `file` | rule and agent files, skill payload files | the whole file | file hash vs lock |
| `block` | `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.github/copilot-instructions.md` | the text between `<!-- grimoire:begin <id> -->` and `<!-- grimoire:end <id> -->` | block-content hash vs lock |
| `entry` | `.mcp.json` `mcpServers.<id>`, `.cursor/mcp.json`, `.gemini/settings.json`, `.codex/config.toml` `[mcp_servers.<id>]`, hook arrays | one key (by JSON-Pointer-style path) or one array element (by value) | canonical-value hash vs lock |

Details:
- **Blocks** sit in a deterministic order. User text outside blocks is never touched. HTML comments are stripped by Claude Code before injection, so markers cost no context there (`CC-MEM`).
- **Entries** use format-preserving editing: `toml_edit` for TOML, and a comment- and order-preserving JSON/JSONC editor for JSON (choice in Phase 2). Unrelated keys and the user's formatting are preserved. Property tests cover merge, unmerge, and round-trips with arbitrary surrounding content.
- **Removal** deletes only owned units. Grimoire deletes a file it created only when the file is now empty of anything Grimoire didn't write. Otherwise it removes its blocks or entries and leaves the file. Directories Grimoire created are removed if empty.

### 7.3 Drift and conflicts

For each output, the planner compares three hashes: on-disk (D), locked (L) and desired (N).

| Situation | Action |
|---|---|
| D = L, N = L | unchanged |
| D = L, N ≠ L | update |
| D = N | already applied: record it, no write |
| D ≠ L, D ≠ N | **conflict**: the user edited managed content. Never overwritten silently. |
| owned unit missing on disk | **conflict** (deleted by the user) |
| destination exists but is not in the lock | **conflict** (an unmanaged file is in the way) |

A conflict is reported with a diff. `--force` overwrites. `--write-new` writes the would-be file as `<path>.new` (for blocks and entries, the whole file as it would be written) and leaves the original alone.

### 7.4 Filesystem safety

- **Paths:** every destination is a normalized **ASCII** relative path under the project root (`RelPath`). ASCII-only rules out invisible and bidi characters, superscript device names and Unicode-normalization collisions. These are rejected: absolute paths, `..`, drive and UNC prefixes, NUL bytes, Windows reserved names and alternate data streams (`:`), and names that collide case-insensitively within one plan. The catalog validator applies the same rules to every catalog file (Phase 1).
- **Confinement:** all filesystem access goes through a capability handle on the project root (`cap-std`). The handle refuses escapes, including through symlinks.
- **Symlinks:** Grimoire never writes through a symlink at a managed path. It refuses (exit 6) rather than follow one.
- **Atomic writes:** a temp file in the same directory, fsync, then rename over the target. Behaviour differences on Windows are covered by tests.
- **No hooks, no execution, no network:** installing never executes asset content and runs no install hooks. The install path has no network code at all in v1 (the builtin source).

### 7.5 Resolution errors

Each of these is a distinct, actionable diagnostic (exit 3):

- unknown asset;
- missing dependency;
- unsatisfiable version requirement;
- conflict between selected assets (naming who selected each one);
- unsupported client for an asset (naming the clients it supports);
- `min-cli-version` too new;
- unaccepted permissions;
- an agent whose tool restriction a selected client can't enforce;
- a pinned version unavailable in this CLI's catalog when content is needed (ADR-0001).

## 8. Client adapters

### 8.1 Adapter contract

An adapter is pure. Given resolved assets and the client set, it returns outputs. It declares:
- its client ID;
- its anchors, with each anchor's **read locations** in preference order;
- the strategies it supports per kind;
- its capability→tool mapping;
- its env-var reference syntax.

Snapshot tests (`insta`) cover every adapter × kind × representative asset.

### 8.2 Client read matrix and cross-client planning

Clients read each other's files (2026-11-16, from `specs.md`). **✓** means documented as read; **–** means not documented as read.

| Location | Claude Code | Codex | Cursor | Gemini CLI | Copilot |
|---|---|---|---|---|---|
| `AGENTS.md` | only if no `CLAUDE.md`¹ | ✓ (32 KiB budget) | ✓ | only via `context.fileName` | ✓ cloud agent, CLI, VS Code |
| `CLAUDE.md` | ✓ | – | ✓ | – | ✓ cloud agent, CLI |
| `GEMINI.md` | – | – | – | ✓ | ✓ cloud agent, CLI |
| `.claude/rules/*.md` | ✓ (`paths`) | – | – | – | VS Code Claude harness |
| `.cursor/rules/*.mdc` | – | – | ✓ | – | – |
| `.github/instructions/*.instructions.md` | – | – | – | – | ✓ |
| `.claude/skills/` | ✓ | – | ✓ | – | ✓ |
| `.agents/skills/` | – | ✓ | ✓ | ✓ | ✓ |
| `.claude/agents/` | ✓ | – | ✓ (as `readonly` only) | – | ✓ VS Code |
| `.mcp.json` | ✓ | – | – | – | ✓ CLI, VS Code |
| `.claude/settings.json` hooks | ✓ | – | ✓ | – | ✓ |

¹ In the default `claude-md-or-agents-md` mode (`CC-MEM`).

Consequences:

1. **Plan over the selected set, not per client.** For each asset and anchor, the planner chooses the smallest set of locations that reaches every selected client exactly once, using a deterministic greedy cover with a fixed priority order. Shared outputs are written once and attributed to several clients in the lock. Examples:
   - `{claude-code, cursor, copilot}` gets skills only in `.claude/skills/`.
   - `{codex, gemini-cli, cursor}` gets skills only in `.agents/skills/`.
   - Unavoidable duplicates are reported by `plan`. Example: all six clients need both skill directories, so Cursor and Copilot see each skill twice.
2. **Rules with Claude Code + `agents-md` (Proposal for Phase 2, to be confirmed by tests).**
   - Always-on rules live once in `AGENTS.md`. A managed block in `CLAUDE.md` holds `@AGENTS.md`, which Claude Code documents as never double-loading. Claude then reads them regardless of whether a user later adds their own `CLAUDE.md`.
   - Path-scoped rules go to `.claude/rules/<id>.md` with `paths`.
   - Risk to test: Copilot CLI expands `@` imports in `CLAUDE.md`, which may double-load there.
3. **Permissions can leak across clients.** Cursor reads `.claude/agents/` without tool restrictions, and Cursor and Copilot also execute hooks from `.claude/settings.json`. Installing for one client can therefore grant something in another, even an *unselected* one. `plan` prints a warning for these cases, and `docs/security-model.md` covers them.

### 8.3 Per-client mapping (v1 proposal; client order per [ADR-0005](adr/0005-client-priority.md))

| Kind | `claude-code` | `agents-md` (portable) | `codex` | `cursor` | `gemini-cli` | `copilot` |
|---|---|---|---|---|---|---|
| Rule, always-on | see §8.2 (2) | `AGENTS.md` block | `AGENTS.md` (via agents-md) | `.cursor/rules/<id>.mdc` `alwaysApply` | `GEMINI.md` block | `.github/instructions/<id>.instructions.md` `applyTo: "**"` |
| Rule, path-scoped | `.claude/rules/<id>.md` `paths` | `AGENTS.md` block + scope line | same | `.mdc` `globs` | `GEMINI.md` block + scope line | `.instructions.md` `applyTo` |
| Skill | `.claude/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` | shared (§8.2) | `.agents/skills/<id>/` | shared (§8.2) |
| Agent | `.claude/agents/<id>.md` with explicit `tools` | – | `.codex/agents/<id>.toml`, only if the restriction is enforceable | `.cursor/agents/<id>.md`, only for `readonly`-expressible agents | `.gemini/agents/<id>.md` (preview) | `.github/agents/<id>.agent.md` with explicit `tools` |
| MCP server | `.mcp.json` entry, `${VAR}` | – | `.codex/config.toml` entry, `env_vars` / `bearer_token_env_var` | `.cursor/mcp.json` entry, `${env:VAR}` | `.gemini/settings.json` entry | `.mcp.json` entry (shared with Claude Code); cloud agent: guidance only |
| Tool hooks | `.claude/settings.json` `hooks` | – | `.codex/hooks.json` | `.cursor/hooks.json` | `.gemini/settings.json` `hooks` | `.github/hooks/grimoire.json` |

Project-scoped Codex config, rules and hooks load only in **trusted** projects (`CODEX-CONFIG`). `doctor` reports this, because Grimoire can't grant trust.

## 9. CLI (Proposal)

### 9.1 Commands

| Command | Purpose | Writes |
|---|---|---|
| `grimoire init [--client …]` | Create `grimoire.toml`; optionally detect clients already in use | `grimoire.toml` |
| `grimoire list [QUERY] [--kind] [--tag] [--client] [--maturity]` | Browse or search the catalog (list and search in one command) | — |
| `grimoire show <id>` | Manifest, permissions, per-client outputs, dependencies, changelog head | — |
| `grimoire add <id\|bundle:…\|tag:…>… [--allow …]` | Add to `grimoire.toml` after resolving and consenting to permissions | `grimoire.toml` |
| `grimoire remove <id>…` | Remove from `grimoire.toml` | `grimoire.toml` |
| `grimoire plan [--check]` | Show what `apply` would do, with diffs. `--check` exits 5 if there are changes | — |
| `grimoire apply [--force] [--write-new]` | Make the managed content on disk match `grimoire.toml` + lock | managed outputs, `grimoire.lock` |
| `grimoire update [<id>…] [--no-apply]` | Re-resolve to the newest versions the requirements allow and update the lock, then apply (`--no-apply` updates the lock only) | `grimoire.lock`, managed outputs |
| `grimoire verify` | Check every locked output on disk against its hash (a CI gate; exit 5 on mismatch) | — |
| `grimoire doctor` | Environment report: clients detected, tool and server binaries on `PATH` and their versions, Codex trust, config parse errors | — |
| `grimoire uninstall [--purge]` | Remove all managed content and the lock (`--purge` also removes `grimoire.toml`) | managed outputs, lock |

`add`/`remove` only change the desired state. `apply` reconciles, which keeps CI flows explicit. `add --apply` is a convenience.

### 9.2 Machine use

- `--json` on every command puts exactly one JSON document on stdout: `{ "format": 1, "command": …, "ok": …, "result": …, "diagnostics": [ … ] }`. Human diagnostics go to stderr.
- `--no-input` disables prompts. It is implied when stdin is not a TTY or `CI` is set. A prompt that would be needed becomes an error that names the flag to pass.
- `-C <dir>` sets the project root.
- Output is deterministic: stable ordering, and no timestamps in written files.

### 9.3 Exit codes

| Code | Meaning |
|---|---|
| 0 | Success, or nothing to do |
| 1 | Unexpected failure (I/O, internal error) |
| 2 | Usage error (bad flags or arguments) |
| 3 | Invalid configuration or manifest, or a resolution failure (§7.5) |
| 4 | Conflict: drift or an unmanaged file is in the way. Nothing was written |
| 5 | Check failed: `plan --check` found changes, `verify` found mismatches, or `doctor` found problems |
| 6 | Refused for safety (path escape, symlink at a managed path) |
| 130 | Interrupted |

## 10. Generated artifacts

| Artifact | Generator | CI check |
|---|---|---|
| `schemas/*.schema.json` (asset, bundle, config, lock) | `cargo xtask schema` (schemars) | `--check` |
| Packed builtin catalog | `cargo xtask pack-catalog` | `--check` (ADR-0001) |
| `.claude-plugin/marketplace.json` + `plugins/claude-code/<name>/` | `cargo xtask gen-marketplace` (Phase 4) | `--check`, and also `claude plugin validate --strict` if the maintainer approves installing Claude Code in CI |

Marketplace notes, verified against `CC-PLUGIN` and `CC-MKT`:
- Plugins can carry skills, agents, hooks and MCP servers, but **not rules** (a plugin's root `CLAUDE.md` is not loaded).
- Plugin MCP servers still need the server binary on `PATH`.
- Codex also reads `.claude-plugin/marketplace.json` as a legacy location (`CODEX-PLUGINS`). Compatibility is unverified and out of v1 scope.
- Plugin granularity is decided in Phase 4. The expected shape is one plugin per eligible asset plus one per bundle that depends on its members.

## 11. Open design issues (to settle in the phase noted)

1. **JSON/JSONC editing** (Phase 2): which crate preserves comments and order for `.mcp.json`, `.cursor/mcp.json` and `settings.json`, or whether to write our own minimal editor. Needs evaluation and possibly an ADR.
2. **Cross-client planning priorities** (Phase 2/4): the fixed priority order for the greedy cover, and how `plan` presents duplicates. Needs snapshot tests per client combination.
3. **`.claude/rules/` vs a `CLAUDE.md` block** for Claude Code always-on rules when `agents-md` isn't selected (Phase 2).
4. **Server-side MCP auth** (Phase 3): rmcp provides client-side OAuth only (`RMCP`). Grimoire needs a resource-server layer covering RFC 9728 metadata, `WWW-Authenticate`, and audience/issuer validation. Choose a JWT/introspection approach. Needs an ADR.
5. **Codex AGENTS.md budget** (Phase 4): warn in `plan` when managed blocks push `AGENTS.md` near Codex's 32 KiB default.
6. **Copilot cloud-agent MCP** can't be configured from repo files. Decide whether `doctor` emits the settings JSON for the user to paste (Phase 4).
