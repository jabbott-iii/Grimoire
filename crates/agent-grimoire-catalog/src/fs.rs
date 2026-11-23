//! Deterministic, symlink-refusing directory walking for catalog validation.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read as _};
use std::path::Path;

use agent_grimoire_model::{Diagnostic, RelPath};

/// Largest file the catalog accepts. Assets ship inside the binary, as
/// [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md)
/// chose, so every file adds to the binary's size.
pub const MAX_FILE_BYTES: u64 = 1024 * 1024;
/// Deepest directory nesting the catalog accepts inside one asset.
const MAX_DEPTH: usize = 12;
/// Most entries the catalog accepts inside one asset.
const MAX_ENTRIES: usize = 2000;

/// What a walked path is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EntryKind {
    File { len: u64 },
    Dir,
}

/// Every file and directory below an asset directory, keyed by portable relative path.
#[derive(Debug, Default)]
pub(crate) struct Tree {
    pub(crate) entries: BTreeMap<RelPath, EntryKind>,
    /// The walk stopped at the entry limit; later checks must not report "missing" files.
    pub(crate) truncated: bool,
}

impl Tree {
    pub(crate) fn get(&self, path: &RelPath) -> Option<EntryKind> {
        self.entries.get(path).copied()
    }

    pub(crate) fn is_file(&self, path: &str) -> bool {
        RelPath::new(path)
            .ok()
            .and_then(|p| self.get(&p))
            .is_some_and(|k| matches!(k, EntryKind::File { .. }))
    }

    pub(crate) fn is_dir(&self, path: &str) -> bool {
        RelPath::new(path)
            .ok()
            .and_then(|p| self.get(&p))
            .is_some_and(|k| k == EntryKind::Dir)
    }

    /// Files at or below `prefix` (which may name a file or a directory).
    pub(crate) fn files_under<'a>(
        &'a self,
        prefix: &'a RelPath,
    ) -> impl Iterator<Item = &'a RelPath> + 'a {
        self.entries
            .iter()
            .filter(move |(path, kind)| {
                matches!(kind, EntryKind::File { .. }) && path.starts_with(prefix)
            })
            .map(|(path, _)| path)
    }
}

/// Lists a directory's entries sorted by name. Names that are not UTF-8 are reported.
pub(crate) fn read_dir_sorted(dir: &Path) -> io::Result<Vec<(String, fs::Metadata)>> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().into_string().map_err(|raw| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("non-UTF-8 file name {}", raw.display()),
            )
        })?;
        // `symlink_metadata` never follows a link, so a symlink is seen as one.
        let meta = fs::symlink_metadata(entry.path())?;
        out.push((name, meta));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// Walks `root` without following symlinks. `display` is `root`'s path from the
/// catalog root, such as `skills/<id>`: it prefixes paths in diagnostics, and
/// counts towards each path's length.
pub(crate) fn walk(root: &Path, display: &str, diagnostics: &mut Vec<Diagnostic>) -> Tree {
    let mut tree = Tree::default();
    walk_into(root, None, 0, display, &mut tree, diagnostics);
    collisions(&tree, display, diagnostics);
    tree
}

/// Paths that differ only in letter case collide on macOS and Windows: one would
/// silently replace the other on install (for example `skill.md` and `SKILL.md`).
/// Paths are ASCII (see `RelPath`), so ASCII case folding is exact.
fn collisions(tree: &Tree, display: &str, diagnostics: &mut Vec<Diagnostic>) {
    let mut folded: BTreeMap<String, Vec<&RelPath>> = BTreeMap::new();
    for path in tree.entries.keys() {
        folded
            .entry(path.as_str().to_ascii_lowercase())
            .or_default()
            .push(path);
    }
    for paths in folded.values().filter(|p| p.len() > 1) {
        let names: Vec<&str> = paths.iter().map(|p| p.as_str()).collect();
        diagnostics.push(
            Diagnostic::error(
                "catalog/path-collision",
                format!(
                    "{} differ only in letter case, which collides on macOS and Windows",
                    names.join(" and ")
                ),
            )
            .in_file(format!("{display}/{}", names[0])),
        );
    }
}

