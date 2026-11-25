# ADR-0008: What the Codex, Cursor, Gemini CLI and Copilot adapters write

- **Status:** Accepted (the maintainer chose the hook and rule policies when asked, 2026-11-17, during Phase 4)
- **Date:** 2026-11-17
- **Specs:** `CODEX-*-P4`, `CURSOR-P4`, `GEMINI-P4`, `COPILOT-P4` in [`../specs.md`](../specs.md), the register of verified external specs
- **Amends:** [ADR-0005](0005-client-priority.md), which planned hooks and agents for every client before these formats were checked in detail

## Context

A client is an AI coding tool Grimoire writes files for, and an adapter is the code that writes one client's files. ADR-0005, an earlier architecture decision record (ADR), put four more clients into v1: `codex`, `cursor`, `gemini-cli` and `copilot`.

The Phase 4 research re-checked every format they read, and found three problems with writing everything ADR-0005 listed. (Phase 4 is the phase of `roadmap.md` that added these adapters; its findings are the `*-P4` entries in `specs.md`.)

**1. No new client can keep a fail-closed hook closed.** A fail-closed hook blocks the action when the hook itself fails. (Cursor has an opt-in `failClosed: true`, in the table below, but it couldn't be tested here.) Grimoire's guard hooks, such as `secret-guard`, which blocks tool calls and prompts that contain likely secrets, rely on Claude Code's `onFailure: "block"`, so a missing or broken binary blocks the event.

| Client | When the hook fails (missing binary, crash, timeout) | How the command runs | Other limits |
|---|---|---|---|
| Codex | the action goes ahead; no fail-closed option exists | always through a shell (bash, zsh, sh, PowerShell or cmd) | each user must trust every project hook by hash before it runs; edits arrive as raw patch text |
| Cursor | goes ahead unless `failClosed: true` | a shell string | no event before a file edit; many payloads undocumented |
| Gemini CLI | goes ahead | `bash -c` or PowerShell, after `$VAR` expansion that can't be escaped | a different reply protocol |
| Copilot | a timeout always goes ahead; the cloud agent (the Copilot agent that runs on GitHub) runs only shell strings | `exec` + `args` in the command-line interface (CLI) and VS Code, shell strings on the cloud agent | prompt hooks in files can't block |

None of these clients can be run here, so none of this can be tested.

**2. Not every client can enforce an agent's tool list** (the tools an agent may use).
- A Codex custom agent inherits the parent's sandbox, approvals, Model Context Protocol (MCP) servers and web search. The only switch, `features.shell_tool = false`, removes shell access but leaves edits. A read-only agent can't be written.
- A Cursor agent has no tool list, only `readonly: true`: "no file edits, no state-changing shell commands". What `readonly` does to MCP tools and web fetches is undocumented.
- Gemini CLI takes an explicit `tools` list, and `tools: []` leaves only `complete_task`.
- Copilot takes a `tools` list of aliases. The CLI adds unnamed default tools unless `strict-tools-list: true` is set, and VS Code doesn't pass that key on.

**3. Rules reach several clients through shared files.**
- Cursor always applies `AGENTS.md`.
- Copilot CLI and VS Code's Copilot harness (the agent runtime VS Code uses for Copilot) read it. So does the cloud agent, though whether the cloud agent still does when a root `CLAUDE.md` exists is undocumented.
- Copilot CLI also reads `CLAUDE.md` and expands its `@AGENTS.md` line (an import that pulls in `AGENTS.md`).

A rule written to `AGENTS.md` and to a client's own rule folder is read twice by that client.

## Decision

**Hooks stay Claude Code only** (the maintainer's choice).
- The `codex`, `cursor`, `gemini-cli` and `copilot` clients have no `hooks` anchor, so an asset that declares hook outputs for them fails validation (`manifest/anchor-client`). (An anchor is a named destination, such as `hooks` or `skills:`, that each adapter maps to a place in its client's layout.)
- `plan` keeps warning that Cursor and Copilot also run the hooks Grimoire writes to `.claude/settings.json` (`plan/cross-client-hooks`).
- Hooks for the other clients come back when they can be tested against real clients.

**Agents are written only where the client limits the declared tools, as closely as it can:**

| Client | Agents |
|---|---|
| `codex` | None. The client has no `agents` anchor. |
| `cursor` | Only an agent whose tools are exactly `read` and `search`, written as `readonly: true`. Anything else is refused (`adapter/unsupported`). `readonly` blocks edits and state-changing shell commands only, so shell commands Cursor judges harmless stay available, and its effect on MCP tools and web fetches is undocumented; `plan` says so (`adapter/agent-readonly`). |
| `gemini-cli` | An explicit `tools` list, with `tools: []` for an agent with no tools. |
| `copilot` | A `tools` list of aliases plus `strict-tools-list: true`, with `tools: []` for none. `plan` warns that VS Code ignores `strict-tools-list`. |

**Rules: `AGENTS.md` covers Cursor and Copilot** (the maintainer's choice). For each rule, when `agents-md` or `codex` is selected, the rule is written once to `AGENTS.md`, and:
- `cursor` and `copilot` get no rule file of their own; the `AGENTS.md` block is attributed to them (the lock records it as theirs too);
- `claude-code` reads it through the `@AGENTS.md` block in `CLAUDE.md`, as in Phase 2 (when the `claude-code` adapter was built);
- `gemini-cli` reads it through a Grimoire-owned `@AGENTS.md` block in `GEMINI.md`.

Otherwise each client gets its own file: `.cursor/rules/<id>.mdc`, `.github/instructions/<id>.instructions.md`, a `GEMINI.md` block, or `.claude/rules/<id>.md`.

A path-scoped rule (one meant only for matching files) in `AGENTS.md` carries its scope line, a line saying which files it applies to, because `AGENTS.md` has no path scoping. So Cursor and Copilot apply it everywhere, as Claude Code already does. Duplicates that remain are reported by `plan` (`plan/duplicate-rule`).

**MCP servers** go in each client's own configuration:
- `codex`: `[mcp_servers.<id>]` in `.codex/config.toml`, with `env_vars` naming the variables to forward. Codex has no `${VAR}` and can't forward a variable under another name, so an `env` override that renames a variable is refused (`adapter/env-remap`).
- `cursor`: `.cursor/mcp.json`, with `${env:NAME}` references.
- `gemini-cli`: `.gemini/settings.json`, with `${NAME}` references. That file is JSONC (JSON with comments), so Grimoire edits it keeping comments and still refuses trailing commas, which Gemini CLI rejects too.
- `copilot`: the same `.mcp.json` entry as `claude-code`, written once and attributed to both.

**Skills** are copied once per location.
- `.claude/skills/` is written when `claude-code` is selected.
- `.agents/skills/` is written when `agents-md`, `codex` or `gemini-cli` is selected, or when only `cursor` and `copilot` are.
- Cursor and Copilot read both locations.

## Consequences

**Positive:**
- No client gets a hook whose failure it can't stop, or an agent with a tool list it ignores altogether. Two gaps remain and `plan` reports both:
  - Cursor's `readonly` leaves harmless shell commands (and possibly MCP and web tools) available;
  - VS Code may add Copilot's default tools.
- A project that also selects a client an asset can't be installed for is told (`resolve/client-not-covered`), for example that `secret-guard` doesn't guard Codex or Gemini CLI.
- Each selected client reads a rule once wherever the formats allow it.
- The adapters share one placement pass (the step that decides which shared file each client reads content from), so a new client needs a read-matrix entry (a record of which files it reads) rather than special cases.

**Negative:**
- `secret-guard` protects only Claude Code. Codex, Gemini CLI and Copilot users get no Grimoire hooks yet. Cursor and Copilot run the Claude Code hooks, but whether they honor `args` and `onFailure` is unverified.
- The `security-reviewer` agent can't be installed for Codex.
- A path-scoped rule loses its scoping in Cursor and Copilot whenever `AGENTS.md` is written.
- Some duplicates can't be avoided, only reported. For example, Copilot CLI most likely reads a rule from `AGENTS.md` and again through the `CLAUDE.md` import. It deduplicates only identical copies, which is documented, but the double read itself is not tested.

**Follow-up:**
- Hooks for the four clients, once they can be run against real clients: shell-safe arguments, per-client reply protocols, Cursor's `failClosed`, and Codex's trust-by-hash review.
- Test the double reads that are documented as likely but unverified: Cursor expanding `@AGENTS.md` in `CLAUDE.md`, Copilot CLI's deduplication, and VS Code's handling of `strict-tools-list`.
- Test whether Copilot cloud agent reads `AGENTS.md` when a root `CLAUDE.md` exists. Until then, a Copilot rule that lives only in `AGENTS.md` may be missed there.
