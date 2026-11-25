# ADR-0004: Package and binary names

- **Status:** Accepted (maintainer approved all Phase 0 proposals, 2026-11-16). **Nothing has been reserved or registered.** Amended by [ADR-0009](0009-release-pipeline.md) (2026-11-17): the repository stays `jabbott-iii/Grimoire`; the winget identifier is `jabbott-iii.agent-grimoire`, with the command alias `agent-grimoire`, because `CatfinityStudios.Grimoire` already uses `grimoire`; PyPI wheels are written by `cargo xtask`, not maturin. Amended by [ADR-0011](0011-container-image.md) (2026-11-17): the container image name must be lowercase.
- **Date:** 2026-11-16

## Context

"grimoire" is already taken on crates.io (Rust's package registry), npm (JavaScript's) and PyPI (the Python Package Index). We need:
- published names for each channel (each way users install Grimoire);
- the installed binary name;
- names for the internal crates (Rust packages) that must be published so that `cargo install` and cargo-binstall (which installs a crate's prebuilt binary) work;
- names for asset binaries, the container image and the Claude Code marketplace (the plugin catalog that Claude Code users add).

Requirements:
- consistency across channels;
- no PATH collisions (no other common program installed under the same command name);
- no impersonation of unrelated projects.

## Evidence (read-only registry queries, 2026-11-16)

Method:
- **crates.io:** `GET https://crates.io/api/v1/crates/<name>`.
- **npm:** `GET https://registry.npmjs.org/<name>`, plus `GET https://registry.npmjs.org/-/org/<scope>/package` for scopes.
- **PyPI:** `GET https://pypi.org/pypi/<name>/json`.
- **Homebrew:** `GET https://formulae.brew.sh/api/{formula,cask}/<name>.json`.
- **Scoop:** GitHub contents API on `ScoopInstaller/{Main,Extras}/bucket/<name>.json`.
- **winget:** GitHub code search on `microsoft/winget-pkgs`.

"Free" means no package exists under that exact name today. It does **not** guarantee the name can be registered:
- registries also apply normalization (treating names that differ only in case, or in `-` and `_`, as the same), similarity and typosquat rules (refusing names that look like an existing one);
- PyPI pending publishers (trusted publishers registered before a project exists) don't reserve names.

| Name | crates.io | npm | PyPI | Homebrew core |
|---|---|---|---|---|
| `grimoire` | **taken** (0.2.1, last updated 2019-03-22, installs a `grimoire` bin) | **taken** (0.1.0, phantomjs framework) | **taken** (0.0.2, biological sequence tools) | free |
| `grimoire-cli` | free | **taken** (1.0.3, 2026-01-09, installs a `grim` bin) | free | free |
| `grimoire-ai` | free | **taken** (0.0.1, "Coming soon") | free | — |
| `grimoire-kit` | free | free | **taken** (3.90.0) | — |
| `grimoire-agents` | free | free | free | — |
| **`agent-grimoire`** | **free** | **free** | **free** | **free** (formula and cask) |
| `grim` | **taken** (0.4.0) | — | — | free (but Ubuntu ships a `grim` package) |

npm scopes (the `@name/` prefix that groups an organization's packages):

| Scope | Status |
|---|---|
| `@grimoire` | **taken** (org; owns `@grimoire/core`) |
| `@grimoire-ai` | **taken** (owns `@grimoire-ai/cli`, among others) |
| `@agent-grimoire` | "Scope not found" |
| `@grimoire-cli` | "Scope not found" |
| `@jabbott-iii` | "Scope not found" |

Other checks:
- **Library-crate names** `agent-grimoire-{core,model,catalog,resolve,install,adapters}` and `grimoire-{core,model,resolve,install,adapters,fs,catalog}`: all free on crates.io.
- **Prefix sharing:** unrelated crates already use the `grimoire` prefix, including `grimoire-sdk`, `grimoire-loader`, `grimoire_css*` and `bevy_grimoire*`.
- **Scoop:** no `grimoire.json` or `agent-grimoire.json` in Main or Extras.
- **winget:** code search returned 0 hits for "grimoire" in `installer.yaml` files. Search indexing may be incomplete.
- **Binary collisions:** the local Ubuntu apt cache has no `grimoire` package, but does have a `grim` package (a Wayland screenshot tool).

## Options

1. **`agent-grimoire` everywhere**, with binary `grimoire`. It is free on every channel checked, identical everywhere, and descriptive.
2. **`grimoire-cli` where free** (crates.io, PyPI), plus something else on npm (e.g. `@agent-grimoire/cli`). This is inconsistent across channels, and npm's `grimoire-cli` is an unrelated active package, so users would confuse them.
3. **A scoped npm package under the maintainer's handle** (`@jabbott-iii/grimoire`). The scope doesn't exist yet, and this ties the project's identity to one person's account.

## Decision (proposed)

In the table, ADR-0002 is the architecture decision record (ADR) that packages each Model Context Protocol (MCP) server and tool as its own binary. `ghcr.io` is the GitHub Container Registry, and OCI (the Open Container Initiative) sets the image and registry standards.

| Thing | Name |
|---|---|
| Installed command-line interface (CLI) binary | **`grimoire`** (rejected `grim` because of the Ubuntu and npm collisions) |
| crates.io CLI package | **`agent-grimoire`** (`[[bin]] name = "grimoire"`), with `[package.metadata.binstall]` (cargo-binstall's metadata) |
| crates.io core libraries | **`agent-grimoire-model`, `-catalog`, `-resolve`, `-install`, `-adapters`** (directories under `crates/` use the same names) |
| npm launcher | **`agent-grimoire`** (unscoped, so `npx agent-grimoire` works) |
| npm platform packages | **`@agent-grimoire/<platform>-<arch>`**: `linux-x64`, `linux-arm64`, `darwin-x64`, `darwin-arm64`, `win32-x64`, `win32-arm64`. Needs an npm organization named `agent-grimoire` |
| PyPI | **`agent-grimoire`** (maturin `bindings = "bin"`) |
| Homebrew | formula **`agent-grimoire`** in tap repository **`jabbott-iii/homebrew-tap`** (`brew install jabbott-iii/tap/agent-grimoire`). A tap is a third-party Homebrew package repository |
| Scoop | manifest **`agent-grimoire.json`** in bucket repository **`jabbott-iii/scoop-bucket`**. A bucket is Scoop's name for a package repository |
| winget | identifier in `Publisher.Package` form, e.g. `JAbbottIII.AgentGrimoire`. Exact form validated against schema 1.28 in Phase 5 (the release-pipeline phase of `roadmap.md`) |
| Asset binaries (ADR-0002) | **`grimoire-mcp-<id>`**, **`grimoire-tool-<id>`** (not published to registries) |
| Container image | **`ghcr.io/jabbott-iii/grimoire`**, lowercase. Settled in Phase 6: lowercase is mandatory, because OCI distribution-spec v1.1.1 allows only lowercase repository names. The name is written out, never built from `github.repository` (`jabbott-iii/Grimoire`) ([ADR-0011](0011-container-image.md)) |
| Claude Code marketplace | **`grimoire`** (not on the list of names Claude Code reserves, `CC-MKT` in [`../specs.md`](../specs.md)). Plugin names = asset IDs |

## Consequences

- **Maintainer-only setup.** Claiming names (first publishes, creating the npm organization and the tap and bucket repositories) is done by the maintainer, as listed in `roadmap.md`. Until then a squatter (someone who registers a name before its owner) could take any of these names. Claim them close to the first release.
- **Six crates on crates.io.**
  - The CLI plus 5 libraries means six one-time bootstrap publishes that need credentials, and six trusted-publisher configurations (`CRATES-TP`, the crates.io trusted-publishing entry in `specs.md`). Trusted publishing lets continuous integration (CI) publish with a short-lived identity token instead of a stored one.
  - The alternative is to collapse the libraries into one `agent-grimoire-core` with modules: fewer names, but weaker compile-time boundaries. The proposal keeps separate crates as the brief describes.
- **Docs wording.** Documentation consistently says "install `agent-grimoire`, run `grimoire`".
- **Repository name case.**
  - The remote is `github.com/jabbott-iii/Grimoire` (capital G), while the brief says `grimoire`.
  - Trusted-publisher configurations, cosign certificate identities and OpenID Connect (OIDC) subjects embed the repository name. (cosign signs release files; the OIDC subject is the claim in a CI token that names the repository and the branch or environment.)
  - **Settle the canonical spelling before Phase 5.** Nothing in this ADR writes the repository URL into any config.
