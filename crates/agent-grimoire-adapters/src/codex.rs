//! The `codex` adapter.
//!
//! What it writes was decided in
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md),
//! the record of what the Codex, Cursor, Gemini CLI and Copilot adapters write.
//! The Codex facts it relies on are in the Codex entries of `docs/specs.md`,
//! starting at
//! [CODEX-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#codex-p4-releases-agentsmd-and-project-trust).
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `instructions` | `AGENTS.md` | the same managed block as `agents-md` |
//! | `skills:` | `.agents/skills/<path>` | a byte-for-byte copy |
//! | `mcp` | `.codex/config.toml` | `[mcp_servers.<id>]`: `command` and `env_vars`, or `url` |
//!
//! Codex has no `agents` or `hooks` anchor, as
//! [ADR-0008](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md)
//! decided: a Codex agent inherits the parent's sandbox, MCP servers and web
//! search, so its tools can't be restricted, and every Codex hook fails open
//! (a hook that fails lets the action through).
//!
//! Codex reads `.codex/config.toml`, and so its MCP servers, only in projects
//! the user has marked trusted.

use agent_grimoire_install::entry::member_key;
use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{
    Anchor, ClientId, Diagnostic, OutputSpec, RelPath, Strategy, Transport,
};
use agent_grimoire_resolve::Resolved;
use serde_json::{Map, Value, json};

use crate::{Report, agents_md, copy_tree, env_remap, no_destination};

/// Codex's project configuration file, where MCP servers live.
pub const CONFIG: &str = ".codex/config.toml";
const SKILLS: &str = ".agents/skills";

/// Renders one declared output.
pub(crate) fn render(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Vec<(RelPath, Content)> {
    match (spec.to.anchor(), spec.strategy) {
        (Anchor::Instructions, Strategy::Block) => agents_md::block(resolved, spec, report)
            .into_iter()
            .collect(),
        (Anchor::Skills, Strategy::Copy) => copy_tree(resolved, spec, SKILLS),
        (Anchor::Mcp, Strategy::Entry) => mcp(resolved, report).into_iter().collect(),
        (anchor, _) => {
            no_destination(report, ClientId::Codex, resolved, anchor);
            Vec::new()
        }
    }
}

/// `[mcp_servers.<id>]` ([`docs/specs.md`, CODEX-MCP-P4](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#codex-mcp-p4-mcp_serversid-in-codexconfigtoml)).
///
/// A stdio server starts with an almost empty environment: Codex passes only a
/// small allowlist plus the variables named in `env_vars`, each under its own
/// name, and skips any that are unset. Codex has no `${VAR}` syntax, so a
/// variable can't be forwarded under another name.
fn mcp(resolved: &Resolved<'_>, report: &mut Report) -> Option<(RelPath, Content)> {
    let manifest = &resolved.asset.manifest;
    let id = &manifest.id;
    let server = manifest.mcp_server.as_ref()?;
    let options = resolved.options.as_ref();
    let key = member_key(&["mcp_servers", id.as_str()]).ok()?;
    let mut entry = Map::new();
    if let Some(Transport::StreamableHttp) = options.and_then(|o| o.transport) {
        // `grimoire.toml` validation guarantees the URL is present. Sign-in is
        // `codex mcp login <id>`; Grimoire writes no credentials.
        entry.insert(
            "url".into(),
            json!(options.and_then(|o| o.url.as_ref())?.as_str()),
        );
    } else {
        if let Some(remap) = env_remap(resolved, report)
            && let Some((name, to)) = remap.iter().find(|(name, to)| name != to)
        {
            report.errors.push(
                Diagnostic::error(
                    "adapter/env-remap",
                    format!("codex can't pass `{to}` to `{id}` as `{name}`: Codex forwards a variable only under its own name; set `{name}` itself, or leave codex out of this asset's `clients`"),
                )
                .at(format!("assets.{id}.env"))
                .in_file("grimoire.toml"),
            );
            return None;
        }
        entry.insert("command".into(), json!(server.binary.as_str()));
        if !server.env.is_empty() {
            entry.insert(
                "env_vars".into(),
                Value::Array(server.env.iter().map(|v| json!(v.name.as_str())).collect()),
            );
        }
    }
    Some((
        RelPath::new(CONFIG).ok()?,
        Content::Entry {
            key,
            value: Value::Object(entry),
        },
    ))
}
