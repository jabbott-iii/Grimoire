# ADR-0012: Crate documentation

- **Status:** Accepted (the maintainer asked for each crate's documentation to be created and maintained the way their Nadir project does it, 2026-11-22)
- **Date:** 2026-11-22
- **Related:** [`../versioning.md`](../versioning.md), which makes the library crates internal to the CLI; [ADR-0004](0004-package-and-binary-names.md), which named the packages

## Context

Grimoire's workspace has eight crates under `crates/`. (A crate is a Rust package.)
- Six are published to crates.io, the Rust package registry: the `agent-grimoire` CLI and its five library crates, `-model`, `-catalog`, `-resolve`, `-install` and `-adapters`.
- The libraries are internal to the CLI, published only so that `cargo install agent-grimoire` can build it ([`versioning.md`](../versioning.md)).
- `agent-grimoire-mcp` and `agent-grimoire-testkit` are never published (`publish = false`).

Each published crate has two pages that people read:
- its crates.io page, which shows the crate's `README.md`;
- its docs.rs page, the API reference that rustdoc (Rust's documentation tool) builds. Its front page is the crate docs: the `//!` comment lines at the top of `src/lib.rs`.

Before this decision:
- Each library's README was about 20 lines. It said what the crate holds and that it is internal, with no example.
- Each crate's crate docs were 5 to 17 lines, with bare references such as `architecture.md §7`.
- Nothing checked that either kept up with the code. CI never built the API docs, and one broken intra-doc link already existed. (An intra-doc link is a ``[`Name`]`` link in a doc comment that rustdoc resolves to an item.) It was in `xtask/src/release/archive.rs`, where ``[`write`]`` named both a function and a macro.

The maintainer's Nadir project documents each crate in two layers (its ADR-0039):
1. a README per crate, in one shape;
2. crate docs per crate, in one shape;

and it compiles each README's examples as doc tests (code examples in documentation that `cargo test` runs). One difference matters here: Nadir's crates are meant to be used directly, so each README shows a `[dependencies]` line. Grimoire's libraries are internal, so their pages must not invite a dependency.

## Decision

### 1. Crate docs, for every crate under `crates/`

The `//!` lines at the top of `src/lib.rs` have, in order:
1. A first paragraph that says what the crate is. Its first sentence is the summary docs.rs shows in search results.
2. `# Where it fits`: which crates use it and which it builds on, how the CLI reaches it, and its design documents, linked.
3. `# The parts`: the main public items, grouped by purpose, each as an intra-doc link.
4. `# Example`: a doc test.
5. Further sections for what a caller must know to use the crate well, such as guarantees, limits and errors. Their headings say what they're about.

A term is explained once, in the crate docs or on the item that defines it. Elsewhere it is linked, not explained again, because a reader often lands on one item's page rather than the crate's front page.

### 2. A README, for every published library crate

Its crates.io page has these sections, in this order:
- what the crate is, and how it fits in Grimoire (no heading);
- `## Status`;
- `## How the CLI uses it`, with at least one Rust example;
- `## Platforms`, only where the crate behaves differently by operating system;
- `## Documentation`, linking the docs.rs page, the design documents, the versioning policy and the repository;
- `## License`.

Two departures from Nadir's shape:
- **Status says "Before 1.0", not a version number.** release-plz, the tool that opens Grimoire's release pull requests ([ADR-0003](0003-release-tooling.md)), bumps the version in `Cargo.toml`. A version number in a README would go stale with every release unless someone updated it by hand.
- **"How the CLI uses it" replaces Nadir's "Usage".** It shows the calls the CLI makes, with no `[dependencies]` snippet, and Status keeps the "don't depend on it" policy and its reason.

Every link is absolute, except the README's own `LICENSE-APACHE` and `LICENSE-MIT`: crates.io resolves a relative link against the crate's directory on GitHub, where `docs/` isn't.

### 3. README examples are doc tests

Each published library's `src/lib.rs` holds this item:

```rust
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
```

`cargo test --workspace`, which CI's test job runs on Linux, macOS and Windows, then compiles and runs the README's examples like any other doc test. So an example on crates.io that stops compiling fails the build.

- The examples use only crates published on crates.io: the crate's dependencies, and a dev-dependency such as `tempfile` where one is needed, said so in a comment. They never use `agent-grimoire-testkit`, which is never published.
- They hide no lines with `# `. rustdoc hides such lines, but GitHub and crates.io show them as written.

### 4. The CLI crate keeps the repository's README

`agent-grimoire`'s crates.io page stays the root `README.md`, which is also the PyPI description and the README in every CLI archive (`AGENTS.md`). The crate gets crate docs only. The root README holds no Rust examples, and the file isn't in the crate's directory, so `include_str!("../README.md")` wouldn't find it.

### 5. Unpublished crates get crate docs only

`agent-grimoire-mcp` and `agent-grimoire-testkit` have no crates.io or docs.rs page, so they get crate docs, for contributors, and no README.

Out of scope:
- `xtask`, whose commands are listed in its `src/main.rs` and in `AGENTS.md`;
- the asset crates under `catalog/tools/` and `catalog/mcp-servers/`, whose READMEs follow [`catalog/README.md`](../../catalog/README.md).

### 6. Checks

- A unit test in `xtask/src/crate_docs.rs` checks the shapes:
  - every crate under `crates/` has the crate-docs sections, in order;
  - every published crate other than `agent-grimoire` has a README of its own, with the sections in order, a docs.rs link, a Rust example with no hidden lines, and only absolute links;
  - its `src/lib.rs` runs that README as doc tests.
- CI's clippy job builds the API docs with `RUSTDOCFLAGS="-D warnings"`, so a broken intra-doc link fails.
- `AGENTS.md`, `CONTRIBUTING.md` and the pull request template say that a change to a crate's public API or behavior updates its crate docs and README in the same change.

## Consequences

- Each crate's pages say what it is, where it fits and how it's called, with examples that compile.
- Six READMEs and eight crate-docs blocks must be kept true as the crates change, and each README partly repeats its crate docs.
- The checks are structural. They catch a missing section, a stale example and a broken link, but not a sentence that has stopped being true. That is left to review.
- CI's clippy job takes longer, because it now builds the API docs too.

## Alternatives considered

- **The README as the crate docs** (`#![doc = include_str!("../README.md")]`): one text to keep. But a README is a landing page, and crates.io shows an intra-doc link as its bracketed text, not as a link. Nadir rejected it because the README's shape is a landing page's.
- **Nadir's "Usage" section with a `[dependencies]` line:** it would invite the dependency that [`versioning.md`](../versioning.md) says not to take.
- **No checks:** the old docs had already drifted (the broken link above).

**Revisit when:**
- a library crate becomes a supported public API. Then its README gets a "Usage" section with its `[dependencies]` line.
- a crate is added. It gets docs in this shape; the test fails until it does.
- a crate gains features or platform-specific public items. Then docs.rs needs `[package.metadata.docs.rs]` to document them.
