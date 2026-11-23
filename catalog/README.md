# The Grimoire catalog

This directory holds everything Grimoire can install. Each installable unit is an asset: a rule, a skill, an agent, a tool or an MCP (Model Context Protocol) server (the table below says what each kind is). Every asset lives in its own directory, `catalog/<kind>/<id>/`.

Bundles live in `catalog/bundles/<id>.toml`. A bundle is a named, versioned list of assets that a project can select together. It isn't an asset itself.

- Adding an asset is a **data-only change** under this directory: it needs no change to Grimoire's own code.
- `cargo xtask validate` checks everything described here, and CI (continuous integration, the checks that run on every pull request and every push to `main`) runs it. `cargo xtask` is the repository's set of helper commands, written in Rust.
- The design is in [`docs/architecture.md`](../docs/architecture.md) §3–§6, which cover the asset kinds, the repository layout, identifiers and versions, and manifests, configuration and catalog validation.
- How to open the pull request is in [`CONTRIBUTING.md`](../CONTRIBUTING.md). Asset versions and deprecation (retiring an asset in favor of a named replacement) follow [`docs/versioning.md`](../docs/versioning.md).

A client is an AI coding tool that Grimoire installs assets for, such as Claude Code or Cursor. MCP is the protocol through which a client uses the tools and other content that a separate server program offers.

