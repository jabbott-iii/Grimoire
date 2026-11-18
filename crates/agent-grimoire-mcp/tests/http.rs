//! The hardened Streamable HTTP transport, over a real loopback socket.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

mod support;

use std::borrow::Cow;
use std::fmt::Write as _;
use std::net::SocketAddr;
use std::path::PathBuf;

use agent_grimoire_mcp::{AuthConfig, HttpLimits, HttpOptions, HttpServer, PROTOCOL_VERSIONS};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Implementation, ProtocolVersion, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};

#[derive(Clone)]
struct Ping {
    tool_router: ToolRouter<Ping>,
}

#[tool_router]
impl Ping {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Answers pong",
        annotations(
            read_only_hint = true,
            destructive_hint = false,
            idempotent_hint = true,
            open_world_hint = false
        )
    )]
    #[allow(clippy::unused_self, reason = "rmcp tools are methods")]
    fn ping(&self) -> String {
        "pong".to_owned()
    }
}

#[tool_handler(router = self.tool_router)]
#[expect(
    clippy::unused_async_trait_impl,
    reason = "rmcp's generated handler methods are async"
)]
impl ServerHandler for Ping {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("ping-test", "0.0.0"))
    }

    fn supported_protocol_versions(&self) -> Cow<'static, [ProtocolVersion]> {
        Cow::Borrowed(&PROTOCOL_VERSIONS)
    }
}

struct Running {
    addr: SocketAddr,
    stop: Option<tokio::sync::oneshot::Sender<()>>,
    task: Option<tokio::task::JoinHandle<Result<(), agent_grimoire_mcp::ServeError>>>,
}

impl Running {
    async fn stop(mut self) {
        let _ = self.stop.take().unwrap().send(());
        self.task.take().unwrap().await.unwrap().unwrap();
    }
}

async fn start(auth: Option<AuthConfig>) -> Running {
    start_with(auth, HttpLimits::default()).await
}

async fn start_with(auth: Option<AuthConfig>, limits: HttpLimits) -> Running {
    let server = HttpServer::bind(
        Ping::new,
        HttpOptions {
            listen: "127.0.0.1:0".parse().unwrap(),
            path: "/mcp".to_owned(),
            allowed_origins: vec!["https://app.example".to_owned()],
            allowed_hosts: Vec::new(),
            auth,
            limits,
        },
    )
    .await
    .unwrap();
    let addr = server.local_addr().unwrap();
    let (stop, stopped) = tokio::sync::oneshot::channel::<()>();
    let task = tokio::spawn(server.run_until(async {
        let _ = stopped.await;
    }));
    Running {
        addr,
        stop: Some(stop),
        task: Some(task),
    }
}

struct Reply {
    status: u16,
    headers: Vec<(String, String)>,
    body: String,
}

impl Reply {
    fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    fn json(&self) -> Value {
        serde_json::from_str(&self.body).unwrap_or_else(|e| panic!("{e}: {}", self.body))
    }
}

/// Sends one HTTP/1.1 request. `Host` defaults to the server's address.
async fn send(
    addr: SocketAddr,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: &str,
) -> Reply {
    let request = head(addr, method, path, headers, body.len()) + body;
    exchange(addr, &request).await
}

/// The request line and headers of a request whose body is `content_length`
/// bytes, up to and including the blank line.
fn head(
    addr: SocketAddr,
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    content_length: usize,
) -> String {
    let mut request = format!("{method} {path} HTTP/1.1\r\n");
    if !headers.iter().any(|(n, _)| n.eq_ignore_ascii_case("host")) {
        let _ = write!(request, "Host: {addr}\r\n");
    }
    for (name, value) in headers {
        let _ = write!(request, "{name}: {value}\r\n");
    }
    let _ = write!(
        request,
        "Content-Length: {content_length}\r\nConnection: close\r\n\r\n"
    );
    request
}

/// Writes `request` on a new connection and reads the reply until the server
/// closes it.
async fn exchange(addr: SocketAddr, request: &str) -> Reply {
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    stream.write_all(request.as_bytes()).await.unwrap();
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).await.unwrap();
    let text = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    let mut lines = head.lines();
    let status = lines
        .next()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
    let headers = lines
        .filter_map(|l| l.split_once(':'))
        .map(|(n, v)| (n.trim().to_owned(), v.trim().to_owned()))
        .collect();
    Reply {
        status,
        headers,
        body: body.to_owned(),
    }
}

fn rpc(method: &str, params: &Value) -> String {
    json!({ "jsonrpc": "2.0", "id": 1, "method": method, "params": params }).to_string()
}

fn meta() -> Value {
    json!({ "_meta": {
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientCapabilities": {}
    }})
}

