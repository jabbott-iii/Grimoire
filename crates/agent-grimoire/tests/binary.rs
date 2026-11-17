//! The real `grimoire` binary: streams, exit statuses and the embedded catalog.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::process::{Command, Output, Stdio};

fn grimoire(dir: &std::path::Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grimoire"))
        .args(args)
        .current_dir(dir)
        .env_remove("CI")
        .stdin(Stdio::null())
        .output()
        .unwrap()
}

#[test]
fn help_and_version_succeed() {
    let dir = tempfile::tempdir().unwrap();
    let help = grimoire(dir.path(), &["--help"]);
    assert_eq!(help.status.code(), Some(0));
    let text = String::from_utf8(help.stdout).unwrap();
    assert!(text.contains("Exit status: 0 success"), "{text}");
    let version = grimoire(dir.path(), &["--version"]);
    assert_eq!(version.status.code(), Some(0));
    assert_eq!(
        String::from_utf8(version.stdout).unwrap().trim(),
        format!("grimoire {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn usage_errors_exit_2() {
    let dir = tempfile::tempdir().unwrap();
    let bad = grimoire(dir.path(), &["plan", "--nope"]);
    assert_eq!(bad.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&bad.stdout), "");
    assert!(String::from_utf8_lossy(&bad.stderr).contains("--nope"));
    let json = grimoire(dir.path(), &["--json", "plan", "--nope"]);
    assert_eq!(json.status.code(), Some(2));
    let doc: serde_json::Value = serde_json::from_slice(&json.stdout).unwrap();
    assert_eq!(doc["ok"], false);
    assert!(json.stderr.is_empty(), "--json keeps stderr quiet");
    let none = grimoire(dir.path(), &[]);
    assert_eq!(none.status.code(), Some(2), "a subcommand is required");
}

#[test]
fn empty_embedded_catalog_still_works_end_to_end() {
    let dir = tempfile::tempdir().unwrap();
    let list = grimoire(dir.path(), &["list", "--json"]);
    assert_eq!(list.status.code(), Some(0));
    let doc: serde_json::Value = serde_json::from_slice(&list.stdout).unwrap();
    assert_eq!(doc["command"], "list");
    assert!(doc["result"]["assets"].is_array());

    assert_eq!(
        grimoire(dir.path(), &["init", "--client", "agents-md"])
            .status
            .code(),
        Some(0)
    );
    assert_eq!(grimoire(dir.path(), &["apply"]).status.code(), Some(0));
    assert_eq!(
        grimoire(dir.path(), &["plan", "--check"]).status.code(),
        Some(0)
    );
    assert_eq!(grimoire(dir.path(), &["verify"]).status.code(), Some(0));
    let unknown = grimoire(dir.path(), &["add", "no-such-asset"]);
    assert_eq!(unknown.status.code(), Some(3));
    assert_eq!(
        grimoire(dir.path(), &["uninstall", "--purge"])
            .status
            .code(),
        Some(0)
    );
    assert_eq!(
        std::fs::read_dir(dir.path()).unwrap().count(),
        0,
        "uninstall --purge leaves nothing"
    );
}

#[test]
fn prompts_are_impossible_without_a_terminal() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(
        grimoire(dir.path(), &["init", "--client", "claude-code"])
            .status
            .code(),
        Some(0)
    );
    let doctor = grimoire(dir.path(), &["doctor", "--json"]);
    assert_eq!(
        doctor.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&doctor.stdout)
    );
}