| Kind | Directory | Content |
|---|---|---|
| rule | `rules/<id>/` | `rule.md`: Markdown instructions that the client loads automatically, without being asked |
| skill | `skills/<id>/` | An [Agent Skills](https://agentskills.io/specification) directory: `SKILL.md` and optional `scripts/`, `references/`, `assets/`. The agent loads a skill only when a task needs it |
| agent | `agents/<id>/` | `agent.md`: the subagent's system prompt (its starting instructions). A subagent is a specialist that the main agent session can hand work to |
| tool | `tools/<id>/` | A Rust crate (package) building the program `grimoire-tool-<id>` |
| mcp-server | `mcp-servers/<id>/` | A Rust crate building the server `grimoire-mcp-<id>` on rmcp, the Rust MCP SDK (software development kit), through `agent-grimoire-mcp` |

Start a new asset with `cargo xtask new <kind> <id> --maintainer <login>` (or `cargo xtask new bundle <id> --maintainer <login> <asset>...`), where `<login>` is your GitHub login.

- Every template passes `cargo xtask validate` as generated, and the code templates (tool and MCP server) build and pass their own tests.
- The exemplars show a finished asset of each kind: `rules/conventional-commits`, `skills/adr-writer`, `agents/security-reviewer`, `tools/secret-guard` and `mcp-servers/project-docs`.

## Every asset needs

- **`asset.toml`**, the manifest: the file that describes the asset, such as its id, version, permissions and what each client gets. It is written in TOML, a configuration file format. Its schema is [`schemas/asset.schema.json`](../schemas/asset.schema.json).
  - `id` must equal the directory name. Use 1–64 characters of `[a-z0-9-]` (lowercase letters, digits and hyphens), and don't start with `claude-`, `anthropic-`, `anthropics-`, `cc-plugin-` or `grimoire-`. Claude Code reserves the first four for plugin names, and Grimoire keeps `grimoire-` for content it owns itself in a project.
  - A skill, agent or MCP server id, and a bundle id, also becomes a Claude Code plugin name (see the marketplace rule below). So it can't contain `claude`, `anthropic` or `anthropics` as a word, or `official` next to one of them. For example, `mcp-for-claude` is refused, but `claudette` is fine.
  - A bundle can't share an asset's id.
  - `cargo xtask gen-marketplace` fails if either of the two rules above is broken.
  - `kind` must match the parent directory, for example `kind = "rule"` under `rules/`.
  - Unknown fields are errors, so a misspelled field fails validation instead of being ignored.
- **`README.md`**, which explains what the asset does, when to use it, and security notes.
- **`CHANGELOG.md`**, with a `## <version>` (or `## [<version>]`) heading for the current version, such as `## 1.2.0`.
- **`evals/` or `tests/`**, with at least one file. Tools and MCP servers need `tests/`. `evals/` is for evaluation cases, such as prompts to try with the asset installed and the behavior to expect.

## Rules that trip people up

- **Explicit outputs per client.**
  - Each `[clients.<client>]` table lists exactly what is installed for that client. Each output has up to three fields:
    - `src`: a file or directory in the asset. It is left out for `entry` outputs, whose content the adapter builds from `asset.toml`.
    - `to`: an anchor, which is a named destination such as `rules:<id>.md`, `skills:<id>/SKILL.md`, `instructions`, `mcp` or `hooks`. Each client's adapter (the code that knows that client's file layout) turns the anchor into a real path. For example, `rules:<id>.md` becomes `.claude/rules/<id>.md` for `claude-code` and `.cursor/rules/<id>.mdc` for `cursor` (or a block in `AGENTS.md`, when that file is written). Anchors are used instead of literal paths so that a change to a client's layout needs one adapter change, not an edit to every manifest.
    - `strategy`: `copy` (a whole file, byte for byte), `render` (a whole file the adapter generates, for example with the client's frontmatter, the metadata block at the top of a file), `block` (a managed block in a shared Markdown file: a section between Grimoire's marker lines, which Grimoire updates without touching the rest of the file) or `entry` (an owned entry in a JSON or TOML configuration file).
  - A client is supported if, and only if, its table is present.
  - Each client has its own anchors. [ADR-0008](../docs/adr/0008-phase-4-client-adapters.md), the architecture decision record (ADR) on what the Codex, Cursor, Gemini CLI and Copilot adapters write, gives the reasons. A dash means the client has no anchor for that kind, so an output there fails validation:

    | Client | Rule | Skill | Agent | MCP server | Hooks |
    |---|---|---|---|---|---|
    | `claude-code` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | `hooks` |
    | `agents-md` | `instructions` | `skills:` | – | – | – |
    | `codex` | `instructions` | `skills:` | – | `mcp` | – |
    | `cursor` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | – |
    | `gemini-cli` | `instructions` | `skills:` | `agents:<id>.md` | `mcp` | – |
    | `copilot` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | – |

  - The destination names stay `<id>.md`. The adapters rename them where a client needs it: `.mdc` for Cursor rules, `.instructions.md` for Copilot rules and `.agent.md` for Copilot agents.
  - `cursor` can only run an agent with exactly `read` and `search` tools (as `readonly`), because Cursor has no tool list for an agent, only a `readonly` setting.
  - `cursor` and `copilot` can't scope a rule to a glob (a file-name pattern such as `src/**/*.rs`) containing a comma, such as `**/*.{rs,toml}`. Both take their globs as one comma-separated list.
  - `cargo xtask validate` renders every asset for each client it declares, so these and every other adapter refusal fail validation, rather than a user's `grimoire apply`.
  - `codex` can't forward a variable under another name, so users can't rename an MCP server's variables there. Forwarding means passing an environment variable from the user's environment to the server, and Codex has no `${VAR}` syntax to pass one under a different name.
