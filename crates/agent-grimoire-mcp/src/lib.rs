//! Shared hardening for Grimoire's MCP servers (ADR-0002, ADR-0007).
//!
//! Every server asset under `catalog/mcp-servers/` uses this crate, so each one
//! starts from the same secure defaults instead of rmcp's:
//!
//! - [`serve_stdio`]: stdio, with nothing but MCP on stdout and a clean stop on
//!   end of input, `SIGINT` or `SIGTERM`.
//! - [`HttpServer`]: Streamable HTTP with `Host` and `Origin` checks before
//!   anything else, loopback-only unless authorization is configured, request
//!   size and time limits, and stateless rmcp settings.
//! - [`auth`]: the OAuth resource-server checks: RFC 9728 metadata, RFC 6750
//!   challenges, and RFC 9068 access tokens verified against a local JWKS file.
//! - [`limits`]: concurrency, rate and time limits for tool work.
//! - [`testing`]: checks a server's live tools against its `asset.toml`.
//!
//! The crate logs to stderr only, and never logs a token.

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
