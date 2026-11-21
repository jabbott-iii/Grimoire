//! The embedded catalog end to end: every asset and bundle installs, verifies
//! and uninstalls cleanly, and what it writes is reviewed by snapshot.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::fmt::Write as _;
use std::io::Cursor;

use agent_grimoire::{Context, Exit, run};
use agent_grimoire_catalog::Source;
use agent_grimoire_model::Version;
use agent_grimoire_testkit::Project;

fn grimoire(source: &Source, project: &Project, args: &[&str]) -> (Exit, String, String) {
    grimoire_with_path(source, project, args, None)
}

fn grimoire_with_path(
    source: &Source,
    project: &Project,
    args: &[&str],
    path: Option<std::ffi::OsString>,
) -> (Exit, String, String) {
    let mut stdin = Cursor::new(Vec::new());
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = {
        let mut ctx = Context {
            source,
            version: Version::parse(env!("CARGO_PKG_VERSION")).unwrap(),
            cwd: project.root().to_path_buf(),
            stdin: &mut stdin,
            stdout: &mut stdout,
            stderr: &mut stderr,
            interactive: false,
            path,
        };
        let mut all = vec!["grimoire"];
        all.extend_from_slice(args);
        run(all, &mut ctx)
    };
    (
        exit,
        String::from_utf8(stdout).unwrap(),
        String::from_utf8(stderr).unwrap(),
    )
}

fn ok(source: &Source, project: &Project, args: &[&str]) -> String {
    let (exit, stdout, stderr) = grimoire(source, project, args);
    assert_eq!(
        exit,
        Exit::Ok,
        "{args:?}\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    stdout
}

fn snapshot_tree(p: &Project) -> String {
    let mut out = String::new();
    for entry in p.tree() {
        writeln!(out, "== {entry}").unwrap();
        if !entry.ends_with('/') && entry != "grimoire.lock" {
            out.push_str(&p.read(&entry));
        }
    }
    out
}

/// One finished asset of each kind, as `catalog/README.md` names them.
const EXEMPLARS: [&str; 5] = [
    "conventional-commits",
    "adr-writer",
    "security-reviewer",
    "secret-guard",
    "project-docs",
];

/// Every other asset in the embedded catalog. The install tests below add these
/// too, so nothing in the catalog ships without an end-to-end test.
const OTHER_ASSETS: [&str; 1] = ["plain-language-docs"];

#[test]
fn the_catalog_holds_one_exemplar_per_kind_and_two_bundles() {
    let source = Source::builtin().unwrap();
    let mut ids: Vec<&str> = source
        .assets()
        .keys()
        .map(agent_grimoire_model::AssetId::as_str)
        .collect();
    ids.sort_unstable();
    let mut expected = [&EXEMPLARS[..], &OTHER_ASSETS[..]].concat();
    expected.sort_unstable();
    assert_eq!(ids, expected);
    let bundles: Vec<&str> = source
        .bundles()
        .keys()
        .map(agent_grimoire_model::AssetId::as_str)
        .collect();
    assert_eq!(bundles, ["rust-cli", "security-baseline"]);
}

/// An asset's `min-cli-version` is at most the workspace version, so the CLI that
/// embeds the asset can install it (`docs/versioning.md` §4). Between releases
/// that version is the last release's, and on the release PR it is the one being
/// released. `cargo xtask release plan` checks the release itself.
#[test]
fn no_embedded_asset_needs_a_newer_grimoire() {
    let source = Source::builtin().unwrap();
    let cli = Version::parse(env!("CARGO_PKG_VERSION")).unwrap();
    let newer: Vec<String> = source
        .assets()
        .values()
        .map(|a| &a.manifest)
        .filter(|m| cli < m.min_cli_version)
        .map(|m| format!("{} (min-cli-version {})", m.id, m.min_cli_version))
        .collect();
    assert!(
        newer.is_empty(),
        "grimoire {cli} would refuse {} with resolve/cli-too-old; an asset that needs an unreleased CLI feature lands after the release that ships it",
        newer.join(", ")
    );
}

#[test]
fn every_exemplar_installs_verifies_and_uninstalls() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    ok(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    );
    let mut add = vec!["add"];
    add.extend_from_slice(&EXEMPLARS);
    add.extend_from_slice(&OTHER_ASSETS);
    add.extend_from_slice(&["--allow", "filesystem-read,hooks,env"]);
    ok(&source, &p, &add);
    ok(&source, &p, &["apply"]);
    insta::assert_snapshot!(snapshot_tree(&p));

    // A second apply changes nothing, and the result verifies.
    let second = ok(&source, &p, &["apply"]);
    assert!(second.contains("apply: 0 files changed"), "{second}");
    ok(&source, &p, &["plan", "--check"]);
    ok(&source, &p, &["verify"]);

    ok(&source, &p, &["uninstall"]);
    assert_eq!(p.tree(), ["grimoire.toml"], "nothing is left behind");
}

