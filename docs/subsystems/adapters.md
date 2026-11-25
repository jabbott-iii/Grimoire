# Subsystem: `agent-grimoire-adapters`

`agent-grimoire-adapters` holds the six client adapters and the cross-client planner. A client is
an AI coding tool that Grimoire installs assets for, such as Claude Code or Cursor. An adapter
turns one resolved asset's declared outputs for one client into outputs at that client's literal
paths; then one placement pass writes content that several selected clients can read from one
place there once, and attributes it to all of them. The crate guarantees that adapters are pure
(the same input always gives the same output, and they read and write no files), that a client
which can't enforce what an asset declares gets nothing rather than something weaker, and that
content one client's install gives to another is reported, never hidden.
[ADR-0005](../adr/0005-client-priority.md) chose the six v1 clients and the order their adapters
were built in, and [ADR-0008](../adr/0008-phase-4-client-adapters.md) amended it after Phase 4
re-checked the formats: hooks stay Claude Code only, Codex gets no agents, Cursor gets only
read-only agents, and Cursor and Copilot read shared rules from `AGENTS.md`. Its API reference is
the crate docs in
[`crates/agent-grimoire-adapters/src/lib.rs`](../../crates/agent-grimoire-adapters/src/lib.rs).

## Why one pass over the selected clients

Clients read each other's files: Cursor always applies `AGENTS.md`, and Cursor and Copilot read
`.claude/skills/` as well as `.agents/skills/`. Independent adapters, each writing everything its
own client needs, would make a client read the same rule twice, and would give a client content
installed for another without saying so. So the adapters render to their own client's locations,
and a second pass over the *selected client set* moves shared content to one place.

The alternative, one adapter per combination of clients, doesn't scale to six clients. The cost of
the placement pass is that some combinations can't reach every client exactly once (all six
clients need skills in two folders), and that a path-scoped rule placed in `AGENTS.md` loses its
scoping for clients that read only that file. Both are reported as warnings rather than solved
silently.

Adapters also refuse rather than widen. An asset declares what it needs, such as an agent's
exact tools; where a client can't hold that, the adapter writes nothing, or writes the closest
form and warns. A refusal can surprise a user, so `grimoire show` lists the clients each asset
supports.

## Adapter contract

An adapter is pure: it has no side effects, and the same input always gives the same output.
Given resolved assets and the client set, it returns outputs. It declares:

- its client ID;
- its anchors, with each anchor's **read locations** (where the client reads that content) in
  preference order. An anchor is a named destination, such as `rules:`, `skills:`, `agents:`,
  `instructions`, `mcp` or `hooks`, so that an asset says what it installs and the adapter says
  where;
- the strategies it supports per kind;
- its capability→tool mapping, from Grimoire's capability names such as `read` to the client's
  tool names;
- its environment-variable reference syntax, such as `${VAR}`.

Snapshot tests (`insta`) cover every adapter × kind × representative asset. A snapshot test
stores reviewed output and fails when the output changes.

Built in M2.6 (for `claude-code` and `agents-md`) and M4.2 to M4.6 (the other four clients and
placement) as described here. The entry point, in `lib.rs`:

```rust
/// The clients this build can render for: all six.
pub const AVAILABLE: [ClientId; 6] = ClientId::ALL;

/// What adapters produce for a whole project.
pub struct Rendered {
    /// Every output, deduplicated across clients, sorted by path and then owner.
    pub desired: Vec<Desired>,
    /// What the user should know before applying (`plan/…` and `adapter/…` warnings).
    pub warnings: Vec<Diagnostic>,
}

pub fn render(resolution: &Resolution<'_>) -> Result<Rendered, Vec<Diagnostic>>;
```

The modules `claude_code`, `agents_md`, `codex`, `cursor`, `gemini_cli` and `copilot` are one per
client, and each module's documentation has a table of what it writes where. Their public
constants name the client's paths, such as `agents_md::AGENTS_MD`, `claude_code::MCP` and
`codex::CONFIG`. `destination` gives where a declared output lands for one client, for
`grimoire show`; placement can still move it, and `grimoire plan` shows the final paths.

