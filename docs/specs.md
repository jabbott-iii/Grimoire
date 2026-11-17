# Specification register

This file lists every external format, protocol, registry or tool that Grimoire implements against. Each entry gives the primary source, the version or revision that was read, and the date it was checked.

**Rules for this file**

- Re-verify an entry before you implement or change the code that depends on it. Update the entry in the same change.
- Facts here were read from the primary source on the date shown. Anything not confirmed is marked **UNVERIFIED** and must not be implemented from memory.
- Many sources are unversioned living docs. For those, "Version" records what the page or its release channel showed on the check date.
- Statuses: **Verified** means all facts below were read from the source. **Partial** means some gaps are listed. **Unverified** means the source was unreachable or silent.

## Register

Rows were checked 2026-11-16 unless they say otherwise; the client rows were re-checked 2026-11-17 for Phase 4. Section IDs below link each row to its notes.

| ID | Spec / source | Version or revision checked | Primary URL | Status |
|---|---|---|---|---|
| CC-* | Claude Code: memory, skills, subagents, settings/hooks, `.mcp.json`, plugins, marketplaces | CLI 2.1.326 (npm `latest`); live docs | https://code.claude.com/docs/en/ | Verified; plugins and marketplaces re-checked 2026-11-17 in CC-PLUGIN-P4 and CC-MKT-P4 |
| SKILLS | Agent Skills specification | unversioned; spec file @ `217be54873` (2026-08-04) | https://agentskills.io/specification | Verified |
| AGENTSMD | AGENTS.md | unversioned; repo @ `d001185d79` (2026-09-10) | https://agents.md | Verified (convention, not normative) |
| CODEX-* | OpenAI Codex CLI: AGENTS.md, config/MCP, skills, agents, hooks, rules, plugins | `rust-v0.169.0` (2026-11-13); prerelease `rust-v0.171.0-alpha.11` | https://learn.chatgpt.com/docs/ ; https://github.com/openai/codex | Verified; re-checked 2026-11-17 in CODEX-*-P4 (some facts from source only) |
| CURSOR-* | Cursor: rules, MCP, skills, subagents, hooks | release notes IDE 3.27 / CLI v2026.10.22; shipped IDE 3.30.14 / CLI 2026.11.13 | https://cursor.com/docs/ | Partial; re-checked 2026-11-17 in CURSOR-P4 |
| GEMINI-* | Gemini CLI: context, settings/MCP, skills, subagents, hooks | v0.68.0 (2026-11-10), source and docs @ tag; preview v0.69.0-preview.0 | https://github.com/google-gemini/gemini-cli | Verified from source; re-checked 2026-11-17 in GEMINI-P4 |
| COPILOT-* | GitHub Copilot: instructions, agents, skills, MCP, hooks | CLI v1.0.112; VS Code 1.146.0; docs fetched 2026-11-17 | https://docs.github.com/en/copilot ; https://code.visualstudio.com/docs | Partial; re-checked 2026-11-17 in COPILOT-P4 |
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
| CC-PLUGIN-P4, CC-MKT-P4 | Claude Code plugin manifest, components, marketplace file, dependencies | CLI 2.1.326 (`latest`) / 2.1.311 (`stable`); CHANGELOG @ `bc1e8a88cb44`; docs fetched 2026-11-17 | https://code.claude.com/docs/en/plugins/manifest-reference.md | Verified (Phase 4), gaps listed |
| CC-CI-P4 | `claude plugin validate` and installing Claude Code in CI | 2.1.311 and 2.1.326 manifests and signatures checked 2026-11-17 | https://code.claude.com/docs/en/plugins/cli-reference.md | Partial (offline behaviour UNVERIFIED) |
| CODEX-P4 … CODEX-PLUGINS-P4 | Codex AGENTS.md, trust, MCP, skills, agents, hooks, tools, plugins | `rust-v0.169.0` (`d32063f96c2d`), diffed against `rust-v0.171.0-alpha.11` | https://github.com/openai/codex | Verified (Phase 4), gaps listed |
| CURSOR-P4 | Cursor rules, MCP, skills, subagents, hooks | docs fetched 2026-11-17; shipped IDE 3.30.14 | https://cursor.com/docs/ | Partial (Phase 4), many gaps listed |
| GEMINI-P4 | Gemini CLI context, settings, trust, MCP, skills, subagents, hooks | v0.68.0 (`5a5c7180ff`) source | https://github.com/google-gemini/gemini-cli | Verified from source (Phase 4), gaps listed |
| COPILOT-P4 | Copilot instructions, agents, skills, MCP, hooks, plugins | CLI 1.0.112; VS Code 1.146.0; docs fetched 2026-11-17 | https://docs.github.com/en/copilot | Partial (Phase 4; docs only, no source) |
| EDIT-P4 | TOML and JSONC entry editing | strip-json-comments 3.1.1; toml_edit 0.25.19; jsonc-parser 0.34.0; checked 2026-11-17 | https://unpkg.com/strip-json-comments@3.1.1/index.js | Verified (Phase 4); which TOML version Codex accepts is UNVERIFIED |

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

## Phase 4 additions (checked 2026-11-17)

These facts came from a five-topic research workflow: Codex, Cursor, Gemini CLI, GitHub Copilot, and Claude Code plugins, marketplaces and CI validation. An independent skeptic re-checked each topic against its primary sources: Codex 93 of 100 confirmed (6 refuted, 1 uncertain), Cursor 89 of 94 (2 refuted, 3 uncertain), Gemini CLI 61 of 68 (5 refuted, 2 uncertain), Copilot 81 of 92 (6 refuted, 5 uncertain), Claude Code plugins 97 of 100 (1 refuted, 2 uncertain). Refuted facts appear below in corrected form, uncertain ones are listed as UNVERIFIED, and facts the skeptics added are included where an implementer needs them. No client binary was downloaded or run. The raw results are not committed. Each entry supersedes the 2026-11-16 entry for the same client where they differ.

### CC-PLUGIN-P4: plugin manifest, components and naming
Sources: https://code.claude.com/docs/en/plugins/manifest-reference.md, …/plugins/components.md, …/plugins/loading.md, …/plugins/troubleshooting.md, …/plugins/create.md, …/plugins/publish.md, …/plugins/host-marketplace.md, …/skills.md, …/sub-agents.md, …/hooks.md, …/mcp.md, …/permissions.md (live docs, Last-Modified 2026-11-17 03:13–03:14 GMT, fetched 2026-11-17). CHANGELOG at https://github.com/anthropics/claude-code/blob/bc1e8a88cb447f43697e848d991efed6cac2a64c/CHANGELOG.md (content-identical to `ccc86e33cd`, cited by CC-P3). https://registry.npmjs.org/@anthropic-ai/claude-code. https://www.schemastore.org/api/json/catalog.json (SchemaStore commits `d6c59e8a9b85`, `51aa82eff63e`). Grimoire ID rules: `crates/agent-grimoire-model/src/values.rs`.
Status: Verified, except the UNVERIFIED list. Supersedes CC-PLUGIN where they differ.

- **Releases checked (2026-11-17):**
  - npm `@anthropic-ai/claude-code`: `latest` = `next` = 2.1.326 (published 2026-11-16T18:43:39Z), `stable` = 2.1.311 (2026-10-26T19:47:57Z). Unchanged since 2026-11-16. 564 versions, none a semver prerelease. 2.1.324 is on npm (2026-11-13T18:42:36Z) but has no GitHub release and no CHANGELOG heading.
  - GitHub release `v2.1.326` (2026-11-16T22:11:05Z) is `/releases/latest`. None of the last 100 releases (back to `v2.1.208`) is a prerelease.
  - Native channel pointers: `https://downloads.claude.ai/claude-code-releases/stable` returns `2.1.311` and `…/latest` returns `2.1.326`.
  - CHANGELOG last changed in `bc1e8a88cb44` (2026-11-16T22:10:49Z), and its top entry is `2.1.326`. Repo HEAD `e2c963a886e3` (2026-11-17) changes only `mods/`.
  - The docs describe `latest`. Version gates below are quoted from the docs unless marked CHANGELOG.
- **Version gates that matter on `stable` 2.1.311:**

  | Feature | Needs | On `stable`? |
  |---|---|---|
  | `metadata.pluginRoot` | 2.1.239 | yes |
  | `claude plugin validate --json` | 2.1.259 | yes |
  | validate MCP checks; directory-listing fields not reported as unknown | 2.1.281 | yes |
  | validate path checks for `outputStyles`, `lspServers`, `monitors`, `themes` | 2.1.283 | yes |
  | `Invalid string: must start with "./"` message | 2.1.285 | yes |
  | validating a directory with both manifests | 2.1.289 | yes |
  | README advice; hook `onFailure` | 2.1.295 | yes |
  | `engines["claude-code"]` in `plugin.json` | 2.1.323 (CHANGELOG) | no |
  | marketplace `ref` set to a commit SHA | 2.1.324 (docs); listed under 2.1.325 (CHANGELOG) | no |
  | malformed display field no longer blocks load | 2.1.326 (CHANGELOG) | no |

- **Manifest file:** `.claude-plugin/plugin.json` under the plugin root. It is optional. Every other plugin file (`skills/`, `commands/`, `hooks/` …) goes at the plugin root, not in `.claude-plugin/`. Without a manifest, the plugin name comes from the marketplace entry, or from the directory name with `--plugin-dir`.
- **Fields:** `name` is the only required key. The top-level keys are:
  - metadata: `$schema` (ignored at load), `name`, `displayName`, `version`, `description`, `author{name, email?, url?}`, `homepage`, `repository`, `license` (SPDX), `keywords`, `metadata` (not read);
  - directory listing, `plugin.json` only: `icon`, `documentationUrl`, `supportUrl`, `privacyPolicyUrl`, `termsOfServiceUrl`;
  - behaviour: `defaultEnabled` (default `true`; the entry value overrides it, and a dependency of an enabled plugin starts enabled regardless), `dependencies`, `userConfig`, `types`, `channels`;
  - components: `skills`, `commands`, `agents`, `hooks`, `mcpServers`, `lspServers`, `outputStyles`, `workflows`, `experimental{themes, monitors, evals}`.
  - `engines["claude-code"]` (CHANGELOG 2.1.323): outside its version range a plugin is not installed, updated or loaded. It isn't in the Fields table.
- **Unknown and malformed fields:**
  - An unknown top-level key is stripped, the plugin loads, and validate warns.
  - An unknown key inside a `userConfig` option, `channels` entry, `lspServers` config or `monitors` entry is an error, and the plugin doesn't load.
  - `homepage` must parse as a URL or the plugin fails to load. That is what 2.1.311 does. CHANGELOG 2.1.326 changed this: a plugin with a malformed display field such as `author` or `homepage` now loads, with the field ignored and a note shown. `repository` isn't validated.
  - `version` is a free string, not checked as semver. Setting it pins users to that copy until it changes. A missing `version`, `description` or `author` is a validate warning.
- **Plugin `name`:** non-empty, with no spaces, `@`, `:`, path separators, control characters or bidi-formatting characters.
  - Use kebab-case. Any other form gets the warning `Plugin name "<name>" is not kebab-case`. The claude.ai marketplace sync needs lowercase letters, digits and hyphens.
  - The reserved-name check ignores case and treats a run of separators as one:

    | Name | Result |
    |---|---|
    | starts with `claude-`, `anthropic-`, `anthropics-` or `cc-plugin-` | Error |
    | is `claude`, `anthropic`, `anthropics`, `claude-code` or `claude-mods` | Error |
    | `official` next to `claude` or `anthropic`, e.g. `official-claude-tools` | Error |
    | `claude`, `anthropic` or `anthropics` as a whole word elsewhere, e.g. `mcp-for-claude` | Warning, so `--strict` fails |

  - `claude plugin init` and `claude plugin tag` refuse error-level names, but Claude Code still installs and loads such plugins.
  - Claude Code documents no length limit. Claude Desktop accepts at most 128 characters of letters, digits, `.`, `_` and `-`, starting alphanumeric, and validate warns otherwise.
  - **Grimoire gap:** asset IDs (kebab-case, 1–64 chars, reserved prefixes `claude-`, `anthropic-`, `anthropics-`, `cc-plugin-`, `grimoire-`) avoid the prefix errors, `claude-code` and `claude-mods`. They don't yet reject `claude`, `anthropic`, `anthropics`, the `official` rule or the whole-word warning.
- **Two names:** a marketplace plugin's entry name is its install and enable key and names its cache directory. Its manifest `name` namespaces its components and is what name conflicts compare.
- **Component paths:**
  - Relative to the plugin root, starting with `./` (`commands/foo.md` fails), existing, and resolving inside the root.
  - A path with `..` gets the error `Path contains ".." which could be a path traversal attempt`. A missing path gets `Path not found`.
  - `skills` also accepts `"."` (the root). `mcpServers` also accepts an `https://` `.mcpb` or `.dxt` URL.
  - On macOS and Linux, a component path that contains any backslash is refused.
  - `agents` entries must be `.md` files, not directories. `skills` entries must be directories: either a directory of `<name>/SKILL.md` folders or one folder holding `SKILL.md`. An entry that points at `SKILL.md` itself gets `Path is a file; skills entries must be directories containing SKILL.md`.
- **Manifest keys and default locations:**

  | Effect | Keys |
  |---|---|
  | Replaces the default scan | `commands`, `agents`, `outputStyles`, `workflows`, `experimental.themes`, `experimental.monitors` |
  | Adds to the default `skills/` scan | `skills` |
  | Merges (default file first; for MCP and LSP a later same-named server replaces an earlier one) | `hooks`, `mcpServers`, `lspServers` |

  - When a key replaces a default folder that exists, `claude plugin list` and `/plugin` warn `Default <folder>/ folder is ignored because the manifest sets "<key>"`.
- **Default layout:**

  | Component | Location |
  |---|---|
  | Manifest | `.claude-plugin/plugin.json` |
  | Skills | `skills/<name>/SKILL.md` |
  | Commands | `commands/*.md` (subfolders add segments) |
  | Agents | `agents/*.md`, scanned recursively |
  | Hooks | `hooks/hooks.json` |
  | MCP / LSP | `.mcp.json` / `.lsp.json` |
  | Output styles, workflows, themes | `output-styles/`, `workflows/`, `themes/` |
  | Monitors | `monitors/monitors.json` |
  | Executables | `bin/` |

  - A root `SKILL.md`, with no `skills/` directory and no `skills` key, loads as a single skill. Set its `name`, or a marketplace install names it after the cache directory.
  - **A root `CLAUDE.md` isn't loaded**, and validate warns `CLAUDE.md at the plugin root is not loaded as project context`. Instructions must ship as a skill, and a rule that must always hold, as a hook.
- **Skills:**
  - `skills/<dir>/SKILL.md` runs as `/<plugin>:<dir>`. A frontmatter `name` replaces only the last segment.
  - From 2.1.246, a `name` that already carries the plugin prefix isn't doubled. 2.1.216–2.1.245 doubled it.
  - With a `name` set, the bare `/<name>` also invokes the skill, unless another command uses that name.
  - Characters that a command name can't hold become `-` when they're in a plugin skill folder name.
  - `${CLAUDE_PLUGIN_ROOT}` and `${CLAUDE_PLUGIN_DATA}` are substituted in the body and in `allowed-tools` Bash rules. `${CLAUDE_SKILL_DIR}` is the skill's own subdirectory, not the plugin root.
- **Agents:**
  - An agent is named `<plugin>:<name>`, with `<name>` from frontmatter `name` or the filename.
  - Subfolders of `agents/` add colon segments: `agents/review/security.md` becomes `my-plugin:review:security`. A file listed in the `agents` key loads without subfolder segments.
  - Supported frontmatter: `name`, `description`, `model`, `effort`, `maxTurns`, `autoCompactWindow`, `tools`, `disallowedTools`, `skills`, `memory`, `background`, `omitClaudeMd`, `isolation` (only `"worktree"`), `color` and `experimental.cacheTtl`. **`tools` is honoured.**
  - Ignored: `permissionMode`, `hooks`, `mcpServers` and `initialPrompt`.
  - A `name` over 256 chars is ignored, and the agent is named after its file.
  - Frontmatter that doesn't parse: the agent still loads under its filename with every field ignored. It loses `tools` and inherits every tool. Its description becomes `Agent from <plugin> plugin`, and validate reports the file.
- **Hooks:**
  - `hooks/hooks.json` has the shape `{"description"?: string, "hooks": {"<Event>": [{"matcher"?, "hooks": [handler]}]}}`, the same as `hooks` in settings.json. A file without the top-level `"hooks"` wrapper fails to load.
  - Inline `hooks` in `plugin.json` is the bare event map. The file and the inline key both load.
  - **Exec form works in plugins:** hooks.md's plugin example uses `command` plus `args`. `${CLAUDE_PLUGIN_ROOT}` is substituted into `command` and each `args` element, with no shell.
  - `${user_config.*}` is substituted only in exec form. A shell-form reference is an error (since 2.1.207).
  - Every plugin hook process receives `CLAUDE_PLUGIN_ROOT`, `CLAUDE_PLUGIN_DATA`, `CLAUDE_PROJECT_DIR` and `CLAUDE_PLUGIN_OPTION_<KEY>`.
  - Hooks register when a session loads the plugin and fire from then on, including on subagents' tool calls.
  - Validate warns about an unquoted `${CLAUDE_PLUGIN_ROOT}` in a shell-form hook, unless `shell` is `"powershell"`.
- **MCP servers:**
  - Configured in `.mcp.json` at the plugin root, `{"mcpServers": {…}}` with the wrapper optional, or inline in `mcpServers`. A manifest server replaces a same-named `.mcp.json` server.
  - **Names:** the server is `plugin:<plugin>:<server>`, and its tools are `mcp__plugin_<plugin>_<server>__<tool>`, with every character outside `A-Za-z0-9_-` replaced by `_`.
    - Use the full name in permission rules (`mcp__plugin_<plugin>_<server>__*`), skill `allowed-tools`, a subagent's `tools` and hook matchers.
    - A matcher on the bare server key never fires.
  - **Substitution:** `${CLAUDE_PLUGIN_ROOT}`, `${CLAUDE_PLUGIN_DATA}` and `${CLAUDE_PROJECT_DIR}` in stdio `command`, `args` and `env`, and in remote `url`, `headers` and `headersHelper`.
    - Plugin configs substitute `${CLAUDE_PROJECT_DIR}` directly, with no `:-` default needed, unlike a project `.mcp.json`.
    - Stdio servers also get the three variables exported.
  - **User environment:** plugin servers see the same variables as manually configured servers (docs example `"DB_URL": "${DB_URL}"`). Credential variables such as `ANTHROPIC_API_KEY` and `NPM_TOKEN` read as empty in a remote `url` and `headers`, for plugins too.
  - **Precedence:** local > project > user > plugin > claude.ai connectors. Plugin servers are matched by endpoint, so a plugin server with the same URL or command as a higher one is a duplicate, and only the higher definition connects.
- **`bin/`:**
  - It is on the PATH of the Bash tool's shell, and of PowerShell outside the sandbox, after the user's own PATH entries. A plugin can't shadow system commands.
  - claude.ai and Cowork refuse a plugin with a top-level `bin/`. host-marketplace.md suggests `scripts/` with `${CLAUDE_PLUGIN_ROOT}/scripts/<name>` instead.
- **On disk:**
  - Copied plugins live at `~/.claude/plugins/cache/<marketplace>/<entry-name>/<version>/`. `CLAUDE_CODE_PLUGIN_CACHE_DIR` moves the plugins root. `${CLAUDE_PLUGIN_ROOT}` points at that directory, so it changes with every version.
  - Files outside the plugin directory aren't copied.
  - Symlinks: one inside the plugin is preserved, one elsewhere in the same marketplace is dereferenced, and one outside the marketplace is skipped. A local-path install keeps only links that stay inside the plugin.
  - `${CLAUDE_PLUGIN_DATA}` is `~/.claude/plugins/data/<id>/`, with characters outside `[A-Za-z0-9_-]` in `<id>` replaced by `-`.
  - Relative-path plugins from a marketplace added by local path load in place, and their `version` isn't used.
- **Repository (project-scope) plugins:**
  - They load only after workspace trust. Trusting a parent folder or running with `-p` isn't enough.
  - Their MCP servers need per-server approval. Their `.mcpb` bundles and MCP files outside the plugin are skipped, and their monitors don't load.
  - Claude Code doesn't scan a project's `.claude/plugins/`.
- **Interaction with project `.claude/` files:**

  | Same name in | Result |
  |---|---|
  | `.claude/skills/` and a plugin skill | Both load: `/deploy` and `/my-plugin:deploy` |
  | `.claude/agents/` and a plugin agent | Two agents, `reviewer` and `my-plugin:reviewer`: non-plugin names can't contain `:`. Precedence for a shared name: managed 1 > `--agents` 2 > `.claude/agents/` 3 > `~/.claude/agents/` 4 > plugin 5 |
  | `.claude/settings.json` hook and `hooks/hooks.json` hook | Runs twice per event. Identical handlers are merged across settings files only |
  | Two enabled plugins with one manifest name | One registers its `hooks/hooks.json` hooks: the managed copy, else the first loaded. Load order: managed > `--plugin-dir` > installed marketplace plugin > skills-dir (user over project) > synced |

  - SubagentStart and SubagentStop matchers for plugin agents use the scoped id, and the colon makes the matcher a regex. Anchor it, as in `^my-plugin:db-agent$`.
  - A folder under `.claude/skills/` or `~/.claude/skills/` that holds `.claude-plugin/plugin.json` loads as plugin `<name>@skills-dir`, with its own agents, hooks and MCP servers. A project copy needs workspace trust first.
