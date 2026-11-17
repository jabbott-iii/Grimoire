//! `cargo xtask pack-catalog [--check]`: the catalog embedded in the binary (ADR-0001).

use std::fs;
use std::path::Path;
use std::process::ExitCode;

/// Where the pack lives, relative to the repository root. It must sit inside the
/// catalog crate so that `cargo publish` packages it.
pub(crate) const PACK_DIR: &str = "crates/agent-grimoire-catalog";
/// The pack's file name inside [`PACK_DIR`].
pub(crate) const PACK_FILE: &str = "catalog.pack";

#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
pub(crate) fn run(root: &Path, catalog: &Path, check: bool) -> Result<ExitCode, String> {
    let bytes = match agent_grimoire_catalog::pack(catalog) {
        Ok(bytes) => bytes,
        Err(diagnostics) => {
            for d in &diagnostics {
                eprintln!("{d}");
            }
            eprintln!("error: the catalog is invalid; run `cargo xtask validate`");
            return Ok(ExitCode::FAILURE);
        }
    };
    let dir = root.join(PACK_DIR);
    let shown = format!("{PACK_DIR}/{PACK_FILE}");
    if check {
        if fs::read(dir.join(PACK_FILE)).ok().as_deref() == Some(bytes.as_slice()) {
            eprintln!(
                "pack-catalog: {shown} is up to date ({} bytes)",
                bytes.len()
            );
            Ok(ExitCode::SUCCESS)
        } else {
            eprintln!("error: {shown} is out of date. Run `cargo xtask pack-catalog`.");
            Ok(ExitCode::FAILURE)
        }
    } else {
        crate::schema::write_atomic(&dir, PACK_FILE, &bytes)?;
        eprintln!("pack-catalog: wrote {shown} ({} bytes)", bytes.len());
        Ok(ExitCode::SUCCESS)
    }
}
