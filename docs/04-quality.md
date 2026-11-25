# 04 — Quality

This document says how Grimoire is checked: the layers of tests and the tools they use, the checks that keep documents true, continuous integration (CI), what the repository's rules and review guarantee, and what review must look for. How to write a test is in `.claude/rules/testing.md`, which this repository adopted from Doctrina; it also sets the rule that no test is weakened to make it pass. `.claude/rules/rust.md` gives the test layout and the check commands.

## Test layers

| Layer | Where | What it checks |
|---|---|---|
| Unit tests | a `#[cfg(test)] mod tests` beside the code, in every crate | One module's behavior, including each error and each limit |
| Integration tests | each crate's `tests/` | A crate through its public API, such as `crates/agent-grimoire-resolve/tests/resolve.rs` and `crates/agent-grimoire-mcp/tests/http.rs` |
| End-to-end tests | `crates/agent-grimoire/tests/cli.rs`, `builtin.rs`, `clients.rs` and `binary.rs` | The installer's guarantees through `agent_grimoire::run` and the built binary: idempotency, drift, removal, confinement, and installing the embedded catalog for every client. A change to planning, applying or removal needs a test in `cli.rs` |
| Snapshot tests | insta snapshots in `snapshots/` beside the tests that write them | Adapter output for each client and client combination. A changed snapshot is read before it is accepted (`cargo insta review`) |
| Property tests | proptest, in files that end in `_props.rs` or `properties.rs` | Round trips that hold for every input, such as inserting and removing a managed block or entry ([the install spec](subsystems/install.md)) |
| Doc tests | every crate's doc examples, and each library README through `ReadmeDoctests` | That documented examples still compile and run ([ADR-0012](adr/0012-crate-documentation.md), crate documentation) |
| Asset tests | each asset crate's `tests/`, and each content asset's `evals/` | A tool's or server's behavior, and the cases a rule, skill or agent is meant to handle |
| Repository checks | unit tests in `xtask/src/` | The repository itself: the README's links, the crate docs' shape, and the permissions of reusable-workflow calls ([the xtask spec](subsystems/xtask.md)) |

There are no benchmarks and no fuzz targets. The hostile-input tests sit beside the code that reads the input, as the **Security checks** in `testing.md` list.

## Test tools

- **`agent-grimoire-testkit`** (never published): fixture catalogs and fixture projects, so tests drive the CLI library with small catalogs made for them instead of the embedded one. It is only ever a dev-dependency.
- **insta** and **cargo-insta** for snapshots, **proptest** for property tests, and **tempfile** for a fresh directory per test. Each is declared once in `[workspace.dependencies]` ([`02-dependency-policy.md`](02-dependency-policy.md)).
- **Nothing measures coverage or runs mutation tests yet:** the **Coverage command** and **Mutation command** in `testing.md` are "none". Until one exists, every line and branch added or changed is covered, and checked by reading the tests.

## Checked documents

Four documents are checked against the code by tests that `cargo test --workspace --locked` runs:

| Document | Check | What fails it |
|---|---|---|
| [`cli.md`](cli.md) | `crates/agent-grimoire/src/docs_check.rs` | A command, flag or exit code in the code but not in the document, or the other way round |
| [`diagnostics.md`](diagnostics.md) | `crates/agent-grimoire/tests/diagnostics_doc.rs` | A user-facing diagnostic code in the code but not in the document, or the other way round |
| The root `README.md` | `xtask/src/readme_links.rs` | A relative link, which breaks on crates.io, on PyPI and in the CLI archives |
| Crate docs and crate READMEs | `xtask/src/crate_docs.rs` | A missing section, sections out of order, or a README without its doc tests ([ADR-0012](adr/0012-crate-documentation.md)) |

The checks compare names, not descriptions, so a description that stops being true is caught only by review. Two more checks cover documents: `cargo xtask release plan` refuses a release while a `before-first-release` marker remains in `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md`, and lychee checks every Markdown link in CI. Nothing yet checks that [`roadmap.md`](roadmap.md) and the decision records hold together; task M8.2 adds that check.

