//! End-to-end behaviour of `grimoire` in temporary fixture repositories.
//!
//! Each test drives the CLI library with a fixture catalog and captured
//! streams, then inspects the project directory.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::fmt::Write as _;
use std::io::Cursor;

use agent_grimoire::{Context, Exit, run};
use agent_grimoire_catalog::Source;
use agent_grimoire_model::Version;
use agent_grimoire_testkit::{FixtureCatalog, Project};

struct Ran {
    exit: Exit,
    stdout: String,
    stderr: String,
}

impl Ran {
    fn ok(self) -> Self {
        assert_eq!(
            self.exit,
            Exit::Ok,
            "stdout:\n{}\nstderr:\n{}",
            self.stdout,
            self.stderr
        );
        self
    }

    fn json(&self) -> serde_json::Value {
        serde_json::from_str(&self.stdout).unwrap_or_else(|e| panic!("{e}: {}", self.stdout))
    }
}

fn grimoire_in(
    source: &Source,
    project: &Project,
    args: &[&str],
    input: &str,
    interactive: bool,
) -> Ran {
    let mut stdin = Cursor::new(input.as_bytes().to_vec());
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let exit = {
        let mut ctx = Context {
            source,
            version: Version::parse("0.1.0").unwrap(),
            cwd: project.root().to_path_buf(),
            stdin: &mut stdin,
            stdout: &mut stdout,
            stderr: &mut stderr,
            interactive,
            path: None,
        };
        let mut all = vec!["grimoire"];
        all.extend_from_slice(args);
        run(all, &mut ctx)
    };
    Ran {
        exit,
        stdout: String::from_utf8(stdout).unwrap(),
        stderr: String::from_utf8(stderr).unwrap(),
    }
}

fn grimoire(source: &Source, project: &Project, args: &[&str]) -> Ran {
    grimoire_in(source, project, args, "", false)
}

