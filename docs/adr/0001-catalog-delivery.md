# ADR-0001: Catalog delivery: embedded in the binary for v1, behind a `Source` abstraction

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16)
- **Date:** 2026-11-16

## Context

The `grimoire` command-line interface (CLI) resolves the assets a user selects in `grimoire.toml` against a catalog. The catalog holds each asset's manifest (its `asset.toml`, which describes the asset) plus its content files. That catalog has to reach the user's machine somehow. The constraints:

- **No network by default.** Installing makes no network calls by default and never executes asset content.
- **Reproducible.**
  - `grimoire.lock`, the lockfile, pins (records exactly) the resolved versions, their source, and the content hash of every written file.
  - `apply` must be idempotent (running it twice has the same effect as running it once) and reproducible in continuous integration (CI).
- **Frequent additions.** New assets are added regularly. Adding one is a data-only change under `catalog/`.
- **Integrity.** Release artifacts are signed and attested: each comes with a signed statement of how and where it was built (see ADR-0003, the release-tooling architecture decision record, or ADR). Whatever delivers the catalog needs integrity guarantees at least as strong.
- **Evolvable.** A remote catalog may be wanted later. Adding one must not break existing lockfiles.

## Options

### (a) Embedded catalog, versioned with the CLI

At build time, the validated `catalog/` tree is packed into a deterministic archive (the same tree always gives the same bytes) that is compiled into the binary. Each CLI release carries a snapshot of the catalog.

- **Pros:**
  - Zero network code in the install path.
  - Works offline and air-gapped (on a machine with no network connection at all).
  - Catalog integrity is covered by the same checksums, signatures, software bill of materials (SBOM, the list of everything the binary contains) and provenance (the signed build record) as the binary. So there is no second trust root: no second key or identity that users must decide to trust.
  - Reproducible: the CLI version plus the lockfile determine the output.
  - Smallest implementation.
- **Cons:**
  - New or updated assets reach users only through a CLI release, so the catalog can change only as often as the CLI is released. This is mitigated by automated releases.
  - The binary grows with the catalog. Text assets are small and compress well, but this should be measured in Phase 1. (Phases are the steps of the implementation plan in `PLAN.md`; Phase 1 sets up the workspace, model, validation and CI.)
  - v1 embeds only the latest version of each asset, so a lockfile pinning an older version can't be re-rendered (its files written again) by a newer CLI (see Consequences).

### (b) Signed remote index fetched on `update`

A static index plus content blobs (the files themselves), hosted (e.g. on GitHub Pages or Releases), fetched only by `grimoire update` and cached locally. Every other command works from the cache and the lockfile.

- **Pros:** the catalog ships independently of CLI releases, and historical versions can be served.
- **Cons:**
  - Needs a real update-security design:
    - signing keys or Sigstore identities (Sigstore signs with a short-lived certificate issued to an identity, such as a CI workflow, instead of a long-lived key);
    - plus rollback, freeze and mix-and-match protection (the TUF threat model). The Update Framework (TUF) names these attacks: serving an older index (rollback), serving a stale index forever (freeze), and combining files from different index versions (mix-and-match).
  - Adds an HTTP and TLS (Transport Layer Security, the encryption under HTTPS) stack to the CLI, cache management, mirror and outage handling, and a second trust root to document and monitor.
  - Much larger scope and attack surface (code an attacker can reach) for v1.

### (c) Git checkout of the catalog repository (considered, not proposed)

`update` clones or fetches this repository at a tag.

- **Pros:** reuses Git's transport and history.
- **Cons:**
  - Depends on `git` being installed, or on linking a Git implementation.
  - Exposes the whole repository, including CI and Rust sources, as the "catalog".
  - Integrity depends on tag signing that we'd need to verify ourselves.
- **Verdict:** strictly worse than (b) for the same trust problem.

## Decision (proposed)

Adopt **(a) for v1**, implemented behind a `Source` abstraction so that (b) can be added later without changing the lockfile format.

- `Source` is a small trait (a Rust interface) with:
  - an identity (recorded in the lock as `source`);
  - an index of available `(id, kind, version, manifest)`;
  - read access to an asset's content files by relative path.
- v1 ships one implementation, `builtin`.
- The lock records each asset's `source = "builtin"`, its version, the manifest digest (a hash of the manifest), and the per-output content hashes. (An output is one unit of content Grimoire writes: a file, a managed block or a config entry.)
  - Content hashes are source-independent. The same asset version from a future remote source produces identical hashes, so moving sources changes only the `source` field.
  - The lockfile has a top-level format `version` from day one, so new source kinds are an additive change.
- **Upgrading the CLI must not touch the lock.**
  - If no resolved asset version changes, the CLI version is not written to the lockfile, so an upgrade produces no lockfile churn (no change to the file).
  - `min-cli-version` constraints live in the manifests and are checked at resolve time (when the CLI works out which asset versions to install).
- **Build integration (a Phase 1 detail, recorded here because it constrains packaging).**
  - The archive must sit inside the published CLI package. `cargo publish` only packages files under the crate directory (a crate is Rust's unit of packaging). So a build script that reaches up into `../../catalog` would break `cargo install`.
  - Plan: `cargo xtask pack-catalog` writes a deterministic archive into the catalog crate. CI checks that it is up to date, the same way generated JSON Schemas are checked.
  - The exact mechanism is decided and measured in Phase 1.

## Consequences

- **Pinned versions can become unavailable.**
  - When a lock pins a version that the running CLI no longer embeds, `verify` still works, because it needs only hashes.
  - `apply` fails with a resolution error only if it must write that asset's content, for example because a client was added. The error names the fix: `grimoire update <asset>`, or use a CLI release that contains the pinned version.
  - Serving historical versions is one of the motivations for (b).
- **Catalog changes need releases.** Release automation (see ADR-0003, which chose release-plz for versions and a hand-written release pipeline) must make frequent catalog-only releases cheap.
- **Tests and threat model.** CI validates the catalog before packing, so an invalid catalog can't be embedded. `docs/security-model.md` treats the catalog as part of the signed binary.
- **Revisiting.** Reconsider (b) once real users need catalog updates faster than the release cadence, or need historical versions. At that point write a new ADR covering index signing (Sigstore/TUF), caching, and the user experience of `update`.
