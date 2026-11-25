//! Selection and dependency resolution for Grimoire. [`resolve`] turns a
//! project's `grimoire.toml`, its lock and a catalog [`Source`] into the set of
//! assets to install: which version of each, why it is selected, which clients
//! it renders for and which permissions it needs, all of them accepted.
//! Every failure is a distinct diagnostic, and all of them are reported at
//! once.
//!
//! The terms, once:
//!
//! - An **asset** is one entry of Grimoire's catalog: a rule, skill, agent, tool
//!   or MCP server (MCP is the Model Context Protocol). Its `asset.toml`, the
//!   **manifest**, gives its version, the clients it supports, the
//!   permissions it needs and the assets it requires or conflicts with, among
//!   other fields.
//! - A **client** is an AI coding tool that Grimoire writes files for, such as
//!   Claude Code or Codex ([`ClientId`]).
//! - `grimoire.toml` ([`ProjectConfig`]) is the project's selection, and
//!   `grimoire.lock`, the **lock** ([`Lockfile`]), records what was last
//!   installed, including each asset's version, its **pin**.
//! - A **diagnostic** ([`Diagnostic`]) is an error or a warning with a stable
//!   code, such as `resolve/permission`.
//!
//! # Where it fits
//!
//! The `grimoire` command-line interface (CLI), in the `agent-grimoire` crate,
//! calls [`resolve`] in two places:
//!
//! - `plan`, `apply`, `update` and `doctor` resolve with the project's lock,
//!   before they plan any change;
//! - `add` and `remove` resolve without the lock, to check the selection they
//!   are about to write, so that a selection that doesn't resolve never
//!   reaches `grimoire.toml`.
//!
//! `add` calls [`missing_permissions`] first, to ask the user to accept what the
//! new selection needs, and `add` and `remove` both call [`selected_ids`] (see
//! "Permission consent", below).
//!
//! The [`Resolution`] then goes to `agent-grimoire-adapters`, whose `render`
//! turns each [`Resolved`] asset into files for its clients. `cargo xtask
//! validate`, the repository's check of the catalog, also builds
//! [`Resolution`] values itself, to render each catalog asset alone for each
//! client it declares.
//!
//! This crate builds on `agent-grimoire-model`, which parses `grimoire.toml` and
//! the lock and defines [`Diagnostic`], and on `agent-grimoire-catalog`, whose
//! [`Source`] holds the catalog built into the CLI.
//!
//! It is internal to the CLI. It is published only so that `cargo install
//! agent-grimoire` can build the CLI from crates.io, and its API changes
//! whenever the CLI needs it to, so don't depend on it. The
//! [versioning policy](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#versioning)
//! says why a break is still released as a new 0.y version.
//!
//! The design is in:
//!
//! - [`docs/subsystems/install.md`, Desired state, plan, apply](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#desired-state-plan-apply):
//!   resolution is the first step of every install, and the lock's pins hold
//!   unless `update` moves them;
//! - [`docs/subsystems/resolve.md`, Resolution errors](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/resolve.md#resolution-errors):
//!   the errors it reports;
//! - [`docs/subsystems/model.md`, `grimoire.toml`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#grimoiretoml):
//!   the selection it reads;
//! - [`docs/subsystems/resolve.md`, Permission consent](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/resolve.md#permission-consent):
//!   that it fails on any permission the project hasn't accepted.
//!
//! # The parts
//!
//! - **Resolving:** [`resolve`] takes a [`Source`] and a [`Request`].
//!   - [`Request`] holds the project's [`ProjectConfig`], already checked on
//!     its own by [`agent_grimoire_model::validate::project_config`]; its
//!     [`Lockfile`], if it has one; the running CLI's version; an [`Update`];
//!     and the clients this build has adapters for.
//!   - [`Update`] says which assets may move away from their pins:
//!     [`Update::None`], [`Update::All`], or only those in [`Update::Only`].
//!   - On success it returns a [`Resolution`]: [`Resolution::assets`], one
//!     [`Resolved`] for each asset to install, by id, and
//!     [`Resolution::warnings`].
//! - **One resolved asset:** a [`Resolved`] holds
//!   - [`Resolved::asset`], the [`SourceAsset`]: the manifest, the SHA-256
//!     digest of its bytes, and the asset's files;
//!   - [`Resolved::selected_by`], every [`Selector`] that selected it: listed
//!     under `[assets]`, a bundle, a tag, or another asset's `requires`;
//!   - [`Resolved::allow`], the permissions it needs, all of which the project
//!     accepted;
//!   - [`Resolved::clients`], the clients it renders for;
//!   - [`Resolved::options`], its [`AssetOptions`], when it has a table under
//!     `[assets]`.
//! - **Before resolving:** [`missing_permissions`] gives the permissions each
//!   selected asset needs that the project hasn't accepted, and
//!   [`selected_ids`] gives the id of every asset the configuration selects
//!   from the catalog, including ones that would fail to resolve.
//!
//! # Example
//!
//! The `security-baseline` bundle, for Claude Code and Codex, with the
//! permissions its assets need already accepted:
//!
//! ```
//! use std::collections::BTreeSet;
//!
//! use agent_grimoire_catalog::Source;
//! use agent_grimoire_model::{AssetId, ClientId, ProjectConfig, Version};
//! use agent_grimoire_resolve::{Request, Update, missing_permissions, resolve};
//!
//! // The catalog built into this build of Grimoire.
//! let source = Source::builtin()?;
//! let mut config = ProjectConfig::from_toml(
//!     r#"
//!     format = 1
//!     clients = ["claude-code", "codex"]
//!     bundles = ["security-baseline"]
//!     "#,
//! )?;
//! // Accept everything the bundle's assets need, as `grimoire add` does once
//! // the user agrees. It writes these to grimoire.toml's `[allow]` table.
//! for (id, needed) in missing_permissions(&source, &config) {
//!     config.allow.insert(id, needed);
//! }
//!
//! let version = Version::parse(env!("CARGO_PKG_VERSION"))?;
//! let resolution = resolve(
//!     &source,
//!     &Request {
//!         config: &config,
//!         lock: None, // no grimoire.lock yet
//!         cli_version: &version,
//!         update: &Update::None,
//!         // The clients the CLI has adapters for: all six in this build.
//!         available_clients: &ClientId::ALL,
//!     },
//! )
//! .expect("the selection resolves");
//!
//! // Codex has no place for an agent or for hooks, so the bundle's agent and
//! // its hook tool are installed for Claude Code only, each with a warning.
//! for id in ["security-reviewer", "secret-guard"] {
//!     let asset = &resolution.assets[&AssetId::new(id)?];
//!     assert_eq!(asset.clients, BTreeSet::from([ClientId::ClaudeCode]));
//! }
//! let not_covered = resolution
//!     .warnings
//!     .iter()
//!     .filter(|d| d.code == "resolve/client-not-covered");
//! assert_eq!(not_covered.count(), 2);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # How assets are selected
//!
//! An asset is selected when:
//!
//! - it is listed under `[assets]` ([`Selector::Direct`]);
//! - it is in a bundle listed in `bundles`, a named list of assets that the
//!   catalog defines ([`Selector::Bundle`]);
//! - it carries a tag listed in `tags` ([`Selector::Tag`]);
//! - a selected asset lists it in its manifest's `requires`, followed to every
//!   depth ([`Selector::Requires`]).
//!
//! `exclude` drops an asset that a bundle or a tag would select. Excluding one
//! that a selected asset requires is an error (`resolve/excluded-dependency`).
//! A tag never selects an excluded or deprecated asset, nor one that supports
//! none of the project's clients. A `resolve/tag-skipped` warning names each
//! asset that a tag skips for having no client.
//!
//! An `[assets]` entry or a bundle that the catalog doesn't have is an error
//! (`resolve/unknown-asset`, `resolve/unknown-bundle`), and the message says so
//! when the id is the other kind. A tag that no asset carries is a warning
//! (`resolve/unknown-tag`). A selected asset that is deprecated gets a warning
//! (`resolve/deprecated`) with the reason and the replacement.
//!
//! # One version of each asset, and the lock
//!
//! A [`Source`] holds one version of each asset, by id. The CLI's source is the
//! catalog embedded in the binary, and each release embeds only one version of
//! each asset
//! ([ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md),
//! the architecture decision record that chose to embed the catalog). So
//! resolution checks that version rather than choosing one:
//!
//! - Every version requirement, from `[assets]`, a bundle or a `requires`, must
//!   match the catalog's version (`resolve/version`, naming who asked). A tag
//!   sets no requirement: it takes whatever version the catalog has,
//!   pre-releases (such as `1.0.0-rc.1`) included, which the requirement `*`
//!   would not match.
//! - When the lock pins an asset and [`Request::update`] doesn't let it move,
//!   the pin must be the catalog's version (`resolve/locked-version-unavailable`),
//!   and the manifest's digest must be the one the lock records
//!   (`resolve/locked-manifest-changed`). Both errors name `grimoire update
//!   <id>`. They are raised even when no file would change, because telling
//!   whether content is needed would mean rendering the pinned version, which
//!   this build doesn't have
//!   ([`docs/subsystems/resolve.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/resolve.md#resolution-errors)).
//!   An asset that the lock doesn't pin takes the catalog's version.
//! - [`Request::cli_version`] must be at least the asset's `min-cli-version`
//!   (`resolve/cli-too-old`).
//!
//! The CLI's `plan`, `apply` and `doctor` pass [`Update::None`]. `update`
//! passes [`Update::All`], or [`Update::Only`] with the ids it is given. `add`
//! and `remove` check the selection they are about to write with no lock at
//! all.
//!
//! # Clients
//!
//! [`Resolved::clients`] is every project client that the asset supports (has
//! a `[clients.<id>]` table for). If the asset's `[assets]` entry lists
//! `clients`, it is exactly those instead, and each must be one the asset
//! supports (`resolve/unsupported-client`). Otherwise, for each project client
//! the asset doesn't support:
//!
//! - if the client has a place for assets of its kind, the asset is skipped
//!   for that client with a `resolve/client-skipped` warning;
//! - if it has none, a `resolve/client-not-covered` warning says why. Grimoire
//!   writes hooks only for Claude Code, and gives Codex no agents, because no
//!   other client can keep a failed hook from letting the action through, and
//!   a Codex agent can't be limited to its declared tools
//!   ([ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//!   which amends
//!   [ADR-0005](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0005-client-priority.md),
//!   the choice of the six clients and their order). `agents-md` never gets
//!   this warning, because by design it holds no agents, MCP servers or hooks.
//!
//! An asset left with no client is a `resolve/unsupported-client` error that
//! names the clients it supports.
//!
//! A project client that isn't in [`Request::available_clients`] is a
//! `resolve/client-unavailable` error. The CLI passes every client, because
//! this build has an adapter for each, so it never raises this one.
//!
//! # Permission consent
//!
//! An asset's manifest declares the permissions it needs, and resolution fails
//! with `resolve/permission` for any that `grimoire.toml` doesn't accept: the
//! declaration is not just documentation
//! ([`docs/subsystems/resolve.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/resolve.md#permission-consent)).
//! The check runs against the catalog's manifest every time, so a new version
//! that needs more permissions stops `update` and `apply` until the user
//! accepts them: there is no silent escalation. Each environment variable is its own grant,
//! `env:<NAME>`, so a version that reads one more variable needs consent again.
//!
//! - An asset listed under `[assets]` takes its grants from that entry's
//!   `allow`, and any other asset from the top-level `[allow]` table
//!   ([`ProjectConfig::allowed`]). A `filesystem-write` grant covers
//!   `filesystem-read`.
//! - The error names the command, or the `allow` entry, that would accept what
//!   is missing.
//! - [`Resolved::allow`] is what the asset needs, not everything the project
//!   granted: an asset that needs `filesystem-read`, under a
//!   `filesystem-write` grant, gets `filesystem-read`. The CLI records this set
//!   in the lock.
//! - An `[allow]` entry for an asset that isn't selected is a
//!   `resolve/allow-unused` warning.
//!
//! [`missing_permissions`] gives what to ask for, ignoring every other problem.
//! `add` asks only for the assets that its arguments select, through
//! [`selected_ids`], never for other selected assets. `remove` uses
//! [`selected_ids`] to keep the grant of an asset that something else still
//! selects.
//!
//! # Errors and diagnostics
//!
//! [`resolve`] returns `Err` with every diagnostic, warnings included, when any
//! of them is an error, and otherwise [`Resolution::warnings`]. They are sorted
//! by file, field, code and message, and exact duplicates are removed. Match on
//! [`Diagnostic::code`], not on the message: a code keeps its meaning, and the
//! message text can change in any release.
//! [`docs/diagnostics.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md#resolution-resolve)
//! lists every `resolve/` code with what raises it and what to do. The CLI
//! prints them all, and `add`, `remove`, `plan`, `apply` and `update` then
//! exit with code 3.
//!
//! An error doesn't stop the run: every selected asset is still checked, with
//! two exceptions. Conflicts (`resolve/conflict`) are checked only between
//! assets that passed all their own checks, and an asset whose clients fail
//! (`resolve/unsupported-client`, including one unsupported client in its
//! `[assets]` `clients` list) isn't checked for permissions. Fixing one error
//! can therefore reveal another.
//!
//! ```
//! use agent_grimoire_catalog::Source;
//! use agent_grimoire_model::{ClientId, ProjectConfig, Version};
//! use agent_grimoire_resolve::{Request, Update, resolve};
//!
//! let source = Source::builtin()?;
//! let config = ProjectConfig::from_toml(
//!     r#"
//!     format = 1
//!     clients = ["claude-code"]
//!     bundles = ["no-such-bundle"]
//!
//!     [assets]
//!     no-such-asset = "^1"
//!     "#,
//! )?;
//! let version = Version::parse(env!("CARGO_PKG_VERSION"))?;
//! let request = Request {
//!     config: &config,
//!     lock: None,
//!     cli_version: &version,
//!     update: &Update::None,
//!     available_clients: &ClientId::ALL,
//! };
//!
//! // Both errors, in one run, sorted by field: `assets.no-such-asset`, then `bundles`.
//! let diagnostics = resolve(&source, &request).unwrap_err();
//! let codes: Vec<&str> = diagnostics.iter().map(|d| d.code).collect();
//! assert_eq!(codes, ["resolve/unknown-asset", "resolve/unknown-bundle"]);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_catalog::{Source, SourceAsset};
use agent_grimoire_model::{
    Anchor, AssetId, AssetOptions, ClientId, Diagnostic, Kind, Lockfile, Maturity, Permission,
    ProjectConfig, Selector, Version, VersionReq, has_errors, sort_diagnostics,
};

