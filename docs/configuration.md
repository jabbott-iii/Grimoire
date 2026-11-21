# Configuring a project

- **Status:** written on 2026-11-17, in Phase 7 of the project plan (`PLAN.md`), the documentation phase, before the first release. It describes the `grimoire` command-line interface (CLI) as built from this repository on that date. The examples, except the made-up server in §1.6, were run with a local debug build in scratch directories, against the catalog embedded in that build.
- **Design:** [`architecture.md`](architecture.md) §6–§8 (manifests and configuration, the install model, and the client adapters); [ADR-0001](adr/0001-catalog-delivery.md), the architecture decision record (ADR) that embeds the catalog in the binary; and [ADR-0008](adr/0008-phase-4-client-adapters.md), which decides what each client gets.
- **Related:** every command, flag and exit code is in [`cli.md`](cli.md), every diagnostic code in [`diagnostics.md`](diagnostics.md), and the versioning and deprecation policy in [`versioning.md`](versioning.md).

Grimoire keeps two files at the root of your project. Both are written in TOML, a configuration file format of `key = value` lines grouped under `[table]` headers.

| File | Written by | Commit it |
|---|---|---|
| `grimoire.toml` | you, and `grimoire init`, `add` and `remove` | yes |
| `grimoire.lock` | `grimoire apply` and `grimoire update` only; `grimoire uninstall` deletes it | yes |

This page has six parts:
1. `grimoire.toml`;
2. permissions and consent;
3. `grimoire.lock`;
4. clients, and what Grimoire writes for each;
5. what Grimoire never touches;
6. drift and conflicts: what happens when content Grimoire wrote was changed by someone else, or when something else is in the way.

## 1. `grimoire.toml`

`grimoire.toml` describes the state you want the project to be in, its desired state: the clients to write for and the assets to install.
- A **client** is an AI coding tool, such as Claude Code or Cursor, that reads instructions and configuration from your repository.
- An **asset** is one entry in Grimoire's catalog, of one of five kinds: a rule (instructions the client reads automatically), a skill (instructions and files the model loads when it needs them), an agent (a subagent with its own prompt and tools), a tool (a program the client runs, for example from a hook) or an MCP server (a program that offers tools to the model over the Model Context Protocol, MCP).

`grimoire apply` makes the project match the file.

Its JSON Schema, generated from Grimoire's Rust types, is [`schemas/grimoire.schema.json`](../schemas/grimoire.schema.json). A JSON Schema is a JSON document that describes which fields a file may hold and what type each one has.

### 1.1 An example

```toml
format = 1
clients = ["claude-code", "agents-md", "cursor"]
bundles = ["security-baseline"]
tags = ["git"]

[assets]
adr-writer = "^0.2.0"
project-docs = { version = "^0.1.0", clients = ["claude-code"], allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"], transport = "streamable-http", url = "http://127.0.0.1:8931/mcp" }

[allow]
secret-guard = ["filesystem-read", "hooks"]
security-reviewer = ["filesystem-read"]
```

This project:
- writes for Claude Code, the portable `AGENTS.md` target (`AGENTS.md` and `.agents/skills/`, which many clients read, §4.1) and Cursor;
- installs the `security-baseline` bundle, a named list of assets that holds `secret-guard` and `security-reviewer`;
- installs every asset tagged `git` (today, `conventional-commits`);
- installs two assets by name, `adr-writer` and `project-docs`;
- configures the `project-docs` MCP server for Claude Code only, over Streamable HTTP (MCP over HTTP, §1.6), for a server you start yourself;
- accepts, in the `[allow]` table, the permissions of the two bundle members, which have no `[assets]` entry of their own.

### 1.2 Top-level fields

| Field | Type | Default | Meaning |
|---|---|---|---|
| `format` | integer | required | The file format. Must be `1`. How the number changes is in [`versioning.md`](versioning.md). |
| `clients` | array of client ids | required | The clients to write for: at least one, each listed once. The ids are `claude-code`, `agents-md`, `codex`, `cursor`, `gemini-cli` and `copilot` (§4.1). |
| `bundles` | array of bundle ids | `[]` | Bundles to install: every member, at the version requirement the bundle gives it. |
| `tags` | array of tags | `[]` | Install every asset that carries one of these tags (§1.4). |
| `exclude` | array of asset ids | `[]` | Never install these, even when a bundle or a tag selects them (§1.4). |
| `[assets]` | table of asset id = selection | empty | Assets you select by name (§1.3). |
| `[allow]` | table of asset id = array of permissions | empty | Consent for assets installed without an `[assets]` entry (§2.2). |

- **Ids.** Asset and bundle ids are 1–64 characters of `a-z`, `0-9` and `-`, with no leading, trailing or doubled `-`. For example, `secret-guard` is valid, and `-guard` and `secret--guard` are not. Tags follow the same rules, with at most 32 characters.
- **Unknown fields are errors**, at the top level and in every `[assets]` table. A misspelled key fails with its line and column and the list of keys that are allowed (`project/config-parse`, exit 3).
- **Values are checked while the file is parsed**, so a bad id, permission, version requirement or URL fails the same way.
- **Duplicates** in `clients`, `bundles`, `tags` or `exclude` are errors (`config/clients`, `config/bundles`, `config/tags`, `config/exclude`).
- **`runtime = "docker"` is not a field.** [ADR-0002](adr/0002-mcp-server-and-tool-packaging.md), the decision on how tools and MCP servers are packaged, proposed it to write an MCP server's entry as a `docker run` command instead of a local binary. It was deferred past v1, the first version of Grimoire ([ADR-0011](adr/0011-container-image.md), the container image decision): no MCP client offers a common variable for the project directory, and the CLI can't embed the digest (the content hash) of the image that contains it. So it fails as an unknown field. [`container.md`](container.md) shows hand-written `docker run` entries instead.

