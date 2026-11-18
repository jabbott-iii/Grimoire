# Configuring a project

- **Status:** written in Phase 7 (2026-11-17), before the first release. It describes the `grimoire` CLI as built from this repository on that date. The examples, except the made-up server in §1.6, were run with a local debug build in scratch directories, against the catalog embedded in that build.
- **Design:** [`architecture.md`](architecture.md) §6–§8, [ADR-0001](adr/0001-catalog-delivery.md) (the embedded catalog) and [ADR-0008](adr/0008-phase-4-client-adapters.md) (what each client gets).
- **Related:** every command, flag and exit code is in [`cli.md`](cli.md), every diagnostic code in [`diagnostics.md`](diagnostics.md), and the versioning and deprecation policy in [`versioning.md`](versioning.md).

Grimoire keeps two files at the root of your project:

| File | Written by | Commit it |
|---|---|---|
| `grimoire.toml` | you, and `grimoire init`, `add` and `remove` | yes |
| `grimoire.lock` | `grimoire apply` and `grimoire update` only; `grimoire uninstall` deletes it | yes |

This file has six parts:
1. `grimoire.toml`;
2. permissions and consent;
3. `grimoire.lock`;
4. clients, and what Grimoire writes for each;
5. what Grimoire never touches;
6. drift and conflicts.

## 1. `grimoire.toml`

`grimoire.toml` is the desired state of the project: the clients to write for and the assets to install. `grimoire apply` makes the project match it. Its JSON Schema, generated from the Rust types, is [`schemas/grimoire.schema.json`](../schemas/grimoire.schema.json).

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

This project writes for Claude Code, the portable `AGENTS.md` target and Cursor. It installs the `security-baseline` bundle (`secret-guard` and `security-reviewer`), every asset tagged `git` (today, `conventional-commits`), and two assets by name. `project-docs` is configured for Claude Code only, over Streamable HTTP, for a server you start yourself. The `[allow]` table accepts the permissions of the two bundle members, which have no `[assets]` entry of their own.

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

- **Ids.** Asset and bundle ids are 1–64 characters of `a-z`, `0-9` and `-`, with no leading, trailing or doubled `-`. Tags follow the same rules, with at most 32 characters.
- **Unknown fields are errors**, at the top level and in every `[assets]` table. A misspelt key fails with its line and column and the list of keys that are allowed (`project/config-parse`, exit 3). Values are checked while the file is parsed, so a bad id, permission, version requirement or URL fails the same way.
- **Duplicates** in `clients`, `bundles`, `tags` or `exclude` are errors (`config/clients`, `config/bundles`, `config/tags`, `config/exclude`).
- **`runtime = "docker"`**, proposed in [ADR-0002](adr/0002-mcp-server-and-tool-packaging.md), is not a field: it was deferred past v1 ([ADR-0011](adr/0011-container-image.md)), so it fails as an unknown field. [`container.md`](container.md) shows hand-written `docker run` entries instead.

### 1.3 `[assets]` entries

Each entry selects one asset by id. The value is a version requirement, or a table that also holds options. Both table forms work:

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

**Which clients an asset is written for.** Without `clients`, an asset is written for every project client it supports. For a project client it doesn't support, `plan` warns (`resolve/client-skipped`). When the client has no place for that kind of asset at all (hooks outside Claude Code, agents in Codex), the warning is `resolve/client-not-covered`. `agents-md` never holds agents, MCP servers or hooks, and gets no warning for them. If the asset supports none of the project's clients, resolution fails (`resolve/unsupported-client`). A `clients` list in the entry turns these warnings off, and makes a client the asset doesn't support an error.

### 1.4 How assets are selected

An asset is installed when anything selects it. The lock records every reason in `selected-by`.

| Selected by | Recorded as | Version requirement |
|---|---|---|
| an `[assets]` entry | `direct` | the entry's `version` |
| a bundle in `bundles` | `bundle:<id>` | the one the bundle gives that member |
| a tag in `tags` | `tag:<tag>` | none: whatever version the catalog has, pre-releases included |
| another installed asset's `requires` | `requires:<id>` | the one that asset's manifest gives |