// The README's examples are compiled and run as doc tests too, so the
// crates.io page breaks the build when the API changes under it.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Which assets may move away from the versions pinned in the lock.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum Update {
    /// Keep every pin.
    #[default]
    None,
    /// Move every asset to the catalog's version.
    All,
    /// Move only these assets.
    Only(BTreeSet<AssetId>),
}

impl Update {
    fn allows(&self, id: &AssetId) -> bool {
        match self {
            Update::None => false,
            Update::All => true,
            Update::Only(ids) => ids.contains(id),
        }
    }
}

/// What to resolve.
#[derive(Debug, Clone, Copy)]
pub struct Request<'a> {
    /// The project configuration, already validated on its own.
    pub config: &'a ProjectConfig,
    /// The current lock, if the project has one.
    pub lock: Option<&'a Lockfile>,
    /// The running CLI's version, checked against `min-cli-version`.
    pub cli_version: &'a Version,
    /// Which pins may move.
    pub update: &'a Update,
    /// Clients this build has adapters for.
    pub available_clients: &'a [ClientId],
}

/// One asset to install.
#[derive(Debug)]
pub struct Resolved<'s> {
    /// The asset in the source.
    pub asset: &'s SourceAsset,
    /// Why it is installed.
    pub selected_by: BTreeSet<Selector>,
    /// The permissions it needs, all of which the user accepted.
    pub allow: BTreeSet<Permission>,
    /// The clients it renders for.
    pub clients: BTreeSet<ClientId>,
    /// Its `[assets]` options, when it has a table there.
    pub options: Option<AssetOptions>,
}

