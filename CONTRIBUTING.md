# Contributing to Grimoire

Grimoire is a catalog of AI-agent assets and `grimoire`, the Rust command-line tool (CLI) that installs the assets a user picks into their repository. The assets are rules, skills, agents, tools and MCP (Model Context Protocol) servers. Grimoire has one maintainer. This file says how to make a contribution that can be merged.

- [`AGENTS.md`](AGENTS.md) holds the working rules for everyone who changes this repository, people and coding agents alike, and the everyday commands.
- [`catalog/README.md`](catalog/README.md) is the reference for assets.
- [`docs/architecture.md`](docs/architecture.md) has the design, and [`docs/adr/`](docs/adr/) the decisions with their reasons.

## What to contribute

- **Assets:** new rules, skills, agents, tools, MCP servers and bundles, and fixes to the ones in the catalog.
- **Bug fixes** in the CLI, its libraries or an asset.
- **Client format updates.** A client is an AI coding assistant that Grimoire writes files for, such as Claude Code or Cursor. Each client has an adapter, the code that writes that client's files. The supported clients change their file formats often. An adapter fix that follows such a change, with the client's own docs or source as evidence, keeps Grimoire's output right.
- **Docs** that are wrong, missing or hard to follow.

[Open an issue](https://github.com/jabbott-iii/Grimoire/issues/new/choose) with one of the forms: "Bug report", "Asset request", "Client format change" or "Propose a change".

The fifth form, "Ask for a private contact", is only for someone who needs a private channel for a security or conduct report. It must not describe the problem ([Security issues](#security-issues)).

Issues labeled `good first issue` or `help wanted` are a good place to start.

**Open an issue before you start a large change**, so that you don't spend time on something that can't be merged. Use the "Propose a change" form, or an asset request for a new asset. Large changes include:
- a new client, asset kind or command;
- a change to the format of `grimoire.toml`, `grimoire.lock`, `asset.toml` or a bundle;
- a new tool or MCP server, because each one ships as its own binary in every release;
- a new dependency, or code that isn't Rust ([language policy](#language-policy));
- anything that needs an architecture decision record (ADR), a short document in `docs/adr/` that records a significant or hard-to-reverse decision and its reasons.

Small fixes can go straight to a pull request.

## Setting up

1. **Install Rust with [rustup](https://rustup.rs/)**, the Rust toolchain installer. [`rust-toolchain.toml`](rust-toolchain.toml) pins the toolchain (the compiler and its tools): Rust 1.100.0 with clippy, Rust's linter, and rustfmt, its code formatter. Run `rustup toolchain install` in the checkout to install exactly that, as continuous integration (CI) does.
2. **Install the MSRV toolchain.** The MSRV (minimum supported Rust version) is the workspace `rust-version` in [`Cargo.toml`](Cargo.toml), 1.96. The workspace is the set of crates (Rust packages) that this repository builds together. Every target (each library, binary, test, benchmark and example) must still build with Rust 1.96. CI checks that with Rust 1.96.0, the `RUST_MSRV` in [`ci.yml`](.github/workflows/ci.yml), and these commands run the same check:
   ```bash
   rustup toolchain install 1.96.0 --profile minimal
   cargo +1.96.0 check --workspace --all-targets --locked
   ```
   Give the full version. rustup reads `1.96` as the latest 1.96 patch release, which is newer than the MSRV.
3. **Other tools are optional.** CI also runs cargo-deny, cargo-shear, lychee, gitleaks, zizmor, actionlint, ShellCheck, gh-actions-lock and Claude Code (to validate the plugins). [CI](#ci), below, says what each one checks.
   - To run one locally, use the version that [`ci.yml`](.github/workflows/ci.yml) or [`security.yml`](.github/workflows/security.yml) pins. Check the download against the SHA-256 pinned there, when there is one.
   - The tools of the release dry-run, a rehearsal of the release that publishes nothing, are pinned in [`release.yml`](.github/workflows/release.yml).
4. **Fork the repository**, clone your fork, and add this repository as `upstream`:
   ```bash
   git remote add upstream https://github.com/jabbott-iii/Grimoire.git
   git fetch upstream
   ```

## Everyday commands

[`AGENTS.md`](AGENTS.md#everyday-commands) lists the commands, including the release steps. Most changes need these:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked   # the API docs, as CI builds them
cargo xtask validate                        # validate catalog/
cargo xtask validate --base upstream/main   # also require version bumps for changed assets
```

- `cargo xtask` is a Cargo alias, set in [`.cargo/config.toml`](.cargo/config.toml), for the repository's own automation in [`xtask/`](xtask/).
- CI treats warnings as errors. The lints (the compiler's and clippy's checks for suspect code) are set once, in the root `Cargo.toml`. They include `unsafe_code = "forbid"`, which rejects any `unsafe` Rust, and clippy pedantic, clippy's group of stricter lints.
- **Generated files are checked in, and CI fails when one is out of date.** Regenerate them; never edit them by hand.
  - Run `cargo xtask schema` after changing a model type: the Rust types for Grimoire's file formats, in `crates/agent-grimoire-model`.
  - Run `cargo xtask pack-catalog` and `cargo xtask gen-marketplace` after changing anything under `catalog/`.
  - Each has a `--check` mode, which is what CI runs.
- **Never weaken a check to get a change through.** Don't relax a lint, test, type check or security control. Fix the cause.

## Adding an asset

An asset lives entirely under `catalog/`, in `catalog/<kind>/<id>/`. Tools and MCP servers are Rust crates in their asset directory; the other kinds are data only.

1. **Check that it fits.** An asset request is the quickest way to ask. A tool or an MCP server needs an issue first (above).
2. **Scaffold it**, that is, generate its starting files:
   ```bash
   cargo xtask new <kind> <id> --maintainer <your-github-login>
   cargo xtask new bundle <id> --maintainer <your-github-login> <asset>...
   ```
   `<kind>` is `rule`, `skill`, `agent`, `tool` or `mcp-server`. Every template passes validation as generated, and the tool and MCP server templates build and pass their own tests; CI checks both on every pull request. The exemplars, the finished assets that [`catalog/README.md`](catalog/README.md) names, show one of each kind.
3. **Fill it in**, following [`catalog/README.md`](catalog/README.md). That covers:
   - the files every asset needs: `asset.toml`, a `README.md` with security notes, a `CHANGELOG.md`, and `evals/` or `tests/`;
   - the explicit outputs for each client, that is, what the asset writes for that client and where;
   - the rules in the section ["Rules that trip people up"](catalog/README.md#rules-that-trip-people-up).

   Declare the permissions the asset needs at runtime, and no others. `cargo xtask validate` checks some of them against what the asset does; review checks the rest.
4. **Validate it** with `cargo xtask validate`, then `cargo xtask validate --base upstream/main`. Validation renders the asset for each client it declares, through the real adapters. So an adapter refusal fails here, rather than in a user's `grimoire apply`.
5. **Build and test a tool or MCP server** like any other crate. Its package is `grimoire-tool-<id>` or `grimoire-mcp-<id>`, so `cargo test -p grimoire-tool-<id>` runs its tests alone.
   - Asset crates must stand alone: for example, they can't depend on Grimoire's core crates. catalog/README.md lists the checks.
   - MCP servers must serve through `agent-grimoire-mcp`, the shared crate that hardens every MCP server in the catalog ([ADR-0007](docs/adr/0007-mcp-server-authorization.md), the decision on server-side MCP authorization and HTTP hardening).
   - A new tool or MCP server is a new workspace member, so `Cargo.lock` changes. `cargo xtask new` leaves the lock file alone. The next cargo command run without `--locked`, such as `cargo xtask validate` in step 4, updates it. Commit that change: without it, every `--locked` command fails, in CI too.
6. **Regenerate the embedded catalog and the Claude Code marketplace**, and commit what changed, with `Cargo.lock` for a new tool or MCP server. The embedded catalog is a pack of the catalog's installable content (each asset's `asset.toml`, `README.md`, `CHANGELOG.md` and installed files, and every bundle) built into the `grimoire` binary. The marketplace is the Claude Code plugin marketplace generated from `catalog/`.
   ```bash
   cargo xtask pack-catalog
   cargo xtask gen-marketplace
   ```
7. **Bump the version and write the changelog.**
   - A new asset starts at the template's `0.1.0`, with a `## 0.1.0` entry.
   - After that, any change to an installed or built file needs a SemVer (semantic versioning) bump in `asset.toml`: a higher major, minor or patch number, depending on the kind of change. It also needs a matching `## <version>` entry in the asset's own `CHANGELOG.md`.
   - A change only to `README.md`, `CHANGELOG.md`, `evals/` or `tests/` needs neither.
   - Asset changelogs are written by hand; release-plz, the tool that writes the root changelog ([Commits](#commits)), doesn't touch them.
8. **Removing an asset takes two changes.** First mark it `maturity = "deprecated"` with a named replacement, then remove it in a later change. [`docs/versioning.md`](docs/versioning.md) says how long a deprecated asset keeps shipping.

## Changing the CLI or the libraries

- **The installer's guarantees are tested end to end** in [`crates/agent-grimoire/tests/cli.rs`](crates/agent-grimoire/tests/cli.rs). A change to planning, applying or removal needs a test there.
- **Read snapshot changes before you accept them.** Adapter output, among other things, is covered by `insta` snapshot tests, which compare the output with a stored copy, the snapshot. When a snapshot changes, review the diff with `cargo insta review` (from cargo-insta), or run the tests with `INSTA_UPDATE=always` and read `git diff`. A change you can't explain is a bug.
- **Keep the user docs in step with the code.**
  - [`docs/cli.md`](docs/cli.md) lists every command, flag and exit code, and [`docs/diagnostics.md`](docs/diagnostics.md) every user-facing diagnostic code.
  - Tests fail when a command, a flag or argument, an exit code or a user-facing diagnostic code is in the code but not in the doc, or the other way round.
  - Descriptions and the other details, such as each command's own exit codes, are checked only by review, so update the doc in the same change.
- **Keep each crate's docs in step with its code** ([ADR-0012](docs/adr/0012-crate-documentation.md), crate documentation).
  - A crate's docs are its crate docs, the `//!` lines at the top of `src/lib.rs` that docs.rs shows, and, for a published library crate, its `README.md`, the crate's crates.io page. `AGENTS.md` lists the sections each one has.
  - A change to a crate's public API or behavior updates both in the same change. A new crate starts in their shape.
  - `cargo test` fails when a README example stops compiling or a section goes missing: the examples run as doc tests, and `xtask/src/crate_docs.rs` checks the shapes. CI also builds the API docs with warnings denied, so a broken intra-doc link (a ``[`Name`]`` link in a doc comment) fails. The words themselves are checked only by review.
- **Diagnostics have stable codes**, such as `init/no-client`. A diagnostic is an error or warning that Grimoire reports. Tests assert on codes, not on message text.
- **Model types reject unknown fields.** Constrained values are newtypes validated while parsing, so errors carry the TOML location. (A newtype is a type that wraps a single value, such as a string, so that only valid values can be built. TOML is the configuration file format of `grimoire.toml` and `asset.toml`.) After changing a model type, run `cargo xtask schema`.
- **The filesystem is hostile input:** treat what you find there as untrusted. Never follow symbolic links (symlinks) in the catalog or in a user's project. In a user's project, writing through a link would put content somewhere the user didn't expect ([ADR-0006](docs/adr/0006-filesystem-confinement.md), which confines every write to the project root). Use the `RelPath` type, a relative path that can't name anything outside the project root, for every path that is written or installed.
- **Write an ADR** in [`docs/adr/`](docs/adr/) for a significant or hard-to-reverse decision.

## Verify external formats first

The agent ecosystem's formats change monthly. Before you implement or change code against any external format, such as a client's files, MCP, Agent Skills, a registry or a CI tool:
1. re-read its primary source, not a summary and not your memory;
2. update its entry in [`docs/specs.md`](docs/specs.md) in the same change, with the version or revision you read, the URL and the date you checked it;
3. mark anything you couldn't confirm as UNVERIFIED, and don't implement it.

If you see a client change but can't fix it yourself, open a client format change issue.

## Commits

- **Use [Conventional Commits](https://www.conventionalcommits.org/)**, a message format that starts with the type of change: `feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:` or `test:`.
- **[`.claude/rules/commits.md`](.claude/rules/commits.md) lists the scopes**, such as `cli`, `catalog`, and `xtask`, and the changes that must land in the same commit, such as a model type and its regenerated `schemas/`. [`.claude/rules/writing.md`](.claude/rules/writing.md) covers the text of commit messages, docs, and comments.
- **release-plz reads them.** It writes the root `CHANGELOG.md` from these messages and picks the next version from them. [ADR-0003](docs/adr/0003-release-tooling.md) chose release-plz for versions and changelogs, and [ADR-0009](docs/adr/0009-release-pipeline.md), the release pipeline, limited it to opening the release pull request, which carries the new version and changelog. The release workflow tags and publishes.
- **Every commit message counts.** Pull requests are merged with merge commits only, so each commit on your branch reaches `main` as you wrote it. Before you ask for review, squash fixups (fold each fix-up commit into the commit it fixes) and reword unclear messages.
- **Mark every breaking change** with `!` after the type (`feat!:`) or a `BREAKING CHANGE:` footer, as [`docs/versioning.md`](docs/versioning.md) requires. Two kinds are easy to miss:
  - **A break in any library crate's public Rust API.** The six published crates depend on each other with `^0.1.0` requirements, which accept any version from 0.1.0 up to, but not including, 0.2.0. So if the break shipped as a patch release, Cargo could build an older CLI against the new library.
  - **An MSRV bump**, that is, a higher `rust-version`, while Grimoire is 0.x. From 1.0 it is a minor change: a `feat:` commit, not marked breaking ([`docs/versioning.md`](docs/versioning.md) §3, the MSRV policy).
- **Why the mark matters.** While Grimoire is at 0.x, release-plz bumps only the patch version for `feat:` and `fix:` commits. Only a commit marked breaking moves the version to the next 0.y. Nothing else catches an unmarked break: `semver_check`, release-plz's automatic check for API breaks, is off in [`release-plz.toml`](release-plz.toml).

## Pull requests

- **Work from a fork.** Only the maintainer has write access to this repository. Push a branch to your fork and open the pull request against `main` here.
- **Keep it to one change**, as small as it can be while complete. Fill in the template, and delete the sections that don't apply.
- **Open it as a draft while you're still working.** GitHub doesn't ask code owners, the reviewers that [`.github/CODEOWNERS`](.github/CODEOWNERS) names, to review a draft. Mark it ready for review when it is.
- **Say which checks you ran, and which you didn't.**

### Review

- The ruleset on every branch, GitHub's set of rules for this repository's branches, requires a pull request, one approving review, and approval from a code owner. [`.github/CODEOWNERS`](.github/CODEOWNERS) names the maintainer for every path. So a pull request from anyone else can't merge without the maintainer's approval, short of the bypass below.
- A push that changes the diff dismisses earlier approvals. So can a change to `main` that alters the merge base, the newest commit that your branch and `main` share. The pull request then needs approving again.
- GitHub doesn't let authors approve their own pull requests, so the maintainer's own pull requests merge through the ruleset's bypass.
- The repository admin role and some GitHub Apps can bypass the ruleset, which also lets them push to `main` without a pull request. **Not every change on `main` has been reviewed by a second person.**

### CI

These workflows run on every pull request:
- **CI** ([`ci.yml`](.github/workflows/ci.yml)): rustfmt; clippy; the tests on Linux, macOS and Windows; the MSRV build; the catalog checks (schemas, validation, the embedded pack, the marketplace and the version bumps); the scaffold templates; Claude Code plugin validation; unused dependencies (cargo-shear); and an offline check of the documentation's links (lychee).
- **Security** ([`security.yml`](.github/workflows/security.yml)):
  - cargo-deny, which checks the dependencies' licenses, security advisories, bans and sources;
  - a gitleaks scan of every commit, for leaked secrets;
  - zizmor and actionlint, which check the workflows for security and correctness problems;
  - ShellCheck, a shell-script linter, on the workflow scripts;
  - a check, with gh-actions-lock, that `actions.lock` (the lock file for the GitHub Actions the workflows use) is complete and current.
- **Release** ([`release.yml`](.github/workflows/release.yml)): a dry-run of the release. It builds four of the six release targets and generates the channel files for them: the npm packages, the PyPI wheels, the Homebrew formula, the Scoop and winget manifests and the install scripts. It runs the install tests, and builds, checks and scans the container image. Its `release checks` job fails unless every dry-run job passed.

The ruleset also requires CodeQL code-scanning and code-quality results. CodeQL is GitHub's code analysis engine.
- **Code scanning:** `.github/workflows/codeql.yml` runs CodeQL on every pull request to `main`, every push to `main` and weekly. Pull request 1 received a passing `CodeQL` check on 2026-11-19. Whether the ruleset's `code_scanning` rule then passes wasn't read.
- **Code quality:** nothing produces those results yet. Whether GitHub's Code Quality setting is on is UNVERIFIED: the setting couldn't be read.

The ruleset refused release-plz's run 44198210657 on `main` (2026-11-17) when it tried to create a commit on its release branch. That was partly because of the code-scanning rule: GitHub reported that it was waiting for code-scanning results, which "may not be configured". While nothing produces the code-quality results, no pull request can meet the ruleset. A contributor's pull request, like the maintainer's, then merges only through the maintainer's bypass.

The ruleset doesn't require the CI, Security or Release checks yet. Adding them is a maintainer step in [`docs/release.md`](docs/release.md) §2 A, the repository settings. A pull request is still expected to pass all of them before it is merged.

## Language policy

Rust is the default for all code ([ADR-0010](docs/adr/0010-language-policy-exceptions.md), the language-policy exceptions). Rust gives the project one toolchain, one lint policy and one test harness. A few pieces can't be Rust: they run before any Grimoire binary is on the machine, inside a tool that reads only its own language, or in another repository that has no Rust toolchain.

The only exceptions are:
- the two install scripts;
- the npm launcher;
- the generated Homebrew formula;
- the shell steps in GitHub Actions workflows;
- the Homebrew tap and Scoop bucket sync workflows;
- `.github/scripts/image-tags.sh`, the container image's registry steps, which [ADR-0011](docs/adr/0011-container-image.md), the container image decision, added.

Each has a stated scope and checks of its own. Any other code that isn't Rust needs a new ADR. Data files that `cargo xtask` generates, such as JSON and YAML manifests, aren't code. YAML is a configuration file format.

## AI-assisted contributions

Contributions written with a coding agent are held to the same standard as any other. [`AGENTS.md`](AGENTS.md) holds the rules the agent must follow ([`CLAUDE.md`](CLAUDE.md) imports it for Claude Code): verify formats first, report only what was run, never handle credentials, never weaken checks, and keep changes small. The rules in [`.claude/rules/`](.claude/rules/) apply too: Claude Code loads them itself, and any other agent must read every file there before it starts. The skills in [`.claude/skills/`](.claude/skills/) are slash commands, such as `/next-task`, that start only when someone types them. You are responsible for every line you submit, so read it all and run the checks yourself.

## Security issues

**Never report a vulnerability in a public issue or pull request.** Report it privately with GitHub's [private vulnerability reporting](https://github.com/jabbott-iii/Grimoire/security/advisories/new). [`SECURITY.md`](SECURITY.md) explains the process. If you can't use that form, the "Ask for a private contact" issue form asks the maintainer for a private channel. Say nothing there about the problem itself.

## License

Grimoire is licensed under `MIT OR Apache-2.0`: the [MIT License](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE), at your option. There is no contributor license agreement and no Developer Certificate of Origin (DCO) sign-off.

When you submit a contribution, you license it under the same terms, `MIT OR Apache-2.0`, with no extra conditions. Section 5 of the Apache License 2.0 describes the same rule for that license. Submit only work that you have the right to license this way.

## Code of conduct

Everyone who takes part in Grimoire follows its code of conduct. [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) says what it covers and how to report a problem.
