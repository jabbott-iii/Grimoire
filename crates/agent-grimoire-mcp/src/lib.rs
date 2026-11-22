//! Shared hardening (security defenses) for Grimoire's Model Context Protocol
//! (MCP) servers: the transports, limits and authorization checks that every
//! server asset under `catalog/mcp-servers/` serves through, so that each one
//! starts from the same secure defaults instead of rmcp's.
//!
//! An MCP server offers an AI client tools, resources and prompts. rmcp, the
//! official Rust library for MCP, is what the servers are built on, but it has
//! no server-side authorization, and its HTTP defaults are unsafe for
//! Grimoire ([ADR-0007], server-side MCP authorization and HTTP hardening):
//! - Origin validation, the check of the `Origin` header that a browser sets
//!   to the site that sent a request, is off;
//! - a request without `MCP-Protocol-Version`, the header that names the
//!   protocol version, is treated as version 2025-03-26;
//! - legacy sessions (server-side state tied to an `Mcp-Session-Id` header)
//!   are on.
//!
//! This crate sets those defaults once, for every server.
//!
//! # Where it fits
//!
//! - **Who uses it.** Every MCP server asset depends on it: today
//!   `grimoire-mcp-project-docs`, in `catalog/mcp-servers/project-docs`, and
//!   the template in `xtask/templates/mcp-server/`, which
//!   `cargo xtask new mcp-server <id> --maintainer <login>` fills in for a new
//!   server. Catalog validation requires the dependency (below, "What catalog
//!   validation requires of a server").
//! - **What it builds on.** rmcp for the protocol; tokio (an asynchronous
//!   runtime) and hyper (an HTTP library) for serving; and jsonwebtoken, with
//!   a verify-only signature provider over the `rsa` and `p256` crates, for
//!   access tokens.
//! - **The CLI never links it.** [ADR-0002], which packages each server as a
//!   binary of its own, keeps server code out of `grimoire`. The installer
//!   runs in continuous integration (CI) with write access to repositories,
//!   so its attack surface (the code an attacker could reach) stays small: it
//!   carries no async runtime, HTTP stack or authorization code.
//!   Installing a server writes only the client's entry for it; Grimoire never
//!   runs the server. The release workflow builds every server binary for all
//!   six targets, beside the CLI, and ships each as its own archive.
//! - **Never published.** Its `Cargo.toml` sets `publish = false`, and the
//!   workspace refers to it by path only, so it has no crates.io or docs.rs
//!   page; `cargo doc -p agent-grimoire-mcp` builds these docs locally. Only
//!   MCP server assets link it, and ADR-0002 keeps those off crates.io too:
//!   every new published crate needs a first publish with the maintainer's
//!   credentials, and crates.io stays limited to the CLI and its core crates.
//! - **Design.** [`docs/architecture.md` §3.5, MCP server][architecture-3.5]
//!   sets the bar every server meets. [ADR-0007] chose how servers check
//!   authorization and made this crate mandatory.
//!   [`docs/security-model.md` §4, MCP servers][security-4] lists the threats
//!   it answers, M1 to M12.
//!
//! # The parts
//!
//! - **Transports**, the two ways MCP is carried:
//!   - [`serve_stdio`] serves MCP on stdin and stdout, which is how a client
//!     runs a server it starts itself. Nothing but MCP goes to stdout, and the
//!     server stops cleanly on end of input, `SIGINT` (Ctrl-C) or, on Unix,
//!     `SIGTERM`.
//!   - [`HttpServer`] serves Streamable HTTP, MCP's HTTP transport, for a
//!     server that runs on its own. It checks `Host` and `Origin` before
//!     anything else, serves only on a loopback address (one that only this
//!     machine can reach, such as `127.0.0.1` or `[::1]`) unless authorization
//!     is configured, limits request size and time, and runs rmcp stateless.
//!     [`HttpServer::bind`] checks the [`HttpOptions`] and binds the address;
//!     [`HttpServer::run`] then serves until a stop signal, and
//!     [`HttpServer::run_until`] until a future completes. Its limits are
//!     [`HttpLimits`] and [`MAX_BODY_BYTES`].
//!   - [`PROTOCOL_VERSIONS`] is what a server's `supported_protocol_versions`
//!     returns: 2026-07-28, and 2025-11-25 for clients that still use the
//!     earlier handshake.
//! - **Authorization**, in [`auth`]: the checks of an OAuth resource server.
//!   OAuth is the standard for issuing access tokens, and a resource server is
//!   one that holds the protected data and accepts tokens that a separate
//!   authorization server issued. The checks follow three Internet standards,
//!   or RFCs (Requests for Comments):
//!   - RFC 9728 metadata: a JSON document, at a well-known URL, that tells a
//!     client which authorization server to get a token from;
//!   - RFC 6750 challenges: the `WWW-Authenticate` response header that says
//!     why a request was refused. RFC 6750 also defines bearer tokens, sent
//!     as `Authorization: Bearer <token>`;
//!   - RFC 9068 access tokens: tokens that are JSON Web Tokens (JWTs, signed
//!     JSON documents that say who issued them, who they are for and what they
//!     allow), verified against a local JWKS file, a JSON Web Key Set that
//!     holds the authorization server's public keys.
//!
//!   [`AuthConfig`] configures the checks, as [`HttpOptions::auth`]. A request
//!   that passes carries a [`Caller`], what the token says about who sent it,
//!   in the HTTP request's extensions. rmcp hands each handler the request's
//!   `http::request::Parts`, so a tool reads it as
//!   `parts.extensions.get::<Caller>()`.
//! - **Limits on tool work**, in [`limits`]: [`limits::Limits`] runs a tool's
//!   blocking work within concurrency, rate and time limits
//!   ([`limits::LimitConfig`]), and says why it refused
//!   ([`limits::Refused`]). [`limits::truncate`] cuts an answer to a number of
//!   characters.
//! - **Testing**, in [`testing`]: [`testing::declared_tools_match`] checks a
//!   server's live tools against its `asset.toml`.
//! - **Logging and errors.** [`log`] writes one line to stderr. [`ServeError`]
//!   is a server that could not start or stopped with an error.
//!   [`may_serve_without_auth`] says whether a listen address may serve
//!   without authorization, and [`is_loopback`] whether a URL's host names
//!   this machine: `localhost`, `127.x.y.z`, or `[::1]` in brackets, each
//!   without a port.
//!
//! # Example
//!
//! The limits and the loopback rule, as a server meets them. This example
//! runs, and opens no socket:
//!
//! ```
//! use agent_grimoire_mcp::limits::{LimitConfig, Limits, Refused};
//! use agent_grimoire_mcp::{HttpLimits, HttpOptions, HttpServer};
//!
//! /// A server with no tools: every `ServerHandler` method has a default.
//! #[derive(Clone)]
//! struct Empty;
//! impl rmcp::ServerHandler for Empty {}
//!
//! let runtime = tokio::runtime::Builder::new_current_thread()
//!     .enable_all()
//!     .build()?;
//! runtime.block_on(async {
//!     // A server keeps one `Limits` and runs each tool's blocking work through
//!     // it. This one allows a burst of two calls and never refills.
//!     let limits = Limits::new(LimitConfig {
//!         burst: 2,
//!         per_second: 0,
//!         ..LimitConfig::default()
//!     });
//!     assert_eq!(limits.run(|| 1 + 1).await, Ok(2));
//!     assert_eq!(limits.run(|| 2 + 2).await, Ok(4));
//!     let refused = limits.run(|| 3 + 3).await;
//!     assert_eq!(refused, Err(Refused::RateLimited));
//!     // The refusal is written for the model, as the tool's error message.
//!     assert_eq!(
//!         refused.unwrap_err().to_string(),
//!         "too many requests; wait a moment and try again"
//!     );
//!
//!     // Without authorization, `bind` refuses an address that isn't loopback,
//!     // before it opens a socket.
//!     let options = HttpOptions {
//!         listen: "0.0.0.0:8931".parse()?,
//!         path: "/mcp".to_owned(),
//!         allowed_origins: Vec::new(),
//!         allowed_hosts: Vec::new(),
//!         auth: None,
//!         limits: HttpLimits::default(),
//!     };
//!     let error = HttpServer::bind(|| Empty, options).await.unwrap_err();
//!     assert!(error.to_string().starts_with("refusing to serve without authorization"));
//!     Ok::<(), Box<dyn std::error::Error>>(())
//! })?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! A server's `main` serves stdio by default, and Streamable HTTP when asked,
//! as `catalog/mcp-servers/project-docs` does. This example is compiled but
//! not run, because serving waits on stdin or a socket:
//!
//! ```no_run
//! use agent_grimoire_mcp::{HttpLimits, HttpOptions, HttpServer, ServeError, log, serve_stdio};
//!
//! #[derive(Clone)]
//! struct Docs;
//! impl rmcp::ServerHandler for Docs {}
//!
//! async fn serve(http: bool) -> Result<(), ServeError> {
//!     if !http {
//!         // How clients start a server: MCP on stdin and stdout.
//!         return serve_stdio(Docs).await;
//!     }
//!     let options = HttpOptions {
//!         listen: "127.0.0.1:8931".parse().expect("a socket address"),
//!         path: "/mcp".to_owned(),
//!         allowed_origins: Vec::new(),
//!         allowed_hosts: Vec::new(),
//!         auth: None, // allowed only because the address is loopback
//!         limits: HttpLimits::default(),
//!     };
//!     // The protocol is stateless: `bind` takes a function that makes the
//!     // handler for each request.
//!     let server = HttpServer::bind(|| Docs, options).await?;
//!     log(&format!("serving on http://{}", server.local_addr()?));
//!     server.run().await
//! }
//!
//! fn main() {
//!     let runtime = tokio::runtime::Builder::new_multi_thread()
//!         .enable_all()
//!         .build()
//!         .expect("an async runtime");
//!     let code = match runtime.block_on(serve(false)) {
//!         Ok(()) => 0,
//!         Err(e) => {
//!             log(&format!("docs: {e}"));
//!             1
//!         }
//!     };
//!     // A read of stdin that cannot be cancelled would otherwise hold the
//!     // runtime open.
//!     runtime.shutdown_background();
//!     std::process::exit(code);
//! }
//! ```
//!
//! # How an HTTP request is checked
//!
//! [`HttpServer`] checks each request in this order before rmcp sees it.
//! ADR-0007 fixed the order so that a probe (a request that tests what the
//! server accepts) learns as little as possible:
//!
//! 1. **`Host`** must be one the server answers to, or the answer is 403.
//!    Those are the bound address; `localhost`, `127.0.0.1` and `[::1]` with
//!    its port, on a loopback address; the resource's host, with
//!    authorization; and [`HttpOptions::allowed_hosts`]. With the `Origin`
//!    check, this defeats DNS rebinding, where a web page's domain name is
//!    switched to resolve to a local address.
//! 2. **`Origin`**, when present, must be on [`HttpOptions::allowed_origins`]
//!    (403). The list is empty unless the server is given origins (in
//!    `project-docs`, with `--allowed-origin`), so by default no browser page
//!    can call the server, and `null` is never allowed. A request without
//!    `Origin`, as
//!    non-browser clients send, passes.
//! 3. **The path and method.** With authorization, the RFC 9728 metadata is
//!    served on `GET` and `HEAD` at its well-known paths. Everything else must
//!    be a `POST` to the MCP path: another path gets 404, and another method
//!    405 with `Allow`.
//! 4. **Authorization**, when configured (401, 403 or 400, with an RFC 6750
//!    challenge; below, "Authorization").
//! 5. **The protocol version.** `MCP-Protocol-Version` must name one of the
//!    [`PROTOCOL_VERSIONS`] (400, with JSON-RPC error code -32022; MCP's
//!    messages are JSON-RPC, a JSON format for requests and their replies).
//!    The header is required on every JSON-RPC request and notification but a
//!    legacy `initialize` (400, -32020); rmcp refuses any other kind of body
//!    with its own error. A 2026-07-28 request must also carry the version in
//!    `_meta` (400, -32602). rmcp itself checks the header only against every
//!    version it knows. This step comes after authorization, so the body of an
//!    unauthenticated request is never parsed.
//!
//! rmcp then applies its own `Host`, `Origin`, media-type, body-size and MCP
//! header checks, configured from the same lists. It runs stateless, with
//! JSON responses (an event stream only when a handler sends a notification
//! or request before its reply) and no sessions (no `Mcp-Session-Id`), and
//! serves 2025-11-25 clients statelessly too.
//!
//! # Limits that keep one client from using up a server
//!
//! Over HTTP, [`HttpServer`] allows:
//! - request bodies of at most [`MAX_BODY_BYTES`], 1 MiB (413 above that);
//! - one request per connection: no keep-alive (reusing a connection for
//!   another request), so no pipelining (sending several requests before the
//!   first reply);
//! - at most [`HttpLimits::max_connections`] connections at once, 64 by
//!   default; more wait to be accepted;
//! - 10 seconds to send the request headers, after which the connection is
//!   closed with no answer, and 60 seconds per request, after which the
//!   answer is 503;
//! - every connection closed after [`HttpLimits::connection_deadline`], 75
//!   seconds by default, whatever it is doing, so a client that stops reading
//!   can't keep its slot;
//! - on shutdown, 5 seconds for requests in flight to finish.
//!
//! With authorization, an access token may be at most 16 KiB (400
//! `invalid_request` above that), and the key file at most 1 MiB.
//!
//! For tool work, MCP requires servers to rate-limit tool calls.
//! [`limits::LimitConfig::default`] allows 8 calls at once, 20 per second on
//! average with bursts of 40, and 10 seconds per call. A call over the time
//! limit gets [`limits::Refused::TimedOut`], but its work keeps its slot until
//! it actually ends, so a timed-out call still counts against the limit on
//! calls at once.
//!
//! # Serving without authorization
//!
//! Without [`HttpOptions::auth`], [`HttpServer::bind`] refuses any listen
//! address that isn't a loopback address, before it opens a socket.
//! [`may_serve_without_auth`] is the same test.
//!
//! Even on loopback, any process on the machine can connect: MCP's own
//! security guidance says loopback is not an authentication boundary. So
//! prefer stdio for local use. `project-docs` and the template need an
//! explicit `--no-auth` to serve HTTP without authorization, so that doing so
//! is a choice.
//!
//! # Authorization
//!
//! Credentials are read from exactly one `Authorization: Bearer` header; a
//! token in the query string is ignored. With authorization, the MCP path is
//! the path of [`AuthConfig::resource`], whatever [`HttpOptions::path`] says.
//! A token is accepted only if:
//! - its `typ` is `at+jwt` or `application/at+jwt`, which keeps out ID
//!   tokens, tokens that prove a sign-in rather than grant access; `JWT` too
//!   only with [`AuthConfig::allow_jwt_typ`];
//! - it is signed with RS256 (RSA with SHA-256) or ES256 (an elliptic-curve
//!   signature on the P-256 curve, with SHA-256), by a key bound to that one
//!   algorithm; RSA keys must be 2048 to 8192 bits;
//! - its key is chosen by `kid` (key ID), or is the only key for its
//!   algorithm. Keys come only from the JWKS file, never from the token;
//! - `iss`, the issuer, is exactly [`AuthConfig::issuer`], as a string;
//! - `aud`, the audience, contains [`AuthConfig::resource`] or one of
//!   [`AuthConfig::audiences`];
//! - `exp` (expiry) and `nbf` (not before) hold, within a clock difference of
//!   [`AuthConfig::leeway_secs`], at most [`auth::MAX_LEEWAY_SECS`] (60
//!   seconds). `aud`, `iss` and `exp` are required;
//! - it has every scope (named permission) in
//!   [`AuthConfig::required_scopes`], read from `scope` or `scp`.
//!
//! A refusal is an RFC 6750 challenge that names the metadata's URL in
//! `resource_metadata`: 401 with no error code for no credentials, or another
//! scheme than Bearer; 400 `invalid_request` for malformed or repeated
//! credentials; 401 `invalid_token` for a token that was checked and failed;
//! 403 `insufficient_scope` for missing scopes.
//!
//! The keys come from a local file, so a server makes no outbound connection.
//! ADR-0007 chose that over fetching the keys from the issuer, which needs an
//! HTTPS client, about 50 more crates and a network dependency. The cost is
//! that operators rotate keys by replacing the file. On a request at least 5
//! seconds after the last check, the file is read again if its modification
//! time has changed. (A new file with the old modification time, as
//! `touch -r` or a restore can leave, isn't noticed.) When it is read again:
//! - a well-formed key set replaces the keys, even with no usable key in it,
//!   so removing a key revokes it;
//! - a deleted file revokes every key;
//! - only a file that can't be read or parsed, for example one being
//!   rewritten in place, keeps the previous keys.
//!
//! Browser clients aren't supported: there is no CORS (Cross-Origin Resource
//! Sharing, the headers that let a web page call a server on another origin),
//! and an `OPTIONS` preflight gets 403 when its `Origin` isn't allowed, and
//! 405 at the MCP path otherwise.
//!
//! # Logging: stderr only, and never a token
//!
//! The crate logs to stderr only, and never logs, stores or forwards a token.
//! Over stdio, stdout carries only MCP messages, so any other line there
//! would corrupt the protocol. MCP doesn't allow a server to pass a token on,
//! and a token in a log can leak.
//!
//! The request a handler gets still carries the token: rmcp gives each
//! handler the request's `http::request::Parts`, `Authorization` header
//! included. So a server's own code must not log or forward those headers
//! either. A server writes its own diagnostics with [`log`], and never with a
//! token in them.
//!
//! # What catalog validation requires of a server
//!
//! `cargo xtask validate` checks three things that tie every MCP server asset
//! to this crate
//! ([`docs/architecture.md` §6.4, Catalog validation][architecture-6.4]):
//! - a `[dependencies]` entry on this crate (`crate/hardening`);
//! - no direct use of rmcp's stdio or Streamable HTTP transports in `src/`
//!   (`crate/raw-transport`);
//! - a test that calls [`testing::declared_tools_match`]
//!   (`crate/declared-tools-test`). It compares the tools the running server
//!   lists with those its `asset.toml` declares, and their annotations: hints
//!   such as whether a tool only reads, each of which must be stated
//!   explicitly.
//!
//! The last two are text searches that catch the usual mistakes; review is
//! what guarantees them. A server depends on this crate as
//! `agent-grimoire-mcp.workspace = true`, because an asset crate may not name
//! a `path` or `git` source itself (`crate/path-dependency`).
//!
//! ```
//! use agent_grimoire_mcp::testing::declared_tools_match;
//!
//! let asset_toml = r#"
//! [mcp-server]
//! tools = [
//!   { name = "search", read-only = true, destructive = false, idempotent = true, open-world = false },
//! ]
//! "#;
//! // A server's test gets these from the running server's `tools/list`, and
//! // passes `include_str!("../asset.toml")`.
//! let tools: Vec<rmcp::model::Tool> = serde_json::from_value(serde_json::json!([{
//!     "name": "search",
//!     "inputSchema": { "type": "object" },
//!     "annotations": { "readOnlyHint": true, "destructiveHint": false, "idempotentHint": true },
//! }]))?;
//! assert_eq!(
//!     declared_tools_match(asset_toml, &tools),
//!     Err("`search` does not state openWorldHint explicitly".to_owned())
//! );
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! # Changing this crate changes every server
//!
//! A change here changes every server binary while nothing in their asset
//! directories changes, so `cargo xtask validate` doesn't see it. Review
//! checks the rule instead: a change that alters what a server binary does,
//! and every security fix, raises each affected asset's version and adds a
//! CHANGELOG entry in the same pull request
//! ([`docs/versioning.md`, Tool and MCP server binaries][versioning-binaries]).
//!
//! [ADR-0002]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0002-mcp-server-and-tool-packaging.md
//! [ADR-0007]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0007-mcp-server-authorization.md
//! [architecture-3.5]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#35-mcp-server
//! [architecture-6.4]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#64-catalog-validation-cargo-xtask-validate-also-a-ci-gate
//! [security-4]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/security-model.md#4-mcp-servers
//! [versioning-binaries]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md#tool-and-mcp-server-binaries-not-checked

