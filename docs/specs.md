# Specification register

This file lists every external format, protocol, registry or tool that Grimoire implements against. Each entry gives the primary source, the version or revision that was read, and the date it was checked.

**Rules for this file**

- Re-verify an entry before you implement or change the code that depends on it. Update the entry in the same change.
- Facts here were read from the primary source on the date shown. Anything not confirmed is marked **UNVERIFIED** and must not be implemented from memory.
- Many sources are unversioned living docs. For those, "Version" records what the page or its release channel showed on the check date.
- Statuses: **Verified** means all facts below were read from the source. **Partial** means some gaps are listed. **Unverified** means the source was unreachable or silent.

## Register

All rows were checked 2026-11-16. Section IDs below link each row to its notes.

| ID | Spec / source | Version or revision checked | Primary URL | Status |
|---|---|---|---|---|
| CC-* | Claude Code: memory, skills, subagents, settings/hooks, `.mcp.json`, plugins, marketplaces | CLI 2.1.326 (npm `latest`); live docs | https://code.claude.com/docs/en/ | Verified; CC-MKT Partial |
| SKILLS | Agent Skills specification | unversioned; spec file @ `217be54873` (2026-08-04) | https://agentskills.io/specification | Verified |
| AGENTSMD | AGENTS.md | unversioned; repo @ `d001185d79` (2026-09-10) | https://agents.md | Verified (convention, not normative) |
| CODEX-* | OpenAI Codex CLI: AGENTS.md, config/MCP, skills, agents, hooks, rules, plugins | `rust-v0.169.0` (2026-11-13) | https://learn.chatgpt.com/docs/ ; https://github.com/openai/codex | Verified (some facts from source only) |
| CURSOR-* | Cursor: rules, MCP, skills, subagents, hooks | IDE 3.27 / CLI v2026.10.22 (2026-10-22) | https://cursor.com/docs/ | Partial |
| GEMINI-* | Gemini CLI: context, settings/MCP, skills, subagents, hooks | v0.68.0 (2026-11-10), docs @ tag | https://github.com/google-gemini/gemini-cli | Partial |
| COPILOT-* | GitHub Copilot: instructions, agents, skills, MCP, hooks | CLI v1.0.112; VS Code 1.146.0; docs fetched 2026-11-16 | https://docs.github.com/en/copilot ; https://code.visualstudio.com/docs | Partial |
| MCP-SPEC | Model Context Protocol | **2026-07-28** (current final) | https://modelcontextprotocol.io/specification/2026-07-28 | Verified |
| RMCP | rmcp (official Rust SDK) | 3.10.0 (2026-11-12); supports 2026-07-28 | https://github.com/modelcontextprotocol/rust-sdk | Verified |
| MCP-REGISTRY | Official MCP Registry / `server.json` | registry 1.8.9; schema `2025-12-11`; **preview** | https://registry.modelcontextprotocol.io | Verified |
| MCPB | MCP bundles | manifest 0.4; `@anthropic-ai/mcpb` 2.1.2 | https://github.com/modelcontextprotocol/mcpb | Verified (not used in v1) |
| CRATES-TP | crates.io trusted publishing | docs @ 2026-10-02; `crates-io-auth-action` v1.0.6 | https://crates.io/docs/trusted-publishing | Verified |
| NPM-TP | npm trusted publishing and token policy | docs @ 2026-10-06; npm 11.21.2 / 12.2.2 | https://docs.npmjs.com/trusted-publishers | Partial (docs conflict) |
| PYPI-TP | PyPI trusted publishing | docs fetched 2026-11-16; `gh-action-pypi-publish` v1.14.3 | https://docs.pypi.org/trusted-publishers/ | Verified |
| MATURIN | maturin binary wheels | 1.16.0; maturin-action v1.51.0 | https://www.maturin.rs | Verified |
| BINSTALL | cargo-binstall metadata | 1.27.0 | https://github.com/cargo-bins/cargo-binstall | Verified |
| BREW | Homebrew taps | brew 7.0.14 | https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap | Partial |
| WINGET | winget-pkgs manifests | schema 1.28 (community repo) | https://github.com/microsoft/winget-pkgs | Partial (Learn page stale) |
| SCOOP | Scoop manifests and buckets | Scoop 0.6.0; wiki unversioned | https://github.com/ScoopInstaller/Scoop/wiki | Verified |
| DIST | dist (cargo-dist) | 0.34.0 (2026-11-02) | https://github.com/axodotdev/cargo-dist | Partial |
| RELEASE-PLZ | release-plz | 0.4.3 (2026-11-02) | https://release-plz.dev | Verified |
| RUNNERS | GitHub-hosted runners | docs fetched 2026-11-16 | https://docs.github.com/en/actions/reference/runners/github-hosted-runners | Partial |
| GHA-SEC | Actions hardening, lockfiles, SHA-pin policy, environments, OIDC | docs fetched 2026-11-16 | https://docs.github.com/en/actions/reference/security/secure-use | Verified |
| ATTEST | Artifact attestations, immutable releases | `actions/attest` v4.2.2 | https://github.com/actions/attest | Verified |
| COSIGN | Sigstore cosign | v3.1.4; installer v4.1.3 | https://github.com/sigstore/cosign | Partial |
| SBOM | cargo-cyclonedx, cargo-auditable, buildx attestations | 0.5.10 / 0.7.7 / docs fetched 2026-11-16 | https://github.com/CycloneDX/cyclonedx-rust-cargo | Verified |
| BASE-IMAGE | distroless static, Chainguard static, OCI annotations | `static-debian13`; image-spec v1.1.1 | https://github.com/GoogleContainerTools/distroless | Verified |
| SCAN | Trivy incident; Grype / scan-action | trivy v0.76.2; grype v0.122.0; scan-action v7.4.2 | https://github.com/anchore/scan-action | Verified |
| DENY | cargo-deny | 0.21.0; action v2.1.2 | https://github.com/EmbarkStudios/cargo-deny | Verified |
| UNUSED-DEPS | Cargo unused-dep lints, cargo-machete, cargo-shear, cargo-udeps | Rust 1.100.0; 0.10.1 / 1.14.1 / 0.1.61 | https://doc.rust-lang.org/stable/cargo/reference/lints.html | Verified |
| SECRETS | GitHub secret scanning, gitleaks, private vulnerability reporting | gitleaks v8.30.1 | https://docs.github.com/en/code-security | Verified |
| DEPENDABOT | Dependabot options | docs fetched 2026-11-16 | https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference | Verified |
| LYCHEE | lychee link checker | 0.24.2; action v2.9.0 | https://github.com/lycheeverse/lychee-action | Verified |
| RUST | Rust stable toolchain | 1.100.0 (channel 2026-11-12) | https://static.rust-lang.org/dist/channel-rust-stable.toml | Verified |
| CRATES | Candidate crate versions | see table | https://crates.io | Verified |

**Not yet researched, to verify in the phase that needs them:**

| Topic | Phase |
|---|---|
| A JSON/JSONC editor that preserves comments | 2 |
| Whether `gh actions-lock` works with the gh version CI uses | 1 |
| The exact Fulcio identity string for keyless verification docs | 5 |
| GHCR OCI 1.1 referrers support for cosign v3 | 6 |
| `winget validate` availability on GitHub Windows runners | 5 |
| Whether npm staged publishing can create a brand-new package | 5 |

---

## Claude Code

Claude Code docs are unversioned live pages, fetched as `.md` from `https://code.claude.com/docs/en/<page>.md` (index: `https://code.claude.com/docs/llms.txt`). Version gates such as "v2.1.277+" are quoted from the pages.

**Release checked:** npm `@anthropic-ai/claude-code`
- `latest`/`next` = 2.1.326 (2026-11-16). The `stable` tag = 2.1.311 (2026-10-26).
- GitHub release `v2.1.326`. CHANGELOG top entry is `2.1.326`.
- Sources: https://registry.npmjs.org/@anthropic-ai/claude-code, https://github.com/anthropics/claude-code

### CC-MEM: memory and rules
URL: https://code.claude.com/docs/en/memory.md. Checked 2026-11-16. Status: Verified.

- **Load order** is broadest to most specific, concatenated rather than overridden:
  1. managed `CLAUDE.md`;
  2. `~/.claude/CLAUDE.md`;
  3. project `./CLAUDE.md` or `./.claude/CLAUDE.md`;
  4. `./CLAUDE.local.md`.