### Which files Grimoire may own

`managed_unit` says what an adapter owns at a path: a whole file in a managed folder (such as
`.claude/skills/` or `.cursor/rules/`), a block in `AGENTS.md`, `CLAUDE.md` or `GEMINI.md`, or an
entry in one of the five configuration files (`.mcp.json`, `.claude/settings.json`,
`.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`). `is_managed_location` is
true wherever it returns a unit. `is_project_location` is true only for `CLAUDE.md` and
`GEMINI.md`, where Grimoire owns a block for itself: the `@AGENTS.md` import.

The CLI refuses a `grimoire.lock` that records anything else (`project/lock-path`). Grimoire never
records an output anywhere else, so such a lock was edited by hand, and following it could make
Grimoire read or delete a user's file. A `file` record on `.codex/config.toml`, for example, would
let it delete the user's whole Codex configuration.

## Client read matrix and cross-client planning

Clients read each other's files. The table shows which locations each client reads, as re-checked
on 2026-11-17 in the Phase 4 (`*-P4`) entries of [`../specs.md`](../specs.md). **✓** means
documented as read; **–** means not documented as read.

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

¹ In Claude Code's default `claude-md-or-agents-md` mode
([`CC-MEM`](../specs.md#cc-mem-memory-and-rules)).
² Whether Cursor expands an `@AGENTS.md` line in `CLAUDE.md` is undocumented.

In the table, "cloud agent" is Copilot cloud agent, which runs on GitHub.com. "Trusted folders"
are folders the user has marked as trusted in Gemini CLI.

### Placement

**Placement (Phase 4, ADR-0008).** Adapters render to their own client's locations. Then one
placement pass finds content that several selected clients can read from one place and moves it
there, so it is written once and attributed to all of them in the lock. ADR-0008 set these
placement rules:

1. **Rules: `AGENTS.md` covers Cursor and Copilot.** When `agents-md` or `codex` puts a rule in
   `AGENTS.md`:
   - `cursor` and `copilot` get no rule file of their own, because they read `AGENTS.md`. Whether
     Copilot's cloud agent still reads it when there is a root `CLAUDE.md` is undocumented;
   - `claude-code` reads it through a Grimoire-owned `@AGENTS.md` block in `CLAUDE.md` (Phase 2,
     `claude_code::agents_md_import`). A line `@AGENTS.md` tells Claude Code to import that file;
   - `gemini-cli` reads it through the same block in `GEMINI.md`.

   Otherwise each client gets its own file.

   A path-scoped rule in `AGENTS.md` carries its scope line, so Claude Code, Cursor and Copilot
   apply it everywhere, with only the scope line to limit it. Manifest validation requires every
   client of a rule to use the same `src` (`manifest/rule-src`), so a shared block never holds two
   versions of a rule.
2. **Skills.** `.claude/skills/` is written when `claude-code` is selected, and `.agents/skills/`
   when `agents-md`, `codex` or `gemini-cli` is. Cursor's and Copilot's copy goes to
   `.agents/skills/` when that is written, else to `.claude/skills/` when that is, else to
   `.agents/skills/`. Examples:
   - `{claude-code, cursor, copilot}` gets skills only in `.claude/skills/`;
   - `{codex, gemini-cli, cursor}` gets skills only in `.agents/skills/`;
   - `{claude-code, codex, cursor}` needs both, so Cursor lists the skill twice.
3. **MCP.** `copilot` writes the same `.mcp.json` entry as `claude-code`, once.

Outputs of one asset that land in the same place, the same file or the same entry in it, are then
merged and attributed to all their clients.

Some refusals depend on placement. Cursor and Copilot take a rule's globs as one comma-separated
list, so a glob that contains a comma can't be written there (`adapter/unsupported`). That error
counts only when the client keeps its own rule file; when the rule is in `AGENTS.md`, the glob goes
in the scope line instead.

### What `plan` reports

These are warnings, never silent:

- `plan/duplicate-rule`: a client selected for a rule that may still read it twice.
  - Copilot CLI may read `AGENTS.md` and the `CLAUDE.md` import, or `.claude/rules/` and
    `.github/instructions/` (it deduplicates only identical copies).
  - Cursor may read `AGENTS.md` and the `CLAUDE.md` import.
- `plan/duplicate-skill`: Cursor, selected for a skill, seeing it in both skill folders.
- `plan/cross-client-rule`: an `AGENTS.md` rule not installed for Claude Code or Gemini CLI that
  they read anyway, through Grimoire's import in `CLAUDE.md` or `GEMINI.md` (or, for Claude Code,
  natively when there is no `CLAUDE.md`).
- `plan/cross-client-agent`, `plan/cross-client-hooks`, `plan/cross-client-mcp`: a
  `.claude/agents/` agent, a `.claude/settings.json` hook or a `.mcp.json` server that Cursor,
  Copilot or Claude Code also loads.
  - Installing for one client can grant something in another.
  - Cursor has no per-tool allow-list, and may run a `.claude/agents/` agent with every tool.
  - Cursor and Copilot run `.claude/settings.json` hooks.
  - [Cross-client leaks](#cross-client-leaks) covers these.
- `plan/codex-instructions-size`: a planned `AGENTS.md` of 24 KiB or more when `codex` is a
  project client, because Codex reads at most 32 KiB of project instructions in total. Phase 4
  settled the 24 KiB threshold.
- `plan/codex-override`: a root `AGENTS.override.md` when `codex` is a project client, because
  Codex then reads it instead of `AGENTS.md`.

The CLI adds the two Codex warnings itself, in its shared pipeline
([`cli.md`](cli.md#the-shared-pipeline)), because they depend on the planned size of `AGENTS.md`
and on the disk. `render` returns the rest in `Rendered::warnings`, with
`adapter/agent-readonly`, `adapter/agent-strict-tools` and `adapter/hook-tools`: an agent or hook
that a client limits less strictly than the asset declares, or that may rarely run. Every code is
in [`../diagnostics.md`](../diagnostics.md#planning-plan).

Other cases where a client reads Grimoire's content without being selected for it are not
reported. An example is Cursor reading an `AGENTS.md` rule installed only for `agents-md`.

## Per-client mapping

The client order is ADR-0005's, amended by ADR-0008. ADR-0005 set the order in which the client
adapters were built, and what each one covers. ADR-0008 amended it after Phase 4 re-checked the
formats: hooks stay Claude Code only, Codex gets no agents, and Cursor and Copilot read shared
rules from `AGENTS.md`.

| Kind | `claude-code` | `agents-md` (portable) | `codex` | `cursor` | `gemini-cli` | `copilot` |
|---|---|---|---|---|---|---|
| Rule, always-on | see [placement](#placement) | `AGENTS.md` block | `AGENTS.md` block (shared) | `AGENTS.md` when written, else `.cursor/rules/<id>.mdc` `alwaysApply: true` | `@AGENTS.md` import block in `GEMINI.md` when `AGENTS.md` is written, else a `GEMINI.md` block | `AGENTS.md` when written, else `.github/instructions/<id>.instructions.md` `applyTo: "**"` |
| Rule, path-scoped | `.claude/rules/<id>.md` `paths` | `AGENTS.md` block + scope line | same | `.mdc` with `alwaysApply: false` and `globs` | block + scope line | `.instructions.md` `applyTo` |
| Skill | `.claude/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` | shared ([placement](#placement)) | `.agents/skills/<id>/` | shared ([placement](#placement)) |
| Agent | `.claude/agents/<id>.md` with explicit `tools` | – | – (a Codex agent can't be restricted) | `.cursor/agents/<id>.md` `readonly: true`, only for an agent with exactly `read` and `search` | `.gemini/agents/<id>.md` with explicit `tools` | `.github/agents/<id>.agent.md` with `tools` and `strict-tools-list: true` |
| MCP server | `.mcp.json` entry, `${VAR}` | – | `.codex/config.toml` `[mcp_servers.<id>]`, `env_vars` | `.cursor/mcp.json` entry, `${env:VAR}` | `.gemini/settings.json` entry, `${VAR}` | `.mcp.json` entry (shared with Claude Code); cloud agent: `doctor` prints the settings JSON |
| Tool hooks | `.claude/settings.json` `hooks` | – | – | – | – | – |

Hooks are Claude Code only, and Codex has no agents (ADR-0008):

- no other client can keep a failed hook from letting the action through, three run hooks only
  through a shell, and none could be tested here;
- a Codex agent inherits the parent's sandbox, MCP servers and web search, and its one switch
  (`features.shell_tool = false`) removes shell access but leaves edits.

Cursor gets only read-only agents: a Cursor agent has no tool list, only `readonly: true`, so the
`cursor` adapter writes an agent only when its tools are exactly `read` and `search`.

An asset that declares those outputs fails validation (`manifest/anchor-client`). In a project that
also selects such a client, an asset it can't host is installed for the others, with a warning
(`resolve/client-not-covered`). For example, the warning says that `secret-guard` doesn't guard
Codex.

Project-scoped content often loads only in a project the user has marked as trusted in that
client:

- Codex `.codex/config.toml` (and so its MCP servers);
- everything Gemini CLI reads from the project;
- Copilot's `.mcp.json`.

`doctor` says so for each selected client, because Grimoire can't check or grant trust
([`cli.md`](cli.md#doctor)).

### `claude-code` details as implemented in Phase 2

These rely on [`CC-P2`](../specs.md#cc-p2-what-the-claude-code-adapter-relies-on).

| Output | Written as |
|---|---|
| Rule | The rule text. A path-scoped rule first gets `paths` frontmatter, a YAML list of quoted globs. |
| Agent | Frontmatter `name` (the id), `description` (the manifest `description`, the delegation text) and `tools`, all JSON-quoted so YAML never reads `true` or `123` as non-strings. |
| MCP server | `mcpServers.<id> = {"type": "stdio", "command": "<binary>", "env": {"VAR": "${VAR}"}}`. |
| Hook binding | `hooks.<Event>[] = {"matcher": …, "hooks": [{"type": "command", "command": "<binary>", "args": […], "timeout": n, "onFailure": "block"}]}`. `onFailure` only with `on-failure = "block"` (Phase 3). |

- **Agent tool mapping:**
  - `read` → `Read`; `search` → `Grep`, `Glob`; `edit` → `Edit`, `Write`, `NotebookEdit`;
    `execute` → `Bash`, `PowerShell`; `web` → `WebFetch`, `WebSearch`.
  - `mcp:<id>` → `mcp__<id>__*`, and `mcp:<id>/<tool>` → `mcp__<id>__<tool>`.
  - An agent with no capabilities gets `tools: ["Read"]` with `disallowedTools: ["Read"]`, the
    documented way to grant no tools.
- **MCP servers:**
  - A server started with no arguments speaks stdio, which every Grimoire server must support.
  - Optional variables are written `${VAR:-}`, with an empty default, because an unset variable
    without a default reaches the server literally, as the text `${VAR}`.
  - `env` overrides map the server's variable to the user's name (`${GH_TOKEN}`).
  - With `transport = "streamable-http"`, the entry is `{"type": "http", "url": "<url>"}`
    (Phase 3; [`model.md`](model.md#grimoiretoml)).
- **Hook bindings:**
  - They use exec form: setting `args` tells Claude Code to start the binary directly, with no
    shell involved ([`CC-P3`](../specs.md#cc-p3-claude-code-facts-for-phase-3)).
  - Matchers are an exact `A|B` list, or an anchored `^(…)$` regex when an MCP tool is involved.

### Phase 4 adapters

These rely on the `*-P4` entries in [`../specs.md`](../specs.md):
[`CODEX-P4`](../specs.md#codex-p4-releases-agentsmd-and-project-trust),
[`CODEX-MCP-P4`](../specs.md#codex-mcp-p4-mcp_serversid-in-codexconfigtoml),
[`CURSOR-P4`](../specs.md#cursor-p4-cursor-facts-for-phase-4),
[`GEMINI-P4`](../specs.md#gemini-p4-gemini-cli-facts-for-phase-4) and
[`COPILOT-P4`](../specs.md#copilot-p4-github-copilot-facts-for-phase-4).

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

How each entry is edited in its file (strict JSON, JSONC or TOML) is in
[`install.md`](install.md#three-entry-formats-chosen-by-path).

### Errors

When any asset can't be rendered, `render` returns every diagnostic at once, errors and warnings,
sorted and without duplicates. Clients that share a renderer, such as `claude-code` and `copilot`,
which write the same `.mcp.json` entry, would otherwise report the same problem once each. The
error codes are:

- `adapter/unsupported`: a client can't express what the asset needs, such as a Cursor agent whose
  tools aren't exactly `read` and `search`, a Copilot agent prompt over 30,000 characters, or an
  MCP tool name that Claude Code would rename or drop;
- `adapter/source`: a source file of the asset is missing or isn't UTF-8;
- `adapter/env-unknown`: an `env` override in `grimoire.toml` names a variable the server doesn't
  read, or one the client sets itself;
- `adapter/env-remap`: Codex can't pass a variable under another name;
- `adapter/entry-key`: the key recorded for a Claude Code hook entry couldn't be formed;
- `adapter/output-clash`: one asset would write different content to the same place for two
  clients, which points to a bug in an adapter.

`cargo xtask validate` renders each catalog asset alone for each client it declares, so the
embedded catalog can't cause `adapter/unsupported` or `adapter/source` in a user's project
([`catalog.md`](catalog.md#catalog-validation)).

## Cross-client leaks

Clients read each other's files, as the [read matrix](#client-read-matrix-and-cross-client-planning)
shows. So content installed for one client can reach another that the user didn't select for it,
or didn't select at all. This is threat F15 in
[`../01-architecture.md`](../01-architecture.md#threats-and-mitigations).

This table, with the `CLAUDE.md` note below it, lists every location Grimoire writes that another
of the six clients is documented as reading, and whether `plan` says so. The warning codes are in
[`../diagnostics.md`](../diagnostics.md#planning-plan).

| Grimoire writes | For | Also read by | What that client gets | Warned? |
|---|---|---|---|---|
| Hooks in `.claude/settings.json` | a tool, for `claude-code` | Cursor (on by default); Copilot CLI and VS Code | The tool's binary runs on their events. Whether they pass `args` and honor `onFailure` is undocumented, so a guard may let actions through there, or block every tool call ([`assets.md`](assets.md#tool-hooks)). | Always: `plan/cross-client-hooks` |
| An agent in `.claude/agents/` | `claude-code` | Cursor, which has no per-tool allow-list; Copilot CLI and VS Code, where the mapping of Claude Code tool names and `disallowedTools` is undocumented | An agent that may have more tools than it declares, up to all of the parent agent's in Cursor. When the agent is installed for `cursor` too, Cursor prefers its `readonly` copy in `.cursor/agents/` (same name). When it is installed for `copilot` too, Copilot CLI prefers `.github/agents/`; which copy VS Code uses is undocumented. | Always: `plan/cross-client-agent` |
| An entry in `.mcp.json` | an MCP server, for `claude-code` or `copilot` (one shared entry) | the other of the two: Claude Code, once the user approves the server; Copilot CLI, after folder trust, and VS Code | A server it can start, with the environment variables the entry forwards | `plan/cross-client-mcp`, whenever the entry is not attributed to both |
| A block in `AGENTS.md` | a rule, for `agents-md` or `codex` | Claude Code, through Grimoire's `@AGENTS.md` block in `CLAUDE.md`, or on its own when the project has no `CLAUDE.md`; Gemini CLI, through Grimoire's block in `GEMINI.md`, or when the user's `context.fileName` names `AGENTS.md`; Codex; Cursor; Copilot | The rule's text, always in context | `plan/cross-client-rule` for Claude Code when the `CLAUDE.md` import is planned or another asset is installed for `claude-code`, and for Gemini CLI when the `GEMINI.md` import is planned. Never for Codex, Cursor or Copilot, nor for Claude Code in a project that installs nothing for it |
| A block in `GEMINI.md` | a rule, for `gemini-cli`, when no `AGENTS.md` block is written for it | Copilot CLI and Copilot cloud agent | The rule's text | No |
| A file in `.claude/rules/` | a rule, for `claude-code` | Copilot CLI; VS Code with its Claude harness | The rule's text | No |
| A skill in `.claude/skills/` | `claude-code`, and Cursor's and Copilot's copy when no `.agents/skills/` copy is written | Cursor; Copilot | The skill's instructions and every file it ships, scripts included | No |
| A skill in `.agents/skills/` | `agents-md`, `codex` or `gemini-cli`, and Cursor's and Copilot's copy otherwise | Codex; Cursor; Gemini CLI, in trusted folders; Copilot | The same | No |

- **When a client is selected too, it's a duplicate, not a leak.** Then `plan/duplicate-rule` and
  `plan/duplicate-skill` report a client that may read the same rule or skill twice.
- **Consent is per asset, not per client** (F14, in
  [`../01-architecture.md`](../01-architecture.md#threats-and-mitigations)). Accepting `shell` for
  a skill installed for Claude Code also covers the copy Cursor and Copilot read.
- **No other location is shared.**
  - No other of the six clients is documented as reading `.cursor/rules/`, `.cursor/agents/`,
    `.cursor/mcp.json`, `.gemini/agents/`, `.gemini/settings.json`, `.github/instructions/`,
    `.github/agents/` or `.codex/config.toml`.
  - Codex and Gemini CLI aren't documented as reading any `.claude/` location, `CLAUDE.md` or
    `.mcp.json`.
  - Cursor and Copilot read Grimoire's `@AGENTS.md` block in `CLAUDE.md` too, but it gives them at
    most `AGENTS.md` again (whether Cursor expands the import is undocumented), which the
    `AGENTS.md` row covers. When either client is selected, `plan/duplicate-rule` reports the
    second read.
- **What isn't warned stays a risk.** Rules carry no permissions, and skills can't pre-approve
  tools (let them run without asking the user;
  [`assets.md`](assets.md#content-that-reaches-the-model)). So what an unwarned rule or skill gives
  another client is text the model reads, and scripts the agent can run with whatever tools that
  client allows. That is the prompt-injection surface, in a client the user may not have chosen.
  (Prompt injection is text that the model reads and that is written to make the agent do
  something the user didn't ask for.) Reviewing catalog content
  ([`../04-quality.md`](../04-quality.md#the-repository-and-contributions)) is the control.

## Invariants

1. Rendering is deterministic: the same resolution always gives the same outputs and warnings,
   in the same order.
2. When `agents-md` or `codex` puts a rule in `AGENTS.md`, `cursor` and `copilot` get no rule file
   of their own, and `claude-code` and `gemini-cli` read it through Grimoire's `@AGENTS.md` block.
3. Skills are written to the folders the placement rules name, and move when a client that needs
   `.agents/skills/` is added or dropped.
4. A selected client that may still read a rule or skill twice gets `plan/duplicate-rule` or
   `plan/duplicate-skill`.
5. A hook, an agent or an MCP server entry that another client also loads is always warned
   (`plan/cross-client-hooks`, `plan/cross-client-agent`, `plan/cross-client-mcp`), and so is an
   `AGENTS.md` rule that Claude Code or Gemini CLI reads without being selected for it.
6. A client that can't hold what an asset declares gets nothing: a Cursor agent whose tools aren't
   exactly `read` and `search`, a comma glob in a Cursor or Copilot rule file, a renamed variable
   for Codex, or an MCP tool name Claude Code or Gemini CLI would change.
7. An `env` override that names a variable the server doesn't read is refused, and reported once.
8. Hooks render only for `claude-code`, in exec form, with `onFailure: "block"` when the binding
   declares `on-failure = "block"`; matchers are exact lists or anchored regexes.
9. Every frontmatter value Grimoire writes parses as YAML and stays a string.
10. Every capability maps to at least one tool name for Claude Code and for Gemini CLI.
11. A lock may record only locations an adapter writes, as the unit `managed_unit` gives for that
    path.
12. A server configured over Streamable HTTP gets an entry with only its URL.
13. Every asset in the embedded catalog renders for every client it declares.
14. The Codex warnings fire for an `AGENTS.md` of 24 KiB or more and for an `AGENTS.override.md`,
    only when `codex` is a project client.

## Tests

| Invariant | Tests |
|---|---|
| 1 | The insta snapshot tests in `crates/agent-grimoire-adapters/tests/adapters.rs`, one per client set: `claude_code_alone`, `agents_md_alone`, `claude_code_and_agents_md`, `codex_alone`, `cursor_alone`, `gemini_cli_alone`, `copilot_alone`, `claude_code_cursor_and_copilot` and `every_client`, with their snapshots in `tests/snapshots/` |
| 2 | The snapshots `claude_code_and_agents_md` and `every_client`; the crate-docs example in `src/lib.rs` (a doc test); `dropping_a_client_removes_its_outputs_and_the_import` in `crates/agent-grimoire/tests/cli.rs`; `a_lock_may_name_grimoires_gemini_import` and `clients_can_be_dropped_and_added_back` in `crates/agent-grimoire/tests/clients.rs` |
| 3 | The snapshot `claude_code_cursor_and_copilot`; `skills_move_when_codex_comes_and_goes` in `clients.rs` |
| 4 | The snapshots `claude_code_cursor_and_copilot` and `every_client`, which record the `plan/duplicate-rule` and `plan/duplicate-skill` warnings |
| 5 | The snapshots `claude_code_alone`, `claude_code_and_agents_md`, `copilot_alone` and `every_client`; `claude_code_reading_agents_md_only_rules_is_reported` and `gemini_reading_agents_md_only_rules_is_reported` in `adapters.rs` |
| 6 | `cursor_refuses_agents_it_cannot_limit`, `comma_globs_are_refused_where_globs_are_comma_separated`, `comma_globs_are_fine_when_agents_md_covers_the_client`, `codex_refuses_env_remaps`, `codex_accepts_an_override_that_renames_nothing`, `mcp_tool_names_claude_code_would_rename_are_refused` and `gemini_refuses_tool_names_it_would_shorten` in `adapters.rs` |
| 7 | `unknown_env_overrides_are_refused` and `an_unknown_env_override_is_reported_once` in `adapters.rs` |
| 8 | `fail_closed_hooks_render_on_failure_block` and `search_hooks_are_flagged` in `adapters.rs`; `matchers_are_exact_lists_or_anchored_regexes` in `src/claude_code.rs` |
| 9 | `frontmatter_is_valid_yaml_with_string_values` and `new_frontmatter_is_valid_yaml` in `adapters.rs`; `yaml_values_are_quoted` and `frontmatter_layout` in `src/text.rs` |
| 10 | `tool_names_cover_every_capability` in `src/claude_code.rs` and in `src/gemini_cli.rs` |
| 11 | `managed_locations` and `new_managed_locations` in `adapters.rs`; `locks_naming_unmanaged_paths_are_refused` in `cli.rs`; `a_lock_may_not_claim_a_whole_config_file` in `clients.rs` |
| 12 | `streamable_http_entries_carry_only_the_url` in `adapters.rs` |
| 13 | `every_exemplar_installs_for_every_client` in `crates/agent-grimoire/tests/builtin.rs`; `cargo xtask validate`, which renders every catalog asset for every client it declares |
| 14 | `codex_warns_when_agents_md_nears_its_budget` and `codex_warnings_cover_every_block_and_the_override` in `clients.rs` |

`every_client_installs_verifies_and_uninstalls` in `clients.rs` installs, verifies and removes
outputs for all six clients end to end. When a snapshot changes, read its diff before accepting it
([`../04-quality.md`](../04-quality.md#test-tools)).
