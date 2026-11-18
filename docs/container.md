# The container image

- **Status:** written in Phase 6 (2026-11-17), before the first release. **No published image exists yet**, so no command in this file has been run against a real release. §7 says what ran locally and what CI runs.
- **Decisions:** [ADR-0011](adr/0011-container-image.md) (the image, its supply chain and its gates).
- **Facts:** the `*-P6` entries in [`specs.md`](specs.md).

`ghcr.io/jabbott-iii/grimoire` holds the `grimoire` CLI and every asset binary, so you can run Grimoire, or an MCP server from the catalog, with only Docker installed. This file has seven parts:
1. what is in the image;
2. verifying it before use;
3. running the CLI on a mounted repository;
4. MCP servers over stdio;
5. MCP servers over Streamable HTTP;
6. hooks;
7. limits and what is not tested.

Every command here names the image by its index digest, `ghcr.io/jabbott-iii/grimoire@sha256:<digest>`. Each release's notes give it.

## 1. What is in the image

- **Platforms:** linux/amd64 and linux/arm64, in one OCI index. Docker pulls the one that matches the host.
- **Base:** `gcr.io/distroless/static-debian13:nonroot` (Debian 13), pinned by digest in the `Dockerfile` at the release's tag. Each image records it in its annotations: `crane manifest ghcr.io/jabbott-iii/grimoire@sha256:<digest> | jq -r '.annotations["org.opencontainers.image.base.digest"]'`.
  - It holds only `base-files`, `ca-certificates`, `media-types`, `netbase`, `tzdata` and `tzdata-legacy`. There is no shell and no libc.
  - The release workflow verifies distroless's signature on the index and on both platform manifests before every build.
- **The CLI:** `/usr/local/bin/grimoire` is the entrypoint, and `--help` the default command. So `docker run … <image> plan` runs `grimoire plan`.
- **Asset binaries:** one per tool and MCP server in the catalog, in `/opt/grimoire/bin/`. Today they are `grimoire-mcp-project-docs` and `grimoire-tool-secret-guard`.
  - They are off `PATH` on purpose. `grimoire doctor` and the `plan/hook-binary-missing` warning read the container's `PATH`, so a binary on it would count as installed although your host may not have it.
  - Start one with `--entrypoint /opt/grimoire/bin/<binary>`, an absolute path.
- **The binaries are the release's**, byte for byte: the static musl builds from the release archives. The workflow checks each one's SHA-256 against its archive.
- **Defaults:** working directory `/repo`, where you mount a repository; user `65532:65532` (distroless's `nonroot`); no exposed ports and no volumes.
  - The binaries need no shell, git, `/tmp` or `HOME`. Under a uid with no `/etc/passwd` entry, `HOME` is `/`, and no binary reads it.
- **Tags:** a release tags `X.Y.Z` and moves `latest`, only after the image is signed, attested and verified. Until then the image is in GHCR by digest only, untagged.
  - The workflow never moves `X.Y.Z` to another digest. It moves `latest` to the new release unless `latest` already names a newer version. A newer release that carries the image's release signature (`sign-image.yml`'s) keeps `latest`, so `latest` never goes back to an older one. If the image `latest` names claims a newer version without that signature, the release stops instead, and `latest` stays until the maintainer has looked.
  - As far as GHCR's docs say, its tags are mutable: anyone with write access to the package can move one.
  - **Pin the digest.** The release notes name it. They stay editable after the release is published, so the signature check in §2, not the notes, is what ties a digest to the release.
- **When it appears:** the image is pushed by digest after the GitHub Release is published, by a job the maintainer approves together with the registry jobs. The next job signs it, and only then tags it. A new GHCR package may stay private until the maintainer makes it public, so a pull can fail for a while after the first release appears.
- **Annotations:** the index and each platform manifest carry `org.opencontainers.image.{created, description, documentation, licenses, revision, source, title, url, version}`, plus `base.name` and `base.digest` for the distroless image. `revision` is the release commit, and `created` is its commit time.
- **Reproducible platform manifests:** builds from the same release archives and commit give the same platform manifest digests (§7). The index digest differs between builds, because the SBOM and provenance record build times.

