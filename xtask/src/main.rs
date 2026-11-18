//! Repository automation: `cargo xtask <command>`.
//!
//! - `schema [--check]`: write or check the JSON Schemas under `schemas/`.
//! - `validate [--catalog DIR] [--base REV]`: validate the catalog. With
//!   `--base`, also require version bumps for assets changed since the merge
//!   base with `REV`.
//! - `pack-catalog [--check]`: write or check the catalog pack embedded in the
//!   binary (ADR-0001).
//! - `new <kind> <id> --maintainer LOGIN [MEMBER]...`: scaffold an asset, or a
//!   bundle of existing assets, that validates as generated.
//! - `release <plan|build|sbom|channels|checksums|verify|notes>`: the release
//!   pipeline's steps (ADR-0009). Paths are relative to the repository root.
//!
//! Diagnostics go to stderr. Exit status: 0 success, 1 validation failure or
//! stale files, 2 usage error.

mod bumps;
mod git;
mod marketplace;
mod new;
mod pack;
#[cfg(test)]
mod readme_links;
mod release;
mod render;
mod schema;
#[cfg(test)]
mod workflow_permissions;

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
    /// Generate the Claude Code plugin marketplace from catalog/.
    GenMarketplace {
        /// Fail if the files on disk differ from the generated ones instead of writing them.
        #[arg(long)]
        check: bool,
    },
    /// Scaffold a new asset or bundle under catalog/.
    New {
        /// rule, skill, agent, tool, mcp-server or bundle.
        #[arg(value_parser = new::What::parse)]
        kind: new::What,
        /// The new id, unique across the catalog.
        #[arg(value_parser = parse_id)]
        id: agent_grimoire_model::AssetId,
        /// GitHub login of the maintainer.
        #[arg(long, value_parser = parse_login)]
        maintainer: agent_grimoire_model::GithubLogin,
        /// For bundles: the member assets.
        members: Vec<String>,
        /// Catalog directory, relative to the repository root.
        #[arg(long, default_value = "catalog")]
        catalog: PathBuf,
    },
    /// Release pipeline steps, run by .github/workflows/release.yml.
    Release {
        #[command(subcommand)]
        command: release::Command,
    },
}

fn parse_id(value: &str) -> Result<agent_grimoire_model::AssetId, String> {
    agent_grimoire_model::AssetId::new(value).map_err(|e| e.to_string())
}

fn parse_login(value: &str) -> Result<agent_grimoire_model::GithubLogin, String> {
    agent_grimoire_model::GithubLogin::new(value).map_err(|e| e.to_string())
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
        Command::GenMarketplace { check } => marketplace::run(&root, &root.join("catalog"), check),
        Command::New {
            kind,
            id,
            maintainer,
            members,
            catalog,
        } => new_asset(&root, &catalog, kind, &id, &maintainer, &members),
        Command::Release { command } => release::run(&root, command),
    });
    match result {
        Ok(code) => code,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

#[allow(clippy::print_stderr, reason = "xtask reports results on stderr")]
fn new_asset(
    root: &Path,
    catalog: &Path,
    kind: new::What,
    id: &agent_grimoire_model::AssetId,
    maintainer: &agent_grimoire_model::GithubLogin,
    members: &[String],
) -> Result<ExitCode, String> {
    let (catalog_dir, display) = catalog_location(root, catalog)?;
    let written = new::scaffold(&catalog_dir, kind, id, maintainer, members)?;
    for file in &written {
        eprintln!("new: wrote {display}/{file}");
    }
    // Line wrapping depends on the id's length, so format what was generated.
    let rust: Vec<PathBuf> = written
        .iter()
        .filter(|f| {
            Path::new(f.as_str())
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("rs"))
        })
        .map(|f| catalog_dir.join(f))
        .collect();
    if !rust.is_empty() {
        let formatted = std::process::Command::new("rustfmt")
            .args(["--edition", "2024"])
            .args(&rust)
            .status();
        if !formatted.is_ok_and(|s| s.success()) {
            eprintln!("new: could not run rustfmt; run `cargo fmt` before committing");
        }
    }
    eprintln!(
        "new: fill in the content, then run `cargo xtask validate` and `cargo xtask pack-catalog`"
    );
    Ok(ExitCode::SUCCESS)
}

#[allow(clippy::print_stderr, reason = "xtask reports diagnostics on stderr")]
fn validate(root: &Path, catalog: &Path, base: Option<&str>) -> Result<ExitCode, String> {
    let (catalog_dir, display) = catalog_location(root, catalog)?;
    let validation = agent_grimoire_catalog::load(&catalog_dir);
    let valid = !validation.has_errors();
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
    if valid {
        diagnostics.extend(render::check(&catalog_dir, &display));
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
