//! Catalog loading and validation (`docs/architecture.md` §6.4).
//!
//! [`load`] reads a catalog directory, the one at `catalog/` in this
//! repository, and runs every check that needs more than one document:
//! - directory layout and file hygiene (no symlinks, portable names, size limits);
//! - required metadata files and changelog entries;
//! - output sources;
//! - Agent Skills rules, checked natively;
//! - crate rules for tools and MCP servers;
//! - dependencies, cycles, deprecations and bundles;
//! - duplicate rules.
//!
//! Version-bump checks need git history and live in `xtask`.
//!
//! [`pack`] turns a validated catalog into the deterministic pack embedded in the
//! binary, and [`Source`] loads a pack into the index the resolver and installer
//! read (ADR-0001).

mod asset;
mod code;
mod cross;
mod fs;
mod pack;
mod rules;
mod skill;
mod source;
mod yaml;

use std::collections::BTreeMap;
use std::path::Path;

use agent_grimoire_model::{AssetId, AssetManifest, Bundle, Diagnostic, Kind, validate};

pub use asset::changelog_has;
pub use fs::MAX_FILE_BYTES;
pub use pack::pack;
pub use source::{Source, SourceAsset, SourceError};

/// A loaded catalog. It may be partial when validation found errors.
#[derive(Debug, Default)]
pub struct Catalog {
    assets: BTreeMap<AssetId, CatalogAsset>,
    bundles: BTreeMap<AssetId, CatalogBundle>,
}

impl Catalog {
    /// Assets by id.
    #[must_use]
    pub fn assets(&self) -> &BTreeMap<AssetId, CatalogAsset> {
        &self.assets
    }

    /// Bundles by id.
    #[must_use]
    pub fn bundles(&self) -> &BTreeMap<AssetId, CatalogBundle> {
        &self.bundles
    }
}

/// An asset that parsed.
#[derive(Debug)]
pub struct CatalogAsset {
    /// Directory relative to the catalog root, e.g. `rules/rust-errors`.
    pub dir: String,
    /// The parsed manifest.
    pub manifest: AssetManifest,
}

/// A bundle that parsed.
#[derive(Debug)]
pub struct CatalogBundle {
    /// File relative to the catalog root, e.g. `bundles/rust-cli.toml`.
    pub file: String,
    /// The parsed bundle.
    pub bundle: Bundle,
}

/// The result of loading a catalog: what parsed, and every finding.
#[derive(Debug, Default)]
pub struct Validation {
    /// The assets and bundles that parsed.
    pub catalog: Catalog,
    /// Errors and warnings, sorted by file. File paths are relative to the catalog root.
    pub diagnostics: Vec<Diagnostic>,
}

impl Validation {
    /// Whether any diagnostic is an error.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        agent_grimoire_model::has_errors(&self.diagnostics)
    }
}

/// Loads and validates the catalog at `root`.
#[must_use]
pub fn load(root: &Path) -> Validation {
    let mut out = Vec::new();
    let mut catalog = Catalog::default();
    let mut rule_texts = Vec::new();

    let entries = match fs::read_dir_sorted(root) {
        Ok(entries) => entries,
        Err(e) => {
            out.push(Diagnostic::error(
                "catalog/unreadable",
                format!("cannot read the catalog root: {e}"),
            ));
            return Validation {
                catalog,
                diagnostics: out,
            };
        }
    };
    for (name, meta) in entries {
        if meta.file_type().is_symlink() {
            out.push(
                Diagnostic::error("catalog/symlink", "symlinks are not allowed in the catalog")
                    .in_file(name),
            );
        } else if name == "README.md" && meta.is_file() {
            // Documentation for contributors.
        } else if name == "bundles" && meta.is_dir() {
            load_bundles(root, &mut catalog, &mut out);
        } else if let Some(kind) = Kind::from_catalog_dir(&name).filter(|_| meta.is_dir()) {
            load_kind(root, kind, &mut catalog, &mut rule_texts, &mut out);
        } else {
            out.push(
                Diagnostic::error(
                    "catalog/unexpected-entry",
                    "the catalog root may contain only README.md, bundles/ and one directory per kind (rules, skills, agents, tools, mcp-servers)",
                )
                .in_file(name),
            );
        }
    }
    cross::check(&catalog.assets, &catalog.bundles, &mut out);
    rules::check(&rule_texts, &mut out);
    agent_grimoire_model::sort_diagnostics(&mut out);
    Validation {
        catalog,
        diagnostics: out,
    }
}

