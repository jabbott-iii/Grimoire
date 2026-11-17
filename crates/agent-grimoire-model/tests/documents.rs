//! Parsing and single-document validation of every Grimoire file format.

#![allow(
    clippy::unwrap_used,
    reason = "fixture helpers fail the test loudly by design"
)]

use agent_grimoire_model::{
    AssetId, AssetManifest, AssetSelection, Bundle, ClientId, Lockfile, Permission, ProjectConfig,
    validate,
};

const RULE: &str = r#"
format = 1
id = "rust-errors"
kind = "rule"
version = "1.0.0"
summary = "Idiomatic error handling for Rust CLIs"
description = "Use typed errors in libraries and context-rich errors in binaries."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
tags = ["rust"]
maturity = "stable"
min-cli-version = "0.1.0"

[rule]
paths = ["**/*.rs"]

[clients.claude-code]
outputs = [{ src = "rule.md", to = "rules:rust-errors.md", strategy = "render" }]

[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
"#;

fn codes(m: &AssetManifest) -> Vec<&'static str> {
    validate::manifest(m).into_iter().map(|d| d.code).collect()
}

fn parse(text: &str) -> AssetManifest {
    AssetManifest::from_toml(text).unwrap_or_else(|e| panic!("{e}"))
}

#[test]
fn a_valid_rule_parses_and_validates_clean() {
    let m = parse(RULE);
    assert_eq!(m.id.as_str(), "rust-errors");
    assert!(m.clients.contains_key(&ClientId::ClaudeCode));
    assert_eq!(codes(&m), Vec::<&str>::new());
}

#[test]
fn unknown_fields_are_errors_with_location() {
    let err = AssetManifest::from_toml(&format!("{RULE}\nsurprise = 1\n")).unwrap_err();
    assert!(err.message().contains("surprise"), "{err}");
    assert!(err.span().is_some());

    let nested = RULE.replace(
        "paths = [\"**/*.rs\"]",
        "paths = [\"**/*.rs\"]\nextra = true",
    );
    assert!(AssetManifest::from_toml(&nested).is_err());

    let output = RULE.replace(
        "strategy = \"render\" }",
        "strategy = \"render\", mode = \"x\" }",
    );
    assert!(AssetManifest::from_toml(&output).is_err());
}

#[test]
fn invalid_values_are_reported_while_parsing() {
    for (from, to, needle) in [
        ("id = \"rust-errors\"", "id = \"Rust_Errors\"", "asset id"),
        ("version = \"1.0.0\"", "version = \"1.0\"", "version"),
        (
            "license = \"MIT OR Apache-2.0\"",
            "license = \"MIT/Apache\"",
            "license",
        ),
        ("format = 1", "format = 2", "format"),
        ("kind = \"rule\"", "kind = \"widget\"", "widget"),
        (
            "to = \"rules:rust-errors.md\"",
            "to = \"rules:../../etc/x.md\"",
            "destination",
        ),
        (
            "to = \"instructions\"",
            "to = \"instructions:AGENTS.md\"",
            "destination",
        ),
        ("[clients.agents-md]", "[clients.vim]", "vim"),
        (
            "summary = \"Idiomatic error handling for Rust CLIs\"",
            "summary = \"two\\nlines\"",
            "summary",
        ),
        (
            "maintainers = [\"example-maintainer\"]",
            "maintainers = [\"@someone\"]",
            "GitHub login",
        ),
    ] {
        let text = RULE.replace(from, to);
        assert_ne!(text, RULE, "fixture replacement `{from}` did not apply");
        let err = AssetManifest::from_toml(&text).expect_err(to);
        assert!(
            err.message().contains(needle),
            "`{to}`: expected `{needle}` in:\n{err}"
        );
    }
}

#[test]
fn rules_must_not_declare_permissions() {
    let m = parse(&RULE.replace("[rule]", "[permissions]\nnetwork = true\n\n[rule]"));
    assert!(codes(&m).contains(&"manifest/rule-permissions"));
}

