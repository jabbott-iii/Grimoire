//! Client adapters and the cross-client planner (`architecture.md` §8).
//!
//! An adapter is pure: it turns one resolved asset's declared outputs for one
//! client into [`Desired`] outputs at that client's literal paths. [`render`]
//! runs the adapters for every selected client and then plans across clients:
//! content that several clients can read from one place is written once and
//! attributed to all of them.
//!
//! v1 so far has two adapters: [`claude_code`] and [`agents_md`].

pub mod agents_md;
pub mod claude_code;
mod text;

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::plan::{Content, Desired, Owner};
use agent_grimoire_model::{
    Anchor, AssetId, ClientId, Diagnostic, OutputSpec, RelPath, sort_diagnostics,
};
use agent_grimoire_resolve::{Resolution, Resolved};

/// The clients this build can render for.
pub const AVAILABLE: [ClientId; 2] = [ClientId::ClaudeCode, ClientId::AgentsMd];

/// What adapters produce for a whole project.
#[derive(Debug, Default)]
pub struct Rendered {
    /// Every output, deduplicated across clients.
    pub desired: Vec<Desired>,
    /// Warnings, such as content another client also reads.
    pub warnings: Vec<Diagnostic>,
}

/// One output of one asset for one client, before cross-client planning.
struct Output {
    owner: AssetId,
    client: ClientId,
    anchor: Anchor,
    src: Option<RelPath>,
    path: RelPath,
    content: Content,
}

/// Errors and warnings collected while rendering.
#[derive(Default)]
pub(crate) struct Report {
    pub(crate) errors: Vec<Diagnostic>,
    pub(crate) warnings: Vec<Diagnostic>,
}

/// Renders every resolved asset for its clients and plans across clients.
///
/// # Errors
///
/// Returns every diagnostic when an asset cannot be rendered, for example an
/// environment override naming a variable the server does not read, or two
/// outputs that would write different content to the same place.
pub fn render(resolution: &Resolution<'_>) -> Result<Rendered, Vec<Diagnostic>> {
    let mut report = Report::default();
    let mut outputs = Vec::new();
    for resolved in resolution.assets.values() {
        for client in &resolved.clients {
            let Some(specs) = resolved.asset.manifest.clients.get(client) else {
                continue;
            };
            for spec in &specs.outputs {
                outputs.extend(render_one(resolved, *client, spec, &mut report));
            }
        }
    }
    let (outputs, import) = share_rules(outputs);
    let mut desired = merge(outputs, &mut report);
    if import {
        desired.push(claude_code::agents_md_import());
    }
    let claude_selected = resolution
        .assets
        .values()
        .any(|r| r.clients.contains(&ClientId::ClaudeCode));
    cross_client_warnings(&desired, import, claude_selected, &mut report);
    if !report.errors.is_empty() {
        let mut all = report.errors;
        all.extend(report.warnings);
        sort_diagnostics(&mut all);
        return Err(all);
    }
    sort_diagnostics(&mut report.warnings);
    desired.sort_by(|a, b| (&a.path, &a.owner).cmp(&(&b.path, &b.owner)));
    Ok(Rendered {
        desired,
        warnings: report.warnings,
    })
}

fn render_one(
    resolved: &Resolved<'_>,
    client: ClientId,
    spec: &OutputSpec,
    report: &mut Report,
) -> Vec<Output> {
    let rendered = match client {
        ClientId::ClaudeCode => claude_code::render(resolved, spec, report),
        ClientId::AgentsMd => agents_md::render(resolved, spec, report),
        // Resolution refuses clients without an adapter (`AVAILABLE`).
        _ => Vec::new(),
    };
    rendered
        .into_iter()
        .map(|(path, content)| Output {
            owner: resolved.id().clone(),
            client,
            anchor: spec.to.anchor(),
            src: spec.src.clone(),
            path,
            content,
        })
        .collect()
}

