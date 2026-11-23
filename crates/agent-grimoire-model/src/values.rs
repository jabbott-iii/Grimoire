//! Constrained identifiers and text fields shared by manifests and configs.

use crate::constrained::{check_kebab, check_text, constrained_string};

/// Reserved prefixes.
///
/// - `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-`: Claude Code reserves them
///   for plugin names, and asset and bundle IDs become plugin names in the generated
///   marketplace ([`docs/specs.md`, CC-PLUGIN](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cc-plugin-plugins)).
/// - `grimoire-`: kept for content Grimoire itself owns in a project.
const RESERVED_ID_PREFIXES: [&str; 5] = [
    "claude-",
    "anthropic-",
    "anthropics-",
    "cc-plugin-",
    "grimoire-",
];
/// Reserved names.
///
/// - `synced`: a reserved skill folder name in Claude Code
///   ([`docs/specs.md`, CC-SKILL](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cc-skill-skills)).
/// - `grimoire`: the owner of Grimoire's own managed blocks, e.g. the `CLAUDE.md`
///   import of `AGENTS.md`
///   ([`docs/architecture.md` §8.2, Client read matrix and cross-client planning](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#82-client-read-matrix-and-cross-client-planning)).
const RESERVED_IDS: [&str; 2] = ["synced", "grimoire"];

fn check_asset_id(value: &str) -> Result<(), String> {
    check_kebab(value, 64)?;
    if RESERVED_IDS.contains(&value) {
        return Err(format!("{value:?} is a reserved name"));
    }
    if let Some(prefix) = RESERVED_ID_PREFIXES.iter().find(|p| value.starts_with(**p)) {
        return Err(format!(
            "must not start with the reserved prefix {prefix:?}"
        ));
    }
    Ok(())
}

constrained_string! {
    /// An asset or bundle identifier, unique across the whole catalog.
    ///
    /// Follows the Agent Skills `name` rules for every kind: 1–64 characters of
    /// `[a-z0-9-]`, no leading, trailing or doubled `-`. Reserved names and
    /// prefixes are rejected.
    pub struct AssetId {
        what: "asset id",
        check: check_asset_id,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 64,
            "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$",
            "description": "Asset or bundle identifier: 1-64 characters of [a-z0-9-], no leading, trailing or doubled '-'. Must not start with claude-, anthropic-, anthropics-, cc-plugin- or grimoire-, and must not be 'synced' or 'grimoire'."
        },
    }
}

fn check_tag(value: &str) -> Result<(), String> {
    check_kebab(value, 32)
}

constrained_string! {
    /// A catalog tag: 1–32 characters of `[a-z0-9-]`.
    pub struct Tag {
        what: "tag",
        check: check_tag,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 32,
            "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$",
            "description": "Tag: 1-32 characters of [a-z0-9-]."
        },
    }
}

fn check_github_login(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 39 {
        return Err("must be 1-39 characters".into());
    }
    if let Some(c) = value
        .chars()
        .find(|c| !(c.is_ascii_alphanumeric() || *c == '-'))
    {
        return Err(format!(
            "may contain only ASCII letters, digits and '-' (found {c:?})"
        ));
    }
    if value.starts_with('-') || value.ends_with('-') {
        return Err("must not start or end with '-'".into());
    }
    Ok(())
}

constrained_string! {
    /// A GitHub login of a maintainer (no leading `@`).
    pub struct GithubLogin {
        what: "GitHub login",
        check: check_github_login,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 39,
            "pattern": "^[A-Za-z0-9]([A-Za-z0-9-]*[A-Za-z0-9])?$",
            "description": "GitHub login without a leading '@'."
        },
    }
}

fn check_env_var(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 128 {
        return Err("must be 1-128 characters".into());
    }
    let mut chars = value.chars();
    let first = chars.next().unwrap_or('0');
    if !(first.is_ascii_uppercase() || first == '_') {
        return Err("must start with an uppercase ASCII letter or '_'".into());
    }
    if let Some(c) = chars.find(|c| !(c.is_ascii_uppercase() || c.is_ascii_digit() || *c == '_')) {
        return Err(format!(
            "may contain only uppercase ASCII letters, digits and '_' (found {c:?})"
        ));
    }
    Ok(())
}

