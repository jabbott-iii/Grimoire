# ADR-0006: Root-confined filesystem access with cap-std, and the deny.toml entries it needs

- **Status:** Accepted (maintainer chose this option on 2026-11-16, during Phase 2)
- **Date:** 2026-11-16

## Context

The installer writes into users' repositories. The brief requires writes that are "atomic and confined to the project root", and that "path traversal, absolute paths and symlink escapes" are rejected. `architecture.md` §7.4 named cap-std for this.

Two parts of the problem are already solved before any I/O happens:
- **Path strings.** Every managed path is a `RelPath`: ASCII, relative, `/`-separated, with no `..`, drive or UNC prefixes, `:`, or Windows device names. A destination string can't name anything outside the root.
- **Policy.** Grimoire refuses any symlink at or above a managed path, even one that stays inside the project. Writing through a link would put content somewhere the user didn't expect.

What remains is **races**. With path-based I/O, a check such as "this directory is not a symlink" can be invalidated before the write that relies on it. Someone who can write into the project directory, such as another user of a shared checkout, could swap a directory for a symlink between the two steps and redirect a write outside the root.

Adding cap-std 4.0.3 to the dependency graph failed `cargo deny check` under the Phase 1 policy, which says "never loosen an entry to get CI green":
- **License:** `winx` 0.36.4, Windows-only, via `cap-primitives`, is licensed `Apache-2.0 WITH LLVM-exception` only.
- **Duplicate versions:**
  - `io-lifetimes` 2.0.4 (via `fs-set-times`) next to 3.0.1;
  - `windows-sys` 0.59.0 (via `winx` and `fs-set-times`) and 0.60.2 (via `io-extras`) next to 0.61.2;
  - `windows-targets` 0.52.6 and 0.53.5, with their `windows_x86_64_msvc` and `windows_aarch64_msvc` crates.
- **Executables:** the older `windows_*_msvc` crates ship prebuilt import libraries (`lib/windows.0.5x.0.lib`), which `bans.build` rejects.

This put two approved decisions in conflict. The maintainer was asked and chose cap-std.

## Options

### (a) cap-std, with scoped deny.toml entries (chosen)

All access goes through a `cap_std::fs::Dir` for the project root:
- **Confinement:** cap-std uses `openat2(RESOLVE_BENEATH)` on Linux, and a handle-relative resolver that opens one component at a time elsewhere. A path can't leave the root even if the tree changes during the operation.
- **Walking to the parent:** Grimoire opens each directory component with `open_dir_nofollow`, which refuses a symlink at every step.
- **Atomic writes:** `cap-tempfile` creates the temporary file inside the destination directory. Grimoire syncs it and renames it over the target.

Costs:
- About 15 transitive crates.
- A dependency whose repository moved from the Bytecode Alliance to a single maintainer (`sunfishcode/cap-std`, 2026-10-15).
- A history that includes a critical sandbox escape in the userspace resolver, RUSTSEC-2026-0482, fixed in 4.0.3.

### (b) std only, with per-component checks

`symlink_metadata` on every component, then path-based writes through a temporary file and `rename`.

- Gains: no new dependencies and no deny.toml changes.
- Cost: it leaves the check-then-use race described above, so a concurrent local attacker could still escape the root.

## Decision

Use (a). The deny.toml entries are scoped as tightly as cargo-deny allows, and each carries a reason that cites this ADR:

| Entry | What it allows |
|---|---|
| `licenses.exceptions` | `winx` only, for `Apache-2.0 WITH LLVM-exception`. The exception only relaxes Apache-2.0's terms. |
| `bans.skip` | Exact versions only: `io-lifetimes@2.0.4`, `windows-sys@0.59.0`, `windows-sys@0.60.2`, `windows-targets@0.52.6`, `windows-targets@0.53.5`, `windows_x86_64_msvc@0.52.6`, `windows_aarch64_msvc@0.52.6`. |
| `bans.build.bypass` | Four import libraries, each pinned to its path and the SHA-256 of the bytes cargo verified from crates.io. |

The policy's intent still holds:
- No license is allowed for any other crate.
- No duplicate is tolerated beyond the listed versions.
- No other prebuilt file is accepted.

`cargo deny check` still fails on anything new.

## Consequences

- **Version floors.**
  - `cap-std`, `cap-fs-ext` and `cap-tempfile` are required at `>= 4.0.3`, because 4.0.0–4.0.2 carry RUSTSEC-2026-0482.
  - The lockfile resolves `rustix` 1.1.6, which fixes RUSTSEC-2026-0486.
- **Maintenance.** When upstream unifies a duplicate, `cargo deny check` reports the skip as unused, and the entry is removed in the same change. The import-library checksums change only with those crate versions, and Dependabot PRs that touch them must update the bypass entries.
- **Durability.** cap-tempfile does not fsync. Grimoire syncs the temporary file before the rename, and syncs the directory afterwards on Unix (best effort). Windows has no directory sync.
- **Windows.** cap-std performs the final rename and removal path-based on Windows, after resolving both parents inside the sandbox. Confinement holds, but on Windows the last step is not handle-relative. This is recorded in `docs/security-model.md`.
- **Reversibility.** All filesystem access is in one module (`crates/agent-grimoire-install/src/fs.rs`). Moving to (b), or to another library, changes that module only.
