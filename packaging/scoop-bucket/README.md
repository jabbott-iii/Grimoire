# Scoop bucket template

Scoop is a command-line installer for Windows, and a bucket is a Git repository of the manifests (install recipes) it installs from. This directory holds what goes into the separate repository `jabbott-iii/scoop-bucket`, Grimoire's bucket.

Nothing here runs in Grimoire itself. Grimoire's CI (continuous integration) only lints [`sync-manifest.yml`](sync-manifest.yml) with actionlint and zizmor, two linters for GitHub Actions workflows.

The bucket has one manifest, `bucket/agent-grimoire.json`. It installs the prebuilt `grimoire.exe` from the GitHub Release, for 64-bit and arm64 Windows. The bucket never writes the manifest itself:

- Every Grimoire release attaches a generated `agent-grimoire.json`, attested and signed like every other asset. An attestation is a signed record of which workflow run produced a file.
- `sync-manifest.yml` copies that file from the latest release, after verifying it, and opens a pull request.

The design is in [ADR-0009](../../docs/adr/0009-release-pipeline.md), the architecture decision record (ADR) for the release pipeline and its distribution channels. The release runbook, the step-by-step guide to a release, is [`docs/release.md`](../../docs/release.md).

## Create the repository (maintainer, once)

1. **Create the public repository `jabbott-iii/scoop-bucket`.**
2. **Add the workflow.** Copy [`sync-manifest.yml`](sync-manifest.yml) unchanged to `.github/workflows/sync-manifest.yml`. `bucket/` can start empty; the first sync pull request creates `bucket/agent-grimoire.json`.
3. **Leave out Scoop's Excavator** (the `excavator.yml` of `ScoopInstaller/BucketTemplate`), the workflow that updates a bucket's manifests automatically.
   - It pushes updated manifests straight to the default branch, with no pull request.
   - At run time it installs Scoop from an unpinned script on Scoop's `master` branch, so what it runs can change.
   - The sync workflow replaces it. The generated manifest has no `checkver` or `autoupdate` (the manifest fields that let Scoop's tools find and write new versions), so nothing else rewrites it.
4. **Set Settings → Actions → General:**
   - *Workflow permissions:* "Read repository contents and packages permissions". `GITHUB_TOKEN`, the token GitHub gives each workflow run, is then read-only by default; the sync job asks for `contents: write` and `pull-requests: write` itself.
   - Turn on *"Allow GitHub Actions to create and approve pull requests"*. Without it, `gh pr create` fails.
   - *Actions permissions:* turn on *"Require actions to be pinned to a full-length commit SHA"*. The sync workflow uses no actions, and the policy keeps any workflow added later pinned.
5. **Protect the default branch** with a ruleset (a set of GitHub branch rules) that requires a pull request. Checks on a pull request opened with `GITHUB_TOKEN` wait for approval: approve them from the pull request.
6. **In `jabbott-iii/Grimoire`, enable immutable releases** (Settings → Releases → "Enable release immutability"). With the setting on, GitHub locks each release's tag and files once it is published.
   - `gh release verify-asset` needs the release attestation that immutable releases create.
   - A release published while the setting was off has none, so the sync fails for it and opens nothing. [`docs/release.md`](../../docs/release.md) §3, on recovery, says how to publish it again.
7. **The workflow then runs daily**, and whenever you start it from the Actions tab (*Run workflow*). Until the first Grimoire release exists, each run ends with a notice that there is nothing to sync.
8. **Keep the schedule alive.** GitHub turns off scheduled workflows in a public repository after 60 days without activity, and this repository gets commits only when a sync pull request merges.
   - If a Grimoire release brings no sync pull request, enable the workflow again (Actions tab → "Enable workflow", or `gh workflow enable sync-manifest.yml --repo jabbott-iii/scoop-bucket`).
   - Then start it with `gh workflow run sync-manifest.yml --repo jabbott-iii/scoop-bucket`.

## What the workflow does

1. It finds the latest release of `jabbott-iii/Grimoire` with `gh release view`. Its tag must look like `vX.Y.Z`. If there is no release yet, it stops with a notice.
2. It downloads `agent-grimoire.json` from that release and verifies it twice:
   - `gh attestation verify --repo jabbott-iii/Grimoire --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main --source-ref refs/heads/main --deny-self-hosted-runners`.
     - This shows that Grimoire's signing workflow, `sign.yml`, attested the file in a run of `jabbott-iii/Grimoire` on its `main` branch, on a GitHub-hosted runner (a machine GitHub provides, not one the repository owner runs).
     - `--repo` and `--source-ref` pin the run, because any repository can call a public reusable workflow (one that other workflows call) and get an attestation that names `sign.yml` too.
     - Every release is attested this way: none was made before the switch to `sign.yml`.
   - `gh release verify-asset <tag> agent-grimoire.json`. This shows that the file is an asset of that immutable release.
3. If `bucket/agent-grimoire.json` on the default branch already has the same bytes, it stops.
4. If a pull request from the branch `sync/agent-grimoire-<tag>` already exists (open, closed or merged), it stops.
   - If the branch exists without a pull request, which an interrupted run can leave, and it holds exactly the verified file, the workflow opens its pull request.
   - Otherwise it warns: delete the branch to retry.
5. Otherwise it creates the commit through the GitHub API, one Git object at a time (the file's blob, a tree, the commit, then the branch), so no Git credentials are involved. Then it opens a pull request with `gh pr create`.

Runs queue one at a time and are never canceled, so a run can't stop between creating the branch and opening its pull request because another run started.

The file is committed byte for byte, so its line endings are the release's. If you add `ScoopInstaller/BucketTemplate`'s Pester tests (Pester is PowerShell's test framework), which expect CRLF (Windows line endings) on checkout, keep that template's `.gitattributes` (`* text=auto eol=crlf`) as well.

The workflow needs the GitHub CLI (command-line interface) 2.102.0 or later, which the `ubuntu-24.04` runner image has, and checks for it.

## Install (users)

Scoop needs Git to add a bucket. The first argument is the local name for the bucket, chosen by the user; `jabbott-iii` is used here.

```powershell
scoop bucket add jabbott-iii https://github.com/jabbott-iii/scoop-bucket
scoop install jabbott-iii/agent-grimoire
```

The installed command is `grimoire`.
