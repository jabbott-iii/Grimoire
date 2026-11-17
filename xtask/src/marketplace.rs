//! `cargo xtask gen-marketplace [--check]`: the Claude Code plugin marketplace
//! generated from `catalog/` (`architecture.md` §10; `CC-PLUGIN-P4` and
//! `CC-MKT-P4` in `docs/specs.md`).
//!
//! - Every skill, agent and MCP server that supports `claude-code` becomes a
//!   plugin named after it under `plugins/claude-code/<id>/`.
//! - Every bundle with at least one such member becomes a plugin that depends on
//!   its members' plugins, and says which members it leaves out.
//! - `.claude-plugin/marketplace.json` at the repository root lists them all.
//!
//! Rules and tools are left out (PLAN.md, Phase 4 decisions). Users add the
//! marketplace as `owner/repo` or `https://github.com/<owner>/<repo>`: relative
//! plugin sources don't work when it is added by a raw file URL.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::ExitCode;

use agent_grimoire_adapters::plugin;
use agent_grimoire_catalog::{Source, SourceAsset};
use agent_grimoire_model::{AssetId, Bundle, Kind, Sha256Digest, SourceId};
use serde_json::{Value, json};

/// The marketplace file, relative to the repository root.
pub(crate) const MARKETPLACE: &str = ".claude-plugin/marketplace.json";
/// The generated plugins, relative to the repository root.
pub(crate) const PLUGINS: &str = "plugins/claude-code";
/// The marketplace name users type after `@` (the maintainer's choice).
const NAME: &str = "grimoire";
/// The marketplace owner (the maintainer's choice; no email is published).
const OWNER: &str = "jabbott-iii";

/// Generated files by repository-relative path, and the assets left out.
struct Generated {
    files: BTreeMap<String, Vec<u8>>,
    left_out: Vec<String>,
}

#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
pub(crate) fn run(root: &Path, catalog: &Path, check: bool) -> Result<ExitCode, String> {
    let bytes = match agent_grimoire_catalog::pack(catalog) {
        Ok(bytes) => bytes,
        Err(diagnostics) => {
            for d in &diagnostics {
                eprintln!("{d}");
            }
            eprintln!("error: the catalog is invalid; run `cargo xtask validate`");
            return Ok(ExitCode::FAILURE);
        }
    };
    let source = Source::from_pack(SourceId::Builtin, &bytes)
        .map_err(|e| format!("cannot load the catalog: {e}"))?;
    let generated = match generate(&source) {
        Ok(g) => g,
        Err(problems) => {
            for p in &problems {
                eprintln!("error: {p}");
            }
            return Ok(ExitCode::FAILURE);
        }
    };
    refuse_links(root)?;
    let on_disk = existing(root)?;
    if check {
        let stale: Vec<&String> = generated
            .files
            .iter()
            .filter(|(path, bytes)| on_disk.get(*path) != Some(*bytes))
            .map(|(path, _)| path)
            .chain(on_disk.keys().filter(|p| !generated.files.contains_key(*p)))
            .collect();
        if stale.is_empty() {
            eprintln!(
                "gen-marketplace: {MARKETPLACE} and {PLUGINS}/ are up to date ({} files)",
                generated.files.len()
            );
            return Ok(ExitCode::SUCCESS);
        }
        for path in stale.iter().take(10) {
            eprintln!("  differs: {path}");
        }
        eprintln!("error: the marketplace is out of date. Run `cargo xtask gen-marketplace`.");
        return Ok(ExitCode::FAILURE);
    }
    for path in on_disk.keys().filter(|p| !generated.files.contains_key(*p)) {
        fs::remove_file(root.join(path)).map_err(|e| format!("cannot remove {path}: {e}"))?;
    }
    for (path, bytes) in &generated.files {
        if on_disk.get(path) == Some(bytes) {
            continue;
        }
        let full = root.join(path);
        let dir = full
            .parent()
            .ok_or_else(|| format!("{path} has no parent"))?;
        let name = full
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| format!("{path} has no file name"))?;
        crate::schema::write_atomic(dir, name, bytes)?;
    }
    remove_empty_dirs(&root.join(PLUGINS))?;
    for line in &generated.left_out {
        eprintln!("gen-marketplace: left out {line}");
    }
    eprintln!(
        "gen-marketplace: wrote {MARKETPLACE} and {} files under {PLUGINS}/",
        generated.files.len() - 1
    );
    Ok(ExitCode::SUCCESS)
}