- **Ancestors:** files in the cwd and every ancestor load at launch, root to cwd. Subdirectory `CLAUDE.md` files load on demand, when Claude touches files in that directory.
- **HTML comments:** block-level comments are stripped before injection. Grimoire's managed-block markers therefore cost no context.
- **Imports:** `@path/to/file`, relative to the importing file, with a maximum depth of four hops. Imports outside the working directory trigger a one-time approval dialog.
- **Size:** keep files under 200 lines. A file over 4 MiB is skipped. On-demand files are truncated at a per-file limit (40,000 chars on a 200K window).
- **`.claude/rules/`:** all `.md` files are discovered recursively. Rules without `paths` load at launch with the same priority as `.claude/CLAUDE.md`. User rules live in `~/.claude/rules/`.
  - The `paths` frontmatter is "the only field Claude Code reads from a rule". It takes a YAML list or a comma-separated string of globs, with brace expansion, relative to the directory that contains `.claude`.
  - A path-scoped rule loads when Read/Write/Edit touches a matching file. **Invalid YAML means the rule loads unscoped.**
- **AGENTS.md** is read natively from v2.1.277, through the built-in plugin `cc-plugin-agents-md@builtin`.
  - The default mode `claude-md-or-agents-md` reads `AGENTS.md` only when no `CLAUDE.md`, `.claude/CLAUDE.md` or `CLAUDE.local.md` exists in the cwd or above. Rules files don't count for that check.
  - Other modes: `claude-md-and-agents-md`, `claude-md`, `managed-only`. They are honoured only in user, CLI or managed settings, not project settings.
  - Not read: `AGENTS.override.md`, `AGENTS.local.md`, `.agents/`.

### CC-SKILL: skills
URL: https://code.claude.com/docs/en/skills.md. Checked 2026-11-16. Status: Verified.

- **Locations:**
  - Project: `.claude/skills/<name>/SKILL.md`, found by walking ancestors to the repo root. Nested `<subdir>/.claude/skills/` loads lazily.
  - Personal: `~/.claude/skills/`.
  - Plugin: `<plugin>/skills/<name>/SKILL.md`, invoked as `/plugin:skill`.
  - Reserved folder names: `synced`, `anthropic-skills*`.
- **Precedence:** enterprise > personal > project. A skill beats a `.claude/commands/` file of the same name.
- **Commands:** "Custom commands have been merged into skills". `.claude/commands/` still works.
- **Frontmatter:** all fields are optional in Claude Code and unknown fields are silently ignored. Open-spec fields per these docs: `name`, `description`, `license`, `compatibility`, `metadata`, `allowed-tools`. **Everything else is a Claude Code extension** (`when_to_use`, `argument-hint`, `arguments`, `disable-model-invocation`, `user-invocable`, `disallowed-tools`, `model`, `effort`, `context`, `agent`, `background`, `hooks`, `paths`, `shell`). "claude.ai upload / Skills API reject extensions with a hard error".
- **Listing:** `description` + `when_to_use` are truncated at 1,536 chars. The listing budget is 1% of the context window.

### CC-AGENT: subagents
URL: https://code.claude.com/docs/en/sub-agents.md. Checked 2026-11-16. Status: Verified.

- **Precedence:** managed > `--agents` CLI JSON > `.claude/agents/` (closest ancestor wins) > `~/.claude/agents/` > plugin `agents/`. The scan is recursive. Identity comes from `name`, and the filename needn't match.
- **Format:** Markdown with YAML frontmatter; the body is the system prompt. **Required: `name`, `description`.** Files missing either, or with bad YAML, are silently skipped. Unknown fields are ignored.
  - `name` is at most 256 chars, has no `:`, and can't start with `-`.
- **Optional fields:**
  - `tools` (comma string or YAML list; **inherits all tools if omitted**), `disallowedTools`.
  - `model` (`sonnet`/`opus`/`haiku`/`fable`/full ID/`inherit`).
  - `permissionMode`, `maxTurns`, `skills`, `mcpServers`, `hooks`, `memory`, `background`, `effort`, `isolation`, `color`, `initialPrompt`, and others.
  - Plugin agents ignore `permissionMode`, `hooks`, `mcpServers` and `initialPrompt`.
- **Tool names:**
  - Built-ins: `Read, Grep, Glob, Bash, Edit, Write, NotebookEdit, WebFetch, WebSearch, TodoWrite, Skill, LSP, PowerShell, Monitor, Agent`.
  - MCP: `mcp__<server>__<tool>`, `mcp__<server>__*`.

### CC-SET: settings, permissions, hooks
URLs: https://code.claude.com/docs/en/settings.md, …/permissions.md, …/hooks.md. Checked 2026-11-16. Status: Verified.

- **Settings precedence:** managed > `--settings` > `.claude/settings.local.json` > `.claude/settings.json` > `~/.claude/settings.json`. List keys such as `permissions.allow` merge across files.
- **Schema:** `"$schema": "https://json.schemastore.org/claude-code-settings.json"` (draft-07). It "can lag behind the newest CLI releases".
- **Permission rules:** `Tool` or `Tool(specifier)`, in `permissions.allow|ask|deny`. Evaluation is deny → ask → allow, first match wins. A deny at any level can't be overridden. Project `allow` rules apply only after workspace trust.
- **Hooks:**
  - Shape: `{"hooks":{"<Event>":[{"matcher":"…","hooks":[{handler}]}]}}`.
  - Sources: settings files, plugin `hooks/hooks.json`, and skill and agent frontmatter. Identical handlers run once.
  - Handler types: `command` (`command`, `args` exec form, `async`, `shell`, `onFailure`), `http`, `mcp_tool`, `prompt`, `agent`.
  - The default timeout is 600 s for `command`. Exit code 2 blocks.
  - Settings-file hooks wait for workspace trust in interactive sessions, but **`-p`/SDK sessions run them without a trust prompt**.

### CC-MCP: project MCP config
URL: https://code.claude.com/docs/en/mcp.md. Checked 2026-11-16. Status: Verified.

- **File:** `.mcp.json` at the repo root, `{"mcpServers":{"<name>":{…}}}`.
- **`type`:** `stdio` (the default when omitted), `http` (alias `streamable-http`), `sse` (deprecated), `ws`. A `url` with no `type` is an error.
- **Fields:** stdio uses `command`, `args`, `env`. Remote uses `url`, `headers`, `headersHelper`, `timeout`, `oauth`.
- **Env expansion:** `${VAR}` and `${VAR:-default}` in `command`, `args`, `env`, `url`, `headers`. An unset variable with no default is left literal, with a warning.
- **Approval:** interactive sessions prompt before using `.mcp.json` servers. `-p`, SDK and cloud sessions load them without asking.
- **Scope precedence:** local > project > user > plugin. The whole entry wins, with no field merge.

### CC-PLUGIN: plugins
URLs: https://code.claude.com/docs/en/plugins/manifest-reference.md, …/plugins/components.md, …/plugins/security.md. Checked 2026-11-16. Status: Verified.

- **Manifest:** `.claude-plugin/plugin.json` is optional. If present, **only `name` is required** (kebab-case).
  - Reserved prefixes and names: `claude-`, `anthropic-`, `anthropics-`, `cc-plugin-`, `claude-code`, and similar.
  - Unknown keys are stripped with a warning. Component paths must start with `./` and stay inside the plugin root.
- **Default layout:** `skills/<n>/SKILL.md`, `commands/*.md`, `agents/*.md`, `hooks/hooks.json`, `.mcp.json`, `.lsp.json`, `output-styles/`, `bin/` (added to the Bash PATH).
  - **A root-level `CLAUDE.md` in a plugin is not loaded.** Rules can't ship through a plugin.
- **Variables:** `${CLAUDE_PLUGIN_ROOT}` (changes on update), `${CLAUDE_PLUGIN_DATA}` (persists).
- **Versioning:** `plugin.json` `version` > marketplace entry `version` > source-derived (a git SHA for git sources).
- **Security:** "A Claude Code plugin you install can execute arbitrary code on your machine with your user privileges." Hooks and MCP servers run outside the sandbox.

### CC-MKT: plugin marketplaces
URLs: https://code.claude.com/docs/en/plugins/marketplace-reference.md, …/plugins/cli-reference.md. Checked 2026-11-16. Status: Partial (no published JSON Schema URL for `marketplace.json` or `plugin.json` was found).

- **File:** `<repo root>/.claude-plugin/marketplace.json`. Relative plugin sources resolve from the repo root.
- **Top-level fields:** **required** `name`, `owner{name}`, `plugins`. Optional: `$schema`, `description`, `version`, `metadata.pluginRoot`, and others.
- **Plugin entry:** **required** `name`, `source`. Optional: `description`, `version`, `category`, `tags`, `strict` (default `true`), `dependencies`, plus any `plugin.json` field.
  - If the plugin has no `plugin.json`, the entry is the manifest.
  - `strict: false` with both a `plugin.json` and entry components is a load error.
