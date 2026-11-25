//! Claude Code plugins for Grimoire's generated marketplace
//! (`cargo xtask gen-marketplace`).
//!
//! The plugin manifest, components and naming rules it follows are in
//! [`docs/specs.md`, CC-PLUGIN-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cc-plugin-p4-plugin-manifest-components-and-naming).
//!
//! Each eligible asset becomes one plugin named after it, with the same files
//! the `claude-code` adapter installs, laid out the way a plugin carries them:
//!
//! | Kind | Plugin files |
//! |---|---|
//! | skill | `skills/<id>/…`, byte for byte |
//! | agent | `agents/<id>.md`, with MCP tools named as a plugin's (`mcp__plugin_<server>_<server>__<tool>`) |
//! | MCP server | `.mcp.json` with the stdio entry |
//!
//! Rules and tools are not plugins: Claude Code loads no instructions from a
//! plugin, and plugin hooks aren't known to fail closed (task M4.7 in docs/roadmap.md).

use agent_grimoire_catalog::SourceAsset;
use agent_grimoire_model::{
    Anchor, AssetId, AssetManifest, Capability, ClientId, Diagnostic, Kind, RelPath, Strategy,
};
use serde_json::json;

use crate::claude_code::{McpNaming, agent_file, stdio_entry};

/// Why an asset isn't published as a Claude Code plugin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ineligible {
    /// A rule: Claude Code loads no instructions from a plugin.
    Rule,
    /// A tool: Grimoire doesn't publish hooks as plugins until a plugin hook is
    /// known to fail closed, because it never installs the binary.
    Tool,
    /// An asset without a `claude-code` table.
    NoClaudeCode,
}

impl Ineligible {
    /// Why, in a few words.
    #[must_use]
    pub fn describe(self) -> &'static str {
        match self {
            Ineligible::Rule => "a rule, and Claude Code loads no instructions from a plugin",
            Ineligible::Tool => {
                "a tool, and Grimoire publishes no hooks as plugins until a plugin hook is known to fail closed"
            }
            Ineligible::NoClaudeCode => "not available for Claude Code",
        }
    }

    /// Whether the grimoire CLI can install it for Claude Code instead.
    #[must_use]
    pub fn cli_installs(self) -> bool {
        self != Ineligible::NoClaudeCode
    }
}

/// Why an asset can't be a Claude Code plugin, or `None` if it can.
#[must_use]
pub fn ineligible(manifest: &AssetManifest) -> Option<Ineligible> {
    match manifest.kind {
        _ if !manifest.clients.contains_key(&ClientId::ClaudeCode) => {
            Some(Ineligible::NoClaudeCode)
        }
        Kind::Rule => Some(Ineligible::Rule),
        Kind::Tool => Some(Ineligible::Tool),
        Kind::Skill | Kind::Agent | Kind::McpServer => None,
    }
}

/// One plugin's component files and the MCP servers its agent uses.
#[derive(Debug, Default)]
pub struct PluginFiles {
    /// Files relative to the plugin root, `.claude-plugin/plugin.json` excluded.
    pub files: Vec<(RelPath, Vec<u8>)>,
    /// MCP-server assets whose tools an agent names in the plugin form; their
    /// plugins must exist for those tools to.
    pub mcp_servers: Vec<AssetId>,
}