/// Builds every generated file in memory. Deterministic: the same catalog
/// always gives the same bytes.
fn generate(source: &Source) -> Result<Generated, Vec<String>> {
    let mut files = BTreeMap::new();
    let mut entries: Vec<Value> = Vec::new();
    let mut problems = Vec::new();
    let mut left_out = Vec::new();
    let plugins: BTreeSet<&AssetId> = source
        .assets()
        .iter()
        .filter(|(_, a)| plugin::ineligible(&a.manifest).is_none())
        .map(|(id, _)| id)
        .collect();
    for (id, asset) in source.assets() {
        if let Some(reason) = plugin::ineligible(&asset.manifest) {
            left_out.push(format!("`{id}`: {}", reason.describe()));
            continue;
        }
        if let Some(problem) = plugin::reserved_name(id.as_str()) {
            problems.push(problem);
            continue;
        }
        match asset_plugin(source, &plugins, asset, &mut files) {
            Ok(entry) => entries.push(entry),
            Err(problem) => problems.push(problem),
        }
    }
    for (id, bundle) in source.bundles() {
        if source.asset(id).is_some() {
            problems.push(format!(
                "bundle `{id}` has the same name as an asset, so it can't be a plugin too"
            ));
            continue;
        }
        if let Some(problem) = plugin::reserved_name(id.as_str()) {
            problems.push(problem);
            continue;
        }
        match bundle_plugin(source, bundle, &mut files) {
            Some(entry) => entries.push(entry),
            None => left_out.push(format!(
                "bundle `{id}`: none of its members can be a plugin"
            )),
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    entries.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    let marketplace = json!({
        "name": NAME,
        "owner": { "name": OWNER },
        "description": "Grimoire's catalog as Claude Code plugins: skills, subagents and MCP servers. Rules and hook tools are installed with the grimoire CLI.",
        "plugins": entries,
    });
    files.insert(MARKETPLACE.to_owned(), json_bytes(&marketplace));
    Ok(Generated { files, left_out })
}

/// A plugin version that changes whenever the plugin does: the catalog version,
/// plus build metadata from a hash of what the plugin holds. Claude Code keeps a
/// user on a cached copy until the version changes (`CC-PLUGIN-P4`), and plugin
/// content also depends on the generator, not only on the catalog.
fn version(base: &str, parts: &[(&str, &[u8])]) -> String {
    let mut all = Vec::new();
    for (path, bytes) in parts {
        all.extend_from_slice(path.as_bytes());
        all.push(0);
        all.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
        all.extend_from_slice(bytes);
    }
    let digest = Sha256Digest::of(&all);
    let hex = digest.as_str().trim_start_matches("sha256:");
    format!("{base}+{}", &hex[..12])
}

/// Members a plugin needs but that aren't plugins, grouped by what to do.
fn left_out_text(missing: &[(&AssetId, Kind, plugin::Ineligible)]) -> Option<String> {
    let names = |cli: bool| -> Vec<String> {
        missing
            .iter()
            .filter(|(_, _, why)| why.cli_installs() == cli)
            .map(|(id, kind, _)| format!("`{id}` ({})", kind_name(*kind)))
            .collect()
    };
    let (cli, unsupported) = (names(true), names(false));
    let mut parts = Vec::new();
    if !cli.is_empty() {
        parts.push(format!(
            "{} {} not published as a plugin; install {} with the grimoire CLI.",
            and_list(&cli),
            if cli.len() == 1 { "is" } else { "are" },
            if cli.len() == 1 { "it" } else { "them" }
        ));
    }
    if !unsupported.is_empty() {
        parts.push(format!(
            "{} {} support Claude Code.",
            and_list(&unsupported),
            if unsupported.len() == 1 {
                "doesn't"
            } else {
                "don't"
            }
        ));
    }
    (!parts.is_empty()).then(|| parts.join(" "))
}

/// The permissions the catalog declares, for a plugin's README: installing a
/// plugin grants them without the consent `grimoire add` asks for.
fn permissions_text(m: &agent_grimoire_model::AssetManifest) -> String {
    let p = &m.permissions;
    let files = match p.filesystem {
        agent_grimoire_model::FsAccess::None => "none",
        agent_grimoire_model::FsAccess::Read => "read",
        agent_grimoire_model::FsAccess::Write => "read and write",
    };
    let yes = |b: bool| if b { "yes" } else { "no" };
    let env = if p.env.is_empty() {
        "none".to_owned()
    } else {
        p.env
            .iter()
            .map(|e| format!("`{e}`"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    let mut text = format!(
        "\n## Permissions\n\nWhat the catalog declares. Installing this plugin grants it without asking, unlike `grimoire add`, which asks for each permission and each environment variable.\n\n- Files: {files}\n- Network: {}\n- Shell: {}\n- Environment variables read: {env}\n",
        yes(p.network),
        yes(p.shell)
    );
    if let Some(agent) = &m.agent {
        let tools: Vec<String> = agent.tools.iter().map(|t| format!("`{t}`")).collect();
        let tools = if tools.is_empty() {
            "none".to_owned()
        } else {
            tools.join(", ")
        };
        text = format!("{text}- Agent tools: {tools}\n");
    }
    text
}

/// Writes one asset's plugin into `files` and returns its marketplace entry.
fn asset_plugin(
    source: &Source,
    plugins: &BTreeSet<&AssetId>,
    asset: &SourceAsset,
    files: &mut BTreeMap<String, Vec<u8>>,
) -> Result<Value, String> {
    let m = &asset.manifest;
    let id = &m.id;
    let parts = plugin::files(asset).map_err(|d| d.message)?;
    // An agent names its MCP server's tools in the plugin form, which exist only
    // when that server is a plugin too.
    let mut dependencies: BTreeSet<&AssetId> = BTreeSet::new();
    for server in &parts.mcp_servers {
        if !plugins.contains(server) {
            return Err(format!(
                "`{id}` uses the tools of `{server}`, which isn't a plugin, so the plugin agent would have none of them"
            ));
        }
        dependencies.insert(server);
    }
    let mut missing = Vec::new();
    for required in m.requires.keys() {
        if plugins.contains(required) {
            dependencies.insert(required);
        } else if let Some(other) = source.asset(required)
            && let Some(why) = plugin::ineligible(&other.manifest)
        {
            missing.push((required, other.manifest.kind, why));
        }
    }
    let dir = format!("{PLUGINS}/{id}");
    let contents: Vec<(&str, &[u8])> = parts
        .files
        .iter()
        .map(|(p, b)| (p.as_str(), b.as_slice()))
        .collect();
    let mut description = m.summary.as_str().to_owned();
    if let Some(dep) = &m.deprecated {
        description = format!(
            "Deprecated: use `{}` instead ({}). {description}",
            dep.replacement,
            dep.reason.as_str().trim_end_matches('.')
        );
    }
    let needs = left_out_text(&missing);
    if let Some(needs) = &needs {
        description = format!("{description}. It also needs: {needs}");
    }
    let mut manifest = json!({
        "name": id.as_str(),
        "version": version(&m.version.to_string(), &contents),
        "description": description,
        "author": { "name": author(&m.maintainers) },
        "license": m.license.to_string(),
        "keywords": m.tags.iter().map(ToString::to_string).collect::<Vec<_>>(),
    });
    if !dependencies.is_empty() {
        manifest["dependencies"] =
            json!(dependencies.iter().map(|d| d.as_str()).collect::<Vec<_>>());
    }
    for (path, bytes) in parts.files {
        files.insert(format!("{dir}/{path}"), bytes);
    }
    files.insert(
        format!("{dir}/.claude-plugin/plugin.json"),
        json_bytes(&manifest),
    );
    let binary = m
        .mcp_server
        .as_ref()
        .map(|s| binary_note(&[s.binary.as_str()]))
        .unwrap_or_default();
    files.insert(
        format!("{dir}/README.md"),
        readme(
            id.as_str(),
            &description,
            &format!(
                "{binary}{}\nGenerated from [`catalog/{kind}/{id}/`](../../../catalog/{kind}/{id}/README.md) by `cargo xtask gen-marketplace`. Don't edit it here.\n",
                permissions_text(m),
                kind = m.kind.catalog_dir()
            ),
        ),
    );
    Ok(entry(
        id.as_str(),
        &description,
        &m.tags.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ))
}

fn binary_note(binaries: &[&str]) -> String {
    if binaries.is_empty() {
        return String::new();
    }
    let names: Vec<String> = binaries.iter().map(|b| format!("`{b}`")).collect();
    format!(
        "\nInstall the {} {} first: neither Grimoire nor Claude Code installs {}.\n",
        and_list(&names),
        if names.len() == 1 {
            "binary"
        } else {
            "binaries"
        },
        if names.len() == 1 { "it" } else { "them" }
    )
}

/// Writes a bundle's plugin into `files` and returns its marketplace entry, or
/// `None` when none of its members can be a plugin.
fn bundle_plugin(
    source: &Source,
    bundle: &Bundle,
    files: &mut BTreeMap<String, Vec<u8>>,
) -> Option<Value> {
    let id = &bundle.id;
    let mut members = Vec::new();
    let mut missing = Vec::new();
    let mut binaries = Vec::new();
    for member in bundle.assets.keys() {
        let Some(asset) = source.asset(member) else {
            continue;
        };
        match plugin::ineligible(&asset.manifest) {
            None => {
                members.push(member.as_str());
                if let Some(server) = &asset.manifest.mcp_server {
                    binaries.push(server.binary.as_str());
                }
            }
            Some(why) => missing.push((member, asset.manifest.kind, why)),
        }
    }
    if members.is_empty() {
        return None;
    }
    let names: Vec<String> = members.iter().map(|m| format!("`{m}`")).collect();
    let plugins = format!(
        "the {} plugin{}",
        and_list(&names),
        if members.len() == 1 { "" } else { "s" }
    );
    let summary = bundle.summary.as_str().trim_end_matches('.');
    let mut description = match left_out_text(&missing) {
        None => format!("{summary}. Installs {plugins}."),
        Some(text) => format!(
            "Part of Grimoire's `{id}` bundle: installs {plugins}. Left out: {text} The whole bundle: {summary}."
        ),
    };
    if !binaries.is_empty() {
        let names: Vec<String> = binaries.iter().map(|b| format!("`{b}`")).collect();
        description = format!("{description} Needs {} on PATH.", and_list(&names));
    }
    let dir = format!("{PLUGINS}/{id}");
    let deps = members.join("\n");
    let manifest = json!({
        "name": id.as_str(),
        "version": version(&bundle.version.to_string(), &[("dependencies", deps.as_bytes()), ("description", description.as_bytes())]),
        "description": description,
        "author": { "name": author(&bundle.maintainers) },
        "dependencies": members,
    });
    files.insert(
        format!("{dir}/.claude-plugin/plugin.json"),
        json_bytes(&manifest),
    );
    files.insert(
        format!("{dir}/README.md"),
        readme(
            id.as_str(),
            &description,
            &format!(
                "{}\nInstalling this plugin also installs the plugins it depends on.\n\nGenerated from [`catalog/bundles/{id}.toml`](../../../catalog/bundles/{id}.toml) by `cargo xtask gen-marketplace`. Don't edit it here.\n",
                binary_note(&binaries)
            ),
        ),
    );
    Some(entry(id.as_str(), &description, &[]))
}

fn entry(name: &str, description: &str, tags: &[String]) -> Value {
    let mut entry = json!({
        "name": name,
        "source": format!("./{PLUGINS}/{name}"),
        "description": description,
    });
    if !tags.is_empty() {
        entry["tags"] = json!(tags);
    }
    entry
}

/// "a", "a and b", "a, b and c".
fn and_list(items: &[String]) -> String {
    match items.split_last() {
        Some((last, rest)) if !rest.is_empty() => format!("{} and {last}", rest.join(", ")),
        Some((last, _)) => last.clone(),
        None => String::new(),
    }
}

fn readme(name: &str, summary: &str, rest: &str) -> Vec<u8> {
    format!("# {name}\n\n{summary}\n{rest}").into_bytes()
}

fn author(maintainers: &[agent_grimoire_model::GithubLogin]) -> &str {
    maintainers
        .first()
        .map_or(OWNER, agent_grimoire_model::GithubLogin::as_str)
}

fn kind_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Rule => "a rule",
        Kind::Skill => "a skill",
        Kind::Agent => "an agent",
        Kind::Tool => "a tool",
        Kind::McpServer => "an MCP server",
    }
}

fn json_bytes(value: &Value) -> Vec<u8> {
    let mut text = serde_json::to_string_pretty(value).unwrap_or_default();
    text.push('\n');
    text.into_bytes()
}

/// The generator writes only below real directories it owns.
fn refuse_links(root: &Path) -> Result<(), String> {
    for path in [".claude-plugin", "plugins", PLUGINS] {
        if fs::symlink_metadata(root.join(path)).is_ok_and(|m| m.file_type().is_symlink()) {
            return Err(format!(
                "{path} is a symbolic link; refusing to write through it"
            ));
        }
    }
    Ok(())
}

/// The generated files on disk now: the marketplace file and everything under
/// `plugins/claude-code/`.
fn existing(root: &Path) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut found = BTreeMap::new();
    if let Ok(bytes) = fs::read(root.join(MARKETPLACE)) {
        found.insert(MARKETPLACE.to_owned(), bytes);
    }
    let mut stack = vec![root.join(PLUGINS)];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries {
            let entry = entry.map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
            let path = entry.path();
            let kind = entry
                .file_type()
                .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
            if kind.is_symlink() {
                return Err(format!("{} is a symbolic link", path.display()));
            } else if kind.is_dir() {
                stack.push(path);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .map_err(|e| e.to_string())?
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().into_owned())
                    .collect::<Vec<_>>()
                    .join("/");
                let bytes = fs::read(&path).map_err(|e| format!("cannot read {rel}: {e}"))?;
                found.insert(rel, bytes);
            }
        }
    }
    Ok(found)
}