#[test]
fn kind_tables_must_match_kind() {
    let m = parse(&RULE.replace("[rule]", "[agent]\ntools = []\n\n[rule]"));
    assert!(codes(&m).contains(&"manifest/kind-table-mismatch"));

    let m = parse(
        &RULE
            .replace("kind = \"rule\"", "kind = \"agent\"")
            .replace("[rule]\npaths = [\"**/*.rs\"]", ""),
    );
    assert!(codes(&m).contains(&"manifest/kind-table-missing"));
}

#[test]
fn deprecation_requires_replacement_and_vice_versa() {
    let m = parse(&RULE.replace("maturity = \"stable\"", "maturity = \"deprecated\""));
    assert!(codes(&m).contains(&"manifest/deprecation"));

    let m = parse(&RULE.replace(
        "maturity = \"stable\"",
        "maturity = \"stable\"\ndeprecated = { replacement = \"x\", reason = \"Old.\" }",
    ));
    assert!(codes(&m).contains(&"manifest/deprecation"));

    let m = parse(&RULE.replace(
        "maturity = \"stable\"",
        "maturity = \"deprecated\"\ndeprecated = { replacement = \"rust-errors\", reason = \"Old.\" }",
    ));
    assert!(codes(&m).contains(&"manifest/deprecation"));
}

#[test]
fn outputs_must_fit_client_anchor_kind_and_strategy() {
    for (from, to, code) in [
        (
            "[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]",
            "[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }]",
            "manifest/anchor-client",
        ),
        (
            "to = \"rules:rust-errors.md\", strategy = \"render\"",
            "to = \"skills:rust-errors/x.md\", strategy = \"copy\"",
            "manifest/anchor-kind",
        ),
        (
            "to = \"rules:rust-errors.md\", strategy = \"render\"",
            "to = \"rules:rust-errors.md\", strategy = \"copy\"",
            "manifest/strategy",
        ),
        (
            "{ src = \"rule.md\", to = \"instructions\"",
            "{ to = \"instructions\"",
            "manifest/output-src",
        ),
        (
            "to = \"rules:rust-errors.md\"",
            "to = \"rules:other-name.md\"",
            "manifest/output-name",
        ),
        (
            "to = \"rules:rust-errors.md\"",
            "to = \"rules:sub/rust-errors.md\"",
            "manifest/output-name",
        ),
        (
            "outputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]",
            "outputs = []",
            "manifest/outputs",
        ),
    ] {
        let text = RULE.replace(from, to);
        assert_ne!(text, RULE, "fixture replacement did not apply: {from}");
        let m = parse(&text);
        assert!(
            codes(&m).contains(&code),
            "expected {code} for `{to}`, got {:?}",
            codes(&m)
        );
    }

    let two = RULE.replace(
        "outputs = [{ src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }]",
        "outputs = [{ src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }, { src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }]",
    );
    let found = codes(&parse(&two));
    assert!(found.contains(&"manifest/output-duplicate"), "{found:?}");
    assert!(found.contains(&"manifest/output-count"), "{found:?}");

    let none = RULE
        .split("[clients.claude-code]")
        .next()
        .unwrap()
        .to_owned()
        + "[clients]\n";
    assert!(codes(&parse(&none)).contains(&"manifest/clients"));
}

const AGENT: &str = r#"
format = 1
id = "security-reviewer"
kind = "agent"
version = "1.0.0"
summary = "Reviews diffs for security problems"
description = "A read-only reviewer."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "experimental"
min-cli-version = "0.1.0"

[permissions]
filesystem = "read"

[agent]
tools = ["read", "search"]

[clients.claude-code]
outputs = [{ src = "agent.md", to = "agents:security-reviewer.md", strategy = "render" }]
"#;

