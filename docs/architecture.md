# Grimoire architecture

- **Status:** Every Phase 0 proposal was approved (2026-11-16). Notes marked with a phase ("Phase 2" to "Phase 7") record what implementation settled, refined or added. Phase 7 (2026-11-17) removed the proposal labels. The user reference for the CLI is [`cli.md`](cli.md) and [`diagnostics.md`](diagnostics.md) (§9).
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

## 3. Asset kinds

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
  - Shared files: `AGENTS.md`, `GEMINI.md`.
  - Phase 4 exception ([ADR-0008](adr/0008-phase-4-client-adapters.md)): when a rule is in `AGENTS.md`, Claude Code, Cursor, Gemini CLI and Copilot read it from there and get no per-rule file or block of their own (§8.2).
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
  - **Grimoire never silently widens the declared tools.** Where a client can't express the restriction at all, no agent is written for it: Codex has no `agents` anchor, because its agents can't override the parent sandbox (`manifest/anchor-client`, and `resolve/client-not-covered` in a project that selects it), and the `cursor` adapter refuses an agent whose tools aren't exactly `read` and `search`, because Cursor offers only `readonly` (`adapter/unsupported`). Where a client enforces the restriction only partly, the adapter writes the agent and warns: `adapter/agent-readonly` for Cursor, `adapter/agent-strict-tools` for Copilot in VS Code (`CURSOR-AGENTS`, `CODEX-SUBAGENTS`). See §8.3 and [`security-model.md`](security-model.md) §6.
- **Quality bar:** minimal tools, explicit scope, no secrets.

### 3.4 Tool

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

**Hooks are not a separate kind.** A hook is a **binding declared by a tool asset**: client event + matcher + argv. Adapters render it into the client's hook configuration as owned entries (§7.2). The command is always that tool's binary in exec form; it is never a shell string.

Rationale:
1. Hooks run automatically on matching events with the user's privileges, and some clients run them without a trust prompt in non-interactive modes (`CC-SET`). Restricting them to reviewed, tested, CI-built, signed Rust binaries removes the "arbitrary shell one-liner in a JSON file" risk class.
2. Hook logic becomes testable.
3. An asset with hook bindings must declare the `hooks` permission, which the user must accept explicitly (§6.3).

Alternatives considered:
- A `hook` kind with inline commands: rejected for v1.
- Excluding hooks entirely: the fallback, not chosen.

**Phase 3: failure behaviour.** A binding may set `on-failure = "block"`: when the hook cannot run (the binary is missing, crashes, times out or prints invalid output), the client blocks the event instead of letting it through. Guards such as `secret-guard` use it so that they fail closed. Claude Code renders it as `onFailure: "block"` (2.1.295 or later, `CC-P3`). The default, `continue`, is every client's own default. Because a missing binary then blocks the session, `plan` and `apply` warn when a fail-closed hook's binary is not on `PATH` (`plan/hook-binary-missing`).

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
- **Phase 3:** the bar is implemented once, in `crates/agent-grimoire-mcp`, and every server must serve through it ([ADR-0007](adr/0007-mcp-server-authorization.md)). It provides:
  - stdio with a clean stop on end of input, `SIGINT` or `SIGTERM`;
  - Streamable HTTP, stateless, loopback-only without authorization, with `Host` and `Origin` checked before anything else;
  - OAuth resource-server checks against a local JWKS file;
  - concurrency, rate and time limits for tool work;
  - a test helper that compares a server's live `tools/list` with its `asset.toml`.

  Servers speak MCP 2026-07-28, and 2025-11-25 for clients that still use the earlier handshake, over both transports. Over HTTP the version is checked after authorization, and each connection carries one request and is closed within 75 seconds.

### 3.6 Bundle (not an asset kind)

A **bundle**, `catalog/bundles/<id>.toml`, is a named, versioned list of asset references with version requirements. Selecting a bundle expands to its members at resolve time. The lock records which bundle brought in which asset, so removing the bundle removes only assets nothing else selected.

## 4. Repository layout

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
docs/  docs/adr/           design, specs, ADRs and runbooks, and the user reference (Phase 7):
                           cli.md, diagnostics.md, configuration.md, versioning.md
.github/                   workflows, scripts/ (image-tags.sh, Phase 6, ADR-0011), dependabot.yml;
                           ISSUE_TEMPLATE/ (issue forms), pull_request_template.md and CODEOWNERS (Phase 7)
