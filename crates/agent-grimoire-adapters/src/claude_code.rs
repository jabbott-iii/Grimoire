//! The `claude-code` adapter.
//!
//! Every format here was checked against the Claude Code docs on 2026-11-16
//! (`CC-MEM`, `CC-SKILL`, `CC-AGENT`, `CC-SET`, `CC-MCP` in `docs/specs.md`):
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `rules:` | `.claude/rules/<path>` | the rule, with `paths` frontmatter when path-scoped |
//! | `skills:` | `.claude/skills/<path>` | a byte-for-byte copy |
//! | `agents:` | `.claude/agents/<path>` | `name`, `description` and an explicit `tools` list |
//! | `mcp` | `.mcp.json` | `mcpServers.<id>`: stdio with `${VAR}` references only, or `http` with the configured URL |
//! | `hooks` | `.claude/settings.json` | one matcher group per binding, exec form |

use std::collections::BTreeSet;

use agent_grimoire_install::entry::{element_key, member_key};
use agent_grimoire_install::plan::{Content, Desired, Owner};
use agent_grimoire_model::{
    Anchor, Capability, ClientId, Diagnostic, EnvVarName, HookEvent, HookFailure, OutputSpec,
    RelPath, Strategy, Transport,
};
use agent_grimoire_resolve::Resolved;
use serde_json::{Map, Value, json};

use crate::text::{with_frontmatter, yaml_list, yaml_string};
use crate::{Report, copy_tree, env_remap, no_destination, rule_paths, source_text, under};

/// Claude Code's project settings file, where hooks live.
pub const SETTINGS: &str = ".claude/settings.json";
/// Claude Code's project MCP configuration.
pub const MCP: &str = ".mcp.json";
const RULES: &str = ".claude/rules";
const SKILLS: &str = ".claude/skills";
const AGENTS: &str = ".claude/agents";
/// The project memory file that imports `AGENTS.md`.
pub const CLAUDE_MD: &str = "CLAUDE.md";
/// The import line; `CC-MEM` documents that it never makes Claude read `AGENTS.md` twice.
pub const AGENTS_MD_IMPORT: &str = "@AGENTS.md\n";

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
        (Anchor::Hooks, Strategy::Entry) => hooks(resolved, report),
        (anchor, _) => {
            no_destination(report, ClientId::ClaudeCode, resolved, anchor);
            Vec::new()
        }
    }
}

/// The `CLAUDE.md` block that makes Claude Code read the rules in `AGENTS.md`.
#[must_use]
pub fn agents_md_import() -> Desired {
    Desired {
        owner: Owner::Project,
        clients: BTreeSet::from([ClientId::ClaudeCode]),
        path: RelPath::new(CLAUDE_MD).unwrap_or_else(|_| unreachable!("a valid path")),
        content: Content::Block(AGENTS_MD_IMPORT.to_owned()),
    }
}

fn rule(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = under(RULES, spec.to.path()?)?;
    let paths = rule_paths(resolved);
    let text = if paths.is_empty() {
        body
    } else {
        with_frontmatter(&[("paths", yaml_list(paths))], &body)
    };
    Some((path, Content::File(text.into_bytes())))
}

/// Claude Code drops a tool whose full name is over 128 characters (`CC-P2`).
const MAX_TOOL_NAME: usize = 128;

/// How Claude Code names an MCP server's tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum McpNaming {
    /// A server in the project's `.mcp.json`: `mcp__<server>__<tool>`.
    Project,
    /// A server shipped by the plugin of the same name, as in Grimoire's
    /// marketplace: `mcp__plugin_<plugin>_<server>__<tool>` (`CC-PLUGIN-P4`).
    Plugin,
}

impl McpNaming {
    fn prefix(self, server: &agent_grimoire_model::AssetId) -> String {
        match self {
            // Asset ids are kebab-case, so nothing needs replacing.
            McpNaming::Project => format!("mcp__{server}__"),
            McpNaming::Plugin => format!("mcp__plugin_{server}_{server}__"),
        }
    }
}

/// Why Claude Code may not expose an MCP tool under the name Grimoire would
/// write. Model-facing tool names allow only `[A-Za-z0-9_-]`; how Claude Code
/// renames other characters in tool names is not documented (`CC-P2`).
fn mcp_name_problem(capability: &Capability, naming: McpNaming) -> Option<String> {
    let Capability::Mcp { server, tool } = capability else {
        return None;
    };
    let tool = tool.as_ref()?;
    let full = format!("{}{tool}", naming.prefix(server));
    if tool.as_str().contains('.') {
        Some(format!(
            "`{full}` contains '.', which Claude Code may rename"
        ))
    } else if full.len() > MAX_TOOL_NAME {
        Some(format!(
            "`{full}` is longer than {MAX_TOOL_NAME} characters, so Claude Code drops the tool"
        ))
    } else {
        None
    }
}

