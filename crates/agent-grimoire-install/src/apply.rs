//! Applying a plan, building the next lock, and verifying a lock against disk.
//!
//! The design is in
//! [`docs/subsystems/install.md`, Desired state, plan, apply](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#desired-state-plan-apply) and
//! [`docs/subsystems/install.md`, Drift and conflicts](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#drift-and-conflicts).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use agent_grimoire_model::{LockedAsset, LockedOutput, Lockfile, RelPath, Sha256Digest, Unit};

use crate::block::Document;
use crate::entry::{EntryDoc, EntryFormat};
use crate::fs::{FsError, Root};
use crate::hash;
use crate::plan::{FilePlan, Owner, Plan};

/// How conflicts are handled.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    /// Overwrite conflicting content.
    pub force: bool,
    /// Write each conflicting file's would-be content to `<path>.new` instead.
    pub write_new: bool,
}

/// What happened to one file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileOutcome {
    /// Written.
    Written,
    /// Deleted (and any directories it left empty).
    Deleted,
    /// Left alone because it conflicts; its would-be content went to this path.
    WroteNew(RelPath),
    /// Left alone because it conflicts, and there was nothing to write beside it.
    Deferred,
}

/// The result of applying a plan.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    /// Files that changed or were deferred, in plan order.
    pub files: Vec<(RelPath, FileOutcome)>,
}

impl Report {
    /// Files left alone because of conflicts.
    #[must_use]
    pub fn deferred(&self) -> BTreeSet<&RelPath> {
        self.files
            .iter()
            .filter(|(_, o)| matches!(o, FileOutcome::WroteNew(_) | FileOutcome::Deferred))
            .map(|(p, _)| p)
            .collect()
    }
}

/// Why nothing (or not everything) was applied.
#[derive(Debug)]
pub enum ApplyError {
    /// The plan has conflicts and neither `--force` nor `--write-new` was given,
    /// or a file cannot be edited at all. Nothing was written.
    Conflicts,
    /// `--write-new` would replace a `<path>.new` that holds something else, such
    /// as a merge in progress. Nothing was written.
    SidecarExists(RelPath),
    /// A file changed between planning and writing.
    Changed(RelPath),
    /// A filesystem operation failed or was refused.
    Fs(FsError),
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::Conflicts => {
                f.write_str("conflicts with content grimoire does not own; nothing was written")
            }
            ApplyError::SidecarExists(path) => write!(
                f,
                "`{path}` already exists with different content; move it away or delete it, then run again"
            ),
            ApplyError::Changed(path) => write!(
                f,
                "`{path}` changed while grimoire was working; run the command again"
            ),
            ApplyError::Fs(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ApplyError {}

impl From<FsError> for ApplyError {
    fn from(e: FsError) -> Self {
        ApplyError::Fs(e)
    }
}

/// An apply that stopped. `report` lists what was done before it stopped, so the
/// caller can still record those files in the lock.
#[derive(Debug)]
pub struct Stopped {
    /// Why it stopped.
    pub error: ApplyError,
    /// What was already written or deleted.
    pub report: Report,
}

/// The sidecar path `--write-new` writes for `path`.
#[must_use]
pub fn new_path(path: &RelPath) -> Option<RelPath> {
    RelPath::new(format!("{path}.new")).ok()
}

/// Applies `plan` under `root`.
///
/// Every precondition is checked before anything is written: with conflicts and
/// neither option set, with a file that cannot be edited, or with a `--write-new`
/// sidecar that is in the way, nothing is written.
///
/// # Errors
///
/// See [`ApplyError`]. After a later failure, [`Stopped::report`] lists the files
/// already changed.
pub fn apply(root: &Root, plan: &Plan, options: Options) -> Result<Report, Stopped> {
    let stop = |error: ApplyError, report: Report| Stopped { error, report };
    if plan.files.iter().any(|f| f.unreadable.is_some())
        || (plan.has_conflicts() && !options.force && !options.write_new)
    {
        return Err(stop(ApplyError::Conflicts, Report::default()));
    }
    // Sidecars are checked up front, so a refusal still means nothing was written.
    let mut sidecars: BTreeMap<&RelPath, (RelPath, bool)> = BTreeMap::new();
    if options.write_new && !options.force {
        for file in plan.files.iter().filter(|f| f.has_conflict()) {
            let (Some(after), Some(sidecar)) = (&file.after, new_path(&file.path)) else {
                continue;
            };
            match root.read(&sidecar) {
                Ok(None) => {
                    sidecars.insert(&file.path, (sidecar, true));
                }
                Ok(Some(existing)) if existing == *after => {
                    sidecars.insert(&file.path, (sidecar, false));
                }
                Ok(Some(_)) => {
                    return Err(stop(ApplyError::SidecarExists(sidecar), Report::default()));
                }
                Err(e) => return Err(stop(ApplyError::Fs(e), Report::default())),
            }
        }
    }
    let mut report = Report::default();
    for file in &plan.files {
        let defer = file.has_conflict() && !options.force;
        if !defer && !file.changes_disk() {
            continue;
        }
        let current = match root.read(&file.path) {
            Ok(current) => current,
            Err(e) => return Err(stop(ApplyError::Fs(e), report)),
        };
        if current != file.before {
            return Err(stop(ApplyError::Changed(file.path.clone()), report));
        }
        if defer {
            let outcome = match sidecars.get(&file.path) {
                Some((sidecar, write)) => {
                    // An identical sidecar from an earlier run is left as it is.
                    if *write {
                        let after = file.after.as_deref().unwrap_or_default();
                        if let Err(e) = root.write_like(sidecar, after, Some(&file.path)) {
                            return Err(stop(ApplyError::Fs(e), report));
                        }
                    }
                    FileOutcome::WroteNew(sidecar.clone())
                }
                None => FileOutcome::Deferred,
            };
            report.files.push((file.path.clone(), outcome));
            continue;
        }
        let result = match &file.after {
            Some(after) => root.write(&file.path, after).map(|()| FileOutcome::Written),
            None => root.remove(&file.path).map(|()| FileOutcome::Deleted),
        };
        match result {
            Ok(outcome) => report.files.push((file.path.clone(), outcome)),
            Err(e) => return Err(stop(ApplyError::Fs(e), report)),
        }
    }
    Ok(report)
}

/// Files the plan would change that `report` does not show as written or
/// deleted: after a stopped apply, these keep their old lock records.
#[must_use]
pub fn not_done<'p>(plan: &'p Plan, report: &Report) -> BTreeSet<&'p RelPath> {
    let done: BTreeSet<&RelPath> = report
        .files
        .iter()
        .filter(|(_, o)| matches!(o, FileOutcome::Written | FileOutcome::Deleted))
        .map(|(p, _)| p)
        .collect();
    plan.files
        .iter()
        .filter(|f| (f.changes_disk() || f.has_conflict()) && !done.contains(&f.path))
        .map(|f| &f.path)
        .collect()
}