#[test]
fn agents_declare_the_permissions_their_tools_imply() {
    assert_eq!(codes(&parse(AGENT)), Vec::<&str>::new());

    let m = parse(&AGENT.replace(
        "[\"read\", \"search\"]",
        "[\"read\", \"edit\", \"execute\", \"web\"]",
    ));
    let found = codes(&m);
    assert_eq!(
        found
            .iter()
            .filter(|c| **c == "manifest/agent-permissions")
            .count(),
        3,
        "{found:?}"
    );

    let m = parse(&AGENT.replace(
        "filesystem = \"read\"",
        "filesystem = \"read\"\nnetwork = true",
    ));
    assert!(codes(&m).contains(&"manifest/permission-unused"));

    let m = parse(&AGENT.replace("[\"read\", \"search\"]", "[\"read\", \"read\"]"));
    assert!(codes(&m).contains(&"manifest/agent-tools"));

    let m = parse(&AGENT.replace(
        "[\"read\", \"search\"]",
        "[\"read\", \"mcp:docs-index/search\"]",
    ));
    assert!(codes(&m).contains(&"manifest/agent-mcp"));

    assert!(
        AssetManifest::from_toml(
            &AGENT.replace("[agent]\ntools = [\"read\", \"search\"]", "[agent]")
        )
        .is_err()
    );
}

const MCP: &str = r#"
format = 1
id = "docs-index"
kind = "mcp-server"
version = "1.0.0"
summary = "Search project documentation"
description = "Read-only search over docs/."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "experimental"
min-cli-version = "0.1.0"

[permissions]
filesystem = "read"
env = ["DOCS_TOKEN"]

[mcp-server]
binary = "grimoire-mcp-docs-index"
transports = ["stdio", "streamable-http"]
env = [{ name = "DOCS_TOKEN", required = false, secret = true, description = "Optional token for remote docs" }]
tools = [{ name = "search_docs", read-only = true, destructive = false, idempotent = true, open-world = false }]

[clients.claude-code]
outputs = [{ to = "mcp", strategy = "entry" }]
"#;

#[test]
fn mcp_server_rules() {
    assert_eq!(codes(&parse(MCP)), Vec::<&str>::new());
    let reserved = MCP
        .replace("id = \"docs-index\"", "id = \"workspace\"")
        .replace("grimoire-mcp-docs-index", "grimoire-mcp-workspace");
    assert_eq!(codes(&parse(&reserved)), ["manifest/mcp-reserved-name"]);
    for (from, to, code) in [
        (
            "[\"stdio\", \"streamable-http\"]",
            "[\"stdio\"]",
            "manifest/mcp-transports",
        ),
        (
            "[\"stdio\", \"streamable-http\"]",
            "[\"stdio\", \"stdio\"]",
            "manifest/mcp-transports",
        ),
        (
            "binary = \"grimoire-mcp-docs-index\"",
            "binary = \"grimoire-mcp-other\"",
            "manifest/binary-name",
        ),
        (
            "read-only = true, destructive = false",
            "read-only = true, destructive = true",
            "manifest/mcp-annotations",
        ),
        (
            "open-world = false",
            "open-world = true",
            "manifest/mcp-open-world",
        ),
        ("env = [\"DOCS_TOKEN\"]", "", "manifest/mcp-env"),
        (
            "outputs = [{ to = \"mcp\", strategy = \"entry\" }]",
            "outputs = [{ src = \"x\", to = \"mcp\", strategy = \"entry\" }]",
            "manifest/output-src",
        ),
    ] {
        let text = MCP.replace(from, to);
        assert_ne!(text, MCP, "fixture replacement did not apply: {from}");
        let found = codes(&parse(&text));
        assert!(
            found.contains(&code),
            "expected {code} for `{to}`, got {found:?}"
        );
    }
    // Every annotation must be stated explicitly.
    assert!(AssetManifest::from_toml(&MCP.replace(", open-world = false", "")).is_err());
}

const TOOL: &str = r#"
format = 1
id = "secret-guard"
kind = "tool"
version = "1.0.0"
summary = "Blocks secrets from being written"
description = "A hook that refuses edits containing likely secrets."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "experimental"
min-cli-version = "0.1.0"

[permissions]
filesystem = "read"
hooks = true