/// The component files of an eligible asset's plugin.
///
/// # Errors
///
/// Returns a diagnostic when the asset is ineligible, a source file is missing
/// or not UTF-8, or an agent's tool can't be named for Claude Code.
pub fn files(asset: &SourceAsset) -> Result<PluginFiles, Diagnostic> {
    let manifest = &asset.manifest;
    let id = &manifest.id;
    let fail = |message: String| Diagnostic::error("plugin/asset", message);
    if let Some(reason) = ineligible(manifest) {
        return Err(fail(format!(
            "`{id}` can't be a plugin: it is {}",
            reason.describe()
        )));
    }
    let specs = manifest
        .clients
        .get(&ClientId::ClaudeCode)
        .map(|c| c.outputs.as_slice())
        .unwrap_or_default();
    let mut out = PluginFiles::default();
    for spec in specs {
        match (spec.to.anchor(), spec.strategy) {
            (Anchor::Skills, Strategy::Copy) => {
                let (Some(src), Some(to)) = (spec.src.as_ref(), spec.to.path()) else {
                    continue;
                };
                for (path, bytes) in asset.files_under(src) {
                    let rest = &path.as_str()[src.as_str().len()..];
                    let dest = RelPath::new(format!("skills/{to}{rest}"))
                        .map_err(|e| fail(e.to_string()))?;
                    out.files.push((dest, bytes.to_vec()));
                }
            }
            (Anchor::Agents, Strategy::Render) => {
                let (Some(src), Some(to)) = (spec.src.as_ref(), spec.to.path()) else {
                    continue;
                };
                let body = asset
                    .file(src)
                    .and_then(|b| std::str::from_utf8(b).ok())
                    .ok_or_else(|| fail(format!("`{id}` has no UTF-8 file `{src}`")))?;
                let text = agent_file(manifest, body, McpNaming::Plugin).map_err(fail)?;
                let dest = RelPath::new(format!("agents/{to}")).map_err(|e| fail(e.to_string()))?;
                out.files.push((dest, text.into_bytes()));
                for capability in manifest.agent.iter().flat_map(|a| &a.tools) {
                    if let Capability::Mcp { server, .. } = capability
                        && !out.mcp_servers.contains(server)
                    {
                        out.mcp_servers.push(server.clone());
                    }
                }
            }
            (Anchor::Mcp, Strategy::Entry) => {
                let Some(server) = manifest.mcp_server.as_ref() else {
                    continue;
                };
                let config = json!({ "mcpServers": { id.as_str(): stdio_entry(server, None) } });
                let mut text =
                    serde_json::to_string_pretty(&config).map_err(|e| fail(e.to_string()))?;
                text.push('\n');
                let dest = RelPath::new(".mcp.json").map_err(|e| fail(e.to_string()))?;
                out.files.push((dest, text.into_bytes()));
            }
            (anchor, _) => {
                return Err(fail(format!(
                    "`{id}` writes to `{anchor}`, which a plugin can't carry"
                )));
            }
        }
    }
    out.files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Why `name` can't be a plugin name, by Claude Code's reserved-name rules
/// ([`docs/specs.md`, CC-PLUGIN-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cc-plugin-p4-plugin-manifest-components-and-naming)).
/// The whole-word rule is only a warning in `claude plugin validate`, but the
/// marketplace is validated with `--strict`, so it counts.
#[must_use]
pub fn reserved_name(name: &str) -> Option<String> {
    const EXACT: [&str; 5] = [
        "claude",
        "anthropic",
        "anthropics",
        "claude-code",
        "claude-mods",
    ];
    const PREFIXES: [&str; 4] = ["claude-", "anthropic-", "anthropics-", "cc-plugin-"];
    const BRANDS: [&str; 3] = ["claude", "anthropic", "anthropics"];
    let words: Vec<String> = name
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    let joined = words.join("-");
    if EXACT.contains(&joined.as_str()) {
        return Some(format!("`{name}` is a reserved plugin name"));
    }
    if let Some(p) = PREFIXES.iter().find(|p| joined.starts_with(**p)) {
        return Some(format!("`{name}` starts with the reserved prefix `{p}`"));
    }
    let brand = |w: &String| BRANDS.contains(&w.as_str());
    if words.windows(2).any(|pair| {
        (pair[0] == "official" && brand(&pair[1])) || (brand(&pair[0]) && pair[1] == "official")
    }) {
        return Some(format!("`{name}` puts `official` next to a reserved name"));
    }
    if words.iter().any(brand) {
        return Some(format!(
            "`{name}` contains `claude`, `anthropic` or `anthropics` as a word, which `claude plugin validate --strict` refuses"
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_names() {
        for bad in [
            "claude",
            "Anthropic",
            "claude-code",
            "claude-helper",
            "cc-plugin-x",
            "official-claude-tools",
            "claude-official",
            "mcp-for-claude",
            "my_claude_thing",
        ] {
            assert!(reserved_name(bad).is_some(), "{bad}");
        }
        for ok in ["adr-writer", "security-reviewer", "claudette", "grimoire"] {
            assert!(reserved_name(ok).is_none(), "{ok}");
        }
    }
}
