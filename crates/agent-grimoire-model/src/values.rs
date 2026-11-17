//! Constrained identifiers and text fields shared by manifests and configs.

use crate::constrained::{check_kebab, check_text, constrained_string};

/// Prefixes Claude Code reserves for plugin names. Asset and bundle IDs become
/// plugin names in the generated marketplace (`CC-PLUGIN` in `docs/specs.md`).
const RESERVED_ID_PREFIXES: [&str; 4] = ["claude-", "anthropic-", "anthropics-", "cc-plugin-"];
/// Names Claude Code reserves outright: `synced` is a reserved skill folder name (`CC-SKILL`).
const RESERVED_IDS: [&str; 1] = ["synced"];

fn check_asset_id(value: &str) -> Result<(), String> {
    check_kebab(value, 64)?;
    if RESERVED_IDS.contains(&value) {
        return Err(format!("{value:?} is reserved by Claude Code"));
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
    /// `[a-z0-9-]`, no leading, trailing or doubled `-`. Claude Code's reserved
    /// plugin prefixes are rejected.
    pub struct AssetId {
        what: "asset id",
        check: check_asset_id,
        schema: {
            "type": "string",
            "minLength": 1,
            "maxLength": 64,
            "pattern": "^[a-z0-9]+(-[a-z0-9]+)*$",
            "description": "Asset or bundle identifier: 1-64 characters of [a-z0-9-], no leading, trailing or doubled '-'. Must not start with claude-, anthropic-, anthropics- or cc-plugin-, and must not be 'synced'."
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
    /// An MCP tool name: 1–128 characters of `[A-Za-z0-9_.-]` (`MCP-SPEC`).
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
    /// The executable name of a tool or MCP server: `grimoire-tool-<id>` or `grimoire-mcp-<id>` (ADR-0002).
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
}