[tool]
binary = "grimoire-tool-secret-guard"
hooks = [{ event = "pre-tool-use", tools = ["edit"], args = ["check", "--stdin"], timeout-secs = 10 }]

[clients.claude-code]
outputs = [{ to = "hooks", strategy = "entry" }]
"#;

#[test]
fn tool_and_hook_rules() {
    assert_eq!(codes(&parse(TOOL)), Vec::<&str>::new());
    for (from, to, code) in [
        ("hooks = true", "hooks = false", "manifest/hooks-permission"),
        (
            "event = \"pre-tool-use\"",
            "event = \"session-start\"",
            "manifest/hook-tools",
        ),
        (
            "timeout-secs = 10",
            "timeout-secs = 0",
            "manifest/hook-timeout",
        ),
        (
            "timeout-secs = 10",
            "timeout-secs = 601",
            "manifest/hook-timeout",
        ),
        (
            "outputs = [{ to = \"hooks\", strategy = \"entry\" }]",
            "outputs = []",
            "manifest/tool-outputs",
        ),
    ] {
        let text = TOOL.replace(from, to);
        assert_ne!(text, TOOL, "fixture replacement did not apply: {from}");
        let found = codes(&parse(&text));
        assert!(
            found.contains(&code),
            "expected {code} for `{to}`, got {found:?}"
        );
    }
    // A tool without hooks writes nothing and needs no hooks permission.
    let plain = TOOL
        .replace("hooks = true\n", "")
        .replace("hooks = [{ event = \"pre-tool-use\", tools = [\"edit\"], args = [\"check\", \"--stdin\"], timeout-secs = 10 }]\n", "")
        .replace("outputs = [{ to = \"hooks\", strategy = \"entry\" }]", "outputs = []");
    assert_eq!(codes(&parse(&plain)), Vec::<&str>::new());
    // Hook arguments are argv elements, never shell strings with control characters.
    assert!(AssetManifest::from_toml(&TOOL.replace("\"--stdin\"", "\"--stdin\\n\"")).is_err());
}

#[test]
fn skill_outputs_keep_the_layout() {
    let skill = RULE
        .replace("id = \"rust-errors\"", "id = \"adr-writer\"")
        .replace("kind = \"rule\"", "kind = \"skill\"")
        .replace("[rule]\npaths = [\"**/*.rs\"]\n", "")
        .replace(
            "outputs = [{ src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }]",
            "outputs = [{ src = \"SKILL.md\", to = \"skills:adr-writer/SKILL.md\", strategy = \"copy\" }]",
        )
        .replace(
            "outputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]",
            "outputs = [{ src = \"SKILL.md\", to = \"skills:adr-writer/other.md\", strategy = \"copy\" }]",
        );
    let found = codes(&parse(&skill));
    assert!(found.contains(&"manifest/skill-layout"), "{found:?}");
}

#[test]
fn bundles_parse_and_validate() {
    let text = r#"
format = 1
id = "security-baseline"
version = "1.0.0"
summary = "Security essentials"
description = "A reviewer agent and a secret guard."
maintainers = ["example-maintainer"]

[assets]
security-reviewer = "^1"
"#;
    let b = Bundle::from_toml(text).unwrap();
    assert_eq!(validate::bundle(&b), Vec::new());
    let empty = Bundle::from_toml(&text.replace("security-reviewer = \"^1\"", "")).unwrap();
    assert!(
        validate::bundle(&empty)
            .iter()
            .any(|d| d.code == "bundle/assets")
    );
}

