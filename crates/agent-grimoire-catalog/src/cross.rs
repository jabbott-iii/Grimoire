//! Catalog-wide checks: identifiers, dependencies, deprecations, agents' MCP references and bundles.

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_model::{AssetId, Capability, Diagnostic, Kind, Maturity};

use crate::{CatalogAsset, CatalogBundle};

pub(crate) fn check(
    assets: &BTreeMap<AssetId, CatalogAsset>,
    bundles: &BTreeMap<AssetId, CatalogBundle>,
    out: &mut Vec<Diagnostic>,
) {
    for (id, bundle) in bundles {
        if let Some(asset) = assets.get(id) {
            out.push(
                Diagnostic::error(
                    "catalog/duplicate-id",
                    format!("`{id}` is used by both this bundle and {}", asset.dir),
                )
                .in_file(bundle.file.clone()),
            );
        }
    }
    for asset in assets.values() {
        installable(asset, assets, out);
        requires(asset, assets, out);
        conflicts(asset, assets, out);
        replacement(asset, assets, out);
        agent_mcp(asset, assets, out);
    }
    cycles(assets, out);
    for bundle in bundles.values() {
        bundle_members(bundle, assets, out);
    }
}

fn manifest_file(asset: &CatalogAsset) -> String {
    format!("{}/asset.toml", asset.dir)
}

fn requires(
    asset: &CatalogAsset,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    for (dep, req) in &asset.manifest.requires {
        let field = format!("requires.{dep}");
        match assets.get(dep) {
            None => out.push(
                Diagnostic::error(
                    "catalog/requires-missing",
                    format!("`{dep}` is not in the catalog"),
                )
                .at(field)
                .in_file(manifest_file(asset)),
            ),
            Some(target) if !req.matches(&target.manifest.version) => out.push(
                Diagnostic::error(
                    "catalog/requires-version",
                    format!(
                        "`{dep}` is at {} in the catalog, which does not satisfy `{req}`",
                        target.manifest.version
                    ),
                )
                .at(field)
                .in_file(manifest_file(asset)),
            ),
            Some(target) if target.manifest.maturity == Maturity::Deprecated => out.push(
                Diagnostic::warning(
                    "catalog/requires-deprecated",
                    format!("`{dep}` is deprecated"),
                )
                .at(field)
                .in_file(manifest_file(asset)),
            ),
            Some(_) => {}
        }
    }
}

fn conflicts(
    asset: &CatalogAsset,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    for other in asset.manifest.conflicts.keys() {
        if !assets.contains_key(other) {
            out.push(
                Diagnostic::warning(
                    "catalog/conflicts-unknown",
                    format!("`{other}` is not in the catalog"),
                )
                .at(format!("conflicts.{other}"))
                .in_file(manifest_file(asset)),
            );
        }
    }
}

fn replacement(
    asset: &CatalogAsset,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    let Some(deprecated) = &asset.manifest.deprecated else {
        return;
    };
    let replacement = &deprecated.replacement;
    let field = "deprecated.replacement";
    match assets.get(replacement) {
        None => out.push(
            Diagnostic::error(
                "catalog/replacement-missing",
                format!("`{replacement}` is not in the catalog"),
            )
            .at(field)
            .in_file(manifest_file(asset)),
        ),
        Some(target) if target.manifest.kind != asset.manifest.kind => out.push(
            Diagnostic::error(
                "catalog/replacement-kind",
                format!(
                    "`{replacement}` is a {}, not a {}",
                    target.manifest.kind, asset.manifest.kind
                ),
            )
            .at(field)
            .in_file(manifest_file(asset)),
        ),
        Some(target) if target.manifest.maturity == Maturity::Deprecated => out.push(
            Diagnostic::warning(
                "catalog/replacement-deprecated",
                format!("`{replacement}` is itself deprecated"),
            )
            .at(field)
            .in_file(manifest_file(asset)),
        ),
        Some(_) => {}
    }
}

fn agent_mcp(
    asset: &CatalogAsset,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    let Some(agent) = &asset.manifest.agent else {
        return;
    };
    for cap in &agent.tools {
        let Capability::Mcp { server, tool } = cap else {
            continue;
        };
        let target = assets.get(server);
        let spec = target.and_then(|t| t.manifest.mcp_server.as_ref());
        match (target, spec) {
            (Some(t), Some(spec)) if t.manifest.kind == Kind::McpServer => {
                if let Some(tool) = tool
                    && !spec.tools.iter().any(|declared| &declared.name == tool)
                {
                    out.push(
                        Diagnostic::error(
                            "catalog/agent-mcp-tool",
                            format!("`{server}` declares no tool named `{tool}`"),
                        )
                        .at("agent.tools")
                        .in_file(manifest_file(asset)),
                    );
                }
            }
            _ => out.push(
                Diagnostic::error(
                    "catalog/agent-mcp",
                    format!("`{server}` is not an mcp-server asset in the catalog"),
                )
                .at("agent.tools")
                .in_file(manifest_file(asset)),
            ),
        }
    }
}

