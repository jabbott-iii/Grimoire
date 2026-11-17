//! The asset manifest, `catalog/<kind>/<id>/asset.toml` (`architecture.md` §6.1).

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AssetId, BinaryName, Capability, ClientId, ClientOutputs, Description, EnvVarName, Format,
    GithubLogin, Glob, HookArg, Kind, LicenseExpr, Maturity, McpToolName, ParseError, Permissions,
    Reason, Summary, Tag, Transport, Version, VersionReq,
};

/// An `asset.toml` manifest.
///
/// Unknown fields are errors. Cross-field rules are checked by
/// [`validate::manifest`](crate::validate::manifest).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[schemars(title = "Grimoire asset manifest (asset.toml)")]
pub struct AssetManifest {
    /// Manifest format version; must be 1.
    pub format: Format,
    /// Catalog-wide unique identifier; equals the asset directory name.
    pub id: AssetId,
    /// Asset kind; must match the parent directory under `catalog/`.
    pub kind: Kind,
    /// The asset's own `SemVer` version, independent of the CLI version.
    pub version: Version,
    /// One-line summary.
    pub summary: Summary,
    /// Longer Markdown description.
    pub description: Description,
    /// SPDX license expression for the asset's content.
    pub license: LicenseExpr,
    /// GitHub logins of the maintainers; at least one.
    pub maintainers: Vec<GithubLogin>,
    /// Catalog tags.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<Tag>,
    /// Maturity.
    pub maturity: Maturity,
    /// Required when `maturity = "deprecated"`; forbidden otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<Deprecation>,
    /// Oldest Grimoire CLI that can install this asset.
    pub min_cli_version: Version,
    /// Assets this asset depends on, with version requirements.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(schema_with = "crate::schema::keyed_map::<AssetId, VersionReq>")]
    pub requires: BTreeMap<AssetId, VersionReq>,
    /// Assets that must not be installed together with this one.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    #[schemars(schema_with = "crate::schema::keyed_map::<AssetId, ConflictSpec>")]
    pub conflicts: BTreeMap<AssetId, ConflictSpec>,
    /// What the asset can cause at runtime.
    #[serde(default)]
    pub permissions: Permissions,
    /// Rule settings; only for `kind = "rule"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<RuleSpec>,
    /// Skill settings; only for `kind = "skill"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill: Option<SkillSpec>,
    /// Agent settings; required for `kind = "agent"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent: Option<AgentSpec>,
    /// Tool settings; required for `kind = "tool"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<ToolSpec>,
    /// MCP server settings; required for `kind = "mcp-server"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcp_server: Option<McpServerSpec>,
    /// Outputs per supported client. A client is supported iff its table is present.
    #[schemars(schema_with = "crate::schema::keyed_map::<ClientId, ClientOutputs>")]
    pub clients: BTreeMap<ClientId, ClientOutputs>,
}

impl AssetManifest {
    /// Parses an `asset.toml` document.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseError`] with the TOML location for syntax errors,
    /// unknown fields and invalid values.
    pub fn from_toml(text: &str) -> Result<Self, ParseError> {
        crate::parse_toml(text)
    }
}

/// Deprecation details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Deprecation {
    /// The asset that replaces this one.
    pub replacement: AssetId,
    /// Why the asset is deprecated.
    pub reason: Reason,
}

/// A declared conflict with another asset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ConflictSpec {
    /// Versions of the other asset this one conflicts with; `"*"` for all.
    pub version: VersionReq,
    /// Why the two must not be installed together.
    pub reason: Reason,
}

/// Rule settings.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct RuleSpec {
    /// Globs the rule applies to. Empty or absent means always-on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paths: Vec<Glob>,
}

/// Skill settings. There are none yet; the skill's own `SKILL.md` carries its metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct SkillSpec {}

/// Agent settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct AgentSpec {
    /// The explicit, least-privilege tool allow-list. Required; `[]` grants no tools.
    pub tools: Vec<Capability>,
}

/// Tool settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct ToolSpec {
    /// Executable name: `grimoire-tool-<id>`.
    pub binary: BinaryName,
    /// Client hooks that run this tool automatically.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub hooks: Vec<HookBinding>,
}

/// A hook that runs the tool's binary on a client event, in exec form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct HookBinding {
    /// The client-neutral event.
    pub event: HookEvent,
    /// For tool events, the capabilities whose tool calls trigger the hook. Empty means all.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<Capability>,
    /// Arguments passed to the binary.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<HookArg>,
    /// Timeout in seconds, 1 to 600.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout_secs: Option<u32>,
    /// What the client does when the hook cannot run: the binary is missing,
    /// crashes, times out or prints invalid output.
    #[serde(default, skip_serializing_if = "HookFailure::is_default")]
    pub on_failure: HookFailure,
}

/// What a client does when a hook cannot run.
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
pub enum HookFailure {
    /// The event goes ahead (the clients' default).
    #[default]
    Continue,
    /// The event is blocked, as if the hook had refused it. For guards that must
    /// fail closed, such as a secret scanner. Only on `pre-tool-use` and
    /// `user-prompt-submit`, the events a hook can block.
    Block,
}

impl HookFailure {
    #[allow(
        clippy::trivially_copy_pass_by_ref,
        reason = "serde's skip_serializing_if passes a reference"
    )]
    fn is_default(&self) -> bool {
        *self == Self::Continue
    }
}

/// A client-neutral hook event. Adapters map these to client event names or refuse.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "kebab-case")]
pub enum HookEvent {
    /// Before a tool call runs.
    PreToolUse,
    /// After a tool call completes.
    PostToolUse,
    /// When a session starts.
    SessionStart,
    /// When a session ends.
    SessionEnd,
    /// When the user submits a prompt.
    UserPromptSubmit,
    /// When the agent finishes responding.
    Stop,
}

impl HookEvent {
    /// Whether the event concerns a tool call, so a `tools` filter applies.
    #[must_use]
    pub fn is_tool_event(self) -> bool {
        matches!(self, HookEvent::PreToolUse | HookEvent::PostToolUse)
    }
}

/// MCP server settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct McpServerSpec {
    /// Executable name: `grimoire-mcp-<id>`.
    pub binary: BinaryName,
    /// Supported transports; production servers support both.
    pub transports: Vec<Transport>,
    /// Environment variables the server reads.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvVarSpec>,
    /// The tools the server exposes, with explicit annotations.
    pub tools: Vec<DeclaredTool>,
}

/// An environment variable an MCP server reads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct EnvVarSpec {
    /// Variable name.
    pub name: EnvVarName,
    /// Whether the server refuses to start without it.
    pub required: bool,
    /// Whether it holds a secret. Secrets are only ever referenced, never written.
    pub secret: bool,
    /// What the variable is for.
    pub description: Summary,
}

/// A tool an MCP server exposes, with every annotation stated explicitly (`MCP-SPEC`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[allow(
    clippy::struct_excessive_bools,
    reason = "mirrors the four boolean MCP tool annotations one-to-one"
)]
pub struct DeclaredTool {
    /// Tool name.
    pub name: McpToolName,
    /// `readOnlyHint`: the tool does not modify its environment.
    pub read_only: bool,
    /// `destructiveHint`: the tool may perform destructive updates.
    pub destructive: bool,
    /// `idempotentHint`: repeated identical calls have no additional effect.
    pub idempotent: bool,
    /// `openWorldHint`: the tool interacts with external entities.
    pub open_world: bool,
}
