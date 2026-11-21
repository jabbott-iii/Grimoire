# ADR-0003: Release tooling: release-plz for versioning plus a hand-written, SHA-pinned release pipeline

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16). Amended by [ADR-0009](0009-release-pipeline.md) (2026-11-17): release-plz only opens the release PR; the release workflow tags and publishes; release-PR CI comes from approved `pull_request` runs, not `workflow_dispatch`; Grimoire writes its own installers (ADR-0010) and wheels; Homebrew and Scoop pull attested files instead of autobump and Excavator.
- **Date:** 2026-11-16
- **Specs:** `DIST`, `RELEASE-PLZ`, `CRATES-TP`, `NPM-TP`, `PYPI-TP`, `MATURIN`, `BINSTALL`, `BREW`, `WINGET`, `SCOOP`, `GHA-SEC`, `ATTEST`, `COSIGN`, `SBOM`, `RUNNERS` in [`../specs.md`](../specs.md), the register of verified external specs. An ID in parentheses below, such as (`GHA-SEC`), names the entry a fact comes from.

## Context

Release requirements from the project brief (the maintainer's original requirements):

- Prebuilt binaries for Linux (x86_64 and aarch64, static musl), macOS (x86_64 and aarch64) and Windows (x86_64 and aarch64). Static musl means linked statically against the musl C library, so the binary runs on any Linux.
- Shell and PowerShell installers.
- SHA-256 checksums, software bills of materials (SBOMs, the list of what each binary contains), build-provenance attestations (signed records of how and where each file was built) and keyless signatures (made with a short-lived certificate from Sigstore, an open-source signing service, instead of a long-lived key).
- Distribution channels:
  - crates.io (Rust's package registry), with cargo-binstall metadata (cargo-binstall installs a crate's prebuilt binary instead of compiling it);
  - npm (the JavaScript package registry), as per-platform packages plus a minimal launcher;
  - PyPI (the Python Package Index), as maturin binary wheels (Python's prebuilt package format, built here by the maturin tool);
  - a Homebrew tap (a third-party package repository for Homebrew, the macOS and Linux package manager);
  - winget and/or Scoop (Windows package managers).
- OpenID Connect (OIDC) trusted publishing wherever supported: continuous integration (CI) proves which workflow it is with a short-lived OIDC token, instead of a stored API token. No long-lived tokens.
- A protected environment that requires the maintainer's manual approval. (A GitHub Actions environment is a named deployment target; jobs that use it can be made to wait for a reviewer.)
- Conventional Commits (commit messages that start with a type, such as `feat:` or `fix:`) with generated changelogs.
- Every publish path has a dry-run (a run that checks everything but publishes nothing) that CI exercises on pull requests.
- Every action pinned to a full commit SHA, with least-privilege jobs. A pinned SHA can't be moved the way a tag can, and each job gets only the permissions it needs.

## Options evaluated (all checked 2026-11-16)

dist is a release tool for Rust projects that generates CI workflows and installers. release-plz automates Rust versioning and changelogs.

| | dist 0.34.0 | release-plz 0.4.3 | Hand-written workflow |
|---|---|---|---|
| **Maintenance** | Releases continue (latest 2026-11-02, commit 2026-11-15). Since 2025-07, mostly one maintainer under a "community-supported model". No official statement from axo, the company behind dist. | Active (release 2026-11-02, commit 2026-11-16) | Ours |
| **Versioning and changelog** | No | **Yes**: release pull requests (PRs), git-cliff (a changelog generator), Conventional Commits | Would have to be built |
| **crates.io trusted publishing** | n/a | **Built in** (no token) | `crates-io-auth-action` |
| **Binaries for the six targets** | Yes, including static musl and Windows arm64 | **No** ("doesn't build and release binaries") | Yes, using native runners (GitHub-hosted machines of each target's OS and CPU) for all six (`RUNNERS`) |
| **Shell/PowerShell installers** | **Yes**, generated | No | Must write them (a language-policy exception: code that isn't Rust) |
| **npm per-platform + launcher** | **No.** A single package that downloads archives at install time. Publishes with `NPM_TOKEN` on Node 20, which **breaks in January 2027** (`NPM-TP`) | No | Yes, with trusted publishing and `npm stage publish` |
| **PyPI wheels** | No | No | Yes: maturin `bindings = "bin"` with a pending publisher (a trusted publisher registered before the PyPI project exists), so **no token at all** |
| **Homebrew** | Formula pushed to the tap with a `HOMEBREW_TAP_TOKEN` PAT (personal access token) | No | Pull model, with no cross-repository credential (see below) |
| **winget / Scoop** | No | No | Generate and validate manifests |
| **Attestations and signing** | `actions/attest@v4`, opt-in | No | `actions/attest` v4.2.2 + cosign v3 bundles |
| **SHA-pinned actions** | **UNVERIFIED.** The workflows are generated and checked for drift, so hand-pinning would mean opting out of regeneration | Uses our workflow | Yes, plus `actions.lock`, GitHub's workflow lockfile (`GHA-SEC`) |

## Decision (proposed)

1. **release-plz** owns:
   - version bumps;
   - per-crate `CHANGELOG.md` generated from Conventional Commits;
   - the release PR, which proposes the next versions and changelog;
   - tags;
   - crates.io publishing through trusted publishing.

   Asset crates (`publish = false`, from ADR-0002, the architecture decision record, or ADR, that keeps them off crates.io) use its `git_only` mode for tags and changelogs.
2. **A hand-written GitHub Actions release pipeline** owns everything else:
   - building the six targets on native runners, with `cargo auditable` (which embeds the binary's dependency list in the binary);
   - archives and SHA-256 checksums;
   - a CycloneDX SBOM (`cargo cyclonedx --spec-version 1.6 -f json`; the tool defaults to 1.3). CycloneDX is a standard SBOM format;
   - `actions/attest` provenance and SBOM attestations;
   - `cosign sign-blob --bundle` for every artifact (cosign is Sigstore's signing tool);
   - a draft GitHub Release, published once all assets are attached (with immutable releases enabled, so a published release's files and tag can't change);
   - then npm, PyPI, Homebrew and Scoop.
3. **dist is not adopted for v1.** The reasons, in order:
   1. Its npm model contradicts the per-platform requirement. It also adds install-time downloads and lifecycle scripts (scripts npm runs when a package is installed).
   2. Its built-in npm and Homebrew publishing depend on long-lived tokens.
   3. Pinning its generated workflows is unverified.
   4. Its maintenance is concentrated in one person.

   A **Phase 5 spike** (a short experiment during Phase 5, the release-pipeline phase of `PLAN.md`) checks whether dist's installer generation can be used *without* its CI. If so, its installers may replace our own scripts. Otherwise we write minimal `install.sh` and `install.ps1` (download, verify the SHA-256 checksum, install) under a language-policy exception ADR. They get ShellCheck and PSScriptAnalyzer (linters for shell and PowerShell scripts) and tests on each OS.
4. **Ordering and approval.**
   - Nothing is published to any registry until every artifact for the release has been built, checksummed, attested and signed.
   - Registry publishing jobs run in one `release` environment with the maintainer as required reviewer. Required reviewers are available because the repository is public (`GHA-SEC`).
   - npm adds a second, registry-side gate: staged versions (uploaded but not yet public) are approved with two-factor authentication (2FA).
5. **Homebrew and Scoop use a pull model, so CI holds no cross-repository credential.** In a pull model, each package repository fetches the release itself:
   - `jabbott-iii/homebrew-tap` uses Homebrew's generated `autobump` workflow.
   - A Scoop bucket (Scoop's name for a package repository) uses Excavator (Scoop's update bot) with `checkver: "github"` and `autoupdate` from the release's `.sha256` files.
   - Each of those repositories updates itself with its own `GITHUB_TOKEN` (the short-lived token GitHub gives each workflow run).
6. **winget:**
   - CI generates and validates the multi-file manifests (schema 1.28) on every release.
   - **Submission to `microsoft/winget-pkgs` is a maintainer-run step**, because every documented automation path needs a PAT (`WINGET`).
   - Scope is v1.x, after Scoop.
7. **Dry-runs on every PR:**
   - `release-plz release --dry-run`, and `cargo publish --dry-run` for each published crate;
   - a build of at least one target per OS, with archive, checksum, SBOM and attestation steps in no-push mode;
   - `npm publish --dry-run` for the launcher and one platform package;
   - `maturin build` plus wheel-tag checks;
   - `brew style`/`audit` on the generated formula (Homebrew's package recipe);
   - Scoop manifest schema validation;
   - `winget validate`, if available on hosted Windows runners (UNVERIFIED);
   - a Docker build and scan without push (Phase 6, the container-image phase).

## Consequences

- **Pipeline maintenance.** We own several hundred lines of release YAML (the configuration file format GitHub Actions uses).
  - They are linted by actionlint and zizmor (linters for GitHub Actions workflows; zizmor looks for security problems).
  - They are pinned by SHA and `actions.lock`, and exercised by PR dry-runs.
- **First publishes are credentialed maintainer steps** (outside CI, never stored). Trusted publishing can't create new packages on crates.io or npm (`CRATES-TP`, `NPM-TP`):
  - one manual first publish per published crate, using a short-lived, crate-scoped token that is then revoked; then `trustpub_only` (the crates.io setting that turns off token publishing) is enabled;
  - one first publish per npm package, using an interactive `npm login` session (2-hour token);
  - PyPI needs no bootstrap (one-time setup).

  [`docs/release.md`](../release.md) §2, the one-time bootstrap, lists these steps (ADR-0009, the release pipeline as built).
- **Release PRs and CI.** PRs and tags created with `GITHUB_TOKEN` don't trigger other workflows (`RELEASE-PLZ`). Two ways around it:
  - Run the build in the same workflow run after the `release` step, and run CI on the release PR via `workflow_dispatch` (a manual run trigger) or a maintainer push. This keeps zero stored credentials.
  - Use a GitHub App, whose private key is itself a long-lived secret.

  **This is an open question for the maintainer.** The default proposal is the first option.
- **Revisiting.** Reconsider dist if it gains per-platform npm packages, OIDC publishing and pinnable workflows.
