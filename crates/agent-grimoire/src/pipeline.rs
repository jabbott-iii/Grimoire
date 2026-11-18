//! Resolve → render → plan → apply, shared by `plan`, `apply`, `update` and `uninstall`.

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_adapters::AVAILABLE;
use agent_grimoire_install::apply::{self, ApplyError, FileOutcome, Options, Stopped};
use agent_grimoire_install::plan::{self, Action, Desired, FilePlan, Plan};
use agent_grimoire_model::{
    ClientId, Diagnostic, LockedAsset, Lockfile, ProjectConfig, RelPath, Unit,
};
use agent_grimoire_resolve::{Request, Update, resolve};
use serde_json::{Value, json};

use crate::out::{Exit, Out};
use crate::project::{self, Project, fs_exit};

/// A computed plan and the lock that applying it would produce.
pub(crate) struct Planned {
    pub(crate) plan: Plan,
    pub(crate) old: Lockfile,
    pub(crate) old_text: Option<String>,
    pub(crate) assets: Vec<LockedAsset>,
}

impl Planned {
    /// The lock after applying, keeping old records for `deferred` files.
    pub(crate) fn next(&self, deferred: &BTreeSet<&RelPath>) -> Lockfile {
        apply::next_lock(&self.old, self.assets.clone(), &self.plan, deferred)
    }
}

/// Resolves and renders the project, then plans against the disk.
pub(crate) fn prepare(
    out: &mut Out<'_, '_>,
    project: &Project,
    config: &ProjectConfig,
    update: &Update,
) -> Result<Planned, Exit> {
    let (old, old_text) = project.lock(out)?;
    let source = out.ctx.source;
    let version = out.ctx.version.clone();
    let resolution = match resolve(
        source,
        &Request {
            config,
            lock: Some(&old),
            cli_version: &version,
            update,
            available_clients: &AVAILABLE,
        },
    ) {
        Ok(r) => r,
        Err(diagnostics) => {
            out.diags(diagnostics);
            return Err(Exit::Invalid);
        }
    };
    out.diags(resolution.warnings.clone());
    let rendered = match agent_grimoire_adapters::render(&resolution) {
        Ok(r) => r,
        Err(diagnostics) => {
            out.diags(diagnostics);
            return Err(Exit::Invalid);
        }
    };
    out.diags(rendered.warnings);
    fail_closed_hooks(out, &resolution);
    let assets = resolution
        .assets
        .values()
        .map(|r| LockedAsset {
            id: r.id().clone(),
            kind: r.asset.manifest.kind,
            version: r.asset.manifest.version.clone(),
            source: source.id(),
            manifest: r.asset.digest.clone(),
            selected_by: r.selected_by.iter().cloned().collect(),
            allow: r.allow.clone(),
            outputs: Vec::new(),
        })
        .collect();
    let planned = finish(out, project, rendered.desired, old, old_text, assets)?;
    // Codex reads all of AGENTS.md, whichever client each block is for.
    if config.clients.contains(&ClientId::Codex) {
        codex_budget(out, &planned.plan);
        codex_override(out, project);
    }
    Ok(planned)
}

/// Codex reads `AGENTS.override.md` instead of `AGENTS.md` in a directory that
/// has both, even an empty one (`CODEX-P4`).
fn codex_override(out: &mut Out<'_, '_>, project: &Project) {
    if project.exists(CODEX_OVERRIDE) {
        out.diag(Diagnostic::warning(
            "plan/codex-override",
            format!("{CODEX_OVERRIDE} exists, so Codex reads it instead of AGENTS.md and none of the rules grimoire writes there"),
        ));
    }
}

/// The file Codex prefers to `AGENTS.md`.
pub(crate) const CODEX_OVERRIDE: &str = "AGENTS.override.md";

/// Codex reads at most `project_doc_max_bytes` (32 KiB by default) of project
/// instructions, shared by every `AGENTS.md` from the repository root to the
/// working directory, and truncates the rest (`CODEX-P4`).
const CODEX_BUDGET: usize = 32 * 1024;

/// Warns when the planned `AGENTS.md` uses most of Codex's budget on its own
/// (`architecture.md` §11.5).
fn codex_budget(out: &mut Out<'_, '_>, plan: &Plan) {
    let Some(size) = plan
        .files
        .iter()
        .find(|f| f.path.as_str() == agent_grimoire_adapters::agents_md::AGENTS_MD)
        .and_then(|f| f.after.as_ref().map(Vec::len))
    else {
        return;
    };
    if size * 4 >= CODEX_BUDGET * 3 {
        out.diag(Diagnostic::warning(
            "plan/codex-instructions-size",
            format!(
                "AGENTS.md would be about {} KiB; Codex reads at most 32 KiB of project instructions in total, including nested AGENTS.md files, and cuts off the rest (`project_doc_max_bytes`)",
                size.div_ceil(1024)
            ),
        ));
    }
}

