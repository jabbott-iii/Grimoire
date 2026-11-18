# agent-grimoire-install

Root-confined, atomic installer for Grimoire outputs: the filesystem layer that stays inside the project root, managed blocks and configuration entries, the planner, and `apply` and `verify`.

**This crate is internal to the `grimoire` CLI.** It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it. A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
