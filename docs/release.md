# Releasing and installing Grimoire

- **Status:** written in Phase 5 (2026-11-17), before the first release, and revised after the Phase 5 review. **No release exists yet**, so no command in this file has been run against a real release.
- **Decisions:** [ADR-0009](adr/0009-release-pipeline.md) (the pipeline and channels) and [ADR-0010](adr/0010-language-policy-exceptions.md) (the code that isn't Rust).
- **Facts:** the `*-P5` entries in [`specs.md`](specs.md).

This file has five parts:
1. how a release flows, for the maintainer;
2. the one-time bootstrap, also for the maintainer;
3. recovering from a failed release;
4. installing, for users;
5. verifying a download, for users.

## 1. How a release flows

1. **Changes land on `main`** through pull requests, as usual.
2. **release-plz opens or updates the release PR.**
   - `.github/workflows/release-plz.yml` runs `release-plz release-pr` on every push to `main`. A manual run works only on `main`: on another branch, release-plz would rebuild the release PR from that branch. Runs queue one at a time and are never cancelled.
   - The PR bumps the shared workspace version and adds a section for the new version to `CHANGELOG.md`, from the Conventional Commits since the last release. It is labelled `release`.
   - release-plz is configured never to tag, create releases or publish ([ADR-0009](adr/0009-release-pipeline.md)). Its token could do the first two; [`security-model.md`](security-model.md) §6b says what still protects users.
3. **Approve the release PR's CI runs.**
   - The PR is opened with `GITHUB_TOKEN`, so its `pull_request` runs (CI, Security and the release dry-run) wait for approval. Open the PR's checks and approve the workflows to run, or push a commit to the branch yourself.
   - Do the same after every update release-plz makes to the PR.
   - Running the checks with `workflow_dispatch` doesn't help: those checks don't count toward the ruleset's required status checks.
4. **Review and merge the release PR.**
5. **The release run starts.**
   - The merge's push to `main` starts `.github/workflows/release.yml`.
   - `plan` chooses release mode only when `GRIMOIRE_RELEASES` is `enabled`, `vX.Y.Z` isn't tagged, and `CHANGELOG.md` has a section for X.Y.Z. Only the merged release PR adds that section. Any other push to `main` stays a dry-run, and `plan` prints a warning that release mode waits for the release PR.
   - `plan` fails at once if the catalog has grown past GitHub's limits: 256 jobs in the SBOM attestation matrix, and 1000 assets in one release. With today's channels, the asset limit is reached at 39 asset binaries.
   - The run builds all six targets, packages every channel, runs the verification jobs, then attests and signs every file.
   - Every release-mode job downloads the release files by artifact ID. Before it attests, signs or publishes anything, it checks `SHA256SUMS` against the hash the package job recorded, and that the files are exactly the ones `SHA256SUMS` lists.
6. **Approve the first deployment, `release`.**
   - The `publish` job waits in the `release` environment. Check that every earlier job passed, then approve.
   - It verifies `SHA256SUMS`, the cosign bundles and the attestation, tags `vX.Y.Z` at the commit it built, and publishes the GitHub Release (draft, upload, publish). It has no `id-token` permission.
   - It then prints the release's `immutable` field, and only warns if it isn't `true`. Check that line before going on; §3 says what to do if it is `false`.
7. **Approve the registry deployments, `crates-io`, `npm` and `pypi`.**
   - Three jobs now wait at once, each in its own environment. Approve them together.
   - `publish-crates` gets a token with `crates-io-auth-action` and runs `cargo publish --locked -p <crate>` for each crate whose version isn't on crates.io yet, in dependency order.
   - `publish-npm` stages the six platform tarballs, then the launcher.
   - `publish-pypi` uploads the wheels with `pypa/gh-action-pypi-publish` (`skip-existing: true`).
   - Each job's OIDC token names its own environment, and each registry's trusted publisher accepts only its own, so no job can publish to another registry.
8. **Approve the npm versions with 2FA, platform packages first.**
   - Approve the six `@agent-grimoire/*` versions, then `agent-grimoire`. Use `npm stage approve <stage-id>`, or the Staged Packages tab on npmjs.com.
   - Batch approval on npmjs.com takes up to 100 versions (one per package) with one 2FA challenge. Each approval is independent and isn't rolled back if another fails, so approve the platform packages before the launcher.
   - npm scans every new version before it can be installed. That typically takes about 5 minutes, sometimes 15 or more, so `npm i -g agent-grimoire` may fail for a while after approval.
9. **Submit the winget manifests by hand.**
   - Fork `microsoft/winget-pkgs` (once).
   - Add the three YAML files from the release under `manifests/j/jabbott-iii/agent-grimoire/X.Y.Z/`.
   - Open one PR per version. A first submission is titled `New package: jabbott-iii.agent-grimoire version X.Y.Z`.
   - Sign the Microsoft CLA yourself the first time.
   - On a Windows machine, run `winget validate --manifest <dir>`, without `--ignore-warnings`, which would hide unknown fields. Validation needs no extra setting.
   - Then test the install. Windows Sandbox is the simplest place: run `Tools\SandboxTest.ps1 <dir>` from your winget-pkgs fork. Otherwise, `winget install --manifest <dir>` needs local manifests turned on first: run `winget settings --enable LocalManifestFiles` in an elevated shell, and turn the setting off again afterwards. Group Policy can block that setting on a managed machine; use the Sandbox there.
   - winget-pkgs then runs its 10-step validation, and a moderator approves.
10. **Merge the tap and bucket sync PRs.**
    - In `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`, the sync workflow runs daily, or when you start it from the Actions tab. It downloads `agent-grimoire.rb` or `agent-grimoire.json` from the latest release, checks its attestation, and opens a PR when the file differs from `Formula/agent-grimoire.rb` or `bucket/agent-grimoire.json`.
    - GitHub turns off scheduled workflows in a public repository after 60 days without activity. The tap and bucket get commits only when a sync PR merges, so a quiet spell between releases can turn the schedule off. If no sync PR appears, enable the workflow again (Actions tab → "Enable workflow", or `gh workflow enable sync-formula.yml --repo jabbott-iii/homebrew-tap` and `gh workflow enable sync-manifest.yml --repo jabbott-iii/scoop-bucket`), then start it with `gh workflow run`.
    - Those PRs are opened with each repository's `GITHUB_TOKEN`, so any checks they have wait for your approval, as in step 3. Review and merge them.

## 2. One-time bootstrap (maintainer)

This checklist is the source for the Phase 5 bootstrap; `PLAN.md` §6 points here. Do the parts in order:
- **A:** the first item before you merge the Phase 5 change, the rest at any time before the first release;
- **B:** close to the first release, because PyPI's pending publisher expires after 30 days;
- **C:** last, after the README rewrite;
- **D:** during the first release run.

Never store a token in GitHub at any step.

### A. Repository settings

- [ ] **Before you merge the Phase 5 change, allow GitHub Actions to create and approve pull requests** (Settings → Actions → General).
  - `release-plz.yml` runs on the next push to `main` and opens the release PR (`chore: release`).
  - With the setting off, each run creates a `release-plz-<timestamp>` branch and then fails to open the PR. Delete any `release-plz-*` branch that has no PR.
  - Leave the release PR unmerged until the rest of this bootstrap is done. release-plz updates it on every push.
  - Keep the default `GITHUB_TOKEN` permissions read-only (`PLAN.md` §6).
- [ ] **Create four environments** (Settings → Environments): `release`, `crates-io`, `npm` and `pypi`. Configure each one the same way:
  - Required reviewer: you.
  - Leave **Prevent self-review** off while you are the only reviewer. With it on, you can't approve a run you started; only the administrator bypass could start it.
  - Deployment branches and tags: "Selected branches and tags", with one branch rule, `main`. Releases run only from `main`, never from a tag.
  - No environment secrets.
  - If an environment already exists, configure it the same way. A run that references a missing environment creates it without protection rules.
  - Each release then needs two approval rounds: `release` for the tag and the GitHub Release, then `crates-io`, `npm` and `pypi` together.
- [ ] **Enable immutable releases** (Settings → Releases → "Enable release immutability").
  - It applies only to releases published afterwards.
  - `GITHUB_TOKEN` can't read this setting, so the `publish` job only warns when a release comes out mutable. Check it with your own login: `gh api repos/jabbott-iii/Grimoire/immutable-releases` should show `"enabled": true`.
  - A mutable release gets no release attestation, so the tap and bucket syncs can't take it (§3).
- [ ] **The CI ruleset on `main`** (`PLAN.md` §6): require pull requests and the CI, Security and `release checks` checks. `release checks` is the release workflow's summary job. On a pull request it fails unless every dry-run job passed; a skipped job counts as a failure, because a required check whose job is skipped would otherwise count as passed. The release PR's checks then come from the runs you approve.
- [ ] **Enable private vulnerability reporting** before the first release ([`security-model.md`](security-model.md) §8).

### B. Registries and repositories, close to the first release

**npm.** The npm research left several points UNVERIFIED; they are marked below. Trusted publishing is configured later, in part D, because a new configuration expires after 48 hours.

- [ ] Sign in with `npm login`. It gives a 2-hour session and enforces 2FA for publishing. `npm stage` and `npm trust` need npm 11.15.0 or later.
- [ ] Create the npm organization `agent-grimoire` on npmjs.com. The `@agent-grimoire/*` packages need it. The name was free on 2026-11-17.
- [ ] **Create each of the seven package names by staging a bootstrap version.** The names are `agent-grimoire` and `@agent-grimoire/{linux-x64,linux-arm64,darwin-x64,darwin-arm64,win32-x64,win32-arm64}`.
  - Staged publishing can create a package; trusted publishing can't.
  - For each name, make a directory with a minimal `package.json`:
    - the name;
    - a version CI will never use, such as `0.0.0-bootstrap.0`;
    - a description;
    - `"license": "MIT OR Apache-2.0"`;
    - `"repository": {"type": "git", "url": "git+https://github.com/jabbott-iii/Grimoire.git"}`.
  - Run `npm stage publish --access public --tag bootstrap` in it.
  - npm creates the package and publishes a public placeholder version, `0.0.0-stage`.
  - Use a separate version because staged and published versions share one version index. A prerelease version needs `--tag`.
  - Then approve or reject the bootstrap stage (2FA).
- [ ] **Check on npmjs.com before going on (UNVERIFIED in the docs):**
  - which dist-tag the `0.0.0-stage` placeholder has. If it is `latest`, `npm i -g agent-grimoire` installs the placeholder until the first real version is approved;
  - whether rejecting the bootstrap stage keeps the package and its placeholder;
  - whether staged versions expire.
- [ ] Set each package's publishing access to "Require two-factor authentication and disallow tokens". Trusted publishing isn't affected.

**PyPI.**

- [ ] Add a pending publisher on pypi.org:
  - project `agent-grimoire`;
  - owner `jabbott-iii`;
  - repository `Grimoire` (PyPI matches it exactly);
  - workflow `release.yml`;
  - environment `pypi`.
- [ ] Create it close to the first release. It doesn't reserve the name, and PyPI deletes pending publishers 30 days after they are created. That rule is in PyPI's source, not its docs.

**Homebrew and Scoop.**

- [ ] Create the public repositories `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`.
- [ ] Copy `packaging/homebrew-tap/sync-formula.yml` and `packaging/scoop-bucket/sync-manifest.yml` into `.github/workflows/` in the matching repository. They write `Formula/agent-grimoire.rb` and `bucket/agent-grimoire.json`. Until the first Grimoire release, each run ends with a notice that there is nothing to sync.
- [ ] In each repository:
  - allow GitHub Actions to create and approve pull requests;
  - keep `GITHUB_TOKEN` read-only by default;
  - require actions pinned to a full commit SHA, as here.

The READMEs next to the two templates ([tap](../packaging/homebrew-tap/README.md), [bucket](../packaging/scoop-bucket/README.md)) have the full steps.

**crates.io.**

- [ ] Have a crates.io account with a verified email address, linked to your GitHub account. You need both to create trusted-publisher configurations.

### C. Switch releases on (last)

- [ ] **Land the Phase 7 README rewrite first.** The root `README.md` is still a one-line placeholder. It goes into every CLI archive, the six crates on crates.io and the npm packages, and it is the PyPI long description. A published version can't be changed.
- [ ] Set the repository variable `GRIMOIRE_RELEASES` to `enabled` (Settings → Secrets and variables → Actions → Variables).
- From then on, a push to `main` runs in release mode when its version isn't tagged and `CHANGELOG.md` has a section for it. Other pushes, Dependabot merges included, stay dry-runs.
- The first release starts when you merge the release PR. Before you merge it, check that it is current (release-plz updates it on each push) and that its checks passed.

### D. During the first release run

Trusted publishing can't create a crate, so you publish the first version of each crate yourself, from the commit the run tagged. npm's trusted publishing is set up here too, so that its 48-hour window starts as late as possible.

1. **Wait for the `release` approval.** Check that the run's commit (the `GITHUB_SHA` shown in the run) is the release PR's merge commit, and that every earlier job passed. If not, reject it.
2. **Approve `release`.** The `publish` job tags `vX.Y.Z` and publishes the GitHub Release. Check that its log says the release is immutable (§3 if not).
3. **The run now waits for `crates-io`, `npm` and `pypi`.** Don't approve yet.
4. **Check out the tag `vX.Y.Z`** in a clean clone, with the pinned toolchain from `rust-toolchain.toml`.
5. **Create a crates.io API token** that expires soon and can only publish these crates. Which scopes crates.io offers was not re-checked in Phase 5.
6. **Publish the six crates:** `cargo publish --workspace --locked`, with the token in `CARGO_REGISTRY_TOKEN` for that one command only.
   - Asset crates, `agent-grimoire-mcp`, the testkit and xtask are skipped because they are `publish = false`.
   - The command isn't atomic, and it refuses versions that already exist. If it stops partway, publish the rest one at a time in dependency order: `cargo publish --locked -p <crate>` for `agent-grimoire-model`, `-catalog`, `-resolve`, `-install`, `-adapters`, then `agent-grimoire`.
7. **Revoke the token.**
8. **Add a trusted publisher to each crate** on crates.io: owner `jabbott-iii`, repository `Grimoire`, workflow `release.yml`, environment `crates-io`. Always set the environment: a configuration without one accepts a token from any environment.
9. **Turn on trusted-publishing-only for each crate** (`trustpub_only`), in the crate's settings or with `PATCH /api/v1/crates/{name}`. API tokens then can't publish it.
10. **Configure npm trusted publishing for each package.** Sign in with `npm login`, then for each of the seven packages:
    ```sh
    npm trust github <package> --file release.yml --repo jabbott-iii/Grimoire --env npm --allow-stage-publish --yes
    ```
    - Every field is case-sensitive, so type `Grimoire` with the capital.
    - Leave direct publishing and dist-tag access off.
    - A new configuration must complete a successful publish within 48 hours, or it expires and must be deleted and created again. Whether a staged (not yet approved) version counts is UNVERIFIED, so do steps 11 and 12 within 48 hours of this one.
11. **Approve `crates-io`, `npm` and `pypi`.** The crates.io job finds all six versions published and skips them. The npm job stages the seven packages, and the PyPI job uploads the wheels and creates the project.
12. **Approve the npm versions** (§1 step 8).

## 3. Recovery

- **A job failed.**
  - Re-run the failed jobs of the same run (Actions → the run → Re-run failed jobs). GitHub allows that for 30 days after the run, and a re-run uses the run's own commit.
  - Each job skips what is already done:
    - `publish` accepts the tag at this commit, finishes a draft release, and checks that a published release holds exactly the files in `dist/`;
    - `publish-crates` skips crate versions already on crates.io;
    - `publish-npm` skips versions already published (staged ones are below);
    - `publish-pypi` skips files already on PyPI (`skip-existing`).
  - The registry jobs are separate, so a failure in one doesn't hold up the others. GitHub also re-runs the jobs that depend on a re-run job. Whether a re-run job waits for its environment's approval again isn't documented; expect it to.
  - Don't wait for a new run instead. Once the tag exists, a run on `main` only skips.
- **`publish-npm` failed after staging some versions.**
  - The job can't see versions that are staged but not approved, because `npm view` reads only published ones. npm refuses to stage the same version twice. So a re-run stops with an error that says to approve or reject the earlier stage first.
  - On npmjs.com (Staged Packages), approve or reject every staged X.Y.Z, platform packages first. A rejected version can be staged again.
  - For approved versions, wait until `npm view <package>@X.Y.Z version` prints the version for each one; npm's scan can take 15 minutes or more. Then re-run `publish-npm`.
- **The run failed before the `release` approval.**
  - Nothing was published. Fix it on `main` through a normal PR. The next push builds the fixed commit for the same version, because the tag doesn't exist yet.
  - The failed run's signatures and attestations stay in the public logs, but its files were never released.
- **The release isn't immutable** (`publish` warned `release vX.Y.Z is not immutable`).
  - GitHub created no release attestation, so the tap and bucket syncs can't take this version. Enabling the setting later doesn't change a release that is already published.
  - Before you approve the registry round, reject it. Enable immutable releases (§2 A). Delete the release but keep the tag: `gh release delete vX.Y.Z --repo jabbott-iii/Grimoire`, without `--cleanup-tag`.
  - Then re-run the `publish` job (Actions → the run → the job → Re-run). It finds the tag at the same commit and no release, so it publishes the release again, now immutable. GitHub re-runs the registry jobs after it; approve them.
  - This path hasn't been tried.
- **A re-run can't finish the release.** That happens when the run is more than 30 days old, or when the fix needs a change to the code or the workflow, because a re-run uses the run's commit.
  - Check `gh release view vX.Y.Z --repo jabbott-iii/Grimoire`.
  - **If the release is missing or still a draft, and no registry has X.Y.Z** (crates.io; npm, published or staged; PyPI):
    1. Delete the draft release: `gh release delete vX.Y.Z --repo jabbott-iii/Grimoire`.
    2. Delete the tag: `gh api -X DELETE repos/jabbott-iii/Grimoire/git/refs/tags/vX.Y.Z`. Nothing locks a tag or a draft before the release is published.
    3. Fix the cause on `main` through a PR.
    4. The next push to `main`, or a manual run, releases X.Y.Z from the fixed commit.
  - **If any registry already has X.Y.Z,** never reuse the version. Release a new patch version.
- **npm trusted publishing failed** (npm/cli#9969, "package not found"):
  1. Download the seven `.tgz` files from the GitHub Release and verify them (§5).
  2. Run `npm login`.
  3. Check which versions are already there: `npm stage list` (an interactive session can list stages) and `npm view <package>@X.Y.Z version`. Skip every package that has a staged or published X.Y.Z.
  4. Run `npm stage publish <file>.tgz --access public --ignore-scripts` for the remaining platform tarballs, then for the launcher.
  5. Approve them in the same order.
  - These versions have no npm provenance. No token is stored at any point.
- **An npm trusted-publisher configuration expired.** It fails with "package not found" too. An expired configuration can't be edited: delete and create it again (§2 D step 10), then re-run `publish-npm`. That keeps the OIDC path, unlike the fallback above.
- **A release turns out to be wrong after it was published.**
  - Release a new patch version.
  - Never delete or move a published tag, and don't try to reuse its version. Immutable releases lock the tag and the assets, and the tag name can never be reused.

## 4. Installing (users)

Install `agent-grimoire`, run `grimoire`. The only exception is winget, where the command is `agent-grimoire` (below). Every channel installs the same binary.

| Channel | Install | Notes |
|---|---|---|
| crates.io, from source | `cargo install agent-grimoire --locked` | Builds on your machine |
| crates.io, prebuilt | `cargo binstall agent-grimoire` | Downloads the release archive. It checks only HTTPS and the crate's checksum, not the archive (§5) |
| npm | `npm install -g agent-grimoire` | Needs Node 18 or later. The launcher runs the binary from a platform package |
| npm, without installing | `npx agent-grimoire --version` | CI tests only `npx grimoire` in a project that already has the package installed |
| PyPI | `pipx install agent-grimoire` or `uv tool install agent-grimoire` | The wheel holds the binary and no Python code. CI tests only `pip install` in a venv |
| Homebrew | `brew install jabbott-iii/tap/agent-grimoire` | The fully qualified name trusts just this formula (Homebrew 6+ requires trust for third-party taps) |
| Scoop | `scoop bucket add jabbott-iii https://github.com/jabbott-iii/scoop-bucket`, then `scoop install jabbott-iii/agent-grimoire` | Needs git |
| winget | `winget install jabbott-iii.agent-grimoire` | **The command is `agent-grimoire`.** `grimoire` belongs to another winget package. Open a new shell after installing |
| Install script | below | Linux, macOS and Windows |

To use the short name with Homebrew instead:
```sh
brew tap jabbott-iii/tap
brew trust --formula jabbott-iii/tap/agent-grimoire
brew install agent-grimoire
```

### Install scripts

On Linux and macOS (`install.sh` needs curl; on Alpine, `apk add curl` first):
```sh
curl --proto '=https' --tlsv1.2 -fsSLO https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.sh
# Verify it (below) and read it, then:
sh install.sh                   # installs to ~/.local/bin
sh install.sh --to ~/bin        # or set GRIMOIRE_INSTALL_DIR
```

On Windows:
```powershell
# Windows PowerShell 5.1 may not offer TLS 1.2, which GitHub requires; this line adds it (harmless in PowerShell 7).
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.ps1 -OutFile install.ps1 -UseBasicParsing
# Verify it (below) and read it, then:
powershell -ExecutionPolicy Bypass -File .\install.ps1                    # installs to %LOCALAPPDATA%\Programs\grimoire\bin
powershell -ExecutionPolicy Bypass -File .\install.ps1 -InstallDir C:\tools  # or set GRIMOIRE_INSTALL_DIR
```
- The TLS line is the one Microsoft gives for Windows PowerShell 5.1 in its own install instructions. `install.ps1` sets it again for its own download.
- `-UseBasicParsing` avoids the "Script Execution Risk" prompt that Windows PowerShell 5.1 shows since its 2025-12-09 security update.

What the scripts do:
- They install the version they were released with. To pin a version, download the script from `https://github.com/jabbott-iii/Grimoire/releases/download/vX.Y.Z/` instead.
- **Download.** `install.sh` downloads with curl only, with `--proto =https --tlsv1.2`, so it also refuses a redirect to anything but HTTPS. `install.ps1` uses `Invoke-WebRequest`. PowerShell 7.4 and later refuse a redirect from HTTPS to HTTP by default; what Windows PowerShell 5.1 does is UNVERIFIED.
- **Checksum.** They check the archive's SHA-256 against the value embedded in the script before extracting it. They fail if no SHA-256 tool is available.
- **Replacing the binary.** They copy the new binary next to the old one and check that it runs before swapping it in. `install.ps1` keeps a backup during the swap and puts the old `grimoire.exe` back if the swap fails.
- **Platform.** On macOS under Rosetta, `install.sh` installs the arm64 build. Under 32-bit PowerShell on 64-bit Windows, `install.ps1` still picks the 64-bit build.
- They never edit PATH or shell startup files. They print a hint when the install directory isn't on PATH.
- They send no telemetry, use no tokens, and write no receipts or updater.
- `GRIMOIRE_DOWNLOAD_URL` changes where archives are downloaded from. It must be `https://`, except `http://127.0.0.1` and `http://localhost`, for testing.

The `releases/latest/download/` address is GitHub's redirect to the newest release. The redirect itself isn't covered by the Phase 5 research (UNVERIFIED). Only `release.yml` is meant to create releases here: release-plz is configured to create none, but its token could ([`security-model.md`](security-model.md) §6b).

**Verify a script before you run it.** It is a release asset like any other. Download `install.sh` (or `install.ps1`) with its `.sigstore.json` bundle, and check it with cosign or `gh attestation verify` as in §5, which show that `release.yml` built it. `gh release verify-asset` alone shows only that the file belongs to a release. Then read it.

## 5. Verifying a download

Each release has, for every file:
- a line in `SHA256SUMS`;
- a build provenance attestation, stored on the repository rather than on the release page;
- a cosign bundle, `<file>.sigstore.json`, on the release page.

Each archive also has a CycloneDX SBOM (`<archive>.cdx.json`) and an SBOM attestation. Download the file you want, plus `SHA256SUMS` and the matching `.sigstore.json` bundles, from the release page.

**Checksums:**
```sh
sha256sum -c SHA256SUMS --ignore-missing
```
- This checks every listed file that you downloaded. `SHA256SUMS` is itself signed and attested, so verify it too, as below.
- `sha256sum` here is GNU coreutils. macOS 15 and later include a `/sbin/sha256sum`, but whether it supports `-c` and `--ignore-missing` wasn't checked. macOS 14 and earlier have none.
- On Windows, compare `Get-FileHash -Algorithm SHA256 <file>` with the file's line in `SHA256SUMS`. `Get-FileHash` prints the hash in upper case.

**Build provenance**, with gh 2.102.0 or later:
```sh
gh attestation verify agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz \
  --repo jabbott-iii/Grimoire \
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/release.yml \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- For the SBOM attestation, add `--predicate-type https://cyclonedx.org/bom`.
- `--signer-workflow` and `--source-ref` are case-sensitive, so type `Grimoire` with the capital.
- gh 2.102.0 fixed prefix matching in `--signer-workflow` (GHSA-wjmr-j3rp-mh2g) and case-insensitive `--source-ref` (GHSA-4mq3-hpgx-9cx8). Don't use an older gh.
- To tie the attestation to the release commit, add `--source-digest <commit>`, the commit that tag `vX.Y.Z` points to.
- **Offline.** The `.sigstore.json` files are cosign bundles, and `gh attestation verify` ignores them (it reports that no attestations were found). Save what you need while you are online:
  - `gh attestation download <file> --repo jabbott-iii/Grimoire` saves the file's attestations as `sha256-<hex>.jsonl` and prints the path. Add `--predicate-type https://cyclonedx.org/bom` for the SBOM attestation only.
  - `gh attestation trusted-root > trusted_root.jsonl` saves the trust root.
  - Offline, add `--bundle sha256-<hex>.jsonl --custom-trusted-root trusted_root.jsonl` to the command above. With `--bundle`, gh checks only the names recorded in the certificate, so it can't notice a repository rename or reuse.
  - This round trip is taken from gh 2.103.3's source and hasn't been run.

**cosign**, version 3.1.3 or later:
```sh
cosign verify-blob agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz \
  --bundle agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz.sigstore.json \
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```
- The identity is compared exactly, case included.
- The ref is `refs/heads/main` because releases run from `main`, not from a tag.
- To tie the signature to the release commit, add `--certificate-github-workflow-sha <commit>`, the commit that tag `vX.Y.Z` points to.
- cosign 3.1.3 fixed a verification bypass (GHSA-fx35-mq7g-6g98).
- Without a local trusted root, cosign fetches Sigstore's trust root over the network.

**GitHub's release attestation**, with gh 2.81.0 or later:
```sh
gh release verify vX.Y.Z --repo jabbott-iii/Grimoire
gh release verify-asset vX.Y.Z agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz --repo jabbott-iii/Grimoire
```
- This shows that the file belongs to the published, immutable release `vX.Y.Z`. A release published before immutable releases were enabled has no such attestation.
- It is GitHub's own attestation (signer `https://dotcom.releases.github.com`), not Grimoire's build provenance, so use both.
- It can't check the source archives GitHub generates automatically.

What each package manager checks on your machine, and what it doesn't, is in [`security-model.md`](security-model.md) §6b.
