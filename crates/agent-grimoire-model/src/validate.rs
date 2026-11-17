//! Cross-field validation of single documents (`architecture.md` §6.4).
//!
//! Parsing already enforces field types, value formats and unknown-field
//! rejection. These checks cover rules that span several fields of one
//! document. Rules that need other files or other assets live in the catalog
//! crate.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;

use crate::{
    Anchor, AssetManifest, Bundle, Capability, ClientId, Diagnostic, Kind, Lockfile, Maturity,
    ProjectConfig, Strategy, Transport, Unit,
};

/// The TOML table name of a kind's settings.
fn table_name(kind: Kind) -> &'static str {
    match kind {
        Kind::Rule => "rule",
        Kind::Skill => "skill",
        Kind::Agent => "agent",
        Kind::Tool => "tool",
        Kind::McpServer => "mcp-server",
    }
}

/// Reports values that appear more than once in `items`.
fn duplicates<'a, T: Ord + Display + 'a>(items: impl IntoIterator<Item = &'a T>) -> Vec<&'a T> {
    let mut seen = BTreeSet::new();
    let mut dups = BTreeSet::new();
    for item in items {
        if !seen.insert(item) {
            dups.insert(item);
        }
    }
    dups.into_iter().collect()
}

fn push_duplicates<'a, T: Ord + Display + 'a>(
    out: &mut Vec<Diagnostic>,
    code: &'static str,
    field: &str,
    items: impl IntoIterator<Item = &'a T>,
) {
    for dup in duplicates(items) {
        out.push(Diagnostic::error(code, format!("`{dup}` is listed more than once")).at(field));
    }
}

/// Validates an asset manifest on its own.
#[must_use]
pub fn manifest(m: &AssetManifest) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    kind_tables(m, &mut out);
    deprecation(m, &mut out);
    people_and_tags(m, &mut out);
    relations(m, &mut out);
    push_duplicates(
        &mut out,
        "manifest/permissions",
        "permissions.env",
        &m.permissions.env,
    );
    match m.kind {
        Kind::Rule => rule(m, &mut out),
        Kind::Skill => {}
        Kind::Agent => agent(m, &mut out),
        Kind::Tool => tool(m, &mut out),
        Kind::McpServer => mcp_server(m, &mut out),
    }
    clients(m, &mut out);
    out
}

fn kind_tables(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    let present = [
        (Kind::Rule, m.rule.is_some()),
        (Kind::Skill, m.skill.is_some()),
        (Kind::Agent, m.agent.is_some()),
        (Kind::Tool, m.tool.is_some()),
        (Kind::McpServer, m.mcp_server.is_some()),
    ];
    for (kind, is_present) in present {
        if is_present && kind != m.kind {
            out.push(
                Diagnostic::error(
                    "manifest/kind-table-mismatch",
                    format!(
                        "a [{}] table is only allowed when kind = \"{kind}\"",
                        table_name(kind)
                    ),
                )
                .at(table_name(kind)),
            );
        }
    }
    let needs_table = matches!(m.kind, Kind::Agent | Kind::Tool | Kind::McpServer);
    let has_table = present.iter().any(|(kind, p)| *kind == m.kind && *p);
    if needs_table && !has_table {
        out.push(
            Diagnostic::error(
                "manifest/kind-table-missing",
                format!(
                    "kind = \"{}\" requires a [{}] table",
                    m.kind,
                    table_name(m.kind)
                ),
            )
            .at(table_name(m.kind)),
        );
    }
}

fn deprecation(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    match (m.maturity, &m.deprecated) {
        (Maturity::Deprecated, None) => out.push(
            Diagnostic::error(
                "manifest/deprecation",
                "maturity = \"deprecated\" requires a [deprecated] table naming the replacement",
            )
            .at("deprecated"),
        ),
        (Maturity::Deprecated, Some(d)) if d.replacement == m.id => out.push(
            Diagnostic::error(
                "manifest/deprecation",
                "an asset cannot be its own replacement",
            )
            .at("deprecated.replacement"),
        ),
        (Maturity::Experimental | Maturity::Stable, Some(_)) => out.push(
            Diagnostic::error(
                "manifest/deprecation",
                "a [deprecated] table requires maturity = \"deprecated\"",
            )
            .at("deprecated"),
        ),
        _ => {}
    }
}