- **JSON Schema:**
  - No Anthropic page publishes a schema URL for `plugin.json` or `marketplace.json`. Both pages say only that `$schema` is ignored at load.
  - The official marketplace sets `https://anthropic.com/claude-code/marketplace.schema.json`. That URL, like `…/plugin.schema.json`, redirects (301) to `www.anthropic.com` and returns 404.
  - SchemaStore has third-party draft-07 schemas `claude-code-plugin-manifest.json` and `claude-code-marketplace.json`, added 2026-04-23. The manifest schema last changed 2026-09-28.
  - They lag the docs. The manifest schema lacks `displayName`, `defaultEnabled`, `metadata`, `experimental` and `workflows`, and has an undocumented `settings`. The marketplace schema lacks `renames`. Don't validate against them.
- **Not documented (UNVERIFIED):**
  - whether `onFailure` works in a plugin `hooks/hooks.json` (it is documented only as a command and http handler field, with no plugin example or exclusion);
  - whether plugin `bin/` is on the PATH used to resolve an exec-form hook `command`;
  - what happens to an unset `${VAR}` in a plugin MCP config, where the docs conflict. Plugin troubleshooting lists `Missing environment variables: <names>`, so the server isn't resolved. mcp.md says the server loads with the text unexpanded. Whether `${VAR:-default}` applies in plugin configs isn't stated. Write optional variables as `${VAR:-}`;
  - how validate, and `--strict`, treats `engines`;
  - a Claude Code length limit for plugin `name`;
  - whether a plugin agent's frontmatter `name` may contain `:`.

### CC-MKT-P4: marketplace file, sources, dependencies and adding a marketplace
Sources: https://code.claude.com/docs/en/plugins/marketplace-reference.md, …/plugins/create-marketplace.md, …/plugins/host-marketplace.md, …/plugins/dependencies.md, …/plugins/install.md, …/plugins/cli-reference.md, …/plugins/loading.md, …/plugins/publish.md, …/settings-reference.md (live docs, fetched 2026-11-17). CHANGELOG @ `bc1e8a88cb44`. https://github.com/anthropics/claude-plugins-official @ `fd30cee3ecd5` (2026-11-16).
Status: Verified, except the UNVERIFIED list. Supersedes CC-MKT and corrects its reserved-name line.

