# @@ID@@

What `@@BINARY@@`'s tools do, what they can reach, and when an agent should use them.

## Running it

Grimoire never installs binaries. Until release builds exist, build it from this repository:

```bash
cargo install --locked --path catalog/mcp-servers/@@ID@@
```

With no command, the server speaks MCP over stdio. `serve-http --no-auth` serves Streamable HTTP on a loopback address.

## Security notes

What the server can reach, what its tools can change, and why it needs each permission it declares.