/// Warns when a selected tool's hooks fail closed (`on-failure = "block"`) and
/// its binary is not on `PATH`: the client would then block every event those
/// hooks cover, such as every tool call, until the binary is installed.
fn fail_closed_hooks(out: &mut Out<'_, '_>, resolution: &agent_grimoire_resolve::Resolution<'_>) {
    for resolved in resolution.assets.values() {
        let Some(tool) = &resolved.asset.manifest.tool else {
            continue;
        };
        let blocking = tool
            .hooks
            .iter()
            .any(|h| h.on_failure == agent_grimoire_model::HookFailure::Block);
        if !blocking || on_path(out.ctx.path.as_deref(), tool.binary.as_str()) {
            continue;
        }
        out.diag(Diagnostic::warning(
            "plan/hook-binary-missing",
            format!(
                "`{}`'s hooks fail closed, and `{}` is not on PATH: until it is installed, the client blocks every event those hooks cover (for Claude Code, every tool call or prompt). Install it first; to recover a blocked Claude Code session, start it with `claude --settings '{{\"disableAllHooks\": true}}'`",
                resolved.id(),
                tool.binary
            ),
        ));
    }
}

/// Whether `binary` is a file in one of `path`'s directories. Never runs it.
pub(crate) fn on_path(path: Option<&std::ffi::OsStr>, binary: &str) -> bool {
    let Some(path) = path else { return false };
    std::env::split_paths(path).any(|dir| {
        let names = if cfg!(windows) {
            vec![format!("{binary}.exe"), binary.to_owned()]
        } else {
            vec![binary.to_owned()]
        };
        names.iter().any(|n| dir.join(n).is_file())
    })
}

/// Plans removing everything the lock records.
pub(crate) fn prepare_uninstall(out: &mut Out<'_, '_>, project: &Project) -> Result<Planned, Exit> {
    let (old, old_text) = project.lock(out)?;
    finish(out, project, Vec::new(), old, old_text, Vec::new())
}

fn finish(
    out: &mut Out<'_, '_>,
    project: &Project,
    desired: Vec<Desired>,
    old: Lockfile,
    old_text: Option<String>,
    assets: Vec<LockedAsset>,
) -> Result<Planned, Exit> {
    let mut current = BTreeMap::new();
    for path in plan::paths(&desired, &old) {
        match project.root.read(&path) {
            Ok(bytes) => {
                current.insert(path, bytes);
            }
            Err(e) => return Err(fs_exit(out, &e)),
        }
    }
    let plan = match plan::plan(desired, &old, &current) {
        Ok(plan) => plan,
        Err(diagnostics) => {
            out.diags(diagnostics);
            return Err(Exit::Invalid);
        }
    };
    Ok(Planned {
        plan,
        old,
        old_text,
        assets,
    })
}

fn unit_label(unit: Unit, key: Option<&str>, owner: &str) -> String {
    match unit {
        Unit::File => String::new(),
        Unit::Block => format!(" [block {owner}]"),
        Unit::Entry => format!(" [{}]", key.unwrap_or_default()),
    }
}

fn reason(action: Action) -> Option<&'static str> {
    match action {
        Action::Conflict(c) => Some(c.describe()),
        Action::Adopt => Some("already as desired on disk"),
        Action::Gone => Some("already removed"),
        _ => None,
    }
}

/// A unified diff of one file, or a note for binary content.
fn diff(file: &FilePlan) -> Option<String> {
    if file.unreadable.is_some() || file.before == file.after {
        return None;
    }
    let before = file.before.as_deref().unwrap_or_default();
    let after = file.after.as_deref().unwrap_or_default();
    match (std::str::from_utf8(before), std::str::from_utf8(after)) {
        (Ok(a), Ok(b)) => {
            let path = file.path.as_str();
            Some(
                similar::TextDiff::from_lines(a, b)
                    .unified_diff()
                    .context_radius(3)
                    .header(&format!("a/{path}"), &format!("b/{path}"))
                    .to_string(),
            )
        }
        _ => Some(format!("binary content of {} differs\n", file.path)),
    }
}

/// Counts of unit actions.
#[derive(Default)]
struct Summary {
    create: usize,
    update: usize,
    delete: usize,
    conflict: usize,
    unchanged: usize,
    other: usize,
}

