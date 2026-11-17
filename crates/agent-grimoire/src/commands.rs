//! One function per command.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use agent_grimoire_adapters::AVAILABLE;
use agent_grimoire_install::apply::{self, Options};
use agent_grimoire_install::plan::Action;
use agent_grimoire_model::{
    AssetId, ClientId, Diagnostic, Kind, Lockfile, Maturity, Permission, RelPath, Tag,
};
use agent_grimoire_resolve::{Request, Update, missing_permissions, resolve};
use serde_json::{Value, json};

use crate::cli::{ApplyArgs, Cli, Command};
use crate::out::{Exit, Out};
use crate::pipeline::{Caller, prepare, prepare_uninstall, run_apply, show};
use crate::project::{CONFIG, ConfigEdit, LOCK, Project, new_config, parse_config};

pub(crate) fn run(cli: Cli, out: &mut Out<'_, '_>) -> Exit {
    let dir = cli.global.directory;
    let no_input = cli.global.no_input;
    if no_input {
        out.ctx.interactive = false;
    }
    match cli.command {
        Command::Init { clients } => {
            out.command("init");
            init(
                out,
                dir.as_ref(),
                clients.into_iter().map(Into::into).collect(),
            )
        }
        Command::List {
            query,
            kind,
            tag,
            client,
            maturity,
        } => {
            out.command("list");
            list(
                out,
                query.as_deref(),
                kind.map(Into::into),
                tag.as_deref(),
                client.map(Into::into),
                maturity.map(Into::into),
            )
        }
        Command::Show { id } => {
            out.command("show");
            show_entry(out, &id)
        }
        Command::Add {
            specs,
            allow,
            apply,
        } => {
            out.command("add");
            add(out, dir.as_ref(), &specs, &allow, apply)
        }
        Command::Remove { specs, apply } => {
            out.command("remove");
            remove(out, dir.as_ref(), &specs, apply)
        }
        Command::Plan { check } => {
            out.command("plan");
            plan(out, dir.as_ref(), check)
        }
        Command::Apply(args) => {
            out.command("apply");
            apply_cmd(out, dir.as_ref(), args, &Update::None, Caller::Apply)
        }
        Command::Update {
            ids,
            no_apply,
            apply,
        } => {
            out.command("update");
            update(out, dir.as_ref(), &ids, no_apply, apply)
        }
        Command::Verify => {
            out.command("verify");
            verify(out, dir.as_ref())
        }
        Command::Uninstall { purge, force } => {
            out.command("uninstall");
            uninstall(out, dir.as_ref(), purge, force)
        }
        Command::Doctor => {
            out.command("doctor");
            doctor(out, dir.as_ref())
        }
    }
}

fn options(args: ApplyArgs) -> Options {
    Options {
        force: args.force,
        write_new: args.write_new,
    }
}

fn exists(project: &Project, name: &str) -> bool {
    std::fs::symlink_metadata(project.dir.join(name)).is_ok()
}

fn init(out: &mut Out<'_, '_>, dir: Option<&PathBuf>, mut clients: Vec<ClientId>) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    match project.read(out, CONFIG) {
        Ok(Some(_)) => {
            return out.fail(
                Exit::Invalid,
                "init/exists",
                format!("{CONFIG} already exists in {}", project.dir.display()),
            );
        }
        Ok(None) => {}
        Err(e) => return e,
    }
    let detected = clients.is_empty();
    if detected {
        if exists(&project, "CLAUDE.md") || exists(&project, ".claude") {
            clients.push(ClientId::ClaudeCode);
        }
        if exists(&project, "AGENTS.md") || exists(&project, ".agents") {
            clients.push(ClientId::AgentsMd);
        }
        if clients.is_empty() {
            return out.fail(
                Exit::Usage,
                "init/no-client",
                "no client detected; pass --client claude-code and/or --client agents-md",
            );
        }
    }
    clients.sort();
    clients.dedup();
    if let Some(c) = clients.iter().find(|c| !AVAILABLE.contains(c)) {
        return out.fail(
            Exit::Invalid,
            "init/client-unavailable",
            format!("this version of grimoire cannot render for `{c}` yet; available: claude-code, agents-md"),
        );
    }
    if let Err(e) = project.write(out, CONFIG, &new_config(&clients)) {
        return e;
    }
    let names: Vec<&str> = clients.iter().map(|c| c.as_str()).collect();
    out.line(format!(
        "init: wrote {CONFIG} for {}{}",
        names.join(", "),
        if detected { " (detected)" } else { "" }
    ));
    out.result(json!({ "config": CONFIG, "clients": names, "detected": detected }));
    Exit::Ok
}