fn catalog() -> FixtureCatalog {
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
        &["**/*.rs"],
        "# Errors\n\nUse typed errors.\n",
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

/// A project with every kind installed for both clients, plus user content in shared files.
fn installed(source: &Source) -> Project {
    let p = Project::new();
    p.write("AGENTS.md", "# Our project\n\nHuman-written guidance.\n");
    p.write(
        ".mcp.json",
        "{\n  \"mcpServers\": {\n    \"theirs\": { \"command\": \"their-server\" }\n  }\n}\n",
    );
    p.write(
        ".claude/settings.json",
        "{\n  \"permissions\": { \"allow\": [\"Bash(ls)\"] },\n  \"hooks\": {\n    \"PreToolUse\": [\n      { \"matcher\": \"Bash\", \"hooks\": [{ \"type\": \"command\", \"command\": \"theirs\" }] }\n    ]\n  }\n}\n",
    );
    grimoire(
        source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        source,
        &p,
        &["add", "commit-style", "rust-errors", "adr-writer"],
    )
    .ok();
    grimoire(
        source,
        &p,
        &[
            "add",
            "security-baseline",
            "--allow",
            "filesystem-read,hooks",
        ],
    )
    .ok();
    grimoire(
        source,
        &p,
        &["add", "docs-index", "--allow", "filesystem-read,env"],
    )
    .ok();
    grimoire(source, &p, &["apply"]).ok();
    p
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

#[test]
fn init_add_apply_produces_every_output() {
    let source = catalog().source();
    let p = installed(&source);
    insta::assert_snapshot!(snapshot_tree(&p));
}

#[test]
fn lockfile_format() {
    let source = catalog().source();
    let p = installed(&source);
    let digest = regex_free_digests(&p.read("grimoire.lock"));
    insta::assert_snapshot!(digest);
}

/// The lock with digests shortened, so the snapshot shows its structure.
fn regex_free_digests(lock: &str) -> String {
    lock.lines()
        .map(|line| match line.find("sha256:") {
            Some(i) => format!("{}sha256:…\"", &line[..i]),
            None => line.to_owned(),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn second_apply_changes_nothing() {
    let source = catalog().source();
    let p = installed(&source);
    let before = snapshot_tree(&p);
    let lock = p.read("grimoire.lock");
    let second = grimoire(&source, &p, &["apply"]).ok();
    assert!(
        second.stdout.contains("apply: 0 files changed"),
        "{}",
        second.stdout
    );
    assert!(!second.stdout.contains("grimoire.lock updated"));
    assert_eq!(snapshot_tree(&p), before);
    assert_eq!(p.read("grimoire.lock"), lock);
    let check = grimoire(&source, &p, &["plan", "--check"]);
    assert_eq!(check.exit, Exit::Ok, "{}{}", check.stdout, check.stderr);
    grimoire(&source, &p, &["verify"]).ok();
}

#[test]
fn plan_check_reports_pending_changes() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "agents-md"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    let check = grimoire(&source, &p, &["plan", "--check"]);
    assert_eq!(check.exit, Exit::Check);
    assert!(
        check
            .stdout
            .contains("+<!-- grimoire:begin commit-style -->"),
        "diff shown:\n{}",
        check.stdout
    );
    assert!(!p.exists("AGENTS.md"), "plan writes nothing");
}

#[test]
fn edited_managed_content_is_never_overwritten_silently() {
    let source = catalog().source();
    let p = installed(&source);
    let edited = p.read(".claude/agents/security-reviewer.md") + "\nMy own note.\n";
    p.write(".claude/agents/security-reviewer.md", &edited);
    let agents = p
        .read("AGENTS.md")
        .replace("Write Conventional Commits.", "Write whatever.");
    p.write("AGENTS.md", &agents);
    // Make every other output change too, so a partial write would be visible.
    let source2 = {
        let c = catalog();
        c.edit_manifest("rules/rust-errors", |m| {
            m.replace("version = \"1.2.0\"", "version = \"1.3.0\"")
        })
        .write("rules/rust-errors/rule.md", "# Errors\n\nUse thiserror.\n")
        .write(
            "rules/rust-errors/CHANGELOG.md",
            "# Changelog\n\n## 1.3.0\n\n- New.\n",
        );
        c.source()
    };
    let plan = grimoire(&source2, &p, &["plan"]);
    assert_eq!(
        plan.exit,
        Exit::Invalid,
        "the lock pins 1.2.0: {}",
        plan.stderr
    );
    let before = snapshot_tree(&p);
    let plan = grimoire(&source, &p, &["plan"]);
    assert_eq!(plan.exit, Exit::Conflict, "{}", plan.stdout);
    assert!(plan.stdout.contains("conflict"), "{}", plan.stdout);
    let apply = grimoire(&source, &p, &["apply"]);
    assert_eq!(apply.exit, Exit::Conflict);
    assert!(
        apply.stderr.contains("edited since grimoire wrote it"),
        "{}",
        apply.stderr
    );
    assert_eq!(snapshot_tree(&p), before, "nothing written");

    // --write-new leaves both files alone and writes what grimoire would have written.
    let new = grimoire(&source, &p, &["apply", "--write-new"]);
    assert_eq!(new.exit, Exit::Conflict, "conflicts remain: {}", new.stderr);
    assert_eq!(p.read(".claude/agents/security-reviewer.md"), edited);
    assert!(
        p.read(".claude/agents/security-reviewer.md.new")
            .contains("You review changes carefully")
    );
    assert!(
        !p.read(".claude/agents/security-reviewer.md.new")
            .contains("My own note")
    );
    assert!(
        p.read("AGENTS.md.new")
            .contains("Write Conventional Commits.")
    );
    assert!(p.read("AGENTS.md.new").contains("Human-written guidance."));
    assert_eq!(
        grimoire(&source, &p, &["apply"]).exit,
        Exit::Conflict,
        "still conflicting"
    );

    // --force overwrites managed content but keeps the user's text around blocks.
    grimoire(&source, &p, &["apply", "--force"]).ok();
    assert!(
        !p.read(".claude/agents/security-reviewer.md")
            .contains("My own note")
    );
    let agents = p.read("AGENTS.md");
    assert!(
        agents.contains("Write Conventional Commits.")
            && agents.contains("Human-written guidance.")
    );
    grimoire(&source, &p, &["plan", "--check"]).ok();
}

#[test]
fn deleted_managed_content_and_unmanaged_files_conflict() {
    let source = catalog().source();
    let p = installed(&source);
    std::fs::remove_file(p.path(".claude/skills/adr-writer/references/notes.md")).unwrap();
    let plan = grimoire(&source, &p, &["plan"]);
    assert_eq!(plan.exit, Exit::Conflict);
    assert!(
        plan.stdout.contains("removed since grimoire wrote it"),
        "{}",
        plan.stdout
    );

    let fresh = Project::new();
    fresh.write(".claude/rules/commit-style.md", "my own rule\n");
    grimoire(&source, &fresh, &["init", "--client", "claude-code"]).ok();
    grimoire(&source, &fresh, &["add", "commit-style"]).ok();
    let apply = grimoire(&source, &fresh, &["apply"]);
    assert_eq!(apply.exit, Exit::Conflict);
    assert!(
        apply.stderr.contains("not managed by grimoire"),
        "{}",
        apply.stderr
    );
    assert_eq!(fresh.read(".claude/rules/commit-style.md"), "my own rule\n");
}

#[test]
fn identical_existing_content_is_adopted_after_an_interrupted_apply() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "claude-code"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    // As if a previous apply wrote the file and stopped before the lock.
    p.write(
        ".claude/rules/commit-style.md",
        "# Commits\n\nWrite Conventional Commits.\n",
    );
    let apply = grimoire(&source, &p, &["apply"]).ok();
    assert!(
        apply
            .stdout
            .contains("0 files changed, grimoire.lock updated"),
        "{}",
        apply.stdout
    );
    grimoire(&source, &p, &["verify"]).ok();
}

#[test]
fn remove_and_uninstall_leave_only_user_content() {
    let source = catalog().source();
    let p = installed(&source);
    grimoire(
        &source,
        &p,
        &["remove", "docs-index", "commit-style", "--apply"],
    )
    .ok();
    let mcp: serde_json::Value = serde_json::from_str(&p.read(".mcp.json")).unwrap();
    assert_eq!(
        mcp,
        serde_json::json!({"mcpServers": {"theirs": {"command": "their-server"}}})
    );
    assert!(!p.read("AGENTS.md").contains("Conventional Commits"));
    assert!(!p.exists(".claude/rules/commit-style.md"));

    let uninstall = grimoire(&source, &p, &["uninstall", "--purge"]).ok();
    assert!(uninstall.stdout.contains("uninstall"));
    assert_eq!(
        p.read("AGENTS.md"),
        "# Our project\n\nHuman-written guidance.\n"
    );
    assert_eq!(
        p.read(".mcp.json"),
        "{\n  \"mcpServers\": {\n    \"theirs\": { \"command\": \"their-server\" }\n  }\n}\n"
    );
    assert_eq!(
        p.read(".claude/settings.json"),
        "{\n  \"permissions\": { \"allow\": [\"Bash(ls)\"] },\n  \"hooks\": {\n    \"PreToolUse\": [\n      { \"matcher\": \"Bash\", \"hooks\": [{ \"type\": \"command\", \"command\": \"theirs\" }] }\n    ]\n  }\n}\n"
    );
    assert_eq!(
        p.tree(),
        [
            ".claude/",
            ".claude/settings.json",
            ".mcp.json",
            "AGENTS.md"
        ]
    );
}

#[test]
fn uninstall_from_scratch_leaves_nothing() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &["add", "commit-style", "rust-errors", "adr-writer"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &[
            "add",
            "security-baseline",
            "docs-index",
            "--allow",
            "filesystem-read,hooks,env",
        ],
    )
    .ok();
    grimoire(&source, &p, &["apply"]).ok();
    assert!(p.exists("CLAUDE.md") && p.exists(".mcp.json") && p.exists(".claude/settings.json"));
    grimoire(&source, &p, &["uninstall", "--purge"]).ok();
    assert_eq!(
        p.tree(),
        Vec::<String>::new(),
        "every file and directory grimoire created is gone"
    );
}

#[test]
fn dropping_a_client_removes_its_outputs_and_the_import() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &["add", "commit-style", "rust-errors", "--apply"],
    )
    .ok();
    assert_eq!(
        p.read("CLAUDE.md"),
        "<!-- grimoire:begin grimoire -->\n@AGENTS.md\n<!-- grimoire:end grimoire -->\n"
    );
    assert!(
        !p.exists(".claude/rules"),
        "rules reach Claude Code through AGENTS.md"
    );
    let config = p.read("grimoire.toml").replace(
        "clients = [\"claude-code\", \"agents-md\"]",
        "clients = [\"claude-code\"]",
    );
    p.write("grimoire.toml", &config);
    grimoire(&source, &p, &["apply"]).ok();
    assert!(!p.exists("CLAUDE.md") && !p.exists("AGENTS.md"));
    assert!(
        p.read(".claude/rules/rust-errors.md")
            .starts_with("---\npaths: [\"**/*.rs\"]\n---\n")
    );
}