impl Resolved<'_> {
    /// The asset id.
    #[must_use]
    pub fn id(&self) -> &AssetId {
        &self.asset.manifest.id
    }
}

/// A successful resolution and its warnings.
#[derive(Debug)]
pub struct Resolution<'s> {
    /// The assets to install, by id.
    pub assets: BTreeMap<AssetId, Resolved<'s>>,
    /// Warnings found while resolving.
    pub warnings: Vec<Diagnostic>,
}

/// One reason an asset is selected, with the requirement that came with it.
struct Demand {
    selector: Selector,
    /// `None` for a tag, which accepts whatever version the catalog has,
    /// pre-releases included (`*` would not match a pre-release).
    req: Option<VersionReq>,
}

/// Resolves `request` against `source`.
///
/// # Errors
///
/// Returns every diagnostic, warnings included, when any error is found.
pub fn resolve<'s>(
    source: &'s Source,
    request: &Request<'_>,
) -> Result<Resolution<'s>, Vec<Diagnostic>> {
    let config = request.config;
    let mut out = Vec::new();

    for client in &config.clients {
        if !request.available_clients.contains(client) {
            out.push(
                Diagnostic::error(
                    "resolve/client-unavailable",
                    format!(
                        "this version of grimoire cannot render for `{client}` yet; available: {}",
                        list(request.available_clients)
                    ),
                )
                .at("clients")
                .in_file("grimoire.toml"),
            );
        }
    }

    let demands = select(source, config, &mut out);
    let mut assets = BTreeMap::new();
    for (id, demands) in &demands {
        let Some(asset) = source.asset(id) else {
            continue;
        };
        if let Some(resolved) = check_asset(asset, demands, request, &mut out) {
            assets.insert(id.clone(), resolved);
        }
    }
    conflicts(&assets, &mut out);
    for id in config.allow.keys().filter(|id| !demands.contains_key(*id)) {
        out.push(
            Diagnostic::warning(
                "resolve/allow-unused",
                format!("`{id}` is not installed, so its `allow` entry has no effect"),
            )
            .at(format!("allow.{id}"))
            .in_file("grimoire.toml"),
        );
    }
    sort_diagnostics(&mut out);
    out.dedup();
    if has_errors(&out) {
        return Err(out);
    }
    Ok(Resolution {
        assets,
        warnings: out,
    })
}

