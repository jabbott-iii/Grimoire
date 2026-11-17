# ADR-0005: v1 client adapters and their order

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16); amended by [ADR-0008](0008-phase-4-client-adapters.md) (2026-11-17): hooks stay Claude Code only, Codex gets no agents, and Cursor and Copilot read shared rules from `AGENTS.md`
- **Date:** 2026-11-16
- **Specs:** `CC-*`, `AGENTSMD`, `SKILLS`, `CODEX-*`, `CURSOR-*`, `GEMINI-*`, `COPILOT-*` in [`../specs.md`](../specs.md); the read matrix in [`../architecture.md`](../architecture.md) §8.2

## Context

The maintainer leans toward Claude Code and generic AGENTS.md first, then Codex, Cursor, Gemini CLI and Copilot. The verified specs (2026-11-16) show three things that bear on that order.

**1. The clients converge heavily.**
- `AGENTS.md` is read natively by Codex, Cursor and Copilot (cloud agent, CLI and VS Code). Claude Code reads it only when there is no `CLAUDE.md`. Gemini CLI reads it only if `context.fileName` is set.
- `.agents/skills/` is read by Codex, Cursor, Gemini CLI and Copilot. `.claude/skills/` is read by Claude Code, Cursor and Copilot.
- `.mcp.json` is read by Claude Code, and by Copilot in its CLI and its VS Code native harness.

**2. Some clients can't enforce agent tool restrictions.**
- Cursor subagents have only `readonly`.
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
| 1 | `claude-code` | 2 | rules (`.claude/rules/`, plus the `CLAUDE.md` block for `@AGENTS.md` when paired), skills, agents, MCP (`.mcp.json`), tool hooks (`.claude/settings.json`) |
| 2 | `agents-md` (portable) | 2 | rules as `AGENTS.md` blocks; skills in `.agents/skills/` (the cross-client convention in `SKILLS`). No agents, MCP or hooks, because there is no portable format for them |
| 3 | `codex` | 4 | rules and skills via the shared locations; MCP as `[mcp_servers.<id>]` in `.codex/config.toml` (the first TOML-entry adapter); agents as `.codex/agents/<id>.toml` only when the restriction is enforceable; hooks |
| 4 | `cursor` | 4 | rules as `.cursor/rules/<id>.mdc`; skills via shared locations; MCP in `.cursor/mcp.json` (`${env:}` syntax); `readonly`-expressible agents; hooks |
| 5 | `gemini-cli` | 4 | rules as `GEMINI.md` blocks; skills in `.agents/skills/`; MCP in `.gemini/settings.json` (`httpUrl`); agents (preview); hooks |
| 6 | `copilot` | 4 | rules as `.github/instructions/*.instructions.md`; skills via shared locations; agents as `.github/agents/<id>.agent.md`; MCP in `.mcp.json` (shared with Claude Code); hooks as `.github/hooks/grimoire.json` |

This keeps the maintainer's order.

- **Phase 2** builds the full install engine against the two clients that exercise every output unit (file, block, JSON entry) and the cross-client planner (the `CLAUDE.md` ↔ `AGENTS.md` interplay).
- **Phase 4** adds the rest in increasing order of surface fragmentation:
  - Codex brings TOML entries and trust-gated config.
  - Cursor brings a new rule format and a new env syntax.
  - Gemini brings settings-embedded MCP and hooks.
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

- **Snapshot tests by combination.** Adapter snapshot tests must cover client *combinations*, not just single clients, because outputs depend on the selected set (`architecture.md` §8.2).
- **Duplicates and leaks.** Where reaching every selected client once is impossible, `plan` reports the duplicates. Cross-client permission leaks are documented in `docs/security-model.md` (Phase 7). For example, Cursor reads `.claude/agents/` without enforcing tools.
- **Re-verification.** Every adapter re-verifies its `specs.md` entries at the start of its phase, because these formats change monthly.