#[test]
fn the_bundles_install_their_members() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    ok(&source, &p, &["init", "--client", "claude-code"]);
    ok(
        &source,
        &p,
        &[
            "add",
            "bundle:rust-cli",
            "bundle:security-baseline",
            "--allow",
            "filesystem-read,hooks,env",
        ],
    );
    ok(&source, &p, &["apply"]);
    for path in [
        ".claude/rules/conventional-commits.md",
        ".claude/skills/adr-writer/SKILL.md",
        ".claude/agents/security-reviewer.md",
        ".claude/settings.json",
        ".mcp.json",
    ] {
        assert!(p.exists(path), "{path}");
    }
    let config = p.read("grimoire.toml");
    assert!(
        config.contains("project-docs = [\"filesystem-read\", \"env:CLAUDE_PROJECT_DIR\"]"),
        "{config}"
    );
    ok(&source, &p, &["verify"]);
}

#[test]
fn project_docs_can_be_configured_over_streamable_http() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"claude-code\"]\n\n[assets]\nproject-docs = { version = \"^0.1\", allow = [\"filesystem-read\", \"env:CLAUDE_PROJECT_DIR\"], transport = \"streamable-http\", url = \"http://127.0.0.1:8931/mcp\" }\n",
    );
    ok(&source, &p, &["apply"]);
    let mcp: serde_json::Value = serde_json::from_str(&p.read(".mcp.json")).unwrap();
    assert_eq!(
        mcp,
        serde_json::json!({ "mcpServers": { "project-docs": { "type": "http", "url": "http://127.0.0.1:8931/mcp" } } })
    );
}

#[test]
fn secret_guard_hooks_fail_closed_and_project_docs_forwards_no_environment() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    ok(&source, &p, &["init", "--client", "claude-code"]);
    ok(
        &source,
        &p,
        &[
            "add",
            "secret-guard",
            "project-docs",
            "--allow",
            "filesystem-read,hooks,env",
        ],
    );
    ok(&source, &p, &["apply"]);
    let settings: serde_json::Value =
        serde_json::from_str(&p.read(".claude/settings.json")).unwrap();
    let pre = &settings["hooks"]["PreToolUse"][0];
    assert!(
        pre.get("matcher").is_none(),
        "every tool call is checked: {pre}"
    );
    assert_eq!(pre["hooks"][0]["command"], "grimoire-tool-secret-guard");
    assert_eq!(pre["hooks"][0]["args"], serde_json::json!(["hook"]));
    assert_eq!(pre["hooks"][0]["onFailure"], "block");
    let prompt = &settings["hooks"]["UserPromptSubmit"][0];
    assert_eq!(prompt["hooks"][0]["onFailure"], "block");
    let mcp: serde_json::Value = serde_json::from_str(&p.read(".mcp.json")).unwrap();
    assert_eq!(
        mcp["mcpServers"]["project-docs"],
        serde_json::json!({ "type": "stdio", "command": "grimoire-mcp-project-docs" }),
        "Claude Code sets CLAUDE_PROJECT_DIR itself; nothing is forwarded"
    );
}

