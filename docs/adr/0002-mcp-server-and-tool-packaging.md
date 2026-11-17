# ADR-0002: Packaging MCP servers and tools: separate binaries, released with the CLI

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16)
- **Date:** 2026-11-16
- **Specs:** `RMCP`, `MCP-SPEC`, `CRATES-TP`, `NPM-TP`, `PYPI-TP`, `MCP-REGISTRY` in [`../specs.md`](../specs.md)

## Context

MCP servers and tools are code-bearing assets: Rust crates whose source lives in their asset directory under `catalog/` (see `architecture.md` §4). Installing such an asset writes only client configuration that points at an executable. Grimoire never downloads or runs anything at install time. That leaves two questions: what the executable is, and how it reaches the user's machine.

Constraints:

- **Data-only additions.** Adding an asset must not require core code changes. A new asset can be added with files under `catalog/` only.
- **Small installer surface.** The `grimoire` CLI runs in CI with write access to repositories. Its attack surface and dependency tree should stay as small as possible. It needs no async runtime, HTTP stack or auth code.
- **Server requirements.** MCP servers need tokio, an HTTP server stack, rmcp, and an auth/validation layer (`RMCP` gap 2).
- **Registry first publishes need a token.** On crates.io, every new crate's first publish needs an API token (`CRATES-TP`). On npm, a package must exist before trusted publishing can be configured (`NPM-TP`). Every new published package is therefore a manual, credentialed bootstrap step for the maintainer.
- **Versions.** Assets have independent SemVer versions.

## Options

### (A) One binary per server or tool (`grimoire-mcp-<id>`, `grimoire-tool-<id>`), each its own crate under `catalog/`

**Pros:**
- The CLI never links server code.
- Each binary contains only its own dependencies. That gives precise SBOMs and per-binary attestations, and lets an OS sandbox profile be applied per server.
- A vulnerability in one server's stack doesn't force a CLI release or affect users of other servers.
- Crate version = asset version, one-to-one.
- Adding a server is one new directory. The workspace glob picks it up with no core edit.

**Cons:**
- Users install each server binary separately.
- The build matrix grows: N binaries × 6 targets. This is mitigated by building all binaries in one `cargo build --workspace` per target.
- If each binary were also published to every package manager, it would multiply packages, and with them the manual bootstrap steps per new asset.

### (B) Subcommands of the single `grimoire` binary (`grimoire mcp serve <id>`)

**Pros:**
- One artifact everywhere. Anyone who has Grimoire has every server.
- The server version always matches the embedded catalog (ADR-0001).
- The fewest moving parts for users.

**Cons:**
- The installer binary links tokio, hyper, rmcp and the auth stack for every user, even users who never run a server. That is a larger attack surface and audit burden on the binary that writes into repositories.
- Every new server must be added as a dependency of the CLI crate and to its dispatch table: a core code change, or generated wiring.
- Publishing the CLI to crates.io then requires publishing every server crate, and each new crate needs a manual first publish with a token.
- A CVE in any server forces a CLI release.

### (C) Two binaries: `grimoire` (installer) plus one `grimoire-mcp` multiplexer containing every server

**Pros:**
- The installer stays small.
- One extra binary ships on every channel.
- Users get all servers at once.

**Cons:**
- Still needs per-server wiring in the multiplexer, either a core edit or a generated registry.
- If the multiplexer is published to crates.io, every server crate must be published too.
- All servers share one process image, so least-privilege isolation between servers is lost.

## Decision (proposed)

Adopt **(A), separate binaries**, with a deliberately narrow v1 distribution scope for them:

1. **Source and naming.**
   - Each MCP server and tool is a workspace crate in its asset directory, with one `[[bin]]`. Binaries are named `grimoire-mcp-<id>` and `grimoire-tool-<id>`; final naming is in ADR-0004.
   - The binary name is declared in `asset.toml` and validated against `Cargo.toml`.
   - The crate version must equal the `asset.toml` version, which is validated.
2. **No crates.io publishing for asset crates** (`publish = false`). This removes the per-asset credentialed bootstrap step and keeps crates.io limited to the CLI and its fixed set of core crates.
3. **Released together with the CLI.**
   - The same release workflow builds every asset binary for all six targets.
   - Each binary is attached to the GitHub Release as its own archive, with the same SHA-256 checksums, CycloneDX SBOM, build-provenance attestation and keyless signature as the CLI (ADR-0003).
   - All binaries are also included in the GHCR image (Phase 6), so a `docker run -i --rm --entrypoint grimoire-mcp-<id> ghcr.io/…@sha256:…` command can act as a stdio server with nothing installed locally. The exact form is fixed in Phase 6.
4. **Package managers carry only `grimoire` in v1.** npm, PyPI, Homebrew, Scoop and winget get the CLI only. Extending them to asset binaries, likely as extra binaries inside the existing per-platform packages rather than new packages, is revisited after v1 with real usage data.
5. **The CLI helps without downloading.**
   - `grimoire doctor` checks that each referenced binary is on `PATH` at a version compatible with the locked asset version (`--version --json`).
   - `apply` and `doctor` print the exact install command.
   - A per-asset override `runtime = "docker"` lets adapters render the MCP config entry as a pinned-by-digest `docker run` command instead of a local binary. *(Implementation note, Phase 1: deferred to Phase 6, where the image exists.)*

   An explicit, opt-in `grimoire fetch` that downloads and verifies binaries is out of scope for v1. It would need its own ADR, because it introduces network code into the CLI.
6. **Shared server hardening lives in one core crate.** A `crates/` library introduced in Phase 3 contains:
   - transport setup (stdio, plus Streamable HTTP bound to localhost by default);
   - Origin validation enforced, because rmcp's default is off;
   - request size limits and timeouts;
   - stderr-only logging on stdio;
   - the RFC 9728 / bearer-validation auth layer.

   New servers depend on it but never modify it, so they inherit secure defaults.

## Consequences

- **Release size.** The release workflow's matrix and asset count grow with the catalog, and release time must be watched. Building per target in one cargo invocation shares compilation of common dependencies.
- **User setup.** Users of MCP servers do one extra install step, unless they use Docker. That step is documented and `doctor` checks it.
- **Data-only additions mostly hold.** A new server or tool is added under `catalog/` only. Its binary appears in the next release with no maintainer-side registry setup. Whether the release workflow can discover binaries automatically, for example from `cargo metadata`, must be verified in Phase 5; it must not be a hand-edited list.
- **MCP Registry listing.** Without crates.io packages for servers, a listing would have to use the `oci` registry type, with one image (or annotation) per server (`MCP-REGISTRY`). The registry is in preview, so this is deferred.
- **Revisiting.** Reconsider (B) or (C) if user research shows the separate install step is the main adoption barrier and the installer-surface concern can be met another way, for example with a separately built "full" CLI variant.
