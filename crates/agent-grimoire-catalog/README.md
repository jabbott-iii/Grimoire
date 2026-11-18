# agent-grimoire-catalog

Catalog loading and catalog-wide validation for Grimoire: loading a catalog and the checks that need more than one document, including native Agent Skills checks, and the catalog pack that is embedded in the CLI.

**This crate is internal to the `grimoire` CLI.** It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it. A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

The catalog is built into this crate, so install the CLI with `cargo install agent-grimoire --locked`. On Cargo 1.96 to 1.99, a build without `--locked` can take a later version of this crate within the same 0.y, and so newer assets than the CLI version it reports.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