fn people_and_tags(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    if m.maintainers.is_empty() {
        out.push(
            Diagnostic::error(
                "manifest/maintainers",
                "at least one maintainer is required",
            )
            .at("maintainers"),
        );
    }
    let lowered: Vec<String> = m
        .maintainers
        .iter()
        .map(|l| l.as_str().to_ascii_lowercase())
        .collect();
    push_duplicates(out, "manifest/maintainers", "maintainers", &lowered);
    push_duplicates(out, "manifest/tags", "tags", &m.tags);
    if m.tags.len() > 10 {
        out.push(Diagnostic::error("manifest/tags", "at most 10 tags are allowed").at("tags"));
    }
}

fn relations(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    if m.requires.contains_key(&m.id) {
        out.push(
            Diagnostic::error("manifest/self-reference", "an asset cannot require itself")
                .at("requires"),
        );
    }
    if m.conflicts.contains_key(&m.id) {
        out.push(
            Diagnostic::error(
                "manifest/self-reference",
                "an asset cannot conflict with itself",
            )
            .at("conflicts"),
        );
    }
    for id in m.requires.keys().filter(|id| m.conflicts.contains_key(*id)) {
        out.push(
            Diagnostic::error(
                "manifest/requires-conflicts",
                format!("`{id}` is both required and declared as a conflict"),
            )
            .at("conflicts"),
        );
    }
}

fn rule(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    if !m.permissions.is_empty() {
        out.push(
            Diagnostic::error(
                "manifest/rule-permissions",
                "rules are pure text and must not declare permissions",
            )
            .at("permissions"),
        );
    }
    if let Some(spec) = &m.rule {
        for dup in duplicates(&spec.paths) {
            out.push(
                Diagnostic::warning(
                    "manifest/rule-paths",
                    format!("`{dup}` is listed more than once"),
                )
                .at("rule.paths"),
            );
        }
    }
}

fn agent(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    let Some(spec) = &m.agent else { return };
    push_duplicates(out, "manifest/agent-tools", "agent.tools", &spec.tools);
    let mut implied = BTreeSet::new();
    for cap in &spec.tools {
        if let Some(needed) = cap.implied_permission() {
            implied.insert(needed);
            if !m.permissions.covers(needed) {
                out.push(
                    Diagnostic::error(
                        "manifest/agent-permissions",
                        format!("the `{cap}` capability requires the asset to declare the `{needed}` permission"),
                    )
                    .at("agent.tools"),
                );
            }
        }
        if let Capability::Mcp { server, .. } = cap
            && !m.requires.contains_key(server)
        {
            out.push(
                Diagnostic::error(
                    "manifest/agent-mcp",
                    format!(
                        "`{cap}` uses the MCP server `{server}`, which must be listed in [requires]"
                    ),
                )
                .at("agent.tools"),
            );
        }
    }
    // Over-declaring is not dangerous, but it misleads users at consent time.
    for declared in m.permissions.required() {
        if !declared.is_satisfied_by(&implied) {
            out.push(
                Diagnostic::warning(
                    "manifest/permission-unused",
                    format!("the `{declared}` permission is declared but no tool in agent.tools needs it"),
                )
                .at("permissions"),
            );
        }
    }
}

fn tool(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    let Some(spec) = &m.tool else { return };
    let expected = format!("grimoire-tool-{}", m.id);
    if spec.binary.as_str() != expected {
        out.push(
            Diagnostic::error("manifest/binary-name", format!("must be `{expected}`"))
                .at("tool.binary"),
        );
    }
    if spec.hooks.is_empty() == m.permissions.hooks {
        out.push(
            Diagnostic::error(
                "manifest/hooks-permission",
                if spec.hooks.is_empty() {
                    "permissions.hooks is true but the tool declares no hooks"
                } else {
                    "a tool with hooks must declare permissions.hooks = true"
                },
            )
            .at("permissions.hooks"),
        );
    }
    let mut seen = BTreeSet::new();
    for (i, hook) in spec.hooks.iter().enumerate() {
        let field = format!("tool.hooks[{i}]");
        if !hook.tools.is_empty() && !hook.event.is_tool_event() {
            out.push(
                Diagnostic::error(
                    "manifest/hook-tools",
                    "only pre-tool-use and post-tool-use hooks can filter by tool",
                )
                .at(format!("{field}.tools")),
            );
        }
        if let Some(secs) = hook.timeout_secs
            && !(1..=600).contains(&secs)
        {
            out.push(
                Diagnostic::error(
                    "manifest/hook-timeout",
                    "timeout-secs must be between 1 and 600",
                )
                .at(format!("{field}.timeout-secs")),
            );
        }
        let key = (hook.event, &hook.tools, &hook.args);
        if !seen.insert(key) {
            out.push(
                Diagnostic::error("manifest/hook-duplicate", "duplicate hook binding").at(field),
            );
        }
    }
}

