# agent-grimoire-catalog

Catalog loading and catalog-wide validation for Grimoire. The catalog is the collection of assets Grimoire can install: rules, skills, agents, tools and MCP (Model Context Protocol) servers. This crate holds:

- loading a catalog, and the checks that need more than one document;
- native Agent Skills checks: skills are checked against the Agent Skills specification (the open format that skills follow) in Rust, with the specification's stricter rules, rather than with its Python reference validator, `skills-ref`, which is "intended for demonstration purposes only";
- the catalog pack, the archive of the catalog that is embedded in the CLI.

**This crate is internal to the `grimoire` CLI** (command-line interface). It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it.

A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

The catalog is built into this crate, so install the CLI with `cargo install agent-grimoire --locked`. `--locked` makes Cargo use the `Cargo.lock` published with the CLI, which holds the version of this crate that the release was built with. On Cargo 1.96 to 1.99, a build without `--locked` can take a later version of this crate within the same 0.y, and so newer assets than the CLI version it reports.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
