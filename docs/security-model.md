# Grimoire security model

- **Status:** first version (Phase 2). It covers the installer's filesystem threat model.
  - Phase 3 adds MCP server permissions and prompt injection in rule and skill content.
  - Phase 7 completes the document.
- **Date:** 2026-11-16

## 1. What Grimoire does on a user's machine

`grimoire` reads a catalog that is embedded in its own signed binary ([ADR-0001](adr/0001-catalog-delivery.md)). It writes files inside one directory, the **project root**, and keeps a record of everything it wrote (`grimoire.lock`). Nothing else.

- **No execution.** Installing never executes asset content, runs no install hooks, and never runs the tool or MCP server binaries it configures. `doctor` only looks for them on `PATH`.
- **No network.** The install path contains no network code.
- **No secrets.** Grimoire never stores secret values. MCP server configuration refers to environment variables by name (`${VAR}`).

## 2. Trust boundaries

| Input | Trusted? | Why |
|---|---|---|
| The embedded catalog | Yes, as part of the binary | Validated in CI before packing (`cargo xtask pack-catalog --check`), parsed and validated again at load, and covered by the binary's checksums, signature and provenance (Phase 5). |
| `grimoire.toml`, `grimoire.lock` | Untrusted input | Parsed strictly: unknown fields are errors, every path is a `RelPath`, and the lock is validated for ownership consistency. They are part of the repository and are reviewed like code. |
| Every other file in the project | Hostile | It may hold symlinks, special files, broken markers, invalid JSON, or edits to managed content. |
| The filesystem tree itself | Hostile and changing | It may change between Grimoire's checks and its writes. |

## 3. Threats and mitigations

