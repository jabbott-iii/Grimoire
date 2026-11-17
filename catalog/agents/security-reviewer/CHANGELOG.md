# Changelog

## 0.2.0

- Supports Cursor (as a `readonly` agent), Gemini CLI and GitHub Copilot, each limited to reading and searching as closely as the client allows: Cursor's `readonly` still allows shell commands it judges harmless, and VS Code may add Copilot's default tools. Codex isn't supported, because a Codex agent can't be limited to its tools.
- The prompt now says the agent must not edit files, run commands or use the network, rather than that it can't, since not every client enforces that.

## 0.1.0

- First release: read-only security review with an explicit evidence standard and report format.
