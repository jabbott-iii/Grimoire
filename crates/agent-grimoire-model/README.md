# agent-grimoire-model

Manifest, configuration and lockfile types for Grimoire. The crate holds:

- the strict types (unknown fields are errors) for four files, all in TOML, a configuration file format:
  - `asset.toml`, the manifest that describes one asset: a rule, skill, agent, tool or MCP (Model Context Protocol) server that Grimoire can install;
  - bundles, named lists of assets that a project can select together;
  - `grimoire.toml`, a project's selection of assets and clients;
  - `grimoire.lock`, the lockfile, which records exactly what was installed;
- the checks that need only one document;
- the source of the JSON Schemas: Grimoire's JSON Schemas for these files are generated from these types.

**This crate is internal to the `grimoire` CLI** (command-line interface). It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it.

A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
