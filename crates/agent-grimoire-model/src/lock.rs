//! The lockfile, `grimoire.lock` (`architecture.md` §6.5).

use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt;
use std::str::FromStr;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};

use crate::{
    AssetId, ClientId, EntryKey, Format, InvalidValue, Kind, ParseError, Permission, RelPath,
    Sha256Digest, Tag, Version,
};

/// A `grimoire.lock` document. Never contains the CLI version (ADR-0001).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(title = "Grimoire lockfile (grimoire.lock)")]
pub struct Lockfile {
    /// Lockfile format version; must be 1.
    pub format: Format,
    /// Resolved assets, sorted by id.
    #[serde(default, rename = "asset", skip_serializing_if = "Vec::is_empty")]
    pub assets: Vec<LockedAsset>,
    /// Content Grimoire itself owns rather than any asset, such as the `CLAUDE.md`
    /// block that imports `AGENTS.md` (`architecture.md` §8.2). Its managed blocks
    /// use the reserved name [`Lockfile::PROJECT_OWNER`].
    #[serde(default, rename = "output", skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<LockedOutput>,
}

impl Lockfile {
    /// An empty lockfile in the current format.
    #[must_use]
    pub fn new() -> Self {
        Self {
            format: Format::CURRENT,
            assets: Vec::new(),
            outputs: Vec::new(),
        }
    }

    /// The owner name of outputs that belong to Grimoire rather than an asset.
    /// It is a reserved asset id, so no asset can claim it.
    pub const PROJECT_OWNER: &'static str = "grimoire";

    /// Parses a `grimoire.lock` document.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] with the TOML location on failure.
    pub fn from_toml(text: &str) -> Result<Self, ParseError> {
        crate::parse_toml(text)
    }

    /// Sorts everything into the canonical order, so serialization is deterministic.
    pub fn canonicalize(&mut self) {
        fn sort_outputs(outputs: &mut [LockedOutput]) {
            for output in outputs.iter_mut() {
                output.clients.sort();
                output.clients.dedup();
            }
            outputs.sort_by(|a, b| (&a.path, &a.key).cmp(&(&b.path, &b.key)));
        }
        self.assets.sort_by(|a, b| a.id.cmp(&b.id));
        for asset in &mut self.assets {
            asset.selected_by.sort();
            asset.selected_by.dedup();
            sort_outputs(&mut asset.outputs);
        }
        sort_outputs(&mut self.outputs);
    }
}

impl Default for Lockfile {
    fn default() -> Self {
        Self::new()
    }
}

/// One resolved asset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct LockedAsset {
    /// Asset id.
    pub id: AssetId,
    /// Asset kind.
    pub kind: Kind,
    /// Resolved version.
    pub version: Version,
    /// Where the asset came from.
    pub source: SourceId,
    /// Digest of the asset's manifest as resolved.
    pub manifest: Sha256Digest,
    /// Why the asset is installed.
    pub selected_by: Vec<Selector>,
    /// Permissions accepted at resolution time.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub allow: BTreeSet<Permission>,
    /// Content Grimoire owns on disk for this asset.
    #[serde(default, rename = "output", skip_serializing_if = "Vec::is_empty")]
    pub outputs: Vec<LockedOutput>,
}

/// A catalog source (ADR-0001). v1 has only the catalog embedded in the binary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum SourceId {
    /// The catalog embedded in the `grimoire` binary.
    Builtin,
}

/// One unit of content Grimoire owns (`architecture.md` §7.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct LockedOutput {
    /// Clients this output serves; several when a location is shared.
    pub clients: Vec<ClientId>,
    /// Path relative to the project root.
    pub path: RelPath,
    /// What Grimoire owns at that path.
    pub unit: Unit,
    /// For `entry` units, the owned key path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<EntryKey>,
    /// Hash of the content Grimoire owns in the unit.
    pub hash: Sha256Digest,
    /// For `block` and `entry` units: Grimoire created the file, so it deletes the
    /// file when the last thing in it is removed. A user's own file is never deleted.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub created_file: bool,
    /// For `entry` units: the containers (JSON Pointers) Grimoire created for this
    /// entry, which it removes again once they are empty. Containers that were
    /// already there are left alone.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub created_keys: Vec<EntryKey>,
}

/// The ownership unit of an output.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
    /// The whole file.
    File,
    /// A delimited managed block inside a Markdown file.
    Block,
    /// One key or array element inside a JSON or TOML file.
    Entry,
}

/// Why an asset is installed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub enum Selector {
    /// Listed under `[assets]`.
    Direct,
    /// Brought in by a bundle.
    Bundle(AssetId),
    /// Brought in by a tag.
    Tag(Tag),
    /// Required by another installed asset.
    Requires(AssetId),
}

impl Selector {
    /// Parses `direct`, `bundle:<id>`, `tag:<tag>` or `requires:<id>`.
    ///
    /// # Errors
    ///
    /// Fails for any other form.
    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        let invalid = |reason: &str| InvalidValue::new("selector", value, reason.to_owned());
        if value == "direct" {
            return Ok(Selector::Direct);
        }
        let (kind, rest) = value
            .split_once(':')
            .ok_or_else(|| invalid("must be direct, bundle:<id>, tag:<tag> or requires:<id>"))?;
        match kind {
            "bundle" => AssetId::new(rest).map(Selector::Bundle),
            "tag" => Tag::new(rest).map(Selector::Tag),
            "requires" => AssetId::new(rest).map(Selector::Requires),
            _ => Err(invalid(
                "must be direct, bundle:<id>, tag:<tag> or requires:<id>",
            )),
        }
        .map_err(|e| invalid(e.reason()))
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Selector::Direct => f.write_str("direct"),
            Selector::Bundle(id) => write!(f, "bundle:{id}"),
            Selector::Tag(tag) => write!(f, "tag:{tag}"),
            Selector::Requires(id) => write!(f, "requires:{id}"),
        }
    }
}

impl TryFrom<String> for Selector {
    type Error = InvalidValue;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<Selector> for String {
    fn from(value: Selector) -> Self {
        value.to_string()
    }
}

impl FromStr for Selector {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl JsonSchema for Selector {
    fn schema_name() -> Cow<'static, str> {
        "Selector".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "pattern": "^(direct|(bundle|requires):[a-z0-9]+(-[a-z0-9]+)*|tag:[a-z0-9]+(-[a-z0-9]+)*)$",
            "description": "Why an asset is installed: direct, bundle:<id>, tag:<tag> or requires:<id>."
        })
    }
}