fn matches_query(text: &str, query: &str) -> bool {
    text.to_lowercase().contains(&query.to_lowercase())
}

fn list(
    out: &mut Out<'_, '_>,
    query: Option<&str>,
    kind: Option<Kind>,
    tag: Option<&str>,
    client: Option<ClientId>,
    maturity: Option<Maturity>,
) -> Exit {
    let source = out.ctx.source;
    let mut assets = Vec::new();
    let mut rows = Vec::new();
    for asset in source.assets().values() {
        let m = &asset.manifest;
        let hit = query.is_none_or(|q| {
            matches_query(m.id.as_str(), q)
                || matches_query(m.summary.as_str(), q)
                || matches_query(m.description.as_str(), q)
                || m.tags.iter().any(|t| matches_query(t.as_str(), q))
        });
        if !hit
            || kind.is_some_and(|k| k != m.kind)
            || tag.is_some_and(|t| !m.tags.iter().any(|x| x.as_str() == t))
            || client.is_some_and(|c| !m.clients.contains_key(&c))
            || maturity.is_some_and(|x| x != m.maturity)
        {
            continue;
        }
        let clients: Vec<&str> = m.clients.keys().map(|c| c.as_str()).collect();
        rows.push(format!(
            "{:<28} {:<10} {:<10} {:<12} {}",
            m.id,
            m.kind,
            m.version,
            maturity_name(m.maturity),
            m.summary
        ));
        assets.push(json!({
            "id": m.id.as_str(),
            "kind": m.kind.as_str(),
            "version": m.version.to_string(),
            "maturity": maturity_name(m.maturity),
            "summary": m.summary.as_str(),
            "tags": m.tags.iter().map(Tag::as_str).collect::<Vec<_>>(),
            "clients": clients,
        }));
    }
    let mut bundles = Vec::new();
    if kind.is_none() && tag.is_none() && client.is_none() && maturity.is_none() {
        for b in source.bundles().values() {
            let hit = query.is_none_or(|q| {
                matches_query(b.id.as_str(), q)
                    || matches_query(b.summary.as_str(), q)
                    || matches_query(b.description.as_str(), q)
            });
            if !hit {
                continue;
            }
            rows.push(format!(
                "{:<28} {:<10} {:<10} {:<12} {}",
                b.id, "bundle", b.version, "", b.summary
            ));
            bundles.push(json!({
                "id": b.id.as_str(),
                "version": b.version.to_string(),
                "summary": b.summary.as_str(),
                "assets": b.assets.keys().map(AssetId::as_str).collect::<Vec<_>>(),
            }));
        }
    }
    if rows.is_empty() {
        out.note("list: nothing in this grimoire's catalog matches");
    }
    for row in rows {
        out.line(row);
    }
    out.result(json!({ "assets": assets, "bundles": bundles }));
    Exit::Ok
}

fn maturity_name(m: Maturity) -> &'static str {
    match m {
        Maturity::Experimental => "experimental",
        Maturity::Stable => "stable",
        Maturity::Deprecated => "deprecated",
    }
}

/// The first `## ` section of a changelog.
fn changelog_head(text: &str) -> Option<String> {
    let start = text
        .find("\n## ")
        .map(|i| i + 1)
        .or_else(|| text.starts_with("## ").then_some(0))?;
    let rest = &text[start..];
    let end = rest[3..].find("\n## ").map_or(rest.len(), |i| i + 4);
    Some(rest[..end].trim_end().to_owned())
}

fn show_entry(out: &mut Out<'_, '_>, id: &str) -> Exit {
    let source = out.ctx.source;
    let Ok(id) = AssetId::new(id) else {
        return out.fail(
            Exit::Invalid,
            "show/unknown",
            format!("`{id}` is not a valid asset or bundle id"),
        );
    };
    if let Some(bundle) = source.bundle(&id) {
        return show_bundle(out, bundle);
    }
    let Some(asset) = source.asset(&id) else {
        return out.fail(
            Exit::Invalid,
            "show/unknown",
            format!("`{id}` is not in this grimoire's catalog"),
        );
    };
    show_asset(out, asset)
}

fn show_bundle(out: &mut Out<'_, '_>, bundle: &agent_grimoire_model::Bundle) -> Exit {
    out.line(format!(
        "{} {} (bundle)\n{}\n\n{}",
        bundle.id, bundle.version, bundle.summary, bundle.description
    ));
    out.line("\nAssets:");
    for (member, req) in &bundle.assets {
        out.line(format!("  {member} {req}"));
    }
    out.result(json!({
        "id": bundle.id.as_str(),
        "kind": "bundle",
        "version": bundle.version.to_string(),
        "summary": bundle.summary.as_str(),
        "description": bundle.description.as_str(),
        "assets": bundle.assets.iter().map(|(k, v)| (k.as_str().to_owned(), Value::String(v.to_string()))).collect::<serde_json::Map<_, _>>(),
    }));
    Exit::Ok
}

