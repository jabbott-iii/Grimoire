# security-reviewer

A subagent that reviews code, diffs and pull requests for security problems and reports each finding with its location, attack path, impact, severity, confidence and a suggested fix.

## When to use it

- Before merging a change that touches input handling, authentication, file or process access, cryptography, dependencies or CI.
- When you want an independent read of specific files without giving the reviewer the ability to change them.

Ask the main session to "use the security-reviewer agent on this diff", or let it delegate on its own when you ask for a security review.

## What it can do

Its tool allow-list is `read` and `search` only, and its prompt tells it never to edit files, run commands or use the network. In Claude Code and Gemini CLI its tools allow nothing else; Cursor and Copilot limit it less strictly (below). It does not confirm a finding by running code; it says when a finding needs dynamic verification.

| Client | File | Tools |
|---|---|---|
| `claude-code` | `.claude/agents/security-reviewer.md` | `Read`, `Grep`, `Glob` |
| `cursor` | `.cursor/agents/security-reviewer.md` | `readonly: true`: no edits and no state-changing shell commands |
| `gemini-cli` | `.gemini/agents/security-reviewer.md` | `read_file`, `read_many_files`, `grep_search`, `glob`, `list_directory` |
| `copilot` | `.github/agents/security-reviewer.agent.md` | `read`, `search`, with `strict-tools-list: true` |

Codex isn't supported: a Codex agent inherits the main session's sandbox, MCP servers and web search, so it can't be limited to reading.

## Security notes

- **Prompt injection.** The prompt tells the agent to treat every file as data and to report text that tries to steer the review. This lowers the risk; it does not remove it. Review its findings, especially a report of "no findings" on code that you did not expect to be clean.
- **Secrets.** The agent is told never to quote a secret and to redact any it finds.
- **Cursor.** `readonly: true` is documented as blocking edits and state-changing shell commands, so shell commands Cursor judges harmless stay available; whether it also blocks MCP tools and web fetches is undocumented. Cursor also reads `.claude/agents/`, and how it treats the tool list there is undocumented; when `cursor` is selected, the `.cursor/agents/` copy takes precedence.
- **Copilot.** VS Code doesn't pass `strict-tools-list` on, so there the agent may also get Copilot's default tools. VS Code also reads the `.claude/agents/` copy.
- **Gemini CLI.** It asks you to acknowledge a project agent before using it, and again after every change to its file.

`grimoire plan` warns about the Cursor and Copilot points; `grimoire doctor` notes Gemini CLI's acknowledgement.

## Permissions

- `filesystem = "read"`: the `read` and `search` capabilities read files.
