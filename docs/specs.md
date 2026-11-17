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
| YAML | Frontmatter YAML parser (serde-saphyr) | 2.1.1 | https://github.com/bourumir-wyngs/serde-saphyr | Verified (Phase 1) |
| SPDX-CRATE, MARKDOWN | spdx and pulldown-cmark behaviour | 0.13.6 / 0.13.6 | https://github.com/EmbarkStudios/spdx ; https://github.com/pulldown-cmark/pulldown-cmark | Verified (Phase 1) |
| SCHEMARS | schemars draft, map keys, key order | 1.2.3 | https://docs.rs/schemars/1.2.3 | Verified (Phase 1) |
| CARGO-LINTS | `[lints.cargo]`, workspace globs, inheritance | Cargo 1.96.0 / 1.99.0 / 1.100.0 | https://doc.rust-lang.org/cargo/reference/lints.html | Verified (Phase 1) |
| CI-PINS | Pinned actions and CI tool binaries | see table | per pin | Verified (Phase 1) |
| ZIZMOR | zizmor audits and personas | 1.32.0 | https://docs.zizmor.sh | Verified (Phase 1) |
| CC-P2 | Claude Code facts the `claude-code` adapter writes against | CLI 2.1.326; docs fetched 2026-11-16 | https://code.claude.com/docs/en/ | Verified (Phase 2), gaps listed |
| FS-CRATES | cap-std, cap-fs-ext, cap-tempfile | 4.0.3 | https://github.com/sunfishcode/cap-std | Verified (Phase 2) |
| EDIT-CRATES | jsonc-parser (CST), toml_edit, similar | 0.34.0 / 0.25.19 / 3.2.1 | https://github.com/dprint/jsonc-parser | Verified (Phase 2), with one finding |
| CC-P3 | Claude Code hooks, remote and stdio `.mcp.json`, OAuth client | CLI 2.1.326 (`latest`) / 2.1.311 (`stable`); CHANGELOG @ `ccc86e33cd`; docs fetched 2026-11-16 | https://code.claude.com/docs/en/hooks.md | Verified (Phase 3), gaps listed |
| MCP-P3 | MCP server duties: Streamable HTTP, stateless lifecycle, resource-server authorization | spec tag `2026-07-28` (`5f5440bb26`), main `05aeb07`; RFC 9728/6750/8707/9068/8725; OAuth 2.1 -13 (-17 latest) | https://modelcontextprotocol.io/specification/2026-07-28 | Verified (Phase 3) |
| RMCP-P3 | rmcp server API, defaults and divergences | rmcp 3.10.0, tag `rmcp-v3.10.0` (`53e64a93c8`) | https://github.com/modelcontextprotocol/rust-sdk | Verified (Phase 3), one claim corrected |
| AUTH-CRATES | Server-side JWT validation crates under `deny.toml` | jsonwebtoken 12.0.0, rsa 0.10.0, p256 0.14.0, hyper 1.12.0, hyper-util 0.1.21; advisory-db `02e2517f45`; cargo-deny 0.21.0 / 0.20.2 | https://github.com/Keats/jsonwebtoken | Verified (Phase 3), two claims corrected |
| SECRET-FORMATS | Vendor credential formats for secret-guard | vendor docs fetched 2026-11-16; gitleaks v8.30.1; GitLab rules @ `c8396d46d7` | https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-authentication-to-github#githubs-token-formats | Partial (many lengths third-party; OpenAI UNVERIFIED) |
| CONTENT-P3 | Conventional Commits, Agent Skills, ADR formats | CC 1.0.0 @ `7d293dc59e`; Agent Skills @ `83021e2be4`; MADR 4.0.0 (`2475fe1973`) | https://www.conventionalcommits.org/en/v1.0.0/ | Verified (Phase 3) |

**Not yet researched, to verify in the phase that needs them:**

