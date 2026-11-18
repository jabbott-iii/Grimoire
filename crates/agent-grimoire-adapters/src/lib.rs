//! Client adapters and the cross-client planner (`architecture.md` §8).
//!
//! An adapter is pure: it turns one resolved asset's declared outputs for one
//! client into outputs at that client's literal paths. [`render`] runs the
//! adapters for every selected client and then places shared content
//! (`placement`): where several clients read one location, the content is
//! written there once and attributed to all of them (ADR-0008).
//!
//! The v1 adapters are [`claude_code`], [`agents_md`], [`codex`], [`cursor`],
//! [`gemini_cli`] and [`copilot`].

pub mod agents_md;
pub mod claude_code;
pub mod codex;
pub mod copilot;
pub mod cursor;
pub mod gemini_cli;
mod placement;
pub mod plugin;
mod text;

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::plan::{Content, Desired, Owner};
use agent_grimoire_model::{
    Anchor, AssetId, Capability, ClientId, Diagnostic, EnvVarName, OutputSpec, RelPath, Unit,
    sort_diagnostics,
};
use agent_grimoire_resolve::{Resolution, Resolved};

/// The clients this build can render for.
///
/// While this is every client, `init/client-unavailable` and
/// `resolve/client-unavailable` are never raised; the resolve crate's tests raise
/// the second with a narrower set. Narrowing this again, for a client the model
/// knows before its adapter exists, means updating those two codes' rows in
/// `docs/diagnostics.md`, `init`'s exit codes in `docs/cli.md` and
/// `docs/architecture.md` §9.1, deciding whether `init`'s detection should look
/// only at these clients, and testing both codes end to end in
/// `crates/agent-grimoire/tests/cli.rs`.
pub const AVAILABLE: [ClientId; 6] = ClientId::ALL;

/// What adapters produce for a whole project.
#[derive(Debug, Default)]
pub struct Rendered {
    /// Every output, deduplicated across clients.
    pub desired: Vec<Desired>,
    /// Warnings, such as content another client also reads.
    pub warnings: Vec<Diagnostic>,
}

/// One output of one asset for one client, before cross-client planning.
#[derive(Debug, Clone)]
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
    /// Errors about one client's output of one asset that count only if
    /// placement keeps that output (see `cursor::comma_globs`).
    pub(crate) deferred: Vec<(AssetId, ClientId, Diagnostic)>,
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
    let placed = placement::place(outputs);
    for (owner, client, error) in std::mem::take(&mut report.deferred) {
        if !placed.shared.contains(&(owner, client)) {
            report.errors.push(error);
        }
    }
    let mut desired = merge(placed.outputs, &mut report);
    desired.extend(placed.imports);
    placement::warnings(resolution, &desired, &mut report);
    // Clients that share a renderer report the same problem once each.
    if !report.errors.is_empty() {
        let mut all = report.errors;
        all.extend(report.warnings);
        sort_diagnostics(&mut all);
        all.dedup();
        return Err(all);
    }
    sort_diagnostics(&mut report.warnings);
    report.warnings.dedup();
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
        ClientId::Codex => codex::render(resolved, spec, report),
        ClientId::Cursor => cursor::render(resolved, spec, report),
        ClientId::GeminiCli => gemini_cli::render(resolved, spec, report),
        ClientId::Copilot => copilot::render(resolved, spec, report),
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

/// An error for an output a client cannot express.
pub(crate) fn unsupported(report: &mut Report, message: String) {
    report
        .errors
        .push(Diagnostic::error("adapter/unsupported", message));
}