#[test]
fn crlf_checkouts_are_not_drift() {
    let source = catalog().source();
    let p = installed(&source);
    for file in [
        "AGENTS.md",
        ".claude/agents/security-reviewer.md",
        "CLAUDE.md",
    ] {
        let text = p.read(file).replace('\n', "\r\n");
        p.write(file, &text);
    }
    grimoire(&source, &p, &["plan", "--check"]).ok();
    grimoire(&source, &p, &["verify"]).ok();
    let apply = grimoire(&source, &p, &["apply"]).ok();
    assert!(apply.stdout.contains("0 files changed"), "{}", apply.stdout);
    assert!(
        p.read("AGENTS.md").contains("\r\n"),
        "line endings are left alone"
    );
}

#[test]
fn verify_reports_mismatches() {
    let source = catalog().source();
    let p = installed(&source);
    p.write(".claude/skills/adr-writer/SKILL.md", "tampered\n");
    let verify = grimoire(&source, &p, &["verify", "--json"]);
    assert_eq!(verify.exit, Exit::Check);
    let json = verify.json();
    assert_eq!(json["ok"], false);
    assert_eq!(
        json["result"]["mismatches"][0]["path"],
        ".claude/skills/adr-writer/SKILL.md"
    );
    assert!(verify.stderr.is_empty(), "--json keeps stderr quiet");
}

