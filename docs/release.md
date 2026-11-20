# Releasing and installing Grimoire

- **Status:** written in Phase 5 (2026-11-17), before the first release, revised after the Phase 5 review, revised in Phase 6 (2026-11-17) for the container image, revised after the Phase 6 review, which moved every signature and attestation into `.github/workflows/sign.yml`, revised in Phase 7 (2026-11-17) for the before-first-release markers, the README's channels and vulnerability reporting, revised after the Phase 7 review (2026-11-18), when release-plz's first run on GitHub was refused by the ruleset (§2 A), and revised again on 2026-11-18, when GitHub refused to start `release.yml` and the image's signing moved into `.github/workflows/sign-image.yml`. **No release exists yet**, so no command in this file has been run against a real release. The image's build, checks, scan, push to a local registry and the tag script's cases were run locally with rootless Docker, and on GitHub in the first Release dry-run (Release run 44353090397, 2026-11-18), never against GHCR.
- **Decisions:** [ADR-0009](adr/0009-release-pipeline.md) (the pipeline and channels), [ADR-0010](adr/0010-language-policy-exceptions.md) (the code that isn't Rust) and [ADR-0011](adr/0011-container-image.md) (the container image).
- **Facts:** the `*-P5` and `*-P6` entries in [`specs.md`](specs.md); SIGN-P6 for the signing identity.

This file has six parts:
1. how a release flows, for the maintainer;
2. the one-time bootstrap, also for the maintainer;
3. recovering from a failed release;
4. installing, for users;
5. verifying a download, for users;
6. maintaining the image pins, for the maintainer.

## 1. How a release flows

1. **Changes land on `main`** through pull requests, as usual.
2. **release-plz opens or updates the release PR.**
   - `.github/workflows/release-plz.yml` runs `release-plz release-pr` on every push to `main`. A manual run works only on `main`: on another branch, release-plz would rebuild the release PR from that branch. Runs queue one at a time and are never cancelled.
   - The PR bumps the shared workspace version and adds a section for the new version to `CHANGELOG.md`, from the Conventional Commits since the last release. It is labelled `release`.
   - release-plz is configured never to tag, create releases or publish ([ADR-0009](adr/0009-release-pipeline.md)). Its token could do the first two; [`security-model.md`](security-model.md) §6b says what still protects users.
   - **Today it can't open the PR.** The ruleset on every branch refused its commit to its `release-plz-*` branch (run 44198210657), so no release PR exists until you change the ruleset (§2 A).
3. **Approve the release PR's CI runs.**
   - The PR is opened with `GITHUB_TOKEN`, so its `pull_request` runs (CI, Security and the release dry-run) wait for approval. Open the PR's checks and approve the workflows to run, or push a commit to the branch yourself.
   - Do the same after every update release-plz makes to the PR.
   - Running the checks with `workflow_dispatch` doesn't help: those checks don't count toward the ruleset's required status checks.
4. **Review and merge the release PR.**
   - **Check the version.** If the new `CHANGELOG.md` section has a breaking entry, or `rust-version` in the root `Cargo.toml` rose since the last release's tag, the PR's version must be the next 0.y, such as 0.1.4 → 0.2.0 ([`versioning.md`](versioning.md) §2 and §3). If it isn't, correct the version before you merge, through a pull request to `main`, not on the PR's branch (§2 C says why). How release-plz bumps the shared version in Grimoire's setup is UNVERIFIED (VERSIONING-P7 in [`specs.md`](specs.md)), so this review is the only check on it.
   - **Check that every advisory is named.** A security fix merged from an advisory's temporary private fork names the advisory's GHSA ID in the first line of its commit message ([`SECURITY.md`](../SECURITY.md), "What happens to a report"). release-plz carries that line into the `CHANGELOG.md` section, and the release notes come from the section. Check that the section names each such advisory. If one is missing, add it to the GitHub Release's notes once the release is published: an immutable release's notes stay editable.
   - Until pull requests receive the code-scanning and code-quality results the ruleset requires, the merge needs the administrator bypass (§2 A).
5. **The release run starts.**
   - The merge's push to `main` starts `.github/workflows/release.yml`.
   - `plan` chooses release mode only when `GRIMOIRE_RELEASES` is `enabled`, `vX.Y.Z` isn't tagged, and `CHANGELOG.md` has a section for X.Y.Z. Only the merged release PR adds that section. Any other push to `main` stays a dry-run, and `plan` prints a warning that release mode waits for the release PR.
   - In release mode, `plan` fails at once while the checked-in `crates/agent-grimoire-catalog/catalog.pack` isn't exactly what `cargo xtask pack-catalog` makes from `catalog/`. The CLI embeds that file, while the asset binaries are built from `catalog/`, so a stale pack would ship an older catalog beside newer binaries. CI's catalog job fails on a stale pack in every pull request and push to `main`, but no ruleset requires it yet ([`security-model.md`](security-model.md) §6d), so a stale pack can still be merged; this check keeps it out of a release. Fix it in a pull request to `main`: run `cargo xtask pack-catalog` and commit the result. Its merge starts the release again, from that commit (§3, "The run failed before the `release` approval").
   - In release mode, `plan` also fails while an asset in the embedded catalog (`crates/agent-grimoire-catalog/catalog.pack`) has a `min-cli-version` above X.Y.Z, which grimoire X.Y.Z would refuse with `resolve/cli-too-old`. CI's tests fail on such an asset in every pull request and push to `main`, but no ruleset requires them yet, so this check keeps it out of a release. Fix it in a pull request to `main`, whose merge starts the release again: lower the asset's `min-cli-version` (with an asset version bump and an entry in the asset's own `CHANGELOG.md`, then `cargo xtask pack-catalog`), or release a version at least that new through a release PR (step 4 says how to correct its version). See [`versioning.md`](versioning.md) §4.
   - `plan` fails at once if the catalog has grown past GitHub's limits: 256 jobs in the SBOM attestation matrix, and 1000 assets in one release. With today's channels, the asset limit is reached at 39 asset binaries.
   - The run builds all six targets, packages every channel and runs the verification jobs. Then `sign-files` signs every file (below).
   - Every release-mode job downloads the release files by artifact ID. Before it attests, signs or publishes anything, it checks `SHA256SUMS` against the hash the package job recorded, and that the files are exactly the ones `SHA256SUMS` lists.
   - **Signing.** Every signature and attestation is made in one of two reusable workflows: `.github/workflows/sign.yml` for the release files, which only `sign-files` calls, and `.github/workflows/sign-image.yml` for the image, which only `sign-image` calls. Their certificates name `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` and `…/sign-image.yml@refs/heads/main`, which no job in `release.yml` can get, so the publish jobs, and the PyPI action's container, can't sign as the release. Their jobs run only when `release.yml` on `main` in this repository calls them.
   - They are two files because GitHub checks every job of a called workflow against the calling job's permissions when the run starts, even a job its `if:` would skip. With one file for both, GitHub refused to start any Release run (`SIGN-P6` in [`specs.md`](specs.md)). The xtask test `every_reusable_workflow_call_grants_what_its_jobs_ask_for` checks every call's grant.
   - `sign-files` makes one provenance attestation over every file, an SBOM attestation for each archive (one matrix job each), and a cosign bundle for each file, which it verifies at once with the identity and the caller pins (§5).
   - **The container image.** The `image` job builds it from the two Linux musl archives, for linux/amd64 and linux/arm64, as an OCI layout with an SBOM and provenance for each platform. It verifies the base image's signatures first, then checks the layout with `cargo xtask release image check` and scans each platform with Grype. Any medium, high or critical match fails the job (§3).
   - `verify-image` pushes the layout to a local registry, pulls it on an amd64 and an arm64 runner, and runs every binary in it. Pull requests run both jobs too. They also sign the image with a throwaway key, offline, and run the release's tag script, `.github/scripts/image-tags.sh`, against the local registry, refusals included.
   - **Where the image digest comes from.** The `image` job prints the layout's index digest. `publish` writes it into the release notes in step 6, before anything is pushed. `publish-image` pushes exactly that index in step 7, and `sign-image` signs it before it gets a tag. `verify-image` and `publish-image` download the layout by artifact ID and check that it is the index the `image` job checked.
6. **Approve the first deployment, `release`.**
   - The `publish` job waits in the `release` environment. Check that every earlier job passed, then approve.
   - It verifies `SHA256SUMS`, every cosign bundle and the provenance of `SHA256SUMS`, each with `sign.yml`'s identity and the caller pins (§5). It also checks that the provenance certificate's Build Config URI is `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, the calling workflow, and that its repository and owner IDs are Grimoire's (`1460666977`, `84321953`), which no cosign or gh flag can pin.
   - It then tags `vX.Y.Z` at the commit it built, and publishes the GitHub Release (draft, upload, publish). It has no `id-token` permission.
   - The notes name the container image by digest, `ghcr.io/jabbott-iii/grimoire@sha256:…`, with the tags `X.Y.Z` and `latest`, and say that it is pushed once the registry jobs are approved.
   - It then prints the release's `immutable` field, and only warns if it isn't `true`. Check that line before going on; §3 says what to do if it is `false`.
7. **Approve the registry deployments, `crates-io`, `npm`, `pypi` and `ghcr`.**
   - Four jobs now wait at once, each in its own environment. Approve them together.
   - `publish-crates` gets a token with `crates-io-auth-action` and runs `cargo publish --locked -p <crate>` for each crate whose version isn't on crates.io yet, in dependency order.
   - `publish-npm` stages the six platform tarballs, then the launcher.
   - `publish-pypi` uploads the wheels with `pypa/gh-action-pypi-publish` (`skip-existing: true`).
   - `publish-image` pushes the checked layout to `ghcr.io/jabbott-iii/grimoire` by digest, untagged, with the job's `GITHUB_TOKEN` (`image-tags.sh push`). An existing `X.Y.Z` that names another digest stops the job; the tag is never overwritten. It has no OIDC token and runs no cargo.
   - **Then `sign-image` runs, with no approval of its own.** A job that calls a reusable workflow can't name an environment, so the `ghcr` approval of the push is the approved step. It calls `sign-image.yml`, which signs the index digest with cosign (keyless), adds a build provenance attestation with `actions/attest`, checks that both are listed in the `sha256-<index hex>` tag, and verifies both.
   - Only then does the image get its tags: `X.Y.Z`, then `latest` (`image-tags.sh tag` and `latest`). `latest` moves to this release unless it names a newer version. It stays on a newer release that carries the image's release signature, and if the newer version doesn't verify, the job stops (§3).
   - `sign-image`'s last step fails while the package can't be pulled anonymously. That can happen only in the first release, before you make the package public (§2 D).
   - Each job's OIDC token names its own environment, and each registry's trusted publisher accepts only its own, so no job can publish to another registry. `publish-image` has no OIDC token. `publish-crates`, `publish-npm` and `publish-pypi` can still get Sigstore certificates, but those name `release.yml`, and no verifier accepts that identity.
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
     - `winget --version` must be v1.28.190 or newer. Older clients don't know schema 1.28.0, so they warn on every manifest's schema header and exit with -1978335192 (`MANIFEST_VALIDATION_WARNING`).
   - Then test the install. Windows Sandbox is the simplest place: run `Tools\SandboxTest.ps1 <dir>` from your winget-pkgs fork. Otherwise, `winget install --manifest <dir>` needs local manifests turned on first: run `winget settings --enable LocalManifestFiles` in an elevated shell, and turn the setting off again afterwards. Group Policy can block that setting on a managed machine; use the Sandbox there.
   - winget-pkgs then runs its 10-step validation, and a moderator approves.
10. **Merge the tap and bucket sync PRs.**
    - In `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`, the sync workflow runs daily, or when you start it from the Actions tab. It downloads `agent-grimoire.rb` or `agent-grimoire.json` from the latest release, checks its attestation, and opens a PR when the file differs from `Formula/agent-grimoire.rb` or `bucket/agent-grimoire.json`.
    - GitHub turns off scheduled workflows in a public repository after 60 days without activity. The tap and bucket get commits only when a sync PR merges, so a quiet spell between releases can turn the schedule off. If no sync PR appears, enable the workflow again (Actions tab → "Enable workflow", or `gh workflow enable sync-formula.yml --repo jabbott-iii/homebrew-tap` and `gh workflow enable sync-manifest.yml --repo jabbott-iii/scoop-bucket`), then start it with `gh workflow run`.
    - Those PRs are opened with each repository's `GITHUB_TOKEN`, so any checks they have wait for your approval, as in step 3. Review and merge them.

## 2. One-time bootstrap (maintainer)

This checklist is the source for the bootstrap of Phases 5 to 7; `PLAN.md` §6 points here. Do the parts in order:
- **A:** the items marked "now", "before you merge the Phase 7 change" or "after you merge the Phase 7 change" at that time, the rest at any time before the first release;
- **B:** close to the first release, because PyPI's pending publisher expires after 30 days;
- **C:** last;
- **D:** during the first release run.

Never store a token in GitHub at any step.

### A. Repository settings

- [ ] **Now: let release-plz commit to its branch.** Until you do, release-plz can't open the release PR, so no release can start (§1).
  - **What happened.** The Release PR workflow's run 44198210657 failed in both attempts, on 2026-11-17 and 2026-11-18 (RELEASE-PLZ in [`specs.md`](specs.md)). Each attempt created a `release-plz-<timestamp>` branch, and then GitHub refused its commit there: "Repository rule violations found … Changes must be made through a pull request … Waiting for Code Scanning results. Code Scanning may not be configured for the target branch."
  - **Why.** The ruleset `BestBranch` covers every branch (`~ALL`), the `release-plz-*` branches included. Its pull-request rule and its code-scanning rule both refuse the commit, so lifting only one of them isn't enough.
  - **Your decision.** Either target the ruleset at the default branch only (`~DEFAULT_BRANCH`), or keep `~ALL` and exclude `refs/heads/release-plz-*`. With the first, other branches also lose the deletion and force-push rules. With either, pull requests into `main`, the release PR included, keep every rule.
  - **Don't add GitHub Actions as a bypass actor** instead. Any workflow job whose token has `contents: write` could then change `main` without a pull request.
  - **Then delete the orphan branches:** `release-plz-2026-11-17T23-18-49Z`, `release-plz-2026-11-18T05-09-04Z`, `release-plz-2026-11-18T14-29-28Z` (run 44324738027, on the push of `dca6c1d`) and `release-plz-2026-11-18T16-57-17Z` (run 44353090254, on the push of `7fc1906`). Each push to `main` adds another until the ruleset lets release-plz commit, so list them with `gh api 'repos/jabbott-iii/Grimoire/branches?per_page=100' --jq '.[].name'` first. While the ruleset still covers them, its deletion rule allows that only through the administrator bypass.
  - Then run `release-plz.yml` on `main` (Actions → Release PR → Run workflow), or push to `main`, and check that the release PR opens. Opening it also needs the Actions setting below.
- [ ] **Now: give the code-quality rule its results, or drop it.** `BestBranch` also requires CodeQL code-scanning results and code-quality results.
  - **CodeQL:** `.github/workflows/codeql.yml` (advanced setup) analyzes Rust, JavaScript, Python and the workflows on every pull request to `main`, every push to `main` and weekly. Its actions are pinned like every other workflow's (CODEQL in [`specs.md`](specs.md)). Pull request 1 received a passing `CodeQL` check on 2026-11-19. Whether the ruleset's `code_scanning` rule then passes wasn't read.
  - **Code quality:** nothing produces those results yet, and whether Code Quality is on couldn't be read (UNVERIFIED, COMMUNITY-P7). Until something does, no pull request into `main`, a contributor's or the release PR, can be merged without the administrator bypass.
  - **Your decision.** Enable Code Quality in the repository's settings, or remove the ruleset's `code_quality` rule. Don't also turn on CodeQL default setup: GitHub doesn't accept advanced-setup results while default setup is on.
  - Then confirm that a pull request receives both results, and that the rules can pass.
- [ ] **Allow GitHub Actions to create and approve pull requests** (Settings → Actions → General). This was due before the Phase 5 merge. Whether it is on couldn't be read: the API answered 403 to the token used.
  - `release-plz.yml` runs on the next push to `main` and opens the release PR (`chore: release`).
  - With the setting off, each run creates a `release-plz-<timestamp>` branch and then fails to open the PR. The ruleset leaves such branches too (the first item). Delete any `release-plz-*` branch that has no PR.
  - Leave the release PR unmerged until the rest of this bootstrap is done. release-plz updates it on every push.
  - Keep the default `GITHUB_TOKEN` permissions read-only (`PLAN.md` §6).
- [ ] **Before you merge the Phase 7 change, watch the repository for security alerts.** It is needed as soon as [`SECURITY.md`](../SECURITY.md) is on `main`, because it promises an acknowledgement within 7 days from then. On the repository page, choose Watch → Custom → Security alerts, or All Activity, and keep notifications on; for email, also select Email under Subscriptions → Watching in your notification settings. Otherwise GitHub may not tell you about a private report ([GitHub's docs](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository), checked 2026-11-17). You are the only collaborator, so nobody else can act on a report.
- [ ] **Confirm that private vulnerability reporting is still on,** before you merge the Phase 7 change and again before the first release. It was on when read on 2026-11-17, and [`SECURITY.md`](../SECURITY.md) sends reports to its form, `https://github.com/jabbott-iii/Grimoire/security/advisories/new`. `gh api repos/jabbott-iii/Grimoire/private-vulnerability-reporting` should show `{"enabled":true}`.
- [ ] **After you merge the Phase 7 change, check the new GitHub files.** GitHub skips a CODEOWNERS line that has an error (COMMUNITY-P7) and doesn't show some invalid issue forms (ISSUE-FORMS-P7). Blank issues are off, so a missing form leaves no way to file that kind of issue.
  - `gh api repos/jabbott-iii/Grimoire/codeowners/errors` should return an empty `errors` list.
  - Open <https://github.com/jabbott-iii/Grimoire/issues/new/choose>, signed in as an account other than yours. It should show the five forms, "Bug report", "Asset request", "Client format change", "Ask for a private contact" and "Propose a change", and the two links, "Report a security vulnerability" and "Documentation", with no second security entry. Whether GitHub adds its own security link to the chooser is UNVERIFIED (ISSUE-FORMS-P7).
  - Record what you saw in `specs.md`, under COMMUNITY-P7 and ISSUE-FORMS-P7.
- [ ] **Create five environments** (Settings → Environments): `release`, `crates-io`, `npm`, `pypi` and `ghcr`. Configure each one the same way:
  - Required reviewer: you.
  - Leave **Prevent self-review** off while you are the only reviewer. With it on, you can't approve a run you started; only the administrator bypass could start it.
  - Deployment branches and tags: "Selected branches and tags", with one branch rule, `main`. Releases run only from `main`, never from a tag.
  - No environment secrets. `ghcr` needs none either: `publish-image` pushes with the job's `GITHUB_TOKEN`.
  - If an environment already exists, configure it the same way. A run that references a missing environment creates it without protection rules.
  - Each release then needs two approval rounds: `release` for the tag and the GitHub Release, then `crates-io`, `npm`, `pypi` and `ghcr` together.
  - `ghcr` approves `publish-image`, the push. `sign-image` follows without an approval of its own, because a job that calls a reusable workflow can't name an environment. `sign-files` has no environment either, and runs before the `release` approval ([`security-model.md`](security-model.md) §6b, "Signing comes before approval").
- [ ] **Enable immutable releases** (Settings → Releases → "Enable release immutability").
  - It applies only to releases published afterwards.
  - `GITHUB_TOKEN` can't read this setting, so the `publish` job only warns when a release comes out mutable. Check it with your own login: `gh api repos/jabbott-iii/Grimoire/immutable-releases` should show `"enabled": true`.
  - A mutable release gets no release attestation, so the tap and bucket syncs can't take it (§3).
- [ ] **The CI ruleset on `main`** (`PLAN.md` §6): require the CI, Security and `release checks` checks. `release checks` is the release workflow's summary job. On a pull request it fails unless every dry-run job passed; a skipped job counts as a failure, because a required check whose job is skipped would otherwise count as passed. The release PR's checks then come from the runs you approve.
  - A ruleset on every branch already requires pull requests, with one approval and code-owner review ([`.github/CODEOWNERS`](../.github/CODEOWNERS)), but no status checks (read on 2026-11-17). The administrator role, Dependabot and two other GitHub Apps can bypass it; GitHub Actions can't.

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

**GHCR.** There is nothing to create: the first release's `publish-image` job creates the package `ghcr.io/jabbott-iii/grimoire`.

- [ ] **Never push to `ghcr.io/jabbott-iii/grimoire` by hand before the first release.** A package that a workflow first publishes with `GITHUB_TOKEN` is linked to that repository. But `GITHUB_TOKEN` can't push to a package in your namespace that was pushed earlier and isn't connected to the repository, so a manual first push would block `publish-image`.
- [ ] Check that your account has no `grimoire` container package yet. Anonymous checks on 2026-11-17 found none, but they can't see a private package (UNVERIFIED). If one exists, open its package settings before the first release: connect it to `jabbott-iii/Grimoire`, and give that repository write access under "Manage Actions access". A package linked after its first push doesn't inherit the repository's access. This case hasn't been tried.

### C. Switch releases on (last)

- [ ] **Clear the before-first-release markers.** Text to change before the first release sits between an opening line, `<!-- before-first-release: … -->`, which says what to do, and a closing line, `<!-- /before-first-release -->`. The marked texts are the README's "not released yet" note, the placeholder for the conduct contact in `CODE_OF_CONDUCT.md`, and the "No release exists yet" sentence in `SECURITY.md`. `grep -n -i -E '^[[:space:]]*<!--[[:space:]]*/?before-first-release' README.md CODE_OF_CONDUCT.md CONTRIBUTING.md SECURITY.md` lists the marker lines. In a pull request of its own, merged to `main` before the release PR, do what each opening line says: in the README and `SECURITY.md`, delete everything from the opening line to the closing one; in `CODE_OF_CONDUCT.md`, fill in the contact and delete both marker lines.
  - `cargo xtask release plan` fails while an opening or a closing line remains in `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md`, in every run that has a `CHANGELOG.md` section for the version: the release PR's checks and the release itself. It matches the lines loosely, in any case and with any spacing around `<!--`, so deleting only the opening line still fails. Other pull requests aren't affected.
  - Make the change on `main`, not on the release PR's branch. release-plz manages that branch and updates it on each push to `main`; whether it keeps a commit you push there is UNVERIFIED.
  - The README goes into every CLI archive, it is the PyPI long description and the `agent-grimoire` crate's page on crates.io, and a published version can't be changed. The npm packages don't carry it: `xtask/src/release/channels/npm.rs` writes their short READMEs.
- [ ] Set the repository variable `GRIMOIRE_RELEASES` to `enabled` (Settings → Secrets and variables → Actions → Variables).
- From then on, a push to `main` runs in release mode when its version isn't tagged and `CHANGELOG.md` has a section for it. Other pushes, Dependabot merges included, stay dry-runs.
- The first release starts when you merge the release PR. Before you merge it, check that it is current (release-plz updates it on each push) and that its checks passed.

### D. During the first release run

Trusted publishing can't create a crate, so you publish the first version of each crate yourself, from the commit the run tagged. npm's trusted publishing is set up here too, so that its 48-hour window starts as late as possible. The container image's package is created by the run itself, and you make it public at the end.

1. **Wait for the `release` approval.** Check that the run's commit (the `GITHUB_SHA` shown in the run) is the release PR's merge commit, and that every earlier job passed. If not, reject it. The one other commit to accept is the merge commit of a fix PR merged after the release PR's run failed before this approval (§3), once you've checked that `main` gained only that fix since the release PR's merge.
2. **Approve `release`.** The `publish` job tags `vX.Y.Z` and publishes the GitHub Release. Check that its log says the release is immutable (§3 if not).
3. **The run now waits for `crates-io`, `npm`, `pypi` and `ghcr`.** Don't approve yet.
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
11. **Approve `crates-io`, `npm`, `pypi` and `ghcr`.** The crates.io job finds all six versions published and skips them. The npm job stages the seven packages, and the PyPI job uploads the wheels and creates the project. `publish-image` creates the GHCR package with its first push, and `sign-image` follows without another approval.
12. **Approve the npm versions** (§1 step 8).
13. **Check the new package before you make it public.** `sign-image` (not `publish-image`) signs, attests, tags `X.Y.Z` and moves `latest`, then fails at its last step if the package can't be pulled anonymously.
    - GitHub's docs disagree on whether a package that a workflow creates starts private or takes the repository's visibility. If the last step passed, the package is already public: do the checks below and skip steps 14 and 15.
    - On the package's page on GitHub, check that it is linked to `jabbott-iii/Grimoire`. The image's `org.opencontainers.image.source` label and annotations name the repository; which of them GHCR reads is UNVERIFIED.
    - The package also shows `sha256-<hex>` tags, because GHCR has no referrers API. `sha256-<index hex>` lists the cosign signature and the provenance attestation, and each `sha256-<platform hex>` lists that platform's BuildKit attestation manifest. Never delete them (§3).
14. **Make the package public, in its package settings.** **This can't be undone**: a public package can never be made private again. No API changes a package's visibility, so this step is manual.
15. **Re-run `sign-image`** (Actions → the run → Re-run failed jobs). It has no environment, so it waits for no approval. The signature and the attestation are skipped because they are done, the tags already name this digest, and the last step now passes (§3).

## 3. Recovery

- **A job failed.**
  - Re-run the failed jobs of the same run (Actions → the run → Re-run failed jobs). GitHub allows that for 30 days after the run, and a re-run uses the run's own commit.
  - Each job skips what is already done:
    - `publish` accepts the tag at this commit, finishes a draft release, and checks that a published release holds exactly the files in `dist/`;
    - `publish-crates` skips crate versions already on crates.io;
    - `publish-npm` skips versions already published (staged ones are below);
    - `publish-pypi` skips files already on PyPI (`skip-existing`);
    - `publish-image` pushes the same digest again, untagged, which is harmless. It stops if `X.Y.Z` names another digest, and if a tag lookup fails for any reason other than the registry's "unknown" answer (`MANIFEST_UNKNOWN`, `NAME_UNKNOWN`);
    - `sign-image` skips the signature when one already verifies with the image's signing identity (`sign-image.yml`) and the caller pins, and the attestation when `gh attestation verify --bundle-from-oci` finds one in GHCR. An attestation that reached GitHub's attestations API but not GHCR is made again, which only adds a second record to the API, so a re-run after a partial attestation still finishes. It then checks and verifies both again, leaves `X.Y.Z` alone when it already names this digest, and applies the `latest` rule below.
  - The registry jobs are separate, so a failure in one doesn't hold up the others. GitHub also re-runs the jobs that depend on a re-run job, so a re-run `publish-image` is followed by `sign-image`. Whether a re-run job waits for its environment's approval again isn't documented; expect it to. `sign-image` has no environment, so it never waits.
  - Use "Re-run failed jobs", not "Re-run all jobs". A new `image` run builds an index with another digest, because its SBOMs and provenance hold build times, and the published notes name the first one.
  - Don't wait for a new run instead. Once the tag exists, a run on `main` only skips.
- **The `image` job failed.** It runs before the `release` approval, so nothing was published. Fix the cause as below, through a normal PR, as for "The run failed before the `release` approval".
  - **A Grype finding, or a coverage failure.** Every platform is scanned before the step fails, so the `image-scan` artifact holds each platform's JSON and CycloneDX reports. A coverage failure (Grype detected no Debian 13 or no Debian packages, or a binary shows no Rust packages) means Grype couldn't see part of the image: investigate it, never ignore it. If a fix exists, take it: a base digest bump (§6) or a crate update. If none exists, add an ignore rule to `.grype.yaml` that names the vulnerability and the package and gives a reason, in a PR reviewed like the exceptions in `deny.toml`. Never lower the threshold or drop `--fail-on`.
  - Grype's database is rebuilt daily, so a re-run can fail where the pull request passed.
  - **A base signature, `context`, `check` or `coverage` failure.** Investigate; never skip or weaken the step. A base that doesn't verify, or an image whose layers aren't exactly the base's and the archived binaries, can mean a compromise.
- **The image's tags.** Only `sign-image` sets them, and only after the signature and the attestation verify in GHCR. `X.Y.Z` is never overwritten. `latest` moves to this release unless it already names a newer version. It stays on a newer release that verifies with the image's signing identity, so it never moves back to an older signed release. If the newer version doesn't verify, the job stops and `latest` stays where it is. `.github/scripts/image-tags.sh` makes these steps, and its errors mean:
  - **`already points at <digest>, not at <digest>`** (`publish-image` or `sign-image`): `X.Y.Z` already names another digest. Stop and investigate. Never overwrite the tag and never delete it. Check whether that digest verifies with the image's identity, `sign-image.yml` (§5, "The container image").
  - **`cannot tell whether <ref> exists`:** a tag lookup failed for a reason other than the registry's "unknown" answer, and crane's own error is printed just before it. The script never treats a tag it couldn't read as absent. Fix the cause, then re-run the failed job.
  - **`crane pushed <ref>, not <image>@<digest>`:** the push didn't keep the digest of the checked layout. Stop and investigate before you re-run.
  - **`latest is <version>, newer than X.Y.Z; leaving it`** is expected when you re-run an older release.
  - **`latest (<digest>) claims <version>, newer than X.Y.Z, but doesn't verify with the release signature: investigate before moving it`** means the image `latest` names claims a newer version but didn't verify with the image's signing identity. Someone with write access to the package moved `latest`, its signature was removed from its `sha256-<hex>` tag, or the check itself failed, for example on a network error: the script can't tell these apart, so it stops and leaves `latest` alone. Investigate. If the check itself failed, re-run the failed job.
  - **The warning `latest named <digest>, which has no release version; moving it`** means `latest` named an image without an `org.opencontainers.image.version` annotation, which no release makes. The script moves `latest` to this release. Find out who tagged that image.
  - **`is not a sha256 digest`, `is not a release version`, a usage line, or `sign-image.yml`'s `unexpected version … or digest`:** the workflow passed bad input. That is a bug in the workflow, so a re-run can't fix it (below, "A re-run can't finish the release").
- **The weekly image scan failed** (`security.yml`, job `image-scan`). Either the latest release's image isn't pushed, tagged, signed or public yet (finish its `publish-image` and `sign-image`, §2 D), which the job's error names, or a new advisory matches it. A released image is never changed, so if a fix exists, take it as for the `image` job and release a new patch version. If none exists, an ignore rule with a reason in `.grype.yaml` on `main` also clears the weekly scan.
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
  - **If the release is missing or still a draft, and no registry has X.Y.Z** (crates.io; npm, published or staged; PyPI; GHCR):
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

Install `agent-grimoire`, run `grimoire`. The only exception is winget, where the command is `agent-grimoire` (below). Every prebuilt channel installs the same binary; `cargo install` builds it from source. [`cli.md`](cli.md) has the commands.

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
| Container image | `docker pull ghcr.io/jabbott-iii/grimoire@sha256:<digest>`, with the digest from the release notes | linux/amd64 and linux/arm64. How to run it is in [`container.md`](container.md) |

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

**Verify a script before you run it.** It is a release asset like any other. Download `install.sh` (or `install.ps1`) with its `.sigstore.json` bundle, and check it with cosign or `gh attestation verify` as in §5, which show that a release run on `main` signed it. `gh release verify-asset` alone shows only that the file belongs to a release. Then read it.

## 5. Verifying a download

Every file in a release, except the `.sigstore.json` bundles themselves, has:
- a build provenance attestation, stored on the repository rather than on the release page;
- a cosign bundle, `<file>.sigstore.json`, on the release page.

`SHA256SUMS` lists every one of those files except itself. Each archive also has a CycloneDX SBOM (`<archive>.cdx.json`) and an SBOM attestation. Download the file you want, plus `SHA256SUMS` and the matching `.sigstore.json` bundles, from the release page.

**Who signs.** Every signature and attestation of the release files is made by `.github/workflows/sign.yml`, Grimoire's signing workflow, when `release.yml` on `main` calls it. The container image is signed by `.github/workflows/sign-image.yml`, under its own identity (below). Any repository can call a public reusable workflow and get a certificate with the same name, so the commands below also pin the caller: the repository `jabbott-iii/Grimoire` and the ref `refs/heads/main`. No release exists yet, so every release is signed this way. Don't accept `…/release.yml@refs/heads/main`: other release jobs can still get certificates with that name, and no release is signed with it.

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
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- `--signer-workflow` names `sign.yml` at `refs/heads/main`, the workflow that signs; without the ref, gh would also accept `sign.yml` at any tag or commit, which any repository can call. `--repo` and `--source-ref` pin the run that called it. `--repo` alone accepts any workflow and any ref of the repository, so don't drop the other flags.
- **Pin what no flag does**, especially with `--bundle-from-oci`, where gh compares names only: add `--format json` and check the caller's workflow file and the repository and owner IDs. The release workflow runs this same check.
  ```sh
  … --format json | jq -e 'length > 0 and all(.[]; .verificationResult.signature.certificate
    | .buildConfigURI == "https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main"
      and .sourceRepositoryIdentifier == "1460666977" and .sourceRepositoryOwnerIdentifier == "84321953")'
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
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-github-workflow-repository jabbott-iii/Grimoire \
  --certificate-github-workflow-ref refs/heads/main
```
- The identity names `sign.yml`. The last two flags pin the caller: a call from another repository gets the same identity, but names its own repository and ref there (SIGN-P6 in [`specs.md`](specs.md)). Keep all four flags.
- The identity and both pins are compared exactly, case included.
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

**The container image.** [`container.md`](container.md) has the details. Take the index digest from the release notes, then use `sign-image.yml`'s identity and the same caller pins:
```sh
cosign verify ghcr.io/jabbott-iii/grimoire@sha256:<index digest> \
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-github-workflow-repository jabbott-iii/Grimoire \
  --certificate-github-workflow-ref refs/heads/main
gh attestation verify oci://ghcr.io/jabbott-iii/grimoire@sha256:<index digest> \
  --bundle-from-oci \
  --repo jabbott-iii/Grimoire \
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- Verify the index digest. Only the index is signed, not each platform's manifest, so a platform manifest's digest doesn't verify on its own.
- `--bundle-from-oci` reads the attestation's copy in GHCR, listed in the `sha256-<index hex>` tag. Without it, gh reads GitHub's attestations API instead.
- The tags `X.Y.Z` and `latest` can move; a digest can't. Pin the digest.
- The signature names a digest, not a version. The index's `org.opencontainers.image.version` annotation, which the signature covers, says which version it is.
- `gh attestation verify` needs gh 2.102.0 or later, a gh login, and a registry login for the `oci://` lookup.

What each package manager checks on your machine, and what it doesn't, is in [`security-model.md`](security-model.md) §6b; the image is in §6c.

## 6. Maintaining the image pins (maintainer)

**The 7-day rule.** Every pin is a release, or a digest, at least 7 days old when it is adopted (CI-PINS in [`specs.md`](specs.md)). Dependabot's cooldown enforces that for its other entries. For the base image it doesn't, so you check by hand.

**The base image** (the one `FROM` line in `Dockerfile`):
- Dependabot's `docker` entry opens a PR when distroless's `nonroot` tag moves to a new index. The PR changes only the digest on the `FROM` line. The `context` step derives everything else from it, such as the `base.digest` annotations.
- Its cooldown has no effect here. Dependabot dates Docker updates only from Docker Hub, so for gcr.io it proposes the new digest at once, with a `cooldown_date_unavailable` warning.
- **Before you merge, check the new digest's age:**
  ```sh
  curl -fsS https://gcr.io/v2/distroless/static-debian13/tags/list \
    | jq -r '.manifest["sha256:<new digest>"].timeUploadedMs | tonumber / 1000 | todate'
  ```
  - It prints when GCR first stored that index, such as `2026-10-31T22:37:00Z` for `sha256:25ac8f0c…`. Merge only when that is at least 7 days ago. The error `null (null) cannot be parsed as a number` means the digest isn't in that repository. Checked with jq 1.7 on 2026-11-17.
  - `timeUploadedMs` is undocumented. For the digests the Phase 6 research looked at, it matched the time of the digest's signature in Rekor.
  - `.manifest["sha256:<new digest>"].tag` lists the index's tags. The newest index carries `nonroot`.
- **Then let the PR's checks run.** The release dry-run verifies distroless's signature on the new index and both platform manifests, then builds, checks and scans the image. Merge only when they pass.
- The PR has no release notes, because distroless sets no labels.
- Dependabot opens no security updates for Docker images. The weekly `image-scan` job in `security.yml` scans the latest release's image instead (§3).
- **Today's pin** is `sha256:25ac8f0c…` (first uploaded 2026-10-31). The current `nonroot`, `sha256:aafe9936…`, was first uploaded 2026-11-12, so it breaks the rule until 2026-11-19T21:16:35Z. The two differ only in the mode of `/usr/lib/os-release`, to which `/etc/os-release` links. Take Dependabot's PR for it after that time.

**Where the other pins live.** Dependabot can't see values in a workflow's `env:`, so bump these by hand:

| Pin | Where |
|---|---|
| `IMAGE` (`ghcr.io/jabbott-iii/grimoire`) | `release.yml` `env`, `sign-image.yml` `env`, `security.yml`'s `image-scan` job, and the `IMAGE` constant in `xtask/src/release/image.rs` |
| `BUILDX_VERSION`, `BUILDX_SHA256` (the linux-amd64 binary) | `release.yml` `env` |
| `CRANE_VERSION`, `CRANE_SHA256_X64` | `release.yml` `env`, `sign-image.yml` `env`, and `security.yml`'s `image-scan` job |
| `CRANE_SHA256_ARM64` (`verify-image`'s arm64 runner) | `release.yml` `env` |
| `GRYPE_VERSION`, `GRYPE_SHA256` (the linux_amd64 tarball) | `release.yml` `env`, and `security.yml`'s `image-scan` job |
| `BUILDKIT_IMAGE`, `SBOM_SCANNER_IMAGE` (`name:tag@sha256:…`) | `release.yml` `env` |
| `DISTROLESS_IDENTITY`, `DISTROLESS_ISSUER` (distroless's signer) | `release.yml` `env` |
| `COSIGN_IDENTITY`, `COSIGN_REPOSITORY`, `COSIGN_REF` (a signing identity and its caller pins) | For the release files, `sign.yml`: `release.yml`'s `publish` job and `sign.yml` `env`. For the image, `sign-image.yml`: `sign-image.yml` `env` and `security.yml`'s `image-scan` job |
| `SHELLCHECK_VERSION`, `SHELLCHECK_SHA256` | `release.yml`'s `verify-linux` job, and `security.yml`'s `workflows` job |
| cosign (`sigstore/cosign-installer`) and `actions/attest` | `uses:` lines, which Dependabot and `actions.lock` cover |

- **The four workflows must agree.** The xtask test `the_image_pins_agree_across_workflows` (run by `cargo test --workspace`) fails:
  - when `IMAGE`, `CRANE_VERSION` or `CRANE_SHA256_X64` differ between `release.yml` and `security.yml` or `sign-image.yml`;
  - when `COSIGN_ISSUER`, `COSIGN_REPOSITORY` or `COSIGN_REF` differ between `release.yml` and `security.yml`, `sign.yml` or `sign-image.yml`;
  - when `GRYPE_VERSION`, `GRYPE_SHA256`, `SHELLCHECK_VERSION` or `SHELLCHECK_SHA256` differ between `release.yml` and `security.yml`;
  - when a file sets one of these keys more than once, or not at all;
  - when `COSIGN_IDENTITY` isn't `…/sign.yml@refs/heads/main` in `sign.yml` and `release.yml`, or `…/sign-image.yml@refs/heads/main` in `sign-image.yml` and `security.yml`;
  - when `sign-image.yml`'s repository isn't `jabbott-iii/Grimoire`, its ref `refs/heads/main`, or its `SIGNER_WORKFLOW` `jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main`;
  - when `release.yml`'s `IMAGE` isn't the xtask constant, or `BUILDKIT_IMAGE` or `SBOM_SCANNER_IMAGE` isn't pinned by digest.

  Change the files in one PR. The test passed locally on 2026-11-18.
- **The identities are also written out in full** where the test doesn't look: the job guards in both signing workflows, the `publish` job's `gh attestation verify` flags and expected Build Config URI, and the tap and bucket sync templates. Change them with the pins.
- **`.github/scripts/image-tags.sh`** is checked by ShellCheck (the pin above) in `security.yml`'s `workflows` job, and every PR dry-run runs its cases in `verify-image` against a local registry. The release runs the same script, in `publish-image` and `sign-image.yml`.
- **Tools.** Date a tool by its GitHub release. Check each new SHA-256 against the release's own checksum file, as for the other CI pins.
- **Images.** Date BuildKit and the SBOM scanner by their GitHub releases, and resolve the tag to its index digest, for example `crane digest moby/buildkit:v0.34.2` (it printed the pinned `sha256:c5f8862f…` on 2026-11-17). Keep `BUILDKIT_IMAGE` in the form `moby/buildkit:vX.Y.Z@sha256:…`: the builder step checks that the `BuildKit version:` line of `docker buildx inspect` reports the tag's version. The scanner's image tags have no `v`.
- **Before a cosign upgrade**, check that it still verifies distroless's legacy `.sig` signatures. cosign's version comes with `sigstore/cosign-installer`, which Dependabot bumps. cosign says its new bundle format will become the only supported one, and distroless publishes only the legacy format.
- **Grype.** `.grype.yaml` holds the gate. Grype ignores unknown keys silently, also inside an ignore rule, so the test `the_grype_config_has_only_known_keys_and_complete_rules` refuses any unknown key and any rule without a GHSA or CVE id, a package name (`package: name:`) and a reason, and the workflows repeat the threshold with `--fail-on medium`. A rule may use only `vulnerability`, `package` (`name`, `version`), `reason` and `include-aliases`; widen the test in the same PR if a rule needs more. Ignore rules have no expiry: review every rule when you bump the base image or Grype, and remove the ones that no longer match.
- **Known follow-ups.** BuildKit v0.35.0 and buildx v0.39.0 (2026-11-11) become eligible on 2026-11-18. A CI check of the base digest's age doesn't exist yet.
