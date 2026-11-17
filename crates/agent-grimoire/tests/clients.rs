//! End-to-end behaviour of the Phase 4 clients (`codex`, `cursor`, `gemini-cli`,
//! `copilot`) together with `claude-code` and `agents-md` (ADR-0008).

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
}

fn grimoire(source: &Source, project: &Project, args: &[&str]) -> Ran {
    let mut stdin = Cursor::new(Vec::new());
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
            interactive: false,
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

const ALL_CLIENTS: [&str; 6] = [
    "claude-code",
    "agents-md",
    "codex",
    "cursor",
    "gemini-cli",
    "copilot",
];

/// A rule, a skill, an agent and an MCP server, each declared for every client
/// that has a destination for it.
fn catalog(rule_body: &str) -> Source {
    let c = FixtureCatalog::new();
    c.rule("commit-style", "1.0.0", &[], rule_body)
        .skill("adr-writer", "0.3.1")
        .agent("security-reviewer", "1.0.0", &["read", "search"])
        .mcp_server("docs-index", "2.0.0", &["DOCS_TOKEN"]);
    c.edit_manifest("rules/commit-style", |m| {
        let mut m = m;
        for (client, to, strategy) in [
            ("codex", "instructions", "block"),
            ("gemini-cli", "instructions", "block"),
            ("cursor", "rules:commit-style.md", "render"),
            ("copilot", "rules:commit-style.md", "render"),
        ] {
            write!(
                m,
                "\n[clients.{client}]\noutputs = [{{ src = \"rule.md\", to = \"{to}\", strategy = \"{strategy}\" }}]\n"
            )
            .unwrap();
        }
        m
    });
    c.edit_manifest("skills/adr-writer", |m| {
        let mut m = m;
        for client in ["codex", "cursor", "gemini-cli", "copilot"] {
            write!(
                m,
                "\n[clients.{client}]\noutputs = [\n  {{ src = \"SKILL.md\", to = \"skills:adr-writer/SKILL.md\", strategy = \"copy\" }},\n  {{ src = \"references\", to = \"skills:adr-writer/references\", strategy = \"copy\" }},\n]\n"
            )
            .unwrap();
        }
        m
    });
    c.edit_manifest("agents/security-reviewer", |m| {
        let mut m = m;
        for client in ["cursor", "gemini-cli", "copilot"] {
            write!(
                m,
                "\n[clients.{client}]\noutputs = [{{ src = \"agent.md\", to = \"agents:security-reviewer.md\", strategy = \"render\" }}]\n"
            )
            .unwrap();
        }
        m
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

/// The user's own content in every shared configuration file.
const CODEX_CONFIG: &str = "# My Codex settings.\nmodel = \"gpt-6\"\n\n[mcp_servers.theirs]\ncommand = \"their-server\" # keep me\n";
const GEMINI_SETTINGS: &str = "{\n  // My Gemini CLI settings.\n  \"theme\": \"dark\",\n  \"mcpServers\": {\n    /* theirs */ \"theirs\": { \"command\": \"their-server\" }\n  }\n}\n";
const CURSOR_MCP: &str =
    "{\n  \"mcpServers\": {\n    \"theirs\": { \"command\": \"their-server\" }\n  }\n}\n";

fn install(source: &Source) -> Project {
    let p = Project::new();
    p.write(".codex/config.toml", CODEX_CONFIG);
    p.write(".gemini/settings.json", GEMINI_SETTINGS);
    p.write(".cursor/mcp.json", CURSOR_MCP);
    p.write("GEMINI.md", "# Our project\n\nHuman-written guidance.\n");
    let mut init = vec!["init"];
    for client in ALL_CLIENTS {
        init.extend(["--client", client]);
    }
    grimoire(source, &p, &init).ok();
    grimoire(source, &p, &["add", "commit-style", "adr-writer"]).ok();
    grimoire(
        source,
        &p,
        &["add", "security-reviewer", "--allow", "filesystem-read"],
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

#[test]
fn every_client_installs_verifies_and_uninstalls() {
    let source = catalog("# Commits\n\nWrite Conventional Commits.\n");
    let p = install(&source);
    let mut tree = String::new();
    for entry in p.tree() {
        writeln!(tree, "== {entry}").unwrap();
        if !entry.ends_with('/') && entry != "grimoire.lock" {
            tree.push_str(&p.read(&entry));
        }
    }
    insta::assert_snapshot!(tree);

    // The user's own content survives in every shared file.
    assert!(
        p.read(".codex/config.toml")
            .contains("command = \"their-server\" # keep me")
    );
    assert!(
        p.read(".codex/config.toml")
            .starts_with("# My Codex settings.\n")
    );
    assert!(
        p.read(".gemini/settings.json")
            .contains("// My Gemini CLI settings.")
    );
    assert!(p.read(".gemini/settings.json").contains("/* theirs */"));

    grimoire(&source, &p, &["verify"]).ok();
    let second = grimoire(&source, &p, &["apply"]).ok();
    assert!(
        second.stdout.contains("apply: 0 files changed"),
        "{}",
        second.stdout
    );

    let doctor = grimoire(&source, &p, &["doctor"]);
    assert!(doctor.stdout.contains("client codex"), "{}", doctor.stdout);
    assert!(
        doctor.stdout.contains("marked trusted"),
        "{}",
        doctor.stdout
    );
    assert!(
        doctor.stdout.contains("copilot cloud agent")
            && doctor.stdout.contains("${COPILOT_MCP_DOCS_TOKEN}"),
        "{}",
        doctor.stdout
    );

    grimoire(&source, &p, &["uninstall"]).ok();
    assert_eq!(p.read(".codex/config.toml"), CODEX_CONFIG);
    assert_eq!(p.read(".gemini/settings.json"), GEMINI_SETTINGS);
    assert_eq!(p.read(".cursor/mcp.json"), CURSOR_MCP);
    assert_eq!(
        p.read("GEMINI.md"),
        "# Our project\n\nHuman-written guidance.\n"
    );
    for gone in [
        "AGENTS.md",
        "CLAUDE.md",
        ".mcp.json",
        ".agents",
        ".claude",
        ".github",
        ".gemini/agents",
        ".cursor/rules",
        ".cursor/agents",
    ] {
        assert!(!p.exists(gone), "{gone} should be gone");
    }
}

#[test]
fn codex_warns_when_agents_md_nears_its_budget() {
    let long = format!(
        "# Commits\n\n{}\n",
        "Write Conventional Commits. ".repeat(1000)
    );
    let source = catalog(&long);
    let p = Project::new();
    grimoire(&source, &p, &["init", "--client", "codex"]).ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    let plan = grimoire(&source, &p, &["plan"]);
    assert!(
        plan.stderr.contains("plan/codex-instructions-size"),
        "{}{}",
        plan.stdout,
        plan.stderr
    );
    let small = catalog("# Commits\n\nShort.\n");
    let q = Project::new();
    grimoire(&small, &q, &["init", "--client", "codex"]).ok();
    grimoire(&small, &q, &["add", "commit-style"]).ok();
    let plan = grimoire(&small, &q, &["plan"]);
    assert!(!plan.stderr.contains("plan/codex-instructions-size"));
}

#[test]
fn a_lock_may_name_grimoires_gemini_import() {
    let source = catalog("# Commits\n\nWrite Conventional Commits.\n");
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "agents-md", "--client", "gemini-cli"],
    )
    .ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    grimoire(&source, &p, &["apply"]).ok();
    assert!(p.read("GEMINI.md").contains("@AGENTS.md"));
    // Reading the lock back accepts GEMINI.md as Grimoire's own output.
    grimoire(&source, &p, &["plan", "--check"]).ok();
    let lock = p.read("grimoire.lock");
    let tampered = lock.replace("path = \"GEMINI.md\"", "path = \"README.md\"");
    assert_ne!(lock, tampered, "the lock records the GEMINI.md block");
    p.write("grimoire.lock", &tampered);
    let refused = grimoire(&source, &p, &["plan"]);
    assert!(
        refused.stderr.contains("project/lock-path"),
        "{}",
        refused.stderr
    );
}

#[test]
fn init_detects_every_client() {
    let source = catalog("# Commits\n");
    let p = Project::new();
    p.write(".codex/config.toml", "");
    p.write(".cursor/mcp.json", "{}\n");
    p.write("GEMINI.md", "# Notes\n");
    p.write(".github/copilot-instructions.md", "Be brief.\n");
    grimoire(&source, &p, &["init"]).ok();
    assert!(
        p.read("grimoire.toml")
            .contains("clients = [\"codex\", \"cursor\", \"gemini-cli\", \"copilot\"]"),
        "{}",
        p.read("grimoire.toml")
    );
}

fn tree(p: &Project) -> String {
    let mut out = String::new();
    for entry in p.tree() {
        writeln!(out, "== {entry}").unwrap();
        if !entry.ends_with('/') && entry != "grimoire.lock" {
            out.push_str(&p.read(&entry));
        }
    }
    out
}

fn set_clients(p: &Project, clients: &[&str]) {
    let text = p.read("grimoire.toml");
    let line = text
        .lines()
        .find(|l| l.starts_with("clients = "))
        .unwrap()
        .to_owned();
    let quoted: Vec<String> = clients.iter().map(|c| format!("\"{c}\"")).collect();
    p.write(
        "grimoire.toml",
        text.replace(&line, &format!("clients = [{}]", quoted.join(", "))),
    );
}

fn settled(source: &Source, p: &Project) {
    grimoire(source, p, &["verify"]).ok();
    let again = grimoire(source, p, &["apply"]).ok();
    assert!(
        again.stdout.contains("apply: 0 files changed"),
        "{}",
        again.stdout
    );
}

/// An edit to Grimoire's own entry in a client's configuration file is a
/// conflict, in TOML, JSONC and JSON alike; `--write-new` leaves the file alone
/// and `--force` restores only the entry.
#[test]
fn edits_to_grimoire_entries_are_conflicts() {
    let source = catalog("# Commits\n\nWrite Conventional Commits.\n");
    let p = install(&source);
    let edits = [
        (
            ".codex/config.toml",
            "command = \"grimoire-mcp-docs-index\"",
            "command = \"tampered\"",
        ),
        (
            ".cursor/mcp.json",
            "\"command\": \"grimoire-mcp-docs-index\"",
            "\"command\": \"tampered\"",
        ),
        (
            ".gemini/settings.json",
            "\"command\": \"grimoire-mcp-docs-index\"",
            "\"command\": \"tampered\"",
        ),
    ];
    let mut edited = Vec::new();
    for (path, from, to) in edits {
        let text = p.read(path);
        assert!(text.contains(from), "{path}: {text}");
        let changed = text.replace(from, to);
        p.write(path, &changed);
        edited.push((path, changed));
    }
    assert_eq!(grimoire(&source, &p, &["apply"]).exit, Exit::Conflict);
    assert_eq!(grimoire(&source, &p, &["uninstall"]).exit, Exit::Conflict);
    for (path, text) in &edited {
        assert_eq!(&p.read(path), text, "{path} untouched");
    }
    grimoire(&source, &p, &["apply", "--write-new"]);
    for (path, text) in &edited {
        assert_eq!(&p.read(path), text, "{path} untouched by --write-new");
        assert!(
            p.read(&format!("{path}.new"))
                .contains("grimoire-mcp-docs-index"),
            "{path}.new"
        );
        std::fs::remove_file(p.path(&format!("{path}.new"))).unwrap();
    }
    grimoire(&source, &p, &["apply", "--force"]).ok();
    assert!(
        p.read(".codex/config.toml")
            .contains("command = \"their-server\" # keep me")
    );
    assert!(
        p.read(".gemini/settings.json")
            .contains("// My Gemini CLI settings.")
    );
    for (path, _) in &edited {
        assert!(!p.read(path).contains("tampered"), "{path}");
    }
    settled(&source, &p);
}

/// Dropping and adding clients moves shared content between files and leaves
/// every user file as it was.
#[test]
fn clients_can_be_dropped_and_added_back() {
    let source = catalog("# Commits\n\nWrite Conventional Commits.\n");
    let p = install(&source);
    let full = tree(&p);
    // Without agents-md and codex, AGENTS.md goes and each client gets its own rule file.
    set_clients(&p, &["claude-code", "cursor", "gemini-cli", "copilot"]);
    grimoire(&source, &p, &["apply"]).ok();
    for gone in ["AGENTS.md", "CLAUDE.md"] {
        assert!(!p.exists(gone), "{gone}");
    }
    assert_eq!(p.read(".codex/config.toml"), CODEX_CONFIG);
    for rule in [
        ".claude/rules/commit-style.md",
        ".cursor/rules/commit-style.mdc",
        ".github/instructions/commit-style.instructions.md",
    ] {
        assert!(
            p.read(rule).contains("Write Conventional Commits."),
            "{rule}"
        );
    }
    let gemini = p.read("GEMINI.md");
    assert!(gemini.starts_with("# Our project\n\nHuman-written guidance.\n"));
    assert!(gemini.contains("Write Conventional Commits.") && !gemini.contains("@AGENTS.md"));
    settled(&source, &p);
    // Claude Code alone: every other client's file is back to the user's own.
    set_clients(&p, &["claude-code"]);
    grimoire(&source, &p, &["apply"]).ok();
    assert_eq!(p.read(".gemini/settings.json"), GEMINI_SETTINGS);
    assert_eq!(p.read(".cursor/mcp.json"), CURSOR_MCP);
    assert_eq!(
        p.read("GEMINI.md"),
        "# Our project\n\nHuman-written guidance.\n"
    );
    for gone in [
        ".agents",
        ".github",
        ".cursor/rules",
        ".cursor/agents",
        ".gemini/agents",
    ] {
        assert!(!p.exists(gone), "{gone}");
    }
    settled(&source, &p);
    // All six again: the same files as the first install.
    set_clients(&p, &ALL_CLIENTS);
    grimoire(&source, &p, &["apply"]).ok();
    assert_eq!(tree(&p), full);
    settled(&source, &p);
}

/// Cursor shares Claude Code's skill folder until another client needs
/// `.agents/skills/`.
#[test]
fn skills_move_when_codex_comes_and_goes() {
    let source = catalog("# Commits\n");
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "claude-code", "--client", "cursor"],
    )
    .ok();
    grimoire(&source, &p, &["add", "adr-writer"]).ok();
    grimoire(&source, &p, &["apply"]).ok();
    assert!(p.exists(".claude/skills/adr-writer/SKILL.md"));
    assert!(!p.exists(".agents"));
    set_clients(&p, &["claude-code", "codex", "cursor"]);
    grimoire(&source, &p, &["apply"]).ok();
    assert!(p.exists(".claude/skills/adr-writer/SKILL.md"));
    assert!(p.exists(".agents/skills/adr-writer/SKILL.md"));
    settled(&source, &p);
    set_clients(&p, &["claude-code", "cursor"]);
    grimoire(&source, &p, &["apply"]).ok();
    assert!(!p.exists(".agents"));
    settled(&source, &p);
}

fn copilot_check(source: &Source, p: &Project) -> Option<serde_json::Value> {
    let doctor = grimoire(source, p, &["doctor", "--json"]);
    let json: serde_json::Value = serde_json::from_str(&doctor.stdout).unwrap();
    let check = json["result"]["checks"]
        .as_array()
        .unwrap()
        .iter()
        .find(|c| c["name"] == "copilot cloud agent")?
        .clone();
    let detail = check["detail"].as_str().unwrap();
    let at = detail.find("{\"mcpServers\"").unwrap();
    Some(serde_json::from_str(&detail[at..]).unwrap())
}

#[test]
fn doctor_prints_the_copilot_cloud_agent_settings() {
    let source = catalog("# Commits\n");
    let p = install(&source);
    assert_eq!(
        copilot_check(&source, &p).unwrap(),
        serde_json::json!({ "mcpServers": { "docs-index": {
            "type": "local",
            "command": "grimoire-mcp-docs-index",
            "args": [],
            "env": { "DOCS_TOKEN": "${COPILOT_MCP_DOCS_TOKEN}" },
            "tools": ["*"],
        } } })
    );
    // A server configured over HTTP is given by its URL.
    let config = p.read("grimoire.toml");
    let line = config
        .lines()
        .find(|l| l.starts_with("docs-index = "))
        .unwrap()
        .to_owned();
    p.write(
        "grimoire.toml",
        config.replace(
            &line,
            "docs-index = { version = \"^2.0.0\", allow = [\"filesystem-read\", \"env:DOCS_TOKEN\"], transport = \"streamable-http\", url = \"https://docs.example.com/mcp\" }",
        ),
    );
    grimoire(&source, &p, &["apply"]).ok();
    assert_eq!(
        copilot_check(&source, &p).unwrap()["mcpServers"]["docs-index"],
        serde_json::json!({ "type": "http", "url": "https://docs.example.com/mcp", "tools": ["*"] })
    );
    // A server not installed for copilot isn't listed.
    p.write(
        "grimoire.toml",
        config.replace(
            &line,
            "docs-index = { version = \"^2.0.0\", allow = [\"filesystem-read\", \"env:DOCS_TOKEN\"], clients = [\"claude-code\"] }",
        ),
    );
    grimoire(&source, &p, &["apply"]).ok();
    assert_eq!(copilot_check(&source, &p), None);
}

#[test]
fn codex_warnings_cover_every_block_and_the_override() {
    let long = format!(
        "# Commits\n\n{}\n",
        "Write Conventional Commits. ".repeat(1000)
    );
    let source = catalog(&long);
    let p = Project::new();
    grimoire(
        &source,
        &p,
        &["init", "--client", "agents-md", "--client", "codex"],
    )
    .ok();
    grimoire(&source, &p, &["add", "commit-style"]).ok();
    // Restricted to agents-md, the block still counts against Codex's budget.
    let config = p.read("grimoire.toml").replace(
        "commit-style = \"^1.0.0\"",
        "commit-style = { version = \"^1.0.0\", clients = [\"agents-md\"] }",
    );
    p.write("grimoire.toml", &config);
    let plan = grimoire(&source, &p, &["plan"]);
    assert!(
        plan.stderr.contains("plan/codex-instructions-size"),
        "{}",
        plan.stderr
    );
    assert!(!plan.stderr.contains("plan/codex-override"));
    p.write("AGENTS.override.md", "");
    let plan = grimoire(&source, &p, &["plan"]);
    assert!(
        plan.stderr.contains("plan/codex-override"),
        "{}",
        plan.stderr
    );
    let doctor = grimoire(&source, &p, &["doctor"]);
    assert!(
        doctor.stdout.contains("AGENTS.override.md exists"),
        "{}",
        doctor.stdout
    );
}

/// A lock may name a client's configuration file only as the entries grimoire
/// owns in it, never as a whole file grimoire could delete.
#[test]
fn a_lock_may_not_claim_a_whole_config_file() {
    let source = catalog("# Commits\n");
    let p = install(&source);
    let lock = p.read("grimoire.lock");
    let at = lock.find("path = \".codex/config.toml\"").unwrap();
    let unit = at + lock[at..].find("unit = \"entry\"").unwrap();
    let tampered = format!(
        "{}unit = \"file\"{}",
        &lock[..unit],
        &lock[unit + "unit = \"entry\"".len()..]
    );
    p.write("grimoire.lock", &tampered);
    let refused = grimoire(&source, &p, &["plan"]);
    assert_eq!(refused.exit, Exit::Invalid);
    assert!(
        refused.stderr.contains("project/lock-path"),
        "{}",
        refused.stderr
    );
}