fn mcp_server(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    let Some(spec) = &m.mcp_server else { return };
    let expected = format!("grimoire-mcp-{}", m.id);
    if spec.binary.as_str() != expected {
        out.push(
            Diagnostic::error("manifest/binary-name", format!("must be `{expected}`"))
                .at("mcp-server.binary"),
        );
    }
    let transports: BTreeSet<Transport> = spec.transports.iter().copied().collect();
    if spec.transports.len() != 2
        || transports != BTreeSet::from([Transport::Stdio, Transport::StreamableHttp])
    {
        out.push(
            Diagnostic::error(
                "manifest/mcp-transports",
                "MCP servers must support both stdio and streamable-http, each listed once",
            )
            .at("mcp-server.transports"),
        );
    }
    if spec.tools.is_empty() {
        out.push(
            Diagnostic::error(
                "manifest/mcp-tools",
                "an MCP server must declare at least one tool",
            )
            .at("mcp-server.tools"),
        );
    }
    push_duplicates(
        out,
        "manifest/mcp-tools",
        "mcp-server.tools",
        spec.tools.iter().map(|t| &t.name),
    );
    for (i, t) in spec.tools.iter().enumerate() {
        if t.read_only && t.destructive {
            out.push(
                Diagnostic::error(
                    "manifest/mcp-annotations",
                    format!("`{}` cannot be both read-only and destructive", t.name),
                )
                .at(format!("mcp-server.tools[{i}]")),
            );
        }
        if t.open_world && !m.permissions.network {
            out.push(
                Diagnostic::warning(
                    "manifest/mcp-open-world",
                    format!(
                        "`{}` is open-world but the server does not declare network access",
                        t.name
                    ),
                )
                .at(format!("mcp-server.tools[{i}]")),
            );
        }
    }
    push_duplicates(
        out,
        "manifest/mcp-env",
        "mcp-server.env",
        spec.env.iter().map(|e| &e.name),
    );
    for (i, env) in spec.env.iter().enumerate() {
        if !m.permissions.env.contains(&env.name) {
            out.push(
                Diagnostic::error(
                    "manifest/mcp-env",
                    format!("`{}` must also be listed in permissions.env", env.name),
                )
                .at(format!("mcp-server.env[{i}]")),
            );
        }
    }
}

fn clients(m: &AssetManifest, out: &mut Vec<Diagnostic>) {
    if m.clients.is_empty() {
        out.push(
            Diagnostic::error("manifest/clients", "at least one client table is required")
                .at("clients"),
        );
    }
    for (client, outputs) in &m.clients {
        let base = format!("clients.{client}");
        client_outputs(m, *client, &outputs.outputs, &base, out);
    }
}

