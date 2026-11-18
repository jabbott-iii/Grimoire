# ADR-0009: Release pipeline and distribution channels

- **Status:** Accepted (the maintainer's decisions when asked, 2026-11-17, during Phase 5, revised the same day after the Phase 5 review). Amended by [ADR-0011](0011-container-image.md) (2026-11-17): a fifth environment, `ghcr`; the `image`, `verify-image`, `publish-image` and `sign-image` jobs; every signature and attestation moves into the reusable workflow `sign.yml`, under a new identity, and `sign-files` replaces `attest` and `attest-sbom`; and PR dry-runs also build `aarch64-unknown-linux-musl`.
- **Date:** 2026-11-17
- **Specs:** `RUST-REGISTRY-P5`, `NPM-P5`, `PYPI-P5`, `BREW-SCOOP-P5`, `WINGET-P5`, `SUPPLY-CHAIN-P5`, `ACTIONS-P5`, `INSTALLERS-P5` in [`../specs.md`](../specs.md)
- **Amends:** [ADR-0002](0002-mcp-server-and-tool-packaging.md) (how asset binaries are found), [ADR-0003](0003-release-tooling.md) (who tags and publishes, release-PR CI, installers, PyPI, Homebrew and Scoop, dry-runs) and [ADR-0004](0004-package-and-binary-names.md) (the winget identifier and command name)
- **Related:** [ADR-0010](0010-language-policy-exceptions.md), the code this pipeline needs that isn't Rust. [`docs/release.md`](../release.md) is the runbook.

## Context

ADR-0003 chose release-plz for versions and changelogs, and a hand-written workflow for everything else. It left three questions open: whether dist's installers could be used, how release PRs get CI, and how Homebrew and Scoop update themselves.

Phase 5 re-checked every channel in eight research topics, each re-checked by a skeptic (`*-P5`). These findings changed the plan:

- **release-plz.** `release-plz/action` installs cargo-binstall with `curl | tar` and no checksum, today from `releases/latest`. It then installs a release-plz archive that has no checksum file. `release-plz release` tags a package only after its own crates.io upload, so it can't tag the commit the pipeline built and signed before anything is published.
- **Release-PR CI.** Checks from `workflow_dispatch` runs don't satisfy required status checks, so ADR-0003's plan fails the ruleset. GitHub's docs say that since 2026-05-22, a PR that `GITHUB_TOKEN` opens or updates gets `pull_request` runs that wait for approval.
- **Installers.** In dist 0.34.0, `installer.ps1` checks no checksum at all. `installer.sh` skips its check when `sha256sum` is missing, and it embeds hashes only for archives dist built itself.
- **PyPI.** maturin can't package a binary that is already built; it always compiles.
- **Homebrew and Scoop.** Homebrew's generated autobump rewrites only top-level `url` and `sha256`, so it can't update a formula with one archive per platform. Scoop's Excavator pushes straight to the bucket's branch and installs Scoop from an unpinned script at run time.
- **winget.** `CatfinityStudios.Grimoire` already uses the command alias `grimoire`, and winget replaces an existing link of the same name. Every automated submission path needs a classic PAT.
- **Attestations.** `actions/attest` has no dry-run and fails without `id-token`. On a public repository every run writes a permanent public Rekor entry.
- **Environments.** A run that references a missing environment creates it with no protection rules.
- **npm.** Trusted publishing has an open Priority 1 bug for repositories that use GitHub's immutable OIDC `sub` (npm/cli#9969). This repository was created on 2026-11-16, so it uses that `sub`.

An adversarial review of the Phase 5 change then found, among other things, that a container action in a job can mint an OIDC token for any audience. With one publish job, the PyPI action's tag-pinned image could therefore have published to crates.io. That led to decisions 9 and 10 and to the changes under "Forced by the review".

## Decision

### Maintainer decisions (2026-11-17)

1. **The repository is `jabbott-iii/Grimoire`**, with a capital G, everywhere. cosign identities and the npm and PyPI publisher fields match it case-sensitively. `repository = "https://github.com/jabbott-iii/Grimoire"`.
2. **One version, one tag.** `agent-grimoire` (binary `grimoire`) and `agent-grimoire-{model,catalog,resolve,install,adapters}` share `[workspace.package] version`. The tag is `vX.Y.Z`.
3. **release-plz skips asset crates.** They are `publish = false`, which release-plz already leaves out. Their hand-written catalog CHANGELOGs stay the source. This replaces ADR-0003's `git_only` plan.
4. **We write our own installers.** The dist spike failed (above), so we write `install.sh` and `install.ps1` under ADR-0010. This is ADR-0003's stated fallback.
5. **PyPI wheels are written by `cargo xtask`** from the binary already built for the release, not by maturin. Every channel ships the same signed bytes.
6. **Homebrew and Scoop pull attested files.**
   - Each release attaches a generated `agent-grimoire.rb` (a prebuilt-binary formula) and `agent-grimoire.json` (a Scoop manifest), both attested and signed.
   - Grimoire ships template workflows for `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`. Each downloads its file from the latest release, checks it with `gh attestation verify`, and opens a PR using only that repository's own `GITHUB_TOKEN`.
   - No autobump and no Excavator.
7. **winget:** identifier `jabbott-iii.agent-grimoire`, `Moniker: agent-grimoire`, and **`PortableCommandAlias: agent-grimoire`**, because `grimoire` collides with `CatfinityStudios.Grimoire`. CI generates and validates the manifests. The maintainer submits them with a fork and a PR.
8. **release-plz opens the release PR only** (versions and changelog). It runs as a pinned, checksum-checked binary, not through `release-plz/action`. The release workflow tags the exact commit it built and signed, and publishes to crates.io with `rust-lang/crates-io-auth-action` and `cargo publish`. This amends ADR-0003.
9. **Four publish jobs, four environments.** Publishing is split so that each registry's credential exists only in its own job:
   - `publish`, in environment `release`: the checks, the tag and the GitHub Release. It has `contents: write` and `attestations: read`, and no `id-token`.
   - `publish-crates` (`crates-io`), `publish-npm` (`npm`) and `publish-pypi` (`pypi`), after it. Each has only `id-token: write`, plus `contents: read` for the crates checkout.
   - Each registry's trusted publisher is bound to its own environment. A token minted in another job names another environment, so the registry refuses it.
   - The cost is two approval rounds per release: `release`, then the three registry environments together.
10. **One build per package.** `cargo xtask release build` runs `cargo auditable build -p <package>` once for the CLI and once for each asset binary. Each binary then has exactly the dependency features its own manifest asks for, as `cargo install` would give it, and its SBOM describes it. One multi-package build would unify features across the CLI and the asset crates.

### Forced by research

- **Release-PR CI comes from approval-required `pull_request` runs.** The maintainer approves them, or pushes to the PR branch. Neither needs a stored credential. This replaces ADR-0003's `workflow_dispatch` plan.
- **Windows MSVC builds use `-C target-feature=+crt-static`**, so the binary needs no VC++ runtime. The flag must be in the same `RUSTFLAGS` as `--remap-path-prefix`, because `RUSTFLAGS` replaces the config file's target `rustflags`. Whether winget's VC++ runtime check passes for such a build is unverified.
- **musl targets are static by default.** No `musl-tools` is needed while no dependency compiles C.
- **PR dry-runs skip `actions/attest`.** They run cosign offline instead, with a throwaway key and a signing config that names no services, so nothing reaches Rekor.
- **`CARGO_REGISTRY_TOKEN` is never set to an empty string.** release-plz reads an empty value as a token and skips trusted publishing, and what Cargo does with an empty token is unverified.
- **Release mode waits until the environments exist**, through the `GRIMOIRE_RELEASES` switch below.

### Forced by the review

- **Release mode needs the version's `CHANGELOG.md` section.** Otherwise any push to `main` after the switch is on would release the untagged version, release PR or not. Without the section, `plan` prints a warning and runs a dry-run, and `cargo xtask release notes` fails rather than writing fallback text.
- **`plan` fails early at GitHub's limits:** 256 jobs in the `attest-sbom` matrix, and 1000 assets in one release. With today's channels the asset limit comes first, at 39 asset binaries. Growing past it needs a new ADR on how to split the release. *(Phase 6, [ADR-0011](0011-container-image.md): the SBOM attestation matrix now lives in `sign.yml`, as its `sboms` job, and `plan` checks the same 256-job limit. Whether a matrix inside a called workflow also counts toward the caller run's limit is UNVERIFIED (`SIGN-P6`).)*
- **Signed files are bound to the package job.** Every release-mode job downloads `release-dist` by artifact ID, from the package job's outputs. Before it attests, signs or publishes, it checks `SHA256SUMS` against the hash the package job recorded, and that `dist/` holds exactly the files `SHA256SUMS` lists.
- **One required check for the release workflow.** The `release checks` job sums up the workflow's checks, and the ruleset requires it next to CI and Security. On a pull request it fails unless every dry-run job passed, because a required check whose job is skipped would count as passed.
- **The npm dry-run on pull requests runs offline.** It points npm at an unreachable local registry (`http://127.0.0.1:9/`), so npm sees no published versions. A PR between releases, whose version is already on npm, still exercises npm's checks. Release mode checks against the real registry. `--force` isn't used, because it also skips the prerelease-tag check.
- **release-plz runs only on `main`.** On another branch it would rebuild the release PR from that branch. Its runs queue in one group and are never cancelled.
- **Workflows call `cargo --locked xtask …`,** so a stale `Cargo.lock` fails instead of being rewritten before the `--locked` build and publish.
- **Format fixes:** the six published crates carry `LICENSE-MIT` and `LICENSE-APACHE` (an xtask test keeps them identical to the root ones); Scoop's licence field is `MIT|Apache-2.0`, Scoop's documented separator; wheel file names sort the compressed platform-tag set; and an xtask test checks the binstall URLs against the archive names.

### The pipeline

One workflow, `.github/workflows/release.yml`, runs on `pull_request`, on `push` to `main` and on `workflow_dispatch`. `cargo xtask release` makes every decision about what a release contains; the YAML orchestrates the external tools (ADR-0010).

The order is **plan → build → package → verify → attest and sign → approval → publish → approval → registries**:

| Step | What happens | Why it is here |
|---|---|---|
| plan | `cargo xtask release plan` picks the mode, version, tag, targets and expected archive names, and checks the limits | Every later job reads the same decision |
| build | One native runner per target: `cargo auditable build --profile dist --locked`, once per package, a `--version` smoke test, then the archives | Native builds need no cross toolchain, and the smoke test runs on the real OS |
| package | On Ubuntu: SBOMs, the channel files, `npm pack`, `SHA256SUMS`, `cargo xtask release verify` | Every channel is cut from the binaries inside the archives |
| verify | Install tests per target, and the registry dry-runs and linters on Ubuntu | Nothing that fails a check gets signed |
| attest and sign | Provenance for every file, an SBOM attestation per archive, a cosign bundle per file | The approver sees a complete, signed set |
| approval | `publish` waits in the `release` environment | One human gate before any public write, apart from transparency-log entries |
| publish | Re-verify, tag, GitHub Release | Everything downloads from the release, so it comes first |
| approval | The three registry jobs wait in `crates-io`, `npm` and `pypi` | Each registry credential exists only behind its own gate |
| registries | crates.io, npm and PyPI, in parallel | Each step skips what already exists |

Inside `publish`, in this order:
1. Check that the downloaded files are the package job's (above), then `SHA256SUMS`, every cosign bundle against the identity below, and `gh attestation verify SHA256SUMS`.
2. Create tag `vX.Y.Z` at `GITHUB_SHA` if it is missing. Fail if it exists at another commit.
3. Write the release notes from `CHANGELOG.md`.
4. Create a draft GitHub Release, upload every file, publish it, then print the release's `immutable` field, with a warning if it isn't `true`. `GITHUB_TOKEN` can't read the repository's immutable-releases setting, so the release object is the only check, and it doesn't fail the job.

Then, in parallel:
- **crates.io** (`publish-crates`): `crates-io-auth-action`, then `cargo publish --locked -p <crate>` for each crate whose version isn't on crates.io yet, in dependency order.
- **npm** (`publish-npm`): `npm stage publish --access public --ignore-scripts` for the six platform tarballs first, then the launcher. The maintainer approves them with 2FA in the same order, because batch approval isn't atomic.
- **PyPI** (`publish-pypi`): `pypa/gh-action-pypi-publish` with `skip-existing: true`, in a job that holds nothing but the wheels.

The GitHub Release comes before the registries because `cargo binstall`, the install scripts, Homebrew, Scoop and winget all download from it. It is also the record of what was signed, and immutable once published if the repository setting is on.

### The release switch: `GRIMOIRE_RELEASES`

`plan` decides the mode:
- `dry-run` for `pull_request`, and for any ref other than `refs/heads/main`;
- on `main`, for `push` and `workflow_dispatch`:
  - `release` when the repository variable `GRIMOIRE_RELEASES` is `enabled`, tag `vX.Y.Z` doesn't exist on `origin`, and `CHANGELOG.md` has a section for X.Y.Z;
  - `skip` when the tag exists;
  - `dry-run` otherwise, with a warning when only the changelog section is missing.

Why a switch:
- A run that references a missing environment creates it with no protection rules. No job may name `release`, `crates-io`, `npm` or `pypi` until the maintainer has created them and added the required reviewer.
- The maintainer sets the variable last, after the bootstrap in `docs/release.md`. Until then, every push to `main` runs the dry-run.
- The variable only turns release mode on; it approves nothing. The required reviewers on the environments are the gates.

### Re-runs

- Once the tag exists, `plan` returns `skip` on `main`, so a later push never releases the same version again.
- Every publish job skips what is already done: the tag at this commit, a finished draft or a published release with the same files, crate versions on crates.io, published npm versions, and PyPI files. Re-running the failed jobs of a run usually finishes the release.
- **Staged npm versions are the exception.** CI can't see a version that is staged but not approved, and npm refuses to stage the same version twice. `publish-npm` then fails with a message telling the maintainer to approve or reject the earlier stage first. PyPI is a separate job, so npm no longer blocks it.
- **Re-runs have limits.** GitHub allows them for 30 days, and a re-run uses the run's own commit, so it can't pick up a fix. If a re-run can't finish the release, and the GitHub Release is missing or still a draft, and no registry has the version, the maintainer deletes the draft and the tag (nothing locks them before publication), fixes `main`, and releases again. If any registry already has the version, it is never reused: the next release bumps it. `docs/release.md` §3 has the steps.
- A published tag is never deleted or moved. With immutable releases, GitHub refuses both, and the tag name can never be reused.

### Assets

Targets and runners (`PLAN.md` §4). PR dry-runs build the three marked *PR*, one per OS.

| Target | Runner | Archive |
|---|---|---|
| `x86_64-unknown-linux-musl` (PR) | `ubuntu-24.04` | `.tar.gz` |
| `aarch64-unknown-linux-musl` | `ubuntu-24.04-arm` | `.tar.gz` |
| `x86_64-apple-darwin` | `macos-15-intel` | `.tar.gz` |
| `aarch64-apple-darwin` (PR) | `macos-15` | `.tar.gz` |
| `x86_64-pc-windows-msvc` (PR) | `windows-2025` | `.zip` |
| `aarch64-pc-windows-msvc` | `windows-11-arm` | `.zip` |

Every file name is ASCII `[A-Za-z0-9._-]`, because GitHub renames anything else, and all files sit flat in one directory:

| Asset | Name and contents |
|---|---|
| CLI archive | `agent-grimoire-{V}-{triple}.tar.gz`, or `.zip` on Windows. Flat: `grimoire` (or `grimoire.exe`, mode 0755), `LICENSE-MIT`, `LICENSE-APACHE`, `README.md` |
| Asset binaries (ADR-0002) | `{bin}-{assetV}-{triple}.tar.gz` or `.zip`. Flat: the binary, both licences and the asset's own `README.md` |
| SBOM | `{archive}.cdx.json`, CycloneDX 1.6 JSON |
| npm | `agent-grimoire-{V}.tgz` (launcher) and `agent-grimoire-{os}-{cpu}-{V}.tgz` (platform packages), from `npm pack` |
| PyPI | `agent_grimoire-{V}-py3-none-{tags}.whl`, with the platform tags sorted and joined by `.` |
| Homebrew, Scoop | `agent-grimoire.rb`, `agent-grimoire.json` |
| winget | `jabbott-iii.agent-grimoire.yaml`, `jabbott-iii.agent-grimoire.installer.yaml`, `jabbott-iii.agent-grimoire.locale.en-US.yaml` |
| Installers | `install.sh` and `install.ps1`, with the version and the CLI archive SHA-256s embedded: four Linux and macOS targets in `install.sh`, the two Windows targets in `install.ps1` |
| Checksums | `SHA256SUMS`: GNU untagged lines (`<hex>  <name>`), sorted by name, for every other file except `*.sigstore.json` |
| Signatures (release mode only) | `{file}.sigstore.json` for every file, `SHA256SUMS` included |

- Archives use a fixed mtime (1980-01-01T00:00:00Z), uid and gid 0 and sorted entries, so the same binaries always give the same archives.
- **Asset binaries are found with `cargo metadata --no-deps`:** every `bin` target of a package under `catalog/tools/` or `catalog/mcp-servers/`. Nobody edits a list by hand. This settles the point ADR-0002 left for Phase 5.

### The signing identity

- cosign v3.1.4 (at least 3.1.3) signs every file keylessly in the `attest` job:
  - certificate identity `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`;
  - issuer `https://token.actions.githubusercontent.com`.
- The ref is `refs/heads/main` because releases run on a push to `main`, not on a tag. Fulcio builds the identity from `job_workflow_ref` and keeps its case. The immutable `sub` doesn't change it.
- `actions/attest` v4.2.2 writes SLSA provenance for every file, and one CycloneDX SBOM attestation per archive.
- Users check downloads with `sha256sum -c`, `gh attestation verify` (gh 2.102.0 or later), `cosign verify-blob`, and `gh release verify-asset` for GitHub's own release attestation. The exact commands are in `docs/release.md`.
- *(Phase 6, [ADR-0011](0011-container-image.md) decision 6: every signature and attestation moved into the reusable workflow `.github/workflows/sign.yml`, which the `sign-files` job calls, because every job in `release.yml` with `id-token` got the identity above. The identity is now `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`. Any repository can call a public reusable workflow and get a certificate with that same name, so verifiers also pin the caller: cosign with `--certificate-github-workflow-repository jabbott-iii/Grimoire --certificate-github-workflow-ref refs/heads/main`, and gh with `--repo jabbott-iii/Grimoire --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml --source-ref refs/heads/main --deny-self-hosted-runners`. No release exists yet, so no release was signed with the identity above: every release is signed by `sign.yml`.)*

### What PR dry-runs check, and what they can't

On every PR, the dry-run:
- runs `plan`, including the limit check, then builds, archives and smoke-tests the three PR targets;
- writes the SBOMs, every channel file, the npm tarballs and `SHA256SUMS`, then runs `cargo xtask release verify`;
- runs `install.sh` and `install.ps1` against a local HTTP server, including a tampered archive they must refuse;
- installs the npm launcher with its platform tarball, and the wheel into a venv, and runs `grimoire --version` from each;
- runs `cargo publish --workspace --dry-run --locked`, `twine check --strict`, ShellCheck, PSScriptAnalyzer, `Test-Json` against the pinned Scoop and winget schemas, `brew style` and `brew audit` in a local tap, and `npm stage publish --dry-run` against an unreachable local registry;
- signs and verifies every file with cosign, offline;
- runs `winget validate` only if `winget` happens to exist on the runner;
- ends with `release checks`, the one check the ruleset requires.

What PR dry-runs can't check:
- **Attestations, keyless signing and Rekor.** They need `id-token`, and every attempt writes a permanent public entry.
- **Any real OIDC exchange.** Dry-run jobs get no `id-token: write`, because `npm publish --dry-run` performs a real exchange when it has one.
- **The publish jobs:** the tag, `cargo xtask release notes`, the GitHub Release, the registry writes, the environment approvals and immutable-release behaviour. Their shell runs first in a real release.
- **The other three targets** (`aarch64-unknown-linux-musl`, `x86_64-apple-darwin`, `aarch64-pc-windows-msvc`). They are first built in release mode, before the approval.
- **The tap and bucket sync workflows, winget-pkgs validation, and npm's publish-time malware scan.**

## Consequences

**Positive:**
- No credential is stored anywhere. crates.io, npm and PyPI use OIDC. Each tap or bucket uses its own `GITHUB_TOKEN`. winget is submitted by hand.
- Each registry's OIDC token can be minted only in that registry's job, behind that registry's environment.
- Every channel carries the bytes that were signed and attested.
- Every file has a checksum, a provenance attestation and a cosign bundle, and every archive also has an SBOM attestation.
- A failed job can usually be re-run without duplicating anything; the limits are under "Re-runs".

**Negative and risks:**
- **npm trusted publishing may fail for this repository.**
  - npm/cli#9969 ("package not found" for repositories with an immutable OIDC `sub`) was open on 2026-11-17, labelled Priority 1, with no fix announced. Registry data shows one other post-cutoff repository publishing through a GitHub trusted publisher in November, so it can work.
  - **Fallback:** the maintainer stages the signed `.tgz` files from the GitHub Release with an interactive `npm login` session (2 hours, 2FA), then approves them. No token is stored.
  - Those versions have no npm provenance. Versions staged over OIDC should carry it, but that is UNVERIFIED (`NPM-P5`). The release's cosign bundles and attestations still cover the same tarballs.
- **First publishes stay manual.** Trusted publishing can't create a crate or an npm package. The maintainer publishes each crate once with a short-lived token, from the commit the first release run tagged, and stages each npm name once with an interactive session. PyPI's pending publisher creates the project, but PyPI deletes it 30 days after it is created.
- **The PyPI publish action's image is pinned by tag, not digest.**
  - `gh-action-pypi-publish` runs `ghcr.io/pypa/gh-action-pypi-publish:<sha>`. Every build of that commit re-pushes the tag, and the tag can be overwritten.
  - Its base image is unpinned, and its dependency hashes sit in a constraints file that only pip 26.1 or later enforces.
  - The container gets the job's OIDC request token, so it can mint a token for any audience. That token names environment `pypi`, which the crates.io and npm publishers refuse, so a re-pushed image can at worst publish to PyPI. The job holds only the wheels.
  - Accepted for now. `uv publish` is the alternative.
- **release-plz's token can do more than open the release PR.** The job's `GITHUB_TOKEN` has `contents: write` and `pull-requests: write`, so a compromised release-plz binary could push branches, create or delete tags, publish GitHub Releases or merge PRs the ruleset allows. It has no `id-token`, so it can't reach a registry. See `security-model.md` §6b.
- **Immutability isn't enforced by the workflow.** `publish` only warns when a release comes out mutable. Such a release has no release attestation, so the tap and bucket can't sync it until it is deleted and published again (`docs/release.md` §3).
- **Rust 1.100.0 is younger than the 7-day rule** used for every other pin. It was released on 2026-11-12, and the newest eligible toolchain is 1.99.1. This was a Phase 1 choice and isn't changed here.
- **release-plz 0.4.3 has no upstream checksum file.** Its pin is GitHub's asset digest. That pins the exact bytes, but says nothing about who built them.
- **`macos-15-intel`'s image is stale.** Its latest release is from 2026-08-24, with Rust 1.98.0, and `macos-26-intel` is now a standard label. Builds must install the pinned toolchain rather than use the image's Rust. Moving `x86_64-apple-darwin` to `macos-26-intel` is a follow-up.
- **Windows runners change from 2026-11-30.** The Windows 11 Arm64 workspace moves to `D:`, and `D:` becomes ReFS on the Server 2022, Server 2025 VS2026 and Windows 11 Arm64 images. Each rollout takes 2 to 3 days. Builds remap the actual workspace and `CARGO_HOME` paths, so a moved workspace is still remapped. Whether builds behave the same on ReFS is untested.
- **cargo-binstall checks neither the archive's checksum nor its Sigstore bundle.** It checks the crates.io crate checksum and relies on HTTPS for the archive. Grimoire's binstall metadata turns off `quick-install`, the only strategy that sends telemetry. Users who want the archive verified download and check it themselves.
- **winget users run `agent-grimoire`, not `grimoire`.** The alias keeps winget from replacing `CatfinityStudios.Grimoire`'s `grimoire` link, and that package from replacing ours. Every other channel installs `grimoire`.
- **Signing comes before approval.** A run the maintainer never approves still leaves Rekor entries and GitHub attestations for its files, under this workflow's identity. Only files attached to the immutable release are released, which `gh release verify-asset` checks.
- **One maintainer.** The author of a release is also its only approver, so "Prevent self-review" must stay off. Every update of the release PR needs its runs approved again, and every release needs two approval rounds.
- **Three targets are built only in release mode.** A break there shows up only in the next release run, before the approval, so nothing is published.
- **The publish jobs' shell is tested only by real releases.** It decides from `gh`, `curl` and `npm` answers (tag and release state, crates.io HTTP codes, `npm view` errors), and no PR dry-run runs it. ADR-0010 records it as a known exception.

**Revisit when:**
- npm/cli#9969 is fixed, or npm documents support for the immutable `sub`: drop the manual fallback.
- `gh-action-pypi-publish` pins its image by digest, or another publisher gives the same PEP 740 attestations with pinned dependencies.
- dist's installers check checksums on every platform (dist PR #2497) and accept checksums for archives dist didn't build: reconsider ADR-0010's scripts.
- cargo-binstall supports Sigstore bundles or GitHub attestations.
- winget-pkgs offers OIDC submission (winget-pkgs#413785).
- GitHub announces an end date for `macos-15-intel`, or its image stays stale: move to `macos-26-intel`.
- A second maintainer joins: turn on "Prevent self-review".
- Homebrew's bump tooling can update per-platform URLs: reconsider the tap-side sync workflow.
- The catalog nears 39 asset binaries: decide how to split a release.
- The publish jobs' decisions can move into `cargo xtask release` subcommands with unit tests (ADR-0010).
