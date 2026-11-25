# ADR-0007: Server-side MCP authorization and HTTP hardening

- **Status:** Accepted (the maintainer chose option (a) when asked, 2026-11-16, during Phase 3)
- **Date:** 2026-11-16
- **Specs:** `MCP-P3`, `RMCP-P3`, `AUTH-CRATES`, `CC-P3` in [`../specs.md`](../specs.md), the register of verified external specs. An ID in parentheses below, such as (`CC-P3`), names the entry a fact comes from.

## Context

Grimoire's Model Context Protocol (MCP) servers serve stdio (standard input and output) and Streamable HTTP, MCP's HTTP transport (`subsystems/catalog.md`, "MCP server", which describes the MCP server asset kind).

Over HTTP, MCP 2026-07-28 (the protocol version Grimoire targets) makes authorization optional. But a server that offers it is an OAuth 2.1 resource server: in OAuth terms, the server that holds the protected data and accepts access tokens that a separate authorization server issues. Such a server must:
- serve RFC 9728 Protected Resource Metadata that names at least one authorization server (RFC 9728, one of the Internet's Request for Comments (RFC) standards, defines a JSON document that tells clients where to get a token);
- answer missing or bad tokens with RFC 6750 challenges (a `WWW-Authenticate` response header that says why a token was refused) that carry `resource_metadata`;
- accept only tokens issued for itself (audience), from the `Authorization` header only, and never pass a token on.

Over stdio, the spec says servers should not use OAuth and should take credentials from the environment.

rmcp 3.10.0 is the software development kit (SDK) Grimoire's servers use (ADR-0002, the architecture decision record, or ADR, that packages each server as its own Rust binary). It has none of the resource-server side. Its own auth module is an OAuth client, and its server examples skip audience, issuer and expiry checks. Its HTTP defaults are also unsafe for Grimoire:
- Origin validation (refusing a request whose `Origin` header names a site that isn't allowed, so a web page can't reach the server) is off;
- a request without `MCP-Protocol-Version` is treated as 2025-03-26;
- legacy sessions (server-side state tied to an `Mcp-Session-Id` header) are on;
- a server reports rmcp's own name and version.

Claude Code is the main HTTP client:
- It runs the OAuth flow itself, discovering the metadata at `/.well-known/oauth-protected-resource`.
- It requests the scopes (named permissions a token carries) that the server names in its challenge.
- A static `Authorization` header in `.mcp.json` turns that flow off.
- Its stable release still has an MCP client that speaks 2025-11-25 over HTTP.

The dependency cost of each option was measured with `cargo deny` against our policy (`AUTH-CRATES`). `cargo deny` checks every dependency's license, security advisories, duplicate versions and bundled files against `deny.toml`. The results:
- Every pure-Rust signature verifier pulls in `subtle` (BSD-3-Clause).
- Any HTTPS client adds ring, rustls-webpki, untrusted (ISC), webpki-roots (CDLA-Permissive-2.0), two duplicate versions and a build-script bypass (a `deny.toml` exception for a crate's build-time files).
- axum (a web framework) adds matchit (MIT AND BSD-3-Clause).
- Any rmcp server, with or without auth:
  - duplicates `syn` (pulls in a second version of it, via tracing-attributes);
  - ships JavaScript and Python test fixtures that `bans.build` (cargo-deny's check for scripts and prebuilt files in crates) rejects.

  jsonwebtoken needs the same `syn` 2, through `zeroize_derive`.

## Options

### (a) JWT access tokens verified against a local JWKS file (chosen)

The operator (whoever runs the server) supplies the authorization server's keys as a JWKS file: a JSON Web Key Set, the JSON list of keys that verify its signatures.

The server checks RFC 9068 access tokens against it and makes no outbound connection. RFC 9068 is the standard form for access tokens that are JWTs (JSON Web Tokens: signed JSON documents that state who a token is for and what it allows).
- **Dependencies:** jsonwebtoken 12 with no crypto backend, plus a small verify-only provider (code that checks signatures and can't make them) over rsa 0.10 and p256 0.14.
- **Policy cost:** one license exception (`subtle`).
- **Limits:** only JWT access tokens, and the operator replaces the file when the authorization server rotates (replaces) its keys.

### (b) JWT with keys fetched from the issuer's `jwks_uri`

The same checks, with key rotation handled automatically. It needs an HTTPS client:
- about 50 more crates, four more license exceptions, two duplicate skips and a ring build bypass;
- plus a network dependency;
- and webpki-roots (the bundled list of trusted certificate authorities), refreshed only by dependency bumps.

### (c) RFC 7662 token introspection

Token introspection means asking the authorization server whether each token is valid. It works with opaque tokens (tokens whose contents the server can't read). It needs the HTTPS client from (b), resource-server credentials, a cache, and a network round trip for every new token. It is the worst fit for a local-first server.

### (d) No OAuth in Phase 3

Only loopback HTTP (reachable only from the same machine) with Origin and Host checks, deferring authorization. Phase 3 is the phase of `roadmap.md` that built one example asset of each kind. This option is the cheapest, but it leaves the HTTP transport without the authorization the brief (the maintainer's original requirements) asks for.

## Decision

We will use (a), in the shared hardening crate `crates/agent-grimoire-mcp` (the library that holds every server's secure defaults and limits), and every MCP server asset must serve through that crate. Catalog validation checks this:
- the dependency on the crate (`crate/hardening`);
- no direct use of rmcp's transports in the server's `src/` (`crate/raw-transport`);
- a test that calls `declared_tools_match` (`crate/declared-tools-test`).

These are text checks that catch the usual mistakes; code review is still what guarantees them.

**Order of checks for each HTTP request.** The order is not specified by MCP; we fix it so that probes (requests that test what the server accepts) learn as little as possible:
1. `Host` must be one the server answers to (403). These are the bound address, `localhost`/`127.0.0.1`/`[::1]` on a loopback bind, the resource's host, and any `--allowed-host`.
2. `Origin`, when present, must exactly match the allowlist (403). The allowlist is empty by default, and `null` is never allowed. A request without `Origin` passes, as non-browser clients send none.
3. The path and method. The metadata is served on `GET` or `HEAD`. Only `POST` goes to the MCP path (405 with `Allow`, or 404).
4. Authorization, when configured (401, 403 or 400).
5. The protocol version, after authorization, so an unauthenticated body is never parsed. In the codes below, the first is the HTTP status and the second the JSON-RPC error code (MCP messages are JSON-RPC):
   - `MCP-Protocol-Version` must name 2025-11-25 or 2026-07-28 (400, -32022). rmcp itself checks the header only against every version it knows.
   - The header is required on everything but a legacy `initialize` (400, -32020).
   - A 2026-07-28 request must also carry the version in `_meta` (400, -32602).
   - These replace rmcp's `stateless_protocol_metadata_required`, which would also reject 2025-11-25 clients, who never send `_meta`.
6. rmcp, configured from the same Host and Origin lists:
   - stateless (`legacy_session_mode(false)`, `NeverSessionManager`), so no `Mcp-Session-Id`, even for 2025-11-25 clients;
   - `json_response(true)`, so replies are plain JSON rather than an event stream;
   - a 1 MiB body limit.

**Serving.** We will serve with hyper and hyper-util (lower-level HTTP libraries), not axum (rmcp doesn't need it, and it would add matchit's license, above). The server:
- one request per connection (no keep-alive, which reuses a connection, so no pipelining, which sends several requests before the first reply);
- at most 64 connections; 10 seconds to send headers and 60 seconds per request;
- every connection closed after 75 seconds whatever it is doing, so a client that stops reading cannot keep its slot;
- a graceful stop on `SIGINT` and `SIGTERM` (the interrupt and terminate signals).

Without authorization, a server binds only to a loopback address, and the binary needs an explicit `--no-auth` to do so.

**Metadata.** The RFC 9728 document is built from the configured canonical resource URI (the one URI that names this server), never from request headers:
- `resource`, `authorization_servers` (the issuer), `scopes_supported` (only the required scopes), `bearer_methods_supported: ["header"]` and `resource_name`;
- served at the path-inserted URL (`/.well-known/oauth-protected-resource/mcp`, the well-known path followed by the resource's path);
- also served at the root well-known URL, which Claude Code documents checking first:
  - That copy names `/mcp` as its resource, which RFC 9728 §3.3 does not allow for a root document. So a strict client rejects it and uses `resource_metadata` from the challenge instead.
  - Whether Claude Code needs the root copy is UNVERIFIED (`CC-P3`);
- `Cache-Control: max-age=3600`, so clients may cache it for an hour.

**Tokens** (RFC 9068 §4 and RFC 8725, the JWT best current practices). Credentials are read from exactly one `Authorization: Bearer` header; the query string is ignored. A token is accepted only if all of these hold:
- `typ` (the token's type) is `at+jwt` or `application/at+jwt`. `JWT` is accepted only with the explicit `--auth-allow-jwt-typ`.
- `alg` (the signature algorithm) is RS256 or ES256, bound to the key: an RSA key only for RS256, a P-256 key only for ES256, and a JWK's own `alg` must agree.
- RSA keys are 2048 to 8192 bits.
- The key is chosen by `kid` (key ID), or is the single key for the algorithm when there is no `kid`. Keys come only from the file, never from token headers.
- `iss` (issuer) equals the configured issuer exactly.
- `aud` (audience) contains the resource, or one of the extra audiences configured with `--auth-audience`. `aud`, `iss` and `exp` are required claims (fields of the token), because jsonwebtoken otherwise checks `aud` only when it is present.
- `iss` is a string. jsonwebtoken also accepts an array that merely contains the issuer, so the server checks this itself.
- `exp` (expiry) and `nbf` (not before) hold within a leeway (allowed clock difference) of at most 60 seconds.
- Every required scope is present, read from `scope` or `scp`.

The JWKS file is re-read when it changes, at most every 5 seconds.
- A well-formed key set replaces the keys, even with no usable key in it: removing keys revokes them.
- A deleted file revokes every key.
- Only a file that cannot be read or parsed, for example one being rewritten in place, keeps the previous keys.

Tokens are never logged, stored or forwarded.

**Challenges** (RFC 6750 §3):
- no credentials, or a scheme other than Bearer: 401 with `resource_metadata` and `scope`, and no error code;
- malformed or repeated credentials: 400 `invalid_request`;
- a token that was checked and failed: 401 `invalid_token`;
- missing scopes: 403 `insufficient_scope`.

**Not supported:** browser clients. There is no CORS (Cross-Origin Resource Sharing, the headers that let a web page call a server on another origin), and `OPTIONS` gets 405.

**Policy entries.** These are the `deny.toml` entries this needs. Each is scoped as tightly as cargo-deny allows and carries a reason that cites this ADR:

| Entry | What it allows |
|---|---|
| `licenses.exceptions` | `subtle` for `BSD-3-Clause`, the constant-time crate (code whose running time doesn't depend on secret values) under every pure-Rust verifier. |
| `bans.skip` | Exactly `syn@2.0.119`, for tracing-attributes 0.1.31 (via rmcp → tracing) and zeroize_derive 1.5.0 (via jsonwebtoken → zeroize). |
| `advisories.ignore` | Exactly RUSTSEC-2026-0543, "chrono is unmaintained" (added 2026-11-17, the maintainer's choice). rmcp depends on chrono directly, not behind a feature, in 3.10.0 and in 3.11.0, so no upgrade removes it. It is an unmaintained notice, not a vulnerability. |
| `bans.build.bypass` | rmcp 3.10.0's six JavaScript and Python test fixtures, each pinned by SHA-256. Its `build.rs` is not trusted as a whole (the maintainer's choice). It runs `git config core.hooksPath .githooks` when its grandparent directory has `.githooks` and `.git`, which can happen under `cargo vendor` (which copies every dependency's source into the project). Grimoire has no `.githooks` directory. |

## Consequences

**Positive:**
- No outbound network from any server.
- One extra license exception for authorization.
- Secure defaults inherited by every server through one crate.
- Every check is pinned by tests:
  - RS256 and ES256 tokens signed at test time with the RFC 7515 example keys (RFC 7515 defines JSON Web Signature);
  - rejections of a missing or wrong `aud`, a wrong `iss`, an expired token, `typ`, algorithm confusion (a token that names an algorithm its key isn't for), an unknown `kid`, short RSA keys and tampering;
  - the challenge formats, revocation by a changed or deleted key file, and an `iss` array;
  - over a real socket: the order of Host, Origin and authorization, the version refusals, a 2025-11-25 client's full flow, and a client that sends its body too slowly.

**Negative:**
- Operators rotate keys by replacing the JWKS file.
- Opaque tokens and authorization servers that do not issue JWT access tokens are not supported.
- A `JWT`-typed token needs the explicit opt-out. *(Corrected 2026-11-20: this said "a typ-less or `JWT`-typed token". A token with no `typ` is always refused, as the decision above and `crates/agent-grimoire-mcp/src/auth.rs` do.)*
- Browser-based MCP clients cannot connect.

**Follow-up and review:**
- Option (b) is the follow-up if users need automatic key rotation. It needs the deny.toml entries listed under (b).
- **rmcp bumps** (version updates).
  - Each one re-pins the fixture checksums.
  - The `syn` skip goes once both tracing-attributes and zeroize_derive move to syn 3; `cargo deny` then reports the entry as unused.
  - The chrono advisory ignore goes once rmcp drops chrono, which `cargo deny` also reports.

**Not verified:**
- That Claude Code completes its OAuth flow against this server end to end.
- That it sends no `Origin` header.
- Whether it sends the RFC 8707 `resource` parameter.

These need a real authorization server and are UNVERIFIED in `CC-P3`. The MCP conformance suite was not run against the docs server (`grimoire-mcp-project-docs`, the catalog's read-only MCP server for a project's docs), because it expects fixture tools.
