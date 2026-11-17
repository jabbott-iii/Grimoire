# Grimoire security model

- **Status:** second version (Phase 3).
  - §1–§3 cover the installer's filesystem threat model (Phase 2).
  - §4–§6 cover MCP servers, tool hooks, and content that reaches the model, including prompt injection (Phase 3).
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
| F14 | An asset update silently gains permissions | Declared permissions must be accepted in `grimoire.toml`. Each environment variable is its own grant (`env:<NAME>`), so a version that reads one more variable needs consent for that name. A new version that needs more stops resolution until the user accepts (exit 3). | No silent escalation |
| F15 | Installing for one client grants something to another client | `plan` and `apply` warn when Cursor also loads `.claude/agents/` (it has no per-tool allow-list), when Cursor and Copilot also run `.claude/settings.json` hooks, and when Copilot also starts `.mcp.json` servers (`specs.md`). | Visible, not prevented |
| F16 | Hooks run without a trust prompt | Claude Code runs project hooks without a trust dialog under `-p` and the SDK (`CC-SET`). Hooks can therefore come only from tool assets with an explicit `hooks` grant. They are written in exec form (`command` plus `args`, no shell) and point at Grimoire's own binaries. | Reviewed binaries only |
| F17 | A shared file's permissions are widened | A replaced file keeps its Unix mode bits, which are applied to the temporary file before any content is written. A `<path>.new` sidecar takes the original's mode. Windows ACLs and Unix ownership are not copied. | |
| F18 | A tampered lock points Grimoire at an unrelated file (`.git/HEAD`, `.env`), to print it in a plan diff or delete it | Every lock path must be a location an adapter writes: `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `AGENTS.md`, `CLAUDE.md`, `.mcp.json`, `.claude/settings.json`. Grimoire's own outputs may only be `CLAUDE.md`. | Exit 3, before reading |
| F19 | `--force` or `--write-new` destroys work in progress | A `<path>.new` that differs from what Grimoire would write is never replaced (exit 4, nothing written). An edited hook element is a conflict, never silently forgotten. | |

## 4. MCP servers

An MCP server asset is a separate Rust process that a client starts (stdio) or connects to (Streamable HTTP). Grimoire only writes the client entry; it never runs the server. Every server serves through `crates/agent-grimoire-mcp`, which catalog validation requires ([ADR-0007](adr/0007-mcp-server-authorization.md)).

| # | Threat | Mitigation | Result |
|---|---|---|---|
| M1 | A web page reaches a local HTTP server through DNS rebinding or a cross-origin request | `Host` must be one the server answers to, and a present `Origin` must be on an allowlist that is empty by default. Both are checked before anything else, including authorization, and rmcp checks them again. | 403 |
| M2 | An HTTP server is reachable from the network without authorization | Without OAuth settings the server listens only on a loopback address. `project-docs`, and every server made from the `cargo xtask new` template, needs an explicit `--no-auth` to serve HTTP without authorization. | Refuses to start |
| M3 | Another local process or user connects to a loopback server | Not prevented: loopback "is not an authentication boundary" (`MCP-P3`). Grimoire configures stdio by default; local HTTP is something the user starts. | Documented |
| M4 | A token issued for another service is accepted (token passthrough, confused deputy) | RFC 9068 checks: `typ`, `iss`, `aud`, `exp`/`nbf` and the required scopes. Tokens are never forwarded. | 401 / 403 |
| M5 | A forged token through algorithm confusion, a weak key, or a key named by the token itself | Only RS256 and ES256, each bound to its key type and the key's own `alg`; RSA keys of 2048 to 8192 bits; keys only from the operator's JWKS file, never from `jku`, `x5u`, `jwk` or `x5c` headers. | 401 |
| M6 | A token leaks | Read only from the `Authorization` header; a token in the query string is ignored. Never logged or stored. A `url` in `grimoire.toml` cannot carry user information, a query or a fragment (`ServerUrl`). | |
| M7 | A tool argument reaches files outside the documentation: `..`, absolute paths, symlinks, `.env` | `project-docs` reads through a capability handle on the root and opens every component no-follow. It never serves hidden names, dependency or build directories, or non-documentation files. A path refused for its form is told why, which depends only on the string it sent. A path refused because of what is on disk (a symbolic link, a directory, a missing file) reads exactly like a missing one. | Not served |
| M8 | Resource exhaustion | HTTP: 1 MiB request bodies, one request per connection, 64 connections, 10 s to send headers and 60 s per request, and every connection closed after 75 s, so a client that stops reading cannot keep its slot. Tool work: 8 at once and about 20 per second; after 10 s the caller gets an error, but the work keeps its slot until it ends. Reads: 1 MiB per document, 30,000 characters per answer, 5,000 documents and 64 MiB per search. | Refused or cut short |
| M9 | Diagnostics corrupt the stdio protocol | Servers log to stderr only. | |
| M10 | The server serves the wrong directory | The root comes only from `--root` or `CLAUDE_PROJECT_DIR`, never from the working directory. | Refuses to start |
| M11 | A new version forwards more of the user's environment | Each variable is its own consent grant (`env:<NAME>`, F14). The client entry forwards only the variables listed in `[mcp-server] env`. | Stops until accepted |
| M12 | A new server skips the hardening | Validation requires the `agent-grimoire-mcp` dependency (`crate/hardening`), refuses direct use of rmcp's transports in `src/` (`crate/raw-transport`), and requires a test that calls `declared_tools_match` (`crate/declared-tools-test`), which compares the live tools with their declared annotations. The source checks are text searches that a determined author can get around; review is the guarantee. | CI fails |

## 5. Tool hooks

Hooks run a tool's binary automatically, with the user's privileges, on client events. They come only from tool assets with an explicit `hooks` grant, in exec form (F16).

- **Failing closed.** A binding can set `on-failure = "block"`, which `secret-guard` uses. If its binary is missing, crashes or times out, the tool call or prompt is blocked rather than let through. The cost is that a missing binary blocks the session; the documented way out is `claude --settings '{"disableAllHooks": true}'`.
- **`secret-guard` is best effort, not a boundary.** It blocks tool calls and prompts that contain credentials in documented formats. It can be bypassed:
  - by workspace trust rules, `disableAllHooks`, managed settings or a Claude Code mod;
  - by files added with `@` mentions, which no hook sees;
  - by a secret written indirectly, through a script.

  It never repeats a matched value in its output. A repository secret scanner in CI is still needed.

## 6. Content that reaches the model

Rules, skills, agents and MCP tool results become text in the model's context. Grimoire cannot make that text safe; it limits what the text can make an agent do.

- **Catalog content is reviewed code.** Rules are plain Markdown with no executable content and no permissions. A skill cannot pre-approve tools (`allowed-tools` is refused), and one that bundles scripts must declare `shell`. An agent lists its tools explicitly, and an adapter refuses a client that cannot enforce the list.
- **Prompt injection in rule and skill content.** A rule is always in context, so a malicious or careless rule acts like a standing instruction. Content changes need a version bump and review (§6.4 of `architecture.md`), and rules are linted for markers and unclosed fences that could change how a user's own `AGENTS.md` is read.
- **Prompt injection through data.** Documents served by `project-docs`, and files read by `security-reviewer`, are untrusted. Each says so in its instructions: the server tells the client to treat document contents as information, and the agent reports text that tries to steer it. Both are read-only, and the server makes no network connection, which limits what an injected instruction can achieve. It is not prevented.
- **Secrets in context.** `secret-guard` keeps detected credentials out of prompts and tool calls. A secret the model has already produced is in the conversation and the local transcript anyway.

## 7. Known gaps and residual risks

- **The final step on Windows.** cap-std performs the final rename and remove path-based on Windows, after resolving the parents inside the sandbox (ADR-0006).
- **Leftover temporary files.** After a crash, a temporary file with a UUID name can be left in a managed directory. Grimoire does not clean these up automatically.
- **Interrupts.** There is no signal handler. Atomic writes keep files consistent, and rerunning converges.
- **Empty directories.** After deleting a file it owns, Grimoire removes every directory above it that is now empty, including one the user created empty. An empty directory holds no content, and git does not track one.
- **Concurrent runs.** Two `grimoire` processes in the same project at once are not locked against each other.
- **A tampered lock within Grimoire's locations.** Someone who can edit `grimoire.lock` could record the hash of a file Grimoire manages, so that a later removal deletes it. F18 keeps this to Grimoire's own locations. Treat lock diffs like code in review.
- **JSON layout.** Adding an entry to a container written on one line reflows it across lines. The values are unchanged.
- **Claude Code behaviour that is not documented** (`specs.md`):
  - whether an `@AGENTS.md` import placed between two HTML-comment marker lines is honoured;
  - what versions before 2.1.139 do with exec-form `args`, and before 2.1.295 with `onFailure`.
- **MCP over HTTP, not yet tested end to end** (`CC-P3`, UNVERIFIED):
  - Claude Code's OAuth flow against a Grimoire server, with a real authorization server;
  - whether Claude Code sends an `Origin` header;
  - whether it sends the RFC 8707 `resource` parameter.
- **Key rotation is manual.** The JWKS file must be replaced when the authorization server rotates its keys.
- **Revocation is per key, not per token.** Removing a key from the JWKS file, or deleting the file, stops its tokens within about 5 seconds. A single token cannot be revoked before it expires. A file that cannot be read or parsed keeps the previous keys.
- **No browser clients.** There is no CORS support.

## 8. Reporting a vulnerability

Private vulnerability reporting will be enabled before the first release (`PLAN.md` §6). Until then, contact the maintainer privately; do not open a public issue.