pub mod auth;
mod http;
pub mod limits;
mod provider;
mod stdio;
pub mod testing;
mod url;

pub use auth::{AuthConfig, Caller};
pub use http::{
    HttpLimits, HttpOptions, HttpServer, MAX_BODY_BYTES, is_loopback, may_serve_without_auth,
};
pub use stdio::serve_stdio;

/// The MCP protocol versions Grimoire's servers speak: 2026-07-28, and
/// 2025-11-25 for clients that still use the earlier handshake. Both are served
/// statelessly over HTTP.
pub const PROTOCOL_VERSIONS: [rmcp::model::ProtocolVersion; 2] = [
    rmcp::model::ProtocolVersion::V_2025_11_25,
    rmcp::model::ProtocolVersion::V_2026_07_28,
];

/// A server that could not start or stopped with an error.
#[derive(Debug)]
pub struct ServeError(pub String);

impl std::fmt::Display for ServeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for ServeError {}

/// Writes one diagnostic line to stderr. Never pass it a token.
pub fn log(message: &str) {
    use std::io::Write as _;
    let mut stderr = std::io::stderr().lock();
    let _ = writeln!(stderr, "{message}");
}

/// Completes on `SIGINT` (Ctrl-C) or, on Unix, `SIGTERM`.
pub(crate) async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut term) => {
                tokio::select! {
                    _ = tokio::signal::ctrl_c() => {}
                    _ = term.recv() => {}
                }
            }
            Err(_) => {
                let _ = tokio::signal::ctrl_c().await;
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}
