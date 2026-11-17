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

/// The fixture catalog with tables for the four Phase 4 clients too (ADR-0008).
fn everywhere() -> Source {
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
    c.edit_manifest("mcp-servers/docs-index", |m| {
        m.replace(
            "env = [{ name = \"DOCS_TOKEN\", required = true",
            "env = [{ name = \"DOCS_CACHE\", required = false, secret = false, description = \"Cache dir\" }, { name = \"DOCS_TOKEN\", required = true",
        )
        .replace("env = [\"DOCS_TOKEN\"]", "env = [\"DOCS_CACHE\", \"DOCS_TOKEN\"]")
    });
    for id in ["commit-style", "rust-errors"] {
        c.edit_manifest(&format!("rules/{id}"), |m| {
            m + &format!(
                r#"
[clients.codex]
outputs = [{{ src = "rule.md", to = "instructions", strategy = "block" }}]

[clients.gemini-cli]
outputs = [{{ src = "rule.md", to = "instructions", strategy = "block" }}]

[clients.cursor]
outputs = [{{ src = "rule.md", to = "rules:{id}.md", strategy = "render" }}]

[clients.copilot]
outputs = [{{ src = "rule.md", to = "rules:{id}.md", strategy = "render" }}]
"#
            )
        });
    }
    c.edit_manifest("skills/adr-writer", |m| {
        let outputs = r#"outputs = [
  { src = "SKILL.md", to = "skills:adr-writer/SKILL.md", strategy = "copy" },
  { src = "references", to = "skills:adr-writer/references", strategy = "copy" },
]"#;
        let mut m = m;
        for client in ["codex", "cursor", "gemini-cli", "copilot"] {
            write!(m, "\n[clients.{client}]\n{outputs}\n").unwrap();
        }
        m
    });
    let agent_table = |id: &str, clients: &[&str]| {
        let mut tables = String::new();
        for client in clients {
            write!(
                tables,
                "\n[clients.{client}]\noutputs = [{{ src = \"agent.md\", to = \"agents:{id}.md\", strategy = \"render\" }}]\n"
            )
            .unwrap();
        }
        tables
    };
    c.edit_manifest("agents/security-reviewer", |m| {
        m + &agent_table("security-reviewer", &["cursor", "gemini-cli", "copilot"])
    });
    c.edit_manifest("agents/silent-reviewer", |m| {
        m + &agent_table("silent-reviewer", &["gemini-cli", "copilot"])
    });
    c.edit_manifest("mcp-servers/docs-index", |m| {
        let mut m = m;
        for client in ["codex", "cursor", "gemini-cli", "copilot"] {
            write!(
                m,
                "\n[clients.{client}]\noutputs = [{{ to = \"mcp\", strategy = \"entry\" }}]\n"
            )
            .unwrap();
        }
        m
    });
    c.source()
}

/// Every fixture asset that one of `clients` supports, with `docs-index`
/// either remapping a variable or not.
fn every_asset(clients: &str, remap: bool) -> String {
    let env = if remap {
        ", env = { DOCS_TOKEN = \"GH_TOKEN\" }"
    } else {
        ""
    };
    let has = |c: &str| clients.contains(&format!("\"{c}\""));
    let mut assets =
        String::from("commit-style = \"^1\"\nrust-errors = \"^1\"\nadr-writer = \"^0.3\"\n");
    if ["claude-code", "cursor", "gemini-cli", "copilot"]
        .iter()
        .any(|c| has(c))
    {
        assets
            .push_str("security-reviewer = { version = \"^1\", allow = [\"filesystem-read\"] }\n");
    }
    if ["claude-code", "gemini-cli", "copilot"]
        .iter()
        .any(|c| has(c))
    {
        assets.push_str("silent-reviewer = \"^1\"\n");
    }
    if has("claude-code") {
        assets.push_str(
            "secret-guard = { version = \"^1\", allow = [\"filesystem-read\", \"hooks\"] }\n",
        );
    }
    format!(
        r#"format = 1
clients = [{clients}]
[assets]
{assets}docs-index = {{ version = "^2", allow = ["filesystem-read", "env:DOCS_CACHE", "env:DOCS_TOKEN"]{env} }}
"#
    )
}

