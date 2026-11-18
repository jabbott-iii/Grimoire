# The Grimoire catalog

Every asset lives in its own directory, `catalog/<kind>/<id>/`. Bundles live in `catalog/bundles/<id>.toml`.

- Adding an asset is a **data-only change** under this directory.
- `cargo xtask validate` checks everything described here, and CI runs it.
- The design is in [`docs/architecture.md`](../docs/architecture.md) §3–§6.
- How to open the pull request is in [`CONTRIBUTING.md`](../CONTRIBUTING.md). Asset versions and deprecation follow [`docs/versioning.md`](../docs/versioning.md).

| Kind | Directory | Content |
|---|---|---|
| rule | `rules/<id>/` | `rule.md`: Markdown instructions loaded automatically |
| skill | `skills/<id>/` | An [Agent Skills](https://agentskills.io/specification) directory: `SKILL.md` and optional `scripts/`, `references/`, `assets/` |
| agent | `agents/<id>/` | `agent.md`: the subagent's system prompt |
| tool | `tools/<id>/` | A Rust crate building `grimoire-tool-<id>` |
| mcp-server | `mcp-servers/<id>/` | A Rust crate building `grimoire-mcp-<id>` on rmcp, through `agent-grimoire-mcp` |

Start a new asset with `cargo xtask new <kind> <id> --maintainer <login>` (or `cargo xtask new bundle <id> --maintainer <login> <asset>...`). Every template validates as generated, and the code templates build and pass their own tests. The exemplars show a finished asset of each kind: `rules/conventional-commits`, `skills/adr-writer`, `agents/security-reviewer`, `tools/secret-guard` and `mcp-servers/project-docs`.

## Every asset needs

- **`asset.toml`**, the manifest. Its schema is [`schemas/asset.schema.json`](../schemas/asset.schema.json).
  - `id` must equal the directory name. Use 1–64 characters of `[a-z0-9-]`, and don't start with `claude-`, `anthropic-`, `anthropics-`, `cc-plugin-` or `grimoire-`.
  - A skill, agent or MCP server id, and a bundle id, also becomes a Claude Code plugin name, so it can't contain `claude`, `anthropic` or `anthropics` as a word, or `official` next to one of them, and a bundle can't share an asset's id. `cargo xtask gen-marketplace` fails otherwise.
  - `kind` must match the parent directory.
  - Unknown fields are errors.
- **`README.md`**, which explains what the asset does, when to use it, and security notes.
- **`CHANGELOG.md`**, with a `## <version>` (or `## [<version>]`) heading for the current version.
- **`evals/` or `tests/`**, with at least one file. Tools and MCP servers need `tests/`.

## Rules that catch people out

- **Explicit outputs per client.**
  - Each `[clients.<client>]` table lists exactly what is installed for that client: `src` (a file or directory in the asset), `to` (an anchor such as `rules:<id>.md`, `skills:<id>/SKILL.md`, `instructions`, `mcp` or `hooks`) and `strategy` (`copy`, `render`, `block` or `entry`).
  - A client is supported if, and only if, its table is present.
  - Each client has its own anchors ([ADR-0008](../docs/adr/0008-phase-4-client-adapters.md)):

    | Client | Rule | Skill | Agent | MCP server | Hooks |
    |---|---|---|---|---|---|
    | `claude-code` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | `hooks` |
    | `agents-md` | `instructions` | `skills:` | – | – | – |
    | `codex` | `instructions` | `skills:` | – | `mcp` | – |
    | `cursor` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | – |
    | `gemini-cli` | `instructions` | `skills:` | `agents:<id>.md` | `mcp` | – |
    | `copilot` | `rules:<id>.md` | `skills:` | `agents:<id>.md` | `mcp` | – |

  - The destination names stay `<id>.md`; adapters rename them where a client needs it (`.mdc`, `.instructions.md`, `.agent.md`).
  - `cursor` can only run an agent with exactly `read` and `search` tools (as `readonly`), and `cursor` and `copilot` can't scope a rule to a glob containing a comma. `cargo xtask validate` renders every asset for each client it declares, so these and every other adapter refusal fail validation. `codex` can't forward a variable under another name, so users can't rename an MCP server's variables there.
- **Skills keep their layout.**
  - `src = "X"` goes to `skills:<id>/X`.
  - Every client must install `SKILL.md` and everything under `scripts/`, `references/` and `assets/`.
  - Only the spec's frontmatter fields are allowed, and `allowed-tools` is refused.
  - Every script must be referenced from `SKILL.md`.
- **Rules are plain Markdown.**
  - No frontmatter (Grimoire writes each client's).
  - No line starting with `<!-- grimoire:`, which is reserved for managed-block markers.
  - No unclosed code fence, which would turn the rest of a user's `AGENTS.md` into code.
  - Every client of a rule uses the same `src`, because clients read each other's rule files and a shared rule is written once.
- **Rule and agent files are named `<id>.md`.** Claude Code discovers only `.md` files.
- **Hook bindings list each capability once.** Avoid `search` in hooks: Claude Code's main session searches with Bash on macOS, Linux and WSL.
- **Guards fail closed.** A hook that must never be skipped, such as a secret scanner, sets `on-failure = "block"`, so a missing or broken binary blocks the event. Say so in the README, with the way out. Only `pre-tool-use` and `user-prompt-submit` bindings may block.
- **MCP tool names used in `mcp:<server>/<tool>` capabilities avoid `.`.** Claude Code may rename such tools, so the claude-code adapter refuses them.
- **An agent's `description` is its delegation text.** Clients show it to decide when to hand work to the agent, so say when to use it.
- **Agents list their tools explicitly.** `[agent] tools = [...]` is required; `[]` is legal. Capabilities that imply a permission must be declared under `[permissions]`: `read`/`search` imply filesystem `read`, `edit` implies filesystem `write`, `execute` implies `shell`, and `web` implies `network`. An `mcp:<server>` capability needs that server in `[requires]`.
- **Permissions match behaviour.**
  - A tool with `hooks` declares `hooks = true`, and a tool without hooks must not.
  - Every environment variable an MCP server reads is listed in `permissions.env`, and needs consent by name (`env:<NAME>`). `[mcp-server] env` lists only the variables the client must forward from the user's environment; one the client sets itself, such as `CLAUDE_PROJECT_DIR`, is in `permissions.env` alone.
  - A skill that bundles `scripts/` declares `shell = true`.
- **No `.claude-plugin/` in a skill.** Claude Code would load the installed skill as a plugin, with its own hooks and MCP servers.
- **Skill references.**
  - Links (`[text](path)`, images, HTML `src`/`href`) must name a file inside the skill that every client installs. `file:` URLs are refused.
  - A script counts as documented when `SKILL.md` names its path, for example `scripts/x.sh`, in prose, inline code or a code block. Mentioning the `scripts/` directory alone does not count.
- **Rules declare no permissions**, and must not duplicate another rule.
- **Version bumps.** Any change to an installed or built file needs a version bump and a CHANGELOG entry. `README.md`, `CHANGELOG.md`, `evals/` and `tests/` are exempt.
- **Removing an asset** is allowed only if it was already `maturity = "deprecated"` (with a named `replacement`) at the comparison base: on `main` for a pull request, or at the previous tip for a push. Deprecate it in an earlier change first. The policy in [`docs/versioning.md`](../docs/versioning.md) also keeps a deprecated asset, with its replacement, in at least one CLI release before removal. `cargo xtask validate` can't check that, so review does.
- **`min-cli-version` is at most the workspace version.** It names the oldest grimoire that can install the asset. `cargo xtask validate` doesn't check it: `cargo test -p agent-grimoire --test builtin` does, and CI runs it. It installs every embedded asset with the workspace's own version and fails with `resolve/cli-too-old` when one needs a newer grimoire. An asset that needs a CLI feature that isn't released yet lands after the release that ships the feature, with that release's version ([`docs/versioning.md`](../docs/versioning.md) §4).
- **Versions** are SemVer without build metadata (`1.2.3`, `1.2.3-rc.1`). The ids `synced` and `grimoire` are reserved, as are the prefixes above. MCP servers can't be named `workspace` or `computer-use`, which Claude Code reserves for built-in servers.
- **An MCP server started with no arguments speaks stdio.** That is how clients are configured to start it.
- **MCP servers serve through `agent-grimoire-mcp`**, which holds the transport, limit and authorization defaults ([ADR-0007](../docs/adr/0007-mcp-server-authorization.md)).
  - The crate must depend on it, and `src/` must not use rmcp's stdio or Streamable HTTP transports directly.
  - A test must call `agent_grimoire_mcp::testing::declared_tools_match`, so the declared tools stay true.
  - These source checks are plain text searches. Review still decides.
- **Asset crates stand alone.** No dependency on Grimoire's core crates, no `path` or `git` dependency except on `agent-grimoire-mcp`, and no `Cargo.toml` below the asset's own.
- **The catalog is embedded in the binary.** After changing anything here, run `cargo xtask pack-catalog`; CI fails if the pack is stale, and so does a release.
- **The Claude Code marketplace is generated from it.** Skills, agents and MCP servers that support `claude-code` become plugins under `plugins/claude-code/`, and bundles become plugins that depend on them. After changing anything here, run `cargo xtask gen-marketplace`; CI fails if it is stale and runs `claude plugin validate --strict` on every plugin.
- **No symlinks**, and no files over 1 MiB. Every path must be portable across Linux, macOS and Windows: ASCII only, no Windows reserved names, and no two paths that differ only in letter case.