#[test]
fn locked_versions_hold_until_update() {
    let p = installed(&catalog().source());
    let newer = {
        let c = catalog();
        c.edit_manifest("rules/rust-errors", |m| {
            m.replace("version = \"1.2.0\"", "version = \"1.3.0\"")
        })
        .write(
            "rules/rust-errors/rule.md",
            "# Errors\n\nUse thiserror in libraries.\n",
        )
        .write(
            "rules/rust-errors/CHANGELOG.md",
            "# Changelog\n\n## 1.3.0\n\n- New.\n",
        );
        c.source()
    };
    let apply = grimoire(&newer, &p, &["apply"]);
    assert_eq!(apply.exit, Exit::Invalid);
    assert!(
        apply.stderr.contains("grimoire update rust-errors"),
        "{}",
        apply.stderr
    );
    grimoire(&newer, &p, &["update", "rust-errors", "--no-apply"]).ok();
    assert!(p.read("grimoire.lock").contains("version = \"1.3.0\""));
    assert!(
        !p.read("AGENTS.md").contains("thiserror"),
        "--no-apply leaves files alone"
    );
    grimoire(&newer, &p, &["verify"]).ok();
    grimoire(&newer, &p, &["apply"]).ok();
    assert!(p.read("AGENTS.md").contains("Use thiserror in libraries."));
    grimoire(&newer, &p, &["plan", "--check"]).ok();
}

#[test]
fn permissions_need_consent() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "claude-code"]).ok();
    let config = p.read("grimoire.toml");
    let refused = grimoire(&source, &p, &["add", "secret-guard"]);
    assert_eq!(refused.exit, Exit::Invalid);
    assert!(
        refused.stderr.contains("--allow filesystem-read,hooks"),
        "{}",
        refused.stderr
    );
    assert_eq!(p.read("grimoire.toml"), config, "nothing written");

    let partial = grimoire(&source, &p, &["add", "secret-guard", "--allow", "hooks"]);
    assert_eq!(partial.exit, Exit::Invalid);

    let declined = grimoire_in(&source, &p, &["add", "secret-guard"], "n\n", true);
    assert_eq!(declined.exit, Exit::Invalid);
    let accepted = grimoire_in(&source, &p, &["add", "secret-guard"], "y\n", true).ok();
    assert!(
        accepted
            .stderr
            .contains("Allow filesystem-read, hooks for `secret-guard`?")
    );
    let written = p.read("grimoire.toml");
    assert!(
        written.contains(
            "secret-guard = { version = \"^1.0.0\", allow = [\"filesystem-read\", \"hooks\"] }"
        ),
        "{written}"
    );

    // --no-input never prompts, even on a terminal.
    let q = Project::new();
    grimoire(&source, &q, &["init", "--client", "claude-code"]).ok();
    assert_eq!(
        grimoire_in(
            &source,
            &q,
            &["add", "secret-guard", "--no-input"],
            "y\n",
            true
        )
        .exit,
        Exit::Invalid
    );

    // Bundle members are granted under [allow].
    grimoire(
        &source,
        &q,
        &[
            "add",
            "bundle:security-baseline",
            "--allow",
            "filesystem-write,hooks",
        ],
    )
    .ok();
    let written = q.read("grimoire.toml");
    assert!(
        written.contains("[allow]\nsecret-guard = [\"filesystem-read\", \"hooks\"]"),
        "{written}"
    );
}

#[test]
fn growing_permissions_stop_until_accepted() {
    let p = installed(&catalog().source());
    let newer = {
        let c = catalog();
        c.edit_manifest("tools/secret-guard", |m| {
            m.replace("version = \"1.0.0\"", "version = \"1.1.0\"").replace("hooks = true\n", "hooks = true\nnetwork = true\n")
        })
        .write("tools/secret-guard/CHANGELOG.md", "# Changelog\n\n## 1.1.0\n\n- Reports.\n")
        .write("tools/secret-guard/Cargo.toml", "[package]\nname = \"grimoire-tool-secret-guard\"\nversion = \"1.1.0\"\nedition = \"2024\"\npublish = false\n");
        c.source()
    };
    let update = grimoire(&newer, &p, &["update"]);
    assert_eq!(update.exit, Exit::Invalid);
    assert!(
        update
            .stderr
            .contains("needs permissions you have not accepted: network"),
        "{}",
        update.stderr
    );
}

#[test]
fn reading_one_more_variable_stops_until_that_variable_is_accepted() {
    let p = installed(&catalog().source());
    let newer = {
        let c = catalog();
        c.edit_manifest("mcp-servers/docs-index", |m| {
            m.replace("version = \"2.0.0\"", "version = \"2.1.0\"")
                .replace("env = [\"DOCS_TOKEN\"]", "env = [\"DOCS_HOST\", \"DOCS_TOKEN\"]")
                .replace(
                    "env = [{ name = \"DOCS_TOKEN\"",
                    "env = [{ name = \"DOCS_HOST\", required = false, secret = false, description = \"Host\" }, { name = \"DOCS_TOKEN\"",
                )
        })
        .write("mcp-servers/docs-index/CHANGELOG.md", "# Changelog\n\n## 2.1.0\n\n- Host.\n")
        .write("mcp-servers/docs-index/Cargo.toml", "[package]\nname = \"grimoire-mcp-docs-index\"\nversion = \"2.1.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nagent-grimoire-mcp.workspace = true\n");
        c.source()
    };
    // The `env:DOCS_TOKEN` grant does not cover a second variable.
    let update = grimoire(&newer, &p, &["update"]);
    assert_eq!(update.exit, Exit::Invalid, "{}", update.stderr);
    assert!(
        update
            .stderr
            .contains("needs permissions you have not accepted: env:DOCS_HOST"),
        "{}",
        update.stderr
    );
    assert!(
        update.stderr.contains("--allow env:DOCS_HOST"),
        "{}",
        update.stderr
    );
    // A bare `env` on the command line is written out name by name.
    grimoire(&newer, &p, &["add", "docs-index", "--allow", "env"]).ok();
    let config = p.read("grimoire.toml");
    assert!(
        config.contains(
            "docs-index = { version = \"^2.0.0\", allow = [\"filesystem-read\", \"env:DOCS_HOST\", \"env:DOCS_TOKEN\"] }"
        ),
        "{config}"
    );
    grimoire(&newer, &p, &["update"]).ok();
}

