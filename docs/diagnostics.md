# Diagnostic codes

Every error or warning the `grimoire` command-line interface (CLI) reports is a diagnostic with a stable code, such as `plan/hook-binary-missing`. This file lists every code the CLI can print, grouped by prefix (the part before the `/`). For each code it gives the severity and exit status, the commands that can raise it, what it means and what to do. [`cli.md`](cli.md) describes the commands, their options and the exit codes.

`grimoire` installs AI-agent assets (rules, skills, agents, tools and Model Context Protocol (MCP) servers) into a project, for the AI coding tools it calls *clients*. You select assets in `grimoire.toml` one by one, through a bundle (a named, versioned set of assets) or through a tag. [`configuration.md`](configuration.md) describes `grimoire.toml` and `grimoire.lock`, the two files many of these codes are about.

- **A test keeps this list complete.** The test `crates/agent-grimoire/tests/diagnostics_doc.rs` compares this list with the code literals in the source. It fails when a code the CLI can raise is missing here, or when a code listed here no longer appears in the source. It finds codes by reading the source as text, and its module documentation (the comment at the top of the file) says what that can miss.
- **Some rows say "report it".** Where a row says **report it**, the condition points to a bug in Grimoire or its catalog. Please [open an issue](https://github.com/jabbott-iii/Grimoire/issues) with the command, its output and, if you can, your `grimoire.toml`.

## Reading a diagnostic

Without `--json`, diagnostics go to standard error (stderr):
```text
error[config/clients] grimoire.toml (clients): at least one client is required
warning[resolve/unknown-tag] grimoire.toml (tags): no asset in this grimoire's catalog has the tag `zzz`
```
- The form is `<severity>[<code>] <file> (<field>): <message>`. The file and the field are left out when the diagnostic has none, and a message can run over several lines.
- Control characters in the file, the field and the message (except newlines in the message) are printed escaped. So a hostile file name can't send terminal escape sequences: byte sequences that control the terminal, for example to move the cursor or change colors.

With `--json`, the diagnostics go into the one JSON document on stdout ([`cli.md`](cli.md#json-output)). Its `diagnostics` array holds an object for each, in the order the command reported them:
```json
{
  "severity": "error",
  "code": "config/clients",
  "message": "at least one client is required",
  "file": "grimoire.toml",
  "field": "clients"
}
```

| Key | Value |
|---|---|
| `severity` | `"error"` or `"warning"` |
| `code` | The code, as listed below |
| `message` | The explanation, written for people |
| `file` | The file the diagnostic is about, relative to the project root with `/` separators, or `null` |
| `field` | The field within that file, such as `clients`, `assets.<id>.clients` or `asset.<id>.output[0]`, or `null` |

All five keys are always present.

**Codes are stable.** Scripts and tests should match on `code`, not on `message`. The codes are part of the interface that the versioning policy, [`versioning.md`](versioning.md), covers: a code keeps its meaning, and removing or renaming one is a breaking change.

**Severity and exit status.** A command that reports an error exits with a non-zero status. A warning never changes the exit status. The tables give the status each error leads to.

Some errors explain a later one that sets the status. For example, each `plan/conflict` comes before the `apply/conflict` that ends the command.

`doctor` is the exception. It reports what it finds in `grimoire.toml`, `grimoire.lock` and the plan, keeps going, and exits 5 with `doctor/problems` if any check failed. Only a project directory it can't open stops it early.

## Which commands raise them

The tables name commands, or one of these groups:

| Group | Commands | What they do |
|---|---|---|
| project commands | every command except `list` and `show` | Open the project directory: `-C DIR`, or the current directory |
| config readers | `add`, `remove`, `plan`, `apply`, `update`, `doctor` | Read `grimoire.toml` |
| lock readers | `plan`, `apply`, `update`, `verify`, `uninstall`, `doctor`, and `add` or `remove` with `--apply` | Read `grimoire.lock` |
| planning commands | `plan`, `apply`, `update`, `doctor`, and `add` or `remove` with `--apply` | Resolve the selection, render it for each client and plan the changes |
| applying commands | `apply`, `update` without `--no-apply`, `uninstall`, and `add` or `remove` with `--apply` | Write the plan to disk |

Resolving works out exactly which assets and versions the selection means, and checks them. Rendering turns each resolved asset into the content of the files a client reads.

`add` and `remove` without `--apply` also resolve: they check the selection they are about to write, without reading `grimoire.lock`, and don't render it. `uninstall` doesn't read `grimoire.toml`, so it raises no `config/`, `resolve/` or `adapter/` codes.

## Codes

### The command line (`cli/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `cli/usage` | error, exit 2 | every command | The command line can't be parsed: an unknown command or option, a missing argument or an invalid value. Only the `--json` document carries this code, with `command` set to `null`. Without `--json`, the parser's own message goes to stderr, without a code. | Read the message, and see `grimoire <command> --help`. |
| `cli/spec` | error, exit 3 | `add`, `remove` | An argument isn't a valid `ID`, `bundle:ID` or `tag:TAG`. The message says what is wrong with it. | Correct the argument. `grimoire list` shows the ids, and `grimoire list --json` each asset's tags. |

### Project files (`project/`)

Problems with `grimoire.toml` and `grimoire.lock` as files. Both are written in TOML, a configuration file format. [`configuration.md`](configuration.md) describes both formats.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `project/no-config` | error, exit 3 | config readers | The project directory has no `grimoire.toml`. | Run `grimoire init`, or pass `-C DIR` to name the project root. |
| `project/config-parse` | error, exit 3 | config readers | `grimoire.toml` is not valid TOML, or doesn't match the format: an unknown key, a missing required key, a value of the wrong type or form, or a `format` other than 1. The message shows the line and column. | Fix the file where the message points. |
| `project/encoding` | error, exit 3 | `init`, config readers, lock readers | `grimoire.toml` or `grimoire.lock` is not UTF-8 text. | Save the file as UTF-8. |
| `project/lock-parse` | error, exit 3 | lock readers | `grimoire.lock` is not valid TOML, or doesn't match the lock format. | Grimoire writes `grimoire.lock`, and it isn't meant to be edited by hand. Restore it from version control. |
| `project/lock-path` | error, exit 3 | lock readers | `grimoire.lock` records an output at a path Grimoire never writes, or a kind of content (`file`, `block` or `entry`) other than the one Grimoire owns at that path. The lock was most likely edited by hand, so Grimoire stops rather than read or delete what it names. | Restore `grimoire.lock` from version control. |
| `project/lock-write` | error, exit 1 | `plan`, applying commands, `update --no-apply` | Grimoire couldn't serialize the new lock (turn it into TOML text). | Report it. |

### The filesystem (`fs/`)

Grimoire reads and writes only inside the project directory. It refuses a symbolic link at or above any path it touches, and a special file at the path itself, such as a FIFO (a named pipe).
- [`architecture.md`](architecture.md) §7.4, on filesystem safety, has the design.
- [ADR-0006](adr/0006-filesystem-confinement.md), the architecture decision record (ADR) on filesystem confinement, explains why every access goes through a handle on the project root. With plain path-based calls, someone who can write in the project could swap a directory for a symbolic link between Grimoire's check and its write, and send the write outside the project.

If one of these refusals stops an applying command partway, the files already written are recorded in `grimoire.lock`, and running the command again continues from there. After `add --apply` or `remove --apply`, `grimoire.toml` has already been changed.

Two kinds of check only ask whether a path exists: whether a client's files are there, and whether `AGENTS.override.md` is. They walk to the path the same way, but count a linked folder on the way as absent rather than failing.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `fs/refused` | error, exit 6 | project commands | Refused for safety: a symbolic link at or above a path Grimoire reads or writes, a special file (such as a FIFO) at the path itself, a file larger than 8 MiB, a path that would lead outside the project directory, or a directory that couldn't be created. | Look at the path in the message. Replace a symbolic link with a real file or directory, or move what is there out of the way. |
| `fs/in-the-way` | error, exit 4 | project commands | A file is where Grimoire needs a directory, or a directory where it needs a file. A special file above a path, such as a FIFO named `.claude`, counts as a file here. | Move or remove what is in the way, then run again. |
| `fs/io` | error, exit 1 | project commands | Any other input/output (I/O) error, such as a `-C` directory that doesn't exist or a permission problem. The message carries the system's error. | Fix the cause the message names. |

### grimoire.toml (`config/`)

Checks across the fields of `grimoire.toml`, once it parses. Each names the field in `field`. The config readers raise them, and `add` and `remove` also check the file they are about to write. Each error exits 3.

Three of these codes are about MCP server options. By default a client starts an MCP server itself and talks to it over stdio (the server's standard input and output). With `transport = "streamable-http"`, you start the server yourself, and the client connects to it at `url`.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `config/clients` | error, exit 3 | config readers | `clients` is empty, or lists a client more than once. | List at least one client, each once. |
| `config/bundles` | error, exit 3 | config readers | `bundles` lists a bundle more than once. | Remove the repeat. |
| `config/tags` | error, exit 3 | config readers | `tags` lists a tag more than once. | Remove the repeat. |
| `config/exclude` | error, exit 3 | config readers | `exclude` lists an asset more than once, or an asset has an `[assets]` entry and is also in `exclude`. | Remove the repeat, or decide whether the asset is selected or excluded. |
| `config/allow-direct` | error, exit 3 | config readers | `[allow]` has an entry for an asset that also has an `[assets]` entry. `[allow]` is only for assets that a bundle, a tag or another asset's `requires` (its list of dependencies) brings in. | Move the permissions into the asset's `[assets]` entry, as `allow = [...]`. |
| `config/allow-empty` | error, exit 3 | config readers | An `[allow]` entry lists no permissions. | Remove the entry. |
| `config/asset-clients` | error, exit 3 | config readers | The `clients` of an `[assets]` entry is empty, lists a client more than once, or names a client that isn't in the top-level `clients`. | Leave `clients` out, so the asset is installed for every project client it supports, or list some of the project's clients, each once. |
| `config/url-missing` | error, exit 3 | config readers | An `[assets]` entry has `transport = "streamable-http"` but no `url`. | Add the server's endpoint, such as `url = "http://127.0.0.1:8931/mcp"`. |
| `config/url-transport` | error, exit 3 | config readers | An `[assets]` entry has a `url` without `transport = "streamable-http"`. | Add the transport, or remove the `url`. |
| `config/env-transport` | error, exit 3 | config readers | An `[assets]` entry has `env` overrides and `transport = "streamable-http"`. Over Streamable HTTP you start the server yourself, so the client passes it no environment. | Remove `env`, or use the `stdio` transport. |

### grimoire.lock (`lock/`)

Checks across the records of `grimoire.lock`, once it parses. The lock readers raise them, and each error exits 3.

Each output record in the lock describes one unit of content Grimoire owns: a whole `file`, a `block` in a shared Markdown file, or an `entry` in a configuration file. Grimoire writes the lock, so these codes mean it was edited by hand or damaged: restore it from version control.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `lock/assets` | error, exit 3 | lock readers | An asset is recorded more than once. | Restore `grimoire.lock` from version control. |
| `lock/output-key` | error, exit 3 | lock readers | An output has a `key` but isn't an `entry`, or an `entry` has no `key`. | Restore `grimoire.lock` from version control. |
| `lock/output-clients` | error, exit 3 | lock readers | An output lists no clients. | Restore `grimoire.lock` from version control. |
| `lock/output-created` | error, exit 3 | lock readers | `created-file` is set on a `file` output, or `created-keys` on an output that isn't an `entry`. | Restore `grimoire.lock` from version control. |
| `lock/output-owner` | error, exit 3 | lock readers | One path mixes `file`, `block` and `entry` outputs, or two records claim the same file, block or entry. | Restore `grimoire.lock` from version control. |

### init (`init/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `init/exists` | error, exit 3 | `init` | The project already has a `grimoire.toml`. `init` never overwrites it. | Edit the file, or use `grimoire add` and `grimoire remove`. |
| `init/no-client` | error, exit 2 | `init` | No `--client` was given, and no client's files were found to detect one from. | Pass `--client` once for each client, such as `--client claude-code`. |
| `init/client-unavailable` | error, exit 3 | `init` | This build has no adapter (the code that writes one client's files) for a client given with `--client`. In this version every client has an adapter, so it isn't raised. | Use a grimoire release that supports the client. |

### show (`show/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `show/unknown` | error, exit 3 | `show` | The argument isn't a valid id, or no asset or bundle in the embedded catalog (the catalog built into this `grimoire`) has it. | Check the id with `grimoire list`. |

### add (`add/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `add/unknown` | error, exit 3 | `add` | No asset in the embedded catalog has the id, or, for `bundle:ID`, no bundle. | Check the id with `grimoire list`. |
| `add/excluded` | error, exit 3 | `add` | The asset is in `exclude` in `grimoire.toml`. | Remove it from `exclude` first. |
| `add/permission` | error, exit 3 with `add/unresolvable` | `add` | An asset this command adds, directly or through a bundle, a tag or `requires`, needs permissions that weren't accepted. Either the prompt was answered no, there was no prompt (with `--no-input` or `--json`, with a non-empty `CI` environment variable, or when stdin is not a terminal), or `--allow` didn't name them all. | Review them with `grimoire show <id>`, then pass the `--allow` list the message gives. |
| `add/unresolvable` | error, exit 3 | `add` | Permissions were refused, or the new selection doesn't resolve. The diagnostics before it say why. `grimoire.toml` was not changed. | Fix what those diagnostics report. |

### remove (`remove/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `remove/absent` | error, exit 3 | `remove` | What the argument names isn't in `grimoire.toml`. | Check the argument against `grimoire.toml`: bundles are in `bundles` and tags in `tags`. |
| `remove/indirect` | error, exit 3 | `remove` | The asset has no `[assets]` entry: a bundle, a tag or another asset's `requires` installs it. | Add it to `exclude` to stop installing it, or remove what brings it in. `exclude` can't drop an asset that another selected asset requires (`resolve/excluded-dependency`). |
| `remove/still-selected` | warning | `remove` | The asset's `[assets]` entry was removed, but a bundle, a tag or another asset's `requires` still selects it, so it stays installed. Its accepted permissions move to `[allow]`. | Add it to `exclude` if it should go. |
| `remove/unresolvable` | error, exit 3 | `remove` | The selection left after removing doesn't resolve. The diagnostics before it say why. `grimoire.toml` was not changed. | Fix what those diagnostics report. |

### update (`update/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `update/unknown` | error, exit 3 | `update` | An id given to `update` isn't valid, or isn't in `grimoire.lock`. | Name installed assets only. `grimoire.lock` lists them. |
| `update/unreadable` | error, exit 4 | `update --no-apply` | A file Grimoire would have to edit can't be edited (each reported before it as `plan/unreadable`), so Grimoire won't record new versions. `grimoire.lock` was not changed. | Fix the files reported, then run again. |

### verify (`verify/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `verify/no-lock` | error, exit 3 | `verify` | The project has no `grimoire.lock`, so there is nothing to verify. | Run `grimoire apply` first. |
| `verify/mismatch` | error, exit 5 with `verify/failed` | `verify` | An output on disk doesn't match its record in `grimoire.lock`: it was edited, it is missing, or its file can't be read the way Grimoire wrote it. The message says which, and `file` names the file. | `grimoire plan` shows what `apply` would do. `grimoire apply --force` restores Grimoire's version of an edited or missing output. |
| `verify/failed` | error, exit 5 | `verify` | How many outputs don't match `grimoire.lock`. | See the `verify/mismatch` diagnostics before it. |

### doctor (`doctor/`)

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `doctor/problems` | error, exit 5 | `doctor` | At least one check failed. The checks marked `FAIL` (with `--json`, those with `"ok": false`) and the diagnostics before them say what. | Fix what the failed checks report. |

### Resolution (`resolve/`)

Resolution turns the selection in `grimoire.toml`, and the pins in `grimoire.lock`, into the assets to install. A pin is what the lock records for an installed asset: its exact version, and the hash of its manifest (`asset.toml`, the file that describes the asset). [`architecture.md`](architecture.md) §7.1, on resolving, planning and applying, and §7.5, on resolution errors, have the design.

Resolution reports every problem at once. `add`, `remove` and the planning commands raise these codes, except the two `resolve/locked-…` codes, which only the planning commands raise, because `add` and `remove` check their selection without the lock. Each error exits 3.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `resolve/unknown-asset` | error, exit 3 | `add`, `remove`, planning commands | An `[assets]` entry names an id that isn't in the embedded catalog. If the id is a bundle, the message says so. | Correct the id, or move a bundle to `bundles`. |
| `resolve/unknown-bundle` | error, exit 3 | `add`, `remove`, planning commands | A `bundles` entry isn't a bundle in the embedded catalog. If the id is an asset, the message says so. | Correct the id, or move an asset to `[assets]`. |
| `resolve/unknown-tag` | warning | `add`, `remove`, planning commands | No asset in the embedded catalog has a tag listed in `tags`. | Correct or remove the tag. |
| `resolve/tag-skipped` | warning | `add`, `remove`, planning commands | A tag matches an asset that supports none of the project's clients, so it isn't installed. | Nothing, unless you want the asset: then add a client it supports. |
| `resolve/excluded-dependency` | error, exit 3 | `add`, `remove`, planning commands | A selected asset requires an asset that `exclude` lists. | Remove the dependency from `exclude`, or stop selecting the asset that requires it. |
| `resolve/missing-dependency` | error, exit 3 | `add`, `remove`, planning commands | A selected asset requires an asset that isn't in the embedded catalog. Catalog validation, which checks the catalog before it is built into a grimoire release, refuses this (`catalog/requires-missing`). | Report it. |
| `resolve/version` | error, exit 3 | `add`, `remove`, planning commands | A version requirement doesn't match the asset's version in the embedded catalog. The message says who asked for it: `grimoire.toml`, a bundle or another asset. | Change the requirement in `grimoire.toml`, such as to `^<catalog version>`, or use a grimoire release whose catalog has a matching version. |
| `resolve/locked-version-unavailable` | error, exit 3 | planning commands | `grimoire.lock` pins a version of the asset that the embedded catalog doesn't have, for example after a change of grimoire version. | Run `grimoire update <id>` to move to the catalog's version, or use a grimoire release that has the pinned one. |
| `resolve/locked-manifest-changed` | error, exit 3 | planning commands | The embedded catalog has the pinned version, but its manifest differs from the one `grimoire.lock` records. | Run `grimoire update <id>`. |
| `resolve/cli-too-old` | error, exit 3 | `add`, `remove`, planning commands | The asset's `min-cli-version` (the oldest grimoire version it works with) is newer than this grimoire. | Use a grimoire release at least that new. |
| `resolve/deprecated` | warning | `add`, `remove`, planning commands | A selected asset is deprecated. The message gives the reason and the replacement. Tags never select deprecated assets. | Move to the replacement. |
| `resolve/options-kind` | error, exit 3 | `add`, `remove`, planning commands | An `[assets]` entry for an asset that isn't an MCP server sets `transport`, `url` or `env`, which are only for MCP servers. | Remove those keys. |
| `resolve/unsupported-client` | error, exit 3 | `add`, `remove`, planning commands | The asset can't be installed for the clients asked: its `clients` names one it doesn't support, or it supports none of the project's clients. | List only clients it supports. To skip an asset that a bundle brings in, add it to `exclude`, as the message says; an asset that another one requires can't be excluded. |
| `resolve/client-skipped` | warning | `add`, `remove`, planning commands | The asset doesn't support one of the project's clients, so it is installed only for the others. | Nothing, unless that client matters. For an `[assets]` entry, listing the clients you want in its `clients` silences it. |
| `resolve/client-not-covered` | warning | `add`, `remove`, planning commands | One of the project's clients has no place for this kind of asset: Grimoire writes hooks (commands a client runs on events, such as before a tool call) only for Claude Code, and a Codex agent can't be limited to its tools. [ADR-0008](adr/0008-phase-4-client-adapters.md) records why: no other client can keep a fail-closed hook closed (blocking the action when the hook fails), and a Codex agent inherits its parent session's sandbox, approvals, MCP servers and web search. `agents-md` never gets this warning, because by design it holds no agents, MCP servers or hooks. | Nothing: the asset is installed for the other clients. |
| `resolve/permission` | error, exit 3 | `add`, `remove`, planning commands | A selected asset needs permissions that `grimoire.toml` doesn't accept ([`architecture.md`](architecture.md) §6.3, on permission consent). The message gives the command, or the `allow` entry, that accepts them. | Review them with `grimoire show <id>`, then accept them as the message says. |
| `resolve/conflict` | error, exit 3 | `add`, `remove`, planning commands | Two selected assets conflict. The message gives the reason and what selected each. | Keep only one of them. Use `exclude` if a bundle or a tag brings one in. |
| `resolve/allow-unused` | warning | `add`, `remove`, planning commands | An `[allow]` entry names an asset that isn't selected, so it has no effect. | Remove the entry. |
| `resolve/client-unavailable` | error, exit 3 | `add`, `remove`, planning commands | `clients` names a client this build has no adapter for. In this version every client has an adapter, so it isn't raised. | Use a grimoire release that supports the client. |

### Adapters (`adapter/`)

An adapter is the code that writes one client's files: it renders each resolved asset into the content that client reads ([`architecture.md`](architecture.md) §8, on client adapters). The planning commands raise these codes, and each error exits 3.

Catalog validation renders every catalog asset for each client it lists ([`architecture.md`](architecture.md) §6.4, on catalog validation). So an error that depends only on the asset can't come from the embedded catalog. The rows below say which errors those are.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `adapter/env-unknown` | error, exit 3 | planning commands | An `env` override in an `[assets]` entry names a variable the server doesn't read, or one the client sets itself and so can't be remapped (passed under another name). The message lists the variables you can remap. | Use one of the listed names. |
| `adapter/env-remap` | error, exit 3 | planning commands | Codex passes a variable to a server only under its own name, so it can't apply an `env` override that renames one. | Set the variable itself, or leave `codex` out of the asset's `clients`. |
| `adapter/agent-readonly` | warning | planning commands | Cursor runs the agent with `readonly: true`, Cursor's only limit for an agent. It blocks edits and state-changing shell commands, but shell commands Cursor judges harmless stay available, and whether it limits MCP tools and web fetches is undocumented. | Keep that in mind before relying on the agent being read-only in Cursor. |
| `adapter/agent-strict-tools` | warning | planning commands | Copilot limits the agent to its tools with `strict-tools-list: true`, which VS Code doesn't pass on, so in VS Code the agent may also get Copilot's default tools. | Keep that in mind when you use the agent in VS Code. |
| `adapter/hook-tools` | warning | planning commands | A Claude Code hook matches Grep and Glob, which Claude Code's main session doesn't have by default on macOS, Linux and Windows Subsystem for Linux (WSL). It searches with Bash instead, so the hook may rarely run. | Nothing to do in Grimoire. |
| `adapter/unsupported` | error, exit 3 | planning commands | A client can't express what an asset needs. For example: a Cursor agent whose tools aren't exactly `read` and `search`; a rule glob (a file-name pattern such as `src/**/*.rs`) with a comma, for Cursor or Copilot; a Copilot agent prompt over its length limit; or an MCP tool name a client would rename or drop. Catalog validation renders every asset this way, so the embedded catalog can't cause it. | Report it. |
| `adapter/source` | error, exit 3 | planning commands | A source file of the asset is missing from the embedded catalog, or isn't UTF-8. Catalog validation renders every asset this way, so the embedded catalog can't cause it. | Report it. |
| `adapter/entry-key` | error, exit 3 | planning commands | Grimoire couldn't form the key it records for a Claude Code hook entry. | Report it. |
| `adapter/output-clash` | error, exit 3 | planning commands | One asset would write different content to the same place for two clients. Grimoire writes a location that several clients read once, for all of them, so their content must agree: this points to a bug in an adapter. | Report it, with your `grimoire.toml`. |

### Planning (`plan/`)

The planner compares what the adapters want on disk with `grimoire.lock` and the files there ([`architecture.md`](architecture.md) §7, the install model). It works in units: a unit is a whole file, a managed block (the text between Grimoire's marker lines) in a shared Markdown file, or one entry in a configuration file.

Many `plan/` warnings are about clients reading each other's files. Where several clients read one location, Grimoire writes it once, and a client can then read content that wasn't installed for it. [`architecture.md`](architecture.md) §8.2 has the table of which client reads which location, and [ADR-0008](adr/0008-phase-4-client-adapters.md) records where Grimoire puts each client's rules, skills, agents and MCP servers. Warnings don't change what Grimoire writes.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `plan/cross-client-rule` | warning | planning commands | A rule in `AGENTS.md` is also read by Claude Code (through the `@AGENTS.md` import Grimoire adds to `CLAUDE.md`, a line that makes Claude Code read `AGENTS.md` too, or when the project has no `CLAUDE.md`) or by Gemini CLI (through the import in `GEMINI.md`), although it isn't installed for that client. | Accept it, or install the rule for that client too. If that client must not read the rule, remove the rule. |
| `plan/cross-client-mcp` | warning | planning commands | An MCP server in `.mcp.json` is also available in GitHub Copilot (VS Code and CLI), or in Claude Code once you approve it, although it isn't installed for that client. | Accept it, or install the server for that client too. |
| `plan/cross-client-agent` | warning | planning commands | An agent in `.claude/agents/` is also loaded by Cursor or GitHub Copilot, and how they apply its tool limits is undocumented. The message gives the details for each. | Don't rely on the agent's tool limits in those clients. |
| `plan/cross-client-hooks` | warning | planning commands | Cursor and GitHub Copilot also run hooks from `.claude/settings.json`. Whether they pass `args` and honor `onFailure` is undocumented. So there a hook may let actions through when it fails, or block every tool call if its arguments are dropped. | If a client blocks every tool call, turn off Claude Code hooks in that client. |
| `plan/duplicate-rule` | warning | planning commands | GitHub Copilot CLI or Cursor may read the same rule more than once, from different files. | Nothing to do in Grimoire. |
| `plan/duplicate-skill` | warning | planning commands | With `cursor` selected, a skill is in both `.claude/skills/` and `.agents/skills/`, so Cursor lists it twice. | Nothing to do in Grimoire. |
| `plan/codex-override` | warning | planning commands | With `codex` in `clients`, `AGENTS.override.md` exists, so Codex reads it instead of `AGENTS.md`, and none of the rules Grimoire writes there. | Remove or rename `AGENTS.override.md` if Codex should read those rules. |
| `plan/codex-instructions-size` | warning | planning commands | With `codex` in `clients`, the planned `AGENTS.md` is 24 KiB or more. Codex reads at most `project_doc_max_bytes` (32 KiB by default) of project instructions in all, nested `AGENTS.md` files included, and cuts off the rest. | Install fewer rules, or raise `project_doc_max_bytes` in your Codex configuration. |
| `plan/hook-binary-missing` | warning | planning commands | A selected tool's hooks fail closed (`on-failure = "block"`: when a hook can't run, the client blocks the event rather than letting it through), and its binary isn't on `PATH`. Until it is installed, the client blocks every event those hooks cover: for Claude Code, every tool call or prompt. | Install the binary first. The message gives the command that starts Claude Code with hooks turned off, to recover a blocked session. |
| `plan/duplicate-output` | error, exit 3 | planning commands | The adapters would write the same file, block or entry twice, or two identical entries into one array. They aren't meant to. | Report it, with your `grimoire.toml`. |
| `plan/mixed-units` | error, exit 3 | planning commands | One path would hold both a whole file and blocks or entries. | Report it, with your `grimoire.toml`. |
| `plan/case-collision` | error, exit 3 | planning commands | Two outputs differ only in letter case, which makes them one file on macOS and Windows. Or `grimoire.lock` records a file that the new plan renames only in case. | For a rename in case, remove the asset and apply, then add it again. Otherwise report it. |
| `plan/conflicts` | error, exit 4 | `plan` | Applying would stop on a conflict. The plan printed before it shows which. | Resolve the conflicts, or apply with `--force` or `--write-new`. A file the plan shows as `cannot be edited` must be fixed by hand first, because neither option helps it. |
| `plan/changes` | error, exit 5 | `plan --check` | Applying would change files or `grimoire.lock`. | Run `grimoire apply`. |
| `plan/conflict` | error, explains the exit 4 that follows | applying commands, `update --no-apply` | One unit conflicts ([`architecture.md`](architecture.md) §7.3, on drift and conflicts): content Grimoire wrote was edited or removed since, a file Grimoire doesn't manage is where it would write, or comments were added inside an entry. `file` names the file. | Merge or undo the change by hand, or use `--force` to overwrite it or `--write-new` to write `<path>.new` (`uninstall` takes `--force` only). For a file Grimoire would delete, `--write-new` writes no `.new` and only leaves the file alone. |
| `plan/unreadable` | error, explains the exit 4 that follows | applying commands, `update --no-apply` | Grimoire can't edit a file it must change. For example, it isn't UTF-8, its managed-block markers are broken, it doesn't parse as JSON or TOML, or `grimoire.lock` records a different kind of content there. `--force` can't help, because Grimoire never guesses what it owns in such a file. | Fix the file by hand, then run again. |

### Applying (`apply/`)

The applying commands raise these, and each exits 4.

| Code | Severity | Commands | Meaning | What to do |
|---|---|---|---|---|
| `apply/conflict` | error, exit 4 | applying commands | The plan has conflicts, each reported before it as `plan/conflict` or `plan/unreadable`, and neither `--force` nor `--write-new` was given, or a file can't be edited at all. Nothing was written, except that `add --apply` and `remove --apply` had already changed `grimoire.toml`. | Resolve the conflicts, or run again with `--force` to overwrite or `--write-new` to write `<path>.new` files (`uninstall` takes `--force` only). A file reported as `plan/unreadable` must be fixed by hand first, because neither option helps it. For a file Grimoire would delete, `--write-new` writes no `.new`. |
| `apply/sidecar-exists` | error, exit 4 | `apply --write-new`, `update --write-new` | `--write-new` would replace a `<path>.new` file (a sidecar, the file it writes beside the original) that holds something else, such as a merge in progress. Nothing was written. | Move the `.new` file away or delete it, then run again. |
| `apply/changed` | error, exit 4 | applying commands | A file changed between planning and writing. The files written before it stopped are recorded in `grimoire.lock`. | Run the command again. It continues from there. |
| `apply/conflict-remains` | error, exit 4 | `apply --write-new`, `update --write-new` | Conflicting files were left alone, each reported before it as `plan/conflict`. For each file Grimoire still wants, its version was written to `<path>.new`. A file it would delete gets no `.new`: it is only shown as `skipped`. Everything else was applied. | Review and merge each `.new` file by hand. Delete a skipped file yourself once you have kept what you need from it, or use `--force`. Then check with `grimoire plan`. |

## Not listed here

- **Catalog-author codes:** `manifest/…`, `bundle/…`, `catalog/…`, `skill/…`, `rule/…`, `agent/…`, `crate/…`, `pack/…` and `version/…`. A CLI user never sees these codes:
  - They come from checking the catalog before it is embedded in a build, and continuous integration (CI) runs each check: `cargo xtask validate` ([`catalog/README.md`](../catalog/README.md)), `cargo xtask validate --base` for the `version/…` codes ([`versioning.md`](versioning.md)), and `cargo xtask pack-catalog` for the `pack/…` codes.
  - Each build parses its embedded catalog again when it starts, and re-checks each manifest and bundle on its own. The checks across the catalog aren't run again, which is why some rows above say **report it**.
  - A build whose catalog fails at start-up stops before any command with `error: this grimoire build is broken: …` and exit 1, without a code.
- **`plugin/asset`:** it comes from generating the Claude Code plugin marketplace (`cargo xtask gen-marketplace`). The CLI never raises it.
- **Messages without a code:**
  - without `--json`, a command line that can't be parsed (`cli/usage` above);
  - `--help` and `--version`, which exit 0, and a bare `grimoire`, which prints the help on stdout and exits 2;
  - the start-up failures (a broken build, an invalid built-in version, an unreadable current directory), which print `error: …` on stderr and exit 1, even with `--json`;
  - the check lines `doctor` prints, and the plan `plan` prints.