### 1.3 `[assets]` entries

Each entry selects one asset by id. The value is a version requirement, or a table that holds the requirement and other options. A table can be written inline, inside `{ }` on one line, or as its own `[assets.<id>]` section. Both forms work:

```toml
[assets]
adr-writer = "^0.2.0"
security-reviewer = { version = "^0.2.0", allow = ["filesystem-read"] }

[assets.secret-guard]
version = "^0.1.0"
allow = ["filesystem-read", "hooks"]
```

| Key | Type | Default | Meaning |
|---|---|---|---|
| `version` | version requirement | required | The versions of the asset you accept (§1.5). |
| `clients` | array of client ids | every project client the asset supports | Write the asset only for these clients. Each must be in the top-level `clients` (`config/asset-clients`) and supported by the asset (`resolve/unsupported-client`). An empty list is an error. |
| `allow` | array of permissions | `[]` | The permissions you accept for this asset (§2). |
| `transport` | `"stdio"` or `"streamable-http"` | `"stdio"` | MCP servers only (§1.6). |
| `url` | URL | none | MCP servers only, and only with `transport = "streamable-http"` (§1.6). |
| `env` | table of variable name = variable name | empty | MCP servers over stdio only: read a server's variable from another name in your environment (§1.6). |

- `transport`, `url` and `env` on a rule, skill, agent or tool are errors (`resolve/options-kind`).
- An `[assets]` entry for a bundle id is an error that tells you to list it under `bundles` (`resolve/unknown-asset`). A `bundles` value that names an asset gets the opposite hint (`resolve/unknown-bundle`).
- An asset in both `[assets]` and `exclude` is an error (`config/exclude`).

**Which clients an asset is written for:**
- Without `clients`, an asset is written for every project client it supports.
- For a project client it doesn't support, `grimoire plan` (which shows what `apply` would change) warns (`resolve/client-skipped`).
- When the client has no place for that kind of asset at all, the warning is `resolve/client-not-covered` instead. Examples are hooks outside Claude Code (hooks are commands a client runs automatically on events, such as before each tool call) and agents in Codex.
- `agents-md` never holds agents, MCP servers or hooks, and gets no warning for them.
- If the asset supports none of the project's clients, resolution fails (`resolve/unsupported-client`). Resolution is the step that works out which assets, at which versions, the project gets, before anything is written.
- A `clients` list in the entry turns these warnings off, and makes a client the asset doesn't support an error.

### 1.4 How assets are selected

An asset is installed when anything selects it. The lock (`grimoire.lock`, §3) records every reason in `selected-by`.

| Selected by | Recorded as | Version requirement |
|---|---|---|
| an `[assets]` entry | `direct` | the entry's `version` |
| a bundle in `bundles` | `bundle:<id>` | the one the bundle gives that member |
| a tag in `tags` | `tag:<tag>` | none: whatever version the catalog has, pre-releases included |
| another installed asset's `requires` | `requires:<id>` | the one that asset's manifest gives |

An asset's `requires` lists the other assets it depends on, and its manifest is its `asset.toml`, the file that describes it.

- **Tags** select every asset with the tag, except deprecated assets (assets being retired in favor of a named replacement) and assets that support none of the project's clients (a warning, `resolve/tag-skipped`). A tag that no asset carries is a warning (`resolve/unknown-tag`).
- **A tag is a moving selection.** A later grimoire release may carry more assets with that tag, and `plan` then shows them as new.
- **`exclude`** drops an asset that a bundle or tag selects. An asset that another selected asset requires can't be excluded (`resolve/excluded-dependency`).
- **Deprecated assets** still install when you, a bundle or `requires` select them, with a warning that names the replacement (`resolve/deprecated`). How long a deprecated asset stays in the catalog is in [`versioning.md`](versioning.md).
- **Conflicts.** Two selected assets stop resolution when either declares a conflict with the other, and the error says what selected each one (`resolve/conflict`).
- `grimoire show <id>` prints an asset's permissions, outputs, dependencies and conflicts, and `grimoire list` searches the catalog.

### 1.5 Version requirements and pins