const MCP: [(&str, &str); 3] = [
    ("Content-Type", "application/json"),
    ("Accept", "application/json, text/event-stream"),
    ("MCP-Protocol-Version", "2026-07-28"),
];

fn headers<'a>(extra: &[(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
    let mut all: Vec<(&str, &str)> = MCP.to_vec();
    all.extend_from_slice(extra);
    all
}

#[tokio::test]
async fn host_and_origin_are_checked_first() {
    let server = start(None).await;
    let body = rpc("tools/list", &meta());
    let ok = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Mcp-Method", "tools/list")]),
        &body,
    )
    .await;
    assert_eq!(ok.status, 200, "{}", ok.body);
    assert_eq!(ok.json()["result"]["tools"][0]["name"], "ping");
    assert!(
        ok.header("mcp-session-id").is_none(),
        "stateless: no sessions"
    );

    let rebound = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Host", "evil.example"), ("Mcp-Method", "tools/list")]),
        &body,
    )
    .await;
    assert_eq!(rebound.status, 403);
    assert!(
        rebound.body.contains("does not answer to that Host"),
        "Grimoire's own check: {}",
        rebound.body
    );
    let origin = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[
            ("Origin", "https://evil.example"),
            ("Mcp-Method", "tools/list"),
        ]),
        &body,
    )
    .await;
    assert_eq!(origin.status, 403);
    assert!(
        origin.body.contains("that Origin may not call this server"),
        "Grimoire's own check: {}",
        origin.body
    );
    let null = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Origin", "null"), ("Mcp-Method", "tools/list")]),
        &body,
    )
    .await;
    assert_eq!(null.status, 403);
    let allowed = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[
            ("Origin", "https://app.example:443"),
            ("Mcp-Method", "tools/list"),
        ]),
        &body,
    )
    .await;
    assert_eq!(
        allowed.status, 200,
        "an allowlisted origin, with its default port"
    );
    server.stop().await;
}

#[tokio::test]
async fn only_post_to_the_endpoint_is_served() {
    let server = start(None).await;
    let get = send(server.addr, "GET", "/mcp", &[], "").await;
    assert_eq!(get.status, 405);
    assert_eq!(get.header("allow"), Some("POST"));
    let delete = send(server.addr, "DELETE", "/mcp", &[], "").await;
    assert_eq!(delete.status, 405);
    assert_eq!(
        send(server.addr, "POST", "/other", &headers(&[]), "{}")
            .await
            .status,
        404
    );
    let metadata = send(
        server.addr,
        "GET",
        "/.well-known/oauth-protected-resource/mcp",
        &[],
        "",
    )
    .await;
    assert_eq!(metadata.status, 404, "no metadata without authorization");
    server.stop().await;
}

#[tokio::test]
async fn discovery_lists_both_protocol_versions_and_legacy_clients_stay_stateless() {
    let server = start(None).await;
    let discover = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Mcp-Method", "server/discover")]),
        &rpc("server/discover", &meta()),
    )
    .await;
    assert_eq!(discover.status, 200, "{}", discover.body);
    assert_eq!(
        discover.json()["result"]["supportedVersions"],
        json!(["2025-11-25", "2026-07-28"])
    );

    let initialize = rpc(
        "initialize",
        &json!({ "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "1" } }),
    );
    let legacy = send(
        server.addr,
        "POST",
        "/mcp",
        &[
            ("Content-Type", "application/json"),
            ("Accept", "application/json, text/event-stream"),
        ],
        &initialize,
    )
    .await;
    assert_eq!(legacy.status, 200, "{}", legacy.body);
    assert_eq!(legacy.json()["result"]["protocolVersion"], "2025-11-25");
    assert!(
        legacy.header("mcp-session-id").is_none(),
        "no session for legacy clients either"
    );
    server.stop().await;
}

#[tokio::test]
async fn oversized_and_malformed_requests_are_refused() {
    let server = start(None).await;
    // Only the head is sent: the server must refuse on the declared length
    // alone, without waiting for the body. It closes the connection without
    // reading one, so a client still writing the body would race that close
    // and could see a broken pipe or a reset instead of the reply.
    let oversized = head(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Mcp-Method", "tools/list")]),
        agent_grimoire_mcp::MAX_BODY_BYTES + 1,
    );
    let reply = exchange(server.addr, &oversized).await;
    assert_eq!(reply.status, 413);
    let no_header = send(
        server.addr,
        "POST",
        "/mcp",
        &[
            ("Content-Type", "application/json"),
            ("Accept", "application/json, text/event-stream"),
            ("Mcp-Method", "tools/list"),
        ],
        &rpc("tools/list", &meta()),
    )
    .await;
    assert_eq!(
        no_header.status, 400,
        "MCP-Protocol-Version is required: {}",
        no_header.body
    );
    server.stop().await;
}

#[tokio::test]
async fn without_authorization_only_loopback_is_allowed() {
    let error = HttpServer::bind(
        Ping::new,
        HttpOptions {
            listen: "0.0.0.0:0".parse().unwrap(),
            path: "/mcp".to_owned(),
            allowed_origins: Vec::new(),
            allowed_hosts: Vec::new(),
            auth: None,
            limits: HttpLimits::default(),
        },
    )
    .await
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("refusing to serve without authorization"),
        "{error}"
    );
    let bad_origin = HttpServer::bind(
        Ping::new,
        HttpOptions {
            listen: "127.0.0.1:0".parse().unwrap(),
            path: "/mcp".to_owned(),
            allowed_origins: vec!["https://app.example/path".to_owned()],
            allowed_hosts: Vec::new(),
            auth: None,
            limits: HttpLimits::default(),
        },
    )
    .await;
    assert!(bad_origin.is_err());
}

