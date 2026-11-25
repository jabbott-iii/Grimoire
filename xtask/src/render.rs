//! Part of `cargo xtask validate`: renders every asset for every client it
//! declares, through the real adapters, so that a refusal only an adapter makes
//! fails here rather than in a user's `grimoire apply`. Examples are a Cursor agent
//! whose tools aren't exactly `read` and `search`, a comma in a Cursor glob, or an
//! MCP tool name a client would rename (`docs/subsystems/catalog.md`, Catalog validation).

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use agent_grimoire_catalog::Source;
use agent_grimoire_model::{Diagnostic, SourceId};
use agent_grimoire_resolve::{Resolution, Resolved};

/// Diagnostics for every asset and client that doesn't render. Each asset is
/// rendered alone for one client, which is the case its own client table
/// promises; sharing with other clients can only remove a refusal.
pub(crate) fn check(catalog_dir: &Path, display: &str) -> Vec<Diagnostic> {
    let source = match agent_grimoire_catalog::pack(catalog_dir)
        .map_err(|_| "the catalog does not pack".to_owned())
        .and_then(|bytes| Source::from_pack(SourceId::Builtin, &bytes).map_err(|e| e.to_string()))
    {
        Ok(source) => source,
        Err(e) => return vec![Diagnostic::error("catalog/render", e)],
    };
    let mut out = Vec::new();
    for (id, asset) in source.assets() {
        let manifest = &asset.manifest;
        let file = format!("{display}/{}/{id}/asset.toml", manifest.kind.catalog_dir());
        for client in manifest.clients.keys() {
            let resolution = Resolution {
                assets: BTreeMap::from([(
                    id.clone(),
                    Resolved {
                        asset,
                        selected_by: BTreeSet::new(),
                        allow: BTreeSet::new(),
                        clients: BTreeSet::from([*client]),
                        options: None,
                    },
                )]),
                warnings: Vec::new(),
            };
            if let Err(diagnostics) = agent_grimoire_adapters::render(&resolution) {
                out.extend(
                    diagnostics
                        .into_iter()
                        .filter(Diagnostic::is_error)
                        .map(|d| {
                            Diagnostic::error(d.code, format!("for {client}: {}", d.message))
                                .at(format!("clients.{client}"))
                                .in_file(&file)
                        }),
                );
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adapter_refusals_fail_validation() {
        let c = agent_grimoire_testkit::FixtureCatalog::new();
        c.agent("editor", "1.0.0", &["read", "edit"]);
        c.edit_manifest("agents/editor", |m| {
            m + "\n[clients.cursor]\noutputs = [{ src = \"agent.md\", to = \"agents:editor.md\", strategy = \"render\" }]\n"
        });
        let diagnostics = check(c.root(), "catalog");
        assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
        assert_eq!(diagnostics[0].code, "adapter/unsupported");
        assert!(diagnostics[0].message.starts_with("for cursor: "));
        assert_eq!(
            diagnostics[0].file.as_deref(),
            Some("catalog/agents/editor/asset.toml")
        );
    }
}
