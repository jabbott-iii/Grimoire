//! Minimal, shell-free git invocations.

use std::path::Path;
use std::process::Command;

/// Runs `git <args>` in `repo` and returns stdout, or a readable error.
pub(crate) fn run(repo: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["-c", "core.quotepath=off"])
        .args(args)
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "`git {}` failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr).trim()
        ))
    }
}

/// Splits NUL-separated `-z` output into UTF-8 paths.
pub(crate) fn nul_paths(raw: &[u8]) -> Result<Vec<String>, String> {
    raw.split(|b| *b == 0)
        .filter(|p| !p.is_empty())
        .map(|p| {
            String::from_utf8(p.to_vec()).map_err(|_| "git reported a non-UTF-8 path".to_owned())
        })
        .collect()
}

/// The contents of `path` at `rev`, or `None` if it does not exist there.
pub(crate) fn show(repo: &Path, rev: &str, path: &str) -> Option<String> {
    run(repo, &["show", &format!("{rev}:{path}")])
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}
