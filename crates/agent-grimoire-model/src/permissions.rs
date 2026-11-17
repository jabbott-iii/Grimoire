//! Declared permissions, consent grants and agent capabilities (`architecture.md` §3.3, §6.3).

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};

use crate::{AssetId, EnvVarName, InvalidValue, McpToolName};

/// Filesystem access an asset can cause.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum FsAccess {
    /// No filesystem access.
    #[default]
    None,
    /// Reads files.
    Read,
    /// Reads and writes files.
    Write,
}

/// What an asset can cause at runtime. Everything defaults to none.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Permissions {
    /// Filesystem access.
    #[serde(default)]
    pub filesystem: FsAccess,
    /// Network access.
    #[serde(default)]
    pub network: bool,
    /// Runs shell commands or other programs.
    #[serde(default)]
    pub shell: bool,
    /// Environment variables the asset reads, by name.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVarName>,
    /// Registers client hooks that run automatically.
    #[serde(default)]
    pub hooks: bool,
}

impl Permissions {
    /// Whether nothing is declared.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        *self == Self::default()
    }

    /// The grants a user must accept before this asset is installed.
    #[must_use]
    pub fn required(&self) -> BTreeSet<Permission> {
        let mut out = BTreeSet::new();
        match self.filesystem {
            FsAccess::None => {}
            FsAccess::Read => {
                out.insert(Permission::FilesystemRead);
            }
            FsAccess::Write => {
                out.insert(Permission::FilesystemWrite);
            }
        }
        if self.network {
            out.insert(Permission::Network);
        }
        if self.shell {
            out.insert(Permission::Shell);
        }
        if !self.env.is_empty() {
            out.insert(Permission::Env);
        }
        if self.hooks {
            out.insert(Permission::Hooks);
        }
        out
    }

    /// The required grants not covered by `allowed`.
    #[must_use]
    pub fn missing(&self, allowed: &BTreeSet<Permission>) -> BTreeSet<Permission> {
        self.required()
            .into_iter()
            .filter(|p| !p.is_satisfied_by(allowed))
            .collect()
    }

    /// Whether this declaration covers `needed`.
    #[must_use]
    pub fn covers(&self, needed: Permission) -> bool {
        needed.is_satisfied_by(&self.required())
    }
}

/// A permission a user grants in `grimoire.toml` (`allow = [...]`).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    /// Read files.
    FilesystemRead,
    /// Read and write files.
    FilesystemWrite,
    /// Use the network.
    Network,
    /// Run shell commands or other programs.
    Shell,
    /// Read environment variables.
    Env,
    /// Register automatic client hooks.
    Hooks,
}

impl Permission {
    /// Whether `granted` covers this permission. A write grant covers read.
    #[must_use]
    pub fn is_satisfied_by(self, granted: &BTreeSet<Permission>) -> bool {
        granted.contains(&self)
            || (self == Permission::FilesystemRead
                && granted.contains(&Permission::FilesystemWrite))
    }

    /// The value used in `allow` lists.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Permission::FilesystemRead => "filesystem-read",
            Permission::FilesystemWrite => "filesystem-write",
            Permission::Network => "network",
            Permission::Shell => "shell",
            Permission::Env => "env",
            Permission::Hooks => "hooks",
        }
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Permission {
    /// Every permission, in `allow` order.
    pub const ALL: [Permission; 6] = [
        Permission::FilesystemRead,
        Permission::FilesystemWrite,
        Permission::Network,
        Permission::Shell,
        Permission::Env,
        Permission::Hooks,
    ];
}

impl FromStr for Permission {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Permission::ALL
            .into_iter()
            .find(|p| p.as_str() == value)
            .ok_or_else(|| {
                InvalidValue::new(
                    "permission",
                    value,
                    "must be one of filesystem-read, filesystem-write, network, shell, env, hooks",
                )
            })
    }
}

/// A client-neutral tool capability granted to an agent or matched by a hook.
///
/// Written `read`, `search`, `edit`, `execute`, `web`, `mcp:<server-id>` or
/// `mcp:<server-id>/<tool>`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Capability {
    /// Read files.
    Read,
    /// Search file names and contents.
    Search,
    /// Create and modify files.
    Edit,
    /// Run shell commands.
    Execute,
    /// Fetch from or search the web.
    Web,
    /// Use tools of a Grimoire MCP server asset: all of them, or one.
    Mcp {
        /// The `mcp-server` asset.
        server: AssetId,
        /// One tool of that server, or all tools when absent.
        tool: Option<McpToolName>,
    },
}

