//! The planner: desired outputs + lock + disk → actions.
//!
//! The design is in
//! [`docs/subsystems/install.md`, Desired state, plan, apply](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#desired-state-plan-apply),
//! [`docs/subsystems/install.md`, Ownership units and merge strategies](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#ownership-units-and-merge-strategies) and
//! [`docs/subsystems/install.md`, Drift and conflicts](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#drift-and-conflicts).
//!
//! [`plan`] is pure. It is given the current bytes of every path involved and
//! decides, for every owned unit, what applying would do and whether that would
//! overwrite something the user changed. It also computes each file's content
//! with every unit applied, which `apply --force` writes, `apply --write-new`
//! writes beside the original, and `plan` shows as a diff.
//!
//! For each unit it compares three hashes: on disk (D), locked (L) and desired (N).
//!
//! | Situation | Action |
//! |---|---|
//! | D = N | unchanged (or "adopt" when the lock does not record N yet) |
//! | D = L, N ≠ L | update |
//! | D ≠ L, D ≠ N | conflict: edited since Grimoire wrote it |
//! | locked unit missing on disk | conflict: removed by the user |
//! | present on disk but not locked | conflict: unmanaged content in the way |
//! | locked, no longer desired, D = L | delete |
//! | locked, no longer desired, D ≠ L | conflict: edited, so not deleted |

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use agent_grimoire_model::{
    AssetId, ClientId, Diagnostic, EntryKey, LockedOutput, Lockfile, RelPath, Sha256Digest, Unit,
};

use crate::block::Document;
use crate::entry::{self, EntryDoc, EntryFormat};
use crate::hash;

/// Who owns an output.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Owner {
    /// An installed asset.
    Asset(AssetId),
    /// Grimoire itself, e.g. the `CLAUDE.md` block that imports `AGENTS.md`.
    Project,
}

impl Owner {
    /// The name used in managed-block markers.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Owner::Asset(id) => id.as_str(),
            Owner::Project => Lockfile::PROJECT_OWNER,
        }
    }
}

impl fmt::Display for Owner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The content of one desired output.
#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    /// A whole file.
    File(Vec<u8>),
    /// The body of a managed block named after the owner.
    Block(String),
    /// A JSON value owned at `key` (see [`crate::entry`]).
    Entry {
        /// Where the value lives.
        key: EntryKey,
        /// The value.
        value: serde_json::Value,
    },
}

impl Content {
    fn unit(&self) -> Unit {
        match self {
            Content::File(_) => Unit::File,
            Content::Block(_) => Unit::Block,
            Content::Entry { .. } => Unit::Entry,
        }
    }

    fn key(&self) -> Option<&EntryKey> {
        match self {
            Content::Entry { key, .. } => Some(key),
            _ => None,
        }
    }

    fn hash(&self) -> Sha256Digest {
        match self {
            Content::File(bytes) => hash::content(bytes),
            // Hash the body as it is written, so the next plan sees D = N.
            Content::Block(body) => hash::content(crate::block::normalize_body(body).as_bytes()),
            Content::Entry { value, .. } => entry::value_hash(value),
        }
    }
}

/// One output an adapter wants on disk.
#[derive(Debug, Clone, PartialEq)]
pub struct Desired {
    /// Who owns it.
    pub owner: Owner,
    /// The clients it serves.
    pub clients: BTreeSet<ClientId>,
    /// Where it goes, relative to the project root.
    pub path: RelPath,
    /// What it holds.
    pub content: Content,
}

/// Why a unit cannot be written without `--force`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Conflict {
    /// Changed on disk since Grimoire wrote it.
    Edited,
    /// Grimoire wrote it, and it is gone from disk.
    Missing,
    /// Something Grimoire does not manage is already there.
    Unmanaged,
    /// No longer wanted, but changed on disk since Grimoire wrote it.
    EditedBeforeDelete,
    /// An array element Grimoire wrote can no longer be found. Elements are found
    /// by their exact value, so an edited element and a removed one look the same.
    NotFound,
    /// An entry in a TOML or JSONC file has comments added inside it, which
    /// updating or removing it would lose. Values are compared without comments.
    Commented,
}