constrained_string! {
    /// The *name* of an environment variable. Grimoire never stores values.
    pub struct EnvVarName {
        what: "environment variable name",
        check: check_env_var,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 128,
            "pattern": "^[A-Z_][A-Z0-9_]*$",
            "description": "Environment variable name (never a value)."
        },
    }
}

fn check_summary(value: &str) -> Result<(), String> {
    check_text(value, 120, true)
}

constrained_string! {
    /// A one-line summary of at most 120 characters.
    pub struct Summary {
        what: "summary",
        check: check_summary,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 120,
            "pattern": "^\\S(.*\\S)?$",
            "description": "One line, at most 120 characters, no leading or trailing whitespace."
        },
    }
}

fn check_description(value: &str) -> Result<(), String> {
    check_text(value, 4000, false)
}

constrained_string! {
    /// A longer Markdown description of at most 4000 characters.
    pub struct Description {
        what: "description",
        check: check_description,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 4000,
            "description": "Markdown, at most 4000 characters."
        },
    }
}

fn check_reason(value: &str) -> Result<(), String> {
    check_text(value, 500, false)
}

constrained_string! {
    /// A short human-readable reason (deprecations, conflicts).
    pub struct Reason {
        what: "reason",
        check: check_reason,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 500,
            "description": "Short explanation, at most 500 characters."
        },
    }
}

fn check_glob(value: &str) -> Result<(), String> {
    if value.is_empty() || value.chars().count() > 200 {
        return Err("must be 1-200 characters".into());
    }
    if let Some(c) = value
        .chars()
        .find(|c| c.is_control() || matches!(c, '\\' | ':'))
    {
        return Err(format!("must not contain {c:?}"));
    }
    if value.starts_with('/') {
        return Err("must be relative to the project root (no leading '/')".into());
    }
    if value.split('/').any(|part| part == "..") {
        return Err("must not contain '..' components".into());
    }
    Ok(())
}

constrained_string! {
    /// A glob relative to the project root, e.g. `**/*.rs`.
    pub struct Glob {
        what: "glob",
        check: check_glob,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 200,
            "description": "Glob relative to the project root using '/' separators, e.g. \"**/*.rs\". No '..', '\\\\' or ':'."
        },
    }
}