- **Requirements are written as in Cargo, Rust's package manager.** Versions follow semantic versioning (SemVer): three numbers, MAJOR.MINOR.PATCH, such as `1.4.2`. A requirement says which versions you accept, in one of these forms: `^0.2.0`, `0.2` (the same as `^0.2`), `~1.4` (any `1.4.x`), `>=1, <3`, `=1.2.3` (exactly that version) or `*` (any version but a pre-release, below).
- **`^` keeps the left-most non-zero part**, as in Cargo: `^0.2.0` accepts `0.2.x` but not `0.3.0`, and `^1.2` accepts every `1.x` from `1.2.0`. A requirement that starts with `^` is called a caret requirement.
- **Pre-releases.** A pre-release is a version with a suffix, such as `1.0.0-rc.1`, that comes before the release it names. A requirement accepts a pre-release only when it names a pre-release of that same version; `*` doesn't.
- **One version per asset.** Each grimoire release embeds exactly one version of every asset ([ADR-0001](adr/0001-catalog-delivery.md), which builds the catalog into the binary). Resolution never picks another one: a requirement that the embedded version doesn't meet is an error (`resolve/version`, exit 3).
- **`min-cli-version`.** Each asset's manifest names the oldest grimoire that can install it. An asset whose `min-cli-version` is newer than the running grimoire is an error (`resolve/cli-too-old`).
- **The lock pins exact versions.** Once an asset is in `grimoire.lock`, `plan` and `apply` keep its locked version and manifest. When the running grimoire carries another version, or the same version with a different manifest, they stop and name the command to run (`resolve/locked-version-unavailable`, `resolve/locked-manifest-changed`, exit 3). This happens after you switch to a newer or older grimoire release. An asset that isn't in the lock yet resolves to the catalog's version.
- **`grimoire update [ID]...`** moves the named installed assets, or all of them when you name none, to the version the running grimoire carries, then applies. `--no-apply` writes only the lock and leaves the files for a later `grimoire apply`.
- **The new version must still fit.** It must meet the requirement in `grimoire.toml`, its `min-cli-version` and the permissions you accepted (§2.4). If it doesn't, nothing is written. An id that isn't in the lock is an error (`update/unknown`). See [`cli.md`](cli.md#grimoire-update).
- **`update` never edits `grimoire.toml`**, and `grimoire add` leaves an existing entry's requirement alone. To accept a version your requirement excludes, such as `0.3.0` under `^0.2.0`, edit the requirement, then run `grimoire update`.
- `update` moves a pin to whatever the running grimoire carries, so an older grimoire moves it back. `grimoire.lock` never records the CLI version, so upgrading grimoire doesn't by itself change the lock ([ADR-0001](adr/0001-catalog-delivery.md)).

### 1.6 MCP server options

By default a client starts an MCP server itself, over stdio: it runs the server as a child process and talks to it through the server's standard input and output. The entry names the server's binary, which must be in a directory on your `PATH` (the environment variable that lists where programs are found). Grimoire never installs the binary; `grimoire doctor` checks that it is there.

**Streamable HTTP**, MCP's transport over HTTP, works the other way round. With `transport = "streamable-http"`, you start the server, and the client connects to it at `url`:

```toml
[assets]
project-docs = { version = "^0.1.0", allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"], transport = "streamable-http", url = "http://127.0.0.1:8931/mcp" }
```

- `url` is required with `streamable-http` (`config/url-missing`) and refused without it (`config/url-transport`).
- The URL must be `https://`, or `http://` for this machine only: `localhost`, `127.0.0.0/8` (any address that starts with `127.`) or `[::1]` (the IPv6 address of this machine).
- It can't hold user information (`user@`), a query (`?…`) or a fragment (`#…`), so no credential can reach a client's configuration through it. The MCP specification also forbids access tokens in URLs.
- It can't hold `$` or `{`, which a client would expand: it would replace a reference such as `${NAME}` with the value of a variable.
- It must be ASCII, and at most 2048 characters long.
- Grimoire writes no headers and no tokens. A server that needs sign-in negotiates it with the client.
- `env` is refused with `streamable-http` (`config/env-transport`): the client doesn't start the server, so it passes no environment.
- The permissions are the same for either transport.
- [`container.md`](container.md) shows how to run the catalog's servers from the container image, and [`project-docs`'s README](../catalog/mcp-servers/project-docs/README.md) its own options.

**Renaming environment variables.** A stdio entry forwards to the server each environment variable that the server's manifest lists for forwarding (its `[mcp-server] env`). It forwards each one under the same name and by reference: the entry holds a reference that the client fills in from your environment, never the value.

When your environment holds the value under another name, map it. No server in today's catalog forwards a variable, so this example uses a made-up `docs-index` that reads `DOCS_TOKEN`:

```toml
[assets]
docs-index = { version = "^2", allow = ["env:DOCS_TOKEN"], env = { DOCS_TOKEN = "GH_TOKEN" } }
```

The key is the server's variable, and the value is the name in your environment: here the server reads `DOCS_TOKEN`, and gets the value of your `GH_TOKEN`. The permission grant (§2) stays `env:DOCS_TOKEN`, under the server's name. Each client writes the reference its own way:

| Client | Written |
|---|---|
| `claude-code`, `copilot` (`.mcp.json`), `gemini-cli` | `"DOCS_TOKEN": "${GH_TOKEN}"` |
| `cursor` | `"DOCS_TOKEN": "${env:GH_TOKEN}"` |
| `codex` | refused (`adapter/env-remap`): Codex forwards a variable only under its own name. Set the variable under the server's name, or leave `codex` out of the asset's `clients`. |

- A key the server doesn't read is an error (`adapter/env-unknown`), and so is a variable the client sets itself, such as `CLAUDE_PROJECT_DIR` for `project-docs`.
- Grimoire writes only names, never values.
- For Claude Code, Copilot and Gemini CLI, a variable the manifest marks optional gets an empty default, `${NAME:-}`. Without a default, an unset reference reaches the server as its own literal text.

### 1.7 What `grimoire add` and `grimoire remove` write

`add` and `remove` edit `grimoire.toml` in place and keep your comments and formatting. Both resolve the edited file before they write it: when it wouldn't resolve, the file stays as it was (`add/unresolvable`, `remove/unresolvable`, exit 3). With `--apply`, they then run `grimoire apply`.

| Command | What it writes |
|---|---|
| `grimoire add <id>` | `<id> = "^<catalog version>"` under `[assets]`. An entry that already exists is left alone. |
| `grimoire add bundle:<id>`, or `add <id>` for a bundle | the id, appended to `bundles` |
| `grimoire add tag:<tag>` | the tag, appended to `tags` |
| `grimoire add … --allow <list>`, or a yes at the prompt | the permissions still missing (§2.3): in the asset's `[assets]` entry, turning `id = "^1"` into an inline table, or under `[allow]` for an asset installed another way |
| `grimoire remove <id>` | deletes the `[assets]` entry, with its grant |
| `grimoire remove bundle:<id>`, `remove tag:<tag>` | deletes the value from `bundles` or `tags`; an emptied array stays as `[]` |

