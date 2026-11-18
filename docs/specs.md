# Specification register

This file lists every external format, protocol, registry or tool that Grimoire implements against. Each entry gives the primary source, the version or revision that was read, and the date it was checked.

**Rules for this file**

- Re-verify an entry before you implement or change the code that depends on it. Update the entry in the same change.
- Facts here were read from the primary source on the date shown. Anything not confirmed is marked **UNVERIFIED** and must not be implemented from memory.
- Many sources are unversioned living docs. For those, "Version" records what the page or its release channel showed on the check date.
- Statuses: **Verified** means all facts below were read from the source. **Partial** means some gaps are listed. **Unverified** means the source was unreachable or silent.

## Register

Rows were checked 2026-11-16 unless they say otherwise; the client rows were re-checked 2026-11-17 for Phase 4, the registry, release-tooling and supply-chain rows on 2026-11-17 for Phase 5, the container rows on 2026-11-17 for Phase 6, and the community-file, security-policy, README and versioning rows on 2026-11-17 for Phase 7. Section IDs below link each row to its notes.

| ID | Spec / source | Version or revision checked | Primary URL | Status |
|---|---|---|---|---|
| CC-* | Claude Code: memory, skills, subagents, settings/hooks, `.mcp.json`, plugins, marketplaces | CLI 2.1.326 (npm `latest`); live docs | https://code.claude.com/docs/en/ | Verified; plugins and marketplaces re-checked 2026-11-17 in CC-PLUGIN-P4 and CC-MKT-P4, `.mcp.json` variables in RUN-P6 (CLI 2.1.327) |
| SKILLS | Agent Skills specification | unversioned; spec file @ `217be54873` (2026-08-04) | https://agentskills.io/specification | Verified |
| AGENTSMD | AGENTS.md | unversioned; repo @ `d001185d79` (2026-09-10) | https://agents.md | Verified (convention, not normative) |
| CODEX-* | OpenAI Codex CLI: AGENTS.md, config/MCP, skills, agents, hooks, rules, plugins | `rust-v0.169.0` (2026-11-13); prerelease `rust-v0.171.0-alpha.11` | https://learn.chatgpt.com/docs/ ; https://github.com/openai/codex | Verified; re-checked 2026-11-17 in CODEX-*-P4 (some facts from source only), and the MCP stdio launch at `rust-v0.169.1` in RUN-P6 |
| CURSOR-* | Cursor: rules, MCP, skills, subagents, hooks | release notes IDE 3.27 / CLI v2026.10.22; shipped IDE 3.30.14 / CLI 2026.11.13 | https://cursor.com/docs/ | Partial; re-checked 2026-11-17 in CURSOR-P4 |
| GEMINI-* | Gemini CLI: context, settings/MCP, skills, subagents, hooks | v0.68.0 (2026-11-10), source and docs @ tag; preview v0.69.0-preview.0 | https://github.com/google-gemini/gemini-cli | Verified from source; re-checked 2026-11-17 in GEMINI-P4; settings expansion corrected in RUN-P6 |
| COPILOT-* | GitHub Copilot: instructions, agents, skills, MCP, hooks | CLI v1.0.112; VS Code 1.146.0; docs fetched 2026-11-17 | https://docs.github.com/en/copilot ; https://code.visualstudio.com/docs | Partial; re-checked 2026-11-17 in COPILOT-P4 |
| MCP-SPEC | Model Context Protocol | **2026-07-28** (current final) | https://modelcontextprotocol.io/specification/2026-07-28 | Verified |
| RMCP | rmcp (official Rust SDK) | 3.10.0 (2026-11-12); supports 2026-07-28 | https://github.com/modelcontextprotocol/rust-sdk | Verified |
| MCP-REGISTRY | Official MCP Registry / `server.json` | registry 1.8.9; schema `2025-12-11`; **preview** | https://registry.modelcontextprotocol.io | Verified |
| MCPB | MCP bundles | manifest 0.4; `@anthropic-ai/mcpb` 2.1.2 | https://github.com/modelcontextprotocol/mcpb | Verified (not used in v1) |
| CRATES-TP | crates.io trusted publishing | docs @ 2026-10-02; `crates-io-auth-action` v1.0.6 | https://crates.io/docs/trusted-publishing | Verified; re-checked 2026-11-17 in RUST-REGISTRY-P5 |
| NPM-TP | npm trusted publishing and token policy | docs @ 2026-10-06; npm 11.21.2 / 12.2.2 | https://docs.npmjs.com/trusted-publishers | Partial (docs conflict); re-checked 2026-11-17 in NPM-P5 |
| PYPI-TP | PyPI trusted publishing | docs fetched 2026-11-16; `gh-action-pypi-publish` v1.14.3 | https://docs.pypi.org/trusted-publishers/ | Verified; re-checked 2026-11-17 in PYPI-P5 |
| MATURIN | maturin binary wheels | 1.16.0; maturin-action v1.51.0 | https://www.maturin.rs | Verified; not used since Phase 5 (PYPI-P5) |
| BINSTALL | cargo-binstall metadata | 1.27.0 | https://github.com/cargo-bins/cargo-binstall | Verified; re-checked 2026-11-17 in RUST-REGISTRY-P5 |
| BREW | Homebrew taps | brew 7.0.14 | https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap | Partial: re-checked from source on 2026-11-17 in BREW-SCOOP-P5; whether `brew style` passes `url` nested in two `on_*` blocks is UNVERIFIED |
| WINGET | winget-pkgs manifests | schema 1.28 (community repo) | https://github.com/microsoft/winget-pkgs | Partial (Learn page stale); re-checked 2026-11-17 in WINGET-P5 |
| SCOOP | Scoop manifests and buckets | Scoop 0.6.0; wiki unversioned | https://github.com/ScoopInstaller/Scoop/wiki | Verified; re-checked 2026-11-17 in BREW-SCOOP-P5 (the wiki lags the source), gaps listed |
| DIST | dist (cargo-dist) | 0.34.0 (2026-11-02) | https://github.com/axodotdev/cargo-dist | Verified; re-checked 2026-11-17 in INSTALLERS-P5 (not adopted) |
| RELEASE-PLZ | release-plz | 0.4.3 (2026-11-02) | https://release-plz.dev | Verified; re-checked 2026-11-17 in RUST-REGISTRY-P5, and its 0.x bump rules in VERSIONING-P7 |
| RUNNERS | GitHub-hosted runners | docs fetched 2026-11-16 | https://docs.github.com/en/actions/reference/runners/github-hosted-runners | Partial; re-checked 2026-11-17 in ACTIONS-P5, and their Docker in BUILD-P6 and RUN-P6 |
| GHA-SEC | Actions hardening, lockfiles, SHA-pin policy, environments, OIDC | docs fetched 2026-11-16 | https://docs.github.com/en/actions/reference/security/secure-use | Verified; re-checked 2026-11-17 in ACTIONS-P5 |
| ATTEST | Artifact attestations, immutable releases | `actions/attest` v4.2.2 | https://github.com/actions/attest | Verified; re-checked 2026-11-17 in SUPPLY-CHAIN-P5, and for images in REGISTRY-P6 |
| COSIGN | Sigstore cosign | v3.1.4; installer v4.1.3 | https://github.com/sigstore/cosign | Verified: the Fulcio SAN was verified 2026-11-17 in SUPPLY-CHAIN-P5, and GHCR referrers settled 2026-11-17 in REGISTRY-P6 (no referrers API; the `sha256-<hex>` fallback tag) |
| SBOM | cargo-cyclonedx, cargo-auditable, buildx attestations | 0.5.10 / 0.7.7 / docs fetched 2026-11-16 | https://github.com/CycloneDX/cyclonedx-rust-cargo | Verified; re-checked 2026-11-17 in SUPPLY-CHAIN-P5, and BuildKit's SBOM scanner in BUILD-P6 |
| BASE-IMAGE | distroless static, Chainguard static, OCI annotations | `static-debian13`; image-spec v1.1.1 | https://github.com/GoogleContainerTools/distroless | Verified; re-checked 2026-11-17 in BASE-IMAGE-P6 (the pinned digest changed) |
| SCAN | Trivy incident; Grype / scan-action | trivy v0.76.2; grype v0.122.0; scan-action v7.4.2 | https://github.com/anchore/scan-action | Verified; re-checked 2026-11-17 in SCAN-P6 (pin Grype v0.121.3; scan-action not used) |
| DENY | cargo-deny | 0.21.0; action v2.1.2 | https://github.com/EmbarkStudios/cargo-deny | Verified |
| UNUSED-DEPS | Cargo unused-dep lints, cargo-machete, cargo-shear, cargo-udeps | Rust 1.100.0; 0.10.1 / 1.14.1 / 0.1.61 | https://doc.rust-lang.org/stable/cargo/reference/lints.html | Verified |
| SECRETS | GitHub secret scanning, gitleaks, private vulnerability reporting | gitleaks v8.30.1 | https://docs.github.com/en/code-security | Verified; private vulnerability reporting re-checked 2026-11-17 in SECURITY-POLICY-P7 (now enabled) |
| DEPENDABOT | Dependabot options | docs fetched 2026-11-16 | https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference | Verified; Docker re-checked 2026-11-17 in BASE-IMAGE-P6 (cooldown fails open outside Docker Hub) |
| LYCHEE | lychee link checker | 0.24.2; action v2.9.0 | https://github.com/lycheeverse/lychee-action | Verified; the v0.24.2 tarball checked locally 2026-11-17 for Phase 7 (its SHA-256 matches `ci.yml`) |
| RUST | Rust stable toolchain | 1.100.0 (channel 2026-11-12) | https://static.rust-lang.org/dist/channel-rust-stable.toml | Verified; re-checked 2026-11-17 in ACTIONS-P5 |
| CRATES | Candidate crate versions | see table | https://crates.io | Verified |
| YAML | Frontmatter YAML parser (serde-saphyr) | 2.1.1 | https://github.com/bourumir-wyngs/serde-saphyr | Verified (Phase 1) |
| SPDX-CRATE, MARKDOWN | spdx and pulldown-cmark behaviour | 0.13.6 / 0.13.6 | https://github.com/EmbarkStudios/spdx ; https://github.com/pulldown-cmark/pulldown-cmark | Verified (Phase 1) |
| SCHEMARS | schemars draft, map keys, key order | 1.2.3 | https://docs.rs/schemars/1.2.3 | Verified (Phase 1) |
| CARGO-LINTS | `[lints.cargo]`, workspace globs, inheritance | Cargo 1.96.0 / 1.99.0 / 1.100.0 | https://doc.rust-lang.org/cargo/reference/lints.html | Verified (Phase 1) |
| CI-PINS | Pinned actions and CI tool binaries | see table | per pin | Verified (Phase 1); re-checked 2026-11-17 |
| ZIZMOR | zizmor audits and personas | 1.32.0 | https://docs.zizmor.sh | Verified (Phase 1); re-checked 2026-11-17 in ACTIONS-P5 |
| CC-P2 | Claude Code facts the `claude-code` adapter writes against | CLI 2.1.326; docs fetched 2026-11-16 | https://code.claude.com/docs/en/ | Verified (Phase 2), gaps listed |
| FS-CRATES | cap-std, cap-fs-ext, cap-tempfile | 4.0.3 | https://github.com/sunfishcode/cap-std | Verified (Phase 2) |
| EDIT-CRATES | jsonc-parser (CST), toml_edit, similar | 0.34.0 / 0.25.19 / 3.2.1 | https://github.com/dprint/jsonc-parser | Verified (Phase 2), with one finding |
| CC-P3 | Claude Code hooks, remote and stdio `.mcp.json`, OAuth client | CLI 2.1.326 (`latest`) / 2.1.311 (`stable`); CHANGELOG @ `ccc86e33cd`; docs fetched 2026-11-16 | https://code.claude.com/docs/en/hooks.md | Verified (Phase 3), gaps listed |
| MCP-P3 | MCP server duties: Streamable HTTP, stateless lifecycle, resource-server authorization | spec tag `2026-07-28` (`5f5440bb26`), main `05aeb07`; RFC 9728/6750/8707/9068/8725; OAuth 2.1 -13 (-17 latest) | https://modelcontextprotocol.io/specification/2026-07-28 | Verified (Phase 3) |
| RMCP-P3 | rmcp server API, defaults and divergences | rmcp 3.10.0, tag `rmcp-v3.10.0` (`53e64a93c8`) | https://github.com/modelcontextprotocol/rust-sdk | Verified (Phase 3), one claim corrected |
| AUTH-CRATES | Server-side JWT validation crates under `deny.toml` | jsonwebtoken 12.0.0, rsa 0.10.0, p256 0.14.0, hyper 1.12.0, hyper-util 0.1.21; advisory-db `02e2517f45`; cargo-deny 0.21.0 / 0.20.2 | https://github.com/Keats/jsonwebtoken | Verified (Phase 3), two claims corrected |
| SECRET-FORMATS | Vendor credential formats for secret-guard | vendor docs fetched 2026-11-16; gitleaks v8.30.1; GitLab rules @ `c8396d46d7` | https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/about-authentication-to-github#githubs-token-formats | Partial (many lengths third-party; OpenAI UNVERIFIED) |
| CONTENT-P3 | Conventional Commits, Agent Skills, ADR formats | CC 1.0.0 @ `7d293dc59e`; Agent Skills @ `83021e2be4`; MADR 4.0.0 (`2475fe1973`) | https://www.conventionalcommits.org/en/v1.0.0/ | Verified (Phase 3); SemVer re-checked 2026-11-17 in VERSIONING-P7 (tag `v2.0.0` and semver.org differ on build metadata) |
| CC-PLUGIN-P4, CC-MKT-P4 | Claude Code plugin manifest, components, marketplace file, dependencies | CLI 2.1.326 (`latest`) / 2.1.311 (`stable`); CHANGELOG @ `bc1e8a88cb44`; docs fetched 2026-11-17 | https://code.claude.com/docs/en/plugins/manifest-reference.md | Verified (Phase 4), gaps listed |
| CC-CI-P4 | `claude plugin validate` and installing Claude Code in CI | 2.1.311 and 2.1.326 manifests and signatures checked 2026-11-17 | https://code.claude.com/docs/en/plugins/cli-reference.md | Partial (offline behaviour UNVERIFIED) |
| CODEX-P4 … CODEX-PLUGINS-P4 | Codex AGENTS.md, trust, MCP, skills, agents, hooks, tools, plugins | `rust-v0.169.0` (`d32063f96c2d`), diffed against `rust-v0.171.0-alpha.11` | https://github.com/openai/codex | Verified (Phase 4), gaps listed |
| CURSOR-P4 | Cursor rules, MCP, skills, subagents, hooks | docs fetched 2026-11-17; shipped IDE 3.30.14 | https://cursor.com/docs/ | Partial (Phase 4), many gaps listed |
| GEMINI-P4 | Gemini CLI context, settings, trust, MCP, skills, subagents, hooks | v0.68.0 (`5a5c7180ff`) source | https://github.com/google-gemini/gemini-cli | Verified from source (Phase 4), gaps listed |
| COPILOT-P4 | Copilot instructions, agents, skills, MCP, hooks, plugins | CLI 1.0.112; VS Code 1.146.0; docs fetched 2026-11-17 | https://docs.github.com/en/copilot | Partial (Phase 4; docs only, no source) |
| EDIT-P4 | TOML and JSONC entry editing | strip-json-comments 3.1.1; toml_edit 0.25.19; jsonc-parser 0.34.0; checked 2026-11-17 | https://unpkg.com/strip-json-comments@3.1.1/index.js | Verified (Phase 4); which TOML version Codex accepts is UNVERIFIED |
| RUST-REGISTRY-P5 | release-plz, `cargo publish --workspace`, crates.io trusted publishing and metadata, cargo-binstall | release-plz 0.4.3 (`92e907a14715`); Cargo @ `3acc79c0c841` (1.100.0); crates.io @ `1aea7ef2ddea`; cargo-binstall v1.27.0 / v1.26.2 | https://github.com/release-plz/release-plz | Verified (Phase 5), gaps listed |
| NPM-P5 | npm staged and trusted publishing, platform packages, the launcher | npm 12.2.2 / 11.21.2; setup-node v7.1.0; Node 24.22.3; docs fetched 2026-11-17 | https://docs.npmjs.com/trusted-publishers | Partial (Phase 5): npm/cli#9969 open, placeholder behaviour UNVERIFIED |
| PYPI-P5 | Binary-only wheels, PyPI upload checks, pending publishers, `gh-action-pypi-publish` | wheel and core-metadata specs; warehouse @ `c2f79b4a6fb0`; gh-action-pypi-publish v1.14.3; twine 7.0.0 | https://packaging.python.org/en/latest/specifications/binary-distribution-format/ | Verified (Phase 5), gaps listed |
| BREW-SCOOP-P5 | Prebuilt formula in a tap, local-tap audits, tap trust, Scoop manifest, schema and licence syntax | Homebrew 7.0.14; Homebrew/actions 2026.11.09.1; Scoop v0.6.0 | https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap | Verified from source (Phase 5), gaps listed; nothing run |
| WINGET-P5 | winget manifests 1.28.0, validation, submission, portable aliases | winget-cli v1.29.380 (schema 1.28.0); winget-pkgs @ `b192fe0471e6` | https://github.com/microsoft/winget-pkgs | Verified (Phase 5); re-checked 2026-11-18 (the runner's winget is too old for 1.28.0, so CI installs v1.29.380); installing it on `windows-2025` UNVERIFIED |
| SUPPLY-CHAIN-P5 | Attestations, gh verification, cosign, the Fulcio SAN, immutable releases, SBOMs, checksums | attest v4.2.2; gh 2.103.3; cosign v3.1.4 / installer v4.1.3; Fulcio v1.10.0; cargo-cyclonedx 0.5.10; cargo-auditable 0.7.7 | https://github.com/actions/attest | Verified (Phase 5), gaps listed; the GHCR referrers gap settled in REGISTRY-P6 |
| ACTIONS-P5 | Runner images, Rust targets and build flags, `cargo metadata`, artifacts, environments, OIDC, `GITHUB_TOKEN` events | runner-images @ `5d331217d4`; Rust 1.100.0; upload-artifact v7.0.2 / download-artifact v8.0.2 | https://docs.github.com/en/actions/reference/runners/github-hosted-runners | Verified (Phase 5), gaps listed; Linux images re-checked 2026-11-17 in RUN-P6 |
| INSTALLERS-P5 | The dist installer spike; ShellCheck and PSScriptAnalyzer for Grimoire's scripts; the platform and tool behaviours the scripts rely on | dist v0.34.0; ShellCheck v0.11.0; PSScriptAnalyzer 1.25.0; curl 8.23.1 manual; Microsoft Learn and Apple pages fetched 2026-11-17 | https://github.com/axodotdev/cargo-dist | Verified (Phase 5), gaps listed |
| BASE-IMAGE-P6 | Distroless base: pins, tags, contents, signatures; OCI annotations; GHCR metadata; Dependabot for a Dockerfile | distroless @ `0af09fb8f491`; `static-debian13:nonroot` indexes `25ac8f0c` (pinned, 2026-10-31) and `aafe9936` (2026-11-12); image-spec v1.1.1; dependabot-core v0.405.0 | https://github.com/GoogleContainerTools/distroless | Verified (Phase 6), gaps listed; base signatures checked locally |
| BUILD-P6 | BuildKit and buildx: drivers, builder image, provenance, SBOM, attestation storage, OCI export, reproducibility, build checks | BuildKit v0.34.2; buildx v0.38.2; buildkit-syft-scanner 1.12.1 (syft v1.51.0); runner-images @ `fb099341ddaf` | https://github.com/moby/buildkit | Verified (Phase 6), gaps listed; build and reproducibility checked locally |
| REGISTRY-P6 | GHCR, cosign image signatures, `actions/attest` for images, OCI push tools, local registries | distribution-spec v1.1.1; cosign v3.1.4; attest v4.2.2; crane v0.22.1; GitHub docs @ `49a012fa50b4` | https://github.com/github/docs (`content/packages/`) ; https://github.com/sigstore/cosign | Verified (Phase 6), gaps listed; crane push and offline signing checked locally, GHCR not used |
| SCAN-P6 | Grype as the image gate: pins, release signing, DB, scan sources, ignore rules, VEX, exit codes | Grype v0.121.3 (syft v1.57.1); DB schema v6.1.10 built 2026-11-17; scan-action v7.4.2 (not used) | https://github.com/anchore/grype | Verified (Phase 6), gaps listed; the gate checked locally |
| RUN-P6 | Running the image: `docker run` stdio and signals, hardening flags, ports, bind mounts, ownership, MCP clients that start containers | docker/cli v29.10.0; moby docker-v29.10.0; runc v1.5.3; Podman v6.2.0; Claude Code 2.1.327; Codex `rust-v0.169.1`; Gemini CLI v0.68.0; VS Code 1.146.0 | https://github.com/docker/cli | Verified from docs and source (Phase 6), gaps listed; partly run locally on rootless Docker 29.9.3 |
| SIGN-P6 | Signing from a reusable workflow: OIDC claims, the Fulcio certificate, who else can call it, cosign and gh pins, `actions/attest`, workflow_call mechanics, zizmor and gh-actions-lock | Fulcio v1.10.1; cosign v3.1.4; gh v2.103.3; sigstore-go v1.3.0; attest v4.2.2; zizmor 1.32.0; gh-actions-lock v0.4.1; GitHub docs @ `49a012fa50b4` | https://github.com/sigstore/fulcio (`config/identity/config.yaml`) | Verified (Phase 6, after the review): 42 facts, 40 confirmed, 1 corrected, 1 left UNVERIFIED; gaps listed |
| COMMUNITY-P7 | GitHub community health files, the community profile, PR templates, CODEOWNERS, Grimoire's rulesets and labels | github/docs @ `49a012fa50b4`; rest-api-description @ `c9c95fe34ced`; live REST reads 2026-11-17 | https://github.com/github/docs (`content/communities/`) | Verified (Phase 7): 60 facts, 57 confirmed, 3 corrected; gaps listed |
| ISSUE-FORMS-P7 | GitHub issue forms, the template chooser, SchemaStore's schemas, check-jsonschema, how submitted forms render | github/docs @ `49a012fa50b4`; SchemaStore @ `0a9268d745bc`; check-jsonschema 0.39.0 | https://github.com/github/docs (`content/communities/using-templates-to-encourage-useful-issues-and-pull-requests/`) | Verified (Phase 7; issue forms are public preview): 45 facts, 38 confirmed, 5 corrected, 2 UNVERIFIED; gaps listed |
| SECURITY-POLICY-P7 | Private vulnerability reporting, `SECURITY.md`, repository advisories and CVEs, Scorecard's Security-Policy check, Contributor Covenant 3.0 | github/docs @ `49a012fa50b4`; Scorecard v5.6.0; Contributor Covenant @ `8d00b9180c88`; cve-website @ `0d524c767399` | https://github.com/github/docs (`content/code-security/`) | Verified (Phase 7): 42 facts, 41 confirmed, 1 corrected; gaps listed |
| README-RENDER-P7 | One README on GitHub, crates.io, PyPI and npm and in the release archives | crates.io @ `dfefd6318249` (comrak 0.59.0, ammonia 4.2.5); readme_renderer 46.0; warehouse @ `c2f79b4a6fb0`; Cargo 1.100.0; npm 11.19.1 / 12.2.3 | https://github.com/rust-lang/crates.io (`crates/crates_io_markdown/`) | Verified from source (Phase 7): 54 facts, 50 confirmed, 3 corrected, 1 UNVERIFIED (npmjs.com rendering); gaps listed |
| VERSIONING-P7 | SemVer 2.0.0, Cargo requirements, MSRV and `cargo install`, release-plz bumps, clap 4.6.8 deprecation and help APIs, CLI doc generators | semver.org (Last-Modified 2026-10-16); Cargo 1.100.0 (`3acc79c0c841`) and 1.96–1.99; release-plz 0.4.3; clap 4.6.8; clap_mangen 0.3.4; clap-markdown 0.1.5 | https://semver.org/spec/v2.0.0.html | Verified (Phase 7): 63 facts, 56 confirmed, 7 corrected; gaps listed; nothing built or run |

**Not yet researched, to verify in the phase that needs them:**

| Topic | Phase |
|---|---|
| ~~The exact Fulcio identity string for keyless verification docs~~ Answered in SUPPLY-CHAIN-P5: `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`; superseded in Phase 6 by `…/sign.yml@refs/heads/main` with caller pins (SIGN-P6) | 5 |
| ~~GHCR OCI 1.1 referrers support for cosign v3~~ Answered in REGISTRY-P6: GHCR has no referrers API (it returns 404), so cosign lists signatures in the `sha256-<hex>` fallback tag | 6 |
| ~~`winget validate` availability on GitHub Windows runners~~ Answered in WINGET-P5: not on `windows-11-arm`; undocumented and conflicting on `windows-2025`, so the CI step was best effort. Re-checked 2026-11-18: `windows-2025` has a winget older than schema 1.28.0, so the dry-run now installs a pinned v1.29.380 | 5 |
| ~~Whether npm staged publishing can create a brand-new package~~ Answered in NPM-P5: yes, with a session or a granular token (not OIDC), leaving a public `0.0.0-stage` placeholder | 5 |

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
- **Re-checked 2026-11-17 (RUN-P6; CLI 2.1.327 `latest`, 2.1.311 `stable`):**
  - Claude Code sets `CLAUDE_PROJECT_DIR` in a stdio server's environment, not in its own. So in the `command` or `args` of a project `.mcp.json` entry, and now also of a local- or user-scope entry in `~/.claude.json`, it needs a default such as `${CLAUDE_PROJECT_DIR:-.}`. Plugin MCP configs substitute it directly.
  - An empty value counts as set. `MCP_TIMEOUT` (startup) defaults to 30000 ms.
  - A stdio server's working directory is still undocumented (UNVERIFIED). A `docker run` entry here therefore names the project by its absolute path.

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
- **Re-checked 2026-11-17 (RUN-P6, `rust-v0.169.1`):** unchanged. The compare from `rust-v0.169.0` touches only model-provider, `core/src/client.rs`, model and TUI files. The stdio fallback working directory and the servers' process group are in CODEX-MCP-P4.

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
- **Re-checked 2026-11-17 (RUN-P6, v0.68.0 source), corrected:**
  - Unset variables are not blanked everywhere. At load, `resolveEnvVarsInObject` leaves an unset `$VAR` or `${VAR}` with no default as literal text. Only `env` and `headers` get the second, connect-time pass, which blanks unset variables (GEMINI-P4).
  - `cwd` is passed only when configured; otherwise a stdio server inherits the CLI's cwd. In an untrusted folder, a stdio server fails to start.
  - The inherited environment drops names containing `CERT`, so `DOCKER_CERT_PATH` is dropped. `DOCKER_HOST`, `DOCKER_CONTEXT` and `DOCKER_CONFIG` match none of the name rules. Under strict sanitization, which v0.68.0 turns on when `GITHUB_SHA` is set or `SURFACE=Github` (so in GitHub Actions), a stdio server inherits only the always-allowed names (`PATH`, `HOME`, `LANG`, `SHELL`, `TMPDIR`, `USER`, `LOGNAME`, `TERM`, `COLORTERM` and a few platform names), `GEMINI_CLI_*`, `GIT_CONFIG_*` and the names in `security.environmentVariableRedaction.allowed`; so `DOCKER_HOST`, `DOCKER_CONTEXT` and `DOCKER_CONFIG` are dropped too, unless they are set in the entry's `env` or allow-listed (packages/core/src/services/environmentSanitization.ts and src/tools/mcp-client.ts at v0.68.0, checked in the Phase 6 review).

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
- **Re-checked 2026-11-17 (RUST-REGISTRY-P5):** still accurate. From crates.io's source:
  - The owner and repository names are compared case-insensitively. `repository_owner_id` must match the ID captured when the config was created; the repository ID isn't checked.
  - The workflow filename comes from `workflow_ref` and must match exactly. The environment is compared case-insensitively, and a config without one accepts any.
  - `sub` isn't read, so the immutable `sub` doesn't matter.
  - One token covers every matching crate, and a crate has at most 5 configs. Creating one needs a verified email and a linked GitHub account.
  - `trustpub_only` is set with `PATCH /api/v1/crates/{name}` or on the crate's settings page.

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
- **Re-checked 2026-11-17 (NPM-P5):**
  - **Status:** the providers are GitHub Actions on GitHub-hosted runners, GitLab.com shared runners and CircleCI cloud. Self-hosted runners aren't supported, and CircleCI gets no provenance.
  - **Package must exist:** true for trusted publishing and `npm trust`. But **staged publishing can create a new package**, with a session or a granular token (not OIDC); it publishes a public `0.0.0-stage` placeholder.
  - **Version line:** `staged-publishing.mdx` last changed on 2026-11-10. Tag commits: v12.2.2 `60e11343e61316d966c86805d803df5a0eb99230`, v11.21.2 `1c66798c9def37e12509ccebb5b08cddf247c2e8`.
  - **Requirements, added:** dist-tags over OIDC need npm 11.21.0 or 12.2.0 and an opt-in. npm 12 needs Node `^22.22.2 || ^24.15.0 || >=26.0.0`. Node 24.22.3 bundles npm 11.19.1 and Node 22.23.4 bundles npm 10.9.9, so check the npm version explicitly.
  - **Workflow rules, added:** all fields are case-sensitive (`Grimoire`), and `repository.url` must match exactly. OIDC is refused for `pull_request_target` and `issue_comment`. A `workflow_call` or `workflow_dispatch` publish matches the calling workflow's filename. `npm publish --dry-run` in a job with `id-token: write` performs a real OIDC exchange.
  - **Tokens, added:** `npm login` gives 2-hour sessions with 2FA for publishing. Stage-only granular tokens exist. January 2027 is a target date.
  - **Conflict:** still there at v12.2.2. The 2026-09-03 changelog says several configurations are GA, with no maximum.
  - **Open bug:** npm/cli#9969, trusted publishing for repositories with the immutable `sub`, which includes this one.

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
- **Re-checked 2026-11-17 (PYPI-P5):** every fact above still holds. Additions and corrections:
  - **Publisher matching:** owner (pinned by ID), repository (matched exactly, so `Grimoire`), workflow filename (`.yml` or `.yaml`), an optional environment (case-insensitive) and the project name. `sub` is unchecked, so the immutable `sub` has no effect. `pull_request_target` and `issue_comment` are refused.
  - **Pending publishers expire 30 days after creation** (warehouse source, not the docs).
  - **Reusable workflows:** not a real conflict. PyPI matches the workflow file named in `job_workflow_ref`, which for a called workflow is the called file. warehouse#11096 is about workflows in *another* repository. A same-repository reusable workflow works if the publisher names the called file. Grimoire publishes from the top-level `release.yml`.
  - **gh-action-pypi-publish:** Linux only, Docker-based, and its image is pinned by tag, not digest (PYPI-P5). It has no dry-run input.
  - **maturin is not used** (ADR-0009): it can't package a prebuilt binary.
  - **maturin-action:** it downloads maturin without a checksum check and defaults to `latest`. It resolves version requirements from a manifest on its main branch at run time. Docker mode uses `curl | tar`, `curl | sh` when rustup or uv is missing, and `:latest` images.
  - **maturin details, for reference:** `bin` builds get no musllinux tag by default; at most one manylinux and one musllinux tag are allowed. maturin embeds a CycloneDX 1.5 SBOM unless `[tool.maturin.sbom] rust = false`. `license-files` globs can't contain `..`. Correct `License-Expression` output dates from 1.9.2. The source detects `bin` whenever there is no pyo3 or uniffi dependency and no cdylib, so keep `bindings = "bin"` explicit.

### BINSTALL: cargo-binstall
- **Version:** v1.27.0 (2026-11-11).
- **Source:** https://github.com/cargo-bins/cargo-binstall `SUPPORT.md`, `SIGNING.md`. Checked 2026-11-16. Status: Verified.
- **Metadata:** `[package.metadata.binstall]` takes `pkg-url`, `bin-dir`, `pkg-fmt` (default `tgz`), `disabled-strategies`, and per-target `overrides`.
- **URL guessing:** default patterns include `{ name }-{ target }-v{ version }{ archive-suffix }` under `{ repo }/releases/download/v{ version }/`.
- **Signing:** **minisign only**, needing an explicit `pubkey`. Signatures are verified "if they are available". Sigstore and GitHub attestations are **not** documented.
- **Telemetry:** the quick-install strategy sends anonymized stats by default. Grimoire's docs should mention `--disable-telemetry`.
- **Re-checked 2026-11-17 (RUST-REGISTRY-P5):**
  - v1.27.0 is the latest but was under 7 days old. The newest eligible release is v1.26.2 (2026-10-31, `851112458b5f28a29e5bc14a690c3f283d9dcc6f`).
  - Also: the `signing {algorithm, pubkey, file}` key, `cfg(...)` override keys, `pkg-fmt` values `tar`, `tbz2`, `tgz`, `txz`, `tzstd`, `zip` and `bin`, and the full list of template variables (RUST-REGISTRY-P5).
  - binstall verifies only the crates.io crate checksum, **not the prebuilt archive's**. Sigstore is still unsupported in v1.27.0. cargo-binstall's own releases have no checksum file, only minisign `.sig` files and GitHub attestations.
  - Telemetry goes only through `quick-install`; Grimoire's metadata disables that strategy.

### BREW: Homebrew taps
- **Version:** brew 7.0.14 (2026-11-16). Doc commits: tap guide 2026-10-25, Acceptable-Formulae 2026-07-23.
- **Source:** https://docs.brew.sh/How-to-Create-and-Maintain-a-Tap, …/Acceptable-Formulae, …/Acceptable-Casks, …/Tap-Trust; https://brew.sh/2026/06/11/homebrew-6.0.0/. Checked 2026-11-16. Status: Partial (no rule found on prebuilt-binary formulae in *third-party* taps).
- **Naming:** `brew tap <user>/<repo>` clones `github.com/<user>/homebrew-<repo>`. Formulae go in `Formula/`.
- **Tap trust:** since Homebrew 6.0.0, "Non-official taps require explicit trust by default". Users run `brew trust`, or install by fully qualified name.
- **Workflows:** `brew tap-new` generates `tests.yml`, `publish.yml` (manually dispatched), **`autobump.yml`** and `dependabot.yml`.
- **Placement:** homebrew/core wants builds from source; "Proprietary or platform-specific binary-only software belongs in a cask"; third-party taps take software outside those criteria.
- **Re-checked 2026-11-17 (BREW-SCOOP-P5):** status is now **Verified**, from source; nothing was run.
  - The binary-URL cop and the source-only audits apply only to homebrew/core, and Package-Acceptance-Policy says other software can live in a third-party tap.
  - **Tap trust:** `brew install jabbott-iii/tap/agent-grimoire` trusts only that formula, and only for install, reinstall and upgrade. `brew trust --formula jabbott-iii/tap/agent-grimoire`, or `brew trust jabbott-iii/tap` for the whole tap, also work. A local tap used by CI needs `brew trust` too.
  - **Workflows:** `autobump.yml` runs a daily `brew bump --no-fork --open-pr --formulae --bump-synced --tap=…` with `GITHUB_TOKEN` (`contents` and `pull-requests` write). It can't rewrite `url`/`sha256` nested in `on_*` blocks and doesn't bump casks. It needs "Allow GitHub Actions to create and approve pull requests", and the runs of the PRs it opens need approval. Grimoire uses its own sync workflow instead (ADR-0009).

### WINGET
- **Version:** the community repo requires manifest schema **1.28** (the winget-cli "latest" schema is 1.31.0). wingetcreate v1.28.1.0 (2026-11-11); Komac v2.17.2 (2026-11-03).
- **Source:** https://github.com/microsoft/winget-pkgs (`doc/Authoring.md` 2026-11-04, `doc/manifest/schema/1.28.0/installer.md`, PR template). Checked 2026-11-16. Status: Partial (the Learn page still shows ManifestVersion 1.12.0).
- **Manifests:** multi-file (version, defaultLocale, installer) under `manifests/<l>/<Publisher>/<Package>/<version>/`.
- **Zip with a portable exe:** `InstallerType: zip`, `NestedInstallerType: portable`, `NestedInstallerFiles` (`RelativeFilePath`, `PortableCommandAlias`). `arm64` is supported.
- **Submission:** a PR to `microsoft/winget-pkgs`, one version per PR. **Komac needs a classic GitHub PAT**, which would be a long-lived credential.
- **Re-checked 2026-11-17 (WINGET-P5):**
  - **Version:** the newest stable winget-cli, v1.29.380 (2026-09-21, `000f6b55151cb0f1afd2933bb54c62a4724b9ca8`), embeds 1.28.0 as its latest schema. 1.31.0 is only master's in-development `latest/`, and v1.30.140-preview carries 1.30.0.
  - The community repository recommends 1.28.0 (PR template, 2026-11-04) and still accepts 1.12.0.
  - **Tools:** wingetcreate v1.28.1.0 is the first to emit 1.28.0 and was under 7 days old; the eligible v1.12.13.0 predates the change. Komac v2.17.2 supports 1.28.0.
  - **Submission:** Komac and wingetcreate both need a classic `public_repo` PAT (wingetcreate also offers an interactive OAuth login). Fine-grained PATs and fork-only GitHub Apps fail, and there is no OIDC path (winget-pkgs#413785, open).
  - **Validation** is now a 10-step GitHub App (`doc/Validation.md`) that replaced Azure DevOps. The Learn page is still stale.

### SCOOP
- **Version:** Scoop v0.6.0 (2026-09-30). The wiki is unversioned.
- **Source:** https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests, …/App-Manifest-Autoupdate, …/Buckets, …/Criteria-for-including-apps-in-the-main-bucket. Checked 2026-11-16. Status: Verified.
- **Manifest fields:** `version`, `description`, `homepage`, `license` (SPDX); `architecture.{64bit,arm64}.{url,hash,bin}`; `checkver: "github"`; `autoupdate` with `hash.url = "$url.sha256"`.
- **Personal bucket:** made from BucketTemplate, whose Excavator autoupdates every 4 hours. Users run `scoop bucket add`.
- **Main bucket:** needs ≥500 stars and ≥150 forks.
- **Re-checked 2026-11-17 (BREW-SCOOP-P5):** Scoop v0.6.0 is commit `e6aa3b366bdee8ed138c1e0f7b85192ebdd35d0f`, and master equals the tag. The wiki pages were last edited between 2022-07-05 and 2024-08-29 and lag the v0.6.0 source.
  - **Manifest fields:** the draft-07 `schema.json` requires only `version`, `homepage` and `license`. The wiki lists `description` too, but calls it "not technically required".
  - `architecture` keys are `32bit`, `64bit` and `arm64`, each with `url`, `hash`, `bin`, `extract_dir` and more. For `checkver: "github"`, `homepage` must be the GitHub repository URL unless `checkver.github` is set.
  - **autoupdate:** `hash.url = "$url.sha256"` still works. The extract mode accepts a bare hash, `hash  filename` lines or an aggregate list matched on `$basename`. v0.6.0 also uses the GitHub asset digest when `hash.url` is omitted, and otherwise downloads the file and hashes it.
  - **Personal bucket:** Excavator is `ScoopInstaller/GithubActions@5bceeda181721d9d10a18fdcc3ddca6f33467ae6` (v3.0.0), on cron `20 */4 * * *` with `contents: write`. It pushes straight to the default branch with no PR, and installs Scoop at run time from an unpinned script. Grimoire doesn't use it (ADR-0009).

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
- **Re-checked 2026-11-17 (INSTALLERS-P5):** v0.34.0 is still the latest: a lightweight tag on `294c1b25d372d86e74cafead8469b9f36d02da29`. The release is immutable and ships `sha256.sum` and per-archive `.sha256` files.
  - **Maintenance:** 108 commits since 2026-08-16: 62 from Dependabot, 29 from the maintainer, 17 from 9 other accounts. The last commit on main was on 2026-11-17 (a Dependabot bump), and the last maintainer-authored one on 2026-11-15.
  - **Installers:** the config reference lists `shell`, `powershell`, `npm`, `homebrew` and `msi`; the source also supports `pkg`.
  - **Status:** SHA-pinned workflows are possible: `[dist.github-action-commits]` (since 0.29.0) replaces each action's tag with a commit SHA. The default refs are tags. `github-attestations` is documented as experimental.
  - **The installer spike failed.** `installer.ps1` checks no checksum, and `installer.sh` checks one fail-open, and only for archives dist built. Grimoire writes its own installers (ADR-0010).

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
- **Re-checked 2026-11-17 (RUST-REGISTRY-P5):** v0.4.3 and action v0.5.148 are still the latest. Since Phase 5, Grimoire uses release-plz only for `release-pr` (ADR-0009).
  - **Tags:** the default is `{{ package }}-v{{ version }}` only when more than one package is release-enabled, otherwise `v{{ version }}`. `git_tag_name` is a Tera template whose only variables are `package` and `version`.
  - **The action** installs cargo-binstall with `curl | tar` and no check, today from `releases/latest`, then release-plz from a release asset with no checksum, signature or attestation. Its `version` input defaults to 0.4.3, although its docs say `latest`.
  - **Trusted publishing:** used only when there is no token, outside a dry-run, with `GITHUB_ACTIONS` set. If getting a token fails, it continues without one, and no setting turns it off. One token covers every crate and is revoked only if all of them succeed. An empty `CARGO_REGISTRY_TOKEN` counts as a token.
  - **Dry-run and recovery:** `--dry-run` still needs a git token and network access to GitHub and crates.io. A version already on crates.io is skipped without creating its missing tag, and an existing tag skips the package before any registry check.
  - **`GITHUB_TOKEN`:** stale for PRs. Since 2026-05-22, PRs that `GITHUB_TOKEN` creates or updates get `pull_request` runs that wait for approval (ACTIONS-P5). Tags and pushes still trigger nothing.
- **Re-checked 2026-11-17 (VERSIONING-P7), version bumps:** in 0.x a `feat` commit bumps the patch, not the minor, and only a breaking commit (`!` or a `BREAKING CHANGE` footer) bumps 0.y. Details are in VERSIONING-P7.
- **Observed 2026-11-17 and 2026-11-18 (Phase 7; read-only `gh api` requests), the first runs on GitHub:** the Release PR workflow (`release-plz.yml`) ran once, as run 44198210657 on `b5bb0e6`, and failed in both attempts (jobs 134445148588, 2026-11-17T23:18Z, and 134592389007, 2026-11-18T05:08Z) at the step "Open or update the release PR".
  - Each attempt created a `release-plz-<timestamp>` branch at `b5bb0e6`, then failed to create its commit there through GraphQL: "Repository rule violations found", "Changes must be made through a pull request" and "Waiting for Code Scanning results. Code Scanning may not be configured for the target branch." The `BestBranch` ruleset covers every branch (COMMUNITY-P7), so its pull-request rule and its code-scanning rule both refused the commit. Lifting the pull-request rule alone wouldn't be enough.
  - The two branches remain, `release-plz-2026-11-17T23-18-49Z` and `release-plz-2026-11-18T05-09-04Z`, both at `b5bb0e6`. The repository has no pull requests.
  - The run never reached the point of opening a PR. Whether "Allow GitHub Actions to create and approve pull requests" is on is still unknown: the Actions-permissions endpoint answered 403 to the token used. What the maintainer can change is in `release.md` §2 A.

### RUNNERS: GitHub-hosted runners
- **Source:** https://docs.github.com/en/actions/reference/runners/github-hosted-runners; https://github.com/actions/runner-images; changelog 2025-09-19 and 2026-02-26. Checked 2026-11-16. Status: Partial (no retirement date for `macos-26-intel`).
- **Cost:** standard runners are "free and unlimited on public repositories".
- **Linux arm64:** `ubuntu-24.04-arm`, `ubuntu-22.04-arm`, `ubuntu-26.04-arm`.
- **Windows arm64:** `windows-11-arm`.
- **macOS Intel:** `macos-15-intel`, `macos-26-intel`. x86_64 macOS support is to end after the macOS 15 image retires in "Fall 2027", but `macos-26-intel` was added in 2026.
- **x64 defaults:** `ubuntu-latest` moves to 26.04 in November 2026. `windows-2025`/`windows-2022`.
- **Re-checked 2026-11-17 (ACTIONS-P5):**
  - `windows-2025` and `windows-latest` now map to Windows Server 2025 with Visual Studio 2026 (`windows-2025-vs2026`), and `windows-11-arm` to Windows 11 Arm64 with VS 2026. Arm64 images are now maintained in `actions/runner-images`; `actions/partner-runner-images` is archived.
  - `ubuntu-latest`'s move to 26.04 is a rollout that ends on 2026-11-19. On 2026-11-17 the README still mapped it to 24.04, so explicit labels are still required.
  - `macos-15-intel`'s latest image is from 2026-08-24 (Rust 1.98.0, Homebrew 6.0.18). `macos-26-intel` is a standard public label, and the "Fall 2027" end of Intel macOS is stale; no current end date was found.
  - From 2026-11-30, the Windows 11 Arm64 workspace moves to `D:`, and `D:` becomes ReFS on the x64 and Arm64 Windows images.
  - `winget` isn't on `windows-11-arm`; on `windows-2025` it is undocumented.
- **Re-checked 2026-11-17 (BUILD-P6, RUN-P6), Docker on the Linux runners:**
  - The 20261116 images, `ubuntu-24.04` 20261116.376.1 and `ubuntu-24.04-arm` 20261116.188.1, ship Docker client and server 28.0.4, Buildx 0.39.0, Compose 2.38.2, Podman 4.9.3, Buildah 1.33.7 and Skopeo 1.13.3. They were published as pre-releases at 22:21Z on 2026-11-17, which starts their deployment; deployment usually takes 2 to 3 days, and until then a job may still get the 20261111 images, which have Buildx 0.38.2 (the runner-images README; the `main` READMEs still showed 20261111).
  - Docker 28.0.4 embeds BuildKit v0.20.2. Its image store is the classic one: the containerd store is the default only from Engine 29.0, on fresh installs, and nothing found enables it (`docker info` wasn't run on a runner). So the runner's own `docker` driver can't produce attestations or multi-platform OCI output.
  - The buildx plugin is installed at `latest` into `/usr/libexec/docker/cli-plugins`, with no checksum check.

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
- **Re-checked 2026-11-17 (ACTIONS-P5):**
  - **OIDC:** confirmed with `GET /repos/jabbott-iii/Grimoire/actions/oidc/customization/sub`: `use_immutable_subject` is true, and the prefix is `repo:jabbott-iii@84321953/Grimoire@1460666977`. An environment job's `sub` should be `<prefix>:environment:release`, but the docs have no literal example of that form.
  - **`pull_request_target` and checkout v7:** `actions/checkout` v7.0.0 blocks fork-PR checkout under both `pull_request_target` and `workflow_run`. The default `pull_request_target` block is enforced from 2026-12-02 for repositories still on the default policy.
  - **Lockfiles:** conflict. zizmor 1.32.0's docs still call them public preview and its own support experimental, while GitHub's docs carry no preview label. Keep GA.
  - **Environments:** a run that references a missing environment creates it with no protection rules. "Prevent self-review" stops the person who started a run from approving it.
  - **`GITHUB_TOKEN`:** PRs it creates or updates get approval-required runs (since 2026-05-22). `workflow_dispatch` checks don't satisfy required status checks.

### ATTEST: artifact attestations and immutable releases
- **Re-checked 2026-11-17 (SIGN-P6):** from Phase 6 every attestation is made in the reusable workflow `sign.yml`. In a called workflow, `actions/attest` needs `id-token: write` and `attestations: write` (and `packages: write` for an image) granted by the calling job too; the provenance's `runDetails.builder.id` names `sign.yml`, while `externalParameters.workflow.path` stays the caller, `release.yml`. Verify with `--signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main` plus `--repo` and `--source-ref`.
- **Source:** https://github.com/actions/attest; https://cli.github.com/manual/gh_attestation_verify; changelog 2025-10-28 (immutable releases GA). Checked 2026-11-16. Status: Verified.
- **`actions/attest`:** **v4.2.2**, commit `1e69f48acb82d1966a394da916b4c1698aa569d6`. It is canonical and covers provenance, SBOM and custom modes.
  - `attest-build-provenance` v4 is "simply a wrapper". `attest-sbom` is "being deprecated".
  - Permissions: `id-token: write`, `attestations: write`, `artifact-metadata: write`.
  - Available in public repos on all plans, via public-good Sigstore.
- **Verify:** `gh attestation verify <file|oci://…> --repo OWNER/REPO [--signer-workflow …]`.
- **Immutable releases:** lock the tag and assets, the tag name can never be reused, and a release attestation is generated automatically. Flow: draft, attach, publish.
- **Re-checked 2026-11-17 (SUPPLY-CHAIN-P5):**
  - **Permissions:** `id-token: write` and `attestations: write`, plus `contents: read` to check out. `artifact-metadata: write` is used only for storage records, which need `push-to-registry` and an organization-owned repository, so it isn't needed here.
  - **Dry-runs and fork PRs:** there is no dry-run input, and the action fails at once without `id-token`, so it can't run on fork or Dependabot PRs. Public repositories always log to public-good Rekor.
  - **Verify:** use gh 2.102.0 or later (GHSA-wjmr-j3rp-mh2g, GHSA-4mq3-hpgx-9cx8); the runners ship 2.103.3. `--repo` is case-insensitive, while `--signer-workflow` and `--cert-identity` are case-sensitive. Fully offline verification needs `--bundle` plus `--custom-trusted-root`.
  - **Immutable releases:** the release attestation is signed by GitHub's Sigstore (SAN `https://dotcom.releases.github.com`, predicate `https://in-toto.io/attestation/release/v0.2`). Verify it with `gh release verify` and `gh release verify-asset` (gh 2.81.0 or later). REST: `GET`, `PUT` and `DELETE /repos/{owner}/{repo}/immutable-releases`. The title and notes stay editable. `GITHUB_TOKEN` can't create or edit a release whose target changes workflow files.
- **Re-checked 2026-11-17 (REGISTRY-P6), for images:**
  - With `push-to-registry: true` the job also needs **`packages: write`**.
  - The action reads registry credentials only from `$HOME/.docker/config.json`; `DOCKER_CONFIG` and credential helpers are ignored. It takes exactly one subject (`subject-name` and a SHA-256 `subject-digest`), lowercases the name and strips a tag.
  - With `create-storage-record` at its default, it calls `GET /repos/{owner}/{repo}` and creates no record for a user-owned repository, so Grimoire sets it to `false`.
  - On GHCR the attestation is listed in the `sha256-<subject hex>` fallback tag, next to the cosign signature.

### COSIGN
- **Re-checked 2026-11-17 (SIGN-P6):** from Phase 6 the release's signatures come from the reusable workflow `sign.yml`, so the SAN is `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`. Any repository can call a public reusable workflow and get that same SAN, so verifiers also pass `--certificate-github-workflow-repository jabbott-iii/Grimoire --certificate-github-workflow-ref refs/heads/main` (deprecated OIDs 1.5 and 1.6, still issued, compared exactly).
- **Version:** cosign **v3.1.4** (2026-10-20); 2.x is still maintained (v2.6.5). `sigstore/cosign-installer` **v4.1.3**, commit `756fe2d284c7292c4e9b62c8a28f1e5629c59532`, defaults to v3.1.4.
- **Source:** https://github.com/sigstore/cosign; https://docs.sigstore.dev/cosign/. Checked 2026-11-16. Status: Verified: the Fulcio SAN in SUPPLY-CHAIN-P5 (2026-11-17), and GHCR's lack of OCI 1.1 referrers support in REGISTRY-P6 (2026-11-17).
- **Bundles:** v3 defaults to protobuf bundles, and "`--bundle` has moved from optional to required". Image signatures are stored as OCI 1.1 referring artifacts.
- **Blob signing:** `cosign sign-blob f --bundle f.sigstore.json`. Verify with `cosign verify-blob f --bundle … --certificate-identity … --certificate-oidc-issuer https://token.actions.githubusercontent.com`.
- **Re-checked 2026-11-17 (SUPPLY-CHAIN-P5):**
  - **The Fulcio SAN is now verified:** `https://github.com/jabbott-iii/Grimoire/.github/workflows/<FILE>@<REF>`, built from `job_workflow_ref`. Case is preserved, and the immutable `sub` doesn't affect it; the `sub` appears only in OID 1.3.6.1.4.1.57264.1.24. GHCR's OCI 1.1 referrers support was left for Phase 6, and is settled below (REGISTRY-P6).
  - **Version floor:** keep v3.1.4, and require at least 3.1.3 (GHSA-fx35-mq7g-6g98, a verification bypass). `--use-signing-config` is deprecated in 3.1.4.
  - **The installer** checks a hard-coded SHA-256 of its bootstrap binary, and supports Windows X64 only. There is no windows-arm64 cosign binary.
- **Re-checked 2026-11-17 (REGISTRY-P6, BUILD-P6, BASE-IMAGE-P6):**
  - **GHCR has no referrers API.** It returns 404 `MANIFEST_UNKNOWN` for `/v2/<name>/referrers/<digest>` (probed in four repositories), so clients fall back to the referrers tag schema. cosign v3.1.4 pushes an image signature as an OCI 1.1 referrer manifest (artifactType `application/vnd.dev.sigstore.bundle.v0.3+json`), and on GHCR it is listed in the `sha256-<subject hex>` fallback tag. This settles the last UNVERIFIED item above.
  - **`cosign verify` falls back silently** to legacy `.sig` lookup when it finds no new-format bundle. Distroless's signatures on gcr.io take that path.
  - Without `--recursive`, `cosign sign <ref>@sha256:<index>` signs only the index.
  - **Checked locally (2026-11-17, cosign 3.1.4, crane v0.22.1):** an offline signature of a pushed index landed in its `sha256-<index hex>` fallback tag, on a local registry without a referrers API. Distroless's index and platform manifests verified with its keyless identity.

### SBOM
- **Source:** https://github.com/CycloneDX/cyclonedx-rust-cargo; https://github.com/rust-secure-code/cargo-auditable; https://docs.docker.com/build/metadata/attestations/. Checked 2026-11-16. Status: Verified.
- **cargo-cyclonedx 0.5.10:** supports spec 1.3–1.7, but the **default is 1.3**, so pass `--spec-version 1.6` explicitly. Output defaults to XML; use `-f json`.
- **cargo-auditable 0.7.7:** embeds the dependency tree in binaries. Grype ≥0.83, Trivy, syft and cargo-audit can read it.
- **syft:** v1.57.2.
- **Docker buildx:** `--sbom=true` (SPDX via BuildKit's Syft) and `--provenance=mode=max`. Max mode "exposes the values of build arguments".
- **Re-checked 2026-11-17 (SUPPLY-CHAIN-P5):**
  - **syft:** v1.57.2 was released on 2026-11-16, too young to pin. The newest release at least 7 days old is v1.57.1 (2026-11-09).
  - **cargo-auditable readers:** cargo audit 0.17.3+, trivy 0.31.0+, grype 0.83.0+, syft 1.15.0+, osv-scanner 2.0.1+ (container images), blint 2.1.3+, wasm-tools 1.227.0+, rust-audit-info and auditable2cdx.
  - **cargo-cyclonedx:** also, there is no `-p`/`--package`; it writes `<package>.cdx.json` next to every workspace member's `Cargo.toml`; and `SOURCE_DATE_EPOCH` drops `serialNumber`, which `actions/attest`'s `sbom-path` requires.
- **Re-checked 2026-11-17 (BUILD-P6), buildx attestations:**
  - BuildKit's SBOM generator, `docker/buildkit-syft-scanner` 1.12.1, bundles **syft v1.51.0**, not the standalone syft above, and writes **SPDX 2.3** JSON.
  - Without `generator=`, BuildKit pulls it by its **`stable-1` tag** on every build, so Grimoire pins it by digest.
  - BuildKit's provenance default is SLSA v1 (since v0.28.0), and attestations are OCI artifacts (since v0.32.0). The docker-container driver adds the GitHub Actions event payload to provenance, in min mode too, unless the builder is created with `provenance-add-gha=false`.

### BASE-IMAGE
- **Source:** https://github.com/GoogleContainerTools/distroless (README 2026-09-24); https://images.chainguard.dev/directory/image/static/overview; https://github.com/opencontainers/image-spec/blob/main/annotations.md (image-spec v1.1.1). Checked 2026-11-16. Status: Verified.
- **distroless:**
  - Now based on **Debian 13**, as `gcr.io/distroless/static-debian13` with tags `latest`, `nonroot`, `debug`, `debug-nonroot`. Debian 12 builds were removed on 2026-09-10.
  - "distroless images by default do not contain a shell". `NONROOT` is uid 65532.
  - Keyless-signed. Verify with `--certificate-oidc-issuer https://accounts.google.com --certificate-identity keyless@distroless.iam.gserviceaccount.com`.
  - Point-in-time index digest of `static-debian13:nonroot`: `sha256:aafe993665bf6200db38a02c11a9351e782a84b6a45e7f33b6695638c88e3931`. Re-resolve at implementation time.
- **Chainguard:** the free `cgr.dev/chainguard/static` has only the `latest`/`latest-dev` tags. Version tags are paid.
- **OCI annotations:** `org.opencontainers.image.{created,source,revision,version,licenses,title,description,url,documentation,vendor,base.digest,base.name}`.
- **Re-checked 2026-11-17 (BASE-IMAGE-P6, RUN-P6, SCAN-P6):**
  - **The pin changed.** The `nonroot` digest above re-resolved unchanged, but it was first uploaded 2026-11-12T21:16:35Z and fails the 7-day rule until 2026-11-19T21:16:35Z. Grimoire pins the previous index, `sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329`, uploaded 2026-10-31T22:37:00Z (linux/amd64 `sha256:336e53e9941facdb69b755e6fd2c63f5400abd3b1ba47bb17ebafc2b1547cebd`, linux/arm64/v8 `sha256:bf6233b64054d17b31c3b555b1bbcee58958896830a305ef4d51faa257990d87`).
  - The two indexes differ only in the mode of `/usr/lib/os-release`, to which `/etc/os-release` links: 0555 in the pin, 0644 in the newer one. Upload times are GCR's undocumented `timeUploadedMs`, cross-checked against Rekor.
  - **Chainguard (corrected):** the free `cgr.dev/chainguard/static` has the plain tags `latest`, `latest-glibc`, `latest-glibc-tzdata` and `latest-tzdata`, not `latest`/`latest-dev`.
  - **Signatures:** the verify command above is unchanged. Distroless signs each index and each platform manifest, in the legacy `.sig` format. Checked locally with cosign 3.1.4 for the pin and both its platform manifests.
  - **OCI annotations:** image-spec v1.1.1 is still the latest release. The keys also include `authors` and `ref.name`.

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
- **Re-checked 2026-11-17 (SCAN-P6):**
  - Grype v0.122.0 (2026-11-16) is too young to pin, so the pin is **v0.121.3** (2026-11-09).
  - `anchore/scan-action` v7.4.2 is unchanged and is **not used**. It runs a downloaded `install.sh` with no pinned hash and no signature check, and defaults to Grype v0.118.0, whose syft is below the fix for GHSA-mw2c-m758-9v5q. Grimoire installs the Grype tarball itself, checked against its SHA-256.

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
- **Re-checked 2026-11-17 (SECURITY-POLICY-P7):** private vulnerability reporting is now **enabled** on this repository: `gh api repos/jabbott-iii/Grimoire/private-vulnerability-reporting` returns `{"enabled":true}`. The docs page above moved to `…/configure-vulnerability-reporting/configure-for-a-repository`, and the old path redirects there. Repository owners, security managers and admins can change the setting, on public repositories only. Reporters start from https://github.com/jabbott-iii/Grimoire/security/advisories/new (an observed URL; the docs name only the "Report a vulnerability" button), and github.com now labels the tab "Security and quality". How reports are made and handled is in SECURITY-POLICY-P7.

### DEPENDABOT
- **Source:** https://docs.github.com/en/code-security/reference/supply-chain-security/dependabot-options-reference; the dependabot-core `cargo` fetcher. Checked 2026-11-16. Status: Verified.
- **Cargo:** supported. The fetcher resolves workspace members and `workspace.dependencies`.
- **github-actions:** updates the SHA *and* a same-line `# vX.Y.Z` comment. "Dependabot … will not create alerts for actions pinned to SHA values."
- **`cooldown`:** default 3 days. Per-semver-level days apply to Cargo, `rust-toolchain` and others, but not to GitHub Actions or Docker, which take `default-days` only. (This corrects the Phase 0 entry, which said Cargo only.)
- **`rust-toolchain` ecosystem:** Dependabot bumps `rust-toolchain.toml`, but it never changes `rust-version`.
- **Re-checked 2026-11-17 (BASE-IMAGE-P6; dependabot-core v0.405.0), the `docker` ecosystem:**
  - `directory: "/"` finds a root `Dockerfile`. The `FROM` line must be literal (`[registry/]image[:tag][@sha256:<digest>]`, no `${ARG}`). A digest pinned on a non-comparable tag such as `nonroot` gets digest-only PRs.
  - **Cooldown fails open outside Docker Hub.** Since dependabot-core `3bfea916` (2026-09-18), the only accepted publication date is Docker Hub's `tag_last_pushed`. For gcr.io the update is allowed at once, with a `cooldown_date_unavailable` warning. The check above missed this. So the maintainer checks a new digest's age by hand before merging, for example with `curl -s https://gcr.io/v2/distroless/static-debian13/tags/list | jq '.manifest["sha256:<digest>"].timeUploadedMs'` (an undocumented field; BASE-IMAGE-P6).
  - **Docker gets no security updates.** Images in workflow `container:`, `services:` or `uses: docker://` are never updated.
  - Distroless bump PRs carry no release notes: Dependabot reads them only from config `Labels`, and distroless sets none.

### LYCHEE
- **Version:** lychee v0.24.2; `lycheeverse/lychee-action` v2.9.0, commit `e7477775783ea5526144ba13e8db5eec57747ce8`.
- **Source:** https://github.com/lycheeverse/lychee-action. Checked 2026-11-16. Status: Verified.
- **Phase 7 (2026-11-17), local use:** the maintainer approved lychee v0.24.2 in the scratchpad. The `x86_64-unknown-linux-gnu` tarball there matches the SHA-256 that `ci.yml` pins (`LYCHEE_SHA256`, `1f4e0ef7…`; CI-PINS), and the binary reports `lychee 0.24.2`. It runs with the same flags as CI: `--offline --no-progress --include-fragments`.

---

## Rust toolchain and core crates

### RUST: toolchain
- **Current stable:** 1.100.0 (`0ef1bc386 2026-11-09`), channel manifest dated 2026-11-12. Source: https://static.rust-lang.org/dist/channel-rust-stable.toml, checked 2026-11-16.
- **Local machine:** the default `stable` toolchain resolves to 1.99.0, and 1.100.0 is installed alongside. CI must install toolchains explicitly and never rely on whatever happens to be installed.
- **Edition 2024:** current releases of `rmcp`, `toml`, `toml_edit`, `clap`, `keyring` and `maturin` themselves use edition 2024. That doesn't block Grimoire from using it, because editions interoperate. This was not checked against the release notes; it will be confirmed by building in Phase 1.
- **Re-checked 2026-11-17 (ACTIONS-P5):** 1.100.0 is still current. Tag `1.100.0` is commit `0ef1bc386059596dada8fb65b50425aab2ea637a`, released on 2026-11-12. That is under the 7-day rule used for every other pin; the newest toolchain at least 7 days old is 1.99.1 (2026-10-15). The 1.100.0 pin in `rust-toolchain.toml` was a Phase 1 choice, and Phase 5 doesn't change it (ADR-0009). Release jobs publish with it, never with the MSRV toolchain: Cargo 1.97 fixed a workspace-publish deadlock regression in 1.96.

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
- **Re-checked 2026-11-17 (RUST-REGISTRY-P5, ACTIONS-P5):** every pin above is still the newest release at least 7 days old, and the actionlint, gitleaks and lychee checksums still match their checksum files. Newer, but too young: install-action v2.90.4 (2026-11-14) and v2.90.5 (2026-11-15), cargo-deny 0.21.0 (2026-11-16) and cargo-shear 1.14.1 (2026-11-13). actionlint, zizmor and gh-actions-lock also publish SLSA provenance attestations, which `gh attestation verify` can check and which are stronger than an API digest. Phase 5's new pins are listed in each `*-P5` entry.
- **Phase 6 (2026-11-17):** the container image's pins are listed in BUILD-P6 (buildx, the BuildKit and SBOM-scanner images), REGISTRY-P6 (crane, cosign), SCAN-P6 (Grype) and BASE-IMAGE-P6 (the distroless base). Each was at least 7 days old when adopted. Dependabot updates the base image and the cosign-installer action, but not the `env:` pins (buildx, crane, Grype and the two BuildKit images), which are bumped by hand.

### ZIZMOR: workflow audit rules
- **Re-checked 2026-11-17 (SIGN-P6):** the `self-repository` audit (since v1.30.0, regular persona, Low) flags `uses: ./…` on reusable workflow calls too; Grimoire's two `sign.yml` calls carry `# zizmor: ignore[self-repository]`, because actionlint 1.7.12 rejects the `$/` form (issue #711). `template-injection` rates a `workflow_call` string input High in a run step, so `sign.yml` passes inputs through `env:`. `concurrency-limits` skips reusable-only workflows.
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
- **Re-checked 2026-11-17 (ACTIONS-P5):**
  - **`use-trusted-publishing`** flags `run:` lines with `cargo publish` or `npm`/`pnpm`/`bun publish` (including `npm stage publish`) without `--dry-run`, **only when the job lacks `id-token: write`**. An OIDC publish job isn't flagged. It doesn't detect release-plz or maturin.
  - **Exit codes:** zizmor exits 11 when the highest finding is Informational. zizmor-action's default SARIF mode (`advanced-security: true`) never uses exit codes of 11 or more, which is why Grimoire sets it to `false`.
  - **`cache-mode`:** GitHub added this workflow key (`read`, `write`, `write-only`, `none`), and zizmor 1.31.0 and later honour it, but actionlint 1.7.12 rejects it. Don't add it; keep having no cache-aware steps.
  - **Version comments:** only a *missing* version comment is pedantic; a mismatched one is flagged at the regular persona.
  - **`concurrency-limits`** (pedantic) wants `cancel-in-progress: true` or an expression. Never cancel a publish run.
  - zizmor-action v0.8.1 installs zizmor wheels pinned by hash in `support/locks/`.

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
  - Re-checked 2026-11-17 (RUN-P6, CLI 2.1.327): the `:-` default rule also applies to local- and user-scope entries in `~/.claude.json`, and stdio servers have received `CLAUDE_PROJECT_DIR` since 2.1.139. The stdio working directory is still UNVERIFIED.
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
  - **Re-checked 2026-11-17 (VERSIONING-P7):** cite semver.org rather than the tag. Tag `v2.0.0` says build metadata SHOULD be ignored for precedence, while semver.org and the repository's `master` say MUST. The live page is also CC BY 3.0. release-plz 0.4.3 doesn't use the `feat`-to-MINOR mapping in 0.x: there a `feat` bumps the patch.
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
  - Re-checked 2026-11-17 (RUN-P6, `rust-v0.169.1` source): without `cwd`, a local stdio server starts in the local environment's cwd, falling back to the session configuration's cwd. Servers are spawned in a new process group. The compare from `rust-v0.169.0` touches only model-provider, `core/src/client.rs`, model and TUI files, so the rest of this entry holds.
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
  - whether every frontend (TUI, `exec`, app server) refuses to start on an invalid entry, as the source suggests.
  - Settled since: the exact fallback directory for a stdio server, from source in RUN-P6 (above).

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

---

## Phase 5 additions (checked 2026-11-17)

These facts came from an eight-topic research workflow: crates.io and release-plz, npm, PyPI, Homebrew and Scoop, winget, the supply chain (attestations, signing, SBOMs, checksums), Actions runners and builds, and installers. An independent skeptic re-checked each topic against its primary sources: crates.io and release-plz 68 of 74 confirmed (5 corrected, 1 uncertain), npm 63 of 70 (3 corrected, 4 uncertain), PyPI 65 of 72 (6 corrected, 1 uncertain), Homebrew and Scoop 64 of 70 (4 corrected, 2 uncertain), winget 67 of 70 (1 corrected, 2 uncertain), supply chain 74 of 75 (1 uncertain), Actions and builds 63 of 72 (5 corrected, 4 uncertain), installers 60 of 64 (3 corrected, 1 uncertain). The skeptics also re-checked all 78 pins; 2 needed corrections, and neither is used.

Corrected facts appear below in corrected form, uncertain ones are marked UNVERIFIED, and facts the skeptics added are included where an implementer needs them. How the release tools behave comes from their source at the pinned tag and from their docs: no release tool was run. The only local commands were `cargo metadata`, `cargo tree` and `rustc --print deployment-target`. The raw results are not committed. Where an entry below differs from a 2026-11-16 entry, this one wins, and the older entry carries a "Re-checked 2026-11-17" note.

### RUST-REGISTRY-P5: release-plz, cargo publish, crates.io and cargo-binstall
Sources: https://github.com/release-plz/release-plz at `92e907a14715` (tag `release-plz-v0.4.3`: `crates/release_plz_core/src/command/release.rs`, `…/tera.rs`, `crates/release_plz/src/config.rs`, `website/docs/config.md`, `website/docs/github/token.md`); https://github.com/release-plz/action/blob/66b324fd6c72d05d2a08568492ecdd62bfb8a62b/action.yml; Cargo at `3acc79c0c841`, the 1.100.0 submodule (`doc/book/src/CHANGELOG.md`, `src/ops/cargo_package/mod.rs`, `tests/testsuite/publish.rs`); https://github.com/rust-lang/crates.io at `1aea7ef2ddea` (`src/controllers/trustpub/`, `crates/crates_io_trustpub/`, `src/controllers/krate/{publish,update}.rs`, `svelte/src/routes/docs/trusted-publishing/+page.svelte`); https://github.com/cargo-bins/cargo-binstall at `5885714d0a40` (`SUPPORT.md`, `SIGNING.md`, `README.md`). Status: Verified, except the gaps listed.

- **Pins:**
  - release-plz 0.4.3 (2026-11-02): `release-plz-x86_64-unknown-linux-musl.tar.gz`, SHA-256 `4a4cfa92ea2abaee02b40daa596a95ca2725816a6a32cb03a8f48ddead287681`. That is the GitHub asset digest: the release has no checksum file, no signing metadata and no attestations.
  - `rust-lang/crates-io-auth-action` v1.0.6 (2026-10-21), commit `3268eb413133e2960fa4ae0e963ca76380c54cd9`. It runs on node24 and revokes its token in a post step.
  - Not used: `release-plz/action` v0.5.148 (`66b324fd6c72`). Its composite steps install cargo-binstall with `curl | tar` and no check. Its empty `version` input means that today it installs `releases/latest` (v1.27.0, under 7 days old), then release-plz's `-gnu` asset, checked only by HTTPS.
- **`release-plz.toml`** (unknown fields are refused):
  - `git_tag_name` and `git_release_name` are Tera templates whose only variables are `package` and `version`. The default is `{{ package }}-v{{ version }}` when more than one package is release-enabled, otherwise `v{{ version }}`. Grimoire never lets release-plz tag, so the default doesn't matter.
  - With `version.workspace = true`, every inheriting member gets the new workspace version, and `version_group` is ignored. A `[workspace.dependencies]` entry with a `path` and a `version` gets the new version.
  - `git_release_enable` defaults to true and `git_release_draft` to false, so release-plz would publish the GitHub Release itself. That clashes with draft, upload, publish under immutable releases. Grimoire sets `git_release_enable = false`.
  - For a crate not yet on crates.io, `release-pr` keeps the current version and still writes the changelog.
  - The `release-pr` job needs `contents: write`, `pull-requests: write`, the repository setting that lets Actions create PRs, and a full-history checkout. It commits through the GraphQL API, so the commit shows as Verified.
- **`release-plz release`** (Grimoire doesn't use it; recorded because ADR-0003 relied on it):
  - Per package, in dependency order: skip it if the rendered tag exists locally, before any registry check; skip it if `cargo info name@version` finds the version, **without** creating the missing tag; otherwise `cargo publish`, then the tag and release.
  - With `release_always = false`, it returns before any `cargo publish --dry-run` unless HEAD belongs to a `release-plz-*` PR. `--dry-run` still needs a git token, and calls the GitHub and crates.io APIs.
  - Built-in trusted publishing runs only with no token, for crates.io, outside a dry-run, with `GITHUB_ACTIONS` set. One token covers every package, and it is revoked only if every package succeeds. If getting a token fails, release-plz warns and goes on without one.
  - An empty `CARGO_REGISTRY_TOKEN` is read as a token, so trusted publishing is skipped and `cargo publish` gets an empty token.
  - `publish_timeout` (default 30m) bounds only the published-check and the wait loop, not `cargo publish` itself. The docs say otherwise.
- **`cargo publish`:**
  - `cargo publish --workspace` (or repeated `-p`) has been stable since Cargo 1.90, and it isn't atomic.
  - It packages the selected crates into a local overlay registry, so `cargo publish --workspace --dry-run` checks interdependent crates that aren't on crates.io yet. `cargo publish -p b` on its own fails while its new dependency `a` isn't in the registry.
  - `--workspace` skips `publish = false` members. In a dry-run an existing version is only a warning; a real `--workspace` publish fails if any version already exists.
  - Cargo 1.97 fixed a 1.96 regression where workspace publish failed with a false deadlock, so never publish with the MSRV toolchain.
  - In Cargo 1.100, `cargo install` uses the packaged lockfile by default (`--locked` is a no-op), and `registry.global-min-publish-age` is stable.
- **crates.io trusted publishing:**
  - Owner and repository names are compared case-insensitively. `repository_owner_id` must equal the ID captured when the config was created; the repository ID isn't checked.
  - The workflow filename comes from `workflow_ref` (the caller's workflow) and must match exactly. The environment is compared case-insensitively, and a config without one accepts any environment.
  - `sub` isn't decoded, so GitHub's immutable `sub` format doesn't matter. `pull_request_target` and `workflow_run` are refused.
  - A token covers every matching crate, and a crate has at most 5 configs. Trusted-publishing tokens can't create a crate. Creating a config needs a verified email and a linked GitHub account.
  - `trustpub_only` is a per-crate boolean (default false), set with `PATCH /api/v1/crates/{name}` or on the crate's settings page; every owner gets an email.
  - A repository rename or transfer changes the `repository` claim and needs new configs.
  - These facts come from crates.io's `master` source on 2026-11-17. Whether that code is deployed wasn't checked.
- **crates.io metadata:**
  - `description` of at most 1000 characters, and `license` or `license-file`.
  - At most 5 keywords: each at most 20 characters, ASCII alphanumeric first, then alphanumerics, `_`, `-` or `+`.
  - At most 5 categories, each an exact slug; an unknown slug fails the publish. `command-line-utilities` and `development-tools` are among the 104.
  - Limits: 10 MB upload, 512 MB unpacked, 30,000 entries.
- **cargo-binstall:**
  - v1.27.0 (2026-11-11) is the latest. The newest release at least 7 days old is v1.26.2 (2026-10-31, `851112458b5f`).
  - `[package.metadata.binstall]` keys: `pkg-url`, `pkg-fmt`, `bin-dir`, `signing {algorithm, pubkey, file}`, `disabled-strategies`, and `overrides.<triple or cfg(...)>`, where an exact triple wins. Unknown keys are ignored, so typos go unnoticed.
  - `pkg-fmt` values: `tar`, `tbz2`, `tgz` (the default), `txz`, `tzstd`, `zip`, `bin`.
  - Template variables: `name`, `version`, `repo`, `bin`, `target`, `archive-suffix`, `archive-format`, `binary-ext`, `format`, `target-family`, `os-name`, `target-arch`, `target-libc`, `target-vendor`.
  - **Verification:** the crates.io crate checksum, plus minisign only when `signing` is declared. **No checksum of the prebuilt archive**, and no Sigstore or GitHub attestations.
  - Strategies: `crate-meta-data`, `quick-install`, `compile`. Telemetry goes only through `quick-install`, so `disabled-strategies = ["quick-install"]` turns it off for one crate.
- **UNVERIFIED:** what Cargo does with an empty token; that release-plz's per-crate dry-run fails for dependents (consistent with the source, not run).

### NPM-P5: staged and trusted publishing, platform packages and the launcher
Sources: https://github.com/npm/documentation (`content/packages-and-modules/securing-your-code/{trusted-publishers,staged-publishing,generating-provenance-statements}.mdx`, `…/contributing-packages-to-the-registry/{creating-and-publishing-scoped-public-packages,package-name-guidelines}.mdx`, `content/integrations/integrating-npm-with-external-services/about-access-tokens.mdx`); npm CLI at tag v12.2.2 (`docs/lib/content/commands/{npm-trust,npm-stage}.md`, `lib/commands/{publish,stage/publish,trust/github}.js`, `lib/utils/oidc.js`, `workspaces/libnpmpublish/lib/publish.js`, and the bundled pacote, bin-links, cmd-shim and npm-install-checks); https://github.com/npm/cli/releases/tag/v12.0.0; https://registry.npmjs.org (npm, `agent-grimoire`, the `@agent-grimoire` scope, esbuild and biome packages); https://nodejs.org/dist/index.json; https://github.blog/changelog/2025-12-09-npm-classic-tokens-revoked-session-based-auth-and-cli-token-management-now-available/, https://github.blog/changelog/2026-10-02-npm-staged-publishing-now-supports-creating-new-packages, https://github.blog/changelog/2026-10-02-unvalidated-npm-trusted-publishing-configurations-now-expire, https://github.blog/changelog/2026-07-28-npm-publish-time-malware-scanning-and-dual-use-metadata/; https://github.com/npm/cli/issues/9969. Status: Partial (gaps listed).

- **Versions and pins:**
  - npm 12.2.2 (`latest`) and 11.21.2 (the newest 11.x), both 2026-11-04; tag commits `60e11343e613` and `1c66798c9def`. npm 12 needs Node `^22.22.2 || ^24.15.0 || >=26.0.0`.
  - Minimums: trusted publishing needs npm 11.5.1 and Node 22.14.0; staged publishing and `npm trust` need npm 11.15.0 and Node 22.14.0; dist-tags over OIDC need npm 11.21.0 or 12.2.0 and an opt-in.
  - `actions/setup-node` v7.1.0 (2026-10-08), commit `949feb2413d6458794dcd2491c4babbbce0c15c1`, with `package-manager-cache: false`.
  - Node 24.22.3 (2026-11-10) bundles npm 11.19.1, enough for staged and trusted publishing. Node 22.23.4 bundles npm 10.9.9, which is too old. Node 24 has been Maintenance LTS since 2026-10-20, and Node 26 Active LTS since 2026-10-28.
- **Staged publishing:**
  - `npm stage publish` is `npm publish` with `stage=true`: the same flags, a POST to `/-/stage/package/<name>`, and no 2FA.
  - Approve with 2FA, with `npm stage approve <stage-id>` or on npmjs.com's Staged Packages tab. Rejecting also needs 2FA.
  - Batch approval takes up to 100 versions, one per package, with one 2FA challenge. Each action is independent and isn't rolled back.
  - OIDC can publish, stage and (when opted in) set dist-tags. It can't list, view, approve or reject.
  - **Staged publishing can create a new package**, with a local session or a granular token but not OIDC. npm then publishes a public placeholder version, `0.0.0-stage`.
  - Staged and published versions share one version index. A staged version's tag is immutable, and a prerelease needs `--tag`.
  - Since 2026-07-28, npm scans every new version before it can be installed. That typically takes about 5 minutes, sometimes 15 or more, and a version can be held for review.
- **Trusted publishing:**
  - GA since 2025-07-31. Providers: GitHub Actions on GitHub-hosted runners, GitLab.com shared runners, and CircleCI cloud. Self-hosted runners aren't supported.
  - The package must already exist (`npm trust` refuses otherwise), so trusted publishing can't create a package.
  - Configurations created after 2026-09-03 allow `npm stage publish` by default; `npm publish` and dist-tags are opt-in.
  - A new configuration must publish successfully within 48 hours, or it expires. Validation binds it to the repository's immutable identity.
  - Fields: organization or user, repository, workflow filename (`.yml` or `.yaml` in `.github/workflows/`), and an optional environment. **All are case-sensitive**, so type `Grimoire`. For `workflow_call` and `workflow_dispatch`, the calling workflow's filename is matched.
  - CLI: `npm trust github <pkg> --file <wf>.yml --repo jabbott-iii/Grimoire --env npm --allow-stage-publish --yes` (Grimoire's npm environment is `npm`, ADR-0009). It needs 2FA, refuses bypass-2FA tokens, and has no dist-tag flag. It sends `repository`, `workflow_ref.file` and `environment`, never `sub`.
  - npm refuses OIDC tokens from `pull_request_target` and `issue_comment`.
  - **npm/cli#9969**, opened 2026-09-14: "OIDC trusted publishing fails with 'package not found' for GitHub repos that use immutable OIDC subject claims". On 2026-11-17 it was open, labelled Priority 1, triaged on 2026-10-28, with no fix announced. One post-cutoff repository published through a GitHub trusted publisher on 2026-11-05 and 2026-11-16, so it can work.
  - npm recommends setting publishing access to "Require two-factor authentication and disallow tokens". Trusted publishers aren't affected.
- **Provenance:**
  - It is automatic only for trusted publishing from a public repository, of a public package. The CLI checks the token's `repository_visibility` and the package's visibility.
  - `repository.url` must match the GitHub repository exactly, case included: `git+https://github.com/jabbott-iii/Grimoire.git`.
  - libnpmpublish refuses provenance for a new or private package unless access is public, so pass `--access public`.
- **Dry-runs:**
  - `npm publish --dry-run` and `npm stage publish --dry-run` need no login; they only warn. But they fetch the packument, and **perform a real OIDC exchange when the job has `id-token: write`**. They run lifecycle scripts unless `--ignore-scripts` is passed.
  - `npm pack --dry-run` on a directory is local, but without `--ignore-scripts` it still runs `prepack`, `prepare` and `postpack`.
  - npm 12 refuses unknown flags.
- **Platform packages and the launcher:**
  - `os` and `cpu` match `process.platform` and `process.arch`. `libc` applies only on Linux, and a package that sets it is refused when the host's libc can't be detected. Static musl packages omit it, as esbuild's do.
  - Optional dependencies that fail the `os`, `cpu` or `libc` check are skipped without failing the install.
  - Only a global top-level package's `bin` reaches PATH, so the launcher must be a JS `bin` in `agent-grimoire`.
  - `npm pack` sets modes to `(mode | 0o600) & ~0o022`, keeping execute bits, and forces `+x` only on declared `bin` files. It sets mtime 1985-10-26 and never includes symlinks. So set the binary to 0755 before packing.
  - npm 12 blocks dependency lifecycle scripts by default, so no `postinstall`.
  - The pattern followed is biome's launcher: `require.resolve`, `spawnSync` with `shell: false` and `stdio: "inherit"`, and 128 plus the signal number when the child dies of a signal. Grimoire's launcher uses `spawn` instead, so that it can forward `SIGINT`, `SIGTERM` and `SIGHUP` to the child (ADR-0010).
- **Names (2026-11-17):** `agent-grimoire` and the `@agent-grimoire` scope are free; no user or organization owns the scope. Organization-scoped packages need an npm organization first.
- **Tokens:** classic tokens were revoked on 2025-12-09. `npm login` gives 2-hour sessions with 2FA enforced for publishing. Granular write tokens last at most 90 days. Direct publishing with granular tokens is targeted for removal in January 2027. Stage-only tokens exist, and get `E_STAGE_REQUIRED` from `npm publish`.
- **Conflicts:**
  - The `npm trust` CLI docs at v12.2.2 still say one configuration per package. The web docs say up to 10, and the changelog says several are GA. Grimoire needs one.
  - The config docs say a new package's access defaults to `public`; the scoped-package docs say scoped packages are private by default. Always pass `--access public`.
- **Checked in the Phase 5 review (2026-11-17):**
  - **Provenance in the pinned npm 11.19.1** (https://github.com/npm/cli/blob/v11.19.1/lib/utils/oidc.js, `lib/commands/publish.js`): `npm publish` and `npm stage publish` both run the OIDC exchange. After a successful exchange the CLI turns provenance on when the token's `repository_visibility` is `public` and the package is public. An error in the visibility lookup is logged only at verbose level, and the publish goes ahead without provenance. Grimoire passes no `--provenance`, so nothing fails when provenance is missing.
  - **Re-staging:** a staged version isn't public, so `npm view` doesn't see it. npm refuses to stage a version that is already staged ("Staged packages share the same semver version unique index as published packages", `npm-stage.md` at v12.2.2). To stage it again, reject the stage first.
  - **Dry-runs and published versions** (npm 11.19.1 `lib/commands/publish.js`, also run locally with npm 11.19.1 on Node 18.19.1): `--dry-run` still fetches the packument and refuses a version that is already published. Only `--force` skips that, and `--force` also skips the check that a prerelease needs `--tag`. With `--registry=http://127.0.0.1:9/ --fetch-retries=0` the lookup fails at once, npm sees no versions, and the other checks still run.
  - **Session tokens can stage** (`npm-stage.md` at v12.2.2, the Token Behavior table): a session token can run `npm stage publish`, staging never asks for 2FA, and a package's publishing-access setting limits only granular-token `npm publish`. So the manual fallback works under "Require two-factor authentication and disallow tokens". Not run against the registry.
  - **`npm audit signatures`** (https://github.com/npm/cli/blob/v12.2.2/lib/commands/audit.js; npm/documentation `generating-provenance-statements.mdx`): it checks the registry signatures and provenance attestations of a project's installed packages. It refuses global installs with `EAUDITGLOBAL`, so it can't check `npm i -g` or `npx`. Read from the v12.2.2 source; the refusal was reproduced with npm 11.20.0.
- **UNVERIFIED:**
  - which dist-tag the `0.0.0-stage` placeholder gets, and whether rejecting a stage keeps the package;
  - whether staged versions expire;
  - whether a stage, before approval, counts as the validating publish within 48 hours;
  - whether npm supports repositories with the immutable `sub` (npm/cli#9969);
  - any registry size limit (none documented; a biome package of 38 MB packed, 120 MB unpacked is accepted);
  - any npm policy specific to native binaries (none found);
  - whether a version staged over OIDC keeps its provenance after approval (the CLI turns it on, as above; the docs don't say whether the registry keeps it);
  - whether setup-node checks the checksum of the Node it downloads.

### PYPI-P5: hand-written wheels, PyPI's checks and the publish action
Sources: https://packaging.python.org/en/latest/specifications/binary-distribution-format/, …/recording-installed-packages/, …/core-metadata/; https://github.com/pypi/warehouse at `c2f79b4a6fb0` (`warehouse/oidc/`, `warehouse/forklift/legacy.py`, `warehouse/utils/wheel.py`, `docs/user/trusted-publishers/`); https://docs.pypi.org/trusted-publishers/; https://github.com/pypa/gh-action-pypi-publish at v1.14.3; https://ghcr.io/v2/pypa/gh-action-pypi-publish/manifests/7245906fb61fa33a13554f9d5fb856fe61758086; https://github.com/pypa/twine/blob/7.0.0/docs/changelog.rst; https://github.com/pypa/pip/blob/26.3/src/pip/_internal/operations/install/wheel.py; maturin v1.16.0 and maturin-action v1.51.0, for comparison; https://pypi.org/pypi/agent-grimoire/json. Status: Verified, except the gaps listed.

- **Pins:**
  - `pypa/gh-action-pypi-publish` v1.14.3 (2026-10-20), commit `7245906fb61fa33a13554f9d5fb856fe61758086`.
  - twine 7.0.0 (2026-07-27), installed from a hash-pinned requirements file. It fixed uploads of Metadata-Version 2.5 and no longer accepts 2.0.
  - The alternative publisher, `uv publish` (uv 0.13.7, 2026-11-03), isn't used.
- **Why Grimoire doesn't use maturin:** maturin 1.16.0 can't package a prebuilt binary. For `bindings = "bin"` it always compiles with `cargo rustc --bin`. uv, ruff and maturin itself build each target's wheel and archive from one compile. Grimoire writes the wheel in `cargo xtask` from the archived binary instead (ADR-0009). maturin-action also downloads maturin without a checksum check.
- **A wheel doesn't depend on its generator;** PyPI checks the format. A binary-only wheel needs:
  - `agent_grimoire-{V}.data/scripts/grimoire` (`grimoire.exe` on Windows), with zip mode 0755. pip 26.3 copies `.data/scripts` files unchanged, rewrites only a leading `#!python`, and sets `+x` only if the zip entry is executable.
  - `.dist-info/METADATA`: Metadata-Version 2.4 added `License-Expression` and `License-File`; core metadata is now at 2.6.
  - `.dist-info/WHEEL`: `Wheel-Version: 1.0`, `Generator`, `Root-Is-Purelib: false`, and one `Tag:` line per tag.
  - `.dist-info/licenses/…`, and `.dist-info/RECORD`: `path,sha256=<urlsafe base64 without padding>,size`, with `RECORD,,` for itself. `.dist-info` goes last.
  - A file name that starts with the normalized name, `agent_grimoire`. PyPI looks projects up by normalized name, and the pending publisher's project name must match the metadata `Name`.
- **Tags:**
  - Static musl: one wheel may carry `manylinux_2_17_<arch>` (with its `manylinux2014_<arch>` alias) and `musllinux_1_2_<arch>` together. PyPI hosts such wheels (maturin 1.16.0, uv 0.13.8).
  - macOS: `macosx_10_12_x86_64` and `macosx_11_0_arm64`, rustc 1.100.0's deployment targets (checked with `rustc --print deployment-target`). Windows: `win_amd64` and `win_arm64`.
  - PyPI's allow-list covers manylinux and musllinux on x86_64 and aarch64, and macOS and Windows on x86_64 and arm64.
  - **Compressed tag sets are sorted** (https://github.com/pypa/packaging.python.org/blob/main/source/specifications/platform-compatibility-tags.rst, "Compressed Tag Sets", fetched 2026-11-17 in the Phase 5 review): each tag group in a file name is a `.`-separated, sorted collection. ASCII order puts `manylinux2014_<arch>` before `manylinux_2_17_<arch>`. PyPI doesn't check the order, because it parses the name into a set, but `wheel tags` rewrites an unsorted name and maturin 1.16.0 sorts. Grimoire sorts.
- **What PyPI checks on upload:**
  - the file name and tags, against the allow-list;
  - zip framing, with STORED or DEFLATED members only, a 50× decompression-ratio limit and normalized member paths;
  - that `RECORD` exists (a mismatch only triggers an email), `METADATA`, `License-File` and entry points;
  - a YARA scan of `.py` files.
  - **It doesn't inspect binaries.**
- **Trusted publishing:**
  - A pending publisher needs owner, repository, workflow filename (ending in `.yml` or `.yaml`, with no `/`) and project name. The environment is optional.
  - PyPI stores the owner's canonical login and numeric ID. It stores the repository name as typed and matches it exactly, so register `Grimoire`. The environment is compared case-insensitively.
  - `sub` and `repository_id` aren't checked, so the immutable `sub` has no effect. `pull_request_target` and `issue_comment` are refused.
  - Tokens last 15 minutes. A pending publisher creates the project on first use, and doesn't reserve the name.
  - **Pending publishers expire.** Since warehouse #19996 (2026-05-04), a daily task deletes them 30 days after creation, with a reminder 5 days before (`warehouse/oidc/tasks.py`). The docs don't say so.
  - Reusable workflows: see the PYPI-TP note.
- **gh-action-pypi-publish:**
  - Linux only, and not in a job with `container:`.
  - It is a composite action that runs `docker://ghcr.io/pypa/gh-action-pypi-publish:<ref>`, so pinning the action by SHA pins its image by **tag**, not digest. On 2026-11-17 the SHA tag resolved to `sha256:411241df99067c98b6d08fd8e26d14e86740a22460599c2af571e6dc26e27ffc`, and `v1.14.3` to `sha256:a20ee5e9a490ada64fce0c9176418a4fd5afad3b80e70fb4745803c1309dcdfc`. Every build of that commit re-pushes the SHA tag (4 builds on 2026-10-20), and it can be overwritten.
  - The base image, `python:3.13-slim`, is unpinned. The Python dependencies (twine 7.0.0, sigstore 4.5.1, pypi-attestations 0.0.30, id 1.6.1) are hash-pinned in a constraints file, which pip enforces only from 26.1. The image upgrades to pip 26.2 using the base image's unpinned pip.
  - `attestations` (PEP 740) defaults to true and needs trusted publishing. `verify-metadata` runs `twine check` first. There is no dry-run input.
- **Offline checks for PRs:** `twine check --strict` renders the long description and validates metadata through `packaging`. Add tag and metadata assertions, and `pip install --no-index` into a venv followed by `grimoire --version`. check-wheel-contents 0.6.3 fails every binary-only wheel on W007 unless given `--ignore W007`.
- **Name:** `agent-grimoire` was free on 2026-11-17: 404 for every spelling, including the ultranormalized `agentgr1m01re`.
- **UNVERIFIED:** whether PyPI's similar-name or prohibited-name checks would refuse `agent-grimoire` (visible only when logged in); whether `uv publish --dry-run` mints an OIDC token; the action's `skip-existing` input, which Grimoire uses but the research didn't re-check.

### BREW-SCOOP-P5: the prebuilt formula, local-tap audits and the Scoop manifest
Sources: https://github.com/Homebrew/brew at tag 7.0.14 (`docs/{Taps,How-to-Create-and-Maintain-a-Tap,Formula-Cookbook,Tap-Trust,Acceptable-Formulae,Package-Acceptance-Policy,Adding-Software-to-Homebrew}.md`, `Library/Homebrew/{rubocops,dev-cmd,livecheck}/`, `formula.rb`, `trust.rb`, `utils/ast.rb`, `.github/workflows/tap-new-*.yml`); https://github.com/Homebrew/actions/blob/2026.11.09.1/README.md; https://github.com/ScoopInstaller/Scoop at v0.6.0 (`schema.json`, `lib/{autoupdate,manifest}.ps1`, `bin/checkver.ps1`, `test/`); https://github.com/ScoopInstaller/BucketTemplate at `dd0ce8ae6a9e`; https://github.com/ScoopInstaller/GithubActions/blob/v3.0.0/src/Action/Scheduled.psm1; https://github.com/PowerShell/PowerShell/blob/v7.6.9/src/Microsoft.PowerShell.Commands.Utility/commands/utility/TestJsonCommand.cs; runner-images readmes; for the licence syntax, https://github.com/ScoopInstaller/Scoop/wiki/App-Manifests and `libexec/scoop-info.ps1` at v0.6.0 (checked in the Phase 5 review). Status: Verified from source and docs, except the gaps listed. No `brew`, `scoop`, `pwsh` or schema validator was run.

- **Pins:**
  - Homebrew 7.0.14 is current (2026-11-16).
  - `Homebrew/actions` 2026.11.09.1, commit `63a2d4d8d4193eab3e62b1b782ff3ab8408f5b29`. All its actions share one CalVer tag, so `setup-homebrew` has no tag of its own.
  - Scoop v0.6.0 (2026-09-30), commit `e6aa3b366bdee8ed138c1e0f7b85192ebdd35d0f`. Its `schema.json` has SHA-256 `64ec829c70758d3fba2c8c168a5901a64408a6493efda25f172b0fe4505caac0`.
- **Formula, not cask:**
  - Homebrew's guidance: a formula for open-source command-line software it can build from source, a cask for binary-only software.
  - homebrew/core is source-only, but its binary-URL cop and source-only audits run only for homebrew/core. Software outside core's criteria can live in a third-party tap.
  - Casks quarantine downloads on macOS, and `brew audit --cask --new` runs a notarization check. Grimoire uses a prebuilt-binary formula in its own tap.
- **Per-platform URLs:**
  - `url`, `sha256` and `version` work inside `on_*` blocks at run time.
  - `brew style`'s ComponentsOrder cop refuses `url` directly inside a top-level `on_*` block. It checks only that block's direct children, so `on_macos do on_arm do url … end end` isn't checked (inferred from source, not run).
  - The OnSystemConditionals cop skips its OS and arch audits outside homebrew/core, so top-level `if OS.mac?` / `Hardware::CPU.arm?` is also allowed in a tap.
- **Bumping:** `bump-formula-pr`, which the generated autobump uses, rewrites `url` and `sha256` only when they are direct children of the formula body. It can't update per-platform pairs, so Grimoire's tap uses its own sync workflow (ADR-0009).
- **Other formula facts:** the version is detected from `…/releases/download/vX.Y.Z/…` URLs. A choice of licences is `license any_of: ["MIT", "Apache-2.0"]`. livecheck's Git strategy reads the repository's tags.
- **Audits in CI:**
  - `brew audit <path>` is disabled. Instead: `brew tap-new --no-git local/<tap>`, copy the formula into its `Formula/`, run `brew trust` on the tap, `brew style`, then `brew audit --strict --formula --os=all --arch=all local/<tap>/agent-grimoire`.
  - The tap needs `brew trust` because the fully-qualified-name exemption covers only install, reinstall and upgrade.
  - `--new` implies `--online`, which fetches the release URLs, so neither runs on PRs. For a tap, `--online`'s licence, archived and notability checks never run anyway.
- **Runners:** ubuntu-24.04 has Homebrew 7.0.13, not on PATH: `eval "$(/home/linuxbrew/.linuxbrew/bin/brew shellenv)"`. Outside a tap repository, `setup-homebrew` moves brew to its newest tag.
- **Tap trust** (since Homebrew 6.0.0, non-official taps need explicit trust):
  - `brew install jabbott-iii/tap/agent-grimoire` trusts only that formula.
  - `brew trust --formula jabbott-iii/tap/agent-grimoire`, or `brew trust jabbott-iii/tap` for the whole tap, also work.
  - The tap repository must be `github.com/jabbott-iii/homebrew-tap`.
- **Tap repositories and `GITHUB_TOKEN`:** a personal-account repository's workflows can't create or approve PRs until "Allow GitHub Actions to create and approve pull requests" is on, and the PRs they open start with runs that need approval. `tap-new`'s `tests.yml` runs `brew test-bot`, which builds bottles; what it does with a prebuilt formula wasn't checked.
- **The Scoop manifest:**
  - `schema.json` is draft-07 and requires only `version`, `homepage` and `license`. The wiki lists `description` as required, but says it is "not technically required"; include it anyway.
  - `architecture.{32bit,64bit,arm64}` take `url`, `hash`, `bin`, `extract_dir` and more. A top-level `bin` applies to every architecture.
  - `checkver: "github"` reads `homepage`, which must then be the GitHub repository URL, or `checkver.github`. It queries `/releases/latest`.
  - `autoupdate`'s `hash.url` accepts a bare-hash file, `hash  filename` lines, or an aggregate list matched on `$basename`. With no `hash.url` and a GitHub release URL, v0.6.0 uses the asset digest; failing that, it downloads the file and hashes it. The wiki lags v0.6.0.
  - Bucket CI checks each file for no BOM, a final newline, CRLF line endings (the template's `.gitattributes` stores LF and checks out CRLF), no trailing whitespace and 4-space JSON.
- **Excavator (not used):** BucketTemplate's Excavator (`ScoopInstaller/GithubActions` v3.0.0, every 4 hours, `contents: write`) pushes updated manifests straight to the branch, with no PR. At run time it installs Scoop from the unpinned `ScoopInstaller/Install` master `install.ps1`, through `irm | iex`. Pushes made with `GITHUB_TOKEN` trigger no workflow, so the bucket's own CI never tests them.
- **Offline validation:** PowerShell 7.4 and later's `Test-Json -SchemaFile` uses JsonSchema.Net (7.4.0 in PowerShell 7.6.9). It supports drafts 6, 7, 2019-09 and 2020-12, not draft 4, and doesn't assert `format`. pwsh is 7.6.10 on ubuntu-24.04 and 7.6.9 on windows-2025.
- **User commands:** `scoop bucket add <name> https://github.com/jabbott-iii/scoop-bucket` (needs git), then `scoop install <name>/agent-grimoire`.
- **Scoop licence syntax** (checked 2026-11-17 in the Phase 5 review):
  - The wiki's `license` entry: separate licences with `|` when the whole application is dual licensed, and with `,` when different files have different licences.
  - `scoop info --verbose` (v0.6.0 `libexec/scoop-info.ps1`, lines 103-117) splits a licence string on `[|,]` and links one spdx.org page per part. Any other string becomes one link, so `MIT OR Apache-2.0` gives a dead link.
  - The schema's `licenseIdentifiers` is a free string, so `Test-Json` doesn't check the syntax.
  - ScoopInstaller/Main follows the pipe form (rustup: `MIT|Apache-2.0`). Grimoire writes `MIT|Apache-2.0`.
- **UNVERIFIED:** whether `brew style` passes a formula with `url` nested two `on_*` levels deep, which is the shape of Grimoire's generated formula (the PR dry-run runs `brew style` on it, but no run has been recorded yet; if it fails, a top-level `if OS.mac? && Hardware::CPU.arm?` is the fallback the OnSystemConditionals cop allows in a tap); whether `Test-Json` loads Scoop's schema (its `$id` ends in `#`) and validates a real manifest cleanly; whether the preinstalled brew auto-updates itself during `tap-new`, `trust` or `audit`.

### WINGET-P5: manifests, validation and submission
Sources: https://github.com/microsoft/winget-pkgs at `b192fe0471e6` (`doc/{Authoring,Validation,ValidationFailureGuide,Policies,FAQ,FirstContribution}.md`, `doc/manifest/schema/1.28.0/`, `.github/PULL_REQUEST_TEMPLATE.md`, `manifests/`); https://github.com/microsoft/winget-cli at tag v1.29.380, commit `000f6b55151cb0f1afd2933bb54c62a4724b9ca8` (`schemas/JSON/manifests/latest/`, `src/AppInstallerCLICore/Commands/ValidateCommand.cpp`, `src/AppInstallerCommonCore/Manifest/`, `src/AppInstallerCLICore/PortableInstaller.cpp`), and master; https://learn.microsoft.com/en-us/windows/package-manager/winget/validate; https://github.com/microsoft/winget-create at v1.28.1.0; https://github.com/russellbanks/Komac at v2.17.2; https://github.com/actions/runner-images/issues/14083, …/13688; https://github.com/microsoft/winget-pkgs/issues/413785. Status: Verified, except the gaps listed. Nothing was run.

- **Schema:**
  - winget-pkgs recommends 1.28.0 (the PR template and `Authoring.md`, 2026-11-04), and still accepts 1.12.0.
  - The newest stable winget-cli, v1.29.380 (2026-09-21), embeds 1.28.0 as its latest schema, byte-identical to master's `v1.28.0/`. That tag has no `v1.28.0/` directory, so the files to vendor are `latest/manifest.*.latest.json` at the tag.
  - 1.31.0 exists only as master's in-development `latest/`. The `aka.ms` schema URLs redirect to the mutable master branch. The Learn manifest page is stale (1.12.0).
- **Pins (`release.yml`, `verify-linux`):** winget-cli tag v1.29.380, commit `000f6b55151cb0f1afd2933bb54c62a4724b9ca8`, directory `schemas/JSON/manifests/latest/`:
  - `manifest.version.latest.json`: SHA-256 `5c9fa37ebe70dd410114157d31e4d9e84451b2bb8890e1c4345b4c690406ee1a`;
  - `manifest.installer.latest.json`: SHA-256 `2caa4cd3a630c284b5ef750522e2e5861f751f376180b89cd29873eb30504ef4`;
  - `manifest.defaultLocale.latest.json`: SHA-256 `734c069c82470e3436d274fb381bfbe03e22f8fbfae2d2dd50df7dc055a3e061`.
  - winget-cli publishes no checksum file for them, so the research computed the hashes locally on 2026-11-17. The review re-hashed local copies and they matched; they weren't downloaded again from the pinned commit for this entry.
  - Dependabot can't see these `env:` pins. Bump them by hand, together with `WINGET_CLI_COMMIT`.
- **Manifests:** community submissions are multi-file (version, defaultLocale, installer), each with the `yaml-language-server` `$schema` header.
  - **Required fields.** Version: `PackageIdentifier`, `PackageVersion`, `DefaultLocale`, `ManifestType`, `ManifestVersion`. DefaultLocale adds `PackageLocale`, `Publisher`, `PackageName`, `License` (3–512 characters) and `ShortDescription` (3–256). Installer adds `Installers[]`, each with `Architecture`, `InstallerUrl` and `InstallerSha256`.
  - `InstallerType` is required by winget and its docs, but not by the JSON Schema. The schema also allows unknown keys and doesn't pin `ManifestVersion` to 1.28.0, so xtask must check those itself.
  - `PackageIdentifier`: 2 to 8 dot-separated segments of 1–32 characters, at most 128 in all, case-sensitive, and equal to the path `manifests/<lowercase first letter>/<segments>/<version>/`.
  - **Zip with a portable exe:** `InstallerType: zip`, `NestedInstallerType: portable`, `NestedInstallerFiles: [{RelativeFilePath, PortableCommandAlias}]`. The alias is valid only with `portable`. `Architecture` includes `x64` and `arm64`.
  - `InstallerSha256` is 64 hex characters, the hash of the zip, not the exe. `ReleaseDate` is optional (`YYYY-MM-DD`). `Moniker` is 1–40 characters.
  - Rust MSVC links the CRT dynamically by default. Build with `+crt-static`, or declare `Microsoft.VCRedist.2015+.x64` / `.arm64`; otherwise validation can fail with `Validation-VCRuntime-Dependency`.
  - Converting the YAML to JSON for `Test-Json` can change types: an unquoted `ReleaseDate` becomes a timestamp, and a version such as `1.10` a float.
- **Names:**
  - `CatfinityStudios.Grimoire` has been in winget-pkgs since 2026-11-04, as zip+portable with `PortableCommandAlias: grimoire` and `Moniker: grimoire`, and it ships often. GitHub code search returns 0 hits for it, so code search isn't a reliable name check.
  - No `jabbott-iii` publisher folder exists. `jabbott-iii.agent-grimoire` and `JAbbottIII.AgentGrimoire` are both schema-valid, and lowercase, hyphenated publisher segments based on GitHub handles exist (`1broseidon.ketch`, `abemedia.stagelint`, `jdx.mise`).
- **Portable installs:**
  - winget extracts the zip under `%LOCALAPPDATA%\Microsoft\WinGet\Packages`, links the alias in `…\WinGet\Links`, and adds that folder to PATH. Without Developer Mode or admin rights it adds the package folder to PATH instead. Either way a new shell is needed.
  - v1.29.380 deletes any existing file at the link path, so two packages with the same alias overwrite each other. Nothing checks for that across packages.
- **`winget validate --manifest <dir>`:** runs full validation, with the schemas compiled into the client. Warnings, unknown fields included, fail unless `--ignore-warnings` is passed, so CI must not pass it. winget's validator also checks what JSON Schema can't: unknown fields, PascalCase and paths.
- **Testing a local manifest** (checked 2026-11-17 in the Phase 5 review: winget-pkgs `README.md`, `doc/FirstContribution.md`, `doc/Validation.md` and `doc/Authoring.md`; winget-cli v1.29.380 `InstallCommand.cpp`, `Argument.cpp`, `Command.cpp` and `AdminSettings.cpp`):
  - `winget install --manifest <dir>` needs the admin setting `LocalManifestFiles`, which is off by default. Turn it on with `winget settings --enable LocalManifestFiles` in an elevated shell. Group Policy can block it.
  - winget-pkgs' `Tools\SandboxTest.ps1` tests a manifest in Windows Sandbox instead.
  - `winget validate --manifest` uses a different argument and doesn't need the setting.
  - That the setting needs elevation comes from the winget-pkgs docs, not from code.
- **Runners:**
  - `winget` isn't on `windows-11-arm`, and runner-images closed the request as not planned (2026-08-17).
  - On `windows-2025` the sources conflict. No runner-images readme mentions winget or App Installer. One issue shows it working on an image from 2026-02-02, before the label moved to the VS2026 image. Learn says Server 2025 gets App Installer through updates, and that winget is unavailable until the first user logon.
- **Tools:** wingetcreate has no `validate` command and builds only for win-x64 and win-x86. Its v1.28.1.0 (2026-11-11), the first to emit 1.28.0, was under 7 days old. Komac v2.17.2 has no `validate`, and its `--dry-run` still asks for a token.
- **Submission:**
  - One package version per PR, with manifest files only. A new package's PR title is `New package: Publisher.Name version X.Y.Z`. The contributor signs the Microsoft CLA personally.
  - Every automated path needs a classic `public_repo` PAT: wingetcreate (which also offers interactive OAuth; fine-grained tokens and fork-only GitHub Apps fail), Komac, and winget-releaser (which also installs Komac unpinned). There is no OIDC path (winget-pkgs#413785, untriaged).
- **Validation:**
  - A GitHub App runs 10 steps: PR structure; schema and path case; URL reachability, HTTPS and SmartScreen; an official publisher domain; content policy; catalog build; installer scan (SHA-256, several antivirus engines, PUA refused); a non-elevated install with a Defender scan; installer metadata; a summary. Then a moderator approves. A `Portable-Archive` label blocks the PR.
  - A signing requirement is documented only for MSIX. One Rust zip+portable package whose release has no Authenticode step merged on 2026-11-17; its binary wasn't inspected.
- **UNVERIFIED:** a fully offline end-to-end `winget validate` run; winget on `windows-2025`; whether validation step 08 install-tests arm64 installers; whether a `+crt-static` build passes the VC++ runtime check.
- **Re-checked 2026-11-18 (the Release dry-run's `winget validate` failure).** Sources: Release run 44198210620, job `verify install (x86_64-pc-windows-msvc)` (read-only); https://github.com/microsoft/winget-cli at v1.29.380 and master (`src/AppInstallerCommonCore/Manifest/ManifestSchemaValidation.cpp`, `ManifestValidation.cpp`, `src/AppInstallerSharedLib/Public/AppInstallerErrors.h`, `schemas/JSON/manifests/latest/`), the same `ManifestSchemaValidation.cpp` at v1.12.470, v1.28.190, v1.28.240 and v1.29.280, and the v1.29.380 release assets; the runner-images readme for `win25-vs2026/20261103.298`. Status: Verified, except the gaps listed.
  - **What failed:** "Manifest validation succeeded with warnings", one warning per manifest: "The schema header URL does not match the expected pattern", on our `# yaml-language-server: $schema=https://aka.ms/winget-manifest.<type>.1.28.0.schema.json` line (line 2, column 25). The exit code, -1978335192, is `0x8A150028`, `APPINSTALLER_CLI_ERROR_MANIFEST_VALIDATION_WARNING`; `0x8A150029` is `…_FAILURE`.
  - **The check:** `ValidateSchemaHeaderUrl` matches `winget-manifest\.(\w+)\.([\d\.]+)\.schema\.json$`, compares the type and version with the manifest's `ManifestType` and `ManifestVersion`, then requires the URL to equal `$schema=` plus the `$id` of the client's own built-in schema, ignoring case. `LoadSchemaDoc` picks the newest built-in schema not newer than `ManifestVersion`. A client without 1.28 therefore compares our header with an older schema's `$id` and warns. master's only change is a 1.31 branch.
  - **Our headers are right:** the `$id`s in v1.29.380's `latest/` schemas are `https://aka.ms/winget-manifest.installer.1.28.0.schema.json`, `…defaultlocale.1.28.0…` and `…version.1.28.0…`.
  - **Which clients know 1.28:** v1.12.470's newest built-in schema is 1.12. v1.28.190 (2026-02-10), v1.28.240 and v1.29.280 have 1.28. So v1.28.190 is the first stable release that validates our manifests without warnings.
  - **The runner:** image `windows-2025-vs2026` 20261103.298.1 has `winget` on PATH, but the job didn't print its version. That it is older than v1.28.190 is inferred from the warnings. The image's readme doesn't mention winget.
  - **Pins (`release.yml`, `verify-install`):** v1.29.380 (2026-09-21), the release whose schemas `verify-linux` already pins:
    - `Microsoft.DesktopAppInstaller_8wekyb3d8bbwe.msixbundle`, 217,276,577 bytes, SHA-256 `65dea9c01ce08ee7b763366b27c0e651f97db857c11ca9b9c301826c10092f2e`;
    - `DesktopAppInstaller_Dependencies.zip`, 97,760,717 bytes, SHA-256 `ba875afe9d190f61218985ac0292a99d1db710bf93e13c68944ca9d89f0d82d1`.
    - Each matches both GitHub's asset digest and the release's own `.txt` hash file. Neither large file was downloaded here.
  - **Dependencies** (`DesktopAppInstaller_Dependencies.json`): Microsoft.VCLibs.140.00 14.0.33519.0, Microsoft.VCLibs.140.00.UWPDesktop 14.0.33728.0 and Microsoft.WindowsAppRuntime.1.8 8000.616.304.0. The zip holds one `.appx` of each under `arm64/`, `x64/` and `x86/`, read from its central directory with HTTP range requests.
  - **UNVERIFIED:** that `Add-AppxPackage` installs the bundle on `windows-2025` from Windows PowerShell; that `winget --version` then prints exactly `v1.29.380`; whether PowerShell 7 could load the Appx module (the step doesn't rely on it).

### SUPPLY-CHAIN-P5: attestations, cosign, immutable releases, SBOMs and checksums
*Phase 6 note (SIGN-P6, 2026-11-17): the SAN this section derives, `…/release.yml@refs/heads/main`, was the release identity until Phase 6 moved all signing into `sign.yml`. No release was made under it.*

Sources: https://github.com/actions/attest at v4.2.2; https://github.com/cli/cli at v2.103.3 (`pkg/cmd/attestation/`, `pkg/cmd/release/`) and releases v2.49.0, v2.81.0 and v2.102.0; https://github.com/sigstore/cosign at v3.1.4 (`cmd/cosign/cli/`, `pkg/cosign/`, `doc/cosign_verify-blob.md`, `.goreleaser.yml`) and v3.1.3; https://github.com/sigstore/cosign-installer/blob/v4.1.3/action.yml; https://github.com/sigstore/sigstore-go at v1.3.0; https://github.com/sigstore/fulcio/blob/v1.10.0/config/identity/config.yaml; https://raw.githubusercontent.com/sigstore/root-signing/refs/heads/main/targets/signing_config.v0.2.json; real certificates from the GitHub attestations API (`lifthrasiir/aqueous`, `Homebrew/brew`); https://docs.github.com/en/actions/reference/security/oidc; https://github.com/github/docs (`content/code-security/concepts/supply-chain-security/immutable-releases.md`, `content/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations.md`); https://github.com/github/rest-api-description; https://github.com/CycloneDX/cyclonedx-rust-cargo at `769fa7dc5d23`; https://github.com/rust-secure-code/cargo-auditable at v0.7.7; https://github.com/coreutils/coreutils/blob/v9.12/doc/coreutils.texi. Status: Verified, except the gaps listed. No cosign, gh or SBOM tool was run; real certificates were decoded with `openssl x509`.

- **Pins:**
  - `actions/attest` v4.2.2 (2026-08-04), commit `1e69f48acb82d1966a394da916b4c1698aa569d6`.
  - `sigstore/cosign-installer` v4.1.3 (2026-11-04), commit `756fe2d284c7292c4e9b62c8a28f1e5629c59532`. Its default is cosign v3.1.4 (2026-10-20, commit `0e48502cf5ef`), whose linux-amd64 SHA-256 is `424e2c97d0b45fd7a6b608a87f090f35633f4c23ece5f092d48bba11aaf4bbfa` (`cosign_checksums.txt`).
  - cargo-auditable 0.7.7, through `taiki-e/install-action` v2.90.3, whose manifest hashes match the tool's `sha256.sum`.
  - cargo-cyclonedx 0.5.10: the `x86_64-unknown-linux-musl` tarball has SHA-256 `528be9dd99ae522c3ee14fb881fd0cbf51767e8f12b74248f437e6a66b1a3c1b` (its `sha256.sum`). install-action v2.90.3 has only 0.5.9; v2.90.4 (2026-11-14) adds 0.5.10 and becomes eligible on 2026-11-21. The prebuilt archives cover 5 targets, without aarch64 musl or Windows arm64.
  - gh 2.103.3, preinstalled on the runners, is the latest release at least 7 days old.
- **`actions/attest`:**
  - Subjects: `subject-path` (a glob or a list; files only; at most 1024; named by basename), `subject-digest` with `subject-name`, or `subject-checksums` (untagged shasum lines only; no `--tag` lines or escaped names).
  - With no predicate inputs it writes SLSA provenance v1. `sbom-path` (SPDX or CycloneDX JSON up to 16 MB; CycloneDX needs `bomFormat`, `serialNumber` and `specVersion`) writes predicate type `https://cyclonedx.org/bom`. That means one step per predicate type.
  - Permissions: `id-token: write` and `attestations: write`, plus `contents: read` to check out. `artifact-metadata: write` is only for storage records, which need `push-to-registry` and an organization-owned repository.
  - **There is no dry-run input.** It fails at once without `id-token`, so it can't run on fork or Dependabot PRs. A public repository signs through public-good Sigstore, and every run writes a Rekor entry.
- **gh verification:**
  - `gh attestation verify` exists since v2.49.0, but use **2.102.0 or later**: that release fixed `--signer-workflow` prefix matching (GHSA-wjmr-j3rp-mh2g) and case-insensitive `--source-ref` (GHSA-4mq3-hpgx-9cx8).
  - One of `--repo` and `--owner` is required, and both are compared case-insensitively.
  - `--signer-workflow [host/]OWNER/REPO/.github/workflows/FILE` becomes the anchored, case-sensitive regex `^https://github.com/…(@(refs/.*|hex))?$`. It excludes `--signer-repo`, `--cert-identity` and `--cert-identity-regex`.
  - Other flags: `--source-ref` (case-sensitive), `--source-digest`, `--signer-digest`, `--deny-self-hosted-runners`, `--cert-oidc-issuer` (GitHub's by default), `--predicate-type` (SLSA provenance v1 by default).
  - Offline: `--bundle` plus `--custom-trusted-root`, made with `gh attestation trusted-root`. With `--bundle`, gh checks only the names in the certificate, so it can't notice a repository rename or reuse.
  - **Getting the bundle** (gh v2.103.3 `pkg/cmd/attestation/{download/download.go,verify/attestation.go,api/attestation.go,trustedroot/trustedroot.go}`, read in the Phase 5 review on 2026-11-17, not run):
    - `gh attestation download <file> --repo OWNER/REPO`, optionally with `--predicate-type`, saves a file's attestations as `sha256-<hex>.jsonl` and prints the path. That it writes to the current directory comes from its help text; the code that writes it wasn't read (UNVERIFIED).
    - `gh attestation trusted-root` prints the trust root to stdout and needs the network.
    - `gh attestation verify --bundle` skips any bundle without a DSSE envelope. A cosign `sign-blob` bundle holds a message signature instead, so gh reports "no attestations found".
  - `gh release verify [<tag>]` and `gh release verify-asset [<tag>] <file>` (since v2.81.0) check only GitHub's release attestation (SAN `https://dotcom.releases.github.com`). They can't check the source archives GitHub generates.
- **cosign:**
  - Require **3.1.3 or later**, which fixed a verification bypass (GHSA-fx35-mq7g-6g98).
  - cosign-installer checks a SHA-256 of the bootstrap v3.1.4 binary, hard-coded in `action.yml`. With the default version, that is the only check. On Windows it supports X64 only, so it fails on `windows-11-arm`, and there is no windows-arm64 cosign binary.
  - **Keyless signing:** `cosign sign-blob --yes --bundle <f>.sigstore.json <f>`, with `id-token: write`.
    - `--bundle` is required in v3. The new bundle format and the TUF signing config are the defaults; `--new-bundle-format`, `--use-signing-config` and `--tlog-upload` are deprecated.
    - The public-good signing config includes Fulcio, Rekor v1 and the timestamp authority, so a bundle carries the certificate, a Rekor entry and a timestamp.
    - `--yes` skips the privacy prompt. Bundles are written with mode 0600. cosign's own releases name them `<file>.sigstore.json`.
  - **Verifying:** `cosign verify-blob <f> --bundle <f>.sigstore.json --certificate-identity <SAN> --certificate-oidc-issuer https://token.actions.githubusercontent.com`.
    - The SAN comparison is exact and case-sensitive. `--certificate-identity-regexp` is RE2 and unanchored, so anchor it.
    - `--certificate-github-workflow-{repository,ref,trigger,sha,name}` exact-match the deprecated OIDs 1.2 to 1.6. No flag checks the repository or owner ID extensions.
  - **Offline dry-run** (from the source, not run):
    1. `COSIGN_PASSWORD=<random> cosign generate-key-pair --output-key-prefix dryrun`;
    2. `cosign signing-config create --out f.json`, which names no services;
    3. `cosign sign-blob --yes --key dryrun.key --signing-config f.json --bundle <f>.sigstore.json <f>`, which uploads nothing and fetches no trust root;
    4. `cosign verify-blob --key dryrun.pub --bundle <f>.sigstore.json --insecure-ignore-tlog <f>`.
    - `--tlog-upload=false` together with a signing config is refused.
- **The Fulcio identity** (answers the open question from Phase 0):
  - Fulcio v1.10.0 builds the SAN as `{{ .url }}/{{ .job_workflow_ref }}`, with `https://github.com`. For Grimoire that is `https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main`, because releases run on a push to `main`.
  - For a top-level workflow, `job_workflow_ref` equals `workflow_ref`: OID 1.9 equals OID 1.18 in real certificates. A reusable workflow gets its own path and ref.
  - Case is preserved, as real certificates for `App-vNext/Polly` and `Homebrew/brew` show.
  - The immutable `sub` doesn't change the SAN. It appears only in OID 1.3.6.1.4.1.57264.1.24, as a certificate for a repository created on 2026-10-13 shows. The repository and owner IDs appear only in extensions 1.15, 1.17 and 1.24.
- **Immutable releases:**
  - Turned on in Settings → Releases → "Enable release immutability", for future releases only. REST: `GET`, `PUT` and `DELETE /repos/{owner}/{repo}/immutable-releases`. The `GET` needs the Administration permission, which `GITHUB_TOKEN` can't have, so CI checks the release object's `immutable` field instead. The setting's current state on this repository is unknown (403 for the research token).
  - Once a release is published, its tag can't be moved, or deleted while the release exists, and its name can never be reused. Its assets are locked. The title, notes and pre-release and latest flags stay editable.
  - The flow is draft, upload, publish. `gh release create` with assets already does that, and deletes the draft on failure. Release assets carry a `digest` field.
  - `GITHUB_TOKEN` with `contents: write` can create releases, but creating or editing one fails if its target commit changes `.github/workflows` relative to the default branch. Whether a target commit that is an ancestor of a newer default branch counts as changing them is UNVERIFIED.
  - `contents: write` also covers creating, moving and deleting refs (tags and branches), creating, publishing and deleting releases and their assets, and merging pull requests (GitHub's fine-grained permission data, checked in the Phase 5 review). A newly published release becomes "latest" unless `make_latest` says otherwise (REST description).
  - Before a release is published, nothing locks its tag or its draft. A release that was published while the setting was off can be deleted with its tag kept; publishing that tag again should make an immutable release (reasoned from the facts above, not tried).
  - GitHub's release attestation is signed by GitHub's own Sigstore instance (SAN `https://dotcom.releases.github.com`, with an RFC 3161 timestamp). Its predicate type is `https://in-toto.io/attestation/release/v0.2`, and its subjects are the tag (with its commit) and each asset's SHA-256.
  - GitHub renames asset files whose names contain special characters or start or end with a period. Keep to `[A-Za-z0-9._-]`.
- **SBOMs:**
  - cargo-cyclonedx 0.5.10 flags: `--spec-version` 1.3 to 1.7 (default 1.3), `-f json|xml` (default xml), `--describe crate|binaries|all-cargo-targets`, `--target <triple>|all`, `--target-in-filename`. `--target` only filters `cargo metadata` (`--filter-platform`); nothing is built.
  - **There is no `-p`.** It writes one file per workspace member, next to that member's `Cargo.toml`: `<bin>_bin[_<triple>].cdx.json` with `--describe binaries`.
  - `SOURCE_DATE_EPOCH` fixes the timestamp but drops `serialNumber`, which `actions/attest`'s `sbom-path` requires. Grimoire leaves it unset.
  - cargo-auditable 0.7.7 passes its arguments straight to cargo. It embeds a Zlib-compressed, sorted, timestamp-free dependency list in a `.dep-v0` linker section. Readers: cargo audit 0.17.3+, trivy 0.31.0+, grype 0.83.0+, syft 1.15.0+, osv-scanner 2.0.1+ (images), blint 2.1.3+, wasm-tools 1.227.0+, rust-audit-info and auditable2cdx.
- **Checksums:**
  - A GNU untagged line is `<hex><space><' ' or '*'><name>`. A name containing `\`, a newline or CR is escaped, and the line starts with `\`. `sha256sum -c` also reads tagged lines, and `--ignore-missing` checks only the files present.
  - Consumers: winget needs `InstallerSha256` in the manifest; Scoop reads `$url.sha256` or an aggregate list; cargo-binstall reads none.
- **UNVERIFIED:** when a draft release's tag is created (no primary source says); whether `--filter-platform` gives correct SBOMs for targets other than the host.
- **Settled in Phase 6:** GHCR support for OCI 1.1 referrers (REGISTRY-P6). GHCR returns 404 for the referrers API, so cosign lists an image signature in the `sha256-<hex>` fallback tag.

### ACTIONS-P5: runners, targets, build flags, artifacts, environments and OIDC
Sources: https://docs.github.com/en/actions/reference/runners/github-hosted-runners; https://github.com/actions/runner-images at main `5d331217d4` (`images/{ubuntu,macos,windows}/*-Readme.md`; issues 14017, 14083, 14602, 14748, 15314 and 15315); Rust at tag 1.100.0 (`src/doc/rustc/src/{platform-support.md,platform-support/apple-darwin.md,remap-source-paths.md,codegen-options/index.md}`, `compiler/rustc_target/src/spec/`); Cargo at `3acc79c0c841` (`doc/book/src/reference/{config,profiles,unstable}.md`, `doc/book/src/commands/cargo-metadata.md`); https://github.com/actions/upload-artifact/blob/v7.0.2/README.md, https://github.com/actions/download-artifact/blob/v8.0.2/README.md; https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments, https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/review-deployments, …/manage-environments; https://docs.github.com/en/actions/concepts/security/github_token; https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks; https://docs.github.com/en/rest/actions/oidc; https://github.com/zizmorcore/zizmor at v1.32.0; https://github.com/rhysd/actionlint at v1.7.12. Status: Verified, except the gaps listed. Only `cargo metadata` and `cargo tree` were run, locally.

- **Pins:** `actions/upload-artifact` v7.0.2 (`cf430e030ddbb5b0abf93d22962f4752f3646cd9`) and `actions/download-artifact` v8.0.2 (`9000827ccba6bdab643e8b6fd33ac0654aef8333`), both 2026-10-07, on node24. The CI-PINS pins are unchanged.
- **Runners** (all free on public repositories):

  | Label | Image on 2026-11-17 | Contents |
  |---|---|---|
  | `ubuntu-24.04` | 20261111.370.1 | Rust 1.99.1, Node 22.23.4, Python 3.12.3, pwsh 7.6.10, PSScriptAnalyzer 1.25.0, apt shellcheck 0.9.0, gh 2.103.3, Homebrew 7.0.13 (not on PATH) |
  | `ubuntu-24.04-arm` | 20261111.182.1 | as `ubuntu-24.04` |
  | `macos-15` (arm64) | 20261104.0514.1 | Rust 1.99.1, Homebrew 7.0.12, pwsh 7.6.9; no shellcheck |
  | `macos-15-intel` | 20260824.0482.1, the latest x64 release | Rust 1.98.0, Homebrew 6.0.18, pwsh 7.6.4: stale |
  | `windows-2025` | Server 2025 with VS2026, 20261103.298.1 | Rust 1.99.1, pwsh 7.6.9, PSScriptAnalyzer 1.25.0; no winget listed |
  | `windows-11-arm` | Windows 11 Arm64 with VS2026, 20261112.217.1 | Rust 1.100.0 (aarch64 host), Node 24.22.3, pwsh 7.6.10; no winget |

  - `windows-latest`, `windows-2025` and `windows-2025-vs2026` map to the VS2026 image; the migration issue closed on 2026-07-06. `windows-11-arm` maps to the Arm64 VS2026 image; that issue closed on 2026-11-17. Arm64 images moved to `actions/runner-images`, and `actions/partner-runner-images` is archived.
  - `ubuntu-latest` moves to 26.04 in a rollout from 2026-10-19 to 2026-11-19. On 2026-11-17 the README still mapped it to 24.04.
  - From 2026-11-30, in rollouts of 2 to 3 days: the Windows 11 Arm64 workspace moves to `D:` (#15314), and `D:` becomes ReFS on the Server 2022, Server 2025 VS2026 and Windows 11 Arm64 VS2026 images (#15315).
  - `macos-26-intel` is a standard public label. The 2025 statement that Intel macOS ends with macOS 15 in Fall 2027 is stale, and no current end date was found.
  - **Re-checked 2026-11-17 (RUN-P6):** the 20261116 images (`ubuntu-24.04` 20261116.376.1, `ubuntu-24.04-arm` 20261116.188.1, Buildx 0.39.0) were published as pre-releases at 22:21Z on 2026-11-17, which marks the start of their deployment. Deployment usually takes 2 to 3 days, so until it ends a job may get 20261111 (Buildx 0.38.2) or 20261116; its "Set up job" log names the image. Both ship Docker 28.0.4; that it uses the classic image store is inferred (UNVERIFIED: `docker info` wasn't run on a runner). See RUNNERS.
- **Targets (Rust 1.100.0):**
  - `x86_64-unknown-linux-musl` and `aarch64-unknown-linux-musl`: Tier 2 with host tools (musl 1.2.5). Both set `crt_static_default = true` and link self-contained CRT objects whenever crt-static is on. No crate built for any target compiles C (`cc` appears only behind `cfg(target_os = "haiku")`), so no `musl-tools` is needed.
  - `x86_64-pc-windows-msvc` and `aarch64-pc-windows-msvc`: Tier 1 with host tools. The CRT is dynamic by default and static with `-C target-feature=+crt-static`.
  - `aarch64-apple-darwin`: Tier 1, minimum macOS 11.0. `x86_64-apple-darwin`: Tier 2 with host tools, minimum 10.12. `MACOSX_DEPLOYMENT_TARGET` is honoured.
- **Build flags:**
  - Cargo's sources of compiler flags are exclusive, and the first match wins: `CARGO_ENCODED_RUSTFLAGS`, then `RUSTFLAGS`, then the config's `target.<triple>.rustflags`, then the rest. With `--target`, none of them reach build scripts or proc macros.
  - `--remap-path-prefix FROM=TO` is a textual rewrite, and the last match wins. It doesn't cover paths the linker writes (MSVC `.pdb`, Apple OSO entries). `--remap-path-scope` is stable; `trim-paths` is still unstable.
  - Release profile defaults: opt-level 3, debug false, lto false, panic unwind, incremental false, codegen-units 16.
  - The profiles page says `strip` defaults to `"none"`, but `debug = false` has implied `strip = "debuginfo"` since 1.77 (and rustc ignores `-C strip` on MSVC). Set it explicitly. rustc warns that `strip = "symbols"` degrades backtraces and isn't a security measure.
- **Finding asset binaries:** `cargo metadata --format-version 1 --no-deps --locked` gives `packages[]`, each with `name`, `version`, an absolute `manifest_path`, `publish` (`null` when unrestricted, `[]` when forbidden) and `targets[]` with `kind`.
  - Filter on `kind` containing `"bin"`, not on `crate_types`, which examples share.
  - Keep packages whose `manifest_path` is under `catalog/tools/` or `catalog/mcp-servers/`. Today that gives `grimoire-tool-secret-guard` and `grimoire-mcp-project-docs`.
  - Run it on Linux.
- **Artifacts:**
  - Artifacts from v4 on are immutable, and `overwrite: true` creates a new ID. `retention-days` is 1 to 90.
  - Zipping resets modes to 644 or 755, so tar executables first.
  - download-artifact v8 fails on a digest mismatch by default, and downloads only from the current run. Keep everything in one run, and re-hash files before attesting.
- **Environments:**
  - Up to six required reviewers, and one approval suffices. On Free plans they are available only for public repositories.
  - "Prevent self-review" stops the person who started the run from approving it, even as a required reviewer. Administrators can bypass the rules by default; that can be turned off per environment. Who counts as the initiator of a re-run, or of a push made by merging a PR, isn't documented.
  - Deployment branch and tag rules are fnmatch patterns matched against `GITHUB_REF`, and `*` doesn't match `/`. Wait timers are 1 to 43,200 minutes.
  - **A run that references a missing environment creates it, with no protection rules.**
- **OIDC:**
  - This repository has `use_immutable_subject: true` and the `sub` prefix `repo:jabbott-iii@84321953/Grimoire@1460666977` (REST `actions/oidc/customization/sub`). A job in `release` should get `<prefix>:environment:release`; the docs give a literal example only for the branch form.
  - Claims include `workflow_ref`, `job_workflow_ref`, `repository`, `repository_id`, `repository_owner`, `repository_owner_id`, `environment`, `ref`, `event_name`, `workflow_sha` and `runner_environment`.
- **Events caused by `GITHUB_TOKEN`:**
  - They create no runs, with these exceptions: `workflow_dispatch` and `repository_dispatch` always do, and since 2026-05-22 `pull_request` opened, synchronize and reopened events create runs that wait until a user with write access approves them.
  - Pushes, tags and releases made with `GITHUB_TOKEN` still trigger nothing.
  - Checks from a `workflow_dispatch` run on a PR's head branch don't satisfy a ruleset's required status checks. Checks must pass on the latest head SHA.
- **zizmor and actionlint:** see the ZIZMOR note.
- **Checked in the Phase 5 review (2026-11-17):**
  - **Limits:** "A job matrix can generate a maximum of 256 jobs per workflow run" (https://docs.github.com/en/actions/reference/limits). "Up to 1000 release assets may be associated with a single release" (https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases). `actions/attest` v4.2.2 takes at most 1024 subjects and fails above that (`src/subject.ts`). Whether the 256 counts each matrix or every matrix in the run is UNVERIFIED.
  - **Re-runs** (https://docs.github.com/en/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs, re-fetched 2026-11-17): allowed "up to 30 days after its initial run"; a re-run uses "the same `GITHUB_SHA` (commit SHA) and `GITHUB_REF` (git ref) of the original event"; re-running failed jobs or one job "also re-runs the jobs that depend on them". The page says nothing about environment approvals on a re-run.
  - **Scheduled workflows** (https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows, `schedule`, re-fetched 2026-11-17): "In a public repository, scheduled workflows are automatically disabled when no repository activity has occurred in 60 days." `gh workflow enable` or the Actions tab turns one on again. What counts as activity, and whether a disabled workflow can still be started by hand, are UNVERIFIED.
  - **Container actions get the OIDC request token** (actions/runner `src/Runner.Worker/Handlers/ContainerActionHandler.cs` on `main`, not a pinned runner version): a container action receives `ACTIONS_ID_TOKEN_REQUEST_URL`, `ACTIONS_ID_TOKEN_REQUEST_TOKEN` and `ACTIONS_RUNTIME_TOKEN`. The audience is a query parameter the caller picks (GitHub's OIDC docs). So any code in a job with `id-token: write` can mint a token for any audience, carrying that job's claims, `environment` included.
  - **Skipped jobs and required checks** (troubleshooting-required-status-checks, in the sources above): a job skipped because a job it needs failed reports success, so a required check needs a summary job with `if: always()`.
  - **Concurrency:** with `cancel-in-progress: true`, a new run cancels the running one in its group; without it, the new run waits as pending.
- **UNVERIFIED:** winget on `windows-2025`; what `download-artifact`'s `merge-multiple` does when names collide; the literal `sub` of an environment job; any GitHub statement about the immutable `sub` and crates.io, npm, PyPI or Fulcio (each registry's own source answers it: RUST-REGISTRY-P5, NPM-P5, PYPI-P5, SUPPLY-CHAIN-P5); whether Windows builds are byte-for-byte reproducible (PE timestamps, PDB paths).

### INSTALLERS-P5: the dist spike, ShellCheck and PSScriptAnalyzer
Sources: https://github.com/axodotdev/cargo-dist at v0.34.0 (`cargo-dist/templates/installer/installer.{sh,ps1}.j2`, `cargo-dist/src/{tasks,manifest,host,lib}.rs`, `cargo-dist/src/backend/installer/shell.rs`, `book/src/`, tests and snapshots), issues #439 and #1672, and PR #2497; https://github.com/koalaman/shellcheck/releases/tag/v0.11.0; https://www.powershellgallery.com/api/v2/FindPackagesById()?id='PSScriptAnalyzer'; https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.psresourceget/install-psresource; runner-images readmes; Apple's `text_cmds` at its `distribution-macOS` tags. Status: Verified, except the gaps listed. No dist, ShellCheck or PSScriptAnalyzer binary was downloaded or run.

- **dist v0.34.0** (2026-11-02, a lightweight tag on `294c1b25d372`; the release is immutable, and its musl tarball has 2 attestations):
  - The installer templates are compiled into the binary and can't be overridden. Only `dist build --artifacts=global` (or `host` or `all`) writes the installers; `dist generate` handles only `ci` and `msi`.
  - **`installer.ps1` checks no checksum at all** (PR #2497 is open).
  - **`installer.sh` checks SHA-256 only with `sha256sum`.** If that is missing, it skips the check and carries on. If no hash was embedded, it prints "no checksums to verify" and installs anyway. Hashes are embedded only from the `*dist-manifest.json` files that dist's own builds write (issue #1672).
  - Using them without dist's CI would need dist's archive naming (`{package}-{target}{ext}`, with tarballs nested in a root directory), a GitHub `repository` URL, the mandatory `cargo-dist-version`, a dist binary in the release job, and hand-made manifests.
  - Defaults: install to `$CARGO_HOME/bin`; edit `.profile`, existing bash files, `.zshrc` (created if missing) and fish config, or the HKCU `Path`; append to `$GITHUB_PATH`; write a receipt; send a Bearer header to any download URL when `{APP}_GITHUB_TOKEN` is set; download with `curl -sSfL`, with no `--proto`. No telemetry.
  - Maintenance: 108 commits since 2026-08-16 (62 from Dependabot, 29 from the maintainer, 17 from 9 others). The maintainer's latest activity was on 2026-11-17.
- **Tools for our scripts:**
  - ShellCheck v0.11.0 (2025-08-04) is the latest. Pin the CI-uploaded `shellcheck-v0.11.0.linux.x86_64.tar.xz`, SHA-256 `8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198`. That is the GitHub asset digest: there is no checksum file, no attestation, and the release isn't immutable. The `.tar.gz` was uploaded by hand on 2026-01-05. ubuntu-24.04 has only apt shellcheck 0.9.0.
  - PSScriptAnalyzer 1.25.0 (2026-03-20) is the Gallery's latest, and is preinstalled on ubuntu-24.04, windows-2025 and the Windows Arm64 image. The Gallery's SHA-512 `PackageHash`, in hex, is `e7fb573262e604bab29914ae62921d24b97c2f88b519b41277b403ef6ce163f14b7d1b36dc73449a68e5ae5a8d9502891b10c264131fd1813adf29bf131c1663`. `Save-PSResource -Name PSScriptAnalyzer -Version 1.25.0 -AsNupkg`, then `Get-FileHash -Algorithm SHA512`, can check it.
  - SHA-256 tools: macOS 15 and later ship `/sbin/sha256sum`, a link to `md5` (from `text_cmds-190.0.1`). macOS 14 and earlier don't have it.
- **UNVERIFIED:** whether stock macOS ships `shasum`, and whether `/sbin` is on PATH; that the Gallery's `PackageHash` covers the `.nupkg` byte for byte (inferred from NuGet's catalog docs); the licence of dist's generated installer output.
- **Behaviours Grimoire's scripts rely on** (checked 2026-11-17, for `install.sh`, `install.ps1` and the docs that tell users how to fetch them). Sources were read in their saved copies unless a line says the Phase 5 review checked them.
  - **Rosetta** (Apple's "About the Rosetta translation environment", https://developer.apple.com/tutorials/data/documentation/apple-silicon/about-the-rosetta-translation-environment.json):
    - Apple's example reads `sysctl.proc_translated` and returns 0 for a native process, 1 for a translated one and -1 on an error. It treats `ENOENT`, a missing key, as native.
    - The page doesn't say that Intel Macs lack the key. XNU defines it only under `CONFIG_ROSETTA` (xnu `main` at `f6217f891ac0`, `bsd/kern/kern_sysctl.c`, read in the review), so that is plausible but UNVERIFIED. `install.sh` treats anything but exactly `1` as Intel, which is right either way, though its comment states the Intel claim as fact.
  - **curl** (https://curl.se/docs/manpage.html, describing curl 8.23.1):
    - `--proto =https` permits only HTTPS.
    - `--proto-redir` limits redirects, and "Protocols denied by --proto are not overridden by this option". So `--proto =https` also refuses a redirect to HTTP.
    - `--tlsv1.2` means TLS 1.2 or later. In old curl versions it was documented as TLS 1.2 only.
    - Which curl version added each option wasn't checked.
  - **wget, and why `install.sh` doesn't use it** (checked in the review):
    - GNU Wget's manual (https://www.gnu.org/software/wget/manual/wget.html): `--https-only` applies "When in recursive mode". The review ran GNU Wget 1.21.4 with it, and it followed a redirect from HTTPS to HTTP.
    - BusyBox wget has no `--https-only` (BusyBox 1.38.0 `networking/wget.c`, from sources.debian.org). Alpine's base image has BusyBox wget and no curl.
  - **`Invoke-WebRequest`** (https://learn.microsoft.com/powershell/module/microsoft.powershell.utility/invoke-webrequest, the 5.1 and 7.6 pages, both `ms.date` 2026-01-20):
    - PowerShell 7.4 and later: "By default, any request that's redirected from HTTPS to HTTP results in an error", unless `-AllowInsecureRedirect` is passed. The review saw PowerShell 7.6.9 refuse such a redirect.
    - Windows PowerShell 5.1: the page has no such parameter and doesn't say what happens on an HTTPS-to-HTTP redirect (UNVERIFIED). `-MaximumRedirection` defaults to 5.
    - Windows PowerShell 5.1: `-UseBasicParsing` "is required when Internet Explorer is not installed". Since the 2025-12-09 update for CVE-2025-54100 (KB5074596), a call without it shows a "Script Execution Risk" prompt, and "There is no way to bypass this prompt without using the UseBasicParsing parameter".
  - **`Expand-Archive`** (https://learn.microsoft.com/powershell/module/microsoft.powershell.archive/expand-archive, the 5.1 page, `ms.date` 2026-04-01): in the `Microsoft.PowerShell.Archive` module, zip archives only, with `-LiteralPath`, built on `System.IO.Compression.ZipArchive`.
  - **`Get-FileHash`** (https://learn.microsoft.com/powershell/module/microsoft.powershell.utility/get-filehash, the 5.1 page, `ms.date` 2022-12-12): SHA256 by default. Its example output shows the hash in upper case, so `install.ps1` lowercases it before comparing.
  - **TLS 1.2 in .NET** (https://learn.microsoft.com/dotnet/api/system.net.securityprotocoltype, updated 2026-05-27, from dotnet-api-docs `xml/System.Net/SecurityProtocolType.xml`): `Tls12` is 3072 and "On Windows systems, this value is supported starting with Windows 7"; `SystemDefault` is 0. Which Windows PowerShell 5.1 installations don't offer TLS 1.2 by default is UNVERIFIED. Microsoft's PowerShellGet install page for 5.1 still tells users to add it to `[Net.ServicePointManager]::SecurityProtocol` (checked in the review).
  - **`File.Replace` and `ReplaceFileW`:**
    - `System.IO.File.Replace(source, destination, backup)` (dotnet-api-docs `xml/System.IO/File.xml`): it replaces the destination's contents with the source's and keeps the replaced file as the backup; `null` means no backup. Source and destination must be on one volume.
    - On Windows it calls `ReplaceFileW` (dotnet/runtime `FileSystem.Windows.cs` and .NET Framework's `file.cs`, read in the review).
    - `ReplaceFileW` (https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew, updated 2025-07-01): all three files must be on one volume. `ERROR_UNABLE_TO_REMOVE_REPLACED` (1175): both files keep their names. `ERROR_UNABLE_TO_MOVE_REPLACEMENT` (1176): with a backup, both keep their names; without one, the replaced file no longer exists. `ERROR_UNABLE_TO_MOVE_REPLACEMENT_2` (1177): the replaced file still exists under another name, the backup name when one was given. Any other error: both keep their names and no backup exists.
    - So with a backup name the old binary always survives, under its own name or the backup name, and `install.ps1` moves it back.
  - **`RuntimeInformation.OSArchitecture`** (dotnet-api-docs `xml/System.Runtime.InteropServices/RuntimeInformation.xml`): "intended to represent the actual architecture of the underlying operating system", taking Windows and macOS emulation into account. The review found it available in .NET Framework only from 4.7.1, so `install.ps1` falls back to environment variables. dist's note that it reports X64 on Arm64 before PowerShell 7.3 or Windows 11 22H2 is UNVERIFIED.
  - **WOW64** (Microsoft's "WOW64 Implementation Details", checked in the review): a 32-bit process sees `PROCESSOR_ARCHITECTURE=x86`, and `PROCESSOR_ARCHITEW6432` holds the real architecture (`AMD64` or `ARM64`).

---

## Phase 6 additions (checked 2026-11-17)

*After the review, a sixth topic, SIGN-P6 (signing from a reusable workflow), was researched and re-checked the same way: 42 facts, 40 confirmed, 1 corrected and 1 left UNVERIFIED. It is the last section below.*

These facts came from a five-topic research workflow for the container image: the distroless base image, BuildKit and buildx, GHCR and image signing, Grype, and running the image. An independent skeptic re-checked each topic against its primary sources. Of 333 facts, 308 were confirmed, 25 corrected and none refuted: base image 52 of 59 (7 corrected), build 59 of 62 (3 corrected), registry 64 of 68 (4 corrected), scan 63 of 68 (5 corrected), run 70 of 76 (6 corrected).

Corrected facts appear below in corrected form, and facts the skeptics added are included where an implementer needs them. The research itself built, pushed, scanned and ran no image: it read docs, source and registry APIs, and RUN-P6 ran only `cargo tree` and `grep` locally. Afterwards the maintainer approved local tooling for Phase 6: rootless Docker 29.9.3 from the installed `dockerd-rootless.sh`, run as the maintainer's user and stopped at the end; Grype 0.121.3, cosign 3.1.4 and crane v0.22.1 in the scratchpad, each checked against its published SHA-256 (both recorded in PLAN.md); and later the `aarch64-unknown-linux-musl` Rust target, to build real arm64 binaries. Facts confirmed that way are marked "Checked locally", and each UNVERIFIED item they settle is removed, with a note on how. The local machine is x86_64 with no emulation, so no arm64 image was run. Rootful Docker, Docker Desktop, Podman, GHCR itself and GitHub Actions weren't used: nothing here has run on GitHub. The raw results are not committed. Where an entry below differs from an earlier entry, this one wins, and the older entry carries a "Re-checked 2026-11-17" note. The decisions these facts led to are in ADR-0011.

### BASE-IMAGE-P6: distroless base, OCI annotations, GHCR metadata and Dependabot for Dockerfiles
Sources: the anonymous registry API at https://gcr.io/v2/distroless/static-debian13/ (`tags/list`, `manifests/nonroot`, and the manifests, configs, layers, `.sig`, `.att` and `referrers/` of the digests below) and https://gcr.io/v2/distroless/static-debian12/tags/list; https://github.com/GoogleContainerTools/distroless at `0af09fb8f491` (`README.md`, `SUPPORT_POLICY.md`, `RELEASES.md`, `.cloudbuild/`, `private/oci/sign_and_push.go`, `private/tools/lifecycle/tag.sh`, `.github/workflows/update-deb-package-snapshots.yml`) and PR #2248; https://cloud.google.com/artifact-registry/docs/transition/gcr-repositories (updated 2026-11-10); https://github.com/opencontainers/image-spec at v1.1.1 (`annotations.md`, `image-index.md`, `manifest.md`, `config.md`) and `main` at `4feed4a6674e`; https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md; https://github.com/moby/buildkit at v0.35.0 (`docs/annotations.md` and the Go source); https://github.com/docker/buildx at v0.38.2 (Go source); https://github.com/docker/docs at `947343021a3e` (`content/manuals/build/metadata/annotations.md`); https://github.com/github/docs at `49a012fa50b4` (`content/packages/`, `content/code-security/reference/supply-chain-security/`, `data/reusables/package_registry/`, `data/reusables/dependabot/supported-package-managers.md`); https://github.com/dependabot/dependabot-core at v0.405.0 (`docker/`, `github_actions/`) and commit `3bfea916472b`; https://github.com/sigstore/cosign at v3.1.4 (`cmd/cosign/cli/verify/`, `cmd/cosign/cli/options/`, `pkg/cosign/verify.go`, `pkg/cosign/tuf.go`); https://github.com/anchore/syft/blob/v1.57.1/syft/pkg/cataloger/debian/cataloger.go; https://github.com/anchore/grype/blob/v0.121.3/go.mod; https://ghcr.io/v2/sigstore/cosign/cosign/ (one public image); https://cgr.dev/v2/chainguard/static/tags/list; https://edu.chainguard.dev/chainguard/chainguard-images/how-to-use/verifying-chainguard-images-and-metadata-signatures-with-cosign/ (updated 2026-11-03). Status: Verified, except the gaps listed. The research pulled and built no image, and ran no cosign, Grype or Syft binary. Layer blobs were downloaded, checked against their digests and listed with `tar`; certificates were decoded with `openssl x509`.

- **Pins** (each body's SHA-256 matched its digest; ages come from GCR's `timeUploadedMs` and match the Rekor times):
  - **Current `static-debian13:nonroot`:** index `sha256:aafe993665bf6200db38a02c11a9351e782a84b6a45e7f33b6695638c88e3931`, unchanged since BASE-IMAGE.
    - Built from distroless `0af09fb8f491449b27f27f87c4dd10a56b401be1` and first uploaded 2026-11-12T21:16:35Z.
    - **It fails the 7-day rule until 2026-11-19T21:16:35Z.**
    - Children: linux/amd64 `sha256:816dc4d589459a7fe33b8a2e69edb70da56ffad7161f4b415937fa2bfe7e92e8` and linux/arm64/v8 `sha256:50e178649a8d77a5d8414f205471c829f06b876cbdb2455dace0b5de549fd1e5`.
  - **Newest eligible index:** the previous one, `sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329`.
    - First uploaded 2026-10-31T22:37:00Z and signed 2026-10-31T22:37:04Z (Rekor logIndex 3615240227). Six distroless commits tag it.
    - Children: linux/amd64 `sha256:336e53e9941facdb69b755e6fd2c63f5400abd3b1ba47bb17ebafc2b1547cebd` and linux/arm64/v8 `sha256:bf6233b64054d17b31c3b555b1bbcee58958896830a305ef4d51faa257990d87`.
    - **Grimoire's `Dockerfile` pins this index**, on its one `FROM` line. Moving to `aafe9936…` waits until that index is 7 days old.
  - **The two differ only in layer 12.** PR #2248 (merged 2026-11-12T21:12:38Z as `0af09fb8`) changed `/usr/lib/os-release` from mode 0555 to 0644. The file's content (SHA-256 `b4d56a90bdd5`) and every other layer are identical, and the configs differ only in that layer's diff_id.
  - **Reference versions** (not build inputs):

    | Project | Version | Released | Commit | 7-day rule |
    |---|---|---|---|---|
    | cosign | v3.1.4 | 2026-10-20 | `0e48502cf5ef34e2e60bbd3617aa19945712f705` | eligible |
    | image-spec | v1.1.1 (still the latest) | 2025-03-03 | `147f9c13cedb47a0c4d9a11a222961073d585877` | eligible |
    | dependabot-core | v0.405.0 | 2026-11-16 | `da6dfa3b074a41c03dc4458a84df3973a04e63b7` | reference only |
    | BuildKit | v0.35.0 | 2026-11-11 | `f55949dea4f03bb9d85b73e9afc9a609291289db` | too young; v0.34.2 (2026-11-03) is the newest eligible |
    | buildx | v0.39.0 | 2026-11-11 | | too young; v0.38.2 (2026-11-03, `3c4cbd35852fc1a476923627a780a7836c28b654`) is the newest eligible |
    | syft | v1.57.1 | 2026-11-09 | `a0c0d3890a9d79421c403e8fcadec50a2e55bf21` | eligible |
    | grype | v0.121.3 | 2026-11-09 | `cd1f23eb50dfb52647090158f453c7359172591f` | eligible; requires syft v1.57.1 |
- **Tags** (`tags/list`, 7,161 tags in all):
  - **Plain tags:** `latest`, `nonroot`, `debug` and `debug-nonroot`, each also with `-amd64`, `-arm`, `-arm64`, `-ppc64le`, `-riscv64` and `-s390x`.
    - The README (last changed in `a3d1f4e2`, 2026-09-24) documents the four tags and the arch suffixes: "Any other tags are considered deprecated and are no longer updated".
  - **Commit tags:** `<tag>-<40-hex commit>` and bare `<40-hex commit>` tags cover 169 commits (116 for riscv64).
    - Most commits re-tag an existing digest. `:nonroot` had 18 distinct indexes between 2025-10-08 and 2026-11-12, most recently on 09-13, 10-20, 10-21, 10-23, 10-31 and 11-12.
  - **cosign tags:** `sha256-<hex>.sig` and `.att`, 472 of each.
  - **Lifecycle tags:** 1,669 `update-available-<digest>` tags.
    - `private/tools/lifecycle/tag.sh` adds one to each digest older than 24 hours whose only tags are commit tags. Its README says 48 hours.
    - `25ac8f0c`, `336e53e9` and `bf6233b6` already carry one.
    - The script's `deprecate` mode adds `deprecated-public-image-<digest>`; `static-debian12` has 1,670 of those.
- **Lifecycle:**
  - Debian 13 `static` is supported until "debian 14 release day + 1yr" (`SUPPORT_POLICY.md`).
  - Debian 12 builds were removed on 2026-09-10 (`3833bee1`). `static-debian12:nonroot` (`sha256:afa5c872`, last uploaded 2026-08-21) is tagged `deprecated-public-image-`.
  - Images are "automatically built and pushed every commit" (`RELEASES.md`). A daily workflow (cron `35 20 * * *`) opens Debian snapshot PRs, after a 1-day buffer by default.
- **Registry:** gcr.io answers anonymous requests and is served by Artifact Registry (header `x-gcr-using-artifact-registry: true`). Google says "gcr.io URLs hosted on Artifact Registry, including Google-owned images with gcr.io URLs, are not affected by the Container Registry shutdown".
- **Image contents** (checked on amd64 `816dc4d5`):
  - **Manifests:**
    - The index lists 6 platforms (amd64, arm64/v8, arm/v7, s390x, ppc64le, riscv64) and has no annotations.
    - Each platform manifest (2864 bytes) has an OCI config and 13 `tar+gzip` layers of about 860 KB in total.
    - Its one annotation is `org.opencontainers.image.source: https://github.com/GoogleContainerTools/distroless`.
  - **Config:**
    - `Env` holds only `PATH` and `SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt`. `User` is `"65532"` and `WorkingDir` is `/home/nonroot`.
    - There is no `Entrypoint`, `Cmd`, `Labels` or `ExposedPorts` (the README's "empty vector entrypoint"), and `created` is the epoch.
  - **Users:**
    - `/etc/passwd` (layer 7) lists root (0), nobody (65534) and nonroot (65532, home `/home/nonroot`). All three have the shell `/sbin/nologin`, which doesn't exist: `/bin` and `/sbin` link to the empty `/usr/bin` and `/usr/sbin`.
    - `/etc/group` lists root, nobody, tty (5), staff (50) and nonroot (65532).
  - **Modes:** `/home/nonroot` is 0700 and owned by 65532:65532, `/root` is 0700, and `/tmp` is 1777. Only `ca-certificates.crt`, its copyright file and `etc/update-motd.d/10-uname` have an executable bit.
  - **Packages:** one `var/lib/dpkg/status.d/<pkg>` file (plus `.md5sums`) each, and no `var/lib/dpkg/status`. Syft v1.57.1 reads them through the glob `**/lib/dpkg/status.d/*`.

    | Package | Version |
    |---|---|
    | base-files | 13.8+deb13u7 |
    | netbase | 6.5 |
    | tzdata | 2026f-0+deb13u1 |
    | tzdata-legacy | 2026f-0+deb13u1 |
    | media-types | 13.0.0 |
    | ca-certificates | 20250419+deb13u1 (148 certificates) |

    - **Checked locally (2026-11-17, Grype 0.121.3):** on Grimoire's image, built on the pinned `25ac8f0c…`, Grype lists these six packages, at these versions, on both platforms, and reports the distro as Debian 13.7.
  - **System files:** `/usr/lib/os-release`, to which `/etc/os-release` links, has `ID="debian"`, `VERSION_ID="13"` and `PRETTY_NAME="Distroless"`. `nsswitch.conf` has `hosts: files dns` and `passwd: compat`.
- **Distroless signatures and SBOMs:**
  - **Verify command:** the README's is unchanged: `cosign verify $IMAGE_NAME --certificate-oidc-issuer https://accounts.google.com --certificate-identity keyless@distroless.iam.gserviceaccount.com`.
    - **Checked locally (2026-11-17, cosign 3.1.4):** the pinned index `25ac8f0c…` and its linux/amd64 and linux/arm64/v8 manifests each verify with that identity and issuer. Grimoire's image job verifies all three before every build.
  - **What is signed:** both indexes and all four platform manifests above have their own `.sig` and `.att`. Across the repository, `.sig` subjects are 398 manifests and 74 indexes.
  - **Format:** the legacy cosign tag scheme.
    - The `.sig` manifest holds one `application/vnd.dev.cosign.simplesigning.v1+json` layer with the annotations `dev.cosignproject.cosign/signature` and `dev.sigstore.cosign/{bundle,certificate,chain}`.
    - The payload's `docker-reference` is `gcr.io/distroless/static-debian13`, and the bundle holds a Rekor SET for a `hashedrekord` entry.
  - **Certificate on `aafe9936`:** SAN `email:keyless@distroless.iam.gserviceaccount.com`, and OID 1.3.6.1.4.1.57264.1.1 is `https://accounts.google.com`. It is valid from 2026-11-12T21:23:06Z, when it was logged as Rekor logIndex 3839886732.
  - **SBOM:**
    - `.att` is a DSSE envelope (`application/vnd.in-toto+json`) holding an in-toto Statement v0.1. Its predicate type is `https://spdx.dev/Document`, and its subject is the index digest.
    - **The predicate is an SPDX 2.3 document encoded as a JSON string, not an object.** It lists 79 packages, Bazel targets included.
    - The amd64 and arm64 `.att` are SPDX too. No SLSA provenance was found.
  - **Signing code:** `sign_and_push.go` runs `cosign verify --offline --trusted-root … --certificate-oidc-issuer https://accounts.google.com --certificate-identity <keyless>` on every pushed digest. Only on exit code 10 or 12 does it run `cosign attest --type spdx`, then `cosign sign`, so an unchanged digest is never signed again.
  - **Referrers:** `GET /v2/distroless/static-debian13/referrers/<index>` returns 200 with `"manifests": null`. distribution-spec v1.1.1 says "If a query results in no matching referrers, an empty manifest list MUST be returned".
- **cosign v3.1.4 against distroless** (read from the source; only the verification above was run):
  - **Bundle fallback:**
    - `verify` defaults to `--new-bundle-format=true`, which is deprecated: "this will be the only supported format in future versions".
    - If `GetBundles` finds no bundle or fails, verify falls back to legacy `.sig` verification. `GetBundles` handles a nil `manifests` list and returns `ErrNoMatchingAttestations`, so gcr.io's `null` takes the fallback.
  - **Trust root:**
    - With `--trusted-root <file>`, cosign loads that file. Otherwise it fetches a live trusted root over TUF.
    - If that fetch fails, legacy verification only warns ("Continuing with individual targets"); new-bundle verification fails.
    - `--offline` and `--experimental-oci11` are deprecated.
  - **Attestations:** `verify-attestation --type spdx` maps to in-toto's SPDX predicate.
- **GHCR signatures** (one public image, `ghcr.io/sigstore/cosign/cosign@sha256:0f06154836d2`; this answers the COSIGN gap):
  - **No referrers API:** it answers 404 `MANIFEST_UNKNOWN`. distribution-spec v1.1.1 says "If the registry supports the referrers API, the registry MUST NOT return a `404 Not Found`".
  - **Tag schema instead:** cosign v3 bundles are stored under the referrers tag schema. Tag `sha256-<hex>` is an index of `application/vnd.dev.sigstore.bundle.v0.3+json` artifacts: DSSE envelopes with predicate type `https://sigstore.dev/cosign/sign/v1`.
  - That image has no `.sig` or `.att` tags.
- **OCI annotations** (image-spec v1.1.1; `annotations.md` on `main` at `4feed4a6674e`, 2026-10-22, is identical):
  - **Keys:** `org.opencontainers.image.{created, authors, url, documentation, source, version, revision, vendor, licenses, ref.name, title, description, base.digest, base.name}`.
    - They are "intended for but not limited to" index, manifest and descriptor authors.
    - `created` is RFC 3339, and `licenses` is an SPDX License Expression.
  - **Base keys:**
    - `base.name` "SHOULD be a fully qualified reference name, without any assumed default registry".
    - Both "SHOULD be the immediate image sharing zero-indexed layers with the image, such as from a Dockerfile `FROM` statement", and "SHOULD NOT reference any other images used to generate the contents of the image (e.g., multi-stage Dockerfile builds)".
  - **Rules:** index `annotations`, manifest `annotations` and config `Labels` "MUST use the annotation rules".
  - **Repository names:** they must match `[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*(\/[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*)*` (distribution-spec v1.1.1). `jabbott-iii/Grimoire`, the value of `github.repository`, doesn't match.
- **BuildKit and buildx:**
  - **Annotation levels:**
    - `manifest` is the default, and each platform manifest gets a copy. The others are `index`, `manifest-descriptor` and `index-descriptor`.
    - An optional `[os/arch]` qualifier narrows a level, as in `--annotation "manifest[linux/amd64]:k=v"`. The output-option form is `annotation-index.k=v`.
    - Asking for a level the exporter doesn't produce fails the build.
  - **Loading:** Docker's docs say "The Docker Engine image store doesn't support loading images with annotations". Buildx falls back to the `docker` exporter "under some circumstances, such as when provenance attestations are explicitly disabled", and `index:` annotations then fail.
  - **Labels:** they live only in the image config, and images built on top inherit them unless overridden. Distroless sets none.
  - **Attestations:** "An attestation is attached as a separate manifest on the image index."
  - **Base annotations:** BuildKit v0.35.0 and buildx v0.38.2 never set `org.opencontainers.image.base.name` or `base.digest`. Outside `vendor/`, their Go source never mentions them. So `cargo xtask release image context` computes both: `base.name` everywhere, and `base.digest` as the base index digest on the index and as each platform's base manifest digest on that platform's manifest.
  - **Platforms:** BuildKit's vendored `containerd/platforms` normalizes the arm64 variant `v8` to empty, so `linux/arm64` matches the base's `arm64/v8` descriptor.
    - **Checked locally (2026-11-17, rootless Docker 29.9.3):** `cargo xtask release image check` found the linux/arm64 image's lower layers equal to the layers of the base's `arm64/v8` manifest.
- **GHCR metadata** (GitHub docs):
  - **Supported keys:**
    - `org.opencontainers.image.source`.
    - `description`, "A text-only description limited to 512 characters".
    - `licenses`, "An SPDX license identifier such as "MIT," limited to 256 characters".
  - **Where they are read:**
    - For single-arch images, from Dockerfile `LABEL`s.
    - For multi-arch images, the description comes from "the `annotations` field in the image's manifest", though the docs' example sets `annotation-index.org.opencontainers.image.description`.
  - **Linking:**
    - Publishing from a workflow with `GITHUB_TOKEN` links the package to that repository.
    - But "the `GITHUB_TOKEN` will not have permission to push the package if you have previously pushed a package to the same namespace, but have not connected the package to the repository."
    - Access permissions are inherited automatically only if the package is linked before its first publish, "such as by adding the `org.opencontainers.image.source` Docker label".
  - **Visibility (the docs conflict):**
    - The access-control page says a first publish is private, and a linked package inherits access permissions "but not the visibility".
    - The Actions publishing page says a package that a workflow creates with `GITHUB_TOKEN` "inherits the visibility and permissions model of the repository".
    - "Once you make a package public, you cannot make it private again." Public GHCR packages "allow anonymous access".
  - **GitHub's example workflow:**
    - It grants `contents: read`, `packages: write`, `attestations: write` and `id-token: write`.
    - It pins the docker actions by SHA but uses `actions/checkout@v6` and `actions/attest@v4` by tag.
    - It sets `IMAGE_NAME: ${{ github.repository }}`.
- **Dependabot for a Dockerfile** (GitHub docs; dependabot-core v0.405.0):
  - **Support:**
    - `docker` gets version updates, and supports private repositories and private registries, but **gets no security updates**.
    - Public registries need no credentials. With none configured, `credentials_for_registry` returns nil.
  - **Discovery:**
    - `directory: "/"` finds a root `Dockerfile`. The fetcher lists only that directory and keeps names matching `/dockerfile|containerfile/i`.
    - YAML files there are read only if they look like Kubernetes resources (top-level `apiVersion` and `kind`) or a Helm chart.
  - **Parsing:**
    - The FROM regex accepts `FROM [--platform=X] [registry/]image[:tag][@sha256:<64 hex>] [AS name]`.
    - The registry must be a dotted domain, and the image name lowercase, so `${ARG}` references don't match.
  - **Digest updates:**
    - A digest pinned on a non-comparable tag such as `nonroot` gets digest-only PRs.
    - Without `--platform`, a new index counts as a no-op only if no platform digest changed.
    - The updater rewrites `@sha256:<old>` in place, and keeps the digest it checked even if the tag moves meanwhile.
  - **Cooldown doesn't delay gcr.io or GHCR bumps.**
    - The docs list Docker as supporting `default-days` only.
    - But since `3bfea916` (2026-09-18), the only accepted publication date is Docker Hub's `tag_last_pushed`. For other registries, the update is allowed with a `cooldown_date_unavailable` warning.
    - Before that commit, a manifest `Last-Modified` header was used. This predates the DEPENDABOT check of 2026-11-16, which missed it.
    - So the maintainer checks a new digest's age before merging. GCR's `tags/list` carries the upload time, for example `curl -s https://gcr.io/v2/distroless/static-debian13/tags/list | jq '.manifest["sha256:<digest>"].timeUploadedMs'`. The field is undocumented (see UNVERIFIED).
  - **Workflow files:**
    - The `github-actions` parser reads only `uses:` and skips `docker://`: "Docker Hub and GitHub Container Registry URLs are currently not supported".
    - Job `container:` and `services:` images are never updated.
  - **PR metadata:**
    - It comes only from config `Labels` (`org.opencontainers.image.source`, `version` and `revision`), and GitHub also requires the source repository to carry the same tags as the image.
    - Distroless sets no Labels, so its bump PRs carry no release notes.
- **Chainguard (comparison only):**
  - **Tags:** all 17,500 tags of the free `cgr.dev/chainguard/static` were listed. The only plain tags are `latest`, `latest-glibc`, `latest-glibc-tzdata` and `latest-tzdata`, which corrects BASE-IMAGE's "`latest`/`latest-dev`".
  - **Current `latest`:** `sha256:fc42af33e643f20d08baf03cddcc1c02ba57052dfb02edb0d79f3fc47bcea8d9` on 2026-11-17.
  - **Signing:** public images are signed by `https://github.com/chainguard-images/images/.github/workflows/release.yaml@refs/heads/main`, with issuer `https://token.actions.githubusercontent.com`.
- **UNVERIFIED:**
  - What GCR's `timeUploadedMs` means. It is undocumented, and `timeCreatedMs` is 0. The ages above were cross-checked against Rekor times.
  - Which dependabot-core version GitHub's hosted service runs.
  - Whether GHCR shows `source` and `licenses` from index annotations, and whether it accepts an SPDX expression for `licenses`.
  - The first-publish visibility of a package a workflow creates (GitHub's docs conflict).
  - Whether distroless publishes SLSA provenance anywhere else.
  - Whether GHCR plans referrers API support (REGISTRY-P6 found none in four repositories), and how its UI shows `sha256-*` tags.
  - How quickly distroless ships Debian security fixes.
  - Whether `cosign verify` prints the Rekor integratedTime of a legacy signature in a machine-readable form.
  - Whether the `update-available` tagging runs on a schedule. The script is manual and defaults to a dry run.
  - Whether distroless builds are reproducible. No document read says so; only re-tagging was observed.
- **Settled after the research:** `HOME` and user lookup for a `--user` uid that isn't in `/etc/passwd`, from Docker's docs and runc v1.5.3 source in RUN-P6 (a numeric uid needs no entry, and `HOME` becomes `/`); not run.

### BUILD-P6: BuildKit and buildx for the Phase 6 image
Sources: https://github.com/moby/buildkit at v0.34.2 (`docs/{multi-platform,build-repro,annotations,buildkitd.toml}.md`, `docs/dev/dockerfile-llb.md`, `docs/attestations/{sbom,sbom-protocol,slsa-provenance,slsa-definitions,attestation-storage}.md`, `frontend/attestations/{parse.go,sbom/sbom.go}`, `frontend/dockerui/config.go`, `frontend/dockerfile/{builder/build.go,docs/reference.md,linter/ruleset.go}`, `solver/llbsolver/{provenance.go,solver.go,compat/compat.go}`, `client/solve.go`, `exporter/oci/export.go`, `exporter/containerimage/{export.go,writer.go,opts.go,exptypes/keys.go}`), its releases v0.28.0, v0.31.0, v0.32.0, v0.33.1, v0.34.1 and v0.34.2, and its security advisories; https://github.com/docker/buildx at v0.38.2 (`build/{opt,git}.go`, `build/resolver/driver.go`, `driver/{docker,docker-container,bkimage}/`, `util/ghutil/ghutil.go`, `util/imagetools/{location,create}.go`, `util/ocilayout/parse.go`, `commands/history/inspect.go`, `policy/{default.go,default.rego,builtins.rego,validate.go,signatures.go,types.go}`, `docs/reference/buildx_{build,create,imagetools_create,history_export}.md`), `releases/download/v0.38.2/checksums.txt`, and releases v0.33.0, v0.36.1 and v0.38.2; https://github.com/docker/buildkit-syft-scanner at v1.12.1 (`go.mod`, `internal/{target,scanner}.go`); https://github.com/anchore/syft at v1.51.0 (`internal/task/package_tasks.go`, `syft/pkg/cataloger/{rust,debian,sbom}/cataloger.go`, `syft/format/common/spdxhelpers/to_format_model.go`, `syft/format/internal/spdxutil/{versions.go,helpers/document_namespace.go}`); https://github.com/docker/docs at `947343021a3e` (`content/manuals/build/{checks.md,exporters/oci-docker.md,exporters/image-registry.md,metadata/attestations/{_index,sbom,slsa-provenance}.md,builders/drivers/{docker,docker-container}.md,ci/github-actions/github-builder-architecture/index.md}`, `content/manuals/engine/storage/containerd.md`, `content/manuals/docker-hub/usage/{_index,pulls}.md`); https://github.com/docker/cli at v28.0.4 (`cli-plugins/manager/manager{,_unix}.go`, `docs/reference/commandline/login.md`); https://github.com/moby/moby/blob/v28.0.4/vendor.mod; https://github.com/actions/runner-images at `fb099341ddaf` (`images/ubuntu/Ubuntu2404-Readme.md`, `toolsets/toolset-2404.json`, `scripts/build/install-docker.sh`) and `6573a3f3872c` (`Ubuntu2404-Arm64-Readme.md`), release `ubuntu24/20261116.376`, and issues 14105 and 14554; docker/setup-buildx-action at `f87e5991a6d7`, docker/build-push-action at `c3c9e263c25d`, docker/metadata-action at `dc8028041006`, docker/login-action at `8aa5831fb8d4` and docker/setup-qemu-action at `990126619549`, each with `action.yml` and `src/`; https://github.com/docker/actions-toolkit at v0.100.0 (`src/buildx/{install,history,build}.ts`, `src/github/summary.ts`); https://github.com/docker/github-builder/blob/58cb9f5b71b1/README.md; https://github.com/zizmorcore/zizmor at v1.32.0 (`crates/zizmor/src/audit/{cache_poisoning,superfluous_actions}.rs`); https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md; the anonymous OCI distribution API on `registry-1.docker.io` (`moby/buildkit`, `docker/buildkit-syft-scanner`, `docker/dockerfile`, `tonistiigi/binfmt`, `ratelimitpreview/test`) and `ghcr.io` (`github/github-mcp-server`); `gh api` release, tag, commit, advisory and licence metadata. Status: Verified, except the gaps listed. The research ran no build, and downloaded or ran no binary. The Sigstore bundles were decoded with `openssl x509`, not verified with cosign.

- **Pins** (all released on or before 2026-11-10; tags resolved with `gh api`, digests with anonymous registry requests on 2026-11-17):
  - **BuildKit image** `moby/buildkit:v0.34.2` (2026-11-03; commit `8c579433824224dc64a158cf8734495b6e2b186a`, tag object `a1b68ebae87cfab67871e25c490042b1b0722672`):
    - index `sha256:c5f8862f52499773af85bc07695b2379ef08e8002352b0df2c4864c91d78dabe`;
    - linux/amd64 `sha256:97719ccb2ac86fb44fd36f330a6d302f63dddd5326f3ada24f9c05ccac0d9098`;
    - linux/arm64 `sha256:92f93f25966c544549b88144406ce8cd9dcdf5df128250711031a4acd9149a8e`.
    - The index also has arm/v7, ppc64le, riscv64 and s390x, each with an attestation manifest, and is annotated `Apache-2.0` with that revision. `buildx-stable-1` points to the same index today; `latest` points to v0.35.0. The rootless variant `v0.34.2-rootless` has index `sha256:c8252268760e1baba80780d63d738ef9f260fb7c7c0973c78494cb9ddd99eda1`.
  - **SBOM generator** `docker/buildkit-syft-scanner:1.12.1` (2026-10-26, immutable release, commit `422dfad83311be8c7e03e219ab6e8f70f4619575`):
    - index `sha256:873a7458c3c8ac8e32c7b796c10e1600b07b963d2aa000a72a2d97aaaa2d6315`;
    - linux/amd64 `sha256:10a413da6b52acacf4af4262eafeae6ca05076fbd65850ad7f233163c6d6a81c`;
    - linux/arm64 `sha256:eecdb89c2cd84b80868ea748063edb7225576994dbeac439c6b831092eeb4bff`.
    - `stable-1` and `latest` point to it. Image tags have no `v`: `v1.12.1` returns 404.
  - **buildx** v0.38.2 (2026-11-03; commit `3c4cbd35852fc1a476923627a780a7836c28b654`, tag object `8a2644f276c5dce115c4724bc23ad665416acb9f`):
    - `buildx-v0.38.2.linux-amd64` `7992ec09413b80ff57ba4e081f56fc833d7bac1d5e71143c6c809e7e6065f6d6`;
    - `buildx-v0.38.2.linux-arm64` `46c0f334772de3cb0b341cc415caf7339fb5cfc8b8438ebcb5a10e7f23800d36`.
    - Both come from `checksums.txt` and match the API asset digests. Each binary also has `.sigstore.json`, `.provenance.json` and `.sbom.json` assets. The release isn't immutable.
  - **Dockerfile frontend** `docker/dockerfile:1.28.0` (2026-10-07, commit `f5b59a1f2fb25a0cb7072c398cab9aed7835da4e`): index `sha256:bb22d9815c728170f72750f4e5b0d672e06176142e1d602c7e66c050100b7e5b`. It is used only with a `# syntax=` line, which Grimoire's `Dockerfile` doesn't have.
  - **Docker's actions:** all are node24, immutable releases, lightweight tags on verified commits, and Apache-2.0. Grimoire uses none of them.

    | Action | Version | Commit | Released |
    |---|---|---|---|
    | docker/setup-buildx-action | v4.4.1 | `f87e5991a6d7451dcb8d9637bfbc97413f497069` | 2026-09-16 |
    | docker/build-push-action | v7.4.0 | `c3c9e263c25d99ce0380d002d59b67737d91b0dc` | 2026-09-15 |
    | docker/metadata-action | v6.2.0 | `dc802804100637a589fabce1cb79ff13a1411302` | 2026-07-02 |
    | docker/login-action | v4.8.0 | `8aa5831fb8d48d8f76594c7ddde61ae58019f550` | 2026-11-05 |
    | docker/setup-qemu-action | v4.4.0 | `99012661954931238ded8c8b007157a8430204e1` | 2026-09-15 |

    setup-buildx-action, build-push-action and setup-qemu-action bundle `@docker/actions-toolkit` 0.100.0 (commit `e0f279f2d19f`).
  - **Too young**, all published 2026-11-11 and eligible from 2026-11-18: BuildKit v0.35.0 (commit `f55949dea4f03bb9d85b73e9afc9a609291289db`, index `sha256:aa632b921ef07411d2cb77292cd34213d4f7b384001d6fceef1e1f25bf9752bb`), buildx v0.39.0, and dockerfile 1.29.0 (index `sha256:572cebbc2b15e6f84c077742952674889cb63cd5e9b80abdffcbe587b6f19183`).
- **Runners** (changed since RUNNERS):
  - `ubuntu-24.04` (20261111.370.1) and `ubuntu-24.04-arm` (20261111.182.1) ship Docker Client and Server 28.0.4, Docker-Buildx 0.38.2 and Compose 2.38.2.
  - The toolset pins `docker-ce` 28.0.4, but installs `containerd.io` and the buildx plugin at `latest`. `install-docker.sh` downloads buildx into `/usr/libexec/docker/cli-plugins/docker-buildx` with no checksum check, and writes no `daemon.json`.
  - Docker Engine enables the containerd image store by default only from 29.0, and only on fresh installs. Moby v28.0.4 (2025-03-25) vendors BuildKit v0.20.2.
  - The pre-release image `ubuntu24/20261116.376` (published 2026-11-17) moves Docker-Buildx to 0.39.0. Its deployment started at 22:21Z on 2026-11-17 and usually takes 2 to 3 days, so jobs may get either image meanwhile (RUNNERS).
  - Issue #14105 ("Docker on Ubuntu 24.04 requires update to v29") is open. #14554 says every image's Docker is below 29.7.2, which fixes "CopyEscape" (CVE-2026-17106).
- **Drivers:**
  - `docker`: BuildKit is the Engine's own, and its version "can't be configured". The driver reports multi-platform, OCI export, direct push and attestations only when a worker carries the containerd snapshotter label (`driver/docker/driver.go`). Otherwise buildx stops with "Attestation is not supported for the docker driver".
  - `docker-container`: BuildKit runs in its own container, so attestations work with any image store. `--load` back into the daemon has the image store's limits.
  - Attestations need a driver with the MultiPlatform feature (`build/opt.go`).
- **Multi-platform without QEMU:**
  - BuildKit uses QEMU only "when your build needs to run a binary" of a foreign architecture. COPY is an LLB file operation, not an exec.
  - The SBOM scanner runs on the build platform, `workers[0].Platforms[0]` (`frontend/dockerui/config.go`, `frontend/dockerfile/builder/build.go`), not on each target platform.
  - buildx sends any platform that no node matches to node 0 (`build/resolver/driver.go`), so a single-node builder gets every platform.
  - **Checked locally (2026-11-17, rootless Docker 29.9.3, on x86_64 with no emulation):** one docker-container builder built and exported linux/amd64 and linux/arm64 for Grimoire's COPY-only `Dockerfile`, and `cargo xtask release image check` passed on the layout. The arm64 image wasn't run.
- **The builder image:**
  - The default is `moby/buildkit:buildx-stable-1` (`driver/bkimage/bkimage.go`). `--driver-opt image=<ref>` replaces it and is passed straight to `ImagePull`.
  - The builder container is always `Privileged: true`.
  - `provenance-add-gha` defaults to `true` (see Provenance). `allow-untrusted-image=true` turns off builder-image verification.
- **Signatures on Docker's images:**
  - moby/buildkit (from v0.27.0, per `policy/default.rego`), docker/dockerfile and docker/buildkit-syft-scanner are keyless-signed by docker/github-builder's reusable `bake.yml`.
  - The signature is a Sigstore bundle v0.3 holding a DSSE envelope (predicate `https://sigstore.dev/cosign/sign/v1`), a Rekor entry and an RFC 3161 timestamp.
  - The bundle is an OCI referrer of each platform's attestation manifest; the DSSE subject is that manifest's digest. The index has no referrers.
  - BuildKit v0.34.2 amd64 (attestation manifest `sha256:f21a1372a4239ed443f1e76279ab3e6bd3a95920732bec7a47a2300aa0aacee0`):
    - SAN `https://github.com/docker/github-builder/.github/workflows/bake.yml@58cb9f5b71b1836d6f690c1e95effdeb9b98cb8a`;
    - issuer `https://token.actions.githubusercontent.com`;
    - OID 1.12 `https://github.com/moby/buildkit`, OID 1.14 `refs/tags/v0.34.2`, OID 1.18 `https://github.com/moby/buildkit/.github/workflows/buildkit.yml@refs/tags/v0.34.2`.
  - Scanner 1.12.1 amd64 (attestation manifest `sha256:8633b39ac9e4ef577ef4f500ad424d7119885dbdff802427e118e29edafff1e0`):
    - SAN `…/docker/github-builder/.github/workflows/bake.yml@a492c6d04fd3315f67230809b44d60cc0acd50b3`;
    - OID 1.12 `https://github.com/docker/buildkit-syft-scanner`, OID 1.14 `refs/tags/v1.12.1`.
- **buildx's default policy:**
  - It is opt-in through `BUILDX_DEFAULT_POLICY` ("a future release may flip the default"). It covers both build sources and the docker-container builder image.
  - The builder-image check is skipped on the classic image store: "Classic-store images are allowed without it" (`driver/docker-container/driver.go`).
  - **moby/buildkit:** a signature is required for its floating tags (`latest`, `buildx-stable-N`, …) and for release tags of v0.27.0 and later.
  - **docker/dockerfile:** a signature is required for `latest`, `labs` and `master`, and for version tags of 1.21 and later.
  - **docker/buildkit-syft-scanner:** a signature is required only for `latest` and for version tags of 1.10 and later. `stable-1`, the default generator tag, isn't covered. The policy strips only `refs/tags/` from the signature's source ref before comparing it with the image tag.
  - The rules match on `input.image.tag`. `policy/validate.go` sets it for any tagged reference, `name:tag@digest` included. The policy comment says "Access by digest is always allowed".
  - The Rego helper checks the signature kind and type, the Sigstore intermediate CA, the issuer, the source repository URI, `github-hosted` runners and at least one timestamp. It doesn't compare the SAN.
- **Security floors:**
  - BuildKit v0.34.2 fixes GHSA-2fqp-xg6f-v7gj (a Git resolver panic), and v0.34.1 fixes GHSA-w26x-xg27-7qjc (xattrs set outside the state directory).
  - v0.33.1 (2026-09-30) fixed 10 advisories. Two are high: GHSA-f2v9-hprr-32q3 (cache poisoning via unvalidated layer DiffIDs) and GHSA-r456-g3gm-cvxf (a `--cdi-disabled` panic).
  - buildx v0.38.2 fixes two Bake-only entitlement bypasses (GHSA-xg6v-cv2h-2w5h, GHSA-rghq-x9hf-hc3r) and vendors BuildKit v0.34.2.
- **Provenance:**
  - **Format:** the default is SLSA v1 since BuildKit v0.28.0 (`defaultSLSAVersion`). Real v0.34.2 output uses predicate `https://slsa.dev/provenance/v1` in an in-toto v1 Statement. Docker's docs at `947343021a3e` still call `version=v0.2` the default, which is stale.
  - **Default mode:** without `--provenance`, buildx adds `mode=min,inline-only=true` (opt out with `BUILDX_NO_DEFAULT_ATTESTATIONS`). BuildKit's own default mode is max.
  - **max mode** adds:
    - the Dockerfile source with sourcemaps;
    - `buildConfig.llbDefinition`;
    - every `build-arg:*` value;
    - secret and SSH IDs. Secret values are never included.
  - **min mode** strips the `build-arg:*` and `label:*` values, plus the secret and SSH IDs, and marks the request incomplete.
  - **In both modes:**
    - `startedOn`, `finishedOn` and a unique `invocationId`;
    - `builder.id` from the `builder-id` attribute;
    - `request.compatibilityVersion`;
    - `dockerfileVersion` when the frontend is `dockerfile.v0`.
  - **Completeness:** a local context leaves `resolvedDependencies` incomplete. The undocumented attribute `complete-materials=true` then fails the build (`provenance.go`). A Dockerfile without `RUN` counts as hermetic.
  - **vcs hints:** buildx sends `vcs:source` (the git remote URL) and `vcs:revision` when the context is inside a git work tree. `BUILDX_GIT_INFO=false` turns this off, and `BUILDX_GIT_CHECK_DIRTY=1` adds `-dirty`. It also sends `vcs:localdir:<name>` relative to the repository root. BuildKit doesn't verify any of these.
- **The GitHub Actions context in provenance:**
  - With `provenance-add-gha=true` (the docker-container default), buildx writes `provenance.d/github_actions_context.json` into the builder when it is created.
  - That file holds the whole `GITHUB_EVENT_PATH` payload; the actor, actor ID and triggering actor; the ref, ref type and `ref_protected`; the repository and owner names and IDs; the run ID, number and attempt; the job; and the workflow ref and SHA. It also holds the runner's name, tracking ID, OS, architecture and environment. It holds no tokens.
  - BuildKit copies it into `buildDefinition.internalParameters` outside the mode switch, so min and max mode both carry it.
  - moby/buildkit's own v0.34.2 amd64 provenance (blob `sha256:058c481ab730eafaa2c53ff712e716904525cc7d5251ea80185e2b760c57cf7a`) has a `github_event_payload` with `after`, `before`, `commits`, `head_commit`, `pusher`, `sender`, `repository` and more.
  - BuildKit's `slsa-provenance.md` still says min mode "does not leak information from any part of the build environment".
  - **Checked locally (2026-11-17, rootless Docker 29.9.3, with the Actions environment simulated):** the provenance held the event payload with the default, and didn't with `--driver-opt provenance-add-gha=false`. `cargo xtask release image check` refuses a provenance that holds `github_event_payload`.
- **SBOM:**
  - **Generator:** `--sbom=true` or `--attest type=sbom` uses `docker/buildkit-syft-scanner:stable-1`, resolved by tag at build time. `generator=<ref>` replaces it.
  - **Scanner environment:** the scanner gets its image's env, `BUILDKIT_SCAN_DESTINATION`, `BUILDKIT_SCAN_SOURCE[_EXTRAS]`, and `BUILDKIT_SCAN_<K>=<V>` for every other attest parameter. Scanner 1.12.1 reads `BUILDKIT_SCAN_SELECT_CATALOGERS`.
  - **syft version:** scanner 1.12.1 bundles syft v1.51.0 (2026-08-10), not the standalone syft pins in SBOM. Its release only bumps brotli (CVE-2026-111497). Real moby/buildkit SBOMs list `Tool: syft-v1.51.0`.
  - **Catalogers:** syft's `image` and `file` tags, plus `sbom-cataloger`.
    - `cargo-auditable-binary-cataloger` parses executables by MIME type.
    - The dpkg cataloger globs include `**/lib/dpkg/status.d/*`.
    - `sbom-cataloger` reads `*.syft.json`, `*.cdx[.*]` and `*.spdx[.*]` files found in the image.
  - **Output:** SPDX 2.3 JSON in an in-toto v1 Statement with predicate `https://spdx.dev/Document`. `created` is `time.Now()` and `documentNamespace` uses a random UUID.
  - **Scope:** only the final stage is scanned. `BUILDKIT_SBOM_SCAN_CONTEXT` and `BUILDKIT_SBOM_SCAN_STAGE` act only when the Dockerfile declares them with `ARG`.
  - **Size:** each attestation file is capped at 80 MiB (BuildKit `sbom.md`, `attestation-storage.md`).
- **Attestation storage** (changed since SBOM):
  - Since BuildKit v0.32.0, attestations are OCI artifacts by default when the compatibility version is 30 or more and OCI media types are on (`DefaultOCIArtifact`).
  - The exporter key is `oci-artifact`; the release note's `oci-artifacts` is a typo. `oci-artifact=false` restores the legacy form, and buildx v0.36.1 added `BUILDX_NO_DEFAULT_OCI_ARTIFACT` for the same thing.
  - An attestation manifest has artifactType `application/vnd.docker.attestation.manifest.v1+json`, config `application/vnd.oci.empty.v1+json`, and `application/vnd.in-toto+json` layers, each annotated with `in-toto.io/predicate-type`. Its `subject` is its platform manifest.
  - In the index, its descriptor has platform `unknown/unknown` and the annotations `vnd.docker.reference.type=attestation-manifest` and `vnd.docker.reference.digest`.
  - Docker's image-registry exporter page still lists `false` as the default, which is stale.
  - **Checked locally (2026-11-17, rootless Docker 29.9.3):** in Grimoire's layout each platform has one attestation manifest, whose `subject` is that platform's manifest, holding an SPDX document and SLSA v1 provenance (`cargo xtask release image check`).
- **GHCR** (answers COSIGN's open item):
  - GHCR stores BuildKit's attestation manifests with their `subject`, as `ghcr.io/github/github-mcp-server:latest` (index `sha256:040c4e99810076035eb1d9beb4b4a96fcf589c43cefed6ff51a46a553eb4e174`) shows.
  - Its referrers endpoint returned 404 `MANIFEST_UNKNOWN` for that index, a platform manifest and an attestation manifest. Under distribution-spec v1.1.1 a registry that supports the referrers API "MUST NOT return a 404", and the client "MUST fallback" to the referrers tag schema. So GHCR has no referrers API as of 2026-11-17.
- **OCI exporter and annotations:**
  - `type=oci` uses the same `ImageWriter.Commit` as the image exporter, with every platform.
  - `tar` defaults to `true`; `tar=false` writes the content into a layout directory. `oci-mediatypes` defaults to `true`, and `annotation.*` and `rewrite-timestamp` are accepted.
  - When the outer descriptor in the layout's `index.json` lacks `org.opencontainers.image.created`, the exporter sets it to the epoch, or to the current time if there is none.
  - `--annotation "[type[,type][<os/arch>]:]key=value"` targets `manifest` (the default), `manifest-descriptor`, `index` or `index-descriptor`. Platform wildcards aren't supported.
  - `index-descriptor` writes only "the OCI layout's descriptor for the index".
- **Pushing a layout** (corrected):
  - buildx v0.33.0 added `oci-layout://<path>` as a source and destination for `imagetools create` and `inspect`, mixed with registry references (`util/imagetools/location.go` at v0.38.2).
  - The reference page's line that sources "must already exist in the registry" is stale.
  - A single index source is carbon-copied. `imagetools create` can annotate only `index:` and `manifest-descriptor:`, not manifests or OCI layouts.
  - Grimoire pushes the layout with crane instead, which keeps the index digest (checked locally; REGISTRY-P6).
- **Reproducibility:**
  - buildx (0.10 and later) passes the client's `SOURCE_DATE_EPOCH` as a build argument. It sets the config `created`, the history `created`, the index `org.opencontainers.image.created`, and `local`/`tar` file times.
  - `SOURCE_DATE_EPOCH=context` is ignored for local contexts.
  - `rewrite-timestamp=true` (BuildKit 0.13 and later) rewrites file times in layers. It leaves layers whose DiffID matches the base image untouched.
  - A missing epoch, or a layer that fails to rewrite, only logs a buildkitd warning (`writer.go`), and the build still succeeds.
  - **compatibility-version:**
    - Values are `10`, `20` and `30` (current). It "pins digest-affecting image assembly behavior" for the `image` and `oci` exporters.
    - The v0.31.0 release note raised it to 30, with OCI media types for every image export. `build-repro.md` says 20 covers "v0.15.0 through v0.31.x", which conflicts.
    - It is a client `SolveOpt.CompatibilityVersion`, where 0 means current. No buildx v0.38.2 option or doc sets it.
  - **Checked locally (2026-11-17, rootless Docker 29.9.3; three builds, one on a fresh builder):** the platform manifest digests were identical in every build. The index digest differed, because the SBOM and provenance attestations hold build times and IDs.
- **Build checks and the frontend:**
  - Checks need Dockerfile 1.8 or later and buildx 0.15.0 or later, and run in the builder. `--check` is `--call=check` and exits non-zero on any violation.
  - A normal build only warns, unless `# check=error=true` or `--build-arg BUILDKIT_DOCKERFILE_CHECK=error=true` is set. The reference recommends pinning the syntax with `error=true`.
  - BuildKit v0.34.2 has 22 rules. Only `InvalidDefinitionDescription` is experimental:
    - StageNameCasing, FromAsCasing, NoEmptyContinuation, ConsistentInstructionCasing, DuplicateStageName and ReservedStageName;
    - JSONArgsRecommended, MaintainerDeprecated, UndefinedArgInFrom, WorkdirRelativePath and UndefinedVar;
    - MultipleInstructionsDisallowed, LegacyKeyValueFormat, InvalidBaseImagePlatform, RedundantTargetPlatform and SecretsUsedInArgOrEnv;
    - InvalidDefaultArgInFrom, FromPlatformFlagConstDisallowed, CopyIgnoredFile, InvalidDefinitionDescription, ExposeProtoCasing and ExposeInvalidFormat.
  - Without `# syntax=`, BuildKit uses its bundled frontend. With one, the frontend image is pulled by its tag at build time. `BUILDKIT_SYNTAX=dockerfile.v0` ignores the directive.
  - BuildKit v0.34.x bundles Dockerfile frontend v1.28.0: the v0.34.0 release notes say "Built-in Dockerfile frontend has been updated to v1.28.0", and the v0.34.1 and v0.34.2 notes don't change it (https://github.com/moby/buildkit/releases/tag/v0.34.0, checked in the Phase 6 review).
  - **Checked locally (2026-11-17, BuildKit v0.34.2 at the pinned digest, no `# syntax=` line):** with `# check=error=true`, a `FromAsCasing` violation fails the build (exit 1); without the directive the build only warns (exit 0); `--check` fails (exit 1) either way.
  - On 2026-11-17 `docker/dockerfile:1` resolves to **1.27.1** (index `sha256:4edf897a3ffa55b89f906fc8cc78afdb3f1834cc9c7083565e611a8a7d5fe99e`, created 2026-09-30), not to 1.28.0 or 1.29.0.
- **Metadata file:** `--metadata-file` writes `containerimage.digest` and `containerimage.descriptor`. Its `buildx.build.provenance` is min unless `BUILDX_METADATA_PROVENANCE` is `max` or `disabled`.
- **CLI plugins:** Docker CLI 28.0.4 searches `CLIPluginsExtraDirs`, then `$DOCKER_CONFIG/cli-plugins`, then `/usr/local/lib`, `/usr/local/libexec`, `/usr/lib` and `/usr/libexec` (each `…/docker/cli-plugins`). The first match wins.
- **Docker's actions:**
  - **setup-buildx-action:**
    - The default driver is `docker-container`. It uses the runner's buildx unless `version` is set or buildx is missing.
    - It downloads buildx without `verifySignature` and caches it in the Actions cache (`cache-binary: true` by default).
    - It pulls `moby/buildkit:buildx-stable-1` unless `driver-opts` has `image=`.
    - It adds `--allow-insecure-entitlement security.insecure --allow-insecure-entitlement network.host` unless `buildkitd-flags` is set.
  - **build-push-action:**
    - It runs only `docker buildx build`.
    - With no `provenance` input, it adds `mode=max` provenance on public repositories (`mode=min,inline-only=true` on private ones), with `builder-id=<run URL>`. `provenance: true` also becomes `builder-id=<run URL>`.
    - It passes `github.token` as a `GIT_AUTH_TOKEN` secret only when the context starts with the default git context.
    - By default it writes a job summary of every input except `github-token` (build args included, secrets as keys only). It also uploads a `.dockerbuild` record (`buildx history export`, buildx 0.23 or later) as an artifact; that record holds the logs and every `build-arg:*` value. `DOCKER_BUILD_SUMMARY` and `DOCKER_BUILD_RECORD_UPLOAD` turn these off.
  - **metadata-action:** it reads repository data from the GitHub API and stamps `org.opencontainers.image.created` with `new Date()`. It has no SOURCE_DATE_EPOCH support.
  - **login-action:** it runs `docker login --password-stdin` and logs out in a post step (`logout: true` by default). Without a credential store, Docker keeps credentials base64-encoded in `config.json`.
  - **setup-qemu-action:** it runs `docker.io/tonistiigi/binfmt:latest` with `--privileged` and caches the image by default. buildx's own `QemuImage` is the same tag, marked "TODO: make this verified".
- **zizmor 1.32.0:**
  - `cache-poisoning` treats setup-buildx-action as cache-aware when `cache-binary` is on and `version` is set, and build-push-action with `push: true` as a publisher.
  - No `docker/*` action is in `superfluous-actions`.
- **Docker Hub limits:**
  - The docs give 100 anonymous pulls per IPv4 address or IPv6 /64 per 6 hours, and say a missing `ratelimit` header means the image or IP is unlimited.
  - On 2026-11-17, from this machine, moby/buildkit, docker/buildkit-syft-scanner and docker/dockerfile returned no ratelimit headers. tonistiigi/binfmt and ratelimitpreview/test returned `ratelimit-limit: 100;w=3600`, a 1-hour window.
- **Docker GitHub Builder** (`docker/github-builder` at `58cb9f5b71b1`): reusable `build.yml` and `bake.yml` workflows that build linux/amd64 on `ubuntu-24.04` and linux/arm64 on `ubuntu-24.04-arm`, sign keylessly, and use signed GitHub Actions cache entries. Reusable workflows aren't covered by `actions.lock` (GHA-SEC).
- **Licences:** moby/buildkit, docker/buildx, docker/buildkit-syft-scanner, anchore/syft and the five Docker actions are all Apache-2.0.
- **Changes to other entries:**
  - **SBOM:** the BuildKit scanner's syft is v1.51.0, the output is SPDX 2.3, and the generator is pulled by the `stable-1` tag. The provenance default is SLSA v1, attestations are OCI artifacts, and the docker-container driver adds the GHA context.
  - **RUNNERS:** Docker 28.0.4 on the classic store, and buildx 0.38.2 moving to 0.39.0.
  - **COSIGN:** GHCR has no referrers API.
- **UNVERIFIED:**
  - the containerd version on the Ubuntu 24.04 runners, and that nothing else on them enables the containerd image store (`docker info` wasn't run on a runner);
  - whether `imagetools create` from `oci-layout://`, regctl or oras copy a layout with the index and attestation-manifest bytes unchanged, and whether any tool does so against GHCR itself;
  - whether the default policy actually rejects `docker/buildkit-syft-scanner:1.12.1` over the `v` prefix, and how the `docker-github-builder` signature kind is assigned (not run, module not read);
  - the exact `cosign verify` command and identity flags for the BuildKit and scanner signatures, which sit on attestation manifests (cosign wasn't run against them);
  - CVE-2026-17106 ("CopyEscape") itself: it is cited only by runner-images #14554 and isn't in moby/moby's or docker/cli's advisories;
  - whether the `.dockerbuild` record also carries the GHA event payload, and whether max-mode `llbDefinition` reveals host paths of the local context;
  - Docker Hub rate limits as seen from runner IPs, where the 1-hour window conflicts with the docs' 6 hours;
  - whether the rootless BuildKit image works with the docker-container driver on the runners;
  - that Docker Engine 28.0.4 pulls `name:tag@sha256:…` strictly by digest.
- **Settled after the research:** that one amd64 BuildKit node exports linux/arm64 for a COPY-only Dockerfile (checked locally, under Multi-platform without QEMU); that crane copies a layout with the index digest unchanged, to a local registry (checked locally, REGISTRY-P6).

### REGISTRY-P6: GHCR, cosign image signatures, image attestations and OCI push tools
Sources: https://github.com/github/docs at `49a012fa50b4` (`content/packages/**`, `data/reusables/package_registry/**`, `data/variables/package_registry.yml`, `src/rest/data/fpt-2026-03-10/packages.json`, `content/actions/reference/workflows-and-actions/workflow-syntax.md`, `data/reusables/actions/jobs/dockerhub-ratelimit-ghr.md`); https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md; https://github.com/sigstore/cosign at v3.1.4 (`cmd/cosign/cli/{sign.go,sign/sign.go,signcommon/common.go,verify/verify.go,verify/common.go,load.go,options/}`, `pkg/oci/{remote,mutate,layout}/`, `pkg/cosign/{verify.go,bundle/sign.go}`, `doc/`); https://github.com/google/go-containerregistry at v0.22.1 (`pkg/v1/remote/`, `pkg/v1/remote/transport/ping.go`, `pkg/name/`, `pkg/registry/`, `cmd/crane/cmd/{push,serve,auth}.go`) and its release assets; https://github.com/actions/attest at v4.2.2 (`action.yml`, `README.md`, `src/`, `package-lock.json`); https://github.com/sigstore/sigstore-js at `@sigstore/oci@0.7.2` (`packages/oci/src/`); https://github.com/cli/cli at v2.103.3 (`pkg/cmd/attestation/`); https://github.com/regclient/regclient at v0.11.7; https://github.com/oras-project/oras at v1.3.6; https://github.com/containers/skopeo at v1.25.1 (`install.md`, `docs/skopeo-copy.1.md`); https://github.com/distribution/distribution at v3.1.2 (`Dockerfile`, `cmd/registry/config-dev.yml`, `registry/api/v2/routes.go`) and its release assets; https://github.com/project-zot/zot/blob/v2.1.23/pkg/api/routes.go; https://github.com/moby/buildkit at v0.34.2 (`exporter/oci/export.go`, `go.mod`); https://github.com/containerd/containerd/blob/v2.4.2/core/images/archive/exporter.go; https://github.com/actions/runner/blob/v2.338.2/src/Runner.Worker/Container/ContainerInfo.cs; https://github.com/zizmorcore/zizmor/blob/v1.32.0/crates/zizmor/src/audit/unpinned_images.rs; https://github.com/actions/runner-images/blob/d9c48363ed37/images/ubuntu/Ubuntu2404-Readme.md; https://github.com/docker/login-action/blob/8aa5831fb8d4/action.yml; GitHub REST release, tag and attestation metadata; anonymous registry API reads of ghcr.io (`sigstore/cosign/cosign`, `actions/actions-runner`, `github/github-mcp-server`, `distribution/distribution`, `jabbott-iii/grimoire`), registry-1.docker.io and hub.docker.com on 2026-11-17 at about 23:45 UTC. Status: Verified, except the gaps listed. The research pushed, installed and ran nothing. Certificates and bundles were only decoded.

- **Pins** (each released on or before 2026-11-10 unless marked):
  - `sigstore/cosign` v3.1.4 (2026-10-20), commit `0e48502cf5ef34e2e60bbd3617aa19945712f705`. `cosign_checksums.txt`: `cosign-linux-amd64` `424e2c97d0b45fd7a6b608a87f090f35633f4c23ece5f092d48bba11aaf4bbfa`, `cosign-linux-arm64` `ade83b7aac62e131cacfe96d958948b2150cc9efec46a570a2649b8e27dedad2`. `sigstore/cosign-installer` v4.1.3 (2026-11-04), `756fe2d284c7292c4e9b62c8a28f1e5629c59532`.
  - `actions/attest` v4.2.2 (2026-08-04), `1e69f48acb82d1966a394da916b4c1698aa569d6`. It bundles `@sigstore/oci` 0.7.2.
  - `docker/login-action` v4.8.0 (2026-11-05), `8aa5831fb8d48d8f76594c7ddde61ae58019f550`, the newest release. It runs on node24, and its post step logs out (`logout` defaults to `true`). Grimoire doesn't use it.
  - crane (go-containerregistry) v0.22.1 (2026-09-04), commit `8a72a424fdecb4caa14f2d525e5d2503331442b5`, Apache-2.0. `checksums.txt`: `go-containerregistry_Linux_x86_64.tar.gz` `0ab7a1d6932a213aed964ce97666c3077fe691c8606413674a8b3e0b9ec4cda0`, `go-containerregistry_Linux_arm64.tar.gz` `898c0cff975f898a33e8c4580bdafb0e7c02c7faa33374e946762f97c4ab7110`.
  - regctl v0.11.7 (2026-10-21), commit `36abda84830dae07db00bf9e5cec95d97b78222a`, Apache-2.0. It has no checksums file. GitHub asset digests: `regctl-linux-amd64` `3737fbbdd791e04e7a831640b681f3397a376498e80e2ad9b1b2d27296f239bc`, `regctl-linux-arm64` `9f349bab5c407ac04f3dabf99c7c7a6b0f9e9723c23f2775380e804d02c6dbb9`.
  - oras v1.3.6 (2026-10-24), commit `9b1fc815f6b7fba743f08b9b5c5846639cc11d3c`, Apache-2.0. `oras_1.3.6_checksums.txt`: linux_amd64 `66fb0ad7206f3f6fa77ed5b552049d364ccf4b50b1684554f731f76302b52ca0`, linux_arm64 `b84269e78dfc2d5329a4aab373da3200de95e6c138338cf6c66c10c0a3f248c1`.
  - distribution v3.1.2 (2026-09-24), commit `3220848f15d9279c66a41aa0a257469e6aede1e9`, Apache-2.0. `registry_3.1.2_linux_amd64.tar.gz` `40df2224d410f72ae425c3371873b078bbdbda3b8b612be9571f0e6751f3acc8`, `…_linux_arm64.tar.gz` `09d26f88d2c0f161bd1b8bfc6c123571cc3d291dcca8af5477b3cacc4ec95e73`. The image `distribution/distribution:3.1.2` (also `3`, `3.1` and `latest`) is index `sha256:d106962e6fe3fa69c178cec77eeeee5edb626b8df6535b7bd6b95e658a0a5ca9` on both docker.io and ghcr.io, pushed 2026-09-24.
  - gh 2.103.3 (2026-10-26), commit `63837fb778d6a39f02be0ce76ff89f2bc8d74bdc`, preinstalled on the runners.
  - skopeo v1.25.1 (2026-11-10), commit `1fb351f23dbde474416ba33c3786a8d04304429b`, has no release assets. zot v2.1.23 (2026-11-02), commit `d7915e82e6693255338ef7fa9bb15300c82db020`.
  - **Too young to pin:** gh 2.104.0 (2026-11-12), oras v1.4.0 (2026-11-17), skopeo v1.25.2 (2026-11-16) and BuildKit v0.35.0 (2026-11-11), whose newest eligible release is v0.34.2 (2026-11-03). Also the Docker Official Image `registry:3`, `3.1`, `3.1.2` and `latest`: all are `sha256:82f6ff4a7f982960c8aa51617fea3aabc485df6e55cce80d22deb1de36f1bc3c`, pushed 2026-11-11T23:38:20Z.
- **GHCR has no referrers API:**
  - An anonymous `GET /v2/<name>/referrers/<digest>` returns 404 `MANIFEST_UNKNOWN`. That held for signed digests in four repositories and for the zero digest.
  - The spec says a registry that supports the API "MUST NOT return a `404 Not Found`". A client that gets a 404 must use the referrers tag schema `<alg>-<ref>` instead (distribution-spec v1.1.1, lines 577 and 720).
- **The fallback index on GHCR:**
  - Referrers are listed in an OCI image index tagged `sha256-<subject hex>`. cosign v3.1.4's own index `sha256:0f06154836d2de34f887a5c273506d52ca43edaa1d9e53e6510fa78a48f69c2d` has one at `sha256-0f06…` (digest `sha256:87565cc0acb1dedc401035be388172c5695da7dea2b31c4c208376706bcda46f`).
  - That index has two entries. Both have `artifactType` `application/vnd.dev.sigstore.bundle.v0.3+json`, `dev.sigstore.bundle.content: dsse-envelope` and `dev.sigstore.bundle.predicateType: https://sigstore.dev/cosign/sign/v1`.
  - `actions/actions-runner`'s index has a provenance entry (`https://slsa.dev/provenance/v1`), pushed by `actions/attest-build-provenance@v4` with `push-to-registry: true`. Its signer is `https://github.com/actions/runner/.github/workflows/docker-publish.yml@refs/heads/main`.
  - Signatures and attestations share one artifact type. Only the `predicateType` annotation tells them apart.
- **GHCR manifest behaviour:**
  - GHCR stores and serves OCI 1.1 manifests unchanged: `subject`, `artifactType` and the empty config (`application/vnd.oci.empty.v1+json`).
  - It serves untagged manifests by digest.
  - Manifest GETs return an `ETag` equal to the digest.
  - Fallback tags are listed by `tags/list` as ordinary tags: 23 of `actions/actions-runner`'s 80 tags. `tags/list` returns at most 1,000 tags per page, with `Link: …; rel="next"`, even when asked for `n=5000`.
- **Names:**
  - The OCI grammar allows only lowercase (`[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*…`, spec line 157).
  - go-containerregistry v0.22.1 rejects any repository name containing an uppercase letter (`repositoryChars`). So do cosign, crane and gh, which use it.
  - `actions/attest` lowercases the subject name when `push-to-registry` is set.
  - GHCR itself served `Sigstore/cosign/cosign` with a 200.
  - `${{ github.repository }}` is `jabbott-iii/Grimoire`. So Grimoire writes the image name as the literal `ghcr.io/jabbott-iii/grimoire`.
- **Auth from Actions:**
  - GitHub's docs recommend `GITHUB_TOKEN`, with `packages: write` and `contents: read`. The login is `docker login ghcr.io -u ${{ github.actor }} --password-stdin`.
  - A package first published by a workflow with `GITHUB_TOKEN` is linked to that repository. `GITHUB_TOKEN` can't push to a name that was pushed earlier but never connected to the repository.
  - Repositories that publish through a workflow get `admin` on the package.
  - A package inherits the repository's access only if it is linked before its first publish, for example through the `org.opencontainers.image.source` label.
  - "Manage Actions access" → **Add Repository** grants other repositories' workflows access. Giving a public repository access to private packages can expose them to forks.
- **Visibility:**
  - The docs conflict. One page says "When you first publish a package, the default visibility is private." Another says a package created by a workflow with `GITHUB_TOKEN` "inherits the visibility and permissions model of the repository". A third says a linked package inherits "the access permissions (but not the visibility)".
  - "Once you make a package public, you cannot make it private again."
  - The REST API has only get, list, delete and restore for packages and versions. No endpoint changes visibility.
  - Public packages can be pulled anonymously. An unauthenticated request gets `401` with `Bearer realm="https://ghcr.io/token",service="ghcr.io"`.
  - The token endpoint returns 200 with a pull token for a public repository, and 403 `DENIED` for a private or missing one.
  - On 2026-11-17, `jabbott-iii/grimoire` gave 403 there and 404 "Package not found" from REST.
- **Other GHCR rules:**
  - Each layer is limited to 10 GB, and each upload to 10 minutes.
  - The package page shows `org.opencontainers.image.source`, `.description` (at most 512 characters) and `.licenses` (at most 256). For a multi-arch image the description comes from the index `annotations`. The docs' example uses `outputs: type=image,name=target,annotation-index.org.opencontainers.image.description=…`.
  - Deleting or restoring versions with `GITHUB_TOKEN` needs admin on the package, and is in public preview.
  - A public package with any version over 5,000 downloads can't be deleted. A restore must happen within 30 days.
  - None of the 87 Packages docs files mention immutability, referrers, untagged versions or signatures.
- **cosign v3.1.4 image signing:**
  - By default (`--new-bundle-format`, deprecated, default `true`), cosign signs a DSSE in-toto statement with predicate `https://sigstore.dev/cosign/sign/v1`. It pushes it as a referrer manifest by digest (`repo@sha256:…`), with `artifactType` `application/vnd.dev.sigstore.bundle.v0.3+json` and `subject` set to the signed digest. `--upload` defaults to `true`.
  - `.sig` tags are written only with `--new-bundle-format=false`. `--registry-referrers-mode` needs `COSIGN_EXPERIMENTAL=1` and is read only on that legacy path.
  - The push goes through go-containerregistry v0.22.1 `remote.Put`. After every PUT of a manifest with a `subject`, ggcr GETs `/referrers/<subject>` and ignores any `OCI-Subject` header.
  - On a 404 or 400, ggcr GETs the `sha256-<hex>` index, appends the new descriptor and PUTs the index back. This fallback is on by default. It sends no conditional request ("TODO: use conditional requests to avoid race conditions").
  - Without `-r/--recursive`, `cosign sign ref@sha256:<index>` signs only the index. With it, cosign walks every child image manifest, including BuildKit's `unknown/unknown` attestation manifests, and signs each. `--bundle` is refused with `--recursive` or with several images.
  - `COSIGN_REPOSITORY` changes the repository where signatures are written and read.
  - If a Rekor upload is configured and `--yes` is absent, cosign makes an unauthenticated GET of the image. If that fails, it warns that the image "appears to be a private repository" and asks. A declined or failed answer drops the Rekor upload. `--yes` skips that check and the privacy prompt.
  - `cosign load --dir` reads only layouts written by `cosign save`. It selects them by `kind` annotations (`dev.cosignproject.cosign/image`, `…/imageIndex`).
- **cosign offline image dry-run** (read from source):
  - `--key` with a `cosign signing-config create` file that names no services fetches no TUF root and gets no OIDC token. It calls no Fulcio, Rekor or TSA.
  - The only traffic goes to the registry: a GET and a HEAD of the subject, uploads of the bundle and the empty-config blob, a PUT of the referrer manifest by digest, a GET of `/referrers`, then a GET and a PUT of the fallback tag.
  - `cosign verify --key <pub> --insecure-ignore-tlog <ref>@sha256:…` skips the trusted-root fetch.
  - `--tlog-upload=false` together with a signing config is refused.
  - **Checked locally (2026-11-17, cosign 3.1.4, on the local `crane registry serve` below):** an offline signature of the pushed index, made with a throwaway key and a signing config that names no services, landed in the `sha256-<index hex>` fallback tag as an `application/vnd.dev.sigstore.bundle.v0.3+json` manifest annotated `dev.sigstore.bundle.predicateType=https://sigstore.dev/cosign/sign/v1`.
- **cosign verify:**
  - It resolves the reference to a digest, lists referrers (falling back to the tag) and fetches each bundle.
  - **If it finds no new-format bundle, it silently switches to legacy `.sig` lookup.**
  - Keyless verification fetches the TUF trusted root unless `--trusted-root` is given. The Grimoire SAN is in SUPPLY-CHAIN-P5.
- **Plain HTTP:**
  - For `localhost:<port>`, `*.localhost`, `127.0.0.1`, `::1` and RFC 1918 addresses, go-containerregistry tries HTTPS first and falls back to HTTP. That covers cosign and crane. A bare `localhost` with no port doesn't match.
  - cosign also has `--allow-http-registry` ("Don't use this for anything but testing").
  - regctl enables TLS for every host until `regctl registry set <host> --tls disabled` is run.
  - oras uses plain HTTP by default only when the host is `localhost`. Other hosts need `--plain-http`.
  - skopeo uses `--dest-tls-verify=false`.
- **`actions/attest` v4.2.2 for images:**
  - Inputs are `subject-name` (fully qualified), `subject-digest` (`sha256:…`) and `push-to-registry: true`. The README says not to include a tag, and v4.2.2 strips one anyway.
  - A push needs exactly one subject with a SHA-256 digest. The name is lowercased.
  - Credentials come only from `$HOME/.docker/config.json`, under `auths[<registry>].auth` or `identitytoken`, matched to the exact canonical registry name. `DOCKER_CONFIG` and credential helpers are ignored.
  - The write path pushes the blobs and the referrer manifest by digest, then GETs `/referrers/sha256:000…0`. If that isn't a 200, or the PUT returned no `OCI-Subject`, it updates the `sha256-<hex>` index and sends the GET's `ETag` as `If-Match`.
  - The README's image example grants `id-token: write`, `attestations: write`, `packages: write`, `contents: read` and `artifact-metadata: write`.
  - Storage records are for organization-owned repositories only. With `create-storage-record` at its default of `true`, the action calls `GET /repos/{owner}/{repo}`, and returns before creating a record when the owner is a user. Errors there only warn.
- **`gh attestation verify` for images** (v2.103.3):
  - `oci://<image>@sha256:… --repo OWNER/REPO` fetches attestations from the GitHub API. The API answers anonymous reads for a public repository, but gh skips its own auth check only with `--bundle`. The help says an OCI URL also needs a registry login.
  - `--bundle-from-oci` reads referrers, falling back to the tag, through go-containerregistry v0.22.1 and `DefaultKeychain`. It keeps artifact types starting with `application/vnd.dev.sigstore.bundle`, and then checks only the names in the certificate.
- **crane v0.22.1:**
  - The release has `checksums.txt` and `multiple.intoto.jsonl`. The latter is a Sigstore v0.3 bundle holding SLSA v0.2 provenance, signed by `https://github.com/slsa-framework/slsa-github-generator/.github/workflows/generator_generic_slsa3.yml@refs/tags/v2.1.0`, with build trigger `https://github.com/google/go-containerregistry/.github/workflows/release.yml@refs/tags/v0.22.1`. Its subjects match `checksums.txt`.
  - There is no cosign signature asset, and the attestations API returns 404.
  - `crane push <dir> <ref>` reads a directory as an OCI layout. If `index.json` has exactly one entry and it is an index, crane pushes it with `remote.WriteIndex`. Child manifests, attestation manifests included, are written first.
  - It prints `<repo>@<digest>` to stdout. `--image-refs <file>` also writes it to a file with mode 0600.
  - With more than one entry it fails ("layout contains %d entries, consider --index"). `--index` pushes `index.json` itself as a new index, with a different digest.
  - BuildKit v0.34.2's OCI exporter, through containerd v2.4.2, writes one `index.json` entry when no image name is given, and one entry per name otherwise.
  - `crane auth login <registry> -u <user> --password-stdin` saves through docker/cli's config package, to `$DOCKER_CONFIG` or `~/.docker/config.json`.
  - `crane registry serve` listens on `:$PORT`, or a random port, on all interfaces, unless `--address` is given.
  - Its manifests are always held in memory. `--disk <dir>` stores blobs only.
  - Its referrers API is off, so `/referrers` returns 404 `METHOD_UNKNOWN`. Image manifests aren't checked for media type, and an index PUT fails until its child manifests exist.
  - **Checked locally (2026-11-17, rootless Docker 29.9.3, crane v0.22.1):** `crane push` of Grimoire's buildx layout (both platforms, with the SBOM and mode=max provenance attestation manifests) to a local `crane registry serve` kept the index digest: the digest it printed equals the layout's single `index.json` entry. The registry answered `/referrers` with 404 `METHOD_UNKNOWN`, so the push wrote `sha256-<platform hex>` fallback tags for BuildKit's attestation manifests, which carry a `subject`. GHCR will show the same tags: never delete them.
- **regctl v0.11.7:**
  - Each binary has a keyless cosign signature: `.sig`, `.pem` and a `.sigstore.json` v0.3 message-signature bundle. The SAN is `https://github.com/regclient/regclient/.github/workflows/go.yml@refs/tags/v0.11.7` and the issuer `https://token.actions.githubusercontent.com`. The bundle's digest equals the asset digest.
  - The release also has SPDX and CycloneDX SBOMs.
  - `regctl image copy ocidir://<dir>@sha256:<d> <ref>` PUTs the source manifest bytes. `--referrers` and `--digest-tags` also copy referrers and `sha256-<digest>.*` tags.
- **oras v1.3.6:**
  - The release has a checksums file and GPG `.asc` signatures. New in 1.3.6, the archives and the checksums file also have GitHub SLSA v1 attestations.
  - `oras cp --from-oci-layout <dir>@<digest> <ref>:<tag>` copies every manifest of an index. `-r` (preview) also copies referrers.
- **skopeo:**
  - The maintainers "prefer to run Skopeo using distro packages or within containers".
  - The `ubuntu-24.04` image 20261111.370.1 ships skopeo 1.13.3, Docker 28.0.4, buildx 0.38.2 and Podman 4.9.3.
  - `skopeo copy --all --preserve-digests oci:<dir> docker://<ref>` copies a layout. `--preserve-digests` fails if a digest can't be kept.
- **Local registries:**
  - distribution v3.1.2 has no referrers route. Its routes are base, manifest, tags, blob, blob-upload, blob-upload-chunk and catalog, and no file in the tree mentions referrers.
  - Its tarballs hold a static `registry` binary plus README and LICENSE.
  - Each `.sha256` file holds the 64 hex characters and a newline, with no file name.
  - Each `.provenance.json` is an unsigned in-toto statement (SLSA v1). The attestations API returns 404.
  - The `distribution/distribution` image is Alpine 3.24 with no `USER`. It runs `registry serve /etc/distribution/config.yml`, copied from `config-dev.yml`: port 5000, debug and Prometheus on 5001, deletes enabled.
  - Its index lists 7 platforms (linux/amd64 and linux/arm64 among them) and BuildKit attestation manifests. It has no fallback tag, so no signatures.
  - zot v2.1.23 registers `/v2/<name>/referrers/<digest>` unconditionally, and publishes `checksums.sha256.txt`.
- **Service containers:**
  - The runner (v2.338.2) passes `services.<id>.image` to Docker unchanged, so `name@sha256:…` works.
  - zizmor 1.32.0's `unpinned-images` audit covers services. A tag without a digest is a pedantic finding; `latest` or no tag is a regular one.
  - A job that runs on the host must map the service's ports and reach it on `localhost`.
  - GitHub-hosted runners "are not subject to" Docker Hub's rate limits.
- **Changed since the register:**
  - GHCR's referrers support, left open in COSIGN and SUPPLY-CHAIN-P5, is settled. GHCR has no referrers API and uses the referrers tag schema. Image signatures are OCI 1.1 referrer manifests, listed on GHCR through the `sha256-<hex>` tag.
  - ATTEST's permissions need `packages: write` when attestations are pushed to an image.
  - Unchanged: cosign v3.1.4, cosign-installer v4.1.3, `actions/attest` v4.2.2 and gh 2.103.3 are still the newest eligible releases.
- **UNVERIFIED:**
  - whether GHCR sends `OCI-Subject` on a manifest PUT with a subject, and whether it honours `If-Match` (both need a push);
  - which visibility a package first pushed by a workflow in a user account actually gets;
  - that GHCR has no tag-immutability feature at all (the docs are silent);
  - whether GHCR links a multi-arch package from the index annotation or from the platform config label;
  - whether referrer manifests and fallback indexes appear as separate package versions, and whether untagged-version cleanup deletes them (listing versions needs `read:packages`);
  - whether a private `grimoire` package already exists;
  - whether the `ubuntu-24.04` image sets a Docker `credsStore` (code search found none);
  - end-to-end digest preservation in regctl, oras and skopeo (read, not run), and in any tool pushing to GHCR itself;
  - how skopeo's `oci:` transport handles attestation manifests;
  - how policy engines treat a signature on the index compared with one on each platform manifest;
  - what cosign's confirmation prompt does without a terminal.
- **Settled after the research:** whether `crane registry serve` accepts a full BuildKit layout with an SBOM and max provenance, and digest preservation by crane, to a local registry; and where cosign's offline image signature lands. All three were checked locally (above).

### SCAN-P6: Grype as the image vulnerability gate
Sources: https://github.com/anchore/grype at v0.121.3 (commit `cd1f23eb50df`: `cmd/grype/cli/{cli.go,options/,commands/}`, `grype/db/v6/{distribution,installation}/`, `grype/load_vulnerability_db.go`, `grype/match/ignore.go`, `grype/vex/`, `grype/vulnerability_matcher.go`, `grype/vulnerability/severity.go`, `internal/format/`, `.goreleaser.yaml`, `.github/workflows/release.yaml`, `install.sh`, `go.mod`), its release API, release assets and attestations API, and https://github.com/anchore/grype/blob/v0.118.0/go.mod; https://github.com/anchore/scan-action at v7.4.2 (`27805bf3b4e8`: `action.yml`, `action.js`, `GrypeVersion.js`, `dist/index.js`, `download-grype/action.yml`) and main `aba84ded01d9`; https://github.com/anchore/syft at v1.57.1 (`internal/task/package_tasks.go`, `syft/pkg/cataloger/{debian,rust,binary}/`); https://github.com/anchore/stereoscope at v0.4.1 (`pkg/image/{platform.go,image.go}`, `pkg/image/oci/`, `pkg/image/docker/tarball_provider.go`); https://github.com/anchore/grype-db at `de4285eb2e68` (`.github/workflows/daily-db-publisher-r2.yaml`, `manager/src/grype_db_manager/cli/db.py`); https://github.com/anchore/oss-docs at `75cad52cd932` (`content/docs/guides/vulnerability/{database,filter-results,scan-targets}.md`, `guides/sbom/scan-targets.md`, `architecture/grype-db.md`, `reference/grype/{cli,configuration}.md`, `capabilities/snippets/ecosystem/rust/`, `announcements/grype-db-v5-eol.md`; the references were generated with Grype 0.119.0); https://grype.anchore.io/databases/v6/latest.json; https://github.com/anchore/vunnel/blob/32e6589725bb/src/vunnel/providers/debian/parser.py; https://github.com/GoogleContainerTools/distroless at `0af09fb8f491` (`static/config.bzl`, `static/static.bzl`, `MODULE.bazel`); https://github.com/GoogleContainerTools/rules_distroless/blob/v0.8.0/apt/private/dpkg_statusd.sh; https://gcr.io/v2/distroless/static-debian13/manifests/nonroot; https://ghcr.io/v2/anchore/grype/manifests/v0.121.3; https://github.com/rust-secure-code/cargo-auditable/blob/v0.7.7/README.md; https://github.com/sigstore/cosign/blob/v3.1.4/cmd/cosign/cli/options/certificate.go; https://github.com/spf13/pflag/blob/v1.0.10/string_array.go; https://github.com/zizmorcore/zizmor at v1.32.0 (`crates/zizmor/src/audit/`, `crates/zizmor/data/`); the repository advisories of anchore/grype, syft, stereoscope and scan-action, and the global advisory database. Status: Verified, except the gaps listed. The research ran no Grype scan and no cosign verification. The checksums bundle's certificate was decoded with `openssl x509`. Registries, the release and attestations APIs and the DB listing were queried anonymously with `curl` or with `gh api`.

- **Pins:**
  - Grype **v0.121.3** (2026-11-09T15:17:25Z), the newest release at least 7 days old. Tag object `c1fb20024a1fd80208894d7a11bfdce98f1e666d`, commit `cd1f23eb50dfb52647090158f453c7359172591f`. The release is immutable.
  - `grype_0.121.3_linux_amd64.tar.gz`: SHA-256 `e9da18cc1429e786722e32161288103600efffd8e483c90761f90485bd1bec85`, 32,643,999 bytes. `grype_0.121.3_linux_arm64.tar.gz`: `b7815598d48da78bfd16f5b98ef28e1bb09c03d34fc8c3e350f99b6ccb3c7cba`. The release API digest, `grype_0.121.3_checksums.txt` and GitHub's release attestation all agree.
  - `grype_0.121.3_checksums.txt`: `1f8cea030e00f362d6b0bb0d1843b123af1d6baad70975dff1cf6fe14096866f`. Its bundle, `grype_0.121.3_checksums.txt.sigstore.json`: `c45178e6831b38291cd48f51d49dfa5ef2251afdbed7c3812d6b33bf9049fe74`.
  - `anchore/scan-action` v7.4.2 (2026-08-28), commit `27805bf3b4e84b4a5c980df22ed233c00390a439`, is unchanged and still the newest release. It is recorded for comparison only. Tag v7.4.1 exists without a release.
  - `gcr.io/distroless/static-debian13:nonroot` re-resolved unchanged: index `sha256:aafe993665bf6200db38a02c11a9351e782a84b6a45e7f33b6695638c88e3931`, linux/amd64 `sha256:816dc4d589459a7fe33b8a2e69edb70da56ffad7161f4b415937fa2bfe7e92e8`, linux/arm64/v8 `sha256:50e178649a8d77a5d8414f205471c829f06b876cbdb2455dace0b5de549fd1e5`. Grimoire pins the previous index, `25ac8f0c…` (BASE-IMAGE-P6).
- **Changed since SCAN (2026-11-16):** v0.122.0 (2026-11-16) is too young to pin, so the pin is v0.121.3. v0.121.3 embeds syft v1.57.1 and stereoscope v0.4.1. Its `go.mod` declares `go 1.26.8` and has no `toolchain` line.
- **Checked locally (2026-11-17, Grype 0.121.3, DB schema v6.1.10 built 2026-11-17T06:20:58Z):** Grimoire's image, on the pinned base `25ac8f0c…`, has **no findings on either platform**.
- **Release signing:**
  - goreleaser signs only the checksums file, with `cosign sign-blob --bundle`. There are no per-archive signatures and no SLSA build provenance: the attestations API holds only GitHub's release attestation for the tarball.
  - The `ghcr.io/anchore/grype` images are unsigned. The goreleaser config has no `docker_signs`, and `v0.121.3` (`sha256:13113eeceab017260370cf540e1d25442c59665fd07d1b09b66d63bb0c282887`) has no `.sig` tag; the referrers query returns 404.
  - The bundle certificate has SAN `https://github.com/anchore/grype/.github/workflows/release.yaml@refs/heads/main` and issuer `https://token.actions.githubusercontent.com`. Its other values: trigger `workflow_dispatch`, workflow name `Release`, repository `anchore/grype`, ref `refs/heads/main`, workflow SHA `cd1f23eb50dfb52647090158f453c7359172591f`. The message digest equals the checksums file's SHA-256. The bundle carries a Rekor v1 inclusion proof (log index 3766582882, 2026-11-09 15:13:21 UTC) and one RFC 3161 timestamp.
  - cosign v3.1.4 can pin all of these with `--certificate-identity`, `--certificate-oidc-issuer` and `--certificate-github-workflow-{sha,trigger,name,repository,ref}`.
  - GitHub's release attestation (SAN `https://dotcom.releases.github.com`, predicate `https://in-toto.io/attestation/release/v0.2`) has 18 subjects: the 17 assets and `pkg:github/anchore/grype@v0.121.3`, keyed by the tag object's SHA-1.
  - Grype's `install.sh` checks the bundle only with `-v`, using the same SAN and issuer. Its error message says bundles need cosign v2.5.0 or later, while a comment in the same script says v2.4.2.
  - Grype's release workflow runs only on `workflow_dispatch`, in a `release` environment. It has a `skip-checks` input that releases even if checks haven't passed on `main`.
- **Release contents:** Apache-2.0. Linux builds use `CGO_ENABLED=0`, `-extldflags '-static'` and `-w -s`. `install.sh` installs `grype` from the archive root.
- **scan-action v7.4.2 (not used):**
  - On Linux it downloads `https://raw.githubusercontent.com/anchore/grype/<tag>/install.sh` and runs it with `sh`, with `DOWNLOAD_TAG_INSTALL_SCRIPT=false`. No hash is pinned and `-v` isn't passed. The installer checks the tarball only against the same release's `checksums.txt`.
  - A `grype-version` that isn't a release tag (`^v\d+\.\d+\.\d+([-+][\w.+-]+)?$`) gets the installer from `main` with `DOWNLOAD_TAG_INSTALL_SCRIPT=true`. That installer then downloads another one from `https://get.anchore.io/grype/<tag>/install.sh` (falling back to raw.githubusercontent.com) and pipes it to `sh`.
  - Its default Grype is v0.118.0, both at v7.4.2 and on `main` (`aba84ded01d9`, 2026-11-13). v0.118.0 embeds syft v1.51.1 and stereoscope v0.3.1. That syft is below the fix for GHSA-mw2c-m758-9v5q (v1.52.0).
  - Defaults: `fail-build: true`, `severity-cutoff: medium` (passed as `--fail-on`), `output-format: sarif`, and `cache-db` off. It runs on node24.
  - It sets `GRYPE_CHECK_FOR_APP_UPDATE=false` and passes the step environment to Grype. It has no platform input. It reads `registry-username` and `registry-password`, which `action.yml` doesn't declare. It keeps the binary in the runner tool cache.
  - Exit 2 fails the step, or only warns when `fail-build` is false. Any other non-zero exit fails it.
  - The `vex` input becomes a single `--vex` argument. Grype's `--vex` is a pflag StringArray, which never splits on commas, so the action can pass only one document.
- **Vulnerability DB:**
  - The default `db.update-url`, `https://grype.anchore.io/databases`, resolves to `…/v6/latest.json`. It is served anonymously over HTTPS through Cloudflare with `cache-control: public,max-age=300`, and names one archive and its SHA-256. Archives are served with `max-age=31536000`.
  - On 2026-11-17 the DB was schema v6.1.10, built 2026-11-17T06:20:58Z: `vulnerability-db_v6.1.10_2026-11-17T00:22:24Z_1794896458.tar.zst`, 203,145,524 bytes, `sha256:3f0cbdbc9d40fb69eaeacec1869b27a8c74f1894108804ace8b7468513e2df07`. The source's "~156MB as of 2024-04-17" is stale.
    - **Checked locally (2026-11-17, Grype 0.121.3):** that DB takes 3.3 GB on disk once unpacked.
  - The archive is checked only against that SHA-256, through a go-getter `checksum` query parameter. Neither file is signed, so authenticity rests on TLS. `history.json` and `latest.json.sig` return 404, and `db list` shows only the latest entry.
  - `grype-db` publishes daily at 06:00 UTC (`cron: '0 6 * * *'`). `db build-and-upload` validates the DB first, including a `latest.json` smoke test. It then uploads the archive and, after it, `latest.json`: two uploads, not an atomic swap. Validation can be skipped with `--skip-validate` or disabled per schema.
  - Schema v5 stopped receiving updates on 2026-03-06. Grype v0.88.0 and later use v6.
- **DB checks and failure modes (v0.121.3 source):**
  - Defaults: `db.auto-update: true`, `validate-by-hash-on-start: true`, `validate-age: true`, `max-allowed-built-age: 120h`, `require-update-check: false`, `update-available-timeout: 30s`, `update-download-timeout: 5m`, `max-update-check-frequency: 2h`, `cache-dir: ~/.cache/grype/db`. The env names are `GRYPE_DB_*`.
  - The load fails, and the scan exits 1, when the DB is older than the maximum age or no longer matches the digest recorded at import. The hash check protects the file on disk, not its origin.
  - If the listing can't be fetched and `require-update-check` is false, Grype only warns and uses any DB already installed. With no installed DB, the scan fails.
  - `grype db import` takes a URL or a file. Only a URL with `?checksum=sha256:…` is verified; a local archive is unpacked and activated with no comparison. `db.update-url` can point at a mirror, and the docs describe mirroring for air-gapped systems.
- **Network at run time:**
  - A scan reaches `grype.anchore.io`. It also reaches `https://toolbox-data.anchore.io/grype/releases/latest/VERSION` (which returned `v0.122.0`) unless `check-for-app-update` is false (env `GRYPE_CHECK_FOR_APP_UPDATE`).
  - External sources (Maven) are off by default.
  - Release downloads redirect from github.com to `release-assets.githubusercontent.com`.
  - Every one of these hosts was reached anonymously.
- **Scan sources and platforms:**
  - Schemes include `oci-dir:`, `oci-archive:`, `docker-archive:`, `registry:`, `docker:`, `podman:`, `containerd:`, `sbom:` and `dir:`. A scheme or `--from` stops auto-detection. The default scope is `squashed`.
  - `oci-dir:` and `oci-archive:` select exactly one image; `oci-archive:` is unpacked and handed to the directory provider.
    - A single-image layout is taken as is.
    - In a multi-image layout, the platform defaults to `linux/<host GOARCH>`, and anything other than exactly one match is an error.
    - Nested indexes are walked, and buildx attestation manifests are skipped.
    - With `--platform`, the image config must match.
    - **Checked locally (2026-11-17, Grype 0.121.3):** `grype oci-dir:<layout> --platform linux/amd64`, and again with `--platform linux/arm64`, each selected that platform's image from Grimoire's buildx layout. The JSON reports list no tags and no repo digests for these `oci-dir:` sources.
  - `registry:` also defaults to the host architecture for an index, and records `<registry>/<repo>@<digest>` as a repo digest. The Syft docs say the default is `linux/amd64`, but the code uses the host architecture.
  - `--platform linux/arm64` matches `linux/arm64/v8`: variant `8` or `v8` is normalised to empty, and an empty variant matches any.
  - `docker-archive:` accepts only one image per tar.
- **What Grype sees in the image:**
  - static-debian13 installs six Debian packages: `base-files`, `ca-certificates`, `netbase`, `tzdata`, `tzdata-legacy` and `media-types` (BASE-IMAGE-P6; the research here listed five, without `ca-certificates`). Their control files are under `/var/lib/dpkg/status.d/` (rules_distroless 0.8.0). Syft v1.57.1's dpkg cataloger reads `**/lib/dpkg/status.d/*` in images, and vunnel maps `trixie` to Debian 13.
    - **Checked locally (2026-11-17, Grype 0.121.3):** Grype lists all six on both platforms.
  - Rust crates come only from cargo-auditable's `.dep-v0` section, through `cargo-auditable-binary-cataloger` (every executable MIME type, reported as `pkg:cargo`). A binary without that section yields no packages and no warning, and only runtime dependencies are kept.
    - **Checked locally (2026-11-17, Grype 0.121.3):** `.dep-v0` survives the `dist` profile on both musl targets. Grype's CycloneDX output lists 92 `pkg:cargo` packages at `/usr/local/bin/grimoire` (91 on arm64), and, on both platforms, 135 at `/opt/grimoire/bin/grimoire-mcp-project-docs` and 41 at `/opt/grimoire/bin/grimoire-tool-secret-guard`. `cargo xtask release image coverage` fails the gate when a binary shows none.
  - Syft's only Rust classifier matches `**/libstd-????????????????.so`, so it never applies to static executables.
  - cargo-auditable doesn't track statically linked C libraries. By inference, that includes the musl libc that the musl targets link.
  - Rust packages are matched without CPEs by default (`match.rust.using-cpes: false`). The documented Rust data source is GHSA, with NVD only when CPE matching is on.
- **The gate:**
  - `--fail-on`/`-f` exits **2** (config `fail-on-severity`, env `GRYPE_FAIL_ON_SEVERITY`). `db check` exits 100 when an update is available. Every other error exits 1.
  - Matches with Unknown severity never trip `--fail-on`, and a match whose metadata lookup fails is skipped.
  - `--only-fixed`, `--only-notfixed`, `--ignore-states`, ignore rules and VEX are all applied first, and the threshold sees only what is left.
  - **Checked locally (2026-11-17, Grype 0.121.3):** a medium GHSA match exits 2, and an ignore rule keyed on the GHSA ID and the package name suppresses it (exit 0).
- **Config and ignore rules:**
  - Config search order: `./.grype.yaml`, `./.grype/config.yaml`, `~/.grype.yaml`, then `$XDG_CONFIG_HOME/grype/config.yaml` (from the docs). **Checked locally (2026-11-17, Grype 0.121.3, `grype config --load`, in the Phase 6 review):** without `-c`, Grype merges every file it finds (ignore rules from all of them, and settings such as `fail-on-severity` from `~/.grype.yaml`); with `-c`, it reads only the named file. `GRYPE_*` environment variables apply either way.
  - The `--ignore-states` flag is read from the config key `ignore-wontfix` (env `GRYPE_IGNORE_WONTFIX`), not `ignore-states`. The generated docs agree.
  - Rule fields: `vulnerability`, `include-aliases`, `reason`, `namespace`, `fix-state`, `match-type`, `vex-status`, `vex-justification`, and `package.{name,version,language,type,location,upstream-name}`.
  - Every field given must match, and a rule with no fields never matches. `reason` is optional and doesn't affect matching. There is no expiry field.
  - `vulnerability` is compared exactly with the match's ID, and with related IDs only when `include-aliases: true`.
  - `package.name` and `upstream-name` become anchored regular expressions when they contain regex characters. `location` is a doublestar glob.
  - **Checked locally (2026-11-17, Grype 0.121.3):** unknown config keys are ignored silently, also inside an ignore rule, where a misspelt criterion is dropped and the rule then matches more (a misspelt `vulnerability` ignores every finding in the package; checked in the Phase 6 review). So Grimoire's workflows repeat the threshold with `--fail-on medium`, and a unit test reads `.grype.yaml` line by line and refuses any unknown key, top-level, under `db` or in a rule, and any rule without a GHSA or CVE id, a package name and a reason.
- **VEX:**
  - Grype reads OpenVEX and CSAF (`--vex`, config `vex-documents`), and takes the format from the first document.
  - By default, `not_affected` and `fixed` statements move matches to the ignored list. `affected` and `under_investigation` add matches only through `vex-add`.
  - Image products are matched on the image's tags and repo digests (as `pkg:oci` purls), then on each package's own PURL. The `oci-dir:` provider records only the manifest digest, so `pkg:oci@sha256` products don't match an `oci-dir:` scan.
- **Outputs:** `json`, `table`, `cyclonedx` (XML), `cyclonedx-json`, `sarif` and `template`. The embedded CycloneDX VEX formats are deprecated. `-o <format>=<file>` can be repeated to write several files. `--show-suppressed` works for table output only.
- **Doc and code conflicts:**
  - Exit code for `--fail-on`: the `fail-on-severity` description and the generated config docs say 1, but the code exits 2.
  - Exit code for `db check`: the DB guide says it returns 1 when an update is available, but the code returns 100.
  - Default platform: the Syft guide says `linux/amd64`, but stereoscope v0.4.1 uses the host architecture.
- **Incidents and advisories:**
  - No compromise of Anchore tools was found.
  - grype: GHSA-6gxw-85q2-q646 / CVE-2025-65965 (credentials in JSON output), fixed in 0.104.1.
  - syft: GHSA-mw2c-m758-9v5q (memory exhaustion parsing binaries), fixed in v1.52.0 and published 2026-09-17. It has no CVE and isn't in the global advisory database.
  - syft: GHSA-rjcw-vg7j-m9rc / CVE-2026-33481 (temporary-file cleanup), fixed in v1.42.3, plus one older advisory.
  - stereoscope: GHSA-hpxr-w9w7-g4gv (OCI tar path traversal, 2024).
  - scan-action has no advisory.
- **zizmor 1.32.0:** no file in its audit sources or data files mentions `anchore`, `grype` or `scan-action`. `unpinned-tools` knows only `aquasecurity/setup-trivy`, `1password/load-secrets-action`, `extractions/setup-just` and `extractions/setup-crate`.
- **UNVERIFIED:**
  - the bundle's signature and Rekor proof (only the certificate was decoded), and `gh release verify-asset` (local gh is 2.45.0);
  - whether `cosign verify-blob --bundle` needs network access for the TUF trusted root;
  - whether older v6 archives stay downloadable by name, and rate limits on `grype.anchore.io`;
  - whether a new GHCR package can be pulled anonymously before it is made public;
  - the Go toolchain that built the v0.121.3 binaries;
  - the VEX status semantics, which come from docs generated with Grype 0.119.0 and weren't checked in v0.121.3 source (the config search order was checked locally, above).
- **Settled after the research:** whether unknown config keys are rejected (they are ignored), whether `.dep-v0` survives `[profile.dist]` on the musl targets (it does), what Grype reports for the image today (nothing), the DB's unpacked size (3.3 GB), whether a buildx layout carries the index platform fields that `--platform` selection needs (it does: each `--platform` scan found its image), and whether `oci-dir:` sources get tags (they don't): all checked locally, above. GHCR's referrers support: GHCR has none (REGISTRY-P6).

### RUN-P6: running the image, hardening flags, mounted repositories and MCP clients that start containers
Sources: https://github.com/docker/cli at v29.10.0 (`fdd4b8e5da33`: `docs/reference/commandline/{container_run,docker}.md`, `docs/reference/run.md`, `cli/command/container/{run,create,opts,signals,hijack}.go`, `opts/mount.go`, `cli/streams/in.go`, `cli/config/config.go`); https://github.com/moby/moby at docker-v29.10.0 (`06db1734267b`: `daemon/oci_linux.go`, `Dockerfile`, `api/swagger.yaml`); https://github.com/opencontainers/runc at v1.5.3 (`46365c62967c`: `libcontainer/{env,init_linux}.go`); https://man7.org/linux/man-pages/man7/pid_namespaces.7.html (man-pages 6.19); https://github.com/docker/docs at `947343021a3e` (`content/manuals/engine/network/{_index,port-publishing}.md`, `…/network/drivers/{bridge,host,none}.md`, `…/engine/storage/bind-mounts.md`, `…/engine/security/rootless/{uid-gid-mapping,troubleshoot}.md`, `…/desktop/{release-notes,settings-and-maintenance/settings,features/networking/_index,setup/install/linux/_index,troubleshoot-and-support/troubleshoot/topics}.md`); https://github.com/podman-container-tools/podman at v6.2.0 (`055de046937d`: `docs/source/markdown/options/{userns.container,volume,init}.md`); the registry API at https://gcr.io/v2/distroless/static-debian13/ (`tags/list`, manifests, config and layer blobs) and https://github.com/GoogleContainerTools/distroless/pull/2248; https://code.claude.com/docs/en/{mcp,hooks,env-vars}.md, https://github.com/anthropics/claude-code/blob/9fa26078be41/CHANGELOG.md and the npm registry; https://github.com/openai/codex at rust-v0.169.1 (`c376cf7396ef`: `codex-rs/core/src/session/mcp.rs`, `codex-rs/rmcp-client/src/stdio_server_launcher.rs`, `codex-rs/codex-mcp/src/rmcp_client.rs`), the compare `rust-v0.169.0...rust-v0.169.1` and https://learn.chatgpt.com/docs/extend/mcp.md; https://cursor.com/docs/mcp.md; https://github.com/google-gemini/gemini-cli at v0.68.0 (`5a5c7180ff3f`: `packages/core/src/tools/mcp-client.ts`, `packages/core/src/services/environmentSanitization.ts`, `packages/cli/src/utils/envVarResolver.ts`, `packages/cli/src/config/settings.ts`, `docs/tools/mcp-server.md`, `package-lock.json`); https://unpkg.com/@modelcontextprotocol/sdk@1.23.0/dist/esm/client/stdio.js; https://github.com/nodejs/node/blob/v24.11.0/doc/api/child_process.md; https://github.com/microsoft/vscode-docs at `449410e3df07` (`docs/agents/reference/mcp-configuration.md`) and https://update.code.visualstudio.com/api/update/linux-x64/stable/latest; https://github.com/github/docs at `49a012fa50b4` (`content/copilot/how-tos/copilot-cli/customize-copilot/add-mcp-servers.md`, `content/code-security/concepts/supply-chain-security/immutable-releases.md`, `content/packages/learn-github-packages/configuring-a-packages-access-control-and-visibility.md`); https://github.com/opencontainers/distribution-spec/blob/v1.1.1/spec.md; https://github.com/modelcontextprotocol/modelcontextprotocol at tag 2026-07-28 (`5f5440bb26a6`: `docs/specification/2026-07-28/basic/transports/{stdio,streamable-http}.mdx`); https://github.com/actions/runner-images at `27322eb52686` and `74456c0b4eff` (`images/ubuntu/Ubuntu2404{,-Arm64}-Readme.md`); Grimoire's own sources. Status: Verified from documentation, source and registry APIs, except the gaps listed. The research ran nothing against a Docker or Podman daemon; locally it ran only `cargo tree --locked --offline -e normal` and `grep`. The local checks below came later.

- **Versions read.** None of these is pinned in a workflow.
  - **Docker and runc:**
    - docker/cli v29.10.0 was tagged on 2026-11-13 (`fdd4b8e5da335e1d0d83d29af5045f57280cd694`) and has no GitHub release object.
    - moby docker-v29.10.0 was released on 2026-11-16 (`06db1734267bf7770914b417bff6556d835bf8cc`).
    - runc v1.5.3 was released on 2026-11-12 (`46365c62967c76a765a5e347f48f0c7b3b571fe9`).
    - All three are younger than 7 days.
  - **tini:** v0.19.0, 2020-04-19 (`de40ad007797e0dcd8b7126f27bb87401d224240`). moby's `Dockerfile` builds it as `docker-init` (`ARG TINI_VERSION=v0.19.0`).
  - **Podman:** v6.2.0, 2026-11-10 (`055de046937d29450f310fce0fe3071485d8bed9`). The repository is now `podman-container-tools/podman`, and `containers/podman` redirects to it.
  - **Docker Desktop:** 4.99.0, 2026-11-09.
  - **Claude Code:** npm `latest` is 2.1.327 (2026-11-17) and `stable` is 2.1.311 (2026-10-26).
  - **Codex:**
    - rust-v0.169.1 was published on 2026-11-17 (`c376cf7396eff83ce4c6ac61b9553df142e7a0be`), and rust-v0.169.0 on 2026-11-13 (`d32063f96c2dff1dc96af839b3952abb6acfeda4`).
    - The newest stable release on or before 2026-11-10 is rust-v0.167.1 (2026-11-10, `0a94398fa60c4fac0e7659e4213a0a36e613dd46`).
  - **Gemini CLI:** v0.68.0 (2026-11-10, `5a5c7180ff3f0a02511f54b54db7f228d4982a30`) is still the latest stable release. It uses `@modelcontextprotocol/sdk` 1.23.0.
  - **VS Code:** 1.146.0 (2026-11-10, build `69047191fc427f2f92828279aae1d90d8eeda7ab`).
  - **GitHub runners on 2026-11-17:**
    - `ubuntu-24.04` has image 20261116.376.1 and `ubuntu-24.04-arm` has 20261116.188.1.
    - Both ship Docker client and server 28.0.4, Buildx 0.39.0, Compose 2.38.2, Podman 4.9.3, Buildah 1.33.7 and Skopeo 1.13.3.
    - That is a major version behind Engine 29.10.0.
- **Changed since earlier entries:**
  - **BASE-IMAGE:** now records upload dates and the cooldown; see the next bullet.
  - **GEMINI-SETTINGS:** corrected. Unset variables are not blanked everywhere.
  - **CC-MCP:** the `:-` default rule now also covers `~/.claude.json` entries.
  - **CODEX-MCP-P4:** the fallback cwd is now verified.
  - **ACTIONS-P5:** it recorded the 20261111 runner images; the 20261116 images began deploying on 2026-11-17 (above), and jobs may get either until that ends.
- **Base image `gcr.io/distroless/static-debian13`:**
  - **Current `nonroot`:**
    - It still resolves to index `sha256:aafe993665bf6200db38a02c11a9351e782a84b6a45e7f33b6695638c88e3931` (tags `nonroot` and `nonroot-0af09fb8f491…`).
    - GCR records the upload as 2026-11-12T21:16:35Z, 4 minutes after distroless PR #2248 ("Update os-release permissions") merged. It is younger than 7 days until 2026-11-19.
    - amd64 is `sha256:816dc4d589459a7fe33b8a2e69edb70da56ffad7161f4b415937fa2bfe7e92e8`, and arm64 is `sha256:50e178649a8d77a5d8414f205471c829f06b876cbdb2455dace0b5de549fd1e5`.
  - **Previous `nonroot` index** (the one Grimoire pins):
    - `sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329`, uploaded 2026-10-31T22:37:00Z.
    - amd64 is `sha256:336e53e9941facdb69b755e6fd2c63f5400abd3b1ba47bb17ebafc2b1547cebd`, and arm64 is `sha256:bf6233b64054d17b31c3b555b1bbcee58958896830a305ef4d51faa257990d87`.
    - Its tags include `nonroot-f7838e0a1072…` and `update-available-25ac8f0c…`. Its `.sig` and `.att` tags exist.
  - **Digests:** every fetched manifest, config and layer hashed to its digest.
  - **What differs:** besides the configs, the two indexes differ in one layer, `usr/lib/os-release`. Its content is identical in both (`b4d56a90…`). Its mode is 0555 in `25ac8f0c` (layer `ebddc55f…`) and 0644 in `aafe9936` (layer `c3c9c2f2…`).
  - **Config:**
    - `User` is `65532`, and `WorkingDir` is `/home/nonroot`.
    - `Env` sets `PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin` and `SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt`.
    - There is no `HOME`.
  - **Files:**
    - `/etc/passwd` holds root, nobody and `nonroot:x:65532:65532:nonroot:/home/nonroot:/sbin/nologin`.
    - `/home/nonroot` is 0700 and owned by 65532. `/tmp` is 1777.
    - `/bin`, `/sbin`, `/lib` and `/lib64` are symlinks into `usr/`. `usr/bin` and `usr/sbin` are empty, so there is no shell.
    - A CA bundle is present.
- **stdio through `docker run`:**
  - **`-i`** keeps the container's stdin open ("Keep STDIN open even if not attached"). Unless `-d` is given, stdout and stderr are attached.
  - **`-i -t`** fails when stdin is a pipe: "cannot attach stdin to a TTY-enabled container because stdin is not a terminal". With `-t`, the CLI also writes the container's error stream to its own stdout.
  - **Pull output:** pull progress and "Unable to find image … locally" go to the CLI's stderr, and `-q` suppresses them (`create.go`).
  - **EOF:**
    - With `-i` attached, the CLI sets `StdinOnce`, which the API describes as "Close `stdin` after one attached client disconnects".
    - When the CLI's own stdin reaches EOF, it half-closes the attach connection (`CloseWrite`), so the container's stdin reads EOF.
  - **Signals:** `--sig-proxy` defaults to true. The CLI forwards every signal it receives through the kill API, except SIGCHLD, SIGPIPE, Go runtime signals (SIGURG) and names missing from its signal map.
  - **`--rm`** sets the daemon-side `HostConfig.AutoRemove`, which "Automatically remove[s] the container when the container's process exits". It conflicts with `--restart`.
  - **`docker stop`** sends the image's `STOPSIGNAL`, or SIGTERM if there is none. It sends SIGKILL after the daemon's stop timeout: 10 s for Linux containers.
  - **Environment:**
    - `-e NAME` without `=` copies the CLI's own value of `NAME` into the container, and leaves it unset if the CLI has none.
    - The CLI also adds the proxy variables from its config file (`run.go`).
- **PID 1 and `--init`:**
  - **The signal rule:** pid_namespaces(7) says a namespace's init receives a signal only if it has a handler for it. SIGKILL and SIGSTOP from an ancestor namespace are forced. The `docker run` reference says a PID 1 process "ignores any signal with the default action".
  - **Docker's `--init`:**
    - The daemon runs `/sbin/docker-init -- <cmd>`, bind-mounted read-only from the daemon's host. It does this only in a private PID namespace (`oci_linux.go`).
    - Docker says it is "backed by tini".
    - The image needs nothing for it. In this base image, `/sbin` is a symlink to `usr/sbin`.
  - **Podman's `--init`:** it mounts `container-init` at `/run/podman-init`. "Mounting over `/run` breaks container execution."
  - **Grimoire's handlers** (`cargo tree` run 2026-11-17):
    - `agent-grimoire-mcp`'s `shutdown_signal` waits for SIGINT and, on Unix, SIGTERM through tokio. The stdio transport also stops on stdin EOF.
    - `grimoire-mcp-project-docs` has tokio 1.54.1 and signal-hook-registry 1.4.8.
    - `agent-grimoire` and `grimoire-tool-secret-guard` have no tokio, signal-hook or ctrlc, and install no handlers.
- **Hardening flags:**
  - **`--read-only`:** "prohibiting writes to locations other than the specified volumes".
  - **Capabilities:** `--cap-add` and `--cap-drop` accept `ALL`.
  - **`--security-opt no-new-privileges`:** "commands that raise privileges such as `su` or `sudo` no longer work."
  - **`--network none`:** creates only `lo`, with `127.0.0.1/8`. "No IPv6 loopback address is configured".
  - **`--user`:**
    - A numeric uid must be in 0 to 2147483647 and needs no passwd entry.
    - A name must exist in the container.
  - **`HOME`:** Docker sets it "based on the value of `USER`". runc v1.5.3 reads it from the container's `/etc/passwd`, and uses `/` for a uid with no entry.
  - **Working directory:**
    - The default is the image's `WorkingDir`, else `/`. `-w` overrides it and creates the directory if it is missing.
    - runc changes to the directory before switching user, and tries again afterwards only on EPERM. So a uid other than 65532 can start inside `/home/nonroot` (0700) without permission to read it. Grimoire's image sets `WORKDIR /repo` instead.
  - **`--pull`:** defaults to `missing`, which uses the cached image when one exists. An image is referenced as `IMAGE[:TAG][@DIGEST]`.
- **Checked locally (2026-11-17, rootless Docker 29.9.3):** on Grimoire's linux/amd64 image, all under `--network none --read-only --cap-drop ALL --security-opt no-new-privileges`:
  - `grimoire --version`, and `--version` of both asset binaries started with `--entrypoint /opt/grimoire/bin/<name>`;
  - `init`, `add … --apply` and `verify` on a bind-mounted repository, with `--user 0:0`, which rootless Docker maps to the host user;
  - an MCP `initialize` over stdio: `docker run -i … --mount …,ro --entrypoint /opt/grimoire/bin/grimoire-mcp-project-docs … --root /repo`;
  - `grimoire-tool-secret-guard scan` on stdin.
- **Ports and networks:**
  - **Publishing without an address:** `-p H:C` publishes on all interfaces. This "also applies if you configured UFW to block this specific port".
  - **Publishing to localhost:**
    - With `127.0.0.1` or `::1`, "only the Docker host can access the published container port".
    - Before Engine 28.0.0, hosts on the same L2 segment could reach ports published to localhost (moby#45610).
  - **Bridge networks:**
    - "all ports of containers on bridge networks are accessible from the Docker host and other containers connected to the same network", whether published or not.
    - Containers started without `--network` join the default bridge.
  - **Loopback:** each container has its own loopback. A `127.0.0.1` listener can be reached only from the same network namespace, for example with `--network container:<id>`.
  - **Host networking:**
    - It ignores `-p`.
    - It is supported by Engine on Linux, and by Docker Desktop 4.34 and later as an opt-in setting. Since Desktop 4.35.0 it no longer requires sign-in.
    - On Desktop, processes "cannot bind to the IP addresses of the host", it works at layer 4, and it doesn't work with Enhanced Container Isolation.
    - Desktop's settings reference lists "Enable host networking" for Mac only.
  - **Docker Desktop `-p`:** it listens on `0.0.0.0` by default and can be restricted to `127.0.0.1`.
  - **Rootless Docker:**
    - A container's `IPAddress` lives in RootlessKit's network namespace and can't be reached from the host.
    - `-p` doesn't propagate source addresses by default.
    - Until Engine v29.5, `--net=host` was namespaced inside RootlessKit, so its ports couldn't be reached from the host.
- **Bind mounts:**
  - **Relative sources:** the CLI makes a relative source that starts with `.` absolute, against the CLI's own cwd (`filepath.Abs`). This applies to both `-v` and `--mount`.
  - **Missing sources:** `-v` creates a missing host source as a directory. `--mount` fails with "bind source path does not exist" unless `bind-create-src` is set.
  - **Read-only:** bind mounts are read-write unless `ro`/`readonly` is set. Read-only mounts are recursively read-only only on kernel 5.12 and later.
  - **SELinux:** `z` and `Z` relabel the host path and work only with `-v`. "It is not possible to modify the SELinux label using the `--mount` flag."
- **Ownership:**
  - **Rootless Docker:**
    - Container uid 0 is the host user, and uid n (n ≥ 1) maps to subuid + (n − 1).
    - "files owned by your host user appear as owned by `root` inside the container."
  - **Rootless Podman:**
    - The default `--userns` is `host`, which maps the host user to container root, unless `PODMAN_USERNS` or `containers.conf` sets it.
    - `keep-id` maps the user's UID:GID unchanged and runs init as that UID. That overrides the image `USER` unless `--user` is set. `keep-id:uid=,gid=` choose the IDs inside the container.
    - `:U` changes "recursively the owner and group of the source volume" on the host.
  - **Docker Desktop:**
    - From Windows, shared volumes get 0777, and that can't be configured.
    - On Mac VirtioFS, 4.80.0 stopped persisting ownership changes: "Calls to `chown` will succeed, but `stat` will not be affected". 4.96.0 fixed ownership and transient `0:0` reporting on Apple Virtualization Framework + VirtioFS bind mounts.
    - Desktop for Linux needs no ID-mapping setup from 4.35.
    - The default virtual file shares are `/Users`, `/Volumes`, `/private`, `/tmp` and `/var/folders`. Other paths give "Mounts denied".
- **MCP 2026-07-28:**
  - **Streamable HTTP:** servers MUST validate `Origin`, with 403 when it is present and invalid. When local, they SHOULD bind only to localhost, and they SHOULD authenticate.
  - **stdio shutdown:**
    - The client SHOULD close the server's stdin, wait, and then force termination (SIGTERM, then SIGKILL, on POSIX).
    - Servers SHOULD exit promptly on EOF, which is "the only portable" graceful signal.
    - A client SHOULD restart a server that exits unexpectedly.
- **What Grimoire's binaries rely on** (source read, not run):
  - **Common:**
    - Outside tests, none spawns a process. The only `Command::new` is `mkfifo` in a `#[cfg(test)]` module.
    - None uses git, reads `HOME`, uses a temp directory or links TLS. No binary's tree has rustls, openssl, native-tls or an HTTP client.
  - **`grimoire`:**
    - It reads its cwd at start, and opens the project at `-C DIR` (resolved against the cwd) or at the cwd.
    - It is interactive only when stdin is a terminal and `CI` is unset or empty.
    - It reads `PATH` for `doctor`, and for the `plan/hook-binary-missing` warning that `plan`, `apply`, `update` and the other planning commands can emit. In a container that is the container's `PATH`, so Grimoire's image keeps the asset binaries off it, in `/opt/grimoire/bin/` (ADR-0011).
    - Writes are atomic: a temp file in the destination directory gets the old file's permissions and is then renamed. The writing uid owns the result.
  - **`grimoire-mcp-project-docs`:**
    - It takes `--root DIR`, which isn't checked for absoluteness, or else `CLAUDE_PROJECT_DIR`, which must be absolute.
    - `serve-http` defaults to `127.0.0.1:8931` and `/mcp`. It speaks plain HTTP/1 (hyper `http1::Builder`) with no TLS.
    - `serve-http` needs either `--auth-resource`, `--auth-issuer` and `--auth-jwks`, or `--no-auth`.
  - **`HttpServer::bind`:**
    - It refuses to run without authorization on a non-loopback address, and `0.0.0.0` isn't loopback.
    - The `Host` allowlist is built from the bound address; `localhost`, `127.0.0.1` and `[::1]` with the port, on a loopback bind only; the resource URI's host, with and without the default port; and each `--allowed-host`.
    - Requests without `Origin` pass. The origin allowlist is empty by default.
    - With auth, the MCP path is the resource URI's path. `--auth-resource` must be `https://`, or `http://` for `localhost`, `127.0.0.0/8` and `[::1]`.
  - **JWKS:**
    - The file is re-read at most every 5 s, and only when its mtime changes.
    - It must be a regular file (symlinks are followed) of at most 1 MiB.
    - Deleting it revokes every key. An unreadable or unparsable file keeps the previous keys.
  - **`grimoire-tool-secret-guard`:**
    - It reads the hook event on stdin.
    - For Write, Edit, MultiEdit and NotebookEdit, it reads the file at `file_path` or `notebook_path` exactly as given, if it is a regular file and not a symlink. It then exempts secrets that the file already holds.
- **Claude Code:**
  - **`CLAUDE_PROJECT_DIR`:**
    - Claude Code sets it in a stdio server's environment, not in its own.
    - In the `command` or `args` of a project `.mcp.json` entry, and now also of a local- or user-scope entry in `~/.claude.json`, it therefore needs a default such as `${CLAUDE_PROJECT_DIR:-.}`.
    - Plugin MCP configs substitute it directly.
    - Stdio servers have received it since 2.1.139.
  - **Expansion:**
    - `command`, `args`, `env`, `url` and `headers` are expanded.
    - An unset variable with no default stays literal, with a warning. An empty value counts as set.
    - `MCP_TIMEOUT` (startup) defaults to 30000 ms.
  - **Stopping a server** on macOS and Linux: SIGINT, then SIGTERM 100 ms later, then SIGKILL after a further 400 ms.
  - **Hooks:**
    - Exec-form hooks substitute `${CLAUDE_PROJECT_DIR}` into each `args` element as a plain string.
    - In a worktree, `${CLAUDE_PROJECT_DIR}` stays at the main checkout, while the hook input's `cwd` follows Claude.
  - **Working directory:** the docs give one only for `headersHelper`.
- **Codex:**
  - **0.169.1:** the `rust-v0.169.0...rust-v0.169.1` compare (4 commits) touches only model-provider, `core/src/client.rs`, model and TUI files. CODEX-MCP-P4 therefore holds: no `${VAR}` interpolation, an emptied environment plus an allowlist, and the `env_vars` and `cwd` keys.
  - **Working directory:** without `cwd`, a local stdio server starts in the local environment's cwd, falling back to the session configuration's cwd.
  - **Process group:** servers are spawned in a new process group.
  - **Startup timeout:** `startup_timeout_sec` defaults to 10 (docs).
- **Cursor** (unchanged from CURSOR-MCP):
  - **Interpolation:** `command`, `args`, `env`, `url` and `headers` resolve `${env:NAME}`, `${userHome}`, `${workspaceFolder}` ("the folder that contains `.cursor/mcp.json`"), `${workspaceFolderBasename}`, `${pathSeparator}` and `${/}`.
  - **stdio fields:** the table marks `type: "stdio"` as required and lists `"docker"` as an example `command`.
  - **Working directory:** a server's cwd is still undocumented.
- **Gemini CLI v0.68.0:**
  - **Settings load (corrects GEMINI-SETTINGS):**
    - `resolveEnvVarsInObject` leaves an unset `$VAR` or `${VAR}` without a default as literal text.
    - Only `env` and `headers` get the second pass, which blanks unset variables (GEMINI-P4).
  - **Trust:** in an untrusted folder, a stdio server fails with "uses stdio transport but current folder is not trusted".
  - **Working directory:** `cwd` is passed only when configured. Otherwise Node's `spawn` inherits the CLI's cwd.
  - **Environment:**
    - The inherited environment always keeps `PATH`, `HOME`, `LANG`, `SHELL`, `TMPDIR`, `USER`, `LOGNAME` and `TERM`.
    - It drops names matching `TOKEN`, `SECRET`, `PASSWORD`, `PASSWD`, `KEY`, `AUTH`, `CREDENTIAL`, `CREDS`, `PRIVATE` or `CERT`. `DOCKER_CERT_PATH` is dropped; `DOCKER_HOST`, `DOCKER_CONTEXT` and `DOCKER_CONFIG` match none of these. Under strict sanitization, which v0.68.0 turns on when `GITHUB_SHA` is set or `SURFACE=Github` (so in GitHub Actions), a stdio server inherits only the always-allowed names (`PATH`, `HOME`, `LANG`, `SHELL`, `TMPDIR`, `USER`, `LOGNAME`, `TERM`, `COLORTERM` and a few platform names), `GEMINI_CLI_*`, `GIT_CONFIG_*` and the names in `security.environmentVariableRedaction.allowed`; so `DOCKER_HOST`, `DOCKER_CONTEXT` and `DOCKER_CONFIG` are dropped too, unless they are set in the entry's `env` or allow-listed (packages/core/src/services/environmentSanitization.ts and src/tools/mcp-client.ts at v0.68.0, checked in the Phase 6 review).
  - **Stopping a server:** SDK 1.23.0's `StdioClientTransport.close()` aborts the child through an `AbortSignal`, and Node's `killSignal` for that defaults to `'SIGTERM'`. It doesn't close stdin first.
  - **Docs example:** the Docker example mounts `"-v", "${PWD}:/workspace"`.
- **VS Code and Copilot** (COPILOT-MCP unchanged):
  - **Native workspace-root `.mcp.json`:**
    - It documents `${NAME}` and `${NAME:-default}` in stdio `command`, `args` and `env`, and no workspace-folder variable.
    - VS Code's configuration form (the editor UI) for the workspace-root `.mcp.json` offers no `cwd` field (the user form does) and rejects `${...}`: edit the file directly ("Other Properties"). The docs say a form restriction doesn't mean the runtime can't read the reference; the file format has `cwd` (COPILOT-MCP).
  - **Legacy `.vscode/mcp.json`:**
    - It supports predefined variables such as `${workspaceFolder}`.
    - Its `cwd` "Defaults to the workspace folder when run in a workspace".
    - The docs say not to use `-d` with Docker stdio servers.
  - **Copilot CLI `/mcp add`:**
    - "The `PATH` variable is automatically inherited from your environment. All other environment variables must be configured here."
    - Its example is `docker run -i --rm -e GITHUB_PERSONAL_ACCESS_TOKEN ghcr.io/github/github-mcp-server`.
  - **docker CLI:**
    - It finds its config directory through `os.UserHomeDir`, falling back to an NSS lookup outside Windows.
    - It reads `DOCKER_HOST`, `DOCKER_CONTEXT` (which overrides `DOCKER_HOST`), `DOCKER_CONFIG` and `DOCKER_CERT_PATH`.
- **Registry and releases:**
  - **Names:** repository names follow `[a-z0-9]+((\.|_|__|-+)[a-z0-9]+)*(\/…)*` (distribution-spec v1.1.1), so they are lowercase.
  - **GHCR visibility:**
    - The access-control page says a package first published under a personal account is private. Other GitHub pages conflict (REGISTRY-P6).
    - "Once you make a package public, you cannot make it private again."
  - **GHCR access:** a package linked to a repository before its first publish, for example with the `org.opencontainers.image.source` label, inherits the repository's access permissions but not its visibility. The linked repository's workflows then get access.
  - **Anonymous pulls:**
    - For the public `github/github-mcp-server`, the anonymous token and the `latest` manifest both returned 200.
    - The token endpoint returned 403 for `jabbott-iii/grimoire`, which doesn't exist yet.
  - **Immutable releases:** the title and notes stay editable. The release attestation covers the tag, the commit SHA and the assets.
- **UNVERIFIED:**
  - **Not exercised:**
    - The local checks above used rootless Docker only. Not checked: `--init` with `--read-only` on this base, `-p`, a `--user` other than `0:0`, an arm64 image, and any rootful Docker, Docker Desktop or Podman run.
    - That StdinOnce gives the server EOF after the CLI is SIGKILLed comes from source only.
  - **Client behaviour:**
    - A stdio server's working directory in Claude Code (project `.mcp.json`), Copilot CLI, the VS Code native harness and Cursor, including whether Cursor honours `cwd`.
    - Whether Claude Code passes its whole environment (for example `DOCKER_HOST`) to stdio servers.
    - How Codex stops a stdio server.
    - Whether VS Code resolves `${workspaceFolder}` in a legacy `.vscode/mcp.json` before forwarding it.
  - **Docker Desktop:**
    - What owner a non-root uid sees on Mac VirtioFS and Windows bind mounts.
    - The uid mapping on Desktop for Linux.
    - Whether a `127.0.0.1` listener in a host-network container can be reached from the host.
    - Which platforms have "Enable host networking".
    - The "bind to localhost by default" setting that the networking page mentions but the settings page doesn't list.
  - **Mounts and networks:**
    - Whether a single-file bind mount sees a host-side atomic replace.
    - Whether `-p '[::1]:H:C'` works on an IPv4-only bridge.
  - **Versions in use:**
    - Which tini the docker-ce packages ship as `docker-init`, and which runc a given engine uses. The runc facts above are from v1.5.3.
    - Whether Podman 4.9.3 on the runners matches the v6.2.0 option docs.
  - **Windows and runc details:**
    - The docker CLI on Windows when only `PATH` is inherited.
    - `${PWD}` on Windows shells.
    - Whether runc drops capabilities before or after its first chdir.
  - **Our own image:** pull behaviour for `ghcr.io/jabbott-iii/grimoire`.

### SIGN-P6: signing from a reusable workflow
Sources: https://github.com/sigstore/fulcio at v1.10.1 (commit `aef42c7754e1`: `config/identity/config.yaml`, `pkg/certificate/extensions.go`, `docs/oid-info.md`) and the v1.10.0...v1.10.1 compare; https://github.com/github/docs at `49a012fa50b4` (`content/actions/reference/security/oidc.md`, `content/actions/how-tos/secure-your-work/security-harden-deployments/oidc-with-reusable-workflows.md`, `content/actions/reference/workflows-and-actions/{reusing-workflow-configurations,workflow-syntax,contexts,events-that-trigger-workflows}.md`, `content/actions/how-tos/reuse-automations/reuse-workflows.md`, `content/actions/how-tos/manage-workflow-runs/re-run-workflows-and-jobs.md`, `content/actions/how-tos/secure-your-work/use-artifact-attestations/increase-security-rating.md`, `content/actions/concepts/security/workflow-dependency-lockfiles.md`, `content/actions/how-tos/administer/control-workflow-execution.md`, `content/actions/reference/limits.md`, `data/reusables/actions/{reusable-workflow-calling-syntax,partial-reruns-with-reusable,workflow-ref-description,forked-write-permission,github-token-available-permissions}.md`, `data/reusables/actions/jobs/section-using-environments-for-jobs.md`); https://github.blog/changelog/2026-07-30-reference-same-repository-actions-with-self-repository-syntax/; https://github.com/sigstore/cosign at v3.1.4 (`cmd/cosign/cli/options/certificate.go`, `cmd/cosign/cli/verify.go`, `cmd/cosign/cli/verify/{verify,verify_blob}.go`, `pkg/cosign/{verify,verify_bundle,certextensions}.go`, `go.mod`); https://github.com/sigstore/sigstore-go at v1.3.0 (`22d3691c7b8e`: `pkg/fulcio/certificate/{extensions,summarize}.go`, `pkg/verify/{certificate_identity,signed_entity}.go`); https://github.com/cli/cli at v2.103.3 (`pkg/cmd/attestation/verify/{verify,policy}.go`, `pkg/cmd/attestation/verification/{extensions,sigstore}.go`, `go.mod`); https://github.com/actions/attest at `1e69f48acb82` (v4.2.2: `README.md`, `dist/index.js`); https://github.com/actions/download-artifact/blob/9000827ccba6/README.md; https://github.com/zizmorcore/zizmor at v1.32.0 (`crates/zizmor/src/audit/{self_repository,unpinned_uses,excessive_permissions,undocumented_permissions,anonymous_definition,secrets_inherit,secrets_outside_env,concurrency_limits,template_injection}.rs`, `crates/zizmor/src/models/workflow.rs`, `crates/github-actions-models/src/common.rs`, `docs/{audits,usage,release-notes}.md`); https://github.com/github/gh-actions-lock at v0.4.1 (`04b7a1a4d00b`: `internal/workflowfile/workflowfile.go`); https://github.com/github/actions-lockfile/blob/go/v0.0.6/go/pkg/lockfile/uses.go; https://github.com/rhysd/actionlint at v1.7.12 (`rule_workflow_call.go`, `docs/{config,usage}.md`) and issue #711; real certificates from the GitHub attestations API (`zizmorcore/zizmor` v1.30.0 `sha256:ec8c95cd…` and v1.32.0 `sha256:bb110cc7…`; `astral-sh/uv` 0.13.9 image index `sha256:d3b601f0…`) and from https://github.com/google/go-containerregistry/releases/download/v0.22.1/multiple.intoto.jsonl (SHA-256 `fb042be9…`); the workflows that made them (`zizmorcore/zizmor` at v1.30.0 and v1.32.0, `astral-sh/uv` at `6b2d39dcc390`, `google/go-containerregistry` at v0.22.1); `GET /repos/jabbott-iii/Grimoire` and `GET /repos/astral-sh/uv/actions/runs/44053076542`.
Checked 2026-11-17. Status: Verified, except the gaps listed. The research ran no cosign, gh, zizmor, actionlint or gh-actions-lock binary. Certificates were decoded with Python cryptography 41.0.7 and OpenSSL 3.0.13.

- **Pins:**
  - Fulcio v1.10.1 (2026-11-16) is the newest tag. Of the 36 files changed since v1.10.0, none is under `config/`, `pkg/certificate/`, `pkg/identity/` or `docs/oid-info.md`.
  - The rest are unchanged from SUPPLY-CHAIN-P5, CI-PINS and ZIZMOR: cosign v3.1.4 and gh 2.103.3, which both use sigstore-go v1.3.0; `actions/attest` v4.2.2; zizmor 1.32.0; actionlint 1.7.12; gh-actions-lock v0.4.1.
  - The repository is public. Its ID is `1460666977` and its owner ID `84321953`.
- **Token claims in a called job:**
  - The standard claims describe the caller: `repository`, `ref`, `sha`, `workflow` (the caller's `name:`), `event_name`, `workflow_ref` and `workflow_sha` (the top-level workflow), and `run_id`.
  - `job_workflow_ref` and `job_workflow_sha` describe the called file.
  - With a local `./` or `$/` call, `job_workflow_ref` carries the caller run's ref, and `job_workflow_sha` equals the run's commit. Real examples: zizmor v1.30.0 (`./`, tag push) and uv 0.13.9 (`$/`, nested twice, `workflow_dispatch` on `refs/heads/main`).
  - `environment` and the environment form of `sub` come from the job inside the called workflow. A called job without an environment has no `environment` claim.
  - Inside the called workflow, the `github` context is the caller's. `job.workflow_ref`, `job.workflow_sha`, `job.workflow_repository` and `job.workflow_file_path` name the called file. A job-level `if:` may use `github`, `needs`, `vars` and `inputs`, but not `job`.
- **The certificate:**
  - The SAN and OID 1.9 are `https://github.com/{job_workflow_ref}`, and 1.10 is `job_workflow_sha`: the called workflow.
  - OID 1.18 is `https://github.com/{workflow_ref}` and 1.19 is `workflow_sha`: the top-level caller.
  - Fulcio v1.10.1 still issues the deprecated 1.1 to 1.6 as raw strings, whenever they are non-empty. Certificates issued on 2026-11-02 and 2026-11-17 carry them.
- **Which OIDs name which workflow** (real certificates, same-repository calls):
  - The called file: the SAN, 1.9 and 1.10.
  - The caller or the run: 1.2 (event), 1.3 (sha), 1.4 (the caller's `name:`), 1.5 (repository), 1.6 (ref), 1.12 to 1.17, 1.18 (top-level file and ref), 1.19, 1.20, 1.21 (the caller's run, which holds the called jobs) and 1.22.
  - The called job's own settings: 1.11 (its runner) and 1.23 (its environment).
  - 1.24 is the raw `sub`, unchanged by the call.
  - An intermediate workflow of a nested call appears nowhere.
- **Expected values for `sign.yml`** (derived, not observed):
  - SAN and 1.9: `https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main`.
  - 1.18: `…/release.yml@refs/heads/main`.
  - 1.3, 1.10, 1.13 and 1.19: `GITHUB_SHA`.
  - 1.4: `Release`. 1.5: `jabbott-iii/Grimoire`. 1.6 and 1.14: `refs/heads/main`. 1.2 and 1.20: `push` or `workflow_dispatch`.
  - 1.15: `1460666977`. 1.17: `84321953`.
  - 1.23 is present only if the `sign.yml` job sets an environment.
- **Anyone can get the same SAN:**
  - A public repository's reusable workflow can be called from any repository on github.com. Access settings exist only for private and internal called repositories.
  - A cross-repository call puts the called repository's file and ref in the SAN, with the written ref normalised (`@v2.1.0` becomes `@refs/tags/v2.1.0`). The docs' example shows a branch as `@refs/heads/main`. Every caller field (1.3 to 1.6 and 1.12 to 1.19) names the calling repository, and 1.10 is the called repository's commit. go-containerregistry's certificate for slsa-github-generator shows exactly this.
  - So once `sign.yml` has `on: workflow_call`, a call of `jabbott-iii/Grimoire/.github/workflows/sign.yml@main` from another repository gets Grimoire's exact SAN over bytes that caller chooses.
  - A call by SHA or tag gives `…@<sha>` or `…@refs/tags/<tag>`. Release tags make the latter callable.
  - `actions/attest` stores attestations on `GITHUB_REPOSITORY`, which is the caller's repository.
- **Pull requests and other workflows in this repository:**
  - Any workflow of the repository can call a local reusable workflow, on any event.
  - `pull_request` runs have `GITHUB_REF` `refs/pull/<N>/merge`. A `closed` run of a merged PR has the target branch's ref.
  - Fork `pull_request` runs get write permissions downgraded to read, and `id-token` has only `write` or `none`, so they get no OIDC token.
  - Another workflow on `main` that calls `sign.yml` differs from a release only in 1.18.
- **cosign v3.1.4:**
  - `--certificate-identity` is an exact comparison with the SAN, which here is the called workflow.
  - `--certificate-github-workflow-{trigger,sha,name,repository,ref}` compare OIDs 1.2 to 1.6, exactly and case-sensitively, and only when set. `-ref` is the git ref, not a workflow path. `-name` is the caller's mutable `name:`.
  - `verify` and `verify-blob` with a new-format bundle use sigstore-go `CompareExtensions`; legacy signatures use `validateCertExtensions`. Both fail closed when an expected OID is missing.
  - The flags aren't deprecated and print no warning. Only `--certificate`, `--ca-intermediates`, `--ca-roots`, `--certificate-chain` and `--sct` are.
  - There is no flag for 1.9 to 1.24, so cosign can't pin the caller file, the environment, or the repository or owner IDs.
- **gh 2.103.3:**
  - The help says the reusable workflow is the signer and needs `--signer-workflow` or `--signer-repo`. Its example is `gh attestation verify example.bin --owner github --signer-repo actions/example`.
  - `--repo O/R` fetches from that repository's API and checks 1.12 and 1.16, case-insensitively. `--owner` checks 1.16 only.
  - `--signer-repo` matches the SAN with `(?i)^https://github.com/<repo>/`.
  - `--signer-workflow` matches the SAN with an anchored, case-sensitive pattern. The ref must match exactly when the value ends in `@refs/…` or `@<hex>`; otherwise any ref is accepted. `--cert-identity` compares the SAN exactly. These four flags are mutually exclusive.
  - `--source-ref` checks 1.14, case-sensitively. `--deny-self-hosted-runners` checks 1.11. `--signer-digest` checks 1.10 and `--source-digest` 1.13.
  - No flag checks 1.18 or 1.23. `--format json` exposes `.verificationResult.signature.certificate.{buildConfigURI,sourceRepositoryIdentifier,sourceRepositoryOwnerIdentifier}`, so jq can pin the caller file and both IDs. sigstore-go v1.3.0 parses nothing after 1.22.
  - GitHub's SLSA L3 guide uses `--owner` or `--repo` for the caller, plus `--signer-repo` or `--signer-workflow` for the reusable workflow. It moves the build steps themselves into the reusable workflow.
- **`actions/attest` in a called job:**
  - Both the calling job and the called job need `attestations: write`, `contents: read` and `id-token: write`, plus `packages: write` for images.
  - In the provenance, `externalParameters.workflow` (path and ref) comes from `workflow_ref` and `ref`, so it names the caller. `runDetails.builder.id` is `https://github.com/{job_workflow_ref}`, the called file. `resolvedDependencies` and `invocationId` describe the caller's repository and run.
  - Nothing else differs from a top-level job.
- **Calling:**
  - `$/.github/workflows/F` is recommended (runner 2.336.0 or later, github.com only). `./.github/workflows/F` also works. Both take no `@ref` and no expressions, and load the called file from the caller's commit.
  - A calling job may use only `name`, `uses`, `with`, `secrets`, `strategy`, `needs`, `if`, `concurrency`, `permissions` and `cache-mode`. That excludes `environment`, `runs-on`, `env`, `outputs` and `timeout-minutes`.
  - Without job-level `permissions`, the called workflow gets "the default permissions for the `GITHUB_TOKEN`". A called or nested workflow can only keep or reduce the permissions it is passed. `github.token` is granted to the called workflow automatically.
- **Data in and out:**
  - Inputs are `boolean`, `number` or `string`. An undeclared input is an error. `with` may use the `github`, `needs`, `strategy`, `matrix`, `inputs` and `vars` contexts. Inputs reach the called workflow through `inputs.*`, not as environment variables.
  - The caller's workflow-level `env` isn't propagated. `needs` holds only direct dependencies.
  - `on.workflow_call.outputs.<id>.value` maps a called job's output. Every downstream caller job reads it as `needs.<calling job>.outputs.<id>`. From a matrix, only the last successful non-empty value survives.
  - `strategy` may use `inputs`. `download-artifact` with `artifact-ids` reads the current run, which is the caller's.
- **Runs:**
  - Re-runs keep `GITHUB_SHA`, `GITHUB_REF` and the original actor. Partial re-runs also re-run the jobs that depend on them, and the others keep their outputs. A run allows at most 50 re-runs.
  - Sharing a `jobs.<id>.concurrency` group with `cancel-in-progress` between caller and callee cancels the caller.
  - Limits: 10 levels of workflows, and 50 unique reusable workflows per workflow file. A run counts as one for the queue limit.
- **Environments:**
  - An environment can be set only on the job inside the called workflow. `environment.name` may use `inputs`.
  - All protection rules must pass before the job is sent to a runner. `deployment: false` doesn't work with custom protection rules.
- **Guard** (uv's `publish-docker-image.yml` does the same): `if: github.repository_id == '1460666977' && github.workflow_ref == 'jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main'` on every `sign.yml` job keeps foreign and other in-repository callers from minting a token.
- **zizmor 1.32.0** (Grimoire runs `persona: pedantic`, so every finding below blocks):
  - `self-repository` (since v1.30.0) flags `./` on calling jobs: Low severity, regular persona, with an unsafe fix to `$/`.
  - `unpinned-uses` ignores every local `uses:`, `./` and `$/` alike.
  - `excessive-permissions`: a calling job without permissions is a regular finding when the workflow sets none at the top. A reusable-only workflow, or a job in it, without permissions is a pedantic finding.
  - `undocumented-permissions` covers calling jobs.
  - `anonymous-definition` needs `name:` on the workflow and on normal jobs.
  - `secrets-inherit` flags `secrets: inherit` (Medium).
  - `concurrency-limits` and `secrets-outside-env` skip reusable workflows.
  - `template-injection` rates `inputs.<string>` High and boolean or number inputs Low (pedantic).
  - Inline ignores take the form `# zizmor: ignore[rule]`.
- **actionlint 1.7.12:**
  - It accepts only `./path` or `owner/repo/path@ref`, so `$/…` is an error (issue #711, open since 2026-07-30).
  - For `./` calls it checks the inputs and secrets of the call.
  - It has no line-scoped ignore, only `-ignore` and `paths.<glob>.ignore` message regexes.
- **Lockfiles:**
  - gh-actions-lock skips a job-level `./…yml` and records a bare `$/…` as pinned; `$/…@ref` is an error.
  - It scans every `*.yml` in `.github/workflows`, so `sign.yml` gets its own entry with its actions.
  - Lockfiles and "Require lockfile" don't cover `jobs.<id>.uses`. A called workflow is covered by its own repository's lockfile, and "Require lockfile" blocks the caller's run if the called workflow isn't onboarded there.
- **Changed since the register:**
  - COSIGN, ATTEST and SUPPLY-CHAIN-P5 assume a top-level signer, where 1.9 equals 1.18. With `sign.yml` the SAN is `…/sign.yml@refs/heads/main`, and 1.18 keeps `release.yml`.
  - The SAN alone no longer identifies Grimoire, so verifiers must also pin the repository (1.5, or 1.12 through `--repo`) and an exact ref.
  - Releases signed before the switch would keep the `release.yml` identity. Grimoire made none: its first release is signed by `sign.yml`.
- **UNVERIFIED:**
  - `job_workflow_ref` for a `pull_request` run calling `sign.yml` (expected `…@refs/pull/<N>/merge`). No real certificate was found.
  - That a cross-repository `@main` call gives `@refs/heads/main`. Only the docs' example token shows it.
  - The literal 1.24 for Grimoire (expected `repo:jabbott-iii@84321953/Grimoire@1460666977:ref:refs/heads/main`).
  - What GitHub does when a called job asks for a permission the caller didn't grant.
  - What a calling job without `permissions` gets under a top-level `permissions: {}`.
  - Whether caller `defaults` propagate.
  - That a called job can't read `needs.<caller job>.outputs`. Inferred from the `needs` definition; not tested.
  - That the environment of a called job in an external caller's run belongs to the caller's repository.
  - How approval appears for a job inside a called workflow.
  - Whether a matrix inside a called workflow counts toward the 256-jobs-per-run limit.
  - Whether an external caller can use a fork-network (impostor) commit of `sign.yml`. Its SAN would end in `@<sha>` anyway.
  - Runtime lockfile enforcement for a local reusable workflow's own entry.
  - The Fulcio version deployed on public-good. Only its behaviour was observed.
  - Whether a later actionlint accepts `$/`.
  - Where cosign's own `sign-blob` bundles keep the certificate (the v0.3 attestation bundles use `verificationMaterial.certificate`).

---

## Phase 7 additions (checked 2026-11-17)

These facts came from a five-topic research workflow for the documentation pass and the contributor files: GitHub's community health files, community profile, PR templates, CODEOWNERS, and Grimoire's rulesets and labels (COMMUNITY-P7); issue forms, the template chooser and their schemas (ISSUE-FORMS-P7); private vulnerability reporting, security policies, advisories, CVEs and the Contributor Covenant (SECURITY-POLICY-P7); how one README renders on GitHub, crates.io, PyPI and npm and in the release archives (README-RENDER-P7); and SemVer, Cargo, release-plz and clap facts for the versioning policy and the CLI reference (VERSIONING-P7). Five researchers each took one topic, and five independent skeptics re-checked them against the primary sources. Of 264 facts, 242 were confirmed, 19 corrected and none refuted, and 3 could be neither confirmed nor refuted: community 57 of 60 (3 corrected), issue forms 38 of 45 (5 corrected, 2 unverifiable), security policy 41 of 42 (1 corrected), README rendering 50 of 54 (3 corrected, 1 unverifiable), and versioning 56 of 63 (7 corrected). The skeptics added 77 facts: 19, 15, 11, 15 and 17.

Corrected facts appear below in corrected form, unverifiable facts and the remaining gaps are marked UNVERIFIED, and facts the skeptics added are included where an implementer needs them. The research changed nothing on GitHub. It read docs and source, made read-only REST and GraphQL requests, fetched public pages without signing in, and sent probe text to GitHub's Markdown rendering API. The only local build commands were `cargo package` and `cargo package --list`, into the scratchpad (git status unchanged), and the installed Cargo man pages were read. Nothing was installed or published. The maintainer's approval of local tooling for Phase 7 is in PLAN.md's "before implementation" list, and was already recorded while the research ran: lychee v0.24.2 in the scratchpad, whose tarball matches the SHA-256 that `ci.yml` pins (LYCHEE), and check-jsonschema through `uvx`, for one check of the issue forms (ISSUE-FORMS-P7). Neither tool's results on the Phase 7 files are recorded here. The raw results are not committed. Where an entry below differs from an earlier entry, this one wins, and the older entry carries a "Re-checked 2026-11-17" note. The decisions these facts led to are in PLAN.md's "after the research" list.

### COMMUNITY-P7: community health files, the community profile, PR templates, CODEOWNERS, rulesets and labels
Sources: https://github.com/github/docs at `49a012fa50b4` (committed 2026-11-17T23:26:49Z): under `content/communities/`, `setting-up-your-project-for-healthy-contributions/{creating-a-default-community-health-file,about-community-profiles-for-public-repositories,adding-a-code-of-conduct-to-your-project,setting-guidelines-for-repository-contributors,adding-support-resources-to-your-project,adding-an-accessibility-page-to-your-repository}.md` and `using-templates-to-encourage-useful-issues-and-pull-requests/{about-issue-and-pull-request-templates,creating-a-pull-request-template-for-your-repository,configuring-issue-templates-for-your-repository,syntax-for-issue-forms}.md`; `content/pull-requests/reference/using-query-parameters-to-create-a-pull-request.md`; `content/get-started/writing-on-github/` (`about-tasklists.md`, `basic-writing-and-formatting-syntax.md`); under `content/repositories/`, `about-code-owners.md`, `about-readmes.md`, `managing-rulesets/available-rules-for-rulesets.md`, `managing-protected-branches/{about-protected-branches,managing-a-branch-protection-rule}.md` and `permission-levels-for-a-personal-account-repository.md`; `content/issues/using-labels-and-milestones-to-track-work/managing-labels.md`; `data/reusables/repositories/{valid-community-issues,rulesets-bypass-step,rulesets-branch-tag-bypass-optional-step,request-changes-tips,repo-rules-permissions}.md`, `data/reusables/pull_requests/security-changes-mergeability.md`, `data/reusables/gated-features/code-owners.md` and `data/variables/product.yml`; and commit `355a0f2df0db` (2026-08-28). https://github.com/github/rest-api-description at `c9c95fe34ced` (API version 2022-11-28). https://github.com/EthicalSource/contributor_covenant at `8d00b9180c88` (branch `release`, 2026-11-12) and https://www.contributor-covenant.org/version/3/0/code_of_conduct/. Read-only REST requests on 2026-11-17: Grimoire's community profile, labels, rulesets 27435535 and 27435544, `rules/branches/main`, `branches/main`, `codeowners/errors`, `private-vulnerability-reporting` and collaborators (counted, not listed); `repos/jabbott-iii/.github`; `codes_of_conduct`; `apps/dependabot` and `apps/copilot-swe-agent`; and the community profiles of rust-lang/rust, rust-lang/cargo, github/docs and astral-sh/uv. Grimoire's Community Standards page, fetched without signing in. Status: Verified, except the gaps listed.

- **Where the files live:**
  - **Supported default files** (github.com): `ACCESSIBILITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, discussion category forms, `FUNDING.yml`, issue and PR templates with `config.yml`, `SECURITY.md`, `VULNERABILITY_REPORT.yml` or `.yaml`, and `SUPPORT.md`.
  - **`GOVERNANCE.md`:** the docs stopped listing it on 2026-08-28 (`355a0f2`, titled as a fix to the guidance; it announces no change in behaviour). No current page says GitHub shows it anywhere.
  - **Precedence:** where a file may live in several places, GitHub looks in `.github/`, then the root, then `docs/`, and uses the first it finds. READMEs and CODEOWNERS follow the same order, so a `.github/README.md` or `.github/CONTRIBUTING.md` would replace the root file.
  - **Fixed places:** `FUNDING.yml` and `VULNERABILITY_REPORT.yml` go in `.github/`, issue templates and `config.yml` in `.github/ISSUE_TEMPLATE/`, and discussion forms in `.github/DISCUSSION_TEMPLATE/`. The others may be in the root, `.github/` or `docs/`.
  - **Defaults:** they come from a public repository named `.github` owned by the account, and apply to its repositories that lack their own file of that type. Inherited files aren't in clones, packages or downloads, and a licence can't be a default. Issue-template inheritance is all or nothing.
  - **Grimoire inherits nothing:** `repos/jabbott-iii/.github` returns 404, signed in or not. Every community file has to be committed in Grimoire itself.
  - **Case:** the docs say names are case-insensitive only for `CONTRIBUTING`, PR templates (`.md` or `.txt`) and issue templates (`.md`; forms need `.yml`). For the other files they give only upper-case names.
  - **What GitHub shows:** a link to CONTRIBUTING when someone opens an issue or PR, plus a Contributing tab on the overview and a sidebar link for `CONTRIBUTING.md`; a link to SUPPORT on the new-issue page; and an Accessibility tab and an About link for `ACCESSIBILITY.md`. A README is truncated past 500 KiB.
- **Community profile** (public repositories only; the REST endpoint refuses forks):
  - **Grimoire's checklist** (2026-11-17) has 8 items: Description, README, Code of conduct, Contributing, License, Security policy, Issue templates and Pull request template. Only Description, README and License were marked as added. Accessibility is listed apart, as an additional file.
  - **What counts:** any code of conduct that isn't empty and doesn't say the project has none. Issue templates count only in `.github/ISSUE_TEMPLATE/`, with `name` and `about` (Markdown) or `name` and `description` (forms).
  - **The REST response:** `health_percentage`, `description`, `documentation`, `files` and `updated_at`, plus `content_reports_enabled` for organization repositories. `files` has no security-policy field.
  - **`files.issue_template` shows nothing useful:** it was null for rust-lang/rust, rust-lang/cargo, github/docs and astral-sh/uv, which all have `.github/ISSUE_TEMPLATE/`. The Community Standards page is where the Issue templates item shows.
  - **Grimoire's profile:** `health_percentage` is 42. GitHub detects the licence as Apache-2.0 from `LICENSE-APACHE` alone, so its licence display doesn't show `MIT OR Apache-2.0`. `documentation` points at `tree/master/docs`, although the default branch is `main`.
- **Codes of conduct:**
  - `GET /codes_of_conduct` lists only `django` and `contributor_covenant`, but `citizen_code_of_conduct` and `other` also resolve. GitHub's `contributor_covenant` template is version 2.0, with a contact placeholder to fill in.
  - Contributor Covenant 3.0 is at https://www.contributor-covenant.org/version/3/0/code_of_conduct/. `/version/3/0/` returns 200 with a meta refresh and a canonical link to that page; it isn't an HTTP redirect. The upstream repository's GitHub releases stop at 2.1 (2021). The 3.0 text has two placeholders for the adopter, for reporting and for enforcement, and its own attribution section states CC BY-SA 4.0 (GitHub detects the repository's licence as NOASSERTION). More in SECURITY-POLICY-P7.
- **PR templates:**
  - A single template goes in the root, `docs/` or `.github/` (for example `.github/pull_request_template.md`), and is used only from the default branch.
  - Several templates go in a `PULL_REQUEST_TEMPLATE/` subdirectory, and one is chosen with `?quick_pull=1&template=<file>` on a compare URL. An invalid parameter, or one the user lacks permission for, gives 404, and an over-long URL gives 414.
  - Markdown task lists (`- [ ]`) work in any comment. An item that starts with `(` needs a backslash before it. The separate tasklist blocks feature is retired, replaced by sub-issues.
  - Issue forms can't be used for PRs (ISSUE-FORMS-P7).
  - An `@username` mention notifies that user if they have read access to the repository.
- **CODEOWNERS:**
  - **Location:** `.github/`, the root or `docs/`, first found. Each branch has its own, and reviews follow the file on the PR's base branch, PRs from forks included. A file of 3 MB or more isn't loaded at all.
  - **Syntax:** gitignore-style patterns, without the `\#` escape, `!` negation or `[ ]` ranges. `#` starts a comment, also after the owners. The last matching pattern wins. A pattern's owners must be on one line, and a pattern with no owners leaves its files unowned. Paths are case-sensitive.
  - **Owners:** `@username`, `@org/team-name`, or an email address added to the user's account. Each needs write access, or isn't assigned. A user-owned repository has no teams.
  - **Errors:** lines with invalid syntax are skipped. `GET /repos/{owner}/{repo}/codeowners/errors` lists them with `line`, `column`, `kind`, `message` and `path`; its optional `ref` defaults to the default branch. Grimoire had no CODEOWNERS on `main` (2026-11-17), and the endpoint returned 404. Phase 7 adds `.github/CODEOWNERS`, and `release.md` §2 A reads this endpoint again after it merges.
  - **Drafts:** code owners aren't asked to review a draft PR; they are notified when it is marked ready for review.
  - **GitHub's advice:** keep CODEOWNERS in `.github/`, and make the repository owner the owner of `/.github/` or of the file.
  - Code owners and rulesets are available for public repositories on GitHub Free.
- **Grimoire's rulesets** (read 2026-11-17; anyone with read access can view active rulesets):
  - **`BestBranch`** (27435535) is active on all branches (`~ALL`). Its rules: restrict deletions, block force pushes, require a pull request, code scanning (CodeQL; alerts `errors`, security alerts `high_or_higher`) and code quality (`errors`). It has no status-check rule. `rules/branches/main` returns only these five.
  - **Its pull-request rule:** 1 approval, code-owner review, stale reviews dismissed on push, no last-push approval, the extra approval for unattributed Copilot PRs on, and the `merge` method only. The repository's settings allow merge, squash and rebase. If they stopped allowing merge commits, the ruleset would block every merge.
  - **`BestTag`** (27435544) blocks deletion and force pushes on all tags.
  - **Bypass:** both list RepositoryRole 5 and the Integrations 29110 (Dependabot), 946600 (unresolved) and 1143301 (Copilot SWE Agent), all in `always` mode, and the API reports that the owner can always bypass. So the owner and those apps can change any branch without a PR, a review or the code-scanning results.
  - **Probably no classic rule:** `branches/main` reports `protected: true` with `protection.enabled: false`. The protection endpoint itself answered 403 to the token used.
  - **People:** one collaborator, the owner, with the admin role. A personal-account repository has only an owner and collaborators, and collaborators can push; there are no Read or Triage roles. Outside contributors therefore work from forks.
  - **Observed 2026-11-17 and 2026-11-18 (Phase 7 review; read-only `gh api` requests), what feeds the code-scanning and code-quality rules:**
    - No workflow in `.github/workflows/` runs CodeQL. `actions/workflows` lists only CI, Release PR, Release, Security and the two dynamic Dependabot workflows, and no dynamic code-scanning workflow. The check suites on `b5bb0e6` come only from the `github-actions` and `dependabot` apps.
    - `code-scanning/default-setup` and `code-scanning/analyses` answered 403 to the token used, so whether CodeQL default setup or Code Quality is on couldn't be read (UNVERIFIED).
    - GitHub's own evaluation of the rules, when they refused release-plz's commit (RELEASE-PLZ), said "Waiting for Code Scanning results. Code Scanning may not be configured for the target branch."
    - So, as far as could be seen, nothing produces the results these rules wait for, and no push or pull request into a branch can meet `BestBranch` without the bypass (an inference).
- **Review mechanics:**
  - A PR's author can't approve it. With the owner as the only person with write access, the owner's own PRs can merge only through the bypass. Copilot approvals show in the sidebar but don't count toward requirements.
  - "Require review from Code Owners" is met by any one of a path's listed owners.
  - With stale reviews dismissed, a push that changes the diff removes approvals, and so does a change of merge base that brings in new changes.
  - When someone with write access requests changes, the PR can't merge until that person approves.
- **Ruleset options:**
  - `bypass_mode` is `always` (the default), `pull_request` (branch rulesets only: the actor must open a PR, but may bypass to merge it) or `exempt` (the rules are skipped with no bypass audit entry).
  - `required_approving_review_count` takes 0 to 10. With 0, a PR must still be opened.
  - Bypass actors work on a user-owned repository, as Grimoire's show. The docs' sentence that only organization repositories can have bypass actors is about classic branch protection.
  - The "Required reviewers" rule (path-based team approvals) needs teams, so a user-owned repository can't use it.
- **Labels:** GitHub creates 10 default labels in every new repository, and Grimoire has exactly those: `accessibility`, `bug`, `documentation`, `duplicate`, `enhancement`, `good first issue`, `help wanted`, `invalid`, `question` and `wontfix`. Anyone with write access can create labels. Issues labelled `good first issue` appear on the repository's `/contribute` page.
- **The security tab's name:** on github.com it is now "Security and quality" (the docs variable `prodname_security_and_quality_tab`, behind a feature that is on for github.com and GHEC). Grimoire's navigation shows that name.
- **Other settings:** Discussions are off (`has_discussions: false`), and the wiki is on.
- **UNVERIFIED:**
  - The `health_percentage` formula. It isn't documented, and 42 isn't 3 of 8. Every score observed fits counting 7 items for a user repository and 8 for an organization one, without the security policy, but that is an inference.
  - Whether the names `CODE_OF_CONDUCT`, `SECURITY`, `SUPPORT`, `ACCESSIBILITY` and `CODEOWNERS` are matched case-insensitively.
  - Whether templates kept only in `PULL_REQUEST_TEMPLATE/` fill a new PR without `template=`.
  - Whether the `User` bypass actor type works on a personal repository's ruleset.
  - That RepositoryRole 5 is the admin role (likely, undocumented), and which app Integration 946600 is.
  - Whether a classic branch protection rule exists on `main`.
  - Whether a code owner's approval also counts toward the required approval count.
  - Why `documentation` points at `tree/master/docs`.
  - Which templates GitHub's web code-of-conduct picker offers.
  - Whether `codeowners/errors` always answers 404 when there is no file.
- **Reporting conduct, and watching releases** (checked 2026-11-18, github/docs @ `49a012fa50b4`):
  - `content/communities/maintaining-your-safety-on-github/reporting-abuse-or-spam.md`: GitHub's "Report abuse" and "Report content" tools send a report to GitHub Support. Reporting content "directly to repository maintainers" needs reported content to be enabled, and `managing-how-contributors-report-abuse-in-your-organizations-repository.md` says that setting exists only for public repositories owned by an organization. So in Grimoire, a personal-account repository, the tools reach GitHub, not the maintainer.
  - `content/subscriptions-and-notifications/get-started/configuring-notifications.md` and `data/reusables/notifications-v2/custom-notification-types.md`: a repository's custom watch settings can select issues, pull requests, releases, security alerts or discussions.
- **Used by:** `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `.github/CODEOWNERS`, `.github/pull_request_template.md` and `README.md`.

### ISSUE-FORMS-P7: issue forms, the template chooser and their schemas
Sources: https://github.com/github/docs at `49a012fa50b4`: under `content/communities/using-templates-to-encourage-useful-issues-and-pull-requests/`, `syntax-for-issue-forms.md`, `syntax-for-githubs-form-schema.md` (`min_length` added in `e85b5032f062` on 2026-10-02, `upload` in `68894b667c27` on 2026-03-05), `common-validation-errors-when-creating-issue-forms.md`, `configuring-issue-templates-for-your-repository.md` and `about-issue-and-pull-request-templates.md`; `data/reusables/form-schema/`, `data/reusables/community/{issue-forms-beta,issue-forms-sample}.md` and `data/features/issue-form-upload.yml`; `content/issues/tracking-your-work-with-issues/using-issues/{managing-issue-types,adding-and-managing-issue-fields,creating-an-issue}.md`; the private-reporting pages `configure-for-a-repository.md` and `report-privately.md`; and the repository's own `.github/ISSUE_TEMPLATE/improve-existing-docs.yaml`. GitHub changelog posts of 2025-02-18, 2026-03-05 and 2026-11-11. https://github.com/SchemaStore/schemastore at `0a9268d745bc` (`src/schemas/json/github-issue-forms.json`, `src/schemas/json/github-issue-config.json`, `src/api/json/catalog.json`). https://github.com/python-jsonschema/check-jsonschema at tag 0.39.0 (`src/check_jsonschema/catalog.py`, `src/check_jsonschema/builtin_schemas/vendor/`, `.pre-commit-hooks.yaml`). https://github.com/github/issue-parser at `a52c1e098622` (`README.md`). GraphQL and REST reads on 2026-11-17: Grimoire's labels, issue types, `contactLinks`, `isSecurityPolicyEnabled` and private vulnerability reporting; forms, `contactLinks` and issues of github/docs (#45620, #46985), cli/cli, actionless/pikaur, vercel/next.js, zed-industries/zed, home-assistant/core and microsoft/PowerToys, and six repositories whose only chooser config is a `config.yaml`. https://github.com/orgs/community/discussions/159007 (no staff replies). Status: Verified, except the gaps listed. Issue forms and the form schema are still in public preview and "subject to change".

- **Files:**
  - Forms are YAML files in `.github/ISSUE_TEMPLATE/`, used only from the default branch. The docs give only `.yml` for forms, and only `config.yml` for the chooser config. File names are case-insensitive (stated for issue templates).
  - **`.yaml` forms work in practice:** github/docs has used `improve-existing-docs.yaml` since 2021-02-01, and its #46985, filed through that form on 2026-11-17, carries the label the form sets.
  - **`config.yaml` doesn't reach the API:** GraphQL `contactLinks` returned nothing for six repositories whose only chooser config is a `config.yaml` with 1 to 3 links, while it returns the links of a `config.yml`.
  - The legacy single file `ISSUE_TEMPLATE.md` was retired on 2025-03-30.
- **Top-level keys:**
  - `name`, `description` and `body` are required; `assignees`, `labels`, `title`, `type` and `projects` are optional. `assignees`, `labels` and `projects` take an array or a comma-separated string.
  - `name` must be unique across all templates, Markdown ones included, and longer than 3 characters, or the template isn't shown.
  - Unknown keys are rejected, at the top level and in elements and attributes. A string can't be empty or only whitespace. Keys such as `y`, `yes`, `true`, `on` and `off`, in any case, are forbidden. A `---` separator between the header and the body causes a "body cannot be empty" error.
  - The docs say to double-quote strings that contain `'`, but GitHub's own samples and its live form use unquoted apostrophes. Quoting matters where YAML would misread a value.
  - **`labels`:** a label that doesn't exist isn't created or added, and the issue is still submitted. actionless/pikaur's forms name a `triage` label that doesn't exist there, and its #927 was filed with only the `feature` label applied.
  - **`type`:** the syntax page still says issue types are set at the organization level. Since 2026-11-10/11 personal accounts have them too (GA; defaults Task, Bug and Feature; up to 25 types), and Grimoire has Task, Bug and Feature enabled. The docs don't describe `type` in a user-owned repository. In actionless/pikaur, which is user-owned, forms with lowercase `type: bug` and `type: feature` gave outside contributors' issues the Bug and Feature types, for example #921 (2026-09-07) and #927 (2026-11-18).
  - **`projects`:** entries are `OWNER/NUMBER`, and the person opening the issue needs write access to each project. Grimoire has no projects.
  - Issue fields (priority, effort, dates) can't be set from a template or a URL.
- **Body elements:**
  - Each element has `type` (`checkboxes`, `dropdown`, `input`, `markdown`, `textarea` or `upload`), an optional `id` (not on `markdown`), `attributes` and optional `validations`.
  - **`id`:** letters, digits, `-` and `_`, unique in the form. It is the field's key for URL prefills, and it never appears in the issue body.
  - **Labels:** unique across the form. Without an `id`, GitHub derives a reference from the label with Rails `parameterize`, so labels such as `Name?` and `Name???` clash ("Labels are too similar"). The docs' example of that error also shows an explicit `id` clashing with another field's derived reference.
  - The body needs at least one element that isn't `markdown`. Some words are forbidden in `input` and `textarea` labels, to keep credentials out of public issues; `password` is the only one documented.
  - **`markdown`:** `value` only. It is shown in the form and not submitted. A heading must be quoted, since YAML reads `#` as a comment.
  - **`textarea`:** `label`, `description`, `placeholder`, `value` and `render`, a Linguist language name. With `render` set, the answer becomes a code block, and the field loses attachments and Markdown editing. Validations: `required` and `min_length`.
  - **`input`:** `label`, `description`, `placeholder` and `value`. Validations: `required` and `min_length`.
  - **`min_length`:** a non-negative integer that blocks submission until the answer is that long. It reached the form-schema page on 2026-10-02, in a commit about private vulnerability report forms.
  - **`dropdown`:** `label`, `description`, `multiple` (default false), `options` (required, unique) and `default` (an index into `options`). `None` is reserved and added automatically when the dropdown isn't required, and with `default` set, `None` and `n/a` can't be options. Options that YAML reads as booleans must be quoted.
  - **`checkboxes`:** `label`, `description` and `options`, each option with `label` and `required` (default false). The element also takes `validations.required`.
  - **`upload`:** `label` and `description`. Validations: `required`, and `accept`, a comma-separated list of extensions (all supported types if left out). Limits: archives (`.zip`, `.gz`, `.tar.gz`) and documents (`.pdf`, `.docx`, `.xlsx`, `.pptx`) 25 MB, images 10 MB, video 100 MB, and text files (`.json`, `.py`, `.js`, `.ts`, `.log`, `.txt`, `.csv`) 25 MB.
  - **`required`:** the docs still say it works only in public repositories, while the 2025-02-18 changelog says it now works in private ones. Grimoire is public.
- **The chooser (`config.yml`):**
  - Keys: `blank_issues_enabled` (a boolean) and `contact_links` (each with `name`, `url` and `about`). It takes effect once it is on the default branch.
  - With `blank_issues_enabled: false`, people with write access or more still see a blank issue option marked for maintainers only. In a personal repository that means collaborators; everyone else sees only the templates.
  - Order: alphanumeric by file name, YAML forms before Markdown templates. A numeric prefix sets the order, zero-padded once there are 10 or more templates.
  - The chooser and `/security/advisories/new` redirect signed-out requests to the login page.
  - GraphQL `contactLinks` returns exactly the links in `config.yml`: cli/cli has a security policy, and its four links are the four in its `config.yml`.
  - URL query parameters can prefill text fields, keyed by `id` where one is set, and can be passed to the chooser. Each needs the matching permission, and an invalid one gives 404.
- **How a submitted form reads** (from real issues; the docs say only that answers become Markdown):
  - Each answered field becomes a `### <label>` heading followed by the answer. `markdown` elements are left out.
  - An empty optional `input` or `textarea` becomes `_No response_`. A ticked option becomes `- [x] <label>`.
  - An unanswered optional dropdown becomes `None` or `_No response_`: zed-industries/zed issues filed through one form in the same week show both.
  - A multi-select dropdown lists its choices on one line, comma-separated, and not in option order (vercel/next.js #101970).
  - An upload becomes a link to `https://github.com/user-attachments/files/<number>/<filename>`, or `_No response_` when empty.
  - GraphQL `Repository.issueTemplates` lists Markdown templates only, and the REST profile's `files.issue_template` stays null (COMMUNITY-P7), so neither shows whether forms are installed.
- **Schemas:**
  - No schema published by GitHub was found. The format is documented in prose, and the errors by message.
  - **SchemaStore `github-issue-forms.json`:** draft-07, last changed in `64cc4a9ee271` (2026-03-26), SHA-256 `c2722dbf00334ce4fdeffa960b8c9047caf4f1cbb8f3809663f4d604b1d3ae76`. It is served at https://www.schemastore.org/github-issue-forms.json, and its `$id` on json.schemastore.org redirects there (301).
  - **SchemaStore `github-issue-config.json`:** last changed in `734c0e501052` (2023-12-20), SHA-256 `899e718f4b8c965413b07ec63d8f089792a10c42409270db560b9a7ec0224a5a`, served at https://www.schemastore.org/github-issue-config.json. It refuses extra keys. `contact_links` needs at least one item, each with `name`, `about` and a `url` matching `^https?://`, so a `mailto:` link fails. Neither top-level key is required.
  - **The catalog** applies the forms schema to `**/.github/ISSUE_TEMPLATE/!(config).yml` and `.yaml`, and the config schema to `config.yml` and `config.yaml`. Neither covers `.github/VULNERABILITY_REPORT.yml`.
  - **Stricter than GitHub:** `validations` allows only `required`, so `min_length` fails. The `checkboxes` element has no `validations`, so only the option-level `required` passes. `projects` must be an array. Textarea `render` must be one of 1,772 listed values: `shell`, `sh`, `bash`, `text`, `console`, `toml`, `json`, `markdown` and `powershell` pass, and `Bash`, `log`, `plaintext` and `shell-session` fail.
  - **Not checked by it:** unique ids or labels, labels that are too similar, the reserved `None`, forbidden words, the name's length, the rule about non-markdown elements, and duplicate checkbox options. It does check the `id` pattern, the element types including `upload`, that `markdown` has no `id`, unique dropdown options, and assignee logins of at most 39 characters.
  - **check-jsonschema 0.39.0** (2026-10-26, the latest release on 2026-11-17) ships both schemas as `vendor.github-issue-forms` and `vendor.github-issue-config`, byte-identical to the SchemaStore files above. Its `check-github-issue-config` hook matches only `^\.github/ISSUE_TEMPLATE/config\.yml$`, and `check-github-issue-forms` everything else in that directory except `config.yml` and `config.yaml`. So a `config.yaml` is checked by neither, and neither hook matches `.github/VULNERABILITY_REPORT.yml`.
  - **Phase 7's one-off check** runs check-jsonschema through `uvx`, as PLAN.md approves; uv 0.13.8 is installed on this machine. Which check-jsonschema version `uvx` resolves wasn't checked.
- **UNVERIFIED:**
  - Whether GitHub supports `.yaml` forms officially, and whether the web chooser reads `config.yaml`.
  - What `type` does with a type name that doesn't exist, and whether type names match case-insensitively (pikaur's lowercase names suggest so).
  - What happens when `projects` names a project the opener can't write to.
  - Whether GitHub adds its own entry to the chooser when a security policy exists or private reporting is on. The docs are silent, and the chooser needs a login.
  - Whether `min_length` is enforced on ordinary issue forms.
  - The full list of words forbidden in labels.
  - Whether GitHub publishes a machine-readable schema.
  - That an unticked option renders as `- [ ] <label>`. Only the issue-parser README says so.
  - Who can see files uploaded to a public issue.
  - Whether GitHub accepts a `mailto:` contact link.
  - A 150-character default minimum for the proof-of-concept field of private reports, which came from a search summary only. Don't rely on it.
- **Used by:** `.github/ISSUE_TEMPLATE/1-bug-report.yml`, `2-asset-request.yml`, `3-client-format-change.yml`, `4-private-contact.yml`, `5-change-proposal.yml` and `config.yml`, and their one-off check-jsonschema run.

### SECURITY-POLICY-P7: private vulnerability reporting, security policies, advisories and CVEs, Scorecard and the Contributor Covenant
Sources: https://github.com/github/docs at `49a012fa50b4`: under `content/code-security/`, `how-tos/report-and-fix-vulnerabilities/` (`configure-vulnerability-reporting/{configure-for-a-repository,add-security-policy}.md`, `report-privately.md`, `fix-reported-vulnerabilities/{manage-vulnerability-reports,create-repository-advisory,publish-repository-advisory}.md`), `tutorials/fix-reported-vulnerabilities/collaborate-in-a-fork.md`, `concepts/vulnerability-reporting-and-management/{coordinated-disclosure,repository-security-advisories,github-advisory-database}.md` and `reference/permissions/repository-security-advisory.md`; `content/communities/`; `data/reusables/security-advisory/`, `data/reusables/repositories/request-security-advisory-cve-id.md`, `data/features/security-and-quality-tab.yml` and `data/variables/product.yml`; the REST data `src/rest/data/fpt-2026-03-10/{security-advisories,repos}.json`; and the security-policy page at `828f8088571b`, before its rewrite on 2026-01-23 (`7809435a`). https://github.com/github/rest-api-description at `c9c95fe34ced`. https://github.blog/changelog/2019-05-22-security-policy/. https://github.com/CVEProject/cve-website at `0d524c767399` (`src/assets/data/CNAsList.json`). https://github.com/ossf/scorecard at v5.6.0 (`61e847433562`, 2026-10-12: `checks/security_policy.go`, `checks/raw/security_policy.go`, `checks/evaluation/security_policy.go`, `probes/securityPolicy*/`, `probes/internal/utils/secpolicy/secpolicy.go`, `clients/githubrepo/client.go`, `docs/checks/internal/checks.yaml`). https://github.com/EthicalSource/contributor_covenant at `8d00b9180c88` (`content/version/3/0/code_of_conduct.md` and its history, `content/version/2/1/code_of_conduct.md`, `content/{faq.md,adopt.html,_index.html}`, `LICENSE.md`); https://www.contributor-covenant.org/version/3/0/code_of_conduct/; https://ethicalsource.dev/blog/contributor-covenant-3 (2025-07-28); https://creativecommons.org/licenses/by-sa/4.0/. https://github.com/rustsec/advisory-db/blob/233155b4025b854c68a4b199c0e32d7fd53291a2/CONTRIBUTING.md. Live reads on 2026-11-17: Grimoire's `private-vulnerability-reporting`, community profile and GraphQL fields (`securityPolicyUrl`, `isSecurityPolicyEnabled`, `codeOfConduct`, `contactLinks`); its `/security/policy` and `/security/advisories` pages, signed out; `/codes_of_conduct`; GraphQL `codeOfConduct` and `contactLinks` of other public repositories; and modelcontextprotocol/rust-sdk's `private-vulnerability-reporting`. Status: Verified, except the gaps listed.

- **Grimoire's state** (2026-11-17):
  - **Private vulnerability reporting is enabled:** `gh api repos/jabbott-iii/Grimoire/private-vulnerability-reporting` returns `{"enabled":true}`, re-read for this entry. This supersedes SECRETS and VULN-REPORTING.
  - The repository is public and owned by a personal account. It has no security policy (`isSecurityPolicyEnabled` false, `securityPolicyUrl` null) and no code of conduct.
  - Signed out, `/security/policy` says no policy was detected. It and `/security/advisories` link "Report a vulnerability" to https://github.com/jabbott-iii/Grimoire/security/advisories/new. That URL is observed; the docs name only the button.
- **Turning it on:** repository owners, organization owners, security managers and admins can, for public repositories on github.com only, under Settings, then Advanced Security in the "Security and quality" section. `PUT` and `DELETE /repos/{owner}/{repo}/private-vulnerability-reporting` need admin access.
- **Reporting:**
  - **A GitHub login is needed:** the form URL sends signed-out visitors to the login page, although the docs say anyone can report.
  - **The default form** requires a summary, details, a proof of concept and an impact statement, with an optional box to declare AI assistance. On submit, GitHub makes the reporter a collaborator and a credited user on the proposed advisory. The reporter can then start a temporary private fork.
  - **A security policy is separate** from private reporting. If there is one, GitHub shows it above the report form.
  - **Without private reporting**, GitHub's coordinated-disclosure page tells reporters to open an issue asking for a security contact, with no details of the problem, and tells maintainers to publish reporting instructions, for example in the README.
  - **API:** `POST /repos/{owner}/{repo}/security-advisories/reports` requires `summary` and `description`; `vulnerabilities`, `cwe_ids`, `severity` or `cvss_vector_string`, and `start_private_fork` are optional. With a custom form, `description` must contain that form's required sections; the default form isn't enforced for API reports.
  - **Rate limits,** per 24 hours: 25 reports per reporter across repositories where they aren't a trusted reporter, 5 per reporter per repository, and 10 per repository by default. Maintainers can change the repository's limit, exempt trusted reporters and require a CWE on every report.
  - **A custom form** lives in `.github/VULNERABILITY_REPORT.yml` or `.yaml`, in issue-form syntax with `checkboxes`, `dropdown`, `input`, `markdown` and `textarea` (no `upload`), and `required` and `min_length`. A repository's form overrides one in the owner's `.github` repository. A form that fails to parse or validate silently falls back to the default.
  - **Nothing checks a custom form:** the docs say the REST API can return it, but the REST data and the OpenAPI description have no such endpoint. SchemaStore maps no schema to that path, and its forms schema rejects `min_length` (ISSUE-FORMS-P7).
- **Notifications:** admins and security managers hear of a new report only if they watch the repository with All Activity, or with Custom and Security alerts, and have notifications on. Email also needs Email selected under Subscriptions, Watching. The coordinated-disclosure page says maintainers are notified immediately, with no such conditions.
- **Handling a report:**
  - **Triage:** a new report starts in Triage. The maintainer can accept it as a draft advisory, which isn't public; start a temporary private fork, which keeps it in Triage; comment, for the reporter and the advisory's collaborators; post a confidential comment, seen only by people with write access; or close it, ideally saying why.
  - **Permissions:** only admins add collaborators, create the fork, merge, close and publish; the reporter may also create the fork. Write permission, which advisory collaborators get, lets them edit the advisory, add and remove credits, push to the fork and open PRs. In a personal-account repository only the owner holds admin, so only jabbott-iii can accept or close a report, merge a fork, publish or request a CVE.
  - **The fork** is named `<repo>-ghsa-xxxx-xxxx-xxxx`. CI and other integrations can't reach it, and status checks don't run on its PRs. Merging doesn't apply the target branch's protection rules; all its PRs merge at once, and only one may target `main`. Publishing the advisory deletes the fork.
  - **Credits:** by username, account email or full name, with one of 9 types (Finder, Reporter, Analyst, Coordinator, Remediation developer, Remediation reviewer, Remediation verifier, Tool, Sponsor). Credited people are invited and may decline, and a username shows after publication only if accepted. The docs disagree on who may add credits: a note says only the advisory's creator, while the permissions reference allows anyone with write or admin permission on the advisory.
  - **Publishing:** the advisory's data and accepted credits become public, the conversation stays with its collaborators, and the URL doesn't change. GitHub reviews it, adds it to the Advisory Database and may send Dependabot alerts, which can take up to 72 hours. Without a fix version, the alerts offer no safe version to update to. Advisories for npm packages are also published to npm's advisories.
  - **Ecosystems:** the Advisory Database covers Composer, Erlang, Go, GitHub Actions, Maven, npm, NuGet, Pip, Pub, RubyGems, Rust and Swift, but not Homebrew, Scoop, winget or container images. A GitHub-reviewed advisory needs an ecosystem, a package name and affected versions.
- **CVEs:**
  - GitHub is a CVE Numbering Authority. Its `GitHub_M` entry (CNA-2019-0009) covers CVEs that code owners request through GitHub Security Advisories, and `GitHub_P` covers GitHub Enterprise Server only.
  - A request needs a draft advisory that lists the affected versions, and anyone with admin permission on the advisory can make it. With "Request CVE ID later", a Request CVE button replaces Publish. Through REST (`POST /repos/{owner}/{repo}/security-advisories/{ghsa_id}/cve`) the caller must be a security manager or admin, and only public repositories qualify.
  - GitHub usually reviews a request within 72 hours, and a request doesn't publish the advisory. For an eligible advisory, GitHub reserves an ID and publishes the CVE after the advisory goes public. It can't assign one when another CNA covers the project.
  - A keyword scan of the CNA list found no CNA whose scope covers third-party projects like Grimoire. The Rust Project's CNA (CNA-2025-0036) covers only its own projects. Anthropic's (CNA-2026-0052) covers Anthropic software, including the `modelcontextprotocol` organization.
  - **Upstream reports:** a CNA assigns IDs; it isn't a reporting channel. rmcp's repository, modelcontextprotocol/rust-sdk, has a `SECURITY.md` and private vulnerability reporting enabled.
  - **RustSec** asks reporters to use the crate's own security policy first. It doesn't handle embargoed reports, and takes advisories as public PRs or issues only after disclosure. A GHSA ID can be recorded in its `aliases` field.
- **The security policy on GitHub:**
  - `SECURITY.md` in `.github/`, the root or `docs/` is shown at `/security/policy` (Reporting, then Security policy, in the "Security and quality" tab).
  - GitHub suggests it say which versions are supported and how to report, and creates it from Security policy, Start setup.
  - The current docs no longer say that a policy link appears when someone opens an issue. The page before 2026-01-23 said so, and so did the 2019 changelog.
  - GraphQL `contactLinks` returned exactly the `config.yml` links of cli/cli and rust-lang/cargo, which both have policies, with no automatic security link.
- **OpenSSF Scorecard v5.6.0, the Security-Policy check** (Medium risk). Grimoire runs no Scorecard workflow, so only outside scanners would score it.
  - **Files:** 13 paths, matched case-insensitively: `SECURITY.md`, `SECURITY.markdown` and `SECURITY.adoc` in the root, `.github/` and `docs/`, and `SECURITY.rst` in the root, `.github/`, `doc/` and `docs/`. With none, it falls back to the owner's `.github` repository. Only the first file found counts, in file-listing order rather than GitHub's precedence.
  - **Score:** 0 with no file, and 0 for an empty one. Otherwise 6 points for an absolute `http://` or `https://` URL or an email address (the `[at]` form counts), 3 for text beyond those links, and 1 for more than one match of a 1–4 digit number, `disclos` or `vuln`, in any case. A relative link earns nothing, and a URL is matched only up to characters such as `#`.
  - It reads file contents only, and never asks whether private reporting is on, although its remediation text recommends turning it on.
- **Contributor Covenant:**
  - **Version:** 3.0 is current. The Organization for Ethical Source announced it on 2025-07-28; it was added to the site's repository on 2025-07-21. The repository has no 3.0 tag or release; those stop at 2.1 (2021-08-04).
  - **The text isn't frozen:** it was edited on 2025-07-28 (formatting) and 2025-08-04 (its title), still as 3.0, and hasn't changed since.
  - **URLs:** the canonical page is https://www.contributor-covenant.org/version/3/0/code_of_conduct/, with `.md`, `.txt` and `.adoc` downloads under that path. The text's attribution section names https://www.contributor-covenant.org/version/3/0/ as its permanent URL, an alias page with a meta refresh to the canonical one.
  - **Licence:** the 3.0 text is CC BY-SA 4.0, stewarded by the Organization for Ethical Source. The repository's own `LICENSE.md` is the Hippocratic License 3.0, and GitHub reports NOASSERTION. The CC BY-SA 4.0 deed asks for credit, a link to the licence and a note of any changes, without implying endorsement, and for adaptations to use the same or a compatible licence.
  - **Adopting:** the text has two placeholders for the adopter, one for how to report and one for enforcement; 2.1 had a single contact placeholder. The site's adoption guide is a four-step builder, and its static download leaves those two parts to fill in. The FAQ invites projects to adapt the enforcement section. None of the site's pages that were read says anything about linking to the text instead of copying it.
  - **On GitHub:** GitHub's docs ask anyone adopting another organization's code of conduct to follow its attribution guidelines, and GitHub's own template is 2.0. GraphQL `codeOfConduct.key` was `other` for six full copies of 3.0 and for six short files that link to it, and `contributor_covenant` for copies of 1.4, 2.0 and 2.1, except vercel/next.js's 2.1, which was `other`. The community profile counts the file either way.
- **Grimoire's files** (checked for this entry): the release archives hold only the binary, `README.md` and the two licence files (`xtask/src/release/build.rs`), so `SECURITY.md` and `CODE_OF_CONDUCT.md` aren't shipped. `cargo xtask release plan` refuses the release PR and the release while `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md` has a line that starts with `<!-- before-first-release` (`MARKED_FILES` and `check_markers` in `xtask/src/release/plan.rs`).
  - **Changed after the Phase 7 review (2026-11-18):** the marked text now sits between an opening line, `<!-- before-first-release: … -->`, and a closing one, `<!-- /before-first-release -->`, and the check refuses while either line remains. It matches both loosely: any spacing around `<!--`, in any case (`is_marker`).
- **UNVERIFIED:**
  - Whether the issue chooser still adds its own link to a security policy.
  - The text GitHub fills in from Start setup.
  - Which Covenant version GitHub's web template picker offers; the API serves 2.0.
  - Whether a short file that copies no Covenant text owes anything under CC BY-SA 4.0. That is a legal question, and no statement from the Covenant's maintainers on adopting by link was found.
  - How GitHub classifies codes of conduct. `other` above is observed, not documented.
  - Whether API reports count toward the rate limits.
  - GitHub's CVE eligibility criteria, undocumented beyond the word "eligible".
  - Every CNA's scope; the scan used keywords.
  - The OSV reporting flow, which wasn't researched; RustSec's comes from its `CONTRIBUTING.md` only.
  - Which platforms Scorecard's check supports: `checks.yaml` lists Azure DevOps, but the probes' definitions name only GitHub, GitLab and local repositories.
- **Used by:** `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` and `.github/ISSUE_TEMPLATE/config.yml`.

### README-RENDER-P7: one README on GitHub, crates.io, PyPI and npm, and in the release archives
Sources: https://github.com/github/docs at `49a012fa50b4` (`content/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax.md`, `content/get-started/writing-on-github/working-with-advanced-formatting/{organizing-information-with-collapsed-sections,organizing-information-with-tables}.md`, `about-readmes.md`, `data/reusables/repositories/{relative-links,section-links,task-list-markdown}.md`); https://github.com/github/markup/blob/8ea04e1d97aac7007563e4d31edd42fb376cf620/README.md; `POST https://api.github.com/markdown` with `mode=markdown` and `mode=gfm` (probe text only, 2026-11-17). https://github.com/rust-lang/crates.io at `dfefd6318249` (`crates/crates_io_markdown/`, `crates/crates_io_tarball/src/lib.rs`, `src/controllers/krate/publish.rs`, `src/worker/jobs/readmes.rs`, `src/bin/crates-io/admin/render_readmes.rs`, `svelte/src/lib/components/{CrateVersionPage,RenderedHtml}.svelte`, `svelte/src/lib/attachments/highlight.ts`, `e2e/acceptance/readme-rendering.spec.ts`); comrak v0.59.0 (`ac9d2081fe21`) and v0.54.0; ammonia v4.2.5 (`d098e2dfcf1c`); Cargo at `3acc79c0c841`, the 1.100.0 submodule (`src/ops/registry/cargo_publish.rs`, `src/ops/cargo_package/{mod,vcs}.rs`, `src/workspace/parser/mod.rs`, `doc/book/src/reference/{manifest,workspaces}.md`, `doc/book/src/commands/cargo-package.md`); https://static.crates.io/readmes/zizmor/zizmor-1.32.0.html with zizmor v1.32.0's `README.md` and Cargo manifests. https://github.com/npm/documentation at `b58b983187b7` (`about-package-readme-files.mdx`); npm/cli at v12.2.3 and v11.19.1 (`lib/commands/publish.js`, `docs/lib/content/configuring-npm/package-json.md`, and the bundled `@npmcli/package-json`, `pacote` and `hosted-git-info`); npm/www issue #272. https://github.com/pypa/readme_renderer at 46.0 (`65b67269dceb`: `readme_renderer/{markdown,clean}.py`, `pyproject.toml`, `tests/fixtures/`); https://github.com/pypi/warehouse at `c2f79b4a6fb0` (`requirements/main.txt`, `warehouse/utils/readme.py`, `warehouse/forklift/legacy.py`, `warehouse/filters.py`, `warehouse/packaging/tasks.py`, `warehouse/static/sass/blocks/_project-description.scss`); nh3 v0.3.7 (`cced3bedcb22`); https://github.com/lmmx/comrak at `py-0.0.16` (`Cargo.lock`); https://github.com/pypa/packaging.python.org at `f9b21277808d` (`source/specifications/{core-metadata,well-known-project-urls}.rst`); Pygments 2.21.0 (`pygments/lexers/_mapping.py`); https://pypi.org/pypi/zizmor/1.32.0/json. The local repository (`Cargo.toml`, `crates/*/Cargo.toml`, `xtask/src/release/{build,targets}.rs`, `xtask/src/release/channels/{mod,npm,wheel}.rs`, `.github/workflows/release.yml`). Status: Verified from source, except the gaps listed. npmjs.com and pypi.org answered with bot challenges, so npmjs.com's rendering is UNVERIFIED and PyPI's comes from its source only. Nothing was published.

- **Where the root `README.md` goes** (local, checked for this entry):
  - **CLI release archives:** a byte copy, beside the binary and the two licence files, with no `docs/`. Each asset binary's archive carries that asset's own `README.md` instead (`build.rs`).
  - **crates.io, for `agent-grimoire` only:** the crate sets `readme.workspace = true`. Cargo resolves the path against the workspace root (`../../README.md` from the crate), copies the file into the package root unchanged, and writes `readme = "README.md"` into the packaged manifest. `.cargo_vcs_info.json` records `path_in_vcs` as `crates/agent-grimoire`. When the skeptic packaged the crate, its `README.md` was byte-identical to the root README.
  - **The five library crates** set `readme = "README.md"` and ship their own `crates/<name>/README.md`. If a package root already holds a file with the readme's name, `cargo package` warns and archives that file instead of the outside one.
  - **PyPI wheels:** the METADATA body is the README verbatim, and generation fails if it isn't UTF-8. `Description-Content-Type: text/markdown` has no variant or charset. The only Project-URLs are `Repository` and `Changelog`, the latter pinned to the tag as `blob/v<version>/CHANGELOG.md` (`wheel.rs`).
  - **Not npm:** `npm.rs` writes the launcher's and each platform package's README itself, with absolute links only. Their `package.json` files set `homepage` and `repository` but no `bugs`.
  - No channel rewrites links or Markdown.
- **GitHub:**
  - **Relative links** resolve against the branch being viewed, and a leading `/` means the repository root. Link text must be on one line. GitHub's docs recommend relative links, because absolute ones may not work in clones. An absolute `blob/main` link followed from a tag's view leads to `main`.
  - **The README** is found in `.github/`, the root, then `docs/`. It is truncated past 500 KiB, and GitHub builds an outline from its headings.
  - **Heading anchors:** letters lowercased, spaces turned into `-`, other punctuation and markup removed (`-` and `_` are kept), and `-1`, `-2` added to repeats. A probe turned "Exit codes & errors" into `exit-codes--errors`. For non-ASCII letters, the docs' example keeps an upper-case Θ, while the renderer lowercased it.
  - **Alerts:** NOTE, TIP, IMPORTANT, WARNING and CAUTION. They can't be nested in other elements, and the docs advise one or two per article.
  - **Also rendered:** footnotes (not in wikis), task lists, pipe tables (with a blank line before them), `<details>` and `<summary>`, and `<picture>`. HTML comments are hidden.
  - **Sanitising:** script tags, inline styles and classes are removed. An `id` becomes `user-content-<id>`, and so does an `<a name>`. `align` is kept. Definition lists aren't rendered. External images go through camo. The file view's full allowlist isn't public.
  - **The Markdown API isn't a preview:** `mode=markdown` leaves alerts and task lists as literal text, and `mode=gfm` renders them but emits no heading ids.
- **crates.io:**
  - **Renderer:** comrak `=0.59.0`, sanitised with ammonia `=4.2.5`. On: alerts, autolinks, description lists, multi-line block quotes, strikethrough, tables, task lists, footnotes, and heading ids prefixed `user-content-`, in hrefs too. Off: tagfilter and emoji shortcodes, which aren't compiled in.
  - **Allowed HTML:** ammonia's defaults, which include `details`, `summary`, `kbd`, `sub` and `sup`, plus `input`, `ol`, `picture`, `section` and `source`. `align` is allowed on every tag, and `id` only on headings, `a` and `li`, with the prefix. There is no `style`, only listed classes survive, and `<a name>` and comments are stripped. `details` keeps no attributes, so `open` is dropped.
  - **Code blocks:** a language class survives, and is highlighted, only for `bash`, `c`, `glsl`, `go`, `ini`, `javascript`, `json`, `xml`, `mermaid`, `protobuf`, `ruby`, `rust`, `scss`, `sql`, `toml`, `yaml`, `rs`, `clike` and `markup`. `sh`, `shell`, `console`, `powershell` and `text` aren't highlighted. Anything after a comma in the info string is dropped. `mermaid` blocks render as diagrams.
  - **Relative links** are rewritten only when the repository's host is exactly github.com, gitlab.com or bitbucket.org; otherwise the href is removed. For a GitHub repository, a link becomes `{repository}/blob/HEAD/{path}`, and a media file (`svg`, `png`, `jpg`, `jpeg`, `gif`, `mp4`, `webm`, `ogg`, `webp`) becomes `{repository}/raw/HEAD/{path}`, with `?sanitize=true` for SVG. HEAD is the default branch, never the published commit or tag.
  - **Base directory:** the parent of `path_in_vcs` joined with the readme's path. A leading `/` starts at the repository root, `../` stops at the root, and fragment-only links are kept.
  - **For Grimoire:** in the root README on the `agent-grimoire` page, `[x](docs/cli.md)` becomes `…/blob/HEAD/crates/agent-grimoire/docs/cli.md`, which doesn't exist, and `[x](/docs/cli.md)` becomes `…/blob/HEAD/docs/cli.md`. zizmor 1.32.0 shows the same break: its root README's `CONTRIBUTING.md` link renders as `blob/HEAD/crates/zizmor/CONTRIBUTING.md`, a 404. In a library crate's own README, `[x](LICENSE-APACHE)` becomes `…/blob/HEAD/crates/agent-grimoire-model/LICENSE-APACHE`, which exists.
  - **Re-rendering:** crates.io's admin re-render tool passes no `path_in_vcs`, so a re-rendered README resolves relative links from the repository root.
  - **In-page links:** on a click or a hash change, the page scrolls to `user-content-<hash>`, so `[x](#install)` reaches the heading.
  - **Publishing:** crates.io takes the README's text and file name from the publish request, the repository from the packaged `Cargo.toml` and `path_in_vcs` from `.cargo_vcs_info.json`, and renders in a background job. A file is rendered as Markdown only with no extension or `md`, `markdown`, `mdown`, `mdwn`, `mkd`, `mkdn` or `mkdown`.
- **PyPI:**
  - **Renderer:** Warehouse pins readme-renderer 46.0, comrak 0.0.16 (the Python binding; it wraps Rust comrak 0.54.0, whose heading anchors match 0.59.0's), nh3 0.3.7 and Pygments 2.21.0. readme-renderer uses comrak, not cmarkgfm.
  - **Always GFM:** Warehouse renders every `text/markdown` description with readme-renderer's default GFM variant, whatever `variant=` says. The core-metadata spec also defaults to GFM, and allows only UTF-8.
  - **On:** alerts, autolinks, footnotes, heading ids prefixed `user-content-`, emoji shortcodes, strikethrough, tables, tagfilter and task lists. Description lists aren't among them.
  - **Allowed HTML:** a fixed list of tags that includes `details`, `summary`, `picture`, `input`, `section` and `aside`, but not `source` or `style`. `<a>` keeps `href` and `title`, `id` is allowed on every tag, the URL schemes are `http`, `https` and `mailto`, and links get `rel="nofollow"`. `details` keeps only `id`, so `open` is dropped.
  - **Relative URLs pass through unchanged,** so they resolve against pypi.org and break. Nothing rewrites them from Project-URL or the repository. Every image `src` is rewritten through camo. Comments are stripped.
  - **Fragments:** every `href="#x"` becomes `#user-content-x` (footnote links aside), so links to headings work, but a link to a hand-written `id` breaks.
  - **Alerts** become `<aside class="admonition …">`, and Warehouse's stylesheet has no rule for them.
  - **Upload check:** an upload whose description renders to nothing is rejected ("The description failed to render").
  - **Re-rendering:** Warehouse re-renders stored descriptions when its readme-renderer version changes. The text is kept, but the HTML can change after upload.
  - **Highlighting** goes by Pygments alias, so `bash`, `sh`, `shell`, `console`, `powershell`, `toml`, `json` and `text` all work. `mermaid` stays a code block.
  - **Project-URL labels** are normalised by removing ASCII punctuation and whitespace and lowercasing, and are at most 32 characters. Well-known labels, with aliases: `homepage`; `source` (`repository`, `sourcecode`, `github`); `download`; `changelog` (`changes`, `whatsnew`, `history`); `releasenotes`; `documentation` (`docs`); `issues` (`bugs`, `issue`, `tracker`, `issuetracker`, `bugtracker`); `funding` (`sponsor`, `donate`, `donation`); `security` (`securitypolicy`).
- **npm:**
  - npm's docs say npmjs.com renders the README as GitHub Flavored Markdown through GitHub's API, without naming a mode. The README must be at the package root, and a package page's README changes only when a new version is published.
  - README and LICENSE (or LICENCE) files are always packed, in any case and with any extension. `repository` is `{type, url}`, with an optional `directory`.
  - **npm 11.19.1,** which `release.yml` pins, publishes Grimoire's tarballs through `pacote.manifest` with `fullReadJson`, which runs `PackageJson.prepare`. That embeds the README as `readme` and `readmeFilename`, and its `normalizeData` step sets a missing `bugs` to https://github.com/jabbott-iii/Grimoire/issues from the repository URL and keeps `homepage`.
- **Side by side** (summarised from the facts above; the archive copy is plain text, where everything shows literally):

  | Markdown | GitHub | crates.io (`agent-grimoire`) | PyPI |
  |---|---|---|---|
  | `[x](docs/a.md)` | resolved on the branch being viewed | `…/blob/HEAD/crates/agent-grimoire/docs/a.md` | left relative; breaks |
  | `[x](#heading)` | works | works | works |
  | `<a name="x">` | prefixed `user-content-` | `name` stripped | `name` stripped |
  | A hand-written `id="x"` | prefixed `user-content-` | prefixed, on headings, `a` and `li` only | kept unprefixed, so `#x` links break |
  | Alerts | rendered | rendered | `<aside>`, with no rule in Warehouse's stylesheet |
  | Definition lists | not rendered | rendered | not rendered |
  | `:emoji:` shortcodes | rendered | literal text | rendered |
  | `<picture>` with `<source>` | kept | kept | `<source>` stripped |
  | `<details open>` | `details` kept; `open` not checked | `open` dropped | `open` dropped |
  | `bash` code blocks | not checked | highlighted | highlighted |
  | HTML comments | hidden | stripped | stripped |

- **UNVERIFIED:**
  - How npmjs.com renders READMEs: relative links, images, sanitising and anchors. Its pages answered with a bot challenge, and its renderer isn't public. Grimoire's npm READMEs are generated and use only absolute links.
  - How pypi.org styles alerts, task lists and `details`; no rendered project page was seen.
  - GitHub's sanitiser allowlist for the file view, and what GitHub does with a `../` that climbs above the repository root.
  - crates.io's styling of alerts, `details` and task lists.
  - Whether npm 12 fills `bugs` and `homepage` when publishing a tarball. Grimoire pins npm 11.19.1, checked above.
  - The README size limits of crates.io and PyPI, and whether PyPI lets a release's description be changed after upload.
  - Anything end to end for Grimoire, since nothing has been published. zizmor 1.32.0 stands in for crates.io.
- **GFM extended autolinks** (checked 2026-11-18 at https://github.github.com/gfm/, spec version 0.29-gfm, 2019-04-06, §6.9 "Autolinks (extension)"): GitHub turns `www.` and `http://` or `https://` text into a link without `<…>`, but only at the start of a line, after whitespace, or after `*`, `_`, `~` or `(`. `xtask/src/readme_links.rs` looks for bare URLs at those boundaries.
- **Used by:** `README.md`.

### VERSIONING-P7: SemVer, Cargo versions and MSRV, release-plz bumps, clap deprecation and help
Sources: https://semver.org/spec/v2.0.0.html (Last-Modified 2026-10-16, fetched 2026-11-17); https://github.com/semver/semver at tag `v2.0.0` (`7c834b3f3a49`) and `master` (`f99d5485190a`, 2025-11-05). Cargo at `3acc79c0c841`, the 1.100.0 submodule (`doc/book/src/reference/{specifying-dependencies,semver,rust-version,resolver,config,manifest}.md`, `doc/man/{cargo-install,cargo-package}.md`, `doc/book/src/CHANGELOG.md`, `src/ops/{cargo_install,common_for_install_and_uninstall,resolve}.rs`), and at `30a34c6821b5` (1.96.0), `c980f4866141` (1.97.0) and `797e8a9bca27` (1.98.0); https://github.com/rust-lang/rust/blob/1.100.0/RELEASES.md; https://github.com/rust-lang/cargo/pull/17388; the local `cargo-install.1` man pages of 1.96.0, 1.99.0, 1.99.1 and 1.100.0. https://github.com/rust-lang/crates.io at `dfefd6318249` (`migrations/2024-10-25-112826_make-unique-version-unique/up.sql`). https://github.com/release-plz/release-plz at `release-plz-v0.4.3` (`92e907a14715`: `crates/next_version/src/lib.rs`, `website/docs/config.md`, `crates/release_plz/tests/all/update.rs`). https://github.com/clap-rs/clap at v4.6.8 (`db86ef6eccb2`: `clap_builder/src/builder/{arg,command}.rs`, `clap_builder/src/derive.rs`, `src/_features.rs`, `src/_derive/mod.rs`, `clap_derive/src/derives/args.rs`, `clap_mangen/src/lib.rs`, `clap_mangen/CHANGELOG.md`) and issues #3321 and #5486; https://github.com/ConnorGray/clap-markdown at v0.1.5 (`558062836b`) and its open issues; the crates.io API for clap, clap_mangen, clap-markdown and roff. https://www.gnu.org/software/bash/manual/html_node/Exit-Status.html (Bash 5.3). The local repository at `a052cad` and its working tree. Status: Verified, except the gaps listed. Nothing was built, installed or generated.

- **SemVer 2.0.0** (cite semver.org; the text is CC BY 3.0):
  - **Public API (rule 1):** it must be declared, in code or only in documentation, and should be precise and comprehensive.
  - **A release never changes (rule 3):** any change needs a new version. The FAQ says to fix an accidental breaking release with a new release that restores compatibility, and to tell users which version is affected.
  - **0.y.z (rule 4)** is for initial development, and anything may change. The patch, minor and major rules (6 to 8) are stated only for a major version above 0, and 1.0.0 defines the public API (rule 5).
  - **Deprecation:** rule 7 bumps the minor version when anything in the public API is deprecated. The FAQ asks for at least one minor release containing the deprecation before the major release that removes the feature.
  - **The FAQ on 0.y.z:** start at 0.1.0 and bump the minor version for each release. Software already used in production should probably be 1.0.0.
  - **Pre-releases (rule 9)** sort below their release. Precedence (rule 11) compares identifiers left to right, numeric ones numerically: `1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 < 1.0.0-beta.11 < 1.0.0-rc.1 < 1.0.0`.
  - **Build metadata (rule 10)** is ignored for precedence. semver.org and `master` say MUST, while tag `v2.0.0` says SHOULD.
  - **Tags:** `v1.2.3` isn't a semantic version, but a `v` prefix is a common way to mark one in a tag.
- **Cargo's rules** (Cargo 1.100.0):
  - **Requirements:** a bare requirement is a caret one, compatible while the left-most non-zero component matches. `0.2.3` means `>=0.2.3, <0.3.0`, `0.0.3` means `>=0.0.3, <0.0.4`, and `1.2.3` or `^1.2.3` means `>=1.2.3, <2.0.0`. An exact pin is `=1.2.3`. Cargo says this differs from SemVer, which treats all versions before 1.0.0 as incompatible, so a `0.1.0` requirement accepts 0.1.9.
  - **Bumps in 0.y.z:** Cargo's SemVer chapter treats a change in y as major and a change in z as minor, and every 0.0.z change as major. So a "minor" change in Cargo's terms is a 0.y.Z bump.
  - **MSRV:** raising `rust-version` is "possibly-breaking", and Cargo recommends treating it as a minor change; `rust-version.md` assumes a minor incompatibility. It advises publishing an MSRV policy (its examples: N-2, every even release, the current calendar year), and its example policy bumps the minor version when `rust-version` changes. On an older toolchain Cargo errors for every target, unless `--ignore-rust-version` is passed. The field is a bare version, respected since Cargo 1.56.
  - **Applications:** Cargo prescribes no SemVer contract for binaries. A project decides how SemVer applies to its CLI and documents it.
  - **Pre-releases** match only requirements that name one, and `cargo install` avoids them.
  - **Build metadata** is ignored by Cargo and shouldn't appear in requirements. crates.io keeps a unique index on the version without its metadata, so it refuses `1.0.0+b` once `1.0.0+a` exists.
  - **`=` requirements** are recommended for tightly coupled packages (the book's example is a proc-macro that is never used on its own), and 0.0.x versions for permanently unstable ones.
  - **Resolver 3** is the edition 2024 default and needs Rust 1.84. Its only change is that `incompatible-rust-versions` defaults to `fallback`, which prefers versions compatible with the members' `rust-version` and never errors. Only the top-level workspace's resolver counts, so Grimoire's setting has no effect on its users' builds.
- **`cargo install` from a registry:**
  - It picks the highest matching version and fails, rather than picking an older one, if that version needs a newer rustc. The error names the required rustc and, for registry sources, the newest version that works with the current one. `--ignore-rust-version` skips the check.
  - It resolves in an ephemeral workspace with `rust-version`-aware resolution turned off (in 1.96.0 and 1.100.0). In 1.100 it also turns off the new minimum publish age.
  - **The lockfile:** Cargo 1.100 uses the `Cargo.lock` packaged in the crate by default, and `--locked` is a no-op (rust-lang/cargo#17388, and Rust 1.100.0's release notes; the Cargo Book's changelog leaves it out). Cargo 1.96.0, 1.97.0, 1.98.0, 1.99.0 and 1.99.1 ignore the packaged lockfile unless `--locked` is passed. The 1.100 `cargo-package` page still says the lockfile is used only with `--locked`. `cargo package` always includes `Cargo.lock` unless `--exclude-lockfile` is passed.
  - It reads `$CARGO_HOME/config.toml`, not the project's `.cargo/config.toml`, except with `--path`. `--version 1.2.3` installs exactly that version.
- **release-plz 0.4.3:**
  - **Bumps:** in 0.x a `feat` bumps the **patch** (0.2.4 to 0.2.5), and a breaking commit (`!` or a `BREAKING CHANGE` footer) bumps the minor (0.4.4 to 0.5.0). From 1.0, a breaking commit bumps the major. 0.0.x always bumps the patch, and so does a commit that isn't a Conventional Commit.
  - **Options:** `features_always_increment_minor` defaults to false. `custom_major_increment_regex` can take 0.x to 1.0.0. `custom_minor_increment_regex` forces a minor bump for matching commits. `semver_check` (cargo-semver-checks) defaults to true for libraries.
  - **One shared version:** a release-plz test shows that when packages inherit `version.workspace`, a breaking commit in any of them sets the next version for all of them.
- **clap 4.6.8** (Grimoire's lockfile: clap 4.6.8, clap_builder 4.6.7, clap_derive 4.6.8):
  - **No deprecation of an application's own flags or commands:** `Arg` and `Command` have no `deprecated` method, and the derive has no such attribute. clap's `deprecated` Cargo feature warns about clap's own legacy API instead. Issue #3321, on stability attributes, has been open since 2022-01-20. #5486 was closed as not planned on 2024-05-04, pointing at Cargo's approach of defining both arguments and warning by hand.
  - **What exists:** `hide` (a hidden `Arg` still shows in usage strings on errors), `hide_short_help`, `hide_long_help`, hidden `alias` and `visible_alias`. Any builder method works as a raw derive attribute, such as `#[arg(hide = true)]` or `#[command(alias = "old")]`.
  - **Aliases can't be told apart after parsing:** `ArgMatches` has no method that reports which alias was typed, and `ValueSource` has only `DefaultValue`, `EnvVariable` and `CommandLine`. Warning about an old name needs a separate hidden argument or subcommand, or a scan of the raw arguments before clap parses them. `index_of` positions differ from argv positions when short flags are combined.
  - **Help without extra crates:** `CommandFactory::command()`, `Command::build()`, `get_subcommands()`, `render_help()`, `render_long_help()`, `render_usage()` and `render_long_version()`. After `build()`, a subcommand's `bin_name` is `grimoire add` and its `display_name` is `grimoire-add`. Global arguments are copied into subcommands only while the parent is built.
  - **A bare `grimoire`:** clap_derive gives a required subcommand `subcommand_required` and `arg_required_else_help`. With no arguments, clap returns `DisplayHelpOnMissingArgumentOrSubcommand`, and Grimoire prints the help on stdout and exits 2 (`usage()` in `out.rs`). With an argument but no subcommand, it is a usage error: exit 2, and with `--json` an envelope whose diagnostic code is `cli/usage`. This was read from the code, not run.
- **CLI doc generators** (researched; PLAN.md keeps `docs/cli.md` hand-written, with no new dependency):
  - **clap_mangen** 0.3.4 (2026-11-16), after 0.3.3 (2026-08-12), 0.3.2 (2026-08-06), 0.3.1 (2026-08-05) and 0.3.0 (2026-03-27). MIT OR Apache-2.0, MSRV 1.85, edition 2024, and released with clap from clap-rs/clap. It depends on clap `^4.6.6` (`std` only) and roff `^1.1.1` (1.1.1, 2026-03-28, MIT OR Apache-2.0, MSRV 1.85). `Man::new` builds the command, and `generate_to` turns off the help subcommand and writes one ROFF page per command that isn't hidden, such as `grimoire-add.1`.
  - **clap-markdown** 0.1.5 (2025-05-02), the last of 7 versions since 0.0.1 (2022-12-13), with no commit after its tag. MIT OR Apache-2.0 in its `Cargo.toml`, though GitHub's API reports Apache-2.0; edition 2021, no `rust-version`, and clap `4.*`. It writes one Markdown document, with a footer advertising itself unless `show_footer` is false. It skips hidden commands and options but not hidden positional arguments. It never calls `build()`, so its subcommand sections would probably lack the global flags (inferred). Open issues include #54 (calling `build()` first duplicates command paths), #33, #49 and #30.
  - Both crates' licences fit `deny.toml`'s allow list. `deny.toml` sets `include-dev = false`, so a dev-dependency wouldn't be licence-checked.
- **Grimoire's versioned surfaces** (local, checked for this entry):
  - **Versions:** the workspace is 0.1.0, with `rust-version = "1.96"` and resolver 3. `rust-toolchain.toml` pins 1.100.0, and CI checks the MSRV with `cargo +1.96.0 check --workspace --all-targets --locked`. The six publishable crates share the workspace version and depend on each other with `version = "0.1.0"`, which Cargo reads as `>=0.1.0, <0.2.0`.
  - **`release-plz.toml`** (read at `a052cad`): `semver_check = false` and the tag `v{{ version }}`. A comment says the library APIs are internal to the CLI for now and that 0.x has no compatibility promise. There is no `CHANGELOG.md` yet.
    - **Changed in Phase 7 (2026-11-18):** the comment now says that a change that breaks a library API must be marked breaking (`!`), so that it ships as the next 0.y (`docs/versioning.md`), and that nothing checks this automatically. `semver_check` is still `false`.
  - **File formats:** `grimoire.toml`, `grimoire.lock`, `asset.toml` and bundle files start with `format`, which accepts only 1 ("this version of Grimoire supports only format 1"). The lockfile never records the CLI version (ADR-0001). `schemas/` covers these four formats only.
  - **`--json`:** the envelope is `{format, command, ok, exit, result, diagnostics}`, with `format` written as a literal 1 in `out.rs` and no JSON Schema. Each diagnostic is `{severity, code, message, file, field}`. Help and `--version` print plain text even with `--json`.
  - **Exit codes:** 0 to 6 only (`out.rs`: Ok, Failure, Usage, Invalid, Conflict, Check, Refused). `architecture.md` §9.3 also lists 130, which nothing in the CLI returns (Phase 7 rewords its row to say so), and the CLI installs no signal handler. Bash reports 128+N for a command killed by signal N, so 130 is what a shell shows after SIGINT.
  - **Version strings:** the model's `Version` type refuses build metadata, and the CLI exits 1 if its own version doesn't parse. The generated marketplace plugins carry the only versions with metadata (`<version>+<12 hex>`, `xtask/src/marketplace.rs`).
  - **Assets:** maturity is `experimental`, `stable` or `deprecated`, a deprecated asset names its replacement and reason, resolution warns with `resolve/deprecated`, and tag selection skips deprecated assets. `min-cli-version` is enforced: an older CLI fails with `resolve/cli-too-old`.
  - **The catalog pack** has its own header, `grimoire catalog pack v1`.
- **UNVERIFIED:**
  - The exit status on Windows when Grimoire is interrupted with Ctrl-C.
  - What "minor" means in 0.x under `custom_minor_increment_regex`.
  - How release-plz bumps the shared version in Grimoire's setup, where only `agent-grimoire` writes a changelog. The test above is indicative only.
  - Whether clap-markdown 0.1.5 builds on Rust 1.96 with clap 4.6.8, and how either generator shows Grimoire's global flags.
  - How Claude Code compares plugin versions that differ only in build metadata.
  - Any `cargo install` behaviour run for real. All of it was read from Cargo's source and man pages.
- **Used by:** `docs/versioning.md`, `docs/cli.md`, `CONTRIBUTING.md`, `.github/pull_request_template.md` and `README.md`.
