//! Checks for code-bearing assets: tools and MCP servers are Rust crates (ADR-0002).

use std::path::Path;

use agent_grimoire_model::{AssetManifest, Diagnostic};

use agent_grimoire_model::RelPath;

use crate::fs::{Tree, read_text};

/// Target and build-script paths must be portable paths inside the asset
/// directory, so a crate cannot compile sources from elsewhere in the repository.
fn target_paths(table: &toml::Table, package: &toml::Table, file: &str, out: &mut Vec<Diagnostic>) {
    let mut paths: Vec<(String, &str)> = Vec::new();
    if let Some(build) = package.get("build").and_then(toml::Value::as_str) {
        paths.push(("package.build".to_owned(), build));
    }
    if let Some(path) = table
        .get("lib")
        .and_then(|l| l.get("path"))
        .and_then(toml::Value::as_str)
    {
        paths.push(("lib.path".to_owned(), path));
    }
    for kind in ["bin", "test", "bench", "example"] {
        for (i, target) in table
            .get(kind)
            .and_then(toml::Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
        {
            if let Some(path) = target.get("path").and_then(toml::Value::as_str) {
                paths.push((format!("{kind}[{i}].path"), path));
            }
        }
    }
    for (field, path) in paths {
        if let Err(e) = RelPath::new(path) {
            out.push(
                Diagnostic::error(
                    "crate/path",
                    format!(
                        "`{path}` must stay inside the asset directory: {}",
                        e.reason()
                    ),
                )
                .at(field)
                .in_file(file.to_owned()),
            );
        }
    }
}

pub(crate) fn check(
    dir: &Path,
    tree: &Tree,
    manifest: &AssetManifest,
    rel_dir: &str,
    out: &mut Vec<Diagnostic>,
) {
    let file = format!("{rel_dir}/Cargo.toml");
    let binary = manifest
        .tool
        .as_ref()
        .map(|t| t.binary.as_str())
        .or_else(|| manifest.mcp_server.as_ref().map(|m| m.binary.as_str()));
    let Some(binary) = binary else {
        // The missing kind table is already reported by manifest validation.
        return;
    };
    if !tree.is_file("Cargo.toml") {
        out.push(
            Diagnostic::error(
                "crate/missing",
                "tools and MCP servers are Rust crates and need a Cargo.toml",
            )
            .in_file(rel_dir.to_owned()),
        );
        return;
    }
    let table: toml::Table = match read_text(&dir.join("Cargo.toml"))
        .and_then(|t| toml::from_str(&t).map_err(|e| e.to_string()))
    {
        Ok(table) => table,
        Err(e) => {
            out.push(Diagnostic::error("crate/parse", e).in_file(file));
            return;
        }
    };
    let Some(package) = table.get("package").and_then(toml::Value::as_table) else {
        out.push(
            Diagnostic::error("crate/package", "Cargo.toml needs a [package] table").in_file(file),
        );
        return;
    };
    if package.get("name").and_then(toml::Value::as_str) != Some(binary) {
        out.push(
            Diagnostic::error(
                "crate/name",
                format!("package.name must equal the binary name `{binary}`"),
            )
            .at("package.name")
            .in_file(&file),
        );
    }
    let version = manifest.version.to_string();
    match package.get("version").and_then(toml::Value::as_str) {
        Some(v) if v == version => {}
        Some(v) => out.push(
            Diagnostic::error(
                "crate/version",
                format!("package.version `{v}` must equal the asset version `{version}`"),
            )
            .at("package.version")
            .in_file(&file),
        ),
        None => out.push(
            Diagnostic::error(
                "crate/version",
                "package.version must be a literal version equal to the asset version, not inherited from the workspace",
            )
            .at("package.version")
            .in_file(&file),
        ),
    }
    if package.get("publish").and_then(toml::Value::as_bool) != Some(false) {
        out.push(
            Diagnostic::error(
                "crate/publish",
                "asset crates are never published to crates.io (ADR-0002): set `publish = false`",
            )
            .at("package.publish")
            .in_file(&file),
        );
    }
    let bins = table.get("bin").and_then(toml::Value::as_array);
    let explicit_ok = bins.is_none_or(|bins| {
        bins.len() == 1 && bins[0].get("name").and_then(toml::Value::as_str) == Some(binary)
    });
    if !explicit_ok || tree.is_dir("src/bin") {
        out.push(
            Diagnostic::error(
                "crate/bin",
                format!("the crate must build exactly one binary, `{binary}`"),
            )
            .at("bin")
            .in_file(&file),
        );
    }
    target_paths(&table, package, &file, out);
    let has_explicit_path = bins.is_some_and(|b| b.iter().any(|t| t.get("path").is_some()));
    if !has_explicit_path && !tree.is_file("src/main.rs") {
        out.push(
            Diagnostic::error(
                "crate/main",
                "the binary's entry point src/main.rs is missing",
            )
            .in_file(rel_dir.to_owned()),
        );
    }
}
