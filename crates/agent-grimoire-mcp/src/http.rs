//! The Streamable HTTP transport, hardened (`MCP-SPEC`, ADR-0007).
//!
//! Each request is checked in this order before rmcp sees it:
//! 1. **`Host`** must be one this server answers to (403). With Origin
//!    validation, this defeats DNS rebinding.
//! 2. **`Origin`**, when present, must be on the allowlist (403). Requests
//!    without one, as non-browser clients send, pass. The default allowlist is
//!    empty, so no browser page can call the server.
//! 3. **Path and method:** RFC 9728 metadata on `GET`, then `POST` to the MCP
//!    path only (404, or 405 with `Allow`).
//! 4. **Authorization**, when configured (401, 403 or 400 with an RFC 6750
//!    challenge). A request that passes carries a [`Caller`](crate::Caller)
//!    in its extensions. Then the `Authorization` header is removed from
//!    every request, configured or not, so no handler ever sees a token.
//!
//! 5. **Protocol version:** `MCP-Protocol-Version` must name a version the
//!    server speaks (400, -32022). It is required on everything but a legacy
//!    `initialize` (400, -32020), and a 2026-07-28 request must also carry it
//!    in `_meta` (400, -32602). rmcp itself checks the header only against
//!    every version it knows.
//!
//! rmcp then applies its own Host, Origin, media-type, body-size and MCP
//! header checks, configured from the same lists: stateless, JSON responses,
//! no sessions. 2025-11-25 clients are served statelessly too.
//!
//! Each connection serves one request (no keep-alive, so no pipelining) and
//! is closed after [`HttpLimits::connection_deadline`] whatever it is doing, so
//! a client that stops reading cannot hold a connection slot.
//!
//! Without authorization the server only listens on a loopback address, and
//! even then any local process can reach it: prefer stdio for local use.

use std::collections::BTreeSet;
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use bytes::Bytes;
use http::{HeaderValue, Method, Request, Response, StatusCode, header};
use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt as _, Full};
use hyper::body::Incoming;
use hyper_util::rt::{TokioIo, TokioTimer};
use rmcp::ServerHandler;
use rmcp::transport::streamable_http_server::session::never::NeverSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use tokio::net::TcpListener;
use tokio::sync::{Semaphore, watch};
use tokio::task::JoinSet;

use crate::auth::{AuthConfig, Authenticator};
use crate::url::is_loopback_host;
use crate::{ServeError, shutdown_signal};

/// The largest request body accepted.
pub const MAX_BODY_BYTES: usize = 1024 * 1024;
/// The most time a client may take to send request headers.
const HEADER_TIMEOUT: Duration = Duration::from_secs(10);
/// The most time one request may take.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(60);
/// The protocol-version header.
const VERSION_HEADER: &str = "mcp-protocol-version";
/// How long in-flight requests get to finish on shutdown.
const SHUTDOWN_GRACE: Duration = Duration::from_secs(5);

/// Connection limits.
#[derive(Debug, Clone, Copy)]
pub struct HttpLimits {
    /// Connections served at once; more wait to be accepted.
    pub max_connections: usize,
    /// The longest a connection may stay open, whatever it is doing.
    pub connection_deadline: Duration,
}

impl Default for HttpLimits {
    fn default() -> Self {
        Self {
            max_connections: 64,
            // Time to send the headers, run the request, and write the reply.
            connection_deadline: HEADER_TIMEOUT + REQUEST_TIMEOUT + Duration::from_secs(5),
        }
    }
}

/// How to serve over HTTP.
#[derive(Debug, Clone)]
pub struct HttpOptions {
    /// The address to listen on. Without `auth` it must be a loopback address.
    pub listen: SocketAddr,
    /// The MCP endpoint's path. With `auth`, the resource URI's path is used.
    pub path: String,
    /// Browser origins (`scheme://host:port`) allowed to call the server.
    pub allowed_origins: Vec<String>,
    /// More `Host` values to answer to, for example a reverse proxy's public name.
    pub allowed_hosts: Vec<String>,
    /// Access-token checks. `None` serves without authorization.
    pub auth: Option<AuthConfig>,
    /// Connection limits.
    pub limits: HttpLimits,
}

