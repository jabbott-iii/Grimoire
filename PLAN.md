# Grimoire: implementation plan

- **Status:** Phases 0 to 6 approved and committed (Phase 6 on 2026-11-17). **Phase 7 implemented and reviewed, awaiting review** (2026-11-18).
- **Date:** 2026-11-18

## Decisions recorded (2026-11-16)

- **All Phase 0 proposals are accepted.** ADR-0001 to ADR-0005 are now *Accepted*, and every proposal in `architecture.md` is approved.
- **Q1, license:** **`MIT OR Apache-2.0`**. `LICENSE` becomes `LICENSE-MIT`, `LICENSE-APACHE` is added, and the Cargo metadata uses the dual expression. This was done in Phase 1, not deferred to Phase 7.
- **Q2–Q10:** as proposed below.
- **Repository spelling (C8): decided 2026-11-17.** The repository stays `jabbott-iii/Grimoire`, capital G. Trusted publishers, cosign identities and every config use that exact spelling.
- **Local tooling approved:** Rust 1.96 (MSRV), cargo-deny, cargo-shear, zizmor, actionlint, gitleaks, and the `gh-actions-lock` extension.
- **Phase 2, filesystem layer:** cap-std with scoped deny.toml entries (the maintainer's choice when asked, [ADR-0006](docs/adr/0006-filesystem-confinement.md)).
- **Phase rules:** each phase ends with a report and a stop. Nothing is committed, pushed, tagged, published or registered by the agent; the maintainer does all of that.
- **Phase 2 approved** (2026-11-16), and Phase 3 started.
- **Phase 3 exemplars confirmed as proposed** (§5). The MCP server is named `project-docs`.
- **Phase 3, MCP server auth:** RFC 9068 JWT access tokens verified locally against an operator-supplied JWKS file, with no outbound network (the maintainer's choice when asked; ADR-0007). This needs one `subtle` (BSD-3-Clause) license exception.
- **Phase 3, rmcp policy entries:** skip exactly `syn@2.0.119`, and allow rmcp 3.10.0's six test-fixture files, each pinned by SHA-256, rather than trusting its build script (the maintainer's choice).
- **Phase 3 approved** (2026-11-17), and Phase 4 started.
- **Phase 4, marketplace** (the maintainer's choices when asked, 2026-11-17):
  - one plugin per eligible asset, plus one per bundle that depends on its members' plugins;
  - marketplace name `grimoire`, owner `jabbott-iii`, no email;
  - CI installs an exact, verified Claude Code version to run `claude plugin validate --strict`, provided it runs without a login or secrets.
  - tool assets with hooks are not plugins until fail-closed plugin hooks are tested;
  - a bundle plugin depends on its eligible members' plugins, and its description names the members left out.
- **Phase 4, adapters** (the maintainer's choices when asked, 2026-11-17, after the format research):
  - **Hooks stay Claude Code only** for now. The `codex`, `cursor`, `gemini-cli` and `copilot` adapters refuse hook outputs: none of those clients can guarantee that a failed hook blocks, three run hooks only through a shell, and none can be tested here.
  - **AGENTS.md covers Cursor and Copilot.** When a rule is in `AGENTS.md`, Cursor and Copilot get no rule file of their own, and Gemini CLI gets an `@AGENTS.md` import in `GEMINI.md`. Native rule files are written only when `AGENTS.md` isn't.
- **Phase 4, `cargo deny`** (the maintainer's choice when asked, 2026-11-17): ignore RUSTSEC-2026-0543 ("chrono is unmaintained"), with its reason, in `deny.toml`. It is an unmaintained notice, not a vulnerability. rmcp 3.10.0 and 3.11.0 depend on chrono directly, so no upgrade removes it. The entry is reviewed on each rmcp bump, and `unused-ignored-advisory = "deny"` fails CI once it no longer applies.

- **Phase 4 approved** (2026-11-17), and Phase 5 started.
- **Phase 5, before implementation** (the maintainer's choices when asked, 2026-11-17):
  - one version for the CLI and its five libraries, one `vX.Y.Z` tag;
  - release-plz skips the asset crates, whose catalog CHANGELOGs stay hand-written;
  - after the research: PyPI wheels are written by `cargo xtask` from the archived binary, not by maturin;
  - Homebrew and Scoop pull the attested formula and manifest from each release, through template workflows in the tap and bucket repos;
  - winget identifier `jabbott-iii.agent-grimoire`, command alias `agent-grimoire` (`grimoire` is taken by another package);
  - release-plz opens the release PR only; the release workflow tags the commit it built and signed, and publishes.
- **Phase 5, after the review** (the maintainer's choices when asked, 2026-11-17):
  - publishing runs in four jobs with four environments (`release`, `crates-io`, `npm`, `pypi`), so one registry's job can't publish to another;
  - each binary is built on its own (`cargo auditable build -p <pkg>`), so its features and SBOM match a `cargo install` build;
  - the first release approves `release` (tag and GitHub Release) before the six crates are published by hand from that tag, and only then the registry jobs (`docs/release.md` §2 D), so crates.io never holds a version without its tag.
- **Local tooling approved for Phase 5** (scratchpad only): ShellCheck, PowerShell with PSScriptAnalyzer, twine, release-plz, cosign, cargo-auditable, cargo-cyclonedx, and the Homebrew Docker image (which couldn't run: Docker isn't reachable for this user).
- **Phase 5 approved** (2026-11-17), and Phase 6 started.
- **Local tooling approved for Phase 6** (the maintainer's choices when asked, 2026-11-17): rootless Docker from the installed `dockerd-rootless.sh`, run as the maintainer's user and stopped at the end; Grype, cosign and an OCI copy tool (crane or regctl) in the scratchpad, each checked against its published SHA-256; and the `aarch64-unknown-linux-musl` Rust target (`rustup target add`, 154 MB in `~/.rustup`), to build real arm64 binaries locally.
- **Disk space** (the maintainer's choice when asked, 2026-11-17): an earlier Grimoire session's 20 GB scratchpad was deleted, because the disk had 3.1 GB free.
- **Phase 6, after the research** (the maintainer's choices when asked, 2026-11-17):
  - `runtime = "docker"` is deferred past v1. Phase 6 documents hand-written, digest-pinned `docker run` entries instead, and `grimoire.toml` keeps rejecting the field;
  - the Grype gate fails on any medium, high or critical match, fixed or not. A finding without a fix needs an ignore rule with a reason in `.grype.yaml`;
  - a release pushes the tags `X.Y.Z` and `latest`;
  - pull request dry-runs also build `aarch64-unknown-linux-musl`, so every PR builds, scans and runs the two-platform image.
- **Phase 6, after the review** (the maintainer's choice when asked, 2026-11-17): every cosign signature and attestation, for the Phase 5 release files and the image, moves into a reusable workflow, `.github/workflows/sign.yml`, that only the signing jobs call. Every job in `release.yml` that can mint OIDC tokens got the same certificate identity, including `publish-pypi`'s tag-pinned container. Certificates now name `sign.yml`, and verifiers also pin the calling repository and `refs/heads/main`.
- **Phase 6, the image tag steps** (the maintainer's choice when asked, 2026-11-17): `.github/scripts/image-tags.sh` (push by digest, tag the version, move `latest`) is a seventh language-policy exception (ADR-0010, recorded in ADR-0011). The release jobs and the PR dry-run run the same script, so the code that guards the release tags is tested on every pull request. A Rust version would put cargo, or every xtask dependency, into the job that holds the signing identity.
- **Phase 6 approved** (2026-11-17), and Phase 7 started.
- **Phase 7, before implementation** (the maintainer's choices when asked, 2026-11-17):
  - `CODE_OF_CONDUCT.md` adopts the Contributor Covenant by reference: a short file that links the exact version and adds scope, reporting and enforcement. The agent doesn't copy the Covenant's text;
  - the conduct contact is a marked placeholder that the maintainer fills in before the first release (a `docs/release.md` checklist item);
  - the README carries a marked "not released yet" note. `cargo xtask release plan` refuses release mode while it is there, and the release PR deletes it;
  - **local tooling approved** (scratchpad only): lychee v0.24.2, checked against the SHA-256 that `ci.yml` pins, and check-jsonschema through `uvx`, to check the issue forms once;
  - `docs/cli.md` is hand-written, and a unit test in the CLI crate fails when a command, flag or exit code is in the code but not in the doc, or the other way round. No new dependency;
  - the user docs list every diagnostic code the CLI can print, and a test compares that list with the code literals in the crates. Catalog-author codes (`manifest/…`, `catalog/…`) stay out;
  - **deprecation window: one release.** A deprecated asset ships, with its replacement, in at least one CLI release before removal; a deprecated flag or command warns for at least one minor release; a file format changes only with a new `format` number, and the CLI reads the previous one for at least one more minor release;
  - no DCO sign-off: submitting a contribution licenses it as `MIT OR Apache-2.0` (Apache-2.0 §5);
  - each of the five library crates gets its own short README on crates.io.
- **Phase 7, after the research** (the maintainer's choices when asked, 2026-11-17):
  - a change that breaks any library crate's public Rust API is marked breaking (`!` or `BREAKING CHANGE`), so release-plz ships it as the next 0.y. The six crates depend on each other with `^0.1.0`, so Cargo would otherwise build an older CLI against it;
  - an MSRV bump is marked breaking too, so in 0.x it lands in the next 0.y (Q6: never in a patch release). After 1.0 it is a minor release;
  - `SECURITY.md` promises an acknowledgement within 7 days, updates in the advisory, and publication when the fix is released. It sets no deadline for the fix.
- **Phase 7, deviation (2026-11-18, forced by the review; for the maintainer's approval):** the release PR doesn't delete the `before-first-release` markers. A pull request merged to `main` ahead of it does, because release-plz manages the release PR's branch and whether it keeps a commit pushed there is UNVERIFIED. The guard also fails the release PR's own checks while a marker remains, so it can't be merged early. Markers are paired (an opening and a closing line around the text), and deleting only the opening line still fails: the review showed that a single marker line let the note it marked ship.
- **CI fixes after Phase 7** (the maintainer's choice when asked, 2026-11-18): the Release dry-run installs winget-cli v1.29.380, pinned by SHA-256, and runs `winget validate` with it. The runner's own winget is older than schema 1.28.0, and accepting its warnings would weaken the check.

## Where things are

| Document | Purpose |
|---|---|
| [`docs/specs.md`](docs/specs.md) | Register of every external spec, with version, URL, date checked and status. IDs such as `CC-MEM` are cited everywhere else |
| [`docs/architecture.md`](docs/architecture.md) | Asset-kind definitions, repo layout, manifest, config and lock formats, install model, adapters, CLI |
| [`docs/adr/0001-catalog-delivery.md`](docs/adr/0001-catalog-delivery.md) | Embedded catalog behind a `Source` abstraction |
| [`docs/adr/0002-mcp-server-and-tool-packaging.md`](docs/adr/0002-mcp-server-and-tool-packaging.md) | Separate binaries vs subcommands, with trade-offs |
| [`docs/adr/0003-release-tooling.md`](docs/adr/0003-release-tooling.md) | dist vs release-plz vs hand-written |
| [`docs/adr/0004-package-and-binary-names.md`](docs/adr/0004-package-and-binary-names.md) | Name availability evidence and the proposal |
| [`docs/adr/0005-client-priority.md`](docs/adr/0005-client-priority.md) | v1 clients and their order |

## 1. Phase 0 findings that change or refine the brief

1. **MCP target is confirmed with no gap.** 2026-07-28 is the current final MCP version, and rmcp 3.10.0 implements it: `ProtocolVersion::LATEST = V_2026_07_28`, Tier 1, 30/30 server conformance (`MCP-SPEC`, `RMCP`). There is no migration plan to write now. The next spec version is expected around 2027-03-30.
   - Two rmcp gaps must be closed in our own code:
     - **Origin validation is off by default.**
     - **Server-side auth exists only in examples.**
2. **Clients read each other's files.** For example, `.agents/skills/` is read by Codex, Cursor, Gemini CLI and Copilot. Cursor and Copilot read `CLAUDE.md`, `.claude/skills/` and Claude Code hooks. Cursor reads `.claude/agents/` without enforcing tools. Adapters therefore can't be independent. The planner works over the *selected client set* to reach each client exactly once (`architecture.md` §8.2), and `plan` warns about cross-client permission leaks.
3. **Claude Code has native path-scoped rules** (`.claude/rules/*.md` with `paths`, `CC-MEM`). Rules don't need to be blocks inside `CLAUDE.md`. A `CLAUDE.md` block is still useful as an `@AGENTS.md` import when both `claude-code` and `agents-md` are selected.
4. **Trivy and `trivy-action` were compromised in March 2026** (CVE-2026-33634, verified). The container gate will use **Grype** (`anchore/scan-action`), pinned by SHA. *(Phase 6: Grype runs from its checksum-pinned release tarball instead, because `anchore/scan-action` pipes an install script to `sh`; SCAN-P6, ADR-0011.)*
5. **GitHub now has workflow lockfiles** (`.github/workflows/actions.lock`, GA 2026-10-28). We will SHA-pin as required *and* adopt the lockfile, which covers transitive actions.
6. **`pull_request_target` is blocked by default in public repos from 2026-12-02.** We never use it anyway.
7. **dist doesn't fit two requirements.** Its npm installer downloads binaries at install time instead of using per-platform packages, and its npm and Homebrew publishing depend on long-lived tokens (ADR-0003).
8. **Agent Skills has no tagged spec versions yet**, and its reference validator is "for demonstration purposes only". Grimoire validates skills natively in Rust, which also avoids a Python dependency.

## 2. Conflicts and constraints needing your decision

Per the working agreement, each item gives the safest alternative.

| # | Conflict | Safest alternative (proposed) |
|---|---|---|
| C1 | **"No long-lived tokens"** vs registries that require a credential for a package's *first* publish. crates.io: "initial publish requires an API token". npm: the package "must already exist" before trusted publishing (`CRATES-TP`, `NPM-TP`). | A one-time **maintainer-run bootstrap**, outside CI. crates.io: a short-expiry, crate-scoped token, revoked immediately after, then enable trusted-publishing-only. npm: an interactive `npm login` session (2-hour token). PyPI needs nothing (pending publisher). No credential is ever stored in GitHub. |
| C2 | **"One crate per MCP server and tool in `crates/`"** vs **"a new asset with files under `catalog/` only"** | Code-bearing assets are workspace crates *inside their asset directory* (`catalog/mcp-servers/<id>/Cargo.toml`), picked up by member globs (`architecture.md` §4). |
| C3 | **"Explicit file map per client"** vs adapters owning client layouts (layouts change monthly) | Keep explicit per-client output lists, but express destinations as **adapter-resolved anchors** (`skills:`, `rules:`…). Resolved literal paths are shown in `plan` and recorded in the lock (`architecture.md` §6.1). Literal paths are the alternative if you prefer them. |
| C4 | **Release PRs need CI**, but PRs and tags created with `GITHUB_TOKEN` don't trigger workflows (`RELEASE-PLZ`). The usual fix is a PAT or a GitHub App key, which is a long-lived secret. | No stored credential. The maintainer triggers CI on the release PR (`workflow_dispatch` or a push), and binaries build in the same run as the release step (ADR-0003). |
| C5 | **Homebrew tap and winget automation** normally need cross-repo or classic PAT credentials (`BREW`, `WINGET`) | Homebrew and Scoop use a **pull model**: each repo autobumps itself with its own `GITHUB_TOKEN`. winget manifests are generated and validated in CI and **submitted by you**. |
| C6 | **Hooks** run automatically with user privileges, and some clients run project hooks without a trust prompt in non-interactive mode (`CC-SET`) | Hooks are bindings declared by **tool** assets only. Exec form, pointing at our signed Rust binaries, never shell strings. They need the explicit `hooks` permission grant (`architecture.md` §3.4, §6.3). |
| C7 | **LICENSE.** The repo already has an **MIT-only** `LICENSE`, while the brief's default suggestion is MIT OR Apache-2.0. | Your call (Q1). Dual licensing means adding `LICENSE-APACHE` and renaming to `LICENSE-MIT`. Dependencies so far (e.g. rmcp, Apache-2.0) are compatible with either. |
| C8 | **Repository name.** The remote is `github.com/jabbott-iii/Grimoire`; the brief says `grimoire`. OIDC subjects, trusted-publisher configs and cosign identities embed it. | **Decided 2026-11-17:** keep `jabbott-iii/Grimoire` exactly. |
| C9 | **Cross-OS CI** can't be verified without pushing, and I may not push. | I run every gate locally on Linux and report exactly what ran. You push a branch to get macOS and Windows results. |

## 3. Open questions

| # | Question | Proposal |
|---|---|---|
| Q1 | License | Your choice between keeping MIT and moving to `MIT OR Apache-2.0`. Catalog assets carry their own SPDX `license` field and default to the repo license. |
| Q2 | Package and binary names | `agent-grimoire` on every registry, binary `grimoire`, npm platform packages `@agent-grimoire/<os>-<arch>`, libraries `agent-grimoire-*`. Also choose the repo spelling `Grimoire` vs `grimoire` (ADR-0004). |
| Q3 | Initial (v1) clients | `claude-code` + `agents-md` (portable: `AGENTS.md` + `.agents/skills/`) in Phase 2, then `codex` → `cursor` → `gemini-cli` → `copilot` in Phase 4 (ADR-0005). Confirm that **all six are v1**, as the definition of done implies. |
| Q4 | Definition of "tool", and hooks | A standalone, non-interactive Rust executable for one local task, following CLI conventions. Installing it never installs the binary. **Hooks are bindings declared by tools**, not a separate kind (`architecture.md` §3.4). The alternative is no hooks in v1. |
| Q5 | OS/arch matrix | The six targets in §4 below, built on native GitHub-hosted runners. PR CI runs Linux x64, macOS arm64 and Windows x64. Release and nightly runs cover all six. |
| Q6 | MSRV | Policy: **stable minus 4 minor releases** (about 6 months), so **1.96** at Phase 1 start (stable is 1.100.0). Deliberate bumps only, never in a patch release. The floor imposed by dependencies today is **1.88** (rmcp, sha2, keyring, proptest). The alternative is pinning MSRV at 1.88 for maximum reach. |
| Q7 | Asset-kind definitions | Confirm rule, skill, agent, MCP server and bundle as defined in `architecture.md` §3, including the **required explicit agent `tools` list** and **refusing clients that can't enforce it**. |
| Q8 | Permission consent | Confirm that declared permissions must be accepted per asset in `grimoire.toml` (`allow = [...]`), and that permission growth on update stops and re-asks (`architecture.md` §6.3). |
| Q9 | MCP and tool packaging | Confirm separate binaries, GitHub Releases + GHCR only for v1, not published to registries (ADR-0002). |
| Q10 | Release tooling | Confirm release-plz + a hand-written pipeline, with dist not adopted (ADR-0003). |

## 4. Proposed target matrix (Q5)

| Target | Build runner | Notes |
|---|---|---|
| `x86_64-unknown-linux-musl` | `ubuntu-24.04` | Static. Pure-Rust dependencies only; no TLS in the CLI. |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | Static. Native arm64 runner, free for public repos. |
| `x86_64-apple-darwin` | `macos-15-intel` | GitHub plans to drop Intel macOS "after the macOS 15 image is retired in Fall 2027". Fall back to cross-compiling from arm64 if needed. |
| `aarch64-apple-darwin` | `macos-15` | |
| `x86_64-pc-windows-msvc` | `windows-2025` | Static CRT: Rust links it dynamically by default, so release builds pass `-C target-feature=+crt-static` (Phase 5). |
| `aarch64-pc-windows-msvc` | `windows-11-arm` | Native arm64 runner. |

Labels are pinned explicitly. `ubuntu-latest` changes to 26.04 this month (`RUNNERS`).

## 5. Phases

Every phase ends with:
- a report of exact commands run and their outcomes;
- a list of checks *not* run;
- a proposed Conventional Commits message with no attribution lines;
- then a stop.

### Phase 1: Workspace skeleton, model, schemas, validation, xtask, CI gates

**Outcome (2026-11-16):** implemented, and every gate passes locally on Linux. CI on macOS and Windows still needs a push.

**Deviations from the plan below**, each recorded where it applies:
- **Cooldown pins.** Pinned actions and tools are releases at least 7 days old: install-action v2.90.3, cargo-deny 0.20.2, cargo-shear 1.14.0 (`CI-PINS` in `specs.md`).
- **Workspace globs.** The `catalog/*/*` member globs wait for the first code asset (Phase 3), because Cargo rejects globs that match nothing.
- **Agent `model` hint deferred.** Neither it, tool permission allow-list outputs, nor `runtime = "docker"` is modelled yet (`architecture.md` §3.4, §6.1, §6.2).
- **ASCII-only paths.** `RelPath` and all catalog files are ASCII, and case-insensitive collisions are rejected.
- **No build caching** in CI.

**Review:** an adversarial review (six lenses, each finding re-checked by a skeptic) produced 54 findings. 51 were confirmed, 2 were uncertain and 1 was refuted. Every confirmed or uncertain finding is fixed, and each fix is pinned by a regression test. The highest-severity one: renames and moves bypassed the version-bump gate (now `git diff --no-renames`).


- **Workspace and crates:** resolver 3, edition 2024, `rust-version` per Q6. Crates `crates/{model,catalog,resolve,install,adapters,cli}` with names per ADR-0004, plus `xtask`.
- **Model:** types for `asset.toml`, bundles, `grimoire.toml` and `grimoire.lock`, with `deny_unknown_fields`, a `format` version field, and the validation rules in `architecture.md` §6.4.
- **Schemas:** `cargo xtask schema [--check]` uses schemars 1.x to write `schemas/*.schema.json`.
- **Validation:** `cargo xtask validate`, covering:
  - per-asset and catalog-wide lints;
  - the version-bump + CHANGELOG check against the merge base;
  - Agent Skills rules implemented natively.
- **Repo conventions:** `deny.toml`, `.github/dependabot.yml` (cargo, github-actions, docker; grouped; cooldown), and `CLAUDE.md` + `AGENTS.md` for this repo. `CLAUDE.md` contains `@AGENTS.md`, giving a single source of truth.
- **CI gates** (SHA-pinned with version comments, plus `actions.lock`; `permissions: {}` at the top with per-job grants):
  - fmt and clippy `-D warnings`;
  - tests on ubuntu, macOS and Windows;
  - MSRV build;
  - cargo-deny;
  - unused dependencies: Cargo's `unused_dependencies` lint denied, plus cargo-shear for dev-dependencies;
  - gitleaks, run as the MIT binary;
  - zizmor + actionlint on workflows;
  - catalog validation;
  - schema freshness;
  - lychee link check.
- **Exit:** all gates pass locally on Linux, with an exact report. You push to confirm macOS and Windows.

### Phase 2: Resolver, installer, lockfile, CLI core, `claude-code` + `agents-md`

**Outcome (2026-11-16):** implemented, and every gate passes locally on Linux. CI on macOS and Windows still needs a push.

What was built:
- **Crates:**
  - `agent-grimoire-resolve`: selection, versions, lock pins, conflicts, clients, consent.
  - `agent-grimoire-install`: confined filesystem, managed blocks, JSON entries, the pure planner, apply and verify.
  - `agent-grimoire-adapters`: `claude-code`, `agents-md` and rule sharing.
  - The `grimoire` CLI, with all eleven commands.
  - `agent-grimoire-testkit`, test-only.
- **Generated:** the embedded catalog pack, with `cargo xtask pack-catalog [--check]` in CI.
- **Specs:** the Claude Code formats were re-verified (`CC-P2`), and the filesystem and editing crates were evaluated (`FS-CRATES`, `EDIT-CRATES`).

Deviations and refinements, each recorded where it applies:
- **`[allow]` table** for assets brought in by bundles, tags or `requires` (`architecture.md` §6.2).
- **Lock additions:**
  - a top-level `[[output]]` for Grimoire's own `CLAUDE.md` import block;
  - `created-file` and `created-keys`, so removal deletes only what Grimoire created (§6.5);
  - the reserved id `grimoire` and prefix `grimoire-`.
- **Path-scoped rules shared via `AGENTS.md`** when both clients are selected, instead of also writing `.claude/rules/` (avoids double loading; §8.2).
- **Pinned versions:** `plan` and `apply` stop whenever a locked version differs from the catalog's, not only when content is needed (§7.5, ADR-0001).
- **Strict JSON only.** jsonc-parser's CST accepts comments even in strict mode, so files are validated with serde_json first (`EDIT-CRATES`).
- **`Source` is a type, not a trait** (§4).
- **Deferred to Phase 3:** HTTP transport configuration; per-name env-var consent (§11.7, §11.8).
- **MCP server ids** `workspace` and `computer-use` are rejected; Claude Code reserves them.

**Review:** an adversarial review (six lenses, each finding re-checked by a skeptic) produced 51 findings: 44 confirmed, 1 uncertain and 6 refuted. The confirmed and uncertain ones, 37 distinct after duplicates across lenses, are all fixed. Most have a regression test; one was settled by documentation (JSON layout, below). Three of the fixes were mutation-checked: each test fails without its fix. The most important:
- **High: second-apply conflict.** A rule whose text did not end in exactly one newline was hashed before normalisation but written after it, so the second `apply` reported a permanent conflict. Blocks are now hashed exactly as written.
- **Consent.**
  - `add --allow` granted the offered permissions to every selected asset that lacked them, including an installed asset whose new version grew permissions. Consent now covers only what the command names.
  - `remove` of an asset that something else still selects dropped its grant and wrote a config that no longer resolved. It now keeps the grant, resolves before writing, and suggests `exclude`.
- **Lock confinement.** A tampered lock could make `plan` print, or `apply` delete, any project file (`.git/HEAD`). Lock paths are now limited to the locations Grimoire writes.
- **Hooks and sidecars.**
  - An edited hook element was silently "forgotten" on removal. It is now a conflict.
  - `--write-new` overwrote an existing `.new` (a merge in progress). It now never replaces a different one.
- **Platform cases.**
  - A case-only rename between versions deleted the file on macOS and Windows. It is now refused.
  - CRLF checkouts of `grimoire.lock` made `plan --check` fail forever.
  - `doctor` exited 0 on an unresolvable project.
- **Hardening:**
  - non-blocking, re-checked reads (FIFO swap);
  - permissions applied before content;
  - partial applies recorded in the lock;
  - "in the way" files as conflicts (exit 4);
  - markers only at column 0;
  - BOM support;
  - unclosed fences rejected in rules;
  - rule and agent files must be `<id>.md`;
  - MCP tool names Claude Code would rename are refused.
- **Documented rather than changed:**
  - JSON containers written on one line are reflowed when Grimoire inserts into them.
  - Empty directories are pruned after a file Grimoire owns is removed.


- **Resolver:** bundles, tags and exclude; dependencies; conflicts; client support; `min-cli-version`; permission consent; clear diagnostics (`architecture.md` §7.5).
- **Installer:**
  - root-confined filesystem (`cap-std`) and atomic writes;
  - file, block and entry units;
  - the drift matrix (§7.3) with `--force` and `--write-new`;
  - a packed builtin catalog (`cargo xtask pack-catalog --check`).
- **JSON/JSONC:** evaluate a comment-preserving editor; write an ADR if we hand-roll one.
- **Commands:** `init`, `list`, `show`, `add`, `remove`, `plan`, `apply`, `update`, `verify`, `uninstall`, `doctor`, all with `--json`, `--no-input` and documented exit codes.
- **Adapters:** `claude-code` and `agents-md`, plus the cross-client planner.
- **Tests:**
  - integration tests in temporary fixture repos: idempotency, drift, removal, uninstall leaves nothing, path traversal, absolute paths, symlink escapes, Windows reserved names;
  - proptest for block and entry merge/unmerge round-trips;
  - insta snapshots per adapter and per client combination.
- **Docs:** `docs/security-model.md`, first version (installer filesystem threat model).

### Phase 3: One exemplar per kind, end to end, plus scaffolding

**Outcome (2026-11-17):** implemented and reviewed. Every gate that runs locally passes on Linux (329 tests). CI on macOS and Windows still needs a push.

What was built:
- **Exemplars** (all `0.1.0`, `experimental`), each installed from the embedded catalog end to end in `crates/agent-grimoire/tests/builtin.rs`:
  - `conventional-commits` (rule);
  - `adr-writer` (skill, with two references and a template);
  - `security-reviewer` (agent; `read` and `search`);
  - `secret-guard` (tool with fail-closed Claude Code hooks; 16 vendor-format rules);
  - `project-docs` (MCP server: `list_docs`, `read_doc`, `search_docs`, `list_adrs`).
- **Bundles:** `security-baseline` and `rust-cli`.
- **MCP hardening:** `crates/agent-grimoire-mcp`, used by every server; the auth decision is [ADR-0007](docs/adr/0007-mcp-server-authorization.md).
- **Scaffolding:** `cargo xtask new <kind> <id>`, with templates in `xtask/templates/`. A CI job generates a tool and a server and builds, lints, tests and validates them.
- **Specs:** five research topics, each re-checked by a skeptic. They are recorded in `docs/specs.md` (Phase 3 additions).
- **Docs:** `docs/security-model.md` gains MCP servers, tool hooks and model-facing content (prompt injection).

Design settled, each recorded where it applies:
- **Environment consent per variable** (`env:<NAME>`; `architecture.md` §6.3, settles §11.7).
- **HTTP transport `url`** (`ServerUrl`; §6.2, settles §11.8).
- **Server-side auth** (ADR-0007, settles §11.4).
- **`on-failure = "block"`** for hook bindings (§3.4).
- **`permissions.env` vs `[mcp-server] env`:** what a server reads versus what the client forwards (§6.1).
- **New catalog checks** (§6.4):
  - for every asset crate: `crate/core-dependency`, `crate/path-dependency` and `crate/nested-manifest`;
  - for MCP servers: `crate/hardening`, `crate/raw-transport` and `crate/declared-tools-test`;
  - in manifests and configs: `manifest/hook-on-failure` and `resolve/options-kind`.

**Review (2026-11-17).** A multi-agent review of the Phase 3 change raised 43 findings. Each was checked by an independent verifier: 41 were confirmed and 2 were uncertain, and all 43 are fixed. The main ones:
- **2025-11-25 clients over HTTP** were refused, which included the stable Claude Code release. The server now checks the protocol version itself, after authorization, and serves both versions statelessly (ADR-0007).
- **A client could keep a connection slot forever** by pipelining requests or stalling. Keep-alive is now off, and every connection is closed after 75 seconds. A test that sends its body slowly pins this; the test was checked against the old code, where it fails.
- **The docs claimed checks that did not exist.** `crate/raw-transport` and `crate/declared-tools-test` now exist, and the docs say they are text searches and that review is the guarantee.
- **Token checks:** an `iss` array that merely contained the issuer was accepted; a non-Bearer scheme got 400 instead of 401; a well-formed but empty key set did not revoke the previous keys.
- **`secret-guard`:** fixed-length patterns missed longer tokens; secrets in object keys and target paths went unscanned; a label could echo a secret.
- **`project-docs`:** dependency and build directories were skipped only in listings, not in reads; search offsets were wrong after case folding; a search could read past its budget.
- **Docs and templates:** the server template now requires `--no-auth`, `cargo xtask new` formats what it generates, and the CI scaffold job checks formatting.

Deviations from the candidate list:
- `secret-guard` uses subcommands: `scan`, `hook` and `rules`.
- `secret-guard`'s hooks check every tool call, not only edits.
- `project-docs` serves tools only, not resources.
- `project-docs` takes its root from `--root` or `CLAUDE_PROJECT_DIR`, and refuses to start without one.

Candidates, as confirmed at Phase 3 start:

| Kind | Candidate |
|---|---|
| Rule | `conventional-commits` |
| Skill | `adr-writer`, with references and a template |
| Agent | `security-reviewer` (`read` + `search` only) |
| Tool | `secret-guard`: detects likely secrets in stdin or paths, with an exit code usable as a hook blocker, and `--json`. It demonstrates a hook binding |
| MCP server | a read-only, path-confined docs/ADR server with list/read/search tools, annotations, size limits, stdio + Streamable HTTP, Origin enforcement and the auth layer |
| Bundles | `rust-cli`, `security-baseline` |

- **MCP hardening:** a shared crate for server hardening, plus an ADR on the server-side auth approach (`architecture.md` §11.4).
- **Scaffolding:** `cargo xtask new <kind> <id>`, with templates derived from the exemplars.
- **Docs:** `docs/security-model.md` gains MCP permissions and prompt injection in rule and skill content.

### Phase 4: Remaining adapters + generated Claude Code marketplace

**Outcome (2026-11-17):** implemented and reviewed. Every gate that runs locally passes on Linux. CI on macOS and Windows, and the new Claude Code plugin validation job, still need a push.

What was built:
- **Specs first.** Five research topics (Codex, Cursor, Gemini CLI, Copilot, and Claude Code plugins and CI), each re-checked by a skeptic: 421 of 454 facts confirmed, 20 corrected, 13 left UNVERIFIED. They are recorded in `docs/specs.md` (Phase 4 additions).
- **Adapters** `codex`, `cursor`, `gemini-cli` and `copilot`, and a placement pass that writes shared rules and skills once ([ADR-0008](docs/adr/0008-phase-4-client-adapters.md), `architecture.md` §8.2–§8.3).
- **Entry editing** for TOML (`.codex/config.toml`) and JSONC (`.gemini/settings.json`), chosen by path.
- **CLI:** a Codex `AGENTS.md` size warning; per-client trust notes and the Copilot cloud-agent MCP snippet in `doctor`; `GEMINI.md` as a second file Grimoire may own a block in.
- **Exemplars** `conventional-commits`, `adr-writer` and `security-reviewer` support the new clients (all `0.2.0`), and both bundles moved to `0.2.0`.
- **Marketplace:** `cargo xtask gen-marketplace [--check]` writes `.claude-plugin/marketplace.json` and five plugins under `plugins/claude-code/`. CI checks it is current and runs `claude plugin validate --strict` with a pinned, checksum-verified Claude Code 2.1.311.

Deviations from the Phase 4 candidate list (each the maintainer's choice or forced by the research, recorded in ADR-0008):
- Hooks stay Claude Code only.
- Codex gets no agents; Cursor gets only `read`-and-`search` agents.
- Cursor and Copilot read shared rules from `AGENTS.md` instead of their own files when `AGENTS.md` is written.
- `project-docs` and `secret-guard` stay Claude Code only: the server needs `CLAUDE_PROJECT_DIR`, which only Claude Code sets, and hooks are Claude Code only.

**Review (2026-11-17).** A five-area review (adapters, installer, CLI, marketplace and CI, docs) raised 48 findings. An independent skeptic checked each one: 47 were confirmed and 1 was uncertain. All 48 are addressed. The main ones:
- **TOML editing could lose or rewrite the user's text** (2 high). A CRLF or byte-order-mark `.codex/config.toml` was rewritten on every apply, and removing Grimoire's table deleted any comment the user had put above it. Unchanged files now keep their bytes, edited ones keep their line endings, and those comments move to the next header. A 5,000-case property test with CRLF files and added comments now covers both.
- **Comments inside Grimoire's entries** in TOML and JSONC files were dropped silently on update or removal. That is now a conflict (`commented`).
- **Placement:** a glob with a comma was refused for Cursor and Copilot even when `AGENTS.md` covered them; Gemini CLI read `AGENTS.md` rules not installed for it, and `plan` didn't say so; projects weren't told that `secret-guard` doesn't guard Codex or Gemini CLI (`resolve/client-not-covered`).
- **Validation:** adapter-only refusals reached users. `cargo xtask validate` now renders every asset for every client it declares, and a test installs the shipped catalog for all six clients.
- **Lock safety:** a hand-edited lock could claim a whole `.codex/config.toml` as a Grimoire file. Lock records must now name the unit Grimoire owns at that path.
- **Marketplace:** plugin versions now change with plugin content; agents depend on their MCP servers' plugins, which must exist; READMEs list the permissions a plugin install grants without asking; bundle descriptions give the real reason for each left-out member.
- **Docs** overstated what Cursor's `readonly` and VS Code's Copilot enforce, and stated some unverified client behaviour as fact. They now say what is documented.

### Phase 5: Release pipeline and every channel, dry-run only

**Outcome (2026-11-17):** implemented and reviewed. Every gate that runs locally passes on Linux, and one target (x86_64 musl) was built and packaged end to end. Nothing has run on GitHub: the release workflow, the other five targets, attestations, keyless signing and every registry still need a push and, for a release, the bootstrap.

What was built:
- **Specs first.** Eight research topics (crates.io and release-plz, npm, PyPI, Homebrew and Scoop, winget, supply chain, Actions and builds, and the installer spike), each re-checked by a skeptic: 524 of 567 facts confirmed, 27 corrected, 16 left UNVERIFIED. They are recorded in `docs/specs.md` (Phase 5 additions).
- **`cargo xtask release`** (`plan`, `build`, `sbom`, `channels`, `checksums`, `verify`, `notes`): the release logic, in Rust and unit-tested, so the workflow is glue ([ADR-0009](docs/adr/0009-release-pipeline.md)).
  - Flat, reproducible archives (`.tar.gz` and a small zip writer of our own), with hostile-archive limits.
  - A CycloneDX 1.6 SBOM per archive.
  - npm launcher and six platform packages; six wheels; the Homebrew formula; the Scoop manifest; the three winget manifests; `install.sh` and `install.ps1` with the archive SHA-256s embedded.
  - `verify` regenerates everything and checks that `dist/` holds exactly the release, with the same binary in every channel.
- **`.github/workflows/release.yml`**: a dry-run on every pull request, and the release from `main` once the maintainer enables it. Plan → native builds → package → install tests on each OS → attest and keyless cosign → four approved publish jobs. **`release-plz.yml`** opens the release PR.
- **Installers** `install.sh` and `install.ps1`, and the npm launcher, under a language-policy exception ([ADR-0010](docs/adr/0010-language-policy-exceptions.md)). The dist spike failed: dist's PowerShell installer checks no checksum, and its shell installer checks one only when dist built the archives.
- **Templates** for the `homebrew-tap` and `scoop-bucket` repositories, and the maintainer's checklist and user verification in [`docs/release.md`](docs/release.md).
- **Cargo:** `repository`, `readme`, crates.io keywords and categories, cargo-binstall metadata, licence files in each published crate, and a `dist` profile (fat LTO, one codegen unit, panic unwind).

Deviations from the plan below, each recorded in ADR-0009:
- release-plz opens the release PR only (it can't tag the commit that was signed).
- PyPI wheels come from xtask, not maturin.
- Homebrew and Scoop pull attested files instead of autobump and Excavator: neither can update a multi-arch prebuilt formula safely.
- The release PR's CI runs by approving its pull_request runs: workflow_dispatch checks don't satisfy required checks (forced by the research).
- `install.sh` requires curl: wget can't refuse an https→http redirect (forced by the review).
- Release mode needs the repository variable `GRIMOIRE_RELEASES=enabled` and a CHANGELOG section for the version, so nothing releases before the bootstrap or outside a release PR.

**Review (2026-11-17).** A six-area review (workflow security, the xtask code, the scripts, channel formats, docs, integration) raised 64 findings. An independent skeptic checked each one: 59 were confirmed (2 medium, the rest low) and 5 refuted. All 59 are addressed. The main ones:
- **Medium: one publish job could publish anywhere.** Every step in it, including the PyPI action's container (an image referenced by tag), could mint a crates.io token. Publishing is now four jobs in four environments (the maintainer's choice).
- **Medium: the npm dry-run failed on every PR after a release** because the version was already on npm. PRs now dry-run against an unreachable local registry.
- **Artifacts are bound to the package job.** Release jobs download by artifact ID and check `SHA256SUMS` against the hash the package job recorded before attesting, signing or publishing.
- **Hostile archives:** duplicate members, extra gzip members and overlapping zip entries are refused, and CLI archives must be byte-for-byte what `build` writes.
- **Re-runs:** staged npm versions block a re-run (CI can't see them), so npm is its own job with a clear error; the recovery steps for a stuck release are documented.
- **Smaller fixes:** per-package builds; `release checks` as one required check; release-plz on `main` only; sorted wheel tags; Scoop's `|` licence separator; the binstall URLs now tested; licence files in the crates; the launcher forwards signals; `install.ps1` restores the old binary if the swap fails.


- **Pipeline:** release-plz config plus the hand-written release workflow (ADR-0003).
- **Artifacts:** checksums, CycloneDX 1.6 SBOMs, `actions/attest` provenance, cosign v3 bundles, immutable-release flow.
- **Channels:** crates.io (with binstall metadata), npm (launcher + platform packages, stage publish), PyPI (maturin), Homebrew tap formula, Scoop manifest, winget manifests.
- **Installers:** a spike on dist's installers, then either adopt them or write our own `install.sh`/`install.ps1` under a language-policy exception ADR.
- **Dry-runs:** every channel is exercised on PRs.
- **Bootstrap:** a written checklist for you (§6).

### Phase 6: Docker image and its CI gates

**Outcome (2026-11-17):** implemented and reviewed. Every gate that runs locally passes on Linux (454 tests), and the two-platform image was built, checked, scanned, pushed to a local registry, signed offline and run, with rootless Docker. Nothing has run on GitHub: the image jobs, keyless image signing, attestations to GHCR, the `ghcr` environment and the arm64 runner still need a push and, for a release, the bootstrap.

What was built:
- **Specs first.** Five research topics (base image, BuildKit, GHCR and signing, Grype, running the image), each re-checked by a skeptic: 333 facts, 308 confirmed, 25 corrected, none refuted. After the review, a sixth (SIGN-P6: signing from a reusable workflow). They are in `docs/specs.md` (Phase 6 additions).
- **`Dockerfile`** (COPY only, on distroless `static-debian13:nonroot` pinned by a digest at least 7 days old) and **`.grype.yaml`** (the gate). The CLI is the entrypoint; asset binaries sit in `/opt/grimoire/bin`, off `PATH`.
- **`cargo xtask release image`** (`base`, `context`, `check`, `coverage`): the build context and buildx arguments from the release archives, and a strict check of the OCI layout BuildKit writes: the verified base's layers and config first, then exactly the archived binaries, read as strictly as Docker reads layers.
- **`release.yml`:** `image` (base signatures, a docker-container builder on pinned BuildKit, SBOM and max provenance, `check`, the Grype gate on both platforms), `verify-image` (each platform on its own runner, from a local registry: every binary, the CLI on a mounted repository, MCP over stdio; on PRs also offline signing and the tag script's cases), `publish-image` (environment `ghcr`: push by digest) and `sign-image`. Pull requests now also build arm64 Linux.
- **`.github/workflows/sign.yml`**: every signature and attestation, for the release files and the image, after the review. **`.github/scripts/image-tags.sh`**: push by digest, tag, move `latest`, run by the release and by every PR.
- **`security.yml`:** a weekly Grype rescan of the latest release's image, and ShellCheck for the script. **Dependabot** watches the Dockerfile.
- **Docs:** [`docs/container.md`](docs/container.md) (verifying and running the image, MCP clients, HTTP with authorization), [ADR-0011](docs/adr/0011-container-image.md), and Phase 6 parts in `docs/release.md` and `docs/security-model.md`.

Deviations from the plan below, each recorded in ADR-0011:
- `runtime = "docker"` is deferred past v1 (the maintainer's choice).
- The OCI layout is built once and pushed byte for byte with crane, rather than rebuilt in the push job, so the registry holds exactly the bytes that were scanned.
- Grype runs from its checksum-pinned tarball, not `anchore/scan-action`, which pipes an install script to `sh`.
- Signing moved into `sign.yml`, and the tag steps into a shell script that is a language-policy exception (both the maintainer's choices, after the review).

**Review (2026-11-17).** A five-area review (workflows, the xtask code, the image and its supply chain, docs, specs) raised 39 findings. An independent skeptic checked each one: 37 were confirmed (1 high, 8 medium, 28 low) and 2 refuted. All 37 are addressed. The main ones:
- **High: `check` read layers more loosely than Docker.** A second gzip member, or a directory entry with a size, could install another binary unseen; the skeptic ran one. `check` now reads layers strictly, compares the config and directories exactly, and refused all seven of the review's tampered layouts.
- **Medium: one signing identity for every OIDC job.** Every job in `release.yml` could get the release's certificate identity, including `publish-pypi`'s tag-pinned container. Signing moved into `sign.yml` (the maintainer's choice), whose jobs refuse any caller but `release.yml` on `main`; verifiers pin the caller too.
- **Medium: a misspelt key in a `.grype.yaml` ignore rule** silently widened it to a whole package. A test now refuses unknown keys and incomplete rules.
- **Medium:** `actions.lock` lacked an entry the new scan job used; a re-run of the image push could deadlock after a partial attestation; the image config and directory metadata were only partly checked.
- **Smaller fixes:** tags only after signing, and `latest` only past a signed release; the Dockerfile allow-list; coverage of the Debian packages; every platform scanned before the gate fails; the weekly scan can't be cancelled by a push; docs that overstated what ran.

- **Image:** multi-arch (amd64, arm64) from the static musl binaries on `gcr.io/distroless/static-debian13:nonroot`, pinned by digest, with the base signature verified via cosign. Non-root, no shell, OCI annotations.
- **Supply chain:** BuildKit SBOM + max provenance, cosign signature, and a Grype gate.
- **Docs:** usage for the CLI against a mounted repo, and for MCP servers on stdio and HTTP.

### Phase 7: Documentation pass and contributor tooling

**Outcome (2026-11-18):** implemented and reviewed. Every gate that runs locally passes on Linux (472 tests). Nothing has run on GitHub: the community files, the issue forms, CODEOWNERS and the README's rendering on GitHub, crates.io and PyPI still need a push to be seen.

What was built:
- **Specs first.** Five research topics (community files and CODEOWNERS, issue forms, the security policy and the Contributor Covenant, README rendering on each registry, versioning), each re-checked by a skeptic: 264 facts, 242 confirmed, 19 corrected, 3 left UNVERIFIED, none refuted, and 77 added by the skeptics. They are in `docs/specs.md` (Phase 7 additions), with the stale private-reporting entry corrected.
- **User docs:** a new `README.md`; `docs/cli.md` (every command, global option, exit code, the `--json` envelope and the environment); `docs/diagnostics.md` (all 92 user-facing diagnostic codes); `docs/configuration.md` (`grimoire.toml`, `grimoire.lock`, permissions and the six clients).
- **Policies:** `docs/versioning.md` (the public API, versions, assets, deprecation, MSRV), `SECURITY.md` and `CODE_OF_CONDUCT.md` (the Contributor Covenant 3.0 by reference).
- **Contributor files:** `CONTRIBUTING.md`, `.github/pull_request_template.md`, `.github/CODEOWNERS`, and five issue forms with a chooser config, checked with check-jsonschema 0.39.0.
- **Checks that keep the docs true**, all in `cargo test --workspace`:
  - `crates/agent-grimoire/src/docs_check.rs`: `docs/cli.md` against clap's command tree and the `Exit` enum;
  - `crates/agent-grimoire/tests/diagnostics_doc.rs`: `docs/diagnostics.md` against the code literals in the five crates;
  - `xtask/src/readme_links.rs`: the root README links only to `https://`, `mailto:` and its own headings, because crates.io and PyPI break relative links;
  - `cargo xtask release plan` fails the release PR and the release while a `before-first-release` marker remains in `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md`.
  Each was shown to fail on a broken copy of its file.
- **The final `docs/security-model.md`**: cross-client leaks in full (§3a), the repository and contribution process as it really is (§6d), reporting through `SECURITY.md`, and new residual risks.
- **Smaller changes:** the five library crates have their own READMEs; `AGENTS.md`, `architecture.md`, `release.md`, `catalog/README.md` and `packaging/README.md` follow the new docs; the lock's `allow` description is corrected, and the schema regenerated; two help texts in `cli.rs` are corrected; `ci.yml`'s catalog job runs `cargo --locked xtask`.

Deviations from the plan below:
- The markers are cleared in a pull request merged ahead of the release PR, not by the release PR itself, and they are paired (recorded above, for the maintainer's approval).
- The README guard parses Markdown with pulldown-cmark, already a workspace dependency, as an xtask dev-dependency (one line in `Cargo.lock`), instead of a hand-written scanner.

**Review (2026-11-18).** A five-area review (user docs, the diagnostics reference and the drift checks, the policies, the contributor files, the security model and cross-document consistency) raised 63 findings. An independent skeptic checked each one: 62 were confirmed (18 medium, 44 low) and 1 refuted. All 62 are addressed. The main ones:
- **Medium: the release guard checked only the marker line.** Deleting that line alone would have published the "not released yet" note to crates.io, PyPI and every archive, where it can't be changed. Markers are now paired and matched loosely, and deleting only the opening line still fails.
- **Medium: release-plz can't open the release PR** (seen on GitHub, run 44198210657): the ruleset on every branch refuses its commit. This is a settings decision for the maintainer (§6 below; `release.md` §2 A).
- **Medium: docs that promised more than the code does.** Content that existed before Grimoire can become Grimoire's (when it's adopted, or overwritten with `--force`) and is then deleted with its asset. The drift tests check names, not descriptions. A deprecated asset selected only by a tag gets no warning period. Assets selected by a tag or a bundle move to breaking versions on `update`. Without `--locked`, an older CLI can carry a newer catalog crate. An asset binary can change without a version bump. Each statement now says what is true.
- **Medium: dead ends for reporters and contributors.** Someone without a GitHub account, or a contributor proposing a large change, had no issue form that fit, so there are now forms for a private contact and a change proposal. `SECURITY.md` named only Dependabot alerts, which few users get; it now tells users to watch releases. `release.md` now has the steps that `SECURITY.md`'s promises depend on.
- **Smaller fixes:** exit-code details, prompt answers, CODEOWNERS paths, the MSRV toolchain (1.96.0, as CI), `Cargo.lock` for new asset crates, stale design-doc lines, and specs entries.

Found but not fixed (code, outside a docs phase; each is documented as it behaves today):
- `uninstall --json` drops the files it removed from its result, and `uninstall --purge` reports removing a `grimoire.toml` that didn't exist.
- `doctor` prints a broken lock's diagnostics twice.
- `list` doesn't pad its columns.
- In an empty project, `plan --check` reports no change, but `apply` then writes a lock holding only `format = 1`.
- `init` and `doctor` check whether client files exist through a plain path, which follows a symbolic link in a parent directory. They read no content.
- `init/client-unavailable` and `resolve/client-unavailable` can't be reached in this build.
- Nothing checks that `min-cli-version` is at most the CLI version that ships the asset.

Seen on GitHub, before Phase 7 (read-only, 2026-11-18). CI has failed on each of the last three pushed commits:
- `test (macos-15)` fails `oversized_and_malformed_requests_are_refused` in `crates/agent-grimoire-mcp/tests/http.rs`;
- `test (windows-2025)` fails `every_exemplar_installs_for_every_client` and `every_exemplar_installs_verifies_and_uninstalls` in `crates/agent-grimoire/tests/builtin.rs`;
- the Release dry-run fails at `winget validate`, which rejects the manifests' schema header comment (exit code -1978335192).

Fixed in the working tree on 2026-11-18, and not yet run on GitHub:
- **Windows:** `Root::remove` held a handle to the file's directory while pruning it. cap-std opens Windows directories without `FILE_SHARE_DELETE`, so removing that directory failed and uninstall left every empty directory behind. It now closes the handle first.
- **macOS:** the test wrote a 1 MiB body after the server had already answered 413 and closed, so the write hit a broken pipe or the read a reset. It now sends only the head, and the server must refuse on the declared length alone. With that check disabled, the test still fails (503 after the 60-second request timeout).
- **winget:** exit code -1978335192 is `MANIFEST_VALIDATION_WARNING`. winget compares each header with the `$id` of its own built-in schema for that manifest version, and the runner's winget predates schema 1.28.0 (first in v1.28.190). The dry-run now installs v1.29.380 (`WINGET-P5` in `docs/specs.md`).
- **CI** runs `cargo test --no-fail-fast`. Cargo had stopped at the first failing test binary, so most of the Windows suite and part of the macOS suite had never run. A read-only audit of those tests found no other platform hazard; only a run on GitHub settles it.

- **Docs:** README rewrite (the current README is a one-line placeholder), plus user docs for every command and exit code.
- **Contributor files:** CONTRIBUTING, a PR template with an asset checklist, CODEOWNERS, issue templates, the deprecation and versioning policy, SECURITY.md (private vulnerability reporting), CODE_OF_CONDUCT.md. The licence files were already done in Phase 1 (Q1).
- **Docs:** `docs/security-model.md` final version.

## 6. Actions only you can take (none done by the agent)

| When | Action |
|---|---|
| Before Phase 1 CI | Enable the repo setting "Require actions to be pinned to a full-length commit SHA"; set `GITHUB_TOKEN` default permissions to read-only; enable secret-scanning push protection. |
| After the first green CI run | Add a branch ruleset on `main` that requires the CI and Security jobs to pass (and requires pull requests). Without it, failing gates do not block merges. Optionally enable "Require lockfile" for `actions.lock`. |
| Before merging Phase 5 | Turn on "Allow GitHub Actions to create and approve pull requests": `release-plz.yml` opens the release PR on the next push to `main`. Leave that PR unmerged until the bootstrap is done. |
| Phase 5 and 6 bootstrap | The checklist in [`docs/release.md`](docs/release.md) §2: the five environments (`release`, `crates-io`, `npm`, `pypi`, `ghcr`), immutable releases, the ruleset (CI, Security and `release checks`), the npm org and packages, the crates.io first publish and trusted publishers, the PyPI pending publisher, the tap and bucket repos, the GHCR checks (never push to `ghcr.io/jabbott-iii/grimoire` by hand; after the first push, make the package public, which can't be undone: §2 B and D), clearing the Phase 7 `before-first-release` markers (the README's note, the conduct contact and SECURITY.md's "No release exists yet"), and finally `GRIMOIRE_RELEASES=enabled`. |
| Before merging Phase 7 | Watch the repository's security alerts (Watch → Custom → Security alerts), because `SECURITY.md` promises a 7-day acknowledgement from the day it is on `main`. Keep private vulnerability reporting on (it read `enabled: true` on 2026-11-17). |
| Now: release-plz is blocked | The `BestBranch` ruleset covers every branch, so release-plz can't commit to its `release-plz-*` branch (run 44198210657: "Changes must be made through a pull request", "Waiting for Code Scanning results"). Choose: target the ruleset at the default branch only, or exclude `refs/heads/release-plz-*`. Don't make GitHub Actions a bypass actor: any workflow token with `contents: write` could then change `main`. Delete the two orphan `release-plz-*` branches (the deletion rule needs the admin bypass). [`docs/release.md`](docs/release.md) §2 A has the details. |
| Now: code scanning | The ruleset requires CodeQL and code-quality results, but no workflow here produces them and the default-setup state couldn't be read. Turn on CodeQL default setup and Code Quality, or drop those two rules, then confirm a pull request gets the results. Until then, every merge needs the bypass. |
| After merging Phase 7 | `gh api repos/jabbott-iii/Grimoire/codeowners/errors` should list no errors, and `/issues/new/choose`, opened as someone other than the owner, should show the five forms and the two links, with no duplicate security entry. |

## 7. Risks

- **Format churn.** Client formats change monthly; several of today's facts are marked Partial in `specs.md`. Each adapter phase re-verifies first, and snapshot tests make format changes visible.
- **Cross-client duplication.** Some client combinations can't avoid duplicate content (e.g. all six clients need skills in two directories). This is reported by `plan` rather than "solved" silently.
- **Agent permission fidelity varies by client.** Refusing to render rather than widening may surprise users. `show` lists the clients each agent supports.
- **Release-time growth.** The number of asset binaries grows with the catalog (ADR-0002).
- **Embedded catalog = release cadence.** Catalog-only updates need CLI releases (ADR-0001).

## 8. Phase 0 evidence and what was not checked

Commands run by me:
- `git log`, `git remote -v`, `git ls-files`, `git status`.
- Tool version probes, including:
  - rustc 1.99.0 default and 1.100.0 installed;
  - gh 2.45.0, authenticated;
  - Docker 29.9.3 with buildx 0.37.2;
  - syft 1.54.1;
  - node 18.19.1 / npm 9.2.0;
  - cargo-deny, dist, release-plz, cosign, gitleaks, zizmor, actionlint, lychee, maturin not installed.
- The Rust stable channel manifest (1.100.0).
- crates.io API version lookups for 33 crates.
- Name availability queries against crates.io, npm, PyPI, Homebrew, Scoop and winget.
- `gh api` reads of repo metadata: public, MIT, created 2026-11-16, no environments, private vulnerability reporting disabled (enabled since; re-checked 2026-11-17 in Phase 7).
- Spot re-checks against primary sources, all matching the research reports: MCP releases, rmcp protocol constants and Origin default at `rmcp-v3.10.0`, the Trivy advisory, and the crates.io first-publish rule.

Delegated: spec research was done by six read-only research subagents in this session. Each fact in `specs.md` carries the URL its agent fetched. I re-checked only the items listed above myself.

Not run, because Phase 0 makes no code changes: no `cargo` build, test, fmt, clippy or deny; no CI; no Docker build; no publish or dry-run of any channel.

Not researched yet: the topics in the "Not yet researched" table in `specs.md`, plus per-client docs for the 46 Agent Skills clients beyond the six target clients.

## 9. Proposed commit message for Phase 0

```text
docs: add phase 0 plan, architecture, spec register and ADRs

Add PLAN.md with the phased plan, open questions and maintainer-only
actions; docs/architecture.md with asset-kind definitions, layout,
manifest/config/lock formats and the install model; docs/specs.md
recording every external spec checked on 2026-11-16; and ADRs
0001-0005 covering catalog delivery, MCP server and tool packaging,
release tooling, package names and client priority.
```
