# Subsystem: `agent-grimoire` (the CLI)

`agent-grimoire` is the `grimoire` command-line interface (CLI): a thin binary, `src/main.rs`,
over a library whose one entry point, `run`, parses a command line, runs one command and returns
its exit status. It turns each command into calls to the five library crates (model, catalog,
resolve, adapters and install) and reports what they found, as human text or as one JSON
document. It guarantees that `--json` puts exactly one JSON document on standard output, that no
command prompts when nobody can answer, that every outcome maps to one of seven documented exit
codes, and that the output is deterministic. [ADR-0001](../adr/0001-catalog-delivery.md) (the
catalog is embedded in the binary) and [ADR-0004](../adr/0004-package-and-binary-names.md) (the
package is `agent-grimoire` and the binary `grimoire`) shaped it.

This spec is the design. The user reference is [`../cli.md`](../cli.md), for the commands, flags,
exit codes, `--json` output and environment, and [`../diagnostics.md`](../diagnostics.md), for
every user-facing diagnostic code. Tests compare both with the code; nothing compares this spec
with it. The API reference is the crate docs in
[`crates/agent-grimoire/src/lib.rs`](../../crates/agent-grimoire/src/lib.rs).

## Why a thin CLI over a library

The CLI is a library so that its tests can drive it as users do, with a fixture catalog (a small
catalog written for a test), streams held in memory and a temporary project, and check the exit
status, the output and the files. The binary only builds the real context: the embedded catalog,
the current directory, the process's streams, whether standard input is a terminal, and `PATH`.

