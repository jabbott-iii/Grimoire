//! The real `@@BINARY@@` binary: streams and exit statuses.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::process::{Command, Output};

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_@@BINARY@@"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn json_output_is_one_document() {
    let out = run(&["--json"]);
    assert_eq!(out.status.code(), Some(0));
    let doc: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(doc["format"], 1);
}

#[test]
fn usage_errors_exit_2() {
    let out = run(&["--nope"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(out.stdout, Vec::<u8>::new());
}