Dockerfile                 the container image: COPY only, base pinned by digest (Phase 6, ADR-0011)
.grype.yaml                Grype config for the image scan gate (Phase 6, ADR-0011)
README.md                  also published on crates.io (agent-grimoire), on PyPI and in every CLI archive (Phase 7)
CONTRIBUTING.md  SECURITY.md  CODE_OF_CONDUCT.md   community files, at the root only (Phase 7)
LICENSE-MIT  LICENSE-APACHE
CLAUDE.md  AGENTS.md       conventions for this repo's own contributors and agents
```

Package and directory names follow [ADR-0004](adr/0004-package-and-binary-names.md).

**Refinement 1: code-bearing assets live under `catalog/`, not under `crates/`.**

The brief asks for two things at once: "one crate per MCP server and per tool" in `crates/`, and that "a new asset can be added with files under `catalog/` only" with "no core code changes". Both can't hold if server crates sit in `crates/`. Making each tool and MCP-server asset directory a workspace crate, picked up by the `catalog/*/*` member globs, satisfies both. Cargo rejects a member glob that matches nothing (`CARGO-LINTS` in `specs.md`), so the globs are added together with the first code-bearing asset, in Phase 3:

- adding a server is one new directory;
- `crates/` stays "core".

A shared hardening library for servers (transport setup, Origin validation, auth layer, limits) lives in `crates/agent-grimoire-mcp` (Phase 3, `publish = false`). It is core code that new servers *use*, not code they modify. Catalog validation requires every MCP server to depend on it (`crate/hardening`), and forbids asset crates from depending on the installer's crates (`crate/core-dependency`).

**Refinement 2: adapters are one crate with a module per client.** They share the output model and test harness. A crate per client would add publish overhead (see ADR-0004) for no isolation benefit.

**Refinement 3: generated files are checked in.** This covers schemas, the packed catalog and the marketplace. Each generator has a `--check` mode that CI runs, so "generated, never hand-maintained" is enforced mechanically.

**Dependency direction.** `model` ← `catalog` ← `resolve`, then `model` + `install` ← `adapters` (adapters construct install's output values), then everything ← `cli`. Tool and server crates depend on none of the core crates.

**Phase 2 notes:**
- **The `Source` abstraction is a type, not a trait.** `agent_grimoire_catalog::Source` is the in-memory index of manifests, digests and installed files. The builtin pack builds it today, and a remote index would build the same type. A trait with a single implementation added nothing.
- **The pack** is `crates/agent-grimoire-catalog/catalog.pack`, a deterministic length-prefixed list of files (asset manifests, READMEs, changelogs, installed files and bundles). `cargo xtask pack-catalog` writes it from a catalog that validates cleanly, and CI runs `--check`.
- **The CLI** is a library (`agent_grimoire::run`) plus a thin binary, so tests can drive it with fixture catalogs.
- **Test fixtures** live in `crates/agent-grimoire-testkit` (`publish = false`).

## 5. Identifiers and versions

- **Asset IDs are unique across the whole catalog**, not just per kind. That gives simple CLI UX (`grimoire add rust-errors`), unambiguous lock keys, and one-to-one plugin names in the generated marketplace.
- **Format:** the Agent Skills `name` rules for every kind. 1–64 chars, `[a-z0-9-]`, no leading, trailing or double hyphen, and equal to the directory name.
  - IDs starting with the plugin-reserved prefixes `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-` are also rejected, because generated Claude Code plugins would fail to load (`CC-PLUGIN`).
- **Asset versions are SemVer, one per asset.** CI requires a version bump and a CHANGELOG entry whenever an asset's files change. The asset version is independent of the CLI version.
- **Manifests carry `min-cli-version`.** Resolution fails clearly if the running CLI is older.

## 6. Manifests and configuration

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
  - `[tool] binary = "…"` plus optional `hooks = [...]` bindings: `event`, `tools`, `args`, `timeout-secs` and (Phase 3) `on-failure`. `on-failure = "block"` is allowed only on `pre-tool-use` and `user-prompt-submit`, the events whose failure can stop something (`manifest/hook-on-failure`);
  - `[mcp-server] binary = "…", transports = ["stdio","streamable-http"], env = [...]`, plus the declared tool list with annotations, which tests compare against the live server.
  - **Phase 3, environment variables:** `permissions.env` lists *every* variable the asset reads, and each one needs consent (§6.3). `[mcp-server] env` lists the subset the client configuration forwards from the user's environment. A variable the client sets itself, such as Claude Code's `CLAUDE_PROJECT_DIR` for stdio servers (`CC-P3`), is in `permissions.env` only, so nothing is forwarded for it.
- **Explicit output map per client (refines the brief's "explicit file map").** Every client an asset supports lists its outputs explicitly: source file → destination → strategy. A reviewer can see exactly what each asset writes for each client.
  - The destination is written as an **anchor** (`rules:`, `skills:`, `agents:`, `instructions`, `mcp`, `hooks`) plus a relative name, rather than a literal path. Each adapter resolves its anchors.
  - Reason 1: a client changing its layout becomes one adapter change instead of an edit to every manifest.
  - Reason 2: the planner can choose a location that several selected clients share (§8.2).
  - The resolved literal paths are shown by `plan` and recorded in the lock.
  - Validation rejects any destination outside the adapter's declared anchor set. An asset therefore *cannot* write, for example, `.github/workflows/x.yml`.
  - `cargo xtask new` scaffolds these tables. The alternative, literal paths, would make every layout change a catalog-wide edit.
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
github-insights = { version = "^1", allow = ["network", "env:GITHUB_TOKEN"], env = { GITHUB_TOKEN = "GH_TOKEN" } }
project-docs = { version = "^0.1", allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"], transport = "streamable-http", url = "http://127.0.0.1:8931/mcp" }
```

Per-asset overrides in v1:
- a subset of clients;
- **`allow`**: accepted permissions (§6.3);
- for MCP servers, an env-var *name* remapping and the transport. `transport = "streamable-http"` needs the server's `url` (Phase 3, below). Setting `transport`, `url` or `env` on any other kind is an error (`resolve/options-kind`).

The ADR-0002 override `runtime = "docker"` is deferred past v1 (the maintainer's choice in Phase 6, [ADR-0011](adr/0011-container-image.md)): no MCP client offers a common variable for the project directory, and the CLI can't embed the digest of the image that contains it. `grimoire.toml` still rejects it as an unknown field. [`container.md`](container.md) shows hand-written, digest-pinned `docker run` entries instead.

Nothing that could alter destinations outside the anchor rules is overridable.

**Consent for assets without an `[assets]` entry (Phase 2).** An asset brought in by a bundle, a tag or another asset's `requires` has no `[assets]` table to hold `allow`. Its grants go in a top-level table instead:

```toml
[allow]
secret-guard = ["filesystem-read", "hooks"]
```

Each asset's grants live in exactly one place. An asset listed under `[assets]` takes them there, and an `[allow]` entry for it is an error (`config/allow-direct`). `grimoire add` writes whichever applies, and moves the grant when an asset becomes direct. An `[allow]` entry for an asset that is not installed is a warning.

**HTTP transport (Phase 3, settles §11.8).** With `transport = "streamable-http"` the user runs the server and the client connects to it, so the entry needs the server's `url`:
- `url` is required with `streamable-http` and refused otherwise (`config/url-missing`, `config/url-transport`). `env` overrides are refused with it (`config/env-transport`): the client does not start the server, so it passes no environment.
- The URL is checked while parsing (`ServerUrl`): `https://`, or `http://` only for this machine (`localhost`, `127.0.0.0/8`, `[::1]`). User information, a query or a fragment are refused, so no credential can be written into a configuration file (MCP forbids access tokens in URLs, `MCP-SPEC`). So are `$` and `{`, which a client would expand. ASCII only, at most 2048 characters.
- `claude-code` writes `mcpServers.<id> = {"type": "http", "url": "<url>"}`. Authorization, when the server requires it, is negotiated by the client with the server's OAuth metadata (§11.4); Grimoire writes no headers and no tokens.

### 6.3 Permission consent

Declared `[permissions]` are not just documentation. Resolution fails if an asset needs a permission that is not in its `allow` list.

**Grant names:** `filesystem-read`, `filesystem-write` (which covers read), `network`, `shell`, `hooks`, and **`env:<NAME>` for each environment variable** (Phase 3, settles §11.7). A bare `env` is refused in `grimoire.toml`. Because each variable is its own grant, a new version that reads one more variable is permission growth like any other, and stops until the user accepts that name. `grimoire add --allow env` is shorthand on the command line only: it accepts every variable the named assets read now and writes each one out by name. The lock records the same names in `allow`.

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
- Skills: Agent Skills rules (`SKILLS`), spec-only frontmatter, referenced files exist, and no `.claude-plugin/` anywhere in the skill, which would make Claude Code load the installed copy as a plugin (`skill/plugin-manifest`, Phase 4).
- Agents: `tools` present and consistent with `[permissions]` (e.g. `execute` requires `shell`).
- Tools and MCP servers: the crate builds, `binary` matches a `[[bin]]`, and the declared tools match the server's `tools/list` in tests.
  - Phase 3, for every asset crate:
    - no dependency on Grimoire's core crates (`crate/core-dependency`);
    - no `path` or `git` dependency except on `agent-grimoire-mcp` (`crate/path-dependency`);
    - no `Cargo.toml` below the asset's own (`crate/nested-manifest`).
  - Phase 3, for MCP servers:
    - a dependency on `agent-grimoire-mcp` (`crate/hardening`);
    - no direct use of rmcp's stdio or Streamable HTTP transports in `src/` (`crate/raw-transport`);
    - a test that calls `agent_grimoire_mcp::testing::declared_tools_match` (`crate/declared-tools-test`).

    The last two are text searches that catch the usual mistakes, not proofs; code review still guarantees them.
- Phase 4, for every asset: `cargo xtask validate` renders it for each client it declares, through the real adapters, so a refusal only an adapter makes (a Cursor agent with other tools, a comma in a Cursor or Copilot glob, an MCP tool name a client would change) fails validation rather than a user's `apply`.
- Rules: no permissions, plus duplicate and near-duplicate detection across rules. Every client's output uses the same `src`, because clients read each other's rule files (`manifest/rule-src`; Phase 4 extends it from `claude-code` and `agents-md` to every client).

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

**Phase 2 additions to the lock:**
- A top-level `[[output]]` list holds content Grimoire itself owns rather than an asset. Today that is the `CLAUDE.md` block that imports `AGENTS.md` (§8.2), and, since Phase 4, the `GEMINI.md` block that imports it for Gemini CLI. Its block markers use the reserved name `grimoire`. The asset id `grimoire` and the prefix `grimoire-` are reserved so that no asset can collide with it.
- `created-file = true` records that adding an output created its (shared) file. `created-keys = ["/mcpServers"]` records the JSON containers it created. Removal deletes exactly those, once empty. A file or container that existed before Grimoire is never deleted. When the creating output is removed while others remain, the record moves to one of them.

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
| `block` | `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` | the text between `<!-- grimoire:begin <id> -->` and `<!-- grimoire:end <id> -->` | block-content hash vs lock |
| `entry` | `.mcp.json` `mcpServers.<id>`, `.cursor/mcp.json`, `.gemini/settings.json`, `.codex/config.toml` `[mcp_servers.<id>]`, hook arrays | one key (by JSON-Pointer-style path) or one array element (by value) | canonical-value hash vs lock |

Details:
- **Blocks** sit in a deterministic order. User text outside blocks is never touched. HTML comments are stripped by Claude Code before injection, so markers cost no context there (`CC-MEM`).
- **Entries** use format-preserving editing: `toml_edit` for TOML, and a comment- and order-preserving JSON/JSONC editor for JSON (choice in Phase 2). Unrelated keys and the user's formatting are preserved. Property tests cover merge, unmerge, and round-trips with arbitrary surrounding content.
- **Removal** deletes only owned units. Grimoire deletes a shared file it created only when the file is now empty of anything Grimoire didn't write. Adopting content that already matches, or overwriting it with `--force`, makes it Grimoire's: a whole file in a managed folder is then deleted with its asset, and a shared file that held only Grimoire's content when it was adopted is recorded as created (Phase 7 check of `plan.rs`). Otherwise it removes its blocks or entries and leaves the file. Directories Grimoire created are removed if empty.
  - Phase 2 implementation:
    - Creation is recorded in the lock (`created-file`, `created-keys`; §6.5).
    - After deleting a file, Grimoire removes the directories above it that are now empty, whether or not it created them. Creating directories is not recorded, and an empty directory holds no content (git does not track one).
- **Entry keys** (Phase 2) are RFC 6901 JSON Pointers. `/mcpServers/<id>` owns that member's value. `/hooks/PreToolUse|<id>#<n>` owns one element of that array, found by the hash of its value; the text after `|` only keeps keys distinct.
- **JSON editing** (Phase 2, settles §11.1) uses `jsonc-parser`'s concrete syntax tree (`EDIT-CRATES` in `specs.md`), so no ADR is needed.
  - Unrelated keys, values and order are always preserved, as is all formatting outside the container Grimoire edits.
  - Inserting into a container written on a single line reflows that container across lines. Removing the entry later restores the values, not the single line. Grimoire edits only **strict JSON**: Claude Code reads its settings as strict JSON, and a file a client can't parse should not be "fixed" silently. Comments, trailing commas, or duplicate keys on Grimoire's path make the file unreadable for Grimoire (§7.3).
- **Phase 4: three entry formats, chosen by path** (`EDIT-P4`), because removing a locked entry has no desired content to say which format the file is:
  - `.toml` files (`.codex/config.toml`): TOML through `toml_edit`. Only whole member keys such as `/mcp_servers/<id>`, written as a standard table; a plain value that would add a `[parent]` header is refused. The hash is of the value converted to JSON, so drift detection works the same way. A file Grimoire doesn't change keeps its bytes; a changed one keeps its byte-order mark and CRLF line endings. Comments above a removed table move to the next header or the end of the file, and an empty table Grimoire created stays if the user gave it a header or comments.
  - `.gemini/settings.json`: JSONC. Comments are allowed and kept; everything else is as strict as JSON, which is what Gemini CLI accepts.
  - Values are hashed without their comments, so a comment the user adds *inside* Grimoire's entry in a TOML or JSONC file would be lost by an update or removal. Planning that update or removal is a conflict instead (`commented`; `--force` drops the comment).
  - Every other entry file: strict JSON, as above.

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

Phase 2 details:
- `apply --write-new` exits 4, because the conflicts remain. The lock keeps the old records for those files, so the conflict is reported again until the user merges the `.new` file or uses `--force`.
- **A file Grimoire cannot edit safely is refused, even with `--force`** (exit 4). That covers malformed or unterminated block markers, non-UTF-8 text, invalid or non-strict JSON, a JSON path that is the wrong type, and duplicate keys on Grimoire's path. Grimoire never guesses what it owns.
- `apply` re-reads each file just before writing it and aborts if it changed after planning (exit 4). A rerun converges.
  - The files already written by then are recorded in the lock, so their creation records (§6.5) are not lost.
  - The message says that some files were written.
- **Array elements are found by their exact value** (hook bindings). When a locked element can't be found, it is a conflict (`NotFound`), whether the entry is still wanted or not. An edited element and a removed one look the same, so Grimoire never forgets one silently.
  - With `--force`, a still-wanted element is written again, and any edited copy is left as the user's.
  - Two keys never claim the same element, and identical desired elements are refused.
- **`--write-new` never replaces a different `<path>.new`.** An identical one from an earlier run is left as it is. Sidecars are checked before anything is written, and get the original file's permissions.
- **Something in the way** is a conflict (exit 4), not a safety refusal: a file where a directory is needed (`.claude` as a file), or a directory where a file is needed.
- **A case-only rename between versions** is refused (exit 3), for example a lock recording `Guide.md` while the catalog now writes `guide.md`. On macOS and Windows both names are one file, and deleting the old name would delete the new one.

### 7.4 Filesystem safety

- **Paths:** every destination is a normalized **ASCII** relative path under the project root (`RelPath`). ASCII-only rules out invisible and bidi characters, superscript device names and Unicode-normalization collisions. These are rejected: absolute paths, `..`, drive and UNC prefixes, NUL bytes, Windows reserved names and alternate data streams (`:`), and names that collide case-insensitively within one plan. The catalog validator applies the same rules to every catalog file (Phase 1).
- **Confinement:** all filesystem access goes through a capability handle on the project root (`cap-std`). The handle refuses escapes, including through symlinks.
  - Every directory on the way to a managed path is opened no-follow, one component at a time. The dependency's deny.toml entries are justified in [ADR-0006](adr/0006-filesystem-confinement.md).
  - The full threat model is in [`security-model.md`](security-model.md).
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
- an agent a selected client can't limit to its tools at all (Codex has no agents; Cursor takes only exactly `read` and `search`, as `readonly`). Where a client limits the tools only partly, `plan` warns instead (ADR-0008);
- a pinned version unavailable in this CLI's catalog when content is needed (ADR-0001).

**Phase 2 refinement of the last item.** With one embedded version per asset, `plan`, `apply` and `update --no-apply` stop whenever a selected asset's locked version (or manifest digest) differs from the catalog's. That includes cases where no file would change, because telling whether content is needed would mean rendering the pinned version, which is not available. The error names `grimoire update <id>`.
- `verify` and `uninstall` still work, because they need only the lock. So does `remove` + `apply`, but only when the removed assets include every asset whose pin differs: `apply` still stops for any selected asset that keeps a differing pin (checked in Phase 7).
- Assets not yet in the lock resolve to the catalog version.

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

Clients read each other's files (re-checked 2026-11-17 in the `*-P4` entries of `specs.md`). **✓** means documented as read; **–** means not documented as read.

| Location | Claude Code | Codex | Cursor | Gemini CLI | Copilot |
|---|---|---|---|---|---|
| `AGENTS.md` | only if no `CLAUDE.md`¹ | ✓ (32 KiB budget shared with nested files) | ✓ always applied | only via `context.fileName` | ✓ cloud agent, CLI, VS Code |
| `CLAUDE.md` | ✓ | – | ✓ always applied² | – | ✓ CLI (expands `@` imports), cloud agent |
| `GEMINI.md` | – | – | – | ✓ trusted folders only | ✓ CLI (no imports), cloud agent |
| `.claude/rules/*.md` | ✓ (`paths`) | – | – | – | ✓ CLI (`paths`); VS Code Claude harness |
| `.cursor/rules/*.mdc` | – | – | ✓ | – | – |
| `.github/instructions/*.instructions.md` | – | – | – | – | ✓ |
| `.claude/skills/` | ✓ | – | ✓ | – | ✓ |
| `.agents/skills/` | – | ✓ | ✓ | ✓ trusted folders only | ✓ (the CLI loads a name found in both skill folders once, from here) |
| `.claude/agents/` | ✓ | – | ✓ (no per-tool allow-list; how it treats `tools` is undocumented) | – | ✓ CLI (after `.github/agents/`), VS Code |
| `.mcp.json` | ✓ | – | – | – | ✓ CLI, VS Code |
| `.claude/settings.json` hooks | ✓ | – | ✓ (on by default) | – | ✓ CLI, VS Code |

¹ In the default `claude-md-or-agents-md` mode (`CC-MEM`).
² Whether Cursor expands an `@AGENTS.md` line in `CLAUDE.md` is undocumented.

**Placement (Phase 4, ADR-0008).** Adapters render to their own client's locations, then one placement pass moves content that several selected clients can read from one place, so it is written once and attributed to all of them in the lock:

1. **Rules: `AGENTS.md` covers Cursor and Copilot.** When `agents-md` or `codex` puts a rule in `AGENTS.md`:
   - `cursor` and `copilot` get no rule file of their own, because they read `AGENTS.md` (Copilot's cloud agent alongside a root `CLAUDE.md` is undocumented);
   - `claude-code` reads it through a Grimoire-owned `@AGENTS.md` block in `CLAUDE.md` (Phase 2);
   - `gemini-cli` reads it through the same block in `GEMINI.md`.

   Otherwise each client gets its own file. A path-scoped rule in `AGENTS.md` carries its scope line, so Claude Code, Cursor and Copilot apply it everywhere. Manifest validation requires every client of a rule to use the same `src` (`manifest/rule-src`), so a shared block never holds two versions of a rule.
2. **Skills.** `.claude/skills/` is written when `claude-code` is selected, and `.agents/skills/` when `agents-md`, `codex` or `gemini-cli` is. Cursor's and Copilot's copy goes to `.agents/skills/` when that is written, else to `.claude/skills/` when that is, else to `.agents/skills/`. Examples:
   - `{claude-code, cursor, copilot}` gets skills only in `.claude/skills/`;
   - `{codex, gemini-cli, cursor}` gets skills only in `.agents/skills/`;
   - `{claude-code, codex, cursor}` needs both, so Cursor lists the skill twice.
3. **MCP.** `copilot` writes the same `.mcp.json` entry as `claude-code`, once.

**What `plan` reports** (warnings, never silent):
- `plan/duplicate-rule`: a client selected for a rule that may still read it twice. Copilot CLI may read `AGENTS.md` and the `CLAUDE.md` import, or `.claude/rules/` and `.github/instructions/` (it deduplicates only identical copies). Cursor may read `AGENTS.md` and the `CLAUDE.md` import.
- `plan/duplicate-skill`: Cursor, selected for a skill, seeing it in both skill folders.
- `plan/cross-client-rule`: an `AGENTS.md` rule not installed for Claude Code or Gemini CLI that they read anyway, through Grimoire's import in `CLAUDE.md` or `GEMINI.md` (or, for Claude Code, natively when there is no `CLAUDE.md`).
- `plan/cross-client-agent`, `plan/cross-client-hooks`, `plan/cross-client-mcp`: a `.claude/agents/` agent, a `.claude/settings.json` hook or a `.mcp.json` server that Cursor, Copilot or Claude Code also loads. Installing for one client can grant something in another: Cursor has no per-tool allow-list and may run a `.claude/agents/` agent with every tool, and Cursor and Copilot run `.claude/settings.json` hooks. `docs/security-model.md` covers these.
- `plan/codex-instructions-size`: a planned `AGENTS.md` of 24 KiB or more when `codex` is a project client, because Codex reads at most 32 KiB of project instructions in total (§11.5).
- `plan/codex-override`: a root `AGENTS.override.md` when `codex` is a project client, because Codex then reads it instead of `AGENTS.md`.

Other cases a client reads Grimoire's content without being selected for it, such as Cursor reading an `AGENTS.md` rule installed only for `agents-md`, are not reported.

### 8.3 Per-client mapping (client order per [ADR-0005](adr/0005-client-priority.md), amended by [ADR-0008](adr/0008-phase-4-client-adapters.md))

| Kind | `claude-code` | `agents-md` (portable) | `codex` | `cursor` | `gemini-cli` | `copilot` |
|---|---|---|---|---|---|---|
| Rule, always-on | see §8.2 | `AGENTS.md` block | `AGENTS.md` block (shared) | `AGENTS.md` when written, else `.cursor/rules/<id>.mdc` `alwaysApply: true` | `@AGENTS.md` import block in `GEMINI.md` when `AGENTS.md` is written, else a `GEMINI.md` block | `AGENTS.md` when written, else `.github/instructions/<id>.instructions.md` `applyTo: "**"` |
| Rule, path-scoped | `.claude/rules/<id>.md` `paths` | `AGENTS.md` block + scope line | same | `.mdc` with `alwaysApply: false` and `globs` | block + scope line | `.instructions.md` `applyTo` |
| Skill | `.claude/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` | shared (§8.2) | `.agents/skills/<id>/` | shared (§8.2) |
| Agent | `.claude/agents/<id>.md` with explicit `tools` | – | – (a Codex agent can't be restricted) | `.cursor/agents/<id>.md` `readonly: true`, only for an agent with exactly `read` and `search` | `.gemini/agents/<id>.md` with explicit `tools` | `.github/agents/<id>.agent.md` with `tools` and `strict-tools-list: true` |
| MCP server | `.mcp.json` entry, `${VAR}` | – | `.codex/config.toml` `[mcp_servers.<id>]`, `env_vars` | `.cursor/mcp.json` entry, `${env:VAR}` | `.gemini/settings.json` entry, `${VAR}` | `.mcp.json` entry (shared with Claude Code); cloud agent: `doctor` prints the settings JSON |
| Tool hooks | `.claude/settings.json` `hooks` | – | – | – | – | – |

Hooks are Claude Code only, and Codex has no agents (ADR-0008): no other client can keep a failed hook from letting the action through, and a Codex agent inherits the parent's sandbox, MCP servers and web search. An asset that declares those outputs fails validation (`manifest/anchor-client`). In a project that also selects such a client, an asset it can't host is installed for the others, with a warning (`resolve/client-not-covered`), for example that `secret-guard` doesn't guard Codex.

Project-scoped content often loads only in a trusted project: Codex `.codex/config.toml` (and so its MCP servers); everything Gemini CLI reads from the project; Copilot's `.mcp.json`. `doctor` says so for each selected client, because Grimoire can't check or grant trust.

**`claude-code` details as implemented in Phase 2** (`CC-P2`):

| Output | Written as |
|---|---|
| Rule | The rule text. A path-scoped rule first gets `paths` frontmatter, a YAML list of quoted globs. |
| Agent | Frontmatter `name` (the id), `description` (the manifest `description`, the delegation text) and `tools`, all JSON-quoted so YAML never reads `true` or `123` as non-strings. |
| MCP server | `mcpServers.<id> = {"type": "stdio", "command": "<binary>", "env": {"VAR": "${VAR}"}}`. |
| Hook binding | `hooks.<Event>[] = {"matcher": …, "hooks": [{"type": "command", "command": "<binary>", "args": […], "timeout": n, "onFailure": "block"}]}`. `onFailure` only with `on-failure = "block"` (Phase 3). |

- **Agent tool mapping:**
  - `read` → `Read`; `search` → `Grep`, `Glob`; `edit` → `Edit`, `Write`, `NotebookEdit`; `execute` → `Bash`, `PowerShell`; `web` → `WebFetch`, `WebSearch`.
  - `mcp:<id>` → `mcp__<id>__*`, and `mcp:<id>/<tool>` → `mcp__<id>__<tool>`.
  - An agent with no capabilities gets `tools: ["Read"]` with `disallowedTools: ["Read"]`, the documented way to grant no tools.
- **MCP servers:**
  - A server started with no arguments speaks stdio, which every Grimoire server must support. Optional variables are written `${VAR:-}`, because an unset variable without a default reaches the server literally.
  - `env` overrides map the server's variable to the user's name (`${GH_TOKEN}`).
  - With `transport = "streamable-http"`, the entry is `{"type": "http", "url": "<url>"}` (Phase 3, §6.2).
- **Hook bindings:**
  - They use exec form: `args` selects "no shell involved".
  - Matchers are an exact `A|B` list, or an anchored `^(…)$` regex when an MCP tool is involved.

**Phase 4 adapters** (`*-P4` in `specs.md`):

| Client | Output | Written as |
|---|---|---|
| `codex` | MCP server | `[mcp_servers.<id>]` in `.codex/config.toml`: `command` plus `env_vars` (the variable names to forward), or `url`. Codex starts stdio servers with an almost empty environment and has no `${VAR}`, so an `env` override that renames a variable is refused (`adapter/env-remap`). |
| `cursor` | Rule | `.cursor/rules/<id>.mdc` with `alwaysApply: true`, or `alwaysApply: false` and `globs` as one quoted, comma-separated string. A glob containing a comma can't be written (`adapter/unsupported`). |
| `cursor` | Agent | `name`, `description`, `readonly: true`. |
| `cursor` | MCP server | `mcpServers.<id>` in `.cursor/mcp.json`: `{"type": "stdio", "command", "env": {"VAR": "${env:SOURCE}"}}`, or `{"url"}`. |
| `gemini-cli` | Rule | A `GEMINI.md` block, with the scope line when path-scoped. |
| `gemini-cli` | Agent | `name`, `description`, `tools`: `read` → `read_file`, `read_many_files`; `search` → `grep_search`, `glob`, `list_directory`; `edit` → `replace`, `write_file`; `execute` → `run_shell_command`; `web` → `web_fetch`, `google_web_search`; `mcp:<id>` → `mcp_<id>_*`. No tools is `tools: []`. |
| `gemini-cli` | MCP server | `mcpServers.<id>` in `.gemini/settings.json`: `{"command", "env": {"VAR": "${SOURCE}"}}` (`${SOURCE:-}` when optional), or `{"url", "type": "http"}`. The file is JSONC: Grimoire keeps its comments and refuses trailing commas, as Gemini CLI does. |
| `copilot` | Rule | `.github/instructions/<id>.instructions.md` with `applyTo`: `"**"`, or the globs joined by commas. |
| `copilot` | Agent | `name`, `description`, `tools` (`read`, `search`, `edit`, `execute`, `web`, `<server>/*`, `<server>/<tool>`), `strict-tools-list: true`. A prompt over 30,000 characters is refused. |
| `copilot` | MCP server | The `claude-code` `.mcp.json` entry. |

## 9. CLI

This section is the design. The user reference is [`cli.md`](cli.md), for the commands, flags, exit codes, `--json` output and environment, and [`diagnostics.md`](diagnostics.md), for every user-facing diagnostic code. Tests compare both with the code; nothing compares this section with it.

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
| `grimoire doctor` | Environment report: each client's installed outputs and notes on what it needs (trust, approvals; not checked), tool and server binaries on `PATH`, the Copilot cloud-agent MCP settings, config parse errors | — |
| `grimoire uninstall [--purge]` | Remove all managed content and the lock (`--purge` also removes `grimoire.toml`) | managed outputs, lock |

`add`/`remove` only change the desired state. `apply` reconciles, which keeps CI flows explicit. `add --apply` (and `remove --apply`) is a convenience.

As implemented in Phase 2:
- **`init`** detects `claude-code` from `CLAUDE.md` or `.claude/`, and `agents-md` from `AGENTS.md` or `.agents/`. With nothing detected and no `--client`, it is a usage error. Clients without an adapter yet are refused.
- **`add`:**
  - Resolves the new selection before writing, so a broken selection never reaches `grimoire.toml`.
  - Asks for each asset's missing permissions on a terminal, or takes them from `--allow`.
  - A plain id that names a bundle is added as a bundle.
  - New direct entries get `^<catalog version>`.
- **`remove`** warns when another selector keeps the asset installed.
- **`update [<id>…]`** lets the named (or all) pins move to the catalog's versions. `--no-apply` writes the lock with the new versions but keeps every output record describing the files as they are, so the next `apply` performs the update.
- **`doctor`** never runs a binary. It only checks that each locked tool or MCP server binary is on `PATH`.
- **Phase 4:**
  - `init` detects every client: `claude-code` (`CLAUDE.md`, `.claude/`), `agents-md` (`AGENTS.md`, `.agents/`), `codex` (`.codex/`), `cursor` (`.cursor/`), `gemini-cli` (`GEMINI.md`, `.gemini/`) and `copilot` (`.github/copilot-instructions.md`, `.github/instructions/`, `.github/agents/`). Every client has an adapter.
  - `doctor` counts each client's outputs from the lock, notes what the client needs before it reads them (Codex and Gemini CLI trust, Cursor's MCP approval, Copilot's folder trust), which Grimoire can't check, and prints the JSON to paste into Copilot cloud agent's MCP settings for each MCP server installed for `copilot`. Optional variables there get an empty default (`${COPILOT_MCP_<NAME>:-}`); a server configured over Streamable HTTP is given by its URL, which the cloud agent can use only if it can reach it without OAuth.

### 9.2 Machine use

- `--json` on every command puts exactly one JSON document on stdout: `{ "format": 1, "command": …, "ok": …, "exit": <code>, "result": …, "diagnostics": [ … ] }`. Human diagnostics go to stderr. With `--json` stderr stays quiet, and usage errors also produce the envelope.
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
| 4 | Conflict: drift or an unmanaged file is in the way. Nothing was written, except when a file changed or failed partway through an `apply` (the message says so, and the lock records what was written) |
| 5 | Check failed: `plan --check` found changes, `verify` found mismatches, or `doctor` found problems |
| 6 | Refused for safety (path escape, symlink at a managed path) |
| 130 | Not returned by Grimoire. It installs no signal handler, so Ctrl-C (`SIGINT`) kills the process, and a POSIX shell reports that as 130 (128 + the signal number). What other callers and Windows see is unverified. A rerun converges (§7.1) |

## 10. Generated artifacts

| Artifact | Generator | CI check |
|---|---|---|
| `schemas/*.schema.json` (asset, bundle, config, lock) | `cargo xtask schema` (schemars) | `--check` |
| Packed builtin catalog | `cargo xtask pack-catalog` | `--check` (ADR-0001) |
| `.claude-plugin/marketplace.json` + `plugins/claude-code/<name>/` | `cargo xtask gen-marketplace` (Phase 4) | `--check`, and `claude plugin validate --strict` on the marketplace and each plugin |

Marketplace (Phase 4, verified against `CC-PLUGIN-P4` and `CC-MKT-P4`):
- **Layout:** `.claude-plugin/marketplace.json` at the repository root (name `grimoire`, owner `jabbott-iii`) lists one plugin per eligible asset and per bundle, each under `plugins/claude-code/<id>/` with its own `.claude-plugin/plugin.json` and a generated `README.md`.
- **Eligible assets** are skills, agents and MCP servers that support `claude-code`. Their plugin holds the files the `claude-code` adapter installs: `skills/<id>/…`, `agents/<id>.md`, or `.mcp.json`. Agent tools name MCP tools the way Claude Code names a plugin's (`mcp__plugin_<server>_<server>__<tool>`), and an agent depends on the plugins of the MCP servers it uses.
- **Left out:** rules, because a plugin's `CLAUDE.md` isn't loaded; and tools, because whether a plugin hook can fail closed is unverified while Grimoire never installs the binary (the maintainer's choice).
- **Bundles** become plugins that depend on their eligible members' plugins (bare names, no version ranges, because ranges need per-plugin git tags). The description names the members left out.
- **Names** must pass Claude Code's reserved-name rules, including the whole-word rule that `claude plugin validate --strict` enforces; `gen-marketplace` fails otherwise.
- **Versions** live only in each `plugin.json`, as the docs advise: the asset or bundle version plus build metadata from a hash of the plugin's files (`0.2.0+<12 hex>`). Claude Code keeps a user on a cached copy until the version changes, and plugin content depends on the generator as well as the catalog.
- **Dependencies:** an agent's plugin depends on the plugins of the MCP servers whose tools it names, and generation fails if such a server isn't a plugin. Any other `[requires]` entry that is a plugin becomes a dependency too; one that isn't is named in the description.
- **Permissions:** each plugin's README lists what the catalog declares (files, network, shell, environment variables, an agent's tools). Installing a plugin grants these without the consent `grimoire add` asks for.
- **Deprecated** assets say so, and name their replacement, at the start of the description.
- **Adding it:** users add the marketplace as `owner/repo` or a GitHub repository URL. A raw `marketplace.json` URL can't resolve relative plugin sources.
- **CI:** `gen-marketplace --check`, then a pinned, checksum-verified Claude Code 2.1.311 runs `claude plugin validate --strict` on the marketplace and on each plugin directory, with no secrets.
- Plugin MCP servers still need the server binary on `PATH`. Codex also reads `.claude-plugin/marketplace.json` (`CODEX-PLUGINS-P4`); compatibility is unverified and out of v1 scope.

Release assets (Phase 5, [ADR-0009](adr/0009-release-pipeline.md)):
- **Generated at release time, not checked in.** `.github/workflows/release.yml` runs `cargo --locked xtask release` (`plan`, `build`, `sbom`, `channels`, `checksums`, `verify`, `notes`). Every PR runs all of them as a dry-run except `notes`, which runs only in the release-mode `publish` job. The only checked-in inputs are the templates and the launcher in `packaging/` (ADR-0010).
- **Release mode needs the version's `CHANGELOG.md` section**, which the merged release PR adds; without it `plan` runs a dry-run and `notes` fails. `plan` also fails when the catalog would exceed GitHub's limits (256 matrix jobs, 1000 release assets).
- **One build per package:** `build` runs `cargo auditable build -p <package>` for the CLI and for each asset binary, so each binary has only the dependency features its own manifest asks for, and its SBOM matches it.
- **What a release holds**, all flat in one directory:
  - the six CLI archives, `agent-grimoire-{V}-{triple}.tar.gz` or `.zip`;
  - one archive per asset binary and target, `{bin}-{assetV}-{triple}.tar.gz` or `.zip`;
  - a CycloneDX 1.6 SBOM per archive, `{archive}.cdx.json`;
  - the npm launcher and six platform tarballs, and the wheels;
  - `agent-grimoire.rb` (Homebrew), `agent-grimoire.json` (Scoop) and the three winget manifests;
  - the rendered `install.sh` and `install.ps1`;
  - `SHA256SUMS`, and a cosign bundle, `{file}.sigstore.json`, for every file.
- **Asset binaries are discovered from `cargo metadata --no-deps`:** every `bin` target of a package under `catalog/tools/` or `catalog/mcp-servers/`. Nothing is listed by hand, so a new tool or MCP server is released with no core edit. This settles the check ADR-0002 left for Phase 5.
- **Reproducible archives:** a fixed mtime, uid and gid 0, and sorted entries, so the same binaries always give the same archives.

Container image (Phase 6, [ADR-0011](adr/0011-container-image.md)):
- **Built from the release archives, not checked in.** The checked-in inputs are `Dockerfile` and `.grype.yaml`. `cargo --locked xtask release image` (`base`, `context`, `check`, `coverage`) holds the logic, and runs only in `release.yml`'s `image` job. `verify-image` runs the image, and `publish-image` pushes it with no cargo. Every PR builds, checks, scans and runs the image; only release mode pushes it.
- **One image, `ghcr.io/jabbott-iii/grimoire`,** for linux/amd64 and linux/arm64, on distroless static pinned by digest. The CLI is the entrypoint, and every asset binary sits in `/opt/grimoire/bin/`, off `PATH`. A release tags it `X.Y.Z` and `latest`, and the release notes name its index digest.
- **Inside a container, `doctor` and `plan/hook-binary-missing` describe the container's `PATH`,** not the host's.
- **Signing and tags.** Every signature and attestation is made in a reusable workflow whose jobs run only when `release.yml` on `main` calls them: `.github/workflows/sign.yml` for the release files and `.github/workflows/sign-image.yml` for the image, each under its own identity. Verifiers pin the signing workflow and that caller. `publish-image` pushes the image by digest, untagged; `sign-image` signs and attests it through `sign-image.yml`, and only then tags it with `.github/scripts/image-tags.sh`, a language-policy exception (ADR-0010, amended by [ADR-0011](adr/0011-container-image.md)).
- [`container.md`](container.md) shows how to run and verify it; ADR-0011 has the design and the supply chain.

## 11. Open design issues (to settle in the phase noted)

1. ~~**JSON/JSONC editing**~~ **Settled in Phase 2:** `jsonc-parser`'s CST, strict JSON only (§7.2, `EDIT-CRATES`).
2. ~~**Cross-client planning priorities**~~ **Settled in Phase 4:** the placement pass in §8.2 ([ADR-0008](adr/0008-phase-4-client-adapters.md)).
3. ~~**`.claude/rules/` vs a `CLAUDE.md` block**~~ **Settled in Phase 2:** `.claude/rules/` (§8.2).
4. ~~**Server-side MCP auth**~~ **Settled in Phase 3:** RFC 9068 JWT access tokens checked against a local JWKS file, in `crates/agent-grimoire-mcp` ([ADR-0007](adr/0007-mcp-server-authorization.md)).
5. ~~**Codex AGENTS.md budget**~~ **Settled in Phase 4:** `plan/codex-instructions-size` warns at 24 KiB. The 32 KiB budget is shared by every project instruction file Codex reads (`CODEX-P4`).
6. ~~**Copilot cloud-agent MCP**~~ **Settled in Phase 4:** `doctor` prints the settings JSON for each MCP server installed for `copilot`, with `$COPILOT_MCP_<NAME>` secrets.
7. ~~**Environment variable consent**~~ **Settled in Phase 3:** each variable is its own grant, `env:<NAME>`, recorded in `grimoire.toml` like every other grant (§6.3). Consent stays in the one file users review; the lock only mirrors it.
8. ~~**HTTP transport configuration**~~ **Settled in Phase 3:** a `url` option next to `transport = "streamable-http"`, validated as `ServerUrl` (§6.2).
