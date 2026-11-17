//! The real `grimoire-mcp-project-docs` binary, over stdio under both protocol
//! lifecycles and over Streamable HTTP.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::path::Path;
use std::process::Stdio;

use rmcp::model::{CallToolRequestParams, CallToolResult, ProtocolVersion};
use rmcp::service::{ClientLifecycleMode, ClientServiceExt as _, RoleClient, RunningService};
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt as _, AsyncReadExt as _, AsyncWriteExt as _};

const BINARY: &str = env!("CARGO_BIN_EXE_grimoire-mcp-project-docs");

/// A project with documentation, ADRs, and files that must never be served.
fn project() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    for (path, text) in [
        ("README.md", "# Project\n\nStart here.\n"),
        ("docs/guide.md", "# Guide\n\nline 3\nline 4\n"),
        (
            "docs/adr/0001-record-decisions.md",
            "# 1. Record decisions\n\nDate: 2026-01-02\n\n## Status\n\nAccepted\n",
        ),
        (
            "docs/adr/0002-use-rust.md",
            "# 2. Use Rust\n\n- **Status:** Superseded by ADR-0003\n",
        ),
        (".env", "SECRET=do-not-serve\n"),
        ("src/lib.rs", "// not documentation\n"),
    ] {
        let p = dir.path().join(path);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, text).unwrap();
    }
    dir
}

type Client = RunningService<RoleClient, ()>;

async fn connect(root: &Path, lifecycle: ClientLifecycleMode) -> (Client, tokio::process::Child) {
    let mut child = tokio::process::Command::new(BINARY)
        .arg("--root")
        .arg(root)
        .env_remove("CLAUDE_PROJECT_DIR")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
    let client = ().serve_with_lifecycle(transport, lifecycle).await.unwrap();
    (client, child)
}

fn lifecycles() -> [ClientLifecycleMode; 2] {
    [
        ClientLifecycleMode::Initialize,
        ClientLifecycleMode::Discover {
            preferred_versions: vec![ProtocolVersion::V_2026_07_28],
        },
    ]
}

async fn call(client: &Client, tool: &str, args: Value) -> CallToolResult {
    let Value::Object(args) = args else {
        panic!("arguments are an object")
    };
    client
        .call_tool(CallToolRequestParams::new(tool.to_owned()).with_arguments(args))
        .await
        .unwrap()
}

fn structured(result: &CallToolResult) -> &Value {
    assert_ne!(result.is_error, Some(true), "{result:?}");
    result.structured_content.as_ref().unwrap()
}

fn error_text(result: &CallToolResult) -> String {
    assert_eq!(result.is_error, Some(true), "{result:?}");
    serde_json::to_string(&result.content).unwrap()
}

#[tokio::test]
async fn the_served_tools_match_asset_toml_under_both_lifecycles() {
    let dir = project();
    for lifecycle in lifecycles() {
        let (client, _child) = connect(dir.path(), lifecycle).await;
        let tools = client.list_all_tools().await.unwrap();
        agent_grimoire_mcp::testing::declared_tools_match(include_str!("../asset.toml"), &tools)
            .unwrap();
        let info = client.peer_info().unwrap();
        let server = info.server_info.as_ref().unwrap();
        assert_eq!(server.name, "grimoire-mcp-project-docs");
        assert_eq!(server.version, env!("CARGO_PKG_VERSION"));
        client.cancel().await.unwrap();
    }
}

