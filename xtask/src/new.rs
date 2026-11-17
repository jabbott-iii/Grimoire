//! `cargo xtask new <kind> <id>`: scaffold a catalog asset or bundle.
//!
//! The templates under `xtask/templates/<kind>/` follow the Phase 3 exemplars
//! (`architecture.md` §4). Each one validates cleanly as generated, so a new
//! asset starts from a passing state and the author fills in its content.
//! Placeholders: `@@ID@@` (the kebab-case id), `@@SNAKE@@` (the id with `_`),
//! `@@BINARY@@` (`grimoire-tool-<id>` or `grimoire-mcp-<id>`) and
//! `@@MAINTAINER@@` (a GitHub login).

use std::fs;
use std::path::{Path, PathBuf};

use agent_grimoire_model::{AssetId, GithubLogin, Kind};

/// One template file: its path inside the asset directory and its text.
type Template = (&'static str, &'static str);

macro_rules! templates {
    ($kind:literal: $($path:literal),+ $(,)?) => {
        &[$(($path, include_str!(concat!("../templates/", $kind, "/", $path)))),+]
    };
}

const RULE: &[Template] = templates!("rule":
    "asset.toml", "rule.md", "README.md", "CHANGELOG.md", "evals/cases.md",
);
const SKILL: &[Template] = templates!("skill":
    "asset.toml", "SKILL.md", "README.md", "CHANGELOG.md", "evals/cases.md",
);
const AGENT: &[Template] = templates!("agent":
    "asset.toml", "agent.md", "README.md", "CHANGELOG.md", "evals/cases.md",
);
const TOOL: &[Template] = templates!("tool":
    "asset.toml", "Cargo.toml", "src/main.rs", "tests/cli.rs", "README.md", "CHANGELOG.md",
);
const MCP_SERVER: &[Template] = templates!("mcp-server":
    "asset.toml", "Cargo.toml", "src/main.rs", "tests/protocol.rs", "README.md", "CHANGELOG.md",
);

/// What to scaffold.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum What {
    /// An asset of this kind.
    Asset(Kind),
    /// A bundle.
    Bundle,
}

impl What {
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        Ok(match value {
            "rule" => What::Asset(Kind::Rule),
            "skill" => What::Asset(Kind::Skill),
            "agent" => What::Asset(Kind::Agent),
            "tool" => What::Asset(Kind::Tool),
            "mcp-server" => What::Asset(Kind::McpServer),
            "bundle" => What::Bundle,
            other => {
                return Err(format!(
                    "unknown kind `{other}`: use rule, skill, agent, tool, mcp-server or bundle"
                ));
            }
        })
    }
}

fn templates(kind: Kind) -> &'static [Template] {
    match kind {
        Kind::Rule => RULE,
        Kind::Skill => SKILL,
        Kind::Agent => AGENT,
        Kind::Tool => TOOL,
        Kind::McpServer => MCP_SERVER,
    }
}

fn substitute(text: &str, id: &AssetId, kind: Kind, maintainer: &GithubLogin) -> String {
    let binary = match kind {
        Kind::Tool => format!("grimoire-tool-{id}"),
        Kind::McpServer => format!("grimoire-mcp-{id}"),
        _ => String::new(),
    };
    text.replace("@@ID@@", id.as_str())
        .replace("@@SNAKE@@", &id.as_str().replace('-', "_"))
        .replace("@@BINARY@@", &binary)
        .replace("@@MAINTAINER@@", maintainer.as_str())
}

/// Whether `id` is already taken by any asset or bundle: ids are unique across
/// the whole catalog (`architecture.md` §5).
fn taken(catalog: &Path, id: &AssetId) -> Option<PathBuf> {
    let mut candidates: Vec<PathBuf> = [
        Kind::Rule,
        Kind::Skill,
        Kind::Agent,
        Kind::Tool,
        Kind::McpServer,
    ]
    .iter()
    .map(|k| catalog.join(k.catalog_dir()).join(id.as_str()))
    .collect();
    candidates.push(catalog.join("bundles").join(format!("{id}.toml")));
    candidates
        .into_iter()
        .find(|p| fs::symlink_metadata(p).is_ok())
}

/// Writes the files of a new asset or bundle and returns their paths, relative
/// to `catalog`.
pub(crate) fn scaffold(
    catalog: &Path,
    what: What,
    id: &AssetId,
    maintainer: &GithubLogin,
    members: &[String],
) -> Result<Vec<String>, String> {
    if let Some(existing) = taken(catalog, id) {
        return Err(format!(
            "`{id}` is already used by {}; asset and bundle ids are unique across the catalog",
            existing.display()
        ));
    }
    match what {
        What::Asset(kind) => {
            if !members.is_empty() {
                return Err("members are only for bundles".to_owned());
            }
            let dir = Path::new(kind.catalog_dir()).join(id.as_str());
            let mut written = Vec::new();
            for (path, text) in templates(kind) {
                let rel = dir.join(path);
                write_new(catalog, &rel, &substitute(text, id, kind, maintainer))?;
                written.push(slash(&rel));
            }
            Ok(written)
        }
        What::Bundle => {
            let text = bundle(catalog, id, maintainer, members)?;
            let rel = Path::new("bundles").join(format!("{id}.toml"));
            write_new(catalog, &rel, &text)?;
            Ok(vec![slash(&rel)])
        }
    }
}