/// The permissions each selected asset needs but `config` has not accepted, for
/// assets that exist in `source`. Used to ask for consent before resolving.
#[must_use]
pub fn missing_permissions(
    source: &Source,
    config: &ProjectConfig,
) -> BTreeMap<AssetId, BTreeSet<Permission>> {
    let mut ignored = Vec::new();
    select(source, config, &mut ignored)
        .keys()
        .filter_map(|id| {
            let asset = source.asset(id)?;
            let missing = asset.manifest.permissions.missing(&config.allowed(id));
            (!missing.is_empty()).then(|| (id.clone(), missing))
        })
        .collect()
}

/// The asset ids `config` selects, found without resolving them and without
/// reporting errors:
///
/// - each `[assets]` id that is in the catalog;
/// - each member of a listed bundle that `exclude` doesn't name, even a member
///   missing from the catalog;
/// - each asset a tag selects (not deprecated, not excluded, and supporting at
///   least one of the project's clients);
/// - and, transitively, each `requires` target of a catalog asset in the set,
///   if the target is in the catalog and not excluded.
///
/// An id in the set can still fail to resolve, for example on its version or
/// its clients.
#[must_use]
pub fn selected_ids(source: &Source, config: &ProjectConfig) -> BTreeSet<AssetId> {
    let mut ignored = Vec::new();
    select(source, config, &mut ignored).into_keys().collect()
}