#[test]
fn project_config_parses_both_selection_forms() {
    let text = r#"
format = 1
clients = ["claude-code", "agents-md"]
bundles = ["security-baseline"]

[assets]
rust-errors = "^1"
changelog-writer = { version = "^0.3", clients = ["claude-code"], allow = ["shell"] }
github-insights = { version = "^1", allow = ["network", "env"], env = { GITHUB_TOKEN = "GH_TOKEN" }, transport = "stdio" }
"#;
    let c = ProjectConfig::from_toml(text).unwrap();
    assert!(
        validate::project_config(&c).is_empty(),
        "{:?}",
        validate::project_config(&c)
    );
    assert!(matches!(
        c.assets[&"rust-errors".parse::<AssetId>().unwrap()],
        AssetSelection::Version(_)
    ));
    let opts = c.assets[&"changelog-writer".parse::<AssetId>().unwrap()]
        .options()
        .unwrap();
    assert!(opts.allow.contains(&Permission::Shell));

    // A bare string that is not a requirement, and unknown option keys, fail with specific messages.
    let err =
        ProjectConfig::from_toml(&text.replace("rust-errors = \"^1\"", "rust-errors = \"latest\""))
            .unwrap_err();
    assert!(err.message().contains("version requirement"), "{err}");
    let err =
        ProjectConfig::from_toml(&text.replace("transport = \"stdio\"", "transprot = \"stdio\""))
            .unwrap_err();
    assert!(err.message().contains("transprot"), "{err}");

    let bad = ProjectConfig::from_toml(
        &text
            .replace(
                "clients = [\"claude-code\", \"agents-md\"]",
                "clients = [\"agents-md\", \"agents-md\"]",
            )
            .replace(
                "bundles = [\"security-baseline\"]",
                "exclude = [\"rust-errors\"]",
            ),
    )
    .unwrap();
    let found: Vec<_> = validate::project_config(&bad)
        .into_iter()
        .map(|d| d.code)
        .collect();
    assert!(found.contains(&"config/clients"), "{found:?}");
    assert!(found.contains(&"config/exclude"), "{found:?}");
    assert!(found.contains(&"config/asset-clients"), "{found:?}");
}

#[test]
fn lockfile_round_trips_canonically() {
    let hash = format!("sha256:{}", "ab".repeat(32));
    let text = format!(
        r#"format = 1

[[asset]]
id = "rust-errors"
kind = "rule"
version = "1.0.0"
source = "builtin"
manifest = "{hash}"
selected-by = ["direct", "bundle:rust-cli"]

[[asset.output]]
clients = ["claude-code"]
path = ".claude/rules/rust-errors.md"
unit = "file"
hash = "{hash}"

[[asset.output]]
clients = ["agents-md"]
path = "AGENTS.md"
unit = "block"
hash = "{hash}"
"#
    );
    let mut lock = Lockfile::from_toml(&text).unwrap();
    assert_eq!(validate::lockfile(&lock), Vec::new());
    lock.canonicalize();
    let written = toml::to_string(&lock).unwrap();
    let reparsed = Lockfile::from_toml(&written).unwrap();
    assert_eq!(lock, reparsed);
    assert!(written.contains("[[asset.output]]"), "{written}");
    // Canonical order: outputs sorted by path, selectors sorted.
    let mut again = reparsed.clone();
    again.canonicalize();
    assert_eq!(toml::to_string(&again).unwrap(), written);
    assert!(
        !written.contains("cli-version"),
        "the lockfile must never record the CLI version"
    );

    let entry_without_key = text.replace("unit = \"block\"", "unit = \"entry\"");
    let lock = Lockfile::from_toml(&entry_without_key).unwrap();
    assert!(
        validate::lockfile(&lock)
            .iter()
            .any(|d| d.code == "lock/output-key")
    );
}

/// Applies a replacement that must change the fixture, then returns the codes it produces.
fn mutate(base: &str, from: &str, to: &str) -> Vec<&'static str> {
    let text = base.replacen(from, to, 1);
    assert_ne!(text, base, "fixture replacement did not apply: {from}");
    codes(&parse(&text))
}