#[test]
fn a_bare_env_grant_in_grimoire_toml_is_refused() {
    let source = catalog().source();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"claude-code\"]\n[assets]\ndocs-index = { version = \"^2\", allow = [\"filesystem-read\", \"env\"] }\n",
    );
    let r = grimoire(&source, &p, &["plan"]);
    assert_eq!(r.exit, Exit::Invalid, "{}", r.stderr);
    assert!(r.stderr.contains("env:<NAME>"), "{}", r.stderr);
}

#[test]
fn tampered_locks_cannot_reach_outside_the_project() {
    let source = catalog().source();
    for bad in [
        "../outside.md",
        "/etc/passwd",
        "C:/Windows/x",
        "aux.md",
        "a/CON",
        "x/../../y",
        ".claude//x",
    ] {
        let p = installed(&source);
        let lock = p.read("grimoire.lock").replacen(
            "path = \".claude/agents/security-reviewer.md\"",
            &format!("path = {bad:?}"),
            1,
        );
        p.write("grimoire.lock", &lock);
        let r = grimoire(&source, &p, &["uninstall"]);
        assert_eq!(r.exit, Exit::Invalid, "{bad}: {}", r.stderr);
        assert!(
            p.exists(".claude/agents/security-reviewer.md"),
            "{bad}: nothing removed"
        );
    }
}

#[cfg(unix)]
#[test]
fn symlinks_are_refused() {
    use std::os::unix::fs::symlink;
    let source = catalog().source();
    let outside = tempfile::tempdir().unwrap();

    // A symlinked directory on the way to a managed path.
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "claude-code"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    symlink(outside.path(), p.path(".claude")).unwrap();
    let r = grimoire(&source, &p, &["apply"]);
    assert_eq!(r.exit, Exit::Refused, "{}", r.stderr);
    assert!(
        std::fs::read_dir(outside.path()).unwrap().next().is_none(),
        "nothing written outside"
    );

    // A symlinked managed file.
    let p = Project::new();
    std::fs::write(outside.path().join("target.md"), "outside\n").unwrap();
    symlink(outside.path().join("target.md"), p.path("AGENTS.md")).unwrap();
    grimoire(&source, &p, &["init", "--client", "agents-md"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    assert_eq!(grimoire(&source, &p, &["apply"]).exit, Exit::Refused);
    assert_eq!(
        std::fs::read_to_string(outside.path().join("target.md")).unwrap(),
        "outside\n"
    );

    // A symlinked grimoire.toml.
    let p = Project::new();
    std::fs::write(
        outside.path().join("grimoire.toml"),
        "format = 1\nclients = [\"agents-md\"]\n",
    )
    .unwrap();
    symlink(
        outside.path().join("grimoire.toml"),
        p.path("grimoire.toml"),
    )
    .unwrap();
    assert_eq!(grimoire(&source, &p, &["plan"]).exit, Exit::Refused);
}

#[test]
fn broken_shared_files_are_refused_even_with_force() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &[
            "add",
            "commit-style",
            "docs-index",
            "--allow",
            "filesystem-read,env",
        ],
    )
    .ok();
    p.write(
        "AGENTS.md",
        "# Ours\n<!-- grimoire:begin commit-style -->\nunterminated\n",
    );
    p.write(".mcp.json", "{ // comments are not JSON\n}\n");
    let r = grimoire(&source, &p, &["apply", "--force"]);
    assert_eq!(r.exit, Exit::Conflict);
    assert!(r.stderr.contains("never closed"), "{}", r.stderr);
    assert!(r.stderr.contains("not valid JSON"), "{}", r.stderr);
    assert_eq!(
        p.read("AGENTS.md"),
        "# Ours\n<!-- grimoire:begin commit-style -->\nunterminated\n"
    );
    assert!(!p.exists(".claude/rules"), "nothing was written anywhere");
}