fn auth_config(jwks: PathBuf) -> AuthConfig {
    AuthConfig {
        resource: support::RESOURCE.to_owned(),
        issuer: support::ISSUER.to_owned(),
        jwks,
        audiences: Vec::new(),
        required_scopes: vec!["docs:read".to_owned()],
        resource_name: None,
        allow_jwt_typ: false,
        leeway_secs: 30,
    }
}

#[tokio::test]
async fn authorization_challenges_then_accepts_a_valid_token() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = support::write_jwks(dir.path(), &[support::ec_jwk("ec1")]);
    let server = start(Some(auth_config(jwks))).await;
    let host = ("Host", "docs.example.com");
    let body = rpc("tools/list", &meta());

    let metadata = send(
        server.addr,
        "GET",
        "/.well-known/oauth-protected-resource/mcp",
        &[host],
        "",
    )
    .await;
    assert_eq!(metadata.status, 200);
    assert_eq!(metadata.json()["resource"], support::RESOURCE);
    assert_eq!(metadata.header("cache-control"), Some("max-age=3600"));
    let root = send(
        server.addr,
        "GET",
        "/.well-known/oauth-protected-resource",
        &[host],
        "",
    )
    .await;
    assert_eq!(
        root.status, 200,
        "also at the root, which some clients try first"
    );

    let missing = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[host, ("Mcp-Method", "tools/list")]),
        &body,
    )
    .await;
    assert_eq!(missing.status, 401);
    assert_eq!(
        missing.header("www-authenticate"),
        Some(
            "Bearer resource_metadata=\"https://docs.example.com/.well-known/oauth-protected-resource/mcp\", scope=\"docs:read\""
        )
    );

    let token = support::es256(&support::good_claims());
    let authorization = format!("Bearer {token}");
    let ok = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[
            host,
            ("Mcp-Method", "tools/list"),
            ("Authorization", &authorization),
        ]),
        &body,
    )
    .await;
    assert_eq!(ok.status, 200, "{}", ok.body);

    // Host is checked before authorization, so a rebinding probe learns nothing.
    let rebound = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[("Host", "evil.example"), ("Mcp-Method", "tools/list")]),
        &body,
    )
    .await;
    assert_eq!(rebound.status, 403);
    // A token in the query string is ignored: the request is unauthenticated.
    let query = format!("/mcp?access_token={token}");
    assert_eq!(
        send(
            server.addr,
            "POST",
            &query,
            &headers(&[host, ("Mcp-Method", "tools/list")]),
            &body
        )
        .await
        .status,
        401
    );
    server.stop().await;
}

#[tokio::test]
async fn a_2025_11_25_client_lists_and_calls_tools_statelessly() {
    let server = start(None).await;
    let legacy = |method: &'static str| {
        vec![
            ("Content-Type", "application/json"),
            ("Accept", "application/json, text/event-stream"),
            ("MCP-Protocol-Version", "2025-11-25"),
            ("Mcp-Method", method),
        ]
    };
    let initialize = rpc(
        "initialize",
        &json!({ "protocolVersion": "2025-11-25", "capabilities": {}, "clientInfo": { "name": "t", "version": "1" } }),
    );
    let init = send(
        server.addr,
        "POST",
        "/mcp",
        &[
            ("Content-Type", "application/json"),
            ("Accept", "application/json, text/event-stream"),
        ],
        &initialize,
    )
    .await;
    assert_eq!(init.status, 200, "{}", init.body);
    let initialized =
        json!({ "jsonrpc": "2.0", "method": "notifications/initialized" }).to_string();
    let notified = send(
        server.addr,
        "POST",
        "/mcp",
        &legacy("notifications/initialized"),
        &initialized,
    )
    .await;
    assert_eq!(notified.status, 202, "{}", notified.body);
    // Ordinary requests carry the header but no per-request _meta.
    let list = send(
        server.addr,
        "POST",
        "/mcp",
        &legacy("tools/list"),
        &rpc("tools/list", &json!({})),
    )
    .await;
    assert_eq!(list.status, 200, "{}", list.body);
    assert_eq!(list.json()["result"]["tools"][0]["name"], "ping");
    let mut call_headers = legacy("tools/call");
    call_headers.push(("Mcp-Name", "ping"));
    let call = send(
        server.addr,
        "POST",
        "/mcp",
        &call_headers,
        &rpc("tools/call", &json!({ "name": "ping" })),
    )
    .await;
    assert_eq!(call.status, 200, "{}", call.body);
    assert!(call.body.contains("pong"), "{}", call.body);
    server.stop().await;
}