fn show_asset(out: &mut Out<'_, '_>, asset: &agent_grimoire_catalog::SourceAsset) -> Exit {
    let m = &asset.manifest;
    let required: Vec<&str> = m
        .permissions
        .required()
        .into_iter()
        .map(Permission::as_str)
        .collect();
    out.line(format!(
        "{} {} ({}, {})",
        m.id,
        m.version,
        m.kind,
        maturity_name(m.maturity)
    ));
    out.line(m.summary.as_str());
    out.line("");
    out.line(m.description.as_str());
    out.line("");
    out.line(format!("License: {}", m.license));
    if let Some(dep) = &m.deprecated {
        out.line(format!(
            "Deprecated: {} (use `{}`)",
            dep.reason, dep.replacement
        ));
    }
    out.line(format!(
        "Permissions: {}",
        if required.is_empty() {
            "none".to_owned()
        } else {
            required.join(", ")
        }
    ));
    if !m.permissions.env.is_empty() {
        let names: Vec<&str> = m
            .permissions
            .env
            .iter()
            .map(agent_grimoire_model::EnvVarName::as_str)
            .collect();
        out.line(format!("Reads environment variables: {}", names.join(", ")));
    }
    for (dep, req) in &m.requires {
        out.line(format!("Requires: {dep} {req}"));
    }
    for (other, spec) in &m.conflicts {
        out.line(format!(
            "Conflicts with: {other} {} ({})",
            spec.version, spec.reason
        ));
    }
    let mut clients = serde_json::Map::new();
    out.line("Outputs:");
    for (client, outputs) in &m.clients {
        let mut list = Vec::new();
        for o in &outputs.outputs {
            let dest = agent_grimoire_adapters::destination(*client, &o.to, &m.id);
            out.line(format!("  {client}: {dest}"));
            list.push(Value::String(dest));
        }
        if outputs.outputs.is_empty() {
            out.line(format!(
                "  {client}: nothing (recorded in grimoire.lock only)"
            ));
        }
        clients.insert(client.as_str().to_owned(), Value::Array(list));
    }
    let changes = asset.changelog().and_then(changelog_head);
    if let Some(changes) = &changes {
        out.line("");
        out.line(changes);
    }
    out.result(json!({
        "id": m.id.as_str(),
        "kind": m.kind.as_str(),
        "version": m.version.to_string(),
        "maturity": maturity_name(m.maturity),
        "summary": m.summary.as_str(),
        "description": m.description.as_str(),
        "license": m.license.to_string(),
        "permissions": required,
        "env": m.permissions.env.iter().map(agent_grimoire_model::EnvVarName::as_str).collect::<Vec<_>>(),
        "requires": m.requires.iter().map(|(k, v)| (k.as_str().to_owned(), Value::String(v.to_string()))).collect::<serde_json::Map<_, _>>(),
        "clients": clients,
        "changelog": changes,
    }));
    Exit::Ok
}

/// What one command-line spec names.
enum Spec {
    Asset(AssetId),
    Bundle(AssetId),
    Tag(Tag),
}

fn parse_spec(out: &mut Out<'_, '_>, spec: &str) -> Result<Spec, Exit> {
    let source = out.ctx.source;
    let parsed = if let Some(rest) = spec.strip_prefix("bundle:") {
        AssetId::new(rest)
            .map(Spec::Bundle)
            .map_err(|e| e.to_string())
    } else if let Some(rest) = spec.strip_prefix("tag:") {
        Tag::new(rest).map(Spec::Tag).map_err(|e| e.to_string())
    } else {
        AssetId::new(spec)
            .map(|id| {
                if source.bundle(&id).is_some() {
                    Spec::Bundle(id)
                } else {
                    Spec::Asset(id)
                }
            })
            .map_err(|e| e.to_string())
    };
    parsed.map_err(|e| out.fail(Exit::Invalid, "cli/spec", format!("`{spec}`: {e}")))
}

