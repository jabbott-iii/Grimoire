# Contributing to Grimoire

Grimoire is a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers) and `grimoire`, the Rust CLI that installs the assets a user picks into their repository. It has one maintainer. This file says how to make a contribution that can be merged.

- [`AGENTS.md`](AGENTS.md) holds the working rules for everyone who changes this repository, people and coding agents alike, and the everyday commands.
- [`catalog/README.md`](catalog/README.md) is the reference for assets.
- [`docs/architecture.md`](docs/architecture.md) has the design, and [`docs/adr/`](docs/adr/) the decisions with their reasons.

## What to contribute

- **Assets:** new rules, skills, agents, tools, MCP servers and bundles, and fixes to the ones in the catalog.
- **Bug fixes** in the CLI, its libraries or an asset.
- **Client format updates.** The supported clients change their file formats often. An adapter fix that follows a change, with the client's own docs or source as evidence, keeps Grimoire's output right.
- **Docs** that are wrong, missing or hard to follow.

[Open an issue](https://github.com/jabbott-iii/Grimoire/issues/new/choose) with one of the forms: "Bug report", "Asset request", "Client format change" or "Propose a change". The fifth, "Ask for a private contact", is only for someone who needs a private channel for a security or conduct report, and it must not describe the problem ([Security issues](#security-issues)). Issues labelled `good first issue` or `help wanted` are a good place to start.

**Open an issue before you start a large change**, so that you don't spend time on something that can't be merged. Use the "Propose a change" form, or an asset request for a new asset. Large changes include:
- a new client, asset kind or command;
- a change to the format of `grimoire.toml`, `grimoire.lock`, `asset.toml` or a bundle;
- a new tool or MCP server, because each one ships as its own binary in every release;
- a new dependency, or code that isn't Rust ([language policy](#language-policy));
- anything that needs an ADR.

Small fixes can go straight to a pull request.

## Setting up

1. **Install Rust with [rustup](https://rustup.rs/).** [`rust-toolchain.toml`](rust-toolchain.toml) pins the toolchain: Rust 1.100.0 with clippy and rustfmt. Run `rustup toolchain install` in the checkout to install exactly that, as CI does.
2. **Install the MSRV toolchain.** The workspace `rust-version` in [`Cargo.toml`](Cargo.toml) is 1.96, and every target must still build on it. CI checks that with Rust 1.96.0, the `RUST_MSRV` in [`ci.yml`](.github/workflows/ci.yml), and these commands run the same check:
   ```bash
   rustup toolchain install 1.96.0 --profile minimal
   cargo +1.96.0 check --workspace --all-targets --locked
   ```
   Give the full version. rustup reads `1.96` as the latest 1.96 patch release, which is newer than the MSRV.
3. **Other tools are optional.** CI also runs cargo-deny, cargo-shear, lychee, gitleaks, zizmor, actionlint, ShellCheck, gh-actions-lock and Claude Code (to validate the plugins). To run one locally, use the version that [`ci.yml`](.github/workflows/ci.yml) or [`security.yml`](.github/workflows/security.yml) pins, and check the download against the SHA-256 pinned there when there is one. The release dry-run's tools are pinned in [`release.yml`](.github/workflows/release.yml).
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
cargo xtask validate                        # validate catalog/
cargo xtask validate --base upstream/main   # also require version bumps for changed assets
```

- `cargo xtask` is a Cargo alias, set in [`.cargo/config.toml`](.cargo/config.toml), for the repository's own automation in [`xtask/`](xtask/).
- CI treats warnings as errors. The lints are set once, in the root `Cargo.toml`: `unsafe_code = "forbid"` and clippy pedantic, among others.
- **Generated files are checked in, and CI fails when one is stale.** Regenerate them; never edit them by hand. Run `cargo xtask schema` after changing a model type, and `cargo xtask pack-catalog` and `cargo xtask gen-marketplace` after changing anything under `catalog/`. Each has a `--check` mode, which is what CI runs.
- **Never weaken a check to get a change through.** Don't relax a lint, test, type check or security control. Fix the cause.

## Adding an asset

An asset lives entirely under `catalog/`, in `catalog/<kind>/<id>/`. Tools and MCP servers are Rust crates in their asset directory; the other kinds are data only.

1. **Check that it fits.** An asset request is the quickest way to ask. A tool or an MCP server needs an issue first (above).
2. **Scaffold it:**
   ```bash
   cargo xtask new <kind> <id> --maintainer <your-github-login>
   cargo xtask new bundle <id> --maintainer <your-github-login> <asset>...
   ```
   `<kind>` is `rule`, `skill`, `agent`, `tool` or `mcp-server`. Every template validates as generated, and the tool and MCP server templates build and pass their own tests; CI checks both on every pull request. The exemplars that [`catalog/README.md`](catalog/README.md) names show a finished asset of each kind.
3. **Fill it in**, following [`catalog/README.md`](catalog/README.md): the files every asset needs (`asset.toml`, a `README.md` with security notes, a `CHANGELOG.md`, and `evals/` or `tests/`), the explicit outputs for each client, and the rules that catch people out. Declare the permissions the asset needs at runtime, and no others. `cargo xtask validate` checks some of them against what the asset does; review checks the rest.
4. **Validate it** with `cargo xtask validate`, then `cargo xtask validate --base upstream/main`. Validation renders the asset for each client it declares, through the real adapters, so an adapter refusal fails here rather than in a user's `grimoire apply`.
5. **Build and test a tool or MCP server** like any other crate. Its package is `grimoire-tool-<id>` or `grimoire-mcp-<id>`, so `cargo test -p grimoire-tool-<id>` runs its tests alone. Asset crates must stand alone, and MCP servers must serve through `agent-grimoire-mcp` ([ADR-0007](docs/adr/0007-mcp-server-authorization.md)); catalog/README.md lists the checks.
   - A new tool or MCP server is a new workspace member, so `Cargo.lock` changes. `cargo xtask new` leaves the lock alone; the next cargo command run without `--locked`, such as `cargo xtask validate` in step 4, updates it. Commit that change: without it, every `--locked` command fails, in CI too.
6. **Regenerate the embedded catalog and the Claude Code marketplace**, and commit what changed, with `Cargo.lock` for a new tool or MCP server:
   ```bash
   cargo xtask pack-catalog
   cargo xtask gen-marketplace
   ```
7. **Bump the version and write the changelog.** A new asset starts at the template's `0.1.0`, with a `## 0.1.0` entry. After that, any change to an installed or built file needs a SemVer bump in `asset.toml` and a matching `## <version>` entry in the asset's own `CHANGELOG.md`. A change only to `README.md`, `CHANGELOG.md`, `evals/` or `tests/` needs neither. Asset changelogs are written by hand; release-plz doesn't touch them.
8. **Removing an asset takes two changes.** First mark it `maturity = "deprecated"` with a named replacement, then remove it in a later change. [`docs/versioning.md`](docs/versioning.md) says how long a deprecated asset keeps shipping.

## Changing the CLI or the libraries

- **The installer's guarantees are tested end to end** in [`crates/agent-grimoire/tests/cli.rs`](crates/agent-grimoire/tests/cli.rs). A change to planning, applying or removal needs a test there.
- **Read snapshot changes before you accept them.** Adapter output, among other things, is covered by `insta` snapshots. When a snapshot changes, review the diff with `cargo insta review` (from cargo-insta), or run the tests with `INSTA_UPDATE=always` and read `git diff`. A change you can't explain is a bug.
- **Keep the user docs in step with the code.** [`docs/cli.md`](docs/cli.md) lists every command, flag and exit code, and [`docs/diagnostics.md`](docs/diagnostics.md) every user-facing diagnostic code. Tests fail when a command, a flag or argument, an exit code or a user-facing diagnostic code is in the code but not in the doc, or the other way round. Descriptions and the other details, such as each command's own exit codes, are checked only by review, so update the doc in the same change.
- **Diagnostics have stable codes**, such as `init/no-client`. Tests assert on codes, not on message text.
- **Model types reject unknown fields**, and constrained values are newtypes validated while parsing, so errors carry the TOML location. After changing a model type, run `cargo xtask schema`.
- **The filesystem is hostile input.** Never follow symlinks in the catalog or in a user's project, and use the `RelPath` type for every path that is written or installed.
- **Write an ADR** in [`docs/adr/`](docs/adr/) for a significant or hard-to-reverse decision.

## Verify external formats first

Agent-ecosystem formats change monthly. Before you implement or change code against any external format, such as a client's files, MCP, Agent Skills, a registry or a CI tool:
1. re-read its primary source, not a summary and not your memory;
2. update its entry in [`docs/specs.md`](docs/specs.md) in the same change, with the version or revision you read, the URL and the date you checked it;
3. mark anything you couldn't confirm as UNVERIFIED, and don't implement it.

If you see a client change but can't fix it yourself, open a client format change issue.

## Commits

- **Use [Conventional Commits](https://www.conventionalcommits.org/):** `feat:`, `fix:`, `docs:`, `chore:`, `ci:`, `refactor:` or `test:`. release-plz writes the root `CHANGELOG.md` from these messages and picks the next version from them ([ADR-0003](docs/adr/0003-release-tooling.md), [ADR-0009](docs/adr/0009-release-pipeline.md)).
- **Every commit message counts.** Pull requests are merged with merge commits only, so each commit on your branch reaches `main` as you wrote it. Squash fixups and reword unclear messages before you ask for review.
- **Mark every breaking change** with `!` after the type (`feat!:`) or a `BREAKING CHANGE:` footer, as [`docs/versioning.md`](docs/versioning.md) requires. Two kinds are easy to miss:
  - **A break in any library crate's public Rust API.** The six published crates depend on each other with `^0.1.0` requirements, so if the break shipped as a patch release, Cargo could build an older CLI against the new library.
  - **An MSRV bump**, that is, a higher `rust-version`, while Grimoire is 0.x. From 1.0 it is a minor change: a `feat:` commit, not marked breaking ([`docs/versioning.md`](docs/versioning.md) §3).
- **Why the mark matters.** While Grimoire is at 0.x, release-plz bumps only the patch version for `feat:` and `fix:` commits. Only a commit marked breaking moves the version to the next 0.y, and nothing else catches an unmarked break: `semver_check` is off in [`release-plz.toml`](release-plz.toml).

## Pull requests

- **Work from a fork.** Only the maintainer has write access to this repository. Push a branch to your fork and open the pull request against `main` here.
- **Keep it to one change**, as small as it can be while complete. Fill in the template, and delete the sections that don't apply.
- **Open it as a draft while you're still working.** GitHub doesn't ask code owners to review a draft. Mark it ready for review when it is.
- **Say which checks you ran, and which you didn't.**

### Review

- The ruleset on every branch requires a pull request, one approving review, and approval from a code owner. [`.github/CODEOWNERS`](.github/CODEOWNERS) names the maintainer for every path, so a pull request from anyone else can't merge without the maintainer's approval, short of the bypass below.
- A push that changes the diff dismisses earlier approvals, and so can a change to `main` that alters the merge base. The pull request then needs approving again.
- GitHub doesn't let authors approve their own pull requests, so the maintainer's own pull requests merge through the ruleset's bypass. The repository admin role and some GitHub Apps can bypass the ruleset, which also lets them push to `main` without a pull request. **Not every change on `main` has been reviewed by a second person.**

### CI

These workflows run on every pull request:
- **CI** ([`ci.yml`](.github/workflows/ci.yml)): rustfmt; clippy; the tests on Linux, macOS and Windows; the MSRV build; the catalog checks (schemas, validation, the embedded pack, the marketplace and the version bumps); the scaffold templates; Claude Code plugin validation; unused dependencies (cargo-shear); and an offline check of the documentation's links (lychee).
- **Security** ([`security.yml`](.github/workflows/security.yml)): cargo-deny, a gitleaks scan of every commit, zizmor and actionlint on the workflows, ShellCheck on the workflow scripts, and a check that `actions.lock` is complete and current.
- **Release** ([`release.yml`](.github/workflows/release.yml)): a dry-run of the release. It builds four of the six release targets, generates the channel files for them, runs the install tests, and builds, checks and scans the container image. Its `release checks` job fails unless every dry-run job passed.

The ruleset also requires CodeQL code-scanning and code-quality results. Those come from GitHub's code scanning and code quality settings, not from a workflow in this repository, and whether they are set up is UNVERIFIED: the settings couldn't be read, and no CodeQL or code-quality check run has been seen. The ruleset refused release-plz's run 44198210657 on `main` (2026-11-17) when it tried to create a commit on its release branch, partly because of the code-scanning rule: GitHub reported that it was waiting for code-scanning results, which "may not be configured". If nothing produces those results, no pull request can meet the ruleset, so a contributor's pull request, like the maintainer's, merges only through the maintainer's bypass.

The ruleset doesn't require the CI, Security or Release checks yet; that is a maintainer step in [`docs/release.md`](docs/release.md) §2 A. A pull request is still expected to pass all of them before it is merged.

## Language policy

Rust is the default for all code ([ADR-0010](docs/adr/0010-language-policy-exceptions.md)). The only exceptions are the two install scripts, the npm launcher, the generated Homebrew formula, the shell steps in GitHub Actions workflows, the Homebrew tap and Scoop bucket sync workflows, and `.github/scripts/image-tags.sh` ([ADR-0011](docs/adr/0011-container-image.md)). Each has a stated scope and checks of its own. Any other code that isn't Rust needs a new ADR. Data files that `cargo xtask` generates, such as JSON and YAML manifests, aren't code.

## AI-assisted contributions

Contributions written with a coding agent are held to the same standard as any other. [`AGENTS.md`](AGENTS.md) holds the rules the agent must follow ([`CLAUDE.md`](CLAUDE.md) imports it for Claude Code): verify formats first, report only what was run, never handle credentials, never weaken checks, and keep changes small. You are responsible for every line you submit, so read it all and run the checks yourself.

## Security issues

**Never report a vulnerability in a public issue or pull request.** Report it privately with GitHub's [private vulnerability reporting](https://github.com/jabbott-iii/Grimoire/security/advisories/new). [`SECURITY.md`](SECURITY.md) explains the process. If you can't use that form, the "Ask for a private contact" issue form asks the maintainer for a private channel. Say nothing there about the problem itself.

## Licence

Grimoire is licensed under `MIT OR Apache-2.0`: the [MIT licence](LICENSE-MIT) or the [Apache License 2.0](LICENSE-APACHE), at your option. There is no contributor licence agreement and no DCO sign-off. When you submit a contribution, you license it under the same terms, `MIT OR Apache-2.0`, with no extra conditions; section 5 of the Apache License 2.0 describes the same rule for that licence. Submit only work that you have the right to license this way.

## Code of conduct

Everyone who takes part in Grimoire follows its code of conduct. [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) says what it covers and how to report a problem.