impl Conflict {
    /// A short explanation.
    #[must_use]
    pub fn describe(self) -> &'static str {
        match self {
            Conflict::Edited => "edited since grimoire wrote it",
            Conflict::Missing => "removed since grimoire wrote it",
            Conflict::Unmanaged => "already exists and is not managed by grimoire",
            Conflict::EditedBeforeDelete => "no longer wanted, but edited since grimoire wrote it",
            Conflict::NotFound => {
                "edited or removed since grimoire wrote it (grimoire finds array entries by their exact value; with --force it writes its entry again and leaves any edited copy alone)"
            }
            Conflict::Commented => {
                "has comments added inside it since grimoire wrote it, which updating or removing it would lose (move them out of the entry, or use --force to drop them)"
            }
        }
    }
}

/// What applying does to one unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Write a new unit.
    Create,
    /// Replace the unit Grimoire wrote earlier.
    Update,
    /// Remove a unit that is no longer wanted.
    Delete,
    /// Nothing to do.
    Unchanged,
    /// Already as desired on disk; only the lock changes.
    Adopt,
    /// No longer wanted and already gone; only the lock changes.
    Gone,
    /// Needs `--force` (or `--write-new`).
    Conflict(Conflict),
}

impl Action {
    /// Whether this action changes a file on disk.
    #[must_use]
    pub fn writes(self) -> bool {
        matches!(
            self,
            Action::Create | Action::Update | Action::Delete | Action::Conflict(_)
        )
    }

    /// The verb shown by `plan`.
    #[must_use]
    pub fn verb(self) -> &'static str {
        match self {
            Action::Create => "create",
            Action::Update => "update",
            Action::Delete => "delete",
            Action::Unchanged => "unchanged",
            Action::Adopt => "adopt",
            Action::Gone => "forget",
            Action::Conflict(_) => "conflict",
        }
    }
}

/// The plan for one owned unit.
#[derive(Debug, Clone, PartialEq)]
pub struct UnitPlan {
    /// Who owns it.
    pub owner: Owner,
    /// The clients it serves (desired, or as locked when it is being removed).
    pub clients: BTreeSet<ClientId>,
    /// The ownership unit.
    pub unit: Unit,
    /// For entries, the owned key.
    pub key: Option<EntryKey>,
    /// What applying does.
    pub action: Action,
    /// The desired hash, when the unit is still wanted.
    pub hash: Option<Sha256Digest>,
    /// The lock's record of the unit, if any.
    pub locked: Option<LockedOutput>,
    /// Grimoire created this unit's file (see [`LockedOutput::created_file`]).
    pub created_file: bool,
    /// Containers Grimoire created for this unit (see [`LockedOutput::created_keys`]).
    pub created_keys: Vec<EntryKey>,
}

impl UnitPlan {
    /// The lock record after this unit is applied, if it is still wanted.
    #[must_use]
    pub fn applied_record(&self, path: &RelPath) -> Option<LockedOutput> {
        self.hash.as_ref().map(|hash| LockedOutput {
            clients: self.clients.iter().copied().collect(),
            path: path.clone(),
            unit: self.unit,
            key: self.key.clone(),
            hash: hash.clone(),
            created_file: self.created_file,
            created_keys: self.created_keys.clone(),
        })
    }
}

/// The plan for one file.
#[derive(Debug, Clone, PartialEq)]
pub struct FilePlan {
    /// The file.
    pub path: RelPath,
    /// Its current content (`None`: absent).
    pub before: Option<Vec<u8>>,
    /// Its content with every unit applied, conflicts included (`None`: absent).
    pub after: Option<Vec<u8>>,
    /// Every owned unit in the file.
    pub units: Vec<UnitPlan>,
    /// Why the file cannot be edited at all, even with `--force`: e.g. broken
    /// block markers or invalid JSON. Grimoire never guesses what it owns.
    pub unreadable: Option<String>,
}

impl FilePlan {
    /// Whether any unit conflicts, or the file cannot be edited.
    #[must_use]
    pub fn has_conflict(&self) -> bool {
        self.unreadable.is_some()
            || self
                .units
                .iter()
                .any(|u| matches!(u.action, Action::Conflict(_)))
    }

    /// Whether applying changes the file on disk.
    #[must_use]
    pub fn changes_disk(&self) -> bool {
        self.unreadable.is_none() && self.before != self.after
    }
}

/// The plan for a whole project.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Plan {
    /// Every file involved, sorted by path.
    pub files: Vec<FilePlan>,
}

impl Plan {
    /// Whether anything conflicts.
    #[must_use]
    pub fn has_conflicts(&self) -> bool {
        self.files.iter().any(FilePlan::has_conflict)
    }