fn add(
    out: &mut Out<'_, '_>,
    dir: Option<&PathBuf>,
    specs: &[String],
    allow: &[Permission],
    then_apply: bool,
) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (config, text) = match project.config(out) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let mut edit = match ConfigEdit::parse(&text) {
        Ok(e) => e,
        Err(e) => {
            return out.fail(
                Exit::Invalid,
                "project/config-parse",
                format!("{CONFIG}: {e}"),
            );
        }
    };
    let (added, named) = match edit_specs(out, &mut edit, &config, specs) {
        Ok(r) => r,
        Err(e) => return e,
    };
    let mut new = match parse_config(out, &edit.render()) {
        Ok(c) => c,
        Err(e) => return e,
    };
    // Consent covers only what this command names, never other selected assets.
    let scope = agent_grimoire_resolve::selected_ids(out.ctx.source, &named.of(&new));
    let granted = match consent(out, &mut edit, &new, allow, &scope) {
        Ok(g) => g,
        Err(e) => return e,
    };
    if !granted.is_empty() {
        new = match parse_config(out, &edit.render()) {
            Ok(c) => c,
            Err(e) => return e,
        };
    }
    // Resolve before writing, so a broken selection never reaches grimoire.toml.
    if !resolves(out, &new) {
        return out.fail(
            Exit::Invalid,
            "add/unresolvable",
            format!("{CONFIG} was not changed"),
        );
    }
    let rendered = edit.render();
    if rendered != text
        && let Err(e) = project.write(out, CONFIG, &rendered)
    {
        return e;
    }
    out.line(format!(
        "add: {}",
        if added.is_empty() {
            "nothing new".to_owned()
        } else {
            added.join(", ")
        }
    ));
    let grants: serde_json::Map<String, Value> = granted
        .iter()
        .map(|(id, p)| {
            out.line(format!(
                "add: allowed {} for {id}",
                p.iter().map(|x| x.as_str()).collect::<Vec<_>>().join(", ")
            ));
            (
                id.as_str().to_owned(),
                json!(p.iter().map(|x| x.as_str()).collect::<Vec<_>>()),
            )
        })
        .collect();
    let summary = json!({ "added": added, "allowed": grants });
    if !then_apply {
        out.result(summary);
        return Exit::Ok;
    }
    let applied = apply_cmd(
        out,
        dir,
        ApplyArgs {
            force: false,
            write_new: false,
        },
        &Update::None,
        Caller::AfterEdit,
    );
    out.extend_result("add", summary);
    applied
}

/// What the specs of one `add` name: the selection consent applies to.
#[derive(Default)]
struct Named {
    assets: BTreeSet<AssetId>,
    bundles: BTreeSet<AssetId>,
    tags: BTreeSet<Tag>,
}

impl Named {
    /// `config` narrowed to the named specs, so its selection is their closure.
    fn of(
        &self,
        config: &agent_grimoire_model::ProjectConfig,
    ) -> agent_grimoire_model::ProjectConfig {
        let mut narrowed = config.clone();
        narrowed.assets.retain(|id, _| self.assets.contains(id));
        narrowed.bundles.retain(|b| self.bundles.contains(b));
        narrowed.tags.retain(|t| self.tags.contains(t));
        narrowed
    }
}

/// Applies `add`'s specs to `edit`. Returns what was newly added and what was named.
fn edit_specs(
    out: &mut Out<'_, '_>,
    edit: &mut ConfigEdit,
    config: &agent_grimoire_model::ProjectConfig,
    specs: &[String],
) -> Result<(Vec<String>, Named), Exit> {
    let source = out.ctx.source;
    let mut added = Vec::new();
    let mut named = Named::default();
    for spec in specs {
        let parsed = parse_spec(out, spec)?;
        let (changed, shown) = match parsed {
            Spec::Asset(id) => {
                let Some(asset) = source.asset(&id) else {
                    return Err(out.fail(
                        Exit::Invalid,
                        "add/unknown",
                        format!("`{id}` is not in this grimoire's catalog; see `grimoire list`"),
                    ));
                };
                if config.exclude.contains(&id) {
                    return Err(out.fail(
                        Exit::Invalid,
                        "add/excluded",
                        format!("`{id}` is in `exclude`; remove it from there first"),
                    ));
                }
                let req = format!("^{}", asset.manifest.version);
                let changed = edit.add_asset(&id, &req);
                // A grant recorded under [allow] moves into the new [assets] entry.
                if changed && let Some(grants) = config.allow.get(&id) {
                    edit.remove_allow(&id);
                    edit.set_inline_allow(&id, grants);
                }
                let shown = id.to_string();
                named.assets.insert(id);
                (changed, shown)
            }
            Spec::Bundle(id) => {
                if source.bundle(&id).is_none() {
                    return Err(out.fail(
                        Exit::Invalid,
                        "add/unknown",
                        format!("bundle `{id}` is not in this grimoire's catalog"),
                    ));
                }
                let changed = edit.add_to_array("bundles", id.as_str());
                let shown = format!("bundle:{id}");
                named.bundles.insert(id);
                (changed, shown)
            }
            Spec::Tag(tag) => {
                let changed = edit.add_to_array("tags", tag.as_str());
                let shown = format!("tag:{tag}");
                named.tags.insert(tag);
                (changed, shown)
            }
        };
        if changed {
            added.push(shown);
        } else {
            out.note(format!("add: {shown} is already in {CONFIG}"));
        }
    }
    Ok((added, named))
}