What Grimoire keeps stable is the command line itself: its commands and options, exit codes,
`--json` output and diagnostic codes ([`../05-release.md`](../05-release.md#the-public-api)). The
library part (`run`, `Context` and `Exit`) is not part of that public API, and nothing outside the
CLI's tests depends on it.

`add` and `remove` only change the desired state, and `apply` reconciles the disk with it. That
keeps CI flows explicit: a pipeline can run `plan --check` or `verify` without anything being
written.

Built in M2.7 as described here. The library's surface: `run` and `Context` in `src/lib.rs`, and `Exit` in `src/out.rs`:

```rust
/// Runs `grimoire` with `args` (including the program name).
pub fn run<I, T>(args: I, ctx: &mut Context<'_>) -> Exit
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone;

pub struct Context<'a> {
    pub source: &'a Source,       // the catalog to read and install from
    pub version: Version,         // the running CLI's version, checked against `min-cli-version`
    pub cwd: PathBuf,             // the directory the command starts in
    pub stdin: &'a mut dyn BufRead, // read only for answers to prompts
    pub stdout: &'a mut dyn Write,
    pub stderr: &'a mut dyn Write,
    pub interactive: bool,        // whether prompts are possible
    pub path: Option<OsString>,   // the `PATH` value
}

pub enum Exit { Ok = 0, Failure = 1, Usage = 2, Invalid = 3, Conflict = 4, Check = 5, Refused = 6 }
```

## Commands

| Command | Purpose | Writes |
|---|---|---|
| `grimoire init [--client …]` | Create `grimoire.toml`; optionally detect clients already in use | `grimoire.toml` |
| `grimoire list [QUERY] [--kind] [--tag] [--client] [--maturity]` | Browse or search the catalog (list and search in one command) | — |
| `grimoire show <id>` | Manifest, permissions, per-client outputs, dependencies, changelog head | — |
| `grimoire add <id\|bundle:…\|tag:…>… [--allow …]` | Add to `grimoire.toml` after resolving and consenting to permissions | `grimoire.toml` |
| `grimoire remove <id>…` | Remove from `grimoire.toml` | `grimoire.toml` |
| `grimoire plan [--check]` | Show what `apply` would do, with diffs. `--check` exits 5 if there are changes | — |
| `grimoire apply [--force] [--write-new]` | Make the managed content on disk match `grimoire.toml` + lock | managed outputs, `grimoire.lock` |
| `grimoire update [<id>…] [--no-apply]` | Re-resolve to the newest versions the requirements allow and update the lock, then apply (`--no-apply` updates the lock only) | `grimoire.lock`, managed outputs |
| `grimoire verify` | Check every locked output on disk against its hash (a CI gate; exit 5 on mismatch) | — |
| `grimoire doctor` | Environment report: each client's installed outputs and notes on what it needs (trust, approvals; not checked), tool and server binaries on `PATH`, the Copilot cloud-agent MCP settings, config parse errors | — |
| `grimoire uninstall [--purge]` | Remove all managed content and the lock (`--purge` also removes `grimoire.toml`) | managed outputs, lock |

`add --apply` (and `remove --apply`) is a convenience that runs `apply` after the edit. Each
command's flags, output and exit codes are in [`../cli.md`](../cli.md).

### As implemented in Phase 2

- **`init`** detects `claude-code` from `CLAUDE.md` or `.claude/`, and `agents-md` from
  `AGENTS.md` or `.agents/`. With nothing detected and no `--client`, it is a usage error
  (`init/no-client`). Clients without an adapter yet are refused.
- **`add`:**
  - Resolves the new selection before writing, so a broken selection never reaches
    `grimoire.toml`.
  - Asks for each asset's missing permissions on a terminal, or takes them from `--allow`.
  - A plain id that names a bundle is added as a bundle.
  - New direct entries get `^<catalog version>`.
- **`remove`** warns when another selector (a bundle, a tag or another asset's `requires`) keeps
  the asset installed.
- **`update [<id>…]`** lets the named (or all) pins move to the catalog's versions. `--no-apply`
  writes the lock with the new versions but keeps every output record describing the files as they
  are, so the next `apply` performs the update.
- **`doctor`** never runs a binary. It only checks that each locked tool or MCP server binary is on
  `PATH`.

### As extended in Phase 4

- `init` detects every client: `claude-code` (`CLAUDE.md`, `.claude/`), `agents-md` (`AGENTS.md`,
  `.agents/`), `codex` (`.codex/`), `cursor` (`.cursor/`), `gemini-cli` (`GEMINI.md`, `.gemini/`)
  and `copilot` (`.github/copilot-instructions.md`, `.github/instructions/`, `.github/agents/`).
  Every client has an adapter.
- `doctor` counts each client's outputs from the lock.
- It notes what the client needs before it reads them, which Grimoire can't check: Codex and
  Gemini CLI trust, Cursor's MCP approval, Copilot's folder trust.
- It prints the JSON to paste into Copilot cloud agent's MCP settings, for each MCP server
  installed for `copilot`. Cloud-agent MCP can be configured only in the repository's settings on
  GitHub ([ADR-0005](../adr/0005-client-priority.md), which chose the six v1 clients), so Grimoire
  can't write it.
  - Optional variables there get an empty default (`${COPILOT_MCP_<NAME>:-}`).
  - A server configured over Streamable HTTP is given by its URL, which the cloud agent can use
    only if it can reach it without OAuth.

### As fixed after Phase 7

- `init` and `doctor` check for client files through the root handle (`Root::exists`), which
  follows no symbolic link on the way to the name; a link at the name itself still counts. So
  does the `AGENTS.override.md` check.
- `uninstall --json` reports the files it removed, with `lock-removed` and `config-removed`, and
  `uninstall --purge` checks `grimoire.toml` first (`Root::has_file`), so a link, a special file or
  a directory there stops it before anything is removed.
- `plan` and `apply` share one comparison of the lock (`lock_differs` in `src/project.rs`). A
  missing lock is a change, so `plan --check` in an empty project reports that the lock would be
  created.
- `list` pads each column but the summary to its longest value among the entries shown.

### `doctor`

`doctor` reports one check per line, `ok` or `FAIL`, and exits 5 (`doctor/problems`) when any
check fails. The checks are:

- `catalog`: how many assets and bundles this build embeds;
- `grimoire.toml`: valid, or missing or invalid;
- `grimoire.lock`: valid, not created yet, or unreadable or invalid;
- `client <id>`, for each selected client: how many outputs the lock records for it, and what the
  client needs before it reads them (trust, approvals), which Grimoire can't check. With an
  unreadable lock, the outputs are unknown;
- `plan`: the pending changes, as `plan --check` counts them, and whether any conflicts. With an
  invalid lock, `doctor` skips planning and this check fails with that reason, so the lock's
  diagnostics are reported once;
- `outputs`: the lock's outputs checked against the disk, as `verify` does;
- `binary <name>`, for each locked tool or MCP server binary: on `PATH` or not, or not needed for a
  server configured over Streamable HTTP;
- `copilot cloud agent`: the settings JSON for each MCP server installed for `copilot`.

## The shared pipeline

`src/pipeline.rs` holds resolve → render → plan → apply, shared by `plan`, `apply`, `update`,
`uninstall`, `doctor`, and `add` and `remove` with `--apply`.

1. `prepare` reads the lock, resolves the project ([`resolve.md`](resolve.md)), and renders it for
   the selected clients ([`adapters.md`](adapters.md)). Resolution and rendering warnings are
   reported as they come; a failure of either exits 3.
2. It warns (`plan/hook-binary-missing`) for each selected tool whose hooks fail closed
   (`on-failure = "block"`) when its binary isn't on `PATH`, because the client would then block
   every event those hooks cover, such as every tool call. It looks for the binary, and never runs
   it.
3. It reads every path the plan needs through the project root and plans
   ([`install.md`](install.md#desired-state-plan-apply)).
4. When `codex` is a project client, it adds the two Codex warnings: `plan/codex-instructions-size`
   when the planned `AGENTS.md` is 24 KiB or more, three quarters of the 32 KiB Codex reads in
   total ([`CODEX-P4`](../specs.md#codex-p4-releases-agentsmd-and-project-trust)), and
   `plan/codex-override` when `AGENTS.override.md` exists, because Codex then reads it instead.
5. `run_apply` applies the plan and writes the lock last. After a failure partway through, the lock
   still records the files already written, and the message says how many.

`uninstall` plans with no desired outputs (`prepare_uninstall`), which removes everything the lock
records. `update --no-apply` stops after writing the lock. `plan` and `doctor` stop after planning.

## Machine use

- `--json` on every command puts exactly one JSON document, the envelope, on stdout:
  `{ "format": 1, "command": …, "ok": …, "exit": <code>, "result": …, "diagnostics": [ … ] }`.
  - Human diagnostics go to stderr. With `--json`, stderr stays quiet.
  - Usage errors also produce the envelope: `command` and `result` are `null`, `exit` is 2, and the
    one diagnostic has the code `cli/usage`.
  - `--help` and `--version` print plain text even with `--json`.
  - Every key, and each command's `result`, is in
    [`../cli.md`](../cli.md#json-output).
- `--no-input` disables prompts. It is implied when stdin is not a TTY (a terminal) or the `CI`
  environment variable is set. A prompt that would be needed becomes an error that names the flag
  to pass. Only `add` asks a question: whether to accept an asset's permissions.
- `-C <dir>` sets the project root.
- Output is deterministic: stable ordering, and no timestamps in written files.
- Control characters in a diagnostic are printed escaped, so a hostile file name can't send escape
  sequences to a terminal.

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Success, or nothing to do |
| 1 | Unexpected failure (I/O, internal error) |
| 2 | Usage error (bad flags or arguments) |
| 3 | Invalid configuration or manifest, or a resolution failure ([`resolve.md`](resolve.md#resolution-errors)) |
| 4 | Conflict: drift or an unmanaged file is in the way. Nothing was written, except when a file changed or failed partway through an `apply` (the message says so, and the lock records what was written) |
| 5 | Check failed: `plan --check` found changes, `verify` found mismatches, or `doctor` found problems |
| 6 | Refused for safety (path escape, symlink at a managed path) |
| 130 | Not returned by Grimoire. It installs no signal handler, so Ctrl-C (`SIGINT`) kills the process, and a POSIX shell reports that as 130 (128 + the signal number). What other callers and Windows see is unverified. A rerun converges ([`install.md`](install.md#desired-state-plan-apply)) |

The `Exit` enum in `src/out.rs` has one variant for each of 0 to 6. Two more exceptions to
"nothing was written" after a conflict: `add --apply` and `remove --apply` have already written
`grimoire.toml`, and `--write-new` has written the files without conflicts. `run` doesn't catch
panics: a panic passes up out of `run`, no `--json` document is written, and the binary exits with
Rust's default status for a panic, 101. Every condition for each code is in
[`../cli.md`](../cli.md#exit-codes).

## Invariants

1. Every exit status is one of 0 to 6, and each kind of failure has its own: a usage error exits 2,
   an invalid project 3, a conflict 4, a failed check 5, a safety refusal 6.
2. With `--json`, standard output holds exactly one JSON document with `format` 1, even for a usage
   error.
3. No command prompts without a terminal; a prompt that would be needed fails and names the flag
   to pass.
4. `--allow` grants only the permissions of the assets the command names.
5. `add` never writes a selection that doesn't resolve, and `remove` of an asset that something
   else still selects keeps its grant.
6. `doctor` never runs a binary, reports a missing binary, fails when the project doesn't resolve,
   and reports a broken lock's diagnostics once.
7. `init` detects every client from its files, and follows no symbolic link doing so.
8. `uninstall` reports what it removed, and `--purge` removes nothing when `grimoire.toml` isn't a
   regular file.
9. A missing lock is a change: `plan --check` reports it, and the first `apply` creates it.
10. `docs/cli.md` lists exactly the commands, flags and exit codes the code has.

## Tests

The end-to-end tests are in `crates/agent-grimoire/tests/`: `cli.rs` runs the commands against a
fixture catalog, `clients.rs` covers the six clients, `builtin.rs` uses the embedded catalog, and
`binary.rs` runs the built `grimoire` binary itself.

| Invariant | Tests |
|---|---|
| 1 | `errors_have_distinct_exit_codes` in `cli.rs`; `usage_errors_exit_2` in `binary.rs` |
| 2 | `json_envelopes` in `cli.rs`; the crate-docs example in `src/lib.rs` (a doc test) |
| 3 | `prompts_are_impossible_without_a_terminal` in `binary.rs`; `hints_name_flags_that_work` in `cli.rs` |
| 4 | `allow_never_reaches_assets_the_command_does_not_name` and `permissions_need_consent` in `cli.rs` |
| 5 | `removing_an_asset_that_stays_selected_keeps_its_grant`, `inline_assets_tables_can_be_added_to` and `bundles_gone_from_the_catalog_can_still_be_removed` in `cli.rs` |
| 6 | `doctor_reports_missing_binaries`, `doctor_fails_when_the_project_does_not_resolve` and `doctor_reports_a_broken_lock_once` in `cli.rs`; `doctor_needs_no_local_binary_for_a_server_over_http` in `builtin.rs`; `doctor_prints_the_copilot_cloud_agent_settings` in `clients.rs` |
| 7 | `init_detects_every_client` in `clients.rs`; `client_detection_follows_no_symbolic_link` (Unix-only) and `a_file_where_a_client_folder_would_be_is_no_client` in `cli.rs` |
| 8 | `uninstall_reports_what_it_removed`, `purge_stops_before_removing_anything_when_grimoire_toml_is_not_a_file`, `purge_stops_before_removing_anything_at_a_linked_grimoire_toml` (Unix-only) and `a_lost_lock_still_lets_uninstall_clean_up` in `cli.rs` |
| 9 | `an_empty_project_gets_its_lock_from_the_first_apply` in `cli.rs`; `empty_embedded_catalog_still_works_end_to_end` in `binary.rs`; `a_missing_lock_differs_and_line_endings_dont` in `src/project.rs` |
| 10 | `docs_cli_md_matches_the_command_line` and `the_parser_reads_only_its_own_structure` in `src/docs_check.rs` ([`../04-quality.md`](../04-quality.md#checked-documents)) |

Also: `help_and_version_succeed` in `binary.rs`, `list_aligns_its_columns` in `cli.rs` and
`columns_pad_every_column_but_the_last` in `src/commands.rs` for `list`, and
`edits_keep_comments_and_formatting` and `inline_tables_and_trailing_comments` in `src/project.rs`
for how `add` and `remove` edit `grimoire.toml`. That every diagnostic code the CLI can print is in
`docs/diagnostics.md` is checked by `diagnostics_doc_lists_every_user_facing_code` in
`tests/diagnostics_doc.rs`. What the installer itself guarantees is tested as
[`install.md`](install.md#tests) lists.