| # | Threat | Mitigation | Result |
|---|---|---|---|
| F1 | A destination or lock path escapes the root: `..`, an absolute path, a drive or UNC prefix, `:` (alternate data streams) | Every path is a `RelPath`. It is ASCII, relative and `/`-separated, with no `.` or `..` components, no `<>:"\|?*`, no control characters, and no trailing `.` or space. Asset destinations are anchors that adapters resolve, so an asset cannot name a literal path. | Rejected while parsing (exit 3) |
| F2 | Windows device names (`CON`, `aux.md`, `COM1`, …) | Rejected by `RelPath` in every component, with or without an extension. | Exit 3 |
| F3 | A symlink at or above a managed path, inside or outside the root | Each directory is opened with `open_dir_nofollow`, one component at a time from the root, after a `symlink_metadata` check. Files are opened no-follow and non-blocking, and the opened handle is checked again. `grimoire.toml`, `grimoire.lock` and `--write-new` sidecars are read the same way, and sidecars are checked before anything is written. | Refused (exit 6). Nothing is written. |
| F4 | A race that swaps a directory for a symlink between check and use | All access goes through a cap-std handle on the root ([ADR-0006](adr/0006-filesystem-confinement.md)): `openat2(RESOLVE_BENEATH)` on Linux, and a handle-relative resolver elsewhere. | The path can't leave the root. The swapped symlink is refused at that component. |
| F5 | Special files (FIFO, device, socket) at a managed path | Checked with `symlink_metadata` before opening. Opens are non-blocking and re-checked afterwards, so a FIFO swapped in after the check can't hang Grimoire. | Exit 6 |
| F6 | An oversized file exhausts memory | Reads are capped at 8 MiB, enforced while reading. | Exit 6 |
| F7 | Managed content the user edited is overwritten | Each unit is compared three ways: on disk, locked and desired (`architecture.md` §7.3). Edited, deleted, or unmanaged-in-the-way content is a **conflict**. `apply` checks every precondition first and writes nothing (exit 4). Only `--force` overwrites. `--write-new` writes `<path>.new` beside the file and leaves it alone. | Never silent |
| F8 | Removal deletes the user's content | Only owned units are removed: a whole file, a marker-delimited block, or one JSON member or array element. A file is deleted only if Grimoire created it (`created-file` in the lock). A JSON container is removed only if Grimoire created it and it is empty (`created-keys`). Edited content that is no longer wanted is a conflict, not a deletion. | Only Grimoire's content is removed |
| F9 | A shared file is corrupted by an edit Grimoire can't do safely | Grimoire refuses the file, even with `--force`, when **Markdown blocks** have malformed, unterminated, nested or duplicate markers, or when **JSON** is not strict RFC 8259 or has duplicate keys on Grimoire's path. Markers inside fenced code are ignored, and a block is never appended inside an unclosed fence. | Exit 4, the file is untouched |
| F10 | A crash or interruption leaves a half-written file | Every file is written to a temporary file in the same directory, fsynced, then renamed over the target. The lock is written last. A rerun converges, because a unit already in its desired state is adopted rather than reported as drift. | No torn files |
| F11 | A file changes between `plan` and the write | `apply` re-reads each file just before writing and aborts if it differs from what was planned. | Exit 4. Rerun. |
| F12 | Two paths that differ only in case | Rejected when planning, before any write. This covers both two desired paths, and a locked path that a new catalog version renames only in case. | Exit 3 |
| F13 | Line-ending conversion (`core.autocrlf`) is read as drift | Text hashes fold `\r\n` to `\n`. Bytes on disk that already match are left as they are. | No false conflict |
| F14 | An asset update silently gains permissions | Declared permissions must be accepted in `grimoire.toml`. A new version that needs more stops resolution until the user accepts (exit 3). | No silent escalation, with the gap noted in §4 |
| F15 | Installing for one client grants something to another client | `plan` and `apply` warn when Cursor also loads `.claude/agents/` (it has no per-tool allow-list), when Cursor and Copilot also run `.claude/settings.json` hooks, and when Copilot also starts `.mcp.json` servers (`specs.md`). | Visible, not prevented |
| F16 | Hooks run without a trust prompt | Claude Code runs project hooks without a trust dialog under `-p` and the SDK (`CC-SET`). Hooks can therefore come only from tool assets with an explicit `hooks` grant. They are written in exec form (`command` plus `args`, no shell) and point at Grimoire's own binaries. | Reviewed binaries only |
| F17 | A shared file's permissions are widened | A replaced file keeps its Unix mode bits, which are applied to the temporary file before any content is written. A `<path>.new` sidecar takes the original's mode. Windows ACLs and Unix ownership are not copied. | |
| F18 | A tampered lock points Grimoire at an unrelated file (`.git/HEAD`, `.env`), to print it in a plan diff or delete it | Every lock path must be a location an adapter writes: `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `AGENTS.md`, `CLAUDE.md`, `.mcp.json`, `.claude/settings.json`. Grimoire's own outputs may only be `CLAUDE.md`. | Exit 3, before reading |
| F19 | `--force` or `--write-new` destroys work in progress | A `<path>.new` that differs from what Grimoire would write is never replaced (exit 4, nothing written). An edited hook element is a conflict, never silently forgotten. | |

## 4. Known gaps and residual risks

- **The final step on Windows.** cap-std performs the final rename and remove path-based on Windows, after resolving the parents inside the sandbox (ADR-0006).
- **Leftover temporary files.** After a crash, a temporary file with a UUID name can be left in a managed directory. Grimoire does not clean these up automatically.
- **Interrupts.** There is no signal handler. Atomic writes keep files consistent, and rerunning converges.
- **Empty directories.** After deleting a file it owns, Grimoire removes every directory above it that is now empty, including one the user created empty. An empty directory holds no content, and git does not track one.
- **Concurrent runs.** Two `grimoire` processes in the same project at once are not locked against each other.
- **Environment variable names.** Consent covers the `env` permission, not each variable name. A new version that reads more variables is not stopped. This is planned with the first MCP server asset (Phase 3).
- **A tampered lock within Grimoire's locations.** Someone who can edit `grimoire.lock` could record the hash of a file Grimoire manages, so that a later removal deletes it. F18 keeps this to Grimoire's own locations. Treat lock diffs like code in review.
- **JSON layout.** Adding an entry to a container written on one line reflows it across lines. The values are unchanged.
- **Claude Code behaviour that is not documented** (`specs.md`):
  - whether an `@AGENTS.md` import placed between two HTML-comment marker lines is honoured;
  - what versions before 2.1.139 do with exec-form `args`.

## 5. Reporting a vulnerability

Private vulnerability reporting will be enabled before the first release (`PLAN.md` §6). Until then, contact the maintainer privately; do not open a public issue.
