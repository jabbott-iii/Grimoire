//! Data model for Grimoire: strict types for asset manifests (`asset.toml`),
//! bundles, the project configuration (`grimoire.toml`) and the lockfile
//! (`grimoire.lock`), the checks that need only one of those documents
//! ([`validate`]), and the source of their JSON Schemas ([`schema`]).
//!
//! The four documents are TOML, a configuration file format. Every one of them
//! deserializes strictly. Unknown fields are errors, and constrained values
//! (identifiers, paths, versions, license expressions and others) are
//! validated while parsing, so errors carry the TOML location. Rules that span
//! several fields of one document are checked by [`validate`]. Rules that span
//! files or assets belong to the catalog crate, `agent-grimoire-catalog`.
//!
//! # Where it fits
//!
//! Grimoire is a catalog of AI-agent assets (rules, skills, agents, tools and
//! MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the
//! command-line interface (CLI) that installs the assets a project selects.
//! This crate is the bottom layer: every other crate of the CLI builds on
//! it, and it depends on none of them.
//!
//! - `agent-grimoire-catalog` parses and checks each `asset.toml` and bundle
//!   with [`AssetManifest::from_toml`], [`Bundle::from_toml`],
//!   [`validate::manifest`] and [`validate::bundle`]: in
//!   `cargo xtask validate`, and again when it loads the catalog built into
//!   the binary.
//! - `agent-grimoire-resolve` reads a project's [`ProjectConfig`] and
//!   [`Lockfile`] to work out which assets to install, check each one's
//!   version and check permission consent.
//! - `agent-grimoire-adapters` resolves each asset's [`OutputSpec`]s to
//!   paths for each client, and `agent-grimoire-install` records the content
//!   it writes as [`LockedOutput`]s.
//! - The CLI, `agent-grimoire`, reads `grimoire.toml` and `grimoire.lock`
//!   with [`ProjectConfig::from_toml`] and [`Lockfile::from_toml`], checks
//!   them with [`validate::project_config`] and [`validate::lockfile`], and
//!   writes the lock after [`Lockfile::canonicalize`].
//! - `cargo xtask schema` writes `schemas/` from [`schema::documents`].
//!
//! The crate is internal to the CLI. It is published only so that
//! `cargo install agent-grimoire` can build the CLI from crates.io, and its
//! Rust API changes whenever the CLI needs it to, so don't depend on it. A
//! change that breaks it still ships as a new 0.y version, as the
//! [versioning policy](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#versioning)
//! explains.
//!
//! The design is in:
//! [`docs/subsystems/model.md`, Identifiers and versions](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#identifiers-and-versions),
//! [`docs/subsystems/catalog.md`, Bundle](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/catalog.md#bundle-not-an-asset-kind),
//! and [`docs/subsystems/model.md`, Manifests and configuration](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#manifests-and-configuration),
//! which specifies `asset.toml`, `grimoire.toml` and `grimoire.lock`, each
//! with an annotated example.
//!
//! # The parts
//!
//! - **The four documents**, each parsed by its `from_toml`, which fails
//!   with a [`ParseError`]:
//!   - [`AssetManifest`], one asset's `asset.toml`. Its kind table is one of
//!     [`RuleSpec`], [`SkillSpec`], [`AgentSpec`], [`ToolSpec`] and
//!     [`McpServerSpec`], and [`Permissions`] says what the asset can cause
//!     at runtime.
//!   - [`Bundle`], a named, versioned list of assets that a project can
//!     select together, in `catalog/bundles/<id>.toml`. A bundle is not an
//!     asset kind.
//!   - [`ProjectConfig`], a project's `grimoire.toml`: its clients, and the
//!     assets it selects directly ([`AssetSelection`], [`AssetOptions`]) or
//!     by bundle or tag.
//!   - [`Lockfile`], `grimoire.lock`, which records exactly what was
//!     installed: each [`LockedAsset`], why it was selected ([`Selector`]),
//!     and each [`LockedOutput`] it owns, by [`Unit`].
//! - **Identifiers and versions:** [`AssetId`], [`Kind`], [`Tag`],
//!   [`Version`] and [`VersionReq`] (below, "Identifiers and versions").
//! - **Where content goes:** [`ClientId`] names each of the six clients
//!   (the AI coding tools Grimoire writes for). An [`OutputSpec`] names a
//!   source file, a [`Destination`] and a [`Strategy`]. A destination is an
//!   [`Anchor`], a named place that each client's adapter resolves to a real
//!   path, plus a [`RelPath`] for anchors that take one, as in
//!   `rules:rust-errors.md`.
//! - **Consent:** a [`Permission`] is a grant a user writes in
//!   `grimoire.toml`, such as `network` or `env:GITHUB_TOKEN`, and
//!   [`Permissions::missing`] lists the grants an asset still needs. A
//!   [`Capability`] is a tool an agent may use, such as `read` or
//!   `mcp:<server>/<tool>`.
//! - **Other constrained values**, such as [`LicenseExpr`] (an SPDX license
//!   expression; SPDX is the Software Package Data Exchange standard),
//!   [`ServerUrl`], [`EnvVarName`] and [`Sha256Digest`]. Each is checked when
//!   it is made, with `new` or `parse`, and when it is parsed from a
//!   document; a value that breaks its rules is an [`InvalidValue`].
//! - **Checks:** the [`validate`] module, with [`validate::manifest`],
//!   [`validate::bundle`], [`validate::project_config`] and
//!   [`validate::lockfile`]. Each returns a list of [`Diagnostic`]s;
//!   [`has_errors`] says whether any of them is an error, and
//!   [`sort_diagnostics`] puts them in a stable order.
//! - **Schemas:** [`schema::documents`], the JSON Schema of each of the four
//!   documents and its file name under `schemas/`.
//!
//! # Example
//!
//! A rule's `asset.toml` parses, and then validates cleanly. An unknown field
//! is a parse error that points at the field, and a rule that declares
//! permissions parses but fails [`validate::manifest`]:
//!
//! ```
//! use agent_grimoire_model::{AssetManifest, Kind, has_errors, validate};
//!
//! let text = r#"
//! format = 1
//! id = "rust-errors"
//! kind = "rule"
//! version = "1.0.0"
//! summary = "Idiomatic error handling for Rust CLIs"
//! description = "Use typed errors in libraries and context-rich errors in binaries."
//! license = "MIT OR Apache-2.0"
//! maintainers = ["example-maintainer"]
//! maturity = "stable"
//! min-cli-version = "0.1.0"
//!
//! [clients.claude-code]
//! outputs = [{ src = "rule.md", to = "rules:rust-errors.md", strategy = "render" }]
//! "#;
//!
//! let manifest = AssetManifest::from_toml(text)?;
//! assert_eq!(manifest.kind, Kind::Rule);
//! assert!(validate::manifest(&manifest).is_empty());
//!
//! // `maturity` misspelled: an unknown field, refused while parsing.
//! let typo = text.replace("maturity", "maturty");
//! let err = AssetManifest::from_toml(&typo).unwrap_err();
//! assert!(err.message().contains("unknown field `maturty`"));
//! assert_eq!(err.span().map(|span| &typo[span]), Some("maturty"));
//!
//! // Rules are pure text, so they must not declare permissions.
//! let rule = AssetManifest::from_toml(&format!("{text}\n[permissions]\nnetwork = true\n"))?;
//! let problems = validate::manifest(&rule);
//! assert!(has_errors(&problems));
//! assert_eq!(problems[0].code, "manifest/rule-permissions");
//! assert_eq!(problems[0].field.as_deref(), Some("permissions"));
//! # Ok::<(), agent_grimoire_model::ParseError>(())
//! ```
//!
//! # Parse errors and diagnostics
//!
//! A document is checked in two steps, and each reports problems its own way.
//!
//! 1. **Parsing** (`from_toml`) checks the TOML syntax, every field's type,
//!    unknown fields and each constrained value. The first problem stops it:
//!    the [`ParseError`]'s [`message`](ParseError::message) includes the line
//!    and column, and its [`span`](ParseError::span) is the byte range of the
//!    offending input, when known.
//! 2. **Validation** ([`validate`]) takes a parsed document and reports every
//!    rule it breaks, as a list. For example, a `deprecated` maturity needs a
//!    `[deprecated]` table that names the replacement, and an asset can't be
//!    both selected and excluded in `grimoire.toml`.
//!
//! Because unknown fields are refused, an older CLI refuses a file that uses
//! a field added after it was released: a project that uses a new field
//! needs a CLI that has it.
//!
//! A [`Diagnostic`] is an error or a warning ([`Severity`]). An error fails
//! validation; a warning is reported, but doesn't. Each has a stable `code`,
//! such as `manifest/kind-table-missing`, so tests and tooling match on the
//! code rather than on the message. Its `file` and `field` say where the
//! problem is, as in `clients.cursor.outputs[0]`: the checks set the field,
//! and the caller adds the file with [`Diagnostic::in_file`].
//!
//! The `config/` and `lock/` codes reach CLI users and are listed in
//! [`docs/diagnostics.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md).
//! The `manifest/` and `bundle/` codes are for catalog authors: catalog
//! validation reports them.
//!
//! A diagnostic's [`Display`](std::fmt::Display) output escapes control
//! characters in its file, field and message (newlines in the message are
//! kept), so that an untrusted name can't inject terminal escape sequences
//! or workflow commands for continuous integration (CI).
//!
//! # Identifiers and versions
//!
//! - **An [`AssetId`] is unique across the whole catalog**, not only within
//!   one kind, so commands need no kind (`grimoire add rust-errors`), the
//!   lock's keys are unambiguous, and the generated Claude Code marketplace
//!   can name each plugin after its asset. It follows the Agent Skills
//!   `name` rules for every kind: 1 to 64 characters of `a-z`, `0-9` and
//!   `-`, with no `-` at the start or end and no `--`. Bundles use the same
//!   type, and share the same namespace.
//! - **Some IDs are reserved.**
//!   - The prefixes `claude-`, `anthropic-`, `anthropics-` and `cc-plugin-`
//!     are refused, because Claude Code reserves them for plugin names, and
//!     each asset and bundle ID becomes a plugin name in the generated
//!     marketplace. Claude Code's plugin tools report such a name as an
//!     error.
//!   - The prefix `grimoire-` and the name `grimoire` are kept for content
//!     Grimoire itself owns in a project, such as the `CLAUDE.md` block that
//!     imports `AGENTS.md` ([`Lockfile::PROJECT_OWNER`]).
//!   - `synced` is a folder name Claude Code reserves for skills.
//! - **A [`Version`] is a semantic version (SemVer) 2.0.0, such as `1.2.3`
//!   or `1.2.3-rc.1`, without build metadata** (the part after a `+`). Build
//!   metadata doesn't change SemVer precedence, so refusing it makes the
//!   type's ordering exactly SemVer precedence. [`Version::parse`] and
//!   deserialization refuse it; `From<semver::Version>` doesn't check, so
//!   convert only a version without build metadata. Each asset has its own
//!   version, independent of the CLI's, and `min-cli-version` names the
//!   oldest CLI that can install it.
//! - **A [`VersionReq`] is a Cargo-style requirement**, such as `^1.2`,
//!   `>=1, <3` or `*`; [`VersionReq::matches`] tests a version against it.
//! - **Every document must set `format = 1`.** [`Format`] accepts only
//!   [`Format::CURRENT`], 1, so this version of Grimoire refuses any other
//!   format number.
//!
//! # Why paths are ASCII `RelPath`s
//!
//! Every path that is written or installed is a [`RelPath`]: relative, with
//! `/` separators, at most [`MAX_REL_PATH`] (200) characters, and safe on
//! Linux, macOS and Windows. It refuses:
//!
//! - an absolute path, a drive prefix such as `C:`, `\`, and empty, `.` or
//!   `..` components, so a destination string can't name anything outside
//!   the project root;
//! - any character Windows forbids in a file name (`<>:"|?*` and control
//!   characters), a component that ends in `.` or a space (Windows strips
//!   them), and the Windows device names, such as `CON` or `NUL.txt`;
//! - every non-ASCII character. That rules out invisible characters,
//!   bidirectional-text characters (which change the direction text is
//!   displayed in), superscript device names (Windows treats `COM¹` as a
//!   device), and names that macOS or Windows normalize or fold into each
//!   other.
//!
//! A `RelPath` checks only the string. A symbolic link on the way to the
//! path is a separate, runtime concern of the installer, which refuses to
//! write through one (architecture decision record
//! [ADR-0006](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0006-filesystem-confinement.md),
//! which confines every write to the project root). The design is in
//! [`docs/subsystems/install.md`, Filesystem safety](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#filesystem-safety).
//!
//! # The lockfile
//!
//! - **It never holds the CLI version**, so an upgrade alone doesn't change
//!   the lock. It changes only when an asset's version changes, or when what
//!   Grimoire writes for a client changes, which `grimoire plan` shows before
//!   `apply` records it
//!   ([ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md),
//!   which embeds the catalog in the binary). Each asset's [`SourceId`] is
//!   `builtin`, the catalog built into the binary, the only source today.
//! - **[`Lockfile::canonicalize`] sorts it**: assets by ID, outputs by path
//!   and then key, and each list of clients and selectors, with repeats
//!   removed. A canonical lock serializes the same way every time, so it
//!   diffs cleanly
//!   ([`docs/subsystems/model.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/model.md#grimoirelock)).
//! - **Content Grimoire owns for itself**, rather than for an asset, is in
//!   [`Lockfile::outputs`], under the reserved owner name
//!   [`Lockfile::PROJECT_OWNER`], `grimoire`.
//! - **[`validate::lockfile`] checks ownership:** a path is owned either as
//!   one whole `file`, or as `block`s, at most one per owner, or as `entry`
//!   units with distinct keys, and never by a mix of these.
//!
//! # The JSON Schemas
//!
//! [`schema::documents`] returns the four schemas, in a fixed order, with
//! the file name each one has under `schemas/`. They are generated with
//! schemars and pinned to JSON Schema draft 2020-12, so that a schemars
//! upgrade can't silently change the published draft. `cargo xtask schema`
//! writes them, and CI runs `cargo xtask schema --check`, which fails when
//! the checked-in files differ.
//!
//! The schemas don't state every rule the parser enforces. For example, the
//! license schema accepts any non-empty string, and the asset ID schema
//! names the reserved prefixes only in its description, so a document can
//! pass its schema and still fail to parse.
//!
//! ```
//! let names: Vec<&str> = agent_grimoire_model::schema::documents()
//!     .iter()
//!     .map(|document| document.file_name)
//!     .collect();
//! assert_eq!(
//!     names,
//!     [
//!         "asset.schema.json",
//!         "bundle.schema.json",
//!         "grimoire.schema.json",
//!         "grimoire-lock.schema.json",
//!     ]
//! );
//! ```

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

// The README's examples are compiled and run as doc tests too, so the
// crates.io page breaks the build when the API changes under it.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

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
    HookEvent, HookFailure, McpServerSpec, RuleSpec, SkillSpec, ToolSpec,
};
pub use output::{Anchor, ClientId, ClientOutputs, Destination, OutputSpec, Strategy};
pub use path::{MAX_REL_PATH, RelPath};
pub use permissions::{Capability, FsAccess, Permission, Permissions};
pub use scalars::{Format, Kind, Maturity, Sha256Digest, Transport};
pub use values::{
    AssetId, BinaryName, Description, EntryKey, EnvVarName, GithubLogin, Glob, HookArg,
    McpToolName, Reason, ServerUrl, Summary, Tag,
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
