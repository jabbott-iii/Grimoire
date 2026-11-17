# Grimoire: implementation plan

- **Status:** Phases 0 and 1 approved and committed 2026-11-16. **Phase 2 implemented 2026-11-16, awaiting review** (see §5, Phase 2 outcome).
- **Date:** 2026-11-16

## Decisions recorded (2026-11-16)

- **All Phase 0 proposals are accepted.** ADR-0001 to ADR-0005 are now *Accepted*, and every proposal in `architecture.md` is approved.
- **Q1, license:** **`MIT OR Apache-2.0`**. `LICENSE` becomes `LICENSE-MIT`, `LICENSE-APACHE` is added, and the Cargo metadata uses the dual expression. This was done in Phase 1, not deferred to Phase 7.
- **Q2–Q10:** as proposed below.
- **Still open: repository spelling (C8).** Until you decide, the `repository` URL is left out of every config, including the Cargo metadata. Decide before Phase 5.
- **Local tooling approved:** Rust 1.96 (MSRV), cargo-deny, cargo-shear, zizmor, actionlint, gitleaks, and the `gh-actions-lock` extension.
- **Phase 2, filesystem layer:** cap-std with scoped deny.toml entries (the maintainer's choice when asked, [ADR-0006](docs/adr/0006-filesystem-confinement.md)).
- **Phase rules:** each phase ends with a report and a stop. Nothing is committed, pushed, tagged, published or registered by the agent; the maintainer does all of that.

## Where things are

| Document | Purpose |
|---|---|
| [`docs/specs.md`](docs/specs.md) | Register of every external spec, with version, URL, date checked and status. IDs such as `CC-MEM` are cited everywhere else |
| [`docs/architecture.md`](docs/architecture.md) | Asset-kind definitions (for confirmation), repo layout, manifest, config and lock formats, install model, adapters, CLI |
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
4. **Trivy and `trivy-action` were compromised in March 2026** (CVE-2026-33634, verified). The container gate will use **Grype** (`anchore/scan-action`), pinned by SHA.
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
| C8 | **Repository name.** The remote is `github.com/jabbott-iii/Grimoire`; the brief says `grimoire`. OIDC subjects, trusted-publisher configs and cosign identities embed it. | Pick the canonical spelling before Phase 5 (Q2). Nothing written so far contains the repo URL in any config. |
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
| `x86_64-pc-windows-msvc` | `windows-2025` | Static CRT; to be verified in Phase 5. |
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

Candidates, to be confirmed at Phase 3 start:

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

- **Adapters:** `codex`, `cursor`, `gemini-cli`, `copilot`, each re-verifying its `specs.md` entries first.
- **Marketplace:** `cargo xtask gen-marketplace [--check]` generates `.claude-plugin/marketplace.json` and `plugins/claude-code/…`. Optionally `claude plugin validate --strict` in CI, if you approve installing Claude Code there.

### Phase 5: Release pipeline and every channel, dry-run only

- **Pipeline:** release-plz config plus the hand-written release workflow (ADR-0003).
- **Artifacts:** checksums, CycloneDX 1.6 SBOMs, `actions/attest` provenance, cosign v3 bundles, immutable-release flow.
- **Channels:** crates.io (with binstall metadata), npm (launcher + platform packages, stage publish), PyPI (maturin), Homebrew tap formula, Scoop manifest, winget manifests.
- **Installers:** a spike on dist's installers, then either adopt them or write our own `install.sh`/`install.ps1` under a language-policy exception ADR.
- **Dry-runs:** every channel is exercised on PRs.
- **Bootstrap:** a written checklist for you (§6).

### Phase 6: Docker image and its CI gates

- **Image:** multi-arch (amd64, arm64) from the static musl binaries on `gcr.io/distroless/static-debian13:nonroot`, pinned by digest, with the base signature verified via cosign. Non-root, no shell, OCI annotations.
- **Supply chain:** BuildKit SBOM + max provenance, cosign signature, and a Grype gate.
- **Docs:** usage for the CLI against a mounted repo, and for MCP servers on stdio and HTTP.

### Phase 7: Documentation pass and contributor tooling

- **Docs:** README rewrite (the current README is a one-line placeholder), plus user docs for every command and exit code.
- **Contributor files:** CONTRIBUTING, a PR template with an asset checklist, CODEOWNERS, issue templates, the deprecation and versioning policy, SECURITY.md (private vulnerability reporting), CODE_OF_CONDUCT.md, and the final license files per Q1.
- **Docs:** `docs/security-model.md` final version.

## 6. Actions only you can take (none done by the agent)

| When | Action |
|---|---|
| Before Phase 1 CI | Enable the repo setting "Require actions to be pinned to a full-length commit SHA"; set `GITHUB_TOKEN` default permissions to read-only; enable secret-scanning push protection. |
| After the first green CI run | Add a branch ruleset on `main` that requires the CI and Security jobs to pass (and requires pull requests). Without it, failing gates do not block merges. Optionally enable "Require lockfile" for `actions.lock`. |
| Before Phase 5 | Decide the repo spelling (C8). Create the `release` environment with yourself as required reviewer (and "Prevent self-review" as you prefer). Enable immutable releases. |
| Phase 5 bootstrap | Create the npm org `agent-grimoire`. Do the first crates.io publish of each published crate (short-lived scoped token, then revoke) and set trusted-publishing-only. Do the first npm publish of each npm package, then configure trusted publishers. Add the PyPI pending publisher. Create the `homebrew-tap` and `scoop-bucket` repos. |
| Phase 7 | Enable private vulnerability reporting (currently `enabled: false` on this repo). |

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
- `gh api` reads of repo metadata: public, MIT, created 2026-11-16, no environments, private vulnerability reporting disabled.
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