/// Collects every asset the configuration selects, directly or transitively.
fn select(
    source: &Source,
    config: &ProjectConfig,
    out: &mut Vec<Diagnostic>,
) -> BTreeMap<AssetId, Vec<Demand>> {
    let excluded: BTreeSet<&AssetId> = config.exclude.iter().collect();
    let mut demands: BTreeMap<AssetId, Vec<Demand>> = BTreeMap::new();
    let mut queue: Vec<AssetId> = Vec::new();
    let mut demand =
        |id: &AssetId, selector: Selector, req: Option<VersionReq>, queue: &mut Vec<AssetId>| {
            let entry = demands.entry(id.clone()).or_default();
            if entry.is_empty() {
                queue.push(id.clone());
            }
            entry.push(Demand { selector, req });
        };

    for (id, selection) in &config.assets {
        if source.asset(id).is_some() {
            demand(
                id,
                Selector::Direct,
                Some(selection.version().clone()),
                &mut queue,
            );
        } else {
            let hint = if source.bundle(id).is_some() {
                format!("; `{id}` is a bundle, so list it under `bundles`")
            } else {
                String::new()
            };
            out.push(
                Diagnostic::error(
                    "resolve/unknown-asset",
                    format!("`{id}` is not in this grimoire's catalog{hint}"),
                )
                .at(format!("assets.{id}"))
                .in_file("grimoire.toml"),
            );
        }
    }
    for bundle_id in &config.bundles {
        let Some(bundle) = source.bundle(bundle_id) else {
            let hint = if source.asset(bundle_id).is_some() {
                format!("; `{bundle_id}` is an asset, so list it under `[assets]`")
            } else {
                String::new()
            };
            out.push(
                Diagnostic::error(
                    "resolve/unknown-bundle",
                    format!("bundle `{bundle_id}` is not in this grimoire's catalog{hint}"),
                )
                .at("bundles")
                .in_file("grimoire.toml"),
            );
            continue;
        };
        for (member, req) in &bundle.assets {
            if !excluded.contains(member) {
                demand(
                    member,
                    Selector::Bundle(bundle_id.clone()),
                    Some(req.clone()),
                    &mut queue,
                );
            }
        }
    }
    for (id, selector) in tagged(source, config, &excluded, out) {
        demand(&id, selector, None, &mut queue);
    }
    while let Some(id) = queue.pop() {
        let Some(asset) = source.asset(&id) else {
            continue;
        };
        for (dep, req) in &asset.manifest.requires {
            if excluded.contains(dep) {
                out.push(
                    Diagnostic::error(
                        "resolve/excluded-dependency",
                        format!("`{id}` requires `{dep}`, which grimoire.toml excludes"),
                    )
                    .at("exclude")
                    .in_file("grimoire.toml"),
                );
            } else if source.asset(dep).is_none() {
                out.push(Diagnostic::error(
                    "resolve/missing-dependency",
                    format!("`{id}` requires `{dep}`, which is not in this grimoire's catalog"),
                ));
            } else {
                demand(
                    dep,
                    Selector::Requires(id.clone()),
                    Some(req.clone()),
                    &mut queue,
                );
            }
        }
    }
    demands
}

