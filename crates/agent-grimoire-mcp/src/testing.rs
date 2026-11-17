//! Test helpers for MCP server assets.
//!
//! Catalog validation requires every server's declared tools to match what it
//! actually serves (`architecture.md` §6.4). Each server's tests call
//! [`declared_tools_match`] with its own `asset.toml` and its live `tools/list`.

use std::collections::BTreeMap;

use rmcp::model::Tool;
use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "mcp-server")]
    mcp_server: Server,
}

#[derive(Deserialize)]
struct Server {
    tools: Vec<Declared>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(
    clippy::struct_excessive_bools,
    reason = "mirrors the four boolean MCP tool annotations"
)]
struct Declared {
    name: String,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    open_world: bool,
}

/// Checks that `tools` (a live `tools/list`) is exactly the set `asset_toml`
/// declares under `[mcp-server] tools`, and that every annotation is stated
/// explicitly and agrees.
///
/// # Errors
///
/// Returns every difference, one per line.
pub fn declared_tools_match(asset_toml: &str, tools: &[Tool]) -> Result<(), String> {
    let manifest: Manifest = toml::from_str(asset_toml).map_err(|e| format!("asset.toml: {e}"))?;
    let declared: BTreeMap<&str, &Declared> = manifest
        .mcp_server
        .tools
        .iter()
        .map(|t| (t.name.as_str(), t))
        .collect();
    let live: BTreeMap<&str, &Tool> = tools.iter().map(|t| (t.name.as_ref(), t)).collect();
    let mut problems = Vec::new();
    for name in declared.keys().filter(|n| !live.contains_key(*n)) {
        problems.push(format!("`{name}` is declared but not served"));
    }
    for name in live.keys().filter(|n| !declared.contains_key(*n)) {
        problems.push(format!("`{name}` is served but not declared"));
    }
    for (name, d) in &declared {
        let Some(tool) = live.get(name) else { continue };
        let Some(a) = &tool.annotations else {
            problems.push(format!("`{name}` has no annotations"));
            continue;
        };
        for (hint, served, wanted) in [
            ("readOnlyHint", a.read_only_hint, d.read_only),
            ("destructiveHint", a.destructive_hint, d.destructive),
            ("idempotentHint", a.idempotent_hint, d.idempotent),
            ("openWorldHint", a.open_world_hint, d.open_world),
        ] {
            match served {
                Some(v) if v == wanted => {}
                Some(v) => problems.push(format!(
                    "`{name}` serves {hint} = {v}, but asset.toml says {wanted}"
                )),
                None => problems.push(format!("`{name}` does not state {hint} explicitly")),
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASSET: &str = r#"
[mcp-server]
tools = [
    { name = "a", read-only = true, destructive = false, idempotent = true, open-world = false },
]
"#;

    fn tool(name: &str, hints: Option<[Option<bool>; 4]>) -> Tool {
        let mut value = serde_json::json!({ "name": name, "inputSchema": { "type": "object" } });
        if let Some([r, d, i, o]) = hints {
            value["annotations"] = serde_json::json!({
                "readOnlyHint": r, "destructiveHint": d, "idempotentHint": i, "openWorldHint": o
            });
        }
        serde_json::from_value(value).unwrap()
    }

    const EXACT: [Option<bool>; 4] = [Some(true), Some(false), Some(true), Some(false)];

    #[test]
    fn an_exact_match_passes() {
        assert_eq!(
            declared_tools_match(ASSET, &[tool("a", Some(EXACT))]),
            Ok(())
        );
    }

    #[test]
    fn every_kind_of_difference_is_reported() {
        let missing = declared_tools_match(ASSET, &[]).unwrap_err();
        assert!(missing.contains("declared but not served"), "{missing}");
        let extra = declared_tools_match(ASSET, &[tool("a", Some(EXACT)), tool("b", Some(EXACT))])
            .unwrap_err();
        assert!(extra.contains("served but not declared"), "{extra}");
        let bare = declared_tools_match(ASSET, &[tool("a", None)]).unwrap_err();
        assert!(bare.contains("has no annotations"), "{bare}");
        for i in 0..4 {
            let mut flipped = EXACT;
            flipped[i] = flipped[i].map(|v| !v);
            let err = declared_tools_match(ASSET, &[tool("a", Some(flipped))]).unwrap_err();
            assert!(err.contains("but asset.toml says"), "hint {i}: {err}");
            let mut unstated = EXACT;
            unstated[i] = None;
            let err = declared_tools_match(ASSET, &[tool("a", Some(unstated))]).unwrap_err();
            assert!(err.contains("does not state"), "hint {i}: {err}");
        }
        assert!(declared_tools_match("not toml", &[]).is_err());
    }
}
