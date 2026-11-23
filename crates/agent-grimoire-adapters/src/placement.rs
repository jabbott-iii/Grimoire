//! Cross-client placement.
//!
//! The design is in
//! [`docs/architecture.md` §8.2, Client read matrix and cross-client planning](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#82-client-read-matrix-and-cross-client-planning),
//! and [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//! the record of what the Codex, Cursor, Gemini CLI and Copilot adapters write,
//! chose both placements below.
//!
//! Each adapter renders to its client's own locations. This pass then moves
//! content that several selected clients can read from one place, so it is
//! written once and attributed to all of them:
//!
//! - **Rules.** When `agents-md` or `codex` puts a rule in `AGENTS.md`, that
//!   block also serves `claude-code` (through Grimoire's `@AGENTS.md` block in
//!   `CLAUDE.md`), `gemini-cli` (through the same block in `GEMINI.md`), and
//!   `cursor` and `copilot`, which read `AGENTS.md`. Their own rule files
//!   are dropped.
//! - **Skills.** Cursor and Copilot read both `.claude/skills/` and
//!   `.agents/skills/`. Their copy goes to `.agents/skills/` when another client
//!   needs that location, else to `.claude/skills/` when `claude-code` writes it,
//!   else to `.agents/skills/`.
//!
//! [`warnings`] then reports what placement can't avoid: duplicates a selected
//! client still reads, and content an unselected client reads too.

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::plan::{Content, Desired, Owner};
use agent_grimoire_model::{Anchor, AssetId, ClientId, Diagnostic, Kind, RelPath};
use agent_grimoire_resolve::Resolution;

use crate::{Output, Report, agents_md, claude_code, copilot, cursor, gemini_cli};

/// Outputs after placement, plus the project-owned import blocks they need.
pub(crate) struct Placed {
    pub(crate) outputs: Vec<Output>,
    pub(crate) imports: Vec<Desired>,
    /// The asset and client pairs whose own rule output was replaced by the
    /// `AGENTS.md` block.
    pub(crate) shared: BTreeSet<(AssetId, ClientId)>,
}

const AGENTS_SKILLS: &str = ".agents/skills/";
const CLAUDE_SKILLS: &str = ".claude/skills/";

pub(crate) fn place(outputs: Vec<Output>) -> Placed {
    let mut by_owner: BTreeMap<AssetId, Vec<Output>> = BTreeMap::new();
    for o in outputs {
        by_owner.entry(o.owner.clone()).or_default().push(o);
    }
    let mut placed = Vec::new();
    let mut shared = BTreeSet::new();
    let mut claude_import = false;
    let mut gemini_import = false;
    for (owner, outputs) in by_owner {
        let (outputs, clients) = share_rule(outputs);
        claude_import |= clients.contains(&ClientId::ClaudeCode);
        gemini_import |= clients.contains(&ClientId::GeminiCli);
        shared.extend(clients.into_iter().map(|c| (owner.clone(), c)));
        placed.extend(share_skills(outputs));
    }
    let mut imports = Vec::new();
    if claude_import {
        imports.push(claude_code::agents_md_import());
    }
    if gemini_import {
        imports.push(gemini_import_block());
    }
    Placed {
        outputs: placed,
        imports,
        shared,
    }
}

/// Grimoire's `GEMINI.md` block that makes Gemini CLI read `AGENTS.md`.
fn gemini_import_block() -> Desired {
    Desired {
        owner: Owner::Project,
        clients: BTreeSet::from([ClientId::GeminiCli]),
        path: RelPath::new(gemini_cli::GEMINI_MD).unwrap_or_else(|_| unreachable!("a valid path")),
        content: Content::Block(gemini_cli::AGENTS_MD_IMPORT.to_owned()),
    }
}

fn is_agents_md(o: &Output) -> bool {
    o.anchor == Anchor::Instructions && o.path.as_str() == agents_md::AGENTS_MD
}

/// Moves one asset's rule outputs into its `AGENTS.md` block when there is one.
/// Returns the outputs and the clients whose own output was replaced.
fn share_rule(outputs: Vec<Output>) -> (Vec<Output>, BTreeSet<ClientId>) {
    let Some(hub) = outputs
        .iter()
        .find(|o| matches!(o.client, ClientId::AgentsMd | ClientId::Codex) && is_agents_md(o))
        .cloned()
    else {
        return (outputs, BTreeSet::new());
    };
    let mut kept = Vec::new();
    let mut clients = BTreeSet::new();
    for o in outputs {
        let shared = o.src == hub.src
            && match (o.client, o.anchor) {
                (ClientId::ClaudeCode | ClientId::Cursor | ClientId::Copilot, Anchor::Rules) => {
                    true
                }
                (ClientId::GeminiCli, Anchor::Instructions) => !is_agents_md(&o),
                _ => false,
            };
        if !shared {
            kept.push(o);
            continue;
        }
        clients.insert(o.client);
        kept.push(Output {
            client: o.client,
            ..hub.clone()
        });
    }
    (kept, clients)
}