/// Assets selected by `config.tags`, never deprecated or excluded ones.
fn tagged(
    source: &Source,
    config: &ProjectConfig,
    excluded: &BTreeSet<&AssetId>,
    out: &mut Vec<Diagnostic>,
) -> Vec<(AssetId, Selector)> {
    let mut selected = Vec::new();
    for tag in &config.tags {
        let tagged: Vec<&SourceAsset> = source
            .assets()
            .values()
            .filter(|a| a.manifest.tags.contains(tag))
            .collect();
        if tagged.is_empty() {
            out.push(
                Diagnostic::warning(
                    "resolve/unknown-tag",
                    format!("no asset in this grimoire's catalog has the tag `{tag}`"),
                )
                .at("tags")
                .in_file("grimoire.toml"),
            );
        }
        // Tags never pull in deprecated assets, nor ones no project client can use.
        for asset in tagged
            .into_iter()
            .filter(|a| a.manifest.maturity != Maturity::Deprecated)
            .filter(|a| !excluded.contains(&a.manifest.id))
        {
            let id = &asset.manifest.id;
            if !config
                .clients
                .iter()
                .any(|c| asset.manifest.clients.contains_key(c))
            {
                out.push(Diagnostic::warning(
                    "resolve/tag-skipped",
                    format!("tag `{tag}` matches `{id}`, which supports none of this project's clients, so it is not installed"),
                ));
                continue;
            }
            selected.push((id.clone(), Selector::Tag(tag.clone())));
        }
    }
    selected
}

