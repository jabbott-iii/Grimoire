//! The `copilot` adapter for GitHub Copilot.
//!
//! What it writes was decided in
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//! the record of what the Codex, Cursor, Gemini CLI and Copilot adapters write.
//! The Copilot facts it relies on are in
//! [`docs/specs.md`, COPILOT-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#copilot-p4-github-copilot-facts-for-phase-4).
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `rules:` | `.github/instructions/<id>.instructions.md`, or `AGENTS.md` when that is written | `applyTo` frontmatter |
//! | `skills:` | `.agents/skills/<path>`, or `.claude/skills/<path>` | a byte-for-byte copy, placed where other clients read it too |
//! | `agents:` | `.github/agents/<id>.agent.md` | `name`, `description`, a `tools` list of aliases and `strict-tools-list: true` |
//! | `mcp` | `.mcp.json` | the same `mcpServers.<id>` entry as `claude-code`, written once for both |
//!
//! Copilot reads `AGENTS.md` (for the cloud agent alongside a root `CLAUDE.md`
//! this is undocumented), so a rule that is written there gets no
//! `.instructions.md` file (placement). Copilot has no `hooks` anchor, as
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)
//! decided: a timed-out Copilot hook always lets the action through,
//! and the cloud agent runs only shell strings. Its cloud agent reads MCP
//! servers only from the repository settings, which `grimoire doctor`
//! explains.

use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{
    Anchor, Capability, ClientId, Diagnostic, OutputSpec, RelPath, Strategy,
};
use agent_grimoire_resolve::Resolved;

use crate::text::{with_frontmatter, yaml_list, yaml_string};
use crate::{
    Report, agent_tools, claude_code, copy_tree, cursor, no_destination, renamed, rule_paths,
    source_text, unsupported,
};

/// Copilot's path-specific instruction directory.
pub const INSTRUCTIONS_DIR: &str = ".github/instructions/";
/// Copilot's custom agent directory.
pub const AGENTS_DIR: &str = ".github/agents/";
const SKILLS: &str = ".agents/skills";
/// The documented cap on a custom agent's prompt
/// ([`docs/specs.md`, COPILOT-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#copilot-p4-github-copilot-facts-for-phase-4)).
const MAX_AGENT_BODY: usize = 30_000;

/// Renders one declared output.
pub(crate) fn render(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Vec<(RelPath, Content)> {
    match (spec.to.anchor(), spec.strategy) {
        (Anchor::Rules, Strategy::Render) => rule(resolved, spec, report).into_iter().collect(),
        (Anchor::Skills, Strategy::Copy) => copy_tree(resolved, spec, SKILLS),
        (Anchor::Agents, Strategy::Render) => agent(resolved, spec, report).into_iter().collect(),
        (Anchor::Mcp, Strategy::Entry) => claude_code::mcp(resolved, report).into_iter().collect(),
        (anchor, _) => {
            no_destination(report, ClientId::Copilot, resolved, anchor);
            Vec::new()
        }
    }
}

/// `applyTo` takes one glob or several separated by commas, relative to the
/// repository root; `**` applies everywhere.
fn rule(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = renamed(
        INSTRUCTIONS_DIR.trim_end_matches('/'),
        spec,
        ".instructions.md",
    )?;
    let apply_to = if rule_paths(resolved).is_empty() {
        yaml_string("**")
    } else {
        cursor::comma_globs(ClientId::Copilot, resolved, ",", report)
    };
    Some((
        path,
        Content::File(with_frontmatter(&[("applyTo", apply_to)], &body).into_bytes()),
    ))
}

/// Copilot tool aliases for a capability
/// ([`docs/specs.md`, COPILOT-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#copilot-p4-github-copilot-facts-for-phase-4)). MCP tools are
/// `<server>/<tool>`, or `<server>/*` for all of a server's tools.
fn tool_names(capability: &Capability) -> String {
    match capability {
        Capability::Read => "read".to_owned(),
        Capability::Search => "search".to_owned(),
        Capability::Edit => "edit".to_owned(),
        Capability::Execute => "execute".to_owned(),
        Capability::Web => "web".to_owned(),
        Capability::Mcp { server, tool } => match tool {
            Some(tool) => format!("{server}/{tool}"),
            None => format!("{server}/*"),
        },
    }
}

/// An agent with an explicit `tools` list. Without `strict-tools-list: true`,
/// Copilot CLI adds its usual default tools to the list; `tools: []` grants none.
fn agent(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    if body.chars().count() > MAX_AGENT_BODY {
        unsupported(
            report,
            format!(
                "copilot caps an agent's prompt at {MAX_AGENT_BODY} characters, and `{}`'s is longer",
                manifest.id
            ),
        );
        return None;
    }
    let path = renamed(AGENTS_DIR.trim_end_matches('/'), spec, ".agent.md")?;
    let mut tools: Vec<String> = Vec::new();
    for name in agent_tools(resolved).iter().map(tool_names) {
        if !tools.contains(&name) {
            tools.push(name);
        }
    }
    report.warnings.push(Diagnostic::warning(
        "adapter/agent-strict-tools",
        format!(
            "copilot limits `{}` to its tools with `strict-tools-list: true`, which VS Code doesn't pass on, so in VS Code the agent may also get Copilot's default tools",
            manifest.id
        ),
    ));
    let fields = [
        ("name", yaml_string(manifest.id.as_str())),
        ("description", yaml_string(manifest.description.as_str())),
        ("tools", yaml_list(tools.iter().map(String::as_str))),
        ("strict-tools-list", "true".to_owned()),
    ];
    Some((
        path,
        Content::File(with_frontmatter(&fields, &body).into_bytes()),
    ))
}
