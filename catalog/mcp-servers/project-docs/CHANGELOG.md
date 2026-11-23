# Changelog

## 0.1.1

- Security: over Streamable HTTP, the `Authorization` header is now removed from each request once its token is checked, so the server's tool handlers never receive the bearer token. A request without authorization configured loses the header too. The tools never read it, so they behave as before.

## 0.1.0

- First release: `list_docs`, `read_doc`, `search_docs` and `list_adrs` over stdio and Streamable HTTP, with an optional OAuth resource-server mode.