/// Claude Code reads `AGENTS.md` once through an `@AGENTS.md` import in
/// `CLAUDE.md` (`CC-MEM`). When a rule is installed for both `claude-code` and
/// `agents-md` from the same source, its `AGENTS.md` block serves both, and its
/// `.claude/rules/` file is dropped: Claude Code would otherwise load the rule
/// twice. Returns the outputs and whether the import is needed.
fn share_rules(outputs: Vec<Output>) -> (Vec<Output>, bool) {
    let shared_src: BTreeMap<&AssetId, Option<&RelPath>> = outputs
        .iter()
        .filter(|o| o.client == ClientId::AgentsMd && o.anchor == Anchor::Instructions)
        .map(|o| (&o.owner, o.src.as_ref()))
        .collect();
    let drop: BTreeSet<AssetId> = outputs
        .iter()
        .filter(|o| o.client == ClientId::ClaudeCode && o.anchor == Anchor::Rules)
        .filter(|o| {
            shared_src
                .get(&o.owner)
                .is_some_and(|src| *src == o.src.as_ref())
        })
        .map(|o| o.owner.clone())
        .collect();
    let mut kept = Vec::new();
    let mut import = false;
    for mut o in outputs {
        if !drop.contains(&o.owner) {
            kept.push(o);
            continue;
        }
        match (o.client, o.anchor) {
            (ClientId::ClaudeCode, Anchor::Rules) => {}
            (ClientId::AgentsMd, Anchor::Instructions) => {
                import = true;
                kept.push(Output {
                    client: ClientId::ClaudeCode,
                    content: o.content.clone(),
                    path: o.path.clone(),
                    src: o.src.clone(),
                    owner: o.owner.clone(),
                    anchor: o.anchor,
                });
                o.client = ClientId::AgentsMd;
                kept.push(o);
            }
            _ => kept.push(o),
        }
    }
    (kept, import)
}

/// Joins outputs that land on the same unit, unioning their clients.
fn merge(outputs: Vec<Output>, report: &mut Report) -> Vec<Desired> {
    let mut merged: BTreeMap<(RelPath, Owner, Option<String>), Desired> = BTreeMap::new();
    for o in outputs {
        let key = match &o.content {
            Content::Entry { key, .. } => Some(key.as_str().to_owned()),
            _ => None,
        };
        let id = (o.path.clone(), Owner::Asset(o.owner.clone()), key);
        match merged.get_mut(&id) {
            Some(existing) if existing.content == o.content => {
                existing.clients.insert(o.client);
            }
            Some(_) => report.errors.push(Diagnostic::error(
                "adapter/output-clash",
                format!(
                    "`{}` would write different content to `{}` for different clients",
                    o.owner, o.path
                ),
            )),
            None => {
                merged.insert(
                    id,
                    Desired {
                        owner: Owner::Asset(o.owner),
                        clients: BTreeSet::from([o.client]),
                        path: o.path,
                        content: o.content,
                    },
                );
            }
        }
    }
    merged.into_values().collect()
}