#[test]
fn every_manifest_rule_has_a_mutation() {
    for (base, from, to, code) in [
        (
            RULE,
            "to = \"rules:rust-errors.md\"",
            "to = \"rules:rust-errors.txt\"",
            "manifest/output-name",
        ),
        (
            TOOL,
            "tools = [\"edit\"]",
            "tools = [\"edit\", \"edit\"]",
            "manifest/hook-tools",
        ),
        (
            RULE,
            "[clients.agents-md]\noutputs = [{ src = \"rule.md\"",
            "[clients.agents-md]\noutputs = [{ src = \"other.md\"",
            "manifest/rule-src",
        ),
        (
            AGENT,
            "filesystem = \"read\"",
            "filesystem = \"read\"\nenv = [\"A\", \"A\"]",
            "manifest/permissions",
        ),
        (
            RULE,
            "maintainers = [\"example-maintainer\"]",
            "maintainers = []",
            "manifest/maintainers",
        ),
        (
            RULE,
            "maintainers = [\"example-maintainer\"]",
            "maintainers = [\"Someone\", \"someone\"]",
            "manifest/maintainers",
        ),
        (
            RULE,
            "tags = [\"rust\"]",
            "tags = [\"rust\", \"rust\"]",
            "manifest/tags",
        ),
        (
            RULE,
            "tags = [\"rust\"]",
            "tags = [\"a\", \"b\", \"c\", \"d\", \"e\", \"f\", \"g\", \"h\", \"i\", \"j\", \"k\"]",
            "manifest/tags",
        ),
        (
            RULE,
            "[rule]",
            "[requires]\nrust-errors = \"*\"\n\n[rule]",
            "manifest/self-reference",
        ),
        (
            RULE,
            "[rule]",
            "[conflicts]\nrust-errors = { version = \"*\", reason = \"Self.\" }\n\n[rule]",
            "manifest/self-reference",
        ),
        (
            RULE,
            "[rule]",
            "[requires]\nother = \"^1\"\n\n[conflicts]\nother = { version = \"*\", reason = \"No.\" }\n\n[rule]",
            "manifest/requires-conflicts",
        ),
        (
            RULE,
            "paths = [\"**/*.rs\"]",
            "paths = [\"**/*.rs\", \"**/*.rs\"]",
            "manifest/rule-paths",
        ),
        (
            TOOL,
            "hooks = [{ event = \"pre-tool-use\", tools = [\"edit\"], args = [\"check\", \"--stdin\"], timeout-secs = 10 }]",
            "hooks = [{ event = \"stop\" }, { event = \"stop\" }]",
            "manifest/hook-duplicate",
        ),
        (
            TOOL,
            "binary = \"grimoire-tool-secret-guard\"",
            "binary = \"grimoire-tool-other\"",
            "manifest/binary-name",
        ),
        (
            MCP,
            "tools = [{ name = \"search_docs\", read-only = true, destructive = false, idempotent = true, open-world = false }]",
            "tools = []",
            "manifest/mcp-tools",
        ),
        (
            MCP,
            "tools = [{ name = \"search_docs\", read-only = true, destructive = false, idempotent = true, open-world = false }]",
            "tools = [{ name = \"search_docs\", read-only = true, destructive = false, idempotent = true, open-world = false }, { name = \"search_docs\", read-only = true, destructive = false, idempotent = true, open-world = false }]",
            "manifest/mcp-tools",
        ),
        (
            MCP,
            "env = [{ name = \"DOCS_TOKEN\", required = false, secret = true, description = \"Optional token for remote docs\" }]",
            "env = [{ name = \"DOCS_TOKEN\", required = false, secret = true, description = \"A\" }, { name = \"DOCS_TOKEN\", required = false, secret = true, description = \"B\" }]",
            "manifest/mcp-env",
        ),
    ] {
        let found = mutate(base, from, to);
        assert!(
            found.contains(&code),
            "expected {code} for `{to}`, got {found:?}"
        );
    }
}

#[test]
fn rule_path_duplicates_are_warnings_only() {
    let m = parse(&RULE.replace(
        "paths = [\"**/*.rs\"]",
        "paths = [\"**/*.rs\", \"**/*.rs\"]",
    ));
    let diags = validate::manifest(&m);
    assert!(diags.iter().all(|d| !d.is_error()), "{diags:?}");
}