- **Tags** select every asset with the tag, except deprecated assets and assets that support none of the project's clients (a warning, `resolve/tag-skipped`). A tag that no asset carries is a warning (`resolve/unknown-tag`). A tag is a moving selection: a later grimoire release may carry more assets with that tag, and `plan` then shows them as new.
- **`exclude`** drops an asset that a bundle or tag selects. An asset that another selected asset requires can't be excluded (`resolve/excluded-dependency`).
- **Deprecated assets** still install when you, a bundle or `requires` select them, with a warning that names the replacement (`resolve/deprecated`). How long a deprecated asset stays in the catalog is in [`versioning.md`](versioning.md).
- **Conflicts.** Two selected assets that declare a conflict stop resolution, and the error says what selected each one (`resolve/conflict`).
- `grimoire show <id>` prints an asset's permissions, outputs, dependencies and conflicts, and `grimoire list` searches the catalog.

### 1.5 Version requirements and pins

- **Requirements are Cargo-style SemVer requirements:** `^0.2.0`, `0.2` (the same as `^0.2`), `~1.4`, `>=1, <3`, `=1.2.3` or `*`. As in Cargo, `^` keeps the left-most non-zero part: `^0.2.0` accepts `0.2.x` but not `0.3.0`, and `^1.2` accepts every `1.x` from `1.2.0`. A requirement accepts a pre-release such as `1.0.0-rc.1` only when it names a pre-release of that same version; `*` doesn't.
- **One version per asset.** Each grimoire release embeds exactly one version of every asset ([ADR-0001](adr/0001-catalog-delivery.md)). Resolution never picks another one: a requirement that the embedded version doesn't meet is an error (`resolve/version`, exit 3). An asset whose `min-cli-version` is newer than the running grimoire is an error too (`resolve/cli-too-old`).
- **The lock pins exact versions.** Once an asset is in `grimoire.lock`, `plan` and `apply` keep its locked version and manifest. When the running grimoire carries another version, or the same version with a different manifest, they stop and name the command to run (`resolve/locked-version-unavailable`, `resolve/locked-manifest-changed`, exit 3). This happens after you switch to a newer or older grimoire release. An asset that isn't in the lock yet resolves to the catalog's version.
- **`grimoire update [ID]...`** moves the named installed assets, or all of them when you name none, to the version the running grimoire carries, then applies. `--no-apply` writes only the lock and leaves the files for a later `grimoire apply`. The new version must still meet the requirement in `grimoire.toml`, its `min-cli-version` and the permissions you accepted (§2.4). If it doesn't, nothing is written. An id that isn't in the lock is an error (`update/unknown`). See [`cli.md`](cli.md#grimoire-update).
- **`update` never edits `grimoire.toml`**, and `grimoire add` leaves an existing entry's requirement alone. To accept a version your requirement excludes, such as `0.3.0` under `^0.2.0`, edit the requirement, then run `grimoire update`.
- `update` moves a pin to whatever the running grimoire carries, so an older grimoire moves it back. `grimoire.lock` never records the CLI version.

### 1.6 MCP server options

By default a client starts an MCP server itself, over stdio: the entry names the server's binary, which must be on your `PATH`. Grimoire never installs it; `grimoire doctor` checks for it.

**Streamable HTTP.** With `transport = "streamable-http"`, you start the server and the client connects to `url`:

```toml
[assets]
project-docs = { version = "^0.1.0", allow = ["filesystem-read", "env:CLAUDE_PROJECT_DIR"], transport = "streamable-http", url = "http://127.0.0.1:8931/mcp" }
```

- `url` is required with `streamable-http` (`config/url-missing`) and refused without it (`config/url-transport`).
- The URL must be `https://`, or `http://` for this machine only: `localhost`, `127.0.0.0/8` or `[::1]`. It can't hold user information (`user@`), a query or a fragment, so no credential can reach a client's configuration through it, and it can't hold `$` or `{`, which a client would expand. ASCII only, at most 2048 characters.
- Grimoire writes no headers and no tokens. A server that needs sign-in negotiates it with the client.
- `env` is refused with `streamable-http` (`config/env-transport`): the client doesn't start the server, so it passes no environment.
- The permissions are the same for either transport.
- [`container.md`](container.md) shows how to run the catalog's servers from the container image, and [`project-docs`'s README](../catalog/mcp-servers/project-docs/README.md) its own options.

