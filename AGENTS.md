# Grimoire: conventions for contributors and coding agents

Grimoire is a catalog of AI-agent assets (rules, skills, agents, tools, MCP servers) plus a Rust command-line tool (CLI), `grimoire`, that installs the assets a user selects into their repository. MCP is the Model Context Protocol.

- [`PLAN.md`](PLAN.md) has the phased plan and its current status.
- [`docs/architecture.md`](docs/architecture.md) has the design.
- [`docs/adr/`](docs/adr/) has the decisions, as architecture decision records (ADRs) that also give the reasons.
- [`docs/specs.md`](docs/specs.md) has the external specifications (specs) that Grimoire relies on, as verified at their sources.

## Working rules

- **Verify formats before implementing them.** The agent ecosystem's formats change monthly. Before you implement or change code against any external format, re-read the primary source and update its entry in `docs/specs.md`: version, URL, date checked. Never fill gaps from memory; mark what you couldn't verify as UNVERIFIED.
- **Report only what you ran.** Never claim a command, test, build or scan passed unless you ran it and saw the result. Say which checks you did not run.
- **Leave git and publishing to the maintainer.** Don't commit, push, tag, release, publish or create repositories, environments or secrets unless the maintainer explicitly asks in this session. Propose a Conventional Commits message instead.
- **Never handle credentials.** Don't hard-code or log credentials, tokens or keys. Secrets appear only as environment-variable *names*.
- **Never weaken checks.** Don't weaken lints, tests, type checks or security controls to get something passing. Fix the cause.
- **Keep changes small.** Prefer the smallest complete change. Write an ADR in `docs/adr/` for significant or hard-to-reverse decisions.

## Rules and skills adopted from Doctrina

