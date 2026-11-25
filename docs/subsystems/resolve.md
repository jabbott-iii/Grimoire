# Subsystem: `agent-grimoire-resolve`

`agent-grimoire-resolve` turns a project's `grimoire.toml`, its lock and a catalog `Source` into
the set of assets to install: which version of each, why it is selected, which clients it renders
for, and which permissions it needs, all of them accepted. It guarantees that nothing reaches the
planner unless every selected asset exists, matches every version requirement on it, matches its
lock pin, supports a client, accepts the running CLI, and has every permission it declares
granted in `grimoire.toml`. Every failure is a distinct diagnostic, and all of them are reported at
once. [ADR-0001](../adr/0001-catalog-delivery.md) (one embedded catalog, so one version of each
asset), [ADR-0005](../adr/0005-client-priority.md) (the six clients) and
[ADR-0008](../adr/0008-phase-4-client-adapters.md) (hooks for Claude Code only, and no agents for
Codex) shaped it.

The terms, once:

- An **asset** is one entry of Grimoire's catalog: a rule, skill, agent, tool or MCP server (MCP
  is the Model Context Protocol). Its `asset.toml`, the **manifest**, gives its version, the
  clients it supports, the permissions it needs, and the assets it requires or conflicts with
  ([`model.md`](model.md#assettoml)).
- A **client** is an AI coding tool that Grimoire writes files for, such as Claude Code or Codex
  (`ClientId`).
- `grimoire.toml` (`ProjectConfig`) is the project's selection, and `grimoire.lock`, the **lock**
  (`Lockfile`), records what was last installed, including each asset's version, its **pin**.
- A **diagnostic** (`Diagnostic`) is an error or a warning with a stable code, such as
  `resolve/permission`. Every `resolve/` code is in
  [`docs/diagnostics.md`](../diagnostics.md#resolution-resolve), with what raises it and what to
  do.

The API reference is the crate docs in `crates/agent-grimoire-resolve/src/lib.rs`.

## Why resolve first, check rather than choose, and report everything

Resolution is the first step of every install ([`install.md`](install.md#desired-state-plan-apply)):
planning works only on assets whose selection, versions, clients and consent are already settled,
so the planner never has to decide what to install, only how to write it.

A package manager usually *chooses* a version that satisfies every requirement. Grimoire's catalog
is built into the binary, and each release embeds exactly one version of each asset (ADR-0001). So
resolution *checks* that one version against every requirement, every pin and `min-cli-version`,
rather than searching for one. The trade-off is that a project pinned to an older version can't be
rendered again by a newer CLI; the error says to run `grimoire update <id>`.

Every selected asset is checked, and every error is returned together, sorted, so that one run
shows the user everything to fix. The alternative, stopping at the first error, makes a user fix
problems one run at a time.

Permission consent is part of resolution, not of a prompt, because resolution runs on every
`plan`, `apply` and `update`. A new version that needs more than the project granted therefore
stops at the same place every time, whether a person or CI runs the command.

## Resolving

Built in M2.2 as described here.

```rust
pub fn resolve<'s>(source: &'s Source, request: &Request<'_>)
    -> Result<Resolution<'s>, Vec<Diagnostic>>;

pub struct Request<'a> {
    pub config: &'a ProjectConfig,         // already checked by validate::project_config
    pub lock: Option<&'a Lockfile>,        // None when there is no grimoire.lock yet
    pub cli_version: &'a Version,          // the running CLI's version
    pub update: &'a Update,                // which assets may move away from their pins
    pub available_clients: &'a [ClientId], // the clients this build has adapters for
}

pub enum Update { None, All, Only(BTreeSet<AssetId>) }

pub struct Resolution<'s> {
    pub assets: BTreeMap<AssetId, Resolved<'s>>, // one per asset to install, by id
    pub warnings: Vec<Diagnostic>,
}

pub struct Resolved<'s> {
    pub asset: &'s SourceAsset,           // manifest, its SHA-256 digest, and its files
    pub selected_by: BTreeSet<Selector>,  // every reason it is selected
    pub allow: BTreeSet<Permission>,      // what it needs, all of it accepted
    pub clients: BTreeSet<ClientId>,      // the clients it renders for
    pub options: Option<AssetOptions>,    // its table under [assets], if it has one
}

pub fn missing_permissions(source: &Source, config: &ProjectConfig)
    -> BTreeMap<AssetId, BTreeSet<Permission>>;
pub fn selected_ids(source: &Source, config: &ProjectConfig) -> BTreeSet<AssetId>;
```

The `grimoire` CLI calls `resolve` in two places:

- `plan`, `apply`, `update` and `doctor` resolve with the project's lock, before they plan any
  change;
- `add` and `remove` resolve without the lock, to check the selection they are about to write, so
  that a selection that doesn't resolve never reaches `grimoire.toml`.

`add` calls `missing_permissions` first, to ask the user to accept what the new selection needs,
and `add` and `remove` both call `selected_ids` ([Permission consent](#permission-consent)).

The `Resolution` then goes to `agent-grimoire-adapters`, whose `render` turns each `Resolved` asset
into files for its clients ([`adapters.md`](adapters.md)). `cargo xtask validate` also builds
`Resolution` values itself, to render each catalog asset alone for each client it declares
([`catalog.md`](catalog.md#catalog-validation)).

## How assets are selected

An asset is selected when:

- it is listed under `[assets]` (`Selector::Direct`);
- it is in a bundle listed in `bundles`, a named list of assets that the catalog defines
  (`Selector::Bundle`; [`catalog.md`](catalog.md#bundle-not-an-asset-kind));
- it carries a tag listed in `tags` (`Selector::Tag`);
- a selected asset lists it in its manifest's `requires`, followed to every depth
  (`Selector::Requires`).

`exclude` drops an asset that a bundle or a tag would select. Excluding one that a selected asset
requires is an error (`resolve/excluded-dependency`). A tag never selects an excluded or deprecated
asset, nor one that supports none of the project's clients. A `resolve/tag-skipped` warning names
each asset that a tag skips for having no client.

An `[assets]` entry or a bundle that the catalog doesn't have is an error (`resolve/unknown-asset`,
`resolve/unknown-bundle`), and the message says so when the id is the other kind. A tag that no
asset carries is a warning (`resolve/unknown-tag`). A selected asset that is deprecated gets a
warning (`resolve/deprecated`) with the reason and the replacement.

## One version of each asset, and the lock

A `Source` holds one version of each asset, by id. So resolution checks that version rather than
choosing one:

- Every version requirement, from `[assets]`, a bundle or a `requires`, must match the catalog's
  version (`resolve/version`, naming who asked). A tag sets no requirement: it takes whatever
  version the catalog has, pre-releases (such as `1.0.0-rc.1`) included, which the requirement `*`
  would not match.
- When the lock pins an asset and `Request::update` doesn't let it move, the pin must be the
  catalog's version (`resolve/locked-version-unavailable`), and the manifest's digest must be the
  one the lock records (`resolve/locked-manifest-changed`). Both errors name
  `grimoire update <id>`. They are raised even when no file would change
  ([Resolution errors](#resolution-errors)). An asset that the lock doesn't pin takes the
  catalog's version.
- `Request::cli_version` must be at least the asset's `min-cli-version` (`resolve/cli-too-old`).

The CLI's `plan`, `apply` and `doctor` pass `Update::None`. `update` passes `Update::All`, or
`Update::Only` with the ids it is given. `add` and `remove` check the selection they are about to
write with no lock at all.

## Clients

`Resolved::clients` is every project client that the asset supports (has a `[clients.<id>]` table
for). If the asset's `[assets]` entry lists `clients`, it is exactly those instead, and each must
be one the asset supports (`resolve/unsupported-client`). Otherwise, for each project client the
asset doesn't support:

- if the client has a place for assets of its kind, the asset is skipped for that client with a
  `resolve/client-skipped` warning;
- if it has none, a `resolve/client-not-covered` warning says why. Grimoire writes hooks only for
  Claude Code, and gives Codex no agents, because no other client can keep a failed hook from
  letting the action through, and a Codex agent can't be limited to its declared tools (ADR-0008,
  which amends ADR-0005). `agents-md` never gets this warning, because by design it holds no
  agents, MCP servers or hooks.

An asset left with no client is a `resolve/unsupported-client` error that names the clients it
supports.

A project client that isn't in `Request::available_clients` is a `resolve/client-unavailable`
error. The CLI passes every client, because this build has an adapter for each, so it never raises
this one. The code is kept on purpose: it is needed again whenever the model gains a client before
its adapter, as it did in Phases 2 and 3.

## Permission consent

Built in M2.2 as described here; per-variable environment grants came in Phase 3.

Declared `[permissions]` are not just documentation. Resolution fails with `resolve/permission`
if an asset needs a permission that is not in its `allow` list.

**Grant names** (`Permission`): `filesystem-read`, `filesystem-write` (which covers read),
`network`, `shell`, `hooks`, and **`env:<NAME>` for each environment variable** (Phase 3).

- A bare `env` is refused in `grimoire.toml`, while the file is parsed.
- Each variable is its own grant. So a new version that reads one more variable is permission
  growth like any other, and stops until the user accepts that name.
- `grimoire add --allow env` is shorthand on the command line only. It accepts every variable the
  named assets read now, and writes each one out by name.
- The lock records the same names in `allow`. Consent stays in the one file users review,
  `grimoire.toml`; the lock only mirrors it.

In practice:

- `grimoire add` shows the permissions and asks interactively, or requires `--allow …`
  non-interactively.
- If a later version of an asset *expands* its permissions, `update`/`apply` stop and ask again.
  There is no silent escalation: the check runs against the catalog's manifest every time.
- `rule` assets are pure text and must declare no permissions
  ([`model.md`](model.md#assettoml)).

Where the grants come from, and what the resolution records:

- An asset listed under `[assets]` takes its grants from that entry's `allow`, and any other asset
  from the top-level `[allow]` table (`ProjectConfig::allowed`;
  [`model.md`](model.md#grimoiretoml)). A `filesystem-write` grant covers `filesystem-read`
  (`Permissions::covers`).
- The error names the command, or the `allow` entry, that would accept what is missing.
- `Resolved::allow` is what the asset needs, not everything the project granted: an asset that
  needs `filesystem-read`, under a `filesystem-write` grant, gets `filesystem-read`. The CLI
  records this set in the lock.
- An `[allow]` entry for an asset that isn't selected is a `resolve/allow-unused` warning.

`missing_permissions` gives what to ask for, ignoring every other problem. `add` asks only for the
assets that its arguments select, through `selected_ids`, never for other selected assets. `remove`
uses `selected_ids` to keep the grant of an asset that something else still selects.

## Resolution errors

Each of these is a distinct, actionable diagnostic, and the CLI exits with code 3 for any of them
([`cli.md`](cli.md#exit-codes)):

- unknown asset;
- missing dependency (`resolve/missing-dependency`; catalog validation already refuses it in the
  embedded catalog, as `catalog/requires-missing`);
- unsatisfiable version requirement;
- conflict between selected assets (naming who selected each one; `resolve/conflict`);
- unsupported client for an asset (naming the clients it supports);
- `min-cli-version` too new;
- unaccepted permissions;
- an agent that Cursor can't limit to its tools: Cursor takes only exactly `read` and `search`, as
  `readonly`. The Cursor adapter refuses it while rendering (`adapter/unsupported`), and catalog
  validation keeps such an agent out of the embedded catalog. Codex has no agents at all, but that
  alone is not an error: an agent can't declare Codex (`manifest/anchor-client`), so a project with
  Codex gets a `resolve/client-not-covered` warning and the agent is installed for its other
  clients. It is an error only as the unsupported-client error above (`resolve/unsupported-client`):
  when the agent's `clients` in `grimoire.toml` names Codex, or when no other client is left. Where
  a client limits the tools only partly, `plan` warns instead, as ADR-0008 decided;
- a pinned version unavailable in this CLI's catalog when content is needed. Each CLI release
  embeds only one version of each asset (ADR-0001).

**Phase 2 refinement of the last item.** With one embedded version per asset, `plan`, `apply` and
`update --no-apply` stop whenever a selected asset's locked version (or manifest digest) differs
from the catalog's.

- That includes cases where no file would change. Telling whether content is needed would mean
  rendering the pinned version, which is not available.
- The error names `grimoire update <id>`.
- `verify` and `uninstall` still work, because they need only the lock. So does `remove` + `apply`,
  but only when the removed assets include every asset whose pin differs: `apply` still stops for
  any selected asset that keeps a differing pin (checked in Phase 7).
- Assets not yet in the lock resolve to the catalog version.

`resolve` returns `Err` with every diagnostic, warnings included, when any of them is an error, and
otherwise `Resolution::warnings`. They are sorted by file, field, code and message, and exact
duplicates are removed. Callers match on `Diagnostic::code`, not on the message: a code keeps its
meaning, and the message text can change in any release.

An error doesn't stop the run: every selected asset is still checked, with two exceptions.
Conflicts (`resolve/conflict`) are checked only between assets that passed all their own checks,
and an asset whose clients fail (`resolve/unsupported-client`, including one unsupported client in
its `[assets]` `clients` list) isn't checked for permissions. Fixing one error can therefore reveal
another.

## Invariants

1. An asset is selected exactly when it is listed under `[assets]`, is a member of a selected
   bundle, carries a selected tag, or is required by a selected asset, and isn't excluded; a tag
   never selects an excluded or deprecated asset, or one with no project client.
2. Excluding an asset that a selected asset requires is an error.
3. An unknown asset and an unknown bundle are distinct errors.
4. Every version requirement on an asset matches the catalog's version.
5. A lock pin holds unless `Update` lets the asset move: a differing pin or manifest digest is an
   error that names `grimoire update <id>`.
6. An asset whose `min-cli-version` is newer than the running CLI is refused.
7. Two selected assets that declare a conflict are an error that names who selected each side.
8. Every selected asset has at least one client, and an explicit `clients` list names only clients
   the asset supports; a client with no place for the asset's kind gets a warning, not an error.
9. Every permission a selected asset declares is granted, from exactly one place, before it
   resolves; a new version that needs more stops until the user grants it.
10. `Resolved::allow` is what the asset needs, never more than that.
11. Every error from one run is returned together, sorted, without duplicates.
12. A selected asset that is missing a dependency is refused (`resolve/missing-dependency`).

## Tests

| Invariant | Test |
|---|---|
| 1 | `direct_bundle_tag_and_requires_selection`, `tags_skip_deprecated_assets` and `tags_accept_pre_releases_and_skip_unusable_assets` in `crates/agent-grimoire-resolve/tests/resolve.rs`; `the_bundles_install_their_members` in `crates/agent-grimoire/tests/builtin.rs` |
| 2 | `excluded_dependencies_and_members` in `tests/resolve.rs` |
| 3 | `unknown_assets_and_bundles_are_distinct_errors` in `tests/resolve.rs`; the second example in the crate docs (a doc test) |
| 4 | `version_requirements_must_match_the_catalog` in `tests/resolve.rs` |
| 5 | `lock_pins_hold_unless_updated` in `tests/resolve.rs`; `locked_versions_hold_until_update` in `crates/agent-grimoire/tests/cli.rs` |
| 6 | `min_cli_version_deprecation_and_unused_grants` in `tests/resolve.rs`; `no_embedded_asset_needs_a_newer_grimoire` in `crates/agent-grimoire/tests/builtin.rs` |
| 7 | `conflicts_name_who_selected_each_side` in `tests/resolve.rs` |
| 8 | `clients_explicit_unsupported_and_unavailable` and `clients_that_cannot_host_a_kind_are_reported` in `tests/resolve.rs` |
| 9 | `permissions_must_be_accepted` and `permission_hints_show_the_merged_grant` in `tests/resolve.rs`; `write_grant_covers_read` and `environment_grants_name_each_variable` in `crates/agent-grimoire-model/src/permissions.rs`; `permissions_need_consent`, `growing_permissions_stop_until_accepted`, `a_bare_env_grant_in_grimoire_toml_is_refused`, `allow_never_reaches_assets_the_command_does_not_name` and `removing_an_asset_that_stays_selected_keeps_its_grant` in `crates/agent-grimoire/tests/cli.rs` |
| 10 | `permissions_must_be_accepted` in `tests/resolve.rs`, which grants `filesystem-write` and asserts that the asset's `allow` is `filesystem-read` and its one variable |
| 11 | `errors_for_indirect_assets_name_the_selector` in `tests/resolve.rs`; the second example in the crate docs |
| 12 | No test yet: no test raises `resolve/missing-dependency`. Catalog validation refuses the same case before a release (`dependencies_are_resolved_catalog_wide` in `crates/agent-grimoire-catalog/tests/catalog.rs`, which asserts `catalog/requires-missing`), so only a damaged pack could reach it |

`server_options_on_other_kinds_and_client_set_variables_are_refused_clearly` in
`crates/agent-grimoire/tests/builtin.rs` checks `resolve/options-kind`, and
`doctor_fails_when_the_project_does_not_resolve` in `crates/agent-grimoire/tests/cli.rs` checks
that `doctor` reports a selection that doesn't resolve. The fixture catalogs these tests use come
from `agent-grimoire-testkit` ([`04-quality.md`](../04-quality.md#test-tools)).
