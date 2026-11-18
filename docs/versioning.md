# Versioning and deprecation

- **Status:** written in Phase 7 (2026-11-17), before the first release. **No release exists yet**, so no part of this policy has been applied to a real release.
- **Decisions:** the maintainer's Phase 7 choices and §3 Q6 (the MSRV) in [`PLAN.md`](../PLAN.md); [ADR-0001](adr/0001-catalog-delivery.md) (the catalog is built into each release); [ADR-0009](adr/0009-release-pipeline.md) (release-plz opens the release PR).
- **Sources:** listed at the end, each checked on 2026-11-17.

This file says what Grimoire's version numbers promise, how they are chosen, and how things are retired. It has six parts:
1. the public API;
2. Grimoire's version;
3. the minimum supported Rust version (MSRV);
4. asset versions;
5. deprecation and removal;
6. build metadata.

## Terms

Versions are [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html) numbers, MAJOR.MINOR.PATCH. In this file:
- a **patch release** raises only the last number: 0.3.0 → 0.3.1, or 1.4.2 → 1.4.3;
- a **minor release** raises the middle number: 0.3.1 → 0.4.0, or 1.4.2 → 1.5.0;
- a **major release** raises the first number: 1.4.2 → 2.0.0.

A patch release never counts as a minor release. While Grimoire is 0.x there are no major releases, and a breaking change ships in the next minor release instead (§2).

SemVer itself says that in 0.y.z anything may change at any time (rule 4). Grimoire still uses its 0.x numbers to say when something breaks, as below.

## 1. The public API

SemVer asks a project to declare its public API (rule 1). Grimoire's is:

- **The commands**, their arguments and options, the global options `--json`, `--no-input` and `-C` ([`cli.md`](cli.md)), and the environment the CLI reads ([`cli.md`](cli.md#environment)). That includes value syntax such as `bundle:ID`, `tag:TAG` and the permission names `--allow` takes.
- **The exit codes**, 0 to 6 ([`cli.md`](cli.md#exit-codes)).
- **The `--json` envelope, format 1:** its keys `format`, `command`, `ok`, `exit`, `result` and `diagnostics`; each diagnostic's keys `severity`, `code`, `message`, `file` and `field`; and the `result` keys that [`cli.md`](cli.md#json-output) documents for each command.
- **The diagnostic codes** listed in [`diagnostics.md`](diagnostics.md), such as `resolve/permission`. A code keeps its meaning. Its message text is not part of the API.
- **`grimoire.toml` and `grimoire.lock`, format 1** ([`configuration.md`](configuration.md)).
- **For catalog authors, the `asset.toml` and bundle formats, format 1** ([`catalog/README.md`](../catalog/README.md)).

The JSON Schemas in `schemas/` describe the four file formats. The `--json` envelope has no schema.

**Not part of the public API:**
- **The library crates' Rust APIs.** `agent-grimoire-model`, `-catalog`, `-resolve`, `-install` and `-adapters` are internal to the CLI. They are published only so that `cargo install agent-grimoire` can build it, so don't depend on them. A change that breaks one is still marked breaking (§2), so that it ships as the next 0.y, which the CLI's requirements on its libraries exclude.
- **The `agent-grimoire` crate's library target:** `run`, `Context` and `Exit`. It exists so that the CLI's tests can drive the CLI with a fixture catalog. Nothing else depends on it, so a change to it isn't breaking. Use the binary.
- **The catalog pack**, the archive of the catalog that is built into the binary (`grimoire catalog pack v1`).
- **Human output:** help text, messages on stdout and stderr, diagnostic message text, and the layout of `plan`'s diffs. Scripts should use `--json` and the diagnostic codes.
- **`result` content that `cli.md` doesn't document.** A result may gain keys in any release, so ignore keys you don't know.
- **Codes that only `cargo xtask validate` prints** for catalog authors, such as `manifest/…` and `catalog/…`.

Two more things change on their own terms:
- **Assets** have versions of their own (§4). The catalog changes in many releases, and those changes follow the asset rules, not the CLI's version.
- **What Grimoire writes for each client** follows that client's documented format ([ADR-0008](adr/0008-phase-4-client-adapters.md)), and changes when the client's format changes. After an upgrade, `grimoire plan` shows such a change before `apply` makes it.

**Breaking changes to the public API** include:
- removing or renaming a command, option or argument, or changing what one does;
- changing what an exit code means, or which exit code an outcome gets;
- removing or renaming a key of the envelope, of a diagnostic, or a documented `result` key, or changing its type;
- removing or renaming a diagnostic code, or using it for another condition;
- making a valid `grimoire.toml`, `grimoire.lock`, `asset.toml` or bundle file invalid, or changing what it means;
- a new `format` number (§5).

New commands, options, envelope and `result` keys, diagnostic codes and optional file fields are not breaking. Older CLIs refuse fields they don't know, so a project that uses a new field needs a CLI that has it.

## 2. Grimoire's version

- **One version** covers the CLI (`agent-grimoire`) and its five libraries. It is set once, as `version` in `[workspace.package]` of the root `Cargo.toml`: 0.1.0 today. The npm packages and the PyPI wheels carry the same version; PyPI spells a pre-release in its own form (`1.0.0-rc.1` becomes `1.0.0rc1`), so a pre-release must be `-alpha.N`, `-beta.N` or `-rc.N`.
- **The tag is `vX.Y.Z`.** The release workflow creates it at the commit it built and signed ([ADR-0009](adr/0009-release-pipeline.md)). The version itself is `X.Y.Z`, without the `v`, and `grimoire --version` prints `grimoire X.Y.Z`.
- **release-plz chooses the next version** from the Conventional Commits since the last release, and writes the new `CHANGELOG.md` section, in the release PR ([`release.md`](release.md) §1). The libraries' commits go into the CLI's changelog. With release-plz 0.4.3, while the version is 0.x:
  - a `feat:` or `fix:` commit raises the patch number: 0.1.0 → 0.1.1;
  - only a commit marked breaking, with `!` after the type (`feat!:`) or a `BREAKING CHANGE:` footer, raises the middle number: 0.1.1 → 0.2.0.

  From 1.0, a commit marked breaking raises the first number.

  These are release-plz's rules for one package. How it applies them to Grimoire's shared version is UNVERIFIED: only `agent-grimoire` writes a changelog, and a commit that changes only files outside `crates/`, such as an MSRV raise (§3), may count for no package. A release-plz test suggests that a breaking commit in any package that inherits the workspace version sets the next version for all of them. So the maintainer checks the release PR's version before merging it ([`release.md`](release.md) §1).
- **So breaking changes must be marked.** `release-plz.toml` turns `semver_check` off, so no tool detects an unmarked break; review has to. Mark a commit breaking when it:
  - breaks the public API (§1);
  - breaks the public Rust API of one of the five library crates. The CLI and the libraries depend on each other with Cargo requirements such as `0.1.0`, which Cargo reads as `>=0.1.0, <0.2.0`: every 0.1.z counts as compatible. Released as 0.1.z, such a break could let Cargo build an older CLI against a library it doesn't compile with. It is marked breaking so that it ships as the next 0.y, which those requirements exclude;
  - raises the MSRV, while Grimoire is 0.x (§3);
  - introduces a new `format` number (§5).

  [`CONTRIBUTING.md`](../CONTRIBUTING.md) has the commit conventions.
- **A released version is never changed** (SemVer rule 3). A mistake is fixed in a new release, never by reusing a version ([`release.md`](release.md) §3). crates.io refuses a version that differs from a published one only in build metadata, and once immutable releases are enabled, GitHub locks each release's tag and files.

## 3. The minimum supported Rust version

- **The MSRV** is `rust-version` in the root `Cargo.toml`: **1.96**. CI's MSRV job runs `cargo +1.96.0 check --workspace --all-targets --locked`. Development uses the toolchain pinned in `rust-toolchain.toml`, 1.100.0.
- **The policy** (`PLAN.md` §3 Q6): the MSRV is set four minor Rust releases behind the latest stable one, about six months: 1.96 when stable was 1.100. It moves only by a deliberate change, and never in a patch release.
- **While Grimoire is 0.x,** the commit that raises `rust-version` is marked breaking, so that it ships in the next minor release. That is stricter than Cargo's SemVer guide, which recommends treating an MSRV raise as a minor change, and in 0.x counts a change to the last number (0.y.Z) as minor. The commit changes files outside `crates/`, such as the root `Cargo.toml` and `ci.yml`, and how release-plz counts such a commit is UNVERIFIED (§2), so the release PR's version is checked ([`release.md`](release.md) §1).
- **From 1.0,** an MSRV raise ships in a minor release, which is how Cargo's SemVer guide recommends treating it, and still never in a patch release. Its commit is then typed `feat:` and not marked breaking, because marked breaking it would make a major release. That release-plz raises the middle number for a `feat:` from 1.0 is UNVERIFIED: the rules read cover 0.x, and breaking commits from 1.0.
- **For users:** building from crates.io needs Rust 1.96 or later. Install with `cargo install agent-grimoire --locked`:
  - On Cargo 1.96 to 1.99, `--locked` makes Cargo use the `Cargo.lock` published in the crate. It holds the dependency versions CI checks on Rust 1.96, and the versions of Grimoire's libraries and catalog that the release was built with. Without it, Cargo resolves the dependencies again without regard to `rust-version`: it may pick versions that need a newer compiler, and a later release of each Grimoire library within the same 0.y. `--version X.Y.Z` then pins only the `agent-grimoire` crate. The catalog is built into `agent-grimoire-catalog`, so such a build can carry newer assets than its version says (§4).
  - On Cargo 1.100, Cargo uses the published `Cargo.lock` by default, and `--locked` changes nothing. Later Cargo releases are UNVERIFIED.
  - On a Rust older than the MSRV, Cargo stops with an error. It doesn't fall back to an older Grimoire.

## 4. Asset versions

Each asset and each bundle has its own SemVer version, in its `asset.toml` or bundle file, independent of the CLI's ([`architecture.md`](architecture.md) §5). These versions carry no build metadata. A tool's or MCP server's crate has the same version as its asset (`crate/version`).

### How a new version reaches a project

- **Each CLI release holds one version of each asset** ([ADR-0001](adr/0001-catalog-delivery.md)). A changed asset reaches users only in a CLI release. The exception is a build from crates.io without the published lockfile (Cargo 1.96 to 1.99 without `--locked`, §3), which can hold a later release's catalog.
- **`grimoire add` writes a caret requirement** for the catalog's version, such as `^0.2.0`. Requirements work as in Cargo: `^0.2.0` accepts 0.2.0 and any later 0.2.z, and `^1.4.0` accepts 1.4.0 and any later 1.y.z.
- **`grimoire.lock` pins the installed version.** After an upgrade to a CLI whose catalog has another version, `plan` and `apply` stop with `resolve/locked-version-unavailable` until you run `grimoire update`. If the new version doesn't meet the requirement in `grimoire.toml`, resolution fails with `resolve/version` until you change the requirement.

So the version number tells a project what an update means, for each asset it lists under `[assets]`: a version within the requirement arrives through `grimoire update`, and a breaking one needs a decision first.

An asset that a tag, a bundle or another asset's `requires` selects has no requirement from the project. A tag sets none, and validate makes each bundle's and each `requires` entry's requirement match the catalog's version. So the next `grimoire update` moves such an asset to whatever version the new grimoire carries, a breaking one included. To review the change first, run `grimoire update --no-apply`, then `grimoire plan`, before `apply` makes it. To hold an asset back, list it under `[assets]` with a requirement: a version outside it then stops resolution with `resolve/version`.

### What needs a new version (checked)

`cargo xtask validate --base <rev>` compares the catalog with the merge base of `<rev>`. CI passes the pull request's base branch, or the previous tip for a push. It refuses:
- a change to `asset.toml`, or to any other file in the asset directory except `README.md`, `CHANGELOG.md`, `evals/` and `tests/`, without a higher version (`version/not-bumped`);
- an asset version lower than the base's (`version/decreased`);
- a change to a bundle file without a higher version, a lower one included (`version/not-bumped`).

The catalog checks also require a `CHANGELOG.md` heading for the asset's current version, `## 1.2.3` or `## [1.2.3]` (`catalog/changelog`). Removing a bundle gives a warning, `version/bundle-removed`: projects that select it stop resolving. Removing or renaming an asset has rules of its own (§5).

Projects select a bundle by its id alone, with no version requirement, so a bundle's version tells readers that something changed but doesn't hold back an update.

### Tool and MCP server binaries (not checked)

Every CLI release builds every tool and MCP server binary again, and names each archive after the asset's version (`<binary>-<asset version>-<target>`). A binary can change while nothing in its asset directory does: through `agent-grimoire-mcp`, a dependency update in the root `Cargo.lock`, or a new toolchain. validate doesn't see those changes, so two releases can ship different binaries under one asset version, and the binary's `--version` doesn't tell them apart.

The rule, which review checks: a change that alters what a tool or MCP server binary does, and every security fix to one, raises the asset's version (and its crate's, `crate/version`) and adds a CHANGELOG entry, in the same pull request. Users identify a build by the release it comes from and its SHA-256 in that release's `SHA256SUMS` ([`SECURITY.md`](../SECURITY.md#supported-versions)).

### Which number to raise (a guideline)

validate checks only that the version went up. Which number goes up is a guideline, and review checks it. It follows the caret rule, because projects' requirements use it: a requirement accepts every later version with the same left-most non-zero number.

| Change | From 1.0.0 | Before 1.0.0 |
|---|---|---|
| Breaking (below) | first number: 1.4.2 → 2.0.0 | middle number: 0.2.1 → 0.3.0 |
| Compatible addition | middle number: 1.4.2 → 1.5.0 | last number: 0.2.1 → 0.2.2 |
| Fix | last number: 1.4.2 → 1.4.3 | last number: 0.2.1 → 0.2.2 |

Don't raise more than the change needs. A project's `^0.2.1` doesn't accept 0.3.0, so an unneeded breaking bump makes every project change its requirement.

**Breaking for an asset**, as a guideline, since validate can't tell:
- removing a client's table, or an output a client gets;
- moving an output: a new destination or file name, or a renamed MCP server, tool binary or skill file;
- letting the asset do more: a new permission or environment variable, a new hook binding, or a wider agent tool list. Where that needs a permission the project hasn't accepted, its next `grimoire update` stops with `resolve/permission` until someone accepts it;
- a new `requires` or `conflicts` entry, which can stop a project that resolves today from resolving;
- for tools and MCP servers: removing or renaming a subcommand, option, MCP tool or tool argument, or changing exit codes or output that hooks, clients or scripts rely on;
- changing what a rule, skill or agent tells the model to do so that it contradicts what it said before. Rewording, clarifying or extending the text is not breaking.

### Maturity

Every asset has a `maturity`, and `grimoire list --maturity` filters on it:
- **`experimental`** may change incompatibly in any release. Its version still says so: a breaking change raises the left-most non-zero number, so that requirements catch it.
- **`stable`** follows SemVer strictly, with the rules above.
- **`deprecated`** names its replacement. It is kept for projects that list it under `[assets]`, or get it through another asset's `requires` or a bundle that still lists it. Tags don't select it, so a project that has it only through `tags` loses it (§5).

### `min-cli-version`

Each `asset.toml` names the oldest CLI that can install the asset. When the running CLI is older, resolution fails with `resolve/cli-too-old`. Raise it when the asset needs a CLI feature that older releases lack.

Today every CLI reads only the catalog built into it, and a release's own catalog needs no newer CLI unless the value is set too high. But a CLI built from crates.io without the published lockfile (Cargo 1.96 to 1.99 without `--locked`, §3) can be built with a newer `agent-grimoire-catalog` than its release had. It then meets assets newer than itself, and this check guards it. The check is also there for a catalog source outside the binary, which ADR-0001 leaves for later. Nothing checks that it is at most the CLI version the asset ships in, so review does: an asset whose `min-cli-version` is above its own release can't be installed from that release.

## 5. Deprecation and removal

The deprecation window is one release (the maintainer's decision). A deprecated asset ships in at least one CLI release before it is removed, a deprecated command or option warns for at least one minor release, and the CLI reads the previous format of a file for at least one more minor release.

For an asset, the window protects projects that list it under `[assets]`, and those that get it through another asset's `requires` or a bundle that still lists it. It doesn't protect a project that selects the asset only through `tags` (step 2 below).

### Assets

1. **Deprecate it.** Set `maturity = "deprecated"` and `deprecated = { replacement = "<id>", reason = "<why>" }` in `asset.toml`, raise the version (a compatible change), and add the CHANGELOG entry. validate refuses a `deprecated` table without that maturity, or that maturity without the table, or an asset that replaces itself (`manifest/deprecation`). The replacement must be in the catalog and of the same kind (`catalog/replacement-missing`, `catalog/replacement-kind`). validate warns when the replacement is deprecated too (`catalog/replacement-deprecated`), and when an asset or a bundle still uses the deprecated one (`catalog/requires-deprecated`, `bundle/member-deprecated`).
   - **A bundle keeps a deprecated member for the window,** and review checks it. `bundle/member-deprecated` is only a warning, but dropping the member removes the asset from every project that has it only through the bundle, with no `resolve/deprecated` warning. The member leaves the bundle when the asset is removed (step 3).
2. **While it is deprecated,** resolving a project that uses it warns with `resolve/deprecated`, which names the replacement and the reason. `tags` in `grimoire.toml` no longer select it. So a project that has the asset only through `tags` loses it at its first `apply` or `update` after the release that deprecates it: `plan` shows it as `delete`, and no `resolve/deprecated` warning names the replacement. Before upgrading, such a project should add the asset, or its replacement, under `[assets]`. If it is a marketplace plugin, the plugin's description starts by saying it is deprecated and naming the replacement.
3. **Remove it only after it has shipped deprecated in at least one CLI release.** Any release counts, a patch release included. validate refuses a removal unless the asset was deprecated at the comparison base (`version/removed-without-deprecation`), but it can't see releases. So review checks that a release tag has the asset deprecated: `git show vX.Y.Z:catalog/<kind>/<id>/asset.toml`.
4. **After removal,** a project that still selects the asset fails with `resolve/unknown-asset` until it drops it.

Renaming an asset counts as removing it. Add the asset under its new id, deprecate the old id with the new one as its replacement, and remove the old id later.

### Commands and options

A deprecated command or option keeps working, with a warning, in at least one minor release before a breaking release removes it. With the terms above, a deprecation first released in 0.3.0 can be removed in 0.4.0, and one first released in 0.3.1, a patch release, not before 0.5.0. From 1.0, the removal needs a major release, after at least one minor release that warned, as SemVer's FAQ describes.

**There is no tooling for this yet.** clap 4.6.8, which parses Grimoire's command line, has no way to mark an argument or a subcommand deprecated. The procedure:
1. Keep the old name working as its own hidden argument or subcommand (`hide = true`) that does what the new one does, or detect it in the raw arguments before clap parses them. A hidden `alias` alone won't do: clap reports a match under the original name, so the CLI can't tell that the old name was used.
2. When the old name is used, report a warning diagnostic with a stable code of its own that names the replacement. A diagnostic goes to stderr, or into the `--json` envelope, so scripts see it too. No such code exists yet; list the new one in [`diagnostics.md`](diagnostics.md), and mark the old name deprecated in [`cli.md`](cli.md). The docs test in `crates/agent-grimoire/src/docs_check.rs` counts hidden arguments and subcommands: list a hidden argument in its command's option table, and give a hidden subcommand its own `## grimoire <name>` section. A name detected before clap parses the arguments isn't clap's, so describe it in prose, outside the option tables, or the test fails.
3. Test in `crates/agent-grimoire/tests/cli.rs` that the old name still works and warns, with and without `--json`.
4. Say in the commit message what is deprecated and what replaces it, so that it reaches the CHANGELOG.
5. Remove it later, once the window has passed, in a commit marked breaking.

### File formats

`grimoire.toml`, `grimoire.lock`, `asset.toml` and bundle files start with `format = 1`, and the `--json` envelope with `"format": 1`. Only format 1 exists. Grimoire refuses any other number in the four files ("this version of Grimoire supports only format 1").
- **A change that leaves every valid file valid, with the same meaning, keeps the number**, for example a new optional field.
- **Any other change needs a new format number.** That is breaking, because older CLIs refuse the new number. Its commit is marked breaking, and the CHANGELOG says how to migrate.
- **`grimoire.toml` and `grimoire.lock` live in users' repositories,** so the CLI keeps reading the previous format for at least one more minor release after the one that introduces the new format. Format 2 in 0.5.0 means 0.6.0 still reads format 1, and 0.7.0 is the first release that may not. Dropping the old format is itself a breaking change.
- **`asset.toml` and bundle files live only in this repository's catalog,** which is packed into each build, so the change that introduces a new format for them converts the whole catalog.
- **The `--json` envelope** is written by the CLI and never read by it, so the reading window doesn't cover it. A new envelope format number is a breaking change for scripts. A script should check `format` before it reads anything else.

## 6. Build metadata

SemVer allows build metadata after a `+`, as in `1.2.3+abc`. SemVer and Cargo both ignore it when they compare versions.
- **Grimoire's own versions never carry it:** not the CLI's, the libraries', an asset's or a bundle's, and not `min-cli-version`. The model refuses it, because the `semver` crate gives build metadata an order that SemVer says it doesn't have. crates.io also refuses a version that differs from a published one only in build metadata.
- **Marketplace plugins carry it.** Each generated `plugin.json` version is the asset's or bundle's version plus the first 12 hex digits of a SHA-256 over the plugin's files, such as `0.2.0+28df06dba97e`. Claude Code keeps a user on its cached copy of a plugin until the version string changes, and a plugin's files depend on the generator as well as on the catalog ([`architecture.md`](architecture.md) §10). The hash makes the string change whenever the files do. It doesn't order two builds of one version, since SemVer and Cargo treat them as equal. Whether Claude Code compares plugin versions or only their strings is UNVERIFIED.

## Sources

Each was read on 2026-11-17.

| Source | Version | Used for |
|---|---|---|
| [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html) | 2.0.0 | Rules 1, 3, 4 and 10; the FAQ on deprecation and on the `v` prefix |
| Cargo Book: [specifying dependencies](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/doc/book/src/reference/specifying-dependencies.md), [SemVer compatibility](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/doc/book/src/reference/semver.md), [`rust-version`](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/doc/book/src/reference/rust-version.md), [`cargo install`](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/doc/man/cargo-install.md) | Cargo 1.100.0 (`3acc79c0`) | Caret requirements and 0.x; the MSRV as a minor change; installing with the packaged lockfile |
| [`cargo install` source](https://github.com/rust-lang/cargo/blob/30a34c6821b57de0aaec83a901aca39f88f6778c/src/cargo/ops/cargo_install.rs) and the man pages of Cargo 1.96 to 1.99 | Cargo 1.96.0 (`30a34c68`) to 1.99.1 | The packaged lockfile is ignored without `--locked` |
| [Rust 1.100.0 release notes](https://github.com/rust-lang/rust/blob/1.100.0/RELEASES.md) | 1.100.0 | `cargo install` uses the packaged lockfile by default |
| [crates.io: unique versions without build metadata](https://github.com/rust-lang/crates.io/blob/dfefd631824936145a24e16b0dd3447dc6a113a0/migrations/2024-10-25-112826_make-unique-version-unique/up.sql) | `dfefd631` | A version that differs only in build metadata is refused |
| [release-plz `next_version`](https://github.com/release-plz/release-plz/blob/92e907a14715182c99c26eb734a4b3d7b3775a0b/crates/next_version/src/lib.rs) | 0.4.3 | How a 0.x version is bumped |
| [clap](https://github.com/clap-rs/clap/tree/v4.6.8), [`ArgMatches`](https://github.com/clap-rs/clap/blob/v4.6.8/clap_builder/src/parser/matches/arg_matches.rs) | 4.6.8 | No deprecation support; aliases can't be told apart |
