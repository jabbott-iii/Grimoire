//! The project configuration, `grimoire.toml` in a user's repository (`architecture.md` §6.2).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use schemars::JsonSchema;
use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    AssetId, ClientId, EnvVarName, Format, ParseError, Permission, Tag, Transport, VersionReq,
};

/// A `grimoire.toml` document: the desired state of a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(title = "Grimoire project configuration (grimoire.toml)")]
pub struct ProjectConfig {
    /// Configuration format version; must be 1.
    pub format: Format,
    /// Clients to render for; at least one.
    pub clients: Vec<ClientId>,
    /// Bundles to install.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bundles: Vec<AssetId>,
    /// Install every asset carrying any of these tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Never install these, even when a bundle or tag selects them.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<AssetId>,
    /// Permissions accepted for assets that are installed without an `[assets]` entry,
    /// i.e. brought in by a bundle, a tag or another asset's `requires`.
    /// An asset listed under `[assets]` takes its grants there instead.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(schema_with = "crate::schema::keyed_map::<AssetId, BTreeSet<Permission>>")]
    pub allow: BTreeMap<AssetId, BTreeSet<Permission>>,
    /// Directly selected assets.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(schema_with = "crate::schema::keyed_map::<AssetId, AssetSelection>")]
    pub assets: BTreeMap<AssetId, AssetSelection>,
}

impl ProjectConfig {
    /// Parses a `grimoire.toml` document.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] with the TOML location on failure.
    pub fn from_toml(text: &str) -> Result<Self, ParseError> {
        crate::parse_toml(text)
    }

    /// The permissions the user accepted for `id`, wherever they are recorded.
    #[must_use]
    pub fn allowed(&self, id: &AssetId) -> BTreeSet<Permission> {
        match self.assets.get(id).and_then(AssetSelection::options) {
            Some(options) => options.allow.clone(),
            None => self.allow.get(id).cloned().unwrap_or_default(),
        }
    }
}

/// A directly selected asset: a bare version requirement, or a table of options.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum AssetSelection {
    /// `id = "^1"`.
    Version(VersionReq),
    /// `id = { version = "^1", ... }`.
    Detailed(AssetOptions),
}

impl AssetSelection {
    /// The version requirement.
    #[must_use]
    pub fn version(&self) -> &VersionReq {
        match self {
            AssetSelection::Version(req) => req,
            AssetSelection::Detailed(options) => &options.version,
        }
    }

    /// The options, when given as a table.
    #[must_use]
    pub fn options(&self) -> Option<&AssetOptions> {
        match self {
            AssetSelection::Version(_) => None,
            AssetSelection::Detailed(options) => Some(options),
        }
    }
}

// Hand-written so errors stay specific: serde's untagged derive reports only
// "data did not match any variant".
impl<'de> Deserialize<'de> for AssetSelection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SelectionVisitor;

        impl<'de> Visitor<'de> for SelectionVisitor {
            type Value = AssetSelection;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a version requirement string or a table with `version`")
            }

            fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
                VersionReq::parse(value)
                    .map(AssetSelection::Version)
                    .map_err(E::custom)
            }

            fn visit_map<A: MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
                AssetOptions::deserialize(de::value::MapAccessDeserializer::new(map))
                    .map(AssetSelection::Detailed)
            }
        }

        deserializer.deserialize_any(SelectionVisitor)
    }
}

/// Per-asset options in `grimoire.toml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AssetOptions {
    /// Version requirement.
    pub version: VersionReq,
    /// Render only for these clients (a subset of the project's clients).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clients: Option<Vec<ClientId>>,
    /// Permissions the user has accepted for this asset.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub allow: BTreeSet<Permission>,
    /// For MCP servers: maps a variable the server reads to the name it has in the user's environment.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(schema_with = "crate::schema::keyed_map::<EnvVarName, EnvVarName>")]
    pub env: BTreeMap<EnvVarName, EnvVarName>,
    /// For MCP servers: the transport to configure.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<Transport>,
}
