//! What gets released, read from `cargo metadata` (ADR-0002, ADR-0009): the shared
//! version, the published crates in dependency order and every asset binary. Nothing
//! here is a hand-kept list, so a new tool or MCP server is picked up automatically.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

use super::targets::PACKAGE;

/// A binary built from an asset crate under `catalog/tools/` or `catalog/mcp-servers/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AssetBin {
    pub(crate) package: String,
    pub(crate) bin: String,
    /// The asset crate's version, which equals its `asset.toml` version.
    pub(crate) version: String,
    /// The asset directory (holds its `README.md`).
    pub(crate) dir: PathBuf,
}

/// The release as `cargo metadata` describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Release {
    /// The workspace root as Cargo spells it (the paths rustc sees).
    pub(crate) workspace_root: PathBuf,
    /// Cargo's target directory.
    pub(crate) target_dir: PathBuf,
    /// The CLI's version, shared by every published crate.
    pub(crate) version: String,
    /// The CLI crate's `description`.
    pub(crate) description: String,
    /// Every workspace member's directory.
    pub(crate) member_dirs: Vec<PathBuf>,
    /// Crates with `publish` unrestricted, dependencies first.
    pub(crate) crates: Vec<String>,
    pub(crate) assets: Vec<AssetBin>,
}

impl Release {
    /// `v{version}`.
    pub(crate) fn tag(&self) -> String {
        format!("v{}", self.version)
    }
}

/// Runs `cargo metadata` in `root`.
pub(crate) fn load(root: &Path) -> Result<Release, String> {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(root)
        .args(["metadata", "--format-version", "1", "--no-deps", "--locked"])
        .output()
        .map_err(|e| format!("cannot run cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let json: Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("cannot parse cargo metadata: {e}"))?;
    parse(&json)
}

fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("cargo metadata: missing `{key}`"))
}

/// Interprets `cargo metadata --no-deps` output.
pub(crate) fn parse(json: &Value) -> Result<Release, String> {
    let root = PathBuf::from(text(json, "workspace_root")?);
    let packages = json
        .get("packages")
        .and_then(Value::as_array)
        .ok_or("cargo metadata: missing `packages`")?;
    let mut version = None;
    let mut description = String::new();
    let mut member_dirs = Vec::new();
    let mut published = BTreeMap::new();
    let mut assets = Vec::new();
    let tools = root.join("catalog").join("tools");
    let servers = root.join("catalog").join("mcp-servers");
    for p in packages {
        let name = text(p, "name")?;
        let manifest = PathBuf::from(text(p, "manifest_path")?);
        if name == PACKAGE {
            version = Some(text(p, "version")?.to_owned());
            text(p, "description")?.clone_into(&mut description);
        }
        // `publish = false` is `[]`; unrestricted is `null`.
        if p.get("publish").is_some_and(Value::is_null) {
            let deps: BTreeSet<String> = p
                .get("dependencies")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter(|d| d.get("path").is_some())
                .filter(|d| {
                    d.get("kind")
                        .is_none_or(|k| k.as_str().is_none_or(|k| k != "dev"))
                })
                .filter_map(|d| d.get("name").and_then(Value::as_str).map(str::to_owned))
                .collect();
            published.insert(name.to_owned(), deps);
        }
        let dir = manifest
            .parent()
            .ok_or("cargo metadata: bad manifest path")?;
        member_dirs.push(dir.to_path_buf());
        if dir.parent() == Some(tools.as_path()) || dir.parent() == Some(servers.as_path()) {
            for t in p
                .get("targets")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                let is_bin = t
                    .get("kind")
                    .and_then(Value::as_array)
                    .is_some_and(|k| k.iter().any(|k| k == "bin"));
                if is_bin {
                    assets.push(AssetBin {
                        package: name.to_owned(),
                        bin: text(t, "name")?.to_owned(),
                        version: text(p, "version")?.to_owned(),
                        dir: dir.to_path_buf(),
                    });
                }
            }
        }
    }
    let version = version.ok_or_else(|| format!("cargo metadata: no `{PACKAGE}` package"))?;
    if !published.contains_key(PACKAGE) {
        return Err(format!("`{PACKAGE}` must be publishable"));
    }
    assets.sort_by(|a, b| a.bin.cmp(&b.bin));
    if assets.windows(2).any(|w| w[0].bin == w[1].bin) {
        return Err("two asset crates build a binary with the same name".to_owned());
    }
    Ok(Release {
        workspace_root: root,
        target_dir: PathBuf::from(text(json, "target_directory")?),
        version,
        description,
        member_dirs,
        crates: dependency_order(&published)?,
        assets,
    })
}