/// Claude Code tool names for a capability (`CC-AGENT`).
fn tool_names(capability: &Capability, naming: McpNaming) -> Vec<String> {
    let names: &[&str] = match capability {
        Capability::Read => &["Read"],
        // Glob and Grep come back for a subagent that lists them without Bash.
        Capability::Search => &["Grep", "Glob"],
        Capability::Edit => &["Edit", "Write", "NotebookEdit"],
        // PowerShell is the shell tool on Windows without Git Bash.
        Capability::Execute => &["Bash", "PowerShell"],
        Capability::Web => &["WebFetch", "WebSearch"],
        Capability::Mcp { server, tool } => {
            let prefix = naming.prefix(server);
            return vec![match tool {
                Some(tool) => format!("{prefix}{tool}"),
                None => format!("{prefix}*"),
            }];
        }
    };
    names.iter().map(|n| (*n).to_owned()).collect()
}

fn agent(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = under(AGENTS, spec.to.path()?)?;
    match agent_file(manifest, &body, McpNaming::Project) {
        Ok(text) => Some((path, Content::File(text.into_bytes()))),
        Err(problem) => {
            report
                .errors
                .push(Diagnostic::error("adapter/unsupported", problem));
            None
        }
    }
}

/// An agent file: frontmatter `name`, `description` and an explicit `tools`
/// list, then the prompt. Fails when a tool can't be named for Claude Code.
pub(crate) fn agent_file(
    manifest: &agent_grimoire_model::AssetManifest,
    body: &str,
    naming: McpNaming,
) -> Result<String, String> {
    let mut tools: Vec<String> = Vec::new();
    for capability in manifest
        .agent
        .as_ref()
        .map(|a| a.tools.as_slice())
        .unwrap_or_default()
    {
        if let Some(problem) = mcp_name_problem(capability, naming) {
            return Err(format!(
                "claude-code cannot grant `{}` the tool `{capability}`: {problem}",
                manifest.id
            ));
        }
        for name in tool_names(capability, naming) {
            if !tools.contains(&name) {
                tools.push(name);
            }
        }
    }
    let mut fields = vec![
        ("name", yaml_string(manifest.id.as_str())),
        ("description", yaml_string(manifest.description.as_str())),
    ];
    if tools.is_empty() {
        // An omitted `tools` grants every tool. The documented way to grant none:
        // "disallowedTools removes every entry in it, … launches the subagent
        // without tools" (`CC-AGENT`). If disallowedTools were ignored, the agent
        // would still have only Read: narrower, never wider.
        fields.push(("tools", yaml_list(["Read"])));
        fields.push(("disallowedTools", yaml_list(["Read"])));
    } else {
        fields.push(("tools", yaml_list(tools.iter().map(String::as_str))));
    }
    Ok(with_frontmatter(&fields, body))
}

/// The `.mcp.json` entry, which GitHub Copilot reads too (`COPILOT-P4`).
pub(crate) fn mcp(resolved: &Resolved<'_>, report: &mut Report) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let id = &manifest.id;
    let server = manifest.mcp_server.as_ref()?;
    let options = resolved.options.as_ref();
    let key = member_key(&["mcpServers", id.as_str()]).ok()?;
    if let Some(Transport::StreamableHttp) = options.and_then(|o| o.transport) {
        // The user runs the server; Claude Code connects to it (`CC-MCP`).
        // `grimoire.toml` validation guarantees the URL is present.
        let url = options.and_then(|o| o.url.as_ref())?;
        let mut entry = Map::new();
        entry.insert("type".into(), json!("http"));
        entry.insert("url".into(), json!(url.as_str()));
        return Some((
            RelPath::new(MCP).ok()?,
            Content::Entry {
                key,
                value: Value::Object(entry),
            },
        ));
    }
    let remap = env_remap(resolved, report);
    Some((
        RelPath::new(MCP).ok()?,
        Content::Entry {
            key,
            value: stdio_entry(server, remap),
        },
    ))
}

/// A stdio server entry: `{"type": "stdio", "command", "env"}` with `${VAR}`
/// references, optionally reading each variable from a user-chosen name.
pub(crate) fn stdio_entry(
    server: &agent_grimoire_model::McpServerSpec,
    remap: Option<&std::collections::BTreeMap<EnvVarName, EnvVarName>>,
) -> Value {
    let mut entry = Map::new();
    entry.insert("type".into(), json!("stdio"));
    entry.insert("command".into(), json!(server.binary.as_str()));
    if !server.env.is_empty() {
        let mut env = Map::new();
        for var in &server.env {
            let source = remap.and_then(|r| r.get(&var.name)).unwrap_or(&var.name);
            // Unset variables without a default are passed through literally
            // (`CC-MCP`), so optional ones get an empty default.
            let reference = if var.required {
                format!("${{{source}}}")
            } else {
                format!("${{{source}:-}}")
            };
            env.insert(var.name.as_str().to_owned(), Value::String(reference));
        }
        entry.insert("env".into(), Value::Object(env));
    }
    Value::Object(entry)
}