fn check_mcp_tool_name(value: &str) -> Result<(), String> {
    if value.is_empty() || value.len() > 128 {
        return Err("must be 1-128 characters".into());
    }
    if let Some(c) = value
        .chars()
        .find(|c| !(c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-')))
    {
        return Err(format!(
            "may contain only ASCII letters, digits, '_', '.' and '-' (found {c:?})"
        ));
    }
    Ok(())
}

constrained_string! {
    /// An MCP tool name: 1–128 characters of `[A-Za-z0-9_.-]`, as the Model
    /// Context Protocol specification recommends
    /// ([`docs/specs.md`, MCP-SPEC](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#mcp-spec-protocol-specification)).
    pub struct McpToolName {
        what: "MCP tool name",
        check: check_mcp_tool_name,
        schema: {
            "type": "string",
            "pattern": "^[A-Za-z0-9_.-]{1,128}$",
            "description": "MCP tool name: 1-128 characters of [A-Za-z0-9_.-]."
        },
    }
}

fn check_binary_name(value: &str) -> Result<(), String> {
    let rest = value
        .strip_prefix("grimoire-mcp-")
        .or_else(|| value.strip_prefix("grimoire-tool-"))
        .ok_or("must start with \"grimoire-mcp-\" or \"grimoire-tool-\"")?;
    check_kebab(rest, 64)
}

constrained_string! {
    /// The executable name of a tool or MCP server: `grimoire-tool-<id>` or
    /// `grimoire-mcp-<id>`. Each is its own binary, released with the CLI
    /// ([ADR-0002](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0002-mcp-server-and-tool-packaging.md)).
    pub struct BinaryName {
        what: "binary name",
        check: check_binary_name,
        schema: {
            "type": "string",
            "pattern": "^grimoire-(mcp|tool)-[a-z0-9]+(-[a-z0-9]+)*$",
            "description": "Executable name: grimoire-mcp-<id> or grimoire-tool-<id>."
        },
    }
}

/// The longest URL a client entry may hold.
const MAX_URL: usize = 2048;

/// Whether `host` (as written in a URL authority) names this machine.
fn is_loopback_host(host: &str) -> bool {
    host.eq_ignore_ascii_case("localhost")
        || host == "[::1]"
        || host
            .strip_prefix("127.")
            .is_some_and(|rest| check_ipv4_tail(rest, 3))
}

/// Whether `rest` is `count` dot-separated decimal octets.
fn check_ipv4_tail(rest: &str, count: usize) -> bool {
    let parts: Vec<&str> = rest.split('.').collect();
    parts.len() == count
        && parts.iter().all(|p| {
            !p.is_empty()
                && p.len() <= 3
                && p.bytes().all(|b| b.is_ascii_digit())
                && p.parse::<u16>().is_ok_and(|n| n <= 255)
        })
}

fn check_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("needs a host".into());
    }
    if let Some(inner) = host.strip_prefix('[') {
        let inner = inner
            .strip_suffix(']')
            .ok_or("an IPv6 host must be enclosed in [ ]")?;
        return if inner.parse::<std::net::Ipv6Addr>().is_ok() {
            Ok(())
        } else {
            Err(format!("`[{inner}]` is not an IPv6 address"))
        };
    }
    if host.bytes().all(|b| b.is_ascii_digit() || b == b'.') {
        return if check_ipv4_tail(host, 4) {
            Ok(())
        } else {
            Err(format!("`{host}` is not an IPv4 address"))
        };
    }
    // URL parsers read a name whose last label is all digits as an IPv4 address.
    if host
        .rsplit('.')
        .next()
        .is_some_and(|last| !last.is_empty() && last.bytes().all(|b| b.is_ascii_digit()))
    {
        return Err(format!(
            "`{host}` is neither a host name nor an IPv4 address"
        ));
    }
    for label in host.split('.') {
        let ok = !label.is_empty()
            && label.len() <= 63
            && label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            && !label.starts_with('-')
            && !label.ends_with('-');
        if !ok {
            return Err(format!("`{host}` is not a valid host name"));
        }
    }
    Ok(())
}

/// The characters allowed in a URL path: RFC 3986 unreserved characters,
/// `/`, and `%` escapes. Anything that could expand (`$`, `{`) or start a
/// query or fragment is refused.
fn check_url_path(path: &str) -> Result<(), String> {
    let bytes = path.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'%' {
            let hex = bytes.get(i + 1..i + 3);
            if !hex.is_some_and(|h| h.iter().all(u8::is_ascii_hexdigit)) {
                return Err("a `%` must start a two-digit hex escape".into());
            }
            i += 3;
            continue;
        }
        if !(b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'.' | b'_' | b'~')) {
            return Err(format!(
                "the path may contain only letters, digits, `/ - . _ ~` and %-escapes (found {:?})",
                char::from(b)
            ));
        }
        i += 1;
    }
    Ok(())
}

fn check_server_url(value: &str) -> Result<(), String> {
    if value.len() > MAX_URL {
        return Err(format!("must be at most {MAX_URL} characters"));
    }
    let (secure, rest) = if let Some(rest) = value.strip_prefix("https://") {
        (true, rest)
    } else if let Some(rest) = value.strip_prefix("http://") {
        (false, rest)
    } else {
        return Err("must start with https:// or http://".into());
    };
    if rest.contains(['?', '#']) {
        return Err("must not have a query or fragment: MCP access tokens never go in URLs".into());
    }
    let (authority, path) = rest.find('/').map_or((rest, ""), |i| rest.split_at(i));
    if authority.contains('@') {
        return Err(
            "must not contain user information (`user@`): credentials never go in URLs".into(),
        );
    }
    // Split a trailing `:port`, minding the colons inside `[IPv6]`.
    let (host, port) = match authority.rfind(':') {
        Some(i) if !authority[i..].contains(']') => (&authority[..i], Some(&authority[i + 1..])),
        _ => (authority, None),
    };
    check_host(host)?;
    if let Some(port) = port
        && !(!port.is_empty()
            && port.len() <= 5
            && port.bytes().all(|b| b.is_ascii_digit())
            && port.parse::<u16>().is_ok_and(|p| p > 0))
    {
        return Err(format!("`{port}` is not a port number (1-65535)"));
    }
    check_url_path(path)?;
    if !secure && !is_loopback_host(host) {
        return Err(
            "plain http:// is allowed only for this machine (localhost, 127.0.0.1, [::1]); use https://"
                .into(),
        );
    }
    Ok(())
}