## Continuous integration

**What CI checks:**
- `ci.yml` runs on every pull request and every push to `main`. It checks:
  - formatting, and clippy (Rust's linter) with warnings denied;
  - the tests on Linux, macOS and Windows, and the build with the minimum supported Rust version (MSRV);
  - the catalog, schemas and generated files (`--check`), and version bumps for changed assets;
  - the scaffold templates (which `cargo xtask new` creates assets from) and `claude plugin validate --strict`;
  - unused dependencies and the documentation's links.
- `security.yml` runs on the same events and weekly: cargo-deny (the dependency policy, [`02-dependency-policy.md`](02-dependency-policy.md)), gitleaks (a secret scanner) over the whole history, zizmor (at its pedantic persona, which reports more than the regular one), actionlint, ShellCheck and `actions.lock` (the record of pinned actions, [`02-dependency-policy.md`](02-dependency-policy.md#the-rule-for-third-party-code)). The weekly and manual runs also scan the released image ([the container image spec](subsystems/container-image.md)).
- `codeql.yml` runs CodeQL (advanced setup, configured by this workflow file) over the Rust, JavaScript and Python code and the workflows, with its `security-extended` queries, on every pull request to `main`, every push to `main` and weekly, and uploads the results to code scanning.
- `release.yml` runs the release dry-run on every pull request, summarized by `release checks` ([the release spec](subsystems/release.md)). A pull request's run is always a dry-run: the signing and publishing jobs run only in release mode on `main`, and the signing workflows skip any other caller.
- **No ruleset requires these checks yet** ([repository settings](05-release.md#repository-settings) lists that step, and task M8.4 tracks it), and the bypass skips them anyway. Until then, a failing check is a signal the maintainer has to read, not a gate.

## Review

The pull request template, `.github/pull_request_template.md`, lists what a contributor ticks for every change, for assets, for the CLI and installer, and for formats and decisions. [`CONTRIBUTING.md`](../CONTRIBUTING.md) explains each item.

**What review must look for in a catalog contribution.** `cargo xtask validate` checks form: the schema, the permissions that declared capabilities imply, a `shell` grant for bundled scripts, markers and fences, links, version bumps, and the MCP hardening's text searches (threat M12 in [the MCP spec](subsystems/mcp.md)). It can't check intent. Review must:
- **Read model-facing text as a prompt-injection surface.** Rules, `SKILL.md` and everything a skill ships, an agent's prompt and its `description`, and an MCP server's instructions and tool descriptions all reach the model ([content that reaches the model](subsystems/assets.md#content-that-reaches-the-model)), and often in clients the asset wasn't installed for ([cross-client leaks](subsystems/adapters.md#cross-client-leaks)).
  - Read them as instructions an agent will follow.
  - Look for text that tells it to run commands, fetch or send data, change settings or files outside the task, or ignore the user and the project's own instructions.
  - Look also for text hidden from a reader ([content that reaches the model](subsystems/assets.md#content-that-reaches-the-model)).
- **Check that the declared permissions match the behavior.** `[permissions]` is what a user consents to ([threat F14](01-architecture.md#threats-and-mitigations)). It is not a sandbox: neither Grimoire nor any client enforces it on a binary or a script. Validation checks declarations against each other and against the files an asset ships, never against what its code does. Review compares the code with the declaration: what it reads and writes, whether it uses the network or starts processes, and that every environment variable it reads is in `permissions.env`.
- **Review binaries as code.** A tool or MCP server is a Rust crate that releases build into a binary, and it runs on users' machines with their privileges: as a hook on client events, such as every tool call, or as a server in every session. Review its source, its dependencies (`deny.toml` applies to them), its input handling and its limits, as for the core crates. That an MCP server serves through `agent-grimoire-mcp` is checked by text searches that a determined author can get around (threat M12).

## The repository and contributions

Grimoire has one maintainer. This section says what the repository's rules, CI and review guarantee today, and what they don't. The settings below were read with the GitHub API on 2026-11-17 and 2026-11-18, and the ruleset again on 2026-11-24, and they can change; anyone who can read the repository can see its active rulesets on GitHub. How to contribute is in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

**Who can change `main`:**
- The repository belongs to a personal account, and its owner is the only collaborator. Everyone else contributes from a fork (their own copy of the repository).
- A ruleset on every branch requires a pull request with one approval and a code owner's review, allows only merge commits, and blocks deleting branches and force-pushing (overwriting a branch's history).
- **CodeQL's results gate nothing.** When read on 2026-11-17 and 2026-11-18, the ruleset also required code-scanning results from CodeQL (GitHub's code analysis) and code-quality results (COMMUNITY-P7 in `specs.md`, the Phase 7 facts about GitHub's community files and rulesets). Read on 2026-11-24, both rules are gone.
  - `codeql.yml` (below) runs CodeQL on pull requests into `main` and pushes to `main`, and pull request 1 received a passing `CodeQL` check on 2026-11-19.
  - No ruleset requires that check, so a failed analysis or a new alert doesn't block a merge. Whether to make CodeQL a gate on `main` again is the maintainer's decision ([repository settings](05-release.md#repository-settings), and task M8.4 in [`roadmap.md`](roadmap.md)).
  - CodeQL default setup must stay off, because GitHub rejects advanced-setup results while it is on (CODEQL in `specs.md`).
- **The ruleset blocks release-plz.** It covers the `release-plz-*` branches too, so its pull-request rule refuses release-plz's commit there (most recently run 45513260333, on 2026-11-24; [repository settings](05-release.md#repository-settings), and task M8.3). Its exclusion for those branches reads `refs/heads/refs/heads/release-plz-*`, with `refs/heads/` twice, and matches none of them. The release pull request, and so every release, depends on the maintainer changing the ruleset.
- [`.github/CODEOWNERS`](../.github/CODEOWNERS), the file that names who must review changes to each path, makes the maintainer the owner of every path. The security-sensitive paths have lines of their own, so that a future co-maintainer added for every path doesn't also get these:
  - `.github/`, the `Dockerfile` and `.grype.yaml`, `deny.toml`, `packaging/`;
  - `xtask/` and `.cargo/` (the release logic and the `cargo xtask` alias);
  - the installer, resolver, adapter and MCP hardening crates;
  - the skill, tool and MCP server assets;
  - the Claude Code marketplace (`plugins/` and `.claude-plugin/`);
  - and the instructions that coding agents follow in this repository (`.claude/`, `AGENTS.md`, and `CLAUDE.md`), `SECURITY.md`, and `release-plz.toml`.
- **The ruleset can be bypassed.** The repository's administrator role and some GitHub Apps can bypass it. GitHub doesn't let a pull request's author approve it, so the maintainer's own changes reach `main` through that bypass. **Most changes have no second reviewer**, and nothing in this document means that every change to `main` was reviewed by a second person, or passed CI.
- A contributor's pull request needs the maintainer's approval, as its code owner. With the code-scanning and code-quality rules gone, that approval meets the ruleset, so the merge no longer needs the bypass. This hasn't been seen on GitHub yet.

**Dependency policy:** Rust crates, updates, actions and tools follow [`02-dependency-policy.md`](02-dependency-policy.md).

**Residual risks:**
- **One account can change everything.** The maintainer's account can change `main`, the workflows and the repository's settings, and approves every release, with no second reviewer and no passing check. A compromised maintainer account is a compromised project, whatever the release's approvals say ([the release spec](subsystems/release.md#the-trust-chain)). The GitHub Apps on the ruleset's bypass list can skip its rules too.
- **Review is one person's.** Prompt injection in model-facing text, permissions that don't match a binary's behavior, and malicious code in an asset crate are caught only if the maintainer catches them.
- **Fixing a reported vulnerability in private skips CI.** GitHub runs no CI or status checks on the pull requests of an advisory's temporary private fork (the private copy of the repository GitHub makes for fixing a reported vulnerability), and merging them doesn't enforce the target branch's protection rules. A fix made there must be checked locally before it is merged.
