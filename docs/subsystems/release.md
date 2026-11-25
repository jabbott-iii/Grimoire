# Subsystem: the release pipeline

The release pipeline turns a merged release pull request into a tagged, signed GitHub Release and
publishes the same binaries to crates.io, npm, PyPI, Homebrew, Scoop and winget. It is made of the
`cargo xtask release` commands in `xtask/src/release/`, which hold every decision about what a
release contains; the workflows `.github/workflows/release.yml`, `sign.yml` and `release-plz.yml`,
which run them and the external tools; `release-plz.toml`; and the hand-written inputs in
`packaging/`. It guarantees that every channel ships the same binary, that every release file has
a checksum, a provenance attestation and a keyless signature, that nothing is published until the
maintainer approves it twice, and that no publishing credential is stored anywhere. The records
that shaped it are [ADR-0003](../adr/0003-release-tooling.md) (release-plz for versions and the
changelog, plus a hand-written, SHA-pinned pipeline), [ADR-0009](../adr/0009-release-pipeline.md)
(the pipeline, its channels and the signing identity) and
[ADR-0010](../adr/0010-language-policy-exceptions.md) (the code in it that isn't Rust). The
container image is built in the same run; its design is in
[`container-image.md`](container-image.md). The maintainer's one-time setup, recovery, and the
user's install and verification steps are in [`05-release.md`](../05-release.md#release-checklist).
The facts behind this design are the Phase 5 entries in [`specs.md`](../specs.md) (`*-P5`), and
SIGN-P6 for the signing identity.

## Why a hand-written pipeline

The off-the-shelf choice for a Rust CLI is dist (formerly cargo-dist). It doesn't fit two of
Grimoire's requirements ([ADR-0003](../adr/0003-release-tooling.md)):
- its npm installer downloads archives at install time instead of shipping per-platform packages;
- its npm and Homebrew publishing depend on long-lived tokens, and Grimoire stores no publishing
  credential.

A spike (a short experiment) on dist's installers also failed: its PowerShell installer checks no
checksum, and its shell installer checks one only when dist built the archives
([ADR-0009](../adr/0009-release-pipeline.md)). So Grimoire writes its own pipeline, and keeps the
logic in Rust:
- `cargo xtask release` makes every decision (the mode, the version, the targets, the file names,
  the channel files) and is unit-tested. The YAML only orchestrates the external tools, so the
  release logic can be tested on every pull request without a release
  ([ADR-0010](../adr/0010-language-policy-exceptions.md)).
- release-plz does one job, the release pull request (the version bump and the changelog). It
  never tags or publishes, because the release workflow must tag the exact commit it built and
  signed, which release-plz can't see.

What it trades away:
- more code of Grimoire's own to maintain, including code that isn't Rust: the six
  language-policy exceptions in [ADR-0010](../adr/0010-language-policy-exceptions.md), and a
  seventh, the image's tag script, in [ADR-0011](../adr/0011-container-image.md);
- two approval rounds in every release, the price of keeping each registry's credential in its own
  job;
- the signing workflows' keyless steps and the publish jobs' shell run for the first time in a
  real release, because a pull request can't exercise them without writing permanent public
  records ("Residual risks", below).

## Terms

The terms this file and [`container-image.md`](container-image.md) use:
- **Workflows, jobs and runners.** GitHub Actions runs the repository's workflows. A workflow is
  made of jobs, and each job runs on a fresh machine, a runner. A reusable workflow is one that a
  job in another workflow calls (`on: workflow_call`). Each job gets a `GITHUB_TOKEN`, with the
  permissions the workflow grants it.
- **Environment.** A named deployment target on GitHub. It can require a reviewer's approval
  before a job that names it runs.
- **OIDC token.** A short-lived OpenID Connect (OIDC) token is GitHub's signed statement of which
  repository, workflow, ref and environment a job runs in. A job can request one only with the
  `id-token: write` permission. Getting a new token is often called minting it.
- **Trusted publishing.** A registry (crates.io, npm or PyPI) is configured to accept an OIDC
  token from one repository, workflow and environment, instead of a stored password or API token.
- **Keyless signing.** Signing through Sigstore, a public signing service, instead of with a
  long-lived private key. Fulcio, Sigstore's certificate authority, gives a certificate that is
  valid for a few minutes to whoever presents an OIDC token, and writes the token's identity into
  it. Rekor, Sigstore's public transparency log, keeps a public record of each keyless signature.
  cosign is Sigstore's signing tool, and a cosign bundle is the file that holds a signature with
  its certificate and log entry. "Public-good Sigstore" is the free public instance.
- **SAN and OIDs.** The SAN (Subject Alternative Name) is the certificate field that holds the
  signer's identity. For GitHub Actions it is the URL of the workflow file that ran, with its ref.
  Other fields record more about the run, each named by an object identifier (OID) under
  1.3.6.1.4.1.57264. This file gives only the last two numbers, so 1.5 means
  1.3.6.1.4.1.57264.1.5.
- **Attestations.** An attestation is a signed statement about a file, made here with GitHub's
  `actions/attest` and checked with `gh attestation verify` (gh is GitHub's command-line tool).
  Provenance is an attestation of how and where the file was built, in the SLSA format
  (Supply-chain Levels for Software Artifacts). An SBOM attestation carries a software bill of
  materials (SBOM), the list of components inside the file, here in the CycloneDX format.
- **Digests and tags.** A digest is the SHA-256 hash that identifies a file or an image exactly. A
  tag is a name, such as `v1` or `latest`, that its owner can move to other content. So a digest
  pin fixes the bytes, and a tag pin doesn't.
- **Dry-run and release mode.** A dry-run builds and checks everything but publishes nothing.
  Release mode tags, signs and publishes.

## The parts

| Part | What it does | Built in |
|---|---|---|
| `cargo xtask release plan` (`xtask/src/release/plan.rs`) | Chooses the mode (`dry-run`, `release` or `skip`), the version, the targets and the archive names; checks GitHub's limits, the `before-first-release` markers, the pack and `min-cli-version` | M5.1, M7.5 |
| `cargo xtask release build` (`build.rs`, `archive.rs`, `zip.rs`) | One `cargo auditable build` per package for one target, a `--version` smoke test, the archives | M5.1 |
| `cargo xtask release sbom` (`sbom.rs`) | A CycloneDX 1.6 SBOM per archive | M5.1 |
| `cargo xtask release channels` (`channels/`) | The npm launcher and platform packages, the wheels, the Homebrew formula, the Scoop and winget manifests, and the rendered install scripts, all from the CLI archives | M5.1, M5.2 |
| `cargo xtask release checksums` (`checksums.rs`) | Writes or checks `dist/SHA256SUMS` | M5.1 |
| `cargo xtask release verify` (`channels/mod.rs`) | Regenerates every channel file and checks that `dist/` holds exactly the release, with the same binary in every channel | M5.1 |
| `cargo xtask release notes` (`notes.rs`) | The GitHub Release text, from the version's `CHANGELOG.md` section, naming the image digest | M5.1 |
| `cargo xtask release image` (`image.rs`) | The container image's context, checks and gates ([`container-image.md`](container-image.md)) | M6.2 |
| `packaging/install/install.sh`, `install.ps1`, `packaging/npm/grimoire.js` | The install scripts and the npm launcher, language-policy exceptions | M5.2 |
| `packaging/homebrew-tap/`, `packaging/scoop-bucket/` | The sync workflows for the tap and bucket repositories, which pull attested files from each release | M5.4 |
| `.github/workflows/release.yml`, `sign.yml` | The dry-run on every pull request; the release from `main`; every signature and attestation of the release files | M5.3 |
| `.github/workflows/release-plz.yml`, `release-plz.toml` | The release pull request | M5.5 |

The commands' flags are listed in "Everyday commands" in [`AGENTS.md`](../../AGENTS.md).

## How a release flows

1. **Changes land on `main`** through pull requests, as usual.
2. **release-plz opens or updates the release pull request.**
   - release-plz is a release tool for Rust projects. Here its only job is the release pull
     request (release-plz calls it the release PR): one pull request that sets the next version
     and writes the changelog.
   - `.github/workflows/release-plz.yml` runs `release-plz release-pr` on every push to `main`.
   - A manual run works only on `main`: on another branch, release-plz would rebuild the release
     pull request from that branch.
   - Runs queue one at a time and are never canceled.
   - The pull request bumps the shared workspace version: the one version that the `grimoire`
     command-line interface (CLI) and its five library crates share. It adds a section for the new
     version to `CHANGELOG.md`, written from the Conventional Commits since the last release:
     commit messages that start with a type, such as `feat:` or `fix:`.
   - It is labeled `release`.
   - release-plz is configured never to tag, create releases or publish. The release workflow
     does those instead, so that it tags the exact commit it built and signed
     ([ADR-0009](../adr/0009-release-pipeline.md)). release-plz's token could still do the first
     two; "Residual risks", below, says what still protects users.
   - The repository's ruleset (the rules GitHub enforces on its branches) refuses release-plz's
     commit on its `release-plz-*` branches until M8.3 changes it
     ([`05-release.md`](../05-release.md#repository-settings)).
3. **Approve the release pull request's continuous integration (CI) runs.**
   - The pull request is opened with `GITHUB_TOKEN`, the token GitHub Actions gives each job. So
     its `pull_request` runs (CI, Security and the release dry-run) wait for approval.
   - Open the pull request's checks and approve the workflows to run, or push a commit to the
     branch yourself.
   - Do the same after every update release-plz makes to the pull request.
   - Running the checks with `workflow_dispatch` (a manual run) doesn't help: those checks don't
     count toward the ruleset's required status checks (the checks that must pass before a pull
     request can merge).
4. **Review and merge the release pull request.**
   - **Check the version.** The pull request's version must be the next 0.y, such as 0.1.4 →
     0.2.0, if either of these holds:
     - the new `CHANGELOG.md` section has a breaking entry, a change marked breaking
       ([`05-release.md`](../05-release.md#grimoires-version) lists what counts);
     - `rust-version` in the root `Cargo.toml`, the minimum supported Rust version (MSRV), rose
       since the last release's tag
       ([`05-release.md`](../05-release.md#the-minimum-supported-rust-version)).
   - If either holds and the version isn't the next 0.y, correct the version before you merge,
     through a pull request to `main`, not on the release pull request's branch
     ([`05-release.md`](../05-release.md#switch-releases-on-last) says why).
   - How release-plz bumps the shared version in Grimoire's setup is UNVERIFIED (VERSIONING-P7 in
     [`specs.md`](../specs.md)), so this review is the only check on it.
   - **Check that every advisory is named.**
     - A security fix merged from an advisory's temporary private fork (a private copy of the
       repository, which GitHub creates so that the fix for a reported vulnerability can be
       prepared out of public view) names the advisory's GHSA ID, its GitHub Security Advisory
       identifier, in the first line of its commit message ([`SECURITY.md`](../../SECURITY.md),
       "What happens to a report").
     - release-plz carries that line into the `CHANGELOG.md` section, and the release notes come
       from the section. Check that the section names each such advisory.
     - If one is missing, add it to the GitHub Release's notes once the release is published. An
       immutable release (one whose tag and files GitHub locks once it is published,
       [`05-release.md`](../05-release.md#repository-settings)) keeps its notes editable.
   - **Approve it before you merge.** The ruleset needs one approving review from a code owner,
     and the maintainer is the only one
     ([`05-release.md`](../05-release.md#repository-settings)). release-plz opens the pull
     request, not the maintainer, so the maintainer's approval meets the rule without the
     administrator bypass, the administrator role's permission to merge past the ruleset. A push
     that changes the pull request dismisses the approval, so approve it again after each update.
     The first release pull request, in M8.7, is the first time this runs on GitHub.
5. **The release run starts.**
   - The merge's push to `main` starts `.github/workflows/release.yml`.
   - **Release mode or dry-run.** The `plan` job chooses release mode, the mode that tags and
     publishes, only when the repository variable `GRIMOIRE_RELEASES` is `enabled`, `vX.Y.Z`
     isn't tagged, and `CHANGELOG.md` has a section for X.Y.Z.
     - Only the merged release pull request adds that section. Without the rule, once releases
       are switched on, any push to `main` would release the untagged version
       ([ADR-0009](../adr/0009-release-pipeline.md)).
     - Any other push to `main` stays a dry-run. When releases are enabled and `vX.Y.Z` isn't
       tagged, `plan` also prints a warning that release mode waits for the release pull request.
       Once the tag exists, `plan` returns `skip`, so a later push never releases the same
       version again.
   - **A stale catalog pack.** In release mode, `plan` fails at once while the checked-in
     `crates/agent-grimoire-catalog/catalog.pack` isn't exactly what `cargo xtask pack-catalog`
     makes from `catalog/`.
     - Why: the CLI embeds that file, while the asset binaries (the programs that the catalog's
       tools and Model Context Protocol servers ship) are built from `catalog/`. So a stale pack
       would ship an older catalog beside newer binaries.
     - CI's catalog job fails on a stale pack in every pull request and push to `main`, but
       nothing in the ruleset requires that job until M8.4
       ([`04-quality.md`](../04-quality.md#the-repository-and-contributions)). So a stale pack can
       still be merged; this check keeps it out of a release.
     - Fix it in a pull request to `main`: run `cargo xtask pack-catalog` and commit the result.
       Its merge starts the release again, from that commit
       ([`05-release.md`](../05-release.md#recovery), "The run failed before the `release`
       approval").
   - **An asset that needs a newer grimoire.** In release mode, `plan` also fails while an asset
     in the embedded catalog (`crates/agent-grimoire-catalog/catalog.pack`) has a
     `min-cli-version` above X.Y.Z. That field names the oldest CLI that can install the asset, so
     grimoire X.Y.Z would refuse it with `resolve/cli-too-old`.
     - CI's tests fail on such an asset in every pull request and push to `main`, but nothing in
       the ruleset requires them until M8.4, so this check keeps it out of a release.
     - Fix it in a pull request to `main`, whose merge starts the release again. Either lower the
       asset's `min-cli-version` (with an asset version bump and an entry in the asset's own
       `CHANGELOG.md`, then `cargo xtask pack-catalog`), or release a version at least that new
       through a release pull request (step 4 says how to correct its version).
     - See [`05-release.md`](../05-release.md#asset-versions), on asset versions.
   - **The `before-first-release` markers.** The run, and the release pull request's dry-run,
     fail while `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md` still has an
     opening `<!-- before-first-release: … -->` or closing `<!-- /before-first-release -->` marker
     line. These markers wrap text that must change before the first release. They are matched
     loosely, in any case and with any spacing around `<!--` (`is_marker` in
     `xtask/src/release/plan.rs`), and deleting only the opening line still fails. M8.6 clears
     them.
   - **GitHub's limits.** `plan` fails at once if the catalog has grown past GitHub's limits: 256
     jobs in the SBOM attestation matrix, and 1000 assets in one release.
     - The matrix is one job definition that GitHub expands into one job per archive. Each job
       attests the archive's software bill of materials (SBOM), the list of components it
       contains.
     - The 1000 assets are release assets, the files attached to one GitHub Release. With today's
       channels, that limit is reached at 39 asset binaries. Growing past it needs a new decision
       record on how to split the release ([ADR-0009](../adr/0009-release-pipeline.md)).
   - The run builds all six targets (Linux, macOS and Windows, each for x86-64 and arm64),
     packages every channel (each way to install Grimoire,
     [`05-release.md`](../05-release.md#installing)) and runs the verification jobs. Then
     `sign-files` signs every file (below).
   - Every release-mode job downloads the release files by artifact ID, the ID of the package
     job's upload within the run. Before it attests, signs or publishes anything, it checks
     `SHA256SUMS` against the hash the package job recorded, and that the files are exactly the
     ones `SHA256SUMS` lists. So an artifact of the same name that another job uploaded is never
     what gets signed.
   - **Signing.** Every signature and attestation is made in one of two reusable workflows
     (workflows that another workflow calls, much as a program calls a function):
     - `.github/workflows/sign.yml` for the release files, which only `sign-files` calls;
     - `.github/workflows/sign-image.yml` for the image, which only `sign-image` calls.
   - An attestation is a signed statement about a file, such as a record of how it was built.
     Signing is keyless: Sigstore, a public signing service, issues a short-lived certificate for
     each signing, and the certificate names the workflow that signed.
     - The two workflows' certificates name
       `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` and
       `…/sign-image.yml@refs/heads/main`, which no job in `release.yml` can get.
     - So the publish jobs, and the third-party container that the PyPI (Python Package Index)
       upload action runs, can't sign as the release.
     - The signing workflows' jobs run only when `release.yml` on `main` in this repository calls
       them.
   - **Why two files.** GitHub checks every job of a called workflow against the calling job's
     permissions when the run starts, even a job that its `if:` would skip. With one file for
     both, the files call didn't grant what the image job asks for, so GitHub refused to start any
     Release run (`SIGN-P6` in [`specs.md`](../specs.md)). The xtask test
     `every_reusable_workflow_call_grants_what_its_jobs_ask_for` checks every call's grant (xtask,
     in `xtask/`, is the repository's own build tooling).
   - `sign-files` makes:
     - one provenance attestation (a signed record of the run that built the files) over every
       file;
     - an SBOM attestation for each archive (one matrix job each);
     - a cosign bundle for each file. cosign is Sigstore's signing tool, and a bundle holds a
       signature with what is needed to verify it. `sign-files` verifies each bundle at once with
       the identity and the caller pins ("The signing identity", below, explains both).
   - **The container image.** The `image` job builds it from the two Linux musl archives (static
     Linux builds, linked with the musl C library), for linux/amd64 and linux/arm64. It writes the
     image as an OCI layout, a directory in the Open Container Initiative's image format, with an
     SBOM and provenance for each platform.
     - It verifies the base image's signatures first.
     - It then checks the layout with `cargo xtask release image check` and scans each platform
       with Grype, a vulnerability scanner. Any medium, high or critical match fails the job
       ([`container-image.md`](container-image.md)).
   - `verify-image` pushes the layout to a local registry, pulls it on an amd64 and an arm64
     runner (the machines GitHub runs jobs on), and runs every binary in it.
   - Pull requests run both jobs too. They also sign the image with a throwaway key, offline, and
     run the release's tag script, `.github/scripts/image-tags.sh`, against the local registry,
     including the cases it must refuse.
   - **Where the image digest comes from.** The `image` job prints the layout's index digest: the
     SHA-256 of the image index, the file that lists each platform's image.
     - `publish` writes it into the release notes in step 6, before anything is pushed.
     - `publish-image` pushes exactly that index in step 7, and `sign-image` signs it before it
       gets a tag.
     - `verify-image` and `publish-image` download the layout by artifact ID and check that it is
       the index the `image` job checked.
6. **Approve the first deployment, `release`.**
   - The `publish` job waits in the `release` environment. An environment is a gate in GitHub:
     with the required reviewer that the bootstrap sets up
     ([`05-release.md`](../05-release.md#repository-settings)), a job that names it waits until
     the maintainer approves. Check that every earlier job passed, then approve.
   - It verifies `SHA256SUMS`, every cosign bundle and the provenance of `SHA256SUMS`, each with
     `sign.yml`'s identity and the caller pins
     ([`05-release.md`](../05-release.md#verifying-a-download)).
   - It also checks fields that no flag of cosign or gh (GitHub's command-line tool) can pin: that
     the provenance certificate's Build Config URI is
     `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, the
     calling workflow, and that its repository and owner IDs are Grimoire's (`1460666977`,
     `84321953`). The Build Config URI (uniform resource identifier) is the certificate field
     that names the workflow file that started the run.
   - It then tags `vX.Y.Z` at the commit it built, and publishes the GitHub Release: it creates a
     draft, uploads the files, then publishes it. A tag that already exists at another commit
     fails the job.
   - It has no `id-token` permission, so it can't get a registry token. A job needs that
     permission to request an OpenID Connect (OIDC) token: GitHub's short-lived proof of which
     workflow and job is running, which registries accept instead of a stored token.
   - The notes name the container image by digest, `ghcr.io/jabbott-iii/grimoire@sha256:…`, with
     the tags `X.Y.Z` and `latest`, and say that it is pushed once the registry jobs are approved.
   - It then prints the release's `immutable` field, and only warns if it isn't `true`. Check
     that line before going on; [`05-release.md`](../05-release.md#recovery) says what to do if
     it is `false`.
7. **Approve the registry deployments, `crates-io`, `npm`, `pypi` and `ghcr`.**
   - Four jobs now wait at once, each in its own environment. Approve them together.
   - `publish-crates` gets a token with `crates-io-auth-action` and runs
     `cargo publish --locked -p <crate>` for each crate whose version isn't on crates.io (Rust's
     package registry) yet, in dependency order.
   - `publish-npm` stages the six platform tarballs, then the launcher.
     - npm is the JavaScript package registry. A staged version is uploaded, but it can't be
       installed until the maintainer approves it (step 8).
     - The launcher is the `agent-grimoire` package, which runs the binary from a platform
       package ([`05-release.md`](../05-release.md#installing)).
   - `publish-pypi` uploads the wheels (Python's package files) with
     `pypa/gh-action-pypi-publish` (`skip-existing: true`).
   - `publish-image` pushes the checked layout to `ghcr.io/jabbott-iii/grimoire` by digest,
     untagged, with the job's `GITHUB_TOKEN` (`image-tags.sh push`).
     - An existing `X.Y.Z` that names another digest stops the job; the tag is never overwritten.
     - It has no OIDC token and runs no cargo. [ADR-0011](../adr/0011-container-image.md) keeps
       cargo out of every job that can push or get an OIDC token, because a dependency's build
       script could use either.
   - **Then `sign-image` runs, with no approval of its own.** A job that calls a reusable
     workflow can't name an environment, so the `ghcr` approval of the push is the approved step.
     - It calls `sign-image.yml`, which signs the index digest with cosign (keyless) and adds a
       build provenance attestation with `actions/attest`.
     - It then checks that both are listed in the `sha256-<index hex>` tag, and verifies both.
       GHCR (the GitHub Container Registry) has no referrers API (the registry API that lists the
       signatures and attestations of an image), so they are listed under that tag instead.
   - Only then does the image get its tags: `X.Y.Z`, then `latest` (`image-tags.sh tag` and
     `latest`). So the release never gives either tag to an image before it is signed, attested
     and verified.
     - `latest` moves to this release unless it names a newer version.
     - It stays on a newer release that carries the image's release signature, and if the newer
       version doesn't verify, the job stops ([`05-release.md`](../05-release.md#recovery)).
   - `sign-image`'s last step fails while the package can't be pulled anonymously. That can
     happen only in the first release, before the maintainer makes the package public
     ([`05-release.md`](../05-release.md#during-the-first-release-run)).
   - **One environment per registry.** Each job's OIDC token names its own environment, and each
     registry's trusted publisher (its setting that names the repository, workflow and
     environment allowed to publish) accepts only its own. So no job can publish to another
     registry.
     - `publish-image` has no OIDC token.
     - `publish-crates`, `publish-npm` and `publish-pypi` can still get Sigstore certificates, but
       those name `release.yml`, and no verifier accepts that identity.
8. **Approve the npm versions with two-factor authentication (2FA), platform packages first.**
   - Approve the six `@agent-grimoire/*` versions, then `agent-grimoire`. Use
     `npm stage approve <stage-id>`, or the Staged Packages tab on npmjs.com.
   - Batch approval on npmjs.com takes up to 100 versions (one per package) with one 2FA
     challenge.
   - Each approval is independent and isn't rolled back if another fails, so approve the platform
     packages before the launcher.
   - npm scans every new version before it can be installed. That typically takes about
     5 minutes, sometimes 15 or more, so `npm i -g agent-grimoire` may fail for a while after
     approval.
9. **Submit the winget manifests by hand.**
   - winget is the Windows Package Manager. A manifest is the set of files that describes one
     version of a package; Microsoft collects them in the `microsoft/winget-pkgs` repository.
   - Fork `microsoft/winget-pkgs` (once).
   - Add the three YAML files from the release (YAML is a configuration file format) under
     `manifests/j/jabbott-iii/agent-grimoire/X.Y.Z/`.
   - Open one pull request per version. A first submission is titled
     `New package: jabbott-iii.agent-grimoire version X.Y.Z`.
   - Sign the Microsoft Contributor License Agreement (CLA) yourself the first time.
   - On a Windows machine, run `winget validate --manifest <dir>`, without `--ignore-warnings`,
     which would hide unknown fields. Validation needs no extra setting.
     - `winget --version` must be v1.28.190 or newer. Older clients don't know schema 1.28.0, so
       they warn on every manifest's schema header and exit with -1978335192
       (`MANIFEST_VALIDATION_WARNING`).
   - Then test the install.
     - Windows Sandbox, an isolated Windows desktop that is thrown away when you close it, is the
       simplest place: run `Tools\SandboxTest.ps1 <dir>` from your winget-pkgs fork.
     - Otherwise, `winget install --manifest <dir>` needs local manifests turned on first: run
       `winget settings --enable LocalManifestFiles` in an elevated shell (one run as
       administrator), and turn the setting off again afterward.
     - Group Policy (the settings an organization applies to the Windows machines it manages) can
       block that setting on a managed machine; use the Sandbox there.
   - winget-pkgs then runs its 10-step validation, and a moderator approves.
10. **Merge the tap and bucket sync pull requests.**
    - The tap, `jabbott-iii/homebrew-tap`, is the repository that Homebrew (a package manager for
      macOS and Linux) installs Grimoire from. The bucket, `jabbott-iii/scoop-bucket`, is the
      same for Scoop (a command-line installer for Windows).
    - In each, the sync workflow runs daily, or when you start it from the Actions tab. It
      downloads `agent-grimoire.rb` or `agent-grimoire.json` from the latest release, checks its
      attestation, and opens a pull request when the file differs from
      `Formula/agent-grimoire.rb` or `bucket/agent-grimoire.json`.
    - GitHub turns off scheduled workflows in a public repository after 60 days without
      activity. The tap and bucket get commits only when a sync pull request merges, so a quiet
      spell between releases can turn the schedule off.
    - If no sync pull request appears, enable the workflow again (Actions tab → "Enable
      workflow", or `gh workflow enable sync-formula.yml --repo jabbott-iii/homebrew-tap` and
      `gh workflow enable sync-manifest.yml --repo jabbott-iii/scoop-bucket`), then start it with
      `gh workflow run`.
    - Those pull requests are opened with each repository's `GITHUB_TOKEN`, so any checks they
      have wait for the maintainer's approval, as in step 3. Review and merge them.

## What a release holds

Every release file is generated at release time and never checked in. `release.yml` runs
`cargo --locked xtask release` (`plan`, `build`, `sbom`, `channels`, `checksums`, `verify`,
`notes`); `--locked` makes Cargo fail on a stale `Cargo.lock` instead of rewriting it.
- Every pull request runs all of them except `notes` as a dry-run. `notes` runs only in the
  release-mode `publish` job.
- The only checked-in inputs are the templates and the launcher in `packaging/`.
  [ADR-0010](../adr/0010-language-policy-exceptions.md) allows them to be in a language other
  than Rust, because they run before any Grimoire binary is on the machine, inside a tool that
  reads only its own language, or in a repository with no Rust toolchain.

**One build per package.** `build` runs `cargo auditable build -p <package>` (a build that also
embeds the list of compiled dependencies in the binary) for the CLI and for each asset binary,
with the `dist` profile ([`05-release.md`](../05-release.md#build-profiles)).
- So each binary has only the dependency features its own manifest asks for, as `cargo install`
  would give it, and its software bill of materials (SBOM, the list of components it was built
  from) matches it.
- One build of several packages would unify features: each dependency shared by packages in the
  build is built once, with the union of the Cargo features any of them asks for, so a binary can
  get features that only another package needs.
- Windows MSVC builds pass `-C target-feature=+crt-static`, so the binary needs no separate VC++
  runtime. The flag must be in the same `RUSTFLAGS` as `--remap-path-prefix` (which replaces
  build-machine paths inside the binary), because `RUSTFLAGS` replaces the config file's target
  `rustflags` (`rustflags` in `xtask/src/release/build.rs`).

**The files**, all flat in one directory, every name ASCII `[A-Za-z0-9._-]` because GitHub
renames anything else:
- the six CLI archives, one per target, `agent-grimoire-{V}-{triple}.tar.gz` or `.zip`.
  `{triple}` is the Rust target name, such as `x86_64-unknown-linux-musl`. Each holds the
  `grimoire` binary, both license files and `README.md`;
- one archive per asset binary and target, `{bin}-{assetV}-{triple}.tar.gz` or `.zip`, where
  `{assetV}` is the asset's own version, holding the binary, both licenses and the asset's
  `README.md`;
- a CycloneDX 1.6 SBOM per archive, `{archive}.cdx.json`;
- the npm launcher and six platform tarballs, and the wheels. Wheels are Python's binary
  packages, for PyPI, the Python Package Index;
- `agent-grimoire.rb` (Homebrew), `agent-grimoire.json` (Scoop) and the three winget manifests;
- the rendered `install.sh` and `install.ps1`, with the version and the CLI archives' SHA-256s
  embedded: the four Linux and macOS targets in `install.sh`, the two Windows targets in
  `install.ps1`;
- `SHA256SUMS`, with GNU untagged lines (`<hex>  <name>`) sorted by name, for every other file
  except the cosign bundles;
- in release mode only, a cosign bundle, `{file}.sigstore.json`, for every file, `SHA256SUMS`
  included.

[ADR-0009](../adr/0009-release-pipeline.md) has the full naming table, and
[`03-platforms.md`](../03-platforms.md#targets) the targets and their runners.

**Asset binaries are found from `cargo metadata --no-deps`:** every `bin` target of a package
under `catalog/tools/` or `catalog/mcp-servers/` (`load` in `xtask/src/release/meta.rs`).
- Nothing is listed by hand, so a new tool or MCP server is released with no core edit.
- This settles the check that [ADR-0002](../adr/0002-mcp-server-and-tool-packaging.md) (each MCP
  server and tool is its own binary) left for Phase 5: that the release can find asset binaries
  without a hand-edited list.

**Reproducible archives:** a fixed modification time (mtime, 1980-01-01T00:00:00Z), owner and
group IDs (uid and gid) 0, and sorted entries, so the same binaries always give the same
archives. The `.zip` writer is Grimoire's own (`xtask/src/release/zip.rs`), and both archive
readers refuse duplicate members, extra gzip members and overlapping zip entries.

## The trust chain

Each link is checked by the next:
1. **Source.** A commit on `main`.
   - The ruleset requires a pull request with one approval and a review by a code owner (an
     owner named in `.github/CODEOWNERS`). But the administrator role and some GitHub Apps
     (integrations installed on the repository) can bypass it
     ([`04-quality.md`](../04-quality.md#the-repository-and-contributions)).
   - release-plz opens the release pull request: it bumps the version and adds the version's
     section to `CHANGELOG.md`, and merging it starts the release. The ruleset refuses
     release-plz's commit on the `release-plz-*` branches until M8.3 changes it.
   - The CI and Security workflows and `release checks` (the release workflow's summary check)
     run on the release pull request, from runs the maintainer approves. The ruleset doesn't
     require them until M8.4 ([`05-release.md`](../05-release.md#repository-settings)), and the
     release run doesn't wait for CI or Security on its commit either. What holds a release back
     is the maintainer, who approves the release pull request's runs and reviews them before
     merging ("How a release flows", steps 3 and 4).
   - Release mode also needs the version's section in `CHANGELOG.md`, which only the merged
     release pull request adds.
   - That run, and the release pull request's dry-run, fail while a `before-first-release` marker
     remains (step 5 above).
   - A release-mode run also fails while `crates/agent-grimoire-catalog/catalog.pack` isn't what
     `cargo xtask pack-catalog` makes from `catalog/`, and while the pack holds an asset whose
     `min-cli-version` is above the version (step 5 above).
2. **Build.** GitHub-hosted runners, one native runner per target (a target is a platform the
   binary is built for, such as `x86_64-unknown-linux-musl`). Native builds need no
   cross-compiling toolchain, and the smoke test (a quick `--version` run) runs on the real
   operating system ([ADR-0009](../adr/0009-release-pipeline.md)). The build step has:
   - the pinned toolchain, `cargo --locked xtask`, and one `cargo auditable build --locked` per
     package (`--locked` builds with exactly the versions in `Cargo.lock`, and `cargo auditable`
     embeds the dependency list in the binary). So each binary has only the dependency features
     its own manifest asks for, and its SBOM describes it;
   - remapped build paths (the build machine's directory names are replaced in the binary);
   - no caches, so there is no cache to poison (to fill with tampered files that a later build
     would use);
   - artifacts (files one job uploads for later jobs) that move only within the run, and
     download-artifact checks their digests.
3. **Checksums.** `SHA256SUMS` lists every release file except itself and the cosign bundles.
   - `cargo xtask release verify` checks that every wheel (a Python package file for PyPI) and
     every npm platform package holds the same binary as its CLI archive. (Each npm platform
     package carries one platform's binary.)
   - Every release-mode job downloads the files by artifact ID. Before it attests, signs or
     publishes, it checks `SHA256SUMS` against the hash the package job recorded, and checks that
     the files are exactly the ones `SHA256SUMS` lists. So an artifact of the same name uploaded
     by another job is never what gets signed.
4. **Attestations.** SLSA provenance for every file except the cosign bundles, and a CycloneDX
   SBOM attestation for every archive, through `actions/attest` and public-good Sigstore, made in
   `sign.yml` (below).
5. **Signatures.** A keyless cosign bundle for every other file, `SHA256SUMS` included, under the
   identity `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`,
   called from `jabbott-iii/Grimoire` on `refs/heads/main`.
6. **First approval.** The `publish` job runs only after the maintainer approves it in the
   `release` environment.
   - It re-checks the checksums, every bundle and the provenance of `SHA256SUMS`, with that
     identity and the caller pins (the required values for the calling repository and ref,
     below). It also checks the provenance's Build Config URI (below).
   - Then it creates the tag and the GitHub Release.
   - It has no `id-token`, so it can't get a registry token.
7. **Immutable release, if the repository setting is on.** Once such a release is published, the
   tag and the assets are locked, and GitHub adds its own release attestation.
   - `GITHUB_TOKEN` can't read the setting, so `publish` reads the release's `immutable` field and
     only warns when it is false.
   - A release published while the setting is off stays mutable and has no release attestation.
     So the tap and bucket syncs (step 9) refuse it until it is deleted and published again
     ([`05-release.md`](../05-release.md#recovery)).
8. **Second approval, then the registries.** crates.io, npm and PyPI are published through OIDC
   trusted publishing, with no stored token:
   - each registry has its own job and its own environment (`crates-io`, `npm`, `pypi`), approved
     by the maintainer; each job has only `id-token: write` (and `contents: read` for the crates
     checkout);
   - npm versions are staged, and go live only after the maintainer approves each one with
     two-factor authentication (2FA);
   - npm should attach provenance to versions staged over OIDC. npm 11.19.1 turns it on for a
     public repository and package, but npm's docs don't say a staged version keeps it after
     approval (UNVERIFIED, `NPM-P5`);
   - PyPI adds PEP 740 attestations (PEP 740, a Python Enhancement Proposal, is the Python
     standard for attestations on uploaded files).
9. **Tap, bucket and winget.**
   - The Homebrew tap (`jabbott-iii/homebrew-tap`, a repository of install formulas) and the
     Scoop bucket (`jabbott-iii/scoop-bucket`, a repository of manifests) take a formula or
     manifest only after `gh attestation verify` passes, and only through a pull request the
     maintainer merges.
   - The maintainer submits the winget manifests, and winget-pkgs, Microsoft's repository of
     winget manifests, validates them.

## The signing identity

SIGN-P6 in [`specs.md`](../specs.md) holds the facts this rests on; the identity itself is the
maintainer's choice after the Phase 6 review ([ADR-0011](../adr/0011-container-image.md),
decision 6).
- **Why it moved.** A keyless certificate's identity names the workflow file that ran, not the
  job. So every job in `release.yml` with `id-token` used to get the same certificate identity,
  `…/release.yml@refs/heads/main`.
  - That included `publish-pypi`'s tag-pinned third-party container, which already gets Sigstore
    certificates for PEP 740 attestations. A re-pushed image of that action could have signed any
    file or image so that it passed the documented verification.
  - Now every signature and attestation of the release files is made in
    `.github/workflows/sign.yml` (`on: workflow_call`), which only `release.yml`'s `sign-files`
    calls.
  - The image's are made in `.github/workflows/sign-image.yml`, which only `sign-image` calls,
    under its own identity ([`container-image.md`](container-image.md#the-trust-chain)).
  - No release was ever signed under the old identity: the first release is signed by `sign.yml`,
    and its image by `sign-image.yml`.
- **Why two files.** GitHub checks every job of a called workflow against the calling job's
  permissions when the run starts, even a job its `if:` would skip, and refuses the whole run if
  one asks for more. With one file for both, the call that signs the release files would have to
  grant the image job's `contents: read` and `packages: write`. Each call now grants exactly what
  its jobs ask for. An xtask test checks every call's grant (`xtask/src/workflow_permissions.rs`),
  which neither zizmor nor actionlint (two linters for GitHub Actions workflows) does.
- **What the certificate says.** The identity (the SAN) names the called workflow:
  `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` for the
  release files, and `…/sign-image.yml@refs/heads/main` for the image. The caller is recorded in
  other fields: OID 1.5 (repository) `jabbott-iii/Grimoire`, 1.6 (ref) `refs/heads/main`, and
  1.18 (Build Config URI)
  `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`.
- **The publish jobs can't sign as the release.** `publish-crates`, `publish-npm` and
  `publish-pypi` still have `id-token`, so they, and the PyPI action's container, can still get
  certificates. Those name `…/release.yml@refs/heads/main`, which no verifier accepts any more.
  `publish-image` has no `id-token`.
- **External callers.** Any repository can call a public reusable workflow and get a certificate
  with the same identity; its own repository then appears in the caller fields (1.5 and 1.12).
  So:
  - every job in `sign.yml` and `sign-image.yml` runs only if
    `github.repository_id == '1460666977'` and
    `github.workflow_ref == 'jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main'`.
    An external caller, another workflow of this repository, or a pull request run never mints a
    token there;
  - every verifier also pins the caller: cosign with
    `--certificate-github-workflow-repository jabbott-iii/Grimoire --certificate-github-workflow-ref refs/heads/main`,
    gh with `--repo jabbott-iii/Grimoire --source-ref refs/heads/main`
    ([`05-release.md`](../05-release.md#verifying-a-download)).
- **The caller file.** Neither cosign nor gh has a flag for 1.18 (the caller file) or 1.23 (the
  environment).
  - So `publish` (for `SHA256SUMS`) and `sign-image.yml` (for the image) also read
    `gh attestation verify --format json` with `jq` (a command-line JSON processor). They require
    the provenance certificate's Build Config URI to be `…/release.yml@refs/heads/main`, and its
    repository and owner IDs to be `1460666977` and `84321953`. They check the IDs because, with
    `--bundle-from-oci`, gh itself compares names only.
  - `--signer-workflow` carries `@refs/heads/main`, so a signing workflow at a release tag, which
    any repository can call, doesn't pass.
- **How the signing workflows are written.**
  - Each has `permissions: {}`. A calling job can't set an environment, `runs-on`, `env` or
    outputs; it grants permissions, which the called jobs can only reduce.
  - Inputs reach run steps only through `env`, never written into the script itself, where an
    input could inject shell code (zizmor's template-injection audit rates that High).
  - Their issuer, repository and ref pins equal `release.yml`'s, as do `sign-image.yml`'s pins
    for the image and for crane (a tool for working with container registries), and each names
    its own identity. An xtask test checks them (`the_image_pins_agree_across_workflows`;
    [`05-release.md`](../05-release.md#maintaining-the-image-pins)).
  - The provenance `actions/attest` makes there names the signing workflow, `sign.yml` or
    `sign-image.yml`, as its builder (`runDetails.builder.id`), and `release.yml` as the workflow
    (`externalParameters.workflow.path`).

## What each channel checks on the user's machine

| Channel | Checks | Doesn't check |
|---|---|---|
| Direct download | What the user runs: `sha256sum -c`, `gh attestation verify`, `cosign verify-blob`, `gh release verify-asset` ([`05-release.md`](../05-release.md#verifying-a-download)) | Anything the user skips |
| Install scripts | An `https://` start URL. `install.sh` uses curl with `--proto =https`, so it also refuses redirects to anything but HTTPS; PowerShell 7.4 and later refuse HTTPS-to-HTTP redirects by default. The archive's SHA-256 against the value embedded in the script, failing closed | The script itself: verify it before running it. Signatures and attestations. HTTPS on redirects under Windows PowerShell 5.1 (UNVERIFIED) |
| `cargo install` | Builds from the crates.io source; Cargo checks each crate against the registry's checksum | Grimoire's signed binaries aren't used |
| `cargo binstall` (which downloads a prebuilt binary instead of building one) | The crates.io crate checksum; HTTPS for the archive | The archive's checksum, its cosign bundle and its attestations (`RUST-REGISTRY-P5`) |
| npm | The tarball's `dist.integrity` hash (the hash the registry lists for the tarball, the `.tgz` package file). In a project that installs `agent-grimoire` locally, `npm audit signatures` checks registry signatures and any provenance (`NPM-P5`) | `npm audit signatures` refuses global installs, so it can't check `npm i -g` or `npx`. cosign bundles and GitHub attestations |
| PyPI (pip, pipx, uv) | The file hash listed by the index (not re-checked in Phase 5) | Whether these installers check PEP 740 attestations wasn't researched (UNVERIFIED) |
| Homebrew | The SHA-256 in the formula, which came from an attested release file; Homebrew's tap trust (a formula from a third-party tap must be explicitly trusted) | Signatures and attestations at install time |
| Scoop | The SHA-256 in the manifest, which came from an attested release file | Signatures and attestations at install time |
| winget | `InstallerSha256` from the manifest; winget-pkgs' validation, including antivirus scans | Code signatures: the binary isn't Authenticode-signed (Authenticode is Windows' code-signing format) |

## The release switch

- No run publishes until the repository variable `GRIMOIRE_RELEASES` is `enabled`
  ([ADR-0009](../adr/0009-release-pipeline.md)). `plan` decides the mode: `dry-run` for a pull
  request and for any ref other than `refs/heads/main`; on `main`, `release` when the variable is
  `enabled`, the tag doesn't exist and `CHANGELOG.md` has the version's section, `skip` when the
  tag exists, and `dry-run` otherwise (`mode` in `xtask/src/release/plan.rs`).
- It exists for ordering, not security. GitHub creates a missing environment without protection
  rules, so no job may reference `release`, `crates-io`, `npm`, `pypi` or `ghcr` before the
  maintainer has created and protected it. The maintainer sets the variable last
  ([`05-release.md`](../05-release.md#switch-releases-on-last)).
- The variable approves nothing. The controls are the required reviewers on the five
  environments.
- As a second check, each registry's trusted publisher is configured for its own environment:
  crates.io for `crates-io`, npm for `npm`, PyPI for `pypi`. An OIDC token from any other job,
  including another publish job, names another environment and is refused. A crates.io
  configuration without an environment would accept any, so the bootstrap always sets one.
- `CARGO_REGISTRY_TOKEN` is never set to an empty string: release-plz reads an empty value as a
  token and skips trusted publishing, and what Cargo does with an empty token is unverified
  ([ADR-0009](../adr/0009-release-pipeline.md)).

**Re-runs.** Every publish job skips what is already done: the tag at this commit, a finished
draft or a published release with the same files, crate versions on crates.io, published npm
versions, and PyPI files. So re-running the failed jobs of a run usually finishes the release. A
staged npm version that isn't approved yet is the exception: CI can't see it, and npm refuses to
stage the same version twice, so `publish-npm` fails with a message to approve or reject the
earlier stage first. [`05-release.md`](../05-release.md#recovery) has the recovery steps.

## Residual risks

The risks that remain after the mitigations:
- **The PyPI publish action's image is pinned by tag, not digest.** The tag can be re-pushed and
  the base image is unpinned, so the code that uploads to PyPI can change under the same pin.
  - A container action gets the job's OIDC request token (the credential a job uses to ask GitHub
    for OIDC tokens) and can mint a token for any audience (the service a token is meant for). In
    `publish-pypi` that token names environment `pypi`, which the crates.io and npm publishers
    refuse, and the job holds only the wheels.
  - A Sigstore certificate it gets names `release.yml`, which no verifier accepts. So a re-pushed
    image can at worst publish to PyPI.
  - It also gets the run's `ACTIONS_RUNTIME_TOKEN` (a token that can upload artifacts to the
    run), but every job downloads artifacts by ID, so an artifact it uploads is never used.
- **One maintainer.** The same person writes, approves and publishes each release, so a
  compromised maintainer account defeats both approval rounds. npm's 2FA approval and immutable
  releases still apply, but they are the same person's controls. The same account can also change
  `main` without a second reviewer
  ([`04-quality.md`](../04-quality.md#the-repository-and-contributions)).
- **Signing comes before approval.** `sign-files` runs before the `release` approval. A run that
  is never approved still leaves Rekor entries and GitHub attestations for its files, under the
  `sign.yml` identity. Those files are never attached to a release. A valid signature shows only
  that a release run of `release.yml` on `main` signed the file; `gh release verify-asset` shows
  it belongs to a published release.
- **No flag pins the caller file or the environment.** cosign and gh can't check OID 1.18 or
  1.23. Another workflow of this repository on `main` that called a signing workflow would differ
  from a release only in 1.18. The signing workflows' guards keep such a caller from minting a
  token, and `publish` checks 1.18 for `SHA256SUMS`, but a user's verification commands can't
  tell. The signing jobs have no environment, so no signature shows that anyone approved
  anything.
- **The signing workflows' own steps first run in a real release.** Neither runs on a pull
  request: their callers run only in release mode, their guards skip every other caller, and pull
  requests from forks (copies of the repository under other accounts) get no OIDC token. So their
  keyless signing, `actions/attest` and the registry checks run for the first time in a release,
  as with the Phase 5 `attest` jobs that `sign-files` replaced. The dry-run signs offline with a
  throwaway key instead.
- **npm trusted publishing may fail** (npm/cli#9969, an npm bug, open when checked on
  2026-11-17, that can make trusted publishing fail with "package not found" for repositories
  whose OIDC tokens carry GitHub's immutable subject, as this one's do; another repository of the
  same kind publishes successfully, so it can work). Then the maintainer stages the signed
  tarballs by hand with a 2-hour session, and those versions have no npm provenance. npm users
  can still compare the tarball with the attested release asset.
- **Some pins protect only the download.** release-plz 0.4.3 and ShellCheck v0.11.0 publish no
  checksums or attestations, so their pins are GitHub asset digests (the SHA-256 GitHub records
  for a release file): they fix the bytes, not who built them.
- **release-plz's token can do more than open the release pull request.** It runs with the job's
  `GITHUB_TOKEN` (`contents: write`, `pull-requests: write`). Compromised release-plz bytes could
  push branches, create and delete tags, create, publish and upload to GitHub Releases (a newly
  published release becomes "latest" by default), and merge pull requests wherever the ruleset
  allows. What still protects users:
  - the job has no `id-token`, and the registries accept only tokens from their own
    environments;
  - the tap and bucket syncs check the provenance with `gh attestation verify`, signed by
    `sign.yml` with the caller pinned;
  - only cosign or `gh attestation verify` with `sign.yml`'s identity and the caller pins proves
    that a release run signed a file. `gh release verify-asset` alone doesn't, because GitHub
    attests any immutable release. Install-script users should check the script's bundle or
    attestation ([`05-release.md`](../05-release.md#installing)).
- **The publish jobs' shell runs only in real releases.** It reads `gh`, `curl` and `npm` answers
  to decide what is already published, and no pull request dry-run exercises it.
  [ADR-0010](../adr/0010-language-policy-exceptions.md) lists this shell code among the
  exceptions to the project's policy that code is written in Rust, and it fails closed on answers
  it doesn't recognize.
- **cargo-binstall users get only HTTPS for the archive.**
- **winget aliases aren't reserved.** Another package could later claim `agent-grimoire` as its
  alias (the command name winget puts on `PATH`), and winget would replace the link without a
  cross-package check.
- **Two targets are built only in release mode** (`x86_64-apple-darwin` and
  `aarch64-pc-windows-msvc`), so a problem specific to them shows up first in a release run,
  before the approval. Since Phase 6, pull requests also build `aarch64-unknown-linux-musl`, for
  the two-platform image.

## Invariants

1. A run is in release mode only on `refs/heads/main`, when `GRIMOIRE_RELEASES` is `enabled`, the
   version's tag doesn't exist, and `CHANGELOG.md` has the version's section. Every pull request
   is a dry-run, and a tagged version is skipped.
2. A release-mode run fails before it builds anything while a `before-first-release` marker
   remains, while the checked-in `catalog.pack` differs from what `pack-catalog` makes from
   `catalog/`, or while an embedded asset's `min-cli-version` is above the version being
   released. The release pull request's dry-run fails on a marker too.
3. `plan` fails when the release would need more than 256 SBOM attestation jobs or more than
   1000 release assets.
4. The same binaries always give byte-identical archives, and an archive with duplicate members,
   unsafe names, extra gzip members or shared zip data is refused.
5. Every wheel and every npm platform package holds the same binary as its CLI archive, and
   `dist/` holds exactly the files the plan expects, no more and no fewer.
6. `SHA256SUMS` lists every release file except itself and the cosign bundles, sorted by name.
7. Every SBOM is CycloneDX 1.6 with a serial number.
8. Every generated channel file is deterministic: generating twice gives the same bytes, and the
   install scripts have every placeholder filled, for every target.
9. The release notes are exactly the version's `CHANGELOG.md` section, and `notes` fails without
   one.
10. Every reusable-workflow call grants at least what each job of the called workflow asks for,
    so GitHub can start the run.
11. Only `sign.yml` and `sign-image.yml` sign or attest, and their jobs run only when
    `release.yml` on `main` in this repository calls them.
12. No publish job runs before the maintainer approves its environment, and each registry's
    credential exists only in that registry's own job.
13. The published crates carry license files identical to the root ones, and the cargo-binstall
    metadata names the release's archives.

## Tests

| Invariant | Checked by |
|---|---|
| 1 | `modes` and `outputs_for_each_mode` in `xtask/src/release/plan.rs` |
| 2 | `markers_block_the_release_pr_and_the_release`, `a_marked_span_fails_until_both_markers_are_gone`, `a_release_refuses_a_stale_pack`, `only_a_run_that_still_releases_checks_the_pack`, `the_checks_read_the_checked_in_pack` and `a_release_refuses_an_asset_that_needs_a_newer_grimoire` in `plan.rs` |
| 3 | `growth_limits` in `plan.rs` |
| 4 | `both_formats_round_trip_sorted_and_deterministic`, `only_canonical_archives_are_accepted_where_it_matters` and `tarballs_are_flat_and_unique` in `archive.rs`; `round_trip_keeps_order_modes_and_bytes`, `rejects_unsafe_names_and_corruption` and `rejects_duplicate_names_and_shared_data` in `zip.rs`; `archives_hold_the_binary_licences_and_readme` in `build.rs` |
| 5 | `verify_accepts_a_complete_dry_run_and_rejects_tampering` and `partial_generation_marks_missing_targets` in `channels/mod.rs`; the `package` job's `cargo xtask release verify` on every pull request |
| 6 | `sums_skip_bundles_and_themselves_and_detect_changes` in `checksums.rs` |
| 7 | `sboms_must_be_cyclonedx_1_6_with_a_serial_number` in `sbom.rs` |
| 8 | `generation_covers_every_channel_and_is_deterministic` in `channels/mod.rs`; `placeholders_are_filled_or_refused` and `the_templates_cover_every_target` in `channels/installers.rs`; the insta snapshots in `channels/homebrew.rs` (`formula`), `scoop.rs` (`manifest`), `npm.rs` (`launcher_and_platform_packages`), `wheel.rs` (`wheels_have_the_binary_metadata_and_a_correct_record`) and `winget.rs` (`manifests_and_json_twins`); `the_launcher_must_know_every_platform`, `versions_map_to_pep_440` and `scalars_are_quoted_when_yaml_would_reinterpret_them` |
| 9 | `takes_the_versions_section_only` in `notes.rs` |
| 10 | `every_reusable_workflow_call_grants_what_its_jobs_ask_for` in `xtask/src/workflow_permissions.rs`, with its unit cases (`refuses_a_called_job_that_asks_for_more_than_the_call_grants`, `accepts_a_call_that_grants_every_job_what_it_asks_for`, `the_called_workflows_top_level_permissions_count`, `a_read_grant_doesnt_cover_write`, `calls_it_cant_check_are_problems`, `shapes_it_doesnt_know_are_errors`) |
| 11 | `the_image_pins_agree_across_workflows` in `image.rs` keeps the signing pins equal across the workflows. The guards themselves have no test, and their keyless steps first run in M8.7 |
| 12 | No automated test: the environments' required reviewers are repository settings ([`05-release.md`](../05-release.md#repository-settings)), first exercised in M8.7 |
| 13 | `published_crates_carry_the_licence_files` and `binstall_metadata_names_the_release_archives` in `xtask/src/release/mod.rs` |

Other unit tests in `xtask/src/release/`: `finds_version_dependency_order_and_asset_bins` and
`the_real_workspace_parses` (`meta.rs`), `names_are_stable_and_ascii` and
`dry_run_targets_and_unique_npm_packages` (`targets.rs`),
`rustflags_remap_paths_and_link_the_crt_statically_on_windows` and
`musl_binaries_run_on_glibc_hosts_of_the_same_architecture` (`build.rs`).

**The dry-run on every pull request** (`release.yml`) runs the pipeline as far as it can without
publishing. It ends with `release checks` (`release-checks`), which fails unless every dry-run job
passed, because a required check whose job is skipped would count as passed:
- `plan`, with the limit and marker checks;
- `build`, which builds, archives and smoke-tests the four pull request targets
  (`x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`, `aarch64-apple-darwin`,
  `x86_64-pc-windows-msvc`);
- `package channels and checksums` (`package`): the SBOMs, every channel file, the npm tarballs,
  `SHA256SUMS`, and `cargo xtask release verify`;
- `verify install` (`verify-install`), on each target: `install.sh` and `install.ps1` against a
  local HTTP server, including a tampered archive they must refuse; the npm launcher with its
  platform tarball, and the wheel in a Python virtual environment, each running
  `grimoire --version`; and, on Windows x64, `winget validate` with a pinned winget-cli
  v1.29.380;
- `verify packages and manifests` (`verify-linux`): `cargo publish --workspace --dry-run --locked`,
  `twine check --strict`, ShellCheck and PSScriptAnalyzer (linters for shell and PowerShell
  scripts), `Test-Json` against the pinned Scoop and winget schemas, `brew style` and `brew audit`
  in a local tap, and `npm stage publish --dry-run` against an unreachable local registry
  (`http://127.0.0.1:9/`), so a pull request between releases still exercises npm's checks;
- `sign (dry-run, offline)` (`sign-dry-run`): signs and verifies every file with cosign and a
  throwaway key, with a signing config that names no services, so nothing reaches Rekor;
- the image jobs, in [`container-image.md`](container-image.md#tests).

What the dry-run can't check, and so first runs in M8.7: attestations, keyless signing and Rekor;
any real OIDC exchange; the publish jobs (the tag, `notes`, the GitHub Release, the registry
writes, the environment approvals and immutable-release behavior); the two targets built only in
release mode; the tap and bucket syncs; winget-pkgs validation; and npm's publish-time malware
scan ([ADR-0009](../adr/0009-release-pipeline.md)).