/// The error for an anchor and strategy pair a client has no destination for.
/// Manifest validation normally refuses these first (`manifest/anchor-client`).
pub(crate) fn no_destination(
    report: &mut Report,
    client: ClientId,
    resolved: &Resolved<'_>,
    anchor: Anchor,
) {
    unsupported(
        report,
        format!(
            "{client} cannot write `{}` to `{anchor}` this way",
            resolved.id()
        ),
    );
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

/// Copies every file at or below `src` to the same layout under `dir`.
pub(crate) fn copy_tree(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    dir: &str,
) -> Vec<(RelPath, Content)> {
    let (Some(src), Some(to)) = (spec.src.as_ref(), spec.to.path()) else {
        return Vec::new();
    };
    resolved
        .asset
        .files_under(src)
        .filter_map(|(path, bytes)| {
            let rest = &path.as_str()[src.as_str().len()..];
            let dest = RelPath::new(format!("{dir}/{to}{rest}")).ok()?;
            Some((dest, Content::File(bytes.to_vec())))
        })
        .collect()
}

/// The single-file destination of a rule or agent output, with its `.md`
/// extension replaced: `rules:x.md` becomes `<dir>/x<extension>`. Manifest
/// validation guarantees the name is `<id>.md`.
pub(crate) fn renamed(dir: &str, spec: &OutputSpec, extension: &str) -> Option<RelPath> {
    let path = spec.to.path()?;
    let stem = path.as_str().strip_suffix(".md")?;
    RelPath::new(format!("{dir}/{stem}{extension}")).ok()
}

/// The globs a rule is scoped to; empty for an always-on rule.
pub(crate) fn rule_paths<'a>(resolved: &'a Resolved<'_>) -> Vec<&'a str> {
    resolved
        .asset
        .manifest
        .rule
        .as_ref()
        .map(|r| {
            r.paths
                .iter()
                .map(agent_grimoire_model::Glob::as_str)
                .collect()
        })
        .unwrap_or_default()
}

/// The capabilities an agent declares.
pub(crate) fn agent_tools<'a>(resolved: &'a Resolved<'_>) -> &'a [Capability] {
    resolved
        .asset
        .manifest
        .agent
        .as_ref()
        .map(|a| a.tools.as_slice())
        .unwrap_or_default()
}

/// The `env` overrides in `grimoire.toml` for an MCP server, after refusing any
/// that name a variable the client configuration does not forward.
pub(crate) fn env_remap<'a>(
    resolved: &'a Resolved<'_>,
    report: &mut Report,
) -> Option<&'a BTreeMap<EnvVarName, EnvVarName>> {
    let manifest = &resolved.asset.manifest;
    let id = &manifest.id;
    let server = manifest.mcp_server.as_ref()?;
    let remap = resolved.options.as_ref().map(|o| &o.env)?;
    for name in remap.keys() {
        if !server.env.iter().any(|e| e.name == *name) {
            // A variable the server reads but the client sets itself, such as
            // CLAUDE_PROJECT_DIR, is not forwarded, so it cannot be remapped.
            let message = if manifest.permissions.env.contains(name) {
                format!(
                    "`{id}` reads `{name}`, but the client sets it, so it cannot be remapped; the variables you can remap are {}",
                    list(server.env.iter().map(|e| e.name.as_str()))
                )
            } else {
                format!(
                    "`{id}` does not read `{name}`; the variables you can remap are {}",
                    list(server.env.iter().map(|e| e.name.as_str()))
                )
            };
            report.errors.push(
                Diagnostic::error("adapter/env-unknown", message)
                    .at(format!("assets.{id}.env"))
                    .in_file("grimoire.toml"),
            );
        }
    }
    Some(remap)
}

/// A list of names for messages: "`A`, `B`", or "none".
pub(crate) fn list<'a>(names: impl Iterator<Item = &'a str>) -> String {
    let names: Vec<String> = names.map(|n| format!("`{n}`")).collect();
    if names.is_empty() {
        "none".to_owned()
    } else {
        names.join(", ")
    }
}

/// Whether `path` is somewhere an available adapter can write: anything a lock
/// names elsewhere was not written by Grimoire and must not be read or deleted.
#[must_use]
pub fn is_managed_location(path: &RelPath) -> bool {
    managed_unit(path).is_some()
}