constrained_string! {
    /// The URL of a Streamable HTTP MCP server, as written into a client's MCP
    /// configuration.
    ///
    /// `https://`, or `http://` for this machine only (`localhost`, `127.0.0.0/8`,
    /// `[::1]`). No user information, query or fragment, the parts of a URL where
    /// a credential usually goes, and no characters a client would expand (`$`,
    /// `{`). ASCII, at most 2048 characters.
    pub struct ServerUrl {
        what: "server URL",
        check: check_server_url,
        schema: {
            "type": "string",
            "maxLength": 2048,
            "pattern": "^https?://[A-Za-z0-9.:\\[\\]-]+(/[A-Za-z0-9/._~%-]*)?$",
            "description": "Streamable HTTP endpoint: https://, or http:// only for localhost, 127.0.0.0/8 or [::1]. No user information, query or fragment."
        },
    }
}

impl ServerUrl {
    /// Whether the URL names this machine.
    #[must_use]
    pub fn is_loopback(&self) -> bool {
        let rest = self
            .0
            .strip_prefix("https://")
            .or_else(|| self.0.strip_prefix("http://"))
            .unwrap_or_default();
        let authority = rest.split('/').next().unwrap_or_default();
        let host = match authority.rfind(':') {
            Some(i) if !authority[i..].contains(']') => &authority[..i],
            _ => authority,
        };
        is_loopback_host(host)
    }
}

fn check_hook_arg(value: &str) -> Result<(), String> {
    if value.chars().count() > 1024 {
        return Err("must be at most 1024 characters".into());
    }
    if let Some(c) = value.chars().find(|c| c.is_control()) {
        return Err(format!("must not contain the control character {c:?}"));
    }
    Ok(())
}

constrained_string! {
    /// One argv element passed to a tool binary by a hook (exec form, never a shell string).
    pub struct HookArg {
        what: "hook argument",
        check: check_hook_arg,
        schema: {
            "type": "string",
            "maxLength": 1024,
            "description": "One argv element passed to the tool binary. Never interpreted by a shell."
        },
    }
}

fn check_entry_key(value: &str) -> Result<(), String> {
    if value.is_empty() || value.chars().count() > 512 {
        return Err("must be 1-512 characters".into());
    }
    if let Some(c) = value.chars().find(|c| c.is_control()) {
        return Err(format!("must not contain the control character {c:?}"));
    }
    Ok(())
}