#[test]
fn codex_alone() {
    let r = render_config(&everywhere(), &every_asset("\"codex\"", false)).unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn cursor_alone() {
    let r = render_config(&everywhere(), &every_asset("\"cursor\"", true)).unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn gemini_cli_alone() {
    let r = render_config(&everywhere(), &every_asset("\"gemini-cli\"", true)).unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn copilot_alone() {
    let r = render_config(&everywhere(), &every_asset("\"copilot\"", true)).unwrap();
    insta::assert_snapshot!(dump(&r));
}

/// Rules shared through AGENTS.md, skills in both shared locations, and every
/// client's agents and MCP entries.
#[test]
fn every_client() {
    let all = "\"claude-code\", \"agents-md\", \"codex\", \"cursor\", \"gemini-cli\", \"copilot\"";
    let r = render_config(&everywhere(), &every_asset(all, false)).unwrap();
    insta::assert_snapshot!(dump(&r));
}

/// Without a client that needs `.agents/skills/`, Cursor and Copilot share
/// Claude Code's `.claude/skills/`, and each client keeps its own rule file.
#[test]
fn claude_code_cursor_and_copilot() {
    let r = render_config(
        &everywhere(),
        &every_asset("\"claude-code\", \"cursor\", \"copilot\"", true),
    )
    .unwrap();
    insta::assert_snapshot!(dump(&r));
}

#[test]
fn new_frontmatter_is_valid_yaml() {
    let all = "\"claude-code\", \"agents-md\", \"codex\", \"cursor\", \"gemini-cli\", \"copilot\"";
    let mut checked = 0;
    for clients in ["\"cursor\"", "\"copilot\"", "\"gemini-cli\"", all] {
        let r = render_config(&everywhere(), &every_asset(clients, false)).unwrap();
        for d in &r.desired {
            let Content::File(bytes) = &d.content else {
                continue;
            };
            let text = std::str::from_utf8(bytes).unwrap();
            let Some(rest) = text.strip_prefix("---\n") else {
                continue;
            };
            let (yaml, _) = rest.split_once("\n---\n").unwrap();
            let parsed: BTreeMap<String, serde_json::Value> = serde_saphyr::from_str(yaml)
                .unwrap_or_else(|e| panic!("{} frontmatter: {e}\n{yaml}", d.path));
            // Booleans are written as YAML booleans, everything else as strings
            // or lists of strings.
            for (key, value) in &parsed {
                let ok = match key.as_str() {
                    "alwaysApply" | "readonly" | "strict-tools-list" => value.is_boolean(),
                    "tools" | "paths" | "disallowedTools" => value
                        .as_array()
                        .is_some_and(|a| a.iter().all(serde_json::Value::is_string)),
                    _ => value.is_string(),
                };
                assert!(ok, "{}: `{key}` is {value}", d.path);
            }
            checked += 1;
        }
    }
    assert!(checked >= 10, "only {checked} files had frontmatter");
}

#[test]
fn cursor_refuses_agents_it_cannot_limit() {
    let c = FixtureCatalog::new();
    c.agent("editor", "1.0.0", &["read", "edit"]);
    c.edit_manifest("agents/editor", |m| {
        m + "\n[clients.cursor]\noutputs = [{ src = \"agent.md\", to = \"agents:editor.md\", strategy = \"render\" }]\n"
    });
    let errors = render_config(
        &c.source(),
        "format = 1\nclients = [\"cursor\"]\n[assets]\neditor = { version = \"^1\", allow = [\"filesystem-write\"] }\n",
    )
    .unwrap_err();
    assert!(
        errors
            .iter()
            .any(|d| d.code == "adapter/unsupported" && d.message.contains("readonly")),
        "{errors:?}"
    );
}

#[test]
fn codex_refuses_env_remaps() {
    let errors = render_config(&everywhere(), &every_asset("\"codex\"", true)).unwrap_err();
    assert!(
        errors.iter().any(|d| d.code == "adapter/env-remap"),
        "{errors:?}"
    );
}

#[test]
fn comma_globs_are_refused_where_globs_are_comma_separated() {
    for client in ["cursor", "copilot"] {
        let c = FixtureCatalog::new();
        c.rule("braces", "1.0.0", &["src/{a,b}/**"], "# Braces\n");
        c.edit_manifest("rules/braces", |m| {
            m + &format!(
                "\n[clients.{client}]\noutputs = [{{ src = \"rule.md\", to = \"rules:braces.md\", strategy = \"render\" }}]\n"
            )
        });
        let errors = render_config(
            &c.source(),
            &format!("format = 1\nclients = [\"{client}\"]\n[assets]\nbraces = \"^1\"\n"),
        )
        .unwrap_err();
        assert!(
            errors
                .iter()
                .any(|d| d.code == "adapter/unsupported" && d.message.contains("comma")),
            "{client}: {errors:?}"
        );
    }
}

#[test]
fn new_managed_locations() {
    use agent_grimoire_adapters::{is_managed_location, is_project_location};
    use agent_grimoire_model::RelPath;
    for ok in [
        "GEMINI.md",
        ".codex/config.toml",
        ".cursor/mcp.json",
        ".gemini/settings.json",
        ".cursor/rules/x.mdc",
        ".cursor/agents/x.md",
        ".gemini/agents/x.md",
        ".github/instructions/x.instructions.md",
        ".github/agents/x.agent.md",
    ] {
        assert!(is_managed_location(&RelPath::new(ok).unwrap()), "{ok}");
    }
    for bad in [
        ".github/workflows/ci.yml",
        ".github/copilot-instructions.md",
        ".github/hooks/grimoire.json",
        ".codex/hooks.json",
        ".cursor/hooks.json",
        ".cursor/permissions.json",
        ".gemini/agents",
    ] {
        assert!(!is_managed_location(&RelPath::new(bad).unwrap()), "{bad}");
    }
    assert!(is_project_location(&RelPath::new("GEMINI.md").unwrap()));
    assert!(!is_project_location(&RelPath::new("AGENTS.md").unwrap()));
}

/// A brace glob can't be a comma-separated list, but when AGENTS.md holds the
/// rule, Cursor and Copilot read it from there with its scope line.
#[test]
fn comma_globs_are_fine_when_agents_md_covers_the_client() {
    let c = FixtureCatalog::new();
    c.rule("braces", "1.0.0", &["src/{a,b}/**"], "# Braces\n");
    c.edit_manifest("rules/braces", |m| {
        m + "\n[clients.cursor]\noutputs = [{ src = \"rule.md\", to = \"rules:braces.md\", strategy = \"render\" }]\n\n[clients.copilot]\noutputs = [{ src = \"rule.md\", to = \"rules:braces.md\", strategy = \"render\" }]\n"
    });
    let source = c.source();
    let r = render_config(
        &source,
        "format = 1\nclients = [\"agents-md\", \"cursor\", \"copilot\"]\n[assets]\nbraces = \"^1\"\n",
    )
    .unwrap();
    let block = r
        .desired
        .iter()
        .find(|d| d.path.as_str() == "AGENTS.md")
        .unwrap();
    let clients: Vec<&str> = block.clients.iter().map(|c| c.as_str()).collect();
    assert_eq!(clients, ["agents-md", "cursor", "copilot"]);
    // Without AGENTS.md, Cursor would need the comma list, so it is refused.
    let errors = render_config(
        &source,
        "format = 1\nclients = [\"cursor\"]\n[assets]\nbraces = \"^1\"\n",
    )
    .unwrap_err();
    assert!(errors.iter().any(|d| d.code == "adapter/unsupported"));
}

#[test]
fn gemini_reading_agents_md_only_rules_is_reported() {
    let r = render_config(
        &everywhere(),
        "format = 1\nclients = [\"agents-md\", \"gemini-cli\"]\n[assets]\ncommit-style = \"^1\"\nrust-errors = { version = \"^1\", clients = [\"agents-md\"] }\n",
    )
    .unwrap();
    let warnings: Vec<String> = r.warnings.iter().map(ToString::to_string).collect();
    assert!(
        warnings.iter().any(|w| w.contains("plan/cross-client-rule")
            && w.contains("Gemini CLI")
            && w.contains("rust-errors")),
        "{warnings:?}"
    );
    assert!(
        !warnings
            .iter()
            .any(|w| w.contains("Gemini CLI") && w.contains("commit-style")),
        "{warnings:?}"
    );
}

#[test]
fn gemini_refuses_tool_names_it_would_shorten() {
    let c = FixtureCatalog::new();
    let long = "a".repeat(60);
    c.mcp_server("docs-index", "2.0.0", &[])
        .edit_manifest("mcp-servers/docs-index", |m| {
            m.replace("name = \"search\"", &format!("name = \"{long}\""))
                + "\n[clients.gemini-cli]\noutputs = [{ to = \"mcp\", strategy = \"entry\" }]\n"
        })
        .asset(
            "agent",
            "long-tools",
            "1.0.0",
            &format!(
                "[requires]\ndocs-index = \"^2\"\n[permissions]\n[agent]\ntools = [\"mcp:docs-index/{long}\"]\n[clients.gemini-cli]\noutputs = [{{ src = \"agent.md\", to = \"agents:long-tools.md\", strategy = \"render\" }}]\n"
            ),
            &[("agent.md", "You search.\n")],
        );
    let errors = render_config(
        &c.source(),
        "format = 1\nclients = [\"gemini-cli\"]\n[assets]\nlong-tools = \"^1\"\n[allow]\ndocs-index = [\"filesystem-read\"]\n",
    )
    .unwrap_err();
    assert!(
        errors
            .iter()
            .any(|d| d.code == "adapter/unsupported" && d.message.contains("shortens")),
        "{errors:?}"
    );
}

#[test]
fn an_unknown_env_override_is_reported_once() {
    let all = "\"claude-code\", \"codex\", \"cursor\", \"gemini-cli\", \"copilot\"";
    let errors = render_config(
        &everywhere(),
        &format!("format = 1\nclients = [{all}]\n[assets]\ndocs-index = {{ version = \"^2\", allow = [\"filesystem-read\", \"env:DOCS_CACHE\", \"env:DOCS_TOKEN\"], env = {{ NOPE = \"X\" }} }}\n"),
    )
    .unwrap_err();
    let unknown = errors
        .iter()
        .filter(|d| d.code == "adapter/env-unknown")
        .count();
    assert_eq!(unknown, 1, "{errors:?}");
}

#[test]
fn codex_accepts_an_override_that_renames_nothing() {
    let r = render_config(
        &everywhere(),
        "format = 1\nclients = [\"codex\"]\n[assets]\ndocs-index = { version = \"^2\", allow = [\"filesystem-read\", \"env:DOCS_CACHE\", \"env:DOCS_TOKEN\"], env = { DOCS_TOKEN = \"DOCS_TOKEN\" } }\n",
    );
    assert!(r.is_ok(), "{:?}", r.err());
}