fn load_kind(
    root: &Path,
    kind: Kind,
    catalog: &mut Catalog,
    rule_texts: &mut Vec<rules::RuleText>,
    out: &mut Vec<Diagnostic>,
) {
    let dir = root.join(kind.catalog_dir());
    let entries = match fs::read_dir_sorted(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            out.push(
                Diagnostic::error("catalog/unreadable", format!("cannot read directory: {e}"))
                    .in_file(kind.catalog_dir()),
            );
            return;
        }
    };
    for (name, meta) in entries {
        let shown = format!("{}/{name}", kind.catalog_dir());
        if meta.file_type().is_symlink() {
            out.push(
                Diagnostic::error("catalog/symlink", "symlinks are not allowed in the catalog")
                    .in_file(shown),
            );
            continue;
        }
        if !meta.is_dir() {
            out.push(
                Diagnostic::error(
                    "catalog/unexpected-entry",
                    format!(
                        "catalog/{}/ may contain only asset directories",
                        kind.catalog_dir()
                    ),
                )
                .in_file(shown),
            );
            continue;
        }
        let id = match AssetId::new(&name) {
            Ok(id) => id,
            Err(e) => {
                out.push(Diagnostic::error("catalog/asset-dir-name", e.to_string()).in_file(shown));
                continue;
            }
        };
        let Some(loaded) = asset::load(root, kind, &id, out) else {
            continue;
        };
        if let Some(existing) = catalog.assets.get(&id) {
            out.push(
                Diagnostic::error(
                    "catalog/duplicate-id",
                    format!("`{id}` is already used by {}", existing.dir),
                )
                .in_file(loaded.dir),
            );
            continue;
        }
        rule_texts.extend(loaded.rule_texts);
        catalog.assets.insert(
            id,
            CatalogAsset {
                dir: loaded.dir,
                manifest: loaded.manifest,
            },
        );
    }
}

fn load_bundles(root: &Path, catalog: &mut Catalog, out: &mut Vec<Diagnostic>) {
    let entries = match fs::read_dir_sorted(&root.join("bundles")) {
        Ok(entries) => entries,
        Err(e) => {
            out.push(
                Diagnostic::error("catalog/unreadable", format!("cannot read directory: {e}"))
                    .in_file("bundles"),
            );
            return;
        }
    };
    for (name, meta) in entries {
        let file = format!("bundles/{name}");
        let stem = name.strip_suffix(".toml");
        if meta.file_type().is_symlink() {
            out.push(
                Diagnostic::error("catalog/symlink", "symlinks are not allowed in the catalog")
                    .in_file(file),
            );
            continue;
        }
        let Some(stem) = stem.filter(|_| meta.is_file()) else {
            out.push(
                Diagnostic::error(
                    "catalog/unexpected-entry",
                    "bundles/ may contain only <id>.toml files",
                )
                .in_file(file),
            );
            continue;
        };
        let bundle = match fs::read_text(&root.join("bundles").join(&name))
            .and_then(|text| Bundle::from_toml(&text).map_err(|e| e.to_string()))
        {
            Ok(bundle) => bundle,
            Err(message) => {
                out.push(Diagnostic::error("catalog/bundle-parse", message).in_file(file));
                continue;
            }
        };
        if bundle.id.as_str() != stem {
            out.push(
                Diagnostic::error(
                    "catalog/bundle-id",
                    format!("id must equal the file name `{stem}`"),
                )
                .at("id")
                .in_file(&file),
            );
        }
        out.extend(
            validate::bundle(&bundle)
                .into_iter()
                .map(|d| d.in_file(&file)),
        );
        if catalog.bundles.contains_key(&bundle.id) {
            out.push(
                Diagnostic::error(
                    "catalog/duplicate-id",
                    format!("bundle `{}` is defined twice", bundle.id),
                )
                .in_file(file),
            );
            continue;
        }
        catalog
            .bundles
            .insert(bundle.id.clone(), CatalogBundle { file, bundle });
    }
}