#[test]
fn json_envelopes() {
    let source = catalog().source();
    let p = Project::new();
    let init = grimoire(&source, &p, &["--json", "init", "--client", "agents-md"]).ok();
    assert_eq!(init.json()["command"], "init");
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    let plan = grimoire(&source, &p, &["plan", "--json"]).ok();
    let json = plan.json();
    assert_eq!(json["format"], 1);
    assert_eq!(json["ok"], true);
    assert_eq!(json["result"]["summary"]["create"], 1);
    assert_eq!(json["result"]["files"][0]["path"], "AGENTS.md");
    assert!(
        json["result"]["files"][0]["diff"]
            .as_str()
            .unwrap()
            .contains("+Write Conventional Commits.")
    );
    let list = grimoire(&source, &p, &["list", "--json", "--kind", "rule"])
        .ok()
        .json();
    assert_eq!(list["result"]["assets"].as_array().unwrap().len(), 2);
    let show = grimoire(&source, &p, &["show", "docs-index", "--json"])
        .ok()
        .json();
    assert_eq!(
        show["result"]["permissions"],
        serde_json::json!(["filesystem-read", "env:DOCS_TOKEN"])
    );
    assert_eq!(
        show["result"]["clients"]["claude-code"][0],
        ".mcp.json (mcpServers.docs-index)"
    );
    let usage = grimoire(&source, &p, &["--json", "plan", "--nope"]);
    assert_eq!(usage.exit, Exit::Usage);
    assert_eq!(usage.json()["diagnostics"][0]["code"], "cli/usage");
}

#[test]
fn errors_have_distinct_exit_codes() {
    let source = catalog().source();
    let p = Project::new();
    assert_eq!(
        grimoire(&source, &p, &["plan"]).exit,
        Exit::Invalid,
        "no grimoire.toml"
    );
    assert_eq!(
        grimoire(&source, &p, &["init"]).exit,
        Exit::Usage,
        "nothing to detect"
    );
    assert_eq!(
        grimoire(&source, &p, &["init", "--client", "nope"]).exit,
        Exit::Usage,
        "unknown client"
    );
    p.write("AGENTS.md", "x\n");
    grimoire(&source, &p, &["init"]).ok();
    assert!(
        p.read("grimoire.toml")
            .contains("clients = [\"agents-md\"]")
    );
    assert_eq!(
        grimoire(&source, &p, &["init"]).exit,
        Exit::Invalid,
        "already exists"
    );
    assert_eq!(grimoire(&source, &p, &["add", "nope"]).exit, Exit::Invalid);
    assert_eq!(
        grimoire(&source, &p, &["remove", "commit-style"]).exit,
        Exit::Invalid,
        "not in the config"
    );
    assert_eq!(
        grimoire(&source, &p, &["verify"]).exit,
        Exit::Invalid,
        "no lock yet"
    );
    assert_eq!(grimoire(&source, &p, &["frobnicate"]).exit, Exit::Usage);
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"agents-md\"]\nunknown = true\n",
    );
    assert_eq!(
        grimoire(&source, &p, &["plan"]).exit,
        Exit::Invalid,
        "unknown fields are errors"
    );
}

#[test]
fn doctor_reports_missing_binaries() {
    let source = catalog().source();
    let p = installed(&source);
    let doctor = grimoire(&source, &p, &["doctor"]);
    assert_eq!(doctor.exit, Exit::Check);
    assert!(
        doctor
            .stdout
            .contains("FAIL binary grimoire-tool-secret-guard"),
        "{}",
        doctor.stdout
    );
    assert!(
        doctor.stdout.contains("ok   outputs: 0 of"),
        "{}",
        doctor.stdout
    );
}

#[test]
fn files_grimoire_did_not_create_are_never_deleted() {
    let source = catalog().source();
    let p = Project::new();
    p.write("AGENTS.md", "");
    p.write(".mcp.json", "{\"mcpServers\": {}}");
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &[
            "add",
            "commit-style",
            "docs-index",
            "--allow",
            "filesystem-read,env",
            "--apply",
        ],
    )
    .ok();
    assert!(p.read("AGENTS.md").contains("Conventional Commits"));
    assert!(p.read(".mcp.json").contains("grimoire-mcp-docs-index"));
    grimoire(&source, &p, &["uninstall"]).ok();
    assert_eq!(p.read("AGENTS.md"), "", "the user's empty file stays");
    assert_eq!(
        p.read(".mcp.json"),
        "{\"mcpServers\": {}}",
        "and their empty container"
    );
    assert!(
        !p.exists("CLAUDE.md"),
        "the import block's file was Grimoire's"
    );
}

#[test]
fn user_text_added_to_a_grimoire_file_keeps_it() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(&source, &p, &["add", "commit-style", "--apply"]).ok();
    let claude = p.read("CLAUDE.md") + "\n# My own notes\n";
    p.write("CLAUDE.md", &claude);
    grimoire(&source, &p, &["uninstall"]).ok();
    assert_eq!(p.read("CLAUDE.md"), "\n# My own notes\n");
    assert!(!p.exists("AGENTS.md"));
}

