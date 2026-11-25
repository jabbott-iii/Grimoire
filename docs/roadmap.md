# Roadmap

- **Status:** Milestones M1 to M7 (the former Phases 1 to 7) are built. Their tasks are ticked from the checks run on Linux on 2026-11-24 and from the latest runs on `main`, all on `baf29f0` and all passing: CI run 45513260683, Security run 45513259615 and Release run 45513260716. Two tasks wait on GitHub: M5.5 (release-plz has never opened a release pull request) and M6.4 (no released image to scan). M8, the first public release, has not started, except M8.1, this document set, which is written and waits for the maintainer's review. **No release exists yet.**
- **Date:** 2026-11-24

What Grimoire is and why is in [`00-overview.md`](00-overview.md); this file says what is built, what comes next, and how each step is checked. It replaced `PLAN.md` on 2026-11-24 ([ADR-0013](adr/0013-roadmap-and-docs-document-set.md), this document set). Each dated decision `PLAN.md` recorded is now a note on the task it shaped, and its policy text is in [`05-release.md`](05-release.md).

How to use this file:
- Work from the top down. A task is unblocked when every task in its `Depends` line is ticked.
- Tick a task only after its `Accept` results hold and its `Verify` commands ran and passed, as the **Definition of done** in `.claude/rules/roadmap-and-docs.md` says.
- A milestone's exit criteria are checked by a human: the maintainer.
- Within each milestone, work moves through the environments in order: Linux, then macOS and Windows in CI, then GitHub itself ([`03-platforms.md`](03-platforms.md#the-order-work-moves-through-them)). A `⏳ needs …` line names the check that remains in a later one.
- Task IDs never change and are never reused. M1 to M7 record work done before this file existed; "Phase N" in older text is milestone MN.

## M1 — A validated catalog and its CI gates (Phase 1)

**Goal:** A contributor can describe an asset in a strict manifest, and CI refuses a catalog that doesn't validate, a stale schema, and any failing lint, test or policy check.

- [x] **M1.1 — repo: workspace, toolchain, and lints**
  - Note: Records work done before this roadmap, in Phase 1 (2026-11-16). Phase 0, the same day, researched the external formats (six research subagents; every fact in [`specs.md`](specs.md) carries the URL its agent read) and proposed the design. The maintainer accepted every Phase 0 proposal on 2026-11-16, which made ADR-0001 to ADR-0005 Accepted.
  - Note: The maintainer's answers on 2026-11-16: the license is `MIT OR Apache-2.0` (Q1; `LICENSE` became `LICENSE-MIT`, and `LICENSE-APACHE` was added in Phase 1); the packages are `agent-grimoire` and `agent-grimoire-*`, with the binary `grimoire` (Q2, [ADR-0004](adr/0004-package-and-binary-names.md)); and the MSRV policy is the latest stable Rust minus four minor releases, so 1.96 when stable was 1.100, moved only on purpose and never in a patch release (Q6, [the MSRV](05-release.md#the-minimum-supported-rust-version)). The floor that dependencies imposed then was 1.88.
  - Note: The repository stays `jabbott-iii/Grimoire`, with a capital G, because trusted publishers, signing identities and every config match it exactly (the maintainer's choice on 2026-11-17, conflict C8).
  - Depends: none
  - Do:
    - Set up the workspace (resolver 3, edition 2024, `rust-version` 1.96), the pinned toolchain in `rust-toolchain.toml`, and the lints once in the root `Cargo.toml` (`unsafe_code = "forbid"`, clippy pedantic), as [layers](01-architecture.md#layers) and [repository layout](01-architecture.md#repository-layout) describe.
    - Add `CLAUDE.md`, which imports `AGENTS.md`, so the repository's conventions have one source.
  - Accept:
    - Every crate inherits the workspace lints, and the workspace builds with the MSRV.
  - Verify: `cargo fmt --all --check` (expect exit 0); `cargo clippy --workspace --all-targets --locked -- -D warnings` (expect no warnings); `cargo +1.96.0 check --workspace --all-targets --locked` (expect exit 0)

- [x] **M1.2 — model: the manifest, bundle, config, and lock types**
  - Note: Records work done before this roadmap, in Phase 1. The maintainer confirmed the asset kinds as Phase 0 defined them, now in [asset kinds](subsystems/catalog.md#asset-kinds), including the required explicit agent `tools` list and refusing clients that can't enforce it (Q7, 2026-11-16). Destinations are adapter-resolved anchors such as `skills:`, not literal paths, and the resolved paths appear in `plan` and the lock (conflict C3).
  - Note: Deferred in Phase 1: the agent `model` hint, tool permission allow-list outputs and `runtime = "docker"` (the last one past v1, M6.1). Paths are ASCII only, and paths that differ only in case are rejected.
  - Depends: M1.1
  - Do:
    - Write strict types for `asset.toml`, bundles, `grimoire.toml` and `grimoire.lock`, with `deny_unknown_fields`, a `format` version and newtypes validated while parsing ([the model spec](subsystems/model.md)).
  - Accept:
    - An unknown field, or an invalid value, is an error that carries its TOML location and a stable code.
  - Verify: `cargo test -p agent-grimoire-model --locked --no-fail-fast` (expect every test to pass)

- [x] **M1.3 — xtask: JSON Schemas and `schema --check`**
  - Note: Records work done before this roadmap, in Phase 1.
  - Depends: M1.2
  - Do:
    - Generate `schemas/*.schema.json` from the model types with schemars 1.x ([the JSON Schemas](subsystems/model.md#the-json-schemas), [`schemas/`](subsystems/xtask.md#schemas)).
  - Accept:
    - `cargo xtask schema --check` fails when a checked-in schema is stale.
  - Verify: `cargo xtask schema --check` (expect exit 0)

- [x] **M1.4 — catalog: catalog loading and validation**
  - Note: Records work done before this roadmap, in Phase 1. Agent Skills had no tagged specification versions, and its reference validator is "for demonstration purposes only", so Grimoire validates skills natively in Rust, which also avoids a Python dependency (Phase 0 finding 8).
  - Depends: M1.2
  - Do:
    - Load a catalog directory, validate each asset and the catalog as a whole, including the native Agent Skills checks ([catalog validation](subsystems/catalog.md#catalog-validation)).
  - Accept:
    - Every check reports a stable code, and the hostile-input limits refuse the first value over each one.
  - Verify: `cargo test -p agent-grimoire-catalog --locked --no-fail-fast` (expect every test to pass)

- [x] **M1.5 — xtask: `validate`, with the version-bump check**
  - Note: Records work done before this roadmap, in Phase 1. The Phase 1 review (six lenses, each finding re-checked by a skeptic) raised 54 findings: 51 confirmed, 2 uncertain and 1 refuted, all fixed with regression tests. The most severe: renames and moves bypassed the version-bump gate, now compared with `git diff --no-renames`.
  - Depends: M1.4
  - Do:
    - Add `cargo xtask validate`, with `--base <ref>` to require a new version and a CHANGELOG entry for each changed asset ([`validate`](subsystems/xtask.md#validate), [asset versions](05-release.md#asset-versions)).
  - Accept:
    - A changed asset without a version bump fails `validate --base`.
  - Verify: `cargo xtask validate` (expect exit 0); `cargo xtask validate --base origin/main` (expect exit 0)

- [x] **M1.6 — ci: the CI and Security gates**
  - Note: Records work done before this roadmap, in Phase 1. The maintainer approved the local tools on 2026-11-16: Rust 1.96, cargo-deny, cargo-shear, zizmor, actionlint, gitleaks and the `gh-actions-lock` extension. Pinned actions and tools are releases at least 7 days old (`CI-PINS` in [`specs.md`](specs.md)), and CI uses no build cache.
  - Note: GitHub workflow lockfiles became generally available on 2026-10-28, so actions are pinned by SHA *and* recorded in `.github/workflows/actions.lock`, which covers the actions they call (Phase 0 finding 5). `pull_request_target`, blocked by default in public repositories from 2026-12-02, is never used (finding 6).
  - Note: macOS and Windows can't run locally, so CI covers them after a push (conflict C9). On 2026-11-18 three pushes had failed there: a macOS test that wrote after a 413 answer, Windows uninstall leaving empty directories (cap-std opens Windows directories without `FILE_SHARE_DELETE`), and a winget schema warning. `dca6c1d` fixed all three, and CI now runs `cargo test --no-fail-fast`, because Cargo had stopped at the first failing test binary. CI and Security first passed on `dca6c1d` (runs 44324738068 and 44324738206).
  - Note: The maintainer's choice on 2026-11-17 (Phase 4): `deny.toml` ignores RUSTSEC-2026-0543 ("chrono is unmaintained") with its reason, because rmcp depends on chrono directly ([exceptions](02-dependency-policy.md#exceptions)). CodeQL's advanced setup was added in `4dd223d` (2026-11-18), its actions pinned in `6eaa59a`, and its `security-extended` queries turned on in `2f0eb6e` (2026-11-24).
  - Depends: M1.3, M1.5
  - Do:
    - Write `ci.yml`, `security.yml` and `codeql.yml` with `permissions: {}` at the top and per-job grants ([continuous integration](04-quality.md#continuous-integration), [`02-dependency-policy.md`](02-dependency-policy.md)).
  - Accept:
    - Every gate runs on each pull request and push to `main`, and the tests run on Linux, macOS and Windows.
  - Verify: `cargo deny check` (expect exit 0); `cargo shear` (expect exit 0); `gh run list --repo jabbott-iii/Grimoire --workflow ci.yml --branch main --limit 1` and the same for `security.yml` and `codeql.yml` (expect success; on 2026-11-24, runs 45513260683, 45513259615 and 45513259596 on `baf29f0`)

**Exit criteria (M1):** the maintainer approved Phase 1 on 2026-11-16.

---

## M2 — Install and remove assets for Claude Code and AGENTS.md (Phase 2)

**Goal:** A user selects assets in `grimoire.toml`, and `grimoire` installs them for Claude Code and `AGENTS.md`, tracks them in `grimoire.lock`, reports drift, and removes exactly what it wrote.

- [x] **M2.1 — testkit: fixture catalogs and projects**
  - Note: Records work done before this roadmap, in Phase 2 (2026-11-16).
  - Depends: M1.4
  - Do:
    - Add `agent-grimoire-testkit`, never published, with fixture catalogs and projects ([test tools](04-quality.md#test-tools)).
  - Accept:
    - The resolver, adapter and CLI tests build their catalogs from it.
  - Verify: `cargo test -p agent-grimoire-testkit --locked --no-fail-fast` (expect every test to pass)

- [x] **M2.2 — resolve: selection, versions, conflicts, clients, and consent**
  - Note: Records work done before this roadmap, in Phase 2. The maintainer confirmed on 2026-11-16 that each asset's permissions are accepted in `grimoire.toml`, and that an update that needs more stops and asks again (Q8). Phase 2 added the `[allow]` table for assets brought in by bundles, tags or `requires`. `plan` and `apply` stop whenever a locked version differs from the catalog's. Each environment variable became its own grant, `env:<NAME>`, in Phase 3.
  - Note: The Phase 2 review fixed two consent bugs: `add --allow` granted the offered permissions to every selected asset that lacked them, and `remove` of an asset that something else still selected dropped its grant.
  - Depends: M1.4, M2.1
  - Do:
    - Select assets by name, bundle, tag and `exclude`; choose versions with the lock's pins; and check conflicts, client support, `min-cli-version` and consent ([resolving](subsystems/resolve.md#resolving), [permission consent](subsystems/resolve.md#permission-consent), [resolution errors](subsystems/resolve.md#resolution-errors)).
  - Accept:
    - Every failure is reported, with its code, in one run.
  - Verify: `cargo test -p agent-grimoire-resolve --locked --no-fail-fast` (expect every test to pass)

- [x] **M2.3 — install: the confined filesystem, managed blocks, and JSON entries**
  - Note: Records work done before this roadmap, in Phase 2. The filesystem layer is cap-std, with scoped `deny.toml` entries (the maintainer's choice when asked, [ADR-0006](adr/0006-filesystem-confinement.md)). JSON is strict only: jsonc-parser's syntax tree accepts comments even in strict mode, so files are validated with serde_json first (`EDIT-CRATES`).
  - Note: The Phase 2 review's hardening, each with a regression test: non-blocking, re-checked reads (a FIFO swapped in); permissions applied before content; markers only at column 0; byte-order-mark support; unclosed fences refused in rules; and lock paths limited to the locations Grimoire writes, after a tampered lock could make `apply` delete `.git/HEAD`.
  - Depends: M1.2
  - Do:
    - Write the root-confined filesystem, atomic writes, managed blocks and JSON entries ([filesystem safety](subsystems/install.md#filesystem-safety), [ownership units and merge strategies](subsystems/install.md#ownership-units-and-merge-strategies)).
  - Accept:
    - Paths that escape the root, symbolic links, special files and oversized files are refused, and a crash never leaves a torn file.
  - Verify: `cargo test -p agent-grimoire-install --locked --no-fail-fast` (expect every test to pass)

- [x] **M2.4 — install: the planner, apply, and verify**
  - Note: Records work done before this roadmap, in Phase 2. The Phase 2 review's most important fix: a rule whose text didn't end in exactly one newline was hashed before normalization but written after it, so the second `apply` reported a permanent conflict. Blocks are now hashed exactly as written. Also fixed: an edited hook element was forgotten on removal, `--write-new` replaced an existing `.new`, a case-only rename deleted the file on macOS and Windows, and CRLF checkouts of `grimoire.lock` made `plan --check` fail forever.
  - Note: Documented rather than changed: a JSON container written on one line is reflowed when Grimoire inserts into it, and empty directories are pruned after Grimoire removes a file it owns.
  - Depends: M2.3
  - Do:
    - Write the pure planner, `apply` and `verify` over the three-way comparison of disk, lock and desired state ([desired state, plan, apply](subsystems/install.md#desired-state-plan-apply), [drift and conflicts](subsystems/install.md#drift-and-conflicts)).
  - Accept:
    - A second `apply` changes nothing, edited content is a conflict, and `--force` and `--write-new` behave as documented.
  - Verify: `cargo test -p agent-grimoire-install --locked --no-fail-fast` (expect every test to pass); `cargo test -p agent-grimoire --locked --no-fail-fast` (expect every test to pass, including `second_apply_changes_nothing` in `tests/cli.rs`)

- [x] **M2.5 — catalog: the embedded pack and `Source`**
  - Note: Records work done before this roadmap, in Phase 2. `Source` is a type, not the trait ADR-0001 proposed, because a trait with one implementation added nothing.
  - Note: 2026-11-22 (`24200d4`): catalog validation now refuses catalog paths that the embedded pack can't load.
  - Depends: M1.4
  - Do:
    - Pack a validated catalog into `crates/agent-grimoire-catalog/catalog.pack` with `cargo xtask pack-catalog`, and load it into `Source` at startup ([the embedded pack and `Source`](subsystems/catalog.md#the-embedded-pack-and-source), [`catalog.pack`](subsystems/xtask.md#catalogpack)).
  - Accept:
    - The same catalog always gives the same bytes, and `pack-catalog --check` fails on a stale pack.
  - Verify: `cargo xtask pack-catalog --check` (expect exit 0)

- [x] **M2.6 — adapters: `claude-code`, `agents-md`, and the cross-client planner**
  - Note: Records work done before this roadmap, in Phase 2. Phase 0 found that clients read each other's files (finding 2), so adapters can't be independent: the planner works over the selected client set to reach each client once and warns about cross-client leaks. Claude Code has native path-scoped rules in `.claude/rules/` (finding 3), so rules don't need to be blocks in `CLAUDE.md`. When both clients are selected, path-scoped rules are shared through `AGENTS.md` instead of also writing `.claude/rules/`, so nothing loads twice.
  - Note: The maintainer confirmed on 2026-11-16 that all six clients are v1: `claude-code` and `agents-md` in Phase 2, then `codex`, `cursor`, `gemini-cli` and `copilot` in Phase 4 (Q3, [ADR-0005](adr/0005-client-priority.md)). MCP server ids `workspace` and `computer-use` are refused, because Claude Code reserves them.
  - Depends: M2.2, M2.4
  - Do:
    - Write the two adapters and the cross-client planner ([adapter contract](subsystems/adapters.md#adapter-contract), [client read matrix and cross-client planning](subsystems/adapters.md#client-read-matrix-and-cross-client-planning), [per-client mapping](subsystems/adapters.md#per-client-mapping)).
  - Accept:
    - Each client and client combination has an insta snapshot, read before it was accepted.
  - Verify: `cargo test -p agent-grimoire-adapters --locked --no-fail-fast` (expect every test to pass)

- [x] **M2.7 — cli: the eleven commands, with end-to-end tests**
  - Note: Records work done before this roadmap, in Phase 2. The maintainer approved Phase 2 on 2026-11-16. Its review raised 51 findings (44 confirmed, 1 uncertain, 6 refuted), and all 37 distinct ones were fixed.
  - Note: 2026-11-18, after Phase 7: six bugs the Phase 7 review found in code were fixed, each with an end-to-end test that failed on the old code: `uninstall --json` dropped the files it removed, `doctor` printed a broken lock's diagnostics twice, `list` didn't pad its columns, `plan --check` missed a lock that `apply` would create, and `init` and `doctor` followed a symbolic link in a parent directory. `init/client-unavailable` and `resolve/client-unavailable` can't be reached in this build and are kept on purpose ([as fixed after Phase 7](subsystems/cli.md#as-fixed-after-phase-7)).
  - Depends: M2.5, M2.6
  - Do:
    - Write `init`, `list`, `show`, `add`, `remove`, `plan`, `apply`, `update`, `verify`, `uninstall` and `doctor`, each with `--json`, `--no-input` and documented exit codes ([commands](subsystems/cli.md#commands), [machine use](subsystems/cli.md#machine-use), [exit codes](subsystems/cli.md#exit-codes)).
    - Test the installer's guarantees end to end in `crates/agent-grimoire/tests/cli.rs`: idempotency, drift, removal, uninstall leaving nothing, path traversal, absolute paths, symbolic-link escapes and Windows reserved names.
  - Accept:
    - Every command works on a fixture project, and its exit codes match [`cli.md`](cli.md#exit-codes).
  - Verify: `cargo test -p agent-grimoire --locked --no-fail-fast` (expect every test to pass); the `test` jobs of the latest CI run on `main` (expect success on `ubuntu-24.04`, `macos-15` and `windows-2025`; on 2026-11-24, run 45513260683)

**Exit criteria (M2):** the maintainer approved Phase 2 on 2026-11-16.

---

## M3 — One working asset of each kind (Phase 3)

**Goal:** The catalog holds one tested exemplar of every asset kind, each installed from the embedded catalog end to end, and a contributor can scaffold a new one.

- [x] **M3.1 — mcp: the shared server hardening**
  - Note: Records work done before this roadmap, in Phase 3 (2026-11-17). Phase 0 confirmed that rmcp 3.10.0 implements MCP 2026-07-28 with no gap, but found two that Grimoire must close itself: Origin validation is off by default, and server-side authorization exists only in examples (finding 1). The next MCP version is expected around 2027-03-30.
  - Note: The maintainer's choices on 2026-11-17: servers accept RFC 9068 JWT access tokens verified locally against an operator-supplied JWKS file, with no outbound network ([ADR-0007](adr/0007-mcp-server-authorization.md)), which needed the `subtle` license exception; and `deny.toml` skips exactly `syn@2.0.119` and allows rmcp 3.10.0's six test files by SHA-256 instead of trusting its build script. MCP servers and tools are separate binaries, released through GitHub Releases and the container image only, not package registries (Q9, [ADR-0002](adr/0002-mcp-server-and-tool-packaging.md)).
  - Note: The Phase 3 review: 2025-11-25 clients over HTTP, including Claude Code's stable release, were refused, so the server now checks the protocol version itself after authorization; a client could hold a connection slot forever, so keep-alive is off and every connection closes after 75 seconds; and three token checks were too loose. 2026-11-22 (`0218f00`): the bearer token is kept from MCP tool handlers.
  - Depends: M1.1
  - Do:
    - Write `crates/agent-grimoire-mcp`: stdio and Streamable HTTP, Origin and `Host` checks, limits, and the OAuth resource-server checks ([the MCP spec](subsystems/mcp.md)).
  - Accept:
    - Each limit and each refusal (missing or bad token, wrong Origin, oversized request) is tested.
  - Verify: `cargo test -p agent-grimoire-mcp --locked --no-fail-fast` (expect every test to pass)

- [x] **M3.2 — catalog: the rule, skill, and agent exemplars**
  - Note: Records work done before this roadmap, in Phase 3: `conventional-commits` (rule), `adr-writer` (skill, with two references and a template) and `security-reviewer` (agent, with `read` and `search` only), all `0.1.0` and `experimental`, confirmed as proposed on 2026-11-17. All three moved to `0.2.0` in Phase 4 to support the new clients.
  - Note: 2026-11-20 (`e0ba0da`): added the `plain-language-docs` rule.
  - Depends: M2.7
  - Do:
    - Write each asset's manifest, content, README, CHANGELOG and `evals/` ([the exemplars](subsystems/assets.md#the-exemplars), [`catalog/README.md`](../catalog/README.md)).
  - Accept:
    - Each one validates and installs from the embedded catalog.
  - Verify: `cargo xtask validate` (expect exit 0); `cargo test -p agent-grimoire --test builtin --locked` (expect every test to pass)

- [x] **M3.3 — catalog: the `secret-guard` tool and its hooks**
  - Note: Records work done before this roadmap, in Phase 3. The maintainer confirmed on 2026-11-16 that a tool is a standalone, non-interactive Rust executable for one local task, that installing it never installs the binary, and that hooks are bindings declared by tools, not a kind of their own (Q4). Hooks run without a trust prompt in some clients, so they come only from tool assets with an explicit `hooks` grant, in exec form, pointing at Grimoire's signed binaries (conflict C6).
  - Note: Code-bearing assets are workspace crates inside their asset directory, picked up by member globs, so adding one needs no core change (conflict C2). `secret-guard` has the subcommands `scan`, `hook` and `rules`, 16 vendor-format rules, and hooks that check every tool call; `on-failure = "block"` makes a failed hook block.
  - Depends: M2.7
  - Do:
    - Write the tool, its fail-closed Claude Code hooks and its tests ([`secret-guard`](subsystems/assets.md#secret-guard), [tool hooks](subsystems/assets.md#tool-hooks), [tool](subsystems/catalog.md#tool)).
  - Accept:
    - A hook event that holds a secret is blocked, and a failure of the hook blocks too.
  - Verify: `cargo test -p grimoire-tool-secret-guard --locked --no-fail-fast` (expect every test to pass)

- [x] **M3.4 — catalog: the `project-docs` MCP server**
  - Note: Records work done before this roadmap, in Phase 3, named `project-docs` on 2026-11-17. It serves tools only (`list_docs`, `read_doc`, `search_docs`, `list_adrs`), not resources, and takes its root from `--root` or `CLAUDE_PROJECT_DIR`, refusing to start without one.
  - Depends: M3.1, M2.7
  - Do:
    - Write the read-only, path-confined server on the shared hardening ([`project-docs`](subsystems/assets.md#project-docs), [the MCP spec](subsystems/mcp.md)).
  - Accept:
    - It serves over stdio and Streamable HTTP, and refuses reads outside its root and past its limits.
  - Verify: `cargo test -p grimoire-mcp-project-docs --locked --no-fail-fast` (expect every test to pass)

- [x] **M3.5 — catalog: the `security-baseline` and `rust-cli` bundles**
  - Note: Records work done before this roadmap, in Phase 3. Both moved to `0.2.0` in Phase 4.
  - Depends: M3.2, M3.3, M3.4
  - Do:
    - Write the two bundles ([`security-baseline` and `rust-cli`](subsystems/assets.md#security-baseline-and-rust-cli), [bundle](subsystems/catalog.md#bundle-not-an-asset-kind)).
  - Accept:
    - Each bundle installs, verifies and uninstalls from the embedded catalog.
  - Verify: `cargo test -p agent-grimoire --test builtin --locked` (expect every test to pass, including `every_exemplar_installs_verifies_and_uninstalls`)

- [x] **M3.6 — xtask: `cargo xtask new` and its templates**
  - Note: Records work done before this roadmap, in Phase 3. The maintainer approved Phase 3 on 2026-11-17 (329 tests then). Its review raised 43 findings, 41 confirmed and 2 uncertain, all fixed; among them, the server template now requires `--no-auth`, and `cargo xtask new` formats what it generates.
  - Depends: M3.3, M3.4
  - Do:
    - Write `cargo xtask new <kind> <id>` with templates in `xtask/templates/`, and a CI job that generates a tool and a server and builds, lints, tests and validates them ([`new`](subsystems/xtask.md#new)).
  - Accept:
    - A generated tool and server pass every check without edits.
  - Verify: the `scaffold templates build` job of the latest CI run on `main` (expect success; on 2026-11-24, run 45513260683)

**Exit criteria (M3):** the maintainer approved Phase 3 on 2026-11-17.

---

## M4 — All six clients and the Claude Code marketplace (Phase 4)

**Goal:** A project can select any combination of the six clients, and Claude Code users can install the eligible assets as plugins.

- [x] **M4.1 — install: TOML and JSONC entries**
  - Note: Records work done before this roadmap, in Phase 4 (2026-11-17). The Phase 4 review found that TOML editing could lose or rewrite the user's text: a CRLF or byte-order-mark `.codex/config.toml` was rewritten on every apply, and removing Grimoire's table deleted a comment above it. A 5,000-case property test now covers both. A comment inside Grimoire's own entry is a conflict (`commented`).
  - Depends: M2.4
  - Do:
    - Edit entries in `.codex/config.toml` (TOML) and `.gemini/settings.json` (JSONC), the format chosen by path ([three entry formats, chosen by path](subsystems/install.md#three-entry-formats-chosen-by-path)).
  - Accept:
    - A file Grimoire doesn't change keeps its bytes, and a changed one keeps its line endings and comments.
  - Verify: `cargo test -p agent-grimoire-install --test entry_formats --test toml_props --locked` (expect every test to pass)

- [x] **M4.2 — adapters: `codex`**
  - Note: Records work done before this roadmap, in Phase 4. Five research topics came first, each re-checked by a skeptic: 421 of 454 facts confirmed, 20 corrected and 13 left UNVERIFIED (`*-P4` in [`specs.md`](specs.md)). Codex gets no agents. `plan/codex-instructions-size` warns at 24 KiB, because Codex's 32 KiB budget is shared by every project instruction file it reads.
  - Note: The maintainer's choices on 2026-11-17: hooks stay Claude Code only, so the `codex`, `cursor`, `gemini-cli` and `copilot` adapters refuse hook outputs; and when a rule is in `AGENTS.md`, Cursor and Copilot get no rule file of their own, and Gemini CLI gets an `@AGENTS.md` import ([ADR-0008](adr/0008-phase-4-client-adapters.md)).
  - Depends: M2.6, M4.1
  - Do:
    - Write the adapter ([Phase 4 adapters](subsystems/adapters.md#phase-4-adapters)).
  - Accept:
    - Its output for each kind it supports has a snapshot, and it refuses what it can't render.
  - Verify: `cargo test -p agent-grimoire-adapters --locked --no-fail-fast` (expect every test to pass)

- [x] **M4.3 — adapters: `cursor`**
  - Note: Records work done before this roadmap, in Phase 4. Cursor gets only `read`-and-`search` agents. The docs had overstated what Cursor's `readonly` enforces; they now say what is documented.
  - Depends: M4.2
  - Do:
    - Write the adapter ([Phase 4 adapters](subsystems/adapters.md#phase-4-adapters)).
  - Accept:
    - Its output has snapshots, alone and with other clients.
  - Verify: `cargo test -p agent-grimoire-adapters --locked --no-fail-fast` (expect every test to pass)

- [x] **M4.4 — adapters: `gemini-cli`**
  - Note: Records work done before this roadmap, in Phase 4. `GEMINI.md` became a second file Grimoire may own a block in.
  - Depends: M4.2
  - Do:
    - Write the adapter ([Phase 4 adapters](subsystems/adapters.md#phase-4-adapters)).
  - Accept:
    - Its output has snapshots, alone and with other clients.
  - Verify: `cargo test -p agent-grimoire-adapters --locked --no-fail-fast` (expect every test to pass)

- [x] **M4.5 — adapters: `copilot`**
  - Note: Records work done before this roadmap, in Phase 4. `doctor` prints the settings JSON for each MCP server installed for Copilot's cloud agent, with `$COPILOT_MCP_<NAME>` secrets.
  - Depends: M4.2
  - Do:
    - Write the adapter ([Phase 4 adapters](subsystems/adapters.md#phase-4-adapters)).
  - Accept:
    - Its output has snapshots, alone and with other clients.
  - Verify: `cargo test -p agent-grimoire-adapters --locked --no-fail-fast` (expect every test to pass)

- [x] **M4.6 — adapters: the placement pass for shared rules and skills**
  - Note: Records work done before this roadmap, in Phase 4. `project-docs` and `secret-guard` stay Claude Code only: the server needs `CLAUDE_PROJECT_DIR`, which only Claude Code sets, and hooks are Claude Code only. `plan` now says when `secret-guard` doesn't guard a selected client (`resolve/client-not-covered`).
  - Note: The Phase 4 review found adapter-only refusals reaching users, so `cargo xtask validate` renders every asset for every client it declares, and a test installs the shipped catalog for all six clients.
  - Depends: M4.3, M4.4, M4.5
  - Do:
    - Write the placement pass that writes content several selected clients read only once ([placement](subsystems/adapters.md#placement), [ADR-0008](adr/0008-phase-4-client-adapters.md)).
  - Accept:
    - Every exemplar installs for every client, and shared content is written once.
  - Verify: `cargo test -p agent-grimoire --test builtin --locked` (expect every test to pass, including `every_exemplar_installs_for_every_client`)

- [x] **M4.7 — xtask: the generated marketplace and plugin validation**
  - Note: Records work done before this roadmap, in Phase 4. The maintainer's choices on 2026-11-17: one plugin per eligible asset, plus one per bundle that depends on its members' plugins; the marketplace is named `grimoire`, owned by `jabbott-iii`, with no email; tool assets with hooks are not plugins until fail-closed plugin hooks are tested; and CI installs an exact, verified Claude Code version to run `claude plugin validate --strict`, without a login or secrets.
  - Note: The maintainer approved Phase 4 on 2026-11-17. Its review raised 48 findings, 47 confirmed and 1 uncertain, all addressed; for the marketplace, plugin versions now change with plugin content.
  - Depends: M4.6
  - Do:
    - Generate `.claude-plugin/marketplace.json` and the plugins under `plugins/claude-code/` with `cargo xtask gen-marketplace [--check]` ([the marketplace spec](subsystems/marketplace.md), [the marketplace](subsystems/xtask.md#the-marketplace)).
  - Accept:
    - The checked-in marketplace is current, and Claude Code validates it strictly.
  - Verify: `cargo xtask gen-marketplace --check` (expect exit 0); the `Claude Code plugin validation` job of the latest CI run on `main` (expect success; on 2026-11-24, run 45513260683)

**Exit criteria (M4):** the maintainer approved Phase 4 on 2026-11-17.

---

## M5 — A release pipeline proven by dry-runs (Phase 5)

**Goal:** Every pull request builds, packages, signs offline and checks a release for every channel, and the release itself is one approval away once the maintainer turns it on.

- [x] **M5.1 — xtask: the release commands**
  - Note: Records work done before this roadmap, in Phase 5 (2026-11-17). The maintainer confirmed release-plz with a hand-written pipeline, and dist not adopted (Q10, [ADR-0003](adr/0003-release-tooling.md)): dist's npm installer downloads binaries at install time, and its npm and Homebrew publishing depend on long-lived tokens (Phase 0 finding 7). The six targets were confirmed as proposed (Q5, [targets](03-platforms.md#targets)).
  - Note: The maintainer's choices on 2026-11-17: one version for the CLI and its five libraries, with one `vX.Y.Z` tag; PyPI wheels written by `cargo xtask` from the archived binary, not by maturin; and each binary built on its own (`cargo auditable build -p <pkg>`), so its features and SBOM match a `cargo install` build. Eight research topics came first: 524 of 567 facts confirmed, 27 corrected, 16 left UNVERIFIED (`*-P5` in [`specs.md`](specs.md)).
  - Note: The maintainer approved these local tools for Phase 5, in the scratchpad only: ShellCheck, PowerShell with PSScriptAnalyzer, twine, release-plz, cosign, cargo-auditable, cargo-cyclonedx, and the Homebrew Docker image, which couldn't run because Docker wasn't reachable then.
  - Depends: M2.7
  - Do:
    - Write `cargo xtask release` (`plan`, `build`, `sbom`, `channels`, `checksums`, `verify`, `notes`), so the workflow is glue around unit-tested Rust ([the parts](subsystems/release.md#the-parts), [`release`](subsystems/xtask.md#release), [ADR-0009](adr/0009-release-pipeline.md)).
  - Accept:
    - Archives are flat and reproducible, hostile archives are refused, and `verify` regenerates everything and checks that `dist/` holds exactly the release.
  - Verify: `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass)

- [x] **M5.2 — release: the install scripts and the npm launcher**
  - Note: Records work done before this roadmap, in Phase 5, under a language-policy exception ([ADR-0010](adr/0010-language-policy-exceptions.md)). The dist spike failed: dist's PowerShell installer checks no checksum, and its shell installer checks one only when dist built the archives. `install.sh` requires curl, because wget can't refuse an HTTPS-to-HTTP redirect (forced by the review).
  - Depends: M5.1
  - Do:
    - Write `packaging/install/install.sh` and `install.ps1` templates, with each archive's SHA-256 embedded at release time, and the npm launcher `packaging/npm/grimoire.js` ([what a release holds](subsystems/release.md#what-a-release-holds), [language policy](02-dependency-policy.md#language-policy)).
  - Accept:
    - Each installer installs and runs the CLI on each operating system in the dry-run.
  - Verify: the `verify install` jobs of the latest Release run on `main` (expect success on all four dry-run targets; on 2026-11-24, run 45513260716)

- [x] **M5.3 — release: `release.yml`, `sign.yml`, and the dry-run**
  - Note: Records work done before this roadmap, in Phase 5. The maintainer's choices after the Phase 5 review (2026-11-17): publishing runs in four jobs with four environments (`release`, `crates-io`, `npm`, `pypi`), so one registry's job can't publish to another; and the first release approves `release` before the six crates are published by hand from its tag, and only then the registry jobs, so crates.io never holds a version without its tag. That review raised 64 findings, 59 confirmed (2 medium) and 5 refuted, all addressed.
  - Note: 2026-11-18 (the maintainer's choice when asked): the dry-run installs winget-cli v1.29.380, pinned by SHA-256, because the runner's winget predated schema 1.28.0 and accepting its warnings would weaken the check. The first passing Release dry-run was run 44353090397, on 2026-11-18.
  - Depends: M5.1, M5.2
  - Do:
    - Write the workflow: plan, native builds, packaging, install tests on each operating system, attestations and keyless signatures through `sign.yml`, and the four approved publish jobs ([how a release flows](subsystems/release.md#how-a-release-flows), [the trust chain](subsystems/release.md#the-trust-chain), [the signing identity](subsystems/release.md#the-signing-identity)).
  - Accept:
    - Every pull request gets a dry-run summarized by `release checks`, and the release-only jobs are skipped outside release mode.
  - Verify: `gh run list --repo jabbott-iii/Grimoire --workflow release.yml --branch main --limit 1` (expect success, with `release checks` passing and the seven release-only jobs skipped; on 2026-11-24, run 45513260716)

- [x] **M5.4 — release: the Homebrew tap and Scoop bucket templates**
  - Note: Records work done before this roadmap, in Phase 5. Homebrew and Scoop pull the attested formula and manifest from each release through template workflows in the tap and bucket repositories, each with its own `GITHUB_TOKEN`, rather than autobump and Excavator, which can't update a multi-architecture prebuilt formula safely (conflict C5; the maintainer's choice on 2026-11-17). winget's identifier is `jabbott-iii.agent-grimoire` with the command alias `agent-grimoire`, because `grimoire` is taken there.
  - Depends: M5.3
  - Do:
    - Write `packaging/homebrew-tap/` and `packaging/scoop-bucket/`, and generate the formula, the Scoop manifest and the three winget manifests in `channels` ([what each channel checks on the user's machine](subsystems/release.md#what-each-channel-checks-on-the-users-machine)).
  - Accept:
    - The dry-run generates and checks every channel's files, including `winget validate`.
  - Verify: the `package channels and checksums` and `verify packages and manifests` jobs of the latest Release run on `main` (expect success; on 2026-11-24, run 45513260716)

- [ ] **M5.5 — release: release-plz**
  - Note: Records work done before this roadmap, in Phase 5. release-plz opens the release pull request only; the release workflow tags the commit it built and signed, and publishes (the maintainer's choice on 2026-11-17). Pull requests created with `GITHUB_TOKEN` don't start workflows, so the maintainer approves the release pull request's `pull_request` runs instead of storing a token (conflict C4). The maintainer approved Phase 5 on 2026-11-17.
  - Note: Left unticked on 2026-11-24: release-plz has never opened a release pull request. The ruleset refuses its commit on every run, most recently run 45513260333 on `baf29f0`, and each refused run leaves an orphan `release-plz-*` branch.
  - ⏳ needs GitHub: after M8.3, a `release-plz.yml` run on `main` opens or updates the release pull request.
  - Depends: M5.3
  - Do:
    - Configure `release-plz.toml` and `.github/workflows/release-plz.yml` to open the release pull request on pushes to `main` ([how a release flows](subsystems/release.md#how-a-release-flows), [Grimoire's version](05-release.md#grimoires-version)).
  - Accept:
    - A push to `main` opens or updates the release pull request, with the next version and its `CHANGELOG.md` section.
  - Verify: `gh run list --repo jabbott-iii/Grimoire --workflow release-plz.yml --branch main --limit 1` (expect success); `gh pr list --repo jabbott-iii/Grimoire --search "head:release-plz"` (expect the release pull request)

**Exit criteria (M5):** the maintainer approved Phase 5 on 2026-11-17. The release pull request's first appearance is checked in M5.5.

---

## M6 — A signed, scanned container image (Phase 6)

**Goal:** Every pull request builds, checks, scans and runs a two-platform image of the release binaries, and a release would publish it signed.

- [x] **M6.1 — release: the `Dockerfile` and the Grype gate**
  - Note: Records work done before this roadmap, in Phase 6 (2026-11-17). Trivy and `trivy-action` were compromised in March 2026 (CVE-2026-33634), so the gate uses Grype, from its checksum-pinned release tarball, because `anchore/scan-action` pipes an install script to `sh` (Phase 0 finding 4, `SCAN-P6`). Five research topics came first: 333 facts, 308 confirmed and 25 corrected (`*-P6` in [`specs.md`](specs.md)).
  - Note: The maintainer's choices on 2026-11-17: `runtime = "docker"` is deferred past v1, with hand-written, digest-pinned `docker run` entries in [`container.md`](container.md) instead; and the Grype gate fails on any medium, high or critical match, fixed or not, so a finding without a fix needs an ignore rule with a reason in `.grype.yaml`. The maintainer also approved rootless Docker, Grype, cosign, an OCI copy tool and the `aarch64-unknown-linux-musl` target for local work.
  - Note: Local evidence from Phase 6, with rootless Docker 29.9.3: Grype 0.121.3 (database v6.1.10, built 2026-11-17T06:20:58Z) found nothing on either platform. A medium GHSA match exited 2, an ignore rule keyed on the GHSA ID and the package name suppressed it, and unknown config keys were silently ignored (so a test now refuses them). Distroless's index and both platform manifests verified with cosign 3.1.4.
  - Note: Dependabot's pull request 1 moved the distroless base from `25ac8f0` to `aafe993`, dated 2026-11-12 and so at least 7 days old from 2026-11-19T21:16:35Z; it was merged on 2026-11-20 (`2f80306`). The base changes only by digest, to one at least 7 days old, checked by hand ([maintaining the image pins](05-release.md#maintaining-the-image-pins)).
  - Depends: M5.1
  - Do:
    - Write the `Dockerfile` (COPY only, on distroless `static-debian13:nonroot` pinned by digest) and `.grype.yaml` ([what the image holds](subsystems/container-image.md#what-the-image-holds), [threats and mitigations](subsystems/container-image.md#threats-and-mitigations)).
  - Accept:
    - The image holds exactly the release binaries on the verified base, and the gate fails on a medium or worse match.
  - Verify: the `container image (build, check, scan)` job of the latest Release run on `main` (expect success; on 2026-11-24, run 45513260716)

- [x] **M6.2 — xtask: `release image`**
  - Note: Records work done before this roadmap, in Phase 6. The OCI layout is built once and pushed byte for byte with crane, rather than rebuilt in the push job, so the registry holds exactly the bytes that were scanned. The Phase 6 review's one high finding: `check` read layers more loosely than Docker, so a second gzip member could install another binary unseen. `check` now reads layers strictly and refused all seven of the review's tampered layouts.
  - Note: Local evidence from Phase 6: three builds gave identical platform manifest digests (the index digest differs, because the SBOMs and provenance hold build times); with the Actions environment simulated, `check` refused a provenance that held the event payload; and `coverage` counted 92 Rust packages in `grimoire` (91 on arm64), 135 in `grimoire-mcp-project-docs` and 41 in `grimoire-tool-secret-guard`, with Debian 13.7 and 6 Debian packages.
  - Depends: M6.1
  - Do:
    - Write `cargo xtask release image` (`base`, `context`, `check`, `coverage`) ([the parts](subsystems/container-image.md#the-parts)).
  - Accept:
    - `check` refuses any layout that differs from the plan, and `coverage` fails unless Grype saw Debian, its packages and every binary's Rust packages.
  - Verify: `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass)

- [x] **M6.3 — release: the image jobs, `sign-image.yml`, and `image-tags.sh`**
  - Note: Records work done before this roadmap, in Phase 6. The maintainer's choices on 2026-11-17: a release pushes the tags `X.Y.Z` and `latest`; pull-request dry-runs also build `aarch64-unknown-linux-musl`, so every pull request builds, scans and runs the two-platform image; after the review, every cosign signature and attestation moved into a reusable workflow that only the signing jobs call; and `.github/scripts/image-tags.sh` became a seventh language-policy exception, recorded in [ADR-0011](adr/0011-container-image.md), so the release and every dry-run run the same tag code. The maintainer approved Phase 6 on 2026-11-17 (454 tests then).
  - Note: 2026-11-18 (`f69c0d0`, the maintainer's choice): GitHub refused to start `release.yml`, because a called workflow can narrow the caller's permissions but never widen them, and `sign.yml`'s image job asked for more than `sign-files` granted. The image's signing moved into its own `sign-image.yml`, whose certificates name it. The image was first built on GitHub in Release run 44353090397, on 2026-11-18.
  - Note: Not run yet: `publish-image`, `sign-image`, keyless image signing and `actions/attest` for the image. `image-tags.sh` has run only against a local registry (every case the dry-run covers, on 2026-11-17), never against GHCR. Their first run is in M8.7.
  - Depends: M6.2, M5.3
  - Do:
    - Add the `image`, `verify-image`, `publish-image` and `sign-image` jobs and the tag script ([the trust chain](subsystems/container-image.md#the-trust-chain), [the tag steps](subsystems/container-image.md#the-tag-steps)).
  - Accept:
    - Each dry-run verifies the image on both platforms, with offline signing and the tag script's cases.
  - Verify: the `verify image (linux/amd64)`, `verify image (linux/arm64)` and `sign (dry-run, offline)` jobs of the latest Release run on `main` (expect success; on 2026-11-24, run 45513260716)

- [ ] **M6.4 — ci: the weekly image scan and ShellCheck**
  - Note: Records work done before this roadmap, in Phase 6. ShellCheck checks `image-tags.sh` in `security.yml`, and Dependabot watches the `Dockerfile`.
  - Note: Left unticked on 2026-11-24: no release exists, so `security.yml`'s `released image scan (Grype)` job has nothing to scan; it was skipped in run 45513259615.
  - ⏳ needs GitHub: after M8.7, the weekly scan pulls, verifies and scans the released image.
  - Depends: M6.3
  - Do:
    - Add the weekly Grype rescan of the latest release's image, and ShellCheck for the workflow scripts ([residual risks](subsystems/container-image.md#residual-risks), [continuous integration](04-quality.md#continuous-integration)).
  - Accept:
    - The weekly run scans the image the latest release tag names, after checking its signature.
  - Verify: the `workflow audit (zizmor, actionlint)` job of the latest Security run on `main` (expect success); the `released image scan (Grype)` job of a Security run after M8.7 (expect success)

**Exit criteria (M6):** the maintainer approved Phase 6 on 2026-11-17. The released image's first scan is checked in M6.4.

---

## M7 — User docs, policies, and contributor tooling (Phase 7)

**Goal:** A user can install and use Grimoire from its documentation alone, a contributor knows how to propose a change, and tests keep the user reference true to the code.

- [x] **M7.1 — docs: the README and the user reference**
  - Note: Records work done before this roadmap, in Phase 7 (2026-11-17 and 2026-11-18): the README; [`cli.md`](cli.md), hand-written; [`diagnostics.md`](diagnostics.md), every user-facing code (catalog-author codes such as `manifest/…` stay out); [`configuration.md`](configuration.md); [`container.md`](container.md); and the versioning policy, now [Versioning](05-release.md#versioning). 2026-11-20 (`9eb612f`): the documentation was rewritten in plain language.
  - Note: The maintainer's Phase 7 choices on 2026-11-17: the deprecation window is one release (a deprecated asset ships with its replacement in at least one CLI release before removal; a deprecated flag or command warns for at least one minor release; a file format changes only with a new `format` number, and the CLI reads the previous one for at least one more minor release); a change that breaks any library crate's public Rust API is marked breaking, because the six crates depend on each other with `^0.1.0`; and an MSRV bump is marked breaking too, so in 0.x it lands in the next 0.y. The maintainer approved Phase 7 on 2026-11-18 and committed it as `dca6c1d`.
  - Depends: M2.7
  - Do:
    - Write the README and the user reference ([`cli.md`](cli.md), [`diagnostics.md`](diagnostics.md), [`configuration.md`](configuration.md), [`container.md`](container.md)).
  - Accept:
    - Every command, flag, exit code and user-facing diagnostic code is documented.
  - Verify: `cargo test -p agent-grimoire --locked --no-fail-fast` (expect every test to pass, including the `docs_check` tests and `tests/diagnostics_doc.rs`)

- [x] **M7.2 — docs: the checks that keep the docs true**
  - Note: Records work done before this roadmap, in Phase 7. Each check was shown to fail on a broken copy of its file. The README guard parses Markdown with pulldown-cmark, as an xtask dev-dependency, instead of a hand-written scanner.
  - Note: Deviation approved by the maintainer on 2026-11-18, forced by the review: the release pull request doesn't delete the `before-first-release` markers; a pull request merged to `main` ahead of it does, because whether release-plz keeps a commit pushed to its branch is UNVERIFIED. Markers are paired, and deleting only the opening line still fails, because a single marker line let the note it marked ship.
  - Depends: M7.1
  - Do:
    - Write the checks in [checked documents](04-quality.md#checked-documents), and the marker guard in `cargo xtask release plan`.
  - Accept:
    - Each check fails on a document that disagrees with the code, and a release refuses to start while a marker remains.
  - Verify: `cargo test -p agent-grimoire --locked --no-fail-fast` (expect every test to pass); `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass); the `documentation links` job of the latest CI run on `main` (expect success; on 2026-11-24, run 45513260683)

- [x] **M7.3 — docs: the community files, issue forms, and CODEOWNERS**
  - Note: Records work done before this roadmap, in Phase 7. The maintainer's choices on 2026-11-17: `CODE_OF_CONDUCT.md` adopts the Contributor Covenant 3.0 by reference, with the conduct contact a marked placeholder until M8.6; there is no DCO sign-off, because submitting a contribution licenses it as `MIT OR Apache-2.0`; and `SECURITY.md` promises an acknowledgment within 7 days, updates in the advisory, and publication when the fix is released. The five issue forms and the chooser config were checked once with check-jsonschema 0.39.0. Five research topics came first: 264 facts, 242 confirmed, 19 corrected, 3 UNVERIFIED (`*-P7` in [`specs.md`](specs.md)).
  - Note: 2026-11-24: `SECURITY.md` asks reporters to test only on their own systems (`2ffe065`); CODEOWNERS lists the agent instructions and release config (`0f7b0e1`).
  - Depends: M7.1
  - Do:
    - Write `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, the pull request template, `.github/CODEOWNERS` and the issue forms ([the repository and contributions](04-quality.md#the-repository-and-contributions)).
  - Accept:
    - GitHub reports no CODEOWNERS errors, and every reporter and contributor has a form that fits.
  - Verify: `gh api repos/jabbott-iii/Grimoire/codeowners/errors` (expect an empty `errors` list)

- [x] **M7.4 — docs: crate docs and READMEs in one shape**
  - Note: 2026-11-22 (`be290ea`, `0265cae`): every crate's crate docs and every library README follow [ADR-0012](adr/0012-crate-documentation.md) (crate documentation), the maintainer's request that day, and module and item docs link to the design they follow.
  - Depends: M7.2
  - Do:
    - Write each crate's crate docs and each published library's README in the ADR-0012 shape, with the check in `xtask/src/crate_docs.rs` ([checked documents](04-quality.md#checked-documents)).
  - Accept:
    - The shape check passes, the README examples run as doc tests, and the API docs build with warnings denied.
  - Verify: `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass); `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps --locked` (expect exit 0)

- [x] **M7.5 — xtask: the workflow-permission and catalog-pack checks**
  - Note: 2026-11-18, after Phase 7: `xtask/src/workflow_permissions.rs` checks every reusable-workflow call's grant against every job it calls; with the old `sign.yml` back in place it reports the same two permissions GitHub did. The maintainer regenerated `actions.lock` with gh-actions-lock v0.4.1 in `f69c0d0`.
  - Note: 2026-11-18 (`20dcfeb`): in release mode, `cargo xtask release plan` packs `catalog/` and refuses the run unless the checked-in pack matches (`check_pack_is_current`), because `release.yml` runs no tests and the binary embeds the checked-in pack. `no_embedded_asset_needs_a_newer_grimoire` in `builtin.rs` names the `min-cli-version` check that CI had enforced only as a side effect.
  - Depends: M5.3, M6.3
  - Do:
    - Add both checks ([reusable-workflow permissions](subsystems/xtask.md#reusable-workflow-permissions-workflow_permissionsrs), [the release switch](subsystems/release.md#the-release-switch)).
  - Accept:
    - A call that grants less than a called job asks for fails the test, and a release with a stale pack is refused.
  - Verify: `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass, including `a_release_refuses_a_stale_pack`)

**Exit criteria (M7):** the maintainer approved Phase 7 on 2026-11-18. Still to check as someone other than the owner, and on each registry: M8.8.

---

## M8 — The first public release

**Goal:** Grimoire's first version is published on every channel, signed and verifiable, and a user can install it with the README's commands.

- [ ] **M8.1 — docs: the roadmap-and-docs document set**
  - Note: Written on 2026-11-24 at the maintainer's request, with the blueprint's ten defaults approved that day ([ADR-0013](adr/0013-roadmap-and-docs-document-set.md)). The URLs of these documents should settle before the first release, because from then on crates.io, docs.rs and PyPI pages link to them. Waits for the maintainer's review and commit.
  - Depends: M7.4
  - Do:
    - Write the core documents `00` to `05`, this roadmap and the specs in [`subsystems/`](subsystems/); move every link to the replaced files; and delete `PLAN.md`, `architecture.md`, `security-model.md`, `versioning.md` and `release.md`.
  - Accept:
    - No file in the repository links to a deleted document, and every check in "Everyday commands" in `AGENTS.md` passes.
  - Verify: `cargo test --workspace --locked --no-fail-fast` (expect every test to pass); `cargo xtask schema --check` (expect exit 0); `git grep -nE 'PLAN\.md|docs/(architecture|security-model|versioning|release)\.md' -- . ':!docs/adr/0013-roadmap-and-docs-document-set.md' ':!docs/roadmap.md'` (expect no match: those two files name the replaced documents on purpose); the `documentation links` job in CI after the push (expect success)

- [ ] **M8.2 — xtask: the roadmap and decision-record check**
  - Depends: M8.1
  - Do:
    - Add a test in `xtask/src/` that reads this file and `docs/adr/`, as the check in `.claude/skills/build-roadmap-and-docs/SKILL.md` step 6 does, adapted to one file per record. It is Rust, under the language policy ([ADR-0010](adr/0010-language-policy-exceptions.md)).
  - Accept:
    - It fails on a task ID used twice, a `Depends` on a later task, a ticked task with an unticked dependency, a backlog row without a trigger, a record out of order or without a status, and a document that cites a missing task or record. Each case is shown on a broken copy.
  - Verify: `cargo test -p xtask --locked --no-fail-fast` (expect every test to pass)

- [ ] **M8.3 — repo: let release-plz open the release pull request**
  - Note: Read on 2026-11-24: the `BestBranch` ruleset covers every branch, so its pull-request rule refuses release-plz's commit (run 45513260333). Its exclusion reads `refs/heads/refs/heads/release-plz-*`, with `refs/heads/` twice, and matches no branch. Two orphan `release-plz-*` branches were left on 2026-11-24, and each push to `main` adds one while the ruleset refuses the commit. Don't make GitHub Actions a bypass actor: any workflow token with `contents: write` could then change `main`.
  - ⏳ needs GitHub: a maintainer change to the repository's settings.
  - Depends: M5.3
  - Decide first: target the ruleset at the default branch only, or correct the exclusion to `refs/heads/release-plz-*`.
  - Do:
    - Change the ruleset, turn on "Allow GitHub Actions to create and approve pull requests", and delete the orphan `release-plz-*` branches, which needs the admin bypass ([repository settings](05-release.md#repository-settings)).
  - Accept:
    - The next push to `main` opens or updates the release pull request, and M5.5 can be ticked.
  - Verify: `gh run list --repo jabbott-iii/Grimoire --workflow release-plz.yml --branch main --limit 1` (expect success)

- [ ] **M8.4 — repo: choose the checks the ruleset requires**
  - Note: Read on 2026-11-17 and 2026-11-18, the ruleset required CodeQL and code-quality results; read on 2026-11-24, both rules are gone, so neither blocks a merge, and no ruleset requires the CI, Security or `release checks` results either ([the repository and contributions](04-quality.md#the-repository-and-contributions)).
  - ⏳ needs GitHub: a maintainer change to the repository's settings.
  - Depends: M1.6, M5.3
  - Decide first: whether CodeQL becomes a required check on `main` again.
  - Do:
    - Require the CI, Security and `release checks` checks on `main`, and CodeQL if chosen ([repository settings](05-release.md#repository-settings)).
  - Accept:
    - A pull request with a failing required check can't be merged without the bypass.
  - Verify: `gh api repos/jabbott-iii/Grimoire/rules/branches/main` (expect a `required_status_checks` rule that lists the chosen checks)

- [ ] **M8.5 — release: the one-time bootstrap**
  - Note: No credential is ever stored in GitHub. A package's first publish needs one, so the bootstrap is run by the maintainer outside CI: on crates.io a short-expiry, crate-scoped token, revoked right after, then trusted publishing only; on npm an interactive `npm login` session (a 2-hour token); PyPI needs only a pending publisher, which expires after 30 days (conflict C1).
  - Note: Since `SECURITY.md` reached `main` on 2026-11-18, it promises a 7-day acknowledgment, so the maintainer watches the repository's security alerts and keeps private vulnerability reporting on (it read `enabled: true` on 2026-11-17).
  - ⏳ needs GitHub: maintainer changes to settings, environments and registries.
  - Depends: M8.3, M8.4
  - Do:
    - Work through [repository settings](05-release.md#repository-settings) and [registries and repositories](05-release.md#registries-and-repositories-close-to-the-first-release): the five environments (`release`, `crates-io`, `npm`, `pypi`, `ghcr`), immutable releases, the npm organization and packages, the crates.io first publish and trusted publishers, the PyPI pending publisher, the tap and bucket repositories, and the GHCR checks.
  - Accept:
    - Every item in those two parts of the checklist is ticked.
  - Verify: `gh api repos/jabbott-iii/Grimoire/environments --jq '.environments[].name'` (expect `crates-io`, `ghcr`, `npm`, `pypi` and `release`)

- [ ] **M8.6 — docs: clear the `before-first-release` markers**
  - Depends: M8.5
  - Do:
    - In a pull request merged to `main` ahead of the release pull request, delete each marked passage: the README's "not released yet" note, the conduct contact placeholder (filled in) and `SECURITY.md`'s "No release exists yet" ([switch releases on](05-release.md#switch-releases-on-last)). Update every status location in the same change.
  - Accept:
    - `cargo xtask release plan` no longer refuses release mode because of a marker.
  - Verify: `git grep -n before-first-release -- README.md CODE_OF_CONDUCT.md CONTRIBUTING.md SECURITY.md` (expect no match)

- [ ] **M8.7 — release: the first release**
  - Note: winget manifests are generated and validated in CI and submitted by the maintainer (conflict C5).
  - ⏳ needs GitHub: the release run and its approvals.
  - Depends: M5.5, M8.6
  - Do:
    - Set the repository variable `GRIMOIRE_RELEASES=enabled`, merge the release pull request, and approve each round as [during the first release run](05-release.md#during-the-first-release-run) says. Then make the GHCR package public, which can't be undone.
  - Accept:
    - The tag and an immutable GitHub Release exist; crates.io, npm and PyPI hold the version; the image is public, signed and tagged; and each download verifies as [verifying a download](05-release.md#verifying-a-download) shows. The status at the top of this file and in `05-release.md` says so, and M6.4 can be ticked.
  - Verify: `gh release view --repo jabbott-iii/Grimoire` (expect the new version, marked immutable); the commands in [verifying a download](05-release.md#verifying-a-download) (expect each to pass)

- [ ] **M8.8 — repo: the checks only a non-owner or a registry can show**
  - Depends: M8.7
  - Do:
    - Open `/issues/new/choose` as someone other than the owner, and read the README as GitHub, crates.io and PyPI render it.
  - Accept:
    - The chooser shows the five forms and the two links, with no duplicate security entry, and each README renders with working links.
  - Verify: observe each page, and record what was seen and the date in a note here

**Exit criteria (M8):** the maintainer confirms that the first release is published on every channel, that it verifies, and that every status location says so.

---

## Backlog (each needs a trigger or a decision)

| Item | Trigger / reason |
| :- | :- |
| `runtime = "docker"`: adapters write a server's client entry as a `docker run` command, instead of the hand-written forms in `container.md` | MCP clients share a project-directory variable, or rendering one form per client becomes worth it, after v1 ([ADR-0011](adr/0011-container-image.md)) |
| Hooks for Codex, Cursor, Gemini CLI and Copilot, which their adapters refuse today | A client documents that a failing hook blocks the action, and it can be tested here ([ADR-0008](adr/0008-phase-4-client-adapters.md)) |
| A network catalog `Source`, so catalog updates reach users between CLI releases | Users need catalog updates faster than the release cadence, or need historical versions; a new ADR then covers index signing, caching and `update` ([ADR-0001](adr/0001-catalog-delivery.md)) |
| Tests of symbolic-link detection on Windows, where today's link tests are Unix-only | A link-related bug is reported on Windows, or a change touches the Windows path of `agent-grimoire-install`'s `fs.rs` |
| Build `x86_64-apple-darwin` by cross-compiling from arm64 | GitHub announces the retirement date of the `macos-15-intel` runner image |
| Move to the next MCP specification version | MCP publishes a new specification version (expected around 2027-03-30) |
| Performance budgets for `plan` and `apply`, and the tests that enforce them | The catalog passes 50 assets, or a user reports a slow `plan` or `apply` |
| Coverage measurement and mutation testing (`testing.md`'s **Coverage command** and **Mutation command** are "none") | Decision: the maintainer chooses the tools |
| Fuzz targets for the parsers of outside data | Decision: the maintainer chooses a fuzzer, or a parser crashes on input in a report |
| The agent `model` hint and tool permission allow-list outputs, deferred in Phase 1 | An asset that needs one is proposed |
| A lock against two `grimoire` processes in one project | A user reports two concurrent runs damaging a project |
| Signing the image's platform manifests (`--recursive`), not only the index | A user's policy engine needs per-platform signatures |
| A CI check of the base image's age | A base digest younger than 7 days is merged |
| More clients, beyond the six | A user asks for a client in an issue, and its formats are documented at a primary source |
| A feature list (the rule's **Feature list** is "none") | Decision: the README's "What Grimoire does and does not do" no longer says what users get |
