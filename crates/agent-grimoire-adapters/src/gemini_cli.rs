//! The `gemini-cli` adapter.
//!
//! What it writes was decided in
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//! the record of what the Codex, Cursor, Gemini CLI and Copilot adapters write.
//! The Gemini CLI facts it relies on are in
//! [`docs/specs.md`, GEMINI-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#gemini-p4-gemini-cli-facts-for-phase-4).
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `instructions` | `GEMINI.md` | a managed block per rule; or, when the rule is in `AGENTS.md`, Grimoire's `@AGENTS.md` import block |
//! | `skills:` | `.agents/skills/<path>` | a byte-for-byte copy |
//! | `agents:` | `.gemini/agents/<path>` | `name`, `description` and an explicit `tools` list |
//! | `mcp` | `.gemini/settings.json` | `mcpServers.<id>`: stdio with `${NAME}` references, or `url` with `type: "http"` |
//!
//! Gemini CLI reads all of these only in a folder the user has trusted. It finds
//! `GEMINI.md` from any directory up to the repository root, but reads
//! `.gemini/` and `.agents/skills/` only from the directory it was started in.
//! It has no `hooks` anchor, as
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)
//! decided: a failed Gemini CLI hook always lets the action through.

use agent_grimoire_install::entry::member_key;
use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{
    Anchor, Capability, ClientId, OutputSpec, RelPath, Strategy, Transport,
};
use agent_grimoire_resolve::Resolved;
use serde_json::{Map, Value, json};

use crate::text::{with_frontmatter, yaml_list, yaml_string};
use crate::{
    Report, agent_tools, agents_md, copy_tree, env_remap, no_destination, source_text, under,
    unsupported,
};

/// Gemini CLI's context file.
pub const GEMINI_MD: &str = "GEMINI.md";
/// Gemini CLI's project settings file, where MCP servers live. It is JSONC.
pub const SETTINGS: &str = ".gemini/settings.json";
/// Gemini CLI's project agent directory.
pub const AGENTS_DIR: &str = ".gemini/agents/";
/// The import line for `GEMINI.md`: Gemini CLI inlines a file named after `@`
/// at the start of a line ([`docs/specs.md`, GEMINI-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#gemini-p4-gemini-cli-facts-for-phase-4)).
pub const AGENTS_MD_IMPORT: &str = "@AGENTS.md\n";
const SKILLS: &str = ".agents/skills";

/// Renders one declared output.
pub(crate) fn render(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Vec<(RelPath, Content)> {
    match (spec.to.anchor(), spec.strategy) {
        (Anchor::Instructions, Strategy::Block) => {
            let Some(text) = agents_md::block_text(resolved, spec, report) else {
                return Vec::new();
            };
            RelPath::new(GEMINI_MD)
                .ok()
                .map(|p| (p, Content::Block(text)))
                .into_iter()
                .collect()
        }
        (Anchor::Skills, Strategy::Copy) => copy_tree(resolved, spec, SKILLS),
        (Anchor::Agents, Strategy::Render) => agent(resolved, spec, report).into_iter().collect(),
        (Anchor::Mcp, Strategy::Entry) => mcp(resolved, report).into_iter().collect(),
        (anchor, _) => {
            no_destination(report, ClientId::GeminiCli, resolved, anchor);
            Vec::new()
        }
    }
}

/// Gemini CLI exposes a tool name of up to this many characters unchanged.
const MAX_TOOL_NAME: usize = 63;

/// Gemini CLI tool names for a capability
/// ([`docs/specs.md`, GEMINI-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#gemini-p4-gemini-cli-facts-for-phase-4)). MCP tools are
/// `mcp_<server>_<tool>`, with the wildcard `mcp_<server>_*`.
fn tool_names(capability: &Capability) -> Result<Vec<String>, String> {
    let names: &[&str] = match capability {
        Capability::Read => &["read_file", "read_many_files"],
        Capability::Search => &["grep_search", "glob", "list_directory"],
        Capability::Edit => &["replace", "write_file"],
        Capability::Execute => &["run_shell_command"],
        Capability::Web => &["web_fetch", "google_web_search"],
        Capability::Mcp { server, tool } => {
            // Gemini CLI splits a tool name at the first `_` after `mcp_`, so a
            // server name can't contain one; asset ids never do.
            let name = match tool {
                Some(tool) => {
                    let t = tool.as_str();
                    if !t
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b"_.:-".contains(&b))
                    {
                        return Err(format!(
                            "Gemini CLI tool names allow only letters, digits, `_`, `.`, `:` and `-`, so `{capability}` can't be listed"
                        ));
                    }
                    let name = format!("mcp_{server}_{t}");
                    // Longer names are shortened when exposed, so a `tools` entry
                    // with the full name would match nothing (`GEMINI-P4`).
                    if name.len() > MAX_TOOL_NAME {
                        return Err(format!(
                            "Gemini CLI shortens tool names over {MAX_TOOL_NAME} characters, so `{name}` would match no tool"
                        ));
                    }
                    name
                }
                None => format!("mcp_{server}_*"),
            };
            return Ok(vec![name]);
        }
    };
    Ok(names.iter().map(|n| (*n).to_owned()).collect())
}

