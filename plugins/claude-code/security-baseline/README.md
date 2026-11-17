# security-baseline

Part of Grimoire's `security-baseline` bundle: installs the `security-reviewer` plugin. Left out: `secret-guard` (a tool) is not published as a plugin; install it with the grimoire CLI. The whole bundle: A read-only security reviewer agent plus hooks that keep likely secrets out of tool calls and prompts.

Installing this plugin also installs the plugins it depends on.

Generated from [`catalog/bundles/security-baseline.toml`](../../../catalog/bundles/security-baseline.toml) by `cargo xtask gen-marketplace`. Don't edit it here.