#[test]
fn fail_closed_hooks_warn_until_their_binary_is_on_path() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    ok(&source, &p, &["init", "--client", "claude-code"]);
    ok(
        &source,
        &p,
        &["add", "secret-guard", "--allow", "filesystem-read,hooks"],
    );
    let (exit, _, stderr) = grimoire(&source, &p, &["plan"]);
    assert_eq!(exit, Exit::Ok);
    assert!(stderr.contains("plan/hook-binary-missing"), "{stderr}");
    assert!(stderr.contains("disableAllHooks"), "{stderr}");

    let bin = tempfile::tempdir().unwrap();
    let name = if cfg!(windows) {
        "grimoire-tool-secret-guard.exe"
    } else {
        "grimoire-tool-secret-guard"
    };
    std::fs::write(bin.path().join(name), "").unwrap();
    let (exit, _, stderr) = grimoire_with_path(
        &source,
        &p,
        &["plan"],
        Some(bin.path().as_os_str().to_owned()),
    );
    assert_eq!(exit, Exit::Ok);
    assert!(!stderr.contains("plan/hook-binary-missing"), "{stderr}");
}

#[test]
fn server_options_on_other_kinds_and_client_set_variables_are_refused_clearly() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"claude-code\"]\n\n[assets]\nconventional-commits = { version = \"^0.1\", transport = \"streamable-http\", url = \"http://127.0.0.1:8931/mcp\" }\n",
    );
    let (exit, _, stderr) = grimoire(&source, &p, &["plan"]);
    assert_eq!(exit, Exit::Invalid);
    assert!(stderr.contains("resolve/options-kind"), "{stderr}");

    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"claude-code\"]\n\n[assets]\nproject-docs = { version = \"^0.1\", allow = [\"filesystem-read\", \"env:CLAUDE_PROJECT_DIR\"], env = { CLAUDE_PROJECT_DIR = \"MY_ROOT\" } }\n",
    );
    let (exit, _, stderr) = grimoire(&source, &p, &["plan"]);
    assert_eq!(exit, Exit::Invalid);
    assert!(
        stderr.contains("the client sets it, so it cannot be remapped"),
        "{stderr}"
    );
}

#[test]
fn doctor_needs_no_local_binary_for_a_server_over_http() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"claude-code\"]\n\n[assets]\nproject-docs = { version = \"^0.1\", allow = [\"filesystem-read\", \"env:CLAUDE_PROJECT_DIR\"], transport = \"streamable-http\", url = \"https://docs.example.com/mcp\" }\n",
    );
    ok(&source, &p, &["apply"]);
    let doctor = ok(&source, &p, &["doctor"]);
    assert!(
        doctor.contains("not needed here: configured over Streamable HTTP"),
        "{doctor}"
    );
}

/// Both bundles, which hold every exemplar, and the other assets, for all six
/// clients: every client table the catalog ships renders, installs, verifies and
/// uninstalls.
#[test]
fn every_exemplar_installs_for_every_client() {
    let source = Source::builtin().unwrap();
    let p = Project::new();
    let mut init = vec!["init"];
    for client in [
        "claude-code",
        "agents-md",
        "codex",
        "cursor",
        "gemini-cli",
        "copilot",
    ] {
        init.extend(["--client", client]);
    }
    ok(&source, &p, &init);
    ok(
        &source,
        &p,
        &[
            &["add", "rust-cli", "security-baseline"][..],
            &OTHER_ASSETS[..],
            &["--allow", "filesystem-read,hooks,env"][..],
        ]
        .concat(),
    );
    ok(&source, &p, &["apply"]);
    insta::assert_snapshot!(snapshot_tree(&p));
    let second = ok(&source, &p, &["apply"]);
    assert!(second.contains("apply: 0 files changed"), "{second}");
    ok(&source, &p, &["plan", "--check"]);
    ok(&source, &p, &["verify"]);
    ok(&source, &p, &["uninstall"]);
    assert_eq!(p.tree(), ["grimoire.toml"], "nothing is left behind");
}
