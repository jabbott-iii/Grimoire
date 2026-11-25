# Subsystem: `agent-grimoire-install`

`agent-grimoire-install` is the installer. It compares the outputs a project should have with
`grimoire.lock`, the record of what Grimoire installed, and with what is on disk, then makes only
the changes needed and checks the disk against the lock afterward. An output is a unit of content
Grimoire owns: a whole file, a managed block in a shared Markdown file such as `AGENTS.md`, or one
entry in a configuration file such as `.mcp.json`. The crate guarantees that every read and write
stays inside the project root and follows no symbolic link
([ADR-0006](../adr/0006-filesystem-confinement.md), which chose cap-std to confine every access to
the project root), that each file is replaced whole or left as it was, and that content the user
edited is never overwritten without `--force`. Its API reference is the crate docs in
[`crates/agent-grimoire-install/src/lib.rs`](../../crates/agent-grimoire-install/src/lib.rs), which
also list how it behaves on each operating system.

## Why a planner over a confined root

A user's project is shared ground. The user, other tools and the clients themselves write the same
files Grimoire writes, and someone who can write into the project could try to redirect a write
outside it. Two choices follow from that.

- **Plan first, then apply.** A pure planner (one that reads nothing and writes nothing itself)
  decides every action before anything is written. So `plan` can show exactly what `apply` would
  do, conflicts stop the whole run before the first write, and a second run that finds nothing
  to do writes nothing. The cost is that each file is read twice: once to plan, and again just
  before it is written, to catch a change in between.
- **Every access through one handle on the root.** Checking a path with the standard library and
  then writing by path leaves a time-of-check to time-of-use (TOCTOU) race: a directory can be
  swapped for a symbolic link between the check and the write. Opening every path from a
  capability handle on the root, with cap-std, closes that race. The cost is a dependency with
  scoped exceptions in `deny.toml` (ADR-0006), and refusing some layouts a user might want, such
  as a symbolic link to a shared rules folder.

Smaller owned units serve the same goal. Grimoire owns a block or an entry rather than the whole
`AGENTS.md` or `.mcp.json`, so the user's own text around it is never touched.

## Desired state, plan, apply

`grimoire.toml` describes the desired state. `apply` works out how the disk differs from it, and
changes only that:

1. **Resolve:** `grimoire.toml` + lock + catalog become the resolved asset set
   ([`resolve.md`](resolve.md)). The versions recorded in the lock (its pins) are kept unless
   `update` asks otherwise.
2. **Render:** adapters turn the resolved set and the selected clients into the desired outputs
   ([`adapters.md`](adapters.md)).
3. **Plan:** compare the desired outputs, the lock and disk. The result is a list of
   `create | update | delete | unchanged | adopt | forget | conflict` actions, each with a reason
   and a diff. `adopt` is content already as desired on disk, and `forget` is content no longer
   wanted and already gone; for both, only the lock changes.
