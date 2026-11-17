//! Selection and dependency resolution (`architecture.md` §7.1, §7.5).
//!
//! [`resolve`] turns a project's `grimoire.toml`, its lock and a catalog
//! [`Source`] into the set of assets to install: which version of each, why it is
//! selected, which clients it renders for and which permissions were accepted.
//! Every failure is a distinct diagnostic, and all of them are reported at once.

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_catalog::{Source, SourceAsset};
use agent_grimoire_model::{
    Anchor, AssetId, AssetOptions, ClientId, Diagnostic, Kind, Lockfile, Maturity, Permission,
    ProjectConfig, Selector, Version, VersionReq, has_errors, sort_diagnostics,
};

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

/// Every asset id `config` selects (directly, through bundles and tags, and
/// through `requires`), whether or not it would resolve.
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
/// kind at all is skipped with a warning too (ADR-0008: hooks are Claude Code
/// only, and Codex has no agents), except `agents-md`, which by design holds no
/// agents, MCP servers or hooks.
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