#[test]
fn rule_bodies_without_one_final_newline_stay_unchanged() {
    for body in [
        "Write Conventional Commits.",
        "Write Conventional Commits.\n\n\n",
    ] {
        let c = FixtureCatalog::new();
        c.rule("commit-style", "1.0.0", &[], body);
        let source = c.source();
        let p = Project::new();
        grimoire(&source, &p, &["init", "--client", "agents-md"]).ok();
        grimoire(&source, &p, &["add", "commit-style", "--apply"]).ok();
        grimoire(&source, &p, &["verify"]).ok();
        grimoire(&source, &p, &["plan", "--check"]).ok();
        let again = grimoire(&source, &p, &["apply"]).ok();
        assert!(
            again.stdout.contains("0 files changed"),
            "{body:?}: {}",
            again.stdout
        );
    }
}

#[test]
fn allow_never_reaches_assets_the_command_does_not_name() {
    let p = installed(&catalog().source());
    // A newer secret-guard needs `network` as well.
    let newer = {
        let c = catalog();
        c.edit_manifest("tools/secret-guard", |m| {
            m.replace("hooks = true\n", "hooks = true\nnetwork = true\n")
        })
        .rule(
            "extra-rule",
            "1.0.0",
            &[],
            "A rule that is added later on.\n",
        );
        c.source()
    };
    let before = p.read("grimoire.toml");
    let add = grimoire(&newer, &p, &["add", "extra-rule", "--allow", "network"]);
    assert_eq!(
        add.exit,
        Exit::Invalid,
        "secret-guard still lacks consent: {}",
        add.stderr
    );
    assert!(add.stderr.contains("secret-guard"), "{}", add.stderr);
    assert_eq!(
        p.read("grimoire.toml"),
        before,
        "nothing was granted silently"
    );
    let named = grimoire(
        &newer,
        &p,
        &["add", "bundle:security-baseline", "--allow", "network"],
    )
    .ok();
    assert!(
        named.stdout.contains("allowed network for secret-guard"),
        "{}",
        named.stdout
    );
}

#[test]
fn removing_an_asset_that_stays_selected_keeps_its_grant() {
    let source = catalog().source();
    let p = installed(&source);
    let before = p.read("grimoire.toml");
    let indirect = grimoire(&source, &p, &["remove", "secret-guard"]);
    assert_eq!(indirect.exit, Exit::Invalid);
    assert!(indirect.stderr.contains("exclude"), "{}", indirect.stderr);
    assert_eq!(p.read("grimoire.toml"), before);

    // Direct and also in the bundle: the grant moves to [allow].
    let both = Project::new();
    grimoire(&source, &both, &["init", "--client", "claude-code"]).ok();
    grimoire(
        &source,
        &both,
        &["add", "secret-guard", "--allow", "filesystem-read,hooks"],
    )
    .ok();
    grimoire(
        &source,
        &both,
        &[
            "add",
            "bundle:security-baseline",
            "--allow",
            "filesystem-read",
        ],
    )
    .ok();
    let removed = grimoire(&source, &both, &["remove", "secret-guard"]).ok();
    assert!(
        removed.stderr.contains("remove/still-selected"),
        "{}",
        removed.stderr
    );
    assert!(
        both.read("grimoire.toml")
            .contains("secret-guard = [\"filesystem-read\", \"hooks\"]")
    );
    grimoire(&source, &both, &["plan"]).ok();
}

#[test]
fn doctor_fails_when_the_project_does_not_resolve() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "agents-md"]).ok();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"agents-md\"]\n[assets]\ntypo = \"^1\"\n",
    );
    let doctor = grimoire(&source, &p, &["doctor", "--json"]);
    assert_eq!(doctor.exit, Exit::Check);
    assert_eq!(doctor.json()["ok"], false);
}

#[test]
fn crlf_lockfiles_are_not_changes() {
    let source = catalog().source();
    let p = installed(&source);
    let lock = p.read("grimoire.lock").replace('\n', "\r\n");
    p.write("grimoire.lock", &lock);
    grimoire(&source, &p, &["plan", "--check"]).ok();
    let apply = grimoire(&source, &p, &["apply"]).ok();
    assert!(
        !apply.stdout.contains("grimoire.lock updated"),
        "{}",
        apply.stdout
    );
    assert_eq!(p.read("grimoire.lock"), lock);
}

#[test]
fn inline_assets_tables_can_be_added_to() {
    let source = catalog().source();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"agents-md\"]\nassets = { commit-style = \"^1\" }\n",
    );
    grimoire(&source, &p, &["add", "rust-errors"]).ok();
    assert!(
        p.read("grimoire.toml").contains("rust-errors"),
        "{}",
        p.read("grimoire.toml")
    );
}

#[test]
fn bundles_gone_from_the_catalog_can_still_be_removed() {
    let source = catalog().source();
    let p = Project::new();
    p.write(
        "grimoire.toml",
        "format = 1\nclients = [\"agents-md\"]\nbundles = [\"retired\"]\n",
    );
    grimoire(&source, &p, &["remove", "retired"]).ok();
    assert!(!p.read("grimoire.toml").contains("retired"));
}