#[test]
fn tool_hook_and_output_branches() {
    // permissions.hooks without hooks.
    let plain = TOOL
        .replace("hooks = [{ event = \"pre-tool-use\", tools = [\"edit\"], args = [\"check\", \"--stdin\"], timeout-secs = 10 }]\n", "")
        .replace("outputs = [{ to = \"hooks\", strategy = \"entry\" }]", "outputs = []");
    assert!(codes(&parse(&plain)).contains(&"manifest/hooks-permission"));
    // A tool without hooks writes nothing.
    let writes = plain.replace("hooks = true\n", "").replace(
        "outputs = []",
        "outputs = [{ to = \"hooks\", strategy = \"entry\" }]",
    );
    assert!(codes(&parse(&writes)).contains(&"manifest/tool-outputs"));
}

#[test]
fn skills_must_install_skill_md() {
    let skill = RULE
        .replace("id = \"rust-errors\"", "id = \"adr-writer\"")
        .replace("kind = \"rule\"", "kind = \"skill\"")
        .replace("[rule]\npaths = [\"**/*.rs\"]\n", "")
        .replace(
            "outputs = [{ src = \"rule.md\", to = \"rules:rust-errors.md\", strategy = \"render\" }]",
            "outputs = [{ src = \"references\", to = \"skills:adr-writer/references\", strategy = \"copy\" }]",
        )
        .replace("[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]\n", "");
    let found = codes(&parse(&skill));
    assert!(found.contains(&"manifest/skill-layout"), "{found:?}");
}

const BUNDLE: &str = r#"
format = 1
id = "security-baseline"
version = "1.0.0"
summary = "Security essentials"
description = "A reviewer agent and a secret guard."
maintainers = ["example-maintainer"]

[assets]
security-reviewer = "^1"
"#;

#[test]
fn bundle_rules() {
    for (from, to, code) in [
        (
            "maintainers = [\"example-maintainer\"]",
            "maintainers = []",
            "bundle/maintainers",
        ),
        (
            "maintainers = [\"example-maintainer\"]",
            "maintainers = [\"A\", \"a\"]",
            "bundle/maintainers",
        ),
        (
            "security-reviewer = \"^1\"",
            "security-baseline = \"^1\"",
            "bundle/assets",
        ),
    ] {
        let b = Bundle::from_toml(&BUNDLE.replacen(from, to, 1)).unwrap();
        let found: Vec<_> = validate::bundle(&b).into_iter().map(|d| d.code).collect();
        assert!(
            found.contains(&code),
            "expected {code} for `{to}`, got {found:?}"
        );
    }
}

const CONFIG: &str = r#"
format = 1
clients = ["claude-code"]
bundles = ["security-baseline"]
tags = ["rust"]

[assets]
changelog-writer = { version = "^0.3", clients = ["claude-code"] }
"#;

#[test]
fn project_config_rules() {
    for (from, to, code) in [
        (
            "clients = [\"claude-code\"]",
            "clients = []",
            "config/clients",
        ),
        (
            "bundles = [\"security-baseline\"]",
            "bundles = [\"x\", \"x\"]",
            "config/bundles",
        ),
        (
            "tags = [\"rust\"]",
            "tags = [\"rust\", \"rust\"]",
            "config/tags",
        ),
        (
            "clients = [\"claude-code\"] }",
            "clients = [] }",
            "config/asset-clients",
        ),
        (
            "clients = [\"claude-code\"] }",
            "clients = [\"claude-code\", \"claude-code\"] }",
            "config/asset-clients",
        ),
    ] {
        let c = ProjectConfig::from_toml(&CONFIG.replacen(from, to, 1)).unwrap();
        let found: Vec<_> = validate::project_config(&c)
            .into_iter()
            .map(|d| d.code)
            .collect();
        assert!(
            found.contains(&code),
            "expected {code} for `{to}`, got {found:?}"
        );
    }
}

