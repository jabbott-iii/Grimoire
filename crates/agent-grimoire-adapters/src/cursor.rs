//! The `cursor` adapter.
//!
//! What it writes was decided in
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//! the record of what the Codex, Cursor, Gemini CLI and Copilot adapters write.
//! The Cursor facts it relies on are in
//! [`docs/specs.md`, CURSOR-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cursor-p4-cursor-facts-for-phase-4).
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `rules:` | `.cursor/rules/<id>.mdc`, or `AGENTS.md` when that is written | `alwaysApply: true`, or `alwaysApply: false` with `globs` |
//! | `skills:` | `.agents/skills/<path>`, or `.claude/skills/<path>` | a byte-for-byte copy, placed where other clients read it too |
//! | `agents:` | `.cursor/agents/<id>.md` | `name`, `description`, `readonly: true`; only for agents limited to `read` and `search` |
//! | `mcp` | `.cursor/mcp.json` | `mcpServers.<id>`: stdio with `${env:NAME}` references, or the configured URL |
//!
//! Cursor always applies `AGENTS.md`, so a rule that is written there gets no
//! `.mdc` file (placement). Cursor has no `hooks` anchor, as
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)
//! decided: its hooks fail open by default (a hook that fails lets the action
//! through) and can't be tested here.

use agent_grimoire_install::entry::member_key;
use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{
    Anchor, Capability, ClientId, Diagnostic, OutputSpec, RelPath, Strategy, Transport,
};
use agent_grimoire_resolve::Resolved;
use serde_json::{Map, Value, json};

use crate::text::{with_frontmatter, yaml_string};
use crate::{
    Report, agent_tools, copy_tree, env_remap, no_destination, renamed, rule_paths, source_text,
    unsupported,
};

/// Cursor's rule directory.
pub const RULES_DIR: &str = ".cursor/rules/";
/// Cursor's agent directory.
pub const AGENTS_DIR: &str = ".cursor/agents/";
/// Cursor's project MCP configuration. Cursor does not read the root `.mcp.json`.
pub const MCP: &str = ".cursor/mcp.json";
const SKILLS: &str = ".agents/skills";

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
        (Anchor::Mcp, Strategy::Entry) => mcp(resolved, report).into_iter().collect(),
        (anchor, _) => {
            no_destination(report, ClientId::Cursor, resolved, anchor);
            Vec::new()
        }
    }
}

/// `globs` is a comma-separated string, so a glob that itself contains a comma,
/// such as a brace set, can't be written. Grimoire quotes the string, because an
/// unquoted pattern starting with `*` is a YAML alias; whether Cursor reads it
/// as YAML is undocumented ([`docs/specs.md`, CURSOR-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cursor-p4-cursor-facts-for-phase-4)).
///
/// The refusal is deferred: when `AGENTS.md` holds the rule, placement drops
/// this client's file, and the glob goes in the scope line instead.
pub(crate) fn comma_globs(
    client: ClientId,
    resolved: &Resolved<'_>,
    separator: &str,
    report: &mut Report,
) -> String {
    let paths = rule_paths(resolved);
    if let Some(bad) = paths.iter().find(|p| p.contains(',')) {
        report.deferred.push((
            resolved.id().clone(),
            client,
            Diagnostic::error(
                "adapter/unsupported",
                format!(
                    "{client} takes a comma-separated glob list, so it can't scope `{}` to `{bad}`, which contains a comma; select agents-md or codex for it too, and {client} reads it from AGENTS.md",
                    resolved.id()
                ),
            ),
        ));
    }
    yaml_string(&paths.join(separator))
}

fn rule(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = renamed(RULES_DIR.trim_end_matches('/'), spec, ".mdc")?;
    let fields = if rule_paths(resolved).is_empty() {
        vec![("alwaysApply", "true".to_owned())]
    } else {
        vec![
            ("alwaysApply", "false".to_owned()),
            (
                "globs",
                comma_globs(ClientId::Cursor, resolved, ", ", report),
            ),
        ]
    };
    Some((
        path,
        Content::File(with_frontmatter(&fields, &body).into_bytes()),
    ))
}

/// A Cursor agent has no tool list, only `readonly: true`, documented as "no
/// file edits, no state-changing shell commands"
/// ([`docs/specs.md`, CURSOR-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cursor-p4-cursor-facts-for-phase-4)). That matches an
/// agent limited to reading and searching, and nothing else.
fn agent(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let tools = agent_tools(resolved);
    let read_only = tools.len() == 2
        && tools.contains(&Capability::Read)
        && tools.contains(&Capability::Search);
    if !read_only {
        let declared: Vec<String> = tools.iter().map(ToString::to_string).collect();
        unsupported(
            report,
            format!(
                "cursor can't limit `{}` to its tools ({}): a Cursor agent has no tool list, only `readonly`, which fits an agent with exactly `read` and `search`",
                manifest.id,
                if declared.is_empty() {
                    "none".to_owned()
                } else {
                    declared.join(", ")
                }
            ),
        );
        return None;
    }
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = renamed(AGENTS_DIR.trim_end_matches('/'), spec, ".md")?;
    report.warnings.push(Diagnostic::warning(
        "adapter/agent-readonly",
        format!(
            "cursor runs `{}` with `readonly: true`, which blocks edits and state-changing shell commands; shell commands Cursor judges harmless stay available, and whether it limits MCP tools and web fetches is undocumented",
            manifest.id
        ),
    ));
    let fields = [
        ("name", yaml_string(manifest.id.as_str())),
        ("description", yaml_string(manifest.description.as_str())),
        ("readonly", "true".to_owned()),
    ];
    Some((
        path,
        Content::File(with_frontmatter(&fields, &body).into_bytes()),
    ))
}

/// `mcpServers.<id>`
/// ([`docs/specs.md`, CURSOR-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cursor-p4-cursor-facts-for-phase-4)). Cursor writes environment references as
/// `${env:NAME}` and documents no default syntax, so optional variables are
/// written the same way.
fn mcp(resolved: &Resolved<'_>, report: &mut Report) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let server = manifest.mcp_server.as_ref()?;
    let options = resolved.options.as_ref();
    let key = member_key(&["mcpServers", manifest.id.as_str()]).ok()?;
    let mut entry = Map::new();
    if let Some(Transport::StreamableHttp) = options.and_then(|o| o.transport) {
        // The documented remote form is a bare `url`; Cursor infers the transport.
        entry.insert(
            "url".into(),
            json!(options.and_then(|o| o.url.as_ref())?.as_str()),
        );
    } else {
        let remap = env_remap(resolved, report);
        entry.insert("type".into(), json!("stdio"));
        entry.insert("command".into(), json!(server.binary.as_str()));
        if !server.env.is_empty() {
            let mut env = Map::new();
            for var in &server.env {
                let source = remap.and_then(|r| r.get(&var.name)).unwrap_or(&var.name);
                env.insert(
                    var.name.as_str().to_owned(),
                    Value::String(format!("${{env:{source}}}")),
                );
            }
            entry.insert("env".into(), Value::Object(env));
        }
    }
    Some((
        RelPath::new(MCP).ok()?,
        Content::Entry {
            key,
            value: Value::Object(entry),
        },
    ))
}
