# 05 — Release

This document says how Grimoire is versioned, built and released: the versioning and deprecation policy, the build profiles, the maintainer's release checklist and recovery steps, and how users install and verify a release. How the release pipeline works, job by job, is in its spec, [`subsystems/release.md`](subsystems/release.md), and the container image's design is in [`subsystems/container-image.md`](subsystems/container-image.md).

- **Status:** **No release exists yet.** No part of this policy has been applied to a real release, and no command in this file has been run against one. The image's build, checks, scan, push to a local registry and the tag script's cases ran locally with rootless Docker (the Docker daemon running as an ordinary user, not as root), and on GitHub in the first Release dry-run (a run that builds and checks everything but publishes nothing), Release run 44353090397 on 2026-11-18. None of it has run against the GitHub Container Registry (GHCR).
- **Decisions** (architecture decision records, ADRs): [ADR-0001](adr/0001-catalog-delivery.md) (the catalog is built into each release), [ADR-0003](adr/0003-release-tooling.md) (release-plz and a hand-written pipeline), [ADR-0009](adr/0009-release-pipeline.md) (the pipeline and its channels), [ADR-0010](adr/0010-language-policy-exceptions.md) (the code that isn't Rust) and [ADR-0011](adr/0011-container-image.md) (the container image). The maintainer's Phase 7 choices, which set the deprecation window and what counts as breaking, are noted on task M7.1 in [`roadmap.md`](roadmap.md).
- **Facts:** the `*-P5` and `*-P6` entries in [`specs.md`](specs.md), the verified research of Phases 5 and 6, and SIGN-P6, the research on signing from a reusable workflow. The sources of the versioning policy are listed at the end of [Versioning](#sources).

## Versioning

This section says what Grimoire's version numbers promise, how they are chosen, and how things are retired, in six parts: [the public API](#the-public-api), [Grimoire's version](#grimoires-version), [the minimum supported Rust version](#the-minimum-supported-rust-version) (MSRV), [asset versions](#asset-versions), [deprecation and removal](#deprecation-and-removal), and [build metadata](#build-metadata).

### Terms

Versions are [SemVer 2.0.0](https://semver.org/spec/v2.0.0.html) numbers. Semantic versioning (SemVer) is a specification of what a version number promises. A version has three numbers, MAJOR.MINOR.PATCH, such as 1.4.2. In this file:
- a **patch release** raises only the last number: 0.3.0 → 0.3.1, or 1.4.2 → 1.4.3;
- a **minor release** raises the middle number: 0.3.1 → 0.4.0, or 1.4.2 → 1.5.0;
- a **major release** raises the first number: 1.4.2 → 2.0.0.

A **breaking change** is one that can make something that worked before stop working; [the public API](#the-public-api) lists them, and [Grimoire's version](#grimoires-version) lists the other changes Grimoire marks as breaking.

A patch release never counts as a minor release. While Grimoire is 0.x (its version starts with 0), there are no major releases, and a breaking change ships in the next minor release instead ([Grimoire's version](#grimoires-version)).

SemVer itself says that in 0.y.z anything may change at any time (rule 4 of the specification). Grimoire still uses its 0.x numbers to say when something breaks, as below.

### The public API

SemVer asks a project to declare its public API (rule 1): the parts that users may rely on, and that the version numbers make promises about. Grimoire's is:

- **The commands** of the `grimoire` command-line interface (CLI), their arguments and options, the global options `--json`, `--no-input` and `-C` ([`cli.md`](cli.md)), and the environment variables the CLI reads ([`cli.md`](cli.md#environment)). That includes value syntax such as `bundle:ID`, `tag:TAG` and the permission names `--allow` takes.
- **The exit codes**, 0 to 6 ([`cli.md`](cli.md#exit-codes)).
- **The `--json` envelope, format 1**, the one JSON document a command prints with `--json`: its keys `format`, `command`, `ok`, `exit`, `result` and `diagnostics`; each diagnostic's keys `severity`, `code`, `message`, `file` and `field`; and the `result` keys that [`cli.md`](cli.md#json-output) documents for each command.
- **The diagnostic codes** listed in [`diagnostics.md`](diagnostics.md), the stable names, such as `resolve/permission`, of each error and warning the CLI reports. A code keeps its meaning. Its message text is not part of the API.
- **`grimoire.toml` and `grimoire.lock`, format 1** ([`configuration.md`](configuration.md)).
- **For catalog authors, the `asset.toml` and bundle formats, format 1** ([`catalog/README.md`](../catalog/README.md)). An `asset.toml` describes one asset, and a bundle file names a list of assets.

The JSON Schemas in `schemas/` describe the four file formats. The `--json` envelope has no schema.

**Not part of the public API:**
- **The library crates' Rust APIs.** A crate is a Rust package. `agent-grimoire-model`, `-catalog`, `-resolve`, `-install` and `-adapters` are internal to the CLI. They are published only so that `cargo install agent-grimoire` can build it, so don't depend on them. A change that breaks one is still marked breaking ([Grimoire's version](#grimoires-version)), so that it ships as the next 0.y (the next minor version, such as 0.2.0 after any 0.1.z), which the CLI's requirements on its libraries exclude.
- **The `agent-grimoire` crate's library target:** `run`, `Context` and `Exit`, the library part of the crate rather than the `grimoire` binary. It exists so that the CLI's tests can drive the CLI with a fixture catalog (a small catalog made for tests). Nothing else depends on it, so a change to it isn't breaking. Use the binary.
- **The catalog pack**, the archive of the catalog that is built into the binary (`grimoire catalog pack v1`).
- **Human output:** help text, messages on standard output and standard error (stdout and stderr), diagnostic message text, and the layout of `plan`'s diffs. Scripts should use `--json` and the diagnostic codes.
- **`result` content that `cli.md` doesn't document.** A result may gain keys in any release, so ignore keys you don't know.
- **Codes that only `cargo xtask validate` prints** for catalog authors, such as `manifest/…` and `catalog/…`.

Two more things change on their own terms:
- **Assets**, the catalog entries Grimoire installs (rules, skills, agents, tools and Model Context Protocol (MCP) servers), have versions of their own ([asset versions](#asset-versions)). The catalog changes in many releases, and those changes follow the asset rules, not the CLI's version.
- **What Grimoire writes for each client** (each AI coding tool it writes for) follows that client's documented format ([ADR-0008](adr/0008-phase-4-client-adapters.md)), and changes when the client's format changes. After an upgrade, `grimoire plan` shows such a change before `apply` makes it.

**Breaking changes to the public API** include:
- removing or renaming a command, option or argument, or changing what one does;
- changing what an exit code means, or which exit code an outcome gets;
- removing or renaming a key of the envelope, of a diagnostic, or a documented `result` key, or changing its type;
- removing or renaming a diagnostic code, or using it for another condition;
- making a valid `grimoire.toml`, `grimoire.lock`, `asset.toml` or bundle file invalid, or changing what it means;
- a new `format` number, the number that names the version of a file format or of the `--json` envelope ([deprecation and removal](#deprecation-and-removal)).

New commands, options, envelope and `result` keys, diagnostic codes and optional file fields are not breaking. Older CLIs refuse fields they don't know, so a project that uses a new field needs a CLI that has it.

### Grimoire's version

- **One version** covers the CLI (`agent-grimoire`) and its five libraries. It is set once, as `version` in `[workspace.package]` of the root `Cargo.toml`: 0.1.0 today.
- **The npm packages and the PyPI wheels carry the same version.** npm is the JavaScript package registry, PyPI (the Python Package Index) is Python's, and a wheel is a prebuilt Python package. PyPI spells a pre-release (a test version before a release, such as `1.0.0-rc.1`) in its own form (`1.0.0-rc.1` becomes `1.0.0rc1`), so a pre-release must be `-alpha.N`, `-beta.N` or `-rc.N`.
- **The tag is `vX.Y.Z`.** The release workflow creates this Git tag at the commit it built and signed ([ADR-0009](adr/0009-release-pipeline.md)). The version itself is `X.Y.Z`, without the `v`, and `grimoire --version` prints `grimoire X.Y.Z`.
- **release-plz chooses the next version** from the commits since the last release, which follow Conventional Commits (a message format that starts with a type, such as `feat:` or `fix:`). It also writes the new `CHANGELOG.md` section, in the release pull request ([How a release flows](subsystems/release.md#how-a-release-flows)). The libraries' commits go into the CLI's changelog. With release-plz 0.4.3, while the version is 0.x:
  - a `feat:` or `fix:` commit raises the patch number: 0.1.0 → 0.1.1;
  - only a commit marked breaking, with `!` after the type (`feat!:`) or a `BREAKING CHANGE:` footer, raises the middle number: 0.1.1 → 0.2.0.

  From 1.0, a commit marked breaking raises the first number.

  These are release-plz's rules for one package. How it applies them to Grimoire's shared version is UNVERIFIED, meaning it wasn't confirmed from a primary source:
  - only `agent-grimoire` writes a changelog;
  - a commit that changes only files outside `crates/`, such as an MSRV raise ([the MSRV](#the-minimum-supported-rust-version)), may count for no package;
  - a release-plz test suggests that a breaking commit in any package that inherits the workspace version (takes its version from the root `Cargo.toml`) sets the next version for all of them.

  So the maintainer checks the release pull request's version before merging it ([How a release flows](subsystems/release.md#how-a-release-flows)).
- **So breaking changes must be marked.** `release-plz.toml` turns `semver_check` off. That option would check the crates' public Rust APIs with cargo-semver-checks, and the comment in `release-plz.toml` gives the reason it is off: the tool would be one more unverified download. So no tool detects an unmarked break; review has to. Mark a commit breaking when it:
  - breaks the public API ([the public API](#the-public-api));
  - breaks the public Rust API of one of the five library crates. The CLI and the libraries depend on each other with Cargo requirements such as `0.1.0`, which Cargo reads as `>=0.1.0, <0.2.0`: every 0.1.z counts as compatible. Released as 0.1.z, such a break could let Cargo build an older CLI against a library it doesn't compile with. It is marked breaking so that it ships as the next 0.y, which those requirements exclude;
  - raises the MSRV, while Grimoire is 0.x ([the MSRV](#the-minimum-supported-rust-version));
  - introduces a new `format` number ([deprecation and removal](#deprecation-and-removal)).

  [`CONTRIBUTING.md`](../CONTRIBUTING.md) has the commit conventions.
- **A released version is never changed** (SemVer rule 3). A mistake is fixed in a new release, never by reusing a version ([Recovery](#recovery)).
  - crates.io, Rust's package registry, refuses a version that differs from a published one only in build metadata (the part after a `+`, [build metadata](#build-metadata)).
  - Once immutable releases are enabled (a GitHub repository setting), GitHub locks each release's tag and files.

### The minimum supported Rust version

- **The MSRV**, the oldest Rust release that Grimoire supports, is `rust-version` in the root `Cargo.toml`: **1.96**. Continuous integration (CI), the checks that run on every pull request and every push to `main`, has an MSRV job that runs `cargo +1.96.0 check --workspace --all-targets --locked`. Development uses the Rust toolchain (the compiler and its tools) pinned in `rust-toolchain.toml`, 1.100.0.
- **The policy** (the maintainer's answer to open question Q6 on 2026-11-16, noted on task M1.1 in [`roadmap.md`](roadmap.md)): the MSRV is set four minor Rust releases behind the latest stable one, about six months: 1.96 when stable was 1.100. It moves only by a deliberate change, and never in a patch release.
- **While Grimoire is 0.x,** the commit that raises `rust-version` is marked breaking, so that it ships in the next minor release.
  - That is stricter than Cargo's SemVer guide. The guide recommends treating an MSRV raise as a minor change, and in 0.x it counts a change to the last number (0.y.Z) as minor.
  - The commit changes files outside `crates/`, such as the root `Cargo.toml` and `ci.yml`. How release-plz counts such a commit is UNVERIFIED ([Grimoire's version](#grimoires-version)), so the release pull request's version is checked ([How a release flows](subsystems/release.md#how-a-release-flows)).
- **From 1.0,** an MSRV raise ships in a minor release, which is how Cargo's SemVer guide recommends treating it, and still never in a patch release.
  - Its commit is then typed `feat:` and not marked breaking, because marked breaking it would make a major release.
  - That release-plz raises the middle number for a `feat:` from 1.0 is UNVERIFIED: the release-plz rules that were read cover 0.x, and breaking commits from 1.0.
- **For users:** building from crates.io needs Rust 1.96 or later. Install with `cargo install agent-grimoire --locked`. `Cargo.lock` is a lockfile: it records the exact version of every dependency.
  - **On Cargo 1.96 to 1.99,** `--locked` makes Cargo use the `Cargo.lock` published in the crate. It holds the dependency versions CI checks on Rust 1.96, and the versions of Grimoire's libraries and catalog that the release was built with.
  - **On those versions without `--locked`,** Cargo resolves the dependencies again (chooses their versions afresh) without regard to `rust-version`. It may pick versions that need a newer compiler, and a later release of each Grimoire library within the same 0.y. `--version X.Y.Z` then pins only the `agent-grimoire` crate. The catalog is built into `agent-grimoire-catalog`, so such a build can carry newer assets than its version says ([asset versions](#asset-versions)).
  - **On Cargo 1.100,** Cargo uses the published `Cargo.lock` by default, and `--locked` changes nothing. Later Cargo releases are UNVERIFIED.
  - **On a Rust older than the MSRV,** Cargo stops with an error. It doesn't fall back to an older Grimoire.

### Asset versions

Each asset and each bundle (a named list of assets) has its own SemVer version, in its `asset.toml` or bundle file, independent of the CLI's ([the model spec](subsystems/model.md#identifiers-and-versions)). These versions carry no build metadata. A tool's or MCP server's crate has the same version as its asset, and catalog validation checks it (`crate/version`).

#### How a new version reaches a project

- **Each CLI release holds one version of each asset** ([ADR-0001](adr/0001-catalog-delivery.md)). A changed asset reaches users only in a CLI release. The exception is a build from crates.io without the published lockfile (Cargo 1.96 to 1.99 without `--locked`, [the MSRV](#the-minimum-supported-rust-version)), which can hold a later release's catalog.
- **`grimoire add` writes a caret requirement**, one that starts with `^`, for the catalog's version, such as `^0.2.0`. Requirements work as in Cargo: `^0.2.0` accepts 0.2.0 and any later 0.2.z, and `^1.4.0` accepts 1.4.0 and any later 1.y.z.
- **`grimoire.lock` pins the installed version.** After an upgrade to a CLI whose catalog has another version, `plan` and `apply` stop with `resolve/locked-version-unavailable` until you run `grimoire update`.
- **A new version must meet the requirement.** If it doesn't meet the requirement in `grimoire.toml`, resolution (working out which asset versions to install) fails with `resolve/version` until you change the requirement.

So the version number tells a project what an update means, for each asset it lists under `[assets]`: a version within the requirement arrives through `grimoire update`, and a breaking one needs a decision first.

An asset that a tag, a bundle or another asset's `requires` (its list of dependencies) selects has no requirement from the project. A tag sets none, and catalog validation (`cargo xtask validate`, called validate below) makes each bundle's and each `requires` entry's requirement match the catalog's version. So the next `grimoire update` moves such an asset to whatever version the new grimoire carries, a breaking one included.
- To review the change first, run `grimoire update --no-apply`, then `grimoire plan`, before `apply` makes it.
- To hold an asset back, list it under `[assets]` with a requirement: a version outside it then stops resolution with `resolve/version`.

#### What needs a new version (checked)

`cargo xtask validate --base <rev>` compares the catalog with the merge base of `<rev>`, the newest commit that `<rev>` and the current commit both have in their history. CI passes the pull request's base branch, or, for a push, the branch's previous tip (its latest commit before the push). It refuses:
- a change to `asset.toml`, or to any other file in the asset directory except `README.md`, `CHANGELOG.md`, `evals/` and `tests/`, without a higher version (`version/not-bumped`);
- an asset version lower than the base's (`version/decreased`);
- a change to a bundle file without a higher version, a lower one included (`version/not-bumped`).

The catalog checks also require a `CHANGELOG.md` heading for the asset's current version, `## 1.2.3` or `## [1.2.3]` (`catalog/changelog`).

Removing a bundle gives a warning, `version/bundle-removed`: projects that select it stop resolving. Removing or renaming an asset has rules of its own ([deprecation and removal](#deprecation-and-removal)).

Projects select a bundle by its id alone, with no version requirement, so a bundle's version tells readers that something changed but doesn't hold back an update.

#### Tool and MCP server binaries (not checked)

Every CLI release builds every tool and MCP server binary again, and names each archive after the asset's version (`<binary>-<asset version>-<target>`, where the target names the platform, such as `x86_64-unknown-linux-musl`).

A binary can change while nothing in its asset directory does: through `agent-grimoire-mcp` (the shared library that hardens MCP servers), a dependency update in the root `Cargo.lock`, or a new toolchain. validate doesn't see those changes, so two releases can ship different binaries under one asset version, and the binary's `--version` doesn't tell them apart.

The rule, which review checks: a change that alters what a tool or MCP server binary does, and every security fix to one, raises the asset's version (and its crate's, `crate/version`) and adds a CHANGELOG entry, in the same pull request. Users identify a build by the release it comes from and its SHA-256 in that release's `SHA256SUMS` file ([`SECURITY.md`](../SECURITY.md#supported-versions)).

#### Which number to raise (a guideline)

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

#### Maturity

Every asset has a `maturity`, and `grimoire list --maturity` filters on it:
- **`experimental`** may change incompatibly in any release. Its version still says so: a breaking change raises the left-most non-zero number, so that requirements catch it.
- **`stable`** follows SemVer strictly, with the rules above.
- **`deprecated`** means the asset is being retired, and it names its replacement. It is kept for projects that list it under `[assets]`, or get it through another asset's `requires` or a bundle that still lists it. Tags don't select it, so a project that has it only through `tags` loses it ([deprecation and removal](#deprecation-and-removal)).

#### `min-cli-version`

Each `asset.toml` names the oldest CLI that can install the asset. When the running CLI is older, resolution fails with `resolve/cli-too-old`. Raise it when the asset needs a CLI feature that older releases lack.

Today every CLI reads only the catalog built into it, and a release's own catalog needs no newer CLI unless the value is set too high. The check still has two uses:
- A CLI built from crates.io without the published lockfile (Cargo 1.96 to 1.99 without `--locked`, [the MSRV](#the-minimum-supported-rust-version)) can be built with a newer `agent-grimoire-catalog` than its release had. It then meets assets newer than itself, and this check guards it.
- It is also there for a catalog source outside the binary, which ADR-0001 leaves for later.

`min-cli-version` must be at most the CLI version the asset ships in, because an asset whose `min-cli-version` is above its own release can't be installed from that release. A pre-release sorts before its release, so X.Y.Z-rc.N can't install an asset whose `min-cli-version` is X.Y.Z.

CI checks this on every pull request and push: `crates/agent-grimoire/tests/builtin.rs` fails with `resolve/cli-too-old` when an embedded asset needs a newer grimoire than the workspace version. Between releases that is the last release's version. So an asset that needs a CLI feature that isn't released yet lands after the release that ships the feature, with that release's version.

`release.yml` runs no tests, so `cargo xtask release plan` checks the release itself: a run that releases fails while an asset in the embedded catalog needs a newer grimoire than the version it releases ([How a release flows](subsystems/release.md#how-a-release-flows)).

### Deprecation and removal

To deprecate something is to mark it for removal while it still works. The deprecation window, how long a deprecated thing stays available, is one release (the maintainer's decision):
- a deprecated asset ships in at least one CLI release before it is removed;
- a deprecated command or option warns for at least one minor release;
- the CLI reads the previous format of a file for at least one more minor release.

For an asset, the window protects projects that list it under `[assets]`, and those that get it through another asset's `requires` or a bundle that still lists it. It doesn't protect a project that selects the asset only through `tags` (step 2 below).

#### Assets

1. **Deprecate it.** Set `maturity = "deprecated"` and `deprecated = { replacement = "<id>", reason = "<why>" }` in `asset.toml`, raise the version (a compatible change), and add the CHANGELOG entry.
   - validate refuses a `deprecated` table without that maturity, or that maturity without the table, or an asset that replaces itself (`manifest/deprecation`).
   - The replacement must be in the catalog and of the same kind (`catalog/replacement-missing`, `catalog/replacement-kind`).
   - validate warns when the replacement is deprecated too (`catalog/replacement-deprecated`), and when an asset or a bundle still uses the deprecated one (`catalog/requires-deprecated`, `bundle/member-deprecated`).
   - **A bundle keeps a deprecated member for the window,** and review checks it. `bundle/member-deprecated` is only a warning, but dropping the member removes the asset from every project that has it only through the bundle, with no `resolve/deprecated` warning. The member leaves the bundle when the asset is removed (step 3).
2. **While it is deprecated:**
   - Resolving a project that uses it warns with `resolve/deprecated`, which names the replacement and the reason.
   - `tags` in `grimoire.toml` no longer select it. So a project that has the asset only through `tags` loses it at its first `apply` or `update` after the release that deprecates it: `plan` shows it as `delete`, and no `resolve/deprecated` warning names the replacement. Before upgrading, such a project should add the asset, or its replacement, under `[assets]`.
   - If it is a marketplace plugin (Grimoire also generates Claude Code plugins for eligible assets, [the marketplace spec](subsystems/marketplace.md)), the plugin's description starts by saying it is deprecated and naming the replacement.
3. **Remove it only after it has shipped deprecated in at least one CLI release.** Any release counts, a patch release included. validate refuses a removal unless the asset was deprecated at the comparison base, the merge base of the `--base` revision and the current commit (`version/removed-without-deprecation`), but it can't see releases. So review checks that a release tag has the asset deprecated: `git show vX.Y.Z:catalog/<kind>/<id>/asset.toml`.
4. **After removal,** a project that still selects the asset fails with `resolve/unknown-asset` until it drops it.

Renaming an asset counts as removing it. Add the asset under its new id, deprecate the old id with the new one as its replacement, and remove the old id later.

#### Commands and options

A deprecated command or option keeps working, with a warning, in at least one minor release before a breaking release removes it. With the terms above:
- a deprecation first released in 0.3.0 can be removed in 0.4.0;
- one first released in 0.3.1, a patch release, can't be removed before 0.5.0, because 0.4.0 is the first minor release that warns.

From 1.0, the removal needs a major release, after at least one minor release that warned, as SemVer's FAQ describes.

**There is no tooling for this yet.** clap 4.6.8, which parses Grimoire's command line, has no way to mark an argument or a subcommand deprecated. The procedure:
1. Keep the old name working as its own hidden argument or subcommand (`hide = true`) that does what the new one does, or detect it in the raw arguments before clap parses them. A hidden `alias` alone won't do: clap reports a match under the original name, so the CLI can't tell that the old name was used.
2. When the old name is used, report a warning diagnostic with a stable code of its own that names the replacement. A diagnostic goes to stderr, or into the `--json` envelope, so scripts see it too.
   - No such code exists yet. List the new one in [`diagnostics.md`](diagnostics.md), and mark the old name deprecated in [`cli.md`](cli.md).
   - The docs test in `crates/agent-grimoire/src/docs_check.rs` counts hidden arguments and subcommands: list a hidden argument in its command's option table, and give a hidden subcommand its own `## grimoire <name>` section.
   - A name detected before clap parses the arguments isn't clap's, so describe it in prose, outside the option tables, or the test fails.
3. Test in `crates/agent-grimoire/tests/cli.rs` that the old name still works and warns, with and without `--json`.
4. Say in the commit message what is deprecated and what replaces it, so that it reaches the CHANGELOG.
5. Remove it later, once the window has passed, in a commit marked breaking.

#### File formats

`grimoire.toml`, `grimoire.lock`, `asset.toml` and bundle files start with `format = 1`, and the `--json` envelope with `"format": 1`. Only format 1 exists. Grimoire refuses any other number in the four files ("this version of Grimoire supports only format 1").
- **A change that leaves every valid file valid, with the same meaning, keeps the number**, for example a new optional field.
- **Any other change needs a new format number.** That is breaking, because older CLIs refuse the new number. Its commit is marked breaking, and the CHANGELOG says how to migrate.
- **`grimoire.toml` and `grimoire.lock` live in users' repositories,** so the CLI keeps reading the previous format for at least one more minor release after the one that introduces the new format. Format 2 in 0.5.0 means 0.6.0 still reads format 1, and 0.7.0 is the first release that may not. Dropping the old format is itself a breaking change.
- **`asset.toml` and bundle files live only in this repository's catalog,** which is packed into each build, so the change that introduces a new format for them converts the whole catalog.
- **The `--json` envelope** is written by the CLI and never read by it, so the reading window doesn't cover it. A new envelope format number is a breaking change for scripts. A script should check `format` before it reads anything else.

### Build metadata

SemVer allows build metadata after a `+`, as in `1.2.3+abc`. SemVer and Cargo both ignore it when they compare versions.
- **Grimoire's own versions never carry it:** not the CLI's, the libraries', an asset's or a bundle's, and not `min-cli-version`. The model (the `agent-grimoire-model` crate, which defines Grimoire's file formats) refuses it, because the `semver` crate gives build metadata an order that SemVer says it doesn't have. crates.io also refuses a version that differs from a published one only in build metadata.
- **Marketplace plugins carry it.** Grimoire generates a Claude Code plugin marketplace from the catalog. Each generated `plugin.json` version is the asset's or bundle's version plus the first 12 hex digits of a SHA-256 over the plugin's files, such as `0.2.0+28df06dba97e`.
  - The reason: Claude Code keeps a user on its cached copy of a plugin until the version string changes, and a plugin's files depend on the generator as well as on the catalog ([the marketplace spec](subsystems/marketplace.md)). The hash makes the string change whenever the files do.
  - It doesn't order two builds of one version, since SemVer and Cargo treat them as equal. Whether Claude Code compares plugin versions or only their strings is UNVERIFIED.

### Sources

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

## Build profiles

Release binaries are built with the `dist` profile in the root `Cargo.toml`. It inherits from `release` and sets `lto = "fat"` (link-time optimization across every crate), `codegen-units = 1`, `debug = false`, `strip = "debuginfo"` and `panic = "unwind"`. `cargo xtask release build` builds each package on its own with `cargo auditable build --locked --profile dist`, which embeds the dependency list in the binary for later auditing. Every release build also remaps the workspace and Cargo home paths (`--remap-path-prefix`), so binaries don't embed the runner's directories, and links the C runtime statically on Windows (`-Ctarget-feature=+crt-static`), so no Visual C++ runtime is needed. `xtask/src/release/build.rs` holds these flags. Everyday builds use Cargo's default `dev` and `release` profiles.

## Packaging and channels

One release publishes the CLI to crates.io, npm, PyPI, Homebrew, Scoop and winget, as install scripts and release archives, and as a container image. Each channel carries the same binary. How each channel's files are generated, signed and checked is in [the release spec](subsystems/release.md); what users run to install from each one is in [Installing](#installing).

## Release checklist

This checklist is the source for the bootstrap, the one-time setup before the first release. Tasks M8.3 to M8.7 in [`roadmap.md`](roadmap.md) point here. Do the parts in order:
- **Repository settings:** the items marked "now", "before you merge the Phase 7 change" or "after you merge the Phase 7 change" at that time, and the rest at any time before the first release;
- **Registries and repositories:** close to the first release, because PyPI's pending publisher expires after 30 days;
- **Switch releases on:** last;
- **During the first release run:** while the release workflow runs.

Never store a token in GitHub at any step.

### Repository settings

- [ ] **Now: let release-plz commit to its branch.** Until you do, release-plz can't open the release pull request, so no release can start ([How a release flows](subsystems/release.md#how-a-release-flows)).
  - **What happened.** The Release PR workflow's run 44198210657 failed in both attempts, on 2026-11-17 and 2026-11-18 (recorded under RELEASE-PLZ in [`specs.md`](specs.md)). Each attempt created a `release-plz-<timestamp>` branch, and then GitHub refused its commit there: "Repository rule violations found … Changes must be made through a pull request … Waiting for Code Scanning results. Code Scanning may not be configured for the target branch." All 11 runs up to 2026-11-24 have failed. The most recent, run 45513260333, was refused with only "Changes must be made through a pull request".
  - **Why.** The ruleset `BestBranch` covers every branch (`~ALL`), the `release-plz-*` branches included. In the first runs, its pull-request rule and its code-scanning rule both refused the commit. Read on 2026-11-24, the code-scanning rule is gone (the next item), and the pull-request rule alone still refuses it.
  - **Your decision.** Either target the ruleset at the default branch only (`~DEFAULT_BRANCH`), or keep `~ALL` and exclude `refs/heads/release-plz-*`.
    - With the first, other branches also lose the deletion and force-push rules.
    - With either, pull requests into `main`, the release pull request included, keep every rule.
    - **Read on 2026-11-24:** the ruleset keeps `~ALL` and excludes `refs/heads/refs/heads/release-plz-*`. That pattern has `refs/heads/` twice, so it matches no release-plz branch: `gh api repos/jabbott-iii/Grimoire/rules/branches/release-plz-2026-11-24T23-50-39Z` still lists `BestBranch`'s deletion, force-push, and pull-request rules. Correct the exclusion until `gh api repos/jabbott-iii/Grimoire/rulesets/27435535 --jq .conditions` shows `refs/heads/release-plz-*`.
  - **Don't add GitHub Actions as a bypass actor** (an app or role that the ruleset lets skip its rules) instead. Any workflow job whose token has `contents: write` could then change `main` without a pull request.
  - **Then delete the orphan branches**, the `release-plz-*` branches that the failed runs left behind: `release-plz-2026-11-17T23-18-49Z`, `release-plz-2026-11-18T05-09-04Z`, `release-plz-2026-11-18T14-29-28Z` (run 44324738027, on the push of `dca6c1d`) and `release-plz-2026-11-18T16-57-17Z` (run 44353090254, on the push of `7fc1906`).
    - Read on 2026-11-24, those four are gone, and two newer ones are left: `release-plz-2026-11-23T00-25-50Z` and `release-plz-2026-11-24T23-50-39Z`.
    - Each push to `main` adds another until the ruleset lets release-plz commit, so list them with `gh api 'repos/jabbott-iii/Grimoire/branches?per_page=100' --jq '.[].name'` first.
    - While the ruleset still covers them, its deletion rule allows that only through the administrator bypass.
  - Then run `release-plz.yml` on `main` (Actions → Release PR → Run workflow), or push to `main`, and check that the release pull request opens. Opening it also needs the Actions setting below.
- [ ] **Now: give the code-quality rule its results, or drop it.** When read on 2026-11-17 and 2026-11-18, `BestBranch` also required CodeQL code-scanning results and code-quality results. Code scanning is GitHub's automated search of the code for security problems; CodeQL is GitHub's analysis engine for it.
  - **Read on 2026-11-24, both rules are gone.** `BestBranch` has only its deletion, force-push, and pull-request rules, so no code-scanning or code-quality result blocks a merge or a commit. Tick this item if removing both was your decision.
  - **CodeQL still runs.** `.github/workflows/codeql.yml` analyzes Rust, JavaScript, Python and the workflows, with the `security-extended` queries, on every pull request to `main`, every push to `main` and weekly.
    - It is an advanced setup: the repository's own workflow file, rather than the default setup that GitHub configures by itself.
    - Its actions are pinned like every other workflow's (CODEQL in [`specs.md`](specs.md), the entry on the CodeQL action).
    - Its results reach a pull request as the `CodeQL` check, which pull request 1 passed on 2026-11-19. Nothing requires that check now.
  - **To make CodeQL a gate again,** require it on `main` only: for example, add the `CodeQL` check to the CI ruleset below. CodeQL doesn't run on other branches, so a code-scanning rule on every branch (`~ALL`) would refuse release-plz's commit again (the first item).
  - Don't turn on CodeQL default setup: GitHub doesn't accept advanced-setup results while default setup is on.
- [ ] **Allow GitHub Actions to create and approve pull requests** (Settings → Actions → General). This was due before the Phase 5 merge. Whether it is on couldn't be read: the API answered 403 (forbidden) to the token used.
  - `release-plz.yml` runs on the next push to `main` and opens the release pull request (`chore: release`).
  - With the setting off, each run creates a `release-plz-<timestamp>` branch and then fails to open the pull request. The ruleset leaves such branches too (the first item). Delete any `release-plz-*` branch that has no pull request.
  - Leave the release pull request unmerged until the rest of this bootstrap is done. release-plz updates it on every push.
  - Keep the default `GITHUB_TOKEN` permissions read-only.
- [ ] **Before you merge the Phase 7 change, watch the repository for security alerts.**
  - It is needed as soon as [`SECURITY.md`](../SECURITY.md) is on `main`, because it promises an acknowledgment within 7 days from then.
  - On the repository page, choose Watch → Custom → Security alerts, or All Activity, and keep notifications on. For email, also select Email under Subscriptions → Watching in your notification settings.
  - Otherwise GitHub may not tell you about a private report ([GitHub's docs](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/configure-vulnerability-reporting/configure-for-a-repository), checked 2026-11-17).
  - You are the only collaborator, so nobody else can act on a report.
- [ ] **Confirm that private vulnerability reporting is still on,** before you merge the Phase 7 change and again before the first release. It is GitHub's form for reporting a vulnerability to the maintainer privately.
  - It was on when read on 2026-11-17, and [`SECURITY.md`](../SECURITY.md) sends reports to its form, `https://github.com/jabbott-iii/Grimoire/security/advisories/new`.
  - `gh api repos/jabbott-iii/Grimoire/private-vulnerability-reporting` should show `{"enabled":true}`.
- [ ] **After you merge the Phase 7 change, check the new GitHub files.**
  - GitHub skips a line of CODEOWNERS (the file that names who must review changes to each path) that has an error (COMMUNITY-P7 in [`specs.md`](specs.md)). It also doesn't show some invalid issue forms (ISSUE-FORMS-P7, the entry on issue forms).
  - Blank issues (issues filed without a form) are off, so a missing form leaves no way to file that kind of issue.
  - `gh api repos/jabbott-iii/Grimoire/codeowners/errors` should return an empty `errors` list.
  - Open <https://github.com/jabbott-iii/Grimoire/issues/new/choose>, the page where you choose an issue form, signed in as an account other than yours. It should show the five forms, "Bug report", "Asset request", "Client format change", "Ask for a private contact" and "Propose a change", and the two links, "Report a security vulnerability" and "Documentation", with no second security entry. Whether GitHub adds its own security link to the chooser is UNVERIFIED (ISSUE-FORMS-P7).
  - Record what you saw in `specs.md`, under COMMUNITY-P7 and ISSUE-FORMS-P7.
- [ ] **Create five environments** (Settings → Environments): `release`, `crates-io`, `npm`, `pypi` and `ghcr`. Configure each one the same way:
  - Required reviewer: you.
  - Leave **Prevent self-review** off while you are the only reviewer. With it on, you can't approve a run you started; only the administrator bypass could start it.
  - Deployment branches and tags: "Selected branches and tags", with one branch rule, `main`. Releases run only from `main`, never from a tag.
  - No environment secrets. `ghcr` needs none either: `publish-image` pushes with the job's `GITHUB_TOKEN`.
  - If an environment already exists, configure it the same way. A run that references a missing environment creates it without protection rules.
  - Each release then needs two approval rounds: `release` for the tag and the GitHub Release, then `crates-io`, `npm`, `pypi` and `ghcr` together.
  - `ghcr` approves `publish-image`, the push. `sign-image` follows without an approval of its own, because a job that calls a reusable workflow can't name an environment.
  - `sign-files` has no environment either, and runs before the `release` approval ([the release spec](subsystems/release.md), "Signing comes before approval").
- [ ] **Enable immutable releases** (Settings → Releases → "Enable release immutability").
  - It applies only to releases published afterward.
  - `GITHUB_TOKEN` can't read this setting, so the `publish` job only warns when a release comes out mutable. Check it with your own login: `gh api repos/jabbott-iii/Grimoire/immutable-releases` should show `"enabled": true`.
  - A mutable release gets no release attestation (GitHub's own signed statement that a file belongs to the release), so the tap and bucket syncs can't take it ([Recovery](#recovery)).
- [ ] **The CI ruleset on `main`** (task M8.4 in [`roadmap.md`](roadmap.md)): require the CI, Security and `release checks` checks.
  - `release checks` is the release workflow's summary job. On a pull request it fails unless every dry-run job passed.
  - A skipped job counts as a failure, because a required check whose job is skipped would otherwise count as passed.
  - The release pull request's checks then come from the runs you approve.
  - A ruleset on every branch already requires pull requests, with one approval and code-owner review ([`.github/CODEOWNERS`](../.github/CODEOWNERS)), but no status checks (read on 2026-11-17, and again on 2026-11-24). The administrator role, Dependabot (GitHub's dependency-update bot) and two other GitHub Apps can bypass it; GitHub Actions can't.

### Registries and repositories, close to the first release

**npm.** The npm research left several points UNVERIFIED; they are marked below. Trusted publishing is configured later, in part D, because a new configuration expires after 48 hours.

- [ ] Sign in with `npm login`. It gives a 2-hour session and enforces 2FA for publishing. `npm stage` and `npm trust` need npm 11.15.0 or later.
- [ ] Create the npm organization `agent-grimoire` on npmjs.com. The `@agent-grimoire/*` packages need it. The name was free on 2026-11-17.
- [ ] **Create each of the seven package names by staging a bootstrap version.** The names are `agent-grimoire` and `@agent-grimoire/{linux-x64,linux-arm64,darwin-x64,darwin-arm64,win32-x64,win32-arm64}`.
  - Staged publishing can create a package; trusted publishing can't.
  - For each name, make a directory with a minimal `package.json`:
    - the name;
    - a version CI will never use, such as `0.0.0-bootstrap.0`;
    - a description;
    - `"license": "MIT OR Apache-2.0"`;
    - `"repository": {"type": "git", "url": "git+https://github.com/jabbott-iii/Grimoire.git"}`.
  - Run `npm stage publish --access public --tag bootstrap` in it.
  - npm creates the package and publishes a public placeholder version, `0.0.0-stage`.
  - Use a separate version because staged and published versions share one version index. A prerelease version (one with a suffix, such as `-bootstrap.0`) needs `--tag`.
  - Then approve or reject the bootstrap stage (2FA).
- [ ] **Check on npmjs.com before going on (UNVERIFIED in the docs):**
  - which dist-tag (npm's label for a version; `npm install` installs the one tagged `latest`) the `0.0.0-stage` placeholder has. If it is `latest`, `npm i -g agent-grimoire` installs the placeholder until the first real version is approved;
  - whether rejecting the bootstrap stage keeps the package and its placeholder;
  - whether staged versions expire.
- [ ] Set each package's publishing access to "Require two-factor authentication and disallow tokens". Trusted publishing isn't affected.

**PyPI.**

- [ ] Add a pending publisher on pypi.org (a trusted publisher for a project that doesn't exist yet; the first upload through it creates the project):
  - project `agent-grimoire`;
  - owner `jabbott-iii`;
  - repository `Grimoire` (PyPI matches it exactly);
  - workflow `release.yml`;
  - environment `pypi`.
- [ ] Create it close to the first release. It doesn't reserve the name, and PyPI deletes pending publishers 30 days after they are created. That rule is in PyPI's source code, not its docs.

**Homebrew and Scoop.**

- [ ] Create the public repositories `jabbott-iii/homebrew-tap` and `jabbott-iii/scoop-bucket`.
- [ ] Copy `packaging/homebrew-tap/sync-formula.yml` and `packaging/scoop-bucket/sync-manifest.yml` into `.github/workflows/` in the matching repository. They write `Formula/agent-grimoire.rb` and `bucket/agent-grimoire.json`. Until the first Grimoire release, each run ends with a notice that there is nothing to sync.
- [ ] In each repository:
  - allow GitHub Actions to create and approve pull requests;
  - keep `GITHUB_TOKEN` read-only by default;
  - require actions pinned to a full commit SHA (the full hash of a commit), as here.

The READMEs next to the two templates ([tap](../packaging/homebrew-tap/README.md), [bucket](../packaging/scoop-bucket/README.md)) have the full steps.

**crates.io.**

- [ ] Have a crates.io account with a verified email address, linked to your GitHub account. You need both to create trusted-publisher configurations.

**GHCR.** There is nothing to create: the first release's `publish-image` job creates the package `ghcr.io/jabbott-iii/grimoire`.

- [ ] **Never push to `ghcr.io/jabbott-iii/grimoire` by hand before the first release.**
  - A package that a workflow first publishes with `GITHUB_TOKEN` is linked to that repository.
  - But `GITHUB_TOKEN` can't push to a package in your namespace that was pushed earlier and isn't connected to the repository. So a manual first push would block `publish-image`.
- [ ] Check that your account has no `grimoire` container package yet. Anonymous checks on 2026-11-17 found none, but they can't see a private package (UNVERIFIED).
  - If one exists, open its package settings before the first release: connect it to `jabbott-iii/Grimoire`, and give that repository write access under "Manage Actions access".
  - A package linked after its first push doesn't inherit the repository's access.
  - This case hasn't been tried.

### Switch releases on (last)

- [ ] **Clear the before-first-release markers.**
  - Text to change before the first release sits between an opening line, `<!-- before-first-release: … -->`, which says what to do, and a closing line, `<!-- /before-first-release -->`.
  - The marked texts are the README's "not released yet" note, the placeholder for the conduct contact in `CODE_OF_CONDUCT.md`, and the "No release exists yet" sentence in `SECURITY.md`.
  - `grep -n -i -E '^[[:space:]]*<!--[[:space:]]*/?before-first-release' README.md CODE_OF_CONDUCT.md CONTRIBUTING.md SECURITY.md` lists the marker lines.
  - In a pull request of its own, merged to `main` before the release pull request, do what each opening line says: in the README and `SECURITY.md`, delete everything from the opening line to the closing one; in `CODE_OF_CONDUCT.md`, fill in the contact and delete both marker lines.
  - `cargo xtask release plan` fails while an opening or a closing line remains in `README.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md` or `SECURITY.md`. It does so in every run that has a `CHANGELOG.md` section for the version: the release pull request's checks and the release itself. Other pull requests aren't affected.
  - It matches the lines loosely, in any case and with any spacing around `<!--`, so deleting only the opening line still fails.
  - Make the change on `main`, not on the release pull request's branch. release-plz manages that branch and updates it on each push to `main`; whether it keeps a commit you push there is UNVERIFIED.
  - **Why before the release.** The README goes into every CLI archive, it is the PyPI long description (the text on the project's PyPI page) and the `agent-grimoire` crate's page on crates.io, and a published version can't be changed. The npm packages don't carry it: `xtask/src/release/channels/npm.rs` writes their short READMEs.
- [ ] Set the repository variable `GRIMOIRE_RELEASES` to `enabled` (Settings → Secrets and variables → Actions → Variables).
  - The switch exists because a run that references a missing environment creates it without protection rules. Release mode names the five environments, so it must wait until you have created them and added the required reviewer ([ADR-0009](adr/0009-release-pipeline.md)).
  - The variable approves nothing: the required reviewers on the environments are the gates.
- From then on, a push to `main` runs in release mode when its version isn't tagged and `CHANGELOG.md` has a section for it. Other pushes, Dependabot merges included, stay dry-runs.
- The first release starts when you merge the release pull request. Before you merge it, check that it is current (release-plz updates it on each push) and that its checks passed.

### During the first release run

- Trusted publishing can't create a crate, so you publish the first version of each crate yourself, from the commit the run tagged.
- npm's trusted publishing is set up here too, so that its 48-hour window starts as late as possible.
- The container image's package is created by the run itself, and you make it public at the end.

1. **Wait for the `release` approval.** Check that the run's commit (the `GITHUB_SHA` shown in the run) is the release pull request's merge commit, and that every earlier job passed. If not, reject it.
   - The one other commit to accept is the merge commit of a fix pull request merged after the release pull request's run failed before this approval ([Recovery](#recovery)), once you've checked that `main` gained only that fix since the release pull request's merge.
2. **Approve `release`.** The `publish` job tags `vX.Y.Z` and publishes the GitHub Release. Check that its log says the release is immutable ([Recovery](#recovery) if not).
3. **The run now waits for `crates-io`, `npm`, `pypi` and `ghcr`.** Don't approve yet.
4. **Check out the tag `vX.Y.Z`** in a clean clone, with the pinned toolchain from `rust-toolchain.toml`.
5. **Create a crates.io API token** that expires soon and can only publish these crates. Which scopes (limits on what a token may do) crates.io offers was not re-checked in Phase 5.
6. **Publish the six crates:** `cargo publish --workspace --locked`, with the token in `CARGO_REGISTRY_TOKEN` for that one command only.
   - Asset crates, `agent-grimoire-mcp`, the testkit and xtask are skipped because they are `publish = false`.
   - The command isn't atomic: crates it published before a failure stay published. It also refuses versions that already exist.
   - If it stops partway, publish the rest one at a time in dependency order: `cargo publish --locked -p <crate>` for `agent-grimoire-model`, `-catalog`, `-resolve`, `-install`, `-adapters`, then `agent-grimoire`.
7. **Revoke the token.**
8. **Add a trusted publisher to each crate** on crates.io: owner `jabbott-iii`, repository `Grimoire`, workflow `release.yml`, environment `crates-io`. Always set the environment: a configuration without one accepts a token from any environment.
9. **Turn on trusted-publishing-only for each crate** (`trustpub_only`), in the crate's settings or with `PATCH /api/v1/crates/{name}`. API tokens then can't publish it.
10. **Configure npm trusted publishing for each package.** Sign in with `npm login`, then for each of the seven packages:
    ```sh
    npm trust github <package> --file release.yml --repo jabbott-iii/Grimoire --env npm --allow-stage-publish --yes
    ```
    - Every field is case-sensitive, so type `Grimoire` with the capital.
    - Leave direct publishing and dist-tag access off.
    - A new configuration must complete a successful publish within 48 hours, or it expires and must be deleted and created again. Whether a staged (not yet approved) version counts is UNVERIFIED, so do steps 11 and 12 within 48 hours of this one.
11. **Approve `crates-io`, `npm`, `pypi` and `ghcr`.**
    - The crates.io job finds all six versions published and skips them.
    - The npm job stages the seven packages, and the PyPI job uploads the wheels and creates the project.
    - `publish-image` creates the GHCR package with its first push, and `sign-image` follows without another approval.
12. **Approve the npm versions** ([How a release flows](subsystems/release.md#how-a-release-flows), step 8).
13. **Check the new package before you make it public.** `sign-image` (not `publish-image`) signs, attests, tags `X.Y.Z` and moves `latest`, then fails at its last step if the package can't be pulled anonymously.
    - GitHub's docs disagree on whether a package that a workflow creates starts private or takes the repository's visibility. If the last step passed, the package is already public: do the checks below and skip steps 14 and 15.
    - On the package's page on GitHub, check that it is linked to `jabbott-iii/Grimoire`. The image's `org.opencontainers.image.source` label and annotations name the repository; which of them GHCR reads is UNVERIFIED.
    - The package also shows `sha256-<hex>` tags, because GHCR has no referrers API. `sha256-<index hex>` lists the cosign signature and the provenance attestation, and each `sha256-<platform hex>` lists that platform's BuildKit attestation manifest (the SBOM and provenance that BuildKit, Docker's build engine, adds for each platform).
    - Never delete them ([Recovery](#recovery)): deleting one removes what it lists, and verification then fails.
14. **Make the package public, in its package settings.** **This can't be undone**: a public package can never be made private again. No API changes a package's visibility, so this step is manual.
15. **Re-run `sign-image`** (Actions → the run → Re-run failed jobs). It has no environment, so it waits for no approval. The signature and the attestation are skipped because they are done, the tags already name this digest, and the last step now passes ([Recovery](#recovery)).

## Recovery

- **A job failed.**
  - Re-run the failed jobs of the same run (Actions → the run → Re-run failed jobs). GitHub allows that for 30 days after the run, and a re-run uses the run's own commit.
  - Each job skips what is already done:
    - `publish` accepts the tag at this commit, finishes a draft release, and checks that a published release holds exactly the files in `dist/`;
    - `publish-crates` skips crate versions already on crates.io;
    - `publish-npm` skips versions already published (staged ones are below);
    - `publish-pypi` skips files already on PyPI (`skip-existing`);
    - `publish-image` pushes the same digest again, untagged, which is harmless. It stops if `X.Y.Z` names another digest, and if a tag lookup fails for any reason other than the registry's "unknown" answer (`MANIFEST_UNKNOWN`, `NAME_UNKNOWN`);
    - `sign-image`:
      - skips the signature when one already verifies with the image's signing identity (`sign-image.yml`) and the caller pins;
      - skips the attestation when `gh attestation verify --bundle-from-oci` finds one in GHCR. An attestation that reached GitHub's attestations API but not GHCR is made again, which only adds a second record to the API, so a re-run after a partial attestation still finishes;
      - then checks and verifies both again, leaves `X.Y.Z` alone when it already names this digest, and applies the `latest` rule below.
  - The registry jobs are separate, so a failure in one doesn't hold up the others.
  - GitHub also re-runs the jobs that depend on a re-run job, so a re-run `publish-image` is followed by `sign-image`.
  - Whether a re-run job waits for its environment's approval again isn't documented; expect it to. `sign-image` has no environment, so it never waits.
  - Use "Re-run failed jobs", not "Re-run all jobs". A new `image` run builds an index with another digest, because its SBOMs and provenance hold build times, and the published notes name the first one.
  - Don't wait for a new run instead. Once the tag exists, a run on `main` only skips.
- **The `image` job failed.** It runs before the `release` approval, so nothing was published. Fix the cause as below, through a normal pull request, as for "The run failed before the `release` approval".
  - **A Grype finding, or a coverage failure.**
    - Every platform is scanned before the step fails, so the `image-scan` artifact holds each platform's JSON and CycloneDX reports (CycloneDX is a standard SBOM format).
    - A coverage failure (Grype detected no Debian 13 or no Debian packages, or a binary shows no Rust packages) means Grype couldn't see part of the image: investigate it, never ignore it.
    - If a fix exists, take it: a base digest bump ([Maintaining the image pins](#maintaining-the-image-pins)) or a crate update.
    - If none exists, add an ignore rule to `.grype.yaml` that names the vulnerability and the package and gives a reason. Do it in a pull request reviewed like the exceptions in `deny.toml`, the configuration of cargo-deny, which checks Rust dependencies.
    - Never lower the threshold or drop `--fail-on`.
  - Grype's database is rebuilt daily, so a re-run can fail where the pull request passed.
  - **A base signature, `context`, `check` or `coverage` failure.** Investigate; never skip or weaken the step. A base that doesn't verify, or an image whose layers aren't exactly the base's and the archived binaries, can mean a compromise.
- **The image's tags.** Only `sign-image` sets them, and only after the signature and the attestation verify in GHCR.
  - `X.Y.Z` is never overwritten.
  - `latest` moves to this release unless it already names a newer version. It stays on a newer release that verifies with the image's signing identity, so it never moves back to an older signed release. If the newer version doesn't verify, the job stops and `latest` stays where it is.
  - `.github/scripts/image-tags.sh` makes these steps, and its errors mean:
    - **`already points at <digest>, not at <digest>`** (`publish-image` or `sign-image`): `X.Y.Z` already names another digest. Stop and investigate. Never overwrite the tag and never delete it. Check whether that digest verifies with the image's identity, `sign-image.yml` ([Verifying a download](#verifying-a-download), "The container image").
    - **`cannot tell whether <ref> exists`:** a tag lookup failed for a reason other than the registry's "unknown" answer, and the error of crane (the command-line registry tool that the script uses) is printed just before it. The script never treats a tag it couldn't read as absent. Fix the cause, then re-run the failed job.
    - **`crane pushed <ref>, not <image>@<digest>`:** the push didn't keep the digest of the checked layout. Stop and investigate before you re-run.
    - **`latest is <version>, newer than X.Y.Z; leaving it`** is expected when you re-run an older release.
    - **`latest (<digest>) claims <version>, newer than X.Y.Z, but doesn't verify with the release signature: investigate before moving it`** means the image `latest` names claims a newer version but didn't verify with the image's signing identity.
      - The cause is one of these: someone with write access to the package moved `latest`; its signature was removed from its `sha256-<hex>` tag; or the check itself failed, for example on a network error.
      - The script can't tell these apart, so it stops and leaves `latest` alone. Investigate. If the check itself failed, re-run the failed job.
    - **The warning `latest named <digest>, which has no release version; moving it`** means `latest` named an image without an `org.opencontainers.image.version` annotation, which no release makes. The script moves `latest` to this release. Find out who tagged that image.
    - **`is not a sha256 digest`, `is not a release version`, a usage line, or `sign-image.yml`'s `unexpected version … or digest`:** the workflow passed bad input. That is a bug in the workflow, so a re-run can't fix it (below, "A re-run can't finish the release").
- **The weekly image scan failed** (`security.yml`, job `image-scan`). It has one of two causes:
  - the latest release's image isn't pushed, tagged, signed or public yet (finish its `publish-image` and `sign-image`, [During the first release run](#during-the-first-release-run)), which the job's error names;
  - or a new advisory (a newly published vulnerability) matches it.
  - A released image is never changed, so if a fix exists, take it as for the `image` job and release a new patch version.
  - If none exists, an ignore rule with a reason in `.grype.yaml` on `main` also clears the weekly scan.
- **`publish-npm` failed after staging some versions.**
  - The job can't see versions that are staged but not approved, because `npm view` reads only published ones. npm refuses to stage the same version twice. So a re-run stops with an error that says to approve or reject the earlier stage first.
  - On npmjs.com (Staged Packages), approve or reject every staged X.Y.Z, platform packages first. A rejected version can be staged again.
  - For approved versions, wait until `npm view <package>@X.Y.Z version` prints the version for each one; npm's scan can take 15 minutes or more. Then re-run `publish-npm`.
- **The run failed before the `release` approval.**
  - Nothing was published. Fix it on `main` through a normal pull request. The next push builds the fixed commit for the same version, because the tag doesn't exist yet.
  - The failed run's signatures and attestations stay in the public logs (Rekor, Sigstore's public transparency log, and GitHub's attestations), but its files were never released.
- **The release isn't immutable** (`publish` warned `release vX.Y.Z is not immutable`).
  - GitHub created no release attestation, so the tap and bucket syncs can't take this version. Enabling the setting later doesn't change a release that is already published.
  - Before you approve the registry round, reject it. Enable immutable releases ([Repository settings](#repository-settings)). Delete the release but keep the tag: `gh release delete vX.Y.Z --repo jabbott-iii/Grimoire`, without `--cleanup-tag`.
  - Then re-run the `publish` job (Actions → the run → the job → Re-run). It finds the tag at the same commit and no release, so it publishes the release again, now immutable. GitHub re-runs the registry jobs after it; approve them.
  - This path hasn't been tried.
- **A re-run can't finish the release.** That happens when the run is more than 30 days old, or when the fix needs a change to the code or the workflow, because a re-run uses the run's commit.
  - Check `gh release view vX.Y.Z --repo jabbott-iii/Grimoire`.
  - **If the release is missing or still a draft, and no registry has X.Y.Z** (crates.io; npm, published or staged; PyPI; GHCR):
    1. Delete the draft release: `gh release delete vX.Y.Z --repo jabbott-iii/Grimoire`.
    2. Delete the tag: `gh api -X DELETE repos/jabbott-iii/Grimoire/git/refs/tags/vX.Y.Z`. Nothing locks a tag or a draft before the release is published.
    3. Fix the cause on `main` through a pull request.
    4. The next push to `main`, or a manual run, releases X.Y.Z from the fixed commit.
  - **If any registry already has X.Y.Z,** never reuse the version. Release a new patch version.
- **npm trusted publishing failed** (npm/cli#9969, "package not found": an npm bug, open when checked on 2026-11-17, that affects repositories like this one, whose OIDC tokens carry GitHub's immutable subject; [ADR-0009](adr/0009-release-pipeline.md)):
  1. Download the seven `.tgz` files from the GitHub Release and verify them ([Verifying a download](#verifying-a-download)).
  2. Run `npm login`.
  3. Check which versions are already there: `npm stage list` (an interactive session can list stages) and `npm view <package>@X.Y.Z version`. Skip every package that has a staged or published X.Y.Z.
  4. Run `npm stage publish <file>.tgz --access public --ignore-scripts` for the remaining platform tarballs, then for the launcher.
  5. Approve them in the same order.
  - These versions have no npm provenance (npm's signed link from a version to the workflow run that published it). No token is stored at any point.
- **An npm trusted-publisher configuration expired.** It fails with "package not found" too. An expired configuration can't be edited: delete and create it again ([During the first release run](#during-the-first-release-run), step 10), then re-run `publish-npm`. That keeps the OIDC path, unlike the fallback above.
- **A release turns out to be wrong after it was published.**
  - Release a new patch version.
  - Never delete or move a published tag, and don't try to reuse its version. Immutable releases lock the tag and the assets, and the tag name can never be reused.

## Installing

Install the package `agent-grimoire`, then run the command `grimoire`. The only exception is winget, where the command is `agent-grimoire` (below). Every prebuilt channel installs the same binary; `cargo install` builds it from source. [`cli.md`](cli.md) has the commands.

| Channel | Install | Notes |
|---|---|---|
| crates.io, from source | `cargo install agent-grimoire --locked` | Builds on your machine |
| crates.io, prebuilt | `cargo binstall agent-grimoire` | Downloads the release archive. It relies on HTTPS for the archive and checks only the crate's checksum, not the archive ([Verifying a download](#verifying-a-download)) |
| npm | `npm install -g agent-grimoire` | Needs Node.js 18 or later. The launcher runs the binary from a platform package |
| npm, without installing | `npx agent-grimoire --version` | CI tests only `npx grimoire` in a project that already has the package installed |
| PyPI | `pipx install agent-grimoire` or `uv tool install agent-grimoire` | The wheel holds the binary and no Python code. CI tests only `pip install` in a venv (a Python virtual environment) |
| Homebrew | `brew install jabbott-iii/tap/agent-grimoire` | The fully qualified name trusts just this formula (Homebrew 6+ requires trust for third-party taps) |
| Scoop | `scoop bucket add jabbott-iii https://github.com/jabbott-iii/scoop-bucket`, then `scoop install jabbott-iii/agent-grimoire` | Needs Git |
| winget | `winget install jabbott-iii.agent-grimoire` | **The command is `agent-grimoire`.** `grimoire` belongs to another winget package. Open a new shell after installing |
| Install script | below | Linux, macOS and Windows |
| Container image | `docker pull ghcr.io/jabbott-iii/grimoire@sha256:<digest>`, with the digest from the release notes | linux/amd64 and linux/arm64. How to run it is in [`container.md`](container.md) |

To use the short name with Homebrew instead:
```sh
brew tap jabbott-iii/tap
brew trust --formula jabbott-iii/tap/agent-grimoire
brew install agent-grimoire
```

### Install scripts

On Linux and macOS (`install.sh` needs curl; on Alpine, `apk add curl` first):
```sh
curl --proto '=https' --tlsv1.2 -fsSLO https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.sh
# Verify it (below) and read it, then:
sh install.sh                   # installs to ~/.local/bin
sh install.sh --to ~/bin        # or set GRIMOIRE_INSTALL_DIR
```

On Windows:
```powershell
# Windows PowerShell 5.1 may not offer TLS 1.2, which GitHub requires; this line adds it (harmless in PowerShell 7).
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.ps1 -OutFile install.ps1 -UseBasicParsing
# Verify it (below) and read it, then:
powershell -ExecutionPolicy Bypass -File .\install.ps1                    # installs to %LOCALAPPDATA%\Programs\grimoire\bin
powershell -ExecutionPolicy Bypass -File .\install.ps1 -InstallDir C:\tools  # or set GRIMOIRE_INSTALL_DIR
```
- The TLS line is the one Microsoft gives for Windows PowerShell 5.1 in its own install instructions. TLS (Transport Layer Security) is the encryption under HTTPS. `install.ps1` sets it again for its own download.
- `-UseBasicParsing` avoids the "Script Execution Risk" prompt that Windows PowerShell 5.1 shows since its 2025-12-09 security update.

What the scripts do:
- They install the version they were released with. To pin a version, download the script from `https://github.com/jabbott-iii/Grimoire/releases/download/vX.Y.Z/` instead.
- **Download.** `install.sh` downloads with curl only, with `--proto =https --tlsv1.2`, so it also refuses a redirect to anything but HTTPS. `install.ps1` uses `Invoke-WebRequest`. PowerShell 7.4 and later refuse a redirect from HTTPS to HTTP by default; what Windows PowerShell 5.1 does is UNVERIFIED.
- **Checksum.** They check the archive's SHA-256 against the value embedded in the script before extracting it. They fail if no SHA-256 tool is available.
- **Replacing the binary.** They copy the new binary next to the old one and check that it runs before swapping it in. `install.ps1` keeps a backup during the swap and puts the old `grimoire.exe` back if the swap fails.
- **Platform.** On macOS under Rosetta (Apple's layer that runs x86-64 programs on Apple silicon), `install.sh` installs the arm64 build. Under 32-bit PowerShell on 64-bit Windows, `install.ps1` still picks the 64-bit build.
- They never edit PATH or shell startup files. They print a hint when the install directory isn't on PATH.
- They send no telemetry, use no tokens, and write no receipts or updater.
- `GRIMOIRE_DOWNLOAD_URL` changes where archives are downloaded from. It must be `https://`, except `http://127.0.0.1` and `http://localhost`, for testing.

The `releases/latest/download/` address is GitHub's redirect to the newest release. The redirect itself isn't covered by the Phase 5 research (UNVERIFIED).

Only `release.yml` is meant to create releases here: release-plz is configured to create none, but its token could ([the release spec](subsystems/release.md)).

**Verify a script before you run it.** It is a release asset like any other. Download `install.sh` (or `install.ps1`) with its `.sigstore.json` bundle, and check it with cosign or `gh attestation verify` as in [Verifying a download](#verifying-a-download). They show that a release run on `main` signed it. `gh release verify-asset` alone shows only that the file belongs to a release. Then read it.

## Verifying a download

Every file in a release, except the `.sigstore.json` bundles themselves, has:
- a build provenance attestation (a signed record of the run that built the file), stored on the repository rather than on the release page;
- a cosign bundle, `<file>.sigstore.json`, on the release page: the file's signature, with what is needed to verify it.

`SHA256SUMS` lists every one of those files except itself. Each archive also has a CycloneDX SBOM (`<archive>.cdx.json`, the list of components in the archive) and an SBOM attestation.

Download the file you want, plus `SHA256SUMS` and the matching `.sigstore.json` bundles, from the release page.

**Who signs.**
- Every signature and attestation of the release files is made by `.github/workflows/sign.yml`, Grimoire's signing workflow, when `release.yml` on `main` calls it.
- The container image is signed by `.github/workflows/sign-image.yml`, under its own identity (below).
- **The caller pins.** Any repository can call a public reusable workflow and get a certificate with the same name. So the commands below also pin the caller, the run that called the signing workflow: the repository `jabbott-iii/Grimoire` and the ref `refs/heads/main` (Git's full name for the `main` branch).
- No release exists yet, so every release is signed this way.
- Don't accept `…/release.yml@refs/heads/main`: other release jobs can still get certificates with that name, and no release is signed with it.

**Checksums:**
```sh
sha256sum -c SHA256SUMS --ignore-missing
```
- This checks every listed file that you downloaded. `SHA256SUMS` is itself signed and attested, so verify it too, as below.
- `sha256sum` here is GNU coreutils. macOS 15 and later include a `/sbin/sha256sum`, but whether it supports `-c` and `--ignore-missing` wasn't checked. macOS 14 and earlier have none.
- On Windows, compare `Get-FileHash -Algorithm SHA256 <file>` with the file's line in `SHA256SUMS`. `Get-FileHash` prints the hash in uppercase.

**Build provenance**, with gh 2.102.0 or later:
```sh
gh attestation verify agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz \
  --repo jabbott-iii/Grimoire \
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- `--signer-workflow` names `sign.yml` at `refs/heads/main`, the workflow that signs. Without the ref, gh would also accept `sign.yml` at any tag or commit, which any repository can call.
- `--repo` and `--source-ref` pin the run that called it. `--repo` alone accepts any workflow and any ref of the repository, so don't drop the other flags.
- **Pin what no flag does**, especially with `--bundle-from-oci`, where gh compares names only: add `--format json` and check the caller's workflow file and the repository and owner IDs. The release workflow runs this same check.
  ```sh
  … --format json | jq -e 'length > 0 and all(.[]; .verificationResult.signature.certificate
    | .buildConfigURI == "https://github.com/jabbott-iii/Grimoire/.github/workflows/release.yml@refs/heads/main"
      and .sourceRepositoryIdentifier == "1460666977" and .sourceRepositoryOwnerIdentifier == "84321953")'
  ```
- For the SBOM attestation, add `--predicate-type https://cyclonedx.org/bom`.
- `--signer-workflow` and `--source-ref` are case-sensitive, so type `Grimoire` with the capital.
- gh 2.102.0 fixed prefix matching in `--signer-workflow` (GHSA-wjmr-j3rp-mh2g) and case-insensitive `--source-ref` (GHSA-4mq3-hpgx-9cx8). Don't use an older gh.
- To tie the attestation to the release commit, add `--source-digest <commit>`, the commit that tag `vX.Y.Z` points to.
- **Offline.** The `.sigstore.json` files are cosign bundles, and `gh attestation verify` ignores them (it reports that no attestations were found). Save what you need while you are online:
  - `gh attestation download <file> --repo jabbott-iii/Grimoire` saves the file's attestations as `sha256-<hex>.jsonl` and prints the path. Add `--predicate-type https://cyclonedx.org/bom` for the SBOM attestation only.
  - `gh attestation trusted-root > trusted_root.jsonl` saves the trust root (the certificates and keys that Sigstore verification starts from).
  - Offline, add `--bundle sha256-<hex>.jsonl --custom-trusted-root trusted_root.jsonl` to the command above. With `--bundle`, gh checks only the names recorded in the certificate, so it can't notice a repository rename or reuse.
  - This round trip is taken from gh 2.103.3's source and hasn't been run.

**cosign**, version 3.1.3 or later:
```sh
cosign verify-blob agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz \
  --bundle agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz.sigstore.json \
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-github-workflow-repository jabbott-iii/Grimoire \
  --certificate-github-workflow-ref refs/heads/main
```
- The identity names `sign.yml`.
- The last two flags pin the caller. A call from another repository gets the same identity, but its certificate names its own repository and ref in the fields these flags check (SIGN-P6 in [`specs.md`](specs.md)).
- Keep all four `--certificate-*` flags.
- The identity and both pins are compared exactly, case included.
- The ref is `refs/heads/main` because releases run from `main`, not from a tag.
- To tie the signature to the release commit, add `--certificate-github-workflow-sha <commit>`, the commit that tag `vX.Y.Z` points to.
- cosign 3.1.3 fixed a verification bypass (GHSA-fx35-mq7g-6g98).
- Without a local trusted root, cosign fetches Sigstore's trust root over the network.

**GitHub's release attestation**, with gh 2.81.0 or later:
```sh
gh release verify vX.Y.Z --repo jabbott-iii/Grimoire
gh release verify-asset vX.Y.Z agent-grimoire-X.Y.Z-x86_64-unknown-linux-musl.tar.gz --repo jabbott-iii/Grimoire
```
- This shows that the file belongs to the published, immutable release `vX.Y.Z`. A release published before immutable releases were enabled has no such attestation.
- It is GitHub's own attestation (signer `https://dotcom.releases.github.com`), not Grimoire's build provenance, so use both.
- It can't check the source archives GitHub generates automatically.

**The container image.** [`container.md`](container.md) has the details. Take the index digest from the release notes, then use `sign-image.yml`'s identity and the same caller pins:
```sh
cosign verify ghcr.io/jabbott-iii/grimoire@sha256:<index digest> \
  --certificate-identity https://github.com/jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --certificate-github-workflow-repository jabbott-iii/Grimoire \
  --certificate-github-workflow-ref refs/heads/main
gh attestation verify oci://ghcr.io/jabbott-iii/grimoire@sha256:<index digest> \
  --bundle-from-oci \
  --repo jabbott-iii/Grimoire \
  --signer-workflow jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main \
  --source-ref refs/heads/main \
  --deny-self-hosted-runners
```
- Verify the index digest. Only the index is signed, not each platform's manifest, so a platform manifest's digest doesn't verify on its own.
- `--bundle-from-oci` reads the attestation's copy in GHCR, listed in the `sha256-<index hex>` tag. Without it, gh reads GitHub's attestations API instead.
- The tags `X.Y.Z` and `latest` can move; a digest can't. Pin the digest.
- The signature names a digest, not a version. The index's `org.opencontainers.image.version` annotation, which the signature covers, says which version it is.
- `gh attestation verify` needs gh 2.102.0 or later, a gh login, and a registry login for the `oci://` lookup.

What each package manager checks on your machine, and what it doesn't, is in [the release spec](subsystems/release.md); the image's is in [the container image spec](subsystems/container-image.md).

## Maintaining the image pins

A pin fixes the exact version and checksum, or the digest, of a tool or image that the workflows use, so that every run uses the same bytes.

**The 7-day rule.** Every pin is a release, or a digest, at least 7 days old when it is adopted (CI-PINS in [`specs.md`](specs.md), the entry on CI pins). Dependabot's cooldown (its setting that holds an update back until the release is old enough) enforces that for its other entries. For the base image it doesn't, so you check by hand.

**The base image** (the one `FROM` line in `Dockerfile`):
- Dependabot's `docker` entry opens a pull request when distroless's `nonroot` tag moves to a new index. distroless is the minimal base image that the Grimoire image is built on.
- The pull request changes only the digest on the `FROM` line. The `context` step (`cargo xtask release image context`) derives everything else from it, such as the `base.digest` annotations.
- Its cooldown has no effect here. Dependabot dates Docker updates only from Docker Hub, so for gcr.io it proposes the new digest at once, with a `cooldown_date_unavailable` warning.
- **Before you merge, check the new digest's age:**
  ```sh
  curl -fsS https://gcr.io/v2/distroless/static-debian13/tags/list \
    | jq -r '.manifest["sha256:<new digest>"].timeUploadedMs | tonumber / 1000 | todate'
  ```
  - It prints when GCR (Google Container Registry) first stored that index, such as `2026-10-31T22:37:00Z` for `sha256:25ac8f0c…`. Merge only when that is at least 7 days ago.
  - The error `null (null) cannot be parsed as a number` means the digest isn't in that repository. Checked with jq 1.7 on 2026-11-17.
  - `timeUploadedMs` is undocumented. For the digests the Phase 6 research looked at, it matched the time of the digest's signature in Rekor.
  - `.manifest["sha256:<new digest>"].tag` lists the index's tags. The newest index carries `nonroot`.
- **Then let the pull request's checks run.** The release dry-run verifies distroless's signature on the new index and both platform manifests, then builds, checks and scans the image. Merge only when they pass.
- The pull request has no release notes, because distroless sets no labels.
- Dependabot opens no security updates for Docker images. The weekly `image-scan` job in `security.yml` scans the latest release's image instead ([Recovery](#recovery)).
- **Today's pin** is `sha256:aafe9936…`, which `nonroot` still pointed to on 2026-11-20. It was first uploaded 2026-11-12T21:16:35Z.
  - It met the rule at 2026-11-19T21:16:35Z. Dependabot's pull request 1 moved the pin to it and was merged on 2026-11-20, after that time.
  - It differs from the previous pin, `sha256:25ac8f0c…` (first uploaded 2026-10-31), only in the mode (the file permissions) of `/usr/lib/os-release`, to which `/etc/os-release` links.
  - The pull request's release dry-run (run 44385784834) verified distroless's signature on the new index and both platform manifests, then built, checked and scanned the image. Grype found no vulnerabilities on either platform.

**Where the other pins live.** Dependabot can't see values in a workflow's `env:`, so bump these by hand:

| Pin | Where |
|---|---|
| `IMAGE` (`ghcr.io/jabbott-iii/grimoire`) | `release.yml` `env`, `sign-image.yml` `env`, `security.yml`'s `image-scan` job, and the `IMAGE` constant in `xtask/src/release/image.rs` |
| `BUILDX_VERSION`, `BUILDX_SHA256` (the linux-amd64 binary) | `release.yml` `env` |
| `CRANE_VERSION`, `CRANE_SHA256_X64` | `release.yml` `env`, `sign-image.yml` `env`, and `security.yml`'s `image-scan` job |
| `CRANE_SHA256_ARM64` (`verify-image`'s arm64 runner) | `release.yml` `env` |
| `GRYPE_VERSION`, `GRYPE_SHA256` (the linux_amd64 tarball) | `release.yml` `env`, and `security.yml`'s `image-scan` job |
| `BUILDKIT_IMAGE`, `SBOM_SCANNER_IMAGE` (`name:tag@sha256:…`) | `release.yml` `env` |
| `DISTROLESS_IDENTITY`, `DISTROLESS_ISSUER` (distroless's signer) | `release.yml` `env` |
| `COSIGN_IDENTITY`, `COSIGN_ISSUER`, `COSIGN_REPOSITORY`, `COSIGN_REF` (a signing identity, the OpenID Connect issuer its certificate must name, and its caller pins) | For the release files, `sign.yml`: `release.yml`'s `publish` job and `sign.yml` `env`. For the image, `sign-image.yml`: `sign-image.yml` `env` and `security.yml`'s `image-scan` job |
| `SHELLCHECK_VERSION`, `SHELLCHECK_SHA256` | `release.yml`'s `verify-linux` job, and `security.yml`'s `workflows` job |
| cosign (`sigstore/cosign-installer`) and `actions/attest` | `uses:` lines, which Dependabot and `actions.lock` cover |

- **The four workflows must agree.** The xtask test `the_image_pins_agree_across_workflows` (run by `cargo test --workspace`) fails:
  - when `IMAGE`, `CRANE_VERSION` or `CRANE_SHA256_X64` differ between `release.yml` and `security.yml` or `sign-image.yml`;
  - when `COSIGN_ISSUER`, `COSIGN_REPOSITORY` or `COSIGN_REF` differ between `release.yml` and `security.yml`, `sign.yml` or `sign-image.yml`;
  - when `GRYPE_VERSION`, `GRYPE_SHA256`, `SHELLCHECK_VERSION` or `SHELLCHECK_SHA256` differ between `release.yml` and `security.yml`;
  - when a file sets one of these keys more than once, or not at all;
  - when `COSIGN_IDENTITY` isn't `…/sign.yml@refs/heads/main` in `sign.yml` and `release.yml`, or `…/sign-image.yml@refs/heads/main` in `sign-image.yml` and `security.yml`;
  - when `sign-image.yml`'s repository isn't `jabbott-iii/Grimoire`, its ref `refs/heads/main`, or its `SIGNER_WORKFLOW` `jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main`;
  - when `release.yml`'s `IMAGE` isn't the xtask constant, or `BUILDKIT_IMAGE` or `SBOM_SCANNER_IMAGE` isn't pinned by digest.

  Change the files in one pull request. The test passed locally on 2026-11-18.
- **The identities are also written out in full** where the test doesn't look: the job guards in both signing workflows, the `publish` job's `gh attestation verify` flags and expected Build Config URI, and the tap and bucket sync templates. Change them with the pins.
- **`.github/scripts/image-tags.sh`** is checked by ShellCheck, a linter for shell scripts (the pin above), in `security.yml`'s `workflows` job. Every pull request dry-run runs its cases in `verify-image` against a local registry, and the release runs the same script, in `publish-image` and `sign-image.yml`. So the code that guards the release tags never runs for the first time in a release.
- **Tools.** Date a tool by its GitHub release. Check each new SHA-256 against the release's own checksum file, as for the other CI pins.
- **Images.**
  - Date BuildKit and the SBOM scanner by their GitHub releases, and resolve the tag to its index digest, for example `crane digest moby/buildkit:v0.34.2` (it printed the pinned `sha256:c5f8862f…` on 2026-11-17).
  - Keep `BUILDKIT_IMAGE` in the form `moby/buildkit:vX.Y.Z@sha256:…`: the builder step checks that the `BuildKit version:` line of `docker buildx inspect` reports the tag's version.
  - The scanner's image tags have no `v`.
- **Before a cosign upgrade**, check that it still verifies distroless's legacy `.sig` signatures.
  - cosign's version comes with `sigstore/cosign-installer`, which Dependabot bumps.
  - cosign says its new bundle format will become the only supported one, and distroless publishes only the legacy format.
- **Grype.** `.grype.yaml` holds the gate: the failure threshold and the ignore rules.
  - Grype ignores unknown keys silently, also inside an ignore rule, where a misspelled key makes the rule match more than intended. So the test `the_grype_config_has_only_known_keys_and_complete_rules` refuses any unknown key, and any rule without a GHSA or CVE ID (a Common Vulnerabilities and Exposures identifier), a package name (`package: name:`) and a reason. The workflows also repeat the threshold with `--fail-on medium`.
  - A rule may use only `vulnerability`, `package` (`name`, `version`), `reason` and `include-aliases`; widen the test in the same pull request if a rule needs more.
  - Ignore rules have no expiry: review every rule when you bump the base image or Grype, and remove the ones that no longer match.
- **Known follow-ups.** BuildKit v0.35.0 and buildx v0.39.0 (Docker's build plugin), both from 2026-11-11, have met the 7-day rule since 2026-11-18, but the workflows still pin v0.34.2 and 0.38.2. A CI check of the base digest's age doesn't exist yet.

## Crash reporting

Grimoire has none. The install path contains no network code ([ADR-0001](adr/0001-catalog-delivery.md)), and the CLI sends no telemetry or crash reports. Users report problems through the issue forms in `.github/ISSUE_TEMPLATE/`, and vulnerabilities privately, as [`SECURITY.md`](../SECURITY.md) says.