fn lock_with(outputs: &[(&str, &str, &str, Option<&str>)]) -> Lockfile {
    use std::fmt::Write as _;
    let hash = format!("sha256:{}", "ab".repeat(32));
    let mut text = String::from("format = 1\n");
    let mut current = "";
    for (id, path, unit, key) in outputs {
        if *id != current {
            write!(
                text,
                "\n[[asset]]\nid = \"{id}\"\nkind = \"rule\"\nversion = \"1.0.0\"\nsource = \"builtin\"\nmanifest = \"{hash}\"\nselected-by = [\"direct\"]\n"
            )
            .unwrap();
            current = id;
        }
        write!(
            text,
            "\n[[asset.output]]\nclients = [\"agents-md\"]\npath = \"{path}\"\nunit = \"{unit}\"\n"
        )
        .unwrap();
        if let Some(key) = key {
            writeln!(text, "key = \"{key}\"").unwrap();
        }
        writeln!(text, "hash = \"{hash}\"").unwrap();
    }
    Lockfile::from_toml(&text).unwrap_or_else(|e| panic!("{e}\n{text}"))
}

fn lock_codes(l: &Lockfile) -> Vec<&'static str> {
    validate::lockfile(l).into_iter().map(|d| d.code).collect()
}

#[test]
fn lockfile_ownership() {
    // Several assets may each own a block in one shared file, and entries with distinct keys.
    let ok = lock_with(&[
        ("a", "AGENTS.md", "block", None),
        ("b", "AGENTS.md", "block", None),
        ("c", ".mcp.json", "entry", Some("mcpServers.c")),
        ("d", ".mcp.json", "entry", Some("mcpServers.d")),
    ]);
    assert_eq!(lock_codes(&ok), Vec::<&str>::new());

    for outputs in [
        vec![("a", "x.md", "file", None), ("b", "x.md", "file", None)],
        vec![
            ("a", "AGENTS.md", "file", None),
            ("b", "AGENTS.md", "block", None),
        ],
        vec![
            ("a", "AGENTS.md", "block", None),
            ("a", "AGENTS.md", "block", None),
        ],
        vec![
            ("a", ".mcp.json", "entry", Some("k")),
            ("b", ".mcp.json", "entry", Some("k")),
        ],
        vec![
            ("a", ".mcp.json", "entry", Some("k")),
            ("b", ".mcp.json", "block", None),
        ],
    ] {
        let found = lock_codes(&lock_with(&outputs));
        assert!(
            found.contains(&"lock/output-owner"),
            "{outputs:?}: {found:?}"
        );
    }
}

#[test]
fn lockfile_shape_rules() {
    let dup = lock_with(&[("a", "x.md", "file", None)]);
    let mut twice = dup.clone();
    twice.assets.push(dup.assets[0].clone());
    twice.assets[1].outputs.clear();
    assert!(lock_codes(&twice).contains(&"lock/assets"));

    let mut no_clients = dup.clone();
    no_clients.assets[0].outputs[0].clients.clear();
    assert!(lock_codes(&no_clients).contains(&"lock/output-clients"));

    let keyed_file = lock_with(&[("a", "x.md", "file", Some("k"))]);
    assert!(lock_codes(&keyed_file).contains(&"lock/output-key"));
}

#[test]
fn canonicalize_sorts_assets_outputs_and_selectors() {
    let mut lock = lock_with(&[
        ("zeta", "z.md", "file", None),
        ("alpha", "b.md", "file", None),
        ("alpha", "a.md", "file", None),
    ]);
    lock.assets[1].selected_by = vec![
        "tag:rust".parse().unwrap(),
        "direct".parse().unwrap(),
        "direct".parse().unwrap(),
    ];
    lock.canonicalize();
    let ids: Vec<&str> = lock.assets.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["alpha", "zeta"]);
    let paths: Vec<&str> = lock.assets[0]
        .outputs
        .iter()
        .map(|o| o.path.as_str())
        .collect();
    assert_eq!(paths, ["a.md", "b.md"]);
    let selectors: Vec<String> = lock.assets[0]
        .selected_by
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(selectors, ["direct", "tag:rust"]);
}