[Doctrina](https://github.com/jabbott-iii/Doctrina) is the maintainer's library of portable rules and skills for coding agents. Grimoire uses five of its rules, in `.claude/rules/`: `writing.md`, `commits.md`, `rust.md`, `testing.md`, and `roadmap-and-docs.md`. Each rule's **Project details** section holds Grimoire's values. All eleven of Doctrina's skills are in `.claude/skills/`, unchanged.

- **Follow the rules as this repository's rules.** Claude Code loads `writing.md`, `commits.md`, and `testing.md` in every session, `rust.md` when it works on Rust files, and `roadmap-and-docs.md` when it works on `PLAN.md`, `AGENTS.md`, `README.md`, `CONTRIBUTING.md`, or files in `docs/`. If you are another agent, read every file in `.claude/rules/` before you start.
- **This file and the ADRs win** where a rule's general text says otherwise, for example on the shape of a crate README ([ADR-0012](docs/adr/0012-crate-documentation.md)) or of a decision record.
- **Claude Code offers the skills as slash commands**, such as `/next-task` and `/fix-doc-drift`. Each one sets `disable-model-invocation: true`, so it starts only when someone types it. `build-course` relies on Doctrina's `teaching-materials.md`, which Grimoire hasn't adopted.
- **Keep the copies in step with Doctrina.** Change a rule's general text or a skill in Doctrina first, then copy it here. Change only **Project details** here. The one local change to general text is `PLAN.md` in `roadmap-and-docs.md`'s `paths`, because Grimoire's roadmap is `PLAN.md`.

## Repository layout

| Path | What |
|---|---|
| `crates/agent-grimoire-model` | Strict types for `asset.toml`, bundles, `grimoire.toml`, `grimoire.lock`; single-document validation; the source of the JSON Schemas |
| `crates/agent-grimoire-catalog` | Catalog loading and catalog-wide validation, including native Agent Skills checks; the embedded pack (`catalog.pack`, the catalog built into the binary) and `Source` |
| `crates/agent-grimoire-resolve` | Selection, versions, conflicts, clients and permission consent |
| `crates/agent-grimoire-install` | Root-confined filesystem (file access that can't leave the project root), managed blocks (the marked sections Grimoire owns in shared Markdown files), JSON, JSONC (JSON with comments) and TOML (a configuration file format) entries, the planner, `apply` and `verify` |
| `crates/agent-grimoire-adapters` | The six client adapters (`claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli`, `copilot`), each of which writes one client's files; the cross-client planner, which writes content that several selected clients read once, in a shared place, and warns about what other clients read too; and the plugin files for the marketplace |
| `crates/agent-grimoire` | The `grimoire` CLI: a library (`run`) plus the binary |
| `crates/agent-grimoire-mcp` | Shared hardening (security defenses) for MCP server assets, never published: stdio, Streamable HTTP, limits and OAuth resource-server checks (checks of the access tokens that clients present) |
| `crates/agent-grimoire-testkit` | Fixture catalogs and projects for tests (never published) |
| `xtask/` | `cargo xtask schema`, `validate`, `pack-catalog`, `gen-marketplace`, `new`, `release`; scaffolding templates (the starting files for new assets) in `xtask/templates/` |
| `packaging/` | Hand-written release inputs that aren't Rust ([ADR-0010](docs/adr/0010-language-policy-exceptions.md), the language-policy exceptions): the `install.sh`/`install.ps1` templates, the npm launcher, and the templates of the workflows that sync the Homebrew tap and the Scoop bucket. Release assets are generated from them at release time and never checked in ([`docs/release.md`](docs/release.md)) |
| `.github/scripts/` | Workflow scripts that are language-policy exceptions ([ADR-0010](docs/adr/0010-language-policy-exceptions.md), [ADR-0011](docs/adr/0011-container-image.md)): `image-tags.sh`, the image's push, tag and `latest` steps. ShellCheck, a shell-script linter, checks them in continuous integration (CI) |
| `Dockerfile` | The container image ([ADR-0011](docs/adr/0011-container-image.md), [`docs/container.md`](docs/container.md)): the release's static musl binaries on distroless static, a base image with no shell and no C library, COPY only. Only `release.yml` builds it, from the context that `cargo xtask release image context` writes |
| `.grype.yaml` | The image's Grype gate ([ADR-0011](docs/adr/0011-container-image.md)); Grype is a vulnerability scanner. Any medium, high or critical match fails, fixed or not. A finding without a fix needs an ignore rule with a reason, reviewed like the exceptions in `deny.toml`, the cargo-deny dependency policy |
| `catalog/<kind>/<id>/` | One directory per asset (data only). See [`catalog/README.md`](catalog/README.md) |
| `.claude-plugin/`, `plugins/claude-code/` | The Claude Code plugin marketplace, generated from `catalog/`. Never edit by hand |
| `.claude/rules/`, `.claude/skills/` | The rules and skills adopted from Doctrina for work on Grimoire itself (see above). Grimoire doesn't install them; they aren't catalog assets |
| `schemas/` | Generated JSON Schemas. Never edit by hand |
| `docs/cli.md`, `docs/diagnostics.md`, `docs/configuration.md`, `docs/versioning.md` | The user reference: commands, flags, exit codes, `--json` output and environment; every user-facing diagnostic code; `grimoire.toml`, `grimoire.lock`, permissions and clients; the versioning and deprecation policy |
| `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md` | The community files, at the root. `README.md` is also the `agent-grimoire` crate's page on crates.io, the PyPI long description and the README in every CLI archive |
| `.github/ISSUE_TEMPLATE/`, `.github/pull_request_template.md`, `.github/CODEOWNERS` | The issue forms and the `config.yml` of the form chooser, the pull request checklist, and the code owners |

## Everyday commands

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked   # the API docs, as CI builds them: a broken intra-doc link fails
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
cargo xtask release plan --event <event> --ref <ref>   # the mode (dry-run, release or skip), version, targets and archive names for a release.yml run; fails past GitHub's matrix and asset limits, and in release mode on a stale catalog.pack or an asset that needs a newer grimoire
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

Terms in the comments above:
- ADR-0009 is the release pipeline and its distribution channels, and ADR-0011 the container image and its supply chain.
- An SBOM (software bill of materials) lists the components inside an archive.
- OCI is the Open Container Initiative, whose image layout buildx writes.
- MSRV is the minimum supported Rust version.

## Code conventions

- **Toolchain and lints:** Rust edition 2024. The toolchain is pinned in `rust-toolchain.toml`, and the MSRV is the workspace `rust-version`. Lints are configured once in the root `Cargo.toml` (`unsafe_code = "forbid"`, clippy pedantic). CI treats warnings as errors.
- **Model types reject unknown fields.** Constrained values are newtypes (types that wrap a single value and accept only valid ones) validated while parsing, so errors carry the TOML location.
- **Diagnostics have stable codes** such as `manifest/kind-table-missing`. A diagnostic is an error or warning that Grimoire reports. Tests assert on codes, not on message text.
- **The filesystem is hostile input.** Never follow symlinks in the catalog or in a user's project. Use the `RelPath` type for every path that is written or installed. In a user's project, writing through a link would put content somewhere the user didn't expect ([ADR-0006](docs/adr/0006-filesystem-confinement.md), which confines every write to the project root).
- **Generated files are checked in, with a `--check` mode in CI.** Regenerate them; don't edit them.
- **Snapshots are reviewed, never accepted unread.** Adapter output is covered by `insta` snapshots, stored copies of the expected output. When one changes, read the diff before accepting it (`cargo insta review`, or `INSTA_UPDATE=always` and `git diff`).
- **The installer's guarantees are tested end to end** in `crates/agent-grimoire/tests/cli.rs`. A change to planning, applying or removal needs a test there.

## Documentation

- **The user reference follows the code.** A change to a command, flag or exit code updates [`docs/cli.md`](docs/cli.md), and a new user-facing diagnostic code updates [`docs/diagnostics.md`](docs/diagnostics.md). A unit test and an integration test fail when a command, flag, exit code or diagnostic code is in the code but not in these files, or the other way round. Descriptions aren't checked, so update them in the same change.
- **The root `README.md` uses absolute links only** (`https://github.com/jabbott-iii/Grimoire/blob/main/…`), and an xtask test enforces it. Relative links break where the file is republished: crates.io resolves them against `crates/agent-grimoire/`, PyPI doesn't rewrite them, and the CLI archives hold no `docs/`.
- **Keep one copy of each community file.** `README.md`, `CONTRIBUTING.md`, `SECURITY.md` and `CODE_OF_CONDUCT.md` live at the root. Never add one in `.github/` or `docs/`, where GitHub also looks for them. GitHub reads `.github/` first, then the root, then `docs/`, so a copy in `.github/` would replace the root file on GitHub.
- **Other READMEs are separate files.** Package, crate and asset READMEs are required where [`catalog/README.md`](catalog/README.md) and [ADR-0012](docs/adr/0012-crate-documentation.md) (crate documentation) need them.
- **Each crate's docs follow its code** ([ADR-0012](docs/adr/0012-crate-documentation.md)).
  - Every crate under `crates/` opens its `src/lib.rs` with crate docs (the `//!` lines, its docs.rs page) in one shape: what it is, then `# Where it fits`, `# The parts` and `# Example`, then what a caller must know.
  - Each published library crate also has a `README.md`, its crates.io page, with the sections `Status`, `How the CLI uses it`, `Platforms` (only where behavior differs by operating system), `Documentation` and `License`. Its Rust examples run as doc tests. `agent-grimoire`'s page is the root `README.md` instead.
  - A change to a crate's public API or behavior updates its crate docs and README in the same change, and a new crate starts in this shape. A test (`xtask/src/crate_docs.rs`) checks the shapes, and the doc tests and `cargo doc` catch stale examples and broken links. Whether the words are still true is checked only by review.
  - In doc comments, explain a term once, in the crate docs or on the item that defines it, and link to it elsewhere with an intra-doc link such as ``[`Source`]``.

## Adding an asset

Adding an asset is a data-only change under `catalog/`. See [`catalog/README.md`](catalog/README.md) for the required files, and run `cargo xtask validate` before you open a pull request.

Any change to installed content needs a version bump and a CHANGELOG entry. Asset versions and deprecation follow [`docs/versioning.md`](docs/versioning.md).

## Commits

Use Conventional Commits (`feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:`, `test:`). The root `CHANGELOG.md` is generated from them by release-plz ([ADR-0003](docs/adr/0003-release-tooling.md), which chose release-plz for versions and changelogs). Each asset's `CHANGELOG.md` is written by hand.

Mark a breaking change with `!` (`feat!: …`) or a `BREAKING CHANGE:` footer. That includes a change that breaks any library crate's public API and, while Grimoire is 0.x, an MSRV bump ([`docs/versioning.md`](docs/versioning.md)). Nothing else catches an unmarked one: release-plz runs no SemVer (semantic versioning) check of the public API.