## 2. Verifying the image

Verify the index digest before you use it, then use that digest everywhere. Only the index is signed. It names each platform manifest and each BuildKit attestation manifest by digest, so the signature covers them too.

The image's signature and attestation are made by `sign-image.yml`, Grimoire's image-signing workflow, when `release.yml` on `main` calls it ([ADR-0011](adr/0011-container-image.md), SIGN-P6 in [`specs.md`](specs.md)). The release files are signed by another workflow, `sign.yml`, under its own identity ([`release.md`](release.md) §5), so don't use the image's commands for them. No release exists yet, so every release is signed this way.

**cosign**, version 3.1.3 or later (the workflows use 3.1.4):
```sh
cosign verify ghcr.io/jabbott-iii/grimoire@sha256:<digest> \
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-github-workflow-repository jabbott-iii/Grimoire \
  --certificate-github-workflow-ref refs/heads/main
```
- **Keep all four flags.** The certificate names the workflow that signed, `sign-image.yml`. Any repository can call a public reusable workflow and get a certificate with that same name; the caller is recorded in other fields. The last two flags pin the caller: a run of `jabbott-iii/Grimoire` on `refs/heads/main`.
- The identity, the repository and the ref are compared exactly, case included: `Grimoire` there, `grimoire` in the image name.
- A platform manifest's digest doesn't verify on its own, because only the index is signed. Verify the index and pull by it.
- The signature names a digest, not a version. The index's `org.opencontainers.image.version` annotation, which the signature covers, says which version it is.
- To tie the signature to the release commit, add `--certificate-github-workflow-sha <commit>`, the commit that tag `vX.Y.Z` points to.
- If cosign finds no valid signature, don't use the image.

**Build provenance**, with gh 2.102.0 or later:
```sh
gh attestation verify oci://ghcr.io/jabbott-iii/grimoire@sha256:<digest> \
  --repo jabbott-iii/Grimoire \
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- **Keep every flag.** `--repo` alone accepts an attestation from any workflow and ref of the repository. `--signer-workflow` requires `sign-image.yml` at `refs/heads/main` (without the ref, gh would also accept `sign-image.yml` at any tag or commit, which any repository can call), `--source-ref` a run on `refs/heads/main`, and `--deny-self-hosted-runners` a GitHub-hosted runner.
- **Pin what no flag does**, especially with `--bundle-from-oci`, where gh compares names only: add `--format json` and check the caller's workflow file and the repository and owner IDs. The release workflow runs this same check.
  ```sh
  … --format json | jq -e 'length > 0 and all(.[]; .verificationResult.signature.certificate
    | .buildConfigURI == "https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main"
      and .sourceRepositoryIdentifier == "1460666977" and .sourceRepositoryOwnerIdentifier == "84321953")'
  ```
- `--signer-workflow` and `--source-ref` are case-sensitive, so type `Grimoire` with the capital.
- gh must be signed in. Its help says the `oci://` form also needs a registry login.
- Add `--bundle-from-oci` to read the copy in GHCR, next to the image, instead of the one stored on the repository. gh then checks only the names recorded in the certificate.
- This attestation is SLSA v1 provenance made by `actions/attest` v4.2.2. It is stored on the repository and also pushed next to the image. Its `runDetails.builder.id` names `sign-image.yml`, and `externalParameters.workflow.path` names the caller, `.github/workflows/release.yml`.
- `sign-image.yml` runs this command with `--bundle-from-oci`, and the cosign command above, before it tags the image. Neither has run on GitHub yet.

