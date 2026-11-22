//! Root-confined, symlink-refusing, atomic filesystem access (`architecture.md` §7.4, ADR-0006).
//!
//! Every operation starts from a capability handle on the project root
//! ([`cap_std::fs::Dir`]) and walks to the parent directory one component at a
//! time with no-follow opens. Confinement is enforced by cap-std itself:
//! `openat2(RESOLVE_BENEATH)` on Linux and a handle-relative resolver elsewhere,
//! so a path can never leave the root, even if the tree changes underneath.
//! On top of that, Grimoire refuses any symlink at or above a path it manages,
//! and any special file at the path itself, because following one would write
//! somewhere the user did not expect. A special file above a path is in the way,
//! like any other file where a directory is needed.
//!
//! Writes go to a temporary file in the destination directory, which is synced
//! and then renamed over the target. An existing file's permissions are kept.

use std::fmt;
use std::io::{self, Read as _, Write as _};
use std::path::Path;

use agent_grimoire_model::RelPath;
use cap_fs_ext::{DirExt as _, FollowSymlinks, OpenOptionsFollowExt as _};
use cap_std::fs::{Dir, OpenOptions};

/// The largest file Grimoire reads in a project. Managed files are small; Claude
/// Code itself skips instruction files over 4 MiB (`CC-MEM`).
pub const MAX_READ_BYTES: u64 = 8 * 1024 * 1024;

/// A filesystem operation that failed.
#[derive(Debug)]
pub enum FsError {
    /// Refused for safety: a symlink at or above a managed path, a special or
    /// oversized file at it, or a path that would leave the root (exit 6).
    Refused {
        /// The path involved.
        path: String,
        /// Why.
        reason: String,
    },
    /// Something is in the way: a file where a directory is needed, or a
    /// directory where a file is needed (exit 4, a conflict).
    InTheWay {
        /// The path involved.
        path: String,
        /// What is there.
        reason: String,
    },
    /// An I/O error (exit 1).
    Io {
        /// The path involved.
        path: String,
        /// The error.
        source: io::Error,
    },
}

impl fmt::Display for FsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsError::Refused { path, reason } => write!(f, "refusing to touch `{path}`: {reason}"),
            FsError::InTheWay { path, reason } => write!(f, "`{path}` is in the way: {reason}"),
            FsError::Io { path, source } => write!(f, "`{path}`: {source}"),
        }
    }
}

impl std::error::Error for FsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            FsError::Refused { .. } | FsError::InTheWay { .. } => None,
            FsError::Io { source, .. } => Some(source),
        }
    }
}

fn refused(path: &str, reason: impl Into<String>) -> FsError {
    FsError::Refused {
        path: path.to_owned(),
        reason: reason.into(),
    }
}

fn io_err(path: &str, source: io::Error) -> FsError {
    // cap-std reports an attempt to leave the root as PermissionDenied.
    if source.kind() == io::ErrorKind::PermissionDenied
        && source.to_string().contains("outside of the filesystem")
    {
        return refused(path, "the path leads outside the project root");
    }
    FsError::Io {
        path: path.to_owned(),
        source,
    }
}

/// The project root.
#[derive(Debug)]
pub struct Root {
    dir: Dir,
}

impl Root {
    /// Opens the project root directory.
    ///
    /// # Errors
    ///
    /// Fails if `path` is not a readable directory.
    pub fn open(path: &Path) -> Result<Self, FsError> {
        let dir = Dir::open_ambient_dir(path, cap_std::ambient_authority())
            .map_err(|e| io_err(&path.display().to_string(), e))?;
        Ok(Self { dir })
    }

