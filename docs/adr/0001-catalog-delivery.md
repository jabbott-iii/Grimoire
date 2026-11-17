# ADR-0001: Catalog delivery: embedded in the binary for v1, behind a `Source` abstraction

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16)
- **Date:** 2026-11-16

## Context

The `grimoire` CLI resolves the assets a user selects in `grimoire.toml` against a catalog: manifests plus content files. That catalog has to reach the user's machine somehow. The constraints:

- **No network by default.** Installing makes no network calls by default and never executes asset content.
- **Reproducible.** `grimoire.lock` pins resolved versions, their source, and the content hash of every written file. `apply` must be idempotent and reproducible in CI.
- **Frequent additions.** New assets are added regularly. Adding one is a data-only change under `catalog/`.
- **Integrity.** Release artifacts are signed and attested (see the release-tooling ADR). Whatever delivers the catalog needs integrity guarantees at least as strong.
- **Evolvable.** A remote catalog may be wanted later. Adding one must not break existing lockfiles.

## Options

### (a) Embedded catalog, versioned with the CLI

At build time, the validated `catalog/` tree is packed into a deterministic archive that is compiled into the binary. Each CLI release carries a snapshot of the catalog.

- **Pros:**
  - Zero network code in the install path.
  - Works offline and air-gapped.
  - Catalog integrity is covered by the same checksums, signatures, SBOM and provenance as the binary, so there is no second trust root.
  - Trivially reproducible: CLI version plus lockfile determines output.
  - Smallest implementation.
- **Cons:**
  - New or updated assets reach users only through a CLI release, so catalog cadence equals release cadence. This is mitigated by automated releases.
  - The binary grows with the catalog. Text assets are small and compress well, but this should be measured in Phase 1.
  - v1 embeds only the latest version of each asset, so a lockfile pinning an older version can't be re-rendered by a newer CLI (see Consequences).

### (b) Signed remote index fetched on `update`

A static index plus content blobs, hosted (e.g. on GitHub Pages or Releases), fetched only by `grimoire update` and cached locally. Every other command works from the cache and the lockfile.

- **Pros:** the catalog ships independently of CLI releases, and historical versions can be served.
- **Cons:**
  - Needs a real update-security design: signing keys or Sigstore identities, plus rollback, freeze and mix-and-match protection (the TUF threat model).
  - Adds an HTTP/TLS stack to the CLI, cache management, mirror and outage handling, and a second trust root to document and monitor.
  - Much larger scope and attack surface for v1.

### (c) Git checkout of the catalog repository (considered, not proposed)

`update` clones or fetches this repository at a tag.

- **Pros:** reuses git's transport and history.
- **Cons:**
  - Depends on `git` being installed, or on linking a git implementation.
  - Exposes the whole repo, including CI and Rust sources, as the "catalog".
  - Integrity depends on tag signing that we'd need to verify ourselves.
- **Verdict:** strictly worse than (b) for the same trust problem.

## Decision (proposed)

Adopt **(a) for v1**, implemented behind a `Source` abstraction so that (b) can be added later without changing the lockfile format.

- `Source` is a small trait:
  - an identity (recorded in the lock as `source`);
  - an index of available `(id, kind, version, manifest)`;
  - read access to an asset's content files by relative path.
- v1 ships one implementation, `builtin`.
- The lock records each asset's `source = "builtin"`, its version, the manifest digest, and the per-output content hashes.
  - Content hashes are source-independent. The same asset version from a future remote source produces identical hashes, so moving sources changes only the `source` field.
  - The lockfile has a top-level format `version` from day one, so new source kinds are an additive change.
- **Upgrading the CLI must not touch the lock.** If no resolved asset version changes, the CLI version is not written to the lockfile, so an upgrade produces no lockfile churn. `min-cli-version` constraints live in the manifests and are checked at resolve time.
- **Build integration (a Phase 1 detail, recorded here because it constrains packaging).**
  - The archive must sit inside the published CLI package. `cargo publish` only packages files under the crate directory, so a build script that reaches up into `../../catalog` would break `cargo install`.
  - Plan: `cargo xtask pack-catalog` writes a deterministic archive into the catalog crate. CI checks that it is up to date, the same way generated JSON Schemas are checked.
  - The exact mechanism is decided and measured in Phase 1.

## Consequences

- **Pinned versions can become unavailable.**
  - When a lock pins a version that the running CLI no longer embeds, `verify` still works, because it needs only hashes.
  - `apply` fails with a resolution error only if it must write that asset's content. Example: a client was added. The error names the remediation: `grimoire update <asset>`, or use a CLI release that contains the pinned version.
  - Serving historical versions is one of the motivations for (b).
- **Catalog changes need releases.** Release automation (see ADR-0003) must make frequent catalog-only releases cheap.
- **Tests and threat model.** CI validates the catalog before packing, so an invalid catalog can't be embedded. `docs/security-model.md` treats the catalog as part of the signed binary.
- **Revisiting.** Reconsider (b) once real users need catalog updates faster than the release cadence, or need historical versions. At that point write a new ADR covering index signing (Sigstore/TUF), caching, and the `update` UX.
