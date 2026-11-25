<!--
Thanks for contributing. CONTRIBUTING.md explains each item below.
Never fix an unreported vulnerability in a public pull request: report it privately first (SECURITY.md).
Tick each check you ran that passed, and each item that is true. Delete the sections that don't apply.
Every box you leave unticked needs a line under "Not run": the check, and why it wasn't run or didn't pass, or why the item doesn't apply.
-->

## Summary

<!-- What does this change, and why? Link the issue it closes, for example "Closes #123". -->

## Checks

- [ ] `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --locked -- -D warnings` and `cargo test --workspace --locked` pass.
- [ ] The title and every commit message follow Conventional Commits. Pull requests land as merge commits, so each commit reaches `main` as written.
- [ ] Breaking changes are marked with `!` or a `BREAKING CHANGE:` footer. That includes library API breaks and, while Grimoire is 0.x, raises of the MSRV, the minimum supported Rust version (`docs/05-release.md`).
- [ ] The docs are updated for anything users or contributors will notice.

Not run: <!-- each box you left unticked, in any section, and why -->

## Assets

Kind: <!-- rule, skill, agent, tool, mcp-server or bundle -->

- [ ] `cargo xtask validate --base upstream/main` passes, where `upstream` is this repository.
- [ ] The version is new or bumped, and the asset's `CHANGELOG.md` has an entry for it.
- [ ] The `[permissions]` in `asset.toml` match what the asset does at runtime, and nothing more.
- [ ] `README.md` says what the asset does and when to use it, with security notes for each permission.
- [ ] `evals/` or `tests/` covers the change.
- [ ] `cargo xtask pack-catalog` and `cargo xtask gen-marketplace` were run and their output committed, with the updated `Cargo.lock` for a new tool or MCP (Model Context Protocol) server.
- [ ] Each supported client has a `[clients.<client>]` table in `asset.toml`, and `cargo xtask validate` renders it.

## CLI and installer

- [ ] A test in `crates/agent-grimoire/tests/cli.rs` covers any change to planning, applying or removal.
- [ ] Changed `insta` snapshots were read before they were accepted.
- [ ] `docs/cli.md` and `docs/diagnostics.md` match the commands, flags, exit codes and diagnostic codes.
- [ ] A crate whose public API or behavior changed has updated crate docs (the `//!` lines in `src/lib.rs`) and, if it is published, an updated `README.md` (ADR-0012, crate documentation).

## Formats and decisions

- [ ] Every external format this relies on was re-read at its source, and its `docs/specs.md` entry updated.
- [ ] A changed model type comes with the regenerated `schemas/` (`cargo xtask schema`).
- [ ] A significant or hard-to-reverse decision has an architecture decision record (ADR) in `docs/adr/`.