type Body = BoxBody<Bytes, Infallible>;

struct App<S: ServerHandler> {
    service: StreamableHttpService<S, NeverSessionManager>,
    limits: HttpLimits,
    auth: Option<Authenticator>,
    path: String,
    hosts: BTreeSet<String>,
    origins: BTreeSet<String>,
}

fn text(status: StatusCode, message: &str) -> Response<Body> {
    let mut response = Response::new(Full::new(Bytes::from(format!("{message}\n"))).boxed());
    *response.status_mut() = status;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

/// A JSON-RPC error with HTTP 400, echoing the request id when there is one.
fn rpc_error(
    id: Option<&serde_json::Value>,
    code: i64,
    message: &str,
    data: Option<serde_json::Value>,
) -> Response<Body> {
    let mut error = serde_json::json!({ "code": code, "message": message });
    if let Some(data) = data {
        error["data"] = data;
    }
    let mut body = serde_json::json!({ "jsonrpc": "2.0", "error": error });
    if let Some(id) = id {
        body["id"] = id.clone();
    }
    let mut response = Response::new(Full::new(Bytes::from(body.to_string())).boxed());
    *response.status_mut() = StatusCode::BAD_REQUEST;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    response
}

type Refusal = Box<Response<Body>>;

/// The `MCP-Protocol-Version` header, if any, refusing one that names a
/// version this server does not speak.
fn version_header(headers: &http::HeaderMap) -> Result<Option<String>, Refusal> {
    let mut values = headers.get_all(VERSION_HEADER).iter();
    let value = match (values.next(), values.next()) {
        (None, _) => return Ok(None),
        (Some(value), None) => value.to_str().map_err(|_| {
            Box::new(rpc_error(
                None,
                -32020,
                "the MCP-Protocol-Version header is not ASCII",
                None,
            ))
        })?,
        (Some(_), Some(_)) => {
            return Err(Box::new(rpc_error(
                None,
                -32020,
                "more than one MCP-Protocol-Version header",
                None,
            )));
        }
    };
    let supported: Vec<&str> = crate::PROTOCOL_VERSIONS
        .iter()
        .map(rmcp::model::ProtocolVersion::as_str)
        .collect();
    if !supported.contains(&value) {
        return Err(Box::new(rpc_error(
            None,
            -32022,
            "Unsupported protocol version",
            Some(serde_json::json!({ "supported": supported, "requested": value })),
        )));
    }
    Ok(Some(value.to_owned()))
}

/// Reads the body, refusing one over [`MAX_BODY_BYTES`].
async fn read_body(request: Request<Incoming>) -> Result<(http::request::Parts, Bytes), Refusal> {
    let too_large = || Box::new(text(StatusCode::PAYLOAD_TOO_LARGE, "Payload Too Large"));
    let declared = request
        .headers()
        .get(header::CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<usize>().ok());
    if declared.is_some_and(|n| n > MAX_BODY_BYTES) {
        return Err(too_large());
    }
    let (parts, body) = request.into_parts();
    match http_body_util::Limited::new(body, MAX_BODY_BYTES)
        .collect()
        .await
    {
        Ok(collected) => Ok((parts, collected.to_bytes())),
        Err(e)
            if e.downcast_ref::<http_body_util::LengthLimitError>()
                .is_some() =>
        {
            Err(too_large())
        }
        Err(_) => Err(Box::new(text(
            StatusCode::BAD_REQUEST,
            "the request body could not be read",
        ))),
    }
}

/// The header rules that depend on the message: the header is required on
/// everything but a legacy `initialize`, and a 2026-07-28 request also needs
/// the version in `_meta`. A body that is not JSON is left to rmcp, which
/// refuses it.
fn check_message(header: Option<&str>, body: &[u8]) -> Result<(), Refusal> {
    let Ok(message) = serde_json::from_slice::<serde_json::Value>(body) else {
        return Ok(());
    };
    let method = message.get("method").and_then(serde_json::Value::as_str);
    let id = message.get("id");
    match header {
        None if method.is_some_and(|m| m != "initialize") => Err(Box::new(rpc_error(
            id,
            -32020,
            "the MCP-Protocol-Version header is required",
            None,
        ))),
        Some("2026-07-28")
            if id.is_some()
                && method.is_some()
                && message
                    .pointer("/params/_meta/io.modelcontextprotocol~1protocolVersion")
                    .is_none() =>
        {
            Err(Box::new(rpc_error(
                id,
                -32602,
                "Invalid params: the request _meta needs io.modelcontextprotocol/protocolVersion",
                None,
            )))
        }
        _ => Ok(()),
    }
}

/// Checks the protocol version (see the module docs) and buffers the body.
async fn version_gate(request: Request<Incoming>) -> Result<Request<Full<Bytes>>, Refusal> {
    let header = version_header(request.headers())?;
    let (parts, body) = read_body(request).await?;
    check_message(header.as_deref(), &body)?;
    Ok(Request::from_parts(parts, Full::new(body)))
}

fn not_allowed(allow: &'static str) -> Response<Body> {
    let mut response = text(StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed");
    response
        .headers_mut()
        .insert(header::ALLOW, HeaderValue::from_static(allow));
    response
}

/// An `Origin` value in its RFC 6454 serialization, lowercased, with default
/// ports dropped. `None` if it is not a `scheme://host[:port]` origin.
fn normalize_origin(value: &str) -> Option<String> {
    let lower = value.trim().to_ascii_lowercase();
    let (scheme, rest) = lower.split_once("://")?;
    if rest.is_empty() || rest.contains(['/', '?', '#', '@']) || !rest.is_ascii() {
        return None;
    }
    let rest = match (scheme, rest.rsplit_once(':')) {
        ("https", Some((host, "443"))) | ("http", Some((host, "80"))) if !host.ends_with('[') => {
            host.to_owned()
        }
        _ => rest.to_owned(),
    };
    Some(format!("{scheme}://{rest}"))
}

impl<S: ServerHandler> App<S> {
    fn host_allowed(&self, request: &Request<Incoming>) -> bool {
        let host = request
            .headers()
            .get(header::HOST)
            .and_then(|h| h.to_str().ok())
            .map(str::to_owned)
            .or_else(|| request.uri().authority().map(|a| a.as_str().to_owned()));
        host.is_some_and(|h| self.hosts.contains(&h.to_ascii_lowercase()))
    }

    fn origin_allowed(&self, request: &Request<Incoming>) -> bool {
        let mut values = request.headers().get_all(header::ORIGIN).iter();
        let Some(origin) = values.next() else {
            return true;
        };
        values.next().is_none()
            && origin
                .to_str()
                .ok()
                .and_then(normalize_origin)
                .is_some_and(|o| self.origins.contains(&o))
    }

    async fn handle(&self, request: Request<Incoming>) -> Response<Body> {
        // `Connection: close` on every reply: one request per connection.
        let mut response = self.serve(request).await;
        response
            .headers_mut()
            .insert(header::CONNECTION, HeaderValue::from_static("close"));
        response
    }

    async fn serve(&self, mut request: Request<Incoming>) -> Response<Body> {
        if !self.host_allowed(&request) {
            return text(
                StatusCode::FORBIDDEN,
                "Forbidden: this server does not answer to that Host",
            );
        }
        if !self.origin_allowed(&request) {
            return text(
                StatusCode::FORBIDDEN,
                "Forbidden: that Origin may not call this server",
            );
        }
        let path = request.uri().path();
        if let Some(auth) = &self.auth
            && auth.is_metadata_path(path)
        {
            return if request.method() == Method::GET || request.method() == Method::HEAD {
                let mut response =
                    Response::new(Full::new(Bytes::from(auth.metadata().to_string())).boxed());
                let headers = response.headers_mut();
                headers.insert(
                    header::CONTENT_TYPE,
                    HeaderValue::from_static("application/json"),
                );
                headers.insert(
                    header::CACHE_CONTROL,
                    HeaderValue::from_static("max-age=3600"),
                );
                response
            } else {
                not_allowed("GET, HEAD")
            };
        }
        if path != self.path {
            return text(StatusCode::NOT_FOUND, "Not Found");
        }
        if request.method() != Method::POST {
            return not_allowed("POST");
        }
        if let Some(auth) = &self.auth {
            match auth.check(request.headers()) {
                Ok(caller) => {
                    request.extensions_mut().insert(caller);
                }
                Err(challenge) => {
                    let (status, value) = auth.challenge(&challenge);
                    let mut response =
                        text(status, status.canonical_reason().unwrap_or("Unauthorized"));
                    response
                        .headers_mut()
                        .insert(header::WWW_AUTHENTICATE, value);
                    return response;
                }
            }
        }
        // The token stops here. rmcp hands each handler the request's
        // `http::request::Parts`, headers included, and reads no
        // `Authorization` header itself; a handler needs only the `Caller`.
        // Without authorization, a token a client sends anyway was never
        // checked, and goes too.
        request.headers_mut().remove(header::AUTHORIZATION);
        let work = async {
            match version_gate(request).await {
                Ok(request) => self.service.handle(request).await,
                Err(response) => *response,
            }
        };
        match tokio::time::timeout(REQUEST_TIMEOUT, work).await {
            Ok(response) => response,
            Err(_) => text(StatusCode::SERVICE_UNAVAILABLE, "the request took too long"),
        }
    }
}

/// A bound, not yet running, HTTP server.
pub struct HttpServer<S: ServerHandler> {
    listener: TcpListener,
    app: Arc<App<S>>,
}

impl<S: ServerHandler> std::fmt::Debug for HttpServer<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpServer")
            .field("listener", &self.listener)
            .field("path", &self.app.path)
            .finish_non_exhaustive()
    }
}