    /// Opens the directory holding `path`. With `create`, missing directories are
    /// created; without it, `Ok(None)` means one is missing.
    fn parent(&self, path: &RelPath, create: bool) -> Result<Option<Dir>, FsError> {
        let parts: Vec<&str> = path.components().collect();
        let mut dir = self.dir.try_clone().map_err(|e| io_err(".", e))?;
        let mut shown = String::new();
        for part in &parts[..parts.len().saturating_sub(1)] {
            if !shown.is_empty() {
                shown.push('/');
            }
            shown.push_str(part);
            match dir.symlink_metadata(part) {
                Ok(meta) if meta.file_type().is_symlink() => {
                    return Err(refused(&shown, "it is a symbolic link"));
                }
                Ok(meta) if !meta.is_dir() => {
                    return Err(FsError::InTheWay {
                        path: shown,
                        reason: "it is a file, and grimoire needs a directory there".to_owned(),
                    });
                }
                Ok(_) => {}
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    if !create {
                        return Ok(None);
                    }
                    match dir.create_dir(part) {
                        Ok(()) => {}
                        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
                        Err(e) => return Err(io_err(&shown, e)),
                    }
                }
                Err(e) => return Err(io_err(&shown, e)),
            }
            // Refuses a symlink even if one appeared since the check above.
            dir = dir.open_dir_nofollow(part).map_err(|e| {
                if dir
                    .symlink_metadata(part)
                    .is_ok_and(|m| m.file_type().is_symlink())
                {
                    refused(&shown, "it is a symbolic link")
                } else {
                    io_err(&shown, e)
                }
            })?;
        }
        Ok(Some(dir))
    }

    /// Checks what is at `name` in `dir`: `Ok(None)` if absent, its metadata for a
    /// regular file, and a refusal for anything else.
    fn check_file(
        dir: &Dir,
        name: &str,
        shown: &str,
    ) -> Result<Option<cap_std::fs::Metadata>, FsError> {
        match dir.symlink_metadata(name) {
            Ok(meta) if meta.file_type().is_symlink() => {
                Err(refused(shown, "it is a symbolic link"))
            }
            Ok(meta) if meta.is_dir() => Err(FsError::InTheWay {
                path: shown.to_owned(),
                reason: "it is a directory, and grimoire needs a file there".to_owned(),
            }),
            Ok(meta) if !meta.is_file() => Err(refused(shown, "it is not a regular file")),
            Ok(meta) => Ok(Some(meta)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(io_err(shown, e)),
        }
    }

    /// Reads `path`. `Ok(None)` when it does not exist.
    ///
    /// # Errors
    ///
    /// Refuses symlinks, special files and files over [`MAX_READ_BYTES`].
    pub fn read(&self, path: &RelPath) -> Result<Option<Vec<u8>>, FsError> {
        let shown = path.as_str();
        let Some(dir) = self.parent(path, false)? else {
            return Ok(None);
        };
        let name = path.file_name();
        let Some(meta) = Self::check_file(&dir, name, shown)? else {
            return Ok(None);
        };
        if meta.len() > MAX_READ_BYTES {
            return Err(refused(
                shown,
                format!("it is larger than {MAX_READ_BYTES} bytes"),
            ));
        }
        let mut options = OpenOptions::new();
        options.read(true).follow(FollowSymlinks::No);
        // Non-blocking, so a FIFO swapped in after the check above cannot hang us.
        #[cfg(unix)]
        {
            use cap_std::fs::OpenOptionsExt as _;
            options.custom_flags(libc::O_NONBLOCK);
        }
        let file = dir.open_with(name, &options).map_err(|e| {
            match Self::check_file(&dir, name, shown) {
                Err(refusal) => refusal,
                Ok(_) => io_err(shown, e),
            }
        })?;
        // Check what was actually opened, not only what the path showed before.
        let opened = file.metadata().map_err(|e| io_err(shown, e))?;
        if !opened.is_file() {
            return Err(refused(shown, "it is not a regular file"));
        }
        if opened.len() > MAX_READ_BYTES {
            return Err(refused(
                shown,
                format!("it is larger than {MAX_READ_BYTES} bytes"),
            ));
        }
        let mut bytes = Vec::new();
        file.take(MAX_READ_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|e| io_err(shown, e))?;
        if u64::try_from(bytes.len()).unwrap_or(u64::MAX) > MAX_READ_BYTES {
            return Err(refused(
                shown,
                format!("it is larger than {MAX_READ_BYTES} bytes"),
            ));
        }
        Ok(Some(bytes))
    }

    /// Whether anything is at `path`: a file, a directory, a special file or a
    /// symbolic link, even a broken one. Nothing at `path` is opened or followed.
    /// Unlike cap-std's `Dir::exists`, which follows symbolic links inside the
    /// root, this follows none.
    ///
    /// # Errors
    ///
    /// As for every other operation, refuses a symbolic link above `path` and
    /// reports a file where a directory is needed as in the way; reports I/O errors.
    pub fn exists(&self, path: &RelPath) -> Result<bool, FsError> {
        let Some(dir) = self.parent(path, false)? else {
            return Ok(false);
        };
        match dir.symlink_metadata(path.file_name()) {
            Ok(_) => Ok(true),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(e) => Err(io_err(path.as_str(), e)),
        }
    }

    /// Whether a regular file is at `path`. Makes the checks [`Root::read`] and
    /// [`Root::remove`] make, without opening it, and changes nothing.
    ///
    /// # Errors
    ///
    /// Refuses a symbolic link at or above `path` and a special file at it,
    /// reports a directory at it as in the way, and reports I/O errors.
    pub fn has_file(&self, path: &RelPath) -> Result<bool, FsError> {
        self.metadata(path).map(|m| m.is_some())
    }

    /// Replaces `path` with `bytes` atomically, creating parent directories.
    ///
    /// # Errors
    ///
    /// Refuses ([`FsError::Refused`]) a symbolic link at or above `path` and a
    /// special file at it. Reports ([`FsError::InTheWay`]) a regular or special
    /// file above `path`, where a directory is needed, and a directory at it.
    /// Reports I/O errors.
    pub fn write(&self, path: &RelPath, bytes: &[u8]) -> Result<(), FsError> {
        self.write_like(path, bytes, None)
    }

    /// Like [`Root::write`], but a new file takes its permissions from `like`
    /// (for example `AGENTS.md.new` from `AGENTS.md`), so a copy of a restricted
    /// file is not more readable than the original.
    ///
    /// # Errors
    ///
    /// As for [`Root::write`].
    pub fn write_like(
        &self,
        path: &RelPath,
        bytes: &[u8],
        like: Option<&RelPath>,
    ) -> Result<(), FsError> {
        let shown = path.as_str();
        let dir = self
            .parent(path, true)?
            .ok_or_else(|| refused(shown, "its directory could not be created"))?;
        let name = path.file_name();
        let existing = match Self::check_file(&dir, name, shown)? {
            Some(meta) => Some(meta),
            None => match like {
                Some(like) => self.metadata(like)?,
                None => None,
            },
        };
        let mut tmp = cap_tempfile::TempFile::new(&dir).map_err(|e| io_err(shown, e))?;
        // Restrict permissions before any content is in the file.
        if let Some(meta) = existing {
            tmp.as_file()
                .set_permissions(meta.permissions())
                .map_err(|e| io_err(shown, e))?;
        }
        tmp.write_all(bytes).map_err(|e| io_err(shown, e))?;
        tmp.as_file().sync_all().map_err(|e| io_err(shown, e))?;
        tmp.replace(name).map_err(|e| io_err(shown, e))?;
        sync_dir(&dir);
        Ok(())
    }

    /// The metadata of the regular file at `path`, if there is one.
    fn metadata(&self, path: &RelPath) -> Result<Option<cap_std::fs::Metadata>, FsError> {
        match self.parent(path, false)? {
            Some(dir) => Self::check_file(&dir, path.file_name(), path.as_str()),
            None => Ok(None),
        }
    }

    /// Deletes the file at `path`, then every parent directory that is left empty.
    /// A missing file is not an error.
    ///
    /// # Errors
    ///
    /// Refuses ([`FsError::Refused`]) a symbolic link at or above `path` and a
    /// special file at it. Reports ([`FsError::InTheWay`]) a regular or special
    /// file above `path`, where a directory is needed, and a directory at it.
    /// Reports I/O errors.
    pub fn remove(&self, path: &RelPath) -> Result<(), FsError> {
        let shown = path.as_str();
        let Some(dir) = self.parent(path, false)? else {
            return Ok(());
        };
        let name = path.file_name();
        if Self::check_file(&dir, name, shown)?.is_none() {
            return Ok(());
        }
        dir.remove_file(name).map_err(|e| io_err(shown, e))?;
        sync_dir(&dir);
        // Close the directory before pruning it. On Windows cap-std opens
        // directories without FILE_SHARE_DELETE, so an open handle makes
        // removing that directory fail, and pruning would stop there.
        drop(dir);
        self.prune(path);
        Ok(())
    }

    /// Removes the now-empty directories above `path`, deepest first. Stops at
    /// the first one that is not empty (or cannot be removed).
    fn prune(&self, path: &RelPath) {
        let parts: Vec<&str> = path.components().collect();
        for depth in (1..parts.len()).rev() {
            let Ok(dir_path) = RelPath::new(parts[..depth].join("/")) else {
                return;
            };
            let Ok(Some(parent)) = self.parent(&dir_path, false) else {
                return;
            };
            let name = dir_path.file_name();
            let is_dir = parent
                .symlink_metadata(name)
                .is_ok_and(|m| m.is_dir() && !m.file_type().is_symlink());
            if !is_dir || parent.remove_dir(name).is_err() {
                return;
            }
        }
    }
}