/// The lock after `plan` has been applied (or would have been, for `plan`).
///
/// `assets` carries each resolved asset's lock entry without outputs. Files in
/// `deferred` keep their old records. An asset that is no longer resolved but
/// still owns deferred content stays in the lock until that content is resolved.
#[must_use]
pub fn next_lock(
    old: &Lockfile,
    assets: Vec<LockedAsset>,
    plan: &Plan,
    deferred: &BTreeSet<&RelPath>,
) -> Lockfile {
    let mut records: BTreeMap<Owner, Vec<LockedOutput>> = BTreeMap::new();
    for file in &plan.files {
        if deferred.contains(&file.path) {
            // Left alone: keep exactly what the old lock says about this file, even
            // when the planner could not read it and has no units for it.
            for asset in &old.assets {
                for o in asset.outputs.iter().filter(|o| o.path == file.path) {
                    records
                        .entry(Owner::Asset(asset.id.clone()))
                        .or_default()
                        .push(o.clone());
                }
            }
            for o in old.outputs.iter().filter(|o| o.path == file.path) {
                records.entry(Owner::Project).or_default().push(o.clone());
            }
            continue;
        }
        for unit in &file.units {
            if let Some(record) = unit.applied_record(&file.path) {
                records.entry(unit.owner.clone()).or_default().push(record);
            }
        }
    }
    let mut lock = Lockfile::new();
    let mut seen = BTreeSet::new();
    for mut asset in assets {
        asset.outputs = records
            .remove(&Owner::Asset(asset.id.clone()))
            .unwrap_or_default();
        seen.insert(asset.id.clone());
        lock.assets.push(asset);
    }
    for old_asset in &old.assets {
        if seen.contains(&old_asset.id) {
            continue;
        }
        if let Some(outputs) = records.remove(&Owner::Asset(old_asset.id.clone())) {
            let mut kept = old_asset.clone();
            kept.outputs = outputs;
            lock.assets.push(kept);
        }
    }
    lock.outputs = records.remove(&Owner::Project).unwrap_or_default();
    lock.canonicalize();
    lock
}