- When an asset gets an `[assets]` entry, `add` moves its grant there from `[allow]`.
- `add` refuses an asset that is in `exclude` (`add/excluded`).
- `remove <id>` refuses an asset that has no `[assets]` entry but is installed through a bundle, a tag or `requires` (`remove/indirect`): add it to `exclude` instead.
- Removing the `[assets]` entry of an asset that something else still selects keeps it installed, moves its grant to `[allow]`, and warns (`remove/still-selected`).
- `remove` leaves the `[allow]` entries of a removed bundle's members. `plan` then warns that they have no effect (`resolve/allow-unused`); delete them by hand.
- `grimoire init` writes a new file: `format = 1`, the clients you name with `--client` or the ones it detects (§4.1), and an empty `[assets]` table. It refuses to replace an existing `grimoire.toml` (`init/exists`).

Everything else in the file is yours to edit. `grimoire plan` shows what an edit would change. See [`cli.md`](cli.md#grimoire-add) for the commands.

## 2. Permissions and consent

Every asset declares what it can cause at run time, as a list of permissions. You accept each declared permission in `grimoire.toml` before Grimoire installs the asset. Until you do, resolution fails (`resolve/permission`, exit 3) and nothing is written. Rules are plain text and declare none.

### 2.1 Permission names

| Grant | The asset can | For example |
|---|---|---|
| `filesystem-read` | read files | an agent with the `read` or `search` tools; `secret-guard`; `project-docs` |
| `filesystem-write` | read and write files. It covers `filesystem-read` | an agent with the `edit` tool |
| `network` | use the network | an agent with the `web` tool |
| `shell` | run shell commands or other programs | an agent with the `execute` tool; a skill that bundles scripts |
| `hooks` | register client hooks, which run automatically | a tool with hooks, such as `secret-guard` |
| `env:<NAME>` | read the environment variable `<NAME>`: one grant per variable | an MCP server that reads a token; `project-docs` reads `CLAUDE_PROJECT_DIR` |

- `<NAME>` is 1–128 characters of `A-Z`, `0-9` and `_`, and doesn't start with a digit. A bare `env` is refused in `grimoire.toml`.
- Each variable is its own grant, so a new version that reads one more variable needs your consent again.
- An agent's tools need matching permissions: `read` and `search` need `filesystem-read`, `edit` needs `filesystem-write`, `execute` needs `shell` and `web` needs `network`. Catalog validation, which checks every asset in the catalog, enforces this.
- A grant is consent, not a sandbox (a sandbox would limit what the program can do). Grimoire checks it before it installs anything, and can't limit what a binary or a client does afterward. What each client enforces of an agent's tool list is in [`security-model.md`](security-model.md).
- `grimoire show <id>` prints an asset's permissions and the variables it reads.

### 2.2 Where consent is recorded

Each asset's grants live in exactly one place:
- for an asset with an `[assets]` entry, that entry's `allow` list;
- for an asset installed through a bundle, a tag or `requires`, the `[allow]` table, under the asset's id.

```toml
[assets]
project-docs = { version = "^0.1.0", allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"] }

[allow]
secret-guard = ["filesystem-read", "hooks"]
```

- An `[allow]` entry for an asset that has an `[assets]` entry is an error (`config/allow-direct`), and so is an empty one (`config/allow-empty`). An `[allow]` entry for an asset that isn't installed is a warning (`resolve/allow-unused`).
- `grimoire.lock` records, as each asset's `allow`, the permissions the installed version needs, all of which you accepted. A change there in a lock diff is a permission change.
- `grimoire.toml` is part of your repository, so a new grant is reviewed like any other change.

### 2.3 Giving consent

- `grimoire add` prints what each asset it adds still needs. In a terminal it then asks, once per asset, for example ``Allow filesystem-read, hooks for `secret-guard`? [y/N]``. Only `y`, `Y`, `yes`, `Yes` or `YES` accepts, and spaces around the answer are ignored.
- There is no prompt, and the answer is no, in four cases: when standard input (stdin) isn't a terminal; when the `CI` environment variable is set to a non-empty value, as continuous integration (CI) services usually do; with `--no-input`; and with `--json`. Pass the grants instead: `grimoire add secret-guard --allow filesystem-read,hooks`.
- `--allow` takes a comma-separated list and can be repeated. On the command line only, a bare `env` accepts every variable the named assets read now, and `grimoire.toml` records each one by name.
- `add` records only the permissions that are still missing, so an `--allow` value that no named asset needs is not written.
- Consent reaches only what the command names: the named assets, the members of named bundles, the assets of named tags, and what they require. Another asset in `grimoire.toml` that lacks consent still fails the command.
- `grimoire add <id> --allow …` also works for an asset that is already listed: it adds the missing grants and changes nothing else. For assets that a bundle or tag brings in, use `add bundle:<id> --allow …` or `add tag:<tag> --allow …`.
- When a grant is declined, `grimoire.toml` is not changed (`add/permission`, exit 3).

### 2.4 When an update needs more

A new version that declares a permission you haven't accepted stops `grimoire update` and `grimoire apply` (`resolve/permission`, exit 3). Nothing is written, not even the lock. There is no silent escalation: an update never gains permissions without your consent (threat F14 in [`security-model.md`](security-model.md)).

The error names the asset, what selected it, the missing grants and how to accept them:

```text
error[resolve/permission] grimoire.toml: `secret-guard` (from bundle `security-baseline`) needs permissions you have not accepted: filesystem-read. Review them with `grimoire show secret-guard`, then run `grimoire add bundle:security-baseline --allow filesystem-read`, or set `secret-guard = ["filesystem-read", "hooks"]` under [allow] in grimoire.toml
```

1. Review what the new version does: `grimoire show <id>` prints its permissions and the latest section of its CHANGELOG.
2. Accept the new grants with the command the error names, or by editing `grimoire.toml`.
3. Run `grimoire update` again.

Taking a grant out of `grimoire.toml` has the same effect: the next `plan` or `apply` stops until you put it back or remove the asset.

## 3. `grimoire.lock`

`grimoire.lock` records exactly what Grimoire installed and what it owns on disk. It is generated: `apply` and `update` write it.
- The first `apply` creates it even when nothing is selected.
- Removing the last asset leaves a lock holding only `format = 1`, which `grimoire uninstall` deletes.
- It starts with `# Generated by grimoire. Do not edit by hand.`

Its JSON Schema is [`schemas/grimoire-lock.schema.json`](../schemas/grimoire-lock.schema.json). An excerpt, with digests (the SHA-256 hashes) shortened:

```toml
# Generated by grimoire. Do not edit by hand.
format = 1

[[asset]]
id = "secret-guard"
kind = "tool"
version = "0.1.0"
source = "builtin"
manifest = "sha256:…"
selected-by = ["bundle:security-baseline"]
allow = ["filesystem-read", "hooks"]

[[asset.output]]
clients = ["claude-code"]
path = ".claude/settings.json"
unit = "entry"
key = "/hooks/PreToolUse|secret-guard#0"
hash = "sha256:…"
created-file = true
created-keys = ["/hooks", "/hooks/PreToolUse"]

[[output]]
clients = ["claude-code"]
path = "CLAUDE.md"
unit = "block"
hash = "sha256:…"
created-file = true
```

In TOML, each `[[name]]` header starts one more element of a list of tables.

| Field | Meaning |
|---|---|
| `format` | The lock format, `1`. |
| `[[asset]]` | One per installed asset, sorted by id. |
| `id`, `kind`, `version` | The asset, and the exact version installed. |
| `source` | Where it came from. `builtin`, the catalog embedded in grimoire, is the only source in v1 ([ADR-0001](adr/0001-catalog-delivery.md)). |
| `manifest` | The SHA-256 of the asset's `asset.toml`, as installed. |
| `selected-by` | Why it is installed: `direct`, `bundle:<id>`, `tag:<tag>` or `requires:<id>` (§1.4). |
| `allow` | The permissions this version needs, all accepted (§2.2). Left out when there are none. |
| `[[asset.output]]` | One per unit of content Grimoire owns for the asset. |
| `clients` | The clients the output serves; several where clients share a location (§4.5). |
| `path` | The file, relative to the project root, with `/` separators. |
| `unit` | `file` (the whole file), `block` (a managed block: the text between Grimoire's marker lines in a Markdown file, §4.3) or `entry` (one key or array element in a configuration file, §4.4). |
| `key` | For an `entry`: a JSON Pointer (RFC 6901), a path to one value with `/` before each key, such as `/mcpServers/project-docs`. `/hooks/PreToolUse\|secret-guard#0` is the form for one hook element; the text after `\|` only keeps the keys distinct. |
| `hash` | The SHA-256 of what Grimoire owns there: the file, the block's text or the entry's value. |
| `created-file` | For a `block` or `entry`: Grimoire created the file, so it deletes the file once nothing else is left in it. |
| `created-keys` | For an `entry`: the containers (the objects or arrays that hold the entry, such as `/hooks` above) that Grimoire created, which it removes again once they are empty. |
| `[[output]]` | Content Grimoire owns for itself rather than for an asset: the `@AGENTS.md` import blocks in `CLAUDE.md` and `GEMINI.md`, whose `@AGENTS.md` line makes Claude Code and Gemini CLI read `AGENTS.md` too (§4.5). |

- **Don't edit it.** Grimoire validates the lock when it reads it. A record for a path Grimoire doesn't manage, or one that claims more than Grimoire owns there (a whole `.codex/config.toml`, for example), is refused before anything is read or written (`project/lock-path`, exit 3). A lock that doesn't parse, or has an unknown field, is refused too (`project/lock-parse`).
- **Commit it**, with `grimoire.toml` and the files Grimoire writes. The lock has three jobs: it is how `apply` tells your edits from its own content, how every asset stays at its version until `update`, and what `verify` checks.
- **If it is lost**, `apply` adopts the content that already matches what it would write (records it as Grimoire's, §6), and reports anything else in the way as a conflict.
- **Review its diffs like code.** A lock diff shows every version, permission and output change. A new `allow` value is new consent, and a new `path` is a new file in your repository.
- **Line endings.** The lock is written with `\n` line endings, and reads the same on every operating system: paths use `/` and records are sorted.
  - A checkout that converted it to `\r\n` (Windows line endings), for example with Git's `core.autocrlf` setting, holds the same lock, and Grimoire doesn't rewrite it for that alone.
  - Hashes of text fold `\r\n` to `\n` too, treating both as the same. So a managed file whose line endings were converted is not drift (managed content that changed since Grimoire wrote it, §6), and Grimoire leaves it as it is (threat F13 in [`security-model.md`](security-model.md), line-ending conversion mistaken for drift).
- **`grimoire verify`** reads only the lock, not `grimoire.toml` or the catalog. It checks every recorded output against the disk: each file's hash, each block's text and each entry's value.
  - It exits 5 and lists every output that was edited or is missing, or whose file it can't parse (`verify/mismatch`). Without a lock it exits 3 (`verify/no-lock`).
  - Because it doesn't depend on which asset versions the running grimoire carries, it is a good CI check for hand edits to managed content.
- **`grimoire plan --check`** is the stricter check: it exits 5 when `apply` would change anything, the lock included, so it also catches a `grimoire.toml` change that nobody applied. See [`cli.md`](cli.md#grimoire-verify) for both.

## 4. Clients

### 4.1 The six clients

| Id | Client | `grimoire init` detects it from |
|---|---|---|
| `claude-code` | Claude Code | `CLAUDE.md` or `.claude` |
| `agents-md` | the portable target, `AGENTS.md` and `.agents/skills/`, which many clients read | `AGENTS.md` or `.agents` |
| `codex` | the Codex CLI | `.codex` |
| `cursor` | Cursor | `.cursor` |
| `gemini-cli` | Gemini CLI | `GEMINI.md` or `.gemini` |
| `copilot` | GitHub Copilot | `.github/copilot-instructions.md`, `.github/instructions` or `.github/agents` |

`grimoire init --client <id>`, repeated as needed, names the clients instead.

When you change `clients` later, `apply` rewrites the outputs to match. For example, going from `clients = ["claude-code", "agents-md"]` to `clients = ["claude-code"]` moves each rule from `AGENTS.md` to `.claude/rules/` and removes the `CLAUDE.md` import (§4.5).

### 4.2 What Grimoire writes for each kind

This is where each client's content goes, when the asset supports that client. "Or `AGENTS.md`" means the rule is written once, as its `AGENTS.md` block, when that block is written (§4.5). `grimoire show <id>` prints one asset's destinations, and `grimoire plan` the final paths.

| Kind | `claude-code` | `agents-md` | `codex` | `cursor` | `gemini-cli` | `copilot` |
|---|---|---|---|---|---|---|
| Rule | `.claude/rules/<id>.md`, or `AGENTS.md` through `CLAUDE.md` | an `AGENTS.md` block | the same `AGENTS.md` block | `.cursor/rules/<id>.mdc`, or `AGENTS.md` | a `GEMINI.md` block, or `AGENTS.md` through `GEMINI.md` | `.github/instructions/<id>.instructions.md`, or `AGENTS.md` |
| Skill | `.claude/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` or `.claude/skills/<id>/` (§4.5) | `.agents/skills/<id>/` | `.agents/skills/<id>/` or `.claude/skills/<id>/` (§4.5) |
| Agent | `.claude/agents/<id>.md` | – | – | `.cursor/agents/<id>.md`, only for an agent with exactly `read` and `search` | `.gemini/agents/<id>.md` | `.github/agents/<id>.agent.md` |
| MCP server | `.mcp.json`, `mcpServers.<id>` | – | `.codex/config.toml`, `[mcp_servers.<id>]` | `.cursor/mcp.json`, `mcpServers.<id>` | `.gemini/settings.json`, `mcpServers.<id>` | `.mcp.json`, the same entry as `claude-code` |
| Tool hooks | `.claude/settings.json`, one `hooks.<Event>` element per binding | – | – | – | – | – |

- **Rules** are written as the rule's text.
  - An always-on rule (one that applies everywhere) gets `alwaysApply: true` for Cursor and `applyTo: "**"` for Copilot.
  - A path-scoped rule applies only to files that match its globs, file patterns such as `src/**/*.rs`. It gets `paths` front matter (the metadata block at the top of the file) for Claude Code, `alwaysApply: false` with `globs` for Cursor, and its globs as `applyTo` for Copilot.
  - Cursor and Copilot take one comma-separated glob list, so a glob that contains a comma can't be written in their own files (`adapter/unsupported`); it can when the rule goes to `AGENTS.md`.
  - `AGENTS.md` and `GEMINI.md` have no path scoping, so a path-scoped rule's block starts with a line that names the files it applies to.
- **Skills** are copied byte for byte, keeping their layout (`SKILL.md`, and any `references/`, `assets/` and `scripts/`).
- **Agents** get a `name`, a `description` and the client's form of the tool list, the tools the agent may use:
  - Claude Code and Gemini CLI get an explicit `tools` list.
  - Copilot gets a `tools` list with `strict-tools-list: true`, because Copilot CLI otherwise adds default tools of its own.
  - Cursor has no tool list, only `readonly: true`, which fits an agent with exactly `read` and `search`; any other agent is refused for Cursor (`adapter/unsupported`).
  - Codex gets no agents: it can't limit an agent's tools ([ADR-0008](adr/0008-phase-4-client-adapters.md)).
  - Where a client limits them only partly, `plan` warns (`adapter/agent-readonly`, `adapter/agent-strict-tools`).
- **MCP servers** over stdio name the server's binary and refer to each variable its manifest says to forward, never to a value: `${NAME}` for Claude Code, Copilot and Gemini CLI, `${env:NAME}` for Cursor, and a list of names in `env_vars` for Codex. Over Streamable HTTP the entry holds the `url`, with `"type": "http"` for Claude Code, Copilot and Gemini CLI.
- **Tool hooks** are written in exec form, never as a shell string: the binary as `command` and its arguments as `args`, so that no shell interprets them.
  - A binding's `timeout` is written when it has one.
  - `"onFailure": "block"` is written when the binding fails closed: when the hook can't run, for example because its binary is missing, the client blocks the event instead of letting it go ahead.
  - Hooks are written for Claude Code only, because none of the other clients can guarantee that a failed hook blocks the event, and none of them could be tested ([ADR-0008](adr/0008-phase-4-client-adapters.md)).
  - A tool without hooks writes nothing for any client: the lock records it, and `doctor` checks its binary.
- **Binaries.** Grimoire never installs a tool or MCP server binary. Each entry names one that must be on `PATH`. `plan` warns when a fail-closed hook's binary is missing, because the client would then block every event the hook covers (`plan/hook-binary-missing`).

### 4.3 Managed blocks in shared files

`AGENTS.md`, `CLAUDE.md` and `GEMINI.md` are shared with you: you can write in them too. Grimoire owns only the text between its marker lines, which are HTML comments:

```markdown
<!-- grimoire:begin conventional-commits -->
...the rule's text...
<!-- grimoire:end conventional-commits -->
```

- A block is named after its asset. The name `grimoire` is reserved for Grimoire's own block, which holds the `@AGENTS.md` import (the line that makes the client read `AGENTS.md` as well).
- `CLAUDE.md` only ever gets that import block. `GEMINI.md` gets rule blocks, the import block, or both.
- Everything outside the blocks is yours and is kept byte for byte. Write above, between and below the blocks as you like. An edit inside a block is drift (§6).
- A new block goes among the existing blocks in name order, or after a blank line at the end of the file.
- Markers count only at the start of a line, and not inside a fenced code block (a block between fence lines of three or more backticks or tildes) in your text, so a file can show them as an example.
- Grimoire refuses to edit a file whose markers are broken (malformed, unterminated, nested or repeated), even with `--force` (exit 4), because it never guesses what it owns. Fix the markers by hand.
- Grimoire won't append a block to a file that ends inside an unclosed code fence, where the block would become part of the code.
- In a file that uses `\r\n`, Grimoire writes its lines with `\r\n` too, and it keeps a byte-order mark (an invisible character that some editors put at the start of a file).

### 4.4 Entries in configuration files

In the five configuration files below, Grimoire owns single entries and leaves everything else alone: other servers, other hooks, other settings, key order, and formatting outside the object or array it edits.

| File | Syntax Grimoire accepts | What it owns |
|---|---|---|
| `.mcp.json`, `.cursor/mcp.json` | strict JSON | `mcpServers.<id>` |
| `.claude/settings.json` | strict JSON | one element of a `hooks.<Event>` array per hook binding, found by its exact value |
| `.gemini/settings.json` | JSON with comments (JSONC), as Gemini CLI reads it; trailing commas are refused, as Gemini CLI refuses them | `mcpServers.<id>` |
| `.codex/config.toml` | TOML | the `[mcp_servers.<id>]` table |

- Grimoire refuses a file it can't read that way, even with `--force` (exit 4). For example: a comment in `.mcp.json`, a duplicate key on Grimoire's path (the keys that lead to its entry), or TOML that doesn't parse. It never "fixes" a file that a client might not read.
- Comments in a TOML or JSONC file are kept. A comment you add *inside* Grimoire's entry is a conflict, because rewriting the entry would lose it: move it out of the entry, or use `--force` to drop it.
- Inserting into a JSON object written on one line spreads that object over several lines. The values stay the same.
- Grimoire removes a container it created (`created-keys`) once it is empty, and a file it created (`created-file`) once nothing else is in it.
- A file or container that was there before Grimoire wrote into it stays, even when it is left empty.
- Adoption is the exception (§6). When every entry Grimoire wants in a file is already there, and the lock records none of them, Grimoire counts each container that holds nothing but those entries as created by it, and the file too if it holds nothing else, as it would after an interrupted `apply`.

### 4.5 Shared rules and skills

Clients read each other's files. So where several selected clients can read one location, Grimoire writes the content there once and lists all of them in the lock's `clients` ([ADR-0008](adr/0008-phase-4-client-adapters.md), [`architecture.md`](architecture.md) §8.2). This is decided per asset, from the clients that asset is installed for.

**Rules.** When `agents-md` or `codex` is one of a rule's clients, the rule is written once, as its `AGENTS.md` block, and:
- `claude-code` gets no `.claude/rules/` file: it reads `AGENTS.md` through Grimoire's `@AGENTS.md` block in `CLAUDE.md`;
- `gemini-cli` gets no rule block: it reads `AGENTS.md` through the same import block in `GEMINI.md`;
- `cursor` and `copilot` get no rule file: they read `AGENTS.md` themselves.

Otherwise each client gets its own file (§4.2). A path-scoped rule in `AGENTS.md` keeps its scope line, but Claude Code, Cursor and Copilot then apply it everywhere.

**Skills:**
- `.claude/skills/` is written when `claude-code` is one of the skill's clients, and `.agents/skills/` when `agents-md`, `codex` or `gemini-cli` is.
- Cursor and Copilot read both. Their copy goes to `.agents/skills/` when that is written, else to `.claude/skills/` when that is, else to `.agents/skills/`.
- So `claude-code` with `cursor` gets one copy, in `.claude/skills/`, while `claude-code`, `agents-md` and `cursor` need both, and Cursor then lists the skill twice.

**MCP servers.** `claude-code` and `copilot` share one `.mcp.json` entry.

**What placement can't avoid, `plan` reports as warnings:**
- a rule or skill that a selected client may read twice (`plan/duplicate-rule`, `plan/duplicate-skill`);
- an `AGENTS.md` rule that Claude Code or Gemini CLI reads although it isn't installed for them (`plan/cross-client-rule`);
- an agent, hook or MCP server that Cursor, Copilot or Claude Code also loads from another client's file (`plan/cross-client-agent`, `plan/cross-client-hooks`, `plan/cross-client-mcp`).

Installing for one client can grant something in another: for example, Cursor and Copilot run the hooks Grimoire writes to `.claude/settings.json`. [`security-model.md`](security-model.md) covers what that means.

### 4.6 Steps only you can take

Several clients read project files only after you trust the project or approve a server. Grimoire can't check or grant that. `grimoire doctor` prints the step for each selected client:

| Client | What it needs |
|---|---|
| `claude-code` | `doctor` prints nothing. Claude Code asks before it uses `.mcp.json` servers in an interactive session (`CC-MCP`, the entry on Claude Code's project MCP configuration in [`specs.md`](specs.md)). |
| `agents-md` | Nothing. |
| `codex` | Codex reads `.codex/config.toml`, and so its MCP servers, only in a project you have marked trusted. When a root `AGENTS.override.md` exists, Codex reads it instead of `AGENTS.md`; `doctor` says so, and `plan` warns (`plan/codex-override`). Codex reads at most 32 KiB (32 × 1,024 bytes) of project instructions in total by default, so `plan` warns from 24 KiB of `AGENTS.md` (`plan/codex-instructions-size`). |
| `cursor` | Cursor asks you to approve each project MCP server. |
| `gemini-cli` | Gemini CLI reads project files only in a folder you have trusted. It finds `GEMINI.md` from any directory in the repository, but reads `.gemini/` (and so its MCP servers and agents) and `.agents/skills/` only when started at the project root. It asks you to acknowledge each project agent again after every change. |
| `copilot` | Copilot reads `.mcp.json` only after you trust the folder. Its cloud agent reads MCP servers only from the repository's settings. For the MCP servers installed for `copilot`, `doctor` prints the JSON to paste there, names each variable as a `COPILOT_MCP_*` secret, and says to install the local servers' binaries in `copilot-setup-steps.yml`. |

`doctor` also checks that the binary of each tool and MCP server in the lock is on `PATH`, except a server configured over Streamable HTTP. It exits 5 when a check fails. See [`cli.md`](cli.md#grimoire-doctor).

## 5. What Grimoire never touches

- **Anything outside the project root** (the directory that holds `grimoire.toml`), including your home directory and every client's user-level settings.
  - Grimoire reads and writes project files only through a handle on the root: an open handle to the root directory that refuses any path leading out of it. It refuses (exit 6) rather than follow a symbolic link at or above a path it manages.
  - Outside the root it only looks for binaries in the `PATH` directories (`doctor`, `plan/hook-binary-missing`).
  - A few checks only ask whether a path exists: `init` and `doctor` look for each client's files (§4.1), and, when `codex` is a client, the commands that plan look for `AGENTS.override.md`. They ask through the same handle and read no content.
  - A symbolic link at one of those names counts as there, and one above it is not followed: with `.github` linked to a folder, `init` doesn't detect `copilot` from the files in it.
- **Any path but its own locations.** Besides `grimoire.toml`, `grimoire.lock` and the `<path>.new` files of `--write-new`, Grimoire writes only in the folders `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `.cursor/rules/`, `.cursor/agents/`, `.gemini/agents/`, `.github/instructions/` and `.github/agents/`, and to the files `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.mcp.json`, `.claude/settings.json`, `.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`. It never writes `.gitignore`, `.github/copilot-instructions.md` or `AGENTS.override.md`.
- **What it didn't write**, even in those locations: your text outside its blocks, other keys and array elements, and other files in those folders.
- **Your edits to its own content**, unless you pass `--force` (§6).
- **Files and keys that were there before it**, unless it took them over. Removing an asset removes only Grimoire's block or entry from them.
  - Grimoire takes content over in two cases (§6): it adopts content that already matches what it would write, and `--force` overwrites content in its way.
  - From then on that file, block or entry is Grimoire's, and removing the asset deletes it. An adopted shared file that holds nothing but Grimoire's blocks or entries is deleted with them.
- **Your directories**, unless they are left empty. After deleting a file, Grimoire removes each directory above it that is now empty, even one you created.
- **Secret values.** Entries refer to environment variables by name, and a `url` can't carry credentials.
- **Binaries.** It never downloads, installs or runs a tool or MCP server, and installing an asset runs nothing from it. The install path has no network code.
- **Trust and approval** in any client (§4.6).

## 6. Drift and conflicts

Drift is managed content that changed on disk since Grimoire wrote it. For each unit it owns, Grimoire compares three things: what is on disk, what the lock recorded, and what it would write now ([`architecture.md`](architecture.md) §7.3).

| What `plan` finds | What `apply` does |
|---|---|
| on disk matches the lock, and the new content differs | updates it |
| on disk is already what Grimoire would write | nothing, or records it in the lock ("adopt"), which makes it Grimoire's: removing the asset later deletes it |
| edited since Grimoire wrote it | conflict |
| deleted since Grimoire wrote it | conflict |
| content Grimoire doesn't manage is in the way | conflict |
| no longer wanted, and unedited | deletes it |
| no longer wanted, but edited | conflict |

- **Any conflict stops `apply` before it writes anything** (`apply/conflict`, exit 4). `plan` shows each conflict, with a diff, and exits 4 too.
- **`--force`** overwrites the conflicting content, and deletes edited content that is no longer wanted. What it overwrites is Grimoire's from then on, even an unmanaged file or entry that was in the way, so removing the asset later deletes it.
- **`--write-new`** leaves each conflicting file alone and writes what Grimoire would write beside it, as `<path>.new` (the whole file, for blocks and entries). It then exits 4 (`apply/conflict-remains`).
  - The lock keeps the old records for those files, so the conflict is reported until you merge the `.new` file or use `--force`.
  - It never replaces a different `<path>.new` that is already there (`apply/sidecar-exists`).
- `--force` and `--write-new` can't be combined.
- **A file Grimoire can't read safely** (broken markers, invalid JSON or TOML, text that isn't UTF-8) is refused even with `--force` (exit 4). Fix it by hand.
- **`grimoire uninstall`** removes nothing while any managed content is edited (exit 4). `--force` removes the edited content too.
- **`grimoire uninstall --purge`** also deletes `grimoire.toml` if there is one. It checks the file first, so a symbolic link, a special file (such as a device or a named pipe) or a directory in its place stops `uninstall` before anything is removed.
- `grimoire verify` and `grimoire plan --check` report drift without writing anything (§3).

[`cli.md`](cli.md#grimoire-apply) describes `apply`, `plan`, `verify` and `uninstall`, and [`cli.md`](cli.md#exit-codes) every exit code.