/// Syncs a directory after a rename or unlink, where the platform allows it.
/// Best effort: the rename is already atomic; this only makes it durable sooner.
#[cfg(unix)]
fn sync_dir(dir: &Dir) {
    if let Ok(handle) = dir.open(".") {
        let _ = handle.sync_all();
    }
}

#[cfg(not(unix))]
fn sync_dir(_dir: &Dir) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn rel(s: &str) -> RelPath {
        RelPath::new(s).unwrap()
    }

    #[test]
    fn write_read_remove_and_prune() {
        let tmp = tempfile::tempdir().unwrap();
        let root = Root::open(tmp.path()).unwrap();
        assert_eq!(root.read(&rel("a/b/c.md")).unwrap(), None);
        root.write(&rel("a/b/c.md"), b"one").unwrap();
        root.write(&rel("a/keep.md"), b"keep").unwrap();
        assert_eq!(root.read(&rel("a/b/c.md")).unwrap().unwrap(), b"one");
        root.write(&rel("a/b/c.md"), b"two").unwrap();
        assert_eq!(std::fs::read(tmp.path().join("a/b/c.md")).unwrap(), b"two");
        root.remove(&rel("a/b/c.md")).unwrap();
        assert!(!tmp.path().join("a/b").exists(), "emptied directory pruned");
        assert!(
            tmp.path().join("a/keep.md").exists(),
            "non-empty directory kept"
        );
        root.remove(&rel("a/b/c.md")).unwrap();
        let leftovers: Vec<_> = std::fs::read_dir(tmp.path().join("a")).unwrap().collect();
        assert_eq!(leftovers.len(), 1, "no temporary files left behind");
    }

    #[test]
    fn things_in_the_way_are_conflicts_not_refusals() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join(".claude"), "a file").unwrap();
        std::fs::create_dir_all(tmp.path().join("dir.md")).unwrap();
        let root = Root::open(tmp.path()).unwrap();
        assert!(matches!(
            root.read(&rel(".claude/rules/x.md")),
            Err(FsError::InTheWay { .. })
        ));
        assert!(matches!(
            root.read(&rel("dir.md")),
            Err(FsError::InTheWay { .. })
        ));
    }

    #[test]
    fn exists_and_has_file_change_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let root = Root::open(tmp.path()).unwrap();
        assert!(!root.exists(&rel("a.md")).unwrap());
        assert!(!root.exists(&rel("missing/a.md")).unwrap());
        assert!(!root.has_file(&rel("a.md")).unwrap());
        assert!(!root.has_file(&rel("missing/a.md")).unwrap());
        root.write(&rel("dir/a.md"), b"x").unwrap();
        assert!(root.exists(&rel("dir")).unwrap());
        assert!(root.exists(&rel("dir/a.md")).unwrap());
        assert!(root.has_file(&rel("dir/a.md")).unwrap());
        assert!(matches!(
            root.has_file(&rel("dir")),
            Err(FsError::InTheWay { .. })
        ));
        // A file where a directory is needed is in the way, as for `read`.
        assert!(matches!(
            root.exists(&rel("dir/a.md/b")),
            Err(FsError::InTheWay { .. })
        ));
        let mut entries: Vec<_> = std::fs::read_dir(tmp.path())
            .unwrap()
            .map(|e| e.unwrap().file_name())
            .collect();
        entries.sort();
        assert_eq!(entries, ["dir"], "nothing was created");
    }

    #[cfg(unix)]
    #[test]
    fn exists_follows_no_symbolic_link() {
        use std::os::unix::fs::symlink;
        let tmp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        std::fs::write(outside.path().join("target.md"), "x").unwrap();
        std::fs::create_dir(tmp.path().join("real")).unwrap();
        std::fs::write(tmp.path().join("real/a.md"), "x").unwrap();
        symlink(outside.path(), tmp.path().join("linked")).unwrap();
        symlink("real", tmp.path().join("inside")).unwrap();
        symlink("/nonexistent-target", tmp.path().join("broken.md")).unwrap();
        let root = Root::open(tmp.path()).unwrap();
        // A link at the path itself is there, even a broken one.
        assert!(root.exists(&rel("broken.md")).unwrap());
        assert!(root.exists(&rel("linked")).unwrap());
        // A link above the path is refused, wherever it points.
        assert!(matches!(
            root.exists(&rel("linked/target.md")),
            Err(FsError::Refused { .. })
        ));
        assert!(matches!(
            root.exists(&rel("inside/a.md")),
            Err(FsError::Refused { .. })
        ));
        // `has_file` refuses a link at the path too.
        assert!(matches!(
            root.has_file(&rel("broken.md")),
            Err(FsError::Refused { .. })
        ));
    }

    #[cfg(unix)]
    #[test]
    fn fifos_are_refused_without_blocking() {
        let tmp = tempfile::tempdir().unwrap();
        let made = std::process::Command::new("mkfifo")
            .arg(tmp.path().join("AGENTS.md"))
            .status();
        if !made.is_ok_and(|s| s.success()) {
            return; // mkfifo is unavailable; nothing to test.
        }
        let root = Root::open(tmp.path()).unwrap();
        assert!(matches!(
            root.read(&rel("AGENTS.md")),
            Err(FsError::Refused { .. })
        ));
    }

    #[cfg(unix)]
    #[test]
    fn copies_take_the_original_permissions() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join(".mcp.json");
        std::fs::write(&file, "{}").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        let root = Root::open(tmp.path()).unwrap();
        root.write_like(&rel(".mcp.json.new"), b"{}", Some(&rel(".mcp.json")))
            .unwrap();
        let mode = std::fs::metadata(tmp.path().join(".mcp.json.new"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);
    }

    #[cfg(unix)]
    #[test]
    fn keeps_existing_permissions() {
        use std::os::unix::fs::PermissionsExt as _;
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("AGENTS.md");
        std::fs::write(&file, "x").unwrap();
        std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o600)).unwrap();
        Root::open(tmp.path())
            .unwrap()
            .write(&rel("AGENTS.md"), b"y")
            .unwrap();
        let mode = std::fs::metadata(&file).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
    }
}
