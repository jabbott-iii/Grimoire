# agent-grimoire-adapters

Client adapters that render Grimoire assets: one adapter per supported client (`claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli`, `copilot`) and the planner that places shared content once across the selected clients.

- An asset is one rule, skill, agent, tool or MCP (Model Context Protocol) server from Grimoire's catalog.
- A client is an AI coding tool that Grimoire installs assets for, and its adapter is the code that turns an asset into that client's own files and layout.
- The planner puts content that several clients can read in one shared place, so it is written only once. For example, when `agents-md` or `codex` puts a rule in `AGENTS.md`, the other selected clients that read that file get no copy of their own.

**This crate is internal to the `grimoire` CLI** (command-line interface). It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it.

A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
