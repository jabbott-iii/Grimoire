# 00 — Overview

## Purpose

Grimoire is a catalog of AI-agent assets and a Rust command-line tool (CLI), `grimoire`, that installs the assets a project selects. The assets are the instructions and programs that AI coding tools use: rules, skills, agents, tools, and Model Context Protocol (MCP) servers, which are programs that give an AI client extra tools over a standard protocol. Each AI coding tool that reads them is a *client*. Grimoire supports six: Claude Code, OpenAI Codex, Cursor, Gemini CLI, GitHub Copilot, and the portable `agents-md` target, which writes `AGENTS.md` and `.agents/skills/` for any client that reads them.

It is for developers and teams who want the same reviewed agent configuration in many repositories and for several clients, without copying files by hand.

Grimoire has three parts:

1. **A catalog** of assets under `catalog/`. Each asset has a strict manifest, `asset.toml`, that describes it and lists exactly what it installs. Strict means that an unknown field is an error ([the catalog spec](subsystems/catalog.md)).
2. **The `grimoire` CLI.** It installs only the assets a user selects, in each chosen client's native layout: the files and folders that the client already reads. It tracks everything it writes, so it can update that content later, detect drift (the content was changed or removed outside Grimoire), and remove exactly that content ([the CLI spec](subsystems/cli.md)).
3. **Generated distribution files:** JSON Schemas, which describe what a valid manifest or configuration file looks like, and a Claude Code plugin marketplace, a list of plugins that Claude Code users can install. All of them are derived from the catalog and the Rust types, never maintained by hand ([the xtask spec](subsystems/xtask.md#generated-artifacts)).

**The rule that shapes the rest:** Grimoire owns exactly what it writes, and nothing else. It writes only inside the project root, through no symbolic link; it uses no network and runs no asset content while installing; and it records every unit it writes in `grimoire.lock`, so that it changes and removes only those units. The principles in [`01-architecture.md`](01-architecture.md#principles) follow from this rule.

## Goals

1. **One configuration, every client.** One `grimoire.toml` installs the selected assets for any combination of the six clients. Checked by `every_exemplar_installs_for_every_client` in `crates/agent-grimoire/tests/builtin.rs`.
2. **Writes stay inside the project root** and follow no symbolic link ([ADR-0006](adr/0006-filesystem-confinement.md), which confines every write to the root with cap-std). Checked by the confinement tests in `agent-grimoire-install` and by `symlinks_are_refused` in `crates/agent-grimoire/tests/cli.rs`.
3. **Installs are reproducible and offline.** `grimoire.lock` pins every version and hash, and the install path contains no network code ([ADR-0001](adr/0001-catalog-delivery.md), which builds the catalog into the binary). Checked by `second_apply_changes_nothing` in `cli.rs`, and by `cargo tree -p agent-grimoire -e normal`, which lists no HTTP or TLS crate.
4. **Removal leaves only the user's content.** Removing an asset deletes only what Grimoire wrote for it, and `grimoire uninstall` leaves nothing of Grimoire's. Checked by `remove_and_uninstall_leave_only_user_content` and `uninstall_from_scratch_leaves_nothing` in `cli.rs`, and by `every_exemplar_installs_verifies_and_uninstalls` in `builtin.rs`.
5. **Every release can be verified.** Each release file has a SHA-256 checksum, a keyless signature and a build-provenance attestation, each archive has a software bill of materials (SBOM, a list of what it contains), and every channel ships the same binary ([ADR-0009](adr/0009-release-pipeline.md), the release pipeline). Checked by `cargo xtask release verify` in every Release dry-run, with `--partial`, because a dry-run builds four of the six targets ([`03-platforms.md`](03-platforms.md#targets)). Not yet checked on a real release: task M8.7 in [`roadmap.md`](roadmap.md).
6. **Adding an asset is a data-only change** under `catalog/`, checked by `cargo xtask validate` ([`catalog/README.md`](../catalog/README.md)).

## Non-goals

Bringing any of these back needs a new decision record.

- **Fetching the catalog over the network** in v1. The catalog is built into each release, so a catalog change reaches users only in a CLI release ([ADR-0001](adr/0001-catalog-delivery.md)).
- **Running asset content while installing.** Grimoire runs no install hooks and never starts the tool and MCP server binaries it configures ([ADR-0001](adr/0001-catalog-delivery.md)).
- **Installing the tool and MCP server binaries.** Grimoire writes only the client wiring. The binaries ship in the release archives and the container image, and are not published to package registries ([ADR-0002](adr/0002-mcp-server-and-tool-packaging.md), separate binaries for each tool and server).
- **Hooks for clients other than Claude Code.** None of the other four clients can guarantee that a failed hook blocks the action, so their adapters refuse hook outputs ([ADR-0008](adr/0008-phase-4-client-adapters.md), the Phase 4 client adapters).
- **`runtime = "docker"`**, a server entry that runs the container image. It is deferred past v1, and `grimoire.toml` rejects the field ([ADR-0011](adr/0011-container-image.md), the container image).
- **Long-lived publishing tokens.** Registries are published through OpenID Connect (OIDC) trusted publishing, and no token is stored in GitHub ([ADR-0003](adr/0003-release-tooling.md), release tooling, and [ADR-0009](adr/0009-release-pipeline.md)).
- **Code that isn't Rust**, except the listed exceptions ([ADR-0010](adr/0010-language-policy-exceptions.md), the language-policy exceptions).

## Budgets

No budgets are set yet. Nothing measures performance: the **Performance budgets** in `.claude/rules/testing.md` read "none". The backlog in [`roadmap.md`](roadmap.md) holds the item that would set them, with its trigger. Each budget, once set, goes here as a number with its unit and the hardware it is measured on.

## Assumptions

Revisit these if any turns out to be wrong.

- **Client formats keep changing about monthly.** Each adapter change starts by re-reading the client's format at its primary source and updating its entry in [`specs.md`](specs.md), and snapshot tests make every change in output visible.
- **The six clients are the v1 set** ([ADR-0005](adr/0005-client-priority.md), client priority, amended by [ADR-0008](adr/0008-phase-4-client-adapters.md)).
- **Embedding the catalog is acceptable:** users get catalog changes with CLI releases ([ADR-0001](adr/0001-catalog-delivery.md)).
- **GitHub-hosted runners exist for all six release targets** ([`03-platforms.md`](03-platforms.md#targets)). GitHub plans to drop Intel macOS runners after the macOS 15 image retires in fall 2027.
- **One maintainer** reviews every change and approves every release ([the repository and contributions](04-quality.md#the-repository-and-contributions)).
- **No release exists yet,** so no published page links to these documents ([ADR-0013](adr/0013-roadmap-and-docs-document-set.md), this document set).

## Scope estimate

Sizes count the lines of Rust in each area, tests included, with `wc -l` over the tracked `.rs` files on 2026-11-24: **S** is under 1,000 lines, **M** 1,000 to 4,999, **L** 5,000 to 9,999, and **XL** 10,000 or more. Planned work is sized by the code it is expected to add.

| Area | Size | Why |
| :- | :- | :- |
| `agent-grimoire-model` | L (5,673) | Every file format, with strict parsing and newtypes for constrained values |
| `agent-grimoire-catalog` | M (4,663) | Catalog loading, catalog-wide validation, native Agent Skills checks, the embedded pack |
| `agent-grimoire-resolve` | M (1,527) | Selection, versions, conflicts, clients and consent |
| `agent-grimoire-install` | L (5,839) | The confined filesystem, three kinds of owned unit in three file formats, the planner and apply |
| `agent-grimoire-adapters` | M (3,373) | Six clients and the placement pass, plus the marketplace's plugin files |
| `agent-grimoire` (the CLI) | L (6,900) | Eleven commands with `--json`, and the end-to-end tests |
| `agent-grimoire-mcp` | M (3,707) | Two transports, limits, Origin checks and token checks |
| `agent-grimoire-testkit` | S (585) | Fixture catalogs and projects |
| Asset crates (`secret-guard`, `project-docs`) | M (3,349) | One tool with hooks, and one MCP server |
| `xtask`, without `release` | M (3,486) | Schemas, validation, packing, the marketplace, scaffolding, repository checks |
| `xtask release` | L (6,537) | Seven release commands and the image commands; the workflows (2,738 lines of YAML) and `packaging/` (1,122 lines) come on top |
| M8.2, the roadmap check | S (planned) | One xtask test over `docs/roadmap.md` and `docs/adr/` |

## Top risks and mitigations

| Risk | Mitigation |
| :- | :- |
| Client formats change, and an adapter writes something a client no longer reads | Each adapter phase re-verifies its formats first ([`specs.md`](specs.md)), and insta snapshots show every change in output. Facts that couldn't be confirmed are marked UNVERIFIED |
| Content installed for one client reaches another | `plan` warns when an agent, a hook or an MCP server reaches a client it wasn't installed for ([cross-client leaks](subsystems/adapters.md#cross-client-leaks)). Some combinations can't avoid duplicate content, and `plan` reports it rather than hiding it |
| Clients enforce agent permissions differently, so refusing to render surprises users | `show` lists the clients each agent supports, and adapters refuse rather than widen ([the adapters spec](subsystems/adapters.md)) |
| The release has never run for real | Every pull request runs the whole pipeline as a dry-run. The bootstrap is a written checklist ([release checklist](05-release.md#release-checklist)), and M8.3 to M8.7 track it |
| The ruleset refuses release-plz's commit, so no release pull request can open | M8.3: the maintainer changes the ruleset ([repository settings](05-release.md#repository-settings)) |
| One account can change everything, and most changes have no second reviewer | Code owners, pinned actions and the release's two approvals; the residual risk is stated in [the repository and contributions](04-quality.md#the-repository-and-contributions) |
| The number of asset binaries grows with the catalog | Each tool and server is its own binary in the archives and the image ([ADR-0002](adr/0002-mcp-server-and-tool-packaging.md)); revisit when release time or archive size becomes a problem |
| Catalog updates need CLI releases | Accepted in [ADR-0001](adr/0001-catalog-delivery.md); a network `Source` is in the backlog with its trigger |

The known gaps of the security model are listed in [`01-architecture.md`](01-architecture.md#known-gaps-and-residual-risks).
