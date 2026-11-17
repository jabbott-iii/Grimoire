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
| `crates/agent-grimoire-mcp` | Shared hardening for MCP server assets: stdio, Streamable HTTP, limits and OAuth resource-server checks (never published) |
| `crates/agent-grimoire-testkit` | Fixture catalogs and projects for tests (never published) |
| `xtask/` | `cargo xtask schema`, `validate`, `pack-catalog`, `gen-marketplace`, `new`, `release`; scaffolding templates in `xtask/templates/` |
| `packaging/` | Hand-written release inputs that aren't Rust ([ADR-0010](docs/adr/0010-language-policy-exceptions.md)): the `install.sh`/`install.ps1` templates, the npm launcher, and the tap and bucket sync workflow templates. Release assets are generated from them at release time and never checked in ([`docs/release.md`](docs/release.md)) |
| `catalog/<kind>/<id>/` | One directory per asset (data only). See [`catalog/README.md`](catalog/README.md) |
| `.claude-plugin/`, `plugins/claude-code/` | The Claude Code plugin marketplace, generated from `catalog/`. Never edit by hand |
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
cargo xtask gen-marketplace   # regenerate .claude-plugin/ and plugins/claude-code/ after changing catalog/
cargo xtask gen-marketplace --check
cargo xtask new <kind> <id> --maintainer <login>   # scaffold an asset
# Release pipeline (ADR-0009). These build and check locally; none of them publishes anything.
# CI runs them as `cargo --locked xtask …`, so a stale Cargo.lock fails instead of being rewritten.
cargo xtask release plan --event <event> --ref <ref>   # the mode (dry-run, release or skip), version, targets and archive names for a release.yml run; fails past GitHub's matrix and asset limits
cargo xtask release build --target <triple> --out dist # build (one cargo auditable build per package), smoke-test and archive the CLI and every asset binary for one target
cargo xtask release sbom --dist dist                   # a CycloneDX 1.6 SBOM per archive
cargo xtask release channels --dist dist --work work   # npm packages, wheels, Homebrew formula, Scoop and winget manifests, install scripts
cargo xtask release checksums --dist dist [--check]    # write or check dist/SHA256SUMS
cargo xtask release verify --dist dist                 # the expected assets, SHA256SUMS, and the same binary in every channel
cargo xtask release notes --version <V> --out <file>   # the GitHub Release text, from the version's CHANGELOG.md section (fails without one)
# Pass --partial to `channels` and `verify` when dist/ holds only some of the six targets.
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