#[tokio::test]
async fn documents_are_listed_read_and_searched() {
    let dir = project();
    for lifecycle in lifecycles() {
        let (client, _child) = connect(dir.path(), lifecycle).await;

        let listing = call(&client, "list_docs", json!({})).await;
        let paths: Vec<&str> = structured(&listing)["docs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["path"].as_str().unwrap())
            .collect();
        assert_eq!(
            paths,
            [
                "README.md",
                "docs/adr/0001-record-decisions.md",
                "docs/adr/0002-use-rust.md",
                "docs/guide.md"
            ]
        );
        assert_eq!(structured(&listing)["docs"][0]["title"], "Project");

        let page = call(&client, "list_docs", json!({ "limit": 2 })).await;
        assert_eq!(structured(&page)["next_cursor"], "offset:2");
        let next = call(
            &client,
            "list_docs",
            json!({ "limit": 2, "cursor": "offset:2" }),
        )
        .await;
        assert!(structured(&next).get("next_cursor").is_none());

        let read = call(
            &client,
            "read_doc",
            json!({ "path": "docs/guide.md", "start_line": 3 }),
        )
        .await;
        assert_eq!(structured(&read)["text"], "line 3\nline 4\n");
        assert_eq!(structured(&read)["total_lines"], 4);

        let found = call(&client, "search_docs", json!({ "query": "accepted" })).await;
        assert_eq!(
            structured(&found)["matches"][0]["path"],
            "docs/adr/0001-record-decisions.md"
        );
        assert_eq!(structured(&found)["matches"][0]["line"], 7);

        let adrs = call(&client, "list_adrs", json!({})).await;
        assert_eq!(
            structured(&adrs)["adrs"],
            json!([
                { "number": 1, "path": "docs/adr/0001-record-decisions.md", "title": "1. Record decisions", "status": "Accepted", "date": "2026-01-02" },
                { "number": 2, "path": "docs/adr/0002-use-rust.md", "title": "2. Use Rust", "status": "Superseded by ADR-0003" },
            ])
        );
        client.cancel().await.unwrap();
    }
}

#[tokio::test]
async fn nothing_outside_the_documentation_is_served() {
    let dir = project();
    let outside = tempfile::tempdir().unwrap();
    std::fs::write(outside.path().join("secret.md"), "# outside\n").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(
        outside.path().join("secret.md"),
        dir.path().join("docs/link.md"),
    )
    .unwrap();
    let (client, _child) = connect(dir.path(), ClientLifecycleMode::Initialize).await;
    for path in [
        ".env",
        "src/lib.rs",
        "../secret.md",
        "/etc/passwd.md",
        "docs/../README.md",
        "docs/link.md",
        "missing.md",
    ] {
        let result = call(&client, "read_doc", json!({ "path": path })).await;
        let text = error_text(&result);
        assert!(
            !text.contains("do-not-serve") && !text.contains("outside"),
            "{path}: {text}"
        );
    }
    let leaked = call(&client, "search_docs", json!({ "query": "do-not-serve" })).await;
    assert_eq!(structured(&leaked)["matches"], json!([]));
    // Arguments are validated: unknown fields, bad cursors and empty queries are tool errors.
    error_text(
        &call(
            &client,
            "read_doc",
            json!({ "path": "README.md", "extra": 1 }),
        )
        .await,
    );
    error_text(&call(&client, "list_docs", json!({ "cursor": "offset:99" })).await);
    error_text(&call(&client, "list_docs", json!({ "cursor": "nonsense" })).await);
    error_text(&call(&client, "search_docs", json!({ "query": "" })).await);
    error_text(&call(&client, "list_docs", json!({ "prefix": "../" })).await);
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn long_documents_are_read_in_parts() {
    let dir = project();
    let line = format!("{}\n", "x".repeat(99));
    std::fs::write(dir.path().join("docs/long.md"), line.repeat(1000)).unwrap();
    let (client, _child) = connect(dir.path(), ClientLifecycleMode::Initialize).await;
    let first = call(&client, "read_doc", json!({ "path": "docs/long.md" })).await;
    let first = structured(&first).clone();
    assert_eq!(
        first["end_line"], 300,
        "30,000 characters of 100-character lines"
    );
    assert_eq!(first["next_line"], 301);
    let last = call(
        &client,
        "read_doc",
        json!({ "path": "docs/long.md", "start_line": 901 }),
    )
    .await;
    assert!(structured(&last).get("next_line").is_none());
    error_text(
        &call(
            &client,
            "read_doc",
            json!({ "path": "docs/long.md", "start_line": 1001 }),
        )
        .await,
    );
    client.cancel().await.unwrap();
}

#[tokio::test]
async fn the_root_comes_from_the_flag_or_claude_project_dir_and_is_never_guessed() {
    let dir = project();
    let none = tokio::process::Command::new(BINARY)
        .env_remove("CLAUDE_PROJECT_DIR")
        .stdin(Stdio::null())
        .output()
        .await
        .unwrap();
    assert_eq!(none.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&none.stderr).contains("no project root"));
    assert_eq!(
        none.stdout,
        Vec::<u8>::new(),
        "stdout is for MCP messages only"
    );

    let relative = tokio::process::Command::new(BINARY)
        .env("CLAUDE_PROJECT_DIR", "relative/dir")
        .stdin(Stdio::null())
        .output()
        .await
        .unwrap();
    assert_eq!(relative.status.code(), Some(1));

    // From the environment, as Claude Code starts it: closing stdin at once is a clean exit.
    let from_env = tokio::process::Command::new(BINARY)
        .env("CLAUDE_PROJECT_DIR", dir.path())
        .stdin(Stdio::null())
        .output()
        .await
        .unwrap();
    assert_eq!(
        from_env.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&from_env.stderr)
    );
    assert_eq!(from_env.stdout, Vec::<u8>::new());
}

#[tokio::test]
async fn http_needs_authorization_or_an_explicit_loopback_no_auth() {
    let dir = project();
    let run = |args: &[&str]| {
        let mut cmd = tokio::process::Command::new(BINARY);
        cmd.arg("--root")
            .arg(dir.path())
            .args(args)
            .stdin(Stdio::null());
        cmd
    };
    let missing = run(&["serve-http", "--listen", "127.0.0.1:0"])
        .output()
        .await
        .unwrap();
    assert_eq!(missing.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&missing.stderr).contains("needs authorization"));
    let exposed = run(&["serve-http", "--no-auth", "--listen", "0.0.0.0:0"])
        .output()
        .await
        .unwrap();
    assert_eq!(exposed.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&exposed.stderr)
            .contains("refusing to serve without authorization")
    );
    let conflicting = run(&[
        "serve-http",
        "--no-auth",
        "--auth-resource",
        "https://x.example/mcp",
    ])
    .output()
    .await
    .unwrap();
    assert_eq!(conflicting.status.code(), Some(2));

    // A loopback server answers tools/list.
    let mut child = run(&["serve-http", "--no-auth", "--listen", "127.0.0.1:0"])
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .unwrap();
    let mut lines = tokio::io::BufReader::new(child.stderr.take().unwrap()).lines();
    let line = lines.next_line().await.unwrap().unwrap();
    let addr = line.rsplit("http://").next().unwrap().trim().to_owned();
    let body = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tools/list",
        "params": { "_meta": {
            "io.modelcontextprotocol/protocolVersion": "2026-07-28",
            "io.modelcontextprotocol/clientCapabilities": {}
        }}
    })
    .to_string();
    let mut stream = tokio::net::TcpStream::connect(&addr).await.unwrap();
    let request = format!(
        "POST /mcp HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nAccept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\nMcp-Method: tools/list\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut reply = String::new();
    stream.read_to_string(&mut reply).await.unwrap();
    assert!(reply.starts_with("HTTP/1.1 200"), "{reply}");
    assert!(reply.contains("\"list_adrs\""), "{reply}");
    child.kill().await.unwrap();
}
