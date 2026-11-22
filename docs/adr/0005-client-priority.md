# ADR-0005: v1 client adapters and their order

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16); amended by [ADR-0008](0008-phase-4-client-adapters.md) (2026-11-17): hooks stay Claude Code only, Codex gets no agents, and Cursor and Copilot read shared rules from `AGENTS.md`
- **Date:** 2026-11-16
- **Specs:** `CC-*`, `AGENTSMD`, `SKILLS`, `CODEX-*`, `CURSOR-*`, `GEMINI-*`, `COPILOT-*` in [`../specs.md`](../specs.md), the register of verified external specs; the read matrix (which client reads which file) in [`../architecture.md`](../architecture.md) §8.2

## Context

A client is an AI coding tool that Grimoire writes files for, and an adapter is the code that knows one client's layout. The maintainer leans toward Claude Code and generic AGENTS.md first, then Codex, Cursor, Gemini CLI and Copilot. (`AGENTS.md` is a plain Markdown instruction file that many of these tools read.) The verified specs (2026-11-16) show three things that bear on that order.

**1. The clients read many of the same files.**
- `AGENTS.md` is read natively by Codex, Cursor and Copilot. For Copilot, that means its cloud agent (the Copilot agent that runs on GitHub), its command-line interface (CLI) and VS Code. Claude Code reads it only when there is no `CLAUDE.md`. Gemini CLI reads it only if `context.fileName` is set.
- `.agents/skills/` is read by Codex, Cursor, Gemini CLI and Copilot. `.claude/skills/` is read by Claude Code, Cursor and Copilot.
- `.mcp.json`, the project's list of Model Context Protocol (MCP) servers, is read by Claude Code, and by Copilot in its CLI and its VS Code native harness (the Copilot agent runtime built into VS Code).

**2. Some clients can't enforce agent tool restrictions** (the list of tools an agent may use).
- Cursor subagents (agents the main agent hands work to) have only `readonly`.
- A Codex custom agent can't override the parent's sandbox or approval policy.
- Gemini subagents are a "preview feature".

**3. Copilot is the most fragmented client.**
- Seven surfaces with different support (VS Code, Visual Studio, JetBrains, Eclipse, Xcode, GitHub.com, CLI).
- Two VS Code MCP formats.
- Cloud-agent MCP that can only be configured in the repository Settings UI.

## Decision (proposed)

**v1 clients** (all six are required for the v1 definition of done):

| Order | Client ID | Phase | Covers |
|---|---|---|---|
| 1 | `claude-code` | 2 | rules (`.claude/rules/`, plus the `CLAUDE.md` block for `@AGENTS.md` when another selected client writes the rule to `AGENTS.md`), skills, agents, MCP (`.mcp.json`), tool hooks (`.claude/settings.json`) |
| 2 | `agents-md` (portable) | 2 | rules as `AGENTS.md` blocks (marked sections that Grimoire owns inside a shared file); skills in `.agents/skills/` (the cross-client convention in `SKILLS`). No agents, MCP or hooks, because there is no portable format for them |
| 3 | `codex` | 4 | rules and skills via the shared locations; MCP as `[mcp_servers.<id>]` in `.codex/config.toml` (the first adapter that writes entries into a TOML file, a configuration file format); agents as `.codex/agents/<id>.toml` only when the restriction is enforceable; hooks |
| 4 | `cursor` | 4 | rules as `.cursor/rules/<id>.mdc`; skills via shared locations; MCP in `.cursor/mcp.json` (`${env:}` syntax); agents whose restrictions `readonly` can express; hooks |
| 5 | `gemini-cli` | 4 | rules as `GEMINI.md` blocks; skills in `.agents/skills/`; MCP in `.gemini/settings.json` (`httpUrl`) *(Corrected 2026-11-22: for Streamable HTTP, Grimoire writes `url` with `"type": "http"`, not `httpUrl`, in `crates/agent-grimoire-adapters/src/gemini_cli.rs` and as [`architecture.md`](../architecture.md) §8.3 says. No ADR records this choice; ADR-0008 settled only the file. It follows GEMINI-P4 in [`specs.md`](../specs.md), which found that `gemini mcp add --transport http` writes this form and that Gemini CLI treats `httpUrl` as deprecated.)*; agents (preview); hooks |
| 6 | `copilot` | 4 | rules as `.github/instructions/*.instructions.md`; skills via shared locations; agents as `.github/agents/<id>.agent.md`; MCP in `.mcp.json` (shared with Claude Code); hooks as `.github/hooks/grimoire.json` |

This keeps the maintainer's order. The Phase column refers to the phases of the implementation plan in `PLAN.md`.

- **Phase 2** (the resolver, installer and CLI core) builds the full install engine against the two clients that exercise:
  - every output unit: a whole file, a block in a shared file, and an entry in a JSON file;
  - the cross-client planner (the code that writes content several clients read only once), through the `CLAUDE.md` ↔ `AGENTS.md` interplay.
- **Phase 4** adds the rest in increasing order of surface fragmentation (how scattered each client's formats are):
  - Codex brings TOML entries and trust-gated config (project config that Codex loads only after the user marks the project as trusted).
  - Cursor brings a new rule format and a new syntax for environment variables.
  - Gemini brings MCP servers and hooks embedded in its settings file.
  - Copilot comes last because it has the largest matrix to test.

**Out of scope for v1** (recorded so it isn't mistaken for an oversight):
- VS Code's deprecated `.vscode/mcp.json` "Local harness" format.
- Copilot cloud-agent MCP (repository Settings only). `doctor` prints guidance instead.
- Copilot prompt files.
- Gemini extensions and custom commands.
- Codex `execpolicy` rules.
- Cursor commands and plugins.
- Codex plugins.

## Consequences

- **Snapshot tests by combination.** Adapter snapshot tests (tests that compare output with a reviewed, stored copy) must cover client *combinations*, not just single clients, because outputs depend on the selected set (`architecture.md` §8.2).
- **Duplicates and leaks.**
  - Where reaching every selected client once is impossible, `plan` reports the duplicates.
  - Cross-client permission leaks, where content installed for one client grants something in another, are documented in `docs/security-model.md` (Phase 7, the documentation phase). For example, Cursor reads `.claude/agents/` without enforcing tools.
- **Re-verification.** Every adapter re-verifies its `specs.md` entries at the start of its phase, because these formats change monthly.
