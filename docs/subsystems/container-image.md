# Subsystem: the container image

The container image, `ghcr.io/jabbott-iii/grimoire`, packages the `grimoire` CLI and every asset
binary of a release for linux/amd64 and linux/arm64. It is built in the release workflow from the
release's own archives, on a minimal base pinned by digest, with COPY steps only, so the image
holds exactly the binaries that were signed for the release and nothing runs while it is built.
It guarantees that the base is the one whose signature was verified, that the image holds only
the planned files, that no medium, high or critical vulnerability match ships, and that neither of
its tags ever names an image that isn't signed, attested and verified. The record that shaped it
is [ADR-0011](../adr/0011-container-image.md) (the image is built only from the release's own
binaries, signed in its own workflow, and gated by Grype). It runs in the pipeline described in
[`release.md`](release.md), whose "Terms" section explains the workflow, signing and attestation
terms used here. How to run and verify the image is in the user guide,
[`container.md`](../container.md), and the maintainer's steps are in
[`05-release.md`](../05-release.md#release-checklist). The facts behind the design are the
Phase 6 entries in [`specs.md`](../specs.md) (`*-P6`).

## Why build the image from the release archives

A container image is often built by compiling inside a build stage. That would give the image
binaries that differ from the release's, built by a second toolchain run that no one signed, and
it would run code (the compiler, and every dependency's build script) inside the image build.
Grimoire builds the image differently ([ADR-0011](../adr/0011-container-image.md)):
- **The binaries come from the release archives**, downloaded by artifact ID and checked against
  `SHA256SUMS`, so the image holds the same bytes as every other channel.
- **The `Dockerfile` copies files and runs nothing.** Neither platform needs emulation (running
  another processor's code in software), and no step can change the image in a way the check
  doesn't see.
- **The base is distroless static**, a minimal image with no shell and no C library, pinned by
  digest and its signature verified before every build.
- **cargo stays out of every job that can push or sign**, because a dependency's build script
  could use either. The registry steps are a short Bash script instead, a language-policy
  exception ([ADR-0010](../adr/0010-language-policy-exceptions.md), amended by ADR-0011).

What it trades away:
- The image can only be built where the release archives exist, so it is built in `release.yml`
  and nowhere else.
- `runtime = "docker"`, an option that would have let adapters write a server's client entry as a
  `docker run` command, is deferred past v1. No client variable names the project directory, so
  each client would need its own form, and the CLI can't embed the digest of the image that
  contains it. [`container.md`](../container.md) documents hand-written, digest-pinned
  `docker run` entries instead, and `grimoire.toml` keeps rejecting the field.
- The registry steps and the keyless signing first run against GitHub Container Registry (GHCR)
  in a real release ("Residual risks", below).

## What the image holds

`ghcr.io/jabbott-iii/grimoire` is one index for linux/amd64 and linux/arm64. An index, in the
image format of the Open Container Initiative (OCI), points to one image manifest per platform,
and each manifest lists that image's configuration and layers (the archives of files it is built
from). The image starts from `gcr.io/distroless/static-debian13:nonroot`, a minimal base with no
shell and no libc (the C standard library). On top, it adds only the release's static musl
binaries (statically linked with musl, a small C library, so they need no libc in the image):
- the CLI at `/usr/local/bin/grimoire`, the entrypoint (the program the container runs by
  default);
- every asset binary in `/opt/grimoire/bin/`, off `PATH`.

It runs as uid 65532 (a non-root user) unless `--user` says otherwise, with `/repo` as its working
directory, the place to mount a repository. A release tags it `X.Y.Z` and `latest`, and the
release notes name its index digest (the digest of the index that lists both platforms' images).

**Inside a container, `doctor` describes the container.** `doctor` and the
`plan/hook-binary-missing` warning read the container's `PATH`, not the host's. The asset binaries
are off it, so a container never counts them as installed: a binary on the image's `PATH` would
count as installed when the host lacks it. The warning can then appear even when the host has the
binary, which is the safe direction.

## The parts

| Part | What it does | Built in |
|---|---|---|
| `Dockerfile` | One `FROM` pinned by index digest, two `COPY --chmod=0755` steps, `WORKDIR /repo`, `USER 65532:65532`, the source label, the entrypoint | M6.1 |
| `.grype.yaml` | The vulnerability gate: `fail-on-severity: medium`, `only-fixed: false`, a database at most 120 hours old, and the ignore rules (none today) | M6.1 |
| `cargo xtask release image base` | Prints the base index reference the `Dockerfile` pins | M6.2 |
| `cargo xtask release image context` | Checks the fetched base index against the pin, checks the `Dockerfile`'s rules, unpacks both Linux musl targets' binaries into a build context, and writes the buildx arguments | M6.2 |
| `cargo xtask release image check` | Checks the OCI layout BuildKit wrote against the plan, binaries byte for byte, and prints the digests | M6.2 |
| `cargo xtask release image coverage` | Fails unless Grype saw Debian, its packages and every binary's Rust packages | M6.2 |
| `release.yml` jobs `image`, `verify-image`, `publish-image`, `sign-image` | Build, check and scan; run on both platforms; push by digest; sign, attest and tag | M6.3 |
| `.github/workflows/sign-image.yml` | The image's signature and attestation, under its own identity, then the tags | M6.3 |
| `.github/scripts/image-tags.sh` | The registry steps: `push`, `tag` and `latest` | M6.3 |
| `security.yml` jobs `image-scan` and `workflows` | The weekly rescan of the latest release's image; ShellCheck on `image-tags.sh` | M6.4 |

The logic is in `xtask/src/release/image.rs` (`parse_dockerfile`, `base_platforms`, `check_elf`,
`expected_dirs`, `annotations`, `buildx_args`, `debian_coverage`, `rust_coverage`). It runs only in
`release.yml`'s `image` job; `verify-image` and `publish-image` run no cargo. The commands' flags
are in "Everyday commands" in [`AGENTS.md`](../../AGENTS.md).

## The trust chain

Each link is checked by the next:
1. **Base.** The `Dockerfile` pins distroless's index digest.
   - The `image` job fetches that index and requires it to be the pinned digest, byte for byte,
     with exactly one linux/amd64 and one linux/arm64 manifest.
   - cosign then verifies distroless's keyless signature
     (`keyless@distroless.iam.gserviceaccount.com`, issuer `https://accounts.google.com`) on the
     index and on both platform manifests.
   - The job also fetches each platform's config blob (`crane blob`), which `check` compares with
     the image. (crane is a tool for working with registries, and the config blob is the JSON
     that holds an image's settings.)
   - This runs before every build, pull requests included.
2. **Inputs.** The binaries come from the release archives, downloaded by artifact ID and checked
   against `SHA256SUMS` ([`release.md`](release.md#the-trust-chain), step 3).
   `cargo xtask release image context` accepts only a static 64-bit ELF executable (Executable
   and Linkable Format, the Linux executable format) for the platform's machine.
3. **Build.** One amd64 job builds both platforms on BuildKit v0.34.2 (Docker's build engine),
   pinned by digest, with COPY steps only: each step copies files in, and none runs a command. It
   has only `contents: read` and pulls anonymously. The output is an OCI layout (an image stored
   as files in a directory) with an SPDX SBOM (SPDX is another SBOM format) and SLSA v1
   provenance (`mode=max`, the detailed mode) for each platform.
4. **Check.** `cargo xtask release image check` requires:
   - one index with exactly the two platform manifests and one attestation manifest for each, and
     the exact annotations (key-value labels);
   - an image config with exactly the Dockerfile's fields (`User`, `Env`, `Entrypoint`, `Cmd`,
     `WorkingDir`, `Labels`, `ArgsEscaped`) and top-level keys, `Env` equal to the base config's,
     and `diff_ids` (the hashes of each layer's uncompressed contents) that are the base's, then
     one for each added layer;
   - the verified base's layers first, byte for byte, with their blobs in the layout read and
     checked;
   - on top, exactly the planned directories, each with its mode and owner (`usr`, `usr/local`,
     `usr/local/bin`, `opt`, `opt/grimoire` and `opt/grimoire/bin` root-owned 0755; `repo`
     65532:65532 0755), and exactly the release binaries: mode 0755, owned by root, each with its
     archived binary's SHA-256.

   Layers are read as strictly as Docker and containerd (the container runtime underneath Docker)
   read them: one gzip member and nothing after it, uncompressed bytes equal to the config's
   `diff_id`, no PAX or GNU extension records (extra tar headers that can override an entry's
   name or size), a size of 0 for every directory, and only zeros after the tarball's end. So no
   reader sees a file `check` didn't. Every blob's digest and size is checked.
5. **Scan.** Grype, a vulnerability scanner, scans each platform (threat I6 below).
6. **Run.** `verify-image` pushes the layout to a local registry that, like GHCR, has no
   referrers API (the registry endpoint that lists the signatures and attestations attached to an
   image). Then it runs every binary from it on an amd64 and an arm64 runner. Its only permission
   is `contents: read`, to check out `image-tags.sh`.
7. **Approval, then the push.** `publish-image` runs only after the GitHub Release, and only once
   the maintainer approves environment `ghcr`. It pushes the same layout by digest, untagged, so
   GHCR gets the digest the release notes name. It has no OIDC token.
8. **Signing, then the tags.** `sign-image` calls `sign-image.yml`, which:
   - signs the index keylessly under its own identity, `…/sign-image.yml@refs/heads/main`, with
     the same caller pins as the release files
     ([`release.md`](release.md#the-signing-identity));
   - adds a provenance attestation with `actions/attest`;
   - checks that the `sha256-<index hex>` tag lists both (without a referrers API, that tag holds
     the list of an image's signatures and attestations);
   - and verifies both.

   Only then does it tag `X.Y.Z` and move `latest`. So the release never gives either tag to an
   image before it is signed, attested and verified.
9. **Weekly rescan.** `security.yml`'s `image-scan` job (Mondays at 05:17 UTC, or on a manual run)
   finds the latest release, verifies its image's signature with the image's signing identity
   (`sign-image.yml`) and the caller pins, and runs the same Grype gate on every platform.

## The tag steps

`.github/scripts/image-tags.sh` makes the registry steps: `push` (by digest, untagged), `tag`
(`X.Y.Z`) and `latest`.

It is a language-policy exception: code that isn't Rust, which
[ADR-0010](../adr/0010-language-policy-exceptions.md) allows only with a reason, a limited scope
and checks of its own, and which ADR-0011 records. The reason is that cargo must not run in a job
that can push or mint OIDC tokens, because a dependency's build script could use either, and a
prebuilt xtask would put all its dependencies in the signing job. Its rules:
- a version tag that names another digest is an error, never overwritten;
- a tag lookup that fails for any reason other than the registry's `MANIFEST_UNKNOWN` or
  `NAME_UNKNOWN` stops the script, so a tag it couldn't read is never treated as absent;
- `latest`'s version annotation is trusted only if `latest`'s digest verifies with the image's
  signing identity (`sign-image.yml`). A package writer could otherwise hold `latest` by claiming
  a high version. `latest` never moves back to an older release, and if it claims a newer version
  that doesn't verify, the job stops and leaves it for the maintainer;
- it runs under `set -euo pipefail` and `inherit_errexit` (Bash options that stop the script on a
  failed command or an unset variable), so a failed lookup inside a command substitution still
  stops it. ShellCheck v0.11.0 (a shell-script linter), pinned, checks it in `security.yml`.

Growth beyond these three steps needs a change to ADR-0011. The release (`publish-image` and
`sign-image.yml`) and every pull request dry-run run the same script, so the code that guards the
release tags never runs for the first time in a release ("Tests", below).

## Threats and mitigations

| # | Threat | Mitigation | Result |
|---|---|---|---|
| I1 | The base image is compromised, or another image is served under its tag | Pinned by index digest. The served index must be that digest byte for byte. cosign verifies distroless's signature on the index and on both platform manifests before every build. `check` requires the image's first layers to be exactly the verified platform manifest's layers, and reads their blobs in the layout. It also compares the image config with the base config `crane blob` fetched: its `Env`, and its `diff_ids` first. | The build stops |
| I2 | Grimoire's tags are moved | The release notes name the index digest, and the docs pin it; `X.Y.Z` and `latest` are a convenience. `publish-image` pushes by digest, untagged. `sign-image` sets the tags only after the signature and the attestation verify in GHCR. `X.Y.Z` is never overwritten: a tag that names another digest stops the job, and so does a lookup that fails for any reason other than the registry's "unknown" answer. `latest` moves last and never back to an older release. If it claims a newer version that doesn't verify with the image's signing identity, the job stops. The signature is on the index digest, and the index's `org.opencontainers.image.version` annotation, which it covers, names the version. | A pinned digest still means the same image |
| I3 | The builder, the SBOM scanner or the Dockerfile frontend (the part of BuildKit that parses the Dockerfile) is swapped | BuildKit and the SBOM scanner are pinned by digest. Unpinned, BuildKit pulls the scanner by its `stable-1` tag on every build. There is no `# syntax=` line, so the frontend is the one inside the pinned BuildKit image and nothing is pulled by tag. buildx (Docker's build command, which drives BuildKit) is installed checksum-verified into a job-scoped `DOCKER_CONFIG`, ahead of the runner's unverified copy. The job checks the buildx version and the `BuildKit version:` line the builder reports. | Fixed bytes; signatures aren't checked (below) |
| I4 | The public provenance leaks the GitHub event payload (the JSON that describes what started the run) | On GitHub Actions, buildx's `docker-container` driver adds the whole event payload and runner details to provenance by default, in min mode as well as max. The builder is created with `provenance-add-gha=false`, and `check` refuses any provenance that holds `github_event_payload`. Max mode also records every build argument and the Dockerfile; the only build argument is `SOURCE_DATE_EPOCH`. | `check` fails |
| I5 | Code runs during the build | COPY only. `context` allows only `FROM`, `ARG`, `COPY --chmod=0755 <src> <dst>`, `WORKDIR`, `USER`, `LABEL`, `ENTRYPOINT` and `CMD`, so `RUN`, `ADD`, `ONBUILD` and `COPY --from` are refused. It also refuses heredocs (inline scripts or file contents written into the Dockerfile), a `# syntax=` line, line continuations (a backslash that joins the next line), and anything but one `FROM name:tag@sha256:…`, with the digest on the same line and no flags or stage name. The first line must be `# check=error=true`, and `docker buildx build --check` (Docker's Dockerfile checks) runs before the build. Nothing executes, so neither platform needs emulation. | `context` fails |
| I6 | A vulnerable base package or crate ships | Grype fails on any medium, high or critical match, fixed or not (the maintainer's choice). A finding without a fix needs an ignore rule with a reason in `.grype.yaml`, reviewed like `deny.toml`'s exceptions. The workflows pass that file with `-c`, so no other Grype config is read. They repeat the threshold with `--fail-on`, because Grype ignores unknown keys silently, also inside an ignore rule, where a misspelled criterion widens the rule. A unit test reads the file line by line and refuses any unknown key, and any rule without a GHSA (GitHub Security Advisory) or CVE (Common Vulnerabilities and Exposures) id, a package name and a reason. Exit 2 (a match) and any other error both fail, and every platform is scanned before the step fails. `coverage` fails when Grype detected no Debian 13 or no Debian packages, or a binary shows no Rust packages: either would be reported as clean. The weekly rescan catches advisories published after a release. cargo-deny, the Rust dependency checker ([`02-dependency-policy.md`](../02-dependency-policy.md)), stays the primary Rust advisory gate. | The release stops before approval |
| I7 | The vulnerability database is tampered with or stale | Grype checks the downloaded database against the SHA-256 in an HTTPS listing. Neither is signed, so authenticity rests on TLS (Transport Layer Security, the encryption under HTTPS). `db.validate-age` with `max-allowed-built-age: 120h` fails a scan when the database is more than five days old. The `image` job logs `grype db status`. | Residual (below) |
| I8 | A signature or attestation is deleted from the `sha256-<index hex>` tag | Anyone with write access to the package can remove an entry. That is a denial of service, not a forgery: verification then fails, and verifiers must fail closed. Never delete `sha256-*` tags or untagged referrer manifests (the manifests that hold signatures and attestations), for example in a retention cleanup. During a release, a re-run of `sign-image` signs again when the signature doesn't verify. It attests again when `gh attestation verify --bundle-from-oci` finds no attestation in GHCR, even if GitHub's API holds one. Checking the API instead, as before the Phase 6 review, would skip an attestation that reached only the API, and the registry check would then fail on every re-run. | Verification fails |
| I9 | The push or signing job's credentials are misused | `publish-image` runs only in release mode, from `main`, after the maintainer approves `ghcr`. It pushes with the job's `GITHUB_TOKEN` (`contents: read`, `packages: write`) and has no `id-token`, so it can't sign. `sign-image.yml`'s image job runs only when `release.yml` on `main` calls it, after `publish-image`; it has `contents: read`, `packages: write`, `id-token: write` and `attestations: write`, and no environment. No token is stored. Neither job runs cargo, so no dependency's build script runs where it could push or mint a token: they check out the repository (without persisted credentials) only for `image-tags.sh`, and read JSON with `jq`. Both log out of GHCR in an `if: always()` step. | One approval, no stored token |
| I10 | The image is signed before anyone approves it | Pull request dry-runs sign only with a throwaway key and a signing config that names no services, so nothing reaches Rekor. The keyless signature and the attestation are made only in `sign-image`, which needs `publish-image`, which waits for the `ghcr` approval. Unlike the release files ([`release.md`](release.md#residual-risks), "Signing comes before approval"), no unapproved image carries the image's signing identity (`sign-image.yml`). A verifier can't see that approval, though ([`release.md`](release.md#residual-risks), "No flag pins the caller file or the environment"). | |
| I11 | A container gets more access than the CLI or a server needs | The documented `docker run` forms ([`container.md`](../container.md)) pin the image by digest and pass `--network none` (no network), `--read-only` (a read-only root file system), `--cap-drop ALL` (no Linux capabilities, the separate powers of root) and `--security-opt no-new-privileges` (no gaining privileges, for example through a setuid program). The image runs as uid 65532 by default. The CLI form passes `--user "$(id -u):$(id -g)"` on rootful Linux (where the Docker daemon runs as root); under rootless Docker (where the daemon runs as an ordinary user), the documented `--user 0:0` is the host user, not root on the host. MCP servers get the repository mounted read-only and `--root /repo`, never `CLAUDE_PROJECT_DIR`, whose host path doesn't exist in the container (MCP threat M10, in [`mcp.md`](mcp.md)). `verify-image` runs every binary with these flags. | Limited to what the user mounts |
| I12 | An HTTP server in a container is reachable without authorization | To be reachable through a published port (a container port Docker forwards from the host), the server must listen on `0.0.0.0`, and the hardening crate refuses that without authorization (MCP threat M2, in [`mcp.md`](mcp.md)). That is right: on rootful Docker, every bridge container (one on Docker's default network) and host process can reach the container's address, whatever `-p` says. The documented form publishes with `-p 127.0.0.1:8931:8931`, which needs Docker 28.0 or later (before 28.0.0, hosts on the same L2 segment, the same local network, could reach ports published to localhost). It sets `--auth-resource` to the exact URL clients use, or the `Host` check answers 403, and mounts the JWKS directory read-only. On Linux, `--network host` with `--listen 127.0.0.1:8931 --no-auth` keeps the loopback-only rule instead; not on Docker Desktop, or on rootless Docker before 29.5. | Refuses to start without authorization |
| I13 | `secret-guard` in a container loses context | It needs the event on stdin (`-i`) and the project at its identical absolute path. Otherwise the "file already holds this secret" exemption is lost, which fails safe: it blocks more. Worktrees outside the mount and Windows hosts can't use identical paths, so the native binary is recommended for hooks. | Blocks more, never less |
| I14 | Another repository or workflow calls `sign.yml` or `sign-image.yml` | Any repository can call a public reusable workflow and get a certificate with the same identity. Every job in the two signing workflows runs only when `github.repository_id` is `1460666977` and `github.workflow_ref` is `jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, so an external caller, another workflow of this repository and a pull request run are all skipped before a token is minted. Every verifier also pins the caller's repository and ref, so another repository's certificate doesn't verify. Another workflow of this repository on `main` differs from a release only in OID 1.18, which only `publish` checks ([`release.md`](release.md#the-signing-identity)). | The jobs are skipped; another repository's certificate is refused |

## Residual risks

- **The base's age is checked by hand.** Dependabot (GitHub's dependency-update bot) has a
  cooldown, a waiting period before it proposes a new version. The cooldown doesn't hold back
  gcr.io digests: Dependabot proposes them at once, with a `cooldown_date_unavailable` warning.
  The maintainer checks the upload time before merging
  ([`05-release.md`](../05-release.md#maintaining-the-image-pins)). The `timeUploadedMs` field of
  Google Container Registry (GCR) is undocumented, and no CI check of the age exists.
- **Docker's images are pinned, not verified.** BuildKit and the SBOM scanner carry keyless
  signatures on their attestation manifests, but nothing checks them, so the digest pins fix the
  bytes, not who built them. The BuildKit container always runs privileged (with almost all of
  the host's privileges). Whether the runners' Docker 28.0.4 pulls a `name:tag@sha256:…`
  reference strictly by digest is UNVERIFIED; the builder step checks only the BuildKit version
  it reports.
- **The SBOM and provenance are BuildKit's own statements.** `check` checks their predicate types
  (the kind of statement each is), their subjects (what each is about) and the absence of the
  event payload, not the rest of their contents.
- **The attached SBOM comes from an older syft** (an SBOM generator). The scanner bundles syft
  v1.51.0; Grype 0.121.3 embeds v1.57.1. So the SBOM and the gate can disagree. v1.51.0 is also
  older than the fix for GHSA-mw2c-m758-9v5q (memory exhaustion parsing binaries, fixed in
  v1.52.0). It reads only the final image: the pinned base and Grimoire's own binaries.
- **The vulnerability database is the one unsigned, unpinned input** (I7). It is rebuilt daily,
  so a release re-run can fail where the pull request passed.
- **What Grype can't see.**
  - Rust crates only through cargo-auditable's data, which `coverage` requires, and Rust matches
    only through GHSA.
  - Statically linked C libraries are invisible to it; by inference, that includes musl libc.
  - Matches of unknown severity never trip `--fail-on`, and low or negligible ones pass by
    design.
- **Ignore rules don't expire.** Grype has no expiry field. Every rule is reviewed when the base
  image or Grype is bumped.
- **Only the index is signed.** Platform manifests aren't signed separately; signing them
  (`--recursive`) is a follow-up. A verifier must check the index digest. How policy engines
  (tools that decide whether a system may run an image) treat a signature on the index rather
  than on each platform manifest is UNVERIFIED.
- **The `sha256-<index hex>` tag is shared and rewritten without a lock.**
  - go-containerregistry (the Go library inside cosign and crane) updates it with no conditional
    request (one that fails if the tag changed since it was read), so a concurrent writer could
    drop an entry.
  - cosign and `actions/attest` run one after the other in one job (`sign-image.yml`'s image
    job), which then checks that both are listed. It also checks the bundle format itself,
    because `cosign verify` falls back silently to the legacy signature format.
  - Whether GHCR's cleanup of untagged versions would delete referrer manifests is UNVERIFIED.
- **The digest in the release notes isn't attested.** Release notes stay editable on an immutable
  release, and the release attestation covers only the tag, the commit and the assets. No release
  asset records the image digest. What proves a digest is the image's own signature and
  attestation.
- **A signature doesn't name a version.** Any image `sign-image.yml` signed in a release run
  verifies under the same identity and pins. The version is in the index's signed annotations.
  The weekly rescan follows the version tag, then checks the signature, so it would not notice
  the tag moved to another signed release.
- **The GHCR login sits in `~/.docker/config.json`**, in `publish-image` and in
  `sign-image.yml`'s image job. `actions/attest` reads credentials only from that file, so the
  signing job's login can't use a job-scoped `DOCKER_CONFIG`. The file holds the token
  base64-encoded, not encrypted, until the logout step.
- **`image-tags.sh` can't tell a failed verification from a missing signature.** If cosign fails
  for another reason, such as a network error, the script takes `latest`'s image for unsigned and
  stops the job, leaving `latest` on the newer release. That can happen only when an older
  release's `sign-image` runs after a newer release.
  [`05-release.md`](../05-release.md#recovery) says what the error means, and a re-run once the
  check works finishes.
- **GHCR's behavior is UNVERIFIED until the first release (M8.7):**
  - whether it sends `OCI-Subject` or honors `If-Match`;
  - whether tags can be made immutable (its docs are silent);
  - a new package's first visibility (its docs conflict);
  - whether it links the package from the config label or the index annotation (the image
    carries both);
  - and how `sha256-*` tags appear as package versions.
- **Making the package public can't be undone.** Anonymous pulls, the documented `docker run`
  forms and the weekly rescan need it
  ([`05-release.md`](../05-release.md#during-the-first-release-run)).
- **CI exercises rootful Docker 28.0.4 only.** Rootless Docker was tested only by hand, in
  Phase 6. Docker Desktop and Podman weren't tested; Docker Desktop's file ownership on macOS and
  Windows and rootless Podman's `--userns=keep-id` are UNVERIFIED.

## Invariants

1. The `Dockerfile` starts with `# check=error=true`, has exactly one `FROM` pinned by digest on
   the same line, and uses only `FROM`, `ARG`, `COPY --chmod=0755`, `WORKDIR`, `USER`, `LABEL`,
   `ENTRYPOINT` and `CMD`, with no heredoc, parser directive or line continuation.
2. The build uses the pinned base only if the served index is that digest byte for byte, with
   exactly one linux/amd64 and one linux/arm64 manifest.
3. The build context holds only static 64-bit ELF executables for each platform's machine.
4. A layout passes `check` only if its index, annotations, attestation manifests, config, base
   layers, directories and binaries are exactly the planned ones, and its layers read the same
   under Docker's and containerd's rules.
5. No provenance in the image holds the GitHub event payload.
6. `.grype.yaml` holds only known keys, and every ignore rule names a GHSA or CVE id, a package
   and a reason.
7. A scan counts only if Grype detected Debian 13 with its packages, and Rust packages in every
   binary.
8. Any medium, high or critical match fails the gate, fixed or not, on every platform.
9. The image pins are equal in every workflow that names them: the image name, crane, and the
   cosign issuer, repository and ref in `release.yml`, `security.yml` and `sign-image.yml`; the
   caller pins in `sign.yml` too; and Grype and ShellCheck in `release.yml` and `security.yml`.
   Each signing workflow names its own identity.
10. The image's license annotation matches the workspace license.
11. A version tag is never overwritten, and `latest` never moves to an older release or off a
    newer signed one.
12. Neither tag names an image before its signature and attestation verify in the registry.

## Tests

| Invariant | Checked by |
|---|---|
| 1 | `the_dockerfile_is_copy_only_and_pinned` and `dockerfiles_that_run_code_or_float_are_refused` in `xtask/src/release/image.rs`; `docker buildx build --check` in the `image` job |
| 2 | `the_base_index_must_be_the_pinned_one_with_both_platforms` in `image.rs`; the `image` job's cosign verification of the base |
| 3 | `only_static_elf_executables_for_the_platform_are_accepted` in `image.rs` |
| 4 | `check_passes_a_layout_shaped_like_buildkits`, `check_refuses_changed_annotations_manifests_and_attestations`, `check_refuses_a_changed_config`, `check_refuses_changed_layers`, `check_refuses_other_binaries_and_missing_ones`, `layers_are_read_strictly` and `layers_with_records_other_readers_see_differently_are_refused` in `image.rs`; `annotations_and_buildx_arguments` and `rfc3339_dates` for the planned annotations |
| 5 | `check_refuses_changed_annotations_manifests_and_attestations` in `image.rs`, which gives one platform a provenance holding `github_event_payload` |
| 6 | `the_grype_config_has_only_known_keys_and_complete_rules` in `image.rs` |
| 7 | `coverage_needs_debian_and_its_packages` and `coverage_needs_rust_packages_in_every_binary` in `image.rs`; `cargo xtask release image coverage` in the `image` job |
| 8 | The Grype step of the `image` job on every pull request, and `security.yml`'s weekly `image-scan` |
| 9 | `the_image_pins_agree_across_workflows` in `image.rs` |
| 10 | `the_licence_annotation_matches_the_workspace` in `image.rs` |
| 11, 12 | No unit test: `image-tags.sh` is Bash. The amd64 leg of `verify-image` runs its cases on every pull request (below), and ShellCheck checks it in `security.yml`'s `workflows` job |

**The image jobs on every pull request** (`release.yml`):
- `container image (build, check, scan)` (`image`): verifies the base and its signatures, runs
  `--check`, builds both platforms, runs `check`, scans both platforms with Grype, and runs
  `coverage`.
- `verify image` (`verify-image`), once per platform on its own runner: pushes the layout to a
  local registry and checks the digest is kept, pulls it, and runs `grimoire --version` and every
  asset binary's `--version` under the hardening flags of I11. On amd64 it also signs the image
  offline with a throwaway key, checks that the signature lands in the `sha256-<index hex>` tag
  and that an unsigned platform manifest digest fails to verify, and runs `image-tags.sh`
  against the local registry:
  - push twice, and tag twice;
  - refusing a version tag that names another digest (by `push` and by `tag`);
  - refusing a registry it can't read;
  - moving `latest` off an unsigned image that claims no newer version;
  - refusing to move it off an unsigned image that claims a newer one;
  - and keeping `latest` on a newer signed release.
- `release checks` fails unless both jobs passed.

`publish-image`, `sign-image` with `sign-image.yml`, and `image-scan` don't run on a pull request.
Their first run, with the image's keyless signing and `actions/attest`, is the first release,
M8.7.
