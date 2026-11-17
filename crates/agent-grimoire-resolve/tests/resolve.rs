//! Resolution against fixture catalogs: selection, versions, conflicts, clients and consent.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::BTreeSet;

use agent_grimoire_catalog::Source;
use agent_grimoire_model::{
    AssetId, ClientId, Diagnostic, Lockfile, ProjectConfig, Selector, Version, validate,
};
use agent_grimoire_resolve::{Request, Resolution, Update, resolve};
use agent_grimoire_testkit::FixtureCatalog;

const CLIENTS: [ClientId; 2] = [ClientId::ClaudeCode, ClientId::AgentsMd];

fn catalog() -> FixtureCatalog {
    let c = FixtureCatalog::new();
    c.rule("rust-errors", "1.2.0", &["**/*.rs"], "Use typed errors.\n")
        .rule(
            "commit-style",
            "1.0.0",
            &[],
            "Write Conventional Commits.\n",
        )
        .skill("adr-writer", "0.3.1")
        .agent("security-reviewer", "1.0.0", &["read", "search"])
        .tool("secret-guard", "1.0.0")
        .mcp_server("docs-index", "2.0.0", &["DOCS_TOKEN"])
        .bundle(
            "security-baseline",
            &[("security-reviewer", "^1"), ("secret-guard", "^1")],
        );
    c
}

fn config(text: &str) -> ProjectConfig {
    let config = ProjectConfig::from_toml(text).unwrap();
    assert_eq!(validate::project_config(&config), Vec::<Diagnostic>::new());
    config
}

fn run<'s>(
    source: &'s Source,
    config: &ProjectConfig,
    lock: Option<&Lockfile>,
    update: &Update,
) -> Result<Resolution<'s>, Vec<Diagnostic>> {
    let version = Version::parse("0.1.0").unwrap();
    resolve(
        source,
        &Request {
            config,
            lock,
            cli_version: &version,
            update,
            available_clients: &CLIENTS,
        },
    )
}