/// Checks one selected asset and decides its clients and permissions.
fn check_asset<'s>(
    asset: &'s SourceAsset,
    demands: &[Demand],
    request: &Request<'_>,
    out: &mut Vec<Diagnostic>,
) -> Option<Resolved<'s>> {
    let m = &asset.manifest;
    let id = &m.id;
    let selected_by: BTreeSet<Selector> = demands.iter().map(|d| d.selector.clone()).collect();
    let mut ok = versions(asset, demands, request, out);

    if let (Maturity::Deprecated, Some(dep)) = (m.maturity, &m.deprecated) {
        out.push(Diagnostic::warning(
            "resolve/deprecated",
            format!(
                "`{id}` is deprecated: {}. Use `{}` instead",
                dep.reason, dep.replacement
            ),
        ));
    }

    let options = request
        .config
        .assets
        .get(id)
        .and_then(|s| s.options())
        .cloned();
    if let Some(o) = &options
        && m.kind != Kind::McpServer
        && (o.transport.is_some() || o.url.is_some() || !o.env.is_empty())
    {
        ok = false;
        out.push(
            Diagnostic::error(
                "resolve/options-kind",
                format!(
                    "`{id}` is a {}: `transport`, `url` and `env` are only for MCP servers",
                    m.kind
                ),
            )
            .at(format!("assets.{id}"))
            .in_file("grimoire.toml"),
        );
    }
    let Some(clients) = clients(asset, options.as_ref(), request, out) else {
        if !request.config.assets.contains_key(id) {
            out.push(Diagnostic::error(
                "resolve/unsupported-client",
                format!(
                    "`{id}` is {}; to skip it, add it to `exclude` in grimoire.toml",
                    describe_all(&selected_by)
                ),
            ));
        }
        return None;
    };

    let accepted = request.config.allowed(id);
    let missing = m.permissions.missing(&accepted);
    if !missing.is_empty() {
        ok = false;
        let names: Vec<String> = missing.iter().map(ToString::to_string).collect();
        let mut all: Vec<String> = accepted.iter().map(ToString::to_string).collect();
        all.extend(names.iter().cloned());
        all.sort_unstable();
        all.dedup();
        let how = if request.config.assets.contains_key(id) {
            format!(
                "run `grimoire add {id} --allow {}`, or set `allow = [{}]` in its [assets] entry",
                names.join(","),
                quoted(&all)
            )
        } else {
            let via = selected_by
                .iter()
                .find_map(|s| match s {
                    Selector::Bundle(b) => Some(format!("bundle:{b}")),
                    Selector::Tag(t) => Some(format!("tag:{t}")),
                    _ => None,
                })
                .map(|spec| format!("run `grimoire add {spec} --allow {}`, or ", names.join(",")))
                .unwrap_or_default();
            format!(
                "{via}set `{id} = [{}]` under [allow] in grimoire.toml",
                quoted(&all)
            )
        };
        out.push(
            Diagnostic::error(
                "resolve/permission",
                format!(
                    "`{id}` ({}) needs permissions you have not accepted: {}. Review them with `grimoire show {id}`, then {how}",
                    describe_all(&selected_by),
                    names.join(", ")
                ),
            )
            .in_file("grimoire.toml"),
        );
    }
    ok.then(|| Resolved {
        asset,
        selected_by,
        allow: m.permissions.required(),
        clients,
        options,
    })
}

/// Checks requirements, lock pins and `min-cli-version`. Returns whether all hold.
fn versions(
    asset: &SourceAsset,
    demands: &[Demand],
    request: &Request<'_>,
    out: &mut Vec<Diagnostic>,
) -> bool {
    let m = &asset.manifest;
    let id = &m.id;
    let mut ok = true;

    for (d, req) in demands
        .iter()
        .filter_map(|d| d.req.as_ref().map(|r| (d, r)))
        .filter(|(_, r)| !r.matches(&m.version))
    {
        ok = false;
        out.push(Diagnostic::error(
            "resolve/version",
            format!(
                "{} needs `{id}` {req}, but this grimoire's catalog has {}",
                describe(&d.selector),
                m.version
            ),
        ));
    }
    if let Some(locked) = request
        .lock
        .and_then(|l| l.assets.iter().find(|a| a.id == *id))
        && !request.update.allows(id)
    {
        if locked.version != m.version {
            ok = false;
            out.push(Diagnostic::error(
                "resolve/locked-version-unavailable",
                format!(
                    "grimoire.lock pins `{id}` {}, but this grimoire's catalog has {}. Run `grimoire update {id}` to move to {}, or use a grimoire release that contains {}",
                    locked.version, m.version, m.version, locked.version
                ),
            ).in_file("grimoire.lock"));
        } else if locked.manifest != asset.digest {
            ok = false;
            out.push(Diagnostic::error(
                "resolve/locked-manifest-changed",
                format!(
                    "`{id}` {} in this grimoire's catalog differs from the one grimoire.lock records. Run `grimoire update {id}`",
                    m.version
                ),
            ).in_file("grimoire.lock"));
        }
    }
    if *request.cli_version < m.min_cli_version {
        ok = false;
        out.push(Diagnostic::error(
            "resolve/cli-too-old",
            format!(
                "`{id}` needs grimoire {} or newer; this is {}",
                m.min_cli_version, request.cli_version
            ),
        ));
    }
    ok
}

