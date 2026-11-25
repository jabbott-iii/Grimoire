# Subsystem: `agent-grimoire-mcp`

`agent-grimoire-mcp` is the shared hardening crate (a crate is a Rust package) for Grimoire's Model Context Protocol (MCP) servers. MCP lets an AI client call tools that a separate server program offers. Every MCP server asset under `catalog/mcp-servers/` serves through this crate, so each one starts from the same secure defaults: the two transports, the `Host` and `Origin` checks, the limits on requests and tool work, and the checks of an OAuth resource server (a server that accepts access tokens that a separate authorization server issued). It guarantees that a server never serves HTTP without authorization except on a loopback address, never lets a tool handler see a bearer token, and never makes an outbound connection to check one. [ADR-0007](../adr/0007-mcp-server-authorization.md) (server-side MCP authorization and HTTP hardening: JWT access tokens checked against a local key file, in this one crate) shaped it, and [ADR-0002](../adr/0002-mcp-server-and-tool-packaging.md) (each MCP server and tool is a binary of its own, outside the `grimoire` CLI) keeps it out of the CLI.

The crate docs, the `//!` lines at the top of [`crates/agent-grimoire-mcp/src/lib.rs`](../../crates/agent-grimoire-mcp/src/lib.rs), are the API reference and give every type and limit in detail. The crate is never published (`publish = false`), so it has no docs.rs page; `cargo doc -p agent-grimoire-mcp --no-deps` builds that reference locally. This spec summarizes it, says why it is built this way, and lists what the tests hold it to.

## Why one hardening crate

rmcp, the official Rust library for MCP that the servers are built on, has no server-side authorization: its auth module is an OAuth client, and its server examples skip the audience, issuer and expiry checks. Its HTTP defaults are also unsafe for Grimoire:
- Origin validation, the check of the `Origin` header that a browser sets to the site that sent a request, is off;
- a request without `MCP-Protocol-Version`, the header that names the protocol version, is treated as version 2025-03-26;
- legacy sessions (server-side state tied to an `Mcp-Session-Id` header) are on;
- a server reports rmcp's own name and version.

If each server fixed these itself, one forgotten setting would leave one server open. So the fixes live in one crate, catalog validation requires every server to use it, and a fix here reaches every server at once.