#[test]
fn locks_naming_unmanaged_paths_are_refused() {
    let source = catalog().source();
    for bad in [".git/HEAD", ".env", "grimoire.toml", "src/main.rs"] {
        let p = installed(&source);
        p.write(bad, "precious\n");
        let lock = p.read("grimoire.lock").replacen(
            "path = \".claude/agents/security-reviewer.md\"",
            &format!("path = {bad:?}"),
            1,
        );
        p.write("grimoire.lock", &lock);
        for cmd in [&["plan"][..], &["uninstall"][..]] {
            let r = grimoire(&source, &p, cmd);
            assert_eq!(r.exit, Exit::Invalid, "{bad} {cmd:?}: {}", r.stderr);
            assert!(!r.stdout.contains("precious"), "{bad}: content leaked");
        }
        assert_eq!(p.read(bad), "precious\n");
    }
}

#[test]
fn an_edited_hook_is_a_conflict_not_forgotten() {
    let source = catalog().source();
    let p = installed(&source);
    let settings = p
        .read(".claude/settings.json")
        .replace("\"timeout\": 10", "\"timeout\": 60");
    p.write(".claude/settings.json", &settings);
    let r = grimoire(
        &source,
        &p,
        &["remove", "bundle:security-baseline", "--apply"],
    );
    assert_eq!(r.exit, Exit::Conflict, "{}", r.stderr);
    assert!(r.stderr.contains("edited or removed"), "{}", r.stderr);
    assert!(
        p.read("grimoire.lock").contains("secret-guard"),
        "still tracked"
    );
}

#[test]
fn write_new_never_replaces_a_different_sidecar() {
    let source = catalog().source();
    let p = installed(&source);
    let edited = p
        .read("AGENTS.md")
        .replace("Write Conventional Commits.", "Mine.");
    p.write("AGENTS.md", &edited);
    p.write("AGENTS.md.new", "my merge in progress\n");
    let r = grimoire(&source, &p, &["apply", "--write-new"]);
    assert_eq!(r.exit, Exit::Conflict);
    assert!(r.stderr.contains("AGENTS.md.new"), "{}", r.stderr);
    assert_eq!(p.read("AGENTS.md.new"), "my merge in progress\n");
    std::fs::remove_file(p.path("AGENTS.md.new")).unwrap();
    assert_eq!(
        grimoire(&source, &p, &["apply", "--write-new"]).exit,
        Exit::Conflict
    );
    let first = p.read("AGENTS.md.new");
    assert_eq!(
        grimoire(&source, &p, &["apply", "--write-new"]).exit,
        Exit::Conflict,
        "rerun is fine"
    );
    assert_eq!(p.read("AGENTS.md.new"), first);
}

#[test]
fn things_in_the_way_are_conflicts() {
    let source = catalog().source();
    let p = Project::new();
    p.write(".claude", "not a directory\n");
    grimoire(&source, &p, &["init", "--client", "claude-code"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    assert_eq!(grimoire(&source, &p, &["plan"]).exit, Exit::Conflict);
}

#[test]
fn update_no_apply_refuses_unreadable_files() {
    let source = catalog().source();
    let p = installed(&source);
    p.write(".mcp.json", "{ // a comment\n}\n");
    let lock = p.read("grimoire.lock");
    assert_eq!(
        grimoire(&source, &p, &["update", "--no-apply"]).exit,
        Exit::Conflict
    );
    assert_eq!(p.read("grimoire.lock"), lock);
}

#[test]
fn hints_name_flags_that_work() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "claude-code"]).ok();
    let partial = grimoire(&source, &p, &["add", "secret-guard", "--allow", "hooks"]);
    assert!(
        partial.stderr.contains("--allow filesystem-read,hooks"),
        "{}",
        partial.stderr
    );

    let q = installed(&source);
    let edited = q.read(".claude/agents/security-reviewer.md") + "mine\n";
    q.write(".claude/agents/security-reviewer.md", &edited);
    let uninstall = grimoire(&source, &q, &["uninstall"]);
    assert_eq!(uninstall.exit, Exit::Conflict);
    assert!(
        uninstall.stderr.contains("--force") && !uninstall.stderr.contains("--write-new"),
        "{}",
        uninstall.stderr
    );
    let add = grimoire(&source, &q, &["add", "adr-writer", "--apply"]);
    assert!(
        add.stderr.contains("grimoire.toml was updated"),
        "{}",
        add.stderr
    );
}

#[test]
fn a_lost_lock_still_lets_uninstall_clean_up() {
    let source = catalog().source();
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "agents-md"],
    )
    .ok();
    grimoire(
        &source,
        &p,
        &[
            "add",
            "commit-style",
            "docs-index",
            "--allow",
            "filesystem-read,env",
            "--apply",
        ],
    )
    .ok();
    // As if the first apply stopped before writing the lock.
    std::fs::remove_file(p.path("grimoire.lock")).unwrap();
    let again = grimoire(&source, &p, &["apply"]).ok();
    assert!(again.stdout.contains("0 files changed"), "{}", again.stdout);
    grimoire(&source, &p, &["uninstall", "--purge"]).ok();
    assert_eq!(p.tree(), Vec::<String>::new());
}