    /// Whether applying would change the disk or the lock.
    #[must_use]
    pub fn has_changes(&self) -> bool {
        self.files.iter().any(|f| {
            f.changes_disk()
                || f.units
                    .iter()
                    .any(|u| !matches!(u.action, Action::Unchanged))
        })
    }
}

/// The paths a plan needs to read: every desired path and every locked one.
#[must_use]
pub fn paths(desired: &[Desired], lock: &Lockfile) -> BTreeSet<RelPath> {
    desired
        .iter()
        .map(|d| d.path.clone())
        .chain(lock_outputs(lock).map(|(_, o)| o.path.clone()))
        .collect()
}

fn lock_outputs(lock: &Lockfile) -> impl Iterator<Item = (Owner, &LockedOutput)> {
    lock.assets
        .iter()
        .flat_map(|a| a.outputs.iter().map(|o| (Owner::Asset(a.id.clone()), o)))
        .chain(lock.outputs.iter().map(|o| (Owner::Project, o)))
}

/// Plans `desired` against `lock` and the current content of each path.
///
/// `current` must hold an entry for every path in [`paths`]; `None` means absent.
///
/// # Errors
///
/// Returns diagnostics when the desired outputs contradict each other: two owners
/// of one file, mixed unit kinds on one path, or paths that differ only in case.
pub fn plan(
    desired: Vec<Desired>,
    lock: &Lockfile,
    current: &BTreeMap<RelPath, Option<Vec<u8>>>,
) -> Result<Plan, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    check_desired(&desired, lock, &mut errors);
    if !errors.is_empty() {
        return Err(errors);
    }
    let mut by_path: BTreeMap<RelPath, (Vec<Desired>, Vec<Locked>)> = BTreeMap::new();
    for d in desired {
        by_path.entry(d.path.clone()).or_default().0.push(d);
    }
    for (owner, o) in lock_outputs(lock) {
        by_path
            .entry(o.path.clone())
            .or_default()
            .1
            .push((owner, o.clone()));
    }
    let mut files = Vec::new();
    for (path, (desired, locked)) in by_path {
        let before = current.get(&path).cloned().flatten();
        let unit = desired
            .first()
            .map(|d| d.content.unit())
            .or_else(|| locked.first().map(|(_, o)| o.unit))
            .unwrap_or(Unit::File);
        if locked.iter().any(|(_, o)| o.unit != unit) {
            files.push(FilePlan {
                units: Vec::new(),
                unreadable: Some(format!(
                    "grimoire.lock records a different kind of content at `{path}` than grimoire now writes there"
                )),
                after: before.clone(),
                before,
                path,
            });
            continue;
        }
        files.push(match unit {
            Unit::File => plan_file(path, before, desired, locked),
            Unit::Block => plan_blocks(path, before, desired, locked),
            Unit::Entry => plan_entries(path, before, desired, locked),
        });
    }
    Ok(Plan { files })
}

fn check_desired(desired: &[Desired], lock: &Lockfile, errors: &mut Vec<Diagnostic>) {
    let mut units: BTreeMap<&RelPath, Unit> = BTreeMap::new();
    let mut seen: BTreeSet<(&RelPath, String)> = BTreeSet::new();
    let mut folded: BTreeMap<String, &RelPath> = BTreeMap::new();
    let mut elements: BTreeSet<(&RelPath, String, Sha256Digest)> = BTreeSet::new();
    for d in desired {
        // Elements are found by value, so two keys must never own identical elements.
        if let Content::Entry { key, value } = &d.content
            && let Some(array) = entry::element_array(key)
            && !elements.insert((&d.path, array, entry::value_hash(value)))
        {
            errors.push(Diagnostic::error(
                "plan/duplicate-output",
                format!(
                    "`{}` would get two identical entries from `{}`",
                    d.path, d.owner
                ),
            ));
        }
        let unit = d.content.unit();
        if let Some(previous) = units.insert(&d.path, unit)
            && previous != unit
        {
            errors.push(Diagnostic::error(
                "plan/mixed-units",
                format!(
                    "`{}` would hold both a whole file and blocks or entries",
                    d.path
                ),
            ));
        }
        let id = match &d.content {
            Content::File(_) => String::new(),
            Content::Block(_) => d.owner.name().to_owned(),
            Content::Entry { key, .. } => key.as_str().to_owned(),
        };
        if !seen.insert((&d.path, id)) {
            errors.push(Diagnostic::error(
                "plan/duplicate-output",
                format!("`{}` is written twice by `{}`", d.path, d.owner),
            ));
        }
        let lower = d.path.as_str().to_ascii_lowercase();
        if let Some(other) = folded.insert(lower, &d.path)
            && *other != d.path
        {
            errors.push(Diagnostic::error(
                "plan/case-collision",
                format!(
                    "`{}` and `{other}` differ only in letter case, which is the same file on macOS and Windows",
                    d.path
                ),
            ));
        }
    }
    // A locked file renamed only in case: on macOS and Windows both names are one
    // file, so deleting the old name would delete the new one too.
    let mut reported = BTreeSet::new();
    for (_, o) in lock_outputs(lock) {
        if let Some(new) = folded.get(&o.path.as_str().to_ascii_lowercase())
            && **new != o.path
            && reported.insert(&o.path)
        {
            errors.push(Diagnostic::error(
                "plan/case-collision",
                format!(
                    "grimoire.lock records `{}`, and it would now write `{new}`, which differs only in letter case and is the same file on macOS and Windows; remove the asset and apply, then add it again",
                    o.path
                ),
            ));
        }
    }
}