/// Places Cursor's and Copilot's copy of one asset's skill (see the module docs).
fn share_skills(outputs: Vec<Output>) -> Vec<Output> {
    let skills = |client: ClientId| {
        outputs
            .iter()
            .any(|o| o.anchor == Anchor::Skills && o.client == client)
    };
    let agents_needed = [ClientId::AgentsMd, ClientId::Codex, ClientId::GeminiCli]
        .into_iter()
        .any(skills);
    if agents_needed || !skills(ClientId::ClaudeCode) {
        return outputs;
    }
    outputs
        .into_iter()
        .map(|mut o| {
            if o.anchor == Anchor::Skills
                && matches!(o.client, ClientId::Cursor | ClientId::Copilot)
                && let Some(rest) = o.path.as_str().strip_prefix(AGENTS_SKILLS)
                && let Ok(path) = RelPath::new(format!("{CLAUDE_SKILLS}{rest}"))
            {
                o.path = path;
            }
            o
        })
        .collect()
}

/// A kind of location that other clients read too.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Place {
    AgentsMd,
    GeminiMd,
    ClaudeRule,
    CopilotRule,
    ClaudeSkill,
    AgentsSkill,
    ClaudeAgent,
    CursorAgent,
    CopilotAgent,
    SettingsHooks,
}

/// Where one asset's content was written.
#[derive(Default)]
struct Written {
    places: BTreeSet<Place>,
    /// The clients the `.mcp.json` entry is attributed to.
    mcp_json: Option<BTreeSet<ClientId>>,
}

impl Written {
    fn has(&self, place: Place) -> bool {
        self.places.contains(&place)
    }
}

fn place_of(path: &str) -> Option<Place> {
    let prefixes = [
        (".claude/rules/", Place::ClaudeRule),
        (copilot::INSTRUCTIONS_DIR, Place::CopilotRule),
        (CLAUDE_SKILLS, Place::ClaudeSkill),
        (AGENTS_SKILLS, Place::AgentsSkill),
        (".claude/agents/", Place::ClaudeAgent),
        (cursor::AGENTS_DIR, Place::CursorAgent),
        (copilot::AGENTS_DIR, Place::CopilotAgent),
    ];
    match path {
        _ if path == agents_md::AGENTS_MD => Some(Place::AgentsMd),
        _ if path == gemini_cli::GEMINI_MD => Some(Place::GeminiMd),
        _ if path == claude_code::SETTINGS => Some(Place::SettingsHooks),
        _ => prefixes
            .into_iter()
            .find(|(prefix, _)| path.starts_with(prefix))
            .map(|(_, place)| place),
    }
}

fn written(desired: &[Desired]) -> BTreeMap<&AssetId, Written> {
    let mut map: BTreeMap<&AssetId, Written> = BTreeMap::new();
    for d in desired {
        let Owner::Asset(id) = &d.owner else { continue };
        let w = map.entry(id).or_default();
        if d.path.as_str() == claude_code::MCP {
            w.mcp_json = Some(d.clients.clone());
        } else if let Some(place) = place_of(d.path.as_str()) {
            w.places.insert(place);
        }
    }
    map
}

/// Reports duplicates placement can't avoid, and content that another client,
/// possibly an unselected one, also reads
/// ([`docs/architecture.md` §8.2, Client read matrix and cross-client planning](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#82-client-read-matrix-and-cross-client-planning)).
pub(crate) fn warnings(resolution: &Resolution<'_>, desired: &[Desired], report: &mut Report) {
    let project = |path: &str| {
        desired
            .iter()
            .any(|d| d.owner == Owner::Project && d.path.as_str() == path)
    };
    let claude_import = project(claude_code::CLAUDE_MD);
    let gemini_import = project(gemini_cli::GEMINI_MD);
    let claude_selected = resolution
        .assets
        .values()
        .any(|r| r.clients.contains(&ClientId::ClaudeCode));
    for d in desired {
        let Owner::Asset(id) = &d.owner else { continue };
        if d.path.as_str() != agents_md::AGENTS_MD {
            continue;
        }
        // Claude Code reads all of AGENTS.md: through the import, or natively when
        // the project has no CLAUDE.md (`CC-MEM`).
        if !d.clients.contains(&ClientId::ClaudeCode) {
            let how = if claude_import {
                Some("through the `@AGENTS.md` import grimoire adds to CLAUDE.md")
            } else if claude_selected {
                Some("whenever the project has no CLAUDE.md")
            } else {
                None
            };
            if let Some(how) = how {
                report.warnings.push(Diagnostic::warning(
                    "plan/cross-client-rule",
                    format!("Claude Code also reads `{id}` from AGENTS.md {how}, although it is not installed for claude-code"),
                ));
            }
        }
        // Gemini CLI reads all of AGENTS.md through Grimoire's import (`GEMINI-P4`).
        if gemini_import && !d.clients.contains(&ClientId::GeminiCli) {
            report.warnings.push(Diagnostic::warning(
                "plan/cross-client-rule",
                format!("Gemini CLI also reads `{id}` from AGENTS.md through the `@AGENTS.md` import grimoire adds to GEMINI.md, although it is not installed for gemini-cli"),
            ));
        }
    }
    for (id, w) in written(desired) {
        let Some(resolved) = resolution.assets.get(id) else {
            continue;
        };
        let selected = |c: ClientId| resolved.clients.contains(&c);
        if resolved.asset.manifest.kind == Kind::Rule {
            duplicate_rule(id, &w, claude_import, &selected, report);
        }
        if w.has(Place::ClaudeSkill) && w.has(Place::AgentsSkill) && selected(ClientId::Cursor) {
            report.warnings.push(Diagnostic::warning(
                "plan/duplicate-skill",
                format!("Cursor reads skills from both .claude/skills/ and .agents/skills/, so it lists `{id}` twice"),
            ));
        }
        if w.has(Place::ClaudeAgent) {
            cross_client_agent(id, &w, report);
        }
        if w.has(Place::SettingsHooks) {
            report.warnings.push(Diagnostic::warning(
                "plan/cross-client-hooks",
                format!("Cursor and GitHub Copilot also run hooks from .claude/settings.json, so `{id}`'s hooks run there too; whether they pass `args` and honor `onFailure` is undocumented, so there a hook may let actions through when it fails, or block every tool call if its arguments are dropped (turn off Claude Code hooks in that client to recover)"),
            ));
        }
        if let Some(clients) = &w.mcp_json {
            if !clients.contains(&ClientId::Copilot) {
                report.warnings.push(Diagnostic::warning(
                    "plan/cross-client-mcp",
                    format!("GitHub Copilot (VS Code and CLI) also starts servers listed in .mcp.json, so `{id}` is available there too"),
                ));
            }
            if !clients.contains(&ClientId::ClaudeCode) {
                report.warnings.push(Diagnostic::warning(
                    "plan/cross-client-mcp",
                    format!("Claude Code also offers servers listed in .mcp.json once you approve them, so `{id}` is available there too"),
                ));
            }
        }
    }
}

