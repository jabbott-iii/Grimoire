//! Adapter output for every kind, snapshotted per client combination.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::BTreeMap;
use std::fmt::Write as _;

use agent_grimoire_adapters::{AVAILABLE, Rendered, render};
use agent_grimoire_catalog::Source;
use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{Diagnostic, ProjectConfig, Version};
use agent_grimoire_resolve::{Request, Update, resolve};
use agent_grimoire_testkit::FixtureCatalog;

fn catalog() -> Source {
    let c = FixtureCatalog::new();
    c.rule(
        "commit-style",
        "1.0.0",
        &[],
        "# Commits\n\nWrite Conventional Commits.\n",
    )
    .rule(
        "rust-errors",
        "1.2.0",
        &["**/*.rs", "src/`odd`/**"],
        "# Errors\n\nUse typed errors.\n",
    )
    .skill("adr-writer", "0.3.1")
    .agent("security-reviewer", "1.0.0", &["read", "search"])
    .agent("silent-reviewer", "1.0.0", &[])
    .tool("secret-guard", "1.0.0")
    .mcp_server("docs-index", "2.0.0", &["DOCS_TOKEN"]);
    // An optional variable, to show the empty default.
    c.edit_manifest("mcp-servers/docs-index", |m| {
        m.replace(
            "env = [{ name = \"DOCS_TOKEN\", required = true",
            "env = [{ name = \"DOCS_CACHE\", required = false, secret = false, description = \"Cache dir\" }, { name = \"DOCS_TOKEN\", required = true",
        )
        .replace("env = [\"DOCS_TOKEN\"]", "env = [\"DOCS_CACHE\", \"DOCS_TOKEN\"]")
    });
    c.source()
}

const ALL_ASSETS: &str = r#"
[assets]
commit-style = "^1"
rust-errors = "^1"
adr-writer = "^0.3"
security-reviewer = { version = "^1", allow = ["filesystem-read"] }
silent-reviewer = "^1"
secret-guard = { version = "^1", allow = ["filesystem-read", "hooks"] }
docs-index = { version = "^2", allow = ["filesystem-read", "env:DOCS_CACHE", "env:DOCS_TOKEN"], env = { DOCS_TOKEN = "GH_TOKEN" } }
"#;

const PORTABLE_ASSETS: &str = r#"
[assets]
commit-style = "^1"
rust-errors = "^1"
adr-writer = "^0.3"
"#;

fn rendered(source: &Source, clients: &str) -> Result<Rendered, Vec<Diagnostic>> {
    let assets = if clients.contains("claude-code") {
        ALL_ASSETS
    } else {
        PORTABLE_ASSETS
    };
    let text = format!("format = 1\nclients = [{clients}]\n{assets}");
    let config = ProjectConfig::from_toml(&text).unwrap();
    let version = Version::parse("0.1.0").unwrap();
    let resolution = resolve(
        source,
        &Request {
            config: &config,
            lock: None,
            cli_version: &version,
            update: &Update::None,
            available_clients: &AVAILABLE,
        },
    )
    .unwrap();
    render(&resolution)
}

fn dump(r: &Rendered) -> String {
    let mut out = String::new();
    for d in &r.desired {
        let clients: Vec<&str> = d.clients.iter().map(|c| c.as_str()).collect();
        let (kind, body) = match &d.content {
            Content::File(bytes) => (
                "file".to_owned(),
                String::from_utf8_lossy(bytes).into_owned(),
            ),
            Content::Block(text) => (format!("block {}", d.owner), text.clone()),
            Content::Entry { key, value } => (
                format!("entry {key}"),
                serde_json::to_string_pretty(value).unwrap() + "\n",
            ),
        };
        writeln!(
            out,
            "== {} [{}] ({}) {kind}",
            d.path,
            d.owner,
            clients.join(", ")
        )
        .unwrap();
        out.push_str(&body);
    }
    for w in &r.warnings {
        writeln!(out, "!! {w}").unwrap();
    }
    out
}

