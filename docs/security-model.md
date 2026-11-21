# Grimoire security model

- **Status:** final version (Phase 7). Phases are the stages of the project plan in [`PLAN.md`](../PLAN.md). Phases 2 to 6 wrote this document a part at a time: the installer (§1–§3), Model Context Protocol (MCP) servers, hooks and model-facing content (§4–§6), the other four clients and the plugin marketplace (§6a), releases (§6b) and the container image (§6c). Phase 7 checked every statement again against the code, the workflows (the GitHub Actions automation in `.github/workflows/`) and the other docs. It also completed the list of cross-client leaks (§3a), which [ADR-0005](adr/0005-client-priority.md), the architecture decision record (ADR) that chose the six supported clients and their order, had promised for Phase 7. And it added the repository and contribution process (§6d).
- **Date:** 2026-11-17, revised after the Phase 7 review on 2026-11-18 (the effect of the ruleset, the repository's branch rules on GitHub, on release-plz, the tool that opens each release's pull request, and on merges: §6b and §6d), and again on 2026-11-18, when the image's signing moved into `sign-image.yml` (§6b and §6c)
- **No release exists yet.** Where a section describes a release, a registry or GitHub's behavior, it says what has and hasn't run.

This is Grimoire's threat model: what could go wrong, what Grimoire does about it, and what risk remains. Some conventions:
- Threats have IDs: F for the installer and the files it writes (§3), M for MCP servers (§4) and I for the container image (§6c).
- A § number points to a section of this document, unless a file is named, as in "`release.md` §2 A".
- IDs such as `CC-SET` or SIGN-P6 name entries in [`specs.md`](specs.md), the register of external formats and facts, each checked against its primary source. A suffix such as `-P4` gives the phase that checked it, so `*-P4` means all of Phase 4's entries.
- ADR-NNNN names an architecture decision record in [`adr/`](adr/).
- UNVERIFIED marks a statement that couldn't be confirmed from its primary source.

## 1. What Grimoire does on a user's machine

Grimoire installs assets for clients. Assets come in five kinds (`architecture.md` §3):
- a **rule** is a Markdown instruction document that a client puts into the model's context (the text the model reads before it acts) without being invoked, always or when matching files are in play;
- a **skill** is a folder of instructions, and optionally scripts, that the model loads when it needs it;
- an **agent** is a subagent definition: a delegated specialist with its own prompt and an explicit list of the tools it may use;
- a **tool** is a Rust program that an agent or a person runs, or that a client runs as a hook (§5);
- an **MCP server** is a program that offers the model more tools over the Model Context Protocol (§4).

Clients are the AI coding tools that read these assets: Claude Code, Codex, Cursor, Gemini CLI and GitHub Copilot, plus the portable `agents-md` target, which writes `AGENTS.md` and `.agents/skills/`.

`grimoire`, the command-line interface (CLI), reads a catalog of assets that is embedded in its own binary. [ADR-0001](adr/0001-catalog-delivery.md) chose that so the install path needs no network code, and so the catalog is covered by the same checksums and signatures as the binary. Every release signs the binary (§6b).

It writes files inside one directory, the **project root** (the directory that holds `grimoire.toml`). It keeps a record of everything it wrote in the lock file, `grimoire.lock`. Nothing else.

- **No execution.** Installing never executes asset content, and runs no install hooks (scripts that some package managers run while installing). It never runs the tool or MCP server binaries it configures. `doctor` and the `plan/hook-binary-missing` warning only check whether a file of that name is on `PATH`.
- **No network.** The install path contains no network code.
- **No secrets.** Grimoire never stores secret values. MCP server configuration refers to environment variables by name (`${VAR}`).

## 2. Trust boundaries

A trust boundary is where input from a source Grimoire can't fully trust reaches code that acts on it. This table says how far Grimoire trusts each input, and why.

