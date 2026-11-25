//! Bundles, `catalog/bundles/<id>.toml`.
//!
//! The design is in [`docs/subsystems/catalog.md`, Bundle (not an asset kind)](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/catalog.md#bundle-not-an-asset-kind).

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{AssetId, Description, Format, GithubLogin, ParseError, Summary, Version, VersionReq};

/// A named, versioned list of asset references. Not an asset kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(title = "Grimoire bundle (catalog/bundles/<id>.toml)")]
pub struct Bundle {
    /// Bundle format version; must be 1.
    pub format: Format,
    /// Identifier, unique across assets and bundles; equals the file stem.
    pub id: AssetId,
    /// The bundle's `SemVer` version.
    pub version: Version,
    /// One-line summary.
    pub summary: Summary,
    /// Longer Markdown description.
    pub description: Description,
    /// GitHub logins of the maintainers; at least one.
    pub maintainers: Vec<GithubLogin>,
    /// Member assets with version requirements; at least one.
    #[schemars(schema_with = "crate::schema::keyed_map::<AssetId, VersionReq>")]
    pub assets: BTreeMap<AssetId, VersionReq>,
}

impl Bundle {
    /// Parses a bundle document.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] with the TOML location on failure.
    pub fn from_toml(text: &str) -> Result<Self, ParseError> {
        crate::parse_toml(text)
    }
}