- **Skills keep their layout.**
  - `src = "X"` goes to `skills:<id>/X`. For example, `src = "references/guide.md"` is installed as `skills:<id>/references/guide.md`.
  - Every client must install `SKILL.md` and everything under `scripts/`, `references/` and `assets/`.
  - Only the spec's frontmatter fields are allowed. Frontmatter is the metadata block between `---` lines at the top of `SKILL.md`, and the spec is the Agent Skills specification. Client-specific fields are left out because some products reject a skill that has another client's fields (claude.ai and the Skills API reject Claude Code's), and other clients ignore unknown fields inconsistently.
  - `allowed-tools`, an experimental field of the spec, is refused as well, because it pre-approves tools outside Grimoire's permission consent (the step in which a project accepts each permission an asset declares).
  - Every script must be referenced from `SKILL.md` (see "Skill references" below for what counts).
- **Rules are plain Markdown.**
  - No frontmatter (Grimoire writes each client's).
  - No line starting with `<!-- grimoire:`, which is reserved for managed-block markers: the lines that mark where a block Grimoire manages starts and ends.
  - No unclosed code fence (a line of three or more backticks or tildes that opens a code block), which would turn the rest of a user's `AGENTS.md` into code.
  - Every client of a rule uses the same `src`, because clients read each other's rule files and a shared rule is written once.
- **Rule and agent files are named `<id>.md`.** Claude Code discovers only `.md` files.
- **Hook bindings list each capability once.**
  - A hook binding, which a tool asset declares, makes the client run the tool's binary automatically on an event, such as before each tool call. Its `tools` list names the capabilities (Grimoire's client-neutral names for agent tools, such as `read`, `search`, `edit` and `execute`) whose tool calls trigger it.
  - Avoid `search` in hooks. It matches Claude Code's Grep and Glob tools, but Claude Code's main session searches with Bash on macOS, Linux and WSL (Windows Subsystem for Linux), so such a hook may rarely run.
- **Guards fail closed.**
  - A hook that must never be skipped, such as a secret scanner, sets `on-failure = "block"`, so a missing or broken binary blocks the event instead of letting it through. That is what failing closed means.
  - Say so in the README, with the way out: how a user gets working again when the binary is missing. The `secret-guard` README is an example.
  - Only `pre-tool-use` and `user-prompt-submit` bindings may block, because they are the events whose failure can stop something: a tool call or a prompt.
- **MCP tool names used in `mcp:<server>/<tool>` capabilities avoid `.`.** Such a capability names one tool of one MCP server. Claude Code may rename tools whose names contain `.`, so the claude-code adapter refuses them.
- **An agent's `description` is its delegation text.** Clients give it to the model, which reads it to decide when to hand work to the agent, so say when to use it.
- **Agents list their tools explicitly.**
  - `[agent] tools = [...]` is required; `[]` is legal and grants no tools. The list is required because several clients give an agent every tool when its list is left out.
  - Capabilities that imply a permission must be declared under `[permissions]`: `read`/`search` imply filesystem `read`, `edit` implies filesystem `write`, `execute` implies `shell`, and `web` implies `network`.
  - An `mcp:<server>` capability needs that server in `[requires]`, the asset's dependencies.
- **Permissions match behavior.**
  - A tool with `hooks` declares `hooks = true`, and a tool without hooks must not.
  - Every environment variable an MCP server reads is listed in `permissions.env`, and needs consent by name (`env:<NAME>`): the project accepts each variable by name in its `grimoire.toml`.
  - `[mcp-server] env` lists only the variables the client must forward from the user's environment. A variable the client sets itself, such as `CLAUDE_PROJECT_DIR` (which Claude Code sets for the servers it starts), is in `permissions.env` alone.
  - A skill that bundles `scripts/` declares `shell = true`, because the agent may run those scripts.
- **No `.claude-plugin/` in a skill.** That directory marks a Claude Code plugin, so Claude Code would load the installed skill as a plugin, with its own hooks and MCP servers.
- **Skill references.**
  - Links (`[text](path)`, images, HTML `src`/`href`) must name a file inside the skill that every client installs. `file:` URLs are refused.
  - A script counts as documented when `SKILL.md` names its path, for example `scripts/x.sh`, in prose, inline code or a code block. Mentioning the `scripts/` directory alone does not count.
- **Rules declare no permissions**, because a rule is plain text that runs nothing. A rule also must not duplicate another rule.
- **Version bumps.** Any change to an asset's files needs a version bump (a higher `version` in `asset.toml`) and a CHANGELOG entry: `asset.toml` itself and every other file in the asset's directory, because they are installed or built. `README.md`, `CHANGELOG.md`, `evals/` and `tests/` are exempt.
- **Removing an asset.**
  - It is allowed only if the asset was already `maturity = "deprecated"` (with a named `replacement`) at the comparison base: on `main` for a pull request, or at the previous tip (the commit the branch pointed to before the push) for a push. Deprecate it in an earlier change first.
  - The policy in [`docs/versioning.md`](../docs/versioning.md) also keeps a deprecated asset, with its replacement, in at least one CLI (command-line interface) release before removal. `cargo xtask validate` can't check that, so review does.
- **`min-cli-version` is at most the workspace version.**
  - `min-cli-version` names the oldest grimoire that can install the asset. The workspace version is `version` in the root `Cargo.toml`, which the CLI and its libraries share.
  - `cargo xtask validate` doesn't check it: `cargo test -p agent-grimoire --test builtin` does, and CI runs it. The test installs every embedded asset with the workspace's own version and fails with `resolve/cli-too-old` when one needs a newer grimoire.
  - Between releases, the workspace version is still the last release's. So an asset that needs a CLI feature that isn't released yet lands after the release that ships the feature, with that release's version ([`docs/versioning.md`](../docs/versioning.md) §4, on asset versions).
- **Versions and reserved names.**
  - Versions are SemVer (semantic versioning, `MAJOR.MINOR.PATCH`) without build metadata, the `+…` suffix SemVer allows. So `1.2.3` and the pre-release `1.2.3-rc.1` are fine, and `1.2.3+build.5` isn't.
  - The ids `synced` and `grimoire` are reserved, as are the prefixes above. Claude Code reserves `synced` as a skill folder name, and `grimoire` owns Grimoire's own managed blocks, such as the `CLAUDE.md` import of `AGENTS.md`.
  - MCP servers can't be named `workspace` or `computer-use`, which Claude Code reserves for built-in servers.
- **An MCP server started with no arguments speaks stdio.** That is how clients are configured to start it: the client runs the server as a child process and exchanges messages over its standard input and output.
- **MCP servers serve through `agent-grimoire-mcp`**, which holds the transport, limit and authorization defaults. The transports are stdio and Streamable HTTP, MCP's HTTP transport. [ADR-0007](../docs/adr/0007-mcp-server-authorization.md) put these defaults in one shared crate so that every server inherits the same secure settings, because rmcp's own HTTP defaults are unsafe for Grimoire: for example, it leaves `Origin` validation (the check that stops a web page in a browser from calling the server) off.
  - The crate must depend on it, and `src/` must not use rmcp's stdio or Streamable HTTP transports directly.
  - A test must call `agent_grimoire_mcp::testing::declared_tools_match`, which compares the tools the running server lists with the tools its `asset.toml` declares, so the declared tools stay true.
  - These source checks are plain text searches that catch the usual mistakes but prove nothing. Review still decides.
- **Asset crates stand alone.**
  - No dependency on Grimoire's core crates (the `grimoire` crate, the libraries it is built from, such as `agent-grimoire-model`, and `agent-grimoire-testkit`).
  - No `path` or `git` dependency (one taken from a local directory or a Git repository instead of a registry) except on `agent-grimoire-mcp`.
  - No `Cargo.toml` below the asset's own.
- **The catalog is embedded in the binary.** The `grimoire` binary carries a pack of this catalog, `crates/agent-grimoire-catalog/catalog.pack`: each asset's `asset.toml`, `README.md`, `CHANGELOG.md` and installed files, and every bundle. [ADR-0001](../docs/adr/0001-catalog-delivery.md) chose this so that installing needs no network. After changing anything here, run `cargo xtask pack-catalog`. CI fails if the pack is stale (out of date), and so does a release.
- **The Claude Code marketplace is generated from it.** A marketplace is a list of plugins that Claude Code users can install.
  - Skills, agents and MCP servers that support `claude-code` become plugins under `plugins/claude-code/`, and bundles become plugins that depend on them.
  - After changing anything here, run `cargo xtask gen-marketplace`. CI fails if it is stale, and runs `claude plugin validate --strict` on every plugin.
- **No symlinks** (symbolic links), and no files over 1 MiB (1,048,576 bytes).
  - Every path must be portable across Linux, macOS and Windows: ASCII only, and no Windows reserved names (device names such as `CON` and `NUL`). ASCII only rules out invisible characters and names that look the same but are stored differently.
  - Every path may be at most 200 characters, counted from the catalog root: `skills/adr-writer/references/status-lifecycle.md` counts as 48. The pack stores each file under that path.
  - No two paths may differ only in letter case, such as `Guide.md` and `guide.md`. A case-insensitive file system, the default on Windows and macOS, can't hold both.
