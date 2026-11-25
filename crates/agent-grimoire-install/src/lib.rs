//! Root-confined, atomic installation of Grimoire outputs: planning what to
//! change in a project, applying the plan, and checking the disk against
//! `grimoire.lock`, the record of what was installed.
//!
//! An output is a unit of content Grimoire owns in a project: a whole file, a
//! managed block in a shared Markdown file such as `AGENTS.md`, or an entry in
//! a configuration file such as `.mcp.json`. *Root-confined* means that every
//! read and write stays inside the project root, the directory that holds
//! `grimoire.toml`, even if the directory tree changes during the operation.
//! *Atomic* means that each file is either replaced whole or left as it was.
//!
//! # Where it fits
//!
//! The `grimoire` command-line interface (CLI), in the `agent-grimoire` crate,
//! opens the project root as an [`fs::Root`] and reads and writes every
//! project file through it, `grimoire.toml` and `grimoire.lock` included. Its
//! commands that plan share one pipeline:
//!
//! 1. `agent-grimoire-resolve` resolves the selected assets.
//! 2. `agent-grimoire-adapters` renders them into [`plan::Desired`] outputs
//!    for the selected clients (a client is an AI coding tool, such as Claude
//!    Code). The adapters build their entry keys with [`entry::member_key`]
//!    and [`entry::element_key`].
//! 3. This crate plans ([`plan::plan`]).
//! 4. This crate applies the plan ([`apply::apply`]) and builds the next lock
//!    ([`apply::next_lock`]), which the CLI writes last.
//!
//! `apply`, `update`, and `add` and `remove` with `--apply` run every step,
//! except that `update --no-apply` doesn't apply and writes only the lock.
//! `plan` and `doctor` stop after step 3. `uninstall` skips steps 1 and 2: it
//! plans with no desired outputs, which removes everything the lock records.
//! `verify` and `doctor` check the disk against the lock with
//! [`apply::verify`].
//!
//! The crate builds on `agent-grimoire-model` for checked paths
//! ([`RelPath`](agent_grimoire_model::RelPath)), the lock's types
//! ([`Lockfile`](agent_grimoire_model::Lockfile),
//! [`LockedOutput`](agent_grimoire_model::LockedOutput)), entry keys and
//! hashes; on cap-std, cap-fs-ext and cap-tempfile for file access that can't
//! leave a directory; on jsonc-parser and `serde_json` for JSON and JSONC; and
//! on `toml_edit` for TOML.
//!
//! It is internal to the CLI. It is published only so that
//! `cargo install agent-grimoire` can build the CLI, and its API changes
//! whenever the CLI needs it to, so don't depend on it
//! ([versioning policy](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#the-public-api)).
//! The design is in
//! [`docs/subsystems/install.md`, Install model](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md),
//! and the threats it defends against are in
//! [`docs/01-architecture.md`, Threats and mitigations](https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#threats-and-mitigations).
//!
//! # The parts
//!
//! - **Files**, in [`fs`]: root-confined, atomic file access that refuses
//!   symbolic links. [`fs::Root`] is the project root and the only way to its
//!   files: [`Root::read`](fs::Root::read), [`Root::write`](fs::Root::write),
//!   [`Root::write_like`](fs::Root::write_like),
//!   [`Root::remove`](fs::Root::remove), [`Root::exists`](fs::Root::exists)
//!   and [`Root::has_file`](fs::Root::has_file). Failures are
//!   [`fs::FsError`]s.
//! - **Managed blocks**, in [`block`]: managed blocks in shared Markdown
//!   files. [`block::Document`] splits a file into the user's text and
//!   Grimoire's blocks, each between a `<!-- grimoire:begin NAME -->` line and
//!   a `<!-- grimoire:end NAME -->` line.
//!   [`Document::set`](block::Document::set) adds or replaces a block,
//!   [`Document::remove`](block::Document::remove) removes one, and
//!   [`Document::render`](block::Document::render) puts the file back
//!   together with the user's text unchanged. [`block::BlockError`] says why a
//!   file's blocks can't be read.
//! - **Configuration entries**, in [`entry`]: owned entries in JSON, JSONC
//!   (JSON with comments) and TOML config files. An entry key names what
//!   Grimoire owns. For an object member it is a JSON Pointer (defined by
//!   RFC 6901, an internet standard): a path to one value in a JSON document,
//!   such as `/mcpServers/docs-index`, built by [`entry::member_key`]. For an
//!   array element it is the array's pointer, then `|` and a label, such as
//!   `/hooks/PreToolUse|secret-guard#0`, built by [`entry::element_key`]:
//!   elements have no names, so the element is found by the hash of its
//!   value, and the label only keeps keys distinct. A TOML file can own only
//!   object members.
//!   [`EntryFormat::for_path`](entry::EntryFormat::for_path) chooses a file's
//!   syntax from its path, and [`entry::value_hash`] hashes a value
//!   regardless of formatting and key order. The editor itself is internal:
//!   the planner and [`apply::verify`] use it.
//! - **Planning**, in [`plan`]: the pure planner that compares desired
//!   outputs, the lock and disk. [`plan::paths`] lists every path a plan
//!   needs to read, and [`plan::plan`] turns [`plan::Desired`] outputs, the
//!   lock and those files' bytes into a [`plan::Plan`]: one
//!   [`plan::FilePlan`] per file, with its content before and after, and one
//!   [`plan::UnitPlan`] per owned unit, with its [`plan::Action`].
//! - **Applying and checking**, in [`apply`]: applying a plan, building the
//!   next lock, and verifying a lock. [`apply::apply`] makes the changes, as
//!   [`apply::Options`] allow, and returns an [`apply::Report`];
//!   [`apply::next_lock`] builds the lock; [`apply::verify`] lists each
//!   locked output whose content on disk differs, as an [`apply::Mismatch`].
//! - **Hashes**, in [`hash`]: [`hash::content`], the hash of file and block
//!   content that the lock records.
//!
//! # Example
//!
//! The CLI's pipeline, from desired outputs to a checked lock, for the block
//! Grimoire adds to `CLAUDE.md` so that Claude Code reads `AGENTS.md`.
//! Planning again afterward finds nothing to do:
//!
//! ```
//! use std::collections::{BTreeMap, BTreeSet};
//! use std::error::Error;
//!
//! use agent_grimoire_install::apply::{self, Options};
//! use agent_grimoire_install::fs::Root;
//! use agent_grimoire_install::plan::{self, Action, Content, Desired, Owner, Plan};
//! use agent_grimoire_model::{ClientId, Lockfile, RelPath};
//!
//! /// Reads every path the plan involves through the root, as the CLI does,
//! /// then plans. Planning itself reads nothing.
//! fn plan_project(
//!     root: &Root,
//!     desired: Vec<Desired>,
//!     lock: &Lockfile,
//! ) -> Result<Plan, Box<dyn Error>> {
//!     let mut current = BTreeMap::new();
//!     for path in plan::paths(&desired, lock) {
//!         let bytes = root.read(&path)?;
//!         current.insert(path, bytes);
//!     }
//!     // Outputs that contradict each other come back as diagnostics.
//!     Ok(plan::plan(desired, lock, &current).map_err(|d| format!("{d:?}"))?)
//! }
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
//! # let project = tempfile::tempdir()?;
//! # std::fs::write(project.path().join("CLAUDE.md"), "# Notes\n")?;
//! // `project` is a directory whose CLAUDE.md holds only the user's text.
//! let root = Root::open(project.path())?;
//! let desired = vec![Desired {
//!     owner: Owner::Project,
//!     clients: BTreeSet::from([ClientId::ClaudeCode]),
//!     path: RelPath::new("CLAUDE.md")?,
//!     content: Content::Block("@AGENTS.md\n".to_owned()),
//! }];
//!
//! // There is no grimoire.lock yet, so the block is new.
//! let lock = Lockfile::new();
//! let first = plan_project(&root, desired.clone(), &lock)?;
//! assert_eq!(first.files[0].units[0].action, Action::Create);
//!
//! let report = apply::apply(&root, &first, Options::default()).map_err(|s| s.error)?;
//! let lock = apply::next_lock(&lock, Vec::new(), &first, &report.deferred());
//! assert!(apply::verify(&root, &lock)?.is_empty());
//!
//! // Against the new lock, every unit is unchanged and nothing would be written.
//! let second = plan_project(&root, desired, &lock)?;
//! assert_eq!(second.files[0].units[0].action, Action::Unchanged);
//! assert!(!second.has_changes());
//! # Ok(())
//! # }
//! ```
//!
//! # Why every file goes through `Root`
//!
//! Grimoire treats every file in a user's project as hostile input, and the
//! directory tree as something that may change while it works
//! ([trust boundaries](https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#trust-boundaries)).
//! So [`fs::Root`] is the only way this crate, and the CLI, reach the
//! project, and it makes these checks:
//!
//! - **The path is checked text.** Every path is a
//!   [`RelPath`](agent_grimoire_model::RelPath): ASCII, relative and
//!   `/`-separated, with no `..` and no absolute or drive prefix, so the
//!   string itself can't name anything outside the root.
//! - **Every access starts from a handle on the root.** cap-std, a Rust
//!   library whose directory handles reach only the paths beneath them,
//!   resolves each path from an open handle on the root, so a path can't
//!   leave the root even if the tree changes underneath.
//!   [ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md)
//!   chose it over checking each directory with the standard library and
//!   then writing by path. That leaves a time-of-check to time-of-use
//!   (TOCTOU) race: someone who can write into the project could swap a
//!   directory for a symbolic link between the check and the write, and so
//!   redirect the write outside the root.
//! - **Symbolic links are refused.** A link at the path, or at any directory
//!   between the root and it, is refused even when it points inside the
//!   project, because writing through it would put content somewhere the user
//!   didn't expect. Each directory is opened no-follow, one component at a
//!   time, so a link that appears after the check is refused too.
//! - **Special files are refused.** A named pipe, a device or a socket at the
//!   path is refused. The file that was actually opened is checked
//!   again, so one swapped in after the check is refused as well.
//! - **Reads are capped** at [`fs::MAX_READ_BYTES`] (8 MiB), checked before
//!   and while reading, so a huge file can't exhaust memory.
//!
//! A refusal is [`FsError::Refused`](fs::FsError::Refused). Something in the
//! way, a file where a directory is needed or a directory where a file is
//! needed, is [`FsError::InTheWay`](fs::FsError::InTheWay) instead: a
//! conflict the user can resolve, not a safety refusal.
//!
//! # Atomic writes, and what an interruption leaves
//!
//! [`Root::write`](fs::Root::write) writes to a temporary file in the
//! destination directory, syncs it (flushes it to the disk), and renames it
//! over the target,
//! so a crash never leaves a half-written file. An existing file's
//! permissions are set on the temporary file before any content is written to
//! it, so the file never holds the content while its permissions are looser
//! than the original's. (Where the temporary file has a name from the start,
//! that is everywhere but Linux with `O_TMPFILE`, a process that opened it in
//! the moment before its permissions were set could still read the content
//! through that handle.)
//! [`Root::write_like`](fs::Root::write_like) gives a new file the
//! permissions of another one, so that a `<path>.new` copy of a restricted
//! file is no more readable than the original.
//! [`Root::remove`](fs::Root::remove) deletes a file and then every directory
//! above it that is left empty, deepest first, including one the user created
//! empty: an empty directory holds no content, and Git doesn't track one.
//!
//! When [`apply::apply`] stops partway with an error,
//! [`Stopped::report`](apply::Stopped::report) lists the files it already
//! changed, and [`apply::not_done`] the files that keep their old lock
//! records, so the lock the CLI then writes still records what was written.
//! When the process is killed instead, no lock is written. On the next run, a
//! unit whose content on disk already equals the desired content counts as
//! applied ([`Action::Adopt`](plan::Action::Adopt)), so the run converges on
//! the desired state instead of reporting drift, managed content on disk that
//! differs from what the lock records
//! ([`docs/subsystems/install.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#desired-state-plan-apply)).
//! A temporary file can still be left behind by a crash, and nothing removes
//! it automatically
//! ([`docs/01-architecture.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#known-gaps-and-residual-risks)).
//! Two `grimoire` processes in one project at once are not locked against
//! each other either.
//!
//! # Conflicts: what is never overwritten silently
//!
//! For each unit, [`plan::plan`] compares three hashes: of the content on
//! disk, the one the lock records, and of the desired content. The table in
//! [`plan`] gives the action for each case. Content the user edited or removed since Grimoire wrote it, and
//! content Grimoire doesn't manage at a path it wants, are conflicts
//! ([`plan::Conflict`]).
//!
//! [`apply::apply`] checks every precondition before it writes anything. If a
//! unit conflicts and neither [`Options::force`](apply::Options::force) nor
//! [`Options::write_new`](apply::Options::write_new) is set, it returns
//! [`ApplyError::Conflicts`](apply::ApplyError::Conflicts) and writes nothing.
//! With `write_new`, each conflicting file that would still exist gets its
//! would-be content in `<path>.new` beside it ([`apply::new_path`]), and the
//! file itself is left alone. A conflicting file that would be deleted is only
//! left alone ([`FileOutcome::Deferred`](apply::FileOutcome::Deferred)). A
//! `<path>.new` that already holds something else is never replaced
//! ([`ApplyError::SidecarExists`](apply::ApplyError::SidecarExists)).
//!
//! A file Grimoire can't edit safely, such as one with broken block markers,
//! invalid JSON or text that isn't UTF-8, is marked
//! [`FilePlan::unreadable`](plan::FilePlan::unreadable) and is refused even
//! with `force`: Grimoire never guesses what it owns.
//!
//! Just before writing each file, `apply` reads it again, and stops with
//! [`ApplyError::Changed`](apply::ApplyError::Changed) if it differs from what
//! was planned.
//!
//! # Errors and how the CLI reports them
//!
//! - [`plan::plan`] returns diagnostics when the desired outputs contradict
//!   each other: `plan/duplicate-output` (one unit written twice, or two
//!   identical array elements), `plan/mixed-units` (a whole file and blocks or
//!   entries at one path) and `plan/case-collision` (two paths that differ
//!   only in letter case, which macOS and Windows treat as one file). The CLI
//!   exits with 3 for them.
//! - The CLI exits with 6 for [`FsError::Refused`](fs::FsError::Refused), with
//!   4, a conflict, for [`FsError::InTheWay`](fs::FsError::InTheWay), and with
//!   1 for [`FsError::Io`](fs::FsError::Io).
//! - Every [`apply::ApplyError`] but [`ApplyError::Fs`](apply::ApplyError::Fs)
//!   is a conflict (exit 4). `Fs` wraps an [`fs::FsError`].
//! - A [`block::BlockError`] or an [`entry::EntryError`] never reaches the
//!   caller of [`plan::plan`]: the planner marks the file unreadable instead,
//!   and [`apply::verify`] reports each output in such a file as an
//!   [`apply::Mismatch`].
//!
//! # Platforms
//!
//! The same code runs on Linux, macOS and Windows. These are the differences:
//!
//! - cap-std resolves paths with `openat2(RESOLVE_BENEATH)` on Linux 5.6 and
//!   later, a system call that refuses to resolve a path outside the directory
//!   it starts from. Where `openat2` is unavailable (an older kernel, or a
//!   sandbox that blocks it), and on other systems, it uses its own resolver,
//!   which opens one component at a time from the root's handle. On Windows,
//!   it does the last step of creating a missing directory, of a write (the
//!   rename over the target) and of a removal by full path, after resolving
//!   the parent directories inside the root. Confinement still holds, but that
//!   last step isn't relative to an open directory handle
//!   ([ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md)
//!   records this).
//! - On Unix, a file is opened for reading with `O_NONBLOCK`, so a named pipe
//!   swapped in after the check can't make the read wait forever.
//! - On Unix, the directory is synced after a rename or a removal, as a best
//!   effort, so the change reaches the disk sooner. Windows has no directory
//!   sync, so the step is skipped there. The rename is atomic either way.
//! - Permissions are copied as cap-std reads them: the mode bits on Unix, such
//!   as `0600`, and only the read-only attribute on Windows. Windows access
//!   control lists (ACLs) and Unix ownership are not copied.
//! - On Linux, cap-tempfile first tries an unnamed temporary file
//!   (`O_TMPFILE`), which gets a name only just before the rename, so a crash
//!   rarely leaves one behind. Elsewhere the temporary file has a random name
//!   from the start.
//! - [`Root::remove`](fs::Root::remove) closes its handle on the file's
//!   directory before removing the directories left empty. On Windows, cap-std
//!   opens directories without `FILE_SHARE_DELETE`, so an open handle would
//!   make removing that directory fail.
//! - [`hash::content`] folds `\r\n` line endings to `\n` everywhere, so a
//!   checkout that converts line endings (Git's `core.autocrlf` on Windows)
//!   is not drift. A Markdown or TOML file that uses `\r\n` keeps it in the
//!   lines Grimoire writes.

pub mod apply;
pub mod block;
pub mod entry;
pub mod fs;
pub mod hash;
pub mod plan;

// The README's examples are compiled and run as doc tests too, so the
// crates.io page breaks the build when the API changes under it.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;
