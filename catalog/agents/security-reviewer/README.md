# security-reviewer

A Claude Code subagent that reviews code, diffs and pull requests for security problems and reports each finding with its location, attack path, impact, severity, confidence and a suggested fix.

## When to use it

- Before merging a change that touches input handling, authentication, file or process access, cryptography, dependencies or CI.
- When you want an independent read of specific files without giving the reviewer the ability to change them.

Ask the main session to "use the security-reviewer agent on this diff", or let it delegate on its own when you ask for a security review.

## What it can do

Its tool allow-list is `read` and `search` only. On Claude Code that renders as `Read`, `Grep` and `Glob`. It cannot edit files, run commands or use the network, so it cannot confirm a finding by running code; it says when a finding needs dynamic verification.

## Security notes

- **Prompt injection.** The prompt tells the agent to treat every file as data and to report text that tries to steer the review. This lowers the risk; it does not remove it. Review its findings, especially a report of "no findings" on code that you did not expect to be clean.
- **Secrets.** The agent is told never to quote a secret and to redact any it finds.
- **Other clients.** Cursor also reads `.claude/agents/` but has no per-tool allow-list, so there the agent runs with Cursor's default tools. `grimoire plan` warns about this.

## Permissions

- `filesystem = "read"`: the `read` and `search` capabilities read files.