**SBOM and provenance in the image.** BuildKit adds one attestation manifest per platform to the index, listed with platform `unknown/unknown`. Each holds:
- an SPDX 2.3 SBOM, written by `docker/buildkit-syft-scanner` 1.12.1 (syft v1.51.0);
- SLSA v1 provenance in `mode=max`. It records the Dockerfile and every build argument; the only one is `SOURCE_DATE_EPOCH`. It holds no GitHub event payload: the workflow turns that off and checks that it is absent.

To read them, for example: `docker buildx imagetools inspect ghcr.io/jabbott-iii/grimoire@sha256:<digest> --format '{{ json .SBOM }}'`, or `'{{ json .Provenance }}'`. Whether these fields work without a local builder is UNVERIFIED. The CycloneDX SBOM of each release archive ([`release.md`](release.md) §5) describes the same binaries.

**`sha256-*` tags.** GHCR has no OCI referrers API. So the signature and the provenance attestation are listed in an index tagged `sha256-<index hex>`, and each BuildKit attestation manifest adds a `sha256-<platform hex>` tag. The package page shows them as ordinary tags.
- Before it tags the version, `sign-image.yml` checks that `sha256-<index hex>` lists both a cosign signature bundle and a SLSA provenance bundle. `cosign verify` alone can't show that, because it falls back silently to legacy signatures.
- Never delete these tags. Deleting one removes what it lists, and verification then fails.

## 3. Running the CLI on a repository

From the repository's root:
```sh
docker run --rm -i --init --network none --read-only --cap-drop ALL \
  --security-opt no-new-privileges --user "$(id -u):$(id -g)" \
  --mount type=bind,src="$PWD",dst=/repo \
  ghcr.io/jabbott-iii/grimoire@sha256:<digest> plan
```
Replace `plan` with any command, such as `init`, `add conventional-commits --apply` or `verify`. Run it from the root: the CLI opens the project at `/repo`, which is whatever `$PWD` was.

**Each flag:**
- **`--rm`:** removes the container when the command exits.
- **`-i`:** keeps stdin open. Add `-t` only in an interactive terminal. The CLI prompts only when stdin is a terminal and `CI` is unset or empty, so without `-t` it acts as with `--no-input`. Don't add `-t` with `--json`: with a terminal, Docker also writes the container's stderr to stdout.
- **`--init`:** the CLI installs no signal handlers. As PID 1 it would ignore Ctrl-C, and `docker stop` would wait 10 seconds before killing it. `--init` makes Docker's own init, which Docker says is backed by tini, PID 1 instead. The image needs nothing for it.
- **`--network none`:** the CLI has no network code. Its catalog is built in.
- **`--read-only`:** nothing in the image can be changed. No `--tmpfs` is needed: the CLI writes each file through a temporary file in the same directory, inside the mount.
- **`--cap-drop ALL` and `--security-opt no-new-privileges`:** the CLI needs no capabilities and never raises its privileges.
- **`--user`:** see ownership, below.
- **`--mount`:** the repository at `/repo`, the image's working directory.