- **Source types:** relative `./…`, `github{repo,ref,sha}`, `url{url,ref,sha}`, `git-subdir`, `npm`, `archive{url,sha256}`, `command`.
- **Reserved marketplace names** include `agent-skills`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-*`, `builtin`, `inline`.
- **Commands:**
  - `claude plugin marketplace add <owner/repo|git-url|url|path>`, or `/plugin marketplace add`.
  - `claude plugin install <plugin>@<marketplace>`.
  - **`claude plugin validate <path> [--strict] [--json]`.**
- **Team settings:** `extraKnownMarketplaces`, `enabledPlugins` (`"plugin@marketplace": true`).

---

## Agent Skills and AGENTS.md

### SKILLS: Agent Skills specification
- **Source:** https://agentskills.io/specification (markdown twin `…/specification.md`), from https://github.com/agentskills/agentskills `docs/specification.mdx`. Checked 2026-11-16. Status: Verified.
- **Version:** unversioned. There are no tags or releases. The spec file last changed in commit `217be54873` (2026-08-04); repo HEAD is `83021e2be4` (2026-11-04).
- **Governance:** "Releases of the specification will be tagged and will carry a changelog". None exist yet. The project is an LF Projects series under the Agentic AI Foundation, with spec and code under Apache-2.0.
- **Directory:** a skill is a directory containing at least `SKILL.md`. Optional `scripts/`, `references/`, `assets/`, "and any additional files or directories". Extra files are therefore allowed, so a catalog asset directory can also be the skill directory.
- **Frontmatter:**
  - `name` (required): 1–64 chars; lowercase `a-z`, `0-9` and `-`; no leading, trailing or double hyphen; "Must match the parent directory name".
  - `description` (required): 1–1024 chars; should say what the skill does and when to use it.
  - Optional: `license`, `compatibility` (1–500 chars), `metadata` (string→string map), `allowed-tools` ("Experimental", space-separated).
- **Body:** "no format restrictions".
- **Progressive disclosure:**
  - Metadata is about 100 tokens, loaded at startup. Instructions should be "< 5000 tokens recommended", and `SKILL.md` "under 500 lines".
  - Resources load on demand. File references are relative to the skill root, kept "one level deep from `SKILL.md`".
- **Validator `skills-ref`:**
  - v0.1.0, Python, "intended for demonstration purposes only".
  - Rejects frontmatter keys outside the six spec fields. NFKC-normalizes the name and the directory before comparing them. Uses `isalnum()`, which is looser than the spec's `a-z0-9`.
  - **Grimoire implements the spec's stricter rules natively in Rust and does not depend on `skills-ref`.**
- **Locations:** the spec "does not mandate where skill directories live". Client-implementation guidance suggests `<project>/.<client>/skills/` and `<project>/.agents/skills/`, calling the latter "a widely-adopted convention for cross-client skill sharing".
- **Clients:** 46 listed, including Claude Code, Codex, Cursor, Gemini CLI, GitHub Copilot and VS Code. No per-client paths are given.

### AGENTSMD: AGENTS.md
- **Source:** https://agents.md, https://github.com/agentsmd/agents.md (HEAD `d001185d79`, 2026-09-10). Checked 2026-11-16. Status: Verified (it is a convention, not a normative spec).
- **Version:** unversioned. There is no spec file, tags or releases. It is stewarded by the Agentic AI Foundation (Linux Foundation); technical charter adopted 2025-12-08.
- **Format:** "just standard Markdown", with no required fields. The file sits at the repo root.
- **Nesting:** "The closest AGENTS.md to the edited file wins; explicit user chat prompts override everything."
- **Supporting tools:** 23 listed, including Codex, Cursor, Gemini CLI (via `context.fileName`), GitHub Copilot coding agent, VS Code, Jules, Aider, Zed, Windsurf, Devin and Junie.

---

## OpenAI Codex

- **Release checked:** `rust-v0.169.0` (stable, 2026-11-13). Newest prerelease `rust-v0.171.0-alpha.10` (2026-11-16). Source: https://api.github.com/repos/openai/codex/releases.
- **Docs:** https://learn.chatgpt.com/docs/… (`developers.openai.com/codex/*` 308-redirects there). The repo's `docs/*.md` files are stubs. Some facts were confirmed in `codex-rs` source and are marked *(source)*.

### CODEX-AGENTS: AGENTS.md discovery
Checked 2026-11-16. Status: Verified.

- **Global file:** `~/.codex/AGENTS.override.md`, otherwise `AGENTS.md`; the first non-empty one wins.
- **Project walk:** from the git root down to cwd, at most one file per directory, in the order `AGENTS.override.md` → `AGENTS.md` → `project_doc_fallback_filenames`. Files are concatenated root→cwd.
- **Size:** `project_doc_max_bytes`, default **32 KiB**. *(source)* It is one shared budget across all project files, and the file that overflows is truncated. The docs disagree on whether the cap is per-file or combined.
- **Trust:** *(source)* project AGENTS.md files are not loaded in untrusted projects.

### CODEX-CONFIG: config.toml and MCP
Checked 2026-11-16. Status: Verified.

- **Locations:** user `~/.codex/config.toml`. Project `.codex/config.toml`, root→cwd, closest wins, "trusted projects only".
- **Precedence:** CLI > project > profile > user > cloud-managed > `/etc/codex/config.toml`.
- **`[mcp_servers.<id>]`, stdio:** `command` (required), `args`, `env`, `env_vars`, `cwd`.
- **`[mcp_servers.<id>]`, Streamable HTTP:** `url` (required), `bearer_token_env_var`, `http_headers`, `env_http_headers`, `http_headers_helper`, `auth`, `oauth.*`, `scopes`, `oauth_resource`.
- **Common keys:** `startup_timeout_sec` (10), `tool_timeout_sec` (60), `enabled`, `required`, `enabled_tools`/`disabled_tools`, `default_tools_approval_mode`, `tools.<tool>.approval_mode`.

### CODEX-SKILLS: skills
Checked 2026-11-16. Status: Verified (paths that appear only in source are noted).

- **Documented locations:** `.agents/skills` in every directory from cwd up to the repo root; `$HOME/.agents/skills`; `/etc/codex/skills`.
- **Source only:** `.codex/skills` (repo scope) and the deprecated `~/.codex/skills`.
- **Parser:** reads `name` (max 64), `description` (required) and `metadata.short-description`. Other keys are ignored.
- **Optional metadata file:** `agents/openai.yaml` inside the skill (`interface.*`, `policy.allow_implicit_invocation`, `dependencies.tools[]`).

### CODEX-SUBAGENTS: custom agents
Checked 2026-11-16. Status: Verified.

- **Location:** one **TOML** file per agent in `.codex/agents/` (project) or `~/.codex/agents/`.
- **Required fields:** `name`, `description`, `developer_instructions`.
- **Optional fields:** `model`, `model_reasoning_effort`, `features.{shell_tool,apps,plugins,memories,request_permissions_tool} = false`, `skills.config`, and others.
- **Not overridable:** `sandbox_mode`, `approval_policy` and `mcp_servers` in an agent file **do not** override the parent session.
- **Consequence:** per-agent tool restriction in Codex is coarse (feature switches only). See ADR-0005.

### CODEX-HOOKS, CODEX-RULES, CODEX-PLUGINS
Checked 2026-11-16. Status: Verified.

- **Hooks:**
  - Sources: `<repo>/.codex/hooks.json` or inline `[hooks]` in `.codex/config.toml`. The shape mirrors Claude Code's (`{"hooks":{"<Event>":[{"matcher","hooks":[…]}]}}`).
  - Project hooks need a trusted project, and non-managed hooks must be trusted by hash via `/hooks`.
  - Handler types: `command` and `mcp_tool`.
- **"Rules":** `.codex/rules/*.rules` are **execpolicy** (Starlark `prefix_rule(...)`) command-approval rules. This is a different concept from Grimoire's *rules* asset kind, and Grimoire does not render to it in v1.
- **Plugins:**
  - Manifest: root `plugin.json` (`$schema` `https://agent-plugins.org/schemas/1.0.0/plugin.schema.json`; that schema's contents were **UNVERIFIED**).
  - Marketplaces: `.agents/plugins/marketplace.json`, and also the **legacy `$REPO_ROOT/.claude-plugin/marketplace.json`**.
- **Custom prompts** (`~/.codex/prompts`): the docs say "deprecated", and PR #16115 (merged 2026-03-28) removed discovery. Out of scope.

---

## Model Context Protocol

### MCP-SPEC: protocol specification
- **Version:** **2026-07-28 is the current final version.** The owner's expectation is correct.
  - GitHub release `2026-07-28` published 2026-07-28 (prerelease=false). `schema/2026-07-28/schema.ts` has `LATEST_PROTOCOL_VERSION = "2026-07-28"`.
  - Final versions: 2024-11-05, 2025-03-26, 2025-06-18, 2025-11-25, 2026-07-28. A `2024-10-07` tag exists, but its status as a published version is UNVERIFIED.
  - Next release expected `2027-03-30`, on a six-month cadence (blog, 2026-11-11).
- **Source:** https://modelcontextprotocol.io/specification (redirects to `/specification/2026-07-28`); https://github.com/modelcontextprotocol/modelcontextprotocol (read at main `05aeb07`; the live site matched for the pages checked). Checked 2026-11-16. Status: Verified.
- **Changes from 2025-11-25** (changelog):
  - The protocol is **stateless**. `initialize` is removed, and every request carries `_meta` `io.modelcontextprotocol/protocolVersion` and `clientCapabilities` (SEP-2575).
  - Servers **MUST** implement `server/discover`.
  - `Mcp-Session-Id` and sessions are removed. `subscriptions/listen` replaces the GET stream.
  - `ping` and `logging/setLevel` are removed. Tasks moved to an extension.
  - MRTR (`InputRequiredResult`) replaces server-initiated requests.
  - `Mcp-Method`/`Mcp-Name` headers are required.
  - Roots, Sampling and Logging are deprecated. DCR is deprecated in favour of CIMD.
- **stdio:**
  - Newline-delimited messages with no embedded newlines.
  - The server "**MUST NOT** write anything to its `stdout` that is not a valid MCP message" and "**MAY** write UTF-8 strings to `stderr`".
  - The server **SHOULD** exit on stdin EOF.
- **Streamable HTTP:**
  - A single POST endpoint. Each request gets a JSON response or a request-scoped SSE stream.
  - "Servers **MUST** validate the `Origin` header". Local servers "**SHOULD** bind only to localhost".
  - Every POST **MUST** carry `MCP-Protocol-Version`, matching `_meta`.
  - HTTP+SSE (2024-11-05) is Deprecated: "New implementations **SHOULD NOT** adopt it".
- **Authorization:**
  - "Authorization is **OPTIONAL**". HTTP implementations **SHOULD** conform. stdio "**SHOULD NOT** follow this specification, and instead retrieve credentials from the environment".
  - When implemented, it uses OAuth 2.1. Servers **MUST** serve RFC 9728 Protected Resource Metadata.
  - Clients use RFC 8707 `resource` and PKCE S256.
  - Servers "**MUST** validate that access tokens were issued specifically for them" and "**MUST NOT** accept or transit any other tokens".
- **Tools:**
  - `inputSchema` (type `object`) is required. `outputSchema` is optional. Both default to JSON Schema **2020-12**.
  - `structuredContent` must conform to `outputSchema`. Names are 1–128 chars from `[A-Za-z0-9_.-]`.
- **Annotations** (hints only; "Clients should never make tool use decisions based on `ToolAnnotations` received from untrusted servers"):

  | Annotation | Default |
  |---|---|
  | `title` | none |
  | `readOnlyHint` | false |
  | `destructiveHint` | true |
  | `idempotentHint` | false |
  | `openWorldHint` | true |

- **Security best practices:** no token passthrough; SSRF defences; and one-click installers **MUST** "Show the exact command that will be executed, without truncation". Grimoire's `plan` output satisfies this for MCP config entries.

### RMCP: official Rust SDK
- **Version:** **rmcp 3.10.0** (with rmcp-macros 3.10.0), published 2026-11-12. Edition 2024, `rust-version = "1.88"`, Apache-2.0.
- **Source:** https://crates.io/api/v1/crates/rmcp; https://github.com/modelcontextprotocol/rust-sdk at tag `rmcp-v3.10.0` (`53e64a9`). Checked 2026-11-16. Status: Verified.
- **Protocol support:** **2026-07-28 is supported, so there is no gap.**
  - `crates/rmcp/src/model.rs` defines `V_2026_07_28` … `V_2024_11_05`, with `LATEST = V_2026_07_28`.
  - README: "implements the stable MCP `2026-07-28` specification while remaining fully compatible with the `2025-11-25` release". ROADMAP reports 100% server conformance (30/30).
  - Milestone "2026-07-28 spec" is closed (28/28).
  - Open follow-ups: #1108 and PR #1256 (residual session/GET-stream handling at ≥2026-07-28), #1425, #1347/#1375/#1303.
- **SDK tier:** the Rust SDK is **Tier 1** on https://modelcontextprotocol.io/docs/sdk.
- **Features:**
  - `default = ["base64","macros","server"]`.
  - `transport-io` provides stdio.
  - `transport-streamable-http-server` provides the HTTP server.
  - `auth*`, `schemars` (schemars 1.x).
  - Macros: `#[tool]`, `#[tool_router]`, `#[tool_handler]`, with annotation attributes `read_only_hint`, `destructive_hint`, `idempotent_hint`, `open_world_hint`.
- **Gaps Grimoire must close itself:**
  1. **Origin validation is disabled by default.** `StreamableHttpServerConfig::default()` has `allowed_origins = []`, which "disables Origin validation for backward compatibility". Grimoire servers must call `enforce_origin_validation()` or `with_allowed_origins(..)`, and test it. Host validation defaults to localhost only.
  2. **Auth is client-side only.** rmcp's OAuth support (PKCE, RFC 8707, RFC 9728 discovery, DCR, CIMD) is for clients. Server-side bearer validation appears only in examples (`examples/servers/src/simple_auth_streamhttp.rs` and others). Grimoire needs its own server-side layer: an RFC 9728 metadata endpoint, `WWW-Authenticate` challenges, and audience and issuer validation. Tracked in `PLAN.md`.

### MCP-REGISTRY: official MCP Registry
- **Version:** deployed 1.8.9 (`GET /v0/version`, build 2026-11-10). Status **preview**: "Breaking changes or data resets may occur before general availability". The `/v0.1` API has been frozen since 2025-10-24.
- **Source:** https://registry.modelcontextprotocol.io; https://github.com/modelcontextprotocol/registry. Checked 2026-11-16. Status: Verified.
- **`server.json` schema:** `2025-12-11` (`https://static.modelcontextprotocol.io/schemas/2025-12-11/server.schema.json`).
- **`registryType` values:** `npm`, `pypi`, `oci`, `nuget`, `mcpb`, **`cargo`** (crates.io only).
- **Ownership proof:**
  - cargo: `mcp-name: <server-name>` must appear as **visible** README text, because crates.io strips HTML comments.
  - OCI: annotation `io.modelcontextprotocol.server.name`, with `ghcr.io` allowed.
- **Namespace auth:** `io.github.<user>/*` via GitHub. **`mcp-publisher login github-oidc`** works in Actions (`id-token: write`).
- **Publisher CLI:** `mcp-publisher` v1.8.9, released with Sigstore bundles and SBOMs.
- **Plan:** publishing to the registry is deferred until it leaves preview (Phase 5 decision). It would be dry-run only before then.

### MCPB: MCP bundles
- **Version:** `@anthropic-ai/mcpb` 2.1.2 (2025-12-04). `LATEST_MANIFEST_VERSION = "0.4"`.
- **Source:** https://github.com/modelcontextprotocol/mcpb. Checked 2026-11-16. Status: Verified.
- **Binaries:** native binary servers are supported (`server.type = "binary"`; platforms `darwin|win32|linux`). There is no CPU-architecture field, so one bundle per OS cannot carry both arm64 and x64. Not in v1 scope.

---

## Cursor

- **Release checked:** Cursor IDE 3.27 and Cursor CLI v2026.10.22, both 2026-10-22. Source: https://cursor.com/docs/release-notes/rss.xml.
- **Docs:** https://cursor.com/docs/*.md (unversioned).

### CURSOR-RULES: rules and instruction files
URLs: https://cursor.com/docs/rules.md, https://cursor.com/help/customization/rules.md. Checked 2026-11-16. Status: Partial (nested `.cursor/rules` in subdirectories is not stated explicitly).

- **Location:** `.cursor/rules/`, with subfolders allowed. "Project rules must use the `.mdc` extension. A plain `.md` file in `.cursor/rules` is ignored".
- **Frontmatter:** `description`, `globs` (comma-separated), `alwaysApply`. These map to rule types:

  | Frontmatter | Rule type |
  |---|---|
  | `alwaysApply: true` | Always |
  | `false` + `globs` | Specific Files |
  | `false` + `description` only | Apply Intelligently |
  | neither | Manual |

- **Size:** keep rules under 500 lines.
- **Instruction files:** reads `AGENTS.md` at the root and nested, and **also reads `CLAUDE.md`** "the same way", always applied.
- **Legacy:** `.cursorrules` is "legacy and will be deprecated".

### CURSOR-MCP
URL: https://cursor.com/docs/mcp.md. Checked 2026-11-16. Status: Partial (`type` for remote servers is not documented).

- **Files:** `.cursor/mcp.json` and `~/.cursor/mcp.json`, with top-level `mcpServers`.
- **stdio:** `type: "stdio"`, `command`, `args`, `env`, `envFile`.
- **Remote:** `url`, `headers`, `auth{CLIENT_ID,…}`.
- **Interpolation:** **`${env:NAME}`**, which differs from Claude Code's `${NAME}`. Also `${workspaceFolder}` and `${userHome}`.

### CURSOR-SKILLS, CURSOR-AGENTS, CURSOR-HOOKS
URLs: https://cursor.com/docs/skills.md, …/subagents.md, …/hooks.md, …/reference/third-party-hooks.md. Checked 2026-11-16. Status: Verified.

- **Skills:**
  - Native locations: `.agents/skills/`, `.cursor/skills/`. Also loads **`.claude/skills/` and `.codex/skills/`** "for compatibility".
  - Frontmatter: `name` (required, must match the folder), `description` (required), `paths`, `disable-model-invocation`, `metadata`, `icon`, `color`.
- **Subagents:**
  - Locations: `.cursor/agents/`, and **also `.claude/agents/` and `.codex/agents/`**. `.cursor/` wins on a name clash.
  - Fields: `name`, `description`, `model`, **`readonly`**, `is_background`. **There is no per-tool allow-list; `readonly` is the only restriction.**
- **Hooks:**
  - File: `.cursor/hooks.json`, `{"version":1,"hooks":{"<event>":[{command,type,timeout,matcher,failClosed,…}]}}`. Exit code 2 blocks, and other non-zero exits fail open.
  - Cursor **also loads Claude Code hooks** from `.claude/settings*.json`.

---

## Gemini CLI

- **Release checked:** v0.68.0 (stable, 2026-11-10). Preview v0.69.0-preview.0. Source: https://api.github.com/repos/google-gemini/gemini-cli/releases.
- **Docs:** read from https://raw.githubusercontent.com/google-gemini/gemini-cli/v0.68.0/docs/…, pinned to the release tag. The in-repo changelog lags; the Releases list is authoritative.

### GEMINI-CONTEXT
`docs/cli/gemini-md.md`, `docs/reference/memport.md`. Checked 2026-11-16. Status: Verified.

- **Load order:**
  1. `~/.gemini/GEMINI.md`;
  2. the configured filename in the cwd and each ancestor up to `.git`;
  3. subdirectories (max 200);
  4. just-in-time files.
- **Context filename:** `context.fileName` accepts a string or array, e.g. `["AGENTS.md","GEMINI.md"]`.
- **Imports:** `@./file.md`, max depth 5.

### GEMINI-SETTINGS and MCP
`docs/reference/configuration.md`, `docs/tools/mcp-server.md`. Checked 2026-11-16. Status: Partial (see the precedence conflict below).

- **Schema:** `https://raw.githubusercontent.com/google-gemini/gemini-cli/main/schemas/settings.schema.json` (draft 2020-12).
- **Precedence:** project `.gemini/settings.json` overrides user settings, and system settings override project.
- **Env expansion:** string values accept `$VAR`, `${VAR}`, `${VAR:-DEFAULT}`. Undefined variables become empty.
- **`mcpServers.<name>` fields:** `command`, `args`, `env`, `cwd`, `url` (SSE), **`httpUrl` (Streamable HTTP)**, `headers`, `timeout` (ms), `trust` (bypasses confirmation; default false), `includeTools`, `excludeTools`, `oauth`, `authProviderType`. The schema also has `type` (`stdio`|`sse`|`http`).
- **Naming:** avoid `_` in server names, because tools are named `mcp_<alias>_<tool>`.
- **Redaction:** env vars matching `*TOKEN*`, `*SECRET*`, `*KEY*` are redacted unless explicitly listed in `env`.

### GEMINI-SKILLS, GEMINI-AGENTS, GEMINI-HOOKS, GEMINI-EXT
`docs/cli/skills.md`, `docs/core/subagents.md`, `docs/hooks/*.md`, `docs/extensions/reference.md`. Checked 2026-11-16. Status: Verified.

- **Skills:**
  - Locations: workspace `.gemini/skills/` or **`.agents/skills/`**, with `.agents` winning.
  - Frontmatter: `name`, `description`.
- **Subagents:**
  - Location: `.gemini/agents/*.md`.
  - Fields: `name` (required), `description` (required), **`tools`** (inherits all if omitted; wildcards `mcp_<server>_*`), `model`, `max_turns`, `timeout_mins`, `kind`.
  - Marked "a preview feature".
- **Hooks:**
  - Configured under `hooks` in settings.json.
  - Events: `BeforeTool`, `AfterTool`, `SessionStart` and others.
  - Timeout is in **ms** (default 60000). Exit code 2 blocks.
- **Extensions:** `gemini-extension.json` plus `commands/*.toml`, `skills/`, `agents/`, `hooks/hooks.json`. Out of scope for v1.
- **Custom commands:** `.gemini/commands/*.toml` (`prompt` required). Out of scope for v1.

---

## GitHub Copilot

- **Releases checked:** Copilot CLI v1.0.112 (2026-11-13); VS Code 1.146.0 (2026-11-11).
- **Docs:** GitHub docs are unversioned. Key pages were last committed 2026-10-19 to 2026-11-16 in `github/docs`, and were fetched via `https://docs.github.com/api/article/body?pathname=/en/copilot/…`. VS Code docs were read from `microsoft/vscode-docs` main (`DateApproved: 11/11/2026`).
- **Naming:** the "coding agent" is now **"Copilot cloud agent"**.
- **Fragmentation:** support differs by surface: VS Code, Visual Studio, JetBrains, Eclipse, Xcode, GitHub.com and the CLI. See the cheat-sheet table below.

### COPILOT-INSTR: instructions
Checked 2026-11-16. Status: Partial (surface tables disagree in places; see Unverified).

- **Repo-wide:** `.github/copilot-instructions.md` (no frontmatter).
- **Path-specific:** `.github/instructions/**/NAME.instructions.md`, with `applyTo` (comma-separated globs) and optional `excludeAgent: "code-review"|"cloud-agent"`.
- **Cloud agent:** also reads `AGENTS.md` (nearest wins), or a root `CLAUDE.md`/`GEMINI.md`.
- **Copilot CLI:** reads repo-wide, path-specific, `AGENTS.md`, `CLAUDE.md` and `GEMINI.md`, with no general precedence order.
- **VS Code:** depends on the selected harness. The Copilot harness reads `.github/copilot-instructions.md` or `AGENTS.md`. The Claude harness reads `CLAUDE.md` and `.claude/rules`.

### COPILOT-AGENTS, COPILOT-SKILLS, COPILOT-PROMPTS
Checked 2026-11-16. Status: Verified.

- **Custom agents:**
  - Location: `.github/agents/NAME.agent.md`.
  - Fields: `description` (**required**), `name`, `target` (`vscode`|`github-copilot`), **`tools`** (omitted means all, `[]` means none; aliases `execute`, `read`, `edit`, `search`, `agent`, `web`, `todo`; `server/tool`, `server/*`), `model`, `mcp-servers`.
  - The prompt body is capped at 30,000 chars.
  - VS Code also reads `.claude/agents/`.
- **Skills:**
  - Project locations: `.github/skills/`, **`.claude/skills/`**, **`.agents/skills/`**.
  - Frontmatter: `name`, `description` (required), `license`, `allowed-tools`.
- **Prompt files:** `.github/prompts/*.prompt.md`. Out of scope for v1 (VS Code now recommends skills for the Copilot harness).

### COPILOT-MCP
Checked 2026-11-16. Status: Verified.

- **VS Code, native Copilot harness:** workspace-root **`.mcp.json`** with `mcpServers`.
  - Fields: `type` `stdio|http|sse`, `command`, `args`, `env`, `cwd`, `url`, `headers`, `tools`.
  - Variables: `${NAME}` and `${NAME:-default}`.
- **VS Code, legacy:** `.vscode/mcp.json` (`servers`, `inputs`, `${env:NAME}`) is now the deprecated "Local harness" format.
- **Copilot CLI:** `.mcp.json` from cwd up to the repo root, or `.github/mcp.json`. Loaded only after folder trust.
- **Cloud agent:** configured in repository **Settings** UI JSON, not in a repo file. `tools` and `type` are required, and secrets must be prefixed `COPILOT_MCP_`. **Grimoire can't write this; `doctor` can only print guidance.**

### COPILOT-HOOKS
Checked 2026-11-16. Status: Verified.

- **Files:** `.github/hooks/*.json`, `{"version":1,"hooks":{"<event>":[{"type":"command","bash","powershell","cwd","env","timeoutSec"}]}}`.
- **Also reads:** inline `hooks` from `.claude/settings*.json`.
- **Events:** camelCase, e.g. `preToolUse`.
- **Surfaces:** hooks are available on GitHub.com and in the CLI. VS Code hooks are in preview.

**Copilot surface × feature** (GitHub "customization cheat sheet"; ✓ supported, P preview, ✗ not supported):

| Feature | VS Code | VS | JetBrains | Eclipse | Xcode | GitHub.com | CLI |
|---|---|---|---|---|---|---|---|
| Custom instructions | ✓ | ✓ | P | P | P | ✓ | ✓ |
| Custom agents | ✓ | ✓ | P | P | P | ✓ | ✓ |
| Agent skills | ✓ | ✓ | P | ✗ | ✗ | ✓ | ✓ |
| Hooks | P | ✗ | ✗ | ✗ | ✗ | ✓ | ✓ |
| MCP | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

---

## Package registries and installers

### CRATES-TP: crates.io trusted publishing
- **Version:** docs page source last changed 2026-10-02. Action `rust-lang/crates-io-auth-action` **v1.0.6** (2026-10-21), commit `3268eb413133e2960fa4ae0e963ca76380c54cd9`; `v1` is a branch.
- **Source:** https://crates.io/docs/trusted-publishing (rendered with JS, so read from `rust-lang/crates.io` `svelte/src/routes/docs/trusted-publishing/+page.svelte`); RFC 3691; https://blog.rust-lang.org/2026/01/21/crates-io-development-update/. Checked 2026-11-16. Status: Verified.
- **Status:** live for GitHub Actions. GitLab is in public beta.
- **First publish:** **needs an API token.** "Your crate must already be published to crates.io (initial publish requires an API token)". There is no pending-publisher equivalent.
- **Config:** repository owner, repository name, workflow filename, and an optional environment. The workflow needs `id-token: write`.
- **Tokens:** expire after 30 minutes, and the action revokes them in its post step.
- **Trusted-publishing-only mode:** a per-crate setting (`trustpub_only`) disables token publishing.
- **Blocked triggers:** `pull_request_target` and `workflow_run` can't use trusted publishing.

### NPM-TP: npm trusted publishing and token policy
- **Version:** `trusted-publishers.mdx` last edited 2026-10-06. npm CLI v12.2.2 / v11.21.2 (2026-11-04).
- **Source:** https://docs.npmjs.com/trusted-publishers, https://docs.npmjs.com/cli/v11/commands/npm-trust, https://docs.npmjs.com/staged-publishing, and GitHub changelog posts from 2025-07-31 to 2026-10-02. Checked 2026-11-16. Status: Partial (see conflicts).
- **Status:** GA since 2025-07-31 for scoped and unscoped packages, on GitHub-hosted runners only.
- **Requirements:** "npm CLI version 11.5.1 or later and Node version 22.14.0 or higher". `npm trust` and staged publishing need npm ≥ 11.15.0.
- **Package must exist:** "The package you're configuring must already exist on the npm registry" (per the `npm trust` docs). A new configuration must publish successfully within 48 hours.
- **Defaults:** configurations created after 2026-09-03 allow only **`npm stage publish`** by default. A maintainer approves each staged version with 2FA.
- **Provenance:** automatic for public repos and packages.
- **Token policy:**
  - Classic tokens were revoked on 2025-12-09.
  - Granular write tokens are capped at 90 days, with 2FA enforced.
  - "The ability to publish new package versions directly with a granular access token will be removed in **January 2027**."
- **Conflict:** the `npm trust` docs say one configuration per package, while the trusted-publishers page says up to 10.

### PYPI-TP and MATURIN
- **Version:**
  - `pypa/gh-action-pypi-publish` **v1.14.3** (2026-10-20), commit `7245906fb61fa33a13554f9d5fb856fe61758086`.
  - maturin **1.16.0** (2026-10-29).
  - `PyO3/maturin-action` **v1.51.0** (2026-04-09), commit `e83996d129638aa358a18fbd1dfb82f0b0fb5d3b`.
- **Source:** https://docs.pypi.org/trusted-publishers/ (incl. `creating-a-project-through-oidc/`); https://www.maturin.rs (read from `PyO3/maturin` `guide/src/*`). Checked 2026-11-16. Status: Verified.
- **Pending publisher:** it "will *create* the project when used for the first time", so **no token is ever needed**. It "does **not** … reserve a project's name".
- **Tokens:** 15 minutes. "Trusted publishing cannot be used from within a reusable workflow".
- **Attestations:** PEP 740 attestations are on by default (`attestations: true`).
- **maturin:**
  - Use `bindings = "bin"` (set it explicitly; the docs disagree on auto-detection). Binaries are packaged as scripts on PATH.
  - The PyPI-accepted tags cover manylinux and musllinux x86_64/aarch64, Windows x86_64/aarch64, and macOS x86_64/arm64.
  - `--compatibility pypi` rejects anything else.

### BINSTALL: cargo-binstall
- **Version:** v1.27.0 (2026-11-11).
- **Source:** https://github.com/cargo-bins/cargo-binstall `SUPPORT.md`, `SIGNING.md`. Checked 2026-11-16. Status: Verified.
- **Metadata:** `[package.metadata.binstall]` takes `pkg-url`, `bin-dir`, `pkg-fmt` (default `tgz`), `disabled-strategies`, and per-target `overrides`.
- **URL guessing:** default patterns include `{ name }-{ target }-v{ version }{ archive-suffix }` under `{ repo }/releases/download/v{ version }/`.
- **Signing:** **minisign only**, needing an explicit `pubkey`. Signatures are verified "if they are available". Sigstore and GitHub attestations are **not** documented.
- **Telemetry:** the quick-install strategy sends anonymized stats by default. Grimoire's docs should mention `--disable-telemetry`.

### BREW: Homebrew taps
- **Version:** brew 7.0.14 (2026-11-16). Doc commits: tap guide 2026-10-25, Acceptable-Formulae 2026-07-23.
- **Source:** https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap, …/Acceptable-Formulae, …/Acceptable-Casks, …/Tap-Trust; https://brew.sh/2026/06/11/homebrew-6.0.0/. Checked 2026-11-16. Status: Partial (no rule found on prebuilt-binary formulae in *third-party* taps).
- **Naming:** `brew tap <user>/<repo>` clones `github.com/<user>/homebrew-<repo>`. Formulae go in `Formula/`.
- **Tap trust:** since Homebrew 6.0.0, "Non-official taps require explicit trust by default". Users run `brew trust`, or install by fully qualified name.
- **Workflows:** `brew tap-new` generates `tests.yml`, `publish.yml` (manually dispatched), **`autobump.yml`** and `dependabot.yml`.
- **Placement:** homebrew/core wants builds from source; "Proprietary or platform-specific binary-only software belongs in a cask"; third-party taps take software outside those criteria.

### WINGET
- **Version:** the community repo requires manifest schema **1.28** (the winget-cli "latest" schema is 1.31.0). wingetcreate v1.28.1.0 (2026-11-11); Komac v2.17.2 (2026-11-03).
- **Source:** https://github.com/microsoft/winget-pkgs (`doc/Authoring.md` 2026-11-04, `doc/manifest/schema/1.28.0/installer.md`, PR template). Checked 2026-11-16. Status: Partial (the Learn page still shows ManifestVersion 1.12.0).
- **Manifests:** multi-file (version, defaultLocale, installer) under `manifests/<l>/<Publisher>/<Package>/<version>/`.
- **Zip with a portable exe:** `InstallerType: zip`, `NestedInstallerType: portable`, `NestedInstallerFiles` (`RelativeFilePath`, `PortableCommandAlias`). `arm64` is supported.
- **Submission:** a PR to `microsoft/winget-pkgs`, one version per PR. **Komac needs a classic GitHub PAT**, which would be a long-lived credential.

### SCOOP
- **Version:** Scoop v0.6.0 (2026-09-30). The wiki is unversioned.
- **Source:** https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests, …/App-Manifest-Autoupdate, …/Buckets, …/Criteria-for-including-apps-in-the-main-bucket. Checked 2026-11-16. Status: Verified.
- **Manifest fields:** `version`, `description`, `homepage`, `license` (SPDX); `architecture.{64bit,arm64}.{url,hash,bin}`; `checkver: "github"`; `autoupdate` with `hash.url = "$url.sha256"`.
- **Personal bucket:** made from BucketTemplate, whose Excavator autoupdates every 4 hours. Users run `scoop bucket add`.
- **Main bucket:** needs ≥500 stars and ≥150 forks.

---

## Release tooling

### DIST: dist (formerly cargo-dist)
- **Version:** **v0.34.0** (2026-11-02). Last commit on main 2026-11-15.
- **Source:** https://github.com/axodotdev/cargo-dist (book, `cargo-dist/src/backend/ci/github.rs`, `templates/ci/github/partials/publish_npm.yml.j2`); https://blog.mistys-internet.website/post/2025/07/20/dist-0-dot-28-dot-1-is-out. Checked 2026-11-16. Status: Partial (whether the generated workflows can emit SHA-pinned action refs is **UNVERIFIED**).
- **Maintenance:**
  - Of the 100 commits since 2026-08-16, 55 are Dependabot, 28 are one maintainer, and the rest come from about 8 community contributors.
  - In 2025-07 the maintainer wrote of being "granted permission to resume development" toward a "community-supported model".
  - No official axo announcement was found.
- **Targets:** x86_64/aarch64 for macOS, Windows and Linux gnu, plus static musl.
- **Installers:** `shell`, `powershell`, `npm`, `homebrew`, `msi`.
- **npm installer:** "fetch[es] your prebuilt archives" at install time, so there are **no per-platform packages**. Its publish job uses `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` on Node 20, which is below the trusted-publishing minimum and **will break when direct granular-token publishing is removed in January 2027**.
- **Homebrew publish:** needs a `HOMEBREW_TAP_TOKEN` secret.
- **Attestations:** `github-attestations = true` (opt-in) generates `actions/attest@v4`.

### RELEASE-PLZ
- **Version:** **v0.4.3** (2026-11-02). Last commit 2026-11-16. `release-plz/action` v0.5.148.
- **Source:** https://release-plz.dev (read from the repo `website/docs/`). Checked 2026-11-16. Status: Verified.
- **Features:**
  - Release PRs.
  - Changelogs via git-cliff, used as a library.
  - Tags `<package>-v<version>`.
  - `cargo publish` with **built-in crates.io trusted publishing**, which needs `id-token: write` and no token. It notes that "new crates can't be published with trusted publishing".
  - `git_only` mode for packages that aren't published.
- **Limits:** it "doesn't build and release binaries". Tags pushed with `GITHUB_TOKEN` can't trigger other workflows.

### RUNNERS: GitHub-hosted runners
- **Source:** https://docs.github.com/en/actions/reference/runners/github-hosted-runners; https://github.com/actions/runner-images; changelog 2025-09-19 and 2026-02-26. Checked 2026-11-16. Status: Partial (no retirement date for `macos-26-intel`).
- **Cost:** standard runners are "free and unlimited on public repositories".
- **Linux arm64:** `ubuntu-24.04-arm`, `ubuntu-22.04-arm`, `ubuntu-26.04-arm`.
- **Windows arm64:** `windows-11-arm`.
- **macOS Intel:** `macos-15-intel`, `macos-26-intel`. x86_64 macOS support is to end after the macOS 15 image retires in "Fall 2027", but `macos-26-intel` was added in 2026.
- **x64 defaults:** `ubuntu-latest` moves to 26.04 in November 2026. `windows-2025`/`windows-2022`.

---

## CI and supply-chain security

### GHA-SEC: GitHub Actions hardening
- **Source:**
  - https://docs.github.com/en/actions/reference/security/secure-use
  - …/concepts/security/workflow-dependency-lockfiles
  - changelog 2025-08-15 (SHA-pinning policy), 2026-10-28 (lockfiles GA), 2026-11-02 (`pull_request_target` blocked by default)
  - https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments
  - …/reference/security/oidc

  Checked 2026-11-16. Status: Verified.
- **Workflow lockfiles:** GA **2026-10-28**. The file is `.github/workflows/actions.lock`, generated by the `gh actions-lock` extension, and covers transitive action dependencies but not reusable workflows. GitHub calls it the "strongest protection available, and supersedes SHA pinning". Dependabot updates it.
- **SHA pinning:** "Pin actions to a full-length commit SHA". A repo or org policy, "Require actions to be pinned to a full-length commit SHA", has existed since 2025-08-15 and fails unpinned workflows.
- **`GITHUB_TOKEN`:** default it to read-only and grant more per job. `permissions: {}` disables everything.
- **`pull_request_target`:** **blocked by default in public repos from 2026-12-02**. `actions/checkout` v7 (latest v7.0.1, commit `3d3c42e5aac5ba805825da76410c181273ba90b1`) refuses fork PR heads under it.
- **Environments:** on Free, Pro and Team plans, required reviewers are "only available for public repositories". This repo is public. One approval suffices, and environment secrets unlock only after the rules pass.
- **OIDC:** repos created after 2026-07-15 use an immutable `sub`: `repo:OWNER@OWNER-ID/REPO@REPO-ID:ref:…`. **This repo was created 2026-11-16**, so its owner ID is `84321953` and its repo ID `1460666977` (from `gh api`).
- **Lint tools:**
  - zizmor v1.32.0 (2026-11-02); zizmor-action v0.8.1, commit `45d7276c9ca790aabd705e31ca99c6185c3e76dd`.
  - actionlint v1.7.12 (2026-03-30).

### ATTEST: artifact attestations and immutable releases
- **Source:** https://github.com/actions/attest; https://cli.github.com/manual/gh_attestation_verify; changelog 2025-10-28 (immutable releases GA). Checked 2026-11-16. Status: Verified.
- **`actions/attest`:** **v4.2.2**, commit `1e69f48acb82d1966a394da916b4c1698aa569d6`. It is canonical and covers provenance, SBOM and custom modes.
  - `attest-build-provenance` v4 is "simply a wrapper". `attest-sbom` is "being deprecated".
  - Permissions: `id-token: write`, `attestations: write`, `artifact-metadata: write`.
  - Available in public repos on all plans, via public-good Sigstore.
- **Verify:** `gh attestation verify <file|oci://…> --repo OWNER/REPO [--signer-workflow …]`.
- **Immutable releases:** lock the tag and assets, the tag name can never be reused, and a release attestation is generated automatically. Flow: draft, attach, publish.

### COSIGN
- **Version:** cosign **v3.1.4** (2026-10-20); 2.x is still maintained (v2.6.5). `sigstore/cosign-installer` **v4.1.3**, commit `756fe2d284c7292c4e9b62c8a28f1e5629c59532`, defaults to v3.1.4.
- **Source:** https://github.com/sigstore/cosign; https://docs.sigstore.dev/cosign/. Checked 2026-11-16. Status: Partial (the exact Fulcio SAN for a GitHub workflow identity, and GHCR support for OCI 1.1 referrers, are **UNVERIFIED**).
- **Bundles:** v3 defaults to protobuf bundles, and "`--bundle` has moved from optional to required". Image signatures are stored as OCI 1.1 referring artifacts.
- **Blob signing:** `cosign sign-blob f --bundle f.sigstore.json`. Verify with `cosign verify-blob f --bundle … --certificate-identity … --certificate-oidc-issuer https://token.actions.githubusercontent.com`.

### SBOM
- **Source:** https://github.com/CycloneDX/cyclonedx-rust-cargo; https://github.com/rust-secure-code/cargo-auditable; https://docs.docker.com/build/metadata/attestations/. Checked 2026-11-16. Status: Verified.
- **cargo-cyclonedx 0.5.10:** supports spec 1.3–1.7, but the **default is 1.3**, so pass `--spec-version 1.6` explicitly. Output defaults to XML; use `-f json`.
- **cargo-auditable 0.7.7:** embeds the dependency tree in binaries. Grype ≥0.83, Trivy, syft and cargo-audit can read it.
- **syft:** v1.57.2.
- **Docker buildx:** `--sbom=true` (SPDX via BuildKit's Syft) and `--provenance=mode=max`. Max mode "exposes the values of build arguments".

### BASE-IMAGE
- **Source:** https://github.com/GoogleContainerTools/distroless (README 2026-09-24); https://images.chainguard.dev/directory/image/static/overview; https://github.com/opencontainers/image-spec/blob/main/annotations.md (image-spec v1.1.1). Checked 2026-11-16. Status: Verified.
- **distroless:**
  - Now based on **Debian 13**, as `gcr.io/distroless/static-debian13` with tags `latest`, `nonroot`, `debug`, `debug-nonroot`. Debian 12 builds were removed on 2026-09-10.
  - "distroless images by default do not contain a shell". `NONROOT` is uid 65532.
  - Keyless-signed. Verify with `--certificate-oidc-issuer https://accounts.google.com --certificate-identity keyless@distroless.iam.gserviceaccount.com`.
  - Point-in-time index digest of `static-debian13:nonroot`: `sha256:aafe993665bf6200db38a02c11a9351e782a84b6a45e7f33b6695638c88e3931`. Re-resolve at implementation time.
- **Chainguard:** the free `cgr.dev/chainguard/static` has only the `latest`/`latest-dev` tags. Version tags are paid.
- **OCI annotations:** `org.opencontainers.image.{created,source,revision,version,licenses,title,description,url,documentation,vendor,base.digest,base.name}`.

### SCAN: container vulnerability scanning
- **Source:**
  - https://github.com/aquasecurity/trivy/security/advisories/GHSA-69fq-xp46-6x23
  - https://github.com/aquasecurity/trivy-action/security/advisories/GHSA-9p44-j4g5-cfx5
  - https://github.com/anchore/scan-action

  Checked 2026-11-16. Status: Verified.
- **Trivy supply-chain compromise, CVE-2026-33634 (critical):**
  - On 2026-03-19, a malicious trivy v0.69.4 was published, 76 of 77 `trivy-action` tags and all `setup-trivy` tags were force-pushed, and malicious Docker Hub images followed on 03-22.
  - The payload was an infostealer that dumped runner memory.
  - Current versions: trivy v0.76.2; trivy-action v0.36.0 (`ed142fd0673e97e23eac54620cfb913e5ce36c25`).
- **Grype:**
  - grype v0.122.0 (2026-11-16); `anchore/scan-action` v7.4.2, commit `27805bf3b4e84b4a5c980df22ed233c00390a439`.
  - Defaults: `fail-build: true`, `severity-cutoff: medium`. No compromise found.

### DENY and UNUSED-DEPS
- **Source:** https://github.com/EmbarkStudios/cargo-deny (CHANGELOG, docs); https://github.com/bnjbvr/cargo-machete; https://github.com/Boshen/cargo-shear; https://github.com/est31/cargo-udeps; https://doc.rust-lang.org/stable/cargo/reference/lints.html. Checked 2026-11-16. Status: Verified.
- **cargo-deny 0.21.0:**
  - Released through `cargo-deny-action` **v2.1.2**, commit `70f81531115945ba6eb6b941ec9d39d063438eb9`.
  - Sections: `[graph] [output] [advisories] [licenses] [bans] [sources]`.
  - 0.21 adds advisory-ignore expiry. Since 0.19.x it needs `git` on PATH.
- **Built-in Cargo lints:** stable Cargo has `unused_dependencies` and `unused_workspace_dependencies` lints, which **do not check dev-dependencies**.
- **cargo-machete:** v0.10.1, stable, "fast (yet imprecise)".
- **cargo-shear:** v1.14.1, stable static analysis.
- **cargo-udeps:** v0.1.61, needs nightly.

### SECRETS and VULN-REPORTING
- **Source:** https://docs.github.com/en/code-security/concepts/secret-security/about-secret-scanning, …/about-push-protection; https://github.com/gitleaks/gitleaks-action; https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configuring-private-vulnerability-reporting-for-a-repository. Checked 2026-11-16. Status: Verified.
- **GitHub secret scanning:** runs automatically and free on public repos. Repo-level push protection is available but "disabled by default".
- **gitleaks:** v8.30.1 (MIT). `gitleaks-action` v3.0.0 needs `GITLEAKS_LICENSE` only for organizations, and the action itself is no longer MIT.
- **trufflehog:** v3.99.9 is **AGPL-3.0**.
- **Private vulnerability reporting:** enabled by the repo owner in Settings. **Currently disabled on this repo** (`gh api …/private-vulnerability-reporting` returns `{"enabled":false}`).

### DEPENDABOT
- **Source:** https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference; the dependabot-core `cargo` fetcher. Checked 2026-11-16. Status: Verified.
- **Cargo:** supported. The fetcher resolves workspace members and `workspace.dependencies`.
- **github-actions:** updates the SHA *and* a same-line `# vX.Y.Z` comment. "Dependabot … will not create alerts for actions pinned to SHA values."
- **`cooldown`:** default 3 days. Per-semver-level days apply to Cargo only.

### LYCHEE
- **Version:** lychee v0.24.2; `lycheeverse/lychee-action` v2.9.0, commit `e7477775783ea5526144ba13e8db5eec57747ce8`.
- **Source:** https://github.com/lycheeverse/lychee-action. Checked 2026-11-16. Status: Verified.

---

## Rust toolchain and core crates

### RUST: toolchain
- **Current stable:** 1.100.0 (`0ef1bc386 2026-11-09`), channel manifest dated 2026-11-12. Source: https://static.rust-lang.org/dist/channel-rust-stable.toml, checked 2026-11-16.
- **Local machine:** the default `stable` toolchain resolves to 1.99.0, and 1.100.0 is installed alongside. CI must install toolchains explicitly and never rely on whatever happens to be installed.
- **Edition 2024:** current releases of `rmcp`, `toml`, `toml_edit`, `clap`, `keyring` and `maturin` themselves use edition 2024. That doesn't block Grimoire from using it, because editions interoperate. This was not checked against the release notes; it will be confirmed by building in Phase 1.

### CRATES: candidate dependencies
Source: the crates.io API (`https://crates.io/api/v1/crates/<name>`), checked 2026-11-16. MSRV is the crate's declared `rust-version`.

| Crate | Latest | Published | MSRV | Edition | License | Use |
|---|---|---|---|---|---|---|
| rmcp | 3.10.0 | 2026-11-12 | 1.88 | 2024 | Apache-2.0 | MCP servers |
| schemars | 1.2.3 | 2026-10-12 | 1.74 | 2021 | MIT | JSON Schema from types |
| serde | 1.0.229 | 2026-07-18 | 1.56 | 2021 | MIT OR Apache-2.0 | |
| serde_json | 1.0.154 | 2026-10-11 | 1.71 | 2021 | MIT OR Apache-2.0 | |
| toml | 1.2.1+spec-1.1.0 | 2026-11-06 | 1.85 | 2024 | MIT OR Apache-2.0 | manifests |
| toml_edit | 0.25.19+spec-1.1.0 | 2026-11-06 | 1.85 | 2024 | MIT OR Apache-2.0 | format-preserving TOML merges |
| jsonc-parser | 0.34.0 | 2026-09-27 | — | 2024 | MIT | candidate for JSONC-preserving edits |
| clap | 4.6.8 | 2026-11-16 | 1.85 | 2024 | MIT OR Apache-2.0 | CLI |
| cap-std | 4.0.3 | 2026-08-20 | — | 2021 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | root-confined filesystem access |
| tempfile | 3.27.0 | 2026-03-11 | 1.63 | 2021 | MIT OR Apache-2.0 | atomic writes |
| sha2 | 0.11.1 | 2026-10-12 | 1.88 | 2024 | MIT OR Apache-2.0 | content hashes |
| semver | 1.0.28 | 2026-04-04 | 1.68 | 2021 | MIT OR Apache-2.0 | |
| thiserror | 2.0.21 | 2026-09-23 | 1.77 | 2021 | MIT OR Apache-2.0 | |
| tokio | 1.54.1 | 2026-10-21 | 1.85 | 2021 | MIT | MCP servers only |
| keyring | 4.2.0 | 2026-08-29 | 1.88.0 | 2024 | MIT OR Apache-2.0 | OS keychain references |
| insta | 1.49.0 | 2026-10-03 | 1.66.0 | 2021 | Apache-2.0 | snapshot tests |
| proptest | 1.12.0 | 2026-10-10 | 1.88 | 2021 | MIT OR Apache-2.0 | property tests |
| assert_cmd | 2.2.2 | 2026-05-11 | 1.85 | 2024 | MIT OR Apache-2.0 | CLI tests |

Repo tooling, run in CI and not linked:

| Tool | Latest | Published | MSRV | License |
|---|---|---|---|---|
| cargo-deny | 0.21.0 | 2026-11-16 | 1.89.0 | MIT OR Apache-2.0 |
| cargo-machete | 0.10.1 | 2026-11-16 | — | MIT |
| cargo-shear | 1.14.1 | 2026-11-13 | 1.95 | MIT |
| cargo-udeps | 0.1.61 | 2026-04-29 | — | MIT OR Apache-2.0 |
| cargo-nextest | 0.9.152 | 2026-11-13 | 1.91 | Apache-2.0 OR MIT |
| cargo-dist (dist) | 0.34.0 | 2026-11-02 | 1.74 | MIT OR Apache-2.0 |
| release-plz | 0.4.3 | 2026-11-02 | — | MIT OR Apache-2.0 |
| git-cliff | 2.14.2 | 2026-09-18 | 1.88.0 | MIT OR Apache-2.0 |
| cargo-cyclonedx | 0.5.10 | 2026-11-06 | 1.86.0 | Apache-2.0 |
| cargo-auditable | 0.7.7 | 2026-10-02 | — | MIT OR Apache-2.0 |
| cargo-binstall | 1.27.0 | 2026-11-11 | 1.96.1 | **GPL-3.0-only** (a tool users run; not linked, not redistributed) |
| maturin | 1.16.0 | 2026-10-29 | 1.89 | MIT OR Apache-2.0 |

The MSRV implied by the likely dependency set is at least **1.88** (rmcp, sha2, keyring, proptest). See the open questions in `PLAN.md`.

---

## Local environment (evidence, not requirements)

Inspected 2026-11-16 on the maintainer's Linux machine:

- **Present:** rustc/cargo 1.99.0 (default) and 1.100.0, rustup 1.29.3, gh 2.45.0 (authenticated), Docker 29.9.3 with buildx 0.37.2, syft 1.54.1, node 18.19.1 / npm 9.2.0, Python 3.12.3.
- **Not installed:** cargo-deny, cargo-machete, cargo-udeps, cargo-dist, release-plz, cargo-nextest, cargo-insta, cargo-binstall, maturin, cosign, gitleaks, trufflehog, actionlint, zizmor, lychee, typos, podman.
- **Note:** local node 18 / npm 9 is too old for npm trusted publishing (see NPM-TP). This only matters if npm steps are run locally. CI pins its own versions.
