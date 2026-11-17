//! Small enums and scalar wrappers shared by several documents.

use std::borrow::Cow;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};

use crate::InvalidValue;
use crate::constrained::constrained_string;

/// The document format version. Every Grimoire file starts with `format = 1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u32", into = "u32")]
pub struct Format(u32);

impl Format {
    /// The only format this version of Grimoire reads and writes.
    pub const CURRENT: Self = Self(1);

    /// The numeric version.
    #[must_use]
    pub fn get(self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for Format {
    type Error = InvalidValue;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == Self::CURRENT.0 {
            Ok(Self(value))
        } else {
            Err(InvalidValue::new(
                "format version",
                &value.to_string(),
                "this version of Grimoire supports only format 1",
            ))
        }
    }
}

impl From<Format> for u32 {
    fn from(value: Format) -> Self {
        value.0
    }
}

impl JsonSchema for Format {
    fn schema_name() -> Cow<'static, str> {
        "Format".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "integer",
            "const": 1,
            "description": "Document format version. Grimoire currently reads and writes format 1."
        })
    }
}

/// An asset kind (`architecture.md` §3).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// Instructions loaded into context automatically, always or for matching paths.
    Rule,
    /// An Agent Skills directory, loaded on demand.
    Skill,
    /// A subagent definition with an explicit tool allow-list.
    Agent,
    /// A standalone Rust executable, optionally bound to client hooks.
    Tool,
    /// A Rust MCP server.
    McpServer,
}

impl Kind {
    /// Every kind, in catalog order.
    pub const ALL: [Kind; 5] = [
        Kind::Rule,
        Kind::Skill,
        Kind::Agent,
        Kind::Tool,
        Kind::McpServer,
    ];

    /// The value used in `asset.toml` (`kind = "…"`).
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Rule => "rule",
            Kind::Skill => "skill",
            Kind::Agent => "agent",
            Kind::Tool => "tool",
            Kind::McpServer => "mcp-server",
        }
    }

    /// The directory under `catalog/` that holds assets of this kind.
    #[must_use]
    pub fn catalog_dir(self) -> &'static str {
        match self {
            Kind::Rule => "rules",
            Kind::Skill => "skills",
            Kind::Agent => "agents",
            Kind::Tool => "tools",
            Kind::McpServer => "mcp-servers",
        }
    }

    /// The kind stored in a `catalog/<dir>` directory, if any.
    #[must_use]
    pub fn from_catalog_dir(dir: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.catalog_dir() == dir)
    }

    /// Whether assets of this kind are Rust crates (ADR-0002).
    #[must_use]
    pub fn is_code_bearing(self) -> bool {
        matches!(self, Kind::Tool | Kind::McpServer)
    }
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Maturity of an asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum Maturity {
    /// May change incompatibly in any release.
    Experimental,
    /// Follows `SemVer` strictly.
    Stable,
    /// Kept for existing users; a replacement is named in `deprecated`.
    Deprecated,
}

/// An MCP transport (`MCP-SPEC`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum Transport {
    /// Newline-delimited JSON-RPC over stdin/stdout.
    Stdio,
    /// The Streamable HTTP transport.
    StreamableHttp,
}

fn check_sha256(value: &str) -> Result<(), String> {
    let hex = value
        .strip_prefix("sha256:")
        .ok_or("must start with \"sha256:\"")?;
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("must be \"sha256:\" followed by 64 lowercase hex digits".into());
    }
    Ok(())
}

constrained_string! {
    /// A SHA-256 content digest, written `sha256:<64 lowercase hex digits>`.
    pub struct Sha256Digest {
        what: "digest",
        check: check_sha256,
        schema: {
            "type": "string",
            "pattern": "^sha256:[0-9a-f]{64}$",
            "description": "SHA-256 digest: \"sha256:\" followed by 64 lowercase hex digits."
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_round_trip_through_catalog_dirs() {
        for kind in Kind::ALL {
            assert_eq!(Kind::from_catalog_dir(kind.catalog_dir()), Some(kind));
        }
        assert_eq!(Kind::from_catalog_dir("bundles"), None);
    }

    #[test]
    fn format_accepts_only_current() {
        assert!(Format::try_from(1).is_ok());
        assert!(Format::try_from(2).is_err());
        assert!(Format::try_from(0).is_err());
    }

    #[test]
    fn digests() {
        let ok = format!("sha256:{}", "a".repeat(64));
        assert!(Sha256Digest::new(ok).is_ok());
        assert!(Sha256Digest::new(format!("sha256:{}", "A".repeat(64))).is_err());
        assert!(Sha256Digest::new(format!("sha1:{}", "a".repeat(40))).is_err());
        assert!(Sha256Digest::new("sha256:abc").is_err());
    }
}
