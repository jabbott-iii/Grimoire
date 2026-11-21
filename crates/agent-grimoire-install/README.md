# agent-grimoire-install

Root-confined, atomic installer for Grimoire outputs: the units of content Grimoire owns in a project, each a whole file, a managed block or a configuration entry.

- **Root-confined:** every read and write stays inside the project root, the directory that holds `grimoire.toml`. A path can't leave the root even if the directory tree changes during the operation, for example when a directory is swapped for a symbolic link.
- **Atomic:** each file is written to a temporary file in the same directory and then renamed over the target, so a file is either replaced whole or left as it was.

The crate holds:
- the filesystem layer that stays inside the project root;
- managed blocks (sections of a shared Markdown file, such as `AGENTS.md`, that Grimoire owns between its marker lines) and configuration entries (keys or array elements Grimoire owns in JSON, JSONC and TOML files; JSONC is JSON with comments, and TOML is a configuration file format);
- the planner, which works out what to change;
- `apply`, which makes the changes, and `verify`, which checks the files on disk against `grimoire.lock`, the record of what was installed.

**This crate is internal to the `grimoire` CLI** (command-line interface). It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API is internal and changes whenever the CLI needs it to, so don't depend on it.

A change that breaks it is marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

To install assets into a repository, use the CLI: see [Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README.

[Grimoire](https://github.com/jabbott-iii/Grimoire) versions its crates together; [docs/versioning.md](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md) has the policy.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