/// An agent with an explicit `tools` list. Gemini CLI refuses a file with an
/// unknown frontmatter key, and `tools: []` leaves only its `complete_task`.
fn agent(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let mut tools: Vec<String> = Vec::new();
    for capability in agent_tools(resolved) {
        match tool_names(capability) {
            Ok(names) => {
                for name in names {
                    if !tools.contains(&name) {
                        tools.push(name);
                    }
                }
            }
            Err(problem) => {
                unsupported(
                    report,
                    format!(
                        "gemini-cli cannot grant `{}` the tool `{capability}`: {problem}",
                        manifest.id
                    ),
                );
                return None;
            }
        }
    }
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let path = under(AGENTS_DIR.trim_end_matches('/'), spec.to.path()?)?;
    let fields = [
        ("name", yaml_string(manifest.id.as_str())),
        ("description", yaml_string(manifest.description.as_str())),
        ("tools", yaml_list(tools.iter().map(String::as_str))),
    ];
    Some((
        path,
        Content::File(with_frontmatter(&fields, &body).into_bytes()),
    ))
}

/// `mcpServers.<id>`
/// ([`docs/specs.md`, GEMINI-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#gemini-p4-gemini-cli-facts-for-phase-4)). Gemini CLI hides variables whose names look
/// secret (`*TOKEN*`, `*KEY*` …) from servers unless they are listed in `env`,
/// so every variable the server reads is listed explicitly.
fn mcp(resolved: &Resolved<'_>, report: &mut Report) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let server = manifest.mcp_server.as_ref()?;
    let options = resolved.options.as_ref();
    let key = member_key(&["mcpServers", manifest.id.as_str()]).ok()?;
    let mut entry = Map::new();
    if let Some(Transport::StreamableHttp) = options.and_then(|o| o.transport) {
        entry.insert(
            "url".into(),
            json!(options.and_then(|o| o.url.as_ref())?.as_str()),
        );
        entry.insert("type".into(), json!("http"));
    } else {
        let remap = env_remap(resolved, report);
        entry.insert("command".into(), json!(server.binary.as_str()));
        if !server.env.is_empty() {
            let mut env = Map::new();
            for var in &server.env {
                let source = remap.and_then(|r| r.get(&var.name)).unwrap_or(&var.name);
                // An unset variable without a default keeps its placeholder text.
                let reference = if var.required {
                    format!("${{{source}}}")
                } else {
                    format!("${{{source}:-}}")
                };
                env.insert(var.name.as_str().to_owned(), Value::String(reference));
            }
            entry.insert("env".into(), Value::Object(env));
        }
    }
    Some((
        RelPath::new(SETTINGS).ok()?,
        Content::Entry {
            key,
            value: Value::Object(entry),
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cap(s: &str) -> Capability {
        Capability::parse(s).unwrap()
    }

    #[test]
    fn tool_names_cover_every_capability() {
        assert_eq!(
            tool_names(&cap("search")).unwrap(),
            ["grep_search", "glob", "list_directory"]
        );
        assert_eq!(
            tool_names(&cap("mcp:docs-index")).unwrap(),
            ["mcp_docs-index_*"]
        );
        assert_eq!(
            tool_names(&cap("mcp:docs-index/get_page")).unwrap(),
            ["mcp_docs-index_get_page"]
        );
    }
}
