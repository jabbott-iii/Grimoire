# ADR-0002: Packaging MCP servers and tools: separate binaries, released with the CLI

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16). Amended by [ADR-0009](0009-release-pipeline.md) (2026-11-17): the release workflow finds asset binaries with `cargo metadata`, which settles the Phase 5 check under Consequences. Amended by [ADR-0011](0011-container-image.md) (2026-11-17): the image's exact form is fixed, and `runtime = "docker"` is deferred past v1.
- **Date:** 2026-11-16
- **Specs:** `RMCP`, `MCP-SPEC`, `CRATES-TP`, `NPM-TP`, `PYPI-TP`, `MCP-REGISTRY` in [`../specs.md`](../specs.md), the register of verified external specs. An ID in parentheses below, such as (`CRATES-TP`), names the entry a fact comes from.

## Context

Model Context Protocol (MCP) servers and tools are code-bearing assets.

- An MCP server is a program that offers an AI client tools, resources and prompts over MCP. A tool is a program the client runs, for example as a hook (a command the client runs before or after an action).
- Both are Rust crates (a crate is Rust's unit of packaging) whose source lives in their asset directory under `catalog/` (see `architecture.md` §4, the repository layout).

Installing such an asset writes only client configuration that points at an executable. Grimoire never downloads or runs anything at install time. That leaves two questions: what the executable is, and how it reaches the user's machine.

Constraints:

- **Data-only additions.** Adding an asset must not require core code changes. A new asset can be added with files under `catalog/` only.
- **Small installer surface.**
  - The `grimoire` command-line interface (CLI) runs in continuous integration (CI) with write access to repositories.
  - Its attack surface (the code an attacker could reach) and dependency tree should stay as small as possible.
  - It needs no async runtime (a library that schedules asynchronous tasks), HTTP stack or auth code.
- **Server requirements.** MCP servers need tokio (an async runtime for Rust), an HTTP server stack, rmcp (the official Rust MCP software development kit, or SDK), and an auth/validation layer. That layer is `RMCP` gap 2: rmcp's own auth support is for clients only.
- **Registry first publishes need a token.**
  - On crates.io (Rust's package registry), every new crate's first publish needs an API token (`CRATES-TP`).
  - On npm (the JavaScript package registry), a package must exist before trusted publishing can be configured (`NPM-TP`). Trusted publishing lets CI publish with a short-lived identity token instead of a stored one.
  - Every new published package is therefore a manual bootstrap step (a one-time setup) that needs the maintainer's credentials.
- **Versions.** Each asset has its own version, under semantic versioning (SemVer).

## Options

### (A) One binary per server or tool (`grimoire-mcp-<id>`, `grimoire-tool-<id>`), each its own crate under `catalog/`

**Pros:**
- The CLI never links server code (never compiles it in).
- Each binary contains only its own dependencies. That gives:
  - precise software bills of materials (SBOMs, the list of what a binary contains);
  - per-binary attestations (signed records of how each binary was built);
  - an operating-system sandbox profile (OS rules that limit what a process may do) applied per server.
- A vulnerability in one server's stack doesn't force a CLI release or affect users of other servers.
- Crate version = asset version, one to one.
- Adding a server is one new directory. The workspace glob (`catalog/tools/*` and `catalog/mcp-servers/*` in the root `Cargo.toml`'s `members`) picks it up with no core edit.

**Cons:**
- Users install each server binary separately.
- The build matrix (the set of builds CI runs) grows: N binaries × 6 targets (platforms). This is mitigated by building all binaries in one `cargo build --workspace` per target.
- If each binary were also published to every package manager, it would multiply packages, and with them the manual bootstrap steps per new asset.

### (B) Subcommands of the single `grimoire` binary (`grimoire mcp serve <id>`)

**Pros:**
- One artifact everywhere. Anyone who has Grimoire has every server.
- The server version always matches the embedded catalog (ADR-0001, the architecture decision record, or ADR, that embeds the catalog in the binary).
- The fewest moving parts for users.

**Cons:**
- The installer binary links tokio, hyper (an HTTP library), rmcp and the auth stack for every user, even users who never run a server. That is a larger attack surface and audit burden on the binary that writes into repositories.
- Every new server must be added as a dependency of the CLI crate and to its dispatch table (the code that maps a subcommand to the server it runs): a core code change, or generated wiring.
- Publishing the CLI to crates.io then requires publishing every server crate, and each new crate needs a manual first publish with a token.
- A CVE (a published vulnerability, from the Common Vulnerabilities and Exposures list) in any server forces a CLI release.

### (C) Two binaries: `grimoire` (installer) plus one `grimoire-mcp` multiplexer containing every server

**Pros:**
- The installer stays small.
- One extra binary ships on every channel (each way users install Grimoire, such as npm or Homebrew).
- Users get all servers at once.

**Cons:**
- The multiplexer (one binary that runs whichever server is asked for) still needs per-server wiring, either a core edit or a generated registry.
- If the multiplexer is published to crates.io, every server crate must be published too.
- All servers share one process image, so least-privilege isolation between servers (each server getting only the access it needs) is lost.

## Decision (proposed)

Adopt **(A), separate binaries**, with a deliberately narrow v1 distribution scope for them:

1. **Source and naming.**
   - Each MCP server and tool is a workspace crate in its asset directory, with one `[[bin]]` (binary) target. Binaries are named `grimoire-mcp-<id>` and `grimoire-tool-<id>`; final naming is in ADR-0004, which fixes every package and binary name.
   - The binary name is declared in `asset.toml` and validated against `Cargo.toml`.
   - The crate version must equal the `asset.toml` version, which is validated.
2. **No crates.io publishing for asset crates** (`publish = false`). This removes the per-asset bootstrap step that needs credentials, and keeps crates.io limited to the CLI and its fixed set of core crates.
3. **Released together with the CLI.**
   - The same release workflow builds every asset binary for all six targets.
   - Each binary is attached to the GitHub Release as its own archive, and gets the same as the CLI (ADR-0003, the release-tooling decision):
     - SHA-256 checksums;
     - a CycloneDX SBOM (CycloneDX is a standard SBOM format);
     - a build-provenance attestation;
     - a keyless signature (made with a short-lived certificate from Sigstore, an open-source signing service, so there is no long-lived key to protect).
   - All binaries are also included in the GitHub Container Registry (GHCR) image (Phase 6, the container-image phase of `PLAN.md`), so a `docker run -i --rm --entrypoint grimoire-mcp-<id> ghcr.io/…@sha256:…` command can act as a stdio server (one that talks to its client over standard input and output) with nothing installed locally. Phase 6 fixed the exact form: asset binaries sit in `/opt/grimoire/bin/`, off the image's `PATH`, so `--entrypoint` takes that absolute path ([ADR-0011](0011-container-image.md), [`docs/container.md`](../container.md)).
4. **Package managers carry only `grimoire` in v1.**
   - npm, PyPI (the Python Package Index), Homebrew (a package manager for macOS and Linux), Scoop and winget (package managers for Windows) get the CLI only.
   - Extending them to asset binaries, likely as extra binaries inside the existing per-platform packages rather than new packages, is revisited after v1 with real usage data.
5. **The CLI helps without downloading.**
   - `grimoire doctor` checks that each referenced binary is on `PATH` at a version compatible with the locked asset version (`--version --json`).
   - `apply` and `doctor` print the exact install command.
   - A per-asset override `runtime = "docker"` lets adapters (the code that writes one client's files) render the MCP config entry as a `docker run` command pinned by digest (the image's SHA-256, which can't change), instead of a local binary. *(Implementation note, Phase 1: deferred to Phase 6, where the image exists. Phase 6: deferred past v1 by the maintainer; `grimoire.toml` still rejects the field, and [`docs/container.md`](../container.md) shows hand-written, digest-pinned `docker run` entries instead ([ADR-0011](0011-container-image.md)).)*

   An explicit, opt-in `grimoire fetch` that downloads and verifies binaries is out of scope for v1. It would need its own ADR, because it introduces network code into the CLI.
6. **Shared server hardening lives in one core crate.** A `crates/` library introduced in Phase 3 (the phase that built one example asset of each kind) holds the secure defaults and limits:
   - transport setup (stdio, plus Streamable HTTP, MCP's HTTP transport, bound to localhost by default);
   - Origin validation (refusing a request whose `Origin` header names a site that isn't allowed, so a web page can't reach the server), enforced because rmcp's default is off;
   - request size limits and timeouts;
   - stderr-only logging on stdio;
   - the RFC 9728 / bearer-validation auth layer. RFC 9728 is an Internet standard (a Request for Comments, or RFC) for the metadata that tells a client where to get a token, and bearer validation checks the token a client sends.

   New servers depend on it but never modify it, so they inherit secure defaults.

## Consequences

- **Release size.** The release workflow's matrix and asset count grow with the catalog, and release time must be watched. Building per target in one cargo invocation shares compilation of common dependencies.
- **User setup.** Users of MCP servers do one extra install step, unless they use Docker. That step is documented and `doctor` checks it.
- **Data-only additions mostly hold.**
  - A new server or tool is added under `catalog/` only. Its binary appears in the next release with no maintainer-side registry setup.
  - Whether the release workflow can discover binaries automatically, for example from `cargo metadata`, must be verified in Phase 5 (the release-pipeline phase); it must not be a hand-edited list.
- **MCP Registry listing.**
  - Without crates.io packages for servers, a listing in the MCP Registry (the official directory of MCP servers) would have to use the `oci` registry type, with one image (or annotation) per server (`MCP-REGISTRY`). `oci` means a container image; OCI is the Open Container Initiative, which defines the image format.
  - The registry is in preview, so this is deferred.
- **Revisiting.** Reconsider (B) or (C) if user research shows the separate install step is the main adoption barrier and the installer-surface concern can be met another way, for example with a separately built "full" CLI variant.
