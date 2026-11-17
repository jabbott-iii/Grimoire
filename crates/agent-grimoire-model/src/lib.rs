//! Data model for Grimoire.
//!
//! Covers asset manifests (`asset.toml`), bundles, the project configuration
//! (`grimoire.toml`) and the lockfile (`grimoire.lock`), as specified in
//! `docs/architecture.md` §6.
//!
//! Every document deserializes strictly. Unknown fields are errors, and
//! constrained values (identifiers, paths, versions, license expressions) are
//! validated while parsing, so errors carry the TOML location. Rules that span
//! several fields of one document are checked by [`validate`]. Rules that span
//! files or assets belong to the catalog crate.

mod bundle;
mod config;
mod constrained;
mod diagnostic;
mod license;
mod lock;
mod manifest;
mod output;
mod path;
mod permissions;
mod scalars;
pub mod schema;
pub mod validate;
mod values;
mod version;

use std::fmt;
use std::ops::Range;

pub use bundle::Bundle;
pub use config::{AssetOptions, AssetSelection, ProjectConfig};
pub use constrained::InvalidValue;
pub use diagnostic::{Diagnostic, Severity, has_errors, sort as sort_diagnostics};
pub use license::LicenseExpr;
pub use lock::{LockedAsset, LockedOutput, Lockfile, Selector, SourceId, Unit};
pub use manifest::{
    AgentSpec, AssetManifest, ConflictSpec, DeclaredTool, Deprecation, EnvVarSpec, HookBinding,
    HookEvent, McpServerSpec, RuleSpec, SkillSpec, ToolSpec,
};
pub use output::{Anchor, ClientId, ClientOutputs, Destination, OutputSpec, Strategy};
pub use path::{MAX_REL_PATH, RelPath};
pub use permissions::{Capability, FsAccess, Permission, Permissions};
pub use scalars::{Format, Kind, Maturity, Sha256Digest, Transport};
pub use values::{
    AssetId, BinaryName, Description, EntryKey, EnvVarName, GithubLogin, Glob, HookArg,
    McpToolName, Reason, Summary, Tag,
};
pub use version::{Version, VersionReq};

/// A TOML document that failed to parse or deserialize.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    message: String,
    span: Option<Range<usize>>,
}

impl ParseError {
    /// The parser's message, including the line and column.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The byte range of the offending input, when known.
    #[must_use]
    pub fn span(&self) -> Option<Range<usize>> {
        self.span.clone()
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.message.trim_end())
    }
}

impl std::error::Error for ParseError {}

pub(crate) fn parse_toml<T: serde::de::DeserializeOwned>(text: &str) -> Result<T, ParseError> {
    toml::from_str(text).map_err(|e| ParseError {
        message: e.to_string(),
        span: e.span(),
    })
}
