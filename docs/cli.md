# The `grimoire` command line

- **Status:** written in Phase 7 (2026-11-17), before the first release, for `grimoire` 0.1.0. The unit test in `crates/agent-grimoire/src/docs_check.rs` fails when a command, a flag or argument in an options table, or a code in the Exit codes table is in the code but not in this file, or the other way round. It also fails when the command sections aren't in the order `grimoire --help` lists them. It doesn't check the synopses, the per-command exit codes, the value names after a flag, the values an option allows, or the JSON keys.
- **Design:** [`architecture.md`](architecture.md) §7 (the install model) and §9 (the CLI).
- **Formats:** [`configuration.md`](configuration.md) describes `grimoire.toml`, `grimoire.lock`, permissions and clients. [`diagnostics.md`](diagnostics.md) lists every diagnostic code the CLI prints.

`grimoire` installs the AI-agent assets you choose (rules, skills, agents, tools and MCP servers) into a repository, for the AI clients you use. You list what you want in `grimoire.toml`. `grimoire apply` writes the files each client reads and records them in `grimoire.lock`. It changes only content it wrote, and stops rather than overwrite content you edited.

The catalog is built into the binary ([ADR-0001](adr/0001-catalog-delivery.md)): a newer catalog comes with a newer `grimoire`. Installing never runs asset content and makes no network connection.

A first session:

```sh
grimoire init --client claude-code    # write grimoire.toml
grimoire list                         # browse the catalog
grimoire show conventional-commits    # review its permissions and outputs
grimoire add conventional-commits     # select it in grimoire.toml
grimoire plan                         # see what apply would change
grimoire apply                        # write the files and grimoire.lock
grimoire verify                       # later, or in CI: check nothing drifted
```

## Conventions

- **Streams.** Human output goes to stdout. Diagnostics, notes and prompts go to stderr.
  - A diagnostic reads `error[<code>]: <message>` or `warning[<code>]: <message>`. When it is about a file, the file follows the code: `error[verify/mismatch] .claude/rules/conventional-commits.md: …`.
  - Codes are stable, and [`diagnostics.md`](diagnostics.md) lists them. Control characters in a diagnostic are escaped on stderr.
  - Output has no colour codes.