**Renaming environment variables.** A stdio entry forwards each variable the server's manifest lists for forwarding (its `[mcp-server] env`), by reference and under the same name. When your environment holds the value under another name, map it. No server in today's catalog forwards a variable, so this example uses a made-up `docs-index` that reads `DOCS_TOKEN`:

```toml
[assets]
docs-index = { version = "^2", allow = ["env:DOCS_TOKEN"], env = { DOCS_TOKEN = "GH_TOKEN" } }
```

The key is the server's variable, and the value is the name in your environment. The grant stays `env:DOCS_TOKEN`, under the server's name. Each client writes the reference its own way:

| Client | Written |
|---|---|
| `claude-code`, `copilot` (`.mcp.json`), `gemini-cli` | `"DOCS_TOKEN": "${GH_TOKEN}"` |
| `cursor` | `"DOCS_TOKEN": "${env:GH_TOKEN}"` |
| `codex` | refused (`adapter/env-remap`): Codex forwards a variable only under its own name. Set the variable under the server's name, or leave `codex` out of the asset's `clients`. |

- A key the server doesn't read is an error (`adapter/env-unknown`), and so is a variable the client sets itself, such as `CLAUDE_PROJECT_DIR` for `project-docs`.
- Grimoire writes only names, never values. For Claude Code, Copilot and Gemini CLI, a variable the manifest marks optional gets an empty default, `${NAME:-}`, because an unset reference without a default reaches the server as its own text.

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
- `remove <id>` refuses an asset that has no `[assets]` entry but is installed through a bundle, a tag or `requires` (`remove/indirect`): add it to `exclude` instead. Removing the `[assets]` entry of an asset that something else still selects keeps it installed, moves its grant to `[allow]`, and warns (`remove/still-selected`).
- `remove` leaves the `[allow]` entries of a removed bundle's members. `plan` then warns that they have no effect (`resolve/allow-unused`); delete them by hand.
- `grimoire init` writes a new file: `format = 1`, the clients you name with `--client` or the ones it detects (§4.1), and an empty `[assets]` table. It refuses to replace an existing `grimoire.toml` (`init/exists`).