/// Whether `config` resolves against the catalog, reporting why not, and its warnings.
fn resolves(out: &mut Out<'_, '_>, config: &agent_grimoire_model::ProjectConfig) -> bool {
    let version = out.ctx.version.clone();
    match resolve(
        out.ctx.source,
        &Request {
            config,
            lock: None,
            cli_version: &version,
            update: &Update::All,
            available_clients: &AVAILABLE,
        },
    ) {
        Ok(resolution) => {
            out.diags(resolution.warnings);
            true
        }
        Err(diagnostics) => {
            out.diags(diagnostics);
            false
        }
    }
}

/// Asks for, or takes from `--allow`, the permissions that assets in `scope` need.
/// Assets outside `scope` are left as they are: their missing permissions stay an
/// error until the user accepts them for that asset.
fn consent(
    out: &mut Out<'_, '_>,
    edit: &mut ConfigEdit,
    config: &agent_grimoire_model::ProjectConfig,
    allow: &[Permission],
    scope: &BTreeSet<AssetId>,
) -> Result<BTreeMap<AssetId, BTreeSet<Permission>>, Exit> {
    let source = out.ctx.source;
    let offered: BTreeSet<Permission> = allow.iter().copied().collect();
    let mut granted = BTreeMap::new();
    let mut refused = Vec::new();
    for (id, missing) in missing_permissions(source, config) {
        if !scope.contains(&id) {
            continue;
        }
        let all: Vec<&str> = missing.iter().map(|p| p.as_str()).collect();
        let mut grant: BTreeSet<Permission> = missing
            .iter()
            .copied()
            .filter(|p| p.is_satisfied_by(&offered))
            .collect();
        let rest: Vec<Permission> = missing
            .iter()
            .copied()
            .filter(|p| !grant.contains(p))
            .collect();
        if !rest.is_empty() {
            let names: Vec<&str> = rest.iter().map(|p| p.as_str()).collect();
            let summary = source
                .asset(&id)
                .map(|a| a.manifest.summary.to_string())
                .unwrap_or_default();
            out.note(format!("`{id}` ({summary}) needs: {}", all.join(", ")));
            if out.confirm(&format!("Allow {} for `{id}`?", names.join(", "))) {
                grant.extend(rest);
            } else {
                refused.push((id, all.join(",")));
                continue;
            }
        }
        let mut accepted = config.allowed(&id);
        accepted.extend(grant.iter().copied());
        if config.assets.contains_key(&id) {
            edit.set_inline_allow(&id, &accepted);
        } else {
            edit.set_allow(&id, &accepted);
        }
        granted.insert(id, grant);
    }
    if refused.is_empty() {
        return Ok(granted);
    }
    for (id, names) in &refused {
        out.diag(Diagnostic::error(
            "add/permission",
            format!("`{id}` needs permissions that were not accepted: {names}. Review them with `grimoire show {id}`, then pass --allow {names}"),
        ));
    }
    Err(out.fail(
        Exit::Invalid,
        "add/unresolvable",
        format!("{CONFIG} was not changed"),
    ))
}

/// Applies `remove`'s specs to `edit`. Returns what was removed, and which of
/// it were direct `[assets]` entries.
fn remove_specs(
    out: &mut Out<'_, '_>,
    edit: &mut ConfigEdit,
    config: &agent_grimoire_model::ProjectConfig,
    specs: &[String],
) -> Result<(Vec<String>, Vec<AssetId>), Exit> {
    let selected = agent_grimoire_resolve::selected_ids(out.ctx.source, config);
    let mut removed = Vec::new();
    let mut direct = Vec::new();
    for spec in specs {
        let parsed = parse_spec(out, spec)?;
        let (changed, shown) = match parsed {
            Spec::Asset(id) if config.assets.contains_key(&id) => {
                direct.push(id.clone());
                (edit.remove_asset(&id), id.to_string())
            }
            // A bundle no longer in the catalog is still named in `bundles`.
            Spec::Asset(id) if config.bundles.contains(&id) => (
                edit.remove_from_array("bundles", id.as_str()),
                format!("bundle:{id}"),
            ),
            Spec::Asset(id) if selected.contains(&id) => {
                return Err(out.fail(
                    Exit::Invalid,
                    "remove/indirect",
                    format!("`{id}` has no [assets] entry; it is installed through a bundle, a tag or another asset's `requires`. Add it to `exclude` in {CONFIG} to stop installing it"),
                ));
            }
            Spec::Asset(id) => (false, id.to_string()),
            Spec::Bundle(id) => (
                edit.remove_from_array("bundles", id.as_str()),
                format!("bundle:{id}"),
            ),
            Spec::Tag(tag) => (
                edit.remove_from_array("tags", tag.as_str()),
                format!("tag:{tag}"),
            ),
        };
        if !changed {
            return Err(out.fail(
                Exit::Invalid,
                "remove/absent",
                format!("{shown} is not in {CONFIG}"),
            ));
        }
        removed.push(shown);
    }
    Ok((removed, direct))
}