/// Decides the action for one unit from its three hashes.
fn decide(
    disk: Option<&Sha256Digest>,
    locked: Option<&Sha256Digest>,
    desired: Option<&Sha256Digest>,
) -> Action {
    match (disk, locked, desired) {
        (_, None, None) => Action::Unchanged,
        // Still wanted.
        (Some(d), l, Some(n)) if d == n => {
            if l == Some(n) {
                Action::Unchanged
            } else {
                Action::Adopt
            }
        }
        (None, None, Some(_)) => Action::Create,
        (None, Some(_), Some(_)) => Action::Conflict(Conflict::Missing),
        (Some(d), Some(l), Some(_)) if d == l => Action::Update,
        (Some(_), Some(_), Some(_)) => Action::Conflict(Conflict::Edited),
        (Some(_), None, Some(_)) => Action::Conflict(Conflict::Unmanaged),
        // No longer wanted.
        (None, Some(_), None) => Action::Gone,
        (Some(d), Some(l), None) if d == l => Action::Delete,
        (Some(_), Some(_), None) => Action::Conflict(Conflict::EditedBeforeDelete),
    }
}

/// A lock record and its owner.
type Locked = (Owner, LockedOutput);
/// A unit's desired and locked state.
type Pair = (Option<Desired>, Option<Locked>);

/// Pairs desired and locked units by their identity within one file.
fn pair<K: Ord + Clone>(
    desired: Vec<Desired>,
    locked: Vec<Locked>,
    key: impl Fn(&Owner, Option<&EntryKey>) -> K,
) -> BTreeMap<K, Pair> {
    let mut out: BTreeMap<K, Pair> = BTreeMap::new();
    for d in desired {
        let k = key(&d.owner, d.content.key());
        out.entry(k).or_default().0 = Some(d);
    }
    for (owner, o) in locked {
        let k = key(&owner, o.key.as_ref());
        out.entry(k).or_default().1 = Some((owner, o));
    }
    out
}

fn unit_plan(
    unit: Unit,
    desired: Option<&Desired>,
    locked: Option<&Locked>,
    action: Action,
) -> UnitPlan {
    let owner = desired
        .map(|d| d.owner.clone())
        .or_else(|| locked.map(|(o, _)| o.clone()))
        .unwrap_or(Owner::Project);
    UnitPlan {
        owner,
        clients: desired.map_or_else(
            || {
                locked
                    .map(|(_, o)| o.clients.iter().copied().collect())
                    .unwrap_or_default()
            },
            |d| d.clients.clone(),
        ),
        unit,
        key: desired
            .and_then(|d| d.content.key().cloned())
            .or_else(|| locked.and_then(|(_, o)| o.key.clone())),
        action,
        hash: desired.map(|d| d.content.hash()),
        locked: locked.map(|(_, o)| o.clone()),
        created_file: locked.is_some_and(|(_, o)| o.created_file),
        created_keys: locked
            .map(|(_, o)| o.created_keys.clone())
            .unwrap_or_default(),
    }
}