/// One locked output whose content on disk is not what the lock records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mismatch {
    /// Who owns it.
    pub owner: Owner,
    /// The file.
    pub path: RelPath,
    /// The unit.
    pub unit: Unit,
    /// Why it does not match.
    pub reason: String,
}

/// Checks every output recorded in `lock` against the disk under `root`.
///
/// # Errors
///
/// Returns a filesystem error if a file cannot be read safely.
pub fn verify(root: &Root, lock: &Lockfile) -> Result<Vec<Mismatch>, FsError> {
    let mut by_path: BTreeMap<&RelPath, Vec<(Owner, &LockedOutput)>> = BTreeMap::new();
    for asset in &lock.assets {
        for o in &asset.outputs {
            by_path
                .entry(&o.path)
                .or_default()
                .push((Owner::Asset(asset.id.clone()), o));
        }
    }
    for o in &lock.outputs {
        by_path
            .entry(&o.path)
            .or_default()
            .push((Owner::Project, o));
    }
    let mut out = Vec::new();
    for (path, outputs) in by_path {
        let bytes = root.read(path)?;
        let text = bytes.as_deref().map(std::str::from_utf8);
        for (owner, o) in outputs {
            let found: Result<Option<Sha256Digest>, String> = match (o.unit, &bytes, &text) {
                (_, None, _) => Ok(None),
                (Unit::File, Some(b), _) => Ok(Some(hash::content(b))),
                (_, Some(_), Some(Err(_))) => Err("it is not UTF-8 text".to_owned()),
                (Unit::Block, _, Some(Ok(t))) => Document::parse(t)
                    .map(|d| d.get(owner.name()).map(|b| hash::content(b.as_bytes())))
                    .map_err(|e| format!("its managed-block markers are broken ({e})")),
                (Unit::Entry, _, Some(Ok(t))) => match &o.key {
                    Some(key) => EntryDoc::parse(EntryFormat::for_path(path), Some(t))
                        .and_then(|d| d.find(key, Some(&o.hash), None))
                        .map_err(|e| e.to_string()),
                    None => Err("the lock records an entry without a key".to_owned()),
                },
                (_, Some(_), None) => Ok(None),
            };
            let reason = match found {
                Ok(Some(h)) if h == o.hash => continue,
                Ok(Some(_)) => "edited since grimoire wrote it".to_owned(),
                Ok(None) => "missing".to_owned(),
                Err(e) => e,
            };
            out.push(Mismatch {
                owner,
                path: path.clone(),
                unit: o.unit,
                reason,
            });
        }
    }
    Ok(out)
}

/// The would-be content of `file`, for `plan` to diff against `before`.
#[must_use]
pub fn after_text(file: &FilePlan) -> Option<&str> {
    file.after
        .as_deref()
        .and_then(|b| std::str::from_utf8(b).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plan::FilePlan;

    fn rel(s: &str) -> RelPath {
        RelPath::new(s).unwrap()
    }

    fn file(path: &str, after: &[u8]) -> FilePlan {
        FilePlan {
            path: rel(path),
            before: None,
            after: Some(after.to_vec()),
            units: Vec::new(),
            unreadable: None,
        }
    }

    #[test]
    fn a_stopped_apply_reports_what_it_wrote() {
        let tmp = tempfile::tempdir().unwrap();
        let root = Root::open(tmp.path()).unwrap();
        let plan = Plan {
            files: vec![file("a.md", b"a"), file("b/c.md", b"c")],
        };
        // Something appears in the way after planning.
        std::fs::write(tmp.path().join("b"), "a file").unwrap();
        let stopped = apply(&root, &plan, Options::default()).unwrap_err();
        assert!(matches!(
            stopped.error,
            ApplyError::Fs(FsError::InTheWay { .. })
        ));
        assert_eq!(stopped.report.files, [(rel("a.md"), FileOutcome::Written)]);
        assert_eq!(
            not_done(&plan, &stopped.report),
            BTreeSet::from([&rel("b/c.md")])
        );
    }
}