fn remove(
    out: &mut Out<'_, '_>,
    dir: Option<&PathBuf>,
    specs: &[String],
    then_apply: bool,
) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (config, text) = match project.config(out) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let mut edit = match ConfigEdit::parse(&text) {
        Ok(e) => e,
        Err(e) => {
            return out.fail(
                Exit::Invalid,
                "project/config-parse",
                format!("{CONFIG}: {e}"),
            );
        }
    };
    let source = out.ctx.source;
    let (removed, direct) = match remove_specs(out, &mut edit, &config, specs) {
        Ok(r) => r,
        Err(e) => return e,
    };
    let new = match parse_config(out, &edit.render()) {
        Ok(c) => c,
        Err(e) => return e,
    };
    // An asset that something else still selects keeps its grant, under [allow].
    let still = agent_grimoire_resolve::selected_ids(source, &new);
    for id in &direct {
        if still.contains(id) {
            let grants = config.allowed(id);
            if !grants.is_empty() {
                edit.set_allow(id, &grants);
            }
            out.diag(Diagnostic::warning(
                "remove/still-selected",
                format!("`{id}` stays installed because a bundle, a tag or another asset's `requires` still selects it; add it to `exclude` to stop installing it"),
            ));
        } else {
            edit.remove_allow(id);
        }
    }
    let rendered = edit.render();
    let new = match parse_config(out, &rendered) {
        Ok(c) => c,
        Err(e) => return e,
    };
    if !resolves(out, &new) {
        return out.fail(
            Exit::Invalid,
            "remove/unresolvable",
            format!("{CONFIG} was not changed"),
        );
    }
    if let Err(e) = project.write(out, CONFIG, &rendered) {
        return e;
    }
    out.line(format!("remove: {}", removed.join(", ")));
    let summary = json!({ "removed": removed });
    if !then_apply {
        out.result(summary);
        return Exit::Ok;
    }
    let applied = apply_cmd(
        out,
        dir,
        ApplyArgs {
            force: false,
            write_new: false,
        },
        &Update::None,
        Caller::AfterEdit,
    );
    out.extend_result("remove", summary);
    applied
}

fn plan(out: &mut Out<'_, '_>, dir: Option<&PathBuf>, check: bool) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (config, _) = match project.config(out) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let planned = match prepare(out, &project, &config, &Update::None) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let lock_changes = match show(out, &planned, true) {
        Ok(l) => l,
        Err(e) => return e,
    };
    if planned.plan.has_conflicts() {
        return out.fail(
            Exit::Conflict,
            "plan/conflicts",
            "apply would stop: some managed content conflicts (see above)",
        );
    }
    if check && (planned.plan.has_changes() || lock_changes) {
        return out.fail(
            Exit::Check,
            "plan/changes",
            "the project does not match grimoire.toml; run `grimoire apply`",
        );
    }
    Exit::Ok
}

fn apply_cmd(
    out: &mut Out<'_, '_>,
    dir: Option<&PathBuf>,
    args: ApplyArgs,
    update: &Update,
    caller: Caller,
) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (config, _) = match project.config(out) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let planned = match prepare(out, &project, &config, update) {
        Ok(p) => p,
        Err(e) => return e,
    };
    run_apply(out, &project, &planned, options(args), caller)
}