/// Records what this file's edits created and hands what removed units created
/// to a unit that stays, so that removing the last unit later cleans up exactly
/// what Grimoire created and nothing the user had.
fn hand_over(
    units: &mut [UnitPlan],
    file_existed: bool,
    orphan_file: bool,
    kept_keys: Vec<EntryKey>,
) {
    if !file_existed
        && let Some(first) = units
            .iter_mut()
            .find(|u| u.hash.is_some() && matches!(u.action, Action::Create | Action::Conflict(_)))
    {
        first.created_file = true;
    }
    if let Some(survivor) = units.iter_mut().find(|u| u.hash.is_some()) {
        survivor.created_file |= orphan_file;
        survivor.created_keys.extend(kept_keys);
        survivor.created_keys.sort();
        survivor.created_keys.dedup();
    }
}

fn plan_file(
    path: RelPath,
    before: Option<Vec<u8>>,
    desired: Vec<Desired>,
    locked: Vec<Locked>,
) -> FilePlan {
    let disk = before.as_deref().map(hash::content);
    let want = desired.into_iter().next();
    let had = locked.into_iter().next();
    let action = decide(
        disk.as_ref(),
        had.as_ref().map(|(_, o)| &o.hash),
        want.as_ref().map(|d| d.content.hash()).as_ref(),
    );
    let after = match (&want, action) {
        // Keep the exact bytes on disk when they already match (e.g. CRLF checkouts).
        (Some(_), Action::Unchanged | Action::Adopt) => before.clone(),
        (
            Some(Desired {
                content: Content::File(bytes),
                ..
            }),
            _,
        ) => Some(bytes.clone()),
        (None, Action::Gone | Action::Delete | Action::Conflict(_)) => None,
        _ => before.clone(),
    };
    let units = vec![unit_plan(Unit::File, want.as_ref(), had.as_ref(), action)];
    FilePlan {
        path,
        before,
        after,
        units,
        unreadable: None,
    }
}

fn plan_blocks(
    path: RelPath,
    before: Option<Vec<u8>>,
    desired: Vec<Desired>,
    locked: Vec<Locked>,
) -> FilePlan {
    let unreadable = |reason: String, before: Option<Vec<u8>>, units| FilePlan {
        path: path.clone(),
        after: before.clone(),
        before,
        units,
        unreadable: Some(reason),
    };
    let text = match before.as_deref().map(std::str::from_utf8) {
        None => String::new(),
        Some(Ok(text)) => text.to_owned(),
        Some(Err(_)) => return unreadable("it is not UTF-8 text".to_owned(), before, Vec::new()),
    };
    let mut doc = match Document::parse(&text) {
        Ok(doc) => doc,
        Err(e) => {
            return unreadable(
                format!("its managed-block markers are broken ({e})"),
                before,
                Vec::new(),
            );
        }
    };
    let pairs = pair(desired, locked, |owner, _| owner.name().to_owned());
    let created_by_us = pairs
        .values()
        .any(|(_, had)| had.as_ref().is_some_and(|(_, o)| o.created_file));
    let mut units = Vec::new();
    let mut orphan_file = false;
    for (name, (want, had)) in &pairs {
        let disk = doc.get(name).map(|b| hash::content(b.as_bytes()));
        let n = want.as_ref().map(|d| d.content.hash());
        let action = decide(
            disk.as_ref(),
            had.as_ref().map(|(_, o)| &o.hash),
            n.as_ref(),
        );
        match (want, action) {
            (
                Some(Desired {
                    content: Content::Block(body),
                    ..
                }),
                Action::Create | Action::Update | Action::Conflict(_),
            ) => {
                if let Err(e) = doc.set(name, body) {
                    return unreadable(
                        format!("grimoire cannot add its block: {e}"),
                        before,
                        Vec::new(),
                    );
                }
            }
            (None, Action::Delete | Action::Conflict(_) | Action::Gone) => {
                doc.remove(name);
                orphan_file |= had.as_ref().is_some_and(|(_, o)| o.created_file);
            }
            _ => {}
        }
        units.push(unit_plan(Unit::Block, want.as_ref(), had.as_ref(), action));
    }
    let blank = doc.is_blank();
    // Blocks adopted from an earlier, interrupted apply, in a file holding nothing
    // else: Grimoire created that file.
    let untracked = pairs.values().all(|(_, had)| had.is_none());
    if before.is_some() && untracked && units.iter().all(|u| u.action == Action::Adopt) {
        let mut probe = doc.clone();
        for u in &units {
            probe.remove(u.owner.name());
        }
        if probe.is_blank()
            && let Some(first) = units.first_mut()
        {
            first.created_file = true;
        }
    }
    hand_over(
        &mut units,
        before.is_some(),
        orphan_file && !blank,
        Vec::new(),
    );
    let rendered = doc.render();
    // Only a file Grimoire created is deleted once nothing is left in it.
    let after = if before.as_deref() == Some(rendered.as_bytes()) || (before.is_none() && blank) {
        before.clone()
    } else if blank && created_by_us {
        None
    } else {
        Some(rendered.into_bytes())
    };
    FilePlan {
        path,
        before,
        after,
        units,
        unreadable: None,
    }
}