#[allow(clippy::too_many_lines)]
fn client_outputs(
    m: &AssetManifest,
    client: ClientId,
    outputs: &[crate::OutputSpec],
    base: &str,
    out: &mut Vec<Diagnostic>,
) {
    if outputs.is_empty() && m.kind != Kind::Tool {
        out.push(
            Diagnostic::error("manifest/outputs", "at least one output is required")
                .at(format!("{base}.outputs")),
        );
    }
    let mut seen = BTreeSet::new();
    for (i, o) in outputs.iter().enumerate() {
        let field = format!("{base}.outputs[{i}]");
        let anchor = o.to.anchor();
        if !client.supports(anchor) {
            out.push(
                Diagnostic::error(
                    "manifest/anchor-client",
                    format!("{client} has no `{anchor}` destination"),
                )
                .at(&field),
            );
        }
        if !anchor.accepts_kind(m.kind) {
            out.push(
                Diagnostic::error(
                    "manifest/anchor-kind",
                    format!("a {} asset cannot target the `{anchor}` anchor", m.kind),
                )
                .at(&field),
            );
        }
        if !o.strategy.allows(anchor) {
            out.push(
                Diagnostic::error(
                    "manifest/strategy",
                    format!(
                        "strategy `{}` cannot write to the `{anchor}` anchor",
                        strategy_name(o.strategy)
                    ),
                )
                .at(&field),
            );
        }
        match (o.strategy.needs_src(), &o.src) {
            (true, None) => out.push(
                Diagnostic::error(
                    "manifest/output-src",
                    format!("strategy `{}` needs `src`", strategy_name(o.strategy)),
                )
                .at(&field),
            ),
            (false, Some(_)) => out.push(
                Diagnostic::error(
                    "manifest/output-src",
                    "entry outputs are generated from the manifest and take no `src`",
                )
                .at(&field),
            ),
            _ => {}
        }
        if !seen.insert(&o.to) {
            out.push(
                Diagnostic::error(
                    "manifest/output-duplicate",
                    format!("`{}` is written twice", o.to),
                )
                .at(&field),
            );
        }
        // Per-kind naming and layout.
        match m.kind {
            Kind::Rule | Kind::Agent => {
                if let Some(path) = o.to.path()
                    && (path.file_stem() != m.id.as_str() || path.components().count() != 1)
                {
                    out.push(
                        Diagnostic::error(
                            "manifest/output-name",
                            format!("must be a single file named after the asset, e.g. `{anchor}:{}.md`", m.id),
                        )
                        .at(&field),
                    );
                }
            }
            Kind::Skill => {
                let expected = o.src.as_ref().map(|src| format!("{}/{src}", m.id));
                if o.to.path().map(crate::RelPath::as_str) != expected.as_deref() {
                    out.push(
                        Diagnostic::error(
                            "manifest/skill-layout",
                            format!(
                                "skill outputs must keep the skill's layout: `src = \"X\"` goes to `skills:{}/X`",
                                m.id
                            ),
                        )
                        .at(&field),
                    );
                }
            }
            Kind::Tool | Kind::McpServer => {}
        }
    }
    let count = outputs.len();
    match m.kind {
        Kind::Rule | Kind::Agent | Kind::McpServer if count > 1 => out.push(
            Diagnostic::error(
                "manifest/output-count",
                format!("a {} asset has exactly one output per client", m.kind),
            )
            .at(format!("{base}.outputs")),
        ),
        Kind::Skill
            if !outputs.is_empty()
                && !outputs
                    .iter()
                    .any(|o| o.src.as_ref().is_some_and(|s| s.as_str() == "SKILL.md")) =>
        {
            out.push(
                Diagnostic::error(
                    "manifest/skill-layout",
                    "skill outputs must include `SKILL.md`",
                )
                .at(format!("{base}.outputs")),
            );
        }
        Kind::Tool => {
            let has_hooks = m.tool.as_ref().is_some_and(|t| !t.hooks.is_empty());
            let hook_outputs = outputs
                .iter()
                .filter(|o| o.to.anchor() == Anchor::Hooks)
                .count();
            if count != usize::from(has_hooks) || hook_outputs != usize::from(has_hooks) {
                out.push(
                    Diagnostic::error(
                        "manifest/tool-outputs",
                        if has_hooks {
                            "a tool with hooks has exactly one `hooks` entry output per client"
                        } else {
                            "a tool without hooks writes nothing; use `outputs = []`"
                        },
                    )
                    .at(format!("{base}.outputs")),
                );
            }
        }
        _ => {}
    }
}

fn strategy_name(strategy: Strategy) -> &'static str {
    match strategy {
        Strategy::Copy => "copy",
        Strategy::Render => "render",
        Strategy::Block => "block",
        Strategy::Entry => "entry",
    }
}

/// Validates a bundle on its own.
#[must_use]
pub fn bundle(b: &Bundle) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    if b.maintainers.is_empty() {
        out.push(
            Diagnostic::error("bundle/maintainers", "at least one maintainer is required")
                .at("maintainers"),
        );
    }
    let lowered: Vec<String> = b
        .maintainers
        .iter()
        .map(|l| l.as_str().to_ascii_lowercase())
        .collect();
    push_duplicates(&mut out, "bundle/maintainers", "maintainers", &lowered);
    if b.assets.is_empty() {
        out.push(
            Diagnostic::error("bundle/assets", "a bundle needs at least one asset").at("assets"),
        );
    }
    if b.assets.contains_key(&b.id) {
        out.push(Diagnostic::error("bundle/assets", "a bundle cannot contain itself").at("assets"));
    }
    out
}

