//! `cargo xtask schema [--check]`.

use std::collections::BTreeSet;
use std::fs;
use std::io::Write as _;
use std::path::Path;
use std::process::ExitCode;

/// Renders a schema the way it is stored: pretty JSON with a trailing newline.
pub(crate) fn render(schema: &schemars::Schema) -> Result<String, String> {
    let mut text = serde_json::to_string_pretty(schema)
        .map_err(|e| format!("cannot serialize schema: {e}"))?;
    text.push('\n');
    Ok(text)
}

#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
pub(crate) fn run(root: &Path, check: bool) -> Result<ExitCode, String> {
    let dir = root.join("schemas");
    let documents = agent_grimoire_model::schema::documents();
    let expected: BTreeSet<&str> = documents.iter().map(|d| d.file_name).collect();
    let mut stale = Vec::new();
    for doc in &documents {
        let text = render(&doc.schema)?;
        let path = dir.join(doc.file_name);
        if check {
            if fs::read_to_string(&path).ok().as_deref() != Some(text.as_str()) {
                stale.push(doc.file_name.to_owned());
            }
        } else {
            write_atomic(&dir, doc.file_name, &text)?;
        }
    }
    // Leftover schema files from renamed or removed types are stale too.
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".schema.json") && !expected.contains(name.as_str()) {
                if check {
                    stale.push(name);
                } else {
                    fs::remove_file(entry.path())
                        .map_err(|e| format!("cannot remove schemas/{name}: {e}"))?;
                }
            }
        }
    }
    if stale.is_empty() {
        if check {
            eprintln!("schemas: {} files up to date", documents.len());
        } else {
            eprintln!("schemas: wrote {} files", documents.len());
        }
        Ok(ExitCode::SUCCESS)
    } else {
        eprintln!(
            "error: schemas/ is out of date: {}. Run `cargo xtask schema`.",
            stale.join(", ")
        );
        Ok(ExitCode::FAILURE)
    }
}

/// Writes `dir/name` through a temporary file in the same directory, then renames it.
fn write_atomic(dir: &Path, name: &str, text: &str) -> Result<(), String> {
    fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
    let mut tmp = tempfile::NamedTempFile::new_in(dir)
        .map_err(|e| format!("cannot create a temporary file: {e}"))?;
    // Temporary files are created 0600; generated files are ordinary readable files.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        tmp.as_file()
            .set_permissions(fs::Permissions::from_mode(0o644))
            .map_err(|e| format!("cannot set permissions on schemas/{name}: {e}"))?;
    }
    tmp.write_all(text.as_bytes())
        .and_then(|()| tmp.as_file().sync_all())
        .map_err(|e| format!("cannot write schemas/{name}: {e}"))?;
    tmp.persist(dir.join(name))
        .map_err(|e| format!("cannot replace schemas/{name}: {e}"))?;
    Ok(())
}
