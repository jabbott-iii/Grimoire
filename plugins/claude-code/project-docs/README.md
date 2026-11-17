# project-docs

A read-only MCP server that lists, reads and searches a project's docs and ADRs

Install the `grimoire-mcp-project-docs` binary first: neither Grimoire nor Claude Code installs it.

## Permissions

What the catalog declares. Installing this plugin grants it without asking, unlike `grimoire add`, which asks for each permission and each environment variable.

- Files: read
- Network: no
- Shell: no
- Environment variables read: `CLAUDE_PROJECT_DIR`

Generated from [`catalog/mcp-servers/project-docs/`](../../../catalog/mcp-servers/project-docs/README.md) by `cargo xtask gen-marketplace`. Don't edit it here.
