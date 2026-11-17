# Grimoire: conventions for contributors and coding agents

Grimoire is a catalog of AI-agent assets (rules, skills, agents, tools, MCP servers) plus a Rust CLI, `grimoire`, that installs the assets a user selects into their repository.

- [`PLAN.md`](PLAN.md) has the phased plan and its current status.
- [`docs/architecture.md`](docs/architecture.md) has the design.
- [`docs/adr/`](docs/adr/) has the decisions.
- [`docs/specs.md`](docs/specs.md) has the verified external specs.

## Working rules

- **Verify formats before implementing them.** Agent-ecosystem formats change monthly. Before you implement or change code against any external format, re-read the primary source and update its entry in `docs/specs.md`: version, URL, date checked. Never fill gaps from memory; mark what you couldn't verify as UNVERIFIED.
- **Report only what you ran.** Never claim a command, test, build or scan passed unless you ran it and saw the result. Say which checks you did not run.
- **Leave git and publishing to the maintainer.** Don't commit, push, tag, release, publish or create repositories, environments or secrets unless the maintainer explicitly asks in this session. Propose a Conventional Commits message instead.
- **Never handle credentials.** Don't hard-code or log credentials, tokens or keys. Secrets appear only as environment-variable *names*.
- **Never weaken checks.** Don't weaken lints, tests, type checks or security controls to get something passing. Fix the cause.
- **Keep changes small.** Prefer the smallest complete change. Write an ADR in `docs/adr/` for significant or hard-to-reverse decisions.

## Repository layout

| Path | What |
|---|---|
| `crates/agent-grimoire-model` | Strict types for `asset.toml`, bundles, `grimoire.toml`, `grimoire.lock`; single-document validation; JSON Schema source |
| `crates/agent-grimoire-catalog` | Catalog loading and catalog-wide validation, including native Agent Skills checks; the embedded pack (`catalog.pack`) and `Source` |
| `crates/agent-grimoire-resolve` | Selection, versions, conflicts, clients and permission consent |
| `crates/agent-grimoire-install` | Root-confined filesystem, managed blocks, JSON entries, the planner, `apply` and `verify` |
| `crates/agent-grimoire-adapters` | Client adapters (`claude-code`, `agents-md`) and the cross-client planner |
| `crates/agent-grimoire` | The `grimoire` CLI: a library (`run`) plus the binary |
| `crates/agent-grimoire-testkit` | Fixture catalogs and projects for tests (never published) |
| `xtask/` | `cargo xtask schema`, `validate`, `pack-catalog` |
| `catalog/<kind>/<id>/` | One directory per asset (data only). See [`catalog/README.md`](catalog/README.md) |
| `schemas/` | Generated JSON Schemas. Never edit by hand |

## Everyday commands

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo xtask schema            # regenerate schemas/ after changing model types
cargo xtask schema --check    # what CI runs
cargo xtask validate          # validate catalog/
cargo xtask validate --base origin/main   # also enforce version bumps for changed assets
cargo xtask pack-catalog      # regenerate the embedded catalog after changing catalog/
cargo xtask pack-catalog --check
cargo +1.96 check --workspace --all-targets --locked   # MSRV
cargo deny check
cargo shear
```

## Code conventions

- **Toolchain and lints:** Rust edition 2024. The toolchain is pinned in `rust-toolchain.toml`, and the MSRV is the workspace `rust-version`. Lints are configured once in the root `Cargo.toml` (`unsafe_code = "forbid"`, clippy pedantic). CI treats warnings as errors.
- **Model types reject unknown fields.** Constrained values are newtypes validated while parsing, so errors carry the TOML location.
- **Diagnostics have stable codes** such as `manifest/kind-table-missing`. Tests assert on codes, not on message text.
- **The filesystem is hostile input.** Never follow symlinks in the catalog or in a user's project. Use the `RelPath` type for every path that is written or installed.
- **Generated files are checked in, with a `--check` mode in CI.** Regenerate them; don't edit them.
- **Snapshots are reviewed, not rubber-stamped.** Adapter output is covered by `insta` snapshots. When one changes, read the diff before accepting it (`cargo insta review`, or `INSTA_UPDATE=always` and `git diff`).
- **The installer's guarantees are tested end to end** in `crates/agent-grimoire/tests/cli.rs`. A change to planning, applying or removal needs a test there.

## Adding an asset

Adding an asset is a data-only change under `catalog/`. See [`catalog/README.md`](catalog/README.md) for the required files, and run `cargo xtask validate` before you open a PR.

Any change to installed content needs a version bump and a CHANGELOG entry.

## Commits

Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:`, `test:`). Changelogs are generated from them by release-plz (ADR-0003).