impl Summary {
    fn of(plan: &Plan) -> Self {
        let mut s = Self::default();
        for file in &plan.files {
            if file.unreadable.is_some() {
                s.conflict += 1;
            }
            for unit in &file.units {
                match unit.action {
                    Action::Create => s.create += 1,
                    Action::Update => s.update += 1,
                    Action::Delete => s.delete += 1,
                    Action::Conflict(_) => s.conflict += 1,
                    Action::Unchanged => s.unchanged += 1,
                    Action::Adopt | Action::Gone => s.other += 1,
                }
            }
        }
        s
    }

    fn json(&self) -> Value {
        json!({
            "create": self.create,
            "update": self.update,
            "delete": self.delete,
            "conflict": self.conflict,
            "unchanged": self.unchanged,
            "record-only": self.other,
        })
    }

    fn text(&self) -> String {
        format!(
            "{} to create, {} to update, {} to delete, {} conflicting, {} unchanged",
            self.create, self.update, self.delete, self.conflict, self.unchanged
        )
    }
}

/// Prints the plan and sets the JSON result. Returns whether the lock would change.
pub(crate) fn show(out: &mut Out<'_, '_>, planned: &Planned, diffs: bool) -> Result<bool, Exit> {
    let next = planned.next(&BTreeSet::new());
    let next_text = project::render_lock(out, &next)?;
    let lock_changes = project::lock_differs(planned.old_text.as_deref(), &next_text);
    let mut files = Vec::new();
    for file in &planned.plan.files {
        let mut units = Vec::new();
        for unit in &file.units {
            let clients: Vec<&str> = unit.clients.iter().map(|c| c.as_str()).collect();
            let label = unit_label(
                unit.unit,
                unit.key
                    .as_ref()
                    .map(agent_grimoire_model::EntryKey::as_str),
                unit.owner.name(),
            );
            if unit.action != Action::Unchanged {
                let why = reason(unit.action)
                    .map(|r| format!(": {r}"))
                    .unwrap_or_default();
                out.line(format!(
                    "  {:<9} {}{label}  {} ({}){why}",
                    unit.action.verb(),
                    file.path,
                    unit.owner,
                    clients.join(", ")
                ));
            }
            units.push(json!({
                "owner": unit.owner.name(),
                "unit": match unit.unit { Unit::File => "file", Unit::Block => "block", Unit::Entry => "entry" },
                "key": unit.key.as_ref().map(agent_grimoire_model::EntryKey::as_str),
                "clients": clients,
                "action": unit.action.verb(),
                "reason": reason(unit.action),
            }));
        }
        if let Some(why) = &file.unreadable {
            out.line(format!(
                "  conflict  {}  cannot be edited: {why}",
                file.path
            ));
        }
        let diff = diff(file);
        files.push(json!({
            "path": file.path.as_str(),
            "unreadable": file.unreadable,
            "changes": file.changes_disk(),
            "units": units,
            "diff": diff,
        }));
    }
    let summary = Summary::of(&planned.plan);
    out.line(format!("plan: {}", summary.text()));
    if lock_changes && !planned.plan.has_changes() {
        out.line(if planned.old_text.is_none() {
            "plan: grimoire.lock would be created"
        } else {
            "plan: grimoire.lock would be updated"
        });
    }
    if diffs && !out.json {
        for file in &planned.plan.files {
            if let Some(d) = diff(file) {
                out.line(d.trim_end());
            }
        }
    }
    out.result(json!({
        "summary": summary.json(),
        "lock-changes": lock_changes,
        "files": files,
    }));
    Ok(lock_changes)
}

/// Applies a plan and writes the lock last.
/// Which command is applying, so the advice after a conflict names flags it has.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Caller {
    /// `apply` or `update`.
    Apply,
    /// `add --apply` or `remove --apply`: grimoire.toml was already written.
    AfterEdit,
    /// `uninstall`.
    Uninstall,
}

fn conflict_advice(caller: Caller, plan: &Plan) -> String {
    if plan.files.iter().any(|f| f.unreadable.is_some()) {
        return "nothing was written; fix the files reported above by hand (--force cannot edit a file grimoire cannot read)".to_owned();
    }
    match caller {
        Caller::Apply => "nothing was written; resolve the conflicts above, or re-run with --force to overwrite or --write-new to write <path>.new files".to_owned(),
        Caller::AfterEdit => "grimoire.toml was updated, but no other file was written; resolve the conflicts above, then run `grimoire apply` (or `grimoire apply --force` / `--write-new`)".to_owned(),
        Caller::Uninstall => "nothing was removed; resolve the conflicts above, or re-run with --force to remove the edited content too".to_owned(),
    }
}