fn walk_into(
    dir: &Path,
    rel: Option<&RelPath>,
    depth: usize,
    display: &str,
    tree: &mut Tree,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let shown = |name: &str| match rel {
        Some(rel) => format!("{display}/{rel}/{name}"),
        None => format!("{display}/{name}"),
    };
    let entries = match read_dir_sorted(dir) {
        Ok(entries) => entries,
        Err(e) => {
            let at = rel.map_or_else(|| display.to_owned(), |r| format!("{display}/{r}"));
            diagnostics.push(
                Diagnostic::error("catalog/unreadable", format!("cannot read directory: {e}"))
                    .in_file(at),
            );
            return;
        }
    };
    for (name, meta) in entries {
        if tree.truncated {
            return;
        }
        if tree.entries.len() >= MAX_ENTRIES {
            tree.truncated = true;
            diagnostics.push(
                Diagnostic::error(
                    "catalog/too-many-files",
                    format!("an asset may hold at most {MAX_ENTRIES} entries"),
                )
                .in_file(display.to_owned()),
            );
            return;
        }
        let path = match entry_path(rel, &name, display) {
            Ok(path) => path,
            Err(message) => {
                diagnostics.push(Diagnostic::error("catalog/path", message).in_file(shown(&name)));
                continue;
            }
        };
        if meta.file_type().is_symlink() {
            diagnostics.push(
                Diagnostic::error("catalog/symlink", "symlinks are not allowed in the catalog")
                    .in_file(shown(&name)),
            );
        } else if meta.is_dir() {
            if depth + 1 > MAX_DEPTH {
                diagnostics.push(
                    Diagnostic::error(
                        "catalog/too-deep",
                        format!("nesting deeper than {MAX_DEPTH} levels"),
                    )
                    .in_file(shown(&name)),
                );
                continue;
            }
            tree.entries.insert(path.clone(), EntryKind::Dir);
            walk_into(
                &dir.join(&name),
                Some(&path),
                depth + 1,
                display,
                tree,
                diagnostics,
            );
        } else if meta.is_file() {
            if meta.len() > MAX_FILE_BYTES {
                diagnostics.push(
                    Diagnostic::error(
                        "catalog/file-size",
                        format!(
                            "files may be at most {MAX_FILE_BYTES} bytes (got {})",
                            meta.len()
                        ),
                    )
                    .in_file(shown(&name)),
                );
            }
            tree.entries
                .insert(path, EntryKind::File { len: meta.len() });
        } else {
            diagnostics.push(
                Diagnostic::error(
                    "catalog/special-file",
                    "only regular files and directories are allowed",
                )
                .in_file(shown(&name)),
            );
        }
    }
}

/// The path of the entry `name` in the directory `rel`, relative to the walked
/// directory `display`. It must be a [`RelPath`], and so must its path from the
/// catalog root: the pack stores each file under that path, and loading the pack
/// requires it to be a `RelPath` too.
fn entry_path(rel: Option<&RelPath>, name: &str, display: &str) -> Result<RelPath, String> {
    let path = match rel {
        Some(rel) => format!("{rel}/{name}"),
        None => name.to_owned(),
    };
    let path = RelPath::new(path).map_err(|e| format!("not a portable path: {}", e.reason()))?;
    RelPath::new(format!("{display}/{path}")).map_err(|e| {
        format!(
            "the embedded pack stores files by their path from the catalog root, and that path {}",
            e.reason()
        )
    })?;
    Ok(path)
}

/// Reads a catalog text file, which must be a regular UTF-8 file of at most
/// [`MAX_FILE_BYTES`]. The limit is enforced while reading, not only reported.
pub(crate) fn read_text(path: &Path) -> Result<String, String> {
    String::from_utf8(read_bytes(path)?).map_err(|_| "must be valid UTF-8".to_owned())
}

/// Reads a catalog file, which must be a regular file of at most [`MAX_FILE_BYTES`].
pub(crate) fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    // Checked without following links and before opening: opening a FIFO would block.
    let meta = fs::symlink_metadata(path).map_err(|e| format!("cannot read: {e}"))?;
    if !meta.is_file() {
        return Err("is not a regular file".to_owned());
    }
    if meta.len() > MAX_FILE_BYTES {
        return Err(format!("is larger than the {MAX_FILE_BYTES}-byte limit"));
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .and_then(|f| f.take(MAX_FILE_BYTES + 1).read_to_end(&mut bytes))
        .map_err(|e| format!("cannot read: {e}"))?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_FILE_BYTES {
        return Err(format!("is larger than the {MAX_FILE_BYTES}-byte limit"));
    }
    Ok(bytes)
}