/// Reports each dependency cycle once, from its smallest member.
fn cycles(assets: &BTreeMap<AssetId, CatalogAsset>, out: &mut Vec<Diagnostic>) {
    #[derive(Clone, Copy, PartialEq)]
    enum State {
        Visiting,
        Done,
    }
    fn visit<'a>(
        id: &'a AssetId,
        assets: &'a BTreeMap<AssetId, CatalogAsset>,
        state: &mut BTreeMap<&'a AssetId, State>,
        stack: &mut Vec<&'a AssetId>,
        found: &mut BTreeSet<Vec<AssetId>>,
    ) {
        match state.get(id) {
            Some(State::Done) => return,
            Some(State::Visiting) => {
                if let Some(start) = stack.iter().position(|s| *s == id) {
                    let mut cycle: Vec<AssetId> =
                        stack[start..].iter().map(|s| (*s).clone()).collect();
                    // Rotate so the smallest id leads; the same cycle is then reported once.
                    if let Some(min) = cycle
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, s)| *s)
                        .map(|(i, _)| i)
                    {
                        cycle.rotate_left(min);
                    }
                    found.insert(cycle);
                }
                return;
            }
            None => {}
        }
        state.insert(id, State::Visiting);
        stack.push(id);
        if let Some(asset) = assets.get(id) {
            for dep in asset.manifest.requires.keys() {
                if assets.contains_key(dep) {
                    visit(dep, assets, state, stack, found);
                }
            }
        }
        stack.pop();
        state.insert(id, State::Done);
    }

    let mut state = BTreeMap::new();
    let mut found = BTreeSet::new();
    for id in assets.keys() {
        visit(id, assets, &mut state, &mut Vec::new(), &mut found);
    }
    for cycle in found {
        let first = &cycle[0];
        let path: Vec<&str> = cycle
            .iter()
            .chain(std::iter::once(first))
            .map(AssetId::as_str)
            .collect();
        if let Some(asset) = assets.get(first) {
            out.push(
                Diagnostic::error(
                    "catalog/requires-cycle",
                    format!("dependency cycle: {}", path.join(" -> ")),
                )
                .at("requires")
                .in_file(manifest_file(asset)),
            );
        }
    }
}

fn bundle_members(
    bundle: &CatalogBundle,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    for (id, req) in &bundle.bundle.assets {
        let field = format!("assets.{id}");
        match assets.get(id) {
            None => out.push(
                Diagnostic::error(
                    "bundle/member-missing",
                    format!("`{id}` is not in the catalog"),
                )
                .at(field)
                .in_file(bundle.file.clone()),
            ),
            Some(member) if !req.matches(&member.manifest.version) => out.push(
                Diagnostic::error(
                    "bundle/member-version",
                    format!(
                        "`{id}` is at {}, which does not satisfy `{req}`",
                        member.manifest.version
                    ),
                )
                .at(field)
                .in_file(bundle.file.clone()),
            ),
            Some(member) if member.manifest.maturity == Maturity::Deprecated => out.push(
                Diagnostic::warning("bundle/member-deprecated", format!("`{id}` is deprecated"))
                    .at(field)
                    .in_file(bundle.file.clone()),
            ),
            Some(_) => {}
        }
    }
    // Everything the bundle installs, dependencies included, must be installable together.
    let mut set = BTreeSet::new();
    for member in bundle.bundle.assets.keys() {
        closure(member, assets, &mut set);
    }
    for (a, b, reason) in conflicts_within(&set, assets) {
        out.push(
            Diagnostic::error(
                "bundle/conflict",
                format!("`{a}` conflicts with `{b}`: {reason}"),
            )
            .at("assets")
            .in_file(bundle.file.clone()),
        );
    }
}

/// Adds `id` and everything it transitively requires (that exists) to `set`.
fn closure<'a>(
    id: &'a AssetId,
    assets: &'a BTreeMap<AssetId, CatalogAsset>,
    set: &mut BTreeSet<&'a AssetId>,
) {
    if !set.insert(id) {
        return;
    }
    if let Some(asset) = assets.get(id) {
        for dep in asset.manifest.requires.keys() {
            if let Some((dep, _)) = assets.get_key_value(dep) {
                closure(dep, assets, set);
            }
        }
    }
}

/// Pairs in `set` where one declares a conflict that matches the other's version.
fn conflicts_within<'a>(
    set: &BTreeSet<&'a AssetId>,
    assets: &'a BTreeMap<AssetId, CatalogAsset>,
) -> Vec<(&'a AssetId, &'a AssetId, String)> {
    let mut found = Vec::new();
    for a in set {
        let Some(asset) = assets.get(*a) else {
            continue;
        };
        for (b, spec) in &asset.manifest.conflicts {
            if let Some((b, other)) = assets.get_key_value(b)
                && set.contains(b)
                && spec.version.matches(&other.manifest.version)
            {
                found.push((*a, b, spec.reason.to_string()));
            }
        }
    }
    found
}

/// An asset whose own dependency closure contains a conflict can never be installed.
fn installable(
    asset: &CatalogAsset,
    assets: &BTreeMap<AssetId, CatalogAsset>,
    out: &mut Vec<Diagnostic>,
) {
    let Some((id, _)) = assets.get_key_value(&asset.manifest.id) else {
        return;
    };
    let mut set = BTreeSet::new();
    closure(id, assets, &mut set);
    for (a, b, reason) in conflicts_within(&set, assets) {
        // A direct requires/conflicts overlap is already reported by manifest validation.
        if a == id && asset.manifest.requires.contains_key(b) {
            continue;
        }
        out.push(
            Diagnostic::error(
                "catalog/uninstallable",
                format!(
                    "installing `{id}` brings in both `{a}` and `{b}`, which conflict: {reason}"
                ),
            )
            .at("requires")
            .in_file(manifest_file(asset)),
        );
    }
}