/// Warns where installing for one client also grants something to another,
/// possibly unselected, client (`architecture.md` §8.2, `specs.md`).
fn cross_client_warnings(
    desired: &[Desired],
    import: bool,
    claude_selected: bool,
    report: &mut Report,
) {
    let mut agents = BTreeSet::new();
    let mut hooks = BTreeSet::new();
    let mut mcp = BTreeSet::new();
    for d in desired {
        let Owner::Asset(id) = &d.owner else { continue };
        let path = d.path.as_str();
        // Claude Code reads all of AGENTS.md: through the import, or natively when
        // the project has no CLAUDE.md (`CC-MEM`).
        if path == agents_md::AGENTS_MD && !d.clients.contains(&ClientId::ClaudeCode) {
            let how = if import {
                Some("through the `@AGENTS.md` import grimoire adds to CLAUDE.md")
            } else if claude_selected {
                Some("whenever the project has no CLAUDE.md")
            } else {
                None
            };
            if let Some(how) = how {
                report.warnings.push(Diagnostic::warning(
                    "plan/cross-client-rule",
                    format!("Claude Code also reads `{id}` from AGENTS.md {how}, although it is installed only for agents-md"),
                ));
            }
        }
        if path.starts_with(".claude/agents/") {
            agents.insert(id);
        } else if path == claude_code::SETTINGS {
            hooks.insert(id);
        } else if path == claude_code::MCP {
            mcp.insert(id);
        }
    }
    for id in agents {
        report.warnings.push(Diagnostic::warning(
            "plan/cross-client-agent",
            format!(
                "Cursor also loads `{id}` from .claude/agents/, and Cursor has no per-tool allow-list (only `readonly`), so it does not enforce this agent's tools; GitHub Copilot in VS Code also reads .claude/agents/, and how it maps Claude Code tool names is not verified"
            ),
        ));
    }
    for id in hooks {
        report.warnings.push(Diagnostic::warning(
            "plan/cross-client-hooks",
            format!("Cursor and GitHub Copilot also run hooks from .claude/settings.json, so `{id}`'s hooks run there too"),
        ));
    }
    for id in mcp {
        report.warnings.push(Diagnostic::warning(
            "plan/cross-client-mcp",
            format!("GitHub Copilot (VS Code and CLI) also starts servers listed in .mcp.json, so `{id}` is available there too"),
        ));
    }
}

/// The text of a source file, or an error diagnostic.
pub(crate) fn source_text(
    resolved: &Resolved<'_>,
    src: &RelPath,
    report: &mut Report,
) -> Option<String> {
    let text = resolved
        .asset
        .file(src)
        .and_then(|b| std::str::from_utf8(b).ok());
    if text.is_none() {
        report.errors.push(Diagnostic::error(
            "adapter/source",
            format!(
                "`{}` has no UTF-8 file `{src}` in this grimoire's catalog",
                resolved.id()
            ),
        ));
    }
    text.map(str::to_owned)
}

/// Joins a literal client directory and a relative path.
pub(crate) fn under(dir: &str, path: &RelPath) -> Option<RelPath> {
    RelPath::new(format!("{dir}/{path}")).ok()
}

/// Whether `path` is somewhere an available adapter can write: anything a lock
/// names elsewhere was not written by Grimoire and must not be read or deleted.
#[must_use]
pub fn is_managed_location(path: &RelPath) -> bool {
    const DIRS: [&str; 4] = [
        ".claude/rules/",
        ".claude/agents/",
        ".claude/skills/",
        ".agents/skills/",
    ];
    const FILES: [&str; 4] = [
        agents_md::AGENTS_MD,
        claude_code::CLAUDE_MD,
        claude_code::MCP,
        claude_code::SETTINGS,
    ];
    let p = path.as_str();
    FILES.contains(&p) || DIRS.iter().any(|d| p.len() > d.len() && p.starts_with(d))
}

/// Where a declared output lands for `client`, for `grimoire show`.
#[must_use]
pub fn destination(
    client: ClientId,
    to: &agent_grimoire_model::Destination,
    id: &AssetId,
) -> String {
    let path = to.path().map(RelPath::as_str).unwrap_or_default();
    match (client, to.anchor()) {
        (ClientId::ClaudeCode, Anchor::Rules) => format!(".claude/rules/{path}"),
        (ClientId::ClaudeCode, Anchor::Skills) => format!(".claude/skills/{path}"),
        (ClientId::ClaudeCode, Anchor::Agents) => format!(".claude/agents/{path}"),
        (ClientId::ClaudeCode, Anchor::Mcp) => format!("{} (mcpServers.{id})", claude_code::MCP),
        (ClientId::ClaudeCode, Anchor::Hooks) => format!("{} (hooks)", claude_code::SETTINGS),
        (ClientId::AgentsMd, Anchor::Instructions) => {
            format!("{} (managed block)", agents_md::AGENTS_MD)
        }
        (ClientId::AgentsMd, Anchor::Skills) => format!(".agents/skills/{path}"),
        (client, _) if !AVAILABLE.contains(&client) => {
            "not available in this version of grimoire".to_owned()
        }
        (_, anchor) => format!("unsupported anchor `{anchor}`"),
    }
}