/// Validates a project configuration on its own.
#[must_use]
pub fn project_config(c: &ProjectConfig) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    if c.clients.is_empty() {
        out.push(
            Diagnostic::error("config/clients", "at least one client is required").at("clients"),
        );
    }
    push_duplicates(&mut out, "config/clients", "clients", &c.clients);
    push_duplicates(&mut out, "config/bundles", "bundles", &c.bundles);
    push_duplicates(&mut out, "config/tags", "tags", &c.tags);
    push_duplicates(&mut out, "config/exclude", "exclude", &c.exclude);
    let project_clients: BTreeSet<ClientId> = c.clients.iter().copied().collect();
    for (id, selection) in &c.assets {
        if c.exclude.contains(id) {
            out.push(
                Diagnostic::error(
                    "config/exclude",
                    format!("`{id}` is both selected and excluded"),
                )
                .at(format!("assets.{id}")),
            );
        }
        let Some(options) = selection.options() else {
            continue;
        };
        if let Some(clients) = &options.clients {
            if clients.is_empty() {
                out.push(
                    Diagnostic::error(
                        "config/asset-clients",
                        "omit `clients` instead of listing none",
                    )
                    .at(format!("assets.{id}.clients")),
                );
            }
            push_duplicates(
                &mut out,
                "config/asset-clients",
                &format!("assets.{id}.clients"),
                clients,
            );
            for client in clients.iter().filter(|c| !project_clients.contains(c)) {
                out.push(
                    Diagnostic::error(
                        "config/asset-clients",
                        format!("`{client}` is not one of the project's clients"),
                    )
                    .at(format!("assets.{id}.clients")),
                );
            }
        }
    }
    out
}

/// Validates a lockfile on its own.
#[must_use]
pub fn lockfile(l: &Lockfile) -> Vec<Diagnostic> {
    let mut out = Vec::new();
    push_duplicates(
        &mut out,
        "lock/assets",
        "asset",
        l.assets.iter().map(|a| &a.id),
    );
    let mut by_path: BTreeMap<&str, Vec<Owner<'_>>> = BTreeMap::new();
    for asset in &l.assets {
        for (i, o) in asset.outputs.iter().enumerate() {
            let field = format!("asset.{}.output[{i}]", asset.id);
            if (o.unit == Unit::Entry) != o.key.is_some() {
                out.push(
                    Diagnostic::error(
                        "lock/output-key",
                        "an output has a `key` exactly when its unit is `entry`",
                    )
                    .at(&field),
                );
            }
            if o.clients.is_empty() {
                out.push(
                    Diagnostic::error(
                        "lock/output-clients",
                        "an output serves at least one client",
                    )
                    .at(&field),
                );
            }
            by_path
                .entry(o.path.as_str())
                .or_default()
                .push((&asset.id, o, field));
        }
    }
    for (path, owners) in by_path {
        ownership(path, &owners, &mut out);
    }
    out
}

/// One owner of a path in a lockfile: the asset, its output, and the output's field name.
type Owner<'a> = (&'a crate::AssetId, &'a crate::LockedOutput, String);

/// Checks one path's owners: one `file`, or blocks (one per asset), or entries with distinct keys.
fn ownership(path: &str, owners: &[Owner<'_>], out: &mut Vec<Diagnostic>) {
    let units: BTreeSet<Unit> = owners.iter().map(|(_, o, _)| o.unit).collect();
    if units.len() > 1 {
        out.push(
            Diagnostic::error(
                "lock/output-owner",
                format!("`{path}` mixes file, block and entry units"),
            )
            .at(&owners[0].2),
        );
        return;
    }
    let mut seen: BTreeMap<String, &crate::AssetId> = BTreeMap::new();
    for (id, o, field) in owners {
        let unit_id = match o.unit {
            Unit::File => String::new(),
            Unit::Block => id.as_str().to_owned(),
            Unit::Entry => o
                .key
                .as_ref()
                .map(|k| k.as_str().to_owned())
                .unwrap_or_default(),
        };
        if let Some(first) = seen.insert(unit_id, id) {
            let message = match o.unit {
                Unit::File => {
                    format!("`{path}` is a whole-file output owned by both `{first}` and `{id}`")
                }
                Unit::Block => format!("`{path}` has two blocks for `{id}`"),
                Unit::Entry => {
                    format!("the same entry in `{path}` is owned by both `{first}` and `{id}`")
                }
            };
            out.push(Diagnostic::error("lock/output-owner", message).at(field));
        }
    }
}
