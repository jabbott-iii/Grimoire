# rust-cli

Part of Grimoire's `rust-cli` bundle: installs the `adr-writer` and `project-docs` plugins. Left out: `conventional-commits` (a rule) is not published as a plugin; install it with the grimoire CLI. The whole bundle: Conventional Commits, architecture decision records and a read-only docs server for Rust CLI projects. Needs `grimoire-mcp-project-docs` on PATH.

Install the `grimoire-mcp-project-docs` binary first: neither Grimoire nor Claude Code installs it.

Installing this plugin also installs the plugins it depends on.

Generated from [`catalog/bundles/rust-cli.toml`](../../../catalog/bundles/rust-cli.toml) by `cargo xtask gen-marketplace`. Don't edit it here.
