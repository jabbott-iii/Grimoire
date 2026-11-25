# agent-grimoire-install

Root-confined, atomic installer for Grimoire's outputs. It works out what to change in a project, makes those changes, and checks the result against `grimoire.lock`, the record of what was installed. Outputs are the units of content Grimoire owns in a project, each a whole file, a managed block (a marked section of a shared Markdown file such as `AGENTS.md`) or a configuration entry (one key in a JSON, JSONC or TOML file, or one array element in a JSON or JSONC file; JSONC is JSON with comments, and TOML is a configuration file format). Root-confined and atomic mean:

- **Root-confined:** every read and write stays inside the project root, the directory that holds `grimoire.toml`. A path can't leave the root even if the directory tree changes during the operation, for example when a directory is swapped for a symbolic link.
- **Atomic:** each file is written to a temporary file in the same directory, flushed to disk, and then renamed over the target, so a file is either replaced whole or left as it was.

It is part of [Grimoire](https://github.com/jabbott-iii/Grimoire): a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the command-line interface (CLI) that installs the assets a project selects. The CLI, in the `agent-grimoire` crate, reads and writes every project file through this crate, `grimoire.toml` and `grimoire.lock` included. It plans with it in `plan`, `apply`, `update`, `uninstall` and `doctor`, and in `add` and `remove` with `--apply`; it applies with it; and it checks the disk against the lock with it in `verify` and `doctor`. `agent-grimoire-adapters`, which turns the selected assets into each client's files (a client is an AI coding tool, such as Claude Code or Codex), describes those files in this crate's types. This crate builds on `agent-grimoire-model` for checked paths and the lock's types, and on [cap-std](https://crates.io/crates/cap-std) for file access that can't leave a directory.

## Status

Before 1.0, and internal to the CLI. It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API changes whenever the CLI needs it to, so don't depend on it: to install assets, use the CLI ([Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README).

A change that breaks it is still marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

What it does:

- **`fs::Root`, the project root**, is the only way to the project's files. It reads (`read`), writes (`write`, `write_like`) and removes (`remove`) files by `RelPath`, the model crate's checked relative path: ASCII, `/`-separated, with no `..` and no absolute or drive prefix. It refuses a symbolic link at the path or in any directory between the root and it, even a link that points inside the project, and a special file at the path, such as a named pipe. It refuses to read a file over 8 MiB (`MAX_READ_BYTES`). A replaced file keeps its permissions, and `remove` also deletes every directory it leaves empty.
- **`block::Document`** splits a Markdown file into the user's text and Grimoire's managed blocks, each between a `<!-- grimoire:begin NAME -->` line and a `<!-- grimoire:end NAME -->` line. The user's text is kept byte for byte, and markers inside fenced code blocks are ignored. A malformed, unterminated, nested, mismatched or repeated marker makes the file unreadable, and Grimoire then refuses to touch it rather than guess what it owns.
- **`entry`, configuration entries.** Grimoire edits only the value it owns in a JSON, JSONC or TOML file, in place, and keeps the user's key order, comments and unrelated content. (Inserting into a JSON container written on one line spreads that container over several lines.) An entry key names what Grimoire owns. For an object member it is a JSON Pointer (RFC 6901, the internet standard for a path such as `/mcpServers/docs-index` that names one value in a JSON document), built by `member_key`. For an array element it is the array's pointer, then `|` and a label, such as `/hooks/PreToolUse|secret-guard#0`, built by `element_key`: elements have no names, so the element is found by the hash of its value, and the label only keeps keys distinct. A TOML file can own only object members. `EntryFormat::for_path` chooses the syntax from the path alone: TOML for a `.toml` file, JSONC for exactly `.gemini/settings.json`, and strict JSON for everything else. `value_hash` hashes a value regardless of formatting and key order, so reformatting a file is not a change.
- **`plan::plan`, the planner,** is pure: given the desired outputs (`Desired`), the lock and the current bytes of every path that `plan::paths` lists, it returns a `Plan` and touches no file. For each owned unit it compares three hashes, of the content on disk, in the lock and desired, and picks an `Action`: create, update, delete, unchanged, adopt, forget or conflict. Content the user edited or removed since Grimoire wrote it, and content Grimoire doesn't manage at a path it wants, are conflicts. A file it can't parse safely, such as one with broken block markers or invalid JSON, is unreadable and is refused even with `--force`.
- **`apply::apply`** makes the plan's changes. It checks every precondition first, and if anything conflicts it writes nothing, unless `Options` sets `force` (the CLI's `--force`, which overwrites) or `write_new` (`--write-new`, which writes the would-be file as `<path>.new` and leaves the original alone). It reads each file again just before writing it, and stops if the file changed since planning. When it stops partway, `Stopped` lists the files it already wrote, so that the lock can still record them.
- **`apply::next_lock`** builds the next `grimoire.lock` from the plan, and files that were left alone keep their old records. **`apply::verify`** checks every output the lock records against the disk.
- **`hash::content`** is the hash the lock records. Text is hashed with `\r\n` line endings folded to `\n`, so a checkout that converts line endings is not reported as a change.

