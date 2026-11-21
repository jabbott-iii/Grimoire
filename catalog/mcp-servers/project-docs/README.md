# project-docs

`grimoire-mcp-project-docs` is a read-only MCP server for a project's documentation and architecture decision records (ADRs). MCP (Model Context Protocol) lets an AI coding tool, the client, call tools that a separate server program offers.

## Tools

All four are read-only, idempotent (calling one again with the same arguments has no further effect) and closed-world (they reach nothing outside the project). They say so in their annotations, the hints an MCP server gives clients about each tool.

| Tool | What it does |
|---|---|
| `list_docs` | Lists documentation files (`.md`, `.markdown`, `.mdx`, `.rst`, `.adoc`) with their titles, 100 per page by default. |
| `read_doc` | Reads one document, up to 30,000 characters per call; `next_line` says where to continue. A single line longer than that is returned cut, and `cut_line` names it; the rest of that line cannot be read through this tool. |
| `search_docs` | Finds lines containing some literal text (not a pattern), case-insensitive by default, up to 100 matches. |
| `list_adrs` | Lists ADRs (`NNNN-title.md` in a directory named `adr`, `adrs` or `decisions`) with number, title, status and date. |

## What it can read

- Only files under the project root. The server opens the root directory once and resolves every path relative to that open handle (a capability handle), never by a full path name. So nothing outside the root can be reached, even if the tree changes during a request.
- Only documentation files, by extension. Source code and configuration are not served.
- Never hidden files or directories (`.env`, `.git`, `.github`), never symbolic links, and never `node_modules`, `target`, `vendor`, `venv` or `__pycache__`.
- Documents up to 1 MiB. A listing or search considers up to 5,000 documents and reads at most 64 MiB.

A path refused for its form, such as `../x.md`, `.env` or `src/main.rs`, is told why; the reason depends only on the string sent.

A well-formed path the server will not serve, such as a symbolic link or a directory, gets the same "not a document in this project" answer as a missing file. So the answer reveals nothing about what exists.

## Running it

Grimoire never installs binaries. Until release builds exist, build it from this repository:

```bash
cargo install --locked --path catalog/mcp-servers/project-docs
```

**stdio (what Grimoire configures).** The client starts the server as a child process and exchanges messages over its standard input and output (stdin and stdout).
- With no command, the server speaks MCP on stdin and stdout. It takes the project root from `--root`, or else from `CLAUDE_PROJECT_DIR`, which Claude Code sets for the servers it starts.
- It never guesses a root from its working directory.
- It stops when the client closes stdin, or on `SIGINT` or `SIGTERM` (the signals that Ctrl+C and, by default, `kill` send).

**Streamable HTTP** (MCP's HTTP transport). Start the server yourself, then point the client at it with `transport = "streamable-http"` and a `url` in `grimoire.toml`, the project's Grimoire configuration:

```bash
grimoire-mcp-project-docs --root . serve-http --no-auth --listen 127.0.0.1:8931
```

- `--no-auth` is allowed only on a loopback address (such as `127.0.0.1`, which only this machine can reach). Even there any local process can connect: prefer stdio.
- With OAuth (the standard for granting access with tokens), the server is a resource server: it accepts access tokens that a separate authorization server issues. It serves RFC 9728 metadata, a document that tells clients which authorization server to get a token from. It checks RFC 9068 access tokens (JSON Web Tokens) against a JWKS file you supply (a JSON Web Key Set: the authorization server's public keys), with no outbound network:

  ```bash
  grimoire-mcp-project-docs --root . serve-http --listen 127.0.0.1:8931 \
    --auth-resource https://docs.example.com/mcp \
    --auth-issuer https://auth.example.com \
    --auth-jwks /etc/project-docs/jwks.json --auth-scope docs:read
  ```

  Put a TLS-terminating proxy in front: a proxy that handles the TLS (Transport Layer Security) encryption of HTTPS for the server and passes the requests on to it. Add `--allowed-host` if the proxy forwards a different `Host` (the header that names the host a request was sent to). The JWKS file is re-read when it changes. Run `grimoire-mcp-project-docs serve-http --help` for every option.
- No browser origin may call the server unless you list it with `--allowed-origin`. An origin is the scheme, host and port of a web page, such as `https://example.com`, and browsers send it in the `Origin` header.

It speaks MCP 2026-07-28, and 2025-11-25 for clients that still use the earlier handshake, over both stdio and HTTP. MCP versions are named by date.

Over HTTP each connection carries one request and is closed after at most 75 seconds, so a client that stops reading can't keep its connection slot.

## Security notes

- **Prompt injection.** Documents are untrusted text that reaches the model, and a document can contain text that tries to give the model instructions. The server's instructions tell the client to treat document content as information, not as instructions, but a hostile document can still try to steer an agent. Review what the agent does with what it reads.
- **No network from the server.** It makes no outbound connections, including in OAuth mode.
- **Limits.** At most 8 tool calls run at once, and about 20 per second. After 10 seconds a call returns an error; the work behind it runs on until it ends and keeps its slot until then.
- **Other clients.** GitHub Copilot also starts servers from `.mcp.json`, and may not set `CLAUDE_PROJECT_DIR`. The server then refuses to start rather than serve an unexpected directory.

## Permissions

- `filesystem = "read"`: it reads documentation files under the project root.
- `env = ["CLAUDE_PROJECT_DIR"]`: it reads the project root from this variable, which Claude Code sets. The client configuration that Grimoire writes forwards no variables from your environment.
- No `network`: the configuration Grimoire writes has the client start the server over stdio. Serving HTTP is something you start yourself.