ADR-0007 compared four ways to check access tokens. It chose JSON Web Token (JWT) access tokens verified against a local key file, over fetching the keys from the issuer (about 50 more crates, an HTTPS client and a network dependency), token introspection (a network round trip for every new token), and no authorization at all. What that trades away:
- Operators rotate keys by replacing the key file.
- Opaque tokens (tokens whose contents the server can't read) and authorization servers that don't issue JWT access tokens aren't supported.
- Browser-based MCP clients can't connect, because the crate sends no Cross-Origin Resource Sharing (CORS) headers.

Keeping the crate out of the CLI keeps the CLI small. The installer runs in continuous integration (CI) with write access to repositories, so it carries no async runtime, HTTP stack or authorization code. Installing a server only writes the client's entry for it; Grimoire never runs the server.

## Transports

Built in M3.1 as described here:

```rust
pub async fn serve_stdio<S: ServerHandler>(server: S) -> Result<(), ServeError>;

pub struct HttpOptions {
    pub listen: SocketAddr,          // without `auth`, a loopback address only
    pub path: String,                // the MCP endpoint; with `auth`, the resource URI's path wins
    pub allowed_origins: Vec<String>,
    pub allowed_hosts: Vec<String>,
    pub auth: Option<AuthConfig>,    // `None` serves without authorization
    pub limits: HttpLimits,
}

pub const PROTOCOL_VERSIONS: [rmcp::model::ProtocolVersion; 2]; // 2026-07-28 and 2025-11-25
```

- **stdio.** [`serve_stdio`] serves MCP on standard input and output (stdin and stdout), which is how a client runs a server it starts itself. Nothing but MCP messages goes to stdout. The server stops cleanly at the end of input, on `SIGINT` (Ctrl-C) or, on Unix, on `SIGTERM`.
- **Streamable HTTP**, MCP's HTTP transport, for a server that runs on its own. `HttpServer::bind` checks the `HttpOptions` and binds the address; `HttpServer::run` then serves until a stop signal, and `HttpServer::run_until` until a future completes. The protocol is stateless, so `bind` takes a function that makes the handler for each request.
- **Protocol versions.** `PROTOCOL_VERSIONS` is what a server's `supported_protocol_versions` returns: 2026-07-28, and 2025-11-25 for clients that still use the earlier handshake. MCP names its versions by date. rmcp runs stateless, with plain JSON responses (an event stream only when a handler sends a notification or a request before its reply) and no `Mcp-Session-Id`, for 2025-11-25 clients too.

## How an HTTP request is checked

ADR-0007 fixed the order of the checks so that a probe (a request that tests what the server accepts) learns as little as possible. `HttpServer` checks each request in this order before rmcp sees it:

1. **`Host`** must be one the server answers to (403 otherwise): the bound address; `localhost`, `127.0.0.1` and `[::1]` with its port, on a loopback address; the resource's host, with authorization; and `allowed_hosts`. With the `Origin` check, this defeats DNS rebinding, where a web page's domain name is switched to resolve to a local address.
2. **`Origin`**, when present, must be on `allowed_origins` (403). The list is empty unless the server is given origins, so by default no browser page can call the server, and `null` is never allowed. A request without `Origin`, as non-browser clients send, passes.
3. **The path and method.** With authorization, the RFC 9728 metadata is served on `GET` and `HEAD` at its well-known paths. Everything else must be a `POST` to the MCP path: another path gets 404, and another method 405 with `Allow`.
4. **Authorization**, when configured (401, 403 or 400, with an RFC 6750 challenge; see [Authorization](#authorization)).
5. **The protocol version.** `MCP-Protocol-Version` must name one of the `PROTOCOL_VERSIONS` (400, with JSON-RPC error code -32022; MCP's messages are JSON-RPC, a JSON format for requests and their replies). The header is required on every JSON-RPC request and notification except a legacy `initialize` (400, -32020). A 2026-07-28 request must also carry the version in `_meta` (400, -32602). This step comes after authorization, so the body of an unauthenticated request is never parsed. These checks replace rmcp's `stateless_protocol_metadata_required`, which would also refuse 2025-11-25 clients, who never send `_meta`.

rmcp then applies its own `Host`, `Origin`, media-type, body-size and MCP header checks, configured from the same lists.

## Authorization

Built in M3.1 as described here:

```rust
pub struct AuthConfig {
    pub resource: String,               // canonical URI of the endpoint; the default audience
    pub issuer: String,                 // tokens' `iss` must equal it
    pub jwks: PathBuf,                  // the issuer's signing keys
    pub audiences: Vec<String>,         // more accepted `aud` values
    pub required_scopes: Vec<String>,
    pub resource_name: Option<String>,
    pub allow_jwt_typ: bool,            // also accept `typ: JWT`; off by default
    pub leeway_secs: u64,               // at most `auth::MAX_LEEWAY_SECS` (60)
}

pub struct Caller {                     // what a valid token says, in the request's extensions
    pub subject: Option<String>,
    pub client_id: Option<String>,
    pub scopes: Vec<String>,
}
```

The checks follow three Internet standards, or RFCs (Requests for Comments):
- **RFC 9728 metadata:** a JSON document, at a well-known URL, that tells a client which authorization server to get a token from. It is built from the configured `resource`, never from request headers, and names the resource, the issuer as its authorization server, only the required scopes, `bearer_methods_supported: ["header"]` and the resource name. It is served at the path-inserted URL (`/.well-known/oauth-protected-resource/mcp`) and at the root well-known URL, which Claude Code documents checking first, with `Cache-Control: max-age=3600`. The root copy names `/mcp` as its resource, which RFC 9728 doesn't allow for a root document, so a strict client rejects it and uses the challenge's `resource_metadata` instead. Whether Claude Code needs the root copy is UNVERIFIED (`CC-P3` in [`../specs.md`](../specs.md)).
- **RFC 6750 challenges:** the `WWW-Authenticate` response header that says why a request was refused, naming the metadata's URL in `resource_metadata`. No credentials, or a scheme other than Bearer, get 401 with no error code; malformed or repeated credentials get 400 `invalid_request`; a token that was checked and failed gets 401 `invalid_token`; missing scopes get 403 `insufficient_scope`.
- **RFC 9068 access tokens:** JWTs (signed JSON documents that say who issued them, who they are for and what they allow), verified against a local JWKS file, a JSON Web Key Set that holds the authorization server's public keys.

Credentials are read from exactly one `Authorization: Bearer` header; a token in the query string is ignored. With authorization, the MCP path is the path of `AuthConfig::resource`, whatever `HttpOptions::path` says. A token is accepted only if all of these hold:
- its `typ` is `at+jwt` or `application/at+jwt`, which keeps out ID tokens (tokens that prove a sign-in rather than grant access); `JWT` too only with `allow_jwt_typ`. A token with no `typ` is always refused;
- it is signed with RS256 (RSA with SHA-256) or ES256 (an elliptic-curve signature on the P-256 curve, with SHA-256), by a key bound to that one algorithm, and a key's own `alg` must agree. RSA keys are 2048 to 8192 bits;
- its key is chosen by `kid` (key ID), or is the only key for its algorithm. Keys come only from the JWKS file, never from the token's `jku`, `x5u`, `jwk` or `x5c` headers;
- `iss` is exactly the configured issuer, as a string. jsonwebtoken, the JWT library underneath, also accepts an array that merely contains the issuer, so the crate checks this itself;
- `aud` contains the resource or one of `audiences`;
- `exp` (expiry) and `nbf` (not before) hold within the leeway. `aud`, `iss` and `exp` are required claims;
- it has every scope in `required_scopes`, read from `scope` or `scp`.

An access token may be at most 16 KiB (400 `invalid_request` above that), and the key file at most 1 MiB.

**The key file.** The keys come from a local file, so a server makes no outbound connection. On a request at least 5 seconds after the last check, the file is read again if its modification time has changed. (A new file with the old modification time, as `touch -r` or a restore can leave, isn't noticed.) When it is read again:
- a well-formed key set replaces the keys, even with no usable key in it, so removing a key revokes it;
- a deleted file revokes every key;
- only a file that can't be read or parsed, for example one being rewritten in place, keeps the previous keys.

**Browser clients** aren't supported: there are no CORS headers, and an `OPTIONS` preflight gets 403 when its `Origin` isn't allowed, and 405 at the MCP path otherwise.

## Limits

Over HTTP, `HttpServer` allows:
- request bodies of at most `MAX_BODY_BYTES`, 1 MiB (413 above that);
- one request per connection: no keep-alive (reusing a connection for another request), so no pipelining (sending several requests before the first reply);
- at most `HttpLimits::max_connections` connections at once, 64 by default; more wait to be accepted;
- 10 seconds to send the request headers, after which the connection is closed with no answer, and 60 seconds per request, after which the answer is 503;
- every connection closed after `HttpLimits::connection_deadline`, 75 seconds by default, whatever it is doing, so a client that stops reading can't keep its slot;
- on shutdown, 5 seconds for requests in flight to finish.

For tool work, MCP requires servers to rate-limit tool calls. Built in M3.1 as described here:

```rust
pub struct LimitConfig {
    pub max_concurrent: usize, // default 8
    pub per_second: u32,       // default 20, on average
    pub burst: u32,            // default 40
    pub timeout: Duration,     // default 10 s
}

pub enum Refused { RateLimited, Busy, TimedOut, Failed }
```

A server keeps one `limits::Limits` and runs each tool's blocking work through it. A call over the time limit gets `Refused::TimedOut`, but its work keeps its slot until it actually ends, so a timed-out call still counts against the limit on calls at once. Each `Refused` value's message is written for the model, as the tool's error. `limits::truncate` cuts an answer to a number of characters.

## Serving without authorization

Without `HttpOptions::auth`, `HttpServer::bind` refuses any listen address that isn't a loopback address (one that only this machine can reach, such as `127.0.0.1` or `[::1]`), before it opens a socket. `may_serve_without_auth` is the same test, and `is_loopback` says whether a URL's host names this machine: `localhost`, `127.x.y.z`, or `[::1]` in brackets, each without a port.

Even on loopback, any process on the machine can connect: MCP's own security guidance says loopback is not an authentication boundary. So stdio is the default for local use. `project-docs` and the server template need an explicit `--no-auth` to serve HTTP without authorization, so that doing so is a choice.

## Logging and the token

The crate logs to standard error (stderr) only, through `log`, and never logs, stores or forwards a token. Over stdio, stdout carries only MCP messages, so any other line there would corrupt the protocol. MCP doesn't allow a server to pass a token on, and a token in a log can leak.

The token stops at the authorization check. rmcp gives each handler the request's `http::request::Parts`, headers included, so `HttpServer` removes the `Authorization` header from every request before rmcp sees it, whether authorization is configured or not. A handler learns who sent a request from its `Caller` instead (`parts.extensions.get::<Caller>()`), and rmcp itself reads no `Authorization` header. This was fixed on 2026-11-22 (commit `0218f00`): before it, a tool handler could read the raw bearer token from the headers rmcp passed it. The other headers still reach handlers, and a client may put anything in them, so a server's own code doesn't log or forward a request's headers whole.

## Security of MCP servers

An MCP server asset is a separate Rust process. A client either starts it and talks to it over stdin and stdout (the stdio transport), or connects to it over HTTP (MCP's Streamable HTTP transport). Grimoire only writes the client's entry for it; it never runs the server.

Every server serves through this crate, and catalog validation requires it. ADR-0007 put server-side authorization and HTTP hardening in this one crate, because rmcp has no server-side authorization and its HTTP defaults are unsafe for Grimoire.

The rows are labeled M1 to M12 (M for MCP). They are threat labels, not roadmap milestones. F4 and F14 are rows of the threat table in [`../01-architecture.md`, "Threats and mitigations"](../01-architecture.md#threats-and-mitigations).

| # | Threat | Mitigation | Result |
|---|---|---|---|
| M1 | A web page reaches a local HTTP server through DNS rebinding (the page's domain name is switched to resolve to a local address, so the browser treats the local server as part of the page's own site) or a cross-origin request (one a page sends to another site) | The `Host` header (the server name the request was sent to) must be one the server answers to. A present `Origin` header (which a browser sets to the site that sent the request) must be on an allowlist that is empty by default. Both are checked before anything else, including authorization, so that a probe learns as little as possible (ADR-0007). rmcp checks them again. | 403 |
| M2 | An HTTP server is reachable from the network without authorization | Without OAuth settings (OAuth is the standard for issuing access tokens; these settings make the server check them), the server listens only on a loopback address (such as `127.0.0.1`, reachable only from the same machine). `project-docs` (the catalog's documentation server) and every server made from the `cargo xtask new` template need an explicit `--no-auth` to serve HTTP without authorization. | Refuses to start |
| M3 | Another local process or user connects to a loopback server | Not prevented: loopback "is not an authentication boundary" (MCP's own security guidance, `MCP-P3`), because any process on the machine can connect to it. Grimoire configures stdio by default; local HTTP is something the user starts. | Documented |
| M4 | A token issued for another service is accepted: token passthrough (a server accepts, or passes on, a token that wasn't issued for it), or a confused deputy (a trusted program tricked into using its authority for someone else) | The checks of RFC 9068, the standard for JSON Web Token (JWT) access tokens: the token type (`typ`), the issuer (`iss`), the audience it was issued for (`aud`), the validity window (`exp`/`nbf`) and the required scopes (what the token allows). Tokens are never forwarded, and no tool handler gets one to forward (M6). | 401 / 403 |
| M5 | A forged token through algorithm confusion (a token that names another signing algorithm than its key was made for), a weak key, or a key named by the token itself | Only RS256 (RSA) and ES256 (elliptic-curve) signatures, each bound to its key type and the key's own `alg`. RSA keys of 2048 to 8192 bits. Keys come only from the operator's JWKS file (a JSON Web Key Set: the authorization server's public keys, which the person running the server supplies), never from the token's `jku`, `x5u`, `jwk` or `x5c` headers, which point to or carry a key. | 401 |
| M6 | A token leaks | Read only from the `Authorization` header; a token in the query string (the `?…` part of a URL) is ignored. Never logged or stored. Once checked, the header is removed from the request, so no tool handler sees the token: rmcp gives every handler the request's headers, and needs no `Authorization` header itself. A handler learns who called from the validated claims instead. Without authorization configured, the header is removed too. A `url` in `grimoire.toml` cannot carry user information (`user:password@`), a query or a fragment (`#…`); the `ServerUrl` type checks this. | |
| M7 | A tool argument reaches files outside the documentation: `..`, absolute paths, symlinks, `.env` | `project-docs` reads through a capability handle on the root (a handle that reaches only paths beneath it, as in F4) and opens every component no-follow. It never serves hidden names, dependency or build directories, or non-documentation files. When a path is refused for its form, the caller is told why, and the answer depends only on the string it sent. A path refused because of what is on disk (a symbolic link, a directory, a missing file) reads exactly like a missing one, so a caller can't probe for files it may not read. | Not served |
| M8 | Resource exhaustion: a client uses up memory, connections or time, so others can't be served | HTTP: 1 MiB request bodies, one request per connection, 64 connections, 10 s to send headers and 60 s per request, and every connection closed after 75 s, so a client that stops reading cannot keep its slot. Tool work: 8 at once and about 20 per second. After 10 s the caller gets an error, but the work keeps its slot until it ends. Reads: 1 MiB per document, 30,000 characters per answer, 5,000 documents and 64 MiB per search. | Refused or cut short |
| M9 | Diagnostics (log output) corrupt the stdio protocol, whose messages travel on standard output | Servers log to standard error (stderr) only. | |
| M10 | The server serves the wrong directory | The root comes only from `--root` or the `CLAUDE_PROJECT_DIR` environment variable, never from the working directory. | Refuses to start |
| M11 | A new version forwards more of the user's environment | Each variable is its own consent grant (`env:<NAME>`, F14). Every client entry names only the variables listed in `[mcp-server] env`. Codex and Copilot CLI start servers with almost nothing else from the environment. Gemini CLI hides variables whose names look secret unless they are listed. Whether Cursor passes its whole environment to a server is undocumented (`*-P4`, the Phase 4 client facts in [`../specs.md`](../specs.md)). | Stops until accepted |
| M12 | A new server skips the hardening | Validation requires the `agent-grimoire-mcp` dependency (`crate/hardening`), refuses direct use of rmcp's transports in `src/` (`crate/raw-transport`), and requires a test that calls `declared_tools_match` (`crate/declared-tools-test`). That test compares the tools the running server lists with their declared annotations (hints such as whether a tool is read-only). The source checks are text searches that a determined author can get around; review is the guarantee. | CI fails |

M7, M8 and M10 describe `project-docs`, the one server in the catalog today; [`assets.md`, "`project-docs`"](assets.md#project-docs) has its rules. The rest hold for every server that uses this crate.

What ADR-0007 left unverified, because it needs a real authorization server (`CC-P3`): that Claude Code completes its OAuth flow against a server end to end, that it sends no `Origin` header, and whether it sends the RFC 8707 `resource` parameter. The MCP conformance suite hasn't been run against `project-docs`, because it expects fixture tools.

## What catalog validation requires of a server

`cargo xtask validate` checks three things that tie every MCP server asset to this crate ([`catalog.md`, "Catalog validation"](catalog.md#catalog-validation)):
- a `[dependencies]` entry on this crate (`crate/hardening`);
- no direct use of rmcp's stdio or Streamable HTTP transports in `src/` (`crate/raw-transport`);
- a test that calls `testing::declared_tools_match` (`crate/declared-tools-test`). It compares the tools the running server lists with those its `asset.toml` declares, and their annotations, each of which must be stated explicitly.

The last two are text searches that catch the usual mistakes; review is what guarantees them. A server depends on this crate as `agent-grimoire-mcp.workspace = true`, because an asset crate may not name a `path` or `git` source itself (`crate/path-dependency`).

Built in M3.1 as described here:

```rust
// `asset_toml` is the server's own `asset.toml`, as `include_str!("../asset.toml")`;
// `tools` comes from the running server's `tools/list`.
pub fn declared_tools_match(asset_toml: &str, tools: &[Tool]) -> Result<(), String>;
```

## Changing this crate changes every server

A change here changes every server binary while nothing in their asset directories changes, so `cargo xtask validate` doesn't see it. Review checks the rule instead: a change that alters what a server binary does, and every security fix, raises each affected asset's version and adds a CHANGELOG entry in the same pull request ([`../05-release.md`, "Tool and MCP server binaries (not checked)"](../05-release.md#tool-and-mcp-server-binaries-not-checked)).

Each rmcp bump also revisits three `deny.toml` entries that ADR-0007 added for rmcp: the SHA-256 pins of its test fixtures, the `syn` skip, and the ignored chrono advisory ([`../02-dependency-policy.md`](../02-dependency-policy.md#the-rule-for-third-party-code)).

## Invariants

1. Without authorization configured, a server binds only to a loopback address, and refuses any other address before it opens a socket.
2. `Host` and `Origin` are checked before the path, the method, authorization and the protocol version. A present `Origin` passes only when it is on the allowlist, which is empty by default, and `null` never passes.
3. Only a `POST` to the MCP path reaches rmcp; with authorization, `GET` and `HEAD` at the well-known paths get the RFC 9728 metadata.
4. A token is accepted only when every check in [Authorization](#authorization) holds: `typ`, the algorithm bound to its key, the key size, `iss` as the exact string, `aud`, `exp` and `nbf` within the leeway, and every required scope. Keys come only from the JWKS file.
5. Every refusal carries the RFC 6750 challenge for its case, naming the metadata's URL.
6. A changed key file replaces the keys, an emptied or deleted one revokes them, and only an unreadable or unparsable one keeps the previous keys.
7. No tool handler ever sees an `Authorization` header, with or without authorization configured; a handler with a valid token sees its `Caller`.
8. Only the protocol versions 2026-07-28 and 2025-11-25 are served, and both are served statelessly, with no `Mcp-Session-Id`.
9. A body over 1 MiB gets 413, and a connection closes after the connection deadline even when the client stops reading.
10. Tool work beyond the rate, the concurrency or the time limit is refused with its `Refused` value; a timed-out call keeps its slot until its work ends.
11. A server's live tools and their annotations match its `asset.toml` exactly.
12. Over stdio, nothing but MCP messages reaches stdout.
13. Every MCP server asset depends on this crate, uses no rmcp transport directly, and has a test that calls `declared_tools_match`.

## Tests

The integration tests are in `crates/agent-grimoire-mcp/tests/`: `http.rs` runs real servers on real sockets, and `auth.rs` checks tokens signed at test time with the RFC 7515 example keys (RFC 7515 defines JSON Web Signature), from `tests/support/` and `tests/fixtures/`.

| Invariant | Tests |
|---|---|
| 1 | `without_authorization_only_loopback_is_allowed` (`tests/http.rs`); `http_needs_authorization_or_an_explicit_loopback_no_auth` in `catalog/mcp-servers/project-docs/tests/protocol.rs` |
| 2 | `host_and_origin_are_checked_first`, `origin_is_checked_before_authorization` (`tests/http.rs`); `origins_normalize` (`src/http.rs`); `endpoints` (`src/url.rs`) |
| 3 | `only_post_to_the_endpoint_is_served` (`tests/http.rs`) |
| 4 | `valid_es256_and_rs256_tokens_are_accepted`, `claims_are_checked`, `headers_and_keys_are_checked`, `jwt_typ_can_be_allowed_explicitly`, `scopes_and_requests_are_checked`, `iss_must_be_the_issuer_string_not_a_list_containing_it`, `extra_audiences_are_accepted_besides_the_resource`, `unusable_keys_and_bad_settings_are_refused_at_startup` (`tests/auth.rs`); `bearer_parsing`, `scopes_from_scope_or_scp` (`src/auth.rs`) |
| 5 | `challenges_follow_rfc_6750_and_name_the_metadata`, `metadata_is_rfc_9728` (`tests/auth.rs`); `authorization_challenges_then_accepts_a_valid_token` (`tests/http.rs`); `quoted_values_drop_quotes_and_backslashes` (`src/auth.rs`) |
| 6 | `a_changed_key_file_is_reloaded_and_a_broken_one_ignored`, `emptying_or_deleting_the_key_file_revokes_the_keys` (`tests/auth.rs`) |
| 7 | `handlers_get_the_caller_but_never_the_token` (`tests/http.rs`), added with the 2026-11-22 fix |
| 8 | `only_the_supported_protocol_versions_are_served`, `discovery_lists_both_protocol_versions_and_legacy_clients_stay_stateless`, `a_2025_11_25_client_lists_and_calls_tools_statelessly` (`tests/http.rs`) |
| 9 | `oversized_and_malformed_requests_are_refused`, `a_client_that_stops_reading_cannot_hold_a_connection_slot` (`tests/http.rs`) |
| 10 | `the_rate_limit_and_the_timeout_apply`, `busy_slots_refuse_more_work`, `truncation_counts_characters` (`src/limits.rs`) |
| 11 | `an_exact_match_passes`, `every_kind_of_difference_is_reported` (`src/testing.rs`); `the_served_tools_match_asset_toml_under_both_lifecycles` in `catalog/mcp-servers/project-docs/tests/protocol.rs` |
| 12 | `the_root_comes_from_the_flag_or_claude_project_dir_and_is_never_guessed` in `catalog/mcp-servers/project-docs/tests/protocol.rs`, which checks that a server that refuses to start writes nothing to stdout; the other `protocol.rs` tests read every stdout line as an MCP message |
| 13 | `servers_use_the_hardening_crate_and_no_asset_uses_core_crates`, `servers_serve_through_the_hardening_crate_and_test_their_declared_tools` in `crates/agent-grimoire-catalog/tests/catalog.rs` |

The crate docs' examples run as doc tests: one shows the tool limits and the loopback refusal without opening a socket, and one shows a `declared_tools_match` failure.