impl<S: ServerHandler> HttpServer<S> {
    /// Checks `options`, binds the listener and prepares the service. `factory`
    /// makes the handler for each request: the protocol is stateless.
    ///
    /// # Errors
    ///
    /// Fails for an unsafe configuration (no authorization on a non-loopback
    /// address, an invalid origin), an invalid authorization setup, or a
    /// listener that cannot be bound.
    pub async fn bind<F>(factory: F, options: HttpOptions) -> Result<Self, ServeError>
    where
        F: Fn() -> S + Send + Sync + 'static,
    {
        if options.auth.is_none() && !options.listen.ip().is_loopback() {
            return Err(ServeError(format!(
                "refusing to serve without authorization on {}: listen on 127.0.0.1 or [::1], or configure authorization",
                options.listen
            )));
        }
        let auth = options
            .auth
            .map(Authenticator::new)
            .transpose()
            .map_err(|e| ServeError(e.to_string()))?;
        let path = match &auth {
            Some(auth) => auth.endpoint().path.clone(),
            None => options.path.clone(),
        };
        if !path.starts_with('/') {
            return Err(ServeError(format!(
                "the MCP path `{path}` must start with `/`"
            )));
        }
        let mut origins = BTreeSet::new();
        for origin in &options.allowed_origins {
            let normalized = normalize_origin(origin)
                .filter(|o| o != "null")
                .ok_or_else(|| {
                    ServeError(format!(
                        "`{origin}` is not an origin such as https://app.example:8443"
                    ))
                })?;
            origins.insert(normalized);
        }
        let listener = TcpListener::bind(options.listen)
            .await
            .map_err(|e| ServeError(format!("cannot listen on {}: {e}", options.listen)))?;
        let local = listener
            .local_addr()
            .map_err(|e| ServeError(format!("cannot read the bound address: {e}")))?;
        let mut hosts: BTreeSet<String> = BTreeSet::new();
        hosts.insert(local.to_string().to_ascii_lowercase());
        if local.ip().is_loopback() {
            for name in ["localhost", "127.0.0.1", "[::1]"] {
                hosts.insert(format!("{name}:{}", local.port()));
            }
        }
        if let Some(auth) = &auth {
            hosts.extend(auth.endpoint().host_headers());
        }
        for host in &options.allowed_hosts {
            hosts.insert(host.to_ascii_lowercase());
        }
        if hosts.iter().any(String::is_empty) {
            return Err(ServeError("an allowed host is empty".to_owned()));
        }
        let config = StreamableHttpServerConfig::default()
            .with_legacy_session_mode(false)
            .with_json_response(true)
            .with_allowed_hosts(hosts.iter().cloned())
            .with_allowed_origins(origins.iter().cloned())
            .enforce_origin_validation()
            .with_max_request_body_bytes(MAX_BODY_BYTES);
        let service = StreamableHttpService::new(
            move || Ok(factory()),
            Arc::new(NeverSessionManager::default()),
            config,
        );
        Ok(Self {
            listener,
            app: Arc::new(App {
                service,
                limits: options.limits,
                auth,
                path,
                hosts,
                origins,
            }),
        })
    }