- **`--json`.** Every command prints exactly one JSON document on stdout instead, and writes nothing to stderr ([JSON output](#json-output)). `--help` and `--version` still print plain text.
- **Prompts.** Only `add` asks a question: whether to accept the permissions an asset needs. It asks only when stdin is a terminal, `CI` is unset or empty, and neither `--no-input` nor `--json` is given.
  - `y`, `Y`, `yes`, `Yes` or `YES` accepts, and spaces around the answer are ignored. Any other answer refuses.
  - Without a prompt the answer is no. `add` then stops, and its error names the `--allow` value that accepts the permissions.
- **The project root** is `-C <DIR>` or, without it, the current directory. Grimoire doesn't look for `grimoire.toml` in parent directories. `list` and `show` read only the catalog, so they ignore `-C`.
- **Global options go before or after the command:** `grimoire -C repo plan` and `grimoire plan -C repo` are the same.
- **Grimoire writes only inside the project root**, and never through a symbolic link: a symbolic link at a managed path, or on the way to one, is refused (exit 6). [`security-model.md`](security-model.md) has the threat model.

## Global options

| Option | Meaning |
|---|---|
| `--json` | Print exactly one JSON document on stdout instead of human output ([JSON output](#json-output)). It also turns prompts off. |
| `--no-input` | Never prompt. Implied when stdin is not a terminal, when `CI` is set to a non-empty value, and by `--json`. |
| `-C <DIR>` | Run as if started in DIR, the project root. A relative DIR is taken from the current directory. A DIR that doesn't exist is an I/O error (exit 1), except for `list` and `show`, which ignore it. |
| `-h`, `--help` | Print help on stdout and exit 0. Every command takes it, as in `grimoire plan --help`. `grimoire help [COMMAND]` prints the same. |
| `-V`, `--version` | Print `grimoire <version>` on stdout and exit 0. It works only before a command: `grimoire --version`. |

A bare `grimoire`, with no arguments, prints the help on stdout and exits 2.

Each command's section below gives its JSON `result`. Results have no JSON Schema, and a release may add keys to them.

## grimoire init

```text
grimoire init [--client <CLIENT>]...
```

Creates `grimoire.toml` in the project root for the clients you give, or for the clients the project already uses. The new file holds two comment lines, `format = 1`, the `clients` list and an empty `[assets]` table. `init` never overwrites an existing `grimoire.toml`.

Without `--client`, `init` detects each client from files or folders that are already there. A symbolic link counts. If it finds none, it stops with a usage error that lists the clients.

| Client | Detected from |
|---|---|
| `claude-code` | `CLAUDE.md`, `.claude` |
| `agents-md` | `AGENTS.md`, `.agents` |
| `codex` | `.codex` |
| `cursor` | `.cursor` |
| `gemini-cli` | `GEMINI.md`, `.gemini` |
| `copilot` | `.github/copilot-instructions.md`, `.github/instructions`, `.github/agents` |

| Argument or option | Meaning |
|---|---|
| `--client <CLIENT>` | A client to render for. Repeat it for more than one. One of `claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli` or `copilot`. With it, nothing is detected. |

- **Reads:** `grimoire.toml`, to check that it isn't there, and the files in the table above.
- **Writes:** `grimoire.toml`.
- **Exit codes:** 0 when written; 2 when no client is given or detected (`init/no-client`), or for an unknown client; 3 when `grimoire.toml` already exists (`init/exists`); 1, 4 or 6 for a filesystem problem ([Exit codes](#exit-codes)).
- **JSON result:** `config` (`"grimoire.toml"`), `clients` (the client ids written, each once) and `detected` (`true` when the clients were detected).

## grimoire list

```text
grimoire list [QUERY] [--kind <KIND>] [--tag <TAG>] [--client <CLIENT>] [--maturity <MATURITY>]
```

Lists this grimoire's catalog, one entry per line: id, kind, version, maturity and summary. Assets come first, then bundles, each in id order. A bundle shows `bundle` as its kind and no maturity. When nothing matches, `list` prints a note on stderr and exits 0.

| Argument or option | Meaning |
|---|---|
| `[QUERY]` | Show only entries whose id, summary, description or tags contain this text, ignoring case. Bundles match on id, summary and description. |
| `--kind <KIND>` | Only assets of this kind: `rule`, `skill`, `agent`, `tool` or `mcp-server`. |
| `--tag <TAG>` | Only assets with exactly this tag. |
| `--client <CLIENT>` | Only assets that support this client. |
| `--maturity <MATURITY>` | Only assets of this maturity: `experimental`, `stable` or `deprecated`. |

Each filter can be given once. Any of `--kind`, `--tag`, `--client` or `--maturity` leaves the bundles out.

- **Reads:** the catalog only. It needs no project.
- **Writes:** nothing.
- **Exit codes:** 0, or 2 for a usage error.
- **JSON result:**
  - `assets`: one object per asset, with `id`, `kind`, `version`, `maturity`, `summary`, `tags` and `clients` (the client ids it supports);
  - `bundles`: one object per bundle, with `id`, `version`, `summary` and `assets` (its member ids).

## grimoire show

```text
grimoire show <ID>
```

Shows one asset or bundle from the catalog. Read it before you accept an asset's permissions with `add`.

- **For an asset:** its id, version, kind and maturity; its summary and description; its licence; its deprecation and replacement, if it is deprecated; the permissions it needs; the environment variables it reads; what it requires and conflicts with; the files it writes for each client; and the newest section of its changelog.
- **For a bundle:** its version, summary and description, and each member asset with its version requirement.

| Argument or option | Meaning |
|---|---|
| `<ID>` | An asset or bundle id. |

- **Reads:** the catalog only. It needs no project.
- **Writes:** nothing.
- **Exit codes:** 0; 2 for a usage error; 3 when the id is invalid or not in the catalog (`show/unknown`).
- **JSON result for an asset:**
  - `id`, `kind`, `version`, `maturity`, `summary`, `description`, and `license` (an SPDX expression);
  - `permissions`: the permissions it needs, written as `--allow` takes them, `env:NAME` included;
  - `env`: the names of the environment variables it reads;
  - `requires`: an object from asset id to version requirement;
  - `clients`: an object from client id to the destinations it writes there, such as `".mcp.json (mcpServers.project-docs)"`;
  - `changelog`: the newest changelog section as Markdown, or `null`.
  - The deprecation and the conflicts are in the human output only.
- **JSON result for a bundle:** `id`, `kind` (`"bundle"`), `version`, `summary`, `description`, and `assets` (an object from member id to version requirement).

## grimoire add

```text
grimoire add <ID|bundle:ID|tag:TAG>... [--allow <ALLOW>] [--apply]
```

Adds what you name to `grimoire.toml`, keeping its comments and formatting:
- `ID` adds an asset under `[assets]` as `ID = "^<version>"`, with the catalog's version. An id that names a bundle adds the bundle.
- `bundle:ID` adds a bundle to `bundles`.
- `tag:TAG` adds a tag to `tags`, which selects the catalog's assets with that tag, except deprecated ones and ones that none of the project's clients supports.

Something already in `grimoire.toml` gets a note on stderr and is left as it is. Before it writes, `add` checks two things, and writes nothing if either fails:
1. **Permissions.** Every asset that what you name selects, including bundle and tag members and what they require, needs its permissions accepted. `add` takes them from `--allow`, or asks on a terminal ([Conventions](#conventions)). If some stay unaccepted, its error names the `--allow` value that accepts them.
   - Accepted permissions are recorded in `grimoire.toml`: in the asset's `[assets]` entry when it has one, otherwise under `[allow]`.
   - Assets that this command doesn't select are never granted anything.
2. **Resolution.** The new selection must resolve against the catalog: versions, dependencies, conflicts and clients.

`add` changes only `grimoire.toml`. Run `grimoire apply`, or pass `--apply`, to change the project's files.

| Argument or option | Meaning |
|---|---|
| `<ID\|bundle:ID\|tag:TAG>...` | What to add. At least one. |
| `--allow <ALLOW>` | Accept these permissions for the assets being added. Comma-separated, and repeatable: `filesystem-read`, `filesystem-write`, `network`, `shell`, `hooks`, or `env:NAME` for one environment variable. A bare `env` accepts every variable those assets read now, and `grimoire.toml` records each one by name. A `filesystem-write` grant covers `filesystem-read`. Only permissions that the assets need are recorded. |
| `--apply` | Also run `apply` (without `--force` or `--write-new`). |

- **Reads:** `grimoire.toml` and the catalog. With `--apply`, also what `apply` reads.
- **Writes:** `grimoire.toml`, only when it changes. With `--apply`, also what `apply` writes.
- **Exit codes:** 0; 2 for a usage error, such as an unknown `--allow` value; 3 when a name is invalid or unknown (`cli/spec`, `add/unknown`), when it is in `exclude` (`add/excluded`), when a permission isn't accepted (`add/permission`) or the selection doesn't resolve (`add/unresolvable`), or when `grimoire.toml` is missing or invalid. With `--apply`, also `apply`'s codes. After a conflict there (exit 4), `grimoire.toml` has already been written, and the message says so.
- **JSON result:** `added` (what was newly added, written as `ID`, `bundle:ID` or `tag:TAG`) and `allowed` (an object from asset id to the permissions this command accepted for it). With `--apply`, the result is `apply`'s, with this object under `add`.

## grimoire remove

```text
grimoire remove <ID|bundle:ID|tag:TAG>... [--apply]
```

Removes what you name from `grimoire.toml`, keeping its comments and formatting:
- `ID` removes the asset's `[assets]` entry and the permissions accepted for it. If the id is a bundle in `bundles`, it removes the bundle.
- `bundle:ID` and `tag:TAG` remove a bundle or a tag.

A few cases need care:
- If a bundle, a tag or another asset's `requires` still selects the asset, it stays installed. `remove` warns (`remove/still-selected`) and keeps its permissions under `[allow]`. To stop installing it, add it to `exclude` ([`configuration.md`](configuration.md)).
- An asset that only a bundle, a tag or `requires` selects has no entry to remove. That is an error (`remove/indirect`) that points to `exclude`.
- Naming something that isn't in `grimoire.toml` is an error (`remove/absent`).
- The new selection must still resolve. If it doesn't, nothing is written (`remove/unresolvable`).

| Argument or option | Meaning |
|---|---|
| `<ID\|bundle:ID\|tag:TAG>...` | What to remove. At least one. |
| `--apply` | Also run `apply` (without `--force` or `--write-new`). |

- **Reads:** `grimoire.toml` and the catalog. With `--apply`, also what `apply` reads.
- **Writes:** `grimoire.toml`. With `--apply`, also what `apply` writes.
- **Exit codes:** 0; 2 for a usage error; 3 for the errors above, or when `grimoire.toml` is missing or invalid. With `--apply`, also `apply`'s codes. After a conflict there (exit 4), `grimoire.toml` has already been written.
- **JSON result:** `removed` (what was removed). With `--apply`, the result is `apply`'s, with this object under `remove`.

## grimoire plan

```text
grimoire plan [--check]
```

Shows what `apply` would change, and changes nothing. It resolves `grimoire.toml` against the catalog and the pins in `grimoire.lock`, renders each client's files, and compares them with the files on disk and the lock ([`architecture.md`](architecture.md) §7.1 and §7.3).

It prints one line for each unit that is not unchanged: the action, the path (with `[block <owner>]` or `[<key>]` for part of a shared file), the owner, the clients and, for a conflict, an adopt or a forget, the reason. A summary line follows, then a unified diff of each file that would change.

| Action | Meaning |
|---|---|
| `create` | Write a new unit. |
| `update` | Replace a unit Grimoire wrote earlier. |
| `delete` | Remove a unit that is no longer wanted. |
| `adopt` | Already as desired on disk. Only the lock changes. |
| `forget` | No longer wanted and already gone. Only the lock changes. |
| `conflict` | Edited or removed since Grimoire wrote it, or an unmanaged file is in the way. The reason says which. `apply` would stop. |
| `unchanged` | Nothing to do. Counted in the summary, not printed. |

A file Grimoire can't edit safely, such as one with broken block markers or invalid JSON, is shown as `conflict` with the reason it can't be edited.

| Argument or option | Meaning |
|---|---|
| `--check` | Exit 5 if applying would change any file or `grimoire.lock`. For CI. |

- **Reads:** `grimoire.toml`, `grimoire.lock`, the catalog, every file the plan involves, and `PATH` ([Environment](#environment)).
- **Writes:** nothing.
- **Exit codes:** 0; 3 when `grimoire.toml` is missing or invalid, `grimoire.lock` is invalid, or the selection doesn't resolve; 4 when anything conflicts (`plan/conflicts`), with or without `--check`; 5 with `--check`, when there are changes to apply (`plan/changes`); 1, 4 (`fs/in-the-way`) or 6 for a filesystem problem.
  - A grimoire whose catalog has another version of a locked asset, or a changed manifest, stops with exit 3 and names `grimoire update <id>` (`resolve/locked-version-unavailable`, `resolve/locked-manifest-changed`).
- **JSON result:**
  - `summary`: counts of units, under `create`, `update`, `delete`, `conflict` (files Grimoire can't edit included), `unchanged` and `record-only` (`adopt` and `forget`);
  - `lock-changes`: whether `grimoire.lock` would change;
  - `files`: one object per file, with `path`; `unreadable` (why Grimoire can't edit the file, or `null`); `changes` (whether the file on disk would change); `diff` (a unified diff, or `null`); and `units`.
  - Each unit has `owner` (the asset id, or `grimoire` for Grimoire's own blocks), `unit` (`file`, `block` or `entry`), `key` (the entry's key, or `null`), `clients`, `action` (from the table above) and `reason` (for `conflict`, `adopt` and `forget`; otherwise `null`).

## grimoire apply

```text
grimoire apply [--force | --write-new]
```

Makes the managed content on disk match `grimoire.toml` and `grimoire.lock`, as `plan` shows it ([`architecture.md`](architecture.md) §7):
- It checks every unit first. If anything conflicts, it writes nothing and exits 4, unless you pass `--force` or `--write-new`.
- It edits only the units it owns: whole files, blocks between `<!-- grimoire:begin <id> -->` and `<!-- grimoire:end <id> -->`, and single entries in JSON or TOML files. Text and keys outside them stay as they are.
- It writes each file atomically: a temporary file in the same directory, synced, then renamed over the target. Just before writing a file, it reads it again; if the file changed since the plan, it stops (`apply/changed`, exit 4).
- It deletes a file it owns whole when the file is no longer wanted. It deletes a shared file, one with blocks or entries, only if it created the file, or adopted it when it held nothing else, and nothing else is left in it. Content it adopted, or overwrote with `--force`, counts as its own ([`configuration.md`](configuration.md) §5). After deleting a file, it removes the directories above it that are now empty.
- It writes `grimoire.lock` last, and only when the lock changes. A second `apply` writes nothing.
- If an error stops it partway, `grimoire.lock` records the files already written, so running it again continues from there.

| Argument or option | Meaning |
|---|---|
| `--force` | Overwrite managed content that was edited, and unmanaged files in the way. A file Grimoire can't edit safely is still refused: broken block markers, text that isn't UTF-8, or invalid or non-strict JSON. It can't be combined with `--write-new`. |
| `--write-new` | Leave each conflicting file alone and write what Grimoire would write to `<path>.new`. It exits 4, because the conflicts remain. It never replaces a different `<path>.new` that is already there (`apply/sidecar-exists`). |

- **Reads:** what `plan` reads.
- **Writes:** the managed files, `<path>.new` files with `--write-new`, and `grimoire.lock`.
- **Exit codes:** 0; 2 for `--force` with `--write-new`, or another usage error; 3 as for `plan`; 4 for a conflict (`apply/conflict`), conflicts left by `--write-new` (`apply/conflict-remains`), an existing `<path>.new` (`apply/sidecar-exists`), a file changed while it ran (`apply/changed`), or something in the way (`fs/in-the-way`); 1 for an I/O error; 6 when refused for safety.
- **JSON result:** `files`, one object per file written, deleted or left alone, with `path`, `outcome` (`wrote`, `deleted`, `wrote-new`, or `skipped` for a conflicting file with nothing to write beside it) and `new-file` (the `<path>.new` written, or `null`); and `lock-written` (whether `grimoire.lock` was written). The result is `null` when `apply` stops before writing anything.

## grimoire update

```text
grimoire update [IDS]... [--no-apply] [--force | --write-new]
```

`grimoire.lock` pins each installed asset's version and manifest digest, and `plan` and `apply` keep them. A grimoire carries one version of each asset ([ADR-0001](adr/0001-catalog-delivery.md)). So after you install a grimoire whose catalog has a new version of an installed asset, or a changed manifest, `plan` and `apply` stop and name `grimoire update <id>`.

`update` lets the pins of the assets you name, or of every asset, move to this catalog's versions, then applies as `apply` does. The requirements in `grimoire.toml` still apply: `^0.1.0` doesn't allow 0.2.0, so edit the requirement first. When a new version needs more permissions than you accepted, resolution fails (exit 3) until you accept them with `grimoire add <id> --allow <permissions>`.

| Argument or option | Meaning |
|---|---|
| `[IDS]...` | Assets to update; all when omitted. Each must be in `grimoire.lock` (`update/unknown`). |
| `--no-apply` | Update `grimoire.lock` only, and leave the files alone. The lock keeps each file's record as it is on disk, so the next `apply` makes the change. `--force` and `--write-new` have no effect with it. |
| `--force` | As for `apply`. |
| `--write-new` | As for `apply`. |

- **Reads:** what `plan` reads.
- **Writes:** what `apply` writes. With `--no-apply`, only `grimoire.lock`.
- **Exit codes:** as for `apply`. With `--no-apply`: 0; 2; 3; 4 when the plan involves a file Grimoire can't edit safely (`update/unreadable`), so the lock is left alone; 1, 4 (`fs/in-the-way`) or 6 for a filesystem problem.
- **JSON result:** as for `apply`. With `--no-apply`, `lock-written` only.

## grimoire verify

```text
grimoire verify
```

Checks every output that `grimoire.lock` records against the file on disk, by hash. It reads neither `grimoire.toml` nor the catalog. Use it in CI to catch edits to managed content. `plan --check` is the other CI check: it also fails when `grimoire.toml` asks for something not yet applied.

`verify` has no arguments or options of its own.

- **Reads:** `grimoire.lock` and the files it records.
- **Writes:** nothing.
- **Exit codes:** 0 when every output matches; 3 when there is no `grimoire.lock` (`verify/no-lock`) or it is invalid; 5 when outputs differ (`verify/mismatch` for each, then `verify/failed`); 1, 4 or 6 for a filesystem problem.
- **JSON result:** `outputs` (the number of outputs checked) and `mismatches`, one object per output that differs, with `owner`, `path` and `reason`. It is there when `verify` fails, too.

## grimoire uninstall

```text
grimoire uninstall [--purge] [--force]
```

Removes every unit that `grimoire.lock` records, as `apply` would with nothing selected, then deletes `grimoire.lock`. Your own text and keys in shared files stay. Without a lock there is nothing to remove, and `uninstall` exits 0.

| Argument or option | Meaning |
|---|---|
| `--purge` | Also delete `grimoire.toml`. |
| `--force` | Remove managed content even where it was edited. Without it, edited content is a conflict and nothing is removed (exit 4). |

- **Reads:** `grimoire.lock` and the files it records. It needs neither `grimoire.toml` nor the catalog.
- **Writes:** removes Grimoire's units, deleting files as `apply` does, then deletes `grimoire.lock` and, with `--purge`, `grimoire.toml`.
- **Exit codes:** 0; 2 for a usage error; 3 when `grimoire.lock` is invalid; 4 for a conflict; 1 or 6 for a filesystem problem.
- **JSON result:** `purged` (whether `--purge` was given). It doesn't list the files.

## grimoire doctor

```text
grimoire doctor
```

Reports on the project and the environment, one check per line, each marked `ok` or `FAIL`. It changes nothing and never runs a binary.

| Check | What it reports | Fails when |
|---|---|---|
| `catalog` | The number of assets and bundles in this grimoire. | Never. |
| `grimoire.toml` | Whether it is valid. | It is missing or invalid. |
| `grimoire.lock` | Whether it is valid, or not created yet. | It is invalid. |
| `client <id>` | For each client in `grimoire.toml`: the outputs installed for it, and what the client needs before it reads them, which Grimoire can't check, such as trusting the project (Codex, Gemini CLI, Copilot) or approving each project MCP server (Cursor). | Never. |
| `plan` | The number of pending changes. | `grimoire.toml` doesn't resolve, the plan can't be made (something in the way or refused, or an I/O error), or a conflict would stop `apply`. Pending changes alone don't fail. |
| `copilot cloud agent` | Shown when an MCP server is installed for `copilot`: the JSON to paste into the repository's Copilot MCP settings. | Never. |
| `outputs` | How many locked outputs differ from disk, as `verify` checks. | Any differ. |
| `binary <name>` | For each locked tool or MCP server, whether its binary is on `PATH`. A server configured over Streamable HTTP needs none. | The binary isn't found. |

`doctor` has no arguments or options of its own.

- **Reads:** `grimoire.toml`, `grimoire.lock`, the files they involve, the catalog, and `PATH`.
- **Writes:** nothing.
- **Exit codes:** 0 when every check passes; 5 when any fails (`doctor/problems`); 1 when the project directory can't be opened; 2 for a usage error. A problem with `grimoire.toml`, `grimoire.lock` or a managed file becomes a failed check, with its diagnostics, not an exit code of its own.
- **JSON result:** `checks`, one object per check, with `name`, `ok` and `detail`.

## Exit codes

| Code | Meaning | Returned by |
|---|---|---|
| 0 | Success, or nothing to do. | Every command. |
| 1 | Unexpected failure: an I/O error (`fs/io`), such as a `-C` directory that doesn't exist. Also when the current directory can't be read, or the build is broken. | Every command. |
| 2 | Usage error: an unknown command or option, a missing or invalid argument, `--force` with `--write-new`, or no command at all. | Every command. Also `init` when no client is given or detected. |
| 3 | Invalid configuration or lock, or a resolution failure ([`architecture.md`](architecture.md) §7.5). | `init`, `show`, `add`, `remove`, `plan`, `apply`, `update`, `verify`, `uninstall`. |
| 4 | Conflict: managed content edited or removed, an unmanaged file in the way, a file where a directory is needed or the reverse, or a file Grimoire can't edit safely. Nothing was written, with three exceptions, each named in the message: `add --apply` and `remove --apply` have already written `grimoire.toml`; `apply --write-new` has written `<path>.new` files; and a run that stopped when a file changed has written some files and recorded them in the lock. | `plan`, `apply`, `update`, `uninstall`, `add --apply`, `remove --apply`. Also `init`, `add`, `remove` and `verify` when something is in the way (`fs/in-the-way`). |
| 5 | A check failed. | `plan --check`, `verify`, `doctor`. |
| 6 | Refused for safety: a symbolic link at or above a managed path, a path that would leave the project root, a managed path that is neither a regular file nor a directory, or a file larger than 8 MiB (`fs/refused`). | `init`, `add`, `remove`, `plan`, `apply`, `update`, `verify`, `uninstall`. |

`grimoire --help`, `grimoire <command> --help`, `grimoire help` and `grimoire --version` exit 0. A bare `grimoire` prints the help and exits 2.

**A crash.** Nothing in the CLI catches a panic, so an internal bug that panics exits with Rust's default status, 101, after printing the panic message on stderr. No `--json` document is printed then. Please report it ([`CONTRIBUTING.md`](../CONTRIBUTING.md)).

**Interruption.** Grimoire installs no signal handler, so Ctrl-C (SIGINT) ends it at once, and it returns no exit code of its own. A POSIX shell then reports status 130 (128 plus signal 2). What Windows reports is unverified.
- No file is left half-written, because each file is written to a temporary file and renamed over the target. A temporary file can be left behind in a managed directory.
- Files written before the interruption stay, and `grimoire.lock`, which is written last, may not record them yet. Running the command again converges: a unit already as desired on disk is adopted, not reported as a conflict ([`security-model.md`](security-model.md) §7, [`architecture.md`](architecture.md) §7.1).

## JSON output

With `--json`, a command prints one JSON document on stdout, and nothing on stderr. The document is indented and ends with a newline; don't depend on its whitespace or its key order. For example, `grimoire verify --json` after a managed file was edited:

```json
{
  "format": 1,
  "command": "verify",
  "ok": false,
  "exit": 5,
  "result": {
    "outputs": 3,
    "mismatches": [
      {
        "owner": "conventional-commits",
        "path": ".claude/rules/conventional-commits.md",
        "reason": "edited since grimoire wrote it"
      }
    ]
  },
  "diagnostics": [
    {
      "severity": "error",
      "code": "verify/mismatch",
      "message": "`.claude/rules/conventional-commits.md` (conventional-commits): edited since grimoire wrote it",
      "file": ".claude/rules/conventional-commits.md",
      "field": null
    },
    {
      "severity": "error",
      "code": "verify/failed",
      "message": "1 of 3 outputs do not match grimoire.lock",
      "file": null,
      "field": null
    }
  ]
}
```

| Key | Meaning |
|---|---|
| `format` | The envelope's format, `1`. It changes only with a breaking change to the envelope, as [`versioning.md`](versioning.md) describes. |
| `command` | The command's name, such as `"verify"`. `null` for a usage error. |
| `ok` | `true` exactly when `exit` is 0. |
| `exit` | The exit code the process returns. |
| `result` | The command's result, as its section above describes, or `null` when it stopped before it had one. A failed command can still have a result, such as `verify`'s mismatches. |
| `diagnostics` | Every error and warning, in the order they were reported. |

Each diagnostic has:

| Key | Meaning |
|---|---|
| `severity` | `"error"` or `"warning"`. |
| `code` | The stable code, such as `verify/mismatch` ([`diagnostics.md`](diagnostics.md)). |
| `message` | The message text. It may hold newlines. |
| `file` | The file it is about, relative to the project root, with `/` separators. Or `null`. |
| `field` | The field within that file, or `null`. |

- **Usage errors** produce the envelope too, when `--json` is anywhere on the command line: `command` and `result` are `null`, `exit` is 2, and the one diagnostic has the code `cli/usage` and the parser's message.
- **`--help` and `--version`** print plain text, even with `--json`, and exit 0.
- **No JSON Schema** describes the envelope or the results. A release may add keys to both, so ignore keys you don't know.
- **The only writes to stderr** under `--json` happen before a command runs: when the current directory can't be read, or the build is broken (exit 1).

## Environment

| Input | Effect |
|---|---|
| `CI` | Set to anything but the empty string, it turns prompts off, as `--no-input` does. |
| stdin | Prompts happen only when stdin is a terminal. |
| `PATH` | `doctor` checks each locked tool and MCP server binary against it. `plan`, `apply`, `update`, `add --apply`, `remove --apply` and `doctor` warn (`plan/hook-binary-missing`) when a selected tool's hooks fail closed and its binary isn't found there. |
| The current directory | The project root, unless `-C` is given. |

A binary counts as found on `PATH` when a file of that name is in one of its directories; on Windows, a file with `.exe` added counts too. Grimoire never runs it, and doesn't check that it can be run.

Grimoire's own code reads no other environment variable, and no configuration file outside the project. In the container image, `PATH` is the container's ([`container.md`](container.md)).