#[tokio::test]
async fn only_the_supported_protocol_versions_are_served() {
    let server = start(None).await;
    let base = [
        ("Content-Type", "application/json"),
        ("Accept", "application/json, text/event-stream"),
    ];
    let with = |version: &'static str| {
        let mut h = base.to_vec();
        h.push(("MCP-Protocol-Version", version));
        h.push(("Mcp-Method", "tools/list"));
        h
    };
    let old = send(
        server.addr,
        "POST",
        "/mcp",
        &with("2025-03-26"),
        &rpc("tools/list", &json!({})),
    )
    .await;
    assert_eq!(old.status, 400);
    assert_eq!(old.json()["error"]["code"], -32022);
    assert_eq!(
        old.json()["error"]["data"]["supported"],
        json!(["2025-11-25", "2026-07-28"])
    );
    assert_eq!(old.json()["error"]["data"]["requested"], "2025-03-26");
    let mut no_header = base.to_vec();
    no_header.push(("Mcp-Method", "tools/list"));
    let missing = send(
        server.addr,
        "POST",
        "/mcp",
        &no_header,
        &rpc("tools/list", &json!({})),
    )
    .await;
    assert_eq!(missing.status, 400);
    assert_eq!(missing.json()["error"]["code"], -32020);
    assert_eq!(missing.json()["id"], 1);
    let no_meta = send(
        server.addr,
        "POST",
        "/mcp",
        &with("2026-07-28"),
        &rpc("tools/list", &json!({})),
    )
    .await;
    assert_eq!(no_meta.status, 400);
    assert_eq!(no_meta.json()["error"]["code"], -32602);
    server.stop().await;
}

#[tokio::test]
async fn origin_is_checked_before_authorization() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = support::write_jwks(dir.path(), &[support::ec_jwk("ec1")]);
    let server = start(Some(auth_config(jwks))).await;
    let refused = send(
        server.addr,
        "POST",
        "/mcp",
        &headers(&[
            ("Host", "docs.example.com"),
            ("Origin", "https://evil.example"),
            ("Mcp-Method", "tools/list"),
        ]),
        &rpc("tools/list", &meta()),
    )
    .await;
    assert_eq!(refused.status, 403);
    assert!(
        refused.header("www-authenticate").is_none(),
        "no challenge for a refused origin"
    );
    assert!(
        refused
            .body
            .contains("that Origin may not call this server"),
        "{}",
        refused.body
    );
    server.stop().await;
}

#[tokio::test]
async fn a_client_that_stops_reading_cannot_hold_a_connection_slot() {
    let server = start_with(
        None,
        HttpLimits {
            max_connections: 1,
            connection_deadline: std::time::Duration::from_secs(2),
        },
    )
    .await;
    // Takes the only slot, sends complete headers, then never finishes the body.
    // Only the connection deadline (2 s here) ends it before the request timeout.
    let mut stalled = tokio::net::TcpStream::connect(server.addr).await.unwrap();
    let head = format!(
        "POST /mcp HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: 1000\r\nMCP-Protocol-Version: 2026-07-28\r\n\r\n{{\"jsonrpc\"",
        server.addr
    );
    stalled.write_all(head.as_bytes()).await.unwrap();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let started = std::time::Instant::now();
    let reply = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        send(
            server.addr,
            "POST",
            "/mcp",
            &headers(&[("Mcp-Method", "tools/list")]),
            &rpc("tools/list", &meta()),
        ),
    )
    .await
    .expect("a new client is served once the stalled connection's deadline passes");
    assert_eq!(reply.status, 200, "{}", reply.body);
    assert!(started.elapsed() < std::time::Duration::from_secs(8));
    assert_eq!(
        reply.header("connection"),
        Some("close"),
        "one request per connection"
    );
    drop(stalled);
    server.stop().await;
}