fn update(
    out: &mut Out<'_, '_>,
    dir: Option<&PathBuf>,
    ids: &[String],
    no_apply: bool,
    args: ApplyArgs,
) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (lock, _) = match project.lock(out) {
        Ok(l) => l,
        Err(e) => return e,
    };
    let mut only = BTreeSet::new();
    for id in ids {
        let Ok(id) = AssetId::new(id) else {
            return out.fail(
                Exit::Invalid,
                "update/unknown",
                format!("`{id}` is not a valid asset id"),
            );
        };
        if !lock.assets.iter().any(|a| a.id == id) {
            return out.fail(
                Exit::Invalid,
                "update/unknown",
                format!("`{id}` is not installed"),
            );
        }
        only.insert(id);
    }
    let selection = if only.is_empty() {
        Update::All
    } else {
        Update::Only(only)
    };
    if !no_apply {
        return apply_cmd(out, dir, args, &selection, Caller::Apply);
    }
    let (config, _) = match project.config(out) {
        Ok(c) => c,
        Err(e) => return e,
    };
    let planned = match prepare(out, &project, &config, &selection) {
        Ok(p) => p,
        Err(e) => return e,
    };
    // A file grimoire cannot read keeps a lock it cannot vouch for: stop instead.
    if planned.plan.files.iter().any(|f| f.unreadable.is_some()) {
        crate::pipeline::report_conflicts(out, &planned.plan);
        return out.fail(
            Exit::Conflict,
            "update/unreadable",
            "grimoire.lock was not changed; fix the files reported above first",
        );
    }
    // Files stay as they are, so every output keeps the record of what is on disk.
    let paths: BTreeSet<&RelPath> = planned.plan.files.iter().map(|f| &f.path).collect();
    let next = planned.next(&paths);
    match project.write_lock(out, &next, planned.old_text.as_deref()) {
        Ok(wrote) => {
            out.line(if wrote {
                "update: grimoire.lock updated; run `grimoire apply`"
            } else {
                "update: grimoire.lock is already up to date"
            });
            out.result(json!({ "lock-written": wrote }));
            Exit::Ok
        }
        Err(e) => e,
    }
}

fn verify(out: &mut Out<'_, '_>, dir: Option<&PathBuf>) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let (lock, text) = match project.lock(out) {
        Ok(l) => l,
        Err(e) => return e,
    };
    if text.is_none() {
        return out.fail(
            Exit::Invalid,
            "verify/no-lock",
            format!("no {LOCK} in {}", project.dir.display()),
        );
    }
    let mismatches = match apply::verify(&project.root, &lock) {
        Ok(m) => m,
        Err(e) => return crate::project::fs_exit(out, &e),
    };
    let count = outputs(&lock);
    let list: Vec<Value> = mismatches
        .iter()
        .map(|m| json!({ "owner": m.owner.name(), "path": m.path.as_str(), "reason": m.reason }))
        .collect();
    out.result(json!({ "outputs": count, "mismatches": list }));
    if mismatches.is_empty() {
        out.line(format!("verify: {count} outputs match {LOCK}"));
        return Exit::Ok;
    }
    for m in &mismatches {
        out.diag(
            Diagnostic::error(
                "verify/mismatch",
                format!("`{}` ({}): {}", m.path, m.owner, m.reason),
            )
            .in_file(m.path.as_str()),
        );
    }
    out.fail(
        Exit::Check,
        "verify/failed",
        format!(
            "{} of {count} outputs do not match {LOCK}",
            mismatches.len()
        ),
    )
}

fn outputs(lock: &Lockfile) -> usize {
    lock.outputs.len() + lock.assets.iter().map(|a| a.outputs.len()).sum::<usize>()
}

fn uninstall(out: &mut Out<'_, '_>, dir: Option<&PathBuf>, purge: bool, force: bool) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let planned = match prepare_uninstall(out, &project) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if planned.old_text.is_some() {
        let exit = run_apply(
            out,
            &project,
            &planned,
            Options {
                force,
                write_new: false,
            },
            Caller::Uninstall,
        );
        if exit != Exit::Ok {
            return exit;
        }
        if let Err(e) = project.remove(out, LOCK) {
            return e;
        }
    }
    if purge && let Err(e) = project.remove(out, CONFIG) {
        return e;
    }
    out.line(format!(
        "uninstall: removed managed content{}",
        if purge { " and grimoire.toml" } else { "" }
    ));
    out.result(json!({ "purged": purge }));
    Exit::Ok
}