fn event_name(event: HookEvent) -> &'static str {
    match event {
        HookEvent::PreToolUse => "PreToolUse",
        HookEvent::PostToolUse => "PostToolUse",
        HookEvent::SessionStart => "SessionStart",
        HookEvent::SessionEnd => "SessionEnd",
        HookEvent::UserPromptSubmit => "UserPromptSubmit",
        HookEvent::Stop => "Stop",
    }
}

/// A matcher for tool names. Names of letters, digits, `_` and `-` joined by `|`
/// are an exact list; anything else is an unanchored JavaScript regex, so it is
/// anchored here (`CC-SET`).
fn matcher(tools: &[Capability]) -> Option<String> {
    let mut names: Vec<String> = Vec::new();
    for capability in tools {
        let mapped = match capability {
            Capability::Mcp { server, tool } => vec![match tool {
                Some(tool) => format!("mcp__{server}__{}", tool.as_str().replace('.', "\\.")),
                None => format!("mcp__{server}__.*"),
            }],
            other => tool_names(other, McpNaming::Project),
        };
        for name in mapped {
            if !names.contains(&name) {
                names.push(name);
            }
        }
    }
    if names.is_empty() {
        return None;
    }
    let exact = names.iter().all(|n| {
        n.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
    });
    Some(if exact {
        names.join("|")
    } else {
        format!("^({})$", names.join("|"))
    })
}

fn hooks(resolved: &Resolved<'_>, report: &mut Report) -> Vec<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let Some(tool) = manifest.tool.as_ref() else {
        return Vec::new();
    };
    let Ok(path) = RelPath::new(SETTINGS) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (n, binding) in tool.hooks.iter().enumerate() {
        let mut handler = Map::new();
        handler.insert("type".into(), json!("command"));
        handler.insert("command".into(), json!(tool.binary.as_str()));
        // `args` selects exec form: no shell, one argv element per entry (`CC-SET`).
        handler.insert(
            "args".into(),
            Value::Array(binding.args.iter().map(|a| json!(a.as_str())).collect()),
        );
        if let Some(secs) = binding.timeout_secs {
            handler.insert("timeout".into(), json!(secs));
        }
        // `onFailure: "block"` makes a hook that cannot start, crashes or times
        // out block the event instead of letting it through (2.1.295+, `CC-P3`).
        if binding.on_failure == HookFailure::Block {
            handler.insert("onFailure".into(), json!("block"));
        }
        if let Some(problem) = binding
            .tools
            .iter()
            .find_map(|c| mcp_name_problem(c, McpNaming::Project))
        {
            report.errors.push(Diagnostic::error(
                "adapter/unsupported",
                format!(
                    "claude-code cannot match `{}`'s hook on that MCP tool: {problem}",
                    manifest.id
                ),
            ));
            continue;
        }
        if binding.tools.contains(&Capability::Search) {
            report.warnings.push(Diagnostic::warning(
                "adapter/hook-tools",
                format!("`{}`'s hook matches Grep and Glob, which Claude Code's main session does not have by default on macOS, Linux and WSL (it searches with Bash), so the hook may rarely run", manifest.id),
            ));
        }
        let mut group = Map::new();
        if let Some(m) = matcher(&binding.tools) {
            group.insert("matcher".into(), Value::String(m));
        }
        group.insert("hooks".into(), Value::Array(vec![Value::Object(handler)]));
        let event = event_name(binding.event);
        match element_key(&["hooks", event], &format!("{}#{n}", manifest.id)) {
            Ok(key) => out.push((
                path.clone(),
                Content::Entry {
                    key,
                    value: Value::Object(group),
                },
            )),
            Err(e) => report
                .errors
                .push(Diagnostic::error("adapter/entry-key", e.to_string())),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cap(s: &str) -> Capability {
        Capability::parse(s).unwrap()
    }

    #[test]
    fn matchers_are_exact_lists_or_anchored_regexes() {
        assert_eq!(matcher(&[]), None);
        assert_eq!(matcher(&[cap("edit")]).unwrap(), "Edit|Write|NotebookEdit");
        assert_eq!(
            matcher(&[cap("execute"), cap("mcp:docs-index")]).unwrap(),
            "^(Bash|PowerShell|mcp__docs-index__.*)$"
        );
        assert_eq!(
            matcher(&[cap("mcp:docs-index/search.v2")]).unwrap(),
            "^(mcp__docs-index__search\\.v2)$"
        );
    }

    #[test]
    fn tool_names_cover_every_capability() {
        let project = McpNaming::Project;
        assert_eq!(tool_names(&cap("search"), project), ["Grep", "Glob"]);
        assert_eq!(
            tool_names(&cap("mcp:docs-index"), project),
            ["mcp__docs-index__*"]
        );
        assert_eq!(
            tool_names(&cap("mcp:docs-index/search"), project),
            ["mcp__docs-index__search"]
        );
        assert_eq!(
            tool_names(&cap("mcp:docs-index/search"), McpNaming::Plugin),
            ["mcp__plugin_docs-index_docs-index__search"]
        );
    }
}
