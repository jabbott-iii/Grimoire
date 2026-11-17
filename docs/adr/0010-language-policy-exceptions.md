# ADR-0010: Language-policy exceptions

- **Status:** Accepted (the maintainer chose to write Grimoire's own installers, 2026-11-17, during Phase 5, revised the same day after the Phase 5 review)
- **Date:** 2026-11-17
- **Specs:** `INSTALLERS-P5`, `NPM-P5`, `BREW-SCOOP-P5`, `ACTIONS-P5` in [`../specs.md`](../specs.md)
- **Related:** [ADR-0003](0003-release-tooling.md), which named this ADR as the fallback if dist's installers didn't fit, and [ADR-0009](0009-release-pipeline.md), the pipeline that needs these files

## Context

Grimoire's code is Rust: the CLI, every asset binary, and `cargo xtask`. Rust gives us one toolchain, one lint policy (`unsafe_code = "forbid"`, clippy pedantic) and one test harness.

A few pieces can't be Rust:
- some run before any Grimoire binary is on the machine;
- some run inside a tool that only reads its own language;
- some run in another repository that has no Rust toolchain.

Rust's checks don't reach these pieces, so each one needs a reason, a limit on what it does, and checks of its own.

## Decision

**Rust is the default for all code.** The six exceptions below are the only ones. Any other code that isn't Rust needs a new ADR.

Data files that `cargo xtask` generates, such as the Scoop manifest (JSON), the winget manifests (YAML) and npm's `package.json`, are not code and aren't exceptions. The Homebrew formula is an exception because it is Ruby.

### 1. `packaging/install/install.sh`

- **Language:** POSIX `sh` (`#!/bin/sh`), no bashisms.
- **Why not Rust:** it runs on a machine that has only a shell and a downloader. A Rust installer would itself have to be downloaded and trusted first. dist's generated installer was rejected in ADR-0009.
- **Scope:** install one `grimoire` binary of one version.
  - Pick the target: Linux maps to static musl on any libc; macOS picks arm64 under Rosetta.
  - Download the archive with curl only, from an `https://` URL, with `--proto =https --tlsv1.2`. curl then also refuses a redirect to anything but HTTPS.
  - wget isn't used. GNU Wget's `--https-only` applies only to recursive downloads, so it can't refuse a redirect from HTTPS to HTTP, and BusyBox wget has no `--https-only` at all. Without curl the script stops with an error that says to install it (`apk add curl` on Alpine).
  - Check the archive's SHA-256 against the value embedded at release time, then extract it.
  - Replace the binary atomically: copy it next to the old one, check that it runs, then rename it over the old one.
  - Fail closed when no SHA-256 tool exists or the embedded value is empty.
  - No PATH or shell rc-file edits, no telemetry, receipts, tokens or updater.
- **Size:** one template, 263 lines after the Phase 5 review. `cargo xtask release channels` renders it, filling in only the version and the archive SHA-256s. Growth beyond the scope above needs a change to this ADR.
- **Checks:**
  - ShellCheck v0.11.0, pinned by SHA-256, with `--shell=sh`, on the template and on the rendered script.
  - Install tests on every built target against a local HTTP server, including a tampered archive the script must refuse.

### 2. `packaging/install/install.ps1`

- **Language:** PowerShell, working on both Windows PowerShell 5.1 and PowerShell 7.
- **Why not Rust:** the same reason, on Windows.
- **Scope:** the same as `install.sh`, for the two Windows targets. No `Invoke-Expression`.
  - It downloads with `Invoke-WebRequest -UseBasicParsing`, after adding TLS 1.2 on Windows PowerShell 5.1. PowerShell 7.4 and later refuse a redirect from HTTPS to HTTP by default; what Windows PowerShell 5.1 does is UNVERIFIED, so there the SHA-256 check is the control.
  - It reads the OS architecture from `RuntimeInformation.OSArchitecture`, and falls back to `PROCESSOR_ARCHITEW6432`, then `PROCESSOR_ARCHITECTURE`. So a 32-bit PowerShell on 64-bit Windows still picks the 64-bit build.
  - It swaps the binary in with `File.Replace` and a backup file, and puts the old `grimoire.exe` back if the swap fails.
- **Size:** one template, 240 lines after the Phase 5 review, rendered the same way.
- **Checks:**
  - PSScriptAnalyzer 1.25.0 with its default rules, on the template and on the rendered script. The runners preinstall 1.25.0, and CI asserts the version.
  - Install tests on both Windows targets, under PowerShell 7 and Windows PowerShell 5.1, including the tampered archive.

### 3. `packaging/npm/grimoire.js`, the npm launcher

- **Language:** Node.js, CommonJS, with a node shebang. No dependencies and no lifecycle scripts. The generated `package.json` sets `engines.node` to `>=18`.
- **Why not Rust:** npm puts only the top-level package's `bin` on PATH, so a native binary inside a platform package can't serve `npm i -g` or `npx` on its own. The `agent-grimoire` package needs a launcher. An install script that copies the binary isn't an option either, because npm 12 blocks dependency lifecycle scripts by default.
- **Scope:**
  - Map `process.platform` and `process.arch` to one of the six `@agent-grimoire/{os}-{cpu}` packages.
  - Find its binary with `require.resolve`, and run it with `spawn` (no shell, inherited stdio).
  - Forward `SIGINT`, `SIGTERM` and `SIGHUP` to the child, so that stopping the launcher stops `grimoire` too. This is why it uses `spawn` rather than the `spawnSync` of the launcher pattern in `NPM-P5`.
  - Exit with the child's status, or 128 plus the signal number if the child died of a signal.
  - Fail with a clear error when the platform is unsupported or the optional dependency was omitted.
  - Nothing else.
- **Size:** one file, 81 lines after the Phase 5 review.
- **Checks:** run rather than linted. The install tests on every built target install the launcher and the platform tarball, then run `npx grimoire --version`. `node --check` catches syntax errors locally.

### 4. The generated Homebrew formula, `agent-grimoire.rb`

- **Language:** Ruby, in Homebrew's formula DSL.
- **Why not Rust:** Homebrew reads formulae only in that DSL.
- **Scope:** a prebuilt-binary formula with the per-platform archive URLs and SHA-256s, the licence and the install step. `cargo xtask release channels` writes it from Rust. No Ruby is written by hand or checked in.
- **Size:** one generated file per release.
- **Checks:** on every PR, `brew style`, then `brew audit --strict --os=all --arch=all` in a trusted local tap.

### 5. GitHub Actions workflows with `bash` and `pwsh` steps

- **Language:** workflow YAML with shell steps.
- **Why not Rust:** GitHub Actions is configured only in YAML, and some steps must call runner tools such as `gh`, `npm`, `cargo publish`, `cosign`, `curl`, `brew` and `pwsh`.
- **Scope:**
  - **The YAML orchestrates external tools.** Checkout, installing tools with checksum checks, calling `cargo xtask release …`, moving artifacts, and calling the external CLIs (`gh`, `npm`, `cargo publish`, `cosign`, `curl`) with short checks around them.
  - **Decisions about what a release contains live in `cargo xtask`** and are unit-tested there: the mode, names, matrices, limits, checksums, rendering and verification.
- **A known exception: the publish jobs' shell.** These steps decide what to do from external answers: whether the tag exists and at which commit, whether the release is missing, a draft or published, which crate versions crates.io lacks, and which npm versions are already published. They also order the npm tarballs.
  - They run only in a real release, after the approval, with no unit test and no PR dry-run.
  - They fail closed on any answer they don't recognise.
  - Keep them small and review every change to them.
  - Moving these decisions into `cargo xtask release` subcommands, with the gh, curl and npm calls left in YAML, is a revisit item.
- **The install-test steps** in `verify-install` are also long, about 45 lines each. They run on every PR, so a break shows up there.
- **Size:** keep other `run:` steps to a few lines. A step that grows a new decision moves into xtask.
- **Checks:**
  - actionlint 1.7.12, and zizmor 1.32.0 at the pedantic persona (`ZIZMOR`).
  - Actions pinned by commit SHA, plus `actions.lock` (`CI-PINS`).

### 6. The tap and bucket sync workflows

- **Files:** `packaging/homebrew-tap/sync-formula.yml` and `packaging/scoop-bucket/sync-manifest.yml`. They are templates that the maintainer copies into `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`.
- **Language:** workflow YAML with shell steps.
- **Why not Rust:** they run in repositories that have no Rust toolchain and no xtask.
- **Scope:**
  - Download `agent-grimoire.rb` or `agent-grimoire.json` from the latest Grimoire release.
  - Check it with `gh attestation verify` against this repository's release workflow.
  - Open a PR with that repository's own `GITHUB_TOKEN`. If an earlier run left the sync branch without a PR, and the branch holds exactly the verified file, open its PR.
  - Before any release exists, end with a notice.
  - Never edit the file.
- **Size:** one workflow each, 168 lines after the Phase 5 review.
- **Checks:** actionlint and zizmor, as for this repository's workflows. They aren't run here, only in their own repositories.

## Consequences

- Every exception has a linter, an execution test, or both. The npm launcher is only run, not linted. The sync workflows are only linted here; they run in the other repositories.
- ShellCheck, PSScriptAnalyzer and Homebrew are CI dependencies only because of these exceptions. Each is pinned or asserted by version.
- The formula is reviewed through its Rust generator, not by hand.
- Hand-written code outside Rust stays small: two installer templates, one launcher, workflow steps around external tools, and the publish jobs' shell, which is the one exception to the rule that decisions live in xtask.

**Revisit when:**
- dist's installers check checksums on every platform and accept checksums for archives they didn't build (ADR-0009). Then the scripts could be dropped.
- A channel offers a way to run a Rust binary where one of these files is needed today.
- The publish jobs' decisions can move into `cargo xtask release` subcommands with unit tests, leaving only the external calls in YAML.
