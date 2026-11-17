//! Repository automation: `cargo xtask <command>`.
//!
//! - `schema [--check]`: write or check the JSON Schemas under `schemas/`.
//! - `validate [--catalog DIR] [--base REV]`: validate the catalog. With
//!   `--base`, also require version bumps for assets changed since the merge
//!   base with `REV`.
//! - `pack-catalog [--check]`: write or check the catalog pack embedded in the
//!   binary (ADR-0001).
//!
//! Diagnostics go to stderr. Exit status: 0 success, 1 validation failure or
//! stale files, 2 usage error.

mod bumps;
mod git;
mod pack;
mod schema;

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "cargo xtask", about = "Grimoire repository automation")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate schemas/*.schema.json from the Rust types.
    Schema {
        /// Fail if the files on disk differ from the generated ones instead of writing them.
        #[arg(long)]
        check: bool,
    },
    /// Validate the catalog.
    Validate {
        /// Catalog directory, relative to the repository root.
        #[arg(long, default_value = "catalog")]
        catalog: PathBuf,
        /// Also require version bumps for assets changed since the merge base with this git revision.
        #[arg(long, value_name = "REV")]
        base: Option<String>,
    },
    /// Pack catalog/ into the archive embedded in the grimoire binary.
    PackCatalog {
        /// Fail if the pack on disk differs from a fresh one instead of writing it.
        #[arg(long)]
        check: bool,
    },
}

/// The repository root: the nearest ancestor of the current directory that holds
/// the workspace `Cargo.toml` and `xtask/`. Found at run time, so a binary built
/// from one checkout never validates or rewrites another.
fn repo_root() -> Result<PathBuf, String> {
    let cwd =
        std::env::current_dir().map_err(|e| format!("cannot read the current directory: {e}"))?;
    cwd.ancestors()
        .find(|dir| {
            dir.join("xtask").join("Cargo.toml").is_file()
                && std::fs::read_to_string(dir.join("Cargo.toml"))
                    .is_ok_and(|t| t.contains("[workspace]"))
        })
        .map(Path::to_path_buf)
        .ok_or_else(|| "run `cargo xtask` from inside the Grimoire repository".to_owned())
}

/// Resolves `catalog` against `root` and returns it with its repository-relative
/// path in `/` form, which is what git reports. Refuses paths outside the repository.
fn catalog_location(root: &Path, catalog: &Path) -> Result<(PathBuf, String), String> {
    let root = root
        .canonicalize()
        .map_err(|e| format!("cannot resolve {}: {e}", root.display()))?;
    let dir = root
        .join(catalog)
        .canonicalize()
        .map_err(|e| format!("cannot resolve the catalog {}: {e}", catalog.display()))?;
    let rel = dir.strip_prefix(&root).map_err(|_| {
        format!(
            "the catalog {} is outside the repository",
            catalog.display()
        )
    })?;
    let parts: Vec<&str> = rel
        .components()
        .map(|c| {
            c.as_os_str()
                .to_str()
                .ok_or("the catalog path is not UTF-8")
        })
        .collect::<Result<_, _>>()?;
    if parts.is_empty() {
        return Err("the catalog cannot be the repository root".to_owned());
    }
    let rel = parts.join("/");
    Ok((dir, rel))
}

#[allow(clippy::print_stderr, reason = "xtask reports diagnostics on stderr")]
fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = repo_root().and_then(|root| match cli.command {
        Command::Schema { check } => schema::run(&root, check),
        Command::Validate { catalog, base } => validate(&root, &catalog, base.as_deref()),
        Command::PackCatalog { check } => pack::run(&root, &root.join("catalog"), check),
    });
    match result {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

#[allow(clippy::print_stderr, reason = "xtask reports diagnostics on stderr")]
fn validate(root: &Path, catalog: &Path, base: Option<&str>) -> Result<ExitCode, String> {
    let (catalog_dir, display) = catalog_location(root, catalog)?;
    let validation = agent_grimoire_catalog::load(&catalog_dir);
    let mut diagnostics: Vec<_> = validation
        .diagnostics
        .into_iter()
        .map(|mut d| {
            d.file = Some(match d.file.take() {
                Some(file) => format!("{display}/{file}"),
                None => display.clone(),
            });
            d
        })
        .collect();
    if let Some(base) = base {
        diagnostics.extend(bumps::check(root, &display, base, &validation.catalog)?);
    }
    agent_grimoire_model::sort_diagnostics(&mut diagnostics);
    for d in &diagnostics {
        eprintln!("{d}");
    }
    let errors = diagnostics.iter().filter(|d| d.is_error()).count();
    let warnings = diagnostics.len() - errors;
    eprintln!(
        "catalog: {} assets, {} bundles; {errors} errors, {warnings} warnings",
        validation.catalog.assets().len(),
        validation.catalog.bundles().len(),
    );
    Ok(if errors > 0 {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_paths_are_normalized_and_confined() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        std::fs::create_dir_all(root.join("catalog")).unwrap();
        for spelling in ["catalog", "./catalog", "catalog/", "./catalog/."] {
            let (_, rel) = catalog_location(root, Path::new(spelling)).unwrap();
            assert_eq!(rel, "catalog", "{spelling}");
        }
        let absolute = root.join("catalog");
        assert_eq!(catalog_location(root, &absolute).unwrap().1, "catalog");
        assert!(catalog_location(root, Path::new(".")).is_err());
        assert!(catalog_location(root, Path::new("..")).is_err());
        assert!(catalog_location(root, Path::new("missing")).is_err());
    }
}