4. **Apply:**
   1. Check every precondition first. Any conflict means nothing is written (exit 4) unless
      `--force` or `--write-new` is given.
   2. Write each output atomically, so a crash never leaves a half-written file
      ([Filesystem safety](#filesystem-safety)).
   3. Write the lock last.

**Idempotence** (running it twice has the same effect as running it once): a second `apply`
produces an all-`unchanged` plan and writes nothing, not even the lock.

**Crash safety:** a unit whose on-disk hash already equals the *desired* hash counts as applied.
Re-running after an interruption therefore converges on the desired state, without false drift.

Built in M2.4 as described here. The planner's actions, in `plan.rs` (`Gone` is shown to users as
`forget`; `Commented` was added in M4.1):

```rust
pub enum Action {
    Create,             // write a new unit
    Update,             // replace the unit Grimoire wrote earlier
    Delete,             // remove a unit that is no longer wanted
    Unchanged,          // nothing to do
    Adopt,              // already as desired on disk; only the lock changes
    Gone,               // no longer wanted and already gone; only the lock changes
    Conflict(Conflict), // needs --force (or --write-new)
}

pub enum Conflict {
    Edited,             // changed on disk since Grimoire wrote it
    Missing,            // Grimoire wrote it, and it is gone from disk
    Unmanaged,          // something Grimoire does not manage is already there
    EditedBeforeDelete, // no longer wanted, but changed on disk since Grimoire wrote it
    NotFound,           // an array element Grimoire wrote can no longer be found
    Commented,          // a TOML or JSONC entry has comments added inside it
}
```

The CLI drives the crate in one pipeline, which the crate docs describe step by step:
[`plan::paths`](../../crates/agent-grimoire-install/src/plan.rs) lists every path the plan needs,
the CLI reads each through the root, `plan::plan` turns the desired outputs, the lock and those
bytes into a `Plan`, `apply::apply` makes the changes, `apply::next_lock` builds the next lock, and
`apply::verify` lists each locked output whose content on disk differs. Which commands run which
steps is in [`cli.md`](cli.md#the-shared-pipeline).

## Ownership units and merge strategies

| Unit | Used for | What Grimoire owns | Drift check |
|---|---|---|---|
| `file` | rule and agent files, skill payload files | the whole file | file hash vs lock |
| `block` | `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` | the text between `<!-- grimoire:begin <id> -->` and `<!-- grimoire:end <id> -->` | block-content hash vs lock |
| `entry` | `.mcp.json` `mcpServers.<id>`, `.cursor/mcp.json`, `.gemini/settings.json`, `.codex/config.toml` `[mcp_servers.<id>]`, hook arrays | one key (by JSON-Pointer-style path) or one array element (by value) | canonical-value hash vs lock |

The canonical-value hash is taken over the value in a fixed form, with object keys sorted. So
reformatting the file or reordering its keys is not drift.

### Blocks

Blocks sit in a deterministic order. User text outside blocks is never touched. Claude Code strips
HTML comments before it adds a file to the model's context, so the markers, which are HTML
comments, cost no context there ([`CC-MEM`](../specs.md#cc-mem-memory-and-rules)).

Built in M2.3 as described here: `block::Document` splits a file into the user's text and
Grimoire's blocks, `Document::set` adds or replaces a block, `Document::remove` removes one, and
`Document::render` puts the file back together with the user's text unchanged.

### Entries

Entries use format-preserving editing: `toml_edit` for TOML, and a comment- and order-preserving
JSON/JSONC editor for JSON and JSONC, JSON with comments (chosen in Phase 2).

- Unrelated keys and the user's formatting are preserved.
- Property tests (tests that check a rule against many generated inputs) cover adding an entry,
  removing it again, and round-trips with arbitrary surrounding content.

### Removal

Removal deletes only owned units.

- Grimoire deletes a shared file it created only when the file is now empty of anything Grimoire
  didn't write. Otherwise it removes its blocks or entries and leaves the file.
- Adopting content that already matches, or overwriting it with `--force`, makes it Grimoire's. A
  whole file in a managed folder is then deleted with its asset. A shared file that held only
  Grimoire's content when it was adopted is recorded as created. (Checked against `plan.rs` in
  Phase 7.)
- Directories Grimoire created are removed if empty.
- Phase 2 implementation:
  - Creation is recorded in the lock (`created-file`, `created-keys`;
    [`model.md`](model.md#grimoirelock)).
  - After deleting a file, Grimoire removes the directories above it that are now empty, whether
    or not it created them. Creating directories is not recorded, and an empty directory holds no
    content (Git does not track one).

### Entry keys

Entry keys (Phase 2) are JSON Pointers, defined in RFC 6901 (an internet standard). A JSON Pointer
is a path such as `/mcpServers/<id>` that names one value inside a JSON document.

- `/mcpServers/<id>` owns that member's value.
- `/hooks/PreToolUse|<id>#<n>` owns one element of that array, found by the hash of its value.
  The text after `|` only keeps keys distinct.

The adapters build these keys with `entry::member_key` and `entry::element_key`, and
`entry::value_hash` hashes a value regardless of formatting and key order.

### JSON editing

JSON editing (Phase 2, which settled the open question of how to edit JSON and JSONC) uses
`jsonc-parser`'s concrete syntax tree (CST), a parse tree that keeps every character of the file,
comments and whitespace included
([`EDIT-CRATES`](../specs.md#edit-crates-jsonc-parser-toml_edit-similar)). So no ADR is needed.

- Unrelated keys, values and order are always preserved, as is all formatting outside the
  container Grimoire edits.
- Inserting into a container written on a single line reflows that container across lines.
  Removing the entry later restores the values, not the single line.
- Grimoire edits only **strict JSON**. Claude Code reads its settings as strict JSON, and a file a
  client can't parse should not be "fixed" silently. Comments, trailing commas, or duplicate keys
  on Grimoire's path make the file unreadable for Grimoire
  ([Drift and conflicts](#drift-and-conflicts)).

### Three entry formats, chosen by path

Phase 4 added TOML and JSONC entries ([`EDIT-P4`](../specs.md#edit-p4-toml-and-jsonc-entry-editing)).
The format comes from the path, not the content, because removing a locked entry has no desired
content to say which format the file is. `EntryFormat::for_path` makes the choice, and the
planner, `apply` and `verify` all use it.

- `.toml` files (`.codex/config.toml`): TOML through `toml_edit`.
  - Only whole member keys such as `/mcp_servers/<id>`, written as a standard table. A plain value
    that would add a `[parent]` header is refused.
  - The hash is of the value converted to JSON, so drift detection works the same way.
  - A file Grimoire doesn't change keeps its bytes. A changed one keeps its byte-order mark (an
    optional marker at the start of a text file) and its CRLF (Windows-style) line endings.
  - Comments above a removed table move to the next header or the end of the file. An empty table
    Grimoire created stays if the user gave it a header or comments.
- `.gemini/settings.json`: JSONC. Comments are allowed and kept; everything else is as strict as
  JSON, which is what Gemini CLI accepts.
- Values are hashed without their comments, so a comment the user adds *inside* Grimoire's entry
  in a TOML or JSONC file would be lost by an update or removal. Planning that update or removal
  is a conflict instead (`commented`; `--force` drops the comment).
- Every other entry file: strict JSON, as above.

Built in M4.1 as described here:

```rust
pub enum EntryFormat {
    Json,  // strict JSON: every entry file but the two below
    Jsonc, // `.gemini/settings.json`: comments allowed and kept
    Toml,  // `.toml` files, such as `.codex/config.toml`: whole member keys only
}
```

## Drift and conflicts

For each output, the planner compares three hashes: on-disk (D), locked (L) and desired (N).

| Situation | Action |
|---|---|
| D = L, N = L | unchanged |
| D = L, N ≠ L | update |
| D = N | already applied: record it, no write |
| D ≠ L, D ≠ N | **conflict**: the user edited managed content. Never overwritten silently. |
| owned unit missing on disk | **conflict** (deleted by the user) |
| destination exists but is not in the lock | **conflict** (an unmanaged file is in the way) |

A conflict is reported with a diff. The user then chooses:

- `--force` overwrites.
- `--write-new` writes the would-be file as `<path>.new` and leaves the original alone. For blocks
  and entries, `<path>.new` is the whole file as it would be written.

Phase 2 details:

- `apply --write-new` exits 4, because the conflicts remain. The lock keeps the old records for
  those files, so the conflict is reported again until the user merges the `.new` file or uses
  `--force`.
- **A file Grimoire cannot edit safely is refused, even with `--force`** (exit 4). Grimoire never
  guesses what it owns. That covers:
  - malformed or unterminated block markers;
  - non-UTF-8 text;
  - invalid or non-strict JSON;
  - a JSON path that is the wrong type;
  - duplicate keys on Grimoire's path.

  The planner marks such a file `FilePlan::unreadable`, so a `block::BlockError` or an
  `entry::EntryError` never reaches the caller; `apply::verify` reports each output in it as a
  mismatch.
- `apply` re-reads each file immediately before writing it, and aborts if it changed after
  planning (exit 4, `ApplyError::Changed`). A rerun converges.
  - The files already written by then are recorded in the lock, so their creation records
    ([`model.md`](model.md#grimoirelock)) are not lost. `Stopped::report` lists them, and
    `apply::not_done` the files that keep their old lock records.
  - The message says that some files were written.
- **Array elements are found by their exact value** (hook bindings). When a locked element can't
  be found, it is a conflict (`NotFound`), whether the entry is still wanted or not. An edited
  element and a removed one look the same, so Grimoire never forgets one silently.
  - With `--force`, a still-wanted element is written again, and any edited copy is left as the
    user's.
  - Two keys never claim the same element, and identical desired elements are refused.
- **`--write-new` never replaces a different `<path>.new`** (`ApplyError::SidecarExists`). An
  identical one from an earlier run is left as it is. Sidecars (the `.new` files) are checked
  before anything is written, and get the original file's permissions.
- **Something in the way** is a conflict (exit 4), not a safety refusal: a file where a directory
  is needed (`.claude` as a file), or a directory where a file is needed. It is
  `FsError::InTheWay`, not `FsError::Refused`.
- **A case-only rename between versions** is refused (exit 3, `plan/case-collision`), for example
  a lock recording `Guide.md` while the catalog now writes `guide.md`. On macOS and Windows both
  names are one file, and deleting the old name would delete the new one. Two desired paths that
  differ only in case are refused the same way.

The planner also refuses desired outputs that contradict each other, with exit 3:
`plan/duplicate-output` (one unit written twice, or two identical array elements) and
`plan/mixed-units` (a whole file and blocks or entries at one path). Every code is in
[`../diagnostics.md`](../diagnostics.md#planning-plan), and what users see is in
[`../configuration.md`](../configuration.md#6-drift-and-conflicts).

## Filesystem safety

- **Paths:** every destination is a normalized **ASCII** relative path under the project root
  (`RelPath`, defined in [`model.md`](model.md)).
  - ASCII-only rules out invisible characters, bidi characters (which change the direction text is
    displayed in), superscript device names (Windows treats a name such as `COM¹` as a device) and
    Unicode-normalization collisions (two different strings that some file systems store as one
    name).
  - These are rejected: absolute paths, `..`, drive and UNC prefixes (`C:`, `\\server\share`),
    NUL bytes, Windows reserved names (such as `CON` and `NUL`) and alternate data streams (`:`,
    as in `file.txt:stream`), and names that collide case-insensitively within one plan.
  - The catalog validator applies the same rules to every catalog file (Phase 1;
    [`catalog.md`](catalog.md#catalog-validation)).
- **Confinement:** all filesystem access goes through a capability handle on the project root,
  from the `cap-std` crate. That is an open handle to the root directory, and every path is
  resolved inside it. The handle refuses escapes, including through symlinks.
  - Every directory on the way to a managed path is opened no-follow (a symlink is refused, not
    followed), one component at a time.
  - ADR-0006 chose cap-std over checks with the standard library to close a race. With path-based
    checks, someone who can write into the project could swap a directory for a symlink between
    the check and the write, and so redirect the write outside the root.
  - cap-std needed scoped entries (a license exception, duplicate versions and prebuilt files) in
    `deny.toml`, the policy that cargo-deny checks every dependency against. ADR-0006 justifies
    each entry ([`../02-dependency-policy.md`](../02-dependency-policy.md#the-rule-for-third-party-code)).
  - The threats this defends against, F1 to F19, are in
    [`../01-architecture.md`](../01-architecture.md#threats-and-mitigations).
- **Symlinks:** Grimoire never writes through a symlink at a managed path. It refuses (exit 6)
  rather than follow one. That holds even when the link points inside the project, because
  writing through it would put content somewhere the user didn't expect.
- **Special files:** a named pipe, a device or a socket at the path is refused. The file that was
  actually opened is checked again, so one swapped in after the check is refused as well.
- **Reads are capped** at `fs::MAX_READ_BYTES` (8 MiB), checked before and while reading, so a
  huge file can't exhaust memory.
- **Atomic writes:** each file is written to a temporary file in the same directory, flushed to
  disk (fsync), then renamed over the target. An existing file's permissions are set on the
  temporary file before any content is written to it. Behavior differences on Windows are covered
  by tests, and listed in the crate docs' "Platforms" section.
- **No hooks, no execution, no network:** installing never executes asset content and runs no
  install hooks. The install path has no network code at all in v1 (the builtin source;
  [ADR-0001](../adr/0001-catalog-delivery.md), which embeds the catalog in the binary).

Built in M2.3 as described here: `fs::Root` is the only way to the project's files, through
`Root::read`, `Root::write`, `Root::write_like`, `Root::remove`, `Root::exists` and
`Root::has_file`. A refusal is `FsError::Refused` (exit 6), something in the way is
`FsError::InTheWay` (exit 4), and any other failure is `FsError::Io` (exit 1). `Root::exists` and
`Root::has_file` were added after Phase 7, so that `init`, `doctor` and `uninstall --purge` check
files without following a link.

What confinement doesn't cover yet: a temporary file left by a crash isn't removed automatically,
and two `grimoire` processes in one project aren't locked against each other
([`../01-architecture.md`](../01-architecture.md#known-gaps-and-residual-risks)).

## Invariants

1. Every read and write reaches only paths inside the project root, and follows no symbolic link
   at the path or at any directory above it; such a link is refused with exit 6.
2. A path from a tampered `grimoire.lock` that names anything outside the root, or a Windows
   device name, is refused before anything is read or removed (exit 3).
3. A named pipe at a managed path is refused without the read blocking.
4. Something in the way of a write is a conflict (exit 4), not a safety refusal.
5. A second `apply` plans only `unchanged` units and writes nothing, not even the lock.
6. The planner's action for each unit follows the drift table above, for every combination of the
   three hashes.
7. Content the user edited or deleted, and unmanaged content at a wanted path, is never
   overwritten or removed without `--force`.
8. After an interrupted `apply`, a rerun adopts content that already matches instead of reporting
   drift, and a run that stops partway records in the lock every file it wrote.
9. Removal deletes only what Grimoire owns: user text around a block, user keys around an entry,
   and files Grimoire didn't create stay.
10. A file Grimoire can't edit safely is refused, even with `--force`.
11. Reformatting a file, reordering its keys, or converting its line endings to CRLF is not drift.
12. A TOML or JSONC entry round-trips: adding it and removing it again restores the rest of the
    file, and a TOML file Grimoire doesn't change keeps its bytes.
13. A comment the user adds inside a TOML or JSONC entry makes updating or removing it a conflict.
14. An edited or missing hook element is a conflict, never forgotten silently.
15. `--write-new` never replaces a `<path>.new` that holds something else, and a sidecar or
    replaced file is no more readable than the original.
16. Two paths that differ only in letter case are refused before any write, including a locked
    path that a new catalog version renames only in case.
17. A file that changes between planning and its write stops `apply` with exit 4.
18. No read returns more than `fs::MAX_READ_BYTES`.

## Tests

| Invariant | Tests |
|---|---|
| 1 | `symlinks_are_refused` and `client_detection_follows_no_symbolic_link` in `crates/agent-grimoire/tests/cli.rs`; `exists_follows_no_symbolic_link` in `src/fs.rs`. All three are Unix-only |
| 2 | `tampered_locks_cannot_reach_outside_the_project` and `locks_naming_unmanaged_paths_are_refused` in `crates/agent-grimoire/tests/cli.rs`; `rejects_unsafe_paths` in `crates/agent-grimoire-model/src/path.rs`, and the proptests `accepted_paths_are_confined` and `traversal_is_always_rejected` in `crates/agent-grimoire-model/tests/properties.rs` |
| 3 | `fifos_are_refused_without_blocking` in `src/fs.rs` (Unix-only) |
| 4 | `things_in_the_way_are_conflicts_not_refusals` in `src/fs.rs`; `things_in_the_way_are_conflicts` in `cli.rs` |
| 5 | `second_apply_changes_nothing` in `cli.rs` |
| 6 | `the_drift_matrix` in `src/plan.rs` |
| 7 | `edited_managed_content_is_never_overwritten_silently` and `deleted_managed_content_and_unmanaged_files_conflict` in `cli.rs`; `edits_to_grimoire_entries_are_conflicts` in `crates/agent-grimoire/tests/clients.rs` |
| 8 | `identical_existing_content_is_adopted_after_an_interrupted_apply` in `cli.rs`; `a_stopped_apply_reports_what_it_wrote` in `src/apply.rs` |
| 9 | `remove_and_uninstall_leave_only_user_content`, `uninstall_from_scratch_leaves_nothing`, `files_grimoire_did_not_create_are_never_deleted` and `user_text_added_to_a_grimoire_file_keeps_it` in `cli.rs`; the proptests `add_then_remove_restores_the_text` and `blocks_are_independent` in `tests/block_props.rs`, and `a_file_grimoire_created_goes_away` and `user_content_added_later_survives_removal` in `tests/entry_props.rs`; `only_listed_empty_containers_are_pruned` in `src/entry.rs` |
| 10 | `broken_shared_files_are_refused_even_with_force` and `update_no_apply_refuses_unreadable_files` in `cli.rs`; `malformed_markers_are_refused` and `refuses_to_append_inside_an_open_fence` in `src/block.rs`; `refuses_what_a_strict_reader_would_refuse` in `src/entry.rs`; `refuses_files_it_cannot_edit_safely` in `src/entry/toml.rs` |
| 11 | `crlf_checkouts_are_not_drift` in `cli.rs`; `hashes_ignore_order_and_formatting` in `src/entry.rs`; `text_ignores_crlf_but_binary_does_not` in `src/hash.rs`; `crlf_files_get_crlf_blocks` in `src/block.rs` |
| 12 | the proptests `member_round_trip` and `element_round_trip` in `tests/entry_props.rs` and `member_round_trip` in `tests/toml_props.rs`; `codex_toml_entry_is_added_kept_and_removed`, `crlf_codex_configs_are_not_rewritten` and `gemini_jsonc_entry_keeps_comments` in `tests/entry_formats.rs`; `untouched_files_keep_their_bytes` and `edited_files_keep_their_line_endings_and_bom` in `src/entry/toml.rs` |
| 13 | `comments_inside_an_entry_are_conflicts` in `tests/entry_formats.rs`; `comments_inside_an_entry_are_reported` in `src/entry/toml.rs` |
| 14 | `an_edited_hook_is_a_conflict_not_forgotten` in `cli.rs`; `elements_are_found_by_value` in `src/entry.rs` |
| 15 | `write_new_never_replaces_a_different_sidecar` in `cli.rs`; `copies_take_the_original_permissions` and `keeps_existing_permissions` in `src/fs.rs` (Unix-only) |
| 16 | No test yet. The check is in `plan::plan` (`plan/case-collision`). The catalog's own check, `case_insensitive_collisions_are_rejected` in `crates/agent-grimoire-catalog/tests/catalog.rs`, covers catalog files, not the planner |
| 17 | No test yet. The check is `ApplyError::Changed` in `src/apply.rs`, reported as `apply/changed` |
| 18 | No test yet. The cap is `fs::MAX_READ_BYTES`, checked three times in `Root::read` |

The format of each path is covered by `formats_follow_the_path` in `src/entry.rs`, and
`only_the_gemini_path_accepts_comments` and `apply_and_verify_use_the_format_of_the_path` in
`tests/entry_formats.rs`. The Unix-only tests don't run on Windows, so symbolic-link and
special-file handling there is covered only by cap-std's own behavior
([`../03-platforms.md`](../03-platforms.md#invariants-on-every-platform)).