    /// The bound address.
    ///
    /// # Errors
    ///
    /// Fails if the operating system cannot report it.
    pub fn local_addr(&self) -> Result<SocketAddr, ServeError> {
        self.listener
            .local_addr()
            .map_err(|e| ServeError(e.to_string()))
    }

    /// Serves until `SIGINT` or `SIGTERM`.
    ///
    /// # Errors
    ///
    /// Fails if accepting connections fails.
    pub async fn run(self) -> Result<(), ServeError> {
        self.run_until(shutdown_signal()).await
    }

    /// Serves until `stop` completes, then gives in-flight requests a few
    /// seconds to finish.
    ///
    /// # Errors
    ///
    /// Fails if accepting connections fails.
    pub async fn run_until(self, stop: impl Future<Output = ()>) -> Result<(), ServeError> {
        let (stopping, stop_rx) = watch::channel(false);
        let permits = Arc::new(Semaphore::new(self.app.limits.max_connections.max(1)));
        let deadline = self.app.limits.connection_deadline;
        let mut connections = JoinSet::new();
        tokio::pin!(stop);
        loop {
            let permit = tokio::select! {
                () = &mut stop => break,
                permit = Arc::clone(&permits).acquire_owned() => permit.map_err(|e| ServeError(e.to_string()))?,
            };
            let (stream, _) = tokio::select! {
                () = &mut stop => break,
                accepted = self.listener.accept() => match accepted {
                    Ok(pair) => pair,
                    Err(e) => {
                        crate::log(&format!("accepting a connection failed: {e}"));
                        continue;
                    }
                },
            };
            let app = Arc::clone(&self.app);
            let mut stop_rx = stop_rx.clone();
            connections.spawn(async move {
                let service = hyper::service::service_fn(move |request| {
                    let app = Arc::clone(&app);
                    async move { Ok::<_, Infallible>(app.handle(request).await) }
                });
                let connection = hyper::server::conn::http1::Builder::new()
                    .timer(TokioTimer::new())
                    .header_read_timeout(HEADER_TIMEOUT)
                    .keep_alive(false)
                    .serve_connection(TokioIo::new(stream), service);
                tokio::pin!(connection);
                tokio::select! {
                    _ = connection.as_mut() => {}
                    _ = stop_rx.changed() => {
                        connection.as_mut().graceful_shutdown();
                        let _ = tokio::time::timeout(SHUTDOWN_GRACE, connection).await;
                    }
                    // Dropping the connection closes it, even if the client stopped reading.
                    () = tokio::time::sleep(deadline) => {}
                }
                drop(permit);
            });
            // Reap finished connections so the set does not grow.
            while connections.try_join_next().is_some() {}
        }
        let _ = stopping.send(true);
        let drained = tokio::time::timeout(SHUTDOWN_GRACE, async {
            while connections.join_next().await.is_some() {}
        })
        .await;
        if drained.is_err() {
            connections.abort_all();
        }
        Ok(())
    }
}

/// Whether `listen` is an address that may serve without authorization.
#[must_use]
pub fn may_serve_without_auth(listen: &SocketAddr) -> bool {
    listen.ip().is_loopback()
}

/// Whether a host name or address names this machine.
#[must_use]
pub fn is_loopback(host: &str) -> bool {
    is_loopback_host(host)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origins_normalize() {
        assert_eq!(
            normalize_origin("HTTPS://App.Example:443").as_deref(),
            Some("https://app.example")
        );
        assert_eq!(
            normalize_origin("http://localhost:3000").as_deref(),
            Some("http://localhost:3000")
        );
        assert_eq!(
            normalize_origin("http://[::1]:80").as_deref(),
            Some("http://[::1]")
        );
        assert_eq!(normalize_origin("null"), None);
        assert_eq!(normalize_origin("https://a.example/path"), None);
        assert_eq!(normalize_origin("https://u@a.example"), None);
    }
}