/// What is on disk for one entry key and what applying does to it.
fn entry_action(
    doc: &EntryDoc,
    key: &EntryKey,
    locked: Option<&Sha256Digest>,
    desired: Option<&Sha256Digest>,
    is_locked: bool,
    claimed: &mut BTreeSet<(String, Sha256Digest)>,
) -> Result<(Option<Sha256Digest>, Action), entry::EntryError> {
    let mut disk = doc.find(key, locked, desired)?;
    let array = entry::element_array(key);
    // Two keys never share one array element.
    if let (Some(array), Some(found)) = (&array, &disk)
        && !claimed.insert((array.clone(), found.clone()))
    {
        disk = None;
    }
    // An element is found by value, so a locked element that is gone may have
    // been edited rather than removed: never treat that as already removed.
    let action = if array.is_some() && disk.is_none() && is_locked {
        Action::Conflict(Conflict::NotFound)
    } else {
        decide(disk.as_ref(), locked, desired)
    };
    Ok((disk, guard_comments(doc, key, action)?))
}

/// Values are compared without their comments, so a comment the user put inside
/// Grimoire's entry would be lost without a word by an update or removal: that
/// becomes a conflict instead.
fn guard_comments(
    doc: &EntryDoc,
    key: &EntryKey,
    action: Action,
) -> Result<Action, entry::EntryError> {
    Ok(match action {
        Action::Update | Action::Delete if doc.has_comments(key, action == Action::Delete)? => {
            Action::Conflict(Conflict::Commented)
        }
        other => other,
    })
}

/// Entries adopted from an earlier, interrupted apply: the containers that hold
/// nothing else, and the file if nothing else is left, were created by Grimoire.
fn infer_created(format: EntryFormat, text: Option<&str>, units: &mut [UnitPlan]) {
    let Ok(mut probe) = EntryDoc::parse(format, text) else {
        return;
    };
    let mut containers = Vec::new();
    for u in units.iter() {
        let (Some(key), Some(hash)) = (&u.key, &u.hash) else {
            return;
        };
        if probe.remove(key, Some(hash)).is_err() {
            return;
        }
        containers.extend(entry::containers(key));
    }
    containers.sort();
    containers.dedup();
    let Ok(kept) = probe.prune(&containers) else {
        return;
    };
    let empty = probe.is_empty();
    if let Some(first) = units.first_mut() {
        first.created_keys = containers
            .into_iter()
            .filter(|c| !kept.contains(c))
            .filter_map(|c| EntryKey::new(c).ok())
            .collect();
        first.created_file = empty;
    }
}