**Ownership.** The CLI writes each file by renaming a temporary file, so the uid it runs as owns every file it writes.
- **Rootful Docker on Linux:** pass `--user "$(id -u):$(id -g)"`, as above. Without it, the CLI runs as 65532, which usually can't write to your repository (an `fs/io` "Permission denied" error); where it can, 65532 owns the files it writes.
- **Rootless Docker:** pass `--user 0:0` instead. Container uid 0 is your own user. Container uid 65532 maps to a subordinate uid (your first subuid + 65531).
- **Rootless Podman:** `--userns=keep-id` maps your uid and gid unchanged (UNVERIFIED: from Podman's docs, not tested). Never add `:U` to a repository mount: it changes the owner of the whole tree on the host.
- **Docker Desktop on macOS and Windows:** how it maps ownership on bind mounts is undocumented (UNVERIFIED). Shares from Windows get mode 0777. Only shared paths can be mounted (by default `/Users`, `/Volumes`, `/private`, `/tmp` and `/var/folders`); others fail with "Mounts denied".

**Mounts.**
- **Use `--mount`, not `-v`.** With a wrong path, `--mount` fails with "bind source path does not exist", while `-v` creates an empty directory on the host.
- **SELinux:** only `-v` can relabel. On an SELinux host, use `-v "$PWD":/repo:z`. Never `:Z`.
- **Read-only mounts** (`ro`, used for MCP servers below) are recursively read-only only on Linux 5.12 and later. On older kernels, mounts below the repository stay writable.
- **Another mount point:** the container starts in `/repo`. If you mount the repository elsewhere, also pass `-w <path>`. The CLI's `-C DIR` names a container path too, resolved against `/repo`; it hasn't been tried in a container.

**`doctor` and hook warnings describe the container, not your host.** `grimoire doctor`, and the `plan/hook-binary-missing` warning from `plan`, `apply`, `update` and the other planning commands, look for binaries on the container's `PATH`. The asset binaries are off it, so in a container they never count as installed, whatever your host has. Check the host's `PATH` yourself.

## 4. MCP servers over stdio

The command for `grimoire-mcp-project-docs`:
```sh
docker run -i --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges \
  --mount type=bind,src=<absolute project path>,dst=/repo,ro \
  --entrypoint /opt/grimoire/bin/grimoire-mcp-project-docs \
  ghcr.io/jabbott-iii/grimoire@sha256:<digest> --root /repo
```
- **`-i`** carries the MCP messages on stdin. Never add `-t`, which fails when stdin is a pipe, or `-d`, which detaches it.
- **`ro`:** the server only reads.
- **`--root /repo`:** always pass it. The server's fallback is `CLAUDE_PROJECT_DIR`, but Docker forwards a variable only with `-e`, and the host path doesn't exist in the container.
- **`--init` is optional here:** the servers stop on SIGINT, on SIGTERM and when stdin closes.
- **Stdout stays clean:** Docker writes pull progress to its own stderr, so a first pull can't corrupt the MCP stream.
- The server runs as uid 65532 unless you add `--user` (§3), and reads only what that uid may read.

**Before you configure a client:**
1. **Pull the image:** `docker pull ghcr.io/jabbott-iii/grimoire@sha256:<digest>`. A first pull can take longer than a client waits for a server to start: 10 seconds by default in Codex (`startup_timeout_sec`), 30 in Claude Code (`MCP_TIMEOUT`).
2. **Test the command by hand.** MCP clients are expected to restart a server that exits, so a wrong path or digest can make one loop. Send `initialize` and check the answer, which names `grimoire-mcp-project-docs` in `serverInfo`:
   ```sh
   printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"0"}}}' \
     | docker run -i --rm --network none --read-only --cap-drop ALL --security-opt no-new-privileges \
         --mount type=bind,src="$PWD",dst=/repo,ro \
         --entrypoint /opt/grimoire/bin/grimoire-mcp-project-docs \
         ghcr.io/jabbott-iii/grimoire@sha256:<digest> --root /repo
   ```

**Grimoire doesn't write these entries.** The override `runtime = "docker"` ([ADR-0002](adr/0002-mcp-server-and-tool-packaging.md) item 5) is deferred past v1 ([ADR-0011](adr/0011-container-image.md)), and `grimoire.toml` still rejects it, for two reasons:
- no variable names the project root in every client, so the mount source can't be written once for all of them;
- the CLI can't contain the digest of the image it is part of.

So write the entry by hand. Grimoire doesn't track it: it never updates the digest or removes the entry. Use either this entry or the one Grimoire writes for `project-docs` (the native binary, for Claude Code), not both.

**Claude Code, VS Code (Copilot harness) and Copilot CLI** read the same workspace-root `.mcp.json`. Claude Code and VS Code offer no variable for the project root there, so the path is absolute, and specific to your machine:
```json
{
  "mcpServers": {
    "project-docs": {
      "type": "stdio",
      "command": "docker",
      "args": [
        "run", "-i", "--rm", "--network", "none", "--read-only",
        "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
        "--mount", "type=bind,src=/home/you/project,dst=/repo,ro",
        "--entrypoint", "/opt/grimoire/bin/grimoire-mcp-project-docs",
        "ghcr.io/jabbott-iii/grimoire@sha256:<digest>", "--root", "/repo"
      ]
    }
  }
}
```
- **Claude Code** substitutes `${CLAUDE_PROJECT_DIR}` directly only in plugin MCP configs. A stdio server's working directory is UNVERIFIED, so don't use `.` either. Its local- and user-scope entries live in `~/.claude.json`, outside the repository, and the same applies to them. Whether Claude Code passes its environment, such as `DOCKER_HOST`, to the server is UNVERIFIED. It asks before it uses `.mcp.json` servers in an interactive session.
- **VS Code:** the legacy `.vscode/mcp.json` has `${workspaceFolder}`, but it belongs to the deprecated Local harness. `.mcp.json` servers run only in a trusted workspace.
- **Copilot CLI** reads `.mcp.json` only after you trust the folder. It passes only `PATH` to a stdio server: set any other variable the docker CLI needs, such as `DOCKER_HOST` or `DOCKER_CONTEXT`, in the entry's `env`.

**OpenAI Codex**, `.codex/config.toml`. Codex has no `${VAR}` interpolation, so the path is absolute:
```toml
[mcp_servers.project-docs]
command = "docker"
args = [
  "run", "-i", "--rm", "--network", "none", "--read-only",
  "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
  "--mount", "type=bind,src=/home/you/project,dst=/repo,ro",
  "--entrypoint", "/opt/grimoire/bin/grimoire-mcp-project-docs",
  "ghcr.io/jabbott-iii/grimoire@sha256:<digest>", "--root", "/repo",
]
env_vars = ["DOCKER_HOST", "DOCKER_CONTEXT", "DOCKER_CONFIG"]
```
- Codex starts a stdio server with an empty environment plus a short allowlist (`HOME`, `PATH` and a few more). `env_vars` forwards the Docker variables if you set them; an unset one is skipped.
- Codex loads a project's `.codex/config.toml` only in a trusted project.
- If the first start times out, pull first, or raise `startup_timeout_sec` (seconds).

**Cursor**, `.cursor/mcp.json`. Cursor's docs define `${workspaceFolder}` as the folder that contains `.cursor/mcp.json`:
```json
{
  "mcpServers": {
    "project-docs": {
      "type": "stdio",
      "command": "docker",
      "args": [
        "run", "-i", "--rm", "--network", "none", "--read-only",
        "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
        "--mount", "type=bind,src=${workspaceFolder},dst=/repo,ro",
        "--entrypoint", "/opt/grimoire/bin/grimoire-mcp-project-docs",
        "ghcr.io/jabbott-iii/grimoire@sha256:<digest>", "--root", "/repo"
      ]
    }
  }
}
```
Cursor requires `"type": "stdio"`. A server's working directory is undocumented, so don't use a relative path.

**Gemini CLI**, `.gemini/settings.json`:
```json
{
  "mcpServers": {
    "project-docs": {
      "command": "docker",
      "args": [
        "run", "-i", "--rm", "--network", "none", "--read-only",
        "--cap-drop", "ALL", "--security-opt", "no-new-privileges",
        "--mount", "type=bind,src=/home/you/project,dst=/repo,ro",
        "--entrypoint", "/opt/grimoire/bin/grimoire-mcp-project-docs",
        "ghcr.io/jabbott-iii/grimoire@sha256:<digest>", "--root", "/repo"
      ]
    }
  }
}
```
- `${PWD}` works in `args` only if the shell that started Gemini CLI exports `PWD`. An unset variable stays literal text, so the example uses an absolute path.
- Gemini CLI drops inherited variables whose names contain `CERT`, `TOKEN`, `KEY` and similar words. `DOCKER_HOST`, `DOCKER_CONTEXT` and `DOCKER_CONFIG` pass; `DOCKER_CERT_PATH` doesn't, so set it under `env` if you use it.
- In an untrusted folder, a stdio server doesn't start.

## 5. MCP servers over Streamable HTTP

Prefer stdio. Nothing in this section has been run in a container.

**Why the server needs authorization.** In a container the server must listen on `0.0.0.0`, or a published port can't reach it. Grimoire's MCP hardening refuses a non-loopback address without authorization, and rightly: on rootful Docker, every host process and every container on the same bridge network can reach the container's address, whatever `-p` says. Rootless Docker hides that address from the host, but not from other containers.

```sh
docker run --rm --read-only --cap-drop ALL --security-opt no-new-privileges \
  -p 127.0.0.1:8931:8931 \
  --mount type=bind,src="$PWD",dst=/repo,ro \
  --mount type=bind,src=<absolute JWKS directory>,dst=/jwks,ro \
  --entrypoint /opt/grimoire/bin/grimoire-mcp-project-docs \
  ghcr.io/jabbott-iii/grimoire@sha256:<digest> \
  --root /repo serve-http --listen 0.0.0.0:8931 \
  --auth-resource http://127.0.0.1:8931/mcp \
  --auth-issuer <issuer URL> --auth-jwks /jwks/<file>
```
- **`-p 127.0.0.1:8931:8931`:** only this host can reach the port. Without the address, Docker publishes on every interface, even where UFW blocks the port. It needs Docker 28.0 or later: before that, hosts on the same network segment could reach ports published on `127.0.0.1`.
- **No `--network none`:** the published port needs a network. The server itself makes no outbound connections.
- **The JWKS directory:** mount the directory read-only and point `--auth-jwks` at the file in it. The server re-reads the file at most every 5 seconds, when its modification time changes, so it sees a file you replace. Whether a single-file mount would is UNVERIFIED. Deleting the file revokes every key.
- **`--auth-resource` is the exact URL clients use, host port included.** The server answers only to that host and port, and to each `--allowed-host`; any other `Host` gets 403. With `-p 127.0.0.1:9000:8931`, it is `http://127.0.0.1:9000/mcp`. Its path is the MCP endpoint, and `http://` is accepted only for loopback addresses.
- **Use `127.0.0.1`, not `localhost`:** `-p 127.0.0.1` publishes on IPv4 only.
- **Other names and browsers:** `--allowed-host` adds a `Host` value, and `--allowed-origin` a browser origin. No origin is allowed by default.
- **HTTPS:** the server speaks plain HTTP/1. Put a TLS-terminating proxy in front that forwards the `Host` header, and set `--auth-resource` to the `https://` URL clients use.
- **The client entry:** Grimoire writes it. In `grimoire.toml`, give the asset `transport = "streamable-http"` and `url = "http://127.0.0.1:8931/mcp"`.

The token checks and the other `--auth-*` options are in the [project-docs README](../catalog/mcp-servers/project-docs/README.md) and [ADR-0007](adr/0007-mcp-server-authorization.md).

**Local use on Linux: host networking.**
```sh
docker run --rm --network host --read-only --cap-drop ALL --security-opt no-new-privileges \
  --mount type=bind,src="$PWD",dst=/repo,ro \
  --entrypoint /opt/grimoire/bin/grimoire-mcp-project-docs \
  ghcr.io/jabbott-iii/grimoire@sha256:<digest> \
  --root /repo serve-http --listen 127.0.0.1:8931 --no-auth
```
- The container shares the host's network, so a `127.0.0.1` listener stays loopback-only and `--no-auth` is allowed. Any local process can still connect.
- Host networking ignores `-p`.
- Not on Docker Desktop: whether the host can reach a `127.0.0.1` listener in a host-network container there is UNVERIFIED. Not on rootless Docker before Engine 29.5, where the port can't be reached from the host.

## 6. Hooks (secret-guard)

`grimoire-tool-secret-guard` runs as a Claude Code hook, and Grimoire writes hook entries for the native binary only. Running the hook in a container has these constraints:
- **`-i`:** the hook event arrives on stdin.
- **The same path inside and out.** For a write, the hook reads the target file at the path in the event, to allow a secret that file already holds. Mount the project read-only at its identical absolute path (`src=<path>,dst=<path>,ro`), with `--network none`. Otherwise that exemption is lost; the hook fails safe and blocks more.
- **Paths that can't match:** files in a worktree outside the mount, and every path on a Windows host.
- **`--init`:** secret-guard installs no signal handlers either.
- **Time:** the hooks fail closed after 30 seconds, so a first image pull inside a hook can block the call.

**Use the native binary for hooks** (see the [secret-guard README](../catalog/tools/secret-guard/README.md)). The container suits a scan, for example of staged changes:
```sh
git diff --cached | docker run -i --rm --init --network none --read-only --cap-drop ALL \
  --security-opt no-new-privileges \
  --entrypoint /opt/grimoire/bin/grimoire-tool-secret-guard \
  ghcr.io/jabbott-iii/grimoire@sha256:<digest> scan
```

## 7. Limits and what is not tested

- **Run locally** (2026-11-17, rootless Docker 29.9.3 on amd64; nothing has run on GitHub yet):
  - three builds of the two-platform image, one of them on a fresh builder. The platform manifest digests matched; the index digest didn't;
  - `cargo xtask release image check` passed on a real build, and refused a changed binary, a corrupted blob and all seven tampered layouts from the Phase 6 review;
  - the cases of the release's tag script, `.github/scripts/image-tags.sh`, that pull requests run (below);
  - Grype 0.121.3 (database v6.1.10, built 2026-11-17T06:20:58Z) found nothing on either platform;
  - distroless's signatures verified with cosign 3.1.4. crane v0.22.1 pushed the layout with its index digest unchanged, and a cosign signature landed in the `sha256-<index hex>` tag;
  - `grimoire --version`, both asset binaries' `--version`, `init`, `add --apply` and `verify` on a bind mount with `--user 0:0`, an MCP `initialize` over stdio, and `grimoire-tool-secret-guard scan` on stdin. All ran with `--network none --read-only --cap-drop ALL --security-opt no-new-privileges`.
  - The arm64 image was built and scanned, but not run: there is no emulation here.
- **In CI** (written, not yet run): every pull request and release run pulls the image on `ubuntu-24.04` (linux/amd64) and `ubuntu-24.04-arm` (linux/arm64), whose Docker 28.0.4 is rootful. It pulls from a local registry that, like GHCR, has no referrers API.
  - It runs every binary's `--version`; the CLI on a mounted repository with `--init` and `--user`, then checks who owns the files; and an MCP `initialize` over stdio.
  - Pull requests also sign the image with a throwaway key and verify it, offline. Then they run the release's tag script against the local registry: a push and a tag, each twice; a version tag that names another digest, refused by push and by tag; a registry that can't be read, refused; `latest` moved off an unsigned image that claims no newer version, refused for an unsigned image that claims a newer one, and kept on a newer signed release.
  - Every Monday, `security.yml` verifies the latest release's image signature and scans every platform again with the same Grype gate.
- **The gate:** any medium, high or critical match fails the build, fixed or not. A finding with no fix blocks until it has an ignore rule with a reason in [`.grype.yaml`](../.grype.yaml).
- **What the scan can't see:** Grype finds Rust crates through the cargo-auditable data in each binary, and the workflow fails if a binary shows none. cargo-auditable doesn't record statically linked C code, which by inference includes musl's libc.
- **Never tested:** Docker Desktop, Podman, Windows hosts, rootful Docker and `--init` (CI runs both, but hasn't run yet), Streamable HTTP in a container (§5), a hook in a container (§6), and the client entries in §4 with a real client.
