//! The real `@@BINARY@@` binary over stdio, under both protocol lifecycles.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::process::Stdio;

use rmcp::model::{CallToolRequestParams, ProtocolVersion};
use rmcp::service::{ClientLifecycleMode, ClientServiceExt as _};

#[tokio::test]
async fn tools_match_asset_toml_and_answer() {
    for lifecycle in [
        ClientLifecycleMode::Initialize,
        ClientLifecycleMode::Discover {
            preferred_versions: vec![ProtocolVersion::V_2026_07_28],
        },
    ] {
        let mut child =
            tokio::process::Command::new(env!("CARGO_BIN_EXE_@@BINARY@@"))
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .kill_on_drop(true)
                .spawn()
                .unwrap();
        let transport = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        let client = ().serve_with_lifecycle(transport, lifecycle).await.unwrap();
        let tools = client.list_all_tools().await.unwrap();
        agent_grimoire_mcp::testing::declared_tools_match(include_str!("../asset.toml"), &tools)
            .unwrap();
        let serde_json::Value::Object(args) = serde_json::json!({ "text": "hi" }) else {
            unreachable!()
        };
        let result = client
            .call_tool(CallToolRequestParams::new("echo").with_arguments(args))
            .await
            .unwrap();
        assert_eq!(result.structured_content.unwrap()["text"], "hi");
        client.cancel().await.unwrap();
    }
}
