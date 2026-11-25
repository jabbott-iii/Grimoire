# 02 — Dependency policy

This document is the policy for third-party code in Grimoire: what counts as a dependency, which ones are allowed, how an exception is granted, and the register of what Grimoire built itself instead of taking a dependency. Grimoire hasn't adopted Doctrina's `dependencies.md` rule, so the policy lives here and in `deny.toml`, and `.claude/rules/rust.md` says how to work with it.

## What counts as a dependency

- **A Rust crate** in any `Cargo.toml` of the workspace, normal, build or dev, including the asset crates under `catalog/`. Each one is declared once, in `[workspace.dependencies]` of the root `Cargo.toml`, with a comment that says why it is there when that isn't obvious, and every member refers to it with `workspace = true`. cargo-deny refuses a duplicate or unused workspace dependency (`[bans.workspace-dependencies]` in `deny.toml`).
- **A GitHub Action** that a workflow uses, and every action it calls in turn.
- **A tool** that a workflow downloads or installs, such as cargo-deny, Grype or cosign.
- **A container base image**, today only the `Dockerfile`'s distroless base.

Data that `cargo xtask` generates, such as JSON and YAML manifests, is not a dependency.

## The rule for third-party code

Ask the maintainer before you add a crate, a feature of one, a dev-dependency, an action, a tool or an exception. A proposal says what the dependency does that `std` or existing code can't, its license, how well it is maintained, how many crates it pulls in, and whether it uses `unsafe`.

The groups in `[workspace.dependencies]` keep the CLI small: the crates under "MCP servers only" and "Release packaging in xtask only" are never linked into the `grimoire` CLI, which has no HTTP, TLS or async crate in its graph.