| Input | Trusted? | Why |
|---|---|---|
| A catalog change in a pull request | Untrusted until merged | `cargo xtask validate` (one of the repository's own check commands, run through `cargo xtask`) checks its form, not its intent. Review decides, and with one maintainer that is usually one person's review (§6d). |
| The embedded catalog | Yes, as part of the binary | It is validated in continuous integration (CI, the automated checks GitHub runs on each change) before packing (`cargo xtask pack-catalog --check`, not yet a required check: §6d). It is compared with a fresh pack of `catalog/` before a release (`cargo xtask release plan`). It is parsed and validated again whenever the binary loads it. And it is covered by the binary's checksums, signature and provenance, a signed record of how the binary was built (§6b). |
| `grimoire.toml`, `grimoire.lock` | Untrusted input | Parsed strictly: unknown fields are errors, and every path is a `RelPath` (a checked relative path, F1). The lock is also validated for consistent ownership: for example, no whole file has two owners, and no entry is owned twice. Both files are part of the repository and are reviewed like code. |
| Every other file in the project | Hostile | Grimoire treats it as if an attacker wrote it. It may hold symbolic links (F3), special files (F5), broken block markers (F9), invalid JSON, or edits to managed content (content Grimoire wrote and tracks in the lock). |
| The filesystem tree itself | Hostile and changing | It may change between Grimoire's checks and its writes (F4). |

## 3. Threats and mitigations

The exit codes in the Result column are the CLI's own, 0 to 6 ([`cli.md`](cli.md#exit-codes), `crates/agent-grimoire/src/out.rs`). The ones used here: 3 is an invalid configuration or lock, or a resolution failure; 4 is a conflict; 6 is a refusal for safety.

| # | Threat | Mitigation | Result |
|---|---|---|---|
| F1 | A destination or lock path escapes the root: `..`, an absolute path, a drive prefix (`C:`) or UNC prefix (`\\server\share`, a Universal Naming Convention path to a Windows network share), or `:` (which names an alternate data stream, a hidden second stream of data inside a Windows file) | Every path is a `RelPath`, Grimoire's type for a checked path. It is ASCII, relative and `/`-separated, with no `.` or `..` components, no `<>:"\|?*`, no control characters, and no trailing `.` or space. ASCII alone rules out invisible and bidirectional-text characters, device names written with superscript digits (Windows treats `COM¹` as `COM1`), and names that collide after Unicode normalization. Asset destinations are anchors, such as `rules:` or `skills:` plus a relative name, which each adapter (the code for one client) turns into a real path. So an asset cannot name a literal path. | Rejected while parsing (exit 3) |
| F2 | Windows device names (`CON`, `aux.md`, `COM1`, …), which Windows treats as devices, not files, in every folder | Rejected by `RelPath` in every component, with or without an extension. | Exit 3 |
| F3 | A symlink (symbolic link: a file that points to another path) at or above a managed path (a path Grimoire writes and tracks), inside or outside the root | Such a link is refused even when it points inside the project, because writing through it would put content where the user didn't expect ([ADR-0006](adr/0006-filesystem-confinement.md)). Each directory is opened with `open_dir_nofollow` (an open that refuses a symlink), one component at a time from the root, after a `symlink_metadata` check (which describes a link itself, not its target). Files are opened no-follow and non-blocking, and the opened handle is checked again. `grimoire.toml`, `grimoire.lock` and `--write-new` sidecars (the `<path>.new` files of F7) are read the same way, and sidecars are checked before anything is written. The checks for client files (`init`, `doctor`) and for `AGENTS.override.md` walk the same way, but count a linked folder on the way as absent rather than failing. `uninstall --purge` checks `grimoire.toml` before it removes anything. | Refused (exit 6). Nothing is written. An existence check refuses nothing: a name under a linked folder counts as absent, so nothing outside the root is detected. |
| F4 | A race that swaps a directory for a symlink between check and use (a time-of-check to time-of-use, or TOCTOU, race) | All access goes through a cap-std handle on the root ([ADR-0006](adr/0006-filesystem-confinement.md)). cap-std is a Rust library whose directory handle reaches only paths beneath it, even if the tree changes during the operation. ADR-0006 chose it because checking each component and then writing by path leaves this race open. It uses `openat2(RESOLVE_BENEATH)` on Linux (a system call that refuses to resolve a path outside the directory), and a handle-relative resolver elsewhere, which opens one component at a time. | The path can't leave the root. The swapped symlink is refused at that component. |
| F5 | Special files at a managed path: a FIFO (first in, first out: a named pipe, whose open for reading waits until a writer opens it), a device or a socket | Checked with `symlink_metadata` before opening. The opened handle is checked again, and on Unix the open is non-blocking, so a FIFO swapped in after the check can't hang Grimoire. | Exit 6 |
| F6 | An oversized file exhausts memory | Reads are capped at 8 MiB, enforced while reading. | Exit 6 |
| F7 | Managed content the user edited is overwritten | Each unit (a whole file, a block or one configuration entry, F8) is compared three ways, by hash: what is on disk, what the lock recorded, and what Grimoire now wants to write (`architecture.md` §7.3, drift and conflicts). Each of these is a **conflict**: content the user edited or deleted; unmanaged content in the way, at a destination the lock doesn't record; and a comment the user added inside Grimoire's entry in a TOML or JSONC file. (TOML is a configuration file format, and JSONC is JSON with comments.) Values are hashed without comments, so an update or removal would otherwise lose that comment. `apply` checks every precondition first, and writes nothing if there is a conflict (exit 4). Only `--force` overwrites. `--write-new` writes `<path>.new` beside the file and leaves the file alone. | Never silent |
| F8 | Removal deletes the user's content | Only owned units are removed: a whole file, a block between Grimoire's begin and end markers, or one JSON member or array element. A whole file in a managed folder is deleted with its asset. That includes a file Grimoire adopted because it already matched, or overwrote with `--force`, because adopting or overwriting makes it Grimoire's. A shared file (`AGENTS.md`, `.mcp.json` and the like) is deleted only if Grimoire created it and nothing else is left in it. The lock records that as `created-file`, which Grimoire also records when it adopts a shared file that holds only its own content. A JSON or TOML container (an object or table that holds entries) is removed only if Grimoire created it and it is empty (`created-keys`). Edited content that is no longer wanted is a conflict, not a deletion. | Only Grimoire's content is removed |
| F9 | A shared file is corrupted by an edit Grimoire can't do safely | Grimoire never guesses what it owns. It refuses the file, even with `--force`, in three cases. **Markdown blocks** have malformed, unterminated, nested or duplicate markers. **JSON** is not strict RFC 8259 (the JSON standard, which allows no comments or trailing commas; RFCs are the Internet's standards documents), or has duplicate keys on Grimoire's path; `.gemini/settings.json` may also hold comments, which Gemini CLI allows. **TOML** (`.codex/config.toml`) doesn't parse. JSON must be strict because Claude Code reads its settings as strict JSON, and a file a client can't parse shouldn't be "fixed" silently. Markers inside a fenced code block (code between two lines of backticks or tildes) are ignored, and a block is never appended inside an unclosed fence. | Exit 4, the file is untouched |
| F10 | A crash or interruption leaves a half-written file | Every file is written to a temporary file in the same directory, fsynced (flushed to the disk), then renamed over the target. The lock is written last. A rerun converges (reaches the same end state), because a unit already in its desired state is adopted rather than reported as drift (a change made outside Grimoire). | No torn files |
| F11 | A file changes between `plan` and the write | `apply` re-reads each file just before writing and aborts if it differs from what was planned. | Exit 4. Rerun. |
| F12 | Two paths that differ only in case | Rejected when planning, before any write. This covers two desired paths, and also a locked path that a new catalog version renames only in case, such as `Guide.md` to `guide.md`. On macOS and Windows, whose file systems ignore case by default, both names are one file, so deleting the old name would delete the new one. | Exit 3 |
| F13 | Line-ending conversion (`core.autocrlf`, the Git setting that can turn `\n` line endings into Windows-style `\r\n` endings, called CRLF for carriage return and line feed) is read as drift | Text hashes fold `\r\n` to `\n`, so both forms hash the same. Bytes on disk that already match are left as they are. A TOML or JSON file Grimoire doesn't change keeps its bytes, and a changed TOML file keeps its CRLF line endings and byte-order mark (an optional invisible character at the start of a file). | No false conflict |
| F14 | An asset update silently gains permissions | Declared permissions (what an asset says it needs, such as `shell` or `hooks`) must be accepted in `grimoire.toml`. Each environment variable is its own grant (`env:<NAME>`), so a version that reads one more variable needs consent for that name. A new version that needs more stops resolution (choosing the asset versions to install) until the user accepts (exit 3). | No silent escalation |
| F15 | Installing for one client grants something to another client | Every case is in §3a. Every command that plans (`plan`, `apply`, `add --apply`, `remove --apply`, `update`, `doctor`) warns when a `.claude/agents/` agent, a `.claude/settings.json` hook or a `.mcp.json` server reaches a client it wasn't installed for. In the cases §3a lists, it also warns when Claude Code or Gemini CLI reads an `AGENTS.md` rule that wasn't installed for it. Other rules and skills that another client reads are not reported. | Visible for agents, hooks and MCP servers; not prevented |
| F16 | Hooks run without a trust prompt. (A hook is a command the client runs automatically on an event, such as before each tool call.) | Claude Code runs project hooks without a trust dialog (the prompt that asks the user to trust a project's settings) in non-interactive runs (`-p`) and under the SDK, its software development kit (`CC-SET`, the Claude Code settings and hooks entry). Hooks can therefore come only from tool assets with an explicit `hooks` grant. They are written in exec form: `command` plus `args`, started directly, with no shell. The command is the bare name of a Grimoire binary (`grimoire-tool-<id>`), which the client finds on `PATH`. Limiting hooks to reviewed, tested, CI-built, signed Rust binaries removes the risk of an arbitrary shell command in a JSON file. | Reviewed binaries only |
| F17 | A shared file's permissions are widened | A replaced file keeps its Unix mode bits (its read, write and execute permissions, such as `0644`), which are applied to the temporary file before any content is written. A `<path>.new` sidecar takes the original's mode. Windows ACLs (access control lists, Windows' file permissions) and Unix ownership are not copied. | |
| F18 | A tampered lock points Grimoire at an unrelated file (`.git/HEAD`, `.env`), to print it in a plan diff or delete it | Every lock path must be a location an adapter writes: the folders `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `.cursor/rules/`, `.cursor/agents/`, `.gemini/agents/`, `.github/instructions/` and `.github/agents/`, and the files `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.mcp.json`, `.claude/settings.json`, `.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`. Each record must also claim only what Grimoire owns there: an entry in a configuration file, a block in an instruction file, and a whole file only in a managed folder. Grimoire's own outputs, which belong to no asset (the blocks that make Claude Code and Gemini CLI read `AGENTS.md`, §3a), may only be blocks in `CLAUDE.md` or `GEMINI.md`. | Exit 3, before reading |
| F19 | `--force` or `--write-new` destroys work in progress | A `<path>.new` that differs from what Grimoire would write is never replaced (exit 4, nothing written). An edited hook element (one entry in a hook array) is a conflict, never silently forgotten. | |

## 3a. Cross-client leaks

Clients read each other's files. The read matrix in `architecture.md` §8.2 lists which client reads which location; it comes from the Phase 4 client facts in `specs.md` (`*-P4`). So content installed for one client can reach another that the user didn't select for it, or didn't select at all.

This table, with the `CLAUDE.md` note below it, lists every location Grimoire writes that another of the six clients is documented as reading, and whether `plan` says so. The warning codes are in [`diagnostics.md`](diagnostics.md).

| Grimoire writes | For | Also read by | What that client gets | Warned? |
|---|---|---|---|---|
| Hooks in `.claude/settings.json` | a tool, for `claude-code` | Cursor (on by default); Copilot CLI and VS Code | The tool's binary runs on their events. Whether they pass `args` and honor `onFailure` is undocumented, so a guard may let actions through there, or block every tool call (§5). | Always: `plan/cross-client-hooks` |
| An agent in `.claude/agents/` | `claude-code` | Cursor, which has no per-tool allow-list; Copilot CLI and VS Code, where the mapping of Claude Code tool names and `disallowedTools` is undocumented | An agent that may have more tools than it declares, up to all of the parent agent's in Cursor. When the agent is installed for `cursor` too, Cursor prefers its `readonly` copy in `.cursor/agents/` (same name). When it is installed for `copilot` too, Copilot CLI prefers `.github/agents/`; which copy VS Code uses is undocumented. | Always: `plan/cross-client-agent` |
| An entry in `.mcp.json` | an MCP server, for `claude-code` or `copilot` (one shared entry) | the other of the two: Claude Code, once the user approves the server; Copilot CLI, after folder trust, and VS Code | A server it can start, with the environment variables the entry forwards | `plan/cross-client-mcp`, whenever the entry is not attributed to both |
| A block in `AGENTS.md` | a rule, for `agents-md` or `codex` | Claude Code, through Grimoire's `@AGENTS.md` block in `CLAUDE.md`, or on its own when the project has no `CLAUDE.md`; Gemini CLI, through Grimoire's block in `GEMINI.md`, or when the user's `context.fileName` names `AGENTS.md`; Codex; Cursor; Copilot | The rule's text, always in context | `plan/cross-client-rule` for Claude Code when the `CLAUDE.md` import is planned or another asset is installed for `claude-code`, and for Gemini CLI when the `GEMINI.md` import is planned. Never for Codex, Cursor or Copilot, nor for Claude Code in a project that installs nothing for it |
| A block in `GEMINI.md` | a rule, for `gemini-cli`, when no `AGENTS.md` block is written for it | Copilot CLI and Copilot cloud agent | The rule's text | No |
| A file in `.claude/rules/` | a rule, for `claude-code` | Copilot CLI; VS Code with its Claude harness | The rule's text | No |
| A skill in `.claude/skills/` | `claude-code`, and Cursor's and Copilot's copy when no `.agents/skills/` copy is written | Cursor; Copilot | The skill's instructions and every file it ships, scripts included | No |
| A skill in `.agents/skills/` | `agents-md`, `codex` or `gemini-cli`, and Cursor's and Copilot's copy otherwise | Codex; Cursor; Gemini CLI, in trusted folders; Copilot | The same | No |

- **When a client is selected too, it's a duplicate, not a leak.** Then `plan/duplicate-rule` and `plan/duplicate-skill` report a client that may read the same rule or skill twice.
- **Consent is per asset, not per client** (F14). Accepting `shell` for a skill installed for Claude Code also covers the copy Cursor and Copilot read.
- **No other location is shared.**
  - No other of the six clients is documented as reading `.cursor/rules/`, `.cursor/agents/`, `.cursor/mcp.json`, `.gemini/agents/`, `.gemini/settings.json`, `.github/instructions/`, `.github/agents/` or `.codex/config.toml`.
  - Codex and Gemini CLI aren't documented as reading any `.claude/` location, `CLAUDE.md` or `.mcp.json`.
  - Cursor and Copilot read Grimoire's `@AGENTS.md` block in `CLAUDE.md` too, but it gives them at most `AGENTS.md` again (whether Cursor expands the import is undocumented), which the `AGENTS.md` row covers. When either client is selected, `plan/duplicate-rule` reports the second read.
- **What isn't warned stays a risk.** Rules carry no permissions, and skills can't pre-approve tools (let them run without asking the user, §6). So what an unwarned rule or skill gives another client is text the model reads, and scripts the agent can run with whatever tools that client allows. That is the prompt-injection surface of §6, in a client the user may not have chosen. (Prompt injection is text that the model reads and that is written to make the agent do something the user didn't ask for.) Reviewing catalog content (§6d) is the control.

## 4. MCP servers

An MCP server asset is a separate Rust process. A client either starts it and talks to it over standard input and output (the stdio transport), or connects to it over HTTP (MCP's Streamable HTTP transport). Grimoire only writes the client's entry for it; it never runs the server.

Every server serves through the shared hardening crate `crates/agent-grimoire-mcp` (a crate is a Rust package), and catalog validation requires it. [ADR-0007](adr/0007-mcp-server-authorization.md) put server-side authorization and HTTP hardening in that one crate, because rmcp, the Rust MCP library the servers are built on, has no server-side authorization and its HTTP defaults are unsafe for Grimoire.

| # | Threat | Mitigation | Result |
|---|---|---|---|
| M1 | A web page reaches a local HTTP server through DNS rebinding (the page's domain name is switched to resolve to a local address, so the browser treats the local server as part of the page's own site) or a cross-origin request (one a page sends to another site) | The `Host` header (the server name the request was sent to) must be one the server answers to. A present `Origin` header (which a browser sets to the site that sent the request) must be on an allowlist that is empty by default. Both are checked before anything else, including authorization, so that a probe learns as little as possible (ADR-0007). rmcp checks them again. | 403 |
| M2 | An HTTP server is reachable from the network without authorization | Without OAuth settings (OAuth is the standard for issuing access tokens; these settings make the server check them), the server listens only on a loopback address (such as `127.0.0.1`, reachable only from the same machine). `project-docs` (the catalog's documentation server) and every server made from the `cargo xtask new` template need an explicit `--no-auth` to serve HTTP without authorization. | Refuses to start |
| M3 | Another local process or user connects to a loopback server | Not prevented: loopback "is not an authentication boundary" (MCP's own security guidance, `MCP-P3`), because any process on the machine can connect to it. Grimoire configures stdio by default; local HTTP is something the user starts. | Documented |
| M4 | A token issued for another service is accepted: token passthrough (a server accepts, or passes on, a token that wasn't issued for it), or a confused deputy (a trusted program tricked into using its authority for someone else) | The checks of RFC 9068, the standard for JSON Web Token (JWT) access tokens: the token type (`typ`), the issuer (`iss`), the audience it was issued for (`aud`), the validity window (`exp`/`nbf`) and the required scopes (what the token allows). Tokens are never forwarded. | 401 / 403 |
| M5 | A forged token through algorithm confusion (a token that names another signing algorithm than its key was made for), a weak key, or a key named by the token itself | Only RS256 (RSA) and ES256 (elliptic-curve) signatures, each bound to its key type and the key's own `alg`. RSA keys of 2048 to 8192 bits. Keys come only from the operator's JWKS file (a JSON Web Key Set: the authorization server's public keys, which the person running the server supplies), never from the token's `jku`, `x5u`, `jwk` or `x5c` headers, which point to or carry a key. | 401 |
| M6 | A token leaks | Read only from the `Authorization` header; a token in the query string (the `?…` part of a URL) is ignored. Never logged or stored. A `url` in `grimoire.toml` cannot carry user information (`user:password@`), a query or a fragment (`#…`); the `ServerUrl` type checks this. | |
| M7 | A tool argument reaches files outside the documentation: `..`, absolute paths, symlinks, `.env` | `project-docs` reads through a capability handle on the root (a handle that reaches only paths beneath it, as in F4) and opens every component no-follow. It never serves hidden names, dependency or build directories, or non-documentation files. When a path is refused for its form, the caller is told why, and the answer depends only on the string it sent. A path refused because of what is on disk (a symbolic link, a directory, a missing file) reads exactly like a missing one, so a caller can't probe for files it may not read. | Not served |
| M8 | Resource exhaustion: a client uses up memory, connections or time, so others can't be served | HTTP: 1 MiB request bodies, one request per connection, 64 connections, 10 s to send headers and 60 s per request, and every connection closed after 75 s, so a client that stops reading cannot keep its slot. Tool work: 8 at once and about 20 per second. After 10 s the caller gets an error, but the work keeps its slot until it ends. Reads: 1 MiB per document, 30,000 characters per answer, 5,000 documents and 64 MiB per search. | Refused or cut short |
| M9 | Diagnostics (log output) corrupt the stdio protocol, whose messages travel on standard output | Servers log to standard error (stderr) only. | |
| M10 | The server serves the wrong directory | The root comes only from `--root` or the `CLAUDE_PROJECT_DIR` environment variable, never from the working directory. | Refuses to start |
| M11 | A new version forwards more of the user's environment | Each variable is its own consent grant (`env:<NAME>`, F14). Every client entry names only the variables listed in `[mcp-server] env`. Codex and Copilot CLI start servers with almost nothing else from the environment. Gemini CLI hides variables whose names look secret unless they are listed. Whether Cursor passes its whole environment to a server is undocumented (`*-P4`, the Phase 4 client facts). | Stops until accepted |
| M12 | A new server skips the hardening | Validation requires the `agent-grimoire-mcp` dependency (`crate/hardening`), refuses direct use of rmcp's transports in `src/` (`crate/raw-transport`), and requires a test that calls `declared_tools_match` (`crate/declared-tools-test`). That test compares the tools the running server lists with their declared annotations (hints such as whether a tool is read-only). The source checks are text searches that a determined author can get around; review is the guarantee. | CI fails |

## 5. Tool hooks

Hooks run a tool's binary automatically, with the user's privileges, on client events, such as a submitted prompt or a tool call. They come only from tool assets with an explicit `hooks` grant, in exec form (F16).

- **Claude Code only** ([ADR-0008](adr/0008-phase-4-client-adapters.md), which decided what the Codex, Cursor, Gemini CLI and Copilot adapters write). The `codex`, `cursor`, `gemini-cli` and `copilot` clients have no `hooks` anchor. In each of them a failed hook can let the action through:
  - in Codex, always;
  - in Gemini CLI, on a timeout, a crash or exit 1 (whether a missing binary there is a block is unverified);
  - in Cursor, unless `failClosed` is set;
  - in Copilot, on a timeout.

  Three of them run hooks only through a shell. So a guard written for them would look like protection it can't give.
- **Cursor and Copilot run Claude Code's hooks anyway** (§3a). Both read `.claude/settings.json`, and whether they pass `args` and honor `onFailure` is undocumented. So `secret-guard`, the catalog's tool that keeps credentials out of tool calls and prompts, can fail there in two ways:
  - a failing `secret-guard` hook may let an action through;
  - or, if `args` is dropped, `secret-guard` exits 2 with a usage error, which both clients read as a block, so every tool call is blocked. Turn off Claude Code hooks in that client to recover.

  `plan` warns (`plan/cross-client-hooks`). Don't count on `secret-guard` in those clients.
- **Other clients are told.** A project that also selects Codex, Cursor, Gemini CLI or Copilot gets `resolve/client-not-covered` for each hook tool it can't install for them.

- **Failing closed.** A guard that fails closed blocks when it can't run, rather than letting the action through. A binding can set `on-failure = "block"`, which `secret-guard` uses. If its binary is missing, crashes or times out, the tool call or prompt is blocked rather than let through. The cost is that a missing binary blocks the session; the documented way out is `claude --settings '{"disableAllHooks": true}'`.
- **`secret-guard` is best effort, not a boundary.** It blocks tool calls and prompts that contain credentials in documented formats. It can be bypassed:
  - by workspace trust rules, `disableAllHooks`, managed settings (settings an administrator deploys, which take precedence over the user's) or a Claude Code mod, which can approve a call a hook blocked;
  - by files added with `@` mentions, which no hook sees;
  - by a secret written indirectly, through a script.

  It never repeats a matched value in its output. A repository secret scanner in CI is still needed.

## 6. Content that reaches the model

Rules, skills, agents and MCP tool results become text in the model's context. Grimoire cannot make that text safe; it limits what the text can make an agent do.

- **Catalog content is validated and reviewed** (§6d says what review looks for, and who does it).
  - Rules are plain Markdown with no executable content and no permissions.
  - A skill cannot pre-approve tools: `allowed-tools` is refused, because it would pre-approve tools outside Grimoire's permission consent. A skill that bundles scripts must declare `shell`.
  - An agent lists its tools explicitly, because several clients grant every tool when the list is missing. Grimoire writes no agent for a client that ignores the list altogether (Codex): an asset can't declare one (`manifest/anchor-client`), and a project that selects Codex gets `resolve/client-not-covered`. `plan` warns where a client limits the list only partly (below).
- **What each client enforces for an agent** (ADR-0008, the record of what the four Phase 4 adapters write):

  | Client | Enforcement | Gap |
  |---|---|---|
  | Claude Code | the `tools` list | – |
  | Gemini CLI | the `tools` list | it asks the user to acknowledge each project agent, again after every change |
  | Copilot | the `tools` list with `strict-tools-list: true` | VS Code doesn't pass `strict-tools-list` on, so it may add default tools (`adapter/agent-strict-tools`) |
  | Cursor | `readonly: true`, only for an agent with exactly `read` and `search` | only edits and state-changing shell commands are blocked, so shell commands Cursor judges harmless stay available, and its effect on MCP tools and web fetches is undocumented (`adapter/agent-readonly`) |
  | Codex | none | not supported: an agent inherits the parent's sandbox, MCP servers and web search |
- **Prompt injection in rule and skill content.** A rule is in context automatically, always or whenever matching files are in play, so a malicious or careless rule acts like a standing instruction, also in clients it wasn't installed for (§3a).
  - Content changes need a version bump and review (`architecture.md` §6.4, catalog validation). Rules are also linted (checked automatically) for markers and unclosed fences that could change how a user's own `AGENTS.md` is read.
  - Nothing checks content for hidden text. That includes an HTML comment, which rendered Markdown hides and the model still reads; only a rule line that starts with Grimoire's reserved `<!-- grimoire:` is refused. It also includes invisible Unicode characters. Only paths must be ASCII (F1).
- **Prompt injection through data.** Documents served by `project-docs`, and files read by `security-reviewer` (the catalog's agent that reviews code for security problems), are untrusted.
  - Each says so in its instructions: the server tells the client to treat document contents as information, and the agent reports text that tries to steer it.
  - The server is read-only and makes no network connection.
  - The agent can't edit files or use the network in Claude Code and Gemini CLI, and in Cursor and Copilot it is limited as closely as each allows (above).

  This limits what an injected instruction can achieve. It is not prevented.
- **Secrets in context.** `secret-guard` keeps detected credentials out of prompts and tool calls. A secret the model has already produced is in the conversation and the local transcript anyway.

## 6a. The plugin marketplace

`cargo xtask gen-marketplace` turns skills, agents and MCP servers into Claude Code plugins (`architecture.md` §10, generated artifacts). A plugin is a package a user adds to Claude Code from a marketplace, a catalog of plugins. A plugin a user installs can run code with their privileges, because its MCP server starts on every session.

- **No hooks in plugins.** Tool assets are left out until a plugin hook is shown to fail closed, so no plugin can silently weaken a guard.
- **No binaries.** An MCP plugin names a binary on `PATH`, as the project entry does, and its README says to install it first.
- **No consent step.** Installing or updating a plugin grants whatever that version declares. F14 and M11 apply to the `grimoire` CLI only. Each plugin's README lists the declared permissions, environment variables and agent tools, and its version changes with its content, so an update is visible in `/plugin` (Claude Code's command for managing plugins).
- **Generated content.** The generator reuses the `claude-code` adapter's renderers, except that agents name MCP tools the way Claude Code names a plugin's, and per-project options (transport, variable remapping) don't apply. `--check` in CI keeps the checked-in plugins equal to what the generator produces. CI also runs `claude plugin validate --strict` on every plugin with a pinned, checksum-verified Claude Code and no secrets.
- **Double installs.** A user who installs an asset with `grimoire` and also enables its plugin gets two skills or two agents. An MCP server with the same command connects once, and per Claude Code's plugin documentation (`CC-PLUGIN-P4`) the project's definition wins, so a plugin agent's plugin-named MCP tools are then unavailable. Neither tool detects the other.

## 6b. Releases and distribution

[ADR-0009](adr/0009-release-pipeline.md) is the decision record for how a release is built, signed and published, and through which channels. [`release.md`](release.md) has the runbook (the maintainer's step-by-step instructions) and the user's verification commands. The facts behind them are the Phase 5 entries in `specs.md` (`*-P5`); the facts behind the signing identity are in SIGN-P6, which checked signing from a reusable workflow. **No release exists yet**, so none of this has run against a real release.

**The terms this section and §6c use:**
- **Workflows, jobs and runners.** GitHub Actions runs the repository's workflows. A workflow is made of jobs, and each job runs on a fresh machine, a runner. A reusable workflow is one that a job in another workflow calls (`on: workflow_call`). Each job gets a `GITHUB_TOKEN`, with the permissions the workflow grants it.
- **Environment.** A named deployment target on GitHub. It can require a reviewer's approval before a job that names it runs.
- **OIDC token.** A short-lived OpenID Connect (OIDC) token is GitHub's signed statement of which repository, workflow, ref and environment a job runs in. A job can request one only with the `id-token: write` permission. Getting a new token is often called minting it.
- **Trusted publishing.** A registry (crates.io, npm or PyPI) is configured to accept an OIDC token from one repository, workflow and environment, instead of a stored password or API token.
- **Keyless signing.** Signing through Sigstore, a public signing service, instead of with a long-lived private key. Fulcio, Sigstore's certificate authority, gives a certificate that is valid for a few minutes to whoever presents an OIDC token, and writes the token's identity into it. Rekor, Sigstore's public transparency log, keeps a public record of each keyless signature. cosign is Sigstore's signing tool, and a cosign bundle is the file that holds a signature with its certificate and log entry. "Public-good Sigstore" is the free public instance.
- **SAN and OIDs.** The SAN (Subject Alternative Name) is the certificate field that holds the signer's identity. For GitHub Actions it is the URL of the workflow file that ran, with its ref. Other fields record more about the run, each named by an object identifier (OID) under 1.3.6.1.4.1.57264. This document gives only the last two numbers, so 1.5 means 1.3.6.1.4.1.57264.1.5.
- **Attestations.** An attestation is a signed statement about a file, made here with GitHub's `actions/attest` and checked with `gh attestation verify` (gh is GitHub's command-line tool). Provenance is an attestation of how and where the file was built, in the SLSA format (Supply-chain Levels for Software Artifacts). An SBOM attestation carries a software bill of materials (SBOM), the list of components inside the file, here in the CycloneDX format.
- **Digests and tags.** A digest is the SHA-256 hash that identifies a file or an image exactly. A tag is a name, such as `v1` or `latest`, that its owner can move to other content. So a digest pin fixes the bytes, and a tag pin doesn't.

**The trust chain.** Each link is checked by the next:
1. **Source.** A commit on `main`.
   - The ruleset requires a pull request with one approval and a review by a code owner (an owner named in `.github/CODEOWNERS`). But the administrator role and some GitHub Apps (integrations installed on the repository) can bypass it (§6d).
   - **The release pull request doesn't exist yet.** release-plz opens it: the pull request bumps the version and adds the version's section to `CHANGELOG.md`, and merging it starts the release. The ruleset covers every branch, so it refused release-plz's commit to its `release-plz-*` branch (run 44198210657). The release pull request depends on the maintainer changing the ruleset (`release.md` §2 A, the repository settings checklist).
   - The CI and Security workflows and `release checks` (the release workflow's summary check) run on the release pull request, from runs the maintainer approves. But no ruleset requires them yet (`release.md` §2 A). The release run doesn't wait for CI or Security on its commit either. What holds a release back is the maintainer, who approves the release pull request's runs and reviews them before merging (`release.md` §1, steps 3 and 4).
   - Release mode (a run that signs and publishes, unlike a dry-run, which builds and checks but publishes nothing) also needs the version's section in `CHANGELOG.md`, which only the merged release pull request adds.
   - That run, and the release pull request's dry-run, fail while `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md` still has an opening `<!-- before-first-release: … -->` or closing `<!-- /before-first-release -->` marker line. These markers wrap text that must change before the first release. They are matched loosely (in any case, and with any spacing around `<!--`; `xtask/src/release/plan.rs`).
   - A release-mode run also fails while `crates/agent-grimoire-catalog/catalog.pack` isn't what `cargo xtask pack-catalog` makes from `catalog/`. The CLI embeds that file, while the asset binaries are built from `catalog/`, so a stale pack would ship an older catalog beside newer binaries. It also fails while the pack holds an asset whose `min-cli-version` is above the version, which that version of `grimoire` would refuse (`release.md` §1 step 5).
2. **Build.** GitHub-hosted runners, one native runner per target (a target is a platform the binary is built for, such as `x86_64-unknown-linux-musl`). Native builds need no cross-compiling toolchain, and the smoke test (a quick `--version` run) runs on the real operating system (ADR-0009). The build step has:
   - the pinned toolchain, `cargo --locked xtask`, and one `cargo auditable build --locked` per package (`--locked` builds with exactly the versions in `Cargo.lock`, and `cargo auditable` embeds the dependency list in the binary). So each binary has only the dependency features its own manifest asks for, and its SBOM describes it;
   - remapped build paths (the build machine's directory names are replaced in the binary);
   - no caches, so there is no cache to poison (to fill with tampered files that a later build would use);
   - artifacts (files one job uploads for later jobs) that move only within the run, and download-artifact checks their digests.
3. **Checksums.** `SHA256SUMS` lists every release file except itself and the cosign bundles.
   - `cargo xtask release verify` checks that every wheel (a Python package file for PyPI, the Python Package Index) and every npm platform package holds the same binary as its CLI archive. (npm is the JavaScript package registry; each platform package carries one platform's binary.)
   - Every release-mode job downloads the files by artifact ID. Before it attests, signs or publishes, it checks `SHA256SUMS` against the hash the package job recorded, and checks that the files are exactly the ones `SHA256SUMS` lists. So an artifact of the same name uploaded by another job is never what gets signed.
4. **Attestations.** SLSA provenance for every file except the cosign bundles, and a CycloneDX SBOM attestation for every archive, through `actions/attest` and public-good Sigstore, made in `sign.yml` (below).
5. **Signatures.** A keyless cosign bundle for every other file, `SHA256SUMS` included, under the identity `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`, called from `jabbott-iii/Grimoire` on `refs/heads/main`.
6. **First approval.** The `publish` job runs only after the maintainer approves it in the `release` environment.
   - It re-checks the checksums, every bundle and the provenance of `SHA256SUMS`, with that identity and the caller pins (the required values for the calling repository and ref, below). It also checks the provenance's Build Config URI (below).
   - Then it creates the tag and the GitHub Release.
   - It has no `id-token`, so it can't get a registry token.
7. **Immutable release, if the repository setting is on.** Once such a release is published, the tag and the assets are locked, and GitHub adds its own release attestation.
   - `GITHUB_TOKEN` can't read the setting, so `publish` reads the release's `immutable` field and only warns when it is false.
   - A release published while the setting is off stays mutable and has no release attestation. So the tap and bucket syncs (step 9) refuse it until it is deleted and published again (`release.md` §3, recovery).
8. **Second approval, then the registries.** crates.io (Rust's package registry), npm and PyPI are published through OIDC trusted publishing, with no stored token:
   - each registry has its own job and its own environment (`crates-io`, `npm`, `pypi`), approved by the maintainer; each job has only `id-token: write` (and `contents: read` for the crates checkout);
   - npm versions are staged, and go live only after the maintainer approves each one with two-factor authentication (2FA);
   - npm should attach provenance to versions staged over OIDC. npm 11.19.1 turns it on for a public repository and package, but npm's docs don't say a staged version keeps it after approval (UNVERIFIED, `NPM-P5`);
   - PyPI adds PEP 740 attestations (PEP 740, a Python Enhancement Proposal, is the Python standard for attestations on uploaded files).
9. **Tap, bucket and winget.**
   - The Homebrew tap (`jabbott-iii/homebrew-tap`, a repository of install formulas for Homebrew, the macOS and Linux package manager) and the Scoop bucket (`jabbott-iii/scoop-bucket`, a repository of manifests for Scoop, a command-line installer for Windows) take a formula or manifest only after `gh attestation verify` passes, and only through a pull request the maintainer merges.
   - The maintainer submits the winget manifests (winget is the Windows Package Manager), and winget-pkgs, Microsoft's repository of winget manifests, validates them.

**The signing identity.** SIGN-P6 holds the facts it rests on; the identity itself is the maintainer's choice after the Phase 6 review.
- **Why it moved.** A keyless certificate's identity names the workflow file that ran, not the job. So every job in `release.yml` with `id-token` used to get the same certificate identity, `…/release.yml@refs/heads/main`.
  - That included `publish-pypi`'s tag-pinned third-party container, which already gets Sigstore certificates for PEP 740 attestations. A re-pushed image of that action could have signed any file or image so that it passed the documented verification.
  - Now every signature and attestation of the release files is made in `.github/workflows/sign.yml` (`on: workflow_call`), which only `release.yml`'s `sign-files` calls.
  - The image's are made in `.github/workflows/sign-image.yml`, which only `sign-image` calls, under its own identity (§6c).
  - No release exists yet, so no release was ever signed under the old identity: every release is signed by `sign.yml`, and its image by `sign-image.yml`.
- **Why two files.** GitHub checks every job of a called workflow against the calling job's permissions when the run starts, even a job its `if:` would skip, and refuses the whole run if one asks for more. With one file for both, the call that signs the release files would have to grant the image job's `contents: read` and `packages: write`. Each call now grants exactly what its jobs ask for. An xtask test checks every call's grant (`xtask/src/workflow_permissions.rs`), which neither zizmor nor actionlint (two linters for GitHub Actions workflows) does.
- **What the certificate says.** The identity (the SAN) names the called workflow: `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` for the release files, and `…/sign-image.yml@refs/heads/main` for the image. The caller is recorded in other fields: OID 1.5 (repository) `jabbott-iii/Grimoire`, 1.6 (ref) `refs/heads/main`, and 1.18 (Build Config URI) `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`.
- **The publish jobs can't sign as the release.** `publish-crates`, `publish-npm` and `publish-pypi` still have `id-token`, so they, and the PyPI action's container, can still get certificates. Those name `…/release.yml@refs/heads/main`, which no verifier accepts any more. `publish-image` has no `id-token`.
- **External callers.** Any repository can call a public reusable workflow and get a certificate with the same identity; its own repository then appears in the caller fields (1.5 and 1.12). So:
  - every job in `sign.yml` and `sign-image.yml` runs only if `github.repository_id == '1460666977'` and `github.workflow_ref == 'jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main'`. An external caller, another workflow of this repository, or a pull request run never mints a token there;
  - every verifier also pins the caller: cosign with `--certificate-github-workflow-repository jabbott-iii/Grimoire --certificate-github-workflow-ref refs/heads/main`, gh with `--repo jabbott-iii/Grimoire --source-ref refs/heads/main` (`release.md` §5, verifying a download).
- **The caller file.** Neither cosign nor gh has a flag for 1.18 (the caller file) or 1.23 (the environment).
  - So `publish` (for `SHA256SUMS`) and `sign-image.yml` (for the image) also read `gh attestation verify --format json` with `jq` (a command-line JSON processor). They require the provenance certificate's Build Config URI to be `…/release.yml@refs/heads/main`, and its repository and owner IDs to be `1460666977` and `84321953`. They check the IDs because, with `--bundle-from-oci`, gh itself compares names only.
  - `--signer-workflow` carries `@refs/heads/main`, so a signing workflow at a release tag, which any repository can call, doesn't pass.
- **How the signing workflows are written.**
  - Each has `permissions: {}`. A calling job can't set an environment, `runs-on`, `env` or outputs; it grants permissions, which the called jobs can only reduce.
  - Inputs reach run steps only through `env`, never written into the script itself, where an input could inject shell code (zizmor's template-injection audit rates that High).
  - Their issuer, repository and ref pins equal `release.yml`'s, as do `sign-image.yml`'s pins for the image and for crane (a tool for working with container registries), and each names its own identity. An xtask test checks them (`release.md` §6).
  - The provenance `actions/attest` makes there names the signing workflow, `sign.yml` or `sign-image.yml`, as its builder (`runDetails.builder.id`), and `release.yml` as the workflow (`externalParameters.workflow.path`).

**What each channel checks on the user's machine:**

| Channel | Checks | Doesn't check |
|---|---|---|
| Direct download | What the user runs: `sha256sum -c`, `gh attestation verify`, `cosign verify-blob`, `gh release verify-asset` (`release.md` §5) | Anything the user skips |
| Install scripts | An `https://` start URL. `install.sh` uses curl with `--proto =https`, so it also refuses redirects to anything but HTTPS; PowerShell 7.4 and later refuse HTTPS-to-HTTP redirects by default. The archive's SHA-256 against the value embedded in the script, failing closed | The script itself: verify it before running it. Signatures and attestations. HTTPS on redirects under Windows PowerShell 5.1 (UNVERIFIED) |
| `cargo install` | Builds from the crates.io source; Cargo checks each crate against the registry's checksum | Grimoire's signed binaries aren't used |
| `cargo binstall` (which downloads a prebuilt binary instead of building one) | The crates.io crate checksum; HTTPS for the archive | The archive's checksum, its cosign bundle and its attestations (`RUST-REGISTRY-P5`) |
| npm | The tarball's `dist.integrity` hash (the hash the registry lists for the tarball, the `.tgz` package file). In a project that installs `agent-grimoire` locally, `npm audit signatures` checks registry signatures and any provenance (`NPM-P5`) | `npm audit signatures` refuses global installs, so it can't check `npm i -g` or `npx`. cosign bundles and GitHub attestations |
| PyPI (pip, pipx, uv) | The file hash listed by the index (not re-checked in Phase 5) | Whether these installers check PEP 740 attestations wasn't researched (UNVERIFIED) |
| Homebrew | The SHA-256 in the formula, which came from an attested release file; Homebrew's tap trust (a formula from a third-party tap must be explicitly trusted) | Signatures and attestations at install time |
| Scoop | The SHA-256 in the manifest, which came from an attested release file | Signatures and attestations at install time |
| winget | `InstallerSha256` from the manifest; winget-pkgs' validation, including antivirus scans | Code signatures: the binary isn't Authenticode-signed (Authenticode is Windows' code-signing format) |

**The release switch.**
- No run publishes until the repository variable `GRIMOIRE_RELEASES` is `enabled` (ADR-0009).
- It exists for ordering, not security. GitHub creates a missing environment without protection rules, so no job may reference `release`, `crates-io`, `npm`, `pypi` or `ghcr` before the maintainer has created and protected it.
- The variable approves nothing. The controls are the required reviewers on the five environments.
- As a second check, each registry's trusted publisher is configured for its own environment: crates.io for `crates-io`, npm for `npm`, PyPI for `pypi`. An OIDC token from any other job, including another publish job, names another environment and is refused. A crates.io configuration without an environment would accept any, so the bootstrap always sets one.

**Residual risks** (the risks that remain after the mitigations):
- **The PyPI publish action's image is pinned by tag, not digest.** The tag can be re-pushed and the base image is unpinned, so the code that uploads to PyPI can change under the same pin.
  - A container action gets the job's OIDC request token (the credential a job uses to ask GitHub for OIDC tokens) and can mint a token for any audience (the service a token is meant for). In `publish-pypi` that token names environment `pypi`, which the crates.io and npm publishers refuse, and the job holds only the wheels.
  - A Sigstore certificate it gets names `release.yml`, which no verifier accepts. So a re-pushed image can at worst publish to PyPI.
  - It also gets the run's `ACTIONS_RUNTIME_TOKEN` (a token that can upload artifacts to the run), but every job downloads artifacts by ID, so an artifact it uploads is never used.
- **One maintainer.** The same person writes, approves and publishes each release, so a compromised maintainer account defeats both approval rounds. npm's 2FA approval and immutable releases still apply, but they are the same person's controls. The same account can also change `main` without a second reviewer (§6d).
- **Signing comes before approval.** `sign-files` runs before the `release` approval. A run that is never approved still leaves Rekor entries and GitHub attestations for its files, under the `sign.yml` identity. Those files are never attached to a release. A valid signature shows only that a release run of `release.yml` on `main` signed the file; `gh release verify-asset` shows it belongs to a published release.
- **No flag pins the caller file or the environment.** cosign and gh can't check OID 1.18 or 1.23. Another workflow of this repository on `main` that called a signing workflow would differ from a release only in 1.18. The signing workflows' guards keep such a caller from minting a token, and `publish` checks 1.18 for `SHA256SUMS`, but a user's verification commands can't tell. The signing jobs have no environment, so no signature shows that anyone approved anything.
- **The signing workflows' own steps first run in a real release.** Neither runs on a pull request: their callers run only in release mode, their guards skip every other caller, and pull requests from forks (copies of the repository under other accounts) get no OIDC token. So their keyless signing, `actions/attest` and the registry checks run for the first time in a release, as with the Phase 5 `attest` jobs that `sign-files` replaced. The dry-run signs offline with a throwaway key instead.
- **npm trusted publishing may fail** (npm/cli#9969, an npm bug, open when checked on 2026-11-17, that can make trusted publishing fail with "package not found" for repositories whose OIDC tokens carry GitHub's immutable subject, as this one's do; another repository of the same kind publishes successfully, so it can work). Then the maintainer stages the signed tarballs by hand with a 2-hour session, and those versions have no npm provenance. npm users can still compare the tarball with the attested release asset.
- **Some pins protect only the download.** release-plz 0.4.3 and ShellCheck v0.11.0 publish no checksums or attestations, so their pins are GitHub asset digests (the SHA-256 GitHub records for a release file): they fix the bytes, not who built them.
- **release-plz's token can do more than open the release pull request.** It runs with the job's `GITHUB_TOKEN` (`contents: write`, `pull-requests: write`). Compromised release-plz bytes could push branches, create and delete tags, create, publish and upload to GitHub Releases (a newly published release becomes "latest" by default), and merge pull requests wherever the ruleset allows. What still protects users:
  - the job has no `id-token`, and the registries accept only tokens from their own environments;
  - the tap and bucket syncs check the provenance with `gh attestation verify`, signed by `sign.yml` with the caller pinned;
  - only cosign or `gh attestation verify` with `sign.yml`'s identity and the caller pins proves that a release run signed a file. `gh release verify-asset` alone doesn't, because GitHub attests any immutable release. Install-script users should check the script's bundle or attestation (`release.md` §4).
- **The publish jobs' shell runs only in real releases.** It reads `gh`, `curl` and `npm` answers to decide what is already published, and no pull request dry-run exercises it. [ADR-0010](adr/0010-language-policy-exceptions.md) lists this shell code among the exceptions to the project's policy that code is written in Rust, and it fails closed on answers it doesn't recognize.
- **cargo-binstall users get only HTTPS for the archive.**
- **winget aliases aren't reserved.** Another package could later claim `agent-grimoire` as its alias (the command name winget puts on `PATH`), and winget would replace the link without a cross-package check.
- **Two targets are built only in release mode** (`x86_64-apple-darwin` and `aarch64-pc-windows-msvc`), so a problem specific to them shows up first in a release run, before the approval. Since Phase 6, pull requests also build `aarch64-unknown-linux-musl`, for the two-platform image.

## 6c. The container image

[ADR-0011](adr/0011-container-image.md) is the decision record for how the image is built, checked and published. How to run and verify it is in [`container.md`](container.md), and the maintainer's steps are in [`release.md`](release.md). The facts are the Phase 6 entries in `specs.md` (`*-P6`). **No release exists yet**, so nothing has been pushed to GHCR (GitHub Container Registry) or signed there. The image was first built, checked and scanned on GitHub in a release dry-run, Release run 44353090397 on 2026-11-18. What ran locally and what that run covered are listed below.

**What the image holds.** `ghcr.io/jabbott-iii/grimoire` is one index for linux/amd64 and linux/arm64. An index, in the image format of the Open Container Initiative (OCI), points to one image manifest per platform, and each manifest lists that image's configuration and layers (the archives of files it is built from). The image starts from `gcr.io/distroless/static-debian13:nonroot`, a minimal base with no shell and no libc (the C standard library). On top, it adds only the release's static musl binaries (statically linked with musl, a small C library, so they need no libc in the image):
- the CLI at `/usr/local/bin/grimoire` (the entrypoint);
- every asset binary in `/opt/grimoire/bin/`, off `PATH`.

It runs as uid 65532 (a non-root user) unless `--user` says otherwise.

**The trust chain.** Each link is checked by the next:
1. **Base.** The `Dockerfile` pins distroless's index digest.
   - The `image` job fetches that index and requires it to be the pinned digest, byte for byte, with exactly one linux/amd64 and one linux/arm64 manifest.
   - cosign then verifies distroless's keyless signature (`keyless@distroless.iam.gserviceaccount.com`, issuer `https://accounts.google.com`) on the index and on both platform manifests.
   - The job also fetches each platform's config blob (`crane blob`), which `check` compares with the image. (crane is a tool for working with registries, and the config blob is the JSON that holds an image's settings.)
   - This runs before every build, pull requests included.
2. **Inputs.** The binaries come from the release archives, downloaded by artifact ID and checked against `SHA256SUMS` (§6b, step 3). `cargo xtask release image context` accepts only a static 64-bit ELF executable (Executable and Linkable Format, the Linux executable format) for the platform's machine.
3. **Build.** One amd64 job builds both platforms on BuildKit v0.34.2 (Docker's build engine), pinned by digest, with COPY steps only: each step copies files in, and none runs a command. It has only `contents: read` and pulls anonymously. The output is an OCI layout (an image stored as files in a directory) with an SPDX SBOM (SPDX is another SBOM format) and SLSA v1 provenance (`mode=max`, the detailed mode) for each platform.
4. **Check.** `cargo xtask release image check` requires:
   - one index with exactly the two platform manifests and one attestation manifest for each, and the exact annotations (key-value labels);
   - an image config with exactly the Dockerfile's fields (`User`, `Env`, `Entrypoint`, `Cmd`, `WorkingDir`, `Labels`, `ArgsEscaped`) and top-level keys, `Env` equal to the base config's, and `diff_ids` (the hashes of each layer's uncompressed contents) that are the base's, then one for each added layer;
   - the verified base's layers first, byte for byte, with their blobs in the layout read and checked;
   - on top, exactly the planned directories, each with its mode and owner (`usr`, `usr/local`, `usr/local/bin`, `opt`, `opt/grimoire` and `opt/grimoire/bin` root-owned 0755; `repo` 65532:65532 0755), and exactly the release binaries: mode 0755, owned by root, each with its archived binary's SHA-256.

   Layers are read as strictly as Docker and containerd (the container runtime underneath Docker) read them: one gzip member and nothing after it, uncompressed bytes equal to the config's `diff_id`, no PAX or GNU extension records (extra tar headers that can override an entry's name or size), a size of 0 for every directory, and only zeros after the tarball's end. So no reader sees a file `check` didn't. Every blob's digest and size is checked.
5. **Scan.** Grype, a vulnerability scanner, scans each platform (threat I6 below).
6. **Run.** `verify-image` pushes the layout to a local registry that, like GHCR, has no referrers API (the registry endpoint that lists the signatures and attestations attached to an image). Then it runs every binary from it on an amd64 and an arm64 runner. Its only permission is `contents: read`, to check out `image-tags.sh`.
7. **Approval, then the push.** `publish-image` runs only after the GitHub Release, and only once the maintainer approves environment `ghcr`. It pushes the same layout by digest, untagged, so GHCR gets the digest the release notes name. It has no OIDC token.
8. **Signing, then the tags.** `sign-image` calls `sign-image.yml`, which:
   - signs the index keylessly under its own identity, `…/sign-image.yml@refs/heads/main`, with the same caller pins as the release files (§6b);
   - adds a provenance attestation with `actions/attest`;
   - checks that the `sha256-<index hex>` tag lists both (without a referrers API, that tag holds the list of an image's signatures and attestations);
   - and verifies both.

   Only then does it tag `X.Y.Z` and move `latest`. So the release never gives either tag to an image before it is signed, attested and verified.
9. **Weekly rescan.** `security.yml`'s `image-scan` job finds the latest release, verifies its image's signature with the image's signing identity (`sign-image.yml`) and the caller pins, and runs the same Grype gate on every platform.

**The tag steps.** `.github/scripts/image-tags.sh` makes the registry steps: `push` (by digest, untagged), `tag` (`X.Y.Z`) and `latest`.

It is a language-policy exception: code that isn't Rust, which ADR-0010 allows only with a reason, a limited scope and checks of its own, and which ADR-0011 records. The reason is that cargo must not run in a job that can push or mint OIDC tokens, because a dependency's build script could use either, and a prebuilt xtask would put all its dependencies in the signing job. Its rules:
- a version tag that names another digest is an error, never overwritten;
- a tag lookup that fails for any reason other than the registry's `MANIFEST_UNKNOWN` or `NAME_UNKNOWN` stops the script, so a tag it couldn't read is never treated as absent;
- `latest`'s version annotation is trusted only if `latest`'s digest verifies with the image's signing identity (`sign-image.yml`). A package writer could otherwise hold `latest` by claiming a high version. `latest` never moves back to an older release, and if it claims a newer version that doesn't verify, the job stops and leaves it for the maintainer;
- it runs under `set -euo pipefail` and `inherit_errexit` (Bash options that stop the script on a failed command or an unset variable), so a failed lookup inside a command substitution still stops it. ShellCheck v0.11.0 (a shell-script linter), pinned, checks it in `security.yml`.

The release (`publish-image` and `sign-image.yml`) and every pull request dry-run run the same script. The dry-run uses `verify-image`'s local registry and the throwaway key, and covers:
- push twice, and tag twice;
- refusing a version tag that names another digest (by `push` and by `tag`);
- refusing a registry it can't read;
- moving `latest` off an unsigned image that claims no newer version;
- refusing to move it off an unsigned image that claims a newer one;
- and keeping `latest` on a newer signed release.

| # | Threat | Mitigation | Result |
|---|---|---|---|
| I1 | The base image is compromised, or another image is served under its tag | Pinned by index digest. The served index must be that digest byte for byte. cosign verifies distroless's signature on the index and on both platform manifests before every build. `check` requires the image's first layers to be exactly the verified platform manifest's layers, and reads their blobs in the layout. It also compares the image config with the base config `crane blob` fetched: its `Env`, and its `diff_ids` first. | The build stops |
| I2 | Grimoire's tags are moved | The release notes name the index digest, and the docs pin it; `X.Y.Z` and `latest` are a convenience. `publish-image` pushes by digest, untagged. `sign-image` sets the tags only after the signature and the attestation verify in GHCR. `X.Y.Z` is never overwritten: a tag that names another digest stops the job, and so does a lookup that fails for any reason other than the registry's "unknown" answer. `latest` moves last and never back to an older release. If it claims a newer version that doesn't verify with the image's signing identity, the job stops. The signature is on the index digest, and the index's `org.opencontainers.image.version` annotation, which it covers, names the version. | A pinned digest still means the same image |
| I3 | The builder, the SBOM scanner or the Dockerfile frontend (the part of BuildKit that parses the Dockerfile) is swapped | BuildKit and the SBOM scanner are pinned by digest. Unpinned, BuildKit pulls the scanner by its `stable-1` tag on every build. There is no `# syntax=` line, so the frontend is the one inside the pinned BuildKit image and nothing is pulled by tag. buildx (Docker's build command, which drives BuildKit) is installed checksum-verified into a job-scoped `DOCKER_CONFIG`, ahead of the runner's unverified copy. The job checks the buildx version and the `BuildKit version:` line the builder reports. | Fixed bytes; signatures aren't checked (below) |
| I4 | The public provenance leaks the GitHub event payload (the JSON that describes what started the run) | On GitHub Actions, buildx's `docker-container` driver adds the whole event payload and runner details to provenance by default, in min mode as well as max. The builder is created with `provenance-add-gha=false`, and `check` refuses any provenance that holds `github_event_payload`. Max mode also records every build argument and the Dockerfile; the only build argument is `SOURCE_DATE_EPOCH`. | `check` fails |
| I5 | Code runs during the build | COPY only. `context` allows only `FROM`, `ARG`, `COPY --chmod=0755 <src> <dst>`, `WORKDIR`, `USER`, `LABEL`, `ENTRYPOINT` and `CMD`, so `RUN`, `ADD`, `ONBUILD` and `COPY --from` are refused. It also refuses heredocs (inline scripts or file contents written into the Dockerfile), a `# syntax=` line, line continuations (a backslash that joins the next line), and anything but one `FROM name:tag@sha256:…`, with the digest on the same line and no flags or stage name. The first line must be `# check=error=true`, and `docker buildx build --check` (Docker's Dockerfile checks) runs before the build. Nothing executes, so neither platform needs emulation (running another processor's code in software). | `context` fails |
| I6 | A vulnerable base package or crate ships | Grype fails on any medium, high or critical match, fixed or not (the maintainer's choice). A finding without a fix needs an ignore rule with a reason in `.grype.yaml`, reviewed like `deny.toml`'s exceptions. The workflows pass that file with `-c`, so no other Grype config is read. They repeat the threshold with `--fail-on`, because Grype ignores unknown keys silently, also inside an ignore rule, where a misspelled criterion widens the rule. A unit test reads the file line by line and refuses any unknown key, and any rule without a GHSA (GitHub Security Advisory) or CVE (Common Vulnerabilities and Exposures) id, a package name and a reason. Exit 2 (a match) and any other error both fail, and every platform is scanned before the step fails. `coverage` fails when Grype detected no Debian 13 or no Debian packages, or a binary shows no Rust packages: either would be reported as clean. The weekly rescan catches advisories published after a release. cargo-deny, the Rust dependency checker (§6d), stays the primary Rust advisory gate. | The release stops before approval |
| I7 | The vulnerability database is tampered with or stale | Grype checks the downloaded database against the SHA-256 in an HTTPS listing. Neither is signed, so authenticity rests on TLS (Transport Layer Security, the encryption under HTTPS). `db.validate-age` with `max-allowed-built-age: 120h` fails a scan when the database is more than five days old. The `image` job logs `grype db status`. | Residual (below) |
| I8 | A signature or attestation is deleted from the `sha256-<index hex>` tag | Anyone with write access to the package can remove an entry. That is a denial of service, not a forgery: verification then fails, and verifiers must fail closed. Never delete `sha256-*` tags or untagged referrer manifests (the manifests that hold signatures and attestations), for example in a retention cleanup. During a release, a re-run of `sign-image` signs again when the signature doesn't verify. It attests again when `gh attestation verify --bundle-from-oci` finds no attestation in GHCR, even if GitHub's API holds one. Checking the API instead, as before the Phase 6 review, would skip an attestation that reached only the API, and the registry check would then fail on every re-run. | Verification fails |
| I9 | The push or signing job's credentials are misused | `publish-image` runs only in release mode, from `main`, after the maintainer approves `ghcr`. It pushes with the job's `GITHUB_TOKEN` (`contents: read`, `packages: write`) and has no `id-token`, so it can't sign. `sign-image.yml`'s image job runs only when `release.yml` on `main` calls it, after `publish-image`; it has `contents: read`, `packages: write`, `id-token: write` and `attestations: write`, and no environment. No token is stored. Neither job runs cargo, so no dependency's build script runs where it could push or mint a token: they check out the repository (without persisted credentials) only for `image-tags.sh`, and read JSON with `jq`. Both log out of GHCR in an `if: always()` step. | One approval, no stored token |
| I10 | The image is signed before anyone approves it | Pull request dry-runs sign only with a throwaway key and a signing config that names no services, so nothing reaches Rekor. The keyless signature and the attestation are made only in `sign-image`, which needs `publish-image`, which waits for the `ghcr` approval. Unlike the release files (§6b, "Signing comes before approval"), no unapproved image carries the image's signing identity (`sign-image.yml`). A verifier can't see that approval, though (§6b, "No flag pins the caller file or the environment"). | |
| I11 | A container gets more access than the CLI or a server needs | The documented `docker run` forms ([`container.md`](container.md)) pin the image by digest and pass `--network none` (no network), `--read-only` (a read-only root file system), `--cap-drop ALL` (no Linux capabilities, the separate powers of root) and `--security-opt no-new-privileges` (no gaining privileges, for example through a setuid program). The image runs as uid 65532 by default. The CLI form passes `--user "$(id -u):$(id -g)"` on rootful Linux (where the Docker daemon runs as root); under rootless Docker (where the daemon runs as an ordinary user), the documented `--user 0:0` is the host user, not root on the host. MCP servers get the repository mounted read-only and `--root /repo`, never `CLAUDE_PROJECT_DIR`, whose host path doesn't exist in the container (M10). `verify-image` runs every binary with these flags. | Limited to what the user mounts |
| I12 | An HTTP server in a container is reachable without authorization | To be reachable through a published port (a container port Docker forwards from the host), the server must listen on `0.0.0.0`, and the hardening crate refuses that without authorization (M2). That is right: on rootful Docker, every bridge container (one on Docker's default network) and host process can reach the container's address, whatever `-p` says. The documented form publishes with `-p 127.0.0.1:8931:8931`, which needs Docker 28.0 or later (before 28.0.0, hosts on the same L2 segment, the same local network, could reach ports published to localhost). It sets `--auth-resource` to the exact URL clients use, or the `Host` check answers 403, and mounts the JWKS directory read-only. On Linux, `--network host` with `--listen 127.0.0.1:8931 --no-auth` keeps the loopback-only rule instead; not on Docker Desktop, or on rootless Docker before 29.5. | Refuses to start without authorization |
| I13 | `secret-guard` in a container loses context | It needs the event on stdin (`-i`) and the project at its identical absolute path. Otherwise the "file already holds this secret" exemption is lost, which fails safe: it blocks more. Worktrees outside the mount and Windows hosts can't use identical paths, so the native binary is recommended for hooks. | Blocks more, never less |
| I14 | Another repository or workflow calls `sign.yml` or `sign-image.yml` | Any repository can call a public reusable workflow and get a certificate with the same identity. Every job in the two signing workflows runs only when `github.repository_id` is `1460666977` and `github.workflow_ref` is `jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, so an external caller, another workflow of this repository and a pull request run are all skipped before a token is minted. Every verifier also pins the caller's repository and ref, so another repository's certificate doesn't verify. Another workflow of this repository on `main` differs from a release only in OID 1.18, which only `publish` checks (§6b). | The jobs are skipped; another repository's certificate is refused |

- **Inside a container, `doctor` describes the container.** `doctor` and the `plan/hook-binary-missing` warning read the container's `PATH`. The asset binaries are off it, so a container never counts them as installed. The warning can then appear even when the host has the binary, which is the safe direction.

**What ran locally, and what didn't.** With rootless Docker 29.9.3 (Docker with its daemon run as an ordinary user), and Grype, cosign and crane each checked against its published SHA-256:
- three builds, one with a fresh builder: the platform manifest digests were identical each time. The index digest wasn't, because the SBOMs and provenance hold build times and IDs;
- `check` passed on a real build, and refused a changed binary and a corrupted blob. After the Phase 6 review it also refused all seven of the review's tampered layouts, including the two that had passed the earlier `check` and run another binary;
- with the Actions environment simulated, the event payload appeared in the provenance by default and not with `provenance-add-gha=false`, and `check` refused a provenance that held it;
- Grype 0.121.3 (database v6.1.10, built 2026-11-17T06:20:58Z) found nothing on either platform. A medium GHSA match exited 2; an ignore rule keyed on the GHSA ID and the package name suppressed it; unknown config keys were silently ignored;
- `coverage` counted 92 Rust packages in `grimoire` (91 on arm64), 135 in `grimoire-mcp-project-docs` and 41 in `grimoire-tool-secret-guard`, so cargo-auditable's data survives the `dist` profile. Grype detected Debian 13.7, with 6 Debian packages;
- distroless's index and both platform manifests verified with cosign 3.1.4 and the identity above. crane v0.22.1 pushed the layout to a local registry with the index digest unchanged, and a cosign signature landed in the `sha256-<index hex>` tag as a Sigstore bundle;
- on amd64, under the hardening flags: every binary's `--version`; `init`, `add --apply` and `verify` on a bind mount (a host folder mounted into the container); an MCP `initialize` over stdio with a read-only mount; and `secret-guard scan` on stdin;
- every case of `image-tags.sh` that the dry-run covers (above), against a local registry with a throwaway key, on 2026-11-17.

Not run locally: the arm64 image (it was built and scanned, but there is no emulation here), rootful Docker, Docker Desktop, Podman and GHCR. On GitHub, the first Release dry-run (Release run 44353090397, 2026-11-18) ran `image`, and `verify-image` on both platforms, with the offline signing and the tag script's cases. `publish-image`, `sign-image` (with `sign-image.yml`) and `image-scan` have never run, and neither has keyless signing or `actions/attest` for the image. `sign-image.yml`'s steps first run in a real release (§6b), and `image-tags.sh` has run only against a local registry, never against GHCR.

**Residual risks:**
- **The base's age is checked by hand.** Dependabot (GitHub's dependency-update bot) has a cooldown, a waiting period before it proposes a new version. The cooldown doesn't hold back gcr.io digests: Dependabot proposes them at once, with a `cooldown_date_unavailable` warning. The maintainer checks the upload time before merging (`release.md` §6, maintaining the image pins). The `timeUploadedMs` field of Google Container Registry (GCR) is undocumented, and there is no CI check of the age yet.
- **Docker's images are pinned, not verified.** BuildKit and the SBOM scanner carry keyless signatures on their attestation manifests, but nothing checks them, so the digest pins fix the bytes, not who built them. The BuildKit container always runs privileged (with almost all of the host's privileges). Whether the runners' Docker 28.0.4 pulls a `name:tag@sha256:…` reference strictly by digest is UNVERIFIED; the builder step checks only the BuildKit version it reports.
- **The SBOM and provenance are BuildKit's own statements.** `check` checks their predicate types (the kind of statement each is), their subjects (what each is about) and the absence of the event payload, not the rest of their contents.
- **The attached SBOM comes from an older syft** (an SBOM generator). The scanner bundles syft v1.51.0; Grype 0.121.3 embeds v1.57.1. So the SBOM and the gate can disagree. v1.51.0 is also older than the fix for GHSA-mw2c-m758-9v5q (memory exhaustion parsing binaries, fixed in v1.52.0). It reads only the final image: the pinned base and Grimoire's own binaries.
- **The vulnerability database is the one unsigned, unpinned input** (I7). It is rebuilt daily, so a release re-run can fail where the pull request passed.
- **What Grype can't see.**
  - Rust crates only through cargo-auditable's data, which `coverage` requires, and Rust matches only through GHSA.
  - Statically linked C libraries are invisible to it; by inference, that includes musl libc.
  - Matches of unknown severity never trip `--fail-on`, and low or negligible ones pass by design.
- **Ignore rules don't expire.** Grype has no expiry field. Every rule is reviewed when the base image or Grype is bumped.
- **Only the index is signed.** Platform manifests aren't signed separately; signing them (`--recursive`) is a follow-up. A verifier must check the index digest. How policy engines (tools that decide whether a system may run an image) treat a signature on the index rather than on each platform manifest is UNVERIFIED.
- **The `sha256-<index hex>` tag is shared and rewritten without a lock.**
  - go-containerregistry (the Go library inside cosign and crane) updates it with no conditional request (one that fails if the tag changed since it was read), so a concurrent writer could drop an entry.
  - cosign and `actions/attest` run one after the other in one job (`sign-image.yml`'s image job), which then checks that both are listed. It also checks the bundle format itself, because `cosign verify` falls back silently to the legacy signature format.
  - Whether GHCR's cleanup of untagged versions would delete referrer manifests is UNVERIFIED.
- **The digest in the release notes isn't attested.** Release notes stay editable on an immutable release, and the release attestation covers only the tag, the commit and the assets. No release asset records the image digest. What proves a digest is the image's own signature and attestation.
- **A signature doesn't name a version.** Any image `sign-image.yml` signed in a release run verifies under the same identity and pins. The version is in the index's signed annotations. The weekly rescan follows the version tag, then checks the signature, so it would not notice the tag moved to another signed release.
- **The GHCR login sits in `~/.docker/config.json`**, in `publish-image` and in `sign-image.yml`'s image job. `actions/attest` reads credentials only from that file, so the signing job's login can't use a job-scoped `DOCKER_CONFIG`. The file holds the token base64-encoded, not encrypted, until the logout step.
- **`image-tags.sh` can't tell a failed verification from a missing signature.** If cosign fails for another reason, such as a network error, the script takes `latest`'s image for unsigned and stops the job, leaving `latest` on the newer release. That can happen only when an older release's `sign-image` runs after a newer release. `release.md` §3 says what the error means, and a re-run once the check works finishes.
- **GHCR's behavior hasn't been seen yet** (UNVERIFIED until the first release):
  - whether it sends `OCI-Subject` or honors `If-Match`;
  - whether tags can be made immutable (its docs are silent);
  - a new package's first visibility (its docs conflict);
  - whether it links the package from the config label or the index annotation (the image carries both);
  - and how `sha256-*` tags appear as package versions.
- **Making the package public can't be undone.** Anonymous pulls, the documented `docker run` forms and the weekly rescan need it (`release.md` §2 D, the first release run).
- **CI exercises rootful Docker 28.0.4 only**, since its first run on 2026-11-18. Rootless Docker was covered only locally. Docker Desktop and Podman weren't tested; Docker Desktop's file ownership on macOS and Windows and rootless Podman's `--userns=keep-id` are UNVERIFIED.

## 6d. The repository and contributions

Grimoire has one maintainer. This section says what the repository's rules, CI and review guarantee today, and what they don't. The settings below were read with the GitHub API on 2026-11-17 and 2026-11-18, and they can change; anyone who can read the repository can see its active rulesets on GitHub. How to contribute is in [`CONTRIBUTING.md`](../CONTRIBUTING.md).

**Who can change `main`:**
- The repository belongs to a personal account, and its owner is the only collaborator. Everyone else contributes from a fork (their own copy of the repository).
- A ruleset on every branch requires a pull request with one approval and a code owner's review, allows only merge commits, and blocks deleting branches and force-pushing (overwriting a branch's history). It also requires code-scanning results from CodeQL (GitHub's code analysis) and code-quality results.
- **Code-scanning results now exist for pull requests into `main`; code-quality results don't.**
  - When read on 2026-11-17 and 2026-11-18, before `codeql.yml` was added, nothing produced either: no workflow ran CodeQL, and the rule evaluation that refused release-plz's commit said "Code Scanning may not be configured" (COMMUNITY-P7 in `specs.md`, the Phase 7 facts about GitHub's community files and rulesets).
  - Since `4dd223d` (2026-11-18), `codeql.yml` (below) runs CodeQL on pull requests into `main` and pushes to `main` only, and pull request 1 received a passing `CodeQL` check on 2026-11-19. Whether the `code_scanning` rule then passes wasn't read. On every other branch, such as `release-plz-*`, nothing produces code-scanning results, so that rule still refuses commits there.
  - Nothing produces code-quality results, and whether Code Quality is on couldn't be read (UNVERIFIED). While that holds, the code-quality rule refuses every merge and every pushed commit, on every branch, except through the bypass. Turning on Code Quality, or dropping that rule, and settling the code-scanning rule for the branches CodeQL doesn't analyze, are the maintainer's decisions (`release.md` §2 A). CodeQL default setup must stay off, because GitHub rejects advanced-setup results while it is on (CODEQL in `specs.md`).
- **The ruleset blocks release-plz.** It covers the `release-plz-*` branches too, so its pull-request and code-scanning rules refused release-plz's commit there (run 44198210657, `release.md` §2 A). The release pull request, and so every release, depends on the maintainer changing the ruleset.
- [`.github/CODEOWNERS`](../.github/CODEOWNERS), the file that names who must review changes to each path, makes the maintainer the owner of every path. The security-sensitive paths have lines of their own, so that a future co-maintainer added for every path doesn't also get these:
  - `.github/`, the `Dockerfile` and `.grype.yaml`, `deny.toml`, `packaging/`;
  - `xtask/` and `.cargo/` (the release logic and the `cargo xtask` alias);
  - the installer, resolver, adapter and MCP hardening crates;
  - the skill, tool and MCP server assets;
  - and the Claude Code marketplace (`plugins/` and `.claude-plugin/`).
- **The ruleset can be bypassed.** The repository's administrator role and some GitHub Apps can bypass it. GitHub doesn't let a pull request's author approve it, so the maintainer's own changes reach `main` through that bypass. **Most changes have no second reviewer**, and nothing in this document means that every change to `main` was reviewed by a second person, or passed CI.
- A contributor's pull request needs the maintainer's approval. While nothing produces the code-quality results, and while it isn't known whether the code-scanning rule passes, it also needs the maintainer to merge it through the bypass.

**What CI checks:**
- `ci.yml` runs on every pull request and every push to `main`. It checks:
  - formatting, and clippy (Rust's linter) with warnings denied;
  - the tests on Linux, macOS and Windows, and the build with the minimum supported Rust version (MSRV);
  - the catalog, schemas and generated files (`--check`), and version bumps for changed assets;
  - the scaffold templates (which `cargo xtask new` creates assets from) and `claude plugin validate --strict`;
  - unused dependencies and the documentation's links.
- `security.yml` runs on the same events and weekly: cargo-deny (the dependency policy, below), gitleaks (a secret scanner) over the whole history, zizmor (at its pedantic persona, which reports more than the regular one), actionlint, ShellCheck and `actions.lock` (the record of pinned actions, below). The weekly and manual runs also scan the released image (§6c).
- `codeql.yml` runs CodeQL (advanced setup, configured by this workflow file) over the Rust, JavaScript and Python code and the workflows, on every pull request to `main`, every push to `main` and weekly, and uploads the results to code scanning.
- `release.yml` runs the release dry-run on every pull request, summarized by `release checks` (§6b). A pull request's run is always a dry-run: the signing and publishing jobs run only in release mode on `main`, and the signing workflows skip any other caller.
- **No ruleset requires these checks yet** (`release.md` §2 A lists that step), and the bypass skips them anyway. Until then, a failing check is a signal the maintainer has to read, not a gate.

**What review must look for in a catalog contribution.** `cargo xtask validate` checks form: the schema, the permissions that declared capabilities imply, a `shell` grant for bundled scripts, markers and fences, links, version bumps, and the MCP hardening's text searches (M12). It can't check intent. Review must:
- **Read model-facing text as a prompt-injection surface.** Rules, `SKILL.md` and everything a skill ships, an agent's prompt and its `description`, and an MCP server's instructions and tool descriptions all reach the model (§6), and often in clients the asset wasn't installed for (§3a).
  - Read them as instructions an agent will follow.
  - Look for text that tells it to run commands, fetch or send data, change settings or files outside the task, or ignore the user and the project's own instructions.
  - Look also for text hidden from a reader (§6).
- **Check that the declared permissions match the behavior.** `[permissions]` is what a user consents to (F14). It is not a sandbox: neither Grimoire nor any client enforces it on a binary or a script. Validation checks declarations against each other and against the files an asset ships, never against what its code does. Review compares the code with the declaration: what it reads and writes, whether it uses the network or starts processes, and that every environment variable it reads is in `permissions.env`.
- **Review binaries as code.** A tool or MCP server is a Rust crate that releases build into a binary, and it runs on users' machines with their privileges: as a hook on client events, such as every tool call, or as a server in every session. Review its source, its dependencies (`deny.toml` applies to them), its input handling and its limits, as for the core crates. That an MCP server serves through `agent-grimoire-mcp` is checked by text searches that a determined author can get around (M12).

**Dependency policy:**
- **Rust crates** follow [`deny.toml`](../deny.toml), which cargo-deny (a checker for Rust dependencies) checks in `security.yml`.
  - It refuses crates with RustSec advisories (RustSec is the Rust security advisory database), unmaintained and unsound ones included (an unsound crate's safe code can still break memory safety), and yanked versions (versions withdrawn from crates.io). An ignored advisory needs a reason; today one is ignored (RUSTSEC-2026-0543, chrono unmaintained).
  - It allows only the MIT and Apache-2.0 licenses, plus named per-crate exceptions.
  - It refuses duplicate versions except exact, explained skips; wildcard version requirements; OpenSSL and native TLS; and three unmaintained or deprecated YAML crates (YAML is a configuration file format).
  - Its `[bans.build]` section refuses executables and interpreted scripts shipped in dependencies that run at build time, except files pinned by checksum.
  - Crates may come only from crates.io, never from Git.
  - Every CI and release build uses `--locked`, xtask included (`cargo --locked xtask`), except in `ci.yml`'s scaffold job, which adds probe crates (test crates) as new members of the Cargo workspace, the set of crates built together, and so can't.
- **Updates** come from Dependabot, weekly, for Cargo, GitHub Actions, the Rust toolchain, the hash-pinned pip requirements (Python packages pinned by hash) and the `Dockerfile` base.
  - Every entry has a cooldown of at least 7 days for version updates, and 30 days for semver-major Cargo and pip updates (a new major version under semantic versioning, SemVer). As [`.github/dependabot.yml`](../.github/dependabot.yml) records, security updates aren't held back.
  - Dependabot never changes the MSRV (`DEPENDABOT`, the Dependabot options entry in `specs.md`).
  - For the base image the cooldown has no effect, so its age is checked by hand (§6c).
- **Actions** are pinned to full commit SHAs with version comments, and [`.github/workflows/actions.lock`](../.github/workflows/actions.lock) records every pin (`GHA-SEC` in `specs.md` covers GitHub's guidance on hardening Actions, pins and lockfiles). `security.yml`'s `actions-lock` job fails when the lockfile is incomplete or out of date. Workflows start with `permissions: {}` and grant each job only what it needs.
- **Tools** that workflows download are pinned to exact versions.
  - Most are checked against a pinned SHA-256, two only against a GitHub asset digest (§6b, "Some pins protect only the download").
  - cargo-deny is built from crates.io with its lockfile, and cargo-shear, cargo-auditable and cosign come through pinned installer actions.
  - Each pinned action and tool was a release at least 7 days old when adopted.

**Residual risks:**
- **One account can change everything.** The maintainer's account can change `main`, the workflows and the repository's settings, and approves every release, with no second reviewer and no passing check. A compromised maintainer account is a compromised project, whatever §6b's approvals say. The GitHub Apps on the ruleset's bypass list can skip its rules too.
- **Review is one person's.** Prompt injection in model-facing text, permissions that don't match a binary's behavior, and malicious code in an asset crate are caught only if the maintainer catches them.
- **Fixing a reported vulnerability in private skips CI.** GitHub runs no CI or status checks on the pull requests of an advisory's temporary private fork (the private copy of the repository GitHub makes for fixing a reported vulnerability), and merging them doesn't enforce the target branch's protection rules. A fix made there must be checked locally before it is merged.

## 7. Known gaps and residual risks

- **The final step on Windows.** cap-std performs the final rename and remove path-based on Windows, after resolving the parents inside the sandbox. Confinement holds, but on Windows the last step isn't relative to a directory handle (ADR-0006, which chose cap-std).
- **Leftover temporary files.** After a crash, a temporary file with a UUID name (a random unique identifier) can be left in a managed directory. Grimoire does not clean these up automatically. On Linux, cap-tempfile 4.0.3 uses an unnamed `O_TMPFILE` file (a Linux file with no name in any directory) where the filesystem supports it, and names it only just before the rename, so a leftover is rare there.
- **Interrupts.** There is no signal handler (code that runs when the process receives a signal, such as Ctrl-C's SIGINT). Atomic writes keep files consistent, and rerunning converges. Grimoire's own exit codes are 0 to 6; a run stopped by Ctrl-C ends with the status the shell reports for a process killed by SIGINT, 130 in a POSIX (standard Unix) shell.
- **Empty directories.** After deleting a file it owns, Grimoire removes every directory above it that is now empty, including one the user created empty. An empty directory holds no content, and Git does not track one.
- **Concurrent runs.** Two `grimoire` processes in the same project at once are not locked against each other.
- **A tampered lock within Grimoire's locations.** Someone who can edit `grimoire.lock` could record the hash of a file Grimoire manages, so that a later removal deletes it. F18 keeps this to Grimoire's own locations. Treat lock diffs like code in review.
- **JSON layout.** Adding an entry to a container written on one line reflows it across lines. The values are unchanged.
- **Claude Code behavior that is not documented** (`specs.md`):
  - whether an `@AGENTS.md` import placed between two HTML-comment marker lines is honored;
  - what versions before 2.1.139 do with exec-form `args`, and before 2.1.295 with `onFailure`.
- **MCP over HTTP, not yet tested end to end** (`CC-P3`, the Phase 3 Claude Code facts, UNVERIFIED):
  - Claude Code's OAuth flow against a Grimoire server, with a real authorization server;
  - whether Claude Code sends an `Origin` header;
  - whether it sends the RFC 8707 `resource` parameter (which names the server a token is requested for).
- **Client behavior that is not documented** (Phase 4, `*-P4`):
  - whether Cursor and Copilot honor `args` and `onFailure` in Claude Code hooks;
  - whether Cursor expands `@AGENTS.md` in `CLAUDE.md`;
  - whether Copilot CLI deduplicates the `AGENTS.md` it reads twice;
  - what an unset `${env:NAME}` becomes in `.cursor/mcp.json`.

  None of the four new clients was run.
- **Trust lives in the client.** Codex loads `.codex/config.toml`, Gemini CLI loads everything from the project, and Copilot loads `.mcp.json` only in a project the user has trusted. Grimoire can't check or grant that; `doctor` says so.
- **Key rotation is manual.** The JWKS file must be replaced when the authorization server rotates its keys.
- **Revocation is per key, not per token.** Removing a key from the JWKS file, or deleting the file, stops its tokens within about 5 seconds. A single token cannot be revoked before it expires. A file that cannot be read or parsed keeps the previous keys.
- **No browser clients.** There is no CORS support (cross-origin resource sharing, the headers that let a web page call a server on another site).
- **Leaks that aren't warned about.** Rules and skills that a client reads without being selected for them are not reported (§3a).
- **Published READMEs are frozen per version.**
  - The root `README.md` is the `agent-grimoire` crate's page on crates.io, the wheels' long description on PyPI, and a file in every CLI release archive. Each library crate carries its own `README.md`, each asset archive its asset's, and `xtask/src/release/channels/npm.rs` writes the npm packages' READMEs.
  - Treat each of them as fixed for a version once it is published:
    - crates.io renders the text in the publish request;
    - PyPI keeps the uploaded text when it renders it again, and whether a release's description can be edited afterward is UNVERIFIED (README-RENDER-P7 in `specs.md`);
    - npm shows a new README only with a new version;
    - and an immutable release locks its archives, but only for a release published while the setting is on (`release.md` §2 A, and §6b step 7).
  - So a wrong or outdated security note or install step stays with that version. A correction reaches users only with the next release, or through an advisory (§8).
  - The text is frozen, but not how it renders: crates.io resolves relative links against the repository's default branch as it is now, not the published version, and PyPI renders stored descriptions again when it upgrades its renderer.
- **The process around the code:** one maintainer, one reviewer, and CI that doesn't yet gate a merge (§6d).
- **Releases and distribution:** the residual risks are listed in §6b, and the container image's in §6c.

## 8. Reporting a vulnerability

Report a vulnerability privately, never in a public issue or pull request. Private vulnerability reporting is enabled on the repository (read on 2026-11-17), and its form is at <https://github.com/jabbott-iii/Grimoire/security/advisories/new>; it needs a signed-in GitHub account. [`SECURITY.md`](../SECURITY.md) is the policy: what to report, what to include, and what happens next.

- **One person handles reports.** The repository has one collaborator, so only the maintainer can triage a report (assess it and decide what to do) and publish an advisory. GitHub notifies them only if they watch the repository for all activity or for security alerts (`release.md` §2 A).
- **Most users get no alert.**
  - GitHub's Advisory Database, which Dependabot alerts are built on, covers crates.io, npm and PyPI packages, but an alert reaches only a repository whose dependency graph lists the package. A CLI installed with `cargo install`, `npm install --global` or `pipx` isn't such a dependency.
  - Homebrew, Scoop, winget, the install scripts and container images aren't among the database's ecosystems at all, and the tool and MCP server binaries ship only in the release archives and the image.
  - So users learn of a fix from the advisory and the release notes, which `SECURITY.md` asks them to watch.
- **A fix made in private skips CI** (§6d, residual risks).