constrained_string! {
    /// The key path of an owned entry inside a JSON or TOML config file.
    pub struct EntryKey {
        what: "entry key",
        check: check_entry_key,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 512,
            "description": "Key path of an owned entry inside a JSON or TOML config file."
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_ids() {
        assert!(AssetId::new("rust-errors").is_ok());
        assert!(AssetId::new("claude-helper").is_err());
        assert!(AssetId::new("anthropic-x").is_err());
        assert!(AssetId::new("cc-plugin-x").is_err());
        assert!(AssetId::new("synced").is_err());
        assert!(AssetId::new("grimoire").is_err());
        assert!(AssetId::new("grimoire-x").is_err());
        assert!(AssetId::new("my-grimoire").is_ok());
        assert!(AssetId::new("synced-notes").is_ok());
        // Only the reserved *prefix* is rejected.
        assert!(AssetId::new("my-claude-helper").is_ok());
        let err = AssetId::new("Bad_Id").unwrap_err();
        assert_eq!(err.what(), "asset id");
        assert!(err.to_string().contains("Bad_Id"));
    }

    #[test]
    fn github_logins() {
        assert!(GithubLogin::new("jabbott-iii").is_ok());
        assert!(GithubLogin::new("@someone").is_err());
        assert!(GithubLogin::new("-x").is_err());
        assert!(GithubLogin::new("a".repeat(40)).is_err());
    }

    #[test]
    fn env_vars() {
        assert!(EnvVarName::new("GITHUB_TOKEN").is_ok());
        assert!(EnvVarName::new("_X1").is_ok());
        assert!(EnvVarName::new("1X").is_err());
        assert!(EnvVarName::new("lower").is_err());
        assert!(EnvVarName::new("").is_err());
    }

    #[test]
    fn globs() {
        assert!(Glob::new("**/*.rs").is_ok());
        assert!(Glob::new("docs/**/*.md").is_ok());
        assert!(Glob::new("/abs/**").is_err());
        assert!(Glob::new("../x").is_err());
        assert!(Glob::new("a\\b").is_err());
        assert!(Glob::new("C:/x").is_err());
    }

    #[test]
    fn binary_names() {
        assert!(BinaryName::new("grimoire-mcp-adr-index").is_ok());
        assert!(BinaryName::new("grimoire-tool-secret-guard").is_ok());
        assert!(BinaryName::new("grimoire").is_err());
        assert!(BinaryName::new("grimoire-mcp-").is_err());
        assert!(BinaryName::new("other-mcp-x").is_err());
    }

    #[test]
    fn mcp_tool_names() {
        assert!(McpToolName::new("search_adrs").is_ok());
        assert!(McpToolName::new("a.b-c").is_ok());
        assert!(McpToolName::new("has space").is_err());
        assert!(McpToolName::new("a".repeat(129)).is_err());
    }

    #[test]
    fn server_urls() {
        for ok in [
            "http://127.0.0.1:8931/mcp",
            "http://localhost:8931/mcp",
            "http://LOCALHOST/mcp",
            "http://[::1]:8931/mcp",
            "http://127.1.2.3/",
            "https://docs.example.com/mcp",
            "https://docs.example.com",
            "https://10.0.0.7:443/a%2Fb/c-d_e~f.g",
            "https://[2001:db8::1]:8443/mcp",
        ] {
            assert!(ServerUrl::new(ok).is_ok(), "{ok}");
        }
        for bad in [
            "ftp://127.0.0.1/mcp",
            "HTTP://127.0.0.1/mcp",
            "http://docs.example.com/mcp",
            "http://10.0.0.7/mcp",
            "http://127.0.0.1.evil.example/mcp",
            "http://user:pass@127.0.0.1/mcp",
            "https://token@docs.example.com/mcp",
            "https://docs.example.com/mcp?token=abc",
            "https://docs.example.com/mcp#x",
            "https://docs.example.com/${TOKEN}",
            "https://docs.example.com/a b",
            "https://docs.example.com/%zz",
            "https://:8443/mcp",
            "https://docs.example.com:0/mcp",
            "https://docs.example.com:65536/mcp",
            "https://-bad.example.com/mcp",
            "https://256.1.1.1/mcp",
            "https://[::1/mcp",
            "https://[abc]/mcp",
            "https://[:]/mcp",
            "https://[1.2.3.4]/mcp",
            "https://[:::::]/mcp",
            "https://docs.123/mcp",
            "https://docs.example.com/\u{e9}",
        ] {
            assert!(ServerUrl::new(bad).is_err(), "{bad}");
        }
        assert!(
            ServerUrl::new("http://127.0.0.1:1/mcp")
                .unwrap()
                .is_loopback()
        );
        assert!(ServerUrl::new("http://[::1]:1/mcp").unwrap().is_loopback());
        assert!(
            !ServerUrl::new("https://docs.example.com/mcp")
                .unwrap()
                .is_loopback()
        );
        assert!(ServerUrl::new(format!("https://a.example/{}", "x".repeat(2048))).is_err());
    }
}