Known gaps, as [`docs/01-architecture.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#known-gaps-and-residual-risks) records them:

- Two `grimoire` processes in the same project at once are not locked against each other.
- A crash can leave a temporary file with a random name in a managed directory, and nothing removes it automatically.

## How the CLI uses it

The CLI reads every path the plan involves through the root, plans, applies, and then writes the next lock. This example does the same for the block Grimoire adds to `CLAUDE.md` so that Claude Code reads `AGENTS.md`:

```rust
use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::apply::{self, FileOutcome, Options};
use agent_grimoire_install::fs::Root;
use agent_grimoire_install::plan::{self, Action, Content, Desired, Owner};
use agent_grimoire_model::{ClientId, Lockfile, RelPath};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A project whose CLAUDE.md holds only the user's text. tempfile, a
    // dev-dependency of this crate, makes a directory that is deleted afterward.
    let project = tempfile::tempdir()?;
    std::fs::write(project.path().join("CLAUDE.md"), "# Notes\n")?;
    let root = Root::open(project.path())?;
    let lock = Lockfile::new(); // No grimoire.lock yet.

    // What the Claude Code adapter wants on disk.
    let desired = vec![Desired {
        owner: Owner::Project,
        clients: BTreeSet::from([ClientId::ClaudeCode]),
        path: RelPath::new("CLAUDE.md")?,
        content: Content::Block("@AGENTS.md\n".to_owned()),
    }];

    // Read every path involved through the root. Planning itself reads nothing.
    let mut current = BTreeMap::new();
    for path in plan::paths(&desired, &lock) {
        let bytes = root.read(&path)?;
        current.insert(path, bytes);
    }
    // Outputs that contradict each other come back as diagnostics.
    let plan = plan::plan(desired, &lock, &current).map_err(|d| format!("{d:?}"))?;
    assert_eq!(plan.files[0].units[0].action, Action::Create);

    // Apply, then build the lock, which the CLI writes last.
    let report = apply::apply(&root, &plan, Options::default()).map_err(|s| s.error)?;
    assert_eq!(report.files, [(RelPath::new("CLAUDE.md")?, FileOutcome::Written)]);
    let next = apply::next_lock(&lock, Vec::new(), &plan, &report.deferred());

    // The user's text is kept, and the disk now matches the new lock.
    assert_eq!(
        std::fs::read_to_string(project.path().join("CLAUDE.md"))?,
        "# Notes\n\n<!-- grimoire:begin grimoire -->\n@AGENTS.md\n<!-- grimoire:end grimoire -->\n",
    );
    assert!(apply::verify(&root, &next)?.is_empty());
    Ok(())
}
```

## Platforms

The same code runs on Linux, macOS and Windows, and continuous integration (CI) runs the tests on all three. These are the differences:

- **Confinement.** cap-std resolves paths with `openat2(RESOLVE_BENEATH)` on Linux 5.6 and later, a system call that refuses to resolve a path outside the directory it starts from. Where `openat2` is unavailable (an older kernel, or a sandbox that blocks it), and on other systems, it uses its own resolver, which opens one component at a time from the root's handle. On Windows, cap-std does the last step of creating a missing directory, of a write (the rename over the target) and of a removal by full path, after resolving the parent directories inside the root. Confinement still holds, but that last step isn't relative to an open directory handle ([ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md), which chose cap-std).
- **Named pipes.** On Unix, a file is opened for reading with `O_NONBLOCK`, so a named pipe swapped in after the check can't make the read wait forever. On every system, the file that was actually opened is checked again and refused if it isn't a regular file.
- **Durability.** On Unix, the directory is synced after a rename or a removal, as a best effort, so that the change reaches the disk sooner. Windows has no directory sync, so the step is skipped there. The rename is atomic either way.
- **Permissions.** A replaced file, and a `<path>.new` file written beside one, gets the original's permissions as cap-std reads them: the mode bits on Unix, such as `0600`, and only the read-only attribute on Windows. Windows access control lists (ACLs) and Unix ownership are not copied.
- **Temporary files.** On Linux, cap-tempfile first tries an unnamed temporary file (`O_TMPFILE`), which gets a name only just before the rename, so a crash rarely leaves one behind. Elsewhere the temporary file has a random name from the start.
- **Removing empty directories.** After deleting a file, Grimoire closes its handle on the file's directory before removing the directories left empty. On Windows, cap-std opens directories without `FILE_SHARE_DELETE`, so an open handle would make removing that directory fail.
- **Letter case and line endings.** Two paths that differ only in letter case are refused on every system, because macOS and Windows treat them as one file by default. Text hashes fold `\r\n` to `\n` everywhere, and a Markdown or TOML file that uses `\r\n` keeps it in the lines Grimoire writes.

## Documentation

- API reference: https://docs.rs/agent-grimoire-install
- Design: [`docs/subsystems/install.md`, Install model](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md): resolve, render, plan and apply ([`docs/subsystems/install.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#desired-state-plan-apply)); the three ownership units and how each file format is edited ([`docs/subsystems/install.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#ownership-units-and-merge-strategies)); drift, conflicts, `--force` and `--write-new` ([`docs/subsystems/install.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#drift-and-conflicts)); and filesystem safety ([`docs/subsystems/install.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#filesystem-safety)).
- [ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md), the architecture decision record that chose cap-std for root-confined file access, to close the race in which a directory is swapped for a symbolic link between a check and a write.
- [`docs/01-architecture.md`, Threats and mitigations](https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#threats-and-mitigations): the threats this crate defends against, such as symbolic links, special files, oversized files, edited managed content and interrupted writes.
- Versioning: [`docs/05-release.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#versioning). Grimoire versions its crates together.
- Grimoire: https://github.com/jabbott-iii/Grimoire

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
