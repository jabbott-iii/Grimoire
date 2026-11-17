//! `secret-guard scan`: files, directories and standard input.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::detect::{self, Finding};

/// Files larger than this are skipped, not read.
pub(crate) const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;
/// Standard input larger than this is refused.
pub(crate) const MAX_STDIN_BYTES: usize = 32 * 1024 * 1024;
/// How deep a directory walk goes.
const MAX_DEPTH: usize = 64;
/// How much of a file is inspected to decide that it is binary.
const BINARY_SNIFF: usize = 8192;

/// The name standard input is reported under.
pub(crate) const STDIN: &str = "<stdin>";

/// The outcome of scanning every input.
#[derive(Debug, Default)]
pub(crate) struct Report {
    /// Findings, by input.
    pub(crate) findings: Vec<(String, Finding)>,
    /// Inputs that were not scanned, and why. Not an error.
    pub(crate) skipped: Vec<(String, String)>,
    /// Inputs that could not be read.
    pub(crate) errors: Vec<(String, String)>,
}

impl Report {
    fn text(&mut self, name: &str, bytes: &[u8]) {
        let sniff = &bytes[..bytes.len().min(BINARY_SNIFF)];
        if sniff.contains(&0) {
            self.skipped
                .push((name.to_owned(), "binary file".to_owned()));
            return;
        }
        self.findings.extend(
            detect::scan(bytes)
                .into_iter()
                .map(|f| (name.to_owned(), f)),
        );
    }
}

/// Reads standard input, refusing more than [`MAX_STDIN_BYTES`].
pub(crate) fn read_stdin(limit: usize) -> Result<Vec<u8>, String> {
    let mut buf = Vec::new();
    std::io::stdin()
        .lock()
        .take(u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1))
        .read_to_end(&mut buf)
        .map_err(|e| format!("cannot read standard input: {e}"))?;
    if buf.len() > limit {
        return Err(format!(
            "standard input is larger than {} MiB",
            limit / (1024 * 1024)
        ));
    }
    Ok(buf)
}

/// Scans `paths`; `-` (or no paths at all) is standard input.
pub(crate) fn run(paths: &[PathBuf]) -> Report {
    let mut report = Report::default();
    if paths.is_empty() {
        stdin(&mut report);
    }
    for path in paths {
        if path.as_os_str() == "-" {
            stdin(&mut report);
        } else {
            visit(path, 0, &mut report);
        }
    }
    report
}

fn stdin(report: &mut Report) {
    match read_stdin(MAX_STDIN_BYTES) {
        Ok(bytes) => report.text(STDIN, &bytes),
        Err(e) => report.errors.push((STDIN.to_owned(), e)),
    }
}

fn shown(path: &Path) -> String {
    path.display().to_string()
}

/// Scans one path. Symbolic links are never followed, so a walk cannot leave
/// the tree it was given, and `.git` directories are not entered.
fn visit(path: &Path, depth: usize, report: &mut Report) {
    let name = shown(path);
    let meta = match fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) => {
            report.errors.push((name, e.to_string()));
            return;
        }
    };
    let kind = meta.file_type();
    if kind.is_symlink() {
        report
            .skipped
            .push((name, "symbolic link (not followed)".to_owned()));
    } else if kind.is_dir() {
        if depth >= MAX_DEPTH {
            report
                .skipped
                .push((name, format!("deeper than {MAX_DEPTH} directories")));
            return;
        }
        let mut entries: Vec<PathBuf> = match fs::read_dir(path) {
            Ok(read) => read.filter_map(Result::ok).map(|e| e.path()).collect(),
            Err(e) => {
                report.errors.push((name, e.to_string()));
                return;
            }
        };
        entries.sort();
        for entry in entries {
            if entry.file_name().is_some_and(|n| n == ".git") {
                continue;
            }
            visit(&entry, depth + 1, report);
        }
    } else if kind.is_file() {
        if meta.len() > MAX_FILE_BYTES {
            report.skipped.push((
                name,
                format!("larger than {} MiB", MAX_FILE_BYTES / (1024 * 1024)),
            ));
            return;
        }
        match fs::read(path) {
            Ok(bytes) => report.text(&name, &bytes),
            Err(e) => report.errors.push((name, e.to_string())),
        }
    } else {
        // FIFOs, sockets and devices could block or never end.
        report.skipped.push((name, "not a regular file".to_owned()));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::samples::github;

    #[test]
    fn walks_directories_in_order_and_skips_git_and_binaries() {
        let dir = tempfile::tempdir().unwrap();
        let token = github(1);
        fs::create_dir_all(dir.path().join("b/.git")).unwrap();
        fs::write(dir.path().join("b/.git/config"), &token).unwrap();
        fs::write(dir.path().join("b/z.txt"), &token).unwrap();
        fs::write(dir.path().join("a.env"), format!("X={token}\n")).unwrap();
        let mut binary = token.clone().into_bytes();
        binary.push(0);
        fs::write(dir.path().join("c.bin"), binary).unwrap();

        let report = run(&[dir.path().to_path_buf()]);
        let names: Vec<String> = report
            .findings
            .iter()
            .map(|(n, _)| {
                Path::new(n)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        assert_eq!(names, ["a.env", "z.txt"]);
        assert_eq!(report.skipped.len(), 1, "{:?}", report.skipped);
        assert_eq!(report.errors, Vec::new());
    }

    #[cfg(unix)]
    #[test]
    fn symlinks_are_not_followed() {
        let dir = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("secret"), github(2)).unwrap();
        std::os::unix::fs::symlink(outside.path(), dir.path().join("link")).unwrap();
        let report = run(&[dir.path().to_path_buf()]);
        assert_eq!(report.findings, Vec::new());
        assert_eq!(report.skipped.len(), 1);
    }

    #[test]
    fn missing_paths_are_errors() {
        let report = run(&[PathBuf::from("definitely/not/here")]);
        assert_eq!(report.errors.len(), 1);
    }
}