#[test]
fn claude_code_alone() {
    let r = rendered(&catalog(), "\"claude-code\"").unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn agents_md_alone() {
    let r = rendered(&catalog(), "\"agents-md\"").unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn claude_code_and_agents_md() {
    let r = rendered(&catalog(), "\"claude-code\", \"agents-md\"").unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn frontmatter_is_valid_yaml_with_string_values() {
    let r = rendered(&catalog(), "\"claude-code\"").unwrap();
    for d in &r.desired {
        let Content::File(bytes) = &d.content else {
            continue;
        };
        let text = std::str::from_utf8(bytes).unwrap();
        let Some(rest) = text.strip_prefix("---\n") else {
            continue;
        };
        let (yaml, _) = rest.split_once("\n---\n").unwrap();
        // Values must stay strings: a YAML reader must not see `true` or `123`.
        let parsed: BTreeMap<String, Field> = serde_saphyr::from_str(yaml)
            .unwrap_or_else(|e| panic!("{} frontmatter: {e}\n{yaml}", d.path));
        assert!(
            parsed.contains_key("name") || parsed.contains_key("paths"),
            "{}",
            d.path
        );
    }
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
#[allow(dead_code, reason = "only parsing matters")]
enum Field {
    Text(String),
    List(Vec<String>),
}

#[test]
fn streamable_http_entries_carry_only_the_url() {
    let r = render_config(
        &catalog(),
        "format = 1\nclients = [\"claude-code\"]\n[assets]\ndocs-index = { version = \"^2\", allow = [\"filesystem-read\", \"env:DOCS_CACHE\", \"env:DOCS_TOKEN\"], transport = \"streamable-http\", url = \"http://127.0.0.1:8931/mcp\" }\n",
    )
    .unwrap();
    let entry = r
        .desired
        .iter()
        .find(|d| d.path.as_str() == ".mcp.json")
        .unwrap();
    let Content::Entry { key, value } = &entry.content else {
        panic!("an entry")
    };
    assert_eq!(key.as_str(), "/mcpServers/docs-index");
    assert_eq!(
        value,
        &serde_json::json!({ "type": "http", "url": "http://127.0.0.1:8931/mcp" })
    );
}

#[test]
fn unknown_env_overrides_are_refused() {
    let errors = render_config(
        &catalog(),
        "format = 1\nclients = [\"claude-code\"]\n[assets]\ndocs-index = { version = \"^2\", allow = [\"filesystem-read\", \"env:DOCS_CACHE\", \"env:DOCS_TOKEN\"], env = { NOPE = \"X\" } }\n",
    )
    .unwrap_err();
    assert!(
        errors.iter().any(|d| d.code == "adapter/env-unknown"),
        "{errors:?}"
    );
}

fn render_config(source: &Source, text: &str) -> Result<Rendered, Vec<Diagnostic>> {
    let config = ProjectConfig::from_toml(text).unwrap();
    let version = Version::parse("0.1.0").unwrap();
    let resolution = resolve(
        source,
        &Request {
            config: &config,
            lock: None,
            cli_version: &version,
            update: &Update::None,
            available_clients: &AVAILABLE,
        },
    )
    .unwrap();
    render(&resolution)
}

#[test]
fn claude_code_reading_agents_md_only_rules_is_reported() {
    let source = catalog();
    let r = render_config(
        &source,
        "format = 1\nclients = [\"claude-code\", \"agents-md\"]\n[assets]\ncommit-style = \"^1\"\nrust-errors = { version = \"^1\", clients = [\"agents-md\"] }\n",
    )
    .unwrap();
    let warnings: Vec<String> = r.warnings.iter().map(ToString::to_string).collect();
    assert!(
        warnings.iter().any(|w| w.contains("plan/cross-client-rule")
            && w.contains("rust-errors")
            && w.contains("import")),
        "{warnings:?}"
    );
}

#[test]
fn mcp_tool_names_claude_code_would_rename_are_refused() {
    let c = FixtureCatalog::new();
    c.mcp_server("docs-index", "2.0.0", &[])
        .edit_manifest("mcp-servers/docs-index", |m| m.replace("name = \"search\"", "name = \"search.v2\""))
        .asset(
        "agent",
        "dotted",
        "1.0.0",
        "[requires]\ndocs-index = \"^2\"\n[permissions]\n[agent]\ntools = [\"mcp:docs-index/search.v2\"]\n[clients.claude-code]\noutputs = [{ src = \"agent.md\", to = \"agents:dotted.md\", strategy = \"render\" }]\n",
        &[("agent.md", "You search.\n")],
    );
    let errors = render_config(
        &c.source(),
        "format = 1\nclients = [\"claude-code\"]\n[assets]\ndotted = \"^1\"\n[allow]\ndocs-index = [\"filesystem-read\"]\n",
    )
    .unwrap_err();
    assert!(
        errors
            .iter()
            .any(|d| d.code == "adapter/unsupported" && d.message.contains("may rename")),
        "{errors:?}"
    );
}

#[test]
fn search_hooks_are_flagged() {
    let c = FixtureCatalog::new();
    c.tool("finder", "1.0.0")
        .edit_manifest("tools/finder", |m| {
            m.replace("tools = [\"edit\"]", "tools = [\"search\"]")
        });
    let r = render_config(
        &c.source(),
        "format = 1\nclients = [\"claude-code\"]\n[assets]\nfinder = { version = \"^1\", allow = [\"filesystem-read\", \"hooks\"] }\n",
    )
    .unwrap();
    assert!(
        r.warnings.iter().any(|d| d.code == "adapter/hook-tools"),
        "{:?}",
        r.warnings
    );
}

#[test]
fn managed_locations() {
    use agent_grimoire_adapters::is_managed_location;
    use agent_grimoire_model::RelPath;
    for ok in [
        "AGENTS.md",
        "CLAUDE.md",
        ".mcp.json",
        ".claude/settings.json",
        ".claude/rules/x.md",
        ".agents/skills/s/SKILL.md",
    ] {
        assert!(is_managed_location(&RelPath::new(ok).unwrap()), "{ok}");
    }
    for bad in [
        ".git/config",
        ".git/HEAD",
        ".env",
        "grimoire.toml",
        "grimoire.lock",
        ".claude/rules",
        ".claude/settings.local.json",
        "src/main.rs",
    ] {
        assert!(!is_managed_location(&RelPath::new(bad).unwrap()), "{bad}");
    }
}

#[test]
fn fail_closed_hooks_render_on_failure_block() {
    let c = FixtureCatalog::new();
    c.tool("guard", "1.0.0").edit_manifest("tools/guard", |m| {
        m.replace(
            "timeout-secs = 10 }",
            "timeout-secs = 10, on-failure = \"block\" }, { event = \"user-prompt-submit\", args = [\"hook\"] }",
        )
    });
    let r = render_config(
        &c.source(),
        "format = 1\nclients = [\"claude-code\"]\n[assets]\nguard = { version = \"^1\", allow = [\"filesystem-read\", \"hooks\"] }\n",
    )
    .unwrap();
    let handlers: Vec<(String, serde_json::Value)> = r
        .desired
        .iter()
        .filter_map(|d| match &d.content {
            Content::Entry { key, value } => Some((key.as_str().to_owned(), value.clone())),
            _ => None,
        })
        .collect();
    assert_eq!(handlers.len(), 2, "{handlers:?}");
    let (_, pre) = handlers
        .iter()
        .find(|(k, _)| k.starts_with("/hooks/PreToolUse"))
        .unwrap();
    assert_eq!(pre["hooks"][0]["onFailure"], "block");
    let (_, prompt) = handlers
        .iter()
        .find(|(k, _)| k.starts_with("/hooks/UserPromptSubmit"))
        .unwrap();
    assert!(prompt["hooks"][0].get("onFailure").is_none(), "{prompt}");
    assert!(
        prompt.get("matcher").is_none(),
        "prompt hooks have no matcher: {prompt}"
    );
}