| Topic | Phase |
|---|---|
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
- **Governance:** "Releases of the specification will be tagged and will carry a changelog". None exist yet. The project is an LF Projects series under the Agentic AI Foundation.
- **Licence** (re-checked in Phase 3, CONTENT-P3): code and specifications are Apache-2.0 (`GOVERNANCE.md`, added in `83021e2be4`). Other documentation, including the skill-creation guides, is CC BY 4.0 (`GOVERNANCE.md`, `README.md`). `docs/specification.mdx` itself sits under `docs/LICENSE`, which is CC BY 4.0, so the spec file's licence is ambiguous. Paraphrase it; don't copy it.
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
  - `structuredContent` must conform to `outputSchema`.
  - Tool names **SHOULD** (not MUST) be 1–128 chars from `[A-Za-z0-9_.-]`, case-sensitive and unique within the server (`server/tools.mdx`; corrected in Phase 3, see MCP-P3).
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
- **`cooldown`:** default 3 days. Per-semver-level days apply to Cargo, `rust-toolchain` and others, but not to GitHub Actions or Docker, which take `default-days` only. (This corrects the Phase 0 entry, which said Cargo only.)
- **`rust-toolchain` ecosystem:** Dependabot bumps `rust-toolchain.toml`, but it never changes `rust-version`.

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
| serde-saphyr | 2.1.1 | 2026-11-05 | 1.89 | 2024 | MIT OR Apache-2.0 | strict YAML for SKILL.md frontmatter (see YAML) |
| pulldown-cmark | 0.13.6 | 2026-10-30 | 1.71.1 | 2021 | MIT | Markdown reference checks; ≥ 0.13.6 for RUSTSEC-2026-0414/-0433 |
| spdx | 0.13.6 | 2026-10-02 | 1.85.0 | 2024 | Apache-2.0 | SPDX expression validation |
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
| rmcp-macros | 3.10.0 | 2026-11-12 | 1.88 | 2024 | Apache-2.0 | `#[tool]` macros; pulls syn 3 and darling 0.24 (RMCP-P3) |
| jsonwebtoken | 12.0.0 | 2026-11-13 | 1.88.0 | 2024 | MIT | JWT validation with a custom provider (AUTH-CRATES) |
| rsa | 0.10.0 | 2026-11-08 | 1.87 | ? | MIT OR Apache-2.0 | RS256 verification (`std`, `pkcs1v15`) |
| p256 | 0.14.0 | 2026-07-03 | 1.85 | ? | Apache-2.0 OR MIT | ES256 verification (`ecdsa`, `std`) |
| hyper | 1.12.0 | 2026-10-06 | 1.63 | ? | MIT | HTTP/1 server for MCP (`server`, `http1`) |
| hyper-util | 0.1.21 | 2026-09-24 | 1.85 | ? | MIT | `TokioIo`, `TowerToHyperService` (`tokio`, `service`) |
| http | 1.5.0 | 2026-07-29 | 1.57.0 | ? | MIT OR Apache-2.0 | HTTP types |
| http-body-util | 0.1 (requirement) | ? | ? | ? | ? | rmcp HTTP server and socket-free tests |
| bytes | 1 (requirement) | ? | ? | ? | ? | rmcp HTTP server and socket-free tests |
| base64 | 0.23 (jsonwebtoken's requirement; 0.23.1 resolved) | ? | ? | ? | ? | transitive |
| regex | 1.13.1 | 2026-07-15 | 1.65 | 2021 | MIT OR Apache-2.0 | secret-guard matching (SECRET-FORMATS) |

Rows added in Phase 3 come from the same API and from the Phase 3 research. `?` means the research didn't record the value (UNVERIFIED).

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

---

## Phase 1 additions (checked 2026-11-16)

These facts came from a ten-topic research workflow in which each topic was then re-checked by an independent skeptic. The raw results are not committed. Each fact below is something Phase 1 code or CI now depends on.

### YAML: frontmatter parser choice
- **Choice:** **serde-saphyr 2.1.1** (`default-features = false, features = ["deserialize"]`).
- **Source:** https://crates.io/api/v1/crates/serde-saphyr; https://github.com/bourumir-wyngs/serde-saphyr (tag 2.1.1). Status: Verified.
- **Why:**
  - It is pure safe Rust (`#![forbid(unsafe_code)]` in it and in its parser, granit-parser) with no RustSec advisories.
  - It is the only serde YAML crate that rejects **duplicate keys at every nesting level** by default.
  - With `no_schema`, `strict_booleans`, `merge_keys = Error`, `reject_unsupported_tags` and `reject_tagged_field_names`, an unquoted `123`, `true` or `~` is not accepted as a string.
  - Its default resource budget stops alias bombs.
- **One gap:** `Option<String>` fields read an explicit null as `None`, so Grimoire uses a non-null deserializer for them.
- **Rejected candidates:**
  - `serde_yaml`: deprecated; its dependency unsafe-libyaml is RUSTSEC-2024-0449.
  - `serde_yml`: RUSTSEC-2025-0068, and it pulls in noyalib 0.0.5, affected by RUSTSEC-2026-0333.
  - `serde_yaml_ng`: also depends on unsafe-libyaml.
  - `serde_norway`: c2rust unsafe code, no commits since 2024-12, duplicate keys inside maps silently resolved last-wins.
  - `saphyr` and `yaml-rust2`: no serde support.

### SPDX-CRATE and MARKDOWN
- **spdx 0.13.6:** `Expression::parse` is `ParseMode::STRICT`. It rejects deprecated (`GPL-2.0`), unknown and lowercase identifiers, and `/` used as OR. STRICT still accepts lowercase operators and irregular whitespace, so Grimoire also requires `Expression::canonicalize(s) == Ok(None)`. Status: Verified.
- **pulldown-cmark 0.13.6:**
  - It never percent-decodes link destinations, so Grimoire decodes them itself.
  - Both 2026 advisories (RUSTSEC-2026-0414, -0433) affect only wikilinks, which stay disabled.
  - Default features (getopts, html) are off.

  Status: Verified.

### SCHEMARS: schema generation
- **Draft:** schemars 1.2.3 emits draft 2020-12 by default, but its source calls that default "liable to change". Grimoire therefore pins it with `SchemaSettings::draft2020_12()`.
- **Map keys:** only a key's `pattern` reaches the map schema, so Grimoire adds `propertyNames` to every keyed map.
- **Key order:** Cargo feature unification can flip key order when some crate enables `serde_json/preserve_order`. Grimoire enables `preserve_order` explicitly, so schema key order follows field order.
- Status: Verified.

### CARGO-LINTS: Cargo behaviour
- **`[lints.cargo]` is stable and enforced from Cargo 1.100**, the pinned toolchain. It covers `unused_dependencies`, `unused_workspace_dependencies`, `unused_workspace_package_fields`, `unknown_lints` and `missing_lints_inheritance`.
  - Cargo 1.96 and 1.99 ignore the table, with an "unused manifest key" warning.
  - `unused_dependencies` never checks dev-dependencies, so cargo-shear covers those.
- **Workspace globs:** a `members` glob that matches nothing is an error on Cargo 1.96–1.100. The `catalog/*/*` globs are therefore added with the first code-bearing asset in Phase 3.
- **Other confirmed behaviour:**
  - Edition 2024 needs Rust ≥ 1.85, and resolver 3 needs ≥ 1.84.
  - `publish.workspace = true` inheritance works.
  - The pedantic group needs `priority = -1`.
- Status: Verified, using local experiments on 1.96.0, 1.99.0 and 1.100.0.

### CI-PINS: GitHub Actions and CI tools

Every pinned action or tool was a release **at least 7 days old when adopted**, matching the Dependabot cooldown. Each tag was resolved to its commit with `gh api` and re-checked by a skeptic.

| Pin | Version | Commit / SHA-256 | Released |
|---|---|---|---|
| actions/checkout | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` | 2026-07-20 |
| taiki-e/install-action | v2.90.3 | `cfdb7ed4ccc7c7dce37759eae89dc5674b469b39` | 2026-11-08 |
| zizmorcore/zizmor-action (zizmor 1.32.0) | v0.8.1 | `45d7276c9ca790aabd705e31ca99c6185c3e76dd` | 2026-11-03 |
| cargo-deny (built with `cargo install --locked`) | 0.20.2 | crates.io checksums | 2026-07-09 |
| cargo-shear (via install-action, `fallback: none`) | 1.14.0 | install-action manifest checksum | 2026-09-22 |
| actionlint, linux_amd64 tarball | 1.7.12 | `8aca8db96f1b94770f1b0d72b6dddcb1ebb8123cb3712530b08cc387b349a3d8` | 2026-03-30 |
| gitleaks, linux_x64 tarball | 8.30.1 | `551f6fc83ea457d62a0d98237cbad105af8d557003051f41f3e7ca7b3f2470eb` | 2026-03-21 |
| lychee, x86_64-unknown-linux-gnu tarball | 0.24.2 | `1f4e0ef7f6554a6ed33dd7ac144fb2e1bbed98598e7af973042fc5cd43951c9a` | 2026-05-01 |
| gh-actions-lock, linux-amd64 binary | v0.4.1 | `39deb2fe714dbf74b073d8245fc7f3d50cd0eee18d5c0900e9e849ae933ae17e` | 2026-11-02 |

The actionlint, gitleaks and lychee hashes were checked against each release's own checksum file. The gh-actions-lock hash is the asset digest from the release API.

**Maintenance:** Dependabot updates only the `uses:` action pins (and `actions.lock`). It cannot see the tool versions and checksums in workflow `env:` blocks, the cargo-deny version, or the cargo-shear version passed to install-action. Bump those by hand, keeping the 7-day rule, and re-verify each checksum against its release.

- **Rust toolchains:** installed with plain `rustup` steps rather than a third-party action, which zizmor's pedantic persona flags as superfluous.
- **No caching** anywhere in CI, so there is no cache-poisoning surface.
- **cargo-deny-action not used:** it downloads cargo-deny with `curl | tar` and verifies no checksum.
- **gh-actions-lock:** every write-mode run needs `--no-narrow`, or it rewrites SHA pins to tags. CI runs `--verify`, which reports the expected, non-blocking "pinned to a bare SHA" note.
- Status: Verified.

### ZIZMOR: workflow audit rules
- **Persona:** zizmor 1.32.0 runs at the **pedantic** persona with online audits, and `advanced-security: false` keeps failing exit codes.
- **Rules every workflow follows:**
  - named workflows and jobs;
  - `permissions: {}` at the top, with minimal per-job grants;
  - SHA pins with a `# vX.Y.Z` comment;
  - `persist-credentials: false`;
  - no `${{ }}` in `run:`;
  - a workflow-level `concurrency` mapping;
  - no caches in publishing workflows;
  - a Dependabot cooldown of at least 7 days.
- **Known gap:** zizmor 1.32.0's `dependabot-cooldown` audit stops after the first compliant entry, so cooldowns on every entry are enforced by review.
- Status: Verified.

---

## Phase 2 additions (checked 2026-11-16)

These facts came from a six-topic research workflow (Claude Code rules and imports, subagents, hooks and settings, `.mcp.json`, filesystem crates, editing crates). An independent skeptic re-fetched every source. Of 370 checks, 369 were confirmed. One overstated claim about jsonc-parser's bug history was corrected, and the conclusion stands. The raw results are not committed.

### CC-P2: what the `claude-code` adapter relies on
Sources: https://code.claude.com/docs/en/memory.md, …/sub-agents.md, …/tools-reference.md, …/errors.md, …/hooks.md, …/settings.md, …/permissions.md, …/mcp.md, …/mcp-quickstart.md, …/headless.md. CHANGELOG at `anthropics/claude-code@1ad046cb01`. CLI `latest` is 2.1.326 and `stable` is 2.1.311 (npm). Status: Verified, except the gaps listed.

- **Rules (`.claude/rules/*.md`):**
  - `paths` is "the only field Claude Code reads from a rule". It takes a YAML list or a comma-separated string, relative to the directory that contains `.claude`.
  - The opening `---` must be the file's first line. A rule without `paths` loads at launch. Invalid YAML makes the rule load unscoped.
  - Grimoire writes a YAML flow list of JSON-quoted strings.
- **`CLAUDE.md` imports:**
  - The syntax is `@path`, relative to the importing file. Imports are skipped in code spans and fenced code, and the maximum depth is four hops.
  - Importing `AGENTS.md` from `CLAUDE.md` is documented explicitly: "Keeping the import never makes Claude read `AGENTS.md` twice, whichever **Project instructions** value you use."
  - Block-level HTML comments are stripped before injection.
  - By default `AGENTS.md` is read only when no `CLAUDE.md`, `.claude/CLAUDE.md` or `CLAUDE.local.md` exists in the working directory or above (v2.1.277+).
- **Subagents (`.claude/agents/*.md`):**
  - `name` and `description` are required. `description` says "when Claude should delegate to this subagent".
  - `tools` is a comma string or a YAML list, and an omitted list "inherits every tool".
  - **No tools at all:** "If you leave the `tools` list empty, or `disallowedTools` removes every entry in it, Claude Code also skips the refusal and launches the subagent without tools" (errors.md). The YAML spelling of an empty list is not documented, so Grimoire writes `tools: ["Read"]` and `disallowedTools: ["Read"]`.
  - **Tool names:** `Read`, `Grep`, `Glob`, `Edit`, `Write`, `NotebookEdit`, `Bash`, `PowerShell`, `WebFetch`, `WebSearch`. MCP tools are `mcp__<server>__<tool>`, and the only documented wildcard in `tools` is `mcp__<server>__*`.
  - Glob and Grep are absent by default on macOS, Linux and WSL, but come back for a subagent that lists them without Bash. PowerShell is the shell tool on Windows without Git Bash.
- **Hooks (`.claude/settings.json`):**
  - **Shape:** `{"hooks":{"<Event>":[{"matcher","hooks":[{handler}]}]}}`.
  - **Events:** `PreToolUse`, `PostToolUse`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `Stop`.
  - **Matchers:** letters, digits, `_`, `-`, space, `,` and `|` form an exact list. Anything else is an *unanchored* JavaScript regex, so Grimoire anchors its regexes with `^(…)$`. `UserPromptSubmit` and `Stop` ignore matchers.
  - **Exec form:** with `args`, "`command` is resolved as an executable and spawned directly with `args` as the argument vector, with no shell involved". `args` was added in 2.1.139 (CHANGELOG). `timeout` is in seconds.
  - **Strict JSON:** "Settings files are strict JSON: a `//` comment or a trailing comma is a syntax error."
  - **No trust prompt:** `-p` and SDK sessions run project hooks and connect `.mcp.json` servers without a trust prompt.
- **`.mcp.json`:**
  - The shape is `{"mcpServers":{"<name>":{…}}}`. An entry with no `type` is stdio, but Grimoire writes `"type": "stdio"` explicitly.
  - `${VAR}` and `${VAR:-default}` expand in `command`, `args`, `env`, `url` and `headers`. An unset variable without a default is passed through *literally*, so optional variables get `:-`.
  - Server names are letters, digits, `-` and `_`. The reserved built-in names are `workspace`, `claude-in-chrome`, `computer-use`, `Claude Preview` and `Claude Browser`.
- **Not documented (UNVERIFIED):**
  - Whether an `@import` between two HTML-comment lines is honoured. Grimoire's import line sits on its own line between two block-level marker comments.
  - The exact YAML spelling of an empty `tools` list (worked around above).
  - Whether `.mcp.json` accepts comments. Grimoire treats it as strict JSON.
  - What versions before 2.1.139 do with `args`.
  - How Claude Code exposes an MCP tool whose name contains characters outside `[A-Za-z0-9_-]`. Model-facing tool names allow only those, and the docs state only that `.` and spaces in *server* names become `_`. The adapter therefore refuses MCP capabilities whose tool name contains `.`, and full names over 128 characters, which Claude Code drops.
  - How GitHub Copilot in VS Code, which also reads `.claude/agents/` (`COPILOT-AGENTS`), maps Claude Code tool names and whether it honours `disallowedTools`. The `plan/cross-client-agent` warning says so.

### FS-CRATES: cap-std 4.0.3
Sources: the crates.io API, `sunfishcode/cap-std` at tag `v4.0.3` (commit `b7acf8e8807f`), docs.rs 4.0.3, RustSec advisory-db at `02e2517f457a`, and the Rust 1.100.0 `std::fs::rename` docs and source. Status: Verified.

- **Releases and licenses:**
  - 4.0.3 was published 2026-08-20. No `rust-version` is declared; the workspace MSRV is 1.70.
  - The license is `Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT`. `winx` 0.36.4 (Windows) is `Apache-2.0 WITH LLVM-exception` only.
- **Advisories:**
  - RUSTSEC-2026-0482 (critical: a trailing-slash `O_NOFOLLOW` bypass that escapes the sandbox) affects `<= 4.0.2` and is fixed in 4.0.3.
  - RUSTSEC-2026-0486 (rustix, unsound) is fixed in rustix 1.1.6.
- **Confinement:**
  - Escapes fail with `PermissionDenied` ("a path led outside of the filesystem").
  - On Linux, opens use `openat2(RESOLVE_BENEATH | RESOLVE_NO_MAGICLINKS)`, with a manual resolver as fallback.
  - `FollowSymlinks::No` controls only the final component. `DirExt::open_dir_nofollow` refuses a symlinked directory.
- **`cap-tempfile`:**
  - `TempFile::replace` renames within the same `Dir` and **does not fsync**, so Grimoire syncs first.
  - Named temporary files use mode `0o666` minus umask, with bare UUID names. On Linux it uses `O_TMPFILE` plus `linkat` through `/proc/self/fd`.
  - On Windows, `rename` and `remove_file` are path-based after the parents are resolved in the sandbox.
- **The deny.toml entries this needs:** ADR-0006.

### EDIT-CRATES: jsonc-parser, toml_edit, similar
Sources: the crates.io API, `dprint/jsonc-parser` at tag `0.34.0` (commit `c7d4cf51f772`), `toml-rs/toml` at `toml-v1.2.1`, and `mitsuhiko/similar` at `3.2.1`. Status: Verified.

- **jsonc-parser 0.34.0** (MIT, 2026-09-27):
  - The `cst` feature has no dependencies and provides `CstRootNode::parse`, object `get`/`append`/`set_value`/`remove`, and array `elements`/`append`/`remove`. Untouched content keeps its comments, whitespace and key order.
  - Strict mode needs all ten `ParseOptions` fields set to `false`. The struct is not `#[non_exhaustive]`, so Grimoire lists every field and a new one breaks the build.
  - Strict mode is reliable only from 0.33.1 (issue #84). There is a recursion limit against stack exhaustion from 0.30.0 (#64).
  - **Finding (checked in source, `src/parse_to_ast.rs`):** `CstRootNode::parse` collects comments `AsTokens`, and that branch skips the `allow_comments` check. A CST parse therefore accepts `//` comments even in strict mode. Grimoire validates every JSON file with `serde_json` (RFC 8259) before building the CST, and a regression test covers it.
  - Inserting into a single-line object makes it multi-line, and inserts trim blank lines just inside the edited brackets.
- **toml_edit 0.25.19:** released from the same tag as `toml` 1.2.1, with identical shared dependency requirements, so there are no duplicate versions. `DocumentMut` preserves comments and formatting.
- **similar 3.2.1:** Apache-2.0, no required dependencies, `TextDiff::from_lines(..).unified_diff()`.
- **sha2 0.11.1:** MIT OR Apache-2.0, MSRV 1.88.

---

## Phase 3 additions (checked 2026-11-16)

These facts came from a five-topic research workflow (Claude Code, MCP server duties and authorization, the rmcp server API, auth dependencies, and content sources). An independent skeptic re-checked each topic against its primary sources and re-ran most experiments; the exceptions are noted in each entry. Confirmed: Claude Code 83 of 84 (1 uncertain), MCP 98 of 98, rmcp 56 of 57 (1 refuted), auth dependencies 37 of 39 (2 refuted), content sources 75 of 76 (1 refuted). Refuted and uncertain facts appear below in corrected form, and facts the skeptics added are included where an implementer needs them. The raw results are not committed.

### CC-P3: Claude Code facts for Phase 3
Sources: https://code.claude.com/docs/en/hooks.md, …/hooks-guide.md, …/mcp.md, …/errors.md, …/permissions.md, …/sub-agents.md, …/skills.md, …/memory.md, …/tools-reference.md, …/code-mode.md, …/agent-sdk/typescript.md (tool input types). CHANGELOG at https://github.com/anthropics/claude-code/blob/ccc86e33cd1b3f60f8dab78eaed58070ea8fcac9/CHANGELOG.md (content-identical to `1ad046cb01`). https://registry.npmjs.org/-/package/@anthropic-ai/claude-code/dist-tags. https://json.schemastore.org/claude-code-settings.json (src `bf7c6745505c`). Status: Verified, except the gaps listed. Subagents, skills and rules list only changes since CC-P2.

- **Versions:**
  - npm `latest`/`next` = 2.1.326 (2026-11-16); `stable` = 2.1.311 (2026-10-26).
  - Gates (CHANGELOG): hook `args` 2.1.139; `onFailure` 2.1.295; Monitor commands reaching Bash-matcher hooks 2.1.314, so not on `stable`; a stdout `{…}` that isn't valid JSON stopped being read as plain text in 2.1.274. Gates from hooks.md: `mcp_server` in tool input 2.1.274, `scratchpad_dir` 2.1.257.
  - **The v1 MCP client is removed on `latest`** (CHANGELOG 2.1.323): `MCP_SDK_GENERATION=v1` now runs the v2 client "in a compatibility mode (earlier handshake, relaxed OAuth sign-in checks)". mcp.md still describes two runtimes (v1 on MCP TypeScript SDK 1.x; v2 on SDK 2.0, negotiating 2026-07-28), which is what `stable` 2.1.311 has.
  - Stdio connections negotiate 2026-07-28 by default on every install since 2.1.292. Since 2.1.302 the client tolerates 2026-07-28 servers that omit `resultType`, `ttlMs` or `cacheScope`.
- **Hook stdin:**
  - Common fields: `session_id`, `prompt_id` (absent before the first user input), `transcript_path`, `cwd`, `scratchpad_dir`, `permission_mode` (not on every event), `effort{level}`, `hook_event_name`. Inside a subagent or with `--agent`, also `agent_id` and `agent_type`. Settings, managed and plugin hooks also fire for subagents' tool calls.
  - PreToolUse adds `tool_name`, `tool_input` and `tool_use_id`, plus `mcp_server{name,source}` for MCP tools.
  - UserPromptSubmit adds `prompt` and, when set, `session_title`. Pasted text arrives expanded, possibly in `<pasted_content id=…>` lines. The event also fires on turns Claude Code starts itself: scheduled tasks and `/loop`, background-subagent reports, cross-session messages, and Code Mode output (with `source: "poll_event"`, documented only in code-mode.md).
- **`tool_input` shapes:**

  | Tool | Fields | Source |
  |---|---|---|
  | Write | `file_path`, `content` | hooks.md |
  | Edit | `file_path`, `old_string`, `new_string`, `replace_all` | hooks.md |
  | Bash, PowerShell | `command`, `description?`, `timeout?` (ms), `run_in_background` | hooks.md; the SDK type adds `dangerouslyDisableSandbox?` |
  | NotebookEdit | `notebook_path`, `cell_id?`, `new_source`, `cell_type?`, `edit_mode?` | SDK types only |
  | WebFetch | `url`, `prompt`, `offset?` (2.1.290+) | hooks.md |
  | Monitor | `description`, `timeout_ms`, `command?`, `ws?{url}` | SDK types only |

  - `file_path` for Write, Edit and Read is always absolute, with backslashes on Windows. MultiEdit is legacy and not a current tool. On Windows without Git Bash the Bash tool isn't registered, so shell hooks match `Bash|PowerShell`.
- **Exit codes and stdout:**
  - **0:** success, and JSON fields apply. Plain stdout becomes Claude context on UserPromptSubmit, UserPromptExpansion, SessionStart and PostModelSwitch. Claude never sees stderr from an exit-0 hook.
  - **2:** a blocking error. The reason is stderr unless JSON makes its own blocking decision, and JSON that fails schema validation still blocks. On PreToolUse, Claude gets stderr as the tool error, prefixed with the event, tool and hook command (`PreToolUse:Bash hook error: [cmd]: msg`). On UserPromptSubmit the prompt never reaches Claude, and stderr is shown to the user but not added to context (except when blocking Code Mode output).
  - **Anything else** (another code, a failure to start, invalid JSON) is a non-blocking error, and the action goes ahead. Valid hook JSON with any other code counts as success: "the fields alone decide".
  - JSON is parsed for every exit code, and stdout must hold only the JSON object. Fields at the wrong level are ignored silently. Exit 2 keeps a PreToolUse call blocked even if the JSON says allow.
  - PreToolUse JSON: `hookSpecificOutput{hookEventName, permissionDecision (allow|deny|ask|defer), permissionDecisionReason, updatedInput, additionalContext}`. Precedence is deny > defer > ask > allow, and the top-level `decision`/`reason` are deprecated there. `defer` works only in `-p` with a single tool call.
  - UserPromptSubmit JSON: top-level `decision: "block"` and `reason`; `hookSpecificOutput{additionalContext, sessionTitle, suppressOriginalPrompt}`. A top-level `additionalContext` is ignored.
  - Universal fields: `continue` (false ends the turn, but doesn't block a PreToolUse call by itself), `stopReason`, `systemMessage`, `terminalSequence`. `suppressOutput` is documented as "Has no effect". `additionalContext`, `systemMessage` and plain stdout are each capped at 10,000 chars.
  - All matching hooks run in parallel to completion, and the most restrictive PreToolUse decision wins.
- **`suppressOriginalPrompt`:** a block message normally ends with `Original prompt:` and the text. `suppressOriginalPrompt: true` drops it, with `decision: "block"` or exit 2, but the prompt still lands in the transcript and prompt history: "a blocking hook isn't a way to keep a secret off disk".
- **`onFailure` and timeouts:**
  - `onFailure: "block"` (command and http hooks, 2.1.295+) makes a failure act like exit 2 on that event. Failures are: can't start, an exit other than 0 or 2 (even with allow JSON), a timeout, and JSON that can't be parsed or fails schema validation. It has no effect on Stop, SubagentStop, TaskCompleted, TeammateIdle or async hooks.
  - Without it, a missing binary or a timed-out PreToolUse or UserPromptSubmit hook lets the action through.
  - `timeout` is in seconds. Defaults: 600 for command, http and mcp_tool; 30 for prompt; 60 for agent; 30 for command, http and mcp_tool on UserPromptSubmit, PreModelSwitch and PostModelSwitch; 10 on MessageDisplay. SessionEnd hooks share 1.5 s. No minimum or maximum is documented.
  - Code Mode (experimental, off by default) refuses a call when a hook exits 126, 127 or above 128, or times out, even without `onFailure`.
- **Exec form:** with `args`, `command` is resolved on PATH and spawned directly, with no shell on any platform. Only `${CLAUDE_PROJECT_DIR}`, `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` and plugin `${user_config.*}` are substituted; `$`, backticks and quotes pass through verbatim. `shell` is ignored when `args` is set. On Windows `command` must resolve to a real executable such as an `.exe`, because `.cmd` and `.bat` shims can't be spawned.
- **Matchers** (general rule in CC-P2):
  - FileChanged and StopFailure use a narrower exact set: letters, digits, `_` and `|`. Matchers are case-sensitive (hooks-guide.md).
  - These events have no matcher support, and a matcher on them is ignored silently: UserPromptSubmit, PostToolBatch, Stop, TeammateIdle, TaskCreated, TaskCompleted, WorktreeCreate, WorktreeRemove, MessageDisplay, CwdChanged.
  - `if` holds one permission rule and is evaluated only on PreToolUse, PostToolUse, PostToolUseFailure, PermissionRequest and PermissionDenied. On other events a hook with `if` never runs.
  - MCP tools: `mcp__<server>__.*` (the `.*` is required). Plugin servers are `mcp__plugin_<plugin>_<server>__<tool>`.
- **Monitor:** from 2.1.314, a PreToolUse or PermissionRequest hook with a `Bash` matcher also runs on Monitor commands and receives a Bash call. `allow` and `updatedInput` have no effect there, but deny works. A matcher that names `Monitor` (or `.*`, or none) gets the Monitor call's own input, which also covers `stable` 2.1.311.
- **Environment and cwd:**
  - Both forms export `CLAUDE_PROJECT_DIR`, `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA`. The hook inherits the parent environment minus `OTEL_*` exporter variables, and minus scrubbed variables when `CLAUDE_CODE_SUBPROCESS_ENV_SCRUB=1`. Also present: `CLAUDE_CODE_REMOTE`, `CLAUDE_EFFORT`, and `CLAUDE_ENV_FILE` (only on SessionStart, Setup, CwdChanged and FileChanged). There is no `CLAUDE_MODEL`.
  - Handlers run in the current directory, falling back to the session start directory, the project root, home, then temp. `${CLAUDE_PROJECT_DIR}` stays at the project root, while the input `cwd` follows `cd` and worktrees.
  - On macOS and Linux, command hooks run with no controlling terminal.
- **Ways hooks are bypassed or turned off:**
  - Interactive sessions hold settings-file hooks until workspace trust; `-p` and SDK sessions run them without it.
  - `disableAllHooks` (an effective-precedence value, so a project can set it; `--settings '{"disableAllHooks": true}'` overrides project and local settings), the managed `allowManagedHooksOnly`, and `--bare`.
  - "a mod can approve a call your `PreToolUse` hook blocked" when there are no managed settings, no managed-mcp.json and no Team/Enterprise `/login` (hooks-guide.md).
  - Files referenced with `@` are added without a tool call, so no PreToolUse hook fires, and UserPromptSubmit sees the prompt as typed.
  - A plugin's UserPromptSubmit hook that exits 2 with empty stdout and stderr saying a file is missing (`No such file or directory`) is treated as a non-blocking error.
  - Otherwise a hook's deny or exit 2 blocks in every permission mode, including bypassPermissions; exit 2 stops the call before permission rules are evaluated, and a hook's allow can't override deny or ask rules. Command hooks run with full user permissions outside the permission rules, so they need no allow rule.
- **Remote `.mcp.json` entries:**
  - `{"type": "http", "url": …, "headers": {…}}`, with `streamable-http` as an alias. A `url` entry without `type` is a config error and the server is skipped (as is `type: "sdk"`), so always write `"type": "http"`. Other fields: `headersHelper` (prints a JSON object, 10 s limit), `timeout` (ms per call; under 1000 is ignored), `alwaysLoad`, and `oauth{clientId, callbackPort, authServerMetadataUrl (https only), scopes (space-separated)}`. The client secret never goes in config.
  - In a remote `url` and `headers`, a fixed set of credential variables reads as empty even when set, and `:-default` on them is ignored. Examples: `ANTHROPIC_API_KEY`, `ANTHROPIC_AUTH_TOKEN`, `AWS_BEARER_TOKEN_BEDROCK`, `HTTPS_PROXY`, `NPM_TOKEN`. An empty string counts as set.
  - A project `headersHelper` runs only after trust, and helpers from repo or plugin config run without credential-like environment variables.
- **Claude Code as an OAuth client** (http and sse servers):
  - A 401 or 403 marks the server as needing auth. Discovery checks RFC 9728 PRM at `/.well-known/oauth-protected-resource` first, then RFC 8414, and follows `WWW-Authenticate`. DCR and CIMD are supported.
  - A root PRM document can't claim an endpoint at `/mcp` (RFC 9728 §3.3, MCP-P3), so send `resource_metadata` in every 401 and 403 and serve the path-inserted document. Whether Claude Code's root-first probe also needs a root document is UNVERIFIED: the Claude Code skeptic says serve both, and the MCP skeptic says a root document is wrong for `/mcp`.
  - Scopes: without `oauth.scopes`, it doesn't request the AS's whole `scopes_supported`. With neither `oauth.scopes` nor `authServerMetadataUrl`, it "requests the scope the server named". It appends `offline_access` when the AS advertises it. A 403 `insufficient_scope` during a call prompts re-authentication (2.1.288).
  - **A configured `Authorization` header, static or from `headersHelper`, disables the OAuth fallback.**
  - The v2 client fails a sign-in whose `iss` doesn't match the metadata issuer (RFC 9207). A redirect without `iss` passes unless the metadata sets `authorization_response_iss_parameter_supported`. Token requests go only to HTTPS or loopback (`localhost`, `127.0.0.1`, `::1`) token endpoints.
  - OAuth can't run in `-p` or SDK runs. Users sign in first with `/mcp` or `claude mcp login <name>` (`--no-browser` is supported).
- **Stdio servers:**
  - **Claude Code sets `CLAUDE_PROJECT_DIR` in the server's environment, not in its own**, so `${CLAUDE_PROJECT_DIR:-.}` in a project `.mcp.json` normally expands to `.`. Read the variable inside the server instead.
  - Claude Code answers `roots/list` with the launch directory plus any added directories.
  - Stopping: on macOS and Linux, SIGINT, then SIGTERM 100 ms later, then SIGKILL after a further 400 ms. On Windows, it closes stdin, then kills the process tree after 300 ms.
- **MCP limits and annotations:**
  - Output: a warning above 10,000 tokens, and `MAX_MCP_OUTPUT_TOKENS` defaults to 25,000. Text over 50,000 chars is saved to a file unless `_meta["anthropic/maxResultSizeChars"]` raises the threshold (500,000 max). HTTP or SSE responses over 16 MiB fail. Error text over about 11,000 chars keeps only the first and last 5,000.
  - Timers: at least 60 s to the first response byte; an idle timeout of 5 min for HTTP and 30 min for stdio; `MCP_TOOL_TIMEOUT` defaults to about 28 h.
  - Tools: top-level input property names are 1–64 chars of `[A-Za-z0-9_.-]`. An input schema that isn't valid JSON Schema 2020-12 excludes the tool. Root `anyOf`, `oneOf` and `allOf` are flattened. A full `mcp__server__tool` name over 128 chars is dropped.
  - `readOnlyHint: true` tools run in parallel with other read-only calls. A `readOnlyHint` or `idempotentHint` call may be repeated once on a new HTTP connection after the old one closes.
- **Subagents, skills, rules and permissions** (changes since CC-P2):
  - Subagents: new fields `autoCompactWindow` (2.1.296+), `omitClaudeMd` (2.1.271+) and `experimental.cacheTtl` (2.1.248+). From 2.1.296 the debug log reports unknown fields. A file whose `---` isn't the first line is skipped. `disallowedTools` is applied first, and an entry with a specifier such as `Bash(git push *)` removes the whole tool. `permissionMode` accepts `manual` as an alias for `default`. A background subagent (the default) keeps every MCP tool but only some built-ins, Read, Grep and Glob among them. Frontmatter hooks and inline `mcpServers` in a project subagent run only after trust, and `-p` doesn't count as trust.
  - Skills: frontmatter is read only when `---` is the first line, and unparseable YAML loads the skill with no fields set. Booleans accept yes, no, on, off, 1 and 0. `name` defaults to the directory name. `metadata` is a free-form map (a non-map value is dropped), and `license` is accepted but not acted on. The body substitutes `$ARGUMENTS`, `$N`, `${CLAUDE_SKILL_DIR}` and `${CLAUDE_PROJECT_DIR}` (escape `$` before a digit with `\`); supporting files aren't substituted. Angle brackets in `description` show as `&lt;` and `&gt;`.
  - Rules: brace expansion has a budget of 1,000 patterns and 4 MiB per rule. An unparseable `[` makes that pattern match nothing. User-level rules are relative to the primary working directory.
  - Permissions: `Bash(x *)` and `Bash(x:*)` are equivalent. `:*` is recognized only at the end, the permission dialog writes the space form, and a trailing ` *` also matches the bare command. Rules match after compound-command splitting and wrapper stripping, every subcommand must be allowed, and a rule doesn't match `/abs/path/cmd` or `sh -c '…'`.
  - Hooks: the same handler defined in several settings files runs once; plugin and skill copies stay separate.
- **SchemaStore gap:** the settings schema (src `bf7c6745505c`, 2026-10-19) has `additionalProperties: false` on command hooks and no `onFailure` property, so a strict validator rejects a correct `onFailure` binding. Don't validate generated settings against it strictly.
- **Not documented (UNVERIFIED):**
  - the size limit and encoding of hook stdin;
  - the first version that supports `suppressOriginalPrompt`, and how versions before 2.1.295 treat an unknown `onFailure` key;
  - whether a blocked prompt's text returns to the input box;
  - whether the OAuth client uses PKCE or sends RFC 8707 `resource` (the docs are silent; CHANGELOG 2.1.321 mentions a managed `oauth.omitResource`);
  - whether Claude Code sends `Origin` on Streamable HTTP requests;
  - whether `http://` server URLs, loopback included, work for `type: "http"` (the only TLS rule found covers OAuth token endpoints);
  - a stdio server's working directory, and whether `roots/list` is still answered on 2026-07-28 connections;
  - the full set of credential variables that read as empty;
  - the `tool_input` shape of legacy MultiEdit;
  - whether the hook PATH differs in sessions started from the desktop app;
  - whether PreToolUse fires for ExitPlanMode and AskUserQuestion in a way a hook must ignore.

### MCP-P3: server duties Grimoire implements (MCP 2026-07-28)
Sources: https://github.com/modelcontextprotocol/modelcontextprotocol at tag `2026-07-28` (commit `5f5440bb26a6`): `docs/specification/2026-07-28/**` and `schema/2026-07-28/schema.ts`, diffed against main `05aeb07`. The live site, https://modelcontextprotocol.io/specification/2026-07-28, tracks main. RFC 9728, 6750, 8707, 9068, 7662, 8414, 8725, 7515 and 9110 from https://www.rfc-editor.org. https://www.ietf.org/archive/id/draft-ietf-oauth-v2-1-13.txt and draft `-17`. Status: Verified. Paths below are relative to `docs/specification/2026-07-28/`.

- **Streamable HTTP** (`basic/transports/streamable-http.mdx`):
  - The server MUST provide one MCP endpoint path that supports POST. Each POST body MUST be a single JSON-RPC request or notification, and clients MUST NOT send responses, so batches are invalid. Request ids MUST NOT be `null` (`basic/index.mdx`).
  - Servers MUST validate `Origin`, and a present, invalid `Origin` MUST get 403 (the body MAY be a JSON-RPC error with no `id`). **Behaviour for an absent `Origin` is not defined**, so it is a policy choice. There is no Host-header rule: the spec's DNS-rebinding defence is Origin validation plus localhost binding.
  - Local servers SHOULD bind only to 127.0.0.1, not 0.0.0.0, and SHOULD authenticate all connections.
  - Clients MUST send `Accept` listing both `application/json` and `text/event-stream`. No server status is defined for a wrong `Accept`.
  - A request gets `application/json` or a request-scoped `text/event-stream`, at the server's choice. On SSE the server MAY send notifications related to the request, MUST NOT send independent requests, SHOULD end the stream with the response and SHOULD set `X-Accel-Buffering: no`. `Last-Event-ID` resumption isn't supported. Closing the stream MUST be treated as cancellation.
  - An accepted notification MUST get 202 with no body, and a refused one an HTTP error status. This revision defines no header requirements for notification POSTs.
  - GET or DELETE to the endpoint SHOULD get 405 (RFC 9110 §15.5.6: a 405 MUST carry `Allow`). An `Mcp-Session-Id` header is ignored and never minted, and `Last-Event-ID` is ignored.
- **Required headers and their errors:**
  - Every POST MUST carry `MCP-Protocol-Version`, matching `_meta` `io.modelcontextprotocol/protocolVersion`. `Mcp-Method` (= the body's `method`) is required on all requests, and `Mcp-Name` (= `params.name` or `params.uri`) on `tools/call`, `resources/read` and `prompts/get`.
  - Values that aren't plain header-safe ASCII use the case-sensitive sentinel `=?base64?<base64 of UTF-8>?=`, which servers MUST decode before comparing with the body. Header names are case-insensitive; values are case-sensitive.
  - A mismatch, a missing required header or invalid header characters get 400 with `-32020` (HeaderMismatch). A server that doesn't support pre-2025-06-18 clients MUST reject a missing `MCP-Protocol-Version` this way; one that does MAY assume 2025-03-26.
  - An unsupported version gets 400 with `-32022` (UnsupportedProtocolVersion), whose `data.supported` and `data.requested` are both required. An unimplemented method, including one behind an unadvertised capability, gets 404 with `-32601`. A modern-only server SHOULD name its versions in any error it returns to `initialize` (`basic/versioning.mdx`).
  - `Mcp-Param-*` headers (`x-mcp-header`) are optional for servers, and servers SHOULD NOT mark sensitive parameters with them.
- **Statelessness and `server/discover`** (`basic/index.mdx`, `server/discover.mdx`, `schema.ts`):
  - Servers MUST NOT rely on earlier requests on the connection for capabilities, version or client identity.
  - Every request's `params._meta` MUST carry `io.modelcontextprotocol/protocolVersion` and `io.modelcontextprotocol/clientCapabilities`. A missing one gets `-32602`, with HTTP 400. Requiring an undeclared client capability gets `-32021` with `data.requiredCapabilities`.
  - `initialize` is removed, and servers MUST implement `server/discover`. `DiscoverResult` has `supportedVersions`, `capabilities` and optional `instructions`, plus the required `resultType`, `ttlMs` and `cacheScope`.
  - Every result MUST include `resultType` (`"complete"`, or `"input_required"` for MRTR). Servers SHOULD put `io.modelcontextprotocol/serverInfo` (`name`, `version`) in every result's `_meta`. Self-reported client and server info SHOULD NOT drive security decisions.
  - Servers MUST NOT send JSON-RPC requests. Logging, Roots and Sampling are deprecated, and `ping` and `logging/setLevel` are removed.
  - "All implementations MUST support the base protocol, versioning, and the message patterns". How a server with no notification capabilities answers `subscriptions/listen` isn't specified. Post-tag commit `71f7c162` adds that a server SHOULD answer the original request with a completion result before closing the stream.
  - JSON Schema defaults to 2020-12, and implementations MUST NOT dereference network `$ref` automatically.
- **Caching hints** (`server/utilities/caching.mdx`): results with `resultType: "complete"` from `server/discover`, `tools/list`, `prompts/list`, `resources/list`, `resources/templates/list` and `resources/read` MUST carry `ttlMs` (≥ 0) and `cacheScope` (`"public"` or `"private"`). Use `"private"` when content depends on the authorization context. All pages of one list share a scope. Access control MUST NOT rely on `cacheScope`. `tools/call` results aren't cacheable.
- **Error codes and pagination:** `-32020` to `-32099` are reserved for MCP, with `-32020`, `-32021` and `-32022` defined. New implementations SHOULD NOT use `-32000` to `-32019` and MUST NOT emit `-32002` or `-32042`. Cursors are opaque strings, an absent `nextCursor` ends the list, and invalid cursors SHOULD get `-32602` (`server/utilities/pagination.mdx`).
- **Tools** (`server/tools.mdx`, `schema.ts`):
  - Declare the `tools` capability. `tools/list` MUST NOT vary per connection, MAY vary by the presented authorization, and SHOULD be in a deterministic order.
  - `inputSchema` (`type: "object"`) is required, and `{type: "object", additionalProperties: false}` is recommended for no parameters. Post-tag commit `38353251fa`: with composition keywords, declare properties at the root and keep branches to presence assertions such as `required`.
  - Tool-name rules are SHOULD-level (MCP-SPEC).
  - `CallToolResult.content` is required even with `structuredContent`. With an `outputSchema`, `structuredContent` MUST conform, and SHOULD also be returned serialized in a TextContent block. Content blocks are text, image, audio, `resource_link` and embedded `resource`.
  - Protocol errors (an unknown tool, a request failing the `CallToolRequest` schema) are JSON-RPC errors such as `-32602`. Execution errors (API failures, input validation, business logic) are results with `isError: true`. **Post-tag erratum `ac00313ff7` (2026-11-16, live on the site):** invalid tool arguments are execution errors with `isError: true`; `-32602` stays for a malformed call, such as `arguments` that isn't an object.
  - Servers MUST validate all tool inputs, implement access controls, rate-limit invocations and sanitize outputs.
  - `destructiveHint` and `idempotentHint` mean something only when `readOnlyHint` is false. The defaults are in MCP-SPEC.
- **Resources** (`server/resources.mdx`): declare `resources` (`{}` is allowed). A `Resource` needs `uri` and `name`, and `size` is raw bytes before base64. `resources/read` returns `contents` of `{uri, mimeType?, text}` or `{uri, mimeType?, blob}`. A missing resource MUST get `-32602` (the example carries `data.uri`), never an empty `contents`, and internal errors SHOULD use `-32603`. `file://` may name filesystem-like resources that aren't physical files, and custom schemes MUST follow RFC 3986. Servers MUST validate URIs and MUST sanitize `file://` paths against traversal. The spec sets no size limit on resource contents or tool results.
- **Authorization as a resource server** (`basic/authorization/*.mdx`):
  - Authorization is OPTIONAL. HTTP implementations SHOULD conform; stdio SHOULD NOT, and takes credentials from the environment. Clients and servers "MAY negotiate their own custom authentication and authorization strategies" (`basic/index.mdx`), so a static bearer is allowed for local HTTP. With no authorization server, though, such a mode can't serve conformant PRM.
  - It is based on OAuth 2.1 draft-13, plus RFC 6750, 8414, 7591, 8707, 9728, 9207 and CIMD draft-00. One link cites draft-14, for refresh tokens.
  - Servers MUST implement RFC 9728 PRM, and **the document MUST list at least one `authorization_servers` entry**, though RFC 9728 makes the field optional. Servers MUST implement at least one discovery mechanism: `resource_metadata` in `WWW-Authenticate`, or the well-known URI. Clients MUST support both, trying the path-inserted URI, then the root.
  - Path insertion (RFC 9728 §3.1): `https://host/mcp` becomes `https://host/.well-known/oauth-protected-resource/mcp`, after removing any trailing slash that follows the host. The returned `resource` MUST be identical to the identifier the suffix was inserted into (§3.3; compared code point by code point with no normalization, §6), so a root document can't claim `https://host/mcp`.
  - PRM fields (§2): `resource` is REQUIRED. `scopes_supported` and `resource_name` are RECOMMENDED. `authorization_servers`, `jwks_uri`, `bearer_methods_supported` (`header`, `body`, `query`) and others are OPTIONAL, as are `resource_signing_alg_values_supported` (never `none`) and `signed_metadata`. The response is 200 `application/json`, parameters with zero values are omitted, and HTTP caching applies.
  - The resource identifier is an https URL with no fragment, and SHOULD have no query (RFC 9728 §1.2). OAuth 2.1 -13 §1.5 requires https for every AS, RS and client URL except loopback redirect URIs. The MCP canonical URI has a lowercase scheme and host (servers SHOULD accept uppercase) and no trailing slash unless it is significant.
  - The 401 challenge is `WWW-Authenticate: Bearer resource_metadata="<PRM URL>"`, and MCP servers SHOULD add `scope="…"`. RFC 6750 §3: each auth-param at most once, with strict character sets. RFC 9110 §15.5.2: every 401 MUST carry `WWW-Authenticate`.
  - RFC 6750 §3.1: `invalid_request` gets 400, including a request using more than one token method; `invalid_token` (expired, revoked, malformed) gets 401; `insufficient_scope` gets 403. **A request with no credentials SHOULD get no `error` attribute.** The header syntax is `"Bearer" 1*SP b64token` (RFC 6750 §2.1; OAuth 2.1 -13 §5.1.1 calls the token part `token68`), and the scheme name is case-insensitive (OAuth 2.1 -13), so don't match `Bearer ` literally.
  - Invalid or expired tokens MUST get 401. Insufficient scope SHOULD get 403 with `error="insufficient_scope"`, `scope` (every scope the operation needs, in one challenge) and `resource_metadata`. Servers MUST account for scope hierarchies, and SHOULD NOT put `offline_access` in challenges or `scopes_supported`.
  - Clients MUST send `Authorization: Bearer` on every request, and resource servers MUST ignore tokens in the query string (OAuth 2.1 §5.1). Servers MUST validate tokens per OAuth 2.1 §5.2 and the audience per RFC 8707 §2, MUST NOT accept or transit other tokens, and MUST store tokens securely, because cached or logged tokens are a theft vector. The AS may map `resource` to a more general audience (RFC 8707 §2).
  - OAuth 2.1 -17 (2026-10-31) is the latest draft. Its resource-server sections, §5 to §5.3, match -13, and its header reads "Obsoletes: 6749, 6750 (if approved)".
- **RFC 9068 resource-server checks:**
  - JWT access tokens MUST be signed and MUST NOT use `none`. Resource servers MUST support RS256.
  - §4: `typ` MUST be `at+jwt` or `application/at+jwt`. It is a media type, so it compares case-insensitively, and a value without `/` means `application/…` (RFC 7515 §4.1.9). `iss` MUST match exactly, `aud` MUST contain the RS's identifier, the signature is verified with the AS's keys (advertised through RFC 8414 `jwks_uri`), and `exp` gets a leeway of no more than a few minutes. Any failure is `invalid_token`. The `typ` check also blocks OIDC ID tokens.
  - §2.2 requires the AS to issue `iss`, `exp`, `aud`, `sub`, `client_id`, `iat` and `jti`, but §4 doesn't require an RS to reject a token without `sub`, `client_id`, `iat` or `jti`. Scopes come from a space-delimited `scope` claim.
  - RFC 8414 §3.3: the metadata's `issuer` MUST be identical to the issuer its URL was built from. RFC 7662 introspection (not adopted): a form-encoded POST over TLS 1.2 or later with RS authentication; only `active` is required in the response; results aren't cached beyond `exp`.
- **RFC 8725 controls:** libraries MUST let the caller set the allowed algorithms, and "each key MUST be used with exactly one algorithm" (§3.1). Following a `jku` or `x5u` header "could result in server-side request forgery (SSRF) attacks" (§3.10), so keys come only from configured JWKS or from AS metadata whose issuer was validated.
- **Official authorization tutorial** (https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/authorization, reworked after the tag in `fd502785`). These are interop notes, not normative text:
  - Its examples don't check `typ`, "because many authorization servers use another value". Some authorization servers use `scp`, or an array instead of a string, for scopes.
  - If the JWKS or introspection endpoint can't be reached, return 5xx rather than 401, which would force a pointless re-login. For opaque tokens, reject a missing `aud`, and treat any introspection error status, even 401, as an outage. Encrypt cached tokens and delete each entry no later than the token expires.
  - Browser clients need CORS middleware that answers preflight requests before authentication and exposes `WWW-Authenticate`.
  - Its example servers run at `http://localhost:3000/mcp`. Deployments go behind a reverse proxy that terminates TLS and keeps the original `Host`.
- **Security guides:**
  - The best-practices page moved: `/specification/2026-07-28/basic/security_best_practices` 308-redirects to https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/security_best_practices, a docs page outside the normative tree. Cite the new URL.
  - It replaces Session Hijacking with State Handle Hijacking: servers implementing authorization MUST verify all inbound requests and MUST NOT treat possession of a state handle as authentication. Servers meant to run locally SHOULD prefer stdio or restrict HTTP access (require an authorization token, or use IPC). Its SSRF guidance targets clients and authorization servers. Publishing every scope in `scopes_supported` is listed as a common mistake.
  - A new guide, added after the tag: https://modelcontextprotocol.io/docs/2026-07-28/tutorials/security/local-server-security. "Binding to 127.0.0.1 is not an authentication boundary".
- **Post-tag commits:** seven commits between the tag and main `05aeb07` touch `docs/specification/2026-07-28` or `schema/2026-07-28`: `db4bfcff`, `71f7c162`, `3f6e5e17`, `9850cae8`, `38353251`, `ac00313f` and `de3550c9`. The transports, discover, caching, pagination and versioning pages are unchanged since the tag.

### RMCP-P3: rmcp 3.10.0 server API
Sources: https://github.com/modelcontextprotocol/rust-sdk at tag `rmcp-v3.10.0` (commit `53e64a93c8e3`; `rmcp-macros-v3.10.0` is the same commit), https://crates.io/api/v1/crates/rmcp/3.10.0, https://docs.rs/rmcp/3.10.0/rmcp/, and issue and advisory states from the GitHub API. The researcher and the skeptic each built a skeleton server against `rmcp =3.10.0` and ran `cargo test`, `cargo +1.96 check`, `cargo +1.100.0 clippy` (pedantic) and cargo-deny with Grimoire's `deny.toml`; the researcher also ran stdio and curl sessions, and the skeptic ran no network server. Neither ran the conformance suite. Status: Verified; one claim (factory calls per request) was corrected. Source paths are under `crates/rmcp/src/`.

- **Features:**
  - `default = ["base64", "macros", "server"]`. `server` already enables `schemars` (schemars ^1.0, re-exported as `rmcp::schemars`), `transport-async-rw`, `pastey` and `uuid`. `macros` adds rmcp-macros, which depends on syn 3 and darling 0.24.
  - `transport-io` (stdio) adds tokio `io-std` and `net`. rmcp's own tokio dependency has only `sync`, `macros`, `rt` and `time`, so a binary adds `rt-multi-thread`, plus `signal` to handle Ctrl-C.
  - `transport-streamable-http-server` brings http 1, http-body 1, http-body-util 0.1, bytes 1, sse-stream and tower-service 0.3. **rmcp has no axum or hyper-server dependency**: axum 0.8 is a dev-dependency only.
  - The `auth*` features are OAuth-client support (oauth2 5, reqwest 0.13, url). `auth-client-credentials-jwt` also pulls jsonwebtoken ^11 with `aws_lc_rs`.
  - With `local` (non-Send handlers), `StreamableHttpService` and its config aren't compiled.
- **`ServerHandler` and macros:**
  - `ServerHandler: Sized + Send + Sync + 'static`, with every method defaulted. `get_info` returns `ServerConfig`, an alias of `InitializeResult` (`ServerInfo` is deprecated). `call_tool` returns `Result<CallToolResponse, ErrorData>` and `read_resource` returns `Result<ReadResourceResponse, ErrorData>`; build the latter with `ReadResourceResult::new(..).into()`. The root README's `ReadResourceResult` return type is wrong. The trait is implemented for `Arc<H>` and `Box<H>`.
  - Capabilities: `ServerCapabilities::builder().enable_tools().enable_resources().build()`.
  - `#[tool]` keys: `name`, `title`, `description` (defaults to the doc comment), `input_schema`, `output_schema`, `annotations(title, read_only_hint, destructive_hint, idempotent_hint, open_world_hint)`, `icons`, `meta`, `local`. **Unset hints are omitted, not defaulted**, so read-only tools set all four explicitly.
  - `#[tool_router]` takes `router`, `vis`, `server_handler` and `allow_empty`. An impl block with no `#[tool]` fn is a compile error.
  - `#[tool_handler]` takes `router`, `meta`, `name`, `version` and `instructions`, and generates `call_tool`, `list_tools`, `get_tool`, plus `get_info` if it is absent. The default router expression rebuilds the router on every call, so store it in a field (`router = self.tool_router`). **The generated `get_info` enables only tools (and prompts), never resources.** The generated `list_tools` sets `ttlMs: 0` and `cacheScope: public` for 2026-07-28 peers.
  - Async `#[tool]` bodies become `Pin<Box<dyn Future + Send>>`, and the generated code uses `rmcp::` paths, so the dependency can't be renamed.
- **Server identity defaults to rmcp's own.** `ServerConfig::new` and a `#[tool_handler]` without `name` and `version` call `Implementation::from_build_env()`, which evaluates `env!("CARGO_CRATE_NAME")` inside rmcp, so the server reports `rmcp` and `3.10.0`. The macro docs wrongly say it defaults to the caller's crate name. Set `with_server_info(Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))`.
- **Schemas:** input schemas come from `Parameters<T>` through schemars with `SchemaSettings::draft2020_12()`, so they carry `$schema` 2020-12. The top-level `title` and `description` are stripped, and nested types go to `$defs`. The root must be `type: object`, or building the router panics. A tool without parameters gets `{"type":"object","properties":{}}` with no `$schema`. rmcp validates nothing beyond serde, so use `#[serde(deny_unknown_fields)]` to reject extra arguments.
- **Structured output:** a `Json<T>` or `Result<Json<T>, E>` return type sets `outputSchema` (2020-12) and returns `structuredContent` plus the same JSON as a text block, with `isError: false`. Output isn't validated against the schema.
- **Errors:**
  - `Ok(CallToolResult::error(..))` is `isError: true`, and `Err(ErrorData)` is a JSON-RPC error. For `Result<T, E>`, an `E` of `String`, `ContentBlock` or `()` becomes `isError: true`, and `ErrorData` becomes a protocol error.
  - Parameter deserialization failures become `isError: true` with the text "failed to deserialize parameters: …". An unknown tool gets `-32602` "tool not found". A missing `Extension<T>`, such as `Extension<http::request::Parts>` on stdio, gets `-32602`.
  - `ErrorData::resource_not_found` (`-32002`) is rewritten to `-32602` for 2026-07-28 peers, and kept for older peers.
- **Resources:** `Resource::new(uri, name)` with `with_title`, `with_description`, `with_mime_type`, `with_size`, `with_icons`, `with_meta` and `with_annotations`. `ResourceTemplate::new(uri_template, name)`. `ResourceContents::text(text, uri)` (its `mimeType` defaults to `text/plain`) and `ResourceContents::blob(b64, uri)`, both with `.with_mime_type()`. List results take `{ resources, ..Default::default() }`. `ReadResourceResult` is `#[non_exhaustive]`, so use `::new`. `RawResource` and `AnnotateAble` don't exist in 3.10.0.
- **Stateless protocol:**
  - `server/discover` is answered automatically, on stdio and HTTP: `DiscoverResult::from_server_info(supported_protocol_versions(), get_info())`, with `ttlMs: 0`, `cacheScope: private` and serverInfo in `_meta`. Keep `get_info` cheap.
  - `supported_protocol_versions()` defaults to every known version, 2024-11-05 to 2026-07-28. The list is advertised by discover, bounds `initialize` (only versions that still have a handshake, up to 2025-11-25), and checks each request's `_meta` version (`-32022`). A 2026-07-28-only list makes `initialize` fail with `-32022`. A `with_protocol_version(old)` in `get_info` is used as the fallback and bypasses the list.
  - Inline-lifecycle requests missing either required `_meta` key get `-32602` naming the missing keys (HTTP 400).
  - At 2026-07-28 and later, rmcp refuses server-to-client requests, answers `ping` and `resources/subscribe`/`unsubscribe` with method-not-found, fills in missing cache hints, and strips `resultType` for legacy peers. `subscriptions/listen` is method-not-found unless `accepted_subscription_filter` returns `Some(..)`.
- **Stdio:**
  - `rmcp::transport::stdio()` returns Tokio's stdin and stdout, which use the blocking pool. `stdio_pipes()` is a non-blocking variant for Linux FIFOs, and panics outside a Tokio runtime with IO enabled.
  - `serve()` waits for `initialize` or a non-discover request with full `_meta`, answering `server/discover` meanwhile. It returns `Err` on EOF before any lifecycle (`ConnectionClosed("initialize request")`, so a `?` in main exits 1), on a notification first (`ExpectedInitializeRequest`), and on a failed initialize (`InitializeFailed`).
  - After the lifecycle, EOF drains in-flight responses for up to 5 s, then `waiting()` returns `QuitReason::Closed`. **Dropping the `RunningService` cancels the server**, so keep it and call `.waiting().await`.
  - The line limit is 16 MiB, and an oversized line is dropped with no reply (#1395). Unparsable JSON is ignored silently, while well-formed JSON of the wrong shape gets an Invalid Request reply.
- **Streamable HTTP:**
  - Requests that negotiate 2026-07-28 are always stateless: no `Mcp-Session-Id`, no GET or DELETE stream. `legacy_session_mode` (formerly `stateful_mode`) affects only older versions, and `initialize` always takes the legacy path.
  - **Corrected:** in stateless mode the service factory runs once per HTTP request, and that instance is reused for SEP-2243 `Mcp-Param` validation. Extra factory calls happen only on legacy-session paths. Return a cheap `Arc<Server>` clone from the factory.
  - The header validator (`transport/common/mcp_headers.rs`) runs when the `MCP-Protocol-Version` header is 2026-07-28 or later. It requires `Mcp-Method`, requires `Mcp-Name` for `tools/call`, `resources/read` and `prompts/get` (400 with `-32020`), decodes the base64 sentinel, and rejects duplicate headers. A `_meta` protocolVersion without the header is rejected separately.
  - Statuses: with `with_legacy_session_mode(false)` and no event store, any method but POST gets 405 with `Allow: POST`. That includes OPTIONS, so browser CORS needs an outer layer. With the default (`true`), GET and DELETE are served for legacy sessions. An `Accept` missing either type gets 406; a `Content-Type` other than `application/json` gets 415; a body over the cap gets 413. Notifications get 202 and are ignored, never reaching the handler. JSON-RPC responses get 400. On the negotiated 2026-07-28 path, `-32022`, `-32021` and `-32020` map to 400, `-32602` to 400 only for malformed `_meta`, `-32601` to 404, and everything else to 200.
  - A legacy request without the protocol header (served as 2025-03-26), or with a header naming any known version such as `2025-03-26`, is served even if `supported_protocol_versions()` excludes that version, because the header check uses only `KNOWN_VERSIONS` (`tower.rs` L434–466). Only `with_stateless_protocol_metadata_required(true)` rejects it (400 with `-32020`).
- **`StreamableHttpServerConfig`** is `#[non_exhaustive]`, so build it from `Default` plus builders:

  | Field | Default |
  |---|---|
  | `sse_keep_alive` / `sse_retry` | 15 s / 3 s |
  | `legacy_session_mode` | `true` |
  | `json_response` | `false` (SSE responses) |
  | `allowed_hosts` | `localhost`, `127.0.0.1`, `::1` |
  | `allowed_origins` | empty, so **Origin validation is off** |
  | `session_store` | `None` (no builder; assign the public field) |
  | `max_request_body_bytes` | 4 MiB |
  | `stateless_protocol_metadata_required` | `false` |

  Builders: `with_allowed_hosts`, `disable_allowed_hosts`, `with_allowed_origins`, `enforce_origin_validation`, `disable_allowed_origins`, `with_sse_keep_alive`, `with_sse_retry`, `with_legacy_session_mode`, `with_json_response`, `with_cancellation_token`, `with_max_request_body_bytes`, `with_stateless_protocol_metadata_required`.
- **Origin and Host checks** (`transport/streamable_http_server/tower.rs`):
  - Origin validation runs only with a non-empty allow-list or `enforce_origin_validation()`. A missing `Origin` always passes. A disallowed, malformed or non-UTF-8 `Origin` gets 403 with a plain-text body and no Content-Type. Matching follows RFC 6454: `:*` means any port, a portless entry matches any port (deprecated, and logged as a warning), and `null` matches only if listed. **Malformed entries, such as a schemeless `127.0.0.1:8080`, are dropped silently** (failing closed), so test the configured list.
  - Host validation is on by default. An entry with a port matches only a `Host` carrying that port, so `example.com:443` never matches `Host: example.com`, and a portless entry matches any port. **An empty list allows every host.** A disallowed `Host` gets 403. An invalid or non-UTF-8 one, or a missing one with no URI authority, gets 400.
  - Both checks run first inside `StreamableHttpService::handle`, before method, `Accept` or body handling, but after any outer layer. The functions are private (#1334), and rmcp does no path routing.
  - So an outer auth layer answers 401 before rmcp's 403, and the PRM route never reaches rmcp. The front service must route `/mcp` and `/.well-known/oauth-protected-resource/mcp` itself (404 elsewhere) and run its own Host and Origin checks before auth, on both routes, from the same config that feeds `allowed_hosts` and `allowed_origins`. Make it stricter, never looser, validate its lists at startup, and keep rmcp's checks on as defence in depth.
- **`json_response`:** with `with_json_response(true)`, a terminal response is one `application/json` body, falling back to SSE if the handler emits a message first. Non-200 terminal errors are always JSON. rmcp sets `X-Accel-Buffering: no` on SSE.
- **Session managers:** construct the service with `StreamableHttpService::new(move || Ok(server.clone()), Arc::new(NeverSessionManager::default()), config)`. The factory is `impl Fn() -> Result<S, std::io::Error>` with `S: ServerHandler + Send + 'static`, and the service is a `tower_service::Service<http::Request<B>>` with `Error = Infallible`. `LocalSessionManager` keeps legacy sessions in memory, and `NeverSessionManager` is for stateless use. `NeverSessionManager` with the default `legacy_session_mode = true` makes a legacy `initialize` return 500 "Session management is not supported", so pair it only with `with_legacy_session_mode(false)`. Configuring an event store re-enables GET.
- **Request data:** rmcp inserts each request's `http::request::Parts` (minus the body) into the rmcp extensions. Tools read it through `RequestContext<RoleServer>` or `Extension<http::request::Parts>`. Values an outer layer adds, such as validated claims (which must be `Clone + Send + Sync + 'static`), are in `parts.extensions`. On stateless HTTP, `ctx.client_info()` falls back to a fabricated `rmcp` identity when `_meta` lacks `clientInfo` (#1425), so use `ctx.meta.client_info()` and treat `None` as unknown.
- **No server-side auth:** rmcp has no bearer validator, RFC 9728 builder or `WWW-Authenticate` builder, and its RFC 9728 struct is private and deserialize-only. Of the examples, `simple_auth_streamhttp.rs` returns a bare 401 with no challenge or metadata. `complex_auth_streamhttp.rs` has a usable shape (a path-suffixed PRM route, a `resource_metadata` challenge, `route_layer`), but validates by map lookup with no `iss`, `aud` or `exp` checks, and leaves Origin validation off. `cimd_auth_streamhttp.rs` mounts `/mcp` without auth.
- **Testing:**
  - In process, with the rmcp `client` feature as a dev-dependency, wire up `tokio::io::duplex(n)`. `().serve(c)` negotiates 2025-11-25, and `().serve_with_lifecycle(c, ClientLifecycleMode::Discover { preferred_versions: vec![ProtocolVersion::V_2026_07_28] })` negotiates 2026-07-28. Client calls include `list_all_tools()`, `call_tool(CallToolRequestParams::new(name).with_arguments(map))`, `read_resource(ReadResourceRequestParams::new(uri))` and `peer_info()`.
  - HTTP without sockets: `StreamableHttpService::handle(http::Request<Full<Bytes>>)` is public, and needs only http-body-util 0.1 and bytes 1.
  - rmcp CI runs `npx @modelcontextprotocol/conformance@0.2.0-alpha.12 server … --suite all` for 2025-11-25 and 2026-07-28 against a fixture server, and ROADMAP reports 30/30. Whether the suite is meaningful against a server without its fixture tools is UNVERIFIED.
- **Divergences from the spec** (found by the MCP skeptic reading the code; not run):
  - `initialize` is exempt from the header checks and is answered with HTTP 200 (with `-32022` if the version list is narrowed), where the spec wants 400 with `-32020`. To get 400, a front layer must reject a POST without `MCP-Protocol-Version` itself.
  - A notification carrying `MCP-Protocol-Version` 2026-07-28 or later but no `Mcp-Method` gets 400 with `-32020`, although the spec defines no header rules for notifications.
  - Any body that doesn't deserialize, such as malformed JSON or an array, gets 415 with a plain-text body, not 400 with `-32700` or `-32600`.
- **Open issues** (state on 2026-11-16):
  - #1425 (P2): every stateless request logs a fabricated `Service initialized as server` peer named `rmcp` at INFO.
  - #1347 (P1; fix PRs #1375 and #1303 still open): a known method whose params fail to deserialize, such as `tools/call` with `name: 7` or `arguments: "x"`, gets `-32601`, which is HTTP 404 on the 2026-07-28 path.
  - #1395 (P1): an oversized stdio line gets no reply. #1334 (P2): the Host-validation functions are private. #1315 (P2): the pre-lifecycle bootstrap loop is unbounded, so a client can stay pre-lifecycle forever. #1325 (P1) affects only `LocalSessionManager`. #1108 and PR #1256 are client-side.
  - All 8 published GHSAs are patched at or below 3.9.0. The floor is in AUTH-CRATES.
- **Clippy:** under clippy 0.1.100 pedantic, the `list_tools` that `#[tool_handler]` generates, and any synchronous `async fn` handler, trigger `clippy::unused_async_trait_impl`. A scoped `#[expect(clippy::unused_async_trait_impl, reason = …)]` placed after `#[tool_handler(..)]` on the impl passes `-D warnings`.
- **`build.rs` and shipped fixtures:** the published crate has a `build.rs`, identical to the tag, that runs `git config core.hooksPath .githooks` in `CARGO_MANIFEST_DIR/../..` when that directory has both `.githooks` and `.git`. It does nothing from `~/.cargo/registry`, but under `cargo vendor` it targets the project root. The package also ships `tests/test_with_js/*.js` and `tests/test_with_python/*.py`, which cargo-deny's `[bans.build]` flags because the crate has a build script. The bypass is in AUTH-CRATES.
- **Metadata:** rmcp and rmcp-macros 3.10.0 are edition 2024, MSRV 1.88 and Apache-2.0. Their LICENSE describes the MIT-to-Apache-2.0 transition and keeps the MIT text for contributions not yet relicensed. The resolved server set's effective MSRV is 1.88, and a skeleton builds on 1.96 and 1.100.0.

### AUTH-CRATES: server-side token validation dependencies
Sources: the crates.io API (`https://crates.io/api/v1/crates/<name>`); https://github.com/Keats/jsonwebtoken at tag `v12.0.0` (`3b6f68fcd213`, identical to the crates.io source); the rsa 0.10.0 crate source; RustSec advisory-db at `02e2517f457a` (2026-11-15); https://github.com/EmbarkStudios/cargo-deny docs at 0.21.0; RFC 7515, 7518, 8725 and 9068. Experiments ran on standalone variants and on copies of Grimoire HEAD `f8713ab` and of the working tree with a probe member, under cargo-deny 0.21.0 and 0.20.2 (the version CI runs; the results matched), cargo 1.100.0, and an MSRV check on 1.96. Status: Verified; two claims were corrected (the rsa release-candidate count, and how jsonwebtoken checks algorithm families).

- **Results per option** (standalone crates; lock packages, and failures under Grimoire's `deny.toml`). The first two rows are the bases. Option (a) adds to the hyper base, and the other rows add to the axum base, so their failures are the ones added:

  | Variant | Lock | Failures |
  |---|---|---|
  | Base: rmcp + axum 0.8.9, no auth | 118 | matchit licence, syn duplicate, 6 rmcp build-ban files |
  | Base: rmcp + hyper 1.12.0 + hyper-util 0.1.21, no auth | 110 | syn duplicate, rmcp build ban |
  | **(a) + jsonwebtoken 12.0.0 (no backend), rsa 0.10.0, p256 0.14.0, sha2** | 140 | only `subtle` (BSD-3-Clause) |
  | + jsonwebtoken `rust_crypto` | 156 | curve25519-dalek, ed25519-dalek and subtle (BSD-3-Clause); a build ban on `curve25519-dalek-5.0.0/tests/build_tests.sh` |
  | + jsonwebtoken `aws_lc_rs` | 130 | ISC and compound licences (aws-lc-rs, aws-lc-sys, untrusted); 83 build-ban files and an executable-script error; needs a C compiler, NASM on Windows x86/x86-64, and clang-cl on aarch64-pc-windows-msvc |
  | + ureq 3.5.2 with rustls (the TLS part of option (b); in the workspace, (b) adds 133) | 178 | ring (`Apache-2.0 AND ISC`), rustls-webpki and untrusted (ISC), webpki-roots (CDLA-Permissive-2.0), subtle; getrandom and windows-sys duplicates; 38 ring `.pl` bans |
  | + reqwest 0.13.5 with `rustls` | 210 | aws-lc (there is no `rustls-tls` feature) and 18 Unicode-3.0 crates through url and idna; needs tower-http ^0.6.8 |
  | + jwt-simple 0.15.0 | 205 | RUSTSEC-2023-0071 through rsa 0.9.10, 15 duplicates, ISC and BSD-2-Clause licences |
  | + josekit 0.10.3 | — | openssl and openssl-sys are banned |
  | + rmcp `auth` feature | — | 18 Unicode-3.0 rejections, 11 duplicate families, zerocopy build bans |

  - Option (c), RFC 7662 introspection over ureq, needs every TLS entry from (b). Native TLS through tokio-rustls 0.26.6 (`ring`, `tls12`) needs (b)'s entries minus webpki-roots.
  - **Workspace test:** option (a) as a member of a HEAD copy passes `cargo deny check` (advisories, bans, licenses, sources) with exactly the three entries below, and the lock grows from 172 to 255 packages. It also passes on the working tree with `catalog/tools/*` and regex 1.13.1 (176 to 259). With axum instead of hyper it also needs `{ crate = "matchit", allow = ["BSD-3-Clause"] }` (+91 packages). tower 0.5.3, tower-http 0.7.1 (`limit`, `timeout`, `validate-request`) and http 1.5.0 add no failures.
  - `[graph] all-features = true`, so an optional feature doesn't avoid its entries. `[bans.build]` and advisories also apply to dev-dependencies, while licences and multiple-versions don't, so a TLS-enabled reqwest dev-dependency still fails the build bans. A crate listed in both `[dependencies]` and `[dev-dependencies]` fails `workspace-duplicate`; share it through `[workspace.dependencies]`.
- **The `deny.toml` entries option (a) needs:**
  - A `[licenses]` exception, `{ crate = "subtle", allow = ["BSD-3-Clause"] }`. subtle arrives through crypto-bigint, elliptic-curve, ctutils and others.
  - A `[bans]` skip, `{ crate = "syn@2.0.119", reason = "…" }`. syn 2 comes from tracing-attributes 0.1.31 (through rmcp's default-featured `tracing`) and from zeroize_derive 1.5.0 (through zeroize, digest and signature, from jsonwebtoken, rsa and p256). The skip stays until both move to syn 3, and its reason should name both. Today's lock has syn 2.0.119 only through zerocopy's `cfg(any())` dependency, which cargo-deny's target filter ignores.
  - A `[bans.build]` bypass for `rmcp@3.10.0`, as a per-file allow with the checksums below. A `build-script = "31b905abf91292bab290271e993e347e06a1944e6305f742f69dbd01e6b89349"` entry also passes, but it skips scanning the whole crate.

    | File | SHA-256 |
    |---|---|
    | `tests/test_with_js/client.js` | `1f1b26431e26706dc2d380f9fc3475bc78ada1c0c0ebe7302c1c59267a64b123` |
    | `tests/test_with_js/server.js` | `760f0721c1b9f44272cc3416a71172a9163044c12d2b58310c72d3286081df81` |
    | `tests/test_with_js/streamable_client.js` | `af4ccaf89ec0b34cda1b2af71edf9acd04924410a4ed4f579a0ccbd1360ec18a` |
    | `tests/test_with_js/streamable_server.js` | `01074613ae942d99028c3f31b1ff35e64817c6de30ab8beabeb3823cb97e7498` |
    | `tests/test_with_python/client.py` | `522e2401016ad7d87efa6b13a113c618cc1c1ee568b8f51bd1b92a605668b0ab` |
    | `tests/test_with_python/server.py` | `d27a404deb187a66f6b521d2deea4743bb8dc1e64179f2bf2e3a6cfb58aaa4d9` |

  - Any rmcp server needs the syn skip and the rmcp bypass. The auth layer's own cost is the subtle exception, provided TLS is terminated outside the binary.
- **jsonwebtoken 12.0.0** (MIT, MSRV 1.88, published 2026-11-13; its Cargo.toml badge says "passively-maintained"):
  - Features: `default = ["use_pem"]` (pem, simple_asn1), so use `default-features = false`: keeping `use_pem` adds pem 4.0.0 and simple_asn1 0.6.4 (ISC, which `deny.toml` rejects). `rust_crypto` (ed25519-dalek, hmac, p256, p384, rsa and sha2, all or nothing) and `aws_lc_rs`. At most one backend may be enabled. With neither, the caller supplies a `CryptoProvider` (public fields `signer_factory`, `verifier_factory`, `key_utils`), installed with `install_default()` at most once per process. `try_get_default()` is `None` when neither or both features are on and nothing is installed.
  - `KeyUtils::new_unimplemented()` panics when JWK utilities run: `Jwk::thumbprint`, `Jwk::from_encoding_key` and `Jwk::from_decoding_key`. `decode` and `DecodingKey::from_jwk` don't call them.
  - 12.0.0 rejects an unknown `crit`, fixes "remaining type confusion with sub, iss, aud", and rejects empty HMAC keys. GHSA-h395-gr6q-cpjc (type confusion leading to a potential authorization bypass, `< 10.3.0`) is GitHub-only and not in RustSec, so cargo-deny won't enforce a floor for it.
  - **`Validation` defaults:** `required_spec_claims = {exp}`, `leeway = 60`, `validate_exp = true`, `validate_nbf = false`, `validate_aud = true`, `algorithms = [alg]`. **`aud` and `iss` are checked only when the token has them**, unless they are in `required_spec_claims`: a token with no `aud` was accepted when only `exp` was required. **`typ` isn't checked.** An array `iss` passes if any element matches.
  - **Algorithm families (corrected):** `decode()` rejects a header `alg` outside `Validation.algorithms`, then requires every allowed algorithm to share the verifier's family. So `algorithms = [RS256, ES256]` rejects every valid token with `InvalidAlgorithm`. The key-family check lives inside each provider, so a custom provider must check the key's family itself.
  - `DecodingKey::from_jwk` ignores the JWK's `alg` and `use` fields, and turns `kty: "oct"` into an HMAC key.
  - Probe: a verify-only provider of about 60 lines (RS256 through rsa `Pkcs1v15Sign` and sha2, ES256 through p256 `VerifyingKey`) verified the RFC 7515 A.2 and A.3 vectors, and rejected a tampered signature, algorithm confusion, a wrong `aud` and an expired token. `alg: "none"` fails to parse. The skeptic's own provider reproduced all of this except the wrong-`aud` and expired cases, which it didn't re-run. It builds on 1.100.0 and 1.96.
- **rsa 0.10.0** (MIT OR Apache-2.0, MSRV 1.87, published 2026-11-08):
  - It followed 23 release candidates and 5 pre-releases (a corrected count). The README warns that the code has diverged significantly since its audit. PKCS#1 v1.5, which RS256 needs, is behind the `pkcs1v15` feature, so use `default-features = false` with `std` and `pkcs1v15` only.
  - RUSTSEC-2023-0071 (the Marvin attack, a private-key timing leak) now lists `patched = [">= 0.10.0"]` (advisory-db `7a40350e`, 2026-11-08; upstream RustCrypto/RSA#626 closed through #749). The GHSA copy, GHSA-c38w-74pg-36hr, still says no patch is available. rsa 0.9.x still fails cargo-deny.
  - `PublicKey::new` accepts 1024- to 8192-bit moduli and an exponent up to 2^33−1. RFC 7518 requires at least 2048 bits for RS256, so use `PublicKey::new_with_size_limits(n, e, 2048..=8192)`. `RsaPublicKey` is deprecated since 0.10.0 in favour of `rsa::PublicKey`.
- **p256 0.14.0** (Apache-2.0 OR MIT, MSRV 1.85, published 2026-07-03), used with `default-features = false` and `ecdsa`, `std` for ES256.
- **hyper and hyper-util instead of axum:** rmcp needs no axum. Serve `StreamableHttpService` with hyper 1.12.0 (`server`, `http1`) and hyper-util 0.1.21 (`tokio`, `service`) through `hyper::server::conn::http1::Builder`, `TokioIo` and `TowerToHyperService`. The SDK's hyper example uses `hyper_util::server::conn::auto::Builder`, which needs `server-auto` and so pulls in http2 and h2. A hand-written front service in front of `StreamableHttpService` compiles with only hyper, hyper-util, http, http-body-util, bytes and tower-service. axum 0.8.9 pins `matchit =0.8.4`, whose licence is `MIT AND BSD-3-Clause` because it bundles httprouter's BSD licence.
- **Advisory floors** (advisory-db `02e2517f457a`): tokio ≥ 1.54.1 (RUSTSEC-2026-0384 affects 1.54.0, and 2026-0386 is patched from 1.53.3); rmcp ≥ 3.9.0 (2026-0189, 0457, 0461 and 0462); rustls ≥ 0.23.46 (2026-0346 to 0349); rustls-webpki ≥ 0.103.16 (2026-0382); webpki-roots ≥ 1.0.10 (2026-0381); h2 ≥ 0.4.16 (2026-0258, and h2 is absent from http1-only builds); ring ≥ 0.17.12 (2025-0009). There are no entries for jsonwebtoken, axum, ureq, reqwest, hyper-util, matchit, p256, subtle or aws-lc-rs.
- **Not adopted:** ring, rustls and webpki-roots. Option (a) reads an operator-supplied JWKS file and makes no outbound requests, so it needs no HTTPS client. It also serves plain HTTP, so TLS must be terminated outside the binary, because OAuth 2.1 §1.5 and RFC 9728 require https resource URLs. ring's last release is 0.17.14 (2025-03-11), although its repository is active.

### SECRET-FORMATS: credential formats for secret-guard
Sources: the vendor pages cited per family, fetched 2026-11-16. Third-party rule sets: gitleaks v8.30.1, https://github.com/gitleaks/gitleaks/blob/83d9cd684c87d95d656c1458ef04895a7f1cbd8e/config/gitleaks.toml (MIT, "Copyright (c) 2019 Zachary Rice"), and GitLab's https://gitlab.com/gitlab-org/security-products/secret-detection/secret-detection-rules at `c8396d46d7` (MIT outside `doc/`). Status: Partial. Prefixes are mostly vendor-verified, many lengths are third-party only, and OpenAI is UNVERIFIED.

- **Source classes:** *vendor* facts come from the vendor's own docs or code. *Third-party* facts come only from gitleaks or GitLab's rule set; GitLab's rules for its own tokens count as vendor. GitHub's supported-patterns page (https://docs.github.com/en/code-security/reference/secret-security/supported-secret-scanning-patterns) lists every vendor below but publishes no regexes. GitHub push-protects only "the most recent token versions" it can identify confidently, rates private keys and database connection strings as High precision, and detects generic passwords with AI rather than regex.
- **GitHub** (vendor: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-authentication-to-github#githubs-token-formats, https://github.blog/engineering/behind-githubs-new-authentication-token-formats/, https://github.blog/changelog/2021-03-31-authentication-token-format-updates-are-generally-available/):
  - Prefixes: `ghp_` classic PAT, `github_pat_` fine-grained PAT, `gho_` OAuth, `ghu_` user-to-server, `ghs_` installation, `ghr_` refresh (also in https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/refreshing-user-access-tokens).
  - The charset changed from `[a-f0-9]` to `[A-Za-z0-9_]`, and the last 6 characters are a Base62 CRC32 checksum. GitHub tells integrators to "plan to support tokens up to 255 characters", so an exact `{36}` body can miss future tokens. The 40-character total for prefixed tokens is inferred from "without changing the token length", and OAuth tokens have 30 random characters. The checksum's input and alphabet are undocumented, so it can't be validated offline.
  - **Installation tokens changed in 2026** (https://github.blog/changelog/2026-04-24-notice-about-upcoming-new-format-for-github-app-installation-tokens/, https://github.blog/changelog/2026-05-15-github-app-installation-tokens-per-request-override-header/, https://github.blog/changelog/2026-10-02-stateless-github-app-installation-tokens-rolled-out/). They are now `ghs_<APPID>_<JWT>`: about 520 characters, variable, with two dots (05-15). The format includes the Actions `GITHUB_TOKEN`, and GitHub Enterprise Server isn't affected (04-24). The rollout finished on 2026-10-02, and the two-way `X-GitHub-Stateless-S2S-Token` override header is deprecated on 2026-11-30 (10-02). GitHub's recommended regex is `ghs_[A-Za-z0-9\.\-_]{36,}` (05-15, editor's note of 2026-05-26), and it warns against patterns such as `ghs_[A-Za-z0-9]{36}` (04-24). A stateless token also matches a JWT shape.
  - Format changes for `ghu_` have been announced, but none has been published.
  - Third-party only, so UNVERIFIED: the `github_pat_` length (gitleaks `github_pat_\w{82}`; GitLab `github_pat_[0-9A-Za-z]{22}_[0-9A-Za-z]{59}`) and the `ghr_` length (36 in gitleaks, 76 in GitLab). gitleaks v8.30.1, gitleaks master `5f6ec56cfa` and GitLab still use `(?:ghu|ghs)_[0-9a-zA-Z]{36}`, which misses stateless tokens.
- **GitLab** (vendor: https://docs.gitlab.com/security/tokens/, and GitLab's own `rules/mit/gitlab/gitlab.toml`):
  - Prefixes: `glpat-` (personal, impersonation, project and group access tokens), `gloas-`, `gldt-`, `glrt-`/`glrtr-`, `glcbt-`, `glptt-`, `glft-`, `glimt-`, `glagent-`, `glwt-`, `glsoat-`, `glffct-`. Only the PAT prefix is configurable per instance, and project and group tokens inherit it. GitLab's own scanners miss custom-prefixed tokens by default (https://docs.gitlab.com/administration/settings/account_and_limit_settings/).
  - GitLab's rules: classic `glpat-` plus 20 of `[0-9a-zA-Z_\-]`; routable `glpat-[0-9a-zA-Z_\-]{27,300}\.[0-9a-z]{2}[0-9a-z]{7}`; and a versioned form with another `\.[0-9a-z]{2}`. All allow an optional `[a-zA-Z0-9]{1,20}-` instance prefix.
- **AWS** (vendor: https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-unique-ids, https://docs.aws.amazon.com/IAM/latest/APIReference/API_AccessKey.html, https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html):
  - `AKIA` is an access key and `ASIA` a temporary STS key. `ABIA` and `ACCA` exist with undocumented shapes, and "Prefixes may vary based on when they were created."
  - `AccessKeyId` is 16–128 characters of `[\w]+`. The 20-character shape (prefix plus 16) comes only from examples, which include digits such as `8` (`ASIAI44QH8DHBEXAMPLE`), so gitleaks' base32 class `[A-Z2-7]` isn't supported by the docs.
  - **The EXAMPLE convention (corrected):** example access key IDs end in `EXAMPLE` and example secret keys in `EXAMPLEKEY`, but the example session token (`AQoEXAMPLEH4aoAH0gNC…`) has `EXAMPLE` mid-string, so an `EXAMPLE$` allowlist doesn't extend to session tokens. gitleaks' AWS rule already allowlists `.+EXAMPLE$`.
  - The secret access key's length and charset are undocumented. The examples are 40 characters of `[A-Za-z0-9/]`; `+` is a base64 inference. Documented names: `aws_access_key_id`, `aws_secret_access_key` and `aws_session_token` in config files, `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY` in the environment, and `SecretAccessKey` in IAM API and CLI JSON.
- **Slack** (vendor: https://docs.slack.dev/authentication/tokens/, …/authentication/using-token-rotation/, …/messaging/sending-messages-using-incoming-webhooks/):
  - Prefixes: `xoxb-` bot, `xoxp-` user, `xapp-` app-level, `xwfp-` workflow (expires 15 minutes after issue), `xoxe.xapp-` rotating app-level. With token rotation, access tokens gain an `xoxe.` prefix (`xoxe.xoxb-1-…`), and refresh tokens start `xoxe-1-`.
  - Tokens are `-`-separated sections, and the last section is the secret: 32 characters, or 6 or 10 for user tokens created before August 2016.
  - Incoming webhook URLs contain a secret. The documented placeholder is a `https://hooks.slack.com/services/` URL with all-zero `T…` and `B…` IDs and a 24-character all-`X` secret. (It isn't quoted here because CI's gitleaks scan could flag it.)
- **Stripe** (vendor: https://docs.stripe.com/keys, https://docs.stripe.com/events/manage-webhook-endpoints): live keys `sk_live_`, `rk_live_`, `pk_live_`; sandbox keys `sk_test_`, `rk_test_`, `pk_test_`; organization keys `sk_org_`; webhook secrets `whsec_`. "Only publishable keys are safe to expose outside your application's back end." No length or charset is documented; minimums are third-party (GitLab `sk_live_[A-Za-z0-9]{24}`, gitleaks `{10,99}`). GitHub push-protects Stripe test secret and restricted keys, but not webhook signing secrets or legacy keys.
- **Google** (vendor: https://docs.cloud.google.com/docs/authentication/api-keys, https://firebase.google.com/docs/projects/api-keys):
  - The documented example key is 39 characters: `AIza` plus 35 that include `-` and `_`. (It isn't quoted here because CI's gitleaks scan could flag it.) `AIza` isn't stated as a rule.
  - "API keys for Firebase services are OK to include in code or checked-in config files", but a Gemini Developer API key "should never be included in your code". That both have the AIza shape is an inference. GitHub doesn't push-protect `google_api_key` or `google_gemini_api_key`.
  - The newer service-account-bound "authorization key" format is UNVERIFIED.
- **Anthropic** (vendor: https://platform.claude.com/docs/en/manage-claude/authentication.md, …/manage-claude/admin-api.md): API keys are static `sk-ant-api…` secrets (example `sk-ant-api03-…`, documented partial hint `sk-ant-api03-R2D...igAA`), and Admin keys start with `sk-ant-admin`. The length and charset are undocumented; gitleaks' 93 characters plus `AA` is third-party. UNVERIFIED: the formats of the Anthropic service-account keys, user API keys and session IDs that GitHub detects, and of `CLAUDE_CODE_OAUTH_TOKEN`, the one-year token from `claude setup-token` (https://code.claude.com/docs/en/iam.md).
- **OpenAI: UNVERIFIED.** platform.openai.com returned 403. The openai-openapi spec (https://github.com/openai/openai-openapi at `d06ec3a778`) has only examples such as `sk-admin-1234abcd`. Third-party: gitleaks requires the marker `T3BlbkFJ`; GitLab uses `sk-proj-` and `sk-svcacct-` with `{40,190}`, and `sk-admin-` with `{124}`.
- **npm** (vendor: https://github.blog/changelog/2021-09-23-npm-has-a-new-access-token-format/): the `npm_` prefix, with a Base62 CRC32 checksum in the last 6 characters. The total length is undocumented; gitleaks uses `npm_[a-z0-9]{36}`, case-insensitive.
- **PyPI** (vendor: https://docs.pypi.org/publishing/publishing-to-pypi/, and https://github.com/pypi/warehouse/blob/99b93c6f8b54e739cc35742f585d1b18a7bbc3f4/warehouse/macaroons/services.py): a token is `pypi-` plus a serialized macaroon v2. For pypi.org the base64 of the v2 header begins `AgEIcHlwaS5vcmc`. That layout is a pymacaroons inference that gitleaks and GitLab corroborate; PyPI doesn't document it. The TestPyPI prefix is UNVERIFIED.
- **Private-key blocks:**
  - RFC 7468 (https://www.rfc-editor.org/rfc/rfc7468.txt) defines only `PRIVATE KEY` (PKCS#8) and `ENCRYPTED PRIVATE KEY`. The boundary is `-----BEGIN ` + label + `-----`, with exactly one space and five hyphens. Parsers may ignore an END-label mismatch, "SHOULD ignore whitespace and other non-base64 characters", and data may precede the boundary. Keys embedded in JSON, such as GCP service-account files, carry literal `\n` escapes.
  - Other labels: OpenSSL 4.1.0 `include/openssl/pem.h` defines `RSA PRIVATE KEY`, `DSA PRIVATE KEY`, `EC PRIVATE KEY`, `SM2 PRIVATE KEY`, `PRIVATE KEY` and `ENCRYPTED PRIVATE KEY` (plus the read-side `ANY PRIVATE KEY`). RFC 5915 §4 documents `EC PRIVATE KEY`, optionally with `Proc-Type` and `DEK-Info` headers. OpenSSH `sshkey.c` (`V_10_6_P2`) has `-----BEGIN OPENSSH PRIVATE KEY-----` (magic `openssh-key-v1`). RFC 9580 Table 16 registers `BEGIN PGP PRIVATE KEY BLOCK`.
- **Other shapes:** a JWT is base64url parts separated by `.` (3 for JWS, 5 for JWE), and an unsecured JWT ends in `.`. An `eyJ` start is an inference from the RFC 7519 examples, not a rule. RFC 3986 §3.2.1 deprecates `user:password` in userinfo, which is the shape of credential-bearing database URLs.
- **gitleaks v8.30.1 (third-party):** 222 rules in Go RE2 syntax, with no look-around. Its `generic-api-key` rule needs 1446 stopwords and 4 allowlists (entropy 3.5), and its private-key rule requires a body of at least 64 characters and an END marker. It supports an inline `gitleaks:allow`, and exits 1 for leaks or errors. Grimoire's CI runs gitleaks 8.30.1 over the full history with `--exit-code 1` (`.github/workflows/security.yml`), so test fixtures must not commit token-shaped literals.
- **Scanner crate behaviour:**
  - regex 1.13.1 (https://github.com/rust-lang/regex/blob/1.13.1/src/lib.rs) has no look-around or backreferences. `RegexSet` reports which patterns matched, not their offsets. `\b` is Unicode-aware unless written `(?-u:\b)`. Iterating with untrusted patterns and haystacks can be quadratic.
  - ignore 0.4.34 skips hidden files (so `.env`) and honours `.gitignore` (only inside a git repo), `.ignore`, the git exclude file and the global gitignore by default. Both ignore and walkdir 2.5.0 follow root symlinks even with `follow_links` off, and walkdir yields symlinks inside the walk as entries, so opening one follows the link.
  - clap 4.6.8 pins clap_builder 4.6.7, whose usage-error exit code is 2, which blocks a Claude Code hook.

### CONTENT-P3: Conventional Commits, Agent Skills, ADR formats
Sources: https://github.com/conventional-commits/conventionalcommits.org at `7d293dc59e` (`content/v1.0.0/index.md`, `content/next/index.md`, `config.yaml`) and https://www.conventionalcommits.org/en/v1.0.0/; https://github.com/semver/semver; https://github.com/agentskills/agentskills at `83021e2be4` and https://agentskills.io/specification.md; https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions; https://github.com/adr/madr/tree/4.0.0 and https://adr.github.io/madr/; https://github.com/adr/adr.github.io at `4f9bd9eada`. Status: Verified.

- **Conventional Commits 1.0.0** is the current final version. The repo has no tags or releases, so versions are content directories, and `content/v1.0.0/index.md` last changed in an editorial commit, `42b627eb40` (2026-03-11).
  - Structure: a header `<type>[optional scope]: <description>`, an optional body, and optional footers, separated by blank lines.
  - The 16 numbered rules, summarized: the header is a noun type, an optional `(scope)` noun, an optional `!`, then a required `: ` and the description. `feat` MUST be used for a new feature and `fix` for a bug fix. The body starts one blank line after the description and is free-form. Footers start one blank line after the body. Other types are allowed.
  - Breaking changes: a `!` immediately before the `:` (with or without a scope; the footer may then be omitted, and the description describes the break), or an uppercase `BREAKING CHANGE: <description>` footer, with `BREAKING-CHANGE` as a synonym token. Both may be used together, on any type.
  - Footers: a word token (with `-` in place of whitespace, as in `Acked-by`; `BREAKING CHANGE` is the exception), then `: ` or ` #`, then a value that may span lines until the next token and separator. They are modelled on git trailers.
  - Case: implementations MUST NOT treat units as case-sensitive, except that `BREAKING CHANGE` MUST be uppercase. The FAQ allows any casing but recommends consistency.
  - SemVer: `fix` maps to PATCH, `feat` to MINOR, and any breaking change to MAJOR. Other types have no implicit effect. Extra types aren't mandated; the spec cites `@commitlint/config-conventional` (Angular): build, chore, ci, docs, style, refactor, perf, test. Revert behaviour is undefined, and the FAQ suggests a `revert` type with a `Refs:` footer.
  - **Licence:** the spec is shown under CC BY 3.0 (`config.yaml`), and the repo LICENSE is MIT. Grimoire writes its own wording and examples, and any quotation needs CC BY 3.0 attribution.
  - The `next` draft (`draft: true`) adds `!!`, an INITIAL STABLE RELEASE footer and 0.y.z rules. It isn't part of 1.0.0. SemVer 2.0.0 (CC BY 3.0; pin tag `v2.0.0`) says 0.y.z is for initial development.
- **Agent Skills:** `docs/specification.mdx` is unchanged since `217be54873` (2026-08-04). HEAD `83021e2be4` (2026-11-04) only added `GOVERNANCE.md` and `MAINTAINERS.md`, the live `specification.md` has the same limits, there are still no tags, and skills-ref is still v0.1.0. The SKILLS entry therefore stands, with its licence line updated. The guidance pages (`docs/skill-creation/best-practices.mdx`, `optimizing-descriptions.mdx`, `using-scripts.mdx`) are CC BY 4.0, so paraphrase them. They recommend descriptions phrased "Use this skill when…"; longer templates in `assets/`, loaded from SKILL.md with a stated reason to read each file; and scripts with no interactive prompts, a `--help`, structured data on stdout, diagnostics on stderr, documented exit codes and bounded output.
- **ADR formats:**
  - **Nygard (2011, CC0 1.0):** Title (a short noun phrase), Context (the forces, value-neutral), Decision (full sentences, active voice, "We will …"), Status (proposed, accepted, deprecated, or superseded with a reference to the replacement) and Consequences (all of them). One decision per ADR, numbered sequentially and monotonically, with numbers never reused. Reversed decisions are kept and marked superseded. An ADR is one or two pages in full sentences. The original location was `doc/arch/adr-NNN.md`.
  - **MADR 4.0.0** (2024-09-17, tag commit `2475fe1973`; MIT OR CC0-1.0): optional YAML front matter (`status`, which also allows `rejected` and `superseded by ADR-0123`; `date`; `decision-makers`; `consulted`; `informed`), a title, Context and Problem Statement, Decision Drivers (optional), Considered Options, Decision Outcome ("Chosen option: …, because …") with optional `### Consequences` and `### Confirmation` subsections, Pros and Cons of the Options (optional) and More Information (optional). The minimal template keeps Context and Problem Statement, Considered Options and Decision Outcome. MADR says to copy it into `docs/decisions`. develop (`ba75bb1b20`, 2026-08-28) has unreleased wording changes.
  - adr.github.io (MIT) defines an ADR as a record of a single architectural decision and its rationale, and calls the collection a decision log.