fn codes(result: &Result<Resolution<'_>, Vec<Diagnostic>>) -> Vec<&'static str> {
    match result {
        Ok(r) => r.warnings.iter().map(|d| d.code).collect(),
        Err(diagnostics) => diagnostics.iter().map(|d| d.code).collect(),
    }
}

fn id(s: &str) -> AssetId {
    AssetId::new(s).unwrap()
}

#[test]
fn direct_bundle_tag_and_requires_selection() {
    let c = catalog();
    c.asset(
        "rule",
        "uses-errors",
        "1.0.0",
        r#"tags = ["rust"]
[requires]
rust-errors = "^1"
[rule]
[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
"#,
        &[(
            "rule.md",
            "Builds on rust-errors with different words entirely.\n",
        )],
    );
    let source = c.source();
    let config = config(
        r#"format = 1
clients = ["claude-code", "agents-md"]
bundles = ["security-baseline"]
tags = ["rust"]

[allow]
security-reviewer = ["filesystem-read"]
secret-guard = ["filesystem-read", "hooks"]

[assets]
adr-writer = "^0.3"
"#,
    );
    let r = run(&source, &config, None, &Update::None).unwrap();
    let ids: Vec<&str> = r.assets.keys().map(AssetId::as_str).collect();
    assert_eq!(
        ids,
        [
            "adr-writer",
            "rust-errors",
            "secret-guard",
            "security-reviewer",
            "uses-errors"
        ]
    );
    assert_eq!(
        r.assets[&id("rust-errors")].selected_by,
        BTreeSet::from([Selector::Requires(id("uses-errors"))])
    );
    assert_eq!(
        r.assets[&id("secret-guard")].selected_by,
        BTreeSet::from([Selector::Bundle(id("security-baseline"))])
    );
    // An agent has no destination in agents-md, so it is skipped quietly.
    assert_eq!(
        r.assets[&id("security-reviewer")].clients,
        BTreeSet::from([ClientId::ClaudeCode])
    );
    // uses-errors supports only agents-md, but claude-code can host rules: a warning.
    assert_eq!(
        r.assets[&id("uses-errors")].clients,
        BTreeSet::from([ClientId::AgentsMd])
    );
    assert_eq!(codes(&Ok(r)), ["resolve/client-skipped"]);
}

#[test]
fn unknown_assets_and_bundles_are_distinct_errors() {
    let source = catalog().source();
    let config = config(
        r#"format = 1
clients = ["claude-code"]
bundles = ["nope", "rust-errors"]
[assets]
missing = "^1"
security-baseline = "^1"
"#,
    );
    let result = run(&source, &config, None, &Update::None);
    let mut found = codes(&result);
    found.sort_unstable();
    assert_eq!(
        found,
        [
            "resolve/unknown-asset",
            "resolve/unknown-asset",
            "resolve/unknown-bundle",
            "resolve/unknown-bundle"
        ]
    );
    let text: Vec<String> = result
        .unwrap_err()
        .iter()
        .map(ToString::to_string)
        .collect();
    assert!(
        text.iter()
            .any(|t| t.contains("is a bundle, so list it under `bundles`")),
        "{text:?}"
    );
    assert!(
        text.iter()
            .any(|t| t.contains("is an asset, so list it under `[assets]`")),
        "{text:?}"
    );
}

#[test]
fn version_requirements_must_match_the_catalog() {
    let source = catalog().source();
    let config =
        config("format = 1\nclients = [\"claude-code\"]\n[assets]\nrust-errors = \"^2\"\n");
    assert_eq!(
        codes(&run(&source, &config, None, &Update::None)),
        ["resolve/version"]
    );
}

#[test]
fn lock_pins_hold_unless_updated() {
    let source = catalog().source();
    let config =
        config("format = 1\nclients = [\"claude-code\"]\n[assets]\nrust-errors = \"^1\"\n");
    let mut lock = Lockfile::from_toml(&format!(
        r#"format = 1
[[asset]]
id = "rust-errors"
kind = "rule"
version = "1.1.0"
source = "builtin"
manifest = "sha256:{}"
selected-by = ["direct"]
"#,
        "0".repeat(64)
    ))
    .unwrap();
    assert_eq!(
        codes(&run(&source, &config, Some(&lock), &Update::None)),
        ["resolve/locked-version-unavailable"]
    );
    assert!(run(&source, &config, Some(&lock), &Update::All).is_ok());
    assert!(
        run(
            &source,
            &config,
            Some(&lock),
            &Update::Only(BTreeSet::from([id("rust-errors")]))
        )
        .is_ok()
    );
    // Same version, different manifest: the catalog changed under the lock.
    lock.assets[0].version = Version::parse("1.2.0").unwrap();
    assert_eq!(
        codes(&run(&source, &config, Some(&lock), &Update::None)),
        ["resolve/locked-manifest-changed"]
    );
    lock.assets[0].manifest = source.asset(&id("rust-errors")).unwrap().digest.clone();
    assert!(run(&source, &config, Some(&lock), &Update::None).is_ok());
}

#[test]
fn conflicts_name_who_selected_each_side() {
    let c = catalog();
    c.asset(
        "rule",
        "legacy-errors",
        "1.0.0",
        r#"[conflicts]
rust-errors = { version = "*", reason = "contradicts the error-type guidance" }
[rule]
[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
"#,
        &[("rule.md", "Return strings as errors everywhere you can.\n")],
    )
    .bundle("old-style", &[("legacy-errors", "^1")]);
    let source = c.source();
    let config = config(
        "format = 1\nclients = [\"agents-md\"]\nbundles = [\"old-style\"]\n[assets]\nrust-errors = \"^1\"\n",
    );
    let errors = run(&source, &config, None, &Update::None).unwrap_err();
    let conflict: Vec<String> = errors
        .iter()
        .filter(|d| d.code == "resolve/conflict")
        .map(ToString::to_string)
        .collect();
    assert_eq!(conflict.len(), 1, "{errors:?}");
    assert!(
        conflict[0].contains("from bundle `old-style`"),
        "{conflict:?}"
    );
    assert!(conflict[0].contains("selected directly"), "{conflict:?}");
    assert!(conflict[0].contains("contradicts the error-type guidance"));
}

#[test]
fn excluded_dependencies_and_members() {
    let c = catalog();
    c.asset(
        "rule",
        "uses-errors",
        "1.0.0",
        r#"[requires]
rust-errors = "^1"
[rule]
[clients.agents-md]
outputs = [{ src = "rule.md", to = "instructions", strategy = "block" }]
"#,
        &[(
            "rule.md",
            "Builds on rust-errors with different words entirely.\n",
        )],
    );
    let source = c.source();
    let config_text = "format = 1\nclients = [\"agents-md\"]\nexclude = [\"rust-errors\"]\n[assets]\nuses-errors = \"^1\"\n";
    assert_eq!(
        codes(&run(&source, &config(config_text), None, &Update::None)),
        ["resolve/excluded-dependency"]
    );
    // A bundle member that is excluded is simply not installed.
    let config = config(
        "format = 1\nclients = [\"claude-code\"]\nbundles = [\"security-baseline\"]\nexclude = [\"secret-guard\"]\n[allow]\nsecurity-reviewer = [\"filesystem-read\"]\n",
    );
    let r = run(&source, &config, None, &Update::None).unwrap();
    assert_eq!(
        r.assets.keys().map(AssetId::as_str).collect::<Vec<_>>(),
        ["security-reviewer"]
    );
}

#[test]
fn permissions_must_be_accepted() {
    let source = catalog().source();
    let config_text = |allow: &str| {
        format!(
            "format = 1\nclients = [\"claude-code\"]\n[assets]\ndocs-index = {{ version = \"^2\"{allow} }}\n"
        )
    };
    let errors = run(&source, &config(&config_text("")), None, &Update::None).unwrap_err();
    assert_eq!(
        errors.iter().map(|d| d.code).collect::<Vec<_>>(),
        ["resolve/permission"]
    );
    let text = errors[0].to_string();
    assert!(text.contains("filesystem-read, env:DOCS_TOKEN"), "{text}");
    assert!(
        text.contains("grimoire add docs-index --allow filesystem-read,env:DOCS_TOKEN"),
        "{text}"
    );

    // Write covers read.
    let ok = config(&config_text(
        ", allow = [\"filesystem-write\", \"env:DOCS_TOKEN\"]",
    ));
    let r = run(&source, &ok, None, &Update::None).unwrap();
    let allow: Vec<String> = r.assets[&id("docs-index")]
        .allow
        .iter()
        .map(ToString::to_string)
        .collect();
    assert_eq!(allow, ["filesystem-read", "env:DOCS_TOKEN"]);

    // Bundle members take their grants from [allow].
    let bundled =
        config("format = 1\nclients = [\"claude-code\"]\nbundles = [\"security-baseline\"]\n");
    let errors = run(&source, &bundled, None, &Update::None).unwrap_err();
    assert!(
        errors.iter().all(|d| d.code == "resolve/permission"),
        "{errors:?}"
    );
    assert!(
        errors.iter().any(|d| d.message.contains("under [allow]")),
        "{errors:?}"
    );
}

#[test]
fn clients_explicit_unsupported_and_unavailable() {
    let source = catalog().source();
    let explicit = config(
        "format = 1\nclients = [\"claude-code\", \"agents-md\"]\n[assets]\nsecurity-reviewer = { version = \"^1\", clients = [\"agents-md\"], allow = [\"filesystem-read\"] }\n",
    );
    assert_eq!(
        codes(&run(&source, &explicit, None, &Update::None)),
        ["resolve/unsupported-client"]
    );
    let none = config(
        "format = 1\nclients = [\"agents-md\"]\n[assets]\nsecurity-reviewer = { version = \"^1\", allow = [\"filesystem-read\"] }\n",
    );
    assert_eq!(
        codes(&run(&source, &none, None, &Update::None)),
        ["resolve/unsupported-client"]
    );
    let cursor = config("format = 1\nclients = [\"cursor\"]\n");
    assert_eq!(
        codes(&run(&source, &cursor, None, &Update::None)),
        ["resolve/client-unavailable"]
    );
}

#[test]
fn min_cli_version_deprecation_and_unused_grants() {
    let c = catalog();
    c.asset(
        "rule",
        "future-rule",
        "1.0.0",
        "[rule]\n[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]\n",
        &[("rule.md", "Something only a newer grimoire understands well.\n")],
    )
    .edit_manifest("rules/future-rule", |m| m.replace("min-cli-version = \"0.1.0\"", "min-cli-version = \"9.0.0\""))
    .asset(
        "rule",
        "old-rule",
        "1.0.0",
        "deprecated = { replacement = \"commit-style\", reason = \"superseded\" }\n[rule]\n[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]\n",
        &[("rule.md", "An older way of describing what commits should look like.\n")],
    )
    .edit_manifest("rules/old-rule", |m| m.replace("maturity = \"stable\"", "maturity = \"deprecated\""));
    let source = c.source();
    let config_text = config(
        "format = 1\nclients = [\"agents-md\"]\n[allow]\nnot-installed = [\"shell\"]\n[assets]\nfuture-rule = \"*\"\nold-rule = \"*\"\n",
    );
    let mut found = codes(&run(&source, &config_text, None, &Update::None));
    found.sort_unstable();
    assert_eq!(
        found,
        [
            "resolve/allow-unused",
            "resolve/cli-too-old",
            "resolve/deprecated"
        ]
    );
}

#[test]
fn tags_skip_deprecated_assets() {
    let c = catalog();
    c.asset(
        "rule",
        "old-rule",
        "1.0.0",
        "tags = [\"style\"]\ndeprecated = { replacement = \"commit-style\", reason = \"superseded\" }\n[rule]\n[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]\n",
        &[("rule.md", "An older way of describing what commits should look like.\n")],
    )
    .edit_manifest("rules/old-rule", |m| m.replace("maturity = \"stable\"", "maturity = \"deprecated\""));
    let source = c.source();
    let r = run(
        &source,
        &config(
            "format = 1\nclients = [\"agents-md\"]\ntags = [\"style\", \"nothing-has-this\"]\n",
        ),
        None,
        &Update::None,
    )
    .unwrap();
    assert!(r.assets.is_empty());
    assert_eq!(codes(&Ok(r)), ["resolve/unknown-tag"]);
}

#[test]
fn tags_accept_pre_releases_and_skip_unusable_assets() {
    let c = catalog();
    c.asset(
        "rule",
        "fresh-rule",
        "0.1.0-alpha.1",
        "tags = [\"style\"]\n[rule]\n[clients.agents-md]\noutputs = [{ src = \"rule.md\", to = \"instructions\", strategy = \"block\" }]\n",
        &[("rule.md", "A brand new rule that is still being tried out.\n")],
    )
    .edit_manifest("agents/security-reviewer", |m| m.replace("min-cli-version", "tags = [\"style\"]\nmin-cli-version"));
    let source = c.source();
    let r = run(
        &source,
        &config("format = 1\nclients = [\"agents-md\"]\ntags = [\"style\"]\n"),
        None,
        &Update::None,
    )
    .unwrap();
    assert_eq!(
        r.assets.keys().map(AssetId::as_str).collect::<Vec<_>>(),
        ["fresh-rule"]
    );
    assert_eq!(codes(&Ok(r)), ["resolve/tag-skipped"]);
}

#[test]
fn errors_for_indirect_assets_name_the_selector() {
    let source = catalog().source();
    let errors = run(
        &source,
        &config("format = 1\nclients = [\"agents-md\"]\nbundles = [\"security-baseline\"]\n[allow]\nsecurity-reviewer = [\"filesystem-read\"]\nsecret-guard = [\"filesystem-read\", \"hooks\"]\n"),
        None,
        &Update::None,
    )
    .unwrap_err();
    let text: Vec<String> = errors.iter().map(ToString::to_string).collect();
    assert!(
        text.iter()
            .any(|t| t.contains("from bundle `security-baseline`") && t.contains("exclude")),
        "{text:?}"
    );
    // The grants are needed, so they are not reported as unused.
    assert!(
        !errors.iter().any(|d| d.code == "resolve/allow-unused"),
        "{text:?}"
    );
}

#[test]
fn permission_hints_show_the_merged_grant() {
    let source = catalog().source();
    let errors = run(
        &source,
        &config("format = 1\nclients = [\"claude-code\"]\nbundles = [\"security-baseline\"]\n[allow]\nsecurity-reviewer = [\"filesystem-read\"]\nsecret-guard = [\"hooks\"]\n"),
        None,
        &Update::None,
    )
    .unwrap_err();
    let text = errors
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        text.contains("grimoire add bundle:security-baseline --allow filesystem-read"),
        "{text}"
    );
    assert!(
        text.contains("secret-guard = [\"filesystem-read\", \"hooks\"]"),
        "{text}"
    );
}

/// Hooks are Claude Code only and Codex has no agents (ADR-0008): a project that
/// also uses those clients is told, and agents-md is skipped quietly.
#[test]
fn clients_that_cannot_host_a_kind_are_reported() {
    let source = catalog().source();
    let c = config(
        "format = 1\nclients = [\"claude-code\", \"agents-md\", \"codex\", \"gemini-cli\"]\n[assets]\nsecret-guard = { version = \"^1\", allow = [\"filesystem-read\", \"hooks\"] }\n",
    );
    let version = Version::parse("0.1.0").unwrap();
    let r = resolve(
        &source,
        &Request {
            config: &c,
            lock: None,
            cli_version: &version,
            update: &Update::None,
            available_clients: &ClientId::ALL,
        },
    )
    .unwrap();
    let not_covered: Vec<&str> = r
        .warnings
        .iter()
        .filter(|d| d.code == "resolve/client-not-covered")
        .map(|d| d.message.as_str())
        .collect();
    assert_eq!(
        not_covered.len(),
        2,
        "codex and gemini-cli: {not_covered:?}"
    );
    assert!(
        not_covered
            .iter()
            .all(|m| m.contains("hooks only for Claude Code"))
    );
    assert!(!not_covered.iter().any(|m| m.contains("agents-md")));
}
