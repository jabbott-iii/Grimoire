# The Grimoire catalog

Every asset lives in its own directory, `catalog/<kind>/<id>/`. Bundles live in `catalog/bundles/<id>.toml`.

- Adding an asset is a **data-only change** under this directory.
- `cargo xtask validate` checks everything described here, and CI runs it.
- The design is in [`docs/architecture.md`](../docs/architecture.md) §3–§6.

| Kind | Directory | Content |
|---|---|---|
| rule | `rules/<id>/` | `rule.md`: Markdown instructions loaded automatically |
| skill | `skills/<id>/` | An [Agent Skills](https://agentskills.io/specification) directory: `SKILL.md` and optional `scripts/`, `references/`, `assets/` |
| agent | `agents/<id>/` | `agent.md`: the subagent's system prompt |
| tool | `tools/<id>/` | A Rust crate building `grimoire-tool-<id>` (from Phase 3) |
| mcp-server | `mcp-servers/<id>/` | A Rust crate building `grimoire-mcp-<id>` on rmcp (from Phase 3) |

## Every asset needs

- **`asset.toml`**, the manifest. Its schema is [`schemas/asset.schema.json`](../schemas/asset.schema.json).
  - `id` must equal the directory name. Use 1–64 characters of `[a-z0-9-]`, and don't start with `claude-`, `anthropic-`, `anthropics-` or `cc-plugin-`.
  - `kind` must match the parent directory.
  - Unknown fields are errors.
- **`README.md`**, which explains what the asset does, when to use it, and security notes.
- **`CHANGELOG.md`**, with a `## <version>` (or `## [<version>]`) heading for the current version.
- **`evals/` or `tests/`**, with at least one file. Tools and MCP servers need `tests/`.

## Rules that catch people out

- **Explicit outputs per client.**
  - Each `[clients.<client>]` table lists exactly what is installed for that client: `src` (a file or directory in the asset), `to` (an anchor such as `rules:<id>.md`, `skills:<id>/SKILL.md`, `instructions`, `mcp` or `hooks`) and `strategy` (`copy`, `render`, `block` or `entry`).
  - A client is supported if, and only if, its table is present.
- **Skills keep their layout.**
  - `src = "X"` goes to `skills:<id>/X`.
  - Every client must install `SKILL.md` and everything under `scripts/`, `references/` and `assets/`.
  - Only the spec's frontmatter fields are allowed, and `allowed-tools` is refused.
  - Every script must be referenced from `SKILL.md`.
- **Agents list their tools explicitly.** `[agent] tools = [...]` is required; `[]` is legal. Capabilities that imply a permission must be declared under `[permissions]`: `read`/`search` imply filesystem `read`, `edit` implies filesystem `write`, `execute` implies `shell`, and `web` implies `network`. An `mcp:<server>` capability needs that server in `[requires]`.
- **Permissions match behaviour.**
  - A tool with `hooks` declares `hooks = true`, and a tool without hooks must not.
  - Every environment variable an MCP server reads is listed in `permissions.env`.
  - A skill that bundles `scripts/` declares `shell = true`.
- **Skill references.**
  - Links (`[text](path)`, images, HTML `src`/`href`) must name a file inside the skill that every client installs. `file:` URLs are refused.
  - A script counts as documented when `SKILL.md` names its path, for example `scripts/x.sh`, in prose, inline code or a code block. Mentioning the `scripts/` directory alone does not count.
- **Rules declare no permissions**, and must not duplicate another rule.
- **Version bumps.** Any change to an installed or built file needs a version bump and a CHANGELOG entry. `README.md`, `CHANGELOG.md`, `evals/` and `tests/` are exempt.
- **Removing an asset** is allowed only if it was already `maturity = "deprecated"` (with a named `replacement`) at the comparison base: on `main` for a pull request, or at the previous tip for a push. Deprecate it in an earlier change first.
- **Versions** are SemVer without build metadata (`1.2.3`, `1.2.3-rc.1`). The id `synced` is reserved, as are the prefixes above.
- **No symlinks**, and no files over 1 MiB. Every path must be portable across Linux, macOS and Windows: ASCII only, no Windows reserved names, and no two paths that differ only in letter case.