/// A rule that a selected client still reads more than once.
fn duplicate_rule(
    id: &AssetId,
    w: &Written,
    claude_import: bool,
    selected: &impl Fn(ClientId) -> bool,
    report: &mut Report,
) {
    if selected(ClientId::Copilot) {
        // Copilot CLI reads AGENTS.md, CLAUDE.md (expanding its imports),
        // GEMINI.md (without imports), `.claude/rules/` and its own files (`COPILOT-P4`).
        let mut places = Vec::new();
        if w.has(Place::AgentsMd) {
            places.push("from AGENTS.md");
            if claude_import {
                places.push("through the `@AGENTS.md` import in CLAUDE.md");
            }
        }
        if w.has(Place::GeminiMd) {
            places.push("from GEMINI.md");
        }
        if w.has(Place::ClaudeRule) {
            places.push("from .claude/rules/");
        }
        if w.has(Place::CopilotRule) {
            places.push("from .github/instructions/");
        }
        // Copilot CLI deduplicates only identical copies (`COPILOT-P4`), and these
        // differ, so it most likely reads each one.
        if places.len() > 1 {
            report.warnings.push(Diagnostic::warning(
                "plan/duplicate-rule",
                format!(
                    "GitHub Copilot CLI may read `{id}` {}: {} (it deduplicates only identical copies)",
                    times(places.len()),
                    and_list(&places)
                ),
            ));
        }
    }
    if selected(ClientId::Cursor) && w.has(Place::AgentsMd) && claude_import {
        report.warnings.push(Diagnostic::warning(
            "plan/duplicate-rule",
            format!("Cursor may read `{id}` twice: it reads AGENTS.md and CLAUDE.md, and whether it expands the `@AGENTS.md` import in CLAUDE.md is undocumented"),
        ));
    }
}

fn times(n: usize) -> String {
    match n {
        2 => "twice".to_owned(),
        3 => "three times".to_owned(),
        n => format!("{n} times"),
    }
}

/// "a and b", "a, b and c".
fn and_list(items: &[&str]) -> String {
    match items.split_last() {
        Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
        Some((last, _)) => (*last).to_owned(),
        None => String::new(),
    }
}

/// An agent in `.claude/agents/`, which Cursor and Copilot load too.
fn cross_client_agent(id: &AssetId, w: &Written, report: &mut Report) {
    let mut parts = Vec::new();
    if !w.has(Place::CursorAgent) {
        parts.push("Cursor also loads it from .claude/agents/, has no per-tool allow-list, and how it treats a Claude Code agent's `tools` is undocumented, so it may get all of the parent agent's tools");
    }
    parts.push(if w.has(Place::CopilotAgent) {
        "VS Code loads the .claude/agents/ copy as well as the .github/agents/ one, and which it uses is undocumented (Copilot CLI uses .github/agents/)"
    } else {
        "GitHub Copilot (CLI and VS Code) also loads it from .claude/agents/, and how it maps Claude Code tool names and `disallowedTools` is undocumented"
    });
    report.warnings.push(Diagnostic::warning(
        "plan/cross-client-agent",
        format!("`{id}`: {}", parts.join("; ")),
    ));
}
