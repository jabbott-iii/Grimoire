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
| `crates/agent-grimoire-install` | Root-confined filesystem, managed blocks, JSON, JSONC and TOML entries, the planner, `apply` and `verify` |
| `crates/agent-grimoire-adapters` | The six client adapters (`claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli`, `copilot`), the cross-client planner, and the plugin files for the marketplace |
| `crates/agent-grimoire` | The `grimoire` CLI: a library (`run`) plus the binary |
| `crates/agent-grimoire-mcp` | Shared hardening for MCP server assets: stdio, Streamable HTTP, limits and OAuth resource-server checks (never published) |
| `crates/agent-grimoire-testkit` | Fixture catalogs and projects for tests (never published) |
| `xtask/` | `cargo xtask schema`, `validate`, `pack-catalog`, `gen-marketplace`, `new`, `release`; scaffolding templates in `xtask/templates/` |
| `packaging/` | Hand-written release inputs that aren't Rust ([ADR-0010](docs/adr/0010-language-policy-exceptions.md)): the `install.sh`/`install.ps1` templates, the npm launcher, and the tap and bucket sync workflow templates. Release assets are generated from them at release time and never checked in ([`docs/release.md`](docs/release.md)) |
| `.github/scripts/` | Workflow scripts that are language-policy exceptions ([ADR-0010](docs/adr/0010-language-policy-exceptions.md), [ADR-0011](docs/adr/0011-container-image.md)): `image-tags.sh`, the image's push, tag and `latest` steps. ShellCheck checks them in CI |
| `Dockerfile` | The container image ([ADR-0011](docs/adr/0011-container-image.md), [`docs/container.md`](docs/container.md)): the release's static musl binaries on distroless static, COPY only. Only `release.yml` builds it, from the context that `cargo xtask release image context` writes |
| `.grype.yaml` | The image's Grype gate ([ADR-0011](docs/adr/0011-container-image.md)): any medium, high or critical match fails, fixed or not. A finding without a fix needs an ignore rule with a reason, reviewed like `deny.toml`'s exceptions |
| `catalog/<kind>/<id>/` | One directory per asset (data only). See [`catalog/README.md`](catalog/README.md) |
| `.claude-plugin/`, `plugins/claude-code/` | The Claude Code plugin marketplace, generated from `catalog/`. Never edit by hand |
| `schemas/` | Generated JSON Schemas. Never edit by hand |
| `docs/cli.md`, `docs/diagnostics.md`, `docs/configuration.md`, `docs/versioning.md` | The user reference: commands, flags, exit codes, `--json` output and environment; every user-facing diagnostic code; `grimoire.toml`, `grimoire.lock`, permissions and clients; the versioning and deprecation policy |
| `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md` | The community files, at the root. `README.md` is also the `agent-grimoire` crate's page on crates.io, the PyPI long description and the README in every CLI archive |
| `.github/ISSUE_TEMPLATE/`, `.github/pull_request_template.md`, `.github/CODEOWNERS` | The issue forms and the chooser's `config.yml`, the pull request checklist, and the code owners |

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
cargo xtask release notes --version <V> --image-digest <sha256:…> --out <file>   # the GitHub Release text, from the version's CHANGELOG.md section (fails without one); names the image digest
# Pass --partial to `channels` and `verify` when dist/ holds only some of the six targets.
# Container image (ADR-0011). The Dockerfile is COPY only. Change its base only by digest, to one at least 7 days old:
# Dependabot can't date gcr.io digests, so check the age by hand. Run as release.yml runs them, these steps need Docker
# with a docker-container buildx builder, crane, cosign and Grype; the steps themselves call none of them.
cargo xtask release image base                         # print the base index reference the Dockerfile pins
cargo xtask release image context --dist dist --base base --out image   # check the base index (from `crane manifest`) against the pin, unpack both Linux musl targets' binaries into a build context, write the buildx arguments
cargo xtask release image check --out image --layout image/layout --base base   # check buildx's OCI layout against the plan, binaries byte for byte; print the digests
cargo xtask release image coverage --out image --report <json> --cdx <file> --platform <p>   # fail unless Grype saw Debian, its packages and every binary's Rust packages
cargo +1.96.0 check --workspace --all-targets --locked   # MSRV, as CI runs it, after rustup toolchain install 1.96.0 --profile minimal
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

## Documentation

- **The user reference follows the code.** A change to a command, flag or exit code updates [`docs/cli.md`](docs/cli.md), and a new user-facing diagnostic code updates [`docs/diagnostics.md`](docs/diagnostics.md). A unit test and an integration test fail when a command, flag, exit code or diagnostic code is in the code but not in these files, or the other way round. Descriptions aren't checked, so update them in the same change.
- **The root `README.md` uses absolute links only** (`https://github.com/jabbott-iii/Grimoire/blob/main/…`), and an xtask test enforces it. Relative links break where the file is republished: crates.io resolves them against `crates/agent-grimoire/`, PyPI doesn't rewrite them, and the CLI archives hold no `docs/`.
- **Keep one copy of each community file.** `README.md`, `CONTRIBUTING.md`, `SECURITY.md` and `CODE_OF_CONDUCT.md` live at the root. Never add one in `.github/` or `docs/`, where GitHub also looks for them: it reads `.github/` first, then the root, then `docs/`, so a copy in `.github/` would replace the root file on GitHub. Package, crate and asset READMEs are different files, and are required where [`catalog/README.md`](catalog/README.md) and the crates need them.

## Adding an asset

Adding an asset is a data-only change under `catalog/`. See [`catalog/README.md`](catalog/README.md) for the required files, and run `cargo xtask validate` before you open a PR.

Any change to installed content needs a version bump and a CHANGELOG entry. Asset versions and deprecation follow [`docs/versioning.md`](docs/versioning.md).

## Commits

Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:`, `test:`). Changelogs are generated from them by release-plz (ADR-0003).

Mark a breaking change with `!` (`feat!: …`) or a `BREAKING CHANGE:` footer. That includes a change that breaks any library crate's public API and, while Grimoire is 0.x, an MSRV bump ([`docs/versioning.md`](docs/versioning.md)). Nothing else catches an unmarked one: release-plz runs no semver check.