/// Applies a plan and writes the lock last. After a failure partway through, the
/// lock still records the files already written.
pub(crate) fn run_apply(
    out: &mut Out<'_, '_>,
    project: &Project,
    planned: &Planned,
    options: Options,
    caller: Caller,
) -> Exit {
    let (report, failure) = match apply::apply(&project.root, &planned.plan, options) {
        Ok(report) => (report, None),
        Err(Stopped {
            error: ApplyError::Conflicts,
            ..
        }) => {
            report_conflicts(out, &planned.plan);
            let advice = conflict_advice(caller, &planned.plan);
            return out.fail(Exit::Conflict, "apply/conflict", advice);
        }
        Err(Stopped {
            error: e @ ApplyError::SidecarExists(_),
            ..
        }) => {
            return out.fail(
                Exit::Conflict,
                "apply/sidecar-exists",
                format!("{e}; nothing was written"),
            );
        }
        Err(Stopped { error, report }) => (report, Some(error)),
    };
    let deferred = if failure.is_some() {
        apply::not_done(&planned.plan, &report)
    } else {
        report.deferred()
    };
    let next = planned.next(&deferred);
    let mut files = Vec::new();
    for (path, outcome) in &report.files {
        let (verb, extra) = match outcome {
            FileOutcome::Written => ("wrote", None),
            FileOutcome::Deleted => ("deleted", None),
            FileOutcome::WroteNew(new) => ("wrote-new", Some(new.as_str().to_owned())),
            FileOutcome::Deferred => ("skipped", None),
        };
        match &extra {
            Some(new) => out.line(format!(
                "  {verb:<9} {new}  (conflict in {path}; review and merge it)"
            )),
            None => out.line(format!("  {verb:<9} {path}")),
        }
        files.push(json!({ "path": path.as_str(), "outcome": verb, "new-file": extra }));
    }
    let wrote_lock = match project.write_lock(out, &next, planned.old_text.as_deref()) {
        Ok(wrote) => wrote,
        Err(exit) => return exit,
    };
    out.result(json!({ "files": files, "lock-written": wrote_lock }));
    if let Some(error) = failure {
        let written = report.files.len();
        let note = format!(
            "; {written} files were written before it stopped and grimoire.lock records them, so running again continues from there"
        );
        return match error {
            ApplyError::Changed(path) => out.fail(
                Exit::Conflict,
                "apply/changed",
                format!("`{path}` changed while grimoire was working{note}"),
            ),
            ApplyError::Fs(e) => {
                let exit = fs_exit(out, &e);
                out.note(format!("apply: stopped{note}"));
                exit
            }
            ApplyError::Conflicts | ApplyError::SidecarExists(_) => Exit::Conflict,
        };
    }
    let changed = report.files.len();
    // `uninstall` reports the lock itself, once it has deleted it.
    let lock_note = match (wrote_lock, caller, planned.old_text.is_some()) {
        (false, _, _) | (true, Caller::Uninstall, _) => "",
        (true, _, false) => ", grimoire.lock created",
        (true, _, true) => ", grimoire.lock updated",
    };
    out.line(format!("apply: {changed} files changed{lock_note}"));
    if deferred.is_empty() {
        Exit::Ok
    } else {
        report_conflicts(out, &planned.plan);
        out.fail(
            Exit::Conflict,
            "apply/conflict-remains",
            format!("{} conflicting files were left alone", deferred.len()),
        )
    }
}

pub(crate) fn report_conflicts(out: &mut Out<'_, '_>, plan: &Plan) {
    for file in &plan.files {
        if let Some(why) = &file.unreadable {
            out.diag(
                Diagnostic::error(
                    "plan/unreadable",
                    format!("`{}` cannot be edited: {why}", file.path),
                )
                .in_file(file.path.as_str()),
            );
        }
        for unit in &file.units {
            if let Action::Conflict(c) = unit.action {
                let label = unit_label(
                    unit.unit,
                    unit.key
                        .as_ref()
                        .map(agent_grimoire_model::EntryKey::as_str),
                    unit.owner.name(),
                );
                out.diag(
                    Diagnostic::error(
                        "plan/conflict",
                        format!("`{}`{label} ({}): {}", file.path, unit.owner, c.describe()),
                    )
                    .in_file(file.path.as_str()),
                );
            }
        }
    }
}