/// A bundle of the named members, each required at `^<its current version>`.
fn bundle(
    catalog: &Path,
    id: &AssetId,
    maintainer: &GithubLogin,
    members: &[String],
) -> Result<String, String> {
    if members.is_empty() {
        return Err(
            "a bundle needs at least one member: `cargo xtask new bundle <id> <asset>...`"
                .to_owned(),
        );
    }
    let validation = agent_grimoire_catalog::load(catalog);
    let mut lines = Vec::new();
    for member in members {
        let member = AssetId::new(member.as_str()).map_err(|e| e.to_string())?;
        let asset = validation
            .catalog
            .assets()
            .get(&member)
            .ok_or_else(|| format!("`{member}` is not an asset in the catalog"))?;
        lines.push(format!("{member} = \"^{}\"", asset.manifest.version));
    }
    lines.sort();
    lines.dedup();
    Ok(format!(
        r#"format = 1
id = "{id}"
version = "0.1.0"
summary = "One line saying what this bundle sets up"
description = """Who the bundle is for and what its members do together."""
maintainers = ["{maintainer}"]

[assets]
{}
"#,
        lines.join("\n")
    ))
}

/// Creates `catalog/rel` and its parent directories, refusing to replace anything.
fn write_new(catalog: &Path, rel: &Path, text: &str) -> Result<(), String> {
    let path = catalog.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
    }
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, text.as_bytes()))
        .map_err(|e| format!("cannot write {}: {e}", path.display()))
}

fn slash(path: &Path) -> String {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn maintainer() -> GithubLogin {
        GithubLogin::new("example-maintainer").unwrap()
    }

    fn catalog() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "# Catalog\n").unwrap();
        dir
    }

    fn errors(root: &Path) -> Vec<String> {
        agent_grimoire_catalog::load(root)
            .diagnostics
            .iter()
            .filter(|d| d.is_error())
            .map(ToString::to_string)
            .collect()
    }

    #[test]
    fn every_template_validates_as_generated() {
        let dir = catalog();
        for (kind, id) in [
            ("rule", "new-rule"),
            ("skill", "new-skill"),
            ("agent", "new-agent"),
            ("tool", "new-tool"),
            ("mcp-server", "new-server"),
        ] {
            let id = AssetId::new(id).unwrap();
            scaffold(
                dir.path(),
                What::parse(kind).unwrap(),
                &id,
                &maintainer(),
                &[],
            )
            .unwrap();
        }
        let members = ["new-rule".to_owned(), "new-agent".to_owned()];
        let id = AssetId::new("new-bundle").unwrap();
        scaffold(dir.path(), What::Bundle, &id, &maintainer(), &members).unwrap();
        assert_eq!(errors(dir.path()), Vec::<String>::new());
        let bundle = fs::read_to_string(dir.path().join("bundles/new-bundle.toml")).unwrap();
        assert!(
            bundle.contains("new-agent = \"^0.1.0\"\nnew-rule = \"^0.1.0\""),
            "{bundle}"
        );
    }

    #[test]
    fn placeholders_are_all_replaced() {
        let id = AssetId::new("my-thing").unwrap();
        for kind in [
            Kind::Rule,
            Kind::Skill,
            Kind::Agent,
            Kind::Tool,
            Kind::McpServer,
        ] {
            for (path, text) in templates(kind) {
                let out = substitute(text, &id, kind, &maintainer());
                assert!(!out.contains("@@"), "{kind} {path}: {out}");
            }
        }
    }

    #[test]
    fn ids_are_unique_and_nothing_is_overwritten() {
        let dir = catalog();
        let id = AssetId::new("taken").unwrap();
        scaffold(dir.path(), What::Asset(Kind::Rule), &id, &maintainer(), &[]).unwrap();
        let again = scaffold(
            dir.path(),
            What::Asset(Kind::Agent),
            &id,
            &maintainer(),
            &[],
        );
        assert!(again.unwrap_err().contains("already used"));
        let bundle = scaffold(
            dir.path(),
            What::Bundle,
            &id,
            &maintainer(),
            &["taken".to_owned()],
        );
        assert!(bundle.unwrap_err().contains("already used"));
    }

    #[test]
    fn bundles_need_existing_members() {
        let dir = catalog();
        let id = AssetId::new("b").unwrap();
        assert!(scaffold(dir.path(), What::Bundle, &id, &maintainer(), &[]).is_err());
        let missing = scaffold(
            dir.path(),
            What::Bundle,
            &id,
            &maintainer(),
            &["nope".to_owned()],
        );
        assert!(missing.unwrap_err().contains("not an asset"));
    }

    #[test]
    fn kinds_parse() {
        assert_eq!(
            What::parse("mcp-server").unwrap(),
            What::Asset(Kind::McpServer)
        );
        assert_eq!(What::parse("bundle").unwrap(), What::Bundle);
        assert!(What::parse("hook").is_err());
    }
}
