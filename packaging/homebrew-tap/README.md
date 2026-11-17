# Homebrew tap template

This directory holds what goes into the separate repository `jabbott-iii/homebrew-tap`. Nothing here runs in Grimoire itself; Grimoire's CI only lints [`sync-formula.yml`](sync-formula.yml) with actionlint and zizmor.

The tap has one formula, `Formula/agent-grimoire.rb`. It installs the prebuilt `grimoire` binary from the GitHub Release. The tap never writes the formula itself:

- Every Grimoire release attaches a generated `agent-grimoire.rb`, attested and signed like every other asset.
- `sync-formula.yml` copies that file from the latest release, after verifying it, and opens a pull request.

The design is in [ADR-0009](../../docs/adr/0009-release-pipeline.md), and the release runbook is [`docs/release.md`](../../docs/release.md).

## Create the repository (maintainer, once)

1. **Create the public repository `jabbott-iii/homebrew-tap`.** The name matters: `brew tap jabbott-iii/tap` clones `github.com/jabbott-iii/homebrew-tap`.
2. **Add the workflow.** Copy [`sync-formula.yml`](sync-formula.yml) unchanged to `.github/workflows/sync-formula.yml`. `Formula/` can start empty; the first sync pull request creates `Formula/agent-grimoire.rb`.
3. **Leave out `brew tap-new`'s workflows.**
   - `autobump.yml` can't update this formula. `brew bump` rewrites only a top-level `url` and `sha256`, and this formula has one archive per platform. The sync workflow replaces it.
   - `publish.yml` makes bottles, which a prebuilt binary doesn't need.
   - `tests.yml` is optional. It uses `actions/cache` and the unpinned container image `ghcr.io/homebrew/brew:main`.
4. **Set Settings → Actions → General:**
   - *Workflow permissions:* "Read repository contents and packages permissions". `GITHUB_TOKEN` is then read-only by default; the sync job asks for `contents: write` and `pull-requests: write` itself.
   - Turn on *"Allow GitHub Actions to create and approve pull requests"*. Without it, `gh pr create` fails.
   - *Actions permissions:* turn on *"Require actions to be pinned to a full-length commit SHA"*. The sync workflow uses no actions, and the policy keeps any workflow added later pinned.
5. **Protect the default branch** with a ruleset that requires a pull request. Checks on a pull request opened with `GITHUB_TOKEN` wait for approval: approve them from the pull request.
6. **In `jabbott-iii/Grimoire`, enable immutable releases** (Settings → Releases → "Enable release immutability"). `gh release verify-asset` needs the release attestation that immutable releases create. A release published while the setting was off has none, so the sync fails for it and opens nothing; [`docs/release.md`](../../docs/release.md) §3 says how to publish it again.
7. **The workflow then runs daily**, and whenever you start it from the Actions tab (*Run workflow*). Until the first Grimoire release exists, each run ends with a notice that there is nothing to sync.
8. **Keep the schedule alive.** GitHub turns off scheduled workflows in a public repository after 60 days without activity, and this repository gets commits only when a sync pull request merges. If a Grimoire release brings no sync pull request, enable the workflow again (Actions tab → "Enable workflow", or `gh workflow enable sync-formula.yml --repo jabbott-iii/homebrew-tap`) and start it with `gh workflow run sync-formula.yml --repo jabbott-iii/homebrew-tap`.

## What the workflow does

1. It finds the latest release of `jabbott-iii/Grimoire` with `gh release view`. Its tag must look like `vX.Y.Z`. If there is no release yet, it stops with a notice.
2. It downloads `agent-grimoire.rb` from that release and verifies it twice:
   - `gh attestation verify --repo jabbott-iii/Grimoire --signer-workflow jabbott-iii/Grimoire/.github/workflows/release.yml --source-ref refs/heads/main --deny-self-hosted-runners`. This shows that `release.yml` built and signed the file on Grimoire's `main` branch, on a GitHub-hosted runner.
   - `gh release verify-asset <tag> agent-grimoire.rb`. This shows that the file is an asset of that immutable release.
3. If `Formula/agent-grimoire.rb` on the default branch already has the same bytes, it stops.
4. If a pull request from the branch `sync/agent-grimoire-<tag>` already exists (open, closed or merged), it stops. If the branch exists without a pull request, which an interrupted run can leave, and it holds exactly the verified file, the workflow opens its pull request. Otherwise it warns: delete the branch to retry.
5. Otherwise it creates the commit through the GitHub API (blob, tree, commit, then the branch), so no git credentials are involved, and opens a pull request with `gh pr create`.

Runs queue one at a time and are never cancelled, so a run can't stop between creating the branch and opening its pull request because another run started.

The workflow needs the GitHub CLI 2.102.0 or later, which the `ubuntu-24.04` runner image has, and checks for it.

## Install (users)

```sh
brew install jabbott-iii/tap/agent-grimoire
```

Since Homebrew 6.0.0, a third-party tap must be trusted before Homebrew loads its formulae. Installing by the fully qualified name taps the repository and trusts only this formula, for `install`, `reinstall` and `upgrade`.

To use the short name instead:

```sh
brew tap jabbott-iii/tap
brew trust --formula jabbott-iii/tap/agent-grimoire
brew install agent-grimoire
```

- To trust the whole tap: `brew trust jabbott-iii/tap`.
- In a Brewfile: `brew "jabbott-iii/tap/agent-grimoire", trusted: true`.