impl Capability {
    /// Parses a capability string.
    ///
    /// # Errors
    ///
    /// Fails for unknown capabilities or malformed `mcp:` references.
    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        match value {
            "read" => return Ok(Capability::Read),
            "search" => return Ok(Capability::Search),
            "edit" => return Ok(Capability::Edit),
            "execute" => return Ok(Capability::Execute),
            "web" => return Ok(Capability::Web),
            _ => {}
        }
        let rest = value.strip_prefix("mcp:").ok_or_else(|| {
            InvalidValue::new(
                "capability",
                value,
                "must be read, search, edit, execute, web, mcp:<server> or mcp:<server>/<tool>",
            )
        })?;
        let (server, tool) = match rest.split_once('/') {
            Some((server, tool)) => (server, Some(tool)),
            None => (rest, None),
        };
        let server = AssetId::new(server)
            .map_err(|e| InvalidValue::new("capability", value, e.reason().to_owned()))?;
        let tool = tool
            .map(McpToolName::new)
            .transpose()
            .map_err(|e| InvalidValue::new("capability", value, e.reason().to_owned()))?;
        Ok(Capability::Mcp { server, tool })
    }

    /// The permission an asset must declare to be granted this capability.
    /// MCP capabilities declare nothing here: the server declares its own.
    #[must_use]
    pub fn implied_permission(&self) -> Option<Permission> {
        match self {
            Capability::Read | Capability::Search => Some(Permission::FilesystemRead),
            Capability::Edit => Some(Permission::FilesystemWrite),
            Capability::Execute => Some(Permission::Shell),
            Capability::Web => Some(Permission::Network),
            Capability::Mcp { .. } => None,
        }
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capability::Read => f.write_str("read"),
            Capability::Search => f.write_str("search"),
            Capability::Edit => f.write_str("edit"),
            Capability::Execute => f.write_str("execute"),
            Capability::Web => f.write_str("web"),
            Capability::Mcp { server, tool: None } => write!(f, "mcp:{server}"),
            Capability::Mcp {
                server,
                tool: Some(tool),
            } => write!(f, "mcp:{server}/{tool}"),
        }
    }
}

impl TryFrom<String> for Capability {
    type Error = InvalidValue;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<Capability> for String {
    fn from(value: Capability) -> Self {
        value.to_string()
    }
}

impl FromStr for Capability {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl JsonSchema for Capability {
    fn schema_name() -> Cow<'static, str> {
        "Capability".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "pattern": "^(read|search|edit|execute|web|mcp:[a-z0-9]+(-[a-z0-9]+)*(/[A-Za-z0-9_.-]{1,128})?)$",
            "description": "Client-neutral capability: read, search, edit, execute, web, mcp:<server-asset-id> or mcp:<server-asset-id>/<tool>."
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_round_trip() {
        for text in [
            "read",
            "search",
            "edit",
            "execute",
            "web",
            "mcp:adr-index",
            "mcp:adr-index/search_adrs",
        ] {
            assert_eq!(Capability::parse(text).unwrap().to_string(), text);
        }
        assert!(Capability::parse("write").is_err());
        assert!(Capability::parse("mcp:").is_err());
        assert!(Capability::parse("mcp:Bad").is_err());
        assert!(Capability::parse("mcp:ok/bad tool").is_err());
    }

    #[test]
    fn write_grant_covers_read() {
        let granted = BTreeSet::from([Permission::FilesystemWrite]);
        assert!(Permission::FilesystemRead.is_satisfied_by(&granted));
        assert!(!Permission::Network.is_satisfied_by(&granted));
        let declared = Permissions {
            filesystem: FsAccess::Read,
            network: true,
            ..Permissions::default()
        };
        assert_eq!(
            declared.missing(&granted),
            BTreeSet::from([Permission::Network])
        );
        assert!(declared.covers(Permission::FilesystemRead));
        assert!(!declared.covers(Permission::FilesystemWrite));
    }
}