/// What an adapter owns at `path`: a whole file in a managed folder, blocks in a
/// shared instruction file, or entries in a configuration file. A lock that
/// records another unit there was edited by hand: a `file` record on
/// `.codex/config.toml` would let Grimoire delete the user's whole file.
#[must_use]
pub fn managed_unit(path: &RelPath) -> Option<Unit> {
    const DIRS: [&str; 9] = [
        ".claude/rules/",
        ".claude/agents/",
        ".claude/skills/",
        ".agents/skills/",
        cursor::RULES_DIR,
        cursor::AGENTS_DIR,
        gemini_cli::AGENTS_DIR,
        copilot::INSTRUCTIONS_DIR,
        copilot::AGENTS_DIR,
    ];
    const ENTRY_FILES: [&str; 5] = [
        claude_code::MCP,
        claude_code::SETTINGS,
        codex::CONFIG,
        cursor::MCP,
        gemini_cli::SETTINGS,
    ];
    let p = path.as_str();
    if is_project_location(path) || p == agents_md::AGENTS_MD {
        Some(Unit::Block)
    } else if ENTRY_FILES.contains(&p) {
        Some(Unit::Entry)
    } else if DIRS.iter().any(|d| p.len() > d.len() && p.starts_with(d)) {
        Some(Unit::File)
    } else {
        None
    }
}

/// Whether Grimoire may own a block for itself at `path`: the files that hold
/// the `@AGENTS.md` imports.
#[must_use]
pub fn is_project_location(path: &RelPath) -> bool {
    [claude_code::CLAUDE_MD, gemini_cli::GEMINI_MD].contains(&path.as_str())
}

/// Where a declared output lands for `client`, for `grimoire show`. Shared
/// placement (ADR-0008) can move a rule or skill to a location another client
/// also reads; `grimoire plan` shows the final paths.
#[must_use]
pub fn destination(
    client: ClientId,
    to: &agent_grimoire_model::Destination,
    id: &AssetId,
) -> String {
    let path = to.path().map(RelPath::as_str).unwrap_or_default();
    let stem = path.strip_suffix(".md").unwrap_or(path);
    match (client, to.anchor()) {
        (ClientId::ClaudeCode, Anchor::Rules) => format!(".claude/rules/{path}"),
        (ClientId::ClaudeCode, Anchor::Skills) => format!(".claude/skills/{path}"),
        (ClientId::ClaudeCode, Anchor::Agents) => format!(".claude/agents/{path}"),
        (ClientId::ClaudeCode | ClientId::Copilot, Anchor::Mcp) => {
            format!("{} (mcpServers.{id})", claude_code::MCP)
        }
        (ClientId::ClaudeCode, Anchor::Hooks) => format!("{} (hooks)", claude_code::SETTINGS),
        (ClientId::AgentsMd | ClientId::Codex, Anchor::Instructions) => {
            format!("{} (managed block)", agents_md::AGENTS_MD)
        }
        (ClientId::Cursor, Anchor::Rules) => format!(
            "{}{stem}.mdc, or {} when that is written",
            cursor::RULES_DIR,
            agents_md::AGENTS_MD
        ),
        (ClientId::Copilot, Anchor::Rules) => format!(
            "{}{stem}.instructions.md, or {} when that is written",
            copilot::INSTRUCTIONS_DIR,
            agents_md::AGENTS_MD
        ),
        (ClientId::GeminiCli, Anchor::Instructions) => format!(
            "{} (managed block), or {} through an import when that is written",
            gemini_cli::GEMINI_MD,
            agents_md::AGENTS_MD
        ),
        (ClientId::AgentsMd | ClientId::Codex | ClientId::GeminiCli, Anchor::Skills) => {
            format!(".agents/skills/{path}")
        }
        (ClientId::Cursor | ClientId::Copilot, Anchor::Skills) => {
            format!(".agents/skills/{path}, or .claude/skills/{path} with claude-code")
        }
        (ClientId::Cursor, Anchor::Agents) => format!("{}{path}", cursor::AGENTS_DIR),
        (ClientId::GeminiCli, Anchor::Agents) => format!("{}{path}", gemini_cli::AGENTS_DIR),
        (ClientId::Copilot, Anchor::Agents) => {
            format!("{}{stem}.agent.md", copilot::AGENTS_DIR)
        }
        (ClientId::Codex, Anchor::Mcp) => format!("{} (mcp_servers.{id})", codex::CONFIG),
        (ClientId::Cursor, Anchor::Mcp) => format!("{} (mcpServers.{id})", cursor::MCP),
        (ClientId::GeminiCli, Anchor::Mcp) => {
            format!("{} (mcpServers.{id})", gemini_cli::SETTINGS)
        }
        (_, anchor) => format!("unsupported anchor `{anchor}`"),
    }
}
