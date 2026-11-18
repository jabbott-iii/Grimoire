# ADR-0011: The container image and its supply chain

- **Status:** Accepted (the maintainer's decisions when asked, 2026-11-17, during Phase 6, revised the same day after the Phase 6 review). Amended on 2026-11-18, at the maintainer's choice: the image is signed in its own reusable workflow, `sign-image.yml`, under its own identity, because GitHub refused to start `release.yml` while one `sign.yml` signed both (decision 6).
- **Date:** 2026-11-17
- **Specs:** `BASE-IMAGE-P6`, `BUILD-P6`, `REGISTRY-P6`, `SCAN-P6`, `RUN-P6`, `SIGN-P6` in [`../specs.md`](../specs.md)
- **Amends:** [ADR-0002](0002-mcp-server-and-tool-packaging.md) (the image's exact form; `runtime = "docker"` is deferred past v1), [ADR-0004](0004-package-and-binary-names.md) (the image name must be lowercase, which settles its Phase 6 check), [ADR-0009](0009-release-pipeline.md) (a fifth environment, `ghcr`; four new jobs; `sign-files` replaces `attest` and `attest-sbom`, and every signature moves into `sign.yml`, under a new identity (the image's into `sign-image.yml` since 2026-11-18); PR dry-runs also build `aarch64-unknown-linux-musl`) and [ADR-0010](0010-language-policy-exceptions.md) (a seventh exception, `.github/scripts/image-tags.sh`)
- **Related:** [`docs/container.md`](../container.md) shows how to run and verify the image. [`docs/release.md`](../release.md) is the runbook.

## Context

`PLAN.md` asked Phase 6 for a two-platform image (linux/amd64 and linux/arm64) of the static musl binaries on `gcr.io/distroless/static-debian13:nonroot`, pinned by digest, with the base's signature checked by cosign. The plan also asked for a non-root user, no shell and OCI annotations, plus a BuildKit SBOM, max provenance, a cosign signature and a Grype gate. ADR-0002 put every asset binary in the image, and left its exact form and `runtime = "docker"` to this phase.

Phase 6 researched five topics, each re-checked by a skeptic (`*-P6`). These findings shaped the design:

- **The runners' Docker is too old.** `ubuntu-24.04` and `ubuntu-24.04-arm` ship Docker 28.0.4, which embeds BuildKit v0.20.2. Its `docker` driver offers attestations, multi-platform builds and OCI export only on the containerd image store, which Docker turns on by default only from 29.0, and only on fresh installs. That the runners use the classic store is inferred: `docker info` hasn't been run on a runner. The runners' buildx plugin is installed with no checksum check. The 20261116 images, which carry buildx 0.39.0 (too young for the 7-day rule), began rolling out on 2026-11-17, so a job may get 0.38.2 or 0.39.0 until the rollout ends; either way the workflow installs its own pinned copy.
- **GHCR has no referrers API.** It answers `/v2/<name>/referrers/<digest>` with 404 `MANIFEST_UNKNOWN`. Clients then use the referrers tag schema: an index tagged `sha256-<subject hex>` lists the signatures and attestations. cosign and `actions/attest` both rewrite that one tag, and cosign's go-containerregistry does so with no conditional request. `cosign verify` silently falls back to legacy `.sig` signatures when it finds no new-format bundle.
- **BuildKit pulls its SBOM scanner by tag.** `--attest type=sbom` runs `docker/buildkit-syft-scanner:stable-1`, resolved on every build. buildx's opt-in default policy doesn't check that tag.
- **buildx embeds the GitHub event payload in the provenance.** The `docker-container` driver's `provenance-add-gha` defaults to `true`. It then writes the whole event payload, the actor, the run and the runner into every provenance, in min mode as well as max. It holds no tokens, but the provenance is public.
- **`anchore/scan-action` pipes an install script to `sh`.** v7.4.2 downloads Grype's `install.sh` and runs it with `sh`. It pins no hash and checks no signature. Its default Grype, v0.118.0, embeds a syft below the fix for GHSA-mw2c-m758-9v5q. `PLAN.md` §1 had planned to use it.
- **Dependabot's Docker cooldown fails open for gcr.io.** Dependabot dates a Docker update only from Docker Hub's `tag_last_pushed`. For any other registry it allows the update at once, with a `cooldown_date_unavailable` warning. It also opens no security updates for Docker.
- **No MCP client offers a common variable for the project directory.** Cursor has `${workspaceFolder}`. Claude Code's project `.mcp.json` has none: only plugin MCP configs substitute `${CLAUDE_PROJECT_DIR}`. Codex has no interpolation. Gemini CLI's `${PWD}` works only if the shell exports it. VS Code's native config has no workspace variable.
- **Repository names are lowercase.** OCI distribution-spec v1.1.1 allows only lowercase, and go-containerregistry, inside cosign, crane and gh, rejects uppercase. `github.repository` is `jabbott-iii/Grimoire`.
- **The newest distroless digest was too young.** The current `:nonroot` index, `sha256:aafe9936…`, was first uploaded on 2026-11-12T21:16:35Z, so it fails the 7-day rule until 2026-11-19T21:16:35Z. The index before it differs only in the mode of `/usr/lib/os-release`.

