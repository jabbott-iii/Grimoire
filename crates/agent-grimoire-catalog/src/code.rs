//! Checks for code-bearing assets: tools and MCP servers are Rust crates, each
//! built as its own binary and released with the CLI
//! ([ADR-0002](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0002-mcp-server-and-tool-packaging.md)).

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

/// Core crates that asset crates must not depend on
/// ([`docs/architecture.md` §4, Repository layout](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#4-repository-layout)).
const CORE_CRATES: [&str; 7] = [
    "agent-grimoire",
    "agent-grimoire-adapters",
    "agent-grimoire-catalog",
    "agent-grimoire-install",
    "agent-grimoire-model",
    "agent-grimoire-resolve",
    "agent-grimoire-testkit",
];
/// The shared hardening crate every MCP server uses.
/// [ADR-0002](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0002-mcp-server-and-tool-packaging.md)
/// puts the shared server hardening in one core crate, and
/// [ADR-0007](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0007-mcp-server-authorization.md)
/// chose its authorization and HTTP checks.
const MCP_HARDENING: &str = "agent-grimoire-mcp";

/// Every dependency in `[dependencies]`, `[dev-dependencies]`,
/// `[build-dependencies]`, their deprecated `_` spellings and their
/// `[target.*]` forms: the table it is in, the crate it names, and its spec.
fn dependency_names(table: &toml::Table) -> Vec<(String, String, toml::Value)> {
    let kinds = [
        "dependencies",
        "dev-dependencies",
        "build-dependencies",
        "dev_dependencies",
        "build_dependencies",
    ];
    let mut out = Vec::new();
    let mut scan = |prefix: &str, t: &toml::Table| {
        for kind in kinds {
            if let Some(deps) = t.get(kind).and_then(toml::Value::as_table) {
                for (name, spec) in deps {
                    // `alias = { package = "real-name" }` depends on `real-name`.
                    let real = spec
                        .get("package")
                        .and_then(toml::Value::as_str)
                        .unwrap_or(name);
                    out.push((format!("{prefix}{kind}"), real.to_owned(), spec.clone()));
                }
            }
        }
    };
    scan("", table);
    if let Some(targets) = table.get("target").and_then(toml::Value::as_table) {
        for (cfg, t) in targets {
            if let Some(t) = t.as_table() {
                scan(&format!("target.{cfg}."), t);
            }
        }
    }
    out
}

fn dependencies(
    table: &toml::Table,
    manifest: &AssetManifest,
    file: &str,
    out: &mut Vec<Diagnostic>,
) {
    let deps = dependency_names(table);
    for (section, name, spec) in &deps {
        // A path or git dependency can bring in crates this check never sees,
        // such as a helper crate that depends on the installer.
        if spec.get("path").is_some() || spec.get("git").is_some() {
            out.push(
                Diagnostic::error(
                    "crate/path-dependency",
                    format!("depend on `{name}` through the workspace (`{name}.workspace = true`) or crates.io, not a path or git source"),
                )
                .at(section.clone())
                .in_file(file.to_owned()),
            );
        }
        if CORE_CRATES.contains(&name.as_str()) {
            out.push(
                Diagnostic::error(
                    "crate/core-dependency",
                    format!("asset crates must not depend on Grimoire's core crate `{name}`"),
                )
                .at(section.clone())
                .in_file(file.to_owned()),
            );
        }
    }
    if manifest.mcp_server.is_some()
        && !deps
            .iter()
            .any(|(section, name, _)| section == "dependencies" && name == MCP_HARDENING)
    {
        out.push(
            Diagnostic::error(
                "crate/hardening",
                format!("MCP servers serve through `{MCP_HARDENING}`, so they get its secure transport defaults: add it to [dependencies]"),
            )
            .at("dependencies")
            .in_file(file.to_owned()),
        );
    }
}

/// Text that shows a server building rmcp's transports itself instead of
/// serving through the hardening crate.
const RAW_TRANSPORTS: [&str; 3] = [
    "StreamableHttpService",
    "rmcp::transport::stdio",
    "transport::io::stdio",
];

/// Source-level rules for MCP servers. These are text checks that catch the
/// common mistakes; review is still what guarantees them.
fn server_sources(dir: &Path, tree: &Tree, rel_dir: &str, out: &mut Vec<Diagnostic>) {
    let rust_files = |prefix: &str| -> Vec<(String, String)> {
        let Ok(prefix) = RelPath::new(prefix) else {
            return Vec::new();
        };
        tree.files_under(&prefix)
            .filter(|p| {
                Path::new(p.as_str())
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
            })
            .filter_map(|p| {
                read_text(&dir.join(p.as_str()))
                    .ok()
                    .map(|text| (p.to_string(), text))
            })
            .collect()
    };
    for (path, text) in rust_files("src") {
        if let Some(raw) = RAW_TRANSPORTS.iter().find(|r| text.contains(**r)) {
            out.push(
                Diagnostic::error(
                    "crate/raw-transport",
                    format!("`{raw}` serves MCP without the hardening defaults: use `agent_grimoire_mcp::serve_stdio` and `HttpServer`"),
                )
                .in_file(format!("{rel_dir}/{path}")),
            );
        }
    }
    if !rust_files("tests")
        .iter()
        .any(|(_, text)| text.contains("declared_tools_match"))
    {
        out.push(
            Diagnostic::error(
                "crate/declared-tools-test",
                "an MCP server's tests must compare its live tools with asset.toml: call `agent_grimoire_mcp::testing::declared_tools_match`",
            )
            .in_file(format!("{rel_dir}/tests")),
        );
    }
}

/// `[package]` fields that must agree with the asset: name, version and `publish`.
fn package_fields(
    package: &toml::Table,
    binary: &str,
    manifest: &AssetManifest,
    file: &str,
    out: &mut Vec<Diagnostic>,
) {
    if package.get("name").and_then(toml::Value::as_str) != Some(binary) {
        out.push(
            Diagnostic::error(
                "crate/name",
                format!("package.name must equal the binary name `{binary}`"),
            )
            .at("package.name")
            .in_file(file),
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
            .in_file(file),
        ),
        None => out.push(
            Diagnostic::error(
                "crate/version",
                "package.version must be a literal version equal to the asset version, not inherited from the workspace",
            )
            .at("package.version")
            .in_file(file),
        ),
    }
    if package.get("publish").and_then(toml::Value::as_bool) != Some(false) {
        out.push(
            Diagnostic::error(
                "crate/publish",
                "asset crates are never published to crates.io (ADR-0002): set `publish = false`",
            )
            .at("package.publish")
            .in_file(file),
        );
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
    for nested in tree
        .entries
        .keys()
        .filter(|p| p.file_name() == "Cargo.toml" && p.as_str() != "Cargo.toml")
    {
        out.push(
            Diagnostic::error(
                "crate/nested-manifest",
                "an asset is exactly one crate: a second Cargo.toml would add a workspace member that validation does not check",
            )
            .in_file(format!("{rel_dir}/{nested}")),
        );
    }
    if manifest.mcp_server.is_some() {
        server_sources(dir, tree, rel_dir, out);
    }
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
    package_fields(package, binary, manifest, &file, out);
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
    dependencies(&table, manifest, &file, out);
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