fn plan_entries(
    path: RelPath,
    before: Option<Vec<u8>>,
    desired: Vec<Desired>,
    locked: Vec<Locked>,
) -> FilePlan {
    let unreadable = |reason: String, before: Option<Vec<u8>>| FilePlan {
        path: path.clone(),
        after: before.clone(),
        before,
        units: Vec::new(),
        unreadable: Some(reason),
    };
    let text = match before.as_deref().map(std::str::from_utf8) {
        None => None,
        Some(Ok(text)) => Some(text),
        Some(Err(_)) => return unreadable("it is not UTF-8 text".to_owned(), before),
    };
    // The format comes from the path: a lock record has no value to tell it.
    let format = EntryFormat::for_path(&path);
    let mut doc = match EntryDoc::parse(format, text) {
        Ok(doc) => doc,
        Err(e) => return unreadable(e.to_string(), before),
    };
    let pairs = pair(desired, locked, |_, key| key.cloned());
    let created_by_us = pairs
        .values()
        .any(|(_, had)| had.as_ref().is_some_and(|(_, o)| o.created_file));
    let mut units = Vec::new();
    let mut orphan_file = false;
    let mut orphan_keys: Vec<String> = Vec::new();
    // Array elements already matched by an earlier key, so two keys never share one.
    let mut claimed: BTreeSet<(String, Sha256Digest)> = BTreeSet::new();
    for (key, (want, had)) in &pairs {
        let Some(key) = key else { continue };
        let n = want.as_ref().map(|d| d.content.hash());
        let l = had.as_ref().map(|(_, o)| o.hash.clone());
        let (disk, action) = match entry_action(
            &doc,
            key,
            l.as_ref(),
            n.as_ref(),
            had.is_some(),
            &mut claimed,
        ) {
            Ok(found) => found,
            Err(e) => return unreadable(e.to_string(), before),
        };
        let mut unit = unit_plan(Unit::Entry, want.as_ref(), had.as_ref(), action);
        let result = match (want, action) {
            (
                Some(Desired {
                    content: Content::Entry { value, .. },
                    ..
                }),
                Action::Create | Action::Update | Action::Conflict(_),
            ) => doc.set(key, l.as_ref(), value).map(|created| {
                unit.created_keys
                    .extend(created.into_iter().filter_map(|p| EntryKey::new(p).ok()));
            }),
            (None, Action::Delete | Action::Conflict(_) | Action::Gone) => {
                if let Some((_, o)) = had {
                    orphan_file |= o.created_file;
                    orphan_keys.extend(o.created_keys.iter().map(|k| k.as_str().to_owned()));
                }
                if action == Action::Gone {
                    Ok(())
                } else {
                    doc.remove(key, disk.as_ref())
                }
            }
            _ => Ok(()),
        };
        if let Err(e) = result {
            return unreadable(e.to_string(), before);
        }
        units.push(unit);
    }
    // Containers Grimoire created go once empty; ones it did not create stay.
    let kept = match doc.prune(&orphan_keys) {
        Ok(kept) => kept,
        Err(e) => return unreadable(e.to_string(), before),
    };
    let empty = doc.is_empty();
    let untracked = pairs.values().all(|(_, had)| had.is_none());
    if doc.existed() && untracked && units.iter().all(|u| u.action == Action::Adopt) {
        infer_created(format, text, &mut units);
    }
    hand_over(
        &mut units,
        doc.existed(),
        orphan_file && !empty,
        kept.into_iter()
            .filter_map(|p| EntryKey::new(p).ok())
            .collect(),
    );
    let after = entries_after(
        &doc,
        before.as_deref(),
        empty && (created_by_us || !doc.existed()),
    );
    FilePlan {
        path,
        before,
        after,
        units,
        unreadable: None,
    }
}

/// The file's content after its entries are edited: gone when `delete`, the exact
/// old bytes when nothing changed, else the edited document.
fn entries_after(doc: &EntryDoc, before: Option<&[u8]>, delete: bool) -> Option<Vec<u8>> {
    if delete {
        return None;
    }
    let rendered = doc.render();
    if before == Some(rendered.as_slice()) {
        before.map(<[u8]>::to_vec)
    } else {
        Some(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(s: &str) -> Sha256Digest {
        hash::content(s.as_bytes())
    }

    #[test]
    fn the_drift_matrix() {
        let (a, b, c) = (h("a"), h("b"), h("c"));
        assert_eq!(decide(Some(&a), Some(&a), Some(&a)), Action::Unchanged);
        assert_eq!(decide(Some(&a), Some(&a), Some(&b)), Action::Update);
        assert_eq!(decide(Some(&b), Some(&a), Some(&b)), Action::Adopt);
        assert_eq!(decide(Some(&b), None, Some(&b)), Action::Adopt);
        assert_eq!(
            decide(Some(&c), Some(&a), Some(&b)),
            Action::Conflict(Conflict::Edited)
        );
        assert_eq!(
            decide(None, Some(&a), Some(&b)),
            Action::Conflict(Conflict::Missing)
        );
        assert_eq!(
            decide(Some(&a), None, Some(&b)),
            Action::Conflict(Conflict::Unmanaged)
        );
        assert_eq!(decide(None, None, Some(&b)), Action::Create);
        assert_eq!(decide(Some(&a), Some(&a), None), Action::Delete);
        assert_eq!(decide(None, Some(&a), None), Action::Gone);
        assert_eq!(
            decide(Some(&b), Some(&a), None),
            Action::Conflict(Conflict::EditedBeforeDelete)
        );
    }
}