/// Orders crates so that every crate comes after the published crates it depends on.
fn dependency_order(deps: &BTreeMap<String, BTreeSet<String>>) -> Result<Vec<String>, String> {
    let mut order: Vec<String> = Vec::new();
    let mut left: BTreeMap<&String, BTreeSet<&String>> = deps
        .iter()
        .map(|(k, v)| (k, v.iter().filter(|d| deps.contains_key(*d)).collect()))
        .collect();
    while !left.is_empty() {
        let ready: Vec<&String> = left
            .iter()
            .filter(|(_, d)| d.iter().all(|d| order.contains(d)))
            .map(|(k, _)| *k)
            .collect();
        if ready.is_empty() {
            return Err("published crates depend on each other in a cycle".to_owned());
        }
        for k in ready {
            left.remove(k);
            order.push(k.clone());
        }
    }
    Ok(order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn package(name: &str, path: &str, publish: &Value, deps: &[&str], bins: &[&str]) -> Value {
        json!({
            "name": name,
            "version": "0.3.0",
            "description": "d",
            "manifest_path": format!("/w/{path}/Cargo.toml"),
            "publish": publish,
            "dependencies": deps.iter().map(|d| json!({"name": d, "path": format!("/w/crates/{d}"), "kind": null}))
                .chain([json!({"name": "agent-grimoire-testkit", "path": "/w/crates/t", "kind": "dev"})])
                .collect::<Vec<_>>(),
            "targets": bins.iter().map(|b| json!({"name": b, "kind": ["bin"]}))
                .chain([json!({"name": "lib", "kind": ["lib"]})]).collect::<Vec<_>>(),
        })
    }

    #[test]
    fn finds_version_dependency_order_and_asset_bins() {
        let json = json!({
            "workspace_root": "/w",
            "target_directory": "/w/target",
            "packages": [
                package("agent-grimoire", "crates/agent-grimoire", &Value::Null, &["agent-grimoire-model", "agent-grimoire-catalog"], &["grimoire"]),
                package("agent-grimoire-catalog", "crates/agent-grimoire-catalog", &Value::Null, &["agent-grimoire-model"], &[]),
                package("agent-grimoire-model", "crates/agent-grimoire-model", &Value::Null, &[], &[]),
                package("agent-grimoire-testkit", "crates/agent-grimoire-testkit", &json!([]), &[], &[]),
                package("grimoire-tool-b", "catalog/tools/b", &json!([]), &[], &["grimoire-tool-b"]),
                package("grimoire-mcp-a", "catalog/mcp-servers/a", &json!([]), &[], &["grimoire-mcp-a"]),
                package("xtask", "xtask", &json!([]), &[], &["xtask"]),
            ],
        });
        let r = parse(&json).unwrap();
        assert_eq!(r.version, "0.3.0");
        assert_eq!(r.tag(), "v0.3.0");
        assert_eq!(
            r.crates,
            [
                "agent-grimoire-model",
                "agent-grimoire-catalog",
                "agent-grimoire"
            ]
        );
        let bins: Vec<&str> = r.assets.iter().map(|a| a.bin.as_str()).collect();
        assert_eq!(bins, ["grimoire-mcp-a", "grimoire-tool-b"]);
        assert_eq!(r.assets[0].dir, PathBuf::from("/w/catalog/mcp-servers/a"));
    }

    #[test]
    fn the_real_workspace_parses() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let r = load(root).unwrap();
        assert_eq!(r.crates.last().map(String::as_str), Some(PACKAGE));
        assert_eq!(r.crates.len(), 6, "{:?}", r.crates);
        assert!(
            r.assets
                .iter()
                .any(|a| a.bin == "grimoire-tool-secret-guard")
        );
        assert!(
            r.assets
                .iter()
                .any(|a| a.bin == "grimoire-mcp-project-docs")
        );
    }
}