/// The clients an asset renders for.
///
/// Clients listed explicitly in its `[assets]` entry must be supported. Otherwise
/// the asset renders for every project client it supports, and one the asset
/// merely does not support gets a warning. A client that cannot host the asset's
/// kind at all is skipped with a warning too (hooks are Claude Code only, and
/// Codex has no agents, as
/// [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)
/// decided), except `agents-md`, which by design holds no agents, MCP servers
/// or hooks.
fn clients(
    asset: &SourceAsset,
    options: Option<&AssetOptions>,
    request: &Request<'_>,
    out: &mut Vec<Diagnostic>,
) -> Option<BTreeSet<ClientId>> {
    let m = &asset.manifest;
    let id = &m.id;
    let supported: Vec<ClientId> = m.clients.keys().copied().collect();
    let mut chosen = BTreeSet::new();
    if let Some(explicit) = options.and_then(|o| o.clients.as_ref()) {
        let mut ok = true;
        for client in explicit {
            if supported.contains(client) {
                chosen.insert(*client);
            } else {
                ok = false;
                out.push(
                    Diagnostic::error(
                        "resolve/unsupported-client",
                        format!(
                            "`{id}` does not support `{client}`; it supports {}",
                            list(&supported)
                        ),
                    )
                    .at(format!("assets.{id}.clients"))
                    .in_file("grimoire.toml"),
                );
            }
        }
        return ok.then_some(chosen);
    }
    for client in &request.config.clients {
        if supported.contains(client) {
            chosen.insert(*client);
        } else if hosts_kind(*client, m.kind) {
            out.push(Diagnostic::warning(
                "resolve/client-skipped",
                format!(
                    "`{id}` does not support `{client}`, so it is installed only for {}",
                    list(
                        &supported
                            .iter()
                            .filter(|c| request.config.clients.contains(c))
                            .copied()
                            .collect::<Vec<_>>()
                    )
                ),
            ));
        } else if *client != ClientId::AgentsMd {
            let why = match m.kind {
                Kind::Tool => "grimoire writes hooks only for Claude Code (ADR-0008)",
                Kind::Agent => "a Codex agent can't be limited to its tools (ADR-0008)",
                _ => "grimoire has no place to put it for that client",
            };
            out.push(Diagnostic::warning(
                "resolve/client-not-covered",
                format!("`{id}` is not installed for `{client}`: {why}"),
            ));
        }
    }
    if chosen.is_empty() {
        out.push(Diagnostic::error(
            "resolve/unsupported-client",
            format!(
                "`{id}` supports none of this project's clients ({}); it supports {}",
                list(&request.config.clients),
                list(&supported)
            ),
        ));
        return None;
    }
    Some(chosen)
}

/// Whether `client` has any destination for assets of `kind`.
fn hosts_kind(client: ClientId, kind: Kind) -> bool {
    Anchor::ALL
        .into_iter()
        .any(|a| a.accepts_kind(kind) && client.supports(a))
}

/// Reports selected pairs that conflict.
fn conflicts(assets: &BTreeMap<AssetId, Resolved<'_>>, out: &mut Vec<Diagnostic>) {
    let mut reported = BTreeSet::new();
    for (id, resolved) in assets {
        for (other, spec) in &resolved.asset.manifest.conflicts {
            let Some(target) = assets.get(other) else {
                continue;
            };
            if !spec.version.matches(&target.asset.manifest.version) {
                continue;
            }
            let pair = if id < other { (id, other) } else { (other, id) };
            if !reported.insert(pair) {
                continue;
            }
            out.push(Diagnostic::error(
                "resolve/conflict",
                format!(
                    "`{id}` ({}) conflicts with `{other}` ({}): {}",
                    describe_all(&resolved.selected_by),
                    describe_all(&target.selected_by),
                    spec.reason
                ),
            ));
        }
    }
}

fn describe(selector: &Selector) -> String {
    match selector {
        Selector::Direct => "grimoire.toml".to_owned(),
        Selector::Bundle(b) => format!("bundle `{b}`"),
        Selector::Tag(t) => format!("tag `{t}`"),
        Selector::Requires(id) => format!("`{id}`"),
    }
}

fn describe_all(selectors: &BTreeSet<Selector>) -> String {
    let parts: Vec<String> = selectors
        .iter()
        .map(|s| match s {
            Selector::Direct => "selected directly".to_owned(),
            Selector::Bundle(b) => format!("from bundle `{b}`"),
            Selector::Tag(t) => format!("from tag `{t}`"),
            Selector::Requires(id) => format!("required by `{id}`"),
        })
        .collect();
    parts.join(", ")
}

fn list(clients: &[ClientId]) -> String {
    if clients.is_empty() {
        return "no clients".to_owned();
    }
    clients
        .iter()
        .map(|c| format!("`{c}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn quoted<S: AsRef<str>>(names: &[S]) -> String {
    names
        .iter()
        .map(|n| format!("\"{}\"", n.as_ref()))
        .collect::<Vec<_>>()
        .join(", ")
}
