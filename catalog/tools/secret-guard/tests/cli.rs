//! The real `grimoire-tool-secret-guard` binary: streams, exit statuses and
//! the Claude Code hook protocol.
//!
//! Synthetic credentials are assembled at run time, so that no literal in this
//! repository looks like a real secret to a scanner.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::io::Write as _;
use std::process::{Command, Output, Stdio};

/// A token-shaped value: 36 varied base62 characters after `ghp_`.
fn token() -> String {
    let alphabet: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
        .chars()
        .collect();
    let body: String = (0..36_usize)
        .map(|i| alphabet[(i * 37 + 11) % 62])
        .collect();
    format!("ghp_{body}")
}

fn guard(args: &[&str], stdin: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_grimoire-tool-secret-guard"))
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    // The child may exit before reading everything; a broken pipe is fine.
    let _ = child.stdin.take().unwrap().write_all(stdin);
    child.wait_with_output().unwrap()
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8(bytes.to_vec()).unwrap()
}

#[test]
fn scan_reports_findings_on_stdout_and_exits_1() {
    let out = guard(&["scan"], format!("key: {}\n", token()).as_bytes());
    assert_eq!(out.status.code(), Some(1));
    let stdout = text(&out.stdout);
    assert!(
        stdout.starts_with("<stdin>:1:6: github-token: "),
        "{stdout}"
    );
    assert!(!stdout.contains(&token()), "the value is never printed");
    assert!(text(&out.stderr).contains("1 likely secret(s)"));
}

#[test]
fn a_clean_scan_exits_0() {
    let out = guard(&["scan", "-"], b"nothing to see\n");
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(text(&out.stdout), "");
}

#[test]
fn scan_json_is_one_document() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("a.txt"), token()).unwrap();
    let missing = dir.path().join("missing");
    let out = guard(
        &[
            "scan",
            "--json",
            dir.path().to_str().unwrap(),
            missing.to_str().unwrap(),
        ],
        b"",
    );
    assert_eq!(out.status.code(), Some(3), "an unreadable input wins");
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(doc["format"], 1);
    assert_eq!(doc["findings"][0]["rule"], "github-token");
    assert_eq!(doc["findings"][0]["line"], 1);
    assert_eq!(doc["errors"].as_array().unwrap().len(), 1);
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn usage_errors_exit_2() {
    let out = guard(&["scan", "--nope"], b"");
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(text(&out.stdout), "");
    let none = guard(&[], b"");
    assert_eq!(none.status.code(), Some(2));
}

#[test]
fn help_documents_exit_statuses_and_version_matches() {
    let help = guard(&["--help"], b"");
    assert_eq!(help.status.code(), Some(0));
    assert!(text(&help.stdout).contains("hook:  0 allow, 2 block"));
    let version = guard(&["--version"], b"");
    assert_eq!(
        text(&version.stdout).trim(),
        format!("grimoire-tool-secret-guard {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn the_hook_blocks_a_write_with_exit_2_and_no_stdout() {
    let event = serde_json::json!({
        "session_id": "abc",
        "transcript_path": "/tmp/t.jsonl",
        "cwd": "/p",
        "hook_event_name": "PreToolUse",
        "tool_name": "Write",
        "tool_input": {"file_path": "/p/.env", "content": format!("TOKEN={}\n", token())},
        "tool_use_id": "toolu_1"
    });
    let out = guard(&["hook"], event.to_string().as_bytes());
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(text(&out.stdout), "", "nothing on stdout for PreToolUse");
    let stderr = text(&out.stderr);
    assert!(
        stderr.contains("blocked this Write call to /p/.env"),
        "{stderr}"
    );
    assert!(!stderr.contains(&token()));
}

#[test]
fn the_hook_allows_clean_events_silently() {
    let event = r#"{"hook_event_name":"PreToolUse","tool_name":"Bash","tool_input":{"command":"cargo test"}}"#;
    let out = guard(&["hook"], event.as_bytes());
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(text(&out.stdout), "");
    assert_eq!(text(&out.stderr), "");
}

#[test]
fn a_blocked_prompt_is_not_echoed_back() {
    let event = serde_json::json!({"hook_event_name": "UserPromptSubmit", "prompt": token()});
    let out = guard(&["hook"], event.to_string().as_bytes());
    assert_eq!(out.status.code(), Some(2));
    let stdout: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(
        stdout,
        serde_json::json!({"hookSpecificOutput": {"hookEventName": "UserPromptSubmit", "suppressOriginalPrompt": true}})
    );
}

#[test]
fn the_hook_fails_closed_on_input_it_cannot_check() {
    let out = guard(&["hook"], b"{ not json");
    assert_eq!(out.status.code(), Some(2));
    assert!(text(&out.stderr).contains("could not check this event"));
}

#[test]
fn rules_are_listed() {
    let out = guard(&["rules", "--json"], b"");
    assert_eq!(out.status.code(), Some(0));
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let ids: Vec<&str> = doc["rules"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["id"].as_str().unwrap())
        .collect();
    assert!(
        ids.contains(&"private-key") && ids.contains(&"github-token"),
        "{ids:?}"
    );
}