- **File and root:** `<root>/.claude-plugin/marketplace.json`. The marketplace root is the directory that contains `.claude-plugin/`, and every relative plugin source resolves from it. A marketplace file anywhere else needs `extraKnownMarketplaces` with `path`, because `marketplace add` has no option for it.
- **Top-level fields:**
  - Required: `name`, `owner`, `plugins`.
  - Optional: `$schema` (ignored), `description` (validate warns when it's missing), `version`, `metadata.description`, `metadata.version`, `metadata.pluginRoot` (2.1.239+), `forceRemoveDeletedPlugins`, `allowCrossMarketplaceDependenciesOn` and `renames`.
  - Unknown keys are ignored at load, and validate warns about each one. At load, each entry is checked on its own, so one bad entry doesn't fail the marketplace.
  - `name`: letters, digits, `.`, `_` and `-`, starting with a letter or digit, and no `..`. Validate fails any other name. Users type it after `@`, and each user can register only one marketplace per name.
  - `owner`: `name` is required, and `email` and `url` are optional. An empty `owner.name` is the error `Author name cannot be empty`.
- **Reserved marketplace names** (there is no blanket `anthropic-` prefix rule; `grimoire` matches none of these):
  - Official names, usable only from a `github` or `git` source under `github.com/anthropics/`:
    - `claude-code-marketplace`, `claude-code-plugins`, `claude-plugins-official`, `anthropic-marketplace`, `anthropic-plugins`;
    - `agent-skills`, `anthropic-agent-skills`, `life-sciences`, `knowledge-work-plugins`, `claude-for-legal`, `claude-for-financial-services`, `financial-services-plugins`, `first-party-plugins`, `claude-tag-plugins`;
    - community: `claude-community`, `claude-plugins-community`, `healthcare`;
    - directory: `anthropic-plugin-directory`, `claude-plugin-directory`.
  - Names that impersonate an official one, such as `official-claude-plugins` or `claude-plugins-v2`, and any non-ASCII name.
  - Another spelling of a reserved name, such as `claude.code.plugins` (2.1.280+).
  - `inline`, `builtin`, `skills-dir`, `synced` and `claude-plugin-test`.
  - JavaScript object member names such as `constructor` and `toString` (2.1.296+).
  - The prefix `claudeai-`.
  - `<owner>-<repo>`, the download folder of another registered GitHub marketplace (2.1.290+).
  - **Validate doesn't catch the exact official names.** They pass validation and are refused only at `marketplace add`.
  - Claude Desktop warnings: the names `org`, `org-provisioned` and `unknown` in any case, and names outside its charset or over 128 chars. These are warnings, so they fail `--strict`.
- **Plugin entry:**
  - Required: `name` (letters, digits, `.`, `_`, `-`, starting alphanumeric) and `source`. Duplicate names are an error.
  - Its own optional fields: `description`, `version`, `category` and `tags` (both free-form), `strict` (default `true`), `relevance`, `dependencies`, `defaultEnabled`, `displayName`, `metadata`, plus `headers` and `headersHelper`.
    - `headers` and `headersHelper` apply only to `archive` sources and need 2.1.238+. `headersHelper` also needs `strict: false`.
  - An entry also accepts every `plugin.json` field except the directory-listing fields, which validate reports as unknown.
  - The official marketplace uses categories such as `development`, `productivity`, `database`, `monitoring` and `security`.
- **Entry and `plugin.json`:**

  | Case | Result |
  |---|---|
  | No `plugin.json` | The entry is the manifest, whatever `strict` says. Entry `hooks` load only as an inline object |
  | `plugin.json` present | Entry `mcpServers`, `lspServers`, `userConfig` and `channels` don't apply |
  | present, `strict: true` (default) | Entry `commands`, `agents`, `skills`, `outputStyles` and `themes` are appended. For each event, entry `hooks` matchers replace the manifest's |
  | present, `strict: false`, entry declares any of the six component fields | Load error `Plugin <name> has conflicting manifests` |

  - Entry `hooks` written as a file path or an array pass validate but never run. The plugin shows `not yet supported in a marketplace entry`.
  - Display fields set on the entry override `plugin.json` in listings: `displayName`, `description`, `author`, `homepage`, `repository`, `license` and `keywords`. Before install, Claude Code can read `plugin.json` only for relative-path entries.
  - If an entry whose source is the marketplace root lists `skills` subdirectories, only those load.
- **Relative-path source:**
  - The source must start with `./`, or be `"."` for the root, or be a bare name under `metadata.pluginRoot`.
  - A source without `./` gets `Invalid string: must start with "./"` (`Invalid input` before 2.1.285). A path containing `..` fails validation.
  - On macOS and Linux, a backslash after `./` is refused.
  - A path to a missing directory passes validate, but install fails with `Source path does not exist: <path>`.
  - `metadata.pluginRoot` must itself be relative and inside the marketplace. It has no effect on `./` sources, and a source containing `/` still needs `./`.
- **Other plugin sources:** `github{repo, ref, sha}`, `url{url, ref, sha}`, `git-subdir{url, path, ref, sha}`, `npm{package, version, registry}`, `archive{url, sha256}` and `command{command, timeout, mode}`. `sha` is a full 40-char lowercase commit SHA.
- **Version:**
  - Precedence: `plugin.json` `version`, then the entry `version`, then a value derived from the source.
  - Derived values: `github`, `url` and `git-subdir` give a 12-char commit SHA. A relative path in a git-hosted marketplace gives the commit SHA of the installed directory. A local non-git directory, or `npm`, gives `unknown`. `archive` gives a 12-char SHA-256.
  - A mismatch gets the warning `Entry declares version "<a>" but <path>/plugin.json says "<b>"` (relative entries). The docs say not to set `version` in both places.
  - publish.md: keep `--strict` in CI, or drop it if you omit `version`.
- **`dependencies`** (in `plugin.json` or the entry):
  - An array whose items are `"name"`, `"name@marketplace"` or `{"name", "marketplace"?, "version"?}`. A bare name resolves in the declaring plugin's own marketplace.
  - `version` is an npm semver range (`~2.1.0`, `^2.0`, `>=1.4`, `=2.1.0`). It doesn't match prereleases unless you opt in, as with `^2.0.0-0`.
  - Ranges resolve against git tags named `<plugin-name>--v<version>`. For a relative-path plugin, the marketplace repository holds the tags, and `claude plugin tag --push` creates them.
  - With no matching tag, a relative-path plugin installs the marketplace's current copy and checks the range at load. If that copy is out of range, the dependent stays disabled.
  - With no matching tag, a plugin with its own repository fails to install (`has no git tag satisfying`). A local non-git marketplace has no tags and installs its current contents.
  - A plugin with only `name` and `dependencies` is a valid bundle plugin.
  - **Behaviour:**
    - Install also installs and enables the declared dependencies, at the same scope.
    - Enable also enables installed-but-disabled dependencies, and fails if one isn't installed.
    - Disable is refused while another enabled plugin needs the plugin.
    - Uninstall leaves auto-installed dependencies until `claude plugin prune`, which needs `-y` without a TTY.
    - `marketplace add`, `/reload-plugins`, auto-update and a repeated install also install missing dependencies.
  - **Cross-marketplace:** allowed only if the root marketplace (the one hosting the plugin being installed) lists the target marketplace in `allowCrossMarketplaceDependenciesOn`, or the user already enabled the dependency at the same scope. Otherwise:
    - a dependency declared in the entry makes the install fail;
    - one declared in `plugin.json` lets the install go ahead without it, and the plugin then fails to load.
- **Adding a marketplace:** `claude plugin marketplace add <source> [--scope user|project|local] [--sparse …] [--json]`, or `/plugin marketplace add`.

  | Input | Source type | Relative entries resolve? |
  |---|---|---|
  | `owner/repo`, `owner/repo#ref`, `owner/repo@ref` | `github` | yes |
  | `user@host:path`; an http(s) URL ending in `.git` or containing `/_git/`; a github.com or gitlab.com repo URL | `git` | yes |
  | any other http(s) URL | `url` (fetches only `marketplace.json`) | **no**: install fails `…does not stay inside the marketplace directory` |
  | `./`, `../`, `/`, `~` path to a directory / to a `.json` file | `directory` / `file` | yes |
  | `settings` (inline in settings) | `settings` | rejected outright |

  - **A raw.githubusercontent.com link is a `url` source**, so Grimoire's relative entries would fail. Users must add `owner/repo` or `https://github.com/<owner>/<repo>`.
  - The registered name is the `name` in `marketplace.json`, not the repo name, and the install id is `<entry-name>@<marketplace-name>`.
  - `--scope project` writes `extraKnownMarketplaces` to `.claude/settings.json`. Teammates get it only after workspace trust, and it is ignored without a message in untrusted folders and in `-p` runs.
  - `claude plugin install <name> --marketplace <source>` adds the marketplace and installs in one step (2.1.292+; `/plugin install … --marketplace` 2.1.275+).
  - A `file` source takes the directory two levels above the file as the root.
  - A marketplace `ref` may be a branch, a tag or a full lowercase commit SHA. The SHA form needs 2.1.324+ (docs; CHANGELOG lists it under 2.1.325) and is refused for reserved names. Older clients, `stable` 2.1.311 included, can't load a SHA-pinned marketplace.
  - `autoUpdate` defaults to false for third-party marketplaces.
- **Not documented (UNVERIFIED):**
  - whether `..` inside an entry `name` is refused, as it is for the marketplace `name`;
  - whether an entry `version` equal to the `plugin.json` value still warns (the docs show only the mismatch message).

### CC-CI-P4: `claude plugin validate` and installing Claude Code in CI
Sources: https://code.claude.com/docs/en/plugins/cli-reference.md, …/plugins/manifest-reference.md, …/plugins/marketplace-reference.md, …/plugins/create-marketplace.md, …/plugins/troubleshooting.md, …/plugins/publish.md, …/plugin-evals.md, …/setup.md, …/devcontainer.md, …/troubleshoot-install.md, …/network-config.md, …/env-vars.md, …/data-usage.md, …/plugins/anthropic-marketplaces.md (live docs, fetched 2026-11-17). https://registry.npmjs.org/@anthropic-ai/claude-code (packument modified 2026-11-16T22:09:51Z). https://registry.npmjs.org/-/npm/v1/keys. https://downloads.claude.ai/claude-code-releases/bootstrap.sh (Last-Modified 2026-11-16 18:33 GMT, sha256 `3a68d3406cf6…a944`). `…/2.1.326/manifest.json` and `…/2.1.311/manifest.json`, each with `.sig`. GitHub releases `v2.1.326` and `v2.1.311`. anthropics/claude-plugins-official `.github/workflows/validate-plugins.yml` and `.github/scripts/validate-plugin-folders.ts` @ `fd30cee3ecd5`. npm CLI `docs/lib/content/commands/npm-audit.md` (branch `latest`).
Status: Partial. Offline behaviour is UNVERIFIED, and everything else is Verified. Signatures were checked with gpg in an isolated keyring. No Claude Code binary was downloaded or run.

- **Command:** `claude plugin validate <path> [--strict] [--json]`. `claude plugins` is an alias. `--strict` fails on warnings. `--json` needs 2.1.259+.
- **What a directory run validates:**
  - It checks `.claude-plugin/marketplace.json` if present, otherwise `.claude-plugin/plugin.json`, otherwise component files. Component files means the files in a directory named `skills`, `agents` or `commands`, those three under a directory named `.claude`, or those under `<dir>/.claude`.
  - A directory with both manifests gets both validated (2.1.289+; earlier versions validated the marketplace only).
  - Given a file, naming it `marketplace.json` or `plugin.json` decides which checks run. Any other file in `.claude-plugin/` is a plugin manifest. Any other file elsewhere is classified by its fields.
- **Marketplace runs:**
  - They check the marketplace file and each local-path entry's `plugin.json`, with messages prefixed `plugins[N] plugin.json →`, and the version mismatch.
  - **They don't open the listed plugins' skill, agent, command, hook or MCP files. Validate each plugin directory too.**
- **What it reports:**
  - JSON syntax of the manifest and of `hooks/hooks.json`; required fields and types;
  - marketplace and plugin naming rules and reserved plugin names;
  - component paths that are missing or contain `..`; skills entries that point at a file;
  - unknown fields (warnings); missing `version`, `description` or `author`, non-kebab names, a root `CLAUDE.md` and an unquoted `${CLAUDE_PLUGIN_ROOT}` in shell form (warnings);
  - `No frontmatter block found` (warning) and `YAML frontmatter failed to parse` in skill, agent and command files;
  - MCP entries Claude Code would drop, an undeclared `${user_config.KEY}` and an invalid remote `url` (errors, 2.1.281+);
  - `http://` or `ws://` to a non-loopback host, and header values that look like literal credentials (warnings);
  - `renames` chains, entry/manifest version mismatches and symlinked local sources.
- **What it doesn't catch:**
  - exact official marketplace names;
  - relative sources that point at missing directories;
  - entry `hooks` given as a path or an array;
  - errors fetching remote sources (they show only at install);
  - anything in `.lsp.json`, which validate never reads;
  - a `SKILL.md` at the plugin root in a plugin run.
  - Anthropic's own script JSON-parses `.mcp.json`, `.lsp.json` and `hooks/hooks.json` itself, because the CLI doesn't report every invalid one.
- **Symlinks:** validate doesn't follow links inside the named directory.
  - A linked `skills`, `agents` or `commands` directory under the plugin or `.claude` root: warning that nothing in it was read.
  - Linked entries inside such a directory: skipped, with a warning per directory.
  - The named directory, or its `.claude` parent, is itself a link: error, and nothing in it is checked.
- **Exit codes:**

  | Exit | Verdict line | Meaning |
  |---|---|---|
  | `0` | `Validation passed` / `Validation passed with warnings` | Loads; with `--strict`, no warnings |
  | `1` | `Validation failed` / `Validation failed (--strict treats warnings as errors)` | An error, or a warning under `--strict` |
  | `2` | `Unexpected error during validation: <reason>` | Validator failure. Nothing on stdout; the message is on stderr |

  - README `Advice` (2.1.295+) never changes the verdict or exit code.
- **`--json` report:** a single JSON object.
  - `success`, `strict`, `target`.
  - `manifest`: the manifest's own result, or `null`. In a directory with both manifests it is the `marketplace.json` result.
  - `contents[]`: `{file, errors, warnings, notes}`, with files that have nothing to report left out. Under it, `gatingHooks` (2.1.290+, mods only).
  - `advice[]` (2.1.295+).
  - Anthropic's script reads each issue as `{path, message}`. It validates each plugin folder alone, then again inside a throwaway marketplace, because a standalone run flags names reserved for Anthropic.
- **Credentials and network:**
  - The docs don't say whether `validate` needs network, a login or an API key. They do say `plugin test` needs no session, sign-in or network, and `plugin eval` needs credentials such as `ANTHROPIC_API_KEY`.
  - Validation reads only files inside the directory and fetches no remote plugin source.
  - **Evidence that no login is needed:** Anthropic's `validate-plugins.yml` runs `claude plugin validate --json` on `ubuntu-latest` with `permissions: contents: read`, and no Anthropic secret or login. It doesn't pass `--strict`.
    - It installs with `npm install -g --include=optional @anthropic-ai/claude-code@latest`, retrying three times and falling back to `node "$(npm root -g)/@anthropic-ai/claude-code/install.cjs"`.
  - CHANGELOG fixes show that `claude plugin` commands renew a stored sign-in token on signed-in machines.
- **Install with npm at an exact version:**
  - Docs pattern: `npm install -g @anthropic-ai/claude-code@X.Y.Z` with `DISABLE_AUTOUPDATER=1` (devcontainer.md). The package installs the same native binary as the native installer.
  - **Package mechanics (2.1.326 and 2.1.311):**
    - bin `claude` → `bin/claude.exe`; `scripts.postinstall` = `node install.cjs`;
    - `engines.node` = `>=22.0.0`; an older Node only gets an `EBADENGINE` warning;
    - no `dependencies`, and eight `optionalDependencies` `@anthropic-ai/claude-code-<platform>` at the same version.
  - **Failure modes:**
    - `--omit=optional` breaks the install, and there is no JS fallback.
    - With `--ignore-scripts`, run `node …/install.cjs`, or launch through `cli-wrapper.cjs`.
    - npm 12 skips postinstall unless `allow-scripts` includes `@anthropic-ai/claude-code`.
  - Integrity:

    | Version | `dist.integrity` |
    |---|---|
    | 2.1.326 | `sha512-F7OqxXKogQYUs+RN1903IWlXGuxn8iwAykx+umRbDKvoBeK0Xm6cdgQx9auu5dtsB+DczN58/EFrlaU8pLfgJg==` |
    | 2.1.311 | `sha512-7SgBitii1ZscdbRJXLcub7Vv2pfvH3wo7RrCKg4vZpklIfz9HgigJFFJGIjCY6jzoPDvbQcwwugvm3A8PTejzw==` |

  - **Registry signatures:** keyid `SHA256:DhQ8wR5APBvFHLF/+Tc+AYvPOdTpcIDqOhxsBHRwC7U`, the current key (no expiry) at `/-/npm/v1/keys`. `npm audit signatures` verifies registry signatures.
  - **No provenance:**
    - The attestations endpoint returns 404 for `@anthropic-ai/claude-code@2.1.326`, `@2.1.311` and `@anthropic-ai/claude-code-linux-x64@2.1.326`, and no version has `dist.attestations`.
    - `_npmUser` shows a GitHub Actions trusted publisher (OIDC).
    - No Claude Code doc mentions provenance.
- **Install with the native installer:**
  - `curl -fsSL https://claude.ai/install.sh | bash -s <version|stable|latest>`. The URL redirects (302) to `https://downloads.claude.ai/claude-code-releases/bootstrap.sh`. Native installs auto-update unless `DISABLE_AUTOUPDATER` or `DISABLE_UPDATES` is set.
  - **bootstrap.sh always downloads the latest binary** and checks it with SHA-256 against `manifest.json`, fetched over HTTPS. It does no GPG check, then runs `<binary> install <target>`. A pinned install therefore executes the current latest binary first.
  - **Per release:**
    - `…/claude-code-releases/<VERSION>/manifest.json` gives `platforms.<platform>.{binary, checksum, size}`, plus the undocumented `manifestSignatureEnforcement: "flag"` and `sdkCompat`.
    - A detached `manifest.json.sig` exists for 2.1.89 and later.
    - Key: `https://downloads.claude.ai/keys/claude-code.asc`, fingerprint `31DD DE24 DDFA B679 F42D 7BD2 BAA9 29FF 1A7E CACE`.
    - Verify with `gpg --verify manifest.json.sig manifest.json`, then sha256 the binary.
    - **Checked 2026-11-17:** good signatures on the 2.1.326 and 2.1.311 manifests.
  - From the bootstrap.sh source, not documented: the binary is at `…/<VERSION>/<platform>/claude`. linux-x64 sha256 is `b6b3647d981130dc44fc670846ae97b6cde531ffb667cf9c4d17cfcafb38cb37` for 2.1.326 (commit `e601e33ab7cb`, built 2026-11-16T17:26:18Z) and `86dc1fcc69498159eaf15f446613e847010f9036bfbbdffd4c49350be8489ada` for 2.1.311.
  - **Platform signing:** Linux binaries aren't code-signed. macOS binaries are signed by Anthropic PBC and notarized. Windows binaries are Authenticode-signed.
  - Signed apt, dnf and apk repositories offer `stable` and `latest` channels under `https://downloads.claude.ai/claude-code/`. apk uses `claude-code.rsa.pub`.
  - GitHub releases carry eight platform archives plus `SHASUMS256.txt` and `SHASUMS256.txt.sig`. For v2.1.326 the API asset digests match `SHASUMS256.txt`. Not documented: `SHASUMS256.txt.sig` checked as a good signature from the same release key.
- **Network hosts:**

  | Host | Used for |
  |---|---|
  | `api.anthropic.com` | API requests, feature flags, telemetry event logging, WebFetch safety check |
  | `downloads.claude.ai` | native installer, auto-updater, update checks, plugin executable downloads |
  | `registry.npmjs.org` | npm installs of Claude Code, npm-source plugins, plugins' Node dependencies |
  | `github.com` | cloning marketplaces and plugins, the official marketplace included |
  | `storage.googleapis.com` | `/plugin` install counts and metadata |
  | `raw.githubusercontent.com` | changelog; never fetched by non-interactive sessions |
  | `http-intake.logs.us5.datadoghq.com`, `browser-intake-us5-datadoghq.com` | optional telemetry and error reports |

- **Turning traffic off:**

  | Variable | Effect |
  |---|---|
  | `CLAUDE_CODE_DISABLE_NONESSENTIAL_TRAFFIC` | Any non-empty value, even `0` or `false`. Turns off auto-updates, telemetry, error reporting, `/feedback`, release notes, status-badge and availability checks, feature flags, background `command`-source runs, and the refresh before a `name@marketplace` install. **Doesn't cover** official-marketplace auto-install or the WebFetch preflight (`skipWebFetchPreflight: true`) |
  | `DISABLE_TELEMETRY` | Any non-empty value; also stops feature-flag fetching |
  | `DO_NOT_TRACK=1` | Standard boolean; same effect as `DISABLE_TELEMETRY` |
  | `DISABLE_ERROR_REPORTING` | Any non-empty value |
  | `CLAUDE_CODE_DISABLE_FEATURE_FLAGS=1` | 2.1.319+. `DISABLE_GROWTHBOOK` is the name 2.1.124–2.1.318 read, and it still works |
  | `DISABLE_AUTOUPDATER=1` / `DISABLE_UPDATES=1` | Background updates only / also blocks `claude update` and `claude install` |
  | `DISABLE_FEEDBACK_COMMAND=1` | `/feedback`, `/bug`, `/share` |
  | `CLAUDE_CODE_DISABLE_OFFICIAL_MARKETPLACE_AUTOINSTALL=1` | Skips registering `claude-plugins-official`, which usually happens at the first interactive launch. The skip is permanent |
  | `CLAUDE_CODE_PLUGIN_PREFER_HTTPS=1` | Clones `owner/repo` over HTTPS; the docs name CI runners |
  | `CLAUDE_CODE_PLUGIN_CACHE_DIR`, `CLAUDE_CONFIG_DIR` | Move the plugins root (`~/.claude/plugins`) and the config directory (`~/.claude`) |

  - Error reports are on by default only for Pro, Max or Ultra sign-ins on 2.1.198+.
- **Not documented (UNVERIFIED):**
  - whether `claude plugin validate` runs with no network at all, and which startup requests (feature flags, telemetry, update checks) a `claude plugin` subcommand makes when no one is signed in;
  - whether non-interactive `claude plugin` subcommands trigger the official-marketplace auto-registration (set the variable anyway);
  - whether the native binary's `install` step verifies `manifest.json.sig`;
  - the per-platform binary URL, which comes from the bootstrap.sh source and not from the docs.

### CODEX-P4: releases, AGENTS.md and project trust
Sources: https://api.github.com/repos/openai/codex/releases and …/releases/latest (fetched 2026-11-17); https://registry.npmjs.org/-/package/@openai/codex/dist-tags (fetched 2026-11-17); source at https://github.com/openai/codex/tree/d32063f96c2dff1dc96af839b3952abb6acfeda4 (`rust-v0.169.0`), diffed against https://github.com/openai/codex/tree/57da98c18f84311dc23abfe1cda8f2c0dbd41cb9 (`rust-v0.171.0-alpha.11`). Files: `codex-rs/core/src/agents_md.rs`, `codex-rs/codex-home/src/instructions/`, `codex-rs/config/src/loader/mod.rs`, `codex-rs/config/src/config_toml.rs`, `codex-rs/config/src/state.rs`, `codex-rs/config/src/strict_config.rs`, `codex-rs/tui/src/onboarding/trust_directory.rs`. Docs (unversioned, no Last-Modified, fetched 2026-11-17): https://learn.chatgpt.com/docs/agent-configuration/agents-md.md, …/config-file/config-advanced.md, …/config-file/config-reference.md, …/non-interactive-mode.md, …/import.md.
Status: Verified, except the gaps listed. An independent skeptic re-checked 100 researcher facts against the source and docs: 93 confirmed, 6 refuted, 1 uncertain. The refuted facts appear below in corrected form, and facts the skeptic added are included where an implementer needs them. Nothing was downloaded as a binary or run. Source paths below are relative to `codex-rs/` at `rust-v0.169.0`. Each file named was compared with alpha.11 and is identical unless noted.

- **Releases checked:**

  | Channel | Version | Published | Tag commit |
  |---|---|---|---|
  | Stable (GitHub `releases/latest`; npm `latest`) | `rust-v0.169.0` / 0.169.0 | 2026-11-13 | `d32063f96c2d` |
  | Prerelease (npm `alpha`) | `rust-v0.171.0-alpha.11` / 0.171.0-alpha.11 | 2026-11-17 | `57da98c18f84` |
  | Patch prerelease | `rust-v0.170.0-alpha.4.1` | 2026-11-17 | `9e07c55808e4` |

  The prerelease supersedes the `alpha.10` recorded on 2026-11-16. The stable release is unchanged.
- **Global instructions:** `$CODEX_HOME` (default `~/.codex`) `AGENTS.override.md`, otherwise `AGENTS.md`. The first non-empty file wins, so a whitespace-only override falls back. This text is user instructions and doesn't count against the project budget.
- **Project walk** (`core/src/agents_md.rs`):
  - The root is the nearest ancestor holding a `project_root_markers` entry (default `.git`). With no marker found, only cwd is checked. Markers are read from non-project layers only.
  - In each directory from root to cwd, Codex takes the first file that **exists** among `AGENTS.override.md`, `AGENTS.md` and `project_doc_fallback_filenames` (default empty).
  - An empty override still claims its directory, so that directory's `AGENTS.md` is not read. The docs only say that empty files are skipped.
  - Files are joined with blank lines. `\n\n--- project-doc ---\n\n` separates user or internal text from the first project file.
- **Size:** `project_doc_max_bytes` defaults to `32 * 1024`, and `0` disables project files.
  - **One budget is shared by all project files and all turn environments.** The file that overflows is truncated, and later files are skipped, each with a warning.
  - This settles the docs conflict. agents-md.md describes a combined cap, which is right. config-advanced.md describes a per-file cap, which is wrong.
- **Instruction cross-reads:** Codex never reads `CLAUDE.md` unless a user lists it in `project_doc_fallback_filenames`.
- **Trust storage and lookup:**
  - Trust is stored as `[projects."<abs path>"] trust_level = "trusted" | "untrusted"` in user config, and managed config can also set it. Project layers can't.
  - For each `.codex/` folder, the first saved decision wins, checked in this order: the folder's parent directory, the project root, then the git repo root. The repo root is the main checkout for linked worktrees.
  - A root entry therefore covers nested folders that have no entry of their own. Windows keys compare case-insensitively. A new grant is saved under the repo-root key, or the project-root key when there is no repo.
- **What trust gates:**
  - **Project `.codex/` layers load only with an explicit `trusted`.** With no decision, or with `untrusted`, the layer is disabled: its `config.toml`, MCP servers, hooks, agents and exec policies don't load. The disabled reason tells the user to add the key as a trusted project in their user config.
  - **Project `AGENTS.md` files are skipped only when the project is explicitly `untrusted`.** A project with no decision still loads them. This corrects the 2026-11-16 entry.
  - Skills load in every case (`CODEX-SKILLS-P4`).
  - The TUI asks "Trust this folder?" and offers "Trust and continue", which saves the decision. For a folder already marked untrusted it offers "Open restricted", which leaves the saved trust unchanged.
  - `codex exec` and `codex review` load project config and project MCP servers only in trusted projects, even with `--sandbox workspace-write` (0.169.0, #60041).
  - `--trust` saves trust. It can't override an explicit `untrusted` and can't be combined with `--ignore-user-config`.
  - The docs warn that project MCP servers can run commands outside the sandbox.
- **Project config loading:**
  - A `.codex/` folder without `config.toml` is still a layer, so its `hooks.json`, `agents/` and `skills/` count.
  - In a trusted project, a TOML syntax error in `.codex/config.toml` is a config-load error ("Error parsing project config file"). In an untrusted project the file is ignored.
  - Even in a trusted project, Codex removes these keys from project config and prints a startup warning: `openai_base_url`, `chatgpt_base_url`, `apps_mcp_product_sku`, `model_provider`, `model_providers`, `notify`, `profile`, `profiles`, `experimental_realtime_ws_base_url` and `otel`. That is the docs list. The source also removes `responses_api_metadata`, `experimental_realtime_webrtc_call_base_url` and some `features.*` keys.
  - Outside strict mode, unknown keys in the merged config cause a startup warning. A value that fails a typed table, though, fails the load (`CODEX-MCP-P4`, `CODEX-HOOKS-P4`).
- **Claude Code cross-reads:** none at runtime.
  - Nothing reads `CLAUDE.md`, `.claude/skills/`, `.claude/agents/`, a repo-root `.mcp.json` or `.claude/settings.json` hooks. `.mcp.json` is read only as a plugin's default MCP file.
  - `/import` in the CLI, or Settings > Import in the desktop app, converts a Claude Code or Cursor setup once. It covers instruction files (to `AGENTS.md`), `settings.json` (to `config.toml`), skills, MCP config, hooks, slash commands (to skills) and subagents.

### CODEX-MCP-P4: `[mcp_servers.<id>]` in `.codex/config.toml`
Sources: `config/src/mcp_types.rs`, `config/src/config_toml.rs`, `rmcp-client/src/utils.rs`, `rmcp-client/src/stdio_server_launcher.rs`, `rmcp-client/src/program_resolver.rs`, `utils/pty/src/child_command.rs`, `codex-mcp/src/rmcp_client.rs`, `codex-mcp/src/tools.rs`, `codex-mcp/src/mcp/mod.rs`, `core/src/tools/handlers/mcp.rs`, `protocol/src/shell_environment.rs` and `network-proxy/src/certs.rs` at `rust-v0.169.0`; `codex-mcp/src/tools.rs` and `protocol/src/shell_environment.rs` at alpha.11. Docs: https://learn.chatgpt.com/docs/extend/mcp.md and …/config-file/config-reference.md (fetched 2026-11-17).
Status: Verified, except the gaps listed.

- **Transport is chosen by key.** `command` selects stdio. Otherwise `url` selects Streamable HTTP. With neither, the entry is an error ("invalid transport").
- **Keys by transport:**

  | Transport | Keys |
  |---|---|
  | stdio | `command` (string), `args` (array of strings), `env` (string map, literal), `env_vars` (array), `cwd` (string) |
  | Streamable HTTP | `url`, `bearer_token_env_var`, `http_headers` (static map), `env_http_headers` (header → variable name), `http_headers_helper` (local command that prints a JSON object; must not be empty; local environment only), `auth` (`oauth` default, `chatgpt`, `ema_auth`), `oauth.{client_id, client_secret, token_endpoint_auth_method, callback_url, callback_port, authorization_server_issuer}`, `oauth_resource` (RFC 8707). An inline `bearer_token` is rejected. |
  | both | `environment_id` (default `local`), `enabled`, `required`, `startup_readiness` (`connection` or `catalog`), `startup_timeout_sec` (float **seconds**, docs default 10) or `startup_timeout_ms`, `tool_timeout_sec` (float seconds, docs default 60), `supports_parallel_tool_calls`, `tool_input_schema_max_bytes` (default 5,000), `omit_tools_from`, `default_tools_approval_mode` (`auto`, `prompt`, `writes` or `approve`), `enabled_tools`, `disabled_tools`, `tools.<tool>.approval_mode`, `tools.<tool>.output_token_limit`, `scopes`, and a legacy `name` |

- **Mixing transports is an error** ("<field> is not supported for <transport>"):
  - On a stdio entry, these keys fail: `url`, `bearer_token_env_var`, `bearer_token`, `http_headers`, `env_http_headers`, `http_headers_helper`, `oauth`, `oauth_resource` and `auth`.
  - On an HTTP entry, these keys fail: `args`, `env`, `env_vars`, `cwd` and `bearer_token`.
  - *(source, not exercised)* `mcp_servers` belongs to the typed config, so one invalid entry fails config loading, not just that server.
- **`env_vars`:** each item is a variable-name string or `{ name = "…", source = "local" | "remote" }`. The table form takes no other keys, and any other source is an error. A string means local.
  - Each named variable is copied from Codex's own environment under the same name. An unset variable is skipped silently.
  - `source = "remote"` makes a local stdio launch fail.
  - A variable can't be forwarded under a different name.
- **Stdio environment:** the child starts empty (`env_clear`). It then receives, in this order:
  1. the default allowlist;
  2. the `env_vars` names;
  3. any non-empty custom-CA variables, made absolute: `CODEX_CA_CERTIFICATE`, `SSL_CERT_FILE`, `REQUESTS_CA_BUNDLE`, `CURL_CA_BUNDLE`, `NODE_EXTRA_CA_CERTS`, `GIT_SSL_CAINFO`, `CARGO_HTTP_CAINFO`, `PIP_CERT`, `BUNDLE_SSL_CA_CERT`, `npm_config_cafile`, `NPM_CONFIG_CAFILE`;
  4. the literal `env` map, which overrides the rest.

  Non-inheritable names are removed last, even when they come from `env`.
  - Unix allowlist: `HOME`, `LOGNAME`, `PATH`, `SHELL`, `USER`, `__CF_USER_TEXT_ENCODING`, `LANG`, `LC_ALL`, `TERM`, `TMPDIR`, `TZ`.
  - Windows allowlist: `PATH`, `PATHEXT`, `SHELL`, `COMSPEC`, `SYSTEMROOT`, `WINDIR`, `SYSTEMDRIVE`, `USERNAME`, `USERDOMAIN`, `USERPROFILE`, `HOMEDRIVE`, `HOMEPATH`, `PROGRAMFILES`, `PROGRAMFILES(X86)`, `PROGRAMW6432`, `PROGRAMDATA`, `LOCALAPPDATA`, `APPDATA`, `TEMP`, `TMP`, `TMPDIR`, `POWERSHELL`, `PWSH`. (Corrected: the research list missed two names.)
  - Non-inheritable names (case-insensitive; also stripped from hooks): `CODEX_EXEC_SERVER_NOISE_AUTH_TOKEN`, `NODE_REPL_AUTH_TOKEN`, `CODEX_GUARDIAN_DECISIONS_API_KEY`, `OPENAI_FEDERATION_RULE_ID`, `OPENAI_IDENTITY_TOKEN_FILE`, `OPENAI_WORKLOAD_IDENTITY_CONTEXT`. alpha.11 adds `OPENAI_IDENTITY_TOKEN_SOURCE`, `OPENAI_SPIFFE_AUDIENCE` and `OPENAI_SPIFFE_ID`.
- **No `${VAR}` interpolation** in user or project MCP config. The config, codex-mcp and rmcp-client crates have none, and the docs document none. Only a plugin's MCP `cwd` accepts `${PLUGIN_ROOT}` and `${PLUGIN_DATA}`.
- **Working directory and program lookup:**
  - Without `cwd`, a stdio server starts in the launcher's fallback directory. A doc comment describes it as the caller's runtime working directory.
  - (Corrected) On Windows the program is resolved with `which` against the server's own `PATH`/`PATHEXT` and that cwd. On Unix it is passed to spawn unchanged.
- **HTTP credentials:**
  - If the variable named by `bearer_token_env_var` is unset, empty or not Unicode, that server fails to start.
  - An `env_http_headers` entry whose variable is unset or blank is dropped silently. So are invalid header names and values.
  - The docs list bearer tokens, OAuth (including CIMD and DCR) and ChatGPT-session auth for first-party origins.
  - Users sign in with `codex mcp login <name>`, and `--no-browser` is available. With no credential, Codex may connect unauthenticated.
- **Server and tool names:**
  - Server names must match `^[a-zA-Z0-9_:@/.-]+$`. A server with any other name fails to start.
  - Model-visible and hook-visible tool names replace every character outside `[A-Za-z0-9_]` with `_`. For example, server `docs-index` with tool `get.page` becomes `mcp__docs_index__get_page`.
  - Names that collide or exceed 128 bytes get `_` plus 12 hex digits of SHA-1. Hook names always carry `mcp__`.
  - alpha.11 only: a server in a non-default environment gets a 12-hex hash of the environment id before the server name.
- **Required servers:** with `required = true`, startup fails, and `codex exec` exits with an error, if the server isn't ready. `startup_readiness = "catalog"` lets a valid cached catalog count as ready.
- **Managed allowlist:** a `requirements.toml` `mcp_servers` table is matched on server name and identity (command or url). Any configured server not on it is disabled.
- **UNVERIFIED:**
  - whether unknown keys inside `[mcp_servers.<id>]` trigger the ignored-key startup warning;
  - whether every frontend (TUI, `exec`, app server) refuses to start on an invalid entry, as the source suggests;
  - the exact fallback directory for a stdio server.

### CODEX-SKILLS-P4: skills
Sources: `ext/skills/src/host_roots.rs`, `ext/skills/src/loader/host.rs`, `ext/skills/src/loader/host_merge.rs`, `skills/src/parser.rs`, `skills/src/lib.rs` and `tui/src/onboarding/trust_directory.rs` at `rust-v0.169.0`; https://learn.chatgpt.com/docs/build-skills.md (fetched 2026-11-17).
Status: Verified, except the gaps listed.

- **Roots:**

  | Root | Scope |
  |---|---|
  | `.agents/skills` in each directory from cwd up to the project root | Repo |
  | `<project>/.codex/skills` for each project layer | Repo (found only in source) |
  | `$CODEX_HOME/skills` | User (deprecated) |
  | `$HOME/.agents/skills` | User |
  | `$CODEX_HOME/skills/.system` | System (bundled) |
  | `/etc/codex/skills` (the system layer folder) | Admin |
  | enabled plugins' skill roots | Plugin |

  Duplicate root paths are removed. `.claude/skills/` is never read.
- **Skills load in untrusted projects.** Roots come from every config layer, disabled ones included, and the `.agents/skills` walk ignores trust. The TUI text for restricted folders says "Skills still load".
- **Frontmatter:**
  - The first line must be `---`, and a closing `---` must follow with at least one line between them.
  - Codex reads only `name`, `description` and `metadata.short-description`, and ignores other keys.
  - `name` is optional and defaults to the parent directory name. Whitespace is collapsed, and the result may be at most 64 characters. No character set is enforced.
  - `description` is required and must be non-empty.
  - If the YAML fails to parse, Codex retries once after quoting unquoted `key: a: b` values.
- **`agents/openai.yaml`** (docs): `interface.{display_name, short_description, icon_small, icon_large, brand_color, default_prompt}`, `policy.allow_implicit_invocation` (default `true`) and `dependencies.tools[]`.
- **Duplicate names:** when two skills share a `name`, the docs say both are kept and both can appear in skill selectors. In source, deduplication is by canonical `SKILL.md` path only.
- **Configuration:**
  - Codex follows symlinked skill folders.
  - Each `[[skills.config]]` entry takes exactly one selector, an absolute `path` or a `name`, plus `enabled = false`. The last matching entry wins.
  - `[skills.bundled] enabled = false` in a trusted project config turns off the bundled skills.
- **UNVERIFIED:** whether two skills with the same name are both offered for implicit invocation.

### CODEX-SUBAGENTS-P4: custom agents
Sources: `agent-roles/src/discovery.rs`, `agent-roles/src/loader.rs`, `agent-roles/src/agent_role_config.rs`, `core/src/agent/role.rs`, `core/src/tools/spec_plan.rs`, `core/src/tools/handlers/multi_agents_spec.rs`, `features/src/lib.rs` and `config/src/state.rs` at `rust-v0.169.0`; https://learn.chatgpt.com/docs/agent-configuration/subagents.md (fetched 2026-11-17).
Status: Verified, except the gaps listed.

- **Files:** every `*.toml` file under `<layer folder>/agents/`, subdirectories included.
  - That covers project `.codex/agents/` (trusted layers only), `~/.codex/agents/` and the system folder (`/etc/codex/agents/`). Corrected: the research listed only the first two.
  - `[agents.<name>] config_file = "…"` in a config.toml also declares an agent.
- **Required fields:** `name`, `description` and `developer_instructions`.
  - `name` must be non-empty. Codex identifies the agent by it, not by the filename.
  - `description` and `developer_instructions` must be non-blank.
  - Optional `nickname_candidates` must be non-empty and unique, using ASCII letters, digits, spaces, `-` and `_`.
- **Malformed files:** every other top-level key must be a config.toml key, because the file is parsed with serde `deny_unknown_fields`. A malformed file is skipped with a startup warning beginning "Ignoring malformed agent role definition".
- **Same name:** within one layer, the later file is skipped with a warning. A higher layer overrides a lower one and inherits its missing fields. A custom `default`, `explorer` or `worker` replaces the built-in agent of that name.
- **Overrides that take effect:** `developer_instructions`, `model`, `model_reasoning_effort`, `model_reasoning_summary`, `model_verbosity`, `personality` and `service_tier`; `features.<key> = false` for `shell_tool`, `apps`, `plugins`, `memories` or `request_permissions_tool`; `skills.config` entries with `enabled = false`, `skills.bundled.enabled = false` and `skills.include_instructions = false`.
- **Everything else is ignored silently.** That includes `sandbox_mode`, `approval_policy`, `mcp_servers`, `model_provider`, `hooks`, `web_search` and any feature set to `true`. The child inherits the parent turn's sandbox, approvals, provider and MCP servers.
- **What `features.shell_tool = false` removes:** only `exec_command` and `write_stdin`. `apply_patch`, `view_image`, MCP tools, MCP resource tools, `web_search` and the agent tools stay.
- **A read-and-search-only agent can't be expressed** (an inference from the tool plan). Reading and searching go through `exec_command`. Disabling it still leaves `apply_patch`, and an agent file can't set a read-only sandbox. The docs confirm that agent instructions don't create a separate read-only sandbox.
- **Flags:** none are needed. `agents.enabled` defaults to `true`. The `multi_agent` feature is Deprecated but on by default, and `multi_agent_v2` is Stable but off by default.
- **How the parent picks an agent:** through the `agent_type` parameter of `spawn_agent`.
  - The tool description lists "Available roles:" with each role's `description`, user-defined roles first. It notes any `model` or `model_reasoning_effort` that the role file fixes.
  - `agent_type` is offered only when at least one user-defined role exists.
  - The docs say local Codex spawns subagents only on a direct request, or when applicable AGENTS.md or skill instructions call for it.
- **Spawn-time validation:** a role file is re-validated as full config when the agent is spawned. On failure, the model gets "agent type is currently not available".
- **UNVERIFIED:** whether `service_tier` in an agent file takes effect. The source applies it, while the docs say the service tier follows the root session.

### CODEX-HOOKS-P4: hooks
Sources: `protocol/src/hooks.rs`, `config/src/hook_config.rs`, `config/src/fingerprint.rs`, `config/src/loader/mod.rs`, `hooks/src/engine/discovery.rs`, `hooks/src/engine/matcher.rs`, `hooks/src/engine/command_runner.rs`, `hooks/src/engine/output_parser.rs`, `hooks/src/engine/mod.rs`, `hooks/src/schema.rs`, `hooks/src/events/common.rs`, `hooks/src/events/pre_tool_use.rs`, `hooks/src/events/user_prompt_submit.rs`, `hooks/src/events/session_end.rs`, `hooks/src/config_rules.rs`, `hooks/src/lib.rs`, `hooks/src/registry.rs`, `hooks/schema/generated/pre-tool-use.command.input.schema.json`, `hooks/schema/generated/user-prompt-submit.command.input.schema.json`, `core/src/tools/hook_names.rs`, `core/src/tools/registry.rs`, `core/src/hook_runtime.rs`, `core/src/session/mod.rs`, `core/src/shell.rs`, `features/src/lib.rs` and `tui/src/startup_hooks_review.rs` at `rust-v0.169.0`. In alpha.11, `command_runner.rs`, `pre_tool_use.rs` and `user_prompt_submit.rs` changed, but only by adding structured content. Docs: https://learn.chatgpt.com/docs/hooks.md (fetched 2026-11-17).
Status: Verified, except the gaps listed.

- **Sources:** `hooks.json` and inline `[hooks]` next to each active config layer (`~/.codex/`, trusted `<repo>/.codex/` and the system folder), plus managed requirements and enabled plugins (default `hooks/hooks.json`).
  - All sources run, and higher layers don't replace lower ones.
  - A layer with both forms gets them merged, with a warning.
  - In a linked worktree, both `hooks.json` and `[hooks]` come from the matching `.codex/` in the main checkout, not from the worktree.
- **Shape:** `{"description"?: string, "hooks": {"<Event>": [{"matcher"?: string, "hooks": [handler, …]}]}}`. In TOML the same structure is `[[hooks.<Event>]]` followed by `[[hooks.<Event>.hooks]]`.
  - The top level uses `deny_unknown_fields`, so any other key makes the file fail to parse. The file is skipped with a warning.
  - Unknown event, group and handler keys in `hooks.json` are ignored silently.
  - An unknown handler `type`, or a field of the wrong type, fails the whole file. Examples: `command` as an array, or a negative `timeout`.
  - *(source, not exercised)* Inline `[hooks]` is part of the typed config, so the same mistakes there fail config loading.
- **Events (14):** `PreModelRequest`, `PostModelRequest`, `PreToolUse`, `PermissionRequest`, `PostToolUse`, `PreCompact`, `PostCompact`, `SessionStart`, `SessionEnd`, `UserPromptSubmit`, `SubagentStart`, `SubagentStop`, `Stop`, `Interrupt`.
- **Handlers:**

  | `type` | Fields |
  |---|---|
  | `command` | `command` (one string; required; not blank); `commandWindows`, alias `command_windows` (replaces `command` on Windows); `timeout` (integer **seconds**, default 600, minimum 1; SessionEnd and Interrupt default to 1 and are clamped to 1–3 with a warning); `async` (bool); `statusMessage`; `additionalContextLimit` (approximate tokens, default 2,500, `0` removes the cap; honoured only on PreToolUse, PostToolUse, SessionStart, UserPromptSubmit and SubagentStart) |
  | `mcp_tool` | `server` and `tool` (both non-empty); `input` (object with `${field.path}` placeholders; no nulls); `timeout`; `statusMessage`. Not supported on SessionEnd. |
  | `prompt`, `agent` | Parsed, then skipped with a warning. |

  **There is no `args`, argv or exec form, and no `onFailure`.** Claude-style keys of that kind are dropped silently.
- **Execution:**
  - A hook always runs through a shell. With a session shell, that is bash, zsh or sh as `-c <command>` (not a login shell), PowerShell as `-NoProfile -Command <command>`, or cmd as `/c`.
  - The fallback is `$SHELL -lc` (else `/bin/sh`) on Unix, and `%COMSPEC% /C` (else `cmd.exe`) on Windows.
  - Under cmd `/C`, the command string is passed raw inside one extra pair of `"`.
  - The working directory is the session cwd. The docs suggest locating repo hooks from the git root.
  - Up to 8 async hooks run at once. Unix hooks run in a new session.
- **Environment and I/O:**
  - A hook inherits the environment Codex had when the session started, minus the non-inheritable names.
  - Plugin hooks also get `PLUGIN_ROOT`, `PLUGIN_DATA`, `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA`. `${KEY}` for exactly those four names is replaced as text in the command. User and project hooks get no substitution.
  - stdin receives the whole input JSON, and no size cap was found. stdout and stderr are read as lossy UTF-8.
  - The timeout also covers writing stdin. On timeout, Codex kills the process group on Unix, or the job on Windows.
- **Matchers:**
  - An omitted matcher, `""` or `"*"` matches everything.
  - A matcher made only of `[A-Za-z0-9_|]` is an exact list separated by `|`. A `-` is not in that set, so it makes the matcher a regex.
  - Anything else is a Rust `regex` and is **not anchored** (`is_match`). An invalid regex skips its group with a warning.
  - Which events use the matcher, and what it is tested against:

    | Event | Matcher tested against |
    |---|---|
    | PreToolUse, PermissionRequest, PostToolUse | tool name and its aliases |
    | SessionStart | `startup`, `resume`, `clear`, `compact` or `fork` |
    | SessionEnd | always `other` |
    | SubagentStart, SubagentStop | `agent_type` |
    | PreCompact, PostCompact | `manual` or `auto` |
    | PreModelRequest, PostModelRequest, UserPromptSubmit, Stop, Interrupt | matcher ignored |

- **Tool names that hooks see.** stdin always carries the canonical name, never an alias.

  | Codex tool | Hook `tool_name` | Also matched by | `tool_input` |
  |---|---|---|---|
  | `exec_command` | `Bash` | – | `{"command": "<cmd string>"}`; stdin `cwd` = the resolved workdir |
  | `write_stdin` | no PreToolUse; PostToolUse as `Bash`; PermissionRequest as `write_stdin` | – | – |
  | `apply_patch` | `apply_patch` | `Edit`, `Write` | `{"command": "<raw patch>"}` in the `*** Begin Patch` grammar, with no `file_path` or `content` |
  | MCP tool | `mcp__<server>__<tool>`, sanitized | – | the argument object |
  | `spawn_agent` | `spawn_agent` | `Agent` | the argument object |
  | other default-namespace tools (`update_plan`, `request_user_input`, `tool_search`, `view_image`, `list_mcp_resources`, `list_mcp_resource_templates`, `read_mcp_resource`) | plain name | – | the argument object; `{}` when empty; the raw string when not JSON |
  | hosted `web_search` | never reaches tool hooks | – | – |

  An `updatedInput` for a shell or patch call must contain a string `command`.
- **PreToolUse stdin:**
  - Required: `session_id`, `turn_id`, `transcript_path` (string or null), `cwd`, `hook_event_name`, `model`, `permission_mode` (`default`, `acceptEdits`, `plan`, `dontAsk` or `bypassPermissions`), `tool_name`, `tool_use_id` and `tool_input`. Optional: `agent_id` and `agent_type`. The schema sets `additionalProperties: false`.
  - A subagent's hooks carry the parent's `session_id`.
- **UserPromptSubmit stdin:** the same common fields plus `prompt` (string), without the tool fields.
- **How a PreToolUse hook blocks.** Only synchronous hooks can block, and any matching hook's block wins:
  - exit 2 with non-empty stderr, which becomes the reason;
  - exit 0 with `hookSpecificOutput{hookEventName: "PreToolUse", permissionDecision: "deny", permissionDecisionReason: <non-empty>}`;
  - the legacy exit-0 form `{"decision": "block", "reason": <non-empty>}`.

  The model then sees "Command blocked by PreToolUse hook: <reason>. Command: <command>" for shell and patch calls, or "Tool call blocked by PreToolUse hook: <reason>. Tool: <name>" for other tools.
- **PreToolUse outputs that mark the hook failed and let the tool run:**
  - exit 2 with empty stderr;
  - `deny` without a non-empty reason;
  - `ask`;
  - `allow` without `updatedInput`, or `updatedInput` without `allow`;
  - legacy `approve`;
  - `continue: false`, `stopReason` or `suppressOutput`;
  - JSON with unknown fields, such as Claude's top-level `additionalContext`, or a `hookSpecificOutput` without `hookEventName`;
  - stdout that starts with `{` or `[` but doesn't parse;
  - any other exit code, or no exit status.

  Plain non-JSON stdout on exit 0 is ignored.
- **UserPromptSubmit:** it blocks on exit 2 with non-empty stderr, on `{"decision": "block", "reason": <non-empty>}`, or on `continue: false`. On exit 0, plain stdout becomes model context.
- **Hooks fail open, and no fail-closed option exists** for user, project and plugin hooks.
  - These all let the tool call or prompt go ahead: a spawn error; a missing binary, which shows up as a shell exit such as 127; a crash; a timeout; and an input serialization failure.
  - Managed `requirements.toml` hooks that fail to *load* stop hook setup.
  - The docs describe tool hooks as a guardrail rather than a full enforcement boundary.
- **Coverage gaps:**
  - Hosted tools such as `web_search` never reach tool hooks.
  - `write_stdin` doesn't run PreToolUse when it sends input or polls. A command typed into an interactive shell that `exec_command` opened is therefore unseen.
  - An `apply_patch` typed through `exec_command` is intercepted inside that handler. PreToolUse sees it only as `Bash`, because `core/src/tools/registry.rs` is the only PreToolUse caller.
- **Trust:**
  - A non-managed hook (user, project, session or plugin) runs only if it is enabled and its stored `trusted_hash` equals its current hash. New hooks are Untrusted and changed hooks are Modified. Both are listed but skipped.
  - Trust is stored only in user config or session flags, as `[hooks.state."<abs source file>:<event_key>:<group>:<handler>"]` with `trusted_hash = "sha256:…"` and an optional `enabled`. Event keys are snake_case, such as `pre_tool_use`. **A repo can't trust its own hooks.**
  - The hash is SHA-256 over sorted JSON of the event, the matcher and the normalized handler: the command selected for the platform, the normalized timeout, `async`, `statusMessage` and `additionalContextLimit`.
  - The state key includes the group and handler positions. Editing a hook, or inserting a group before it, therefore forces another review.
  - At startup the TUI shows "Hooks need review", with the choices "Review hooks", "Trust all and continue" and "Continue without trusting (hooks won't run)". `/hooks` reviews hooks individually.
  - `--dangerously-bypass-hook-trust` runs enabled hooks for one invocation without persisted trust.
  - System, MDM, cloud and `requirements.toml` hooks are managed and trusted by policy. `allow_managed_hooks_only = true` skips user, project, session and plugin hooks.
- **Flags:** the `[features] hooks` feature is Stable and on by default. `codex_hooks` is a deprecated alias, and `requirements.toml` can force the value.
- **Output cap:** model-visible hook output is limited to about 2,500 tokens. The full text is saved under `<temp>/hook_outputs/<session_id>/<uuid>.txt`, and the model gets a preview.
- **PermissionRequest:** a hook decides with `hookSpecificOutput.decision.behavior = "allow" | "deny"`, plus an optional `message` for deny. Any deny wins. Returning `updatedInput`, `updatedPermissions` or `interrupt` fails closed.
- **UNVERIFIED:**
  - whether invalid inline `[hooks]` aborts startup in every frontend;
  - whether unknown keys in inline `[hooks]` produce the ignored-key warning;
  - how cmd.exe and PowerShell actually quote a binary path with spaces plus arguments (the source was read, nothing was run);
  - any operating-system limit on hook stdin size.

### CODEX-TOOLS-P4: built-in tools for agent and hook mapping
Sources: `core/src/tools/spec_plan.rs`, `core/src/tools/handlers/shell_spec.rs`, `core/src/tools/handlers/unified_exec.rs`, `core/src/tools/handlers/apply_patch_spec.rs`, `core/src/tools/handlers/mcp_resource_spec.rs`, `core/src/tools/handlers/multi_agents_spec.rs`, `core/src/tools/handlers/plan_spec.rs`, `core/src/tools/mod.rs`, `core/src/tools/registry.rs` and `protocol/src/config_types.rs` at `rust-v0.169.0`; https://learn.chatgpt.com/docs/config-file/config-reference.md (fetched 2026-11-17).
Status: Verified, except the gaps listed.

| Grimoire capability | Codex tool | Notes |
|---|---|---|
| `read`, `search` | `exec_command` | No dedicated read, grep, glob or list tool exists. `view_image` covers local images. With MCP servers configured, `list_mcp_resources`, `list_mcp_resource_templates` and `read_mcp_resource` are added. |
| `edit` | `apply_patch` | A freeform tool with a Lark grammar, registered whenever the model defines `apply_patch_tool_type`, whatever the features say. |
| `execute` | `exec_command` + `write_stdin` | `exec_command` takes `cmd` plus optional `workdir`, `shell`, `login`, `tty`, `yield_time_ms` and `max_output_tokens`. `login` defaults to the config's `allow_login_shell`, which defaults to `true`. When managed requirements turn unified exec off, only a one-shot `exec_command` exists. |
| `web` | hosted `web_search` | (Corrected) `web_search = "disabled" \| "cached" \| "indexed" \| "live"`. The default is `cached`, and the docs say full-access sandbox settings default it to `live`. Not per agent, and never hooked. A standalone `web.run` is used when the model sets `use_responses_lite`, or when the `standalone_web_search` feature is on (UnderDevelopment, default off). |
| agents | `spawn_agent` | Hook alias `Agent`. |

- **Other tools:** `update_plan`, `request_user_input`, `tool_search`, and `request_permissions` (UnderDevelopment, default off).
- **Multi-agent namespaces:** the multi-agent tools sit in namespace `multi_agent_v1` (v1) or `collaboration` (v2): `send_input`, `send_message`, `followup_task`, `wait_agent`, `list_agents`, `close_agent`, `interrupt_agent` and `resume_agent`.
- **UNVERIFIED:** the hook-visible names of the namespaced multi-agent tools other than `spawn_agent`. `flat_tool_name` joins namespace and name with no separator, which would give names such as `multi_agent_v1send_input`. This was not exercised.

### CODEX-PLUGINS-P4: plugin and marketplace layouts
Sources: `core-plugins/src/marketplace.rs`, `utils/plugins/src/plugin_namespace.rs`, `exec-server-protocol/src/protocol.rs`, `core-plugins/src/loader.rs` and `hooks/src/engine/discovery.rs` at `rust-v0.169.0`; https://developers.openai.com/plugins/build/plugins.md (fetched 2026-11-17); https://agent-plugins.org/schemas/1.0.0/plugin.schema.json (Last-Modified 2026-11-13, fetched 2026-11-17).
Status: Verified, except the gaps listed.

- **Marketplace file:** under a marketplace root, Codex checks `.agents/plugins/marketplace.json`, `.agents/plugins/api_marketplace.json`, `.claude-plugin/marketplace.json` and `.cursor-plugin/marketplace.json`, in that order, and uses the first that exists. Corrected: the 2026-11-16 entry listed only two of these.
- **Documented locations:** the docs list three marketplace files for the desktop app: `$REPO_ROOT/.agents/plugins/marketplace.json`, the legacy-compatible `$REPO_ROOT/.claude-plugin/marketplace.json`, and `~/.agents/plugins/marketplace.json`.
- **Marketplace format:** `name`, optional `interface.displayName`, and `plugins[]` entries of `{name, source, policy, category, …other manifest fields}`.
  - `source` is a path string, or an object whose `source` is `local {path}`, `url {url, path?, ref?, sha?}`, `git-subdir {url, path, ref?, sha?}` or `npm {package, version?, registry?}`.
  - Other kinds, such as Claude's `github`, parse as unsupported.
  - A local path must be `.` or start with `./`, and may contain only normal components. Only under `.cursor-plugin/` may the `./` be omitted.
- **Plugin manifest:** a root `plugin.json` counts only when its `$schema` starts with `https://agent-plugins.org/schemas/`; 1.0.0 is the supported version. Otherwise Codex checks `.codex-plugin/plugin.json`, `.claude-plugin/plugin.json` and `.cursor-plugin/plugin.json`, in that order.
- **Agent Plugins 1.0.0 schema:**
  - Required: `$schema` (a constant) and `name` (1–64 characters, matching `^(?!.*(?:--|\.\.))[a-z0-9](?:[a-z0-9.-]*[a-z0-9])?$`).
  - Optional: `version`, `description`, `author{name, email, url}`, `homepage`, `repository`, `license`, `keywords[]` and `extensions`, an object of objects keyed by reverse-domain namespace.
  - `additionalProperties: false`.
- **Plugin parts:** plugin hooks default to `hooks/hooks.json` and receive `CLAUDE_PLUGIN_ROOT` and `CLAUDE_PLUGIN_DATA` (`CODEX-HOOKS-P4`). A plugin's default MCP file is `.mcp.json` inside the plugin root.
- **UNVERIFIED:** whether the CLI, as opposed to the desktop app, discovers `$REPO_ROOT/.claude-plugin/marketplace.json` without `codex plugin marketplace add`. CLI discovery scans caller-supplied roots, and the callers were not traced.

### CURSOR-P4: Cursor facts for Phase 4
Sources: https://cursor.com/docs/rules.md, …/mcp.md, …/skills.md, …/subagents.md, …/hooks.md, …/reference/third-party-hooks.md, …/reference/plugins.md, …/reference/permissions.md, …/agent/security.md, …/sdk/typescript.md, …/cli/using.md, …/cli/reference/parameters.md, …/cli/reference/permissions.md, …/cloud-agent/self-hosted/pool.md; https://cursor.com/help/customization/rules.md, …/mcp.md, …/skills.md, https://cursor.com/help/troubleshooting/minimum-version.md (all unversioned, fetched 2026-11-17). Release notes: https://cursor.com/docs/release-notes/rss.xml, …/release-notes/ide.md, …/release-notes.md, https://cursor.com/docs/cli/changelog.md (fetched 2026-11-17). https://cursor.com/api/download?platform=<p>&releaseTrack=<t> and https://cursor.com/install (read as text, not run), queried 2026-11-17. https://github.com/cursor/plugins at `e1cc03121a0e` (main, 2026-11-17). Status: Partial. An independent skeptic re-read every source. Of 94 facts, 89 were confirmed, 2 refuted (corrected below) and 3 left uncertain (listed under UNVERIFIED). This entry supersedes the 2026-11-16 CURSOR-* entries where they differ.

- **Releases checked (2026-11-17):**
  - Release notes: IDE **3.27** and CLI **v2026.10.22**, both 2026-10-22. Nothing newer for either. Same feed: Cursor SDK 1.2.0 (2026-10-27), Origin CLI (2026-11-03).
  - Shipped builds are newer than the notes. The download API returns IDE `3.30.14` (commit `5f54c8173e55`) on `releaseTrack=stable` and `latest` for linux-x64, darwin-arm64 and win32-x64. The CLI installer pins agent build `2026.11.13-eacade1` from `downloads.cursor.com/lab/`. No notes exist for IDE 3.28–3.30 or CLI 2026.11.13.
  - No prerelease channel is named. `prerelease`, `early-access`, `nightly`, `insiders` and `beta` all return HTTP 500 "Failed to fetch download link". Any unknown value does the same, so this proves nothing. CLI v2026.06.22 mentions "release channels" without naming them.
  - The docs are unversioned and don't say which build they describe. A fact with no version comes from the live docs.
  - Minimum versions (schedule as of 2026-10-26): allowed 3.6 and supported 3.18. Allowed becomes 3.12 on 2026-11-22 and 3.18 on 2026-12-14.
- **Rules (`.cursor/rules/`):**
  - Only `.mdc` files are rules. A plain `.md` file there is ignored. Subfolders work, though help recommends a flat folder.
  - Frontmatter has `description`, `globs` and `alwaysApply`:

    | `alwaysApply` | `description` | `globs` | Behaviour |
    |---|---|---|---|
    | `true` | – | – | Always included; `globs` and `description` ignored |
    | `false` | – | set | Attached when a matching file is in context |
    | `false` | set | unset | Agent decides from the description |
    | `false` | unset | unset | Only when @-mentioned |

  - `globs` in rules.md is a string, with patterns separated by commas (`docs/**/*.md, docs/**/*.mdx`). Its example is unquoted: `globs: src/components/**/*.tsx`. Only the plugin reference types `globs` as "string or array", for plugin `rules/`, with a quoted example `"**/*.ts"`.
  - Rules are identified by full path. Two rules with the same filename in different folders both apply.
  - Nested `.cursor/rules/` and `.cursor/skills/` in project subdirectories are discovered (CLI changelog May 14, 2026: "matching the IDE").
  - User rule files can also live in `~/.cursor/rules` (not synced). Precedence: Team → Project → User. All applicable rules are merged, and the earlier source wins on conflict. Keep rules under 500 lines. `.cursorrules` is legacy.
  - A file mention such as `@x.ts` in a rule is not inlined. The agent reads the file with its tools when it needs it.
- **`AGENTS.md` and `CLAUDE.md`:**
  - `AGENTS.md` is read at the root and in any subdirectory. A nested file is combined with its parents, and the more specific one wins. Since CLI v2026.10.22, editing or creating a file in a subdirectory brings in that directory's `AGENTS.md` and nested always-apply rules, each once per conversation.
  - `CLAUDE.md` is read "the same way" as `AGENTS.md` and is always applied, whatever its `alwaysApply`. The docs name only the project root, and the CLI reads both files at the root.
  - De-duplication: IDE 3.26 delivers the instructions once when `CLAUDE.md` is a symlink, a hard link or an identical copy of `AGENTS.md`. Cloud Agents (week of 2026-10-12) no longer get duplicate `AGENTS.md` or `CLAUDE.md` instructions, with no condition given.
- **MCP (`.cursor/mcp.json`):**
  - Files are `.cursor/mcp.json` and `~/.cursor/mcp.json`, each with top-level `mcpServers`. They are merged, and the project entry wins on a name clash.
  - **The root `.mcp.json` is not read.** The documented sources are those two files, plugins, team servers and the extension API (`vscode.cursor.mcp.registerServer()`). SDK `settingSources` are `project`, `user`, `team`, `mdm`, `plugins` and `all`.
  - stdio fields: `type` (`"stdio"`, marked required, though every example omits it), `command` (on PATH or a full path), `args`, `env`, `envFile` (stdio only).
  - Remote fields: `url`, `headers`, and `auth{CLIENT_ID (required), CLIENT_SECRET, scopes: [..]}`. Supported transports: stdio, SSE and Streamable HTTP. mcp.md and help never show `type` on a `url` server.
  - Evidence on `type`:
    - The plugin reference says Cursor Plugins "infer the transport from `command` or `url`".
    - The SDK `McpServerConfig` has `type?: "stdio"` (`command`, `args`, `env`, `cwd`, with `cwd` local only) or `type?: "http" | "sse"` (`url`, `headers`, `auth`). It has no `streamable-http`.
    - cursor/plugins @ `e1cc0312` has 121 entries. Of 119 remote entries, 118 have `"type": "http"` and 1 has none. Of 2 stdio entries, 1 has `"type": "stdio"` and 1 has none.
  - Interpolation applies to `command`, `args`, `env`, `url` and `headers`, and `auth` values support it too. Syntax: `${env:NAME}`, `${userHome}`, `${workspaceFolder}` (the folder that contains `.cursor/mcp.json`), `${workspaceFolderBasename}`, `${pathSeparator}` and `${/}`. No default syntax is documented for `${env:NAME}`.
  - Unfilled variables: not documented for `.cursor/mcp.json`. History:
    - IDE 3.20 fixed plugin `${env:NAME}` placeholders reaching the server as-is.
    - CLI v2026.10.08 treats an unresolved `${VAR}` OAuth client ID as unset.
    - On self-hosted workers, Cursor doesn't connect to an HTTP server whose URL still has an unfilled variable.
  - Plugin variables are `${VAR}`, not `${env:NAME}`. Official plugins use `${VAR}` 35 times and `${VAR:-}` 4 times. CLI March 2026: `${VAR}` placeholders "expand in MCP configs everywhere". Plugin MCP `command`, `args`, `env` values and `cwd` expand `${CURSOR_PLUGIN_ROOT}` and `${CLAUDE_PLUGIN_ROOT}`. A plugin server shows as `plugin-<plugin>-<server>`.
  - Environment: no doc says stdio servers inherit Cursor's environment. Help says shell-profile variables must be available to Cursor. Since IDE 3.20, processes Cursor starts no longer inherit code-injecting variables such as `NODE_OPTIONS` and `LD_PRELOAD`.
  - `startupTimeoutSec` on a stdio server is honoured by SDK local agents (1.0.38: default 60 s, up to 5 min) and by self-hosted workers (CLI v2026.10.15). mcp.md doesn't list it.
  - OAuth:
    - Flow: authorization code with PKCE. Cursor reads RFC 9728 metadata for the authorization server and the RFC 8707 resource indicator. With DCR it registers as a public client.
    - Static `auth` credentials are allowed. Without `auth.scopes`, Cursor reads `scopes_supported` from `/.well-known/oauth-authorization-server`.
    - Refresh tokens are used. If the provider issues them only for a scope such as `offline_access`, add that scope to `auth.scopes`.
    - Redirects are fixed: desktop `http://localhost:8787/callback`, web and Cloud `https://www.cursor.com/agents/mcp/oauth/callback`. The server is identified by `state`.
  - Approval:
    - IDE: every MCP connection needs approval, and then each tool call does too. `permissions.json` `mcpAllowlist` entries are `server:tool`, matched case-insensitively with `*`, where the server is the mcp.json key. Auto-review (the default since 3.6) runs allowlisted tools and sends the rest to a classifier.
    - `permissions.json` is read from `~/.cursor/` and `<workspace>/.cursor/`, and the arrays are concatenated. **A committed project file can pre-approve tools.**
    - CLI: project servers need approval, while `~/.cursor/mcp.json` servers are auto-approved (April 2026). `--approve-mcps` approves all servers, and `agent mcp enable <id>` adds one to the local approved list.
    - CLI tokens are `Mcp(server:tool)` in `~/.cursor/cli-config.json` or `<project>/.cursor/cli.json`.
  - Trust:
    - CLI headless runs in an untrusted workspace fail unless `--trust` or `--force` is passed (January 2026).
    - `--trust` also works interactively since v2026.07.20, though parameters.md still says "headless mode only".
    - IDE workspace trust is disabled by default.
- **Skills:**
  - Locations: `.agents/skills/`, `.cursor/skills/`, `~/.agents/skills/` and `~/.cursor/skills/`. For compatibility Cursor also reads `.claude/skills/`, `.codex/skills/`, `~/.claude/skills/` and `~/.codex/skills/`.
  - The skills root is walked recursively. A skill's identity is the folder that holds `SKILL.md`, not a category folder above it.
  - A `.cursor/skills/` or `.agents/skills/` folder anywhere in the repo is discovered and scoped to files under its directory. The docs don't say this for `.claude/skills/` or `.codex/skills/`.
  - Frontmatter:
    - `name` is required, uses lowercase letters, digits and hyphens, and must match the parent folder.
    - `description` is required.
    - Optional: `paths` (a list or a comma-separated string), `disable-model-invocation`, `icon`, `color` and `metadata`. The legacy `globs` is still accepted as a fallback.
    - The CLI also accepts `user-invocable: false` (v2026.07.06), which hides the skill from `/` while keeping it available to the model.
  - CLI v2026.10.08 still loads a skill that breaks the spec, with a warning. That covers a `name` that doesn't match its folder or isn't lowercase-hyphen, a name over 64 chars, a description over 1,024 chars, and a body over about 5,000 tokens.
  - Skills that share a name show as separate rows in the `/` menu (IDE 3.25). No precedence between locations is documented.
  - CLI skill and subagent scans skip hidden dot-directories (v2026.08.11). Skill discovery follows symlinked directories (v2026.06.22).
- **Subagents:**
  - Locations: `.cursor/agents/`, `.claude/agents/`, `.codex/agents/`, and the same three under `~/`. Project beats user on a name clash, and `.cursor/` beats `.claude/` and `.codex/`.
  - Fields (none required): `name` (defaults to the filename), `description`, `model` (default `inherit`; a model ID can take `[k=v,…]` options), `readonly` (default `false`) and `is_background` (default `false`).
  - `readonly: true` is documented only as "no file edits, no state-changing shell commands".
  - Subagents inherit all parent tools, MCP included. Cloud subagents use the team's MCP servers instead. **There is no per-tool allow-list in agent files.** The SDK's `tools` and `disallowedTools` exist only on `Agent.create` for local agents, and the SDK `AgentDefinition` has only `description`, `prompt`, `model` and `mcpServers`.
  - A pinned model that is blocked, needs Max Mode or isn't available falls back to the parent's model (CLI v2026.10.22).
  - Built-in subagents: Explore and Browser.
- **Hooks: files, sources and merge:**
  - Shape: `{"version": 1, "hooks": {"<event>": [definition, …]}}`. `version` is required and must be a positive integer.
  - Sources, highest priority first: Enterprise, Team, Project (`.cursor/hooks.json`), User (`~/.cursor/hooks.json`), then Claude's `.claude/settings.local.json`, `.claude/settings.json` and `~/.claude/settings.json`. Plugin hooks come last and run command hooks only.
  - Every matching hook from every source runs. `deny` beats `ask`, which beats `allow`, and `user_message` and `agent_message` are concatenated. For other fields hooks.md says the last response wins, so a lower-priority source overrides a higher one. third-party-hooks.md says the higher priority wins. The docs contradict each other here.
  - Project hooks run only in a trusted workspace. Cursor watches `hooks.json` and reloads it on change.
  - Events:
    - Agent: `sessionStart`, `sessionEnd`, `preToolUse`, `postToolUse`, `postToolUseFailure`, `subagentStart`, `subagentStop`, `beforeShellExecution`, `afterShellExecution`, `beforeMCPExecution`, `afterMCPExecution`, `beforeReadFile`, `afterFileEdit`, `beforeSubmitPrompt`, `preCompact`, `stop`, `afterAgentResponse`, `afterAgentThought`.
    - Tab: `beforeTabFileRead`, `afterTabFileEdit`.
    - App: `workspaceOpen`.
  - **There is no before-edit event.** Only `preToolUse` with tool `Write` runs before an edit, and its `tool_input` shape is undocumented.
- **Hooks: per-hook options:**

  | Key | Type | Default | Notes |
  |---|---|---|---|
  | `command` | string | required | "a shell string, an absolute path, or a relative path" |
  | `type` | `"command"` \| `"prompt"` | `"command"` | Prompt hooks also take `prompt` and `model` |
  | `timeout` | number | "platform default" | **Seconds** |
  | `loop_limit` | number \| null | 5 (Cursor), null (Claude hooks) | `stop` and `subagentStop` follow-ups only |
  | `failClosed` | boolean | `false` | Failures block instead of passing |
  | `matcher` | string (regex) | – | `""` or `"*"` matches everything |

  - Matcher input by event:
    - `preToolUse`, `postToolUse`, `postToolUseFailure`: the tool name.
    - `subagentStart`, `subagentStop`: the subagent type (`generalPurpose`, `explore`, `shell`, …). Cursor's own advisor plugin matches its custom subagent by name with `^advisor-subagent$`.
    - `before/afterShellExecution`: the full command string.
    - Fixed values: `beforeReadFile` `Read`, `afterFileEdit` `Write`, `beforeTabFileRead` `TabRead`, `afterTabFileEdit` `TabWrite`, `beforeSubmitPrompt` `UserPromptSubmit`, `stop` `Stop`, `afterAgentResponse` `AgentResponse`, `afterAgentThought` `AgentThought`.
    - Not stated for `before/afterMCPExecution`.
  - Anchoring isn't stated. The docs say `curl|wget|nc ` runs when the command "contains" one of them.
  - Working directory: project hooks run from the project root (`.cursor/hooks/x.sh`) and user hooks from `~/.cursor/`. Plugin hooks run from the plugin directory (`CURSOR_PLUGIN_ROOT`), except plugin `stop` and `subagentStop`, which run from the workspace root.
  - On Windows the desktop app runs hook commands through PowerShell. A command naming a script without an extension runs the matching `.ps1`, `.exe`, `.bat` or `.cmd` file next to it (IDE 3.23).
- **Hooks: stdin and output:**
  - Every event gets `conversation_id`, `generation_id`, `model`, `model_id`, `model_params`, `hook_event_name`, `cursor_version`, `workspace_roots`, `user_email` and `transcript_path`. `workspaceOpen` omits the conversation fields. Payloads arrive over stdin (CLI May 20, 2026).

    | Event | Input beyond the common fields | Output |
    |---|---|---|
    | `preToolUse` | `tool_name`, `tool_input` (object; Shell `{command, working_directory}`), `tool_use_id`, `cwd`, `agent_message`; `parent_tool_call_id` in a subagent (IDE 3.22) | `permission` (`allow`/`deny`; `ask` accepted but **not enforced**), `user_message`, `agent_message`, `updated_input` |
    | `beforeShellExecution` | `command` (full command line), `cwd`, `sandbox`; `tool_use_id` (IDE 3.24) | `permission` (`allow`/`deny`/`ask`), `user_message`, `agent_message` |
    | `beforeMCPExecution` | `tool_name`, `tool_input` (JSON **string**), `mcp_server_name` (the mcp.json key); HTTP/SSE adds `url` and `mcp_server_url`; stdio adds `command` (the launch string joined with spaces); `tool_use_id` (IDE 3.24) | as for shell. The docs say to decide by `mcp_server_name` and to deny when it is missing or unexpected |
    | `beforeReadFile` | `file_path` (absolute), `content` (whole file), `attachments[{type: file\|rule, file_path}]` | `permission` (`allow`/`deny`), `user_message` |
    | `afterFileEdit` | `file_path`, `edits[{old_string, new_string}]` | none |
    | `beforeSubmitPrompt` | `prompt`, `attachments` | `continue` (false blocks), `user_message` |
    | `subagentStart` | `subagent_id`, `subagent_type`, `task`, `parent_conversation_id`, `tool_call_id`, `subagent_model`, `is_parallel_worker`, `git_branch?` | `permission` (`ask` is treated as deny), `user_message` |
    | `postToolUse` | `tool_output` (JSON string), `duration` (ms) | `updated_mcp_tool_output` (MCP only), `additional_context` |
    | `stop`, `subagentStop` | `status`, `loop_count` (plus subagent details) | `followup_message` |
    | `sessionStart` | `session_id`, `is_background_agent`, `composer_mode` | `env` (passed to later hooks), `additional_context`. Fire and forget, so `continue` isn't enforced |

- **Hooks: exit codes and failure:**

  | Outcome | Default | `failClosed: true` |
  |---|---|---|
  | Exit 0 with valid JSON | JSON applies | same |
  | Exit 0, invalid or off-schema JSON, on a permission hook¹ | **blocks** | blocks |
  | Exit 0 with empty stdout on a permission hook | UNVERIFIED | blocks ("no output") |
  | Exit 2 | blocks (same as `deny`) | blocks |
  | Other exit, crash, timeout, command not found | **action proceeds** (fail-open) | blocks |

  ¹ `beforeShellExecution`, `beforeMCPExecution`, `beforeReadFile`, `beforeTabFileRead`, `subagentStart`, `preToolUse`.
  - A command that can't be found is reported as failed (CLI v2026.10.15, a Windows fix; before that it passed silently). It then follows the failure rules above.
  - A fail-closed hook must print valid JSON on every path, including allow.
- **Hooks: environment and cloud:**
  - Hooks get `CURSOR_PROJECT_DIR`, `CURSOR_VERSION` (always), `CURSOR_USER_EMAIL`, `CURSOR_TRANSCRIPT_PATH`, `CURSOR_CODE_REMOTE` and `CURSOR_PLUGIN_ROOT` (plugin hooks). **`CLAUDE_PROJECT_DIR` is always set as an alias**, so a hook can't use it to tell Claude Code from Cursor.
  - Cloud agents run command hooks only. They don't run `sessionStart`, `sessionEnd`, `before/afterMCPExecution`, Tab hooks or `workspaceOpen`. HTTP MCP tools there reach `preToolUse` as `MCP:<tool_name>`, where `updated_input` is ignored.
- **Claude Code hooks in Cursor:**
  - They load when **Include Third-Party Plugins, Skills, and Other Configs** (Settings → Agents → Third-Party Imports) is on. **It is on by default.**
  - Event mapping: `PreToolUse`→`preToolUse`, `PostToolUse`→`postToolUse`, `UserPromptSubmit`→`beforeSubmitPrompt`, `Stop`→`stop`, `SubagentStop`→`subagentStop`, `SessionStart`→`sessionStart`, `SessionEnd`→`sessionEnd`, `PreCompact`→`preCompact`. `Notification` and `PermissionRequest` aren't supported, and no other Claude event is listed.
  - Responses: both the nested `hookSpecificOutput` and flat forms work. `permissionDecision` becomes `permission`, `permissionDecisionReason` becomes `user_message` and `updatedInput` becomes `updated_input`. `Stop`/`SubagentStop` `decision: "block"` with a `reason` becomes a follow-up. SDK 1.0.40 fixed nested deny in local agents.
  - Documented support covers command and prompt hooks, exit-2 blocking, regex tool matchers and timeouts. Since CLI v2026.10.17, Claude `PostToolUse` matchers for `Task` fire when a subagent finishes.
  - **Exec-form `args` and `onFailure` are never mentioned.**
- **Tool names:**

  | Capability | Hook `tool_name` / matcher | Claude name mapped (third-party-hooks) | CLI permission token | SDK `ToolName` |
  |---|---|---|---|---|
  | read | `Read` | `Read`→`Read` | `Read(pathOrGlob)` | `read` |
  | search | `Grep` | `Grep`→`Grep`; `Glob` unmapped | – | `grep`, `glob`, `ls` |
  | edit | `Write`, `Delete` | `Write`, `Edit`→`Write` | `Write(pathOrGlob)` | `edit` |
  | execute | `Shell` | `Bash`→`Shell` | `Shell(commandBase[:args])` | `shell` (group) |
  | web | `WebFetch`, `WebSearch` (mapping table only) | same names | `WebFetch(domainOrPattern)` | `webSearch` |
  | MCP | `MCP:<tool_name>` (no server name) | not stated | `Mcp(server:tool)` | `mcp` (group) |
  | subagent | `Task` | `Task`→`Task` | – | `task` |

  - SDK 1.2.0 names the native file tools Read, Write, StrReplace, LS, Grep, Glob and Delete.
- **No official JSON Schema** exists for `hooks.json` or `mcp.json`. `cursor.com/schemas/hooks.json` returns HTML, and SchemaStore lists only `.cursor/environment.json`.
- **Not documented (UNVERIFIED):**
  - whether `.cursor/rules/*.mdc` accepts `globs` as a YAML list, and whether the frontmatter parser is strict YAML or strips quotes;
  - what globs in a nested `.cursor/rules/` are relative to, and whether `CLAUDE.md` in subdirectories is read;
  - whether an `@AGENTS.md` line in `CLAUDE.md` is expanded, and whether `AGENTS.md` then loads twice;
  - de-duplication of `CLAUDE.md` and `AGENTS.md` beyond the IDE 3.26 cases;
  - what an unset `${env:NAME}` becomes in `.cursor/mcp.json`, and whether a `:-` default exists;
  - whether `"type": "http"` or `"streamable-http"` is accepted in `.cursor/mcp.json`, and how a typeless `url` picks SSE or Streamable HTTP;
  - whether stdio servers inherit the parent environment, a stdio server's working directory, whether `cwd` is honoured in `.cursor/mcp.json`, and whether the IDE honours `startupTimeoutSec`;
  - which copy the model uses when two skill locations hold the same name, whether nested `.claude/skills/` or `.codex/skills/` are scanned, and whether the third-party setting also gates `.claude/skills/` and `.claude/agents/`;
  - what `readonly: true` does to MCP tools and web fetches, and how a state-changing shell command is judged;
  - how Cursor treats a Claude agent's `tools`, `disallowedTools` and `permissionMode`;
  - the default hook timeout, the shell used on macOS and Linux, and PATH lookup for `command`;
  - matcher anchoring, and the matcher input on `before/afterMCPExecution`;
  - whether empty stdout from an exit-0 permission hook blocks;
  - the `tool_input` shapes for Write, Read, Grep, Delete and MCP in `preToolUse`, and the order of `preToolUse` and `beforeShellExecution` for one shell call;
  - stdin size and encoding limits;
  - whether Claude hooks honour exec-form `args` and `onFailure`, get Claude-format or Cursor-format stdin, and in which directory they run;
  - whether Claude names inside matcher regexes (`^(Bash|PowerShell)$`, `mcp__x__.*`) are translated, and whether `PostToolUseFailure`, `SubagentStart` or `UserPromptSubmit` `decision: "block"` are mapped;
  - whether hooks ever see `Glob`, `LS`, `StrReplace` or browser tools as `tool_name`, and the `MCP:<tool_name>` form for names containing `.` or `/`;
  - how project-hook trust works while IDE workspace trust is disabled (the default);
  - which shipped build (3.27 or 3.30.14) the docs describe.

### GEMINI-P4: Gemini CLI facts for Phase 4
Sources: https://api.github.com/repos/google-gemini/gemini-cli/releases and https://registry.npmjs.org/-/package/@google/gemini-cli/dist-tags (fetched 2026-11-17). Source at tag `v0.68.0`, commit `5a5c7180ff3f0a02511f54b54db7f228d4982a30` (https://github.com/google-gemini/gemini-cli/tree/5a5c7180ff3f0a02511f54b54db7f228d4982a30). Files read: `packages/core/src/` `tools/memoryTool.ts`, `utils/memoryDiscovery.ts`, `utils/memoryImportProcessor.ts`, `context/memoryContextManager.ts`, `utils/trust.ts`, `utils/envExpansion.ts`, `utils/shell-utils.ts`, `services/environmentSanitization.ts`, `tools/mcp-client.ts`, `tools/mcp-client-manager.ts`, `tools/mcp-tool.ts`, `tools/tool-names.ts`, `tools/definitions/`, `mcp/oauth-utils.ts`, `mcp/oauth-provider.ts`, `skills/`, `agents/`, `hooks/`, `scheduler/`, `config/config.ts`, `config/storage.ts`; `packages/cli/src/` `config/settings.ts`, `config/settingsSchema.ts`, `config/settings-validation.ts`, `config/trustedFolders.ts`, `config/config.ts`, `utils/envVarResolver.ts`, `utils/commentJson.ts`, `commands/mcp/add.ts`, `commands/hooks/migrate.ts`. Docs at the tag: https://raw.githubusercontent.com/google-gemini/gemini-cli/v0.68.0/docs/ (`cli/gemini-md.md`, `reference/memport.md`, `reference/configuration.md`, `cli/trusted-folders.md`, `tools/mcp-server.md`, `cli/skills.md`, `core/subagents.md`, `hooks/index.md`, `hooks/reference.md`, `hooks/best-practices.md`, `extensions/reference.md`, `reference/tools.md`), plus `schemas/settings.schema.json` at the tag. Dependencies as pinned at the tag: https://unpkg.com/strip-json-comments@3.1.1/index.js and https://unpkg.com/dotenv-expand@12.0.3/lib/main.js. Diff `v0.68.0...v0.69.0-preview.0` from the GitHub compare API.
Status: Verified (Phase 4) from source at v0.68.0, except the gaps listed. Where the docs at the tag disagree with the source, this entry records the source and lists the doc as stale. A skeptic re-checked 68 researcher facts: 61 confirmed, 5 refuted (shown corrected), 2 uncertain (listed as UNVERIFIED). Nothing was run.

- **Releases checked (2026-11-17):**

  | Channel | Version | Published | Tag commit |
  |---|---|---|---|
  | stable, npm `latest` | v0.68.0 | 2026-11-10T21:17:07Z | `5a5c7180ff` |
  | preview, npm `preview` | v0.69.0-preview.0 | 2026-11-10T21:05:08Z | `d8f2b28dec` |
  | npm `nightly` | 0.67.0-nightly.20261104.g89e9d25d6 | 2026-11-04 | older than stable |

  - There has been no release since the 2026-11-16 entry. `docs/changelogs/latest.md` at the tag still names v0.66.0, so the Releases list is authoritative.
  - v0.69.0-preview.0 is 34 commits ahead. In the areas below it only adds `model.requestTimeoutMs`, a dummy-key fix in `envExpansion.ts`, a macOS Seatbelt trust prompt and new numeric minimums in validation. Every fact here holds for both versions.
  - On main, unreleased: `aeb6c04b09` (2026-11-11) makes `${VAR:-default}` use the default when `VAR` is set but empty; `4fa64e71b9` (2026-11-16) skips system MCP servers whose command has insecure permissions.
- **Stale docs at v0.68.0** (the source wins):
  - `trusted-folders.md` says folder trust is disabled by default; the default is `true`.
  - `configuration.md`:
    - It describes a 200-directory downward GEMINI.md scan; memory discovery has none.
    - It puts project settings at the project root; the code uses the launch directory.
    - It names `security.allowedEnvironmentVariables` and `blockedEnvironmentVariables`; the real keys are under `security.environmentVariableRedaction`.
  - `mcp-server.md` calls `url` SSE and `httpUrl` Streamable HTTP. It also describes automatic OAuth on a 401 without `oauth.enabled`.
  - `subagents.md` names the field `mcpServers`; the strict schema accepts only `mcp_servers`.
  - `hooks/index.md` and `hooks/reference.md`:
    - They say exit codes other than 0 and 2 are warnings, and that exit 2 takes its reason from stderr.
    - They say `tool_input` "merges with" the arguments.
    - They give a layer "precedence" for hooks.
  - The `gemini-md.md` example implies that `context.fileName` replaces GEMINI.md.
- **Context files:**
  - The default name is `GEMINI.md`. `context.fileName` (a string or array, default undefined) adds names in front of `GEMINI.md` rather than replacing it, so `"fileName": "AGENTS.md"` loads both `AGENTS.md` and `GEMINI.md`. `AGENTS.md` and `CLAUDE.md` are not read by default.
  - Global: each configured name in `~/.gemini/`. These load whatever the trust state, as do extension context files and the user's private per-project memory under `~/.gemini`.
  - Project:
    - The walk starts at each workspace directory (the cwd plus include directories). It goes up to the first directory holding a boundary marker (`context.memoryBoundaryMarkers`, default `[".git"]`). With no marker, only the start directory is checked. Every configured name is checked in each directory.
    - These files load only in a trusted folder.
  - Order: the project list is deduplicated, then sorted as plain strings of absolute paths. That is usually root before leaf, but not always: `/repo/Apps/GEMINI.md` sorts before `/repo/GEMINI.md`. Within a directory, names sort alphabetically, whatever order the config gives.
  - Duplicates are removed by file identity (`stat` dev and inode, following symlinks), so a symlink `AGENTS.md` → `GEMINI.md` loads once.
  - There is no startup scan of subdirectories. Just in time: when a tool touches a path inside a workspace directory, the files from that path up to the git root (or the workspace root) load, each once. This happens only in a trusted folder. `context.discoveryMaxDirs` (default 200) still exists, but discovery doesn't use it.
  - Each file is wrapped in `--- Context from: <path> ---` … `--- End of Context from: <path> ---`.
- **Imports:**
  - Syntax: an `@` at the start of the text or after a space, tab or newline. The path runs to the next whitespace and must start with `.`, `/` or an ASCII letter, so `@AGENTS.md` and `@./x.md` both qualify. An `@` inside backtick code is ignored.
  - Imports nest at most 5 deep. The target, resolved through realpath, must lie inside the project root found from the importing file's directory (by boundary marker, else that directory). `file://` and `http(s)://` targets are rejected.
  - Output:
    - A successful import is wrapped in `<!-- Imported from: X -->` … `<!-- End of import from: X -->`.
    - A rejected path becomes `<!-- Import failed: X - Path traversal attempt -->`.
    - A repeat within the same import chain becomes `<!-- File already processed: X -->`.
    - A read error becomes `<!-- Import failed: X - <message> -->`.
- **Settings files:**
  - Layers, with later layers winning for single values:
    1. schema defaults;
    2. `system-defaults.json`;
    3. `~/.gemini/settings.json`;
    4. `<cwd>/.gemini/settings.json`;
    5. system `settings.json`.

    Environment variables and CLI arguments sit above all files. The system directory is `/etc/gemini-cli/`, `C:\ProgramData\gemini-cli\` or `/Library/Application Support/GeminiCli/`; the overrides are `GEMINI_CLI_SYSTEM_SETTINGS_PATH` and `GEMINI_CLI_SYSTEM_DEFAULTS_PATH`. This resolves the precedence conflict noted in GEMINI-SETTINGS.
  - Merging: `mcpServers` is a shallow merge, so a same-named entry from a higher layer replaces the whole entry. `hooks.<Event>` and `context.includeDirectories` are concatenated across layers.
  - The workspace file is only `<cwd>/.gemini/settings.json`, with no upward search. It is skipped when the cwd is the home directory.
  - **The format is JSONC:** the file is parsed as `JSON.parse(stripJsonComments(text))` with strip-json-comments 3.1.1. `//` and `/* */` comments are allowed, but trailing commas are not. The CLI's own writes go through comment-json 4.2.5, which keeps comments, and use an atomic rename.
  - Parse errors: a syntax error, or a file that isn't an object, throws `FatalConfigError` and the CLI exits. This includes the workspace file in an untrusted folder, because it is read before trust is decided.
  - Schema failures only produce a warning, and the invalid values are put back unvalidated and used. Top-level unknown keys pass through. `MCPServerConfig` is strict (`additionalProperties: false`), but that too only warns.
  - Environment expansion at load:
    - It applies to every string value (not to keys) in every layer, hook `command` strings included: `$VAR`, `${VAR}` and `${VAR:-default}`, read from `process.env` after `.env` files load.
    - An unset variable with no default keeps its placeholder literally. A variable set to an empty string gives `""` even when a default is given (v0.68.0).
    - There is no escape for `$`.
- **Folder trust:**
  - `security.folderTrust.enabled` defaults to `true`. A folder with no decision is untrusted, and interactive mode shows a dialog.
  - Overrides, checked in this order:
    1. `GEMINI_RESTRICTED_MODE=true` or `GEMINI_CLI_TRUST_WORKSPACE=false`: untrusted.
    2. `GEMINI_CLI_TRUST_WORKSPACE=true`, which `--skip-trust` also sets: trusted.
    3. Folder trust disabled: trusted.
    4. The IDE's trust signal.
    5. `~/.gemini/trustedFolders.json`, with `TRUST_FOLDER`, `TRUST_PARENT` or `DO_NOT_TRUST`. Its location can be overridden with `GEMINI_CLI_TRUSTED_FOLDERS_PATH`.
  - **An untrusted folder ignores:**
    - the workspace settings, which are replaced by `{}`, so its `mcpServers`, `hooks` and `context.fileName` are all ignored;
    - project `.env` files;
    - project and just-in-time context;
    - workspace skills and project agents.

    Beyond that, no MCP server starts, whatever its layer or transport. Every settings-file hook is skipped, user and system ones too, and only extension hooks run. Approval modes are limited to `default` and `plan`.
- **MCP servers (`mcpServers.<name>`):**
  - Schema fields: `command`, `args`, `env`, `cwd`, `url`, `httpUrl`, `headers`, `tcp`, `type` (`stdio`|`sse`|`http`), `timeout` (ms, default 600000), `trust` (default false), `description`, `includeTools`, `excludeTools`, `extension`, `oauth`, `authProviderType`, `targetAudience`, `targetServiceAccount`.
  - **Stdio:** `{"command": …, "args": […], "env": {…}}`. Leave out `type`: the runtime type is only `sse`|`http`, and `gemini mcp add` writes none.
  - **Streamable HTTP:** `{"url": …, "type": "http"}`, which is what `gemini mcp add --transport http` writes.
    - Transport choice: `httpUrl` (deprecated), then `url` with `type`, then `url` without `type`. A `url` without `type` is tried as Streamable HTTP and falls back to SSE on a non-401 failure.
    - `url`/`httpUrl` are checked before `command`.
  - **Stdio environment:**
    - The server gets a sanitized copy of `process.env`, with redaction forced on for MCP servers whatever the setting says. Extension env and a Gemini CLI identification variable are added, then the explicit `env` entries.
    - Explicit entries are not redacted, except the `BLOCKED_EXECUTION_ENVS` names (`NODE_OPTIONS`, `NODE_PATH`, `LD_PRELOAD`, `LD_LIBRARY_PATH`, `DYLD_*`, `PYTHONPATH`, `RUBYOPT`, `PERL5OPT` and others), which are dropped with a warning.
  - **Redaction:**
    - Names containing `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, `KEY`, `AUTH`, `CREDENTIAL`, `CREDS`, `PRIVATE` or `CERT` are dropped (case-insensitive), as is a fixed list (`CLIENT_ID`, `DB_URI`, `CONNECTION_STRING`, `DATABASE_URL`, `GOOGLE_CLOUD_PROJECT`, `AWS_DEFAULT_REGION` and others).
    - Values that look like secrets are dropped too: PEM keys, credentials in URLs, `ghp_`/`github_pat_`, `AIzaSy`, `AKIA`, JWTs, Stripe and Slack tokens.
    - `GEMINI_CLI_*`, `GIT_CONFIG_*` and basic system variables are kept. With `GITHUB_SHA` set or `SURFACE=Github`, everything not explicitly allowed is dropped.
    - Settings: `security.environmentVariableRedaction.{allowed, blocked, enabled}`, where `enabled` defaults to false and affects hooks, not MCP. An `allowed` entry overrides the name rules but not the value rules.
  - **Two expansion passes for `env` and `headers`:**
    - The first runs at load, against the unsanitized `process.env`.
    - The second runs at connect, with dotenv-expand 12.0.3 against the sanitized environment. An unresolved placeholder becomes `""`. Any `$NAME` or `${…}` left inside an already-resolved value is expanded again, usually to `""`, and `\$` becomes `$`. `%VAR%` is supported only on Windows.
    - So `"GH_TOKEN": "${GH_TOKEN}"` passes the real value when `GH_TOKEN` is set, and the server gets `""` when it is unset. `command`, `args`, `cwd` and `url` get only the load-time pass.
  - **`trust`** bypasses confirmations only in a trusted folder, and extension servers can't set it.
  - **Names:**
    - A tool is exposed as `mcp_<server>_<tool>`. Characters outside `[A-Za-z0-9_.:-]` become `_`, and a name over 63 characters becomes its first 30 + `...` + its last 30.
    - Parsers split at the first `_` after `mcp_`, so a server name must not contain `_`.
    - `mcp.allowed` and `mcp.excluded` filter servers by name.
  - **OAuth** (http and sse):
    - A 401 marks the server as needing auth. Only with `oauth.enabled: true` does the CLI then discover endpoints, register dynamically, open the browser at `http://localhost:<random-port>/oauth/callback` (or `oauth.redirectUri`) and retry. Otherwise it fails with a message to run `/mcp auth <name>`.
    - Tokens are stored in `~/.gemini/mcp-oauth-tokens.json` by default.
    - Discovery tries the RFC 9728 path-inserted protected-resource URL, then the root one, and checks that the metadata's `resource` matches the server URL. It then reads RFC 8414/OIDC metadata.
    - The flow uses PKCE, sends an RFC 8707 `resource`, and validates the RFC 9207 `iss`. Endpoints must use HTTPS unless they are loopback.
    - `oauth` fields: `enabled`, `clientId`, `clientSecret`, `authorizationUrl`, `issuer`, `authorizationResponseIssParameterSupported`, `tokenUrl`, `scopes`, `redirectUri`, `tokenParamName`, `audiences`, `allowPrivateNetwork`. `authProviderType` is `dynamic_discovery` (the default), `google_credentials` or `service_account_impersonation`.
- **Skills:**
  - Precedence, lowest to highest: built-in, extensions, `~/.gemini/skills`, `~/.agents/skills`, `<cwd>/.gemini/skills`, `<cwd>/.agents/skills`. Workspace paths are relative to the cwd and load only in a trusted folder.
  - A skill is `SKILL.md` directly in the directory or one level down (`*/SKILL.md`); `node_modules` and `.git` are ignored.
  - Frontmatter must start with `---`; a BOM is allowed. Both `name` and `description` must be strings, or the skill is skipped silently. When YAML fails, a line parser is tried. Other keys are ignored, and the directory name need not match. In `name`, the characters `:\/<>*?"|` become `-`.
  - Collisions are matched on the exact, case-sensitive name: the later tier wins and a warning is shown. `activate_skill` lookup and `skills.disabled` are case-insensitive.
  - `skills.enabled` and `admin.skills.enabled` both default to `true`. Activating a skill shows a consent prompt (docs).
- **Subagents:**
  - Files: `<cwd>/.gemini/agents/*.md` (skipped when the cwd is home) and `~/.gemini/agents/*.md`. Only regular files load, so symlinks are skipped; so are names starting with `_`. Each file must begin with YAML frontmatter, and the body becomes the system prompt.
  - Local frontmatter (strict zod; an unknown key fails the whole file):

    | Key | Rule / default |
    |---|---|
    | `kind` | `local` (default) or `remote` |
    | `name` | required, `^[a-z0-9-_]+$` |
    | `description` | required, non-empty |
    | `display_name` | optional |
    | `tools` | optional list of names; validated, wildcards allowed |
    | `mcp_servers` | inline servers; snake_case fields (`http_url`, `include_tools`, `exclude_tools`, `auth`) |
    | `model` | default `inherit` |
    | `temperature` | default 1 |
    | `max_turns` | positive integer, default 30 |
    | `timeout_mins` | positive integer, default 10 |

  - Remote agents: `kind: remote` with `agent_card_url` or `agent_card_json`, plus optional `auth`. A YAML list as frontmatter defines several.
  - **`tools`:**
    - When omitted, the agent gets every parent tool except other agents and `update_topic`. `tools: []` gives only the mandatory `complete_task`.
    - A name that isn't registered at runtime is skipped silently, but a syntactically invalid name rejects the file. Subagents can never call subagents.
  - Built-in tool names:

    | Function | Name |
    |---|---|
    | read | `read_file`, `read_many_files` |
    | search | `grep_search` (legacy alias `search_file_content`), `glob` |
    | list | `list_directory` |
    | edit | `replace` |
    | write | `write_file` |
    | shell | `run_shell_command` |
    | web | `web_fetch`, `google_web_search` |

  - MCP tools in lists: `mcp_<server>_<tool>`, or the wildcards `*`, `mcp_*` and `mcp_<server>_*`. `mcp_*_<tool>` is rejected. The parts must match `/^[a-z0-9_.:-]+$/i`, and the server part can't contain `_`.
  - Subagents are on by default: `experimental.enableAgents` is `true`, and the docs no longer call them a preview.
  - A project agent registers only in a trusted folder (or with trust off), and only once acknowledged. The acknowledgment is keyed by project root, name and the file's sha256, and stored in `~/.gemini/acknowledgments/agents.json`. Any edit means acknowledging it again.
  - On a duplicate name, the first registered wins and a warning is shown. The order is built-ins (`codebase_investigator`, `cli_help`, `generalist`, `browser_agent` if enabled), then project, user, and extension agents.
- **Hooks (`hooks.<Event>` in settings.json):**
  - Shape: `[{ "matcher"?, "sequential"?, "hooks": [{ "type": "command", "command", "name"?, "description"?, "timeout"?, "env"? }] }]`. `env` exists only in source, and the schema doesn't forbid extra keys. The keys `enabled`, `disabled` and `notifications` inside `hooks` are skipped, and an unknown event name is skipped with a warning.
  - Events, matchers and stdin additions:

    | Event | Matcher | stdin adds |
    |---|---|---|
    | `BeforeTool` | regex on `tool_name` | `tool_name`, `tool_input`, `mcp_context?`, `original_request_name?` |
    | `AfterTool` | regex on `tool_name` | `tool_name`, `tool_input`, `tool_response` |
    | `BeforeAgent` | ignored | `prompt` (after a prompt is submitted, before the model; once per `prompt_id`) |
    | `AfterAgent`, `Notification` | ignored | event-specific |
    | `BeforeModel`, `AfterModel`, `BeforeToolSelection` | ignored | `llm_request` (`model`, `messages`, `config`, `toolConfig`), plus `llm_response` on AfterModel |
    | `SessionStart` | exact `source` (`startup`, `resume`, `clear`) | `source` |
    | `SessionEnd` | exact `reason` | `reason`; not awaited |
    | `PreCompress` | exact `trigger` | `trigger` |

  - Tool matchers use `new RegExp(matcher).test(tool_name)`, unanchored, so write `^(…)$` for exact names. An invalid regex falls back to exact equality, and `""` or `*` matches everything.
  - Common stdin fields: `session_id`, `transcript_path`, `cwd`, `hook_event_name`, `timestamp` (ISO 8601), written as one `JSON.stringify` with no size cap. `mcp_context` is `{server_name, tool_name, command?, args?, cwd?, url?, tcp?}`.
  - `tool_input` is the call's validated arguments as the model gave them; paths aren't made absolute.

    | Tool | Fields |
    |---|---|
    | `run_shell_command` | `command`, `description?`, `dir_path?`, `is_background?`, `delay_ms?`, `additional_permissions?` (with tool sandboxing) |
    | `write_file` | `file_path`, `content` |
    | `replace` | `file_path`, `instruction`, `old_string`, `new_string`, `allow_multiple?` |
    | `read_file` | `file_path`, `start_line?`, `end_line?` |
    | `web_fetch` | `prompt` |
    | `google_web_search` | `query` |

  - **Execution:**
    - `command` is a shell string; there is no argv form. On Unix it runs as `bash -c <command>`, with `bash` resolved on PATH. On Windows it runs through `ComSpec` if that is PowerShell, else `pwsh.exe`, else `powershell.exe`, with `-NoProfile [-NonInteractive] -Command` and `; if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }` appended.
    - Before the hook runs, the text forms `$GEMINI_PROJECT_DIR`, `$GEMINI_CWD`, `$GEMINI_PLANS_DIR`, `$GEMINI_SESSION_ID` and `$CLAUDE_PROJECT_DIR` are replaced with shell-escaped values; the `${…}` forms are not.
    - The working directory is the session cwd.
  - Timeout: `timeout` is in ms, default 60000. On expiry the CLI sends SIGTERM (`taskkill /f /t` on Windows), then SIGKILL 5 s later.
  - Environment: `process.env`, sanitized only when `security.environmentVariableRedaction.enabled` is true or in GitHub Actions. The CLI adds `GEMINI_PROJECT_DIR`, `GEMINI_CWD` and `CLAUDE_PROJECT_DIR` (all the session cwd), `GEMINI_PLANS_DIR` and `GEMINI_SESSION_ID`, then the hook's `env`.
  - **Output.** The CLI reads trimmed stdout, or stderr when stdout is empty:

    | Output | Exit | Result |
    |---|---|---|
    | JSON object (or a JSON string holding one) | any | the fields decide; the exit code is ignored |
    | plain text | 0 | allow; the text becomes `systemMessage` |
    | plain text | 1 | allow; `Warning: <text>` |
    | plain text | anything else (2, 126, 127…) | **deny**; the text is the reason |
    | nothing | any | allow |

    Any result other than exit 0 shows a "Hook(s) […] failed" warning. Only the parsed output can block; errors never do.
  - **Failures fail open:** a timeout, a failure to spawn (for example, no `bash` on PATH), a kill by signal (close code null, read as 0), a crash with exit 1, or a crash that prints nothing all let the action through, with a warning.
  - **JSON fields:**
    - `decision` (`allow`, `deny`, `block` or `ask`), `reason`, `continue`, `stopReason`, `systemMessage`, `suppressOutput`, `hookSpecificOutput`.
    - On BeforeTool:
      - `deny`/`block` gives the model `Tool execution blocked: <reason>`.
      - `continue: false` stops the agent.
      - `ask` forces a confirmation.
      - **`hookSpecificOutput.tool_input` replaces the whole argument object**, which must pass the tool's validation. With several hooks, the last one wins.
    - On BeforeAgent, a block stops the turn, and `hookSpecificOutput.additionalContext` is appended with `<` and `>` escaped.
    - If any hook blocks, the action is blocked.
    - The hook runs before the policy check, so `allow` never skips policy or confirmations.
    - The docs recommend `{"decision":"deny","reason":…}` with exit 0.
  - Registration and trust:
    - Settings hooks from every layer are concatenated (system-defaults, user, workspace, system), tagged `project`, and registered only in a trusted folder; the runner checks again. Extension hooks always run.
    - Workspace hooks are fingerprinted as `name:command` per project root in `~/.gemini/trusted_hooks.json`. A new or changed hook triggers one warning, then is trusted and run with no consent step.
    - `hooksConfig.enabled` defaults to `true`. `hooksConfig.disabled` matches a hook by `name`, or by `command` when it has no name.
    - Within an event, identical `name:command` hooks run once. They run in parallel unless any matching definition sets `sequential: true`.
    - Subagents' tool calls go through the same scheduler, so BeforeTool and AfterTool fire for them.
  - `gemini hooks migrate` maps PreToolUse→BeforeTool, PostToolUse→AfterTool, UserPromptSubmit→BeforeAgent, Stop and SubagentStop→AfterAgent, PreCompact→PreCompress, and keeps SessionStart, SessionEnd and Notification. It converts seconds to ms and drops `args`. Its tool table (`Grep`→`grep`, `LS`→`ls`) is wrong, so it isn't authoritative.
- **Extensions:**
  - Extensions load only from `~/.gemini/extensions`. A workspace can only enable or disable them (`--scope workspace`), and installing, updating or uninstalling needs a trusted folder.
  - When an extension server and a settings server share a name, the settings entry wins. Extensions can't set `trust`.
  - Out of scope for project files.
- **Not verified (UNVERIFIED):**
  - Inferred from source, not run:
    - whether `@AGENTS.md` in GEMINI.md, with AGENTS.md also in `context.fileName`, loads the content twice;
    - whether a missing hook binary on Unix exits 127 and so denies.
  - Not checked:
    - the Windows exit code for a missing hook binary;
    - how a relative `mcpServers.<name>.cwd` is resolved;
    - whether a configured `Authorization` header suppresses the OAuth path (`mcp-client.ts` has no such check; the SDK was not traced);
    - how an unacknowledged project agent can be acknowledged in headless (`-p`) runs;
    - the encoding of hook stdin;
    - whether a BOM in `settings.json` parses.
  - Docs only:
    - Headless runs in an untrusted folder exit with `FatalUntrustedWorkspaceError` (the class exists, but no throw site was found in the files read).
    - Subagents are exposed as a tool named after the agent; the source also defines `invoke_agent`.

### COPILOT-P4: GitHub Copilot facts for Phase 4
Sources: Copilot CLI releases at https://api.github.com/repos/github/copilot-cli/releases (read 2026-11-17) and https://github.com/github/copilot-cli/blob/16604af55c440e9e6bac7ea14a6c48c1b0a7e0c9/changelog.md (through 1.0.112). GitHub docs rendered via https://docs.github.com/api/article/body?pathname=/en/copilot/… at `github/docs` main `afa12590d7` (2026-11-16):
- reference/: hooks-reference (`2d5d2a6c26`, 2026-11-09; the exec form was already there at `b87a8cfb59`, 2026-09-01), copilot-cli-reference/cli-command-reference (`4c6da2fbbf`, 2026-11-11), copilot-cli-reference/cli-config-dir-reference, copilot-cli-reference/cli-plugin-reference, custom-agents-configuration (`8e7396e5e2`, 2026-10-28), custom-instructions-support, customization-cheat-sheet.
- how-tos/copilot-cli/customize-copilot/: add-custom-instructions (`b4ab111fb6`, 2026-10-19), add-mcp-servers (`59cb2c2429`, 2026-11-09), add-skills, create-custom-agents-for-cli, use-hooks.
- how-tos/copilot-on-github/customize-copilot/: configure-mcp-servers (`ba073ff7c2`, 2026-11-16), add-custom-instructions/add-repository-instructions, and customize-cloud-agent/ create-custom-agents, add-skills, configure-secrets-and-variables, use-hooks.
- concepts/customizations/: mcp, hooks.

VS Code docs at https://github.com/microsoft/vscode-docs/tree/f7a9f9debe92e1c9226a44e596b9b965bf5f1442 (2026-11-16; every page `DateApproved: 11/11/2026`):
- docs/agents/reference/: custom-instructions-reference.md, custom-agent-file-reference.md, mcp-configuration.md, ai-settings.md, tools-reference.md.
- docs/agents/concepts/agent-harnesses.md; docs/agents/run/agent-harnesses.md and manage-mcp-servers.md.
- docs/agent-customization/: agent-skills.md, custom-instructions.md, hooks.md, migrate-customizations.md.
- release-notes/v1_146.md and v1_147.md.

VS Code update API: https://update.code.visualstudio.com/api/update/linux-x64/stable/latest and …/insider/latest.

Status: Partial. Every fact was read in the docs or the changelog on 2026-11-17. The Copilot CLI runtime is not public, so nothing here comes from source code or an experiment. A skeptic re-read every source for the researcher's 92 facts: 81 confirmed, 6 corrected (4 were version gates), and 5 uncertain. Gaps are listed under UNVERIFIED.

- **Releases checked (2026-11-17):**
  - Copilot CLI: stable `v1.0.112` (2026-11-13T23:59Z). Prereleases `v1.0.113-0` (2026-11-16) and `v1.0.113-1` (2026-11-17) change no customization format. `-1` adds the managed setting `features.browserUse`.
  - VS Code: stable 1.146.0 (commit `69047191fc42`, GitHub release 2026-11-11). Insiders 1.147.0-insider (commit `95906867e076`, built 2026-11-16). The 1.147 notes are dated 2026-11-18, were last updated November 13, and change no customization format.
  - VS Code 1.146 **deprecates the Local harness**, which is removed in February 2027. The Copilot harness, built on the Copilot SDK, has been the default since 1.143. 1.146 also adds Skills over MCP (preview, `chat.agentHost.mcpSkills.enabled`).
  - **Changelog quirk:** the `1.0.107` section repeats `1.0.106` exactly, and `v1.0.106` is a stable release (2026-11-06). So every change listed under 1.0.107 shipped in 1.0.106.
  - Facts below apply to CLI 1.0.112 and VS Code 1.146 unless a version is given.
- **Surfaces:**
  - "Copilot harness" is VS Code's default harness. It shares a runtime with Copilot CLI and the Copilot app.
  - "Cloud agent" is Copilot cloud agent on GitHub.com.
  - VS Code's Local harness is deprecated and is not a Phase 4 target.
  - Hooks: VS Code is public preview; GitHub.com and the CLI are supported.

**Instructions**
- **Files:**
  - `.github/copilot-instructions.md` is Markdown. VS Code says project instruction files need no frontmatter.
  - `.github/instructions/**/NAME.instructions.md` takes frontmatter `applyTo`: one glob, or several separated by commas. The CLI also accepts a YAML list (1.0.6) and unquoted globs (1.0.48).
  - `excludeAgent` takes `"code-review"` or `"cloud-agent"`. Without it, both use the file. No doc says whether the CLI or VS Code acts on it.
  - VS Code also reads `name` (defaults to the file name) and `description`.
  - Globs are relative to the workspace or repository root, not to the file's directory. `**` matches every file. There is no default pattern. VS Code writes the list as `**/*.ts, **/*.tsx`; GitHub writes it without the space.
- **Copilot CLI discovery:**
  - The "standard locations" are the repo root, the cwd, the directories between them, and directories on the path to a file the CLI works on. `.github/copilot-instructions.md`, `AGENTS.md`, `CLAUDE.md` (also `.claude/CLAUDE.md`) and `GEMINI.md` are read there.
  - `*.instructions.md` is read from the standard locations except the directories in between. The command-reference table says "Git root and cwd" for every file.
  - All files are merged with no general precedence. Only identical copies are deduplicated (1.0.26). Since 1.0.26, files with a specific `applyTo` are listed in a table instead of inlined.
  - The CLI also reads `.claude/rules/**/*.md` like `.instructions.md` files, scoped by `paths` or `applyTo` (1.0.89). A scoped rule is delivered when the agent reads or edits a matching file.
  - Subagents: a custom agent that runs as a subagent gets no repository instruction files unless it sets `include-custom-instructions: true` (1.0.86). The session agent and `general-purpose` always get them. The built-in `explore`, `task` and `code-review` subagents never do. `--no-custom-instructions` overrides every agent.
- **CLI `@` imports:**
  - A line that starts with `@` followed by a path inlines that file. This works in `AGENTS.md`, `CLAUDE.md` and `.github/copilot-instructions.md` (1.0.66).
  - Imports resolve recursively, with depth, cycle and size guards. Since 1.0.106 they respect Markdown code fences.
  - They are not expanded in `GEMINI.md` or `*.instructions.md`.
  - The docs disagree on paths. The how-to says paths are relative and must stay inside the repository, and it skips absolute and `~/` paths. The command reference allows paths relative to the file, or absolute. `@AGENTS.md` is valid under both.
  - **So Copilot CLI expands Grimoire's `@AGENTS.md` block in `CLAUDE.md`.**
- **Cloud agent:** reads `.github/copilot-instructions.md` and `.github/instructions/**/*.instructions.md`. It also reads `AGENTS.md` (the nearest file takes precedence) or, "alternatively", a single root `CLAUDE.md` or `GEMINI.md`.
- **Code review:** on GitHub.com it reads repo-wide and path-specific files plus `AGENTS.md`, `CLAUDE.md`, `GEMINI.md` or `REVIEW.md`. In VS Code it reads only `.github/copilot-instructions.md`.
- **VS Code, by harness:**
  - Copilot harness: `.github/copilot-instructions.md` or `AGENTS.md`, plus `.github/instructions/**/*.instructions.md` and nested `AGENTS.md` files. Nested files are found at the root, the cwd, the directories between, and along the paths of files the agent works on.
  - Claude harness: `CLAUDE.md`, `.claude/CLAUDE.md` and `.claude/rules` (`paths`).
  - Local harness: reads `CLAUDE.md` while `chat.useClaudeMdFile` is on, which is the default (`true`).
  - The VS Code docs never say whether the Copilot harness reads `CLAUDE.md` or expands `@` imports. They link to the CLI page for "additional native formats".

**Custom agents**
- **Files:**
  - Agents live in `.github/agents/` as `.agent.md` or `.md` files.
  - The CLI agent ID is the path relative to `agents/`, without the extension, with `/` replaced by `--`. For example, `agents/team/reviewer.agent.md` becomes `team--reviewer`.
  - On the cloud agent, filenames may use only `.`, `-`, `_`, `a-z`, `A-Z` and `0-9`, and the file must be merged into the default branch.
- **Shared fields (custom-agents-configuration):**
  - `description` is **required**. Optional: `name`, `target`, `tools`, `model`, `disable-model-invocation`, `user-invocable`, `mcp-servers`, `metadata`.
  - `target` is `vscode` or `github-copilot`; unset means both.
  - `disable-model-invocation` defaults to `false` and wins over `infer`. `user-invocable` defaults to `true`. `infer` is retired.
  - IDE agents don't use `mcp-servers` or `metadata`.
  - The body is capped at 30,000 characters.
  - The cloud agent ignores `argument-hint` and `handoffs`.
- **CLI fields:**
  - The CLI reference lists `description`, `include-custom-instructions` (default `false`), `infer` (default `true`), `mcp-servers` (same schema as `~/.copilot/mcp-config.json`), `model`, `models`, `modelPolicy`, `name` (defaults to the agent ID, and also selects the agent with `--agent`), `reasoningEffort` and `tools`.
  - `tools` defaults to `["*"]`, and a `*` anywhere in the list grants every tool.
  - The CLI how-to example spells `model-policy` instead and adds `strict-tools-list: true`. That key limits the agent to the listed tool groups without the "usual default tools", which are not named.
- **VS Code Copilot schema:**
  - It adds `skills`, `deferred-tool-loading`, `strict-tools-list`, `include-custom-instructions`, `github`, `hooks`, `model-policy` and `reasoning-effort`. `model` may be a list.
  - When VS Code converts agents for the runtime, it **does not forward** `model-policy`, `mcp-servers`, `github`, `deferred-tool-loading`, `strict-tools-list` or `include-custom-instructions`. It does forward `hooks`.
  - The Copilot target rejects `handoffs`, `agents`, `argument-hint` and `advancedOptions`.
  - `target` changes only how the editor reads the file, not which harness runs it.
  - VS Code also reads `.claude/agents/` in Claude format.
- **`name` and duplicates:**
  - `name` defaults to the agent ID in the CLI, to the file name without `.md`/`.agent.md` on the cloud agent, and to the file name in VS Code.
  - Agents are deduplicated by ID (file name), never by `name`. On the cloud agent a repository agent beats an organization agent, which beats an enterprise agent.
- **`tools`:**
  - Omitted or `["*"]` grants every tool. A list grants only what it names. `[]` grants none.
  - Unknown names are ignored silently, aliases are case-insensitive, and a comma-separated string also works.
  - MCP tools are named `server/tool` or `server/*`. Tool names that contain `/` match since 1.0.89.
  - Extension tools are named `<extensionId>/<tool>`.
  - The cloud agent has `github/*` and `playwright/*` built in.
- **Aliases:**

  | Alias | Also accepts | Cloud agent tool |
  |---|---|---|
  | `execute` | `shell`, `Bash`, `powershell` | `bash` or `powershell` |
  | `read` | `Read`, `NotebookRead` | `view` |
  | `edit` | `Edit`, `MultiEdit`, `Write`, `NotebookEdit` | e.g. `str_replace`, `str_replace_editor` |
  | `search` | `Grep`, `Glob` | `search` |
  | `agent` | `custom-agent`, `Task` | custom-agent tools |
  | `web` | `WebSearch`, `WebFetch` | not available |
  | `todo` | `TodoWrite` | not available |

  - In the CLI, a shell alias also grants the matching read, list and stop shell tools (1.0.71).
  - `search`, `web`, `todo` and `github-mcp-server/web_search` grant their tools since 1.0.100.
  - The CLI's runtime tools behind `read`, `edit` and `search` are not listed.
- **CLI load order, first ID wins:**
  1. `~/.copilot/agents/`
  2. `.github/agents/`, from the cwd up to the Git root
  3. `.claude/agents/`, the same walk
  4. `--add-dir` roots
  5. plugins
  6. organization or enterprise agents

  So `.github/agents/x.agent.md` beats `.claude/agents/x.md`.
- **Unknown fields:** they cause a warning and the agent still loads (0.0.402, 0.0.415). Since 1.0.106, `-p --agent` prints ignored keys to stderr.
- **`mcp-servers` on the cloud agent:**
  - `stdio` maps to `local`.
  - Values may use `$X`, `${X}`, `${X:-d}`, `${{ secrets.X }}` and `${{ vars.X }}`, where X is a `COPILOT_MCP_` name.
  - Processing order: built-in servers, then the agent's servers, then repository settings.

**Skills**
- **CLI locations,** with the first one found winning for a duplicate name:
  1. `.github/skills/`
  2. `.agents/skills/`
  3. `.claude/skills/`
  4. parent `.github/skills/`
  5. `~/.copilot/skills/`
  6. `~/.agents/skills/`
  7. plugins
  8. `COPILOT_SKILLS_DIRS`
  9. `--add-dir` roots
  10. built-in skills (lowest)

  **A skill in both `.agents/skills/` and `.claude/skills/` loads once in the CLI, from `.agents/skills/`.**
  - `.claude/commands/*.md` commands rank below skills of the same name.
  - Since 1.0.35 and 1.0.36, nothing is loaded from `~/.claude/`.
- **Other surfaces:**
  - GitHub's how-tos list `.github/skills`, `.claude/skills` and `.agents/skills` for the cloud agent, code review, the CLI, the app and VS Code agent mode.
  - VS Code's table gives only `.github/skills/` and `~/.copilot/skills/` for the Copilot harness. It notes that the table shows "not every directory it recognizes".
  - VS Code documents how duplicate names behave only for Marketplace installs, where both copies are listed.
- **CLI frontmatter:**
  - `name` (required): up to 64 characters; starts with a letter or digit; may contain letters, digits, `-`, `_`, `.`, `:` and spaces.
  - `description` (required): up to 1024 characters.
  - Optional: `argument-hint`; `allowed-tools` (a string or a list; `"*"` means all tools); `user-invocable` (default `true`); `disable-model-invocation` (default `false`).
  - The GitHub how-to adds an optional `license` and asks for names that are lowercase with hyphens.
- **VS Code is stricter:**
  - `name` may contain only lowercase letters, digits and hyphens, and must match the parent directory. Otherwise the skill **silently fails to load**.
  - VS Code also documents `context: fork`, behind a feature flag.
- **`allowed-tools`:** pre-approves the listed tools, so Copilot doesn't ask for confirmation while the skill is active.

**MCP**
- **Copilot CLI files:**
  - The CLI reads `.mcp.json` and `.github/mcp.json` in each directory from the cwd up to the Git root, after folder trust. Untrusted folders are skipped silently.
  - When the same directory has both files, `.github/mcp.json` is ignored rather than merged (help text clarified in 1.0.108).
  - On a name clash, the file closer to the cwd wins.
  - Precedence: `--additional-mcp-config`, then plugins, then workspace files, then `~/.copilot/mcp-config.json`.
  - The top level may be an `mcpServers` object, or bare server names (0.0.401).
  - The CLI does not read `.vscode/mcp.json` (removed in 1.0.22).
  - An invalid entry is skipped with `Warning: workspace MCP config …` and its siblings still load. A malformed or unreadable file is skipped entirely.
- **CLI types:**
  - `local` or `stdio` (default `local`).
  - `http`, meaning Streamable HTTP; `streamable-http` is normalized to `http`.
  - `sse`.
  - A remote entry without `type` defaults to `http` (1.0.29), although the reference table marks `type` as required.
- **CLI local fields:**
  - `command`, `args`, `tools`, `env`, `cwd`, `type`, `deferTools` (`auto` | `never`), `disableToolCache`, `slowConnectionThresholdMs` (default 10000).
  - `timeout` is in **milliseconds** (default 30000, connection floored at 60000).
  - The table marks `args` and `tools` as required. In practice a missing `args` means `[]` (1.0.49) and a missing `tools` means all tools (0.0.404).
- **CLI remote fields:** `type`, `url`, `tools`, `headers`, `oauthClientId`, `oauthScopes` (needs `oauthClientId`), `oauthPublicClient` (default `true`), `oauthGrantType` (`authorization_code` | `client_credentials`), `oidc`, `timeout`, `deferTools`, `slowConnectionThresholdMs`.
- **Compared with Claude Code:** the format is not identical. OAuth uses flat fields, not a nested `oauth` object, and there is a `tools` filter. Claude's `headersHelper` and `alwaysLoad` are not documented. Claude-style `{type: "stdio", command, env}` and `{type: "http", url}` entries are accepted.
- **CLI variables:**
  - `env` supports `$VAR`, `${VAR}` and `${VAR:-default}`, and `headers` "supports variable expansion".
  - Since 1.0.108, interactive sessions pass `env` values literally unless they use `${NAME}`.
  - Variables referenced in `command`, `args` or `cwd` are added to the server's environment (0.0.419).
- **CLI stdio environment:**
  - Only `PATH` is inherited; every other variable must be in `env`.
  - The CLI also sets `COPILOT_AGENT_SESSION_ID` (1.0.29).
  - Non-JSON stdout lines are dropped. Lines over 1 MB bypass that check.
  - Since 1.0.106, a response over 16 MiB fails the request.
- **CLI prompt mode:**
  - In a trusted folder, `-p` loads workspace servers.
  - Otherwise it loads them only when `GITHUB_COPILOT_PROMPT_MODE_WORKSPACE_MCP=true`, or when `COPILOT_ALLOW_ALL` is exactly `true`. Other truthy spellings of `COPILOT_ALLOW_ALL` only auto-approve tools.
- **CLI tool names and permissions:**
  - The model sees `serverName-toolName`, sanitized to `[A-Za-z0-9_-]` with other characters replaced by `-`, Punycode for Unicode, and at most 64 characters.
  - Permission rules write MCP tools as `Server(tool)`.
- **VS Code Copilot harness:**
  - It reads workspace-root `.mcp.json` and `$COPILOT_HOME/mcp-config.json` (default `~/.copilot/mcp-config.json`), both with `mcpServers`.
  - Fields: `type` (`stdio` | `http` | `sse`), `command`, `args`, `env`, `cwd`, `url`, `headers`, `tools`, `oauthClientId`.
  - `${NAME}` and `${NAME:-default}` work in stdio `command`, `args` and `env`, and in HTTP header values. **Use the braced form**: VS Code's MCP client treats a bare `$NAME` as literal text.
  - The workspace-root form editor rejects `${…}`, so edit the JSON directly.
  - `.vscode/mcp.json` (`servers`, `${env:NAME}`) is the Local format. It reaches Copilot only through editor forwarding.
  - The docs don't mention `.github/mcp.json` or parent directories.
  - `.mcp.json` inherits Workspace Trust. Restricted mode stops its servers, and a trusted workspace starts them with no separate MCP prompt.
- **Cloud agent:**
  - There is no repository file. Configuration is JSON under repository **Settings → Copilot → MCP servers**, a page shared with code review.
  - The JSON needs an `mcpServers` object. Every entry needs `tools` (`string[]`) and `type` (`local`, `stdio`, `http` or `sse`).
  - Local entries need `command` and `args` (`string[]`) and may add `env`. Remote entries need `url` and may add `headers`.
  - `$VAR`, `${VAR}` and `${VAR:-default}` work in every string and `string[]` field except `tools` and `type`. Referenced names must start with `COPILOT_MCP_`. The env key the server sees may differ, for example `"TOKEN": "$COPILOT_MCP_TOKEN"`.
  - Values come from **Agents** secrets or variables at repository or organization level. A cloud sandbox gets only secrets.
  - Secret and variable names use only `A-Z`, `0-9` and `_`, may not start with `GITHUB_` or a digit, and are upper-cased.
  - Limits: tools only, with no resources or prompts. Remote servers that use OAuth are not supported. Tools run without approval. HTTP and SSE URLs can't contain query parameters. At most 50 servers across personal and repository config, and the repository wins a name clash with a personal server.
  - Code review uses only tools with `annotations.readOnlyHint: true`.
  - **Doctor snippet:** `{"mcpServers":{"<id>":{"type":"local","command":"<binary>","args":[],"env":{"VAR":"${COPILOT_MCP_VAR}"},"tools":["*"]}}}`, with `${COPILOT_MCP_VAR:-}` for an optional variable. What an unset reference without a default becomes on the cloud agent is UNVERIFIED.

**Hooks**
- **Format:**
  - Hook files are `{"version": 1, "hooks": {"<event>": [entries]}}`. `version` may be omitted (1.0.5).
  - A top-level `disableAllHooks: true` in one `.github/hooks/*.json` file skips only that file. In repository `settings.json`, it skips every non-policy source (CLI only).
- **Command hook fields:**

  | Field | Notes |
  |---|---|
  | `type` | `"command"`, the default |
  | `bash`, `powershell` | Shell strings for Unix and Windows |
  | `command` | Cross-platform fallback, copied to `bash` and `powershell` when they are absent |
  | `exec`, `args` | Shell-free executable and argument list. CLI only. Never combined with `bash`, `powershell` or `command` |
  | `cwd` | Repo-relative or absolute. Without it, the hook runs in the project root (1.0.88) |
  | `env` | Merged into the inherited environment; supports expansion |
  | `timeoutSec` | **Seconds**, default 30. `timeout` is an alias, used only when `timeoutSec` is absent |
  | `matcher` | See Matchers |

  - The VS Code Copilot harness documents `exec`/`args` too.
  - **The cloud agent honors only `bash`, falling back to `command`**, so it doesn't run `exec` entries.
- **Other hook types:**
  - `http`: a JSON `POST` with `url`, `headers`, `allowedEnvVars` and `timeoutSec`. Only https is allowed, except `localhost`, `127.*` and `[::1]` when `COPILOT_HOOK_ALLOW_LOCALHOST=1`. `preToolUse` and `permissionRequest` always require https.
  - `prompt`: `sessionStart` only, and only for new interactive CLI sessions.
- **Agent-scoped hooks (CLI):**
  - A `hooks` map in agent frontmatter. `command` is the fallback, with `windows`, `linux` and `osx` overrides.
  - Repository agents need workspace admission. Remote or organization agents don't run scoped hooks.
- **Events:**

  | Event | Behaviour on the cloud agent |
  |---|---|
  | `sessionStart`, `sessionEnd` | Once per job |
  | `userPromptSubmitted` | At most once |
  | `userPromptTransformed` | Fires |
  | `preToolUse` | `ask` counts as deny |
  | `postToolUse`, `postToolUseFailure` | Fire |
  | `permissionRequest` | No effect |
  | `agentStop`, `subagentStart`, `subagentStop`, `errorOccurred` | Fire |
  | `preCompact` | Only with `trigger: "auto"` |
  | `notification` | Never fires |

  - The CLI handles every event in the table.
- **Payload format, chosen by the case of the event name:**
  - camelCase names send camelCase fields with an epoch-ms `timestamp`.
  - PascalCase names (`SessionStart`, `UserPromptSubmit`, `PreToolUse`, `PostToolUse`, `PostToolUseFailure`, `Stop`, `SubagentStop`, `ErrorOccurred`, `PreCompact`, `PermissionRequest`) send snake_case fields with an ISO `timestamp` and `hook_event_name` (1.0.21).
  - Inside a subagent, payloads also carry `agentId`, `agentType` and `parentAgentId` (snake_case under PascalCase names).
- **`preToolUse` stdin:**
  - camelCase: `{sessionId, timestamp, cwd, toolName, toolArgs}`.
  - PascalCase: `{hook_event_name, session_id, timestamp, cwd, tool_name, tool_input}`, where `tool_input` is "parsed from JSON string when possible".
  - `toolArgs` is typed `unknown`, and the docs' test fixture passes it as a JSON string.
- **`userPromptSubmitted` stdin:** `{sessionId, timestamp, cwd, prompt}`, or `{hook_event_name: "UserPromptSubmit", session_id, timestamp, cwd, prompt}`.
- **Decisions:**
  - A `preToolUse` hook decides with top-level stdout JSON: `permissionDecision` (`allow`, `deny` or `ask`), `permissionDecisionReason` (required with `deny`) and `modifiedArgs`. Empty output means no decision.
  - If any hook denies, the call is blocked. `ask` is denied on the cloud agent and in CLI autopilot.
  - **Config-file `userPromptSubmitted` hooks cannot block a prompt: their output is dropped** (the reference as of 2026-11-09). `modifiedPrompt` works only in SDK hooks.
  - `postToolUse` can return `modifiedResult` and `additionalContext`. Contexts are joined and capped at 10 KB.
  - `agentStop` and `subagentStop` can return `decision: "block"` with a `reason`. After 8 consecutive blocks, the CLI ends the turn anyway.
  - `permissionRequest` (CLI only) returns `behavior` (`allow` | `deny`), `message` and `interrupt`.
- **Exit codes and failure:**
  - **0:** success, and stdout is parsed.
  - **2:** denies `preToolUse` and `permissionRequest`, even if the JSON says allow. On `postToolUseFailure`, stdout is added to the failure the agent sees. On every other event it is a warning.
  - **Other non-zero exit or a crash:** denies `preToolUse` ("hook errored"; 1.0.57). On other events it is logged and skipped.
  - **Timeouts always fail open**, policy hooks included (1.0.67). **HTTP `preToolUse` hooks fail open** on a network error, a timeout or a non-2xx response.
- **stdout parsing:**
  - Lines that are complete `{"type":"progress",…}` objects are removed first. The rest is parsed as one JSON document.
  - Empty output, invalid JSON or two objects count as no output.
  - Output over 10 MiB per call is truncated.
- **Sources and combining (CLI):**
  - The CLI reads, in this order:
    1. policy files (`/etc/github-copilot/policy.d/*.json`, alphabetical)
    2. `.github/hooks/*.json` at the repo root
    3. `~/.copilot/hooks/*.json`
    4. inline `hooks` in `.github/copilot/settings*.json` and `.claude/settings*.json`
    5. `~/.copilot/settings.json`
    6. plugins
  - **Every entry from every source runs**, and no deduplication is documented. The order of files within `.github/hooks/` is not documented.
  - Hooks for one call run in order. Hooks for parallel calls run concurrently.
- **Validation:**
  - In a `.github/hooks` file, a malformed entry is dropped and its siblings still load (1.0.71). Invalid JSON, a bad `version` or a non-array event list rejects the whole file.
  - **Inline settings hooks are strict:** any item-level error rejects the whole `hooks` field.
- **Trust:**
  - Repo hooks load only after folder trust (1.0.8). In `-p` they also load with `GITHUB_COPILOT_PROMPT_MODE_REPO_HOOKS=true` or `COPILOT_ALLOW_ALL`.
  - Since 1.0.111, `-p --trust-folder` started in a subdirectory warns that the root hooks were not loaded.
  - When the session sandbox is on, repo hooks run inside it, and `cwd` or `env` can't widen their access. Policy hooks run on the host.
  - Since 1.0.106, command hooks receive `COPILOT_HOOK_DEPTH`.
- **Cloud agent environment:**
  - Linux, in `/workspace`. `HOME` is `/root`. `GITHUB_COPILOT_API_TOKEN`, `GITHUB_COPILOT_GIT_TOKEN` and `COPILOT_AGENT_PROMPT` are set, but `GITHUB_TOKEN` is not.
  - The network is firewalled.
  - Only `.github/hooks/*.json` is read, with no `settings.json` or plugins, and it must be on the default branch.
- **VS Code:**
  - The Copilot harness uses the CLI hook format. It reads `.github/hooks`, inline `.github/copilot/settings*.json`, `~/.copilot/settings.json` and repository `.claude/settings*.json`.
  - The Local harness reads Claude settings only with `chat.useClaudeHooks` (default `false`). It ignores matchers and sends its own payloads.
- **Claude compatibility:**
  - Claude event names in `.claude/settings.json` are PascalCase, so they get the snake_case payload. Claude's nested `{matcher, hooks: [...]}` groups are accepted (1.0.6, 1.0.10; inline settings since 1.0.66).
  - For PascalCase `PreToolUse` and `PermissionRequest`, Claude matcher rules apply and `tool_name` is the Claude name (1.0.62).
  - `PostToolUse` matchers such as `Edit|Write` are honored (1.0.63).
  - Changelog 1.0.24 says `preToolUse` respects "modifiedArgs/updatedInput". The docs list only `modifiedArgs`.

**Matchers and tool names**
- **Native matchers:**
  - A matcher compiles to `^(?:PATTERN)$` and must match the whole value. An invalid regex skips the entry.
  - `preToolUse`, `postToolUse` and `permissionRequest` match against `toolName`; `notification` against `notification_type`; `preCompact` against `trigger`; `subagentStart` against `agentName`.
- **Claude-format matchers (PascalCase `PreToolUse`):**
  - `*`, `**` or an empty matcher matches every tool.
  - A literal name or an `A|B` list matches either the runtime name or the Claude name.
  - Anything else is a case-sensitive `^(?:P)$` regex, tested against the Claude name (or the runtime name for tools without one).
- **Runtime names and their Claude names:**

  | Runtime tool | Claude name |
  |---|---|
  | `bash`, `powershell` | `Bash` |
  | `view` | `Read` |
  | `create` | `Write` |
  | `edit`, `str_replace_editor`, `apply_patch` | `Edit` |
  | `grep`, `rg` | `Grep` |
  | `glob` | `Glob` |
  | `web_fetch` | `WebFetch` |
  | `web_search` | `WebSearch` |
  | `ask_user` | `AskUserQuestion` |
  | `update_todo` | `TodoWrite` |
  | `task` | `Agent` (`Task` also accepted) |

  - Other tools keep their runtime names: `list_`, `read_`, `stop_` and `write_` `bash`/`powershell`, plus `list_agents`, `read_agent`, `write_agent` and `skill`.
  - `powershell` never appears on the cloud agent.
- **CLI permission kinds:** `read`, `shell`, `write`, `url`, `memory`, and an MCP server name, as in `shell(git:*)` or `MyMCP(create_issue)`.

**Plugins (for the Phase 4 marketplace)**
- **Marketplace lookup order:** `marketplace.json`, then `.plugin/marketplace.json`, `.github/plugin/marketplace.json`, `.claude-plugin/marketplace.json`.
- **Plugin manifest:** a root `plugin.json` that targets Agent Plugins takes precedence. Legacy plugins are checked in the order `.plugin/plugin.json`, `plugin.json`, `.github/plugin/plugin.json`, `.claude-plugin/plugin.json`.
- **Claude settings:** the CLI also reads `enabledPlugins`, `extraKnownMarketplaces`, `disableAllHooks` and `hooks` from `.claude/settings*.json`.

**Not documented (UNVERIFIED)**
- whether Copilot CLI loads `AGENTS.md` twice when `CLAUDE.md` imports it. Only identical files are documented as deduplicated, and Grimoire's block adds marker lines, so a double load is likely;
- whether the VS Code Copilot harness reads `CLAUDE.md` or expands `@` imports; whether the cloud agent expands imports, or reads both `AGENTS.md` and a root `CLAUDE.md`;
- whether the CLI or VS Code honors `excludeAgent`;
- which "usual default tools" the CLI adds without `strict-tools-list`, and whether CLI `tools: []` grants none;
- which runtime tools the `read`, `edit`, `search` and `web` aliases map to in the CLI and VS Code;
- how the CLI and VS Code treat `target` when loading an agent, and whether the cloud agent reads `.claude/agents/`;
- which spellings the CLI accepts: `modelPolicy` or `model-policy`, `reasoningEffort` or `reasoning-effort`;
- how VS Code and the cloud agent resolve a skill name found in both `.claude/skills/` and `.agents/skills/`;
- whether an unknown key (`headersHelper`, `alwaysLoad`, nested `oauth`) makes a CLI `.mcp.json` entry invalid; whether `url` is expanded in the CLI; what VS Code does when `tools` is missing;
- whether stdio servers in VS Code and on the cloud agent inherit the parent environment; whether VS Code reads `.github/mcp.json` or parent-directory `.mcp.json` files;
- how Copilot treats Claude-format `command` plus `args`, `onFailure`, or any unknown key in `.claude/settings.json` hooks. Inline validation is strict, so one bad item might reject the whole `hooks` field;
- whether `hookSpecificOutput.permissionDecision` is honored for PascalCase hooks;
- whether camelCase `toolArgs` arrives as a JSON string or an object in command hooks;
- the argument schemas of Copilot runtime tools (`create`, `edit`, `view`, `bash`, `apply_patch`);
- whether a missing `exec` executable fails closed on `preToolUse`; the maximum `timeoutSec`; the hook stdin size limit; the order of files in `.github/hooks/`; whether `.claude/settings.json` hooks wait for folder trust;
- whether PascalCase `PostToolUse` gets Claude tool names and Claude matcher rules;
- the `toolName` a hook receives for an MCP tool, and how `mcp__server__.*` style matchers behave;
- whether command `userPromptSubmitted` hooks can add context or handle a prompt. Changelog 1.0.44, 1.0.65 and 1.0.111 describe `handled`, `responseContent` and `additionalContext` without naming the hook type, while the reference says command and HTTP output is dropped.

### EDIT-P4: TOML and JSONC entry editing
Sources: https://unpkg.com/strip-json-comments@3.1.1/index.js (the version Gemini CLI v0.68.0 pins, `GEMINI-P4`; fetched 2026-11-17); toml_edit 0.25.19 and jsonc-parser 0.34.0 as locked in `Cargo.lock` (read in `~/.cargo/registry`). Status: Verified, except the gaps listed.

- **Format by path:** Grimoire edits an entry file as TOML when its name ends in `.toml` (`.codex/config.toml`), as JSONC when it is exactly `.gemini/settings.json`, and as strict JSON otherwise. The format comes from the path because removing a locked entry has nothing else to go on.
- **JSONC:** Gemini CLI reads settings as `JSON.parse(stripJsonComments(text))`.
  - Grimoire finds comments with jsonc-parser, blanks them, and requires the rest to parse as strict JSON, so trailing commas, single quotes and raw control characters in strings are refused, as Gemini CLI refuses them.
  - strip-json-comments 3.1.1 ends a `//` comment only at `\r\n` or `\n` (index.js lines 51 and 57), while jsonc-parser also ends it at a lone `\r`. Grimoire refuses a `//` comment that ends in a lone `\r`, where the two would disagree.
  - It blanks a comment with `replace(/\S/g, ' ')` (index.js line 5), so whitespace inside a comment reaches `JSON.parse`, which accepts only space, tab, CR and LF. Grimoire keeps JavaScript's whitespace when blanking (Rust's set without U+0085, plus U+FEFF), so serde_json refuses the same files: a non-breaking space, form feed, vertical tab, U+2028 or U+3000 in a comment.
- **TOML:** toml_edit keeps the user's tables, comments and order. Grimoire owns whole member keys only (`/mcp_servers/<id>`), writes the entry as a standard table, and refuses `null`, array-element keys, a plain value where it would add a `[parent]` header, files that don't parse, and duplicate keys. A datetime or non-finite float on disk never matches a desired value, so it shows as a conflict rather than being overwritten.
  - toml_edit writes every line ending as `\n`, drops a byte-order mark and adds a final newline. So an unchanged file is returned as its original bytes, and a changed one gets its CRLF line endings (each lone `\n`; multi-line strings already hold `\r\n`) and byte-order mark back.
  - toml_edit attaches comment lines before a `[header]` to that header. They usually belong to the item above, such as a setting commented out at the end of the previous table, so when Grimoire removes its table it moves them to the next header or the end of the file.
- **Comments inside an entry:** values are hashed without their comments, so a comment the user adds inside Grimoire's TOML table or JSONC object would be lost by an update or a removal. The planner makes that update or removal a conflict (`Conflict::Commented`).
- **UNVERIFIED:** toml_edit 0.25 targets TOML 1.1. What Grimoire writes uses only TOML 1.0 syntax (single-line inline tables, `\uXXXX` escapes), but which TOML version Codex's own parser accepts was not checked.