An adversarial review of the Phase 6 change, in five areas (workflows, the xtask code, the image and its supply chain, docs, specs), then raised 39 findings. An independent skeptic re-checked each one: 37 were confirmed (1 high, 8 medium, 28 low) and 2 refuted. All 37 are addressed. The high one was `check`'s lax layer reader: a second gzip member, or a directory entry with a size, could install another binary unseen. A medium one was that every job in `release.yml` with `id-token` got the same certificate identity, `release.yml@refs/heads/main`. A sixth research topic, `SIGN-P6`, then checked signing from a reusable workflow:

- **Anyone can get the same certificate name.** A public repository's reusable workflow can be called from any repository. The certificate's SAN names the called file and ref; the caller goes into other fields, such as OIDs 1.5 (repository) and 1.6 (ref).
- **Neither verifier can pin the caller's file.** cosign and gh have no flag for OID 1.18 (the Build Config URI, the top-level caller file) or 1.23 (the environment). `gh attestation verify --format json` exposes `buildConfigURI`, so `jq` can check it.
- **actionlint 1.7.12 rejects the `$/` call form** (issue #711), and zizmor 1.32.0's `self-repository` audit flags the `./` form (Low, regular persona).

That led to decisions 6 and 7, and to the changes marked "after the review" below.

## Decision

### Maintainer decisions (2026-11-17)

1. **Local tooling for Phase 6.** Rootless Docker, run as the maintainer's user and stopped at the end; Grype, cosign and crane in the scratchpad, each checked against its published SHA-256. Later also the `aarch64-unknown-linux-musl` Rust target, to build real arm64 binaries locally.
2. **`runtime = "docker"` (ADR-0002 item 5) is deferred past v1.** [`docs/container.md`](../container.md) documents hand-written, digest-pinned `docker run` entries instead, and `grimoire.toml` keeps rejecting the field. The reasons:
   - no client variable names the project directory (above), so each client would need its own form;
   - the CLI can't embed the digest of the image that contains it.
3. **The Grype gate fails on any medium, high or critical match, fixed or not.** A finding with no fix blocks until `.grype.yaml` has an ignore rule for it with a reason, reviewed like the exceptions in `deny.toml`.
4. **A release pushes two tags: `X.Y.Z` and `latest`.**
5. **Pull request dry-runs also build `aarch64-unknown-linux-musl`.** Every PR then builds, scans and runs the two-platform image, and runs an arm64 install test. This adds a fourth PR target to ADR-0009's three.

### Maintainer decisions after the review (2026-11-17)

6. **A separate signing workflow, `.github/workflows/sign.yml`.**
   - Every cosign signature and attestation, for the Phase 5 release files and for the image, is made in `sign.yml` (`on: workflow_call`). Only `release.yml`'s `sign-files` and `sign-image` jobs call it.
   - **Why:** every job in `release.yml` with `id-token` got the same certificate identity, `release.yml@refs/heads/main`. That includes `publish-pypi`, whose tag-pinned third-party container already gets Sigstore certificates for PEP 740 attestations. A re-pushed image of that container could have signed any image or file so that it passed the documented verification.
   - Certificates now name `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`, and every verifier also pins the caller (under Forced by research).
   - No release exists yet, so no release was ever signed under the old identity: every release is signed by `sign.yml`.
   - **Amended on 2026-11-18** (the maintainer's choice when asked): the image's job moved into a second reusable workflow, `.github/workflows/sign-image.yml`, which only `sign-image` calls. `sign.yml` keeps the release files' `files` and `sboms` jobs, and the `target` input is gone.
     - **Why:** GitHub checks every job of a called workflow against the calling job's permissions when a run starts, even a job whose `if:` would skip it. With one file, `sign-files` had to grant the image job's `contents: read` and `packages: write`. It didn't, so GitHub refused every Release run, pull requests' dry-runs included (`SIGN-P6`, re-checked 2026-11-18). With two files, each call grants exactly what its jobs ask for.
     - The image's certificates name `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main`. The release files' still name `sign.yml`. No release exists yet, so no image was ever signed by `sign.yml`.
     - **Rejected:** granting `sign-files` the image job's permissions too. No running job's token would change, but the files call would hold `packages: write`, with only the `if:` guards keeping it from the image job.
     - Neither zizmor nor actionlint checks a call's grant, so an xtask test, `xtask/src/workflow_permissions.rs`, now checks every reusable-workflow call in `.github/workflows/`.
7. **`.github/scripts/image-tags.sh` is a seventh language-policy exception.** This amends [ADR-0010](0010-language-policy-exceptions.md), whose six exceptions were the only ones.
   - **Language:** Bash, with `set -euo pipefail` and `shopt -s inherit_errexit`, so a failed lookup inside a command substitution still stops the script. It needs crane and `jq`.
   - **Why not Rust:** cargo must not run in a job that can push or mint OIDC tokens (under Chosen), and a prebuilt xtask would put all its dependencies in the signing job.
   - **Scope:** the image's registry steps, in three subcommands, and nothing else:
     - `push IMAGE DIGEST VERSION LAYOUT` pushes the layout by digest, untagged, and checks that crane pushed that digest;
     - `tag IMAGE DIGEST VERSION` tags the version;
     - `latest IMAGE DIGEST VERSION -- VERIFY...` moves `latest` to the digest, unless `latest` names a newer release: then it leaves `latest` if `VERIFY` accepts that release, and stops with an error if not.
   - **Rules:**
     - A `VERSION` tag that names another digest is an error, never overwritten. `push` and `tag` both check.
     - A lookup that fails for any reason other than the registry's `MANIFEST_UNKNOWN` or `NAME_UNKNOWN` stops the script, so a tag it couldn't read is never treated as absent.
     - `latest`'s version annotation is trusted only if `VERIFY` (`cosign verify` with the image's signing identity, `sign-image.yml`, and the caller pins) accepts `latest`'s digest. Otherwise anyone who can write to the package could keep `latest` by claiming a high version. `latest` never moves back to an older signed release.
   - **Size:** one file, 111 lines. Growth beyond the scope above needs a change to this ADR.
   - **Checks:**
     - ShellCheck v0.11.0, pinned by SHA-256, in `security.yml`'s `workflows` job. An xtask test keeps its pin equal to `release.yml`'s.
     - The release (`publish-image`, then `sign-image.yml`) and every PR dry-run (`verify-image`, against a local registry, with the throwaway key) run the same script. So the code that guards the release tags never runs for the first time in a release.
     - The dry-run covers a push and a tag, each twice; a version tag that names another digest, refused by `push` and by `tag`; a registry that can't be read, refused; `latest` moved off an unsigned image that claims no newer version; `latest` refused for an unsigned image that claims a newer one; and `latest` kept on a newer signed release.

### Chosen

- **One image holds the CLI and every asset binary** (ADR-0002 item 3):

  | Path | What |
  |---|---|
  | `/usr/local/bin/grimoire` | The entrypoint: `ENTRYPOINT ["/usr/local/bin/grimoire"]`, `CMD ["--help"]` |
  | `/opt/grimoire/bin/` | Every `grimoire-mcp-<id>` and `grimoire-tool-<id>`, off `PATH`. Start one with `--entrypoint /opt/grimoire/bin/<bin>` |
  | `/repo` | `WORKDIR`, where a repository is mounted |

  It runs as `USER 65532:65532`, distroless's nonroot user, stated in the Dockerfile.
- **COPY only.** The Dockerfile has no `RUN`, `ADD` or `ONBUILD`, so nothing executes during the build. One `ubuntu-24.04` runner builds both platforms, with no QEMU.
- **The OCI layout is built once, then pushed byte for byte.** The `image` job writes it (`type=oci,tar=false,rewrite-timestamp=true`, with `SOURCE_DATE_EPOCH` set to the commit time). `cargo xtask release image check` checks it, and Grype scans it. `publish-image` pushes that same layout by digest, untagged, with `image-tags.sh push`, which checks that the pushed digest is the checked one. The registry then holds exactly the bytes that were scanned, under the digest the release notes name.
- **What `check` requires** of the layout (made stricter after the review):
  - exactly one index in `index.json`, with the two platform manifests and one attestation manifest each, holding an SPDX SBOM and a SLSA v1 provenance with no GitHub event payload;
  - the exact annotations, and an image config with exactly the expected top-level keys and exactly the fields the Dockerfile sets (user, working directory, entrypoint, command, the one label, `ArgsEscaped`), the verified base's `Env` unchanged, and the base's `diff_ids` first, then one per layer. The workflow fetches each base platform's config with `crane blob`, after verifying that platform's signature;
  - lower layers that are exactly the verified base manifest's layers, whose bytes in the layout match them;
  - layers on top that hold only the planned directories (`usr/`…`usr/local/bin`, `opt/`…`opt/grimoire/bin`, root-owned, and `repo`, owned by 65532, all 0755) and exactly the release binaries (mode 0755, owned by root, the same SHA-256 as the archived binary);
  - layers read as strictly as Docker and containerd read them: one gzip member and nothing after it, uncompressed bytes matching the config's `diff_id`, no PAX or GNU extension records, no size on a directory, and only zeros after the tarball's end. A reviewer showed that a laxer reader let a second gzip member or a sized directory install another binary unseen;
  - the right digest and size for every blob.
- **Provenance in `mode=max`**, with `builder-id` set to the run's URL. Max mode records every build argument and the Dockerfile. No build argument is secret: the only one is `SOURCE_DATE_EPOCH`.
- **Annotations computed by xtask**, on the index (`index:`) and on each platform manifest (`manifest[linux/<arch>]:`): `org.opencontainers.image.{created, description, documentation, licenses, revision, source, title, url, version, base.name}`. `base.digest` is the base index digest on the index, and each platform's base manifest digest on that platform's manifest. BuildKit and buildx never set the two `base.*` keys themselves.
- **Sign the index only.** `cosign sign --yes` signs the index digest, without `--recursive`. A platform manifest digest doesn't verify on its own.
- **Push by digest, then sign, then tag** (after the review). `publish-image` pushes the image by digest, untagged, and holds no OIDC token. `sign-image` then signs, attests and verifies it in `sign-image.yml`, and only then tags `X.Y.Z` and moves `latest`. So neither tag ever names an image that isn't signed, attested and verified.
- **The version tag never moves.** A different digest under `X.Y.Z` is an error, never an overwrite; the same digest is skipped. `image-tags.sh` checks before the push and again before the tag.
- **`latest` never moves back to an older release, and a newer one must carry the image's release signature** (after the review). `latest` stays where it is if it names a newer version and its digest verifies with the image's signing identity and the caller pins. If it names a newer version that doesn't verify, the job stops for the maintainer to investigate and doesn't move `latest`. An image with an older version, or none, is replaced.
- **No cargo in a job that can push or mint OIDC tokens**, because a dependency's build script could use either. `image` runs xtask with `contents: read` only. `publish-image`, `sign.yml` and `sign-image.yml` run no cargo. `publish-image` uses crane, `jq` and `image-tags.sh`; `sign-image.yml`'s image job uses crane, cosign, gh, `actions/attest`, `jq`, `curl` and the same script.
- **A weekly rescan.** `security.yml`'s `image-scan` job runs on its weekly schedule and on demand. It finds the latest release with `gh release view`, resolves its version tag to a digest, and checks the image's signature on that digest, under `sign-image.yml`'s identity, with the caller pins. It then runs the same Grype gate on every platform in the index, from the registry. While there is no release, it passes with a notice. When the version tag can't be resolved, it names a missing or private image as the cause. An xtask test fails when its pins differ from `release.yml`'s, or its identity from `sign-image.yml`'s.
  - After the review, `security.yml`'s concurrency group includes the event name, so a push to `main` can't cancel the weekly scan.

### Forced by research

- **The image name is the literal `ghcr.io/jabbott-iii/grimoire`**, never built from `github.repository`. Lowercase is mandatory (distribution-spec v1.1.1). This settles ADR-0004's open check.
- **Asset binaries stay off `PATH`.** Inside a container, `grimoire doctor` and the `plan/hook-binary-missing` warning read the container's `PATH`, not the host's. A binary on the image's `PATH` would count as installed when the host lacks it.
- **`WORKDIR /repo`.** The base's working directory is `/home/nonroot`, mode 0700, so any other `--user` would start in a directory it can't read.
- **The source label.** `LABEL org.opencontainers.image.source="https://github.com/jabbott-iii/Grimoire"` links the GHCR package to the repository on its first push. A package inherits the repository's access permissions only if it is linked before its first publish. Whether GHCR reads the link from a multi-platform image's config label or from its index annotation is UNVERIFIED, so the image carries both.
- **The Dockerfile rules.** `cargo xtask release image context` allows only `FROM`, `ARG`, `COPY --chmod=0755 <src> <dst>`, `WORKDIR`, `USER`, `LABEL`, `ENTRYPOINT` and `CMD`, so no `RUN`, `ADD`, `ONBUILD` or `COPY --from` (an allow-list since the review). It also refuses a `# syntax=` or `# escape=` directive, line continuations and heredocs. It requires exactly one `FROM name:tag@sha256:…`, on one line, with no flags or stage name: that is the form Dependabot parses.
  - With no `# syntax=` line, the frontend is the one inside the digest-pinned BuildKit image, so nothing is pulled by tag.
  - The first line is `# check=error=true`, so a build-check warning fails the build. `context` (through `parse_dockerfile`) refuses a Dockerfile whose first line isn't that, and a unit test covers it. Checked locally on the pinned BuildKit v0.34.2 with no `# syntax=` line: with the directive a `FromAsCasing` warning fails the build; without it the build only warns.
- **The base is pinned and verified before every build.**
  - The pin is `gcr.io/distroless/static-debian13:nonroot@sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329`, the newest index at least 7 days old (first uploaded 2026-10-31). Its linux/amd64 manifest is `sha256:336e53e9941facdb69b755e6fd2c63f5400abd3b1ba47bb17ebafc2b1547cebd`, and its linux/arm64/v8 manifest `sha256:bf6233b64054d17b31c3b555b1bbcee58958896830a305ef4d51faa257990d87`.
  - It holds base-files, ca-certificates, media-types, netbase, tzdata and tzdata-legacy (Debian 13.7, per Grype). No shell, no libc.
  - `crane manifest` fetches the index. `context` checks that its bytes hash to the pinned digest, and that it lists exactly one linux/amd64 and one linux/arm64 manifest.
  - `cosign verify` then checks distroless's keyless signature on the index and on both platform manifests, with `--certificate-identity keyless@distroless.iam.gserviceaccount.com --certificate-oidc-issuer https://accounts.google.com`.
  - `context` refuses any binary that isn't a static 64-bit ELF executable for the platform (no program interpreter), because the base has no libc or dynamic loader.
- **A `docker-container` builder on a pinned BuildKit.**
  - buildx v0.38.2 is installed checksum-verified into a job-scoped `DOCKER_CONFIG`, so it wins over the runner's copy.
  - A plain `docker buildx create` runs `moby/buildkit:v0.34.2@sha256:c5f8862f52499773af85bc07695b2379ef08e8002352b0df2c4864c91d78dabe`. It doesn't use setup-buildx-action, which adds two insecure entitlements unless `buildkitd-flags` is set, and downloads buildx without checking its signature.
  - `docker buildx build --check` runs before the build.
- **`--driver-opt provenance-add-gha=false`.** `builder-id` names the run instead. `check` refuses a provenance that holds `github_event_payload`.
- **The SBOM scanner is pinned:** `--attest type=sbom,generator=docker/buildkit-syft-scanner:1.12.1@sha256:873a7458c3c8ac8e32c7b796c10e1600b07b963d2aa000a72a2d97aaaa2d6315`. It bundles syft v1.51.0 and writes SPDX 2.3.
- **Grype comes from its release tarball, not from scan-action.**
  - Grype v0.121.3 is pinned by SHA-256 in the workflow, like every other CI tool.
  - Each platform is scanned on its own (`oci-dir:` with `--platform`), with `-c .grype.yaml --fail-on medium`. Every platform is scanned before the step fails, so the uploaded reports show every finding.
  - Grype ignores unknown config keys silently, also inside an ignore rule, where a misspelt criterion widens the rule. So `--fail-on` repeats the threshold, and an xtask test reads `.grype.yaml` line by line: it refuses any unknown key and any rule without a GHSA or CVE id, a package name and a reason.
  - `.grype.yaml` sets `fail-on-severity: medium`, `only-fixed: false`, `check-for-app-update: false`, `db.validate-age: true` with `db.max-allowed-built-age: 120h`, and `ignore: []`. An ignore rule names the vulnerability and the package and gives a reason. Rust matches use GHSA IDs.
  - `cargo xtask release image coverage` fails if Grype didn't detect Debian 13, found no Debian package, or shows no Rust package located at some binary. Grype reads crates only from cargo-auditable data, and reports a binary without it as empty, not as unknown; a base whose distribution it can't detect would match no Debian package at all.
  - The job logs `grype db status`.
- **crane pushes the layout.** crane v0.22.1, checksum-verified, pushes a buildx OCI layout with its index digest unchanged. In `verify-image` it also serves the local registry, `crane registry serve` on `127.0.0.1:5000`, which, like GHCR, has no referrers API.
- **Sign, then attest, then check the fallback tag**, in `sign-image.yml`'s `image` job.
  - Both write the same `sha256-<index hex>` tag, so they run one after the other in one job, never at once.
  - cosign signs the index unless it already verifies with the image's signing identity and the caller pins.
  - **A re-run looks for the attestation in GHCR** (after the review). `actions/attest` writes to the attestations API before it pushes to the registry, and has no "already done" check. So `gh attestation verify --bundle-from-oci` decides whether to attest: an attestation that reached GitHub's API but not GHCR is attested again, which adds a harmless API record. This fixes a re-run deadlock the review found.
  - `actions/attest` v4.2.2 runs with `subject-name`, `subject-digest`, `push-to-registry: true` and `create-storage-record: false`, because storage records exist only for organisation-owned repositories.
  - `actions/attest` reads registry credentials only from `~/.docker/config.json`, so the job logs in there with `crane auth login`, using `GITHUB_TOKEN`, and checks that the login is in that file.
  - Then the job reads the fallback tag and requires both a cosign signature bundle (`https://sigstore.dev/cosign/sign/v1`) and a SLSA provenance bundle (`https://slsa.dev/provenance/v1`). It runs `cosign verify` and `gh attestation verify --bundle-from-oci`, both with the caller pins. A passing `cosign verify` alone doesn't prove the bundle is there, because it falls back silently to legacy `.sig` signatures.
- **Every job in `sign.yml` and `sign-image.yml` runs only for `release.yml` on `main` in this repository** (after the review). Each has `if: github.repository_id == '1460666977' && github.workflow_ref == 'jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main'`. So an external caller, another workflow of this repository, or a pull request run never mints a token there.
- **Every verifier pins the caller** (after the review), because the certificate's name alone doesn't identify Grimoire's run:
  - cosign: `--certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` for the release files, or `…/sign-image.yml@refs/heads/main` for the image, with `--certificate-oidc-issuer https://token.actions.githubusercontent.com --certificate-github-workflow-repository jabbott-iii/Grimoire --certificate-github-workflow-ref refs/heads/main`;
  - gh: `--repo jabbott-iii/Grimoire --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main --source-ref refs/heads/main --deny-self-hosted-runners`, with `sign-image.yml` in place of `sign.yml` for the image. `--repo` alone accepts any workflow and ref of the repository, and `--signer-workflow` without the ref would also accept the signing workflow at one of our release tags, which any repository can call.
  - Neither can pin the caller's file, and with `--bundle-from-oci` gh compares names only. So `publish` (for `SHA256SUMS`) and `sign-image.yml` (for the image) also check, with `gh attestation verify --format json` and `jq`, that the provenance certificate has `buildConfigURI` `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, `sourceRepositoryIdentifier` `1460666977` and `sourceRepositoryOwnerIdentifier` `84321953`.
- **The calls are `uses: ./.github/workflows/sign.yml` and `uses: ./.github/workflows/sign-image.yml`, each with `# zizmor: ignore[self-repository]`** (after the review). zizmor's fix is the `$/` form, which actionlint 1.7.12 rejects.
  - A calling job can't set `environment`, `runs-on`, `env` or `outputs`. It grants permissions, which the called jobs can only reduce. GitHub refuses the run if a called job asks for more (decision 6, amended).
  - Each has `permissions: {}`, `defaults.run.shell: bash`, its own `env` and no workflow-level concurrency. Their issuer, repository and ref pins equal `release.yml`'s, as do `sign-image.yml`'s image and crane pins, and each names its own identity. An xtask test checks them. Inputs reach `run` steps only through `env`.
  - In the provenance, `runDetails.builder.id` names the signing workflow, `sign.yml` or `sign-image.yml`, and `externalParameters.workflow.path` stays `.github/workflows/release.yml`, the caller.
- **Dependabot gets a `docker` entry** for the root Dockerfile, weekly, with `cooldown: default-days: 7`. The cooldown has no effect for gcr.io, so the maintainer checks a new digest's age before merging (under Consequences).
- **The image job in `sign-image.yml` ends with an anonymous-pull check.** A new package can be private until the maintainer makes it public, and only a public package serves anonymous `docker run`, cosign and Grype pulls.

### The jobs

`.github/workflows/release.yml` gains four jobs. Two of them call the new reusable workflows, `.github/workflows/sign.yml` and, since the 2026-11-18 amendment, `.github/workflows/sign-image.yml`:

| Job | When and where | Permissions | What it does |
|---|---|---|---|
| `image` | Dry-run and release, `ubuntu-24.04` | `contents: read` | Checks `dist/` against the package job's hash. Installs buildx, crane and Grype (checksum-verified) and cosign. Verifies the base, fetches its manifests and configs, creates the builder, runs `--check`, builds, checks and scans. Uploads the scan reports and the layout (`image-layout`), and outputs the index digest. Pulls are anonymous |
| `verify-image` | Dry-run and release; linux/amd64 on `ubuntu-24.04`, linux/arm64 on `ubuntu-24.04-arm` | `contents: read`, to check out `image-tags.sh` | Checks out the commit, checks the layout's index digest, pushes it to the local registry and checks the digest is kept, and pulls it on the runner's Docker. Runs `grimoire --version` and every asset binary's `--version` with `--network none --read-only --cap-drop ALL --security-opt no-new-privileges`. Runs `init`, `add conventional-commits --apply` and `verify` on a bind-mounted repository with `--user "$(id -u):$(id -g)"`, and checks file ownership. Sends an MCP `initialize` over stdio to `grimoire-mcp-project-docs`, with a read-only mount and `--root /repo` |
| `publish-image` | Release only, environment `ghcr`, after `publish` | `contents: read`, `packages: write`; no `id-token` | Checks out the commit, checks the layout's digest, installs crane, logs in, pushes the layout by digest, untagged (`image-tags.sh push`), and logs out |
| `sign-image` | Release only, after `publish-image`. It calls `sign-image.yml`, and can't name an environment: the approved step was the push | Grants `contents: read`, `packages: write`, `id-token: write`, `attestations: write` | See `sign-image.yml`'s `image` job |

`sign.yml` and `sign-image.yml` (`on: workflow_call`) have three jobs between them. Each runs only when `release.yml` on `main` in this repository calls it (the guard above):

| Job | Called by | Permissions | What it does |
|---|---|---|---|
| `sign.yml`'s `files` | `sign-files` | `id-token: write`, `attestations: write` | Checks `dist/` against the package job's hash. One provenance attestation over `dist/*`, then a keyless cosign bundle per file, each verified at once with the caller pins. Uploads the bundles (output `sigstore-artifact-id`) |
| `sign.yml`'s `sboms` | `sign-files`; one job per archive | `id-token: write`, `attestations: write` | Checks `dist/` the same way, and attests the archive's CycloneDX SBOM |
| `sign-image.yml`'s `image` | `sign-image` | `contents: read`, `packages: write`, `id-token: write`, `attestations: write` | Checks the inputs, installs crane and cosign, and logs in to GHCR. Signs the index (skipped if it already verifies), attests it (skipped if `--bundle-from-oci` finds an attestation), checks the fallback tag, and verifies both with the caller pins. Only then runs `image-tags.sh tag` and `latest`. Logs out, and checks the anonymous pull |

- **On dry-runs**, the amd64 leg of `verify-image` also signs the image offline, with a throwaway key and a signing config that names no services. It checks that the signature lands in the `sha256-<index hex>` fallback tag, and that an unsigned platform manifest digest fails to verify. Then it runs `image-tags.sh`'s cases against the local registry (decision 7). Neither signing workflow runs on a PR: their guards stop them, and PRs from forks get no `id-token`. Its steps first run in a real release, like the Phase 5 attest jobs' steps.
- **`sign-files`** replaces ADR-0009's `attest` and `attest-sbom` jobs. It calls `sign.yml`, and grants `id-token: write` and `attestations: write`.
- **`publish`** needs `sign-files` in place of the attest jobs, and now also `image` and `verify-image`. It verifies every cosign bundle and the provenance of `SHA256SUMS` with the new identity and the caller pins, plus the `buildConfigURI` check. It passes the index digest to `cargo xtask release notes --image-digest`. The notes name `ghcr.io/jabbott-iii/grimoire@sha256:…`, its two tags, and `docs/container.md`.
- **`release checks`** now also needs `image` and `verify-image`.
- **Environment `ghcr`** is named only in release mode, like the other four. The maintainer approves it in the second round, together with `crates-io`, `npm` and `pypi`.
- **Re-runs:** every step of `publish-image` and of `sign-image.yml`'s `image` job skips what is already done, so re-running the failed jobs finishes a partial push, signature or tag.
- **New pins** in `release.yml`'s `env`: `IMAGE`, `BUILDX_VERSION` and its SHA-256, `CRANE_VERSION` and its two SHA-256s, `GRYPE_VERSION` and its SHA-256, `BUILDKIT_IMAGE`, `SBOM_SCANNER_IMAGE`, and distroless's identity and issuer. `sign-image.yml` repeats the image name and the crane pin, and each signing workflow names its own identity.
- **`actions.lock`** was regenerated with gh-actions-lock v0.4.1 `--no-narrow`: `sign.yml` gets its own entry, `security.yml` gains `sigstore/cosign-installer`, and `release.yml` loses `actions/attest`. With the 2026-11-18 amendment, `sign-image.yml` has its own entry too, and `sign.yml`'s no longer lists `actions/checkout`. The maintainer regenerated it the same way (`f69c0d0`), and `--verify` passes.

### What ran locally, and what didn't

Nothing has run on GitHub yet. These were checked locally during Phase 6, with rootless Docker 29.9.3:

- **Reproducibility:** three builds, one with a fresh builder, gave identical platform manifest digests. The index digest differed each time.
- **`check`** passes on a real build, and refuses a changed binary and a corrupted blob. After the review it also refused all seven of a reviewer's tampered layouts (a second gzip member, a sized directory hiding a header, an extra directory, an xattr, an added `OnBuild` and `Env`, a swapped base layer and a wrong `diff_id`), including the two that had passed before and run another binary. Unit tests build a small layout like BuildKit's and change one thing at a time.
- **`image-tags.sh`:** every case the PR dry-run covers (decision 7) also ran locally: a push and a tag, each twice; a version tag that names another digest, refused by `push` and by `tag`; a registry that can't be read, refused; `latest` moved off an unsigned image; and `latest` kept on a newer signed release.
- **Tests:** `cargo test --workspace --locked` passes, 454 tests, 77 of them in xtask.
- **The event payload:** with the Actions environment simulated, the provenance held it with the default and didn't with `provenance-add-gha=false`.
- **Grype** v0.121.3, with DB v6.1.10 built 2026-11-17T06:20:58Z:
  - no findings on either platform today;
  - a medium GHSA match exits 2, and an ignore rule keyed on the GHSA ID and the package name suppresses it (exit 0);
  - unknown config keys are silently ignored;
  - the DB takes 3.3 GB unpacked, from a download of about 203 MB.
- **`coverage`:** 92 Rust packages in `grimoire` (91 on arm64), 135 in `grimoire-mcp-project-docs` and 41 in `grimoire-tool-secret-guard`. So cargo-auditable's data survives the `dist` profile. Grype detected Debian 13.7, with 6 Debian packages.
- **Signing:** cosign 3.1.4 verifies distroless's index and both platform manifests with the identity above. crane v0.22.1 pushes the layout with the index digest unchanged. Pushing creates `sha256-<platform hex>` fallback tags for BuildKit's attestation manifests, and cosign's signature lands in the `sha256-<index hex>` tag.
- **Running:** `grimoire --version`, both asset binaries' `--version`, `init`, `add --apply` and `verify` on a bind mount, an MCP `initialize` over stdio and `grimoire-tool-secret-guard scan` on stdin, all with the hardening flags above.

Not run locally: the arm64 image (built and scanned, but there is no emulation here to start it), rootful Docker, Docker Desktop, Podman, GHCR, and any GitHub Actions run, so no keyless signature from `sign.yml` or `sign-image.yml` and no `actions/attest` call. Still UNVERIFIED from the research:
- the certificates `sign.yml` and `sign-image.yml` get: their values are derived from GitHub's docs and other projects' certificates, not observed (`SIGN-P6`);
- whether GHCR sends `OCI-Subject` and honours `If-Match`;
- the first visibility of a package a workflow creates in a user account (GitHub's docs conflict);
- whether GHCR shows `source` and `licenses` from index annotations, and accepts an SPDX expression for `licenses`;
- whether fallback indexes and referrer manifests appear as package versions, and whether a cleanup of untagged versions deletes them;
- ownership on rootless Podman (`--userns=keep-id`) and on Docker Desktop for macOS and Windows.

## Consequences

**Positive:**
- The registry holds the bytes that were checked, scanned and run, under the digest the release notes name.
- Nothing executes during the build, so no platform needs emulation.
- Every input is pinned: the base, BuildKit and the SBOM scanner by digest; buildx, crane and Grype by SHA-256.
- No credential is stored. GHCR uses the job's `GITHUB_TOKEN`, and signing is keyless.
- The public provenance holds no event payload or runner details.
- Anyone with Docker can run every MCP server and tool without installing them.
- Platform manifests are reproducible from the same binaries and commit.
- Only `sign.yml` and `sign-image.yml`, called by `release.yml` on `main`, make signatures and attestations that the documented checks accept. The publish jobs, and the third-party code they run, can't sign as the release.
- Neither image tag ever names an image that isn't signed, attested and verified.
- The registry steps that guard the release tags run on every pull request, so they never run for the first time in a release.

**Negative and risks:**
- **A fifth environment.** `ghcr` needs a required reviewer and deployment branch `main` only, created during the bootstrap ([`docs/release.md`](../release.md) §2). It joins the second approval round.
- **Making the package public is a manual, one-time step that can't be undone.**
  - No REST endpoint changes visibility, and GitHub's docs disagree on a workflow-created package's first visibility. Either way, the last step of `sign-image.yml`'s `image` job fails until the package is public. Then a re-run finishes.
  - After the first push, the maintainer also checks that the package is linked to `jabbott-iii/Grimoire`.
  - The maintainer must never push to `ghcr.io/jabbott-iii/grimoire` by hand before the first release. `GITHUB_TOKEN` can't push to a package in the namespace that isn't connected to the repository.
- **The base digest's age is checked by hand.** Dependabot proposes a new gcr.io digest at once. Before merging, the maintainer reads its upload time from GCR's tag list: `curl -s https://gcr.io/v2/distroless/static-debian13/tags/list | jq '.manifest["sha256:<digest>"].timeUploadedMs'`. `timeUploadedMs` is undocumented (UNVERIFIED); the research found it matched the Rekor times. A CI check of the age is a follow-up.
- **Two more image pins to bump by hand.** Dependabot can't see `BUILDKIT_IMAGE` or `SBOM_SCANNER_IMAGE`, nor the buildx, crane and Grype versions and checksums. They follow `CI-PINS`: the 7-day rule, and each checksum re-verified. `security.yml` and `sign-image.yml` repeat the image name, the crane pin and the image's signing identity, and `security.yml` also the Grype and ShellCheck pins. An xtask test fails when they differ.
- **The image index isn't reproducible.** The SBOM and provenance attestations hold build times and IDs, so every build has a new index digest. The release run's digest is the one signed; a PR's index digest differs from it.
- **Only the index is signed.** Users pin and verify the index digest, as [`docs/container.md`](../container.md) shows. `cosign verify` of a platform manifest digest fails.
- **GHCR shows `sha256-*` tags.** They hold the signature, the attestation and BuildKit's attestation manifests. Never delete them. Deleting one removes a signature: a denial of service, not a forgery, so verifiers must fail closed. Anyone with write access to the package could do it.
- **The Grype DB is the one unpinned input.** It is rebuilt daily, unsigned, and checked only against the SHA-256 in an HTTPS listing. `validate-age` fails the scan when the DB is more than 120 hours old. A release run can fail where its PR passed, if an advisory lands in between; the release run is the one that counts.
- **Grype sees Rust crates only through cargo-auditable data.** `coverage` catches a binary without it. musl libc, linked statically, is invisible (an inference from cargo-auditable's docs). cargo-deny stays the main Rust advisory gate. Grype adds the Debian packages and a check of the built binaries.
- **The attached SBOM and the gate can disagree.** The image's SBOM comes from syft v1.51.0 inside the pinned scanner; Grype v0.121.3 embeds syft v1.57.1.
- **The release notes name the digest, but the notes aren't attested.** `publish-image` runs after the GitHub Release is published. Release notes stay editable, and GitHub's release attestation covers the tag, the commit and the assets, not the notes. The image's own signature and attestation are what users check. Until `publish-image` succeeds, the notes name a digest GHCR doesn't serve yet; until `sign-image` succeeds, that digest is unsigned and untagged.
- **Verifiers must pin the caller.** The certificate names `sign.yml` or `sign-image.yml`, and any repository can call a public reusable workflow and get a certificate with the same name. A check without the caller pins accepts a signature from any caller. Every command in the docs carries them.
- **No verifier can pin the caller's file or an environment.** cosign and gh have no flag for OID 1.18 or 1.23. Another workflow of this repository on `main` that called a signing workflow would differ only in 1.18; the job guards stop it from minting a token there. `publish` checks the `buildConfigURI` of `SHA256SUMS`'s provenance with `jq`, but users' commands can't.
- **The other OIDC jobs can still get `release.yml` certificates.** `publish-crates`, `publish-npm` and `publish-pypi` keep `id-token`, for trusted publishing. No verifier accepts that identity any more.
- **`sign-image` has no environment of its own.** A job that calls a reusable workflow can't name one, and `sign-image.yml`'s `image` job sets none. The approved step is `publish-image`'s push.
- **The signing workflows' steps first run in a real release**, like the Phase 5 attest jobs' steps. PR dry-runs exercise the same commands offline with a throwaway key, and the tag script against a local registry.
- **Two signing identities** (since the 2026-11-18 amendment). Image verifiers pin `sign-image.yml` and release-file verifiers pin `sign.yml`. Swapping them fails closed: the check refuses.
- **The `./` calls carry a zizmor ignore.** `self-repository` flags it, and the `$/` form that zizmor suggests fails actionlint 1.7.12.
- **A seventh language-policy exception.** `image-tags.sh` is 111 lines of Bash, with ShellCheck and the PR dry-run as its checks (decision 7).
- **BuildKit and the scanner are pinned by digest, not signature-checked.** buildx's default policy is opt-in, and skips the builder image on the classic store. The builder container always runs privileged.
- **Docker gets no Dependabot security updates.** The weekly `image-scan` is the signal for a new advisory in the base.
- **The base check relies on cosign's legacy path.** distroless publishes only legacy `.sig` signatures, and cosign says bundles will become its only supported format. Re-check before any cosign upgrade.
- **CI will run rootful Docker 28.0.4 only** (it hasn't run yet). Rootless Docker was tested locally. Rootless Podman and Docker Desktop weren't tested.

**Revisit when:**
- BuildKit v0.35.0 and buildx v0.39.0 pass the 7-day rule (from 2026-11-18): bump them, and re-check `provenance-add-gha` and the attestation layout `check` expects.
- distroless's `aafe9936…` passes the 7-day rule (after 2026-11-19T21:16:35Z): re-pin the base.
- GHCR supports the referrers API: the fallback-tag check must also accept referrers.
- Dependabot dates Docker updates for gcr.io or GHCR: drop the hand check of the base digest's age.
- MCP clients share a project-directory variable, or rendering one form per client is worth it: reconsider `runtime = "docker"`, after v1.
- The MCP Registry leaves preview and an `oci` listing is wanted (ADR-0002): it needs one image, or annotation, per server.
- A server needs an image of its own, for least privilege or for that listing: per-server images.
- A consumer needs each platform manifest signed: `cosign sign --recursive`.
- actionlint accepts the `$/` call form (issue #711): switch the two calls to it and drop the zizmor ignore.
- cosign or gh gain a flag for the Build Config URI (OID 1.18) or the environment (1.23): pin the caller's file in every verifier and in the docs.