fn remove_empty_dirs(dir: &Path) -> Result<bool, String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(false);
    };
    let mut empty = true;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
            empty &= remove_empty_dirs(&entry.path())?;
        } else {
            empty = false;
        }
    }
    if empty {
        fs::remove_dir(dir).map_err(|e| format!("cannot remove {}: {e}", dir.display()))?;
    }
    Ok(empty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_builtin_catalog_generates() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let bytes = agent_grimoire_catalog::pack(&root.join("catalog")).unwrap();
        let source = Source::from_pack(SourceId::Builtin, &bytes).unwrap();
        let generated = generate(&source).unwrap();
        let marketplace: Value = serde_json::from_slice(&generated.files[MARKETPLACE]).unwrap();
        assert_eq!(marketplace["name"], NAME);
        for entry in marketplace["plugins"].as_array().unwrap() {
            let name = entry["name"].as_str().unwrap();
            assert!(
                generated
                    .files
                    .contains_key(&format!("{PLUGINS}/{name}/.claude-plugin/plugin.json")),
                "{name}"
            );
            assert!(entry["source"].as_str().unwrap().starts_with("./"));
        }
        // Rules and hook tools are never plugins.
        assert!(
            generated
                .left_out
                .iter()
                .any(|l| l.contains("conventional-commits"))
        );
        assert!(
            generated
                .left_out
                .iter()
                .any(|l| l.contains("secret-guard"))
        );
    }

    fn fixture() -> agent_grimoire_testkit::FixtureCatalog {
        let c = agent_grimoire_testkit::FixtureCatalog::new();
        c.mcp_server("docs-index", "2.0.0", &["DOCS_TOKEN"])
            .asset(
                "agent",
                "doc-reader",
                "1.0.0",
                "[requires]\ndocs-index = \"^2\"\n[permissions]\nfilesystem = \"read\"\n[agent]\ntools = [\"read\", \"mcp:docs-index/search\"]\n[clients.claude-code]\noutputs = [{ src = \"agent.md\", to = \"agents:doc-reader.md\", strategy = \"render\" }]\n",
                &[("agent.md", "You read the docs.\n")],
            )
            .rule("style", "1.0.0", &[], "# Style\n")
            .bundle("docs", &[("doc-reader", "^1"), ("style", "^1")]);
        c
    }

    #[test]
    fn agents_depend_on_their_servers_and_name_plugin_tools() {
        let c = fixture();
        let generated = generate(&c.source()).unwrap();
        let manifest: Value = serde_json::from_slice(
            &generated.files[&format!("{PLUGINS}/doc-reader/.claude-plugin/plugin.json")],
        )
        .unwrap();
        assert_eq!(manifest["dependencies"], json!(["docs-index"]));
        assert!(manifest["version"].as_str().unwrap().starts_with("1.0.0+"));
        let agent = String::from_utf8(
            generated.files[&format!("{PLUGINS}/doc-reader/agents/doc-reader.md")].clone(),
        )
        .unwrap();
        assert!(
            agent.contains("\"mcp__plugin_docs-index_docs-index__search\""),
            "{agent}"
        );
        let bundle: Value = serde_json::from_slice(
            &generated.files[&format!("{PLUGINS}/docs/.claude-plugin/plugin.json")],
        )
        .unwrap();
        assert_eq!(bundle["dependencies"], json!(["doc-reader"]));
        let description = bundle["description"].as_str().unwrap();
        assert!(description.contains("`style` (a rule)"), "{description}");
    }

    #[test]
    fn an_agent_whose_server_is_no_plugin_is_refused() {
        let c = fixture();
        // Without a claude-code table the server is no plugin, so the agent's
        // plugin-named tools would not exist.
        c.edit_manifest("mcp-servers/docs-index", |m| {
            m.replace("[clients.claude-code]", "[clients.codex]")
        });
        let problems = generate(&c.source()).err().unwrap();
        assert!(
            problems.iter().any(|p| p.contains("isn't a plugin")),
            "{problems:?}"
        );
    }

    #[test]
    fn plugin_versions_change_with_content() {
        let a = version("1.0.0", &[("skills/x/SKILL.md", b"one")]);
        let b = version("1.0.0", &[("skills/x/SKILL.md", b"two")]);
        assert_ne!(a, b);
        assert!(
            a.starts_with("1.0.0+") && a.len() == "1.0.0+".len() + 12,
            "{a}"
        );
        assert_eq!(a, version("1.0.0", &[("skills/x/SKILL.md", b"one")]));
    }

    #[test]
    fn check_and_write_touch_only_the_marketplace() {
        let c = fixture();
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let catalog = c.root();
        // Nothing generated yet: --check fails, writing creates everything.
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::FAILURE);
        assert_eq!(run(root, catalog, false).unwrap(), ExitCode::SUCCESS);
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::SUCCESS);
        // A stale file, an extra file and a missing file each fail --check.
        let readme = root.join(PLUGINS).join("docs/README.md");
        let original = fs::read(&readme).unwrap();
        fs::write(&readme, "edited\n").unwrap();
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::FAILURE);
        fs::write(&readme, &original).unwrap();
        let extra = root.join(PLUGINS).join("gone/README.md");
        fs::create_dir_all(extra.parent().unwrap()).unwrap();
        fs::write(&extra, "old plugin\n").unwrap();
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::FAILURE);
        fs::remove_file(root.join(MARKETPLACE)).unwrap();
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::FAILURE);
        // Writing removes the stale plugin and its folder, and nothing else.
        fs::write(root.join("keep.md"), "mine\n").unwrap();
        fs::create_dir_all(root.join("plugins/other")).unwrap();
        fs::write(root.join("plugins/other/x.md"), "mine\n").unwrap();
        assert_eq!(run(root, catalog, false).unwrap(), ExitCode::SUCCESS);
        assert!(!root.join(PLUGINS).join("gone").exists());
        assert!(root.join("keep.md").exists() && root.join("plugins/other/x.md").exists());
        assert_eq!(run(root, catalog, true).unwrap(), ExitCode::SUCCESS);
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_links_are_refused() {
        let c = fixture();
        let root = tempfile::tempdir().unwrap();
        let elsewhere = tempfile::tempdir().unwrap();
        std::os::unix::fs::symlink(elsewhere.path(), root.path().join("plugins")).unwrap();
        assert!(run(root.path(), c.root(), false).is_err());
        assert_eq!(fs::read_dir(elsewhere.path()).unwrap().count(), 0);
    }
}