Everything else in the file is yours to edit. `grimoire plan` shows what an edit would change. See [`cli.md`](cli.md#grimoire-add) for the commands.

## 2. Permissions and consent

Every asset declares what it can cause at run time. You accept each declared permission in `grimoire.toml` before Grimoire installs the asset. Until you do, resolution fails (`resolve/permission`, exit 3) and nothing is written. Rules are plain text and declare none.

### 2.1 Permission names

| Grant | The asset can | For example |
|---|---|---|
| `filesystem-read` | read files | an agent with the `read` or `search` tools; `secret-guard`; `project-docs` |
| `filesystem-write` | read and write files. It covers `filesystem-read` | an agent with the `edit` tool |
| `network` | use the network | an agent with the `web` tool |
| `shell` | run shell commands or other programs | an agent with the `execute` tool; a skill that bundles scripts |
| `hooks` | register client hooks, which run automatically | a tool with hooks, such as `secret-guard` |
| `env:<NAME>` | read the environment variable `<NAME>`: one grant per variable | an MCP server that reads a token; `project-docs` reads `CLAUDE_PROJECT_DIR` |

- `<NAME>` is 1–128 characters of `A-Z`, `0-9` and `_`, and doesn't start with a digit. A bare `env` is refused in `grimoire.toml`. Each variable is its own grant, so a new version that reads one more variable needs your consent again.
- An agent's tools need matching permissions: `read` and `search` need `filesystem-read`, `edit` needs `filesystem-write`, `execute` needs `shell` and `web` needs `network`. Catalog validation enforces this.
- A grant is consent, not a sandbox. Grimoire checks it before it installs anything, and can't limit what a binary or a client does afterwards. What each client enforces of an agent's tool list is in [`security-model.md`](security-model.md).
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
- There is no prompt, and the answer is no, when stdin isn't a terminal, when `CI` is set to a non-empty value, with `--no-input` and with `--json`. Pass the grants instead: `grimoire add secret-guard --allow filesystem-read,hooks`.
- `--allow` takes a comma-separated list and can be repeated. On the command line only, a bare `env` accepts every variable the named assets read now, and `grimoire.toml` records each one by name.
- `add` records only the permissions that are still missing, so an `--allow` value that no named asset needs is not written.
- Consent reaches only what the command names: the named assets, the members of named bundles, the assets of named tags, and what they require. Another asset in `grimoire.toml` that lacks consent still fails the command.
- `grimoire add <id> --allow …` also works for an asset that is already listed: it adds the missing grants and changes nothing else. For assets that a bundle or tag brings in, use `add bundle:<id> --allow …` or `add tag:<tag> --allow …`.
- When a grant is declined, `grimoire.toml` is not changed (`add/permission`, exit 3).

### 2.4 When an update needs more

A new version that declares a permission you haven't accepted stops `grimoire update` and `grimoire apply` (`resolve/permission`, exit 3). Nothing is written, not even the lock. There is no silent escalation ([`security-model.md`](security-model.md), F14). The error names the asset, what selected it, the missing grants and how to accept them:

```text
error[resolve/permission] grimoire.toml: `secret-guard` (from bundle `security-baseline`) needs permissions you have not accepted: filesystem-read. Review them with `grimoire show secret-guard`, then run `grimoire add bundle:security-baseline --allow filesystem-read`, or set `secret-guard = ["filesystem-read", "hooks"]` under [allow] in grimoire.toml
```

1. Review what the new version does: `grimoire show <id>` prints its permissions and the latest section of its CHANGELOG.
2. Accept the new grants with the command the error names, or by editing `grimoire.toml`.
3. Run `grimoire update` again.

Taking a grant out of `grimoire.toml` has the same effect: the next `plan` or `apply` stops until you put it back or remove the asset.

## 3. `grimoire.lock`

`grimoire.lock` records exactly what Grimoire installed and what it owns on disk. It is generated: `apply` and `update` write it, and it starts with `# Generated by grimoire. Do not edit by hand.` Its JSON Schema is [`schemas/grimoire-lock.schema.json`](../schemas/grimoire-lock.schema.json). An excerpt, with digests shortened:

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
| `unit` | `file` (the whole file), `block` (a managed block in a Markdown file, §4.3) or `entry` (one key or array element in a configuration file, §4.4). |
| `key` | For an `entry`: a JSON Pointer, such as `/mcpServers/project-docs`, or `/hooks/PreToolUse\|secret-guard#0` for one hook element. |
| `hash` | The SHA-256 of what Grimoire owns there: the file, the block's text or the entry's value. |
| `created-file` | For a `block` or `entry`: Grimoire created the file, so it deletes the file once nothing else is left in it. |
| `created-keys` | For an `entry`: the containers Grimoire created, which it removes again once they are empty. |
| `[[output]]` | Content Grimoire owns for itself rather than for an asset: the `@AGENTS.md` import blocks in `CLAUDE.md` and `GEMINI.md` (§4.5). |

- **Don't edit it.** Grimoire validates the lock when it reads it. A record for a path Grimoire doesn't manage, or one that claims more than Grimoire owns there (a whole `.codex/config.toml`, say), is refused before anything is read or written (`project/lock-path`, exit 3). A lock that doesn't parse, or has an unknown field, is refused too (`project/lock-parse`).
- **Commit it**, with `grimoire.toml` and the files Grimoire writes. The lock is how `apply` tells your edits from its own content, how every asset stays at its version until `update`, and what `verify` checks. If it is lost, `apply` adopts the content that already matches what it would write, and reports anything else in the way as a conflict.
- **Review its diffs like code.** A lock diff shows every version, permission and output change. A new `allow` value is new consent, and a new `path` is a new file in your repository.
- **Line endings.** The lock is written with `\n` line endings, and reads the same on every operating system: paths use `/` and records are sorted. A checkout that converted it to `\r\n`, for example with `core.autocrlf`, holds the same lock, and Grimoire doesn't rewrite it for that alone. Hashes of text fold `\r\n` to `\n` too, so a managed file whose line endings were converted is not drift, and Grimoire leaves it as it is ([`security-model.md`](security-model.md), F13).
- **`grimoire verify`** reads only the lock, not `grimoire.toml` or the catalog. It checks every recorded output against the disk: each file's hash, each block's text and each entry's value. It exits 5 and lists every output that was edited or is missing, or whose file it can't parse (`verify/mismatch`). Without a lock it exits 3 (`verify/no-lock`). Because it doesn't depend on which asset versions the running grimoire carries, it is a good CI check for hand edits to managed content.
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

`grimoire init --client <id>`, repeated as needed, names the clients instead. When you change `clients` later, `apply` rewrites the outputs to match. For example, going from `clients = ["claude-code", "agents-md"]` to `clients = ["claude-code"]` moves each rule from `AGENTS.md` to `.claude/rules/` and removes the `CLAUDE.md` import (§4.5).

### 4.2 What Grimoire writes for each kind

This is where each client's content goes, when the asset supports that client. "Or `AGENTS.md`" means the rule is written once, as its `AGENTS.md` block, when that block is written (§4.5). `grimoire show <id>` prints one asset's destinations, and `grimoire plan` the final paths.

| Kind | `claude-code` | `agents-md` | `codex` | `cursor` | `gemini-cli` | `copilot` |
|---|---|---|---|---|---|---|
| Rule | `.claude/rules/<id>.md`, or `AGENTS.md` through `CLAUDE.md` | an `AGENTS.md` block | the same `AGENTS.md` block | `.cursor/rules/<id>.mdc`, or `AGENTS.md` | a `GEMINI.md` block, or `AGENTS.md` through `GEMINI.md` | `.github/instructions/<id>.instructions.md`, or `AGENTS.md` |
| Skill | `.claude/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` | `.agents/skills/<id>/` or `.claude/skills/<id>/` (§4.5) | `.agents/skills/<id>/` | `.agents/skills/<id>/` or `.claude/skills/<id>/` (§4.5) |
| Agent | `.claude/agents/<id>.md` | – | – | `.cursor/agents/<id>.md`, only for an agent with exactly `read` and `search` | `.gemini/agents/<id>.md` | `.github/agents/<id>.agent.md` |
| MCP server | `.mcp.json`, `mcpServers.<id>` | – | `.codex/config.toml`, `[mcp_servers.<id>]` | `.cursor/mcp.json`, `mcpServers.<id>` | `.gemini/settings.json`, `mcpServers.<id>` | `.mcp.json`, the same entry as `claude-code` |
| Tool hooks | `.claude/settings.json`, one `hooks.<Event>` element per binding | – | – | – | – | – |

- **Rules** are written as the rule's text. An always-on rule gets `alwaysApply: true` for Cursor and `applyTo: "**"` for Copilot. A path-scoped rule gets `paths` front matter for Claude Code, `alwaysApply: false` with `globs` for Cursor, and its globs as `applyTo` for Copilot. Cursor and Copilot take one comma-separated glob list, so a glob that contains a comma can't be written in their own files (`adapter/unsupported`); it can when the rule goes to `AGENTS.md`. `AGENTS.md` and `GEMINI.md` have no path scoping, so a path-scoped rule's block starts with a line that names the files it applies to.
- **Skills** are copied byte for byte, keeping their layout (`SKILL.md`, and any `references/`, `assets/` and `scripts/`).
- **Agents** get a `name`, a `description` and the client's form of the tool list. Claude Code and Gemini CLI get an explicit `tools` list, and Copilot a `tools` list with `strict-tools-list: true`. Cursor has no tool list, only `readonly: true`, which fits an agent with exactly `read` and `search`; any other agent is refused for Cursor (`adapter/unsupported`). Codex gets no agents: it can't limit an agent's tools ([ADR-0008](adr/0008-phase-4-client-adapters.md)). Where a client limits them only partly, `plan` warns (`adapter/agent-readonly`, `adapter/agent-strict-tools`).
- **MCP servers** over stdio name the server's binary and refer to each variable its manifest says to forward, never to a value: `${NAME}` for Claude Code, Copilot and Gemini CLI, `${env:NAME}` for Cursor, and a list of names in `env_vars` for Codex. Over Streamable HTTP the entry holds the `url`, with `"type": "http"` for Claude Code, Copilot and Gemini CLI.
- **Tool hooks** are written in exec form, the binary as `command` and its arguments as `args`, never a shell string. A binding's `timeout` is written when it has one, and `"onFailure": "block"` when it fails closed. Hooks are written for Claude Code only ([ADR-0008](adr/0008-phase-4-client-adapters.md)). A tool without hooks writes nothing for any client: the lock records it, and `doctor` checks its binary.
- **Binaries.** Grimoire never installs a tool or MCP server binary. Each entry names one that must be on `PATH`. `plan` warns when a fail-closed hook's binary is missing, because the client would then block every event the hook covers (`plan/hook-binary-missing`).

### 4.3 Managed blocks in shared files

`AGENTS.md`, `CLAUDE.md` and `GEMINI.md` are shared with you. Grimoire owns only the text between its marker lines:

```markdown
<!-- grimoire:begin conventional-commits -->
...the rule's text...
<!-- grimoire:end conventional-commits -->
```

- A block is named after its asset. The name `grimoire` is reserved for Grimoire's own block, which holds the `@AGENTS.md` import. `CLAUDE.md` only ever gets that import block. `GEMINI.md` gets rule blocks, the import block, or both.
- Everything outside the blocks is yours and is kept byte for byte. Write above, between and below the blocks as you like. An edit inside a block is drift (§6).
- A new block goes among the existing blocks in name order, or after a blank line at the end of the file.
- Markers count only at the start of a line, and not inside a fenced code block in your text, so a file can show them as an example.
- Grimoire refuses to edit a file whose markers are broken (malformed, unterminated, nested or repeated), even with `--force` (exit 4). It never guesses what it owns. Fix the markers by hand.
- Grimoire won't append a block to a file that ends inside an unclosed code fence, where the block would become part of the code.
- In a file that uses `\r\n`, Grimoire writes its lines with `\r\n` too, and it keeps a byte-order mark.

### 4.4 Entries in configuration files

In the five configuration files below, Grimoire owns single entries and leaves everything else alone: other servers, other hooks, other settings, key order, and formatting outside the object or array it edits.

| File | Syntax Grimoire accepts | What it owns |
|---|---|---|
| `.mcp.json`, `.cursor/mcp.json` | strict JSON | `mcpServers.<id>` |
| `.claude/settings.json` | strict JSON | one element of a `hooks.<Event>` array per hook binding, found by its exact value |
| `.gemini/settings.json` | JSON with comments, as Gemini CLI reads it; trailing commas are refused | `mcpServers.<id>` |
| `.codex/config.toml` | TOML | the `[mcp_servers.<id>]` table |

- Grimoire refuses a file it can't read that way, even with `--force` (exit 4): for example, a comment in `.mcp.json`, a duplicate key on Grimoire's path, or TOML that doesn't parse. It never "fixes" a file that a client might not read.
- Comments in a TOML or JSONC file are kept. A comment you add *inside* Grimoire's entry is a conflict, because rewriting the entry would lose it: move it out of the entry, or use `--force` to drop it.
- Inserting into a JSON object written on one line spreads that object over several lines. The values stay the same.
- Grimoire removes a container it created (`created-keys`) once it is empty, and a file it created (`created-file`) once nothing else is in it. A file or container that was there before Grimoire wrote into it stays, even when it is left empty. Adoption is the exception (§6): when every entry Grimoire wants in a file is already there, and the lock records none of them, Grimoire counts each container that holds nothing but those entries as created by it, and the file too if it holds nothing else, as it would after an interrupted `apply`.

### 4.5 Shared rules and skills

Clients read each other's files. So where several selected clients can read one location, Grimoire writes the content there once and lists all of them in the lock's `clients` ([ADR-0008](adr/0008-phase-4-client-adapters.md), [`architecture.md`](architecture.md) §8.2). This is decided per asset, from the clients that asset is installed for.

**Rules.** When `agents-md` or `codex` is one of a rule's clients, the rule is written once, as its `AGENTS.md` block, and:
- `claude-code` gets no `.claude/rules/` file: it reads `AGENTS.md` through Grimoire's `@AGENTS.md` block in `CLAUDE.md`;
- `gemini-cli` gets no rule block: it reads `AGENTS.md` through the same import block in `GEMINI.md`;
- `cursor` and `copilot` get no rule file: they read `AGENTS.md` themselves.

Otherwise each client gets its own file (§4.2). A path-scoped rule in `AGENTS.md` keeps its scope line, but Claude Code, Cursor and Copilot then apply it everywhere.

**Skills.** `.claude/skills/` is written when `claude-code` is one of the skill's clients, and `.agents/skills/` when `agents-md`, `codex` or `gemini-cli` is. Cursor and Copilot read both. Their copy goes to `.agents/skills/` when that is written, else to `.claude/skills/` when that is, else to `.agents/skills/`. So `claude-code` with `cursor` gets one copy, in `.claude/skills/`, while `claude-code`, `agents-md` and `cursor` need both, and Cursor then lists the skill twice.

**MCP servers.** `claude-code` and `copilot` share one `.mcp.json` entry.

**What placement can't avoid, `plan` reports as warnings:**
- a rule or skill that a selected client may read twice (`plan/duplicate-rule`, `plan/duplicate-skill`);
- an `AGENTS.md` rule that Claude Code or Gemini CLI reads although it isn't installed for them (`plan/cross-client-rule`);
- an agent, hook or MCP server that Cursor, Copilot or Claude Code also loads from another client's file (`plan/cross-client-agent`, `plan/cross-client-hooks`, `plan/cross-client-mcp`).

Installing for one client can grant something in another. [`security-model.md`](security-model.md) covers what that means.

### 4.6 Steps only you can take

Several clients read project files only after you trust the project or approve a server. Grimoire can't check or grant that. `grimoire doctor` prints the step for each selected client:

| Client | What it needs |
|---|---|
| `claude-code` | `doctor` prints nothing. Claude Code asks before it uses `.mcp.json` servers in an interactive session (`CC-MCP` in [`specs.md`](specs.md)). |
| `agents-md` | Nothing. |
| `codex` | Codex reads `.codex/config.toml`, and so its MCP servers, only in a project you have marked trusted. When a root `AGENTS.override.md` exists, Codex reads it instead of `AGENTS.md`; `doctor` says so, and `plan` warns (`plan/codex-override`). Codex reads at most 32 KiB of project instructions in total by default, so `plan` warns from 24 KiB of `AGENTS.md` (`plan/codex-instructions-size`). |
| `cursor` | Cursor asks you to approve each project MCP server. |
| `gemini-cli` | Gemini CLI reads project files only in a folder you have trusted. It finds `GEMINI.md` from any directory in the repository, but reads `.gemini/` (and so its MCP servers and agents) and `.agents/skills/` only when started at the project root. It asks you to acknowledge each project agent again after every change. |
| `copilot` | Copilot reads `.mcp.json` only after you trust the folder. Its cloud agent reads MCP servers only from the repository's settings. For the MCP servers installed for `copilot`, `doctor` prints the JSON to paste there, names each variable as a `COPILOT_MCP_*` secret, and says to install the local servers' binaries in `copilot-setup-steps.yml`. |

`doctor` also checks that the binary of each tool and MCP server in the lock is on `PATH`, except a server configured over Streamable HTTP. It exits 5 when a check fails. See [`cli.md`](cli.md#grimoire-doctor).

## 5. What Grimoire never touches

- **Anything outside the project root**, including your home directory and every client's user-level settings. Grimoire reads and writes project files only through a handle on the root, and refuses (exit 6) rather than follow a symbolic link at or above a path it manages. Outside the root it only looks for binaries in the `PATH` directories (`doctor`, `plan/hook-binary-missing`). A few checks only ask whether a path exists, and ask it by name rather than through the handle: `init` and `doctor` look for each client's files (§4.1), and, when `codex` is a client, the commands that plan look for `AGENTS.override.md`. They read no content, but they do follow a symbolic link in a parent directory: with `.github` linked to a folder outside the root, `init` can detect `copilot` from the files there.
- **Any path but its own locations.** Besides `grimoire.toml`, `grimoire.lock` and the `<path>.new` files of `--write-new`, Grimoire writes only in the folders `.claude/rules/`, `.claude/agents/`, `.claude/skills/`, `.agents/skills/`, `.cursor/rules/`, `.cursor/agents/`, `.gemini/agents/`, `.github/instructions/` and `.github/agents/`, and to the files `AGENTS.md`, `CLAUDE.md`, `GEMINI.md`, `.mcp.json`, `.claude/settings.json`, `.codex/config.toml`, `.cursor/mcp.json` and `.gemini/settings.json`. It never writes `.gitignore`, `.github/copilot-instructions.md` or `AGENTS.override.md`.
- **What it didn't write**, even in those locations: your text outside its blocks, other keys and array elements, and other files in those folders.
- **Your edits to its own content**, unless you pass `--force` (§6).
- **Files and keys that were there before it**, unless it took them over. Removing an asset removes only Grimoire's block or entry from them. Grimoire takes content over in two cases (§6): it adopts content that already matches what it would write, and `--force` overwrites content in its way. From then on that file, block or entry is Grimoire's, and removing the asset deletes it. An adopted shared file that holds nothing but Grimoire's blocks or entries is deleted with them.
- **Your directories**, unless they are left empty. After deleting a file, Grimoire removes each directory above it that is now empty, even one you created.
- **Secret values.** Entries refer to environment variables by name, and a `url` can't carry credentials.
- **Binaries.** It never downloads, installs or runs a tool or MCP server, and installing an asset runs nothing from it. The install path has no network code.
- **Trust and approval** in any client (§4.6).

## 6. Drift and conflicts

For each unit it owns, Grimoire compares three things: what is on disk, what the lock recorded, and what it would write now ([`architecture.md`](architecture.md) §7.3).

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
- **`--write-new`** leaves each conflicting file alone and writes what Grimoire would write beside it, as `<path>.new` (the whole file, for blocks and entries). It then exits 4 (`apply/conflict-remains`). The lock keeps the old records for those files, so the conflict is reported until you merge the `.new` file or use `--force`. It never replaces a different `<path>.new` that is already there (`apply/sidecar-exists`). `--force` and `--write-new` can't be combined.
- **A file Grimoire can't read safely** (broken markers, invalid JSON or TOML, text that isn't UTF-8) is refused even with `--force` (exit 4). Fix it by hand.
- **`grimoire uninstall`** removes nothing while any managed content is edited (exit 4). `--force` removes the edited content too. `--purge` also deletes `grimoire.toml`.
- `grimoire verify` and `grimoire plan --check` report drift without writing anything (§3).

[`cli.md`](cli.md#grimoire-apply) describes `apply`, `plan`, `verify` and `uninstall`, and [`cli.md`](cli.md#exit-codes) every exit code.