- **Rust crates** follow [`deny.toml`](../deny.toml), which cargo-deny (a checker for Rust dependencies) checks in `security.yml`.
  - It refuses crates with RustSec advisories (RustSec is the Rust security advisory database), unmaintained and unsound ones included (an unsound crate's safe code can still break memory safety), and yanked versions (versions withdrawn from crates.io). An ignored advisory needs a reason; today one is ignored (RUSTSEC-2026-0543, chrono unmaintained).
  - It allows only the MIT and Apache-2.0 licenses, plus named per-crate exceptions.
  - It refuses duplicate versions except exact, explained skips; wildcard version requirements; OpenSSL and native TLS; and three unmaintained or deprecated YAML crates (YAML is a configuration file format).
  - Its `[bans.build]` section refuses executables and interpreted scripts shipped in dependencies that run at build time, except files pinned by checksum.
  - Crates may come only from crates.io, never from Git.
  - Every CI and release build uses `--locked`, xtask included (`cargo --locked xtask`), except in `ci.yml`'s scaffold job, which adds probe crates (test crates) as new members of the Cargo workspace, the set of crates built together, and so can't.
- **Updates** come from Dependabot, weekly, for Cargo, GitHub Actions, the Rust toolchain, the hash-pinned pip requirements (Python packages pinned by hash) and the `Dockerfile` base.
  - Every entry has a cooldown of at least 7 days for version updates, and 30 days for semver-major Cargo and pip updates (a new major version under semantic versioning, SemVer). As [`.github/dependabot.yml`](../.github/dependabot.yml) records, security updates aren't held back.
  - Dependabot never changes the MSRV (`DEPENDABOT`, the Dependabot options entry in `specs.md`).
  - For the base image the cooldown has no effect, so its age is checked by hand ([maintaining the image pins](05-release.md#maintaining-the-image-pins)).
- **Actions** are pinned to full commit SHAs with version comments, and [`.github/workflows/actions.lock`](../.github/workflows/actions.lock) records every pin (`GHA-SEC` in `specs.md` covers GitHub's guidance on hardening Actions, pins and lockfiles). `security.yml`'s `actions-lock` job fails when the lockfile is incomplete or out of date. Workflows start with `permissions: {}` and grant each job only what it needs.
- **Tools** that workflows download are pinned to exact versions.
  - Most are checked against a pinned SHA-256, two only against a GitHub asset digest ([the release spec](subsystems/release.md), "Some pins protect only the download").
  - cargo-deny is built from crates.io with its lockfile, and cargo-shear, cargo-auditable and cosign come through pinned installer actions.
  - Each pinned action and tool was a release at least 7 days old when adopted.

## Exceptions

An exception is an entry in `deny.toml` that allows what the policy would refuse. Each one has a reason in the file, names exactly the crate and version it covers, and is reviewed like code. cargo-deny fails when an exception no longer applies (`unused-ignored-advisory`, `unused-allowed-license` and `unused-license-exception` are `"deny"`), so none outlives its reason. Today's exceptions:

| Kind | Entry | Why |
|---|---|---|
| Advisory | RUSTSEC-2026-0543, chrono unmaintained | An unmaintained notice, not a vulnerability. rmcp 3.10.0 and 3.11.0 depend on chrono directly, so no upgrade removes it. The maintainer's choice on 2026-11-17, reviewed again on each rmcp bump |
| License | `encoding_rs`: BSD-3-Clause | Through serde-saphyr's `deserialize` feature |
| License | `foldhash`: Zlib | Through the `preserve_order` features of serde_json and schemars |
| License | `subtle`: BSD-3-Clause | Constant-time primitives under the MCP servers' signature verifiers ([ADR-0007](adr/0007-mcp-server-authorization.md)). The maintainer's choice in Phase 3 |
| License | `unicode-ident`: Unicode-3.0 | Through proc-macro2, under the derive macros |
| License | `winx`: Apache-2.0 WITH LLVM-exception | Windows only, through cap-std ([ADR-0006](adr/0006-filesystem-confinement.md)) |
| Duplicate versions | `io-lifetimes`, `windows-sys`, `windows-targets` and `windows_*_msvc` at exact versions | Each forced by cap-std 4.0.3. Removed as soon as upstream unifies it |
| Duplicate versions | `syn@2.0.119` | `tracing-attributes` (through rmcp) and `zeroize_derive` (through jsonwebtoken) still need syn 2. The maintainer's choice in Phase 3 |
| Build-time files | The `windows_*_msvc` import libraries, each pinned by SHA-256 | Shipped for linking on Windows |
| Build-time files | rmcp 3.10.0's six JavaScript and Python test files, each pinned by SHA-256 | Never built or run; its build script isn't trusted as a whole. The maintainer's choice in Phase 3 |

To add one, propose it with its reason, as above, and add it to `deny.toml` with that reason in the same change once the maintainer agrees.

## Language policy

Rust is the default for all code ([ADR-0010](adr/0010-language-policy-exceptions.md), the language-policy exceptions). Rust gives the project one toolchain, one lint policy and one test harness. A few pieces can't be Rust: they run before any Grimoire binary is on the machine, inside a tool that reads only its own language, or in another repository that has no Rust toolchain. The only exceptions are:

- the two install scripts, `packaging/install/install.sh` and `install.ps1`;
- the npm launcher, `packaging/npm/grimoire.js`;
- the generated Homebrew formula;
- the shell steps in GitHub Actions workflows;
- the Homebrew tap and Scoop bucket sync workflows, in `packaging/`;
- `.github/scripts/image-tags.sh`, the container image's registry steps, which [ADR-0011](adr/0011-container-image.md) added.

Each has a stated scope and checks of its own, in ADR-0010. Any other code that isn't Rust needs a new ADR. Data files that `cargo xtask` generates, such as JSON and YAML manifests, aren't code.

## Build-or-buy register

What Grimoire depends on for each need, and what it built itself, with the reason and where the decision is recorded. Entries in `specs.md` hold the research.

| Need | Built or bought | Why | Recorded in |
|---|---|---|---|
| A filesystem confined to the project root | Bought: cap-std 4.0.3, with cap-fs-ext and cap-tempfile | Its directory handle resolves one component at a time, which closes the race between checking a path and writing it | [ADR-0006](adr/0006-filesystem-confinement.md), `FS-CRATES` |
| Comment-preserving JSON and JSONC edits | Bought: jsonc-parser's concrete syntax tree, with every non-standard extension off, after a strict serde_json parse | Its strict mode still accepts comments, so strictness comes from serde_json | `EDIT-CRATES` |
| Comment-preserving TOML edits | Bought: toml_edit | Edits to `grimoire.toml` and `.codex/config.toml` keep the user's comments | [the install spec](subsystems/install.md) |
| YAML front matter | Bought: serde-saphyr | serde_yaml, serde_yml and yaml-rust are refused in `deny.toml` as unmaintained or unsound | `deny.toml` |
| Markdown structure | Bought: pulldown-cmark, default features off | Rule and skill checks, and the README link test, read Markdown as a renderer does | [the catalog spec](subsystems/catalog.md#catalog-validation) |
| Agent Skills validation | Built, natively in Rust | The reference validator is "for demonstration purposes only", and a native check needs no Python | `SKILLS` |
| The MCP protocol | Bought: rmcp, the Rust MCP SDK | It implements the current MCP version; Grimoire closes its two gaps (Origin validation off by default, server-side authorization only in examples) in `agent-grimoire-mcp` | [ADR-0002](adr/0002-mcp-server-and-tool-packaging.md), [ADR-0007](adr/0007-mcp-server-authorization.md), `RMCP` |
| Access-token checks | Bought, narrowed: jsonwebtoken with no crypto backend, plus a verify-only provider over rsa and p256 | Verifies tokens locally against a JWKS file, with no outbound network | [ADR-0007](adr/0007-mcp-server-authorization.md) |
| Secret detection in `secret-guard` | Built: vendor-format rules with regex and memchr | Known formats only, no entropy heuristics | `SECRET-FORMATS` |
| Release orchestration | Built: a hand-written pipeline in `cargo xtask release`, with release-plz for versions and changelogs; dist not adopted | dist's npm installer downloads binaries at install time, and its npm and Homebrew publishing need long-lived tokens | [ADR-0003](adr/0003-release-tooling.md), [ADR-0009](adr/0009-release-pipeline.md) |
| Install scripts | Built: `install.sh` and `install.ps1` | dist's PowerShell installer checks no checksum, and its shell installer checks one only when dist built the archives | [ADR-0009](adr/0009-release-pipeline.md), [ADR-0010](adr/0010-language-policy-exceptions.md) |
| PyPI wheels | Built, by `cargo xtask` from the archived binary, not maturin | The wheel holds the same binary as every other channel | [ADR-0009](adr/0009-release-pipeline.md) |
| Zip archives | Built: a small writer in `xtask/src/release/zip.rs`, over flate2 | Reproducible archives with hostile-archive limits | [ADR-0009](adr/0009-release-pipeline.md) |
| `.tar.gz` archives | Bought: tar, without xattr support, and flate2 | Archives carry only names, modes and contents | [ADR-0009](adr/0009-release-pipeline.md) |
| SBOMs | Bought: cargo-cyclonedx 0.5.10, run once per target | CycloneDX 1.6, one SBOM per archive | `SBOM` |
| Image scanning | Bought: Grype, from its checksum-pinned release tarball | Trivy and its action were compromised in March 2026 (CVE-2026-33634), and `anchore/scan-action` pipes an install script to `sh` | [ADR-0011](adr/0011-container-image.md), `SCAN-P6` |
| Unified diffs in `grimoire plan` | Bought: similar | | |