fn doctor(out: &mut Out<'_, '_>, dir: Option<&PathBuf>) -> Exit {
    let project = match Project::open(out, dir) {
        Ok(p) => p,
        Err(e) => return e,
    };
    let mut checks: Vec<Check> = Vec::new();
    let source = out.ctx.source;
    checks.push((
        "catalog".into(),
        true,
        format!(
            "{} assets and {} bundles in this grimoire",
            source.assets().len(),
            source.bundles().len()
        ),
    ));
    let config = project.config(out).ok();
    checks.push((
        CONFIG.into(),
        config.is_some(),
        if config.is_some() {
            "valid".into()
        } else {
            "missing or invalid (see above)".into()
        },
    ));
    let lock = project.lock(out).ok();
    let lock_ok = lock.is_some();
    checks.push((
        LOCK.into(),
        lock_ok,
        match &lock {
            Some((_, Some(_))) => "valid".into(),
            Some((_, None)) => "not created yet".into(),
            None => "invalid (see above)".into(),
        },
    ));
    if let Some((config, _)) = &config {
        config_checks(out, &project, config, &mut checks);
    }
    if let Some((lock, _)) = &lock {
        lock_checks(out, &project, lock, &mut checks);
    }
    let ok = checks.iter().all(|(_, ok, _)| *ok);
    for (name, good, detail) in &checks {
        out.line(format!(
            "  {} {name}: {detail}",
            if *good { "ok  " } else { "FAIL" }
        ));
    }
    out.result(json!({ "checks": checks.iter().map(|(n, ok, d)| json!({ "name": n, "ok": ok, "detail": d })).collect::<Vec<_>>() }));
    if ok {
        Exit::Ok
    } else {
        out.fail(Exit::Check, "doctor/problems", "doctor found problems")
    }
}

/// One `doctor` check: name, whether it passed, and what was found.
type Check = (String, bool, String);

fn config_checks(
    out: &mut Out<'_, '_>,
    project: &Project,
    config: &agent_grimoire_model::ProjectConfig,
    checks: &mut Vec<Check>,
) {
    for client in &config.clients {
        let found = match client {
            ClientId::ClaudeCode => exists(project, "CLAUDE.md") || exists(project, ".claude"),
            ClientId::AgentsMd => exists(project, "AGENTS.md") || exists(project, ".agents"),
            _ => false,
        };
        checks.push((
            format!("client {client}"),
            true,
            if found {
                "files present".into()
            } else {
                "no files yet".into()
            },
        ));
    }
    let Ok(planned) = prepare(out, project, config, &Update::None) else {
        checks.push((
            "plan".into(),
            false,
            "grimoire.toml does not resolve or cannot be planned (see above)".into(),
        ));
        return;
    };
    {
        let pending = planned
            .plan
            .files
            .iter()
            .flat_map(|f| &f.units)
            .filter(|u| u.action != Action::Unchanged)
            .count();
        checks.push((
            "plan".into(),
            !planned.plan.has_conflicts(),
            format!(
                "{pending} pending changes{}",
                if planned.plan.has_conflicts() {
                    ", with conflicts"
                } else {
                    ""
                }
            ),
        ));
    }
}

fn lock_checks(out: &mut Out<'_, '_>, project: &Project, lock: &Lockfile, checks: &mut Vec<Check>) {
    let source = out.ctx.source;
    match apply::verify(&project.root, lock) {
        Ok(m) => checks.push((
            "outputs".into(),
            m.is_empty(),
            format!(
                "{} of {} locked outputs differ from disk",
                m.len(),
                outputs(lock)
            ),
        )),
        Err(e) => checks.push(("outputs".into(), false, e.to_string())),
    }
    for asset in &lock.assets {
        let binary = source.asset(&asset.id).and_then(|a| {
            a.manifest
                .tool
                .as_ref()
                .map(|t| t.binary.to_string())
                .or_else(|| a.manifest.mcp_server.as_ref().map(|s| s.binary.to_string()))
        });
        if let Some(binary) = binary {
            let found = on_path(out.ctx.path.as_deref(), &binary);
            checks.push((
                format!("binary {binary}"),
                found,
                if found {
                    "on PATH".into()
                } else {
                    "not on PATH; install it from the Grimoire release".into()
                },
            ));
        }
    }
}

/// Whether `binary` is a file in one of `path`'s directories. Never runs it.
fn on_path(path: Option<&std::ffi::OsStr>, binary: &str) -> bool {
    let Some(path) = path else { return false };
    std::env::split_paths(path).any(|dir| {
        let names = if cfg!(windows) {
            vec![format!("{binary}.exe"), binary.to_owned()]
        } else {
            vec![binary.to_owned()]
        };
        names.iter().any(|n| dir.join(n).is_file())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn changelog_heads() {
        assert_eq!(
            changelog_head("# Changelog\n\n## 1.1.0\n\n- b\n\n## 1.0.0\n- a\n").as_deref(),
            Some("## 1.1.0\n\n- b")
        );
        assert_eq!(
            changelog_head("## 1.0.0\n- a\n").as_deref(),
            Some("## 1.0.0\n- a")
        );
        assert_eq!(changelog_head("nothing"), None);
    }
}
