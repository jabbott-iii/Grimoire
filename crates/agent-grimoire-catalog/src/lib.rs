//! Catalog loading and catalog-wide validation for Grimoire, and the catalog
//! pack that is built into the `grimoire` binary.
//!
//! The catalog is the collection of assets Grimoire can install: rules, skills,
//! agents, tools and Model Context Protocol (MCP) servers, one directory per
//! asset under `catalog/` in the Grimoire repository. [`load`] reads that
//! directory and runs every check that needs more than one document. [`pack`]
//! turns a valid catalog into one deterministic file, and [`Source`] loads such a
//! file into the in-memory index that the command-line interface (CLI) installs
//! from.
//!
//! # Where it fits
//!
//! The crate builds on `agent-grimoire-model`, which holds the strict types for
//! `asset.toml` and bundle files and the checks that need only one document.
//! This crate adds the checks that need the files on disk or the whole catalog.
//!
//! The `grimoire` binary, in the `agent-grimoire` crate, calls
//! [`Source::builtin`] once when it starts and passes the [`Source`] to every
//! command. If the pack doesn't load, the CLI reports that its build is broken
//! and exits with code 1. Two libraries read the [`Source`] for it:
//! - `agent-grimoire-resolve` looks up assets and bundles in it, to work out
//!   which assets and versions to install;
//! - `agent-grimoire-adapters` reads each asset's files from its
//!   [`SourceAsset`], to write each client's files.
//!
//! [`load`] and [`pack`] run in the Grimoire repository, not on a user's
//! machine. `cargo xtask validate` runs [`load`] and, when the catalog is
//! valid, renders every asset through the adapters. `cargo xtask pack-catalog`
//! runs [`pack`] and writes the result to `catalog.pack` in this crate, which
//! embeds that file, so every build of the CLI carries it. Version-bump checks
//! need git history and live in `xtask` (`cargo xtask validate --base`), not
//! here.
//!
//! The crate is internal to the CLI. It is published only so that
//! `cargo install agent-grimoire` can build the CLI, and its API changes
//! whenever the CLI needs it to, so don't depend on it
//! ([`docs/05-release.md`, The public API](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#the-public-api)).
//!
//! The design is in
//! [`docs/subsystems/catalog.md`, Catalog validation](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/catalog.md#catalog-validation),
//! which lists the checks per asset, per kind and across the catalog, and in
//! [ADR-0001, Catalog delivery](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md),
//! which builds the catalog into the binary behind a `Source` (below, "Why the
//! catalog is built in").
//!
//! # The parts
//!
//! - **Validating a catalog directory:** [`load`] returns a [`Validation`]: the
//!   [`Catalog`] that parsed, and its [`diagnostics`](Validation::diagnostics).
//!   [`Validation::has_errors`] says whether any diagnostic is an error. The
//!   [`Catalog`] holds the assets ([`Catalog::assets`], each a
//!   [`CatalogAsset`]) and the bundles ([`Catalog::bundles`], each a
//!   [`CatalogBundle`]). [`MAX_FILE_BYTES`] is the largest file the catalog
//!   accepts, and [`changelog_has`] says whether a `CHANGELOG.md` has an entry
//!   for a version.
//! - **Packing:** [`pack`] validates a catalog with [`load`] and, when there is
//!   no error, packs it.
//! - **Reading a pack:** [`Source::builtin`] loads the pack built into this
//!   crate, and [`Source::from_pack`] loads any pack. A [`Source`] gives its
//!   assets ([`Source::asset`], [`Source::assets`]), its bundles
//!   ([`Source::bundle`], [`Source::bundles`]) and its identity
//!   ([`Source::id`]), which `grimoire.lock` records as `source`.
//! - **One asset in a pack:** a [`SourceAsset`] holds the asset's
//!   [`manifest`](SourceAsset::manifest), the SHA-256
//!   [`digest`](SourceAsset::digest) (a hash) of its manifest file, which
//!   `grimoire.lock` also records, and its files: [`SourceAsset::file`],
//!   [`SourceAsset::files_under`], [`SourceAsset::readme`] and
//!   [`SourceAsset::changelog`].
//! - **Errors:** a pack that doesn't load gives a [`SourceError`].
//!
//! # Example
//!
//! The catalog built into this crate, loaded as `grimoire` loads it, and one
//! asset in it:
//!
//! ```
//! use agent_grimoire_catalog::Source;
//! use agent_grimoire_model::{AssetId, Kind, RelPath, SourceId};
//!
//! let source = Source::builtin()?;
//! assert_eq!(source.id(), SourceId::Builtin);
//!
//! let id = AssetId::new("plain-language-docs")?;
//! let asset = source.asset(&id).ok_or("not in the catalog")?;
//! assert_eq!(asset.manifest.kind, Kind::Rule);
//!
//! // Beside the files it installs, the pack holds the asset's own README.md
//! // and CHANGELOG.md, whose latest entry `grimoire show` prints.
//! let rule = RelPath::new("rule.md")?;
//! assert!(asset.file(&rule).is_some());
//! assert!(asset.readme().is_some() && asset.changelog().is_some());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # What `load` checks
//!
//! [`load`] runs every check that needs more than one document, and also the
//! model's single-document checks on each manifest and bundle:
//! - **Directory layout and file hygiene.** The catalog root may hold only
//!   `README.md`, `bundles/` and one directory per kind (`rules`, `skills`,
//!   `agents`, `tools`, `mcp-servers`). Each asset's directory is walked without
//!   following symbolic links (symlinks), and:
//!   - symlinks and special files are refused: only regular files and
//!     directories are allowed. A file is checked before it is opened, because
//!     opening a FIFO (a named pipe) would block;
//!   - every path must be a portable
//!     [`RelPath`](agent_grimoire_model::RelPath), the same on Linux, macOS and
//!     Windows, and at most [`MAX_REL_PATH`](agent_grimoire_model::MAX_REL_PATH)
//!     (200) characters counted from the catalog root, such as
//!     `skills/<id>/references/guide.md`, because the pack stores each file
//!     under that path;
//!   - no two paths may differ only in letter case, such as `skill.md` and
//!     `SKILL.md`. They collide on macOS and Windows, where one would silently
//!     replace the other on install;
//!   - size limits: at most [`MAX_FILE_BYTES`] (1 MiB) per file, because assets
//!     ship inside the binary, enforced while reading and not only reported; at
//!     most 2,000 entries per asset; at most 12 levels of nesting.
//! - **Required metadata files and changelog entries.** Each asset needs an
//!   `asset.toml` whose `id` is its directory's name and whose `kind` matches
//!   its parent directory, a non-empty `README.md` and `CHANGELOG.md`, and
//!   `evals/` or `tests/` with at least one file. Tools and MCP servers need
//!   `tests/`. The changelog needs a level-2 heading for the current version
//!   ([`changelog_has`]).
//! - **Output sources.** Every `src` file or directory an output names exists.
//! - **Each kind's own rules.**
//!   - A rule stays under 500 lines, with a warning from 200. A rule can be
//!     installed as a managed block (a marked section Grimoire owns) in a
//!     shared file such as `AGENTS.md`, so it may not start a line with
//!     Grimoire's `<!-- grimoire:` marker or end inside an open code fence,
//!     either of which would break the block around it. It may not start with
//!     frontmatter either (a YAML block between `---` lines at the top of a
//!     Markdown file), which a client would read as its own.
//!   - An agent's prompt is at most 30,000 characters, GitHub Copilot's limit.
//!   - Skills follow the Agent Skills rules, checked natively (below).
//!   - Tools and MCP servers are Rust crates
//!     ([ADR-0002](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0002-mcp-server-and-tool-packaging.md),
//!     which ships them as separate binaries), and follow the crate rules: for
//!     example exactly one binary, the asset's own version, `publish = false`,
//!     no dependency on Grimoire's core crates, and, for MCP servers, a
//!     dependency on the shared hardening crate `agent-grimoire-mcp`.
//! - **Dependencies, cycles, deprecations and bundles.** Ids are unique across
//!   assets and bundles. Each dependency exists, matches its version
//!   requirement and forms no cycle. A deprecated asset names a replacement
//!   that exists. Bundle members exist, and no bundle holds assets that
//!   conflict.
//! - **Duplicate rules.** Two rules with the same text, or nearly the same, are
//!   reported. Rules that contradict each other can't be found this way: they
//!   declare `conflicts` instead.
//!
//! # Agent Skills, checked natively
//!
//! Skills follow the Agent Skills specification, the open format for skills
//! ([`docs/specs.md`, SKILLS](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#skills-agent-skills-specification)).
//! This crate checks them in Rust rather than with the specification's Python
//! reference validator, `skills-ref`, which describes itself as a
//! demonstration. Where the specification only recommends, the checks are
//! deliberately stricter:
//! - unknown frontmatter keys are errors, and the frontmatter's YAML is read
//!   strictly: duplicate keys are refused at every level, and an unquoted
//!   `123`, `true` or `~` is not taken as a string;
//! - `allowed-tools` is refused, because it pre-approves tools outside
//!   Grimoire's permission consent;
//! - every bundled script must be referenced from `SKILL.md`, and a skill with
//!   scripts must declare the `shell` permission;
//! - `SKILL.md` must stay under 500 lines.
//!
//! A skill may not contain `.claude-plugin/`, because Claude Code would load the
//! installed skill as a plugin, with its own hooks and MCP servers.
//!
//! # Errors and diagnostics
//!
//! [`load`] never fails. Each problem is a [`Diagnostic`], an error or a
//! warning with a stable code such as `catalog/unexpected-entry`. Its file path
//! is relative to the catalog root, and the list is sorted by file. Test the
//! code, not the message text. The CLI never calls [`load`]. When it loads
//! its pack, it runs only the model's single-document checks again (below,
//! "The catalog pack") and reports a failure as a broken build, without a
//! code. So only `cargo xtask` commands print these codes, for catalog
//! authors, and the codes aren't part of Grimoire's public API
//! ([`docs/diagnostics.md`, Not listed here](https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md#not-listed-here)).
//!
//! The [`Catalog`] may be partial: an asset whose `asset.toml` is missing or
//! doesn't parse is left out, and the others are still checked.
//!
//! [`pack`] returns every diagnostic instead of a pack when [`load`] finds an
//! error, so an invalid catalog can never be embedded. It also fails, with
//! `pack/read` or `pack/limits`, when a file can't be read again or the pack
//! would break its limits, such as 100,000 files. [`Source::from_pack`]
//! returns a [`SourceError`] for a malformed pack or an invalid document in it.
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let dir = tempfile::tempdir()?;
//! # let root = dir.path();
//! use agent_grimoire_catalog::{Source, load, pack};
//! use agent_grimoire_model::SourceId;
//!
//! // The catalog root may hold only README.md, bundles/ and the kind directories.
//! std::fs::write(root.join("notes.txt"), "stray")?;
//! let validation = load(root);
//! assert!(validation.has_errors());
//! let found: Vec<_> = validation
//!     .diagnostics
//!     .iter()
//!     .map(|d| (d.code, d.file.as_deref()))
//!     .collect();
//! assert_eq!(found, [("catalog/unexpected-entry", Some("notes.txt"))]);
//! assert!(pack(root).is_err());
//!
//! // An empty catalog is valid: it packs, and the pack loads.
//! std::fs::remove_file(root.join("notes.txt"))?;
//! let bytes = pack(root).map_err(|d| format!("{d:?}"))?;
//! let source = Source::from_pack(SourceId::Builtin, &bytes)?;
//! assert!(source.assets().is_empty());
//!
//! // Bytes that aren't a pack are refused.
//! assert!(Source::from_pack(SourceId::Builtin, b"not a pack").is_err());
//! # Ok(())
//! # }
//! ```
//!
//! # The catalog pack
//!
//! A pack is a deterministic, binary-safe list of files: the same catalog always
//! gives the same bytes, so continuous integration (CI) can check that
//! `catalog.pack` is up to date (`cargo xtask pack-catalog --check`). It holds
//! only these files: each asset's `asset.toml`, `README.md` and
//! `CHANGELOG.md`, the files its outputs install, and every bundle. Its format,
//! `grimoire catalog pack v1`, isn't part of Grimoire's public API.
//!
//! [`Source::from_pack`] reads a pack as untrusted input:
//! - at most 100,000 files, each at most [`MAX_FILE_BYTES`];
//! - every path a portable relative path from the catalog root, of at most
//!   [`MAX_REL_PATH`](agent_grimoire_model::MAX_REL_PATH) characters, in
//!   strictly sorted order, so no path appears twice. [`load`] applies the same
//!   path rule to every catalog file, so a pack that [`pack`] writes never fails
//!   it;
//! - every manifest and bundle parsed strictly and checked again on its own,
//!   so a damaged pack fails when it is loaded rather than at install time.
//!   Each manifest's `id` and `kind` must match its directory, and no id may be
//!   used twice. The checks across the catalog aren't run again: [`pack`]
//!   runs them before it packs.
//!
//! [`Source::builtin`] fails only if the embedded pack is corrupt, which CI
//! prevents.
//!
//! # Why the catalog is built in
//!
//! [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md)
//! chose to build the catalog into the binary for the first version:
//! - installing needs no network code, and works offline;
//! - the catalog is covered by the binary's own checksums, signatures, software
//!   bill of materials (SBOM, the list of what the binary contains) and
//!   provenance (the signed record of how it was built), so users have no
//!   second key or identity to trust;
//! - the CLI version and the lockfile together determine what is installed.
//!
//! The cost is that a new or changed asset reaches users only in a CLI release,
//! and that a pack holds one version of each asset. A newer CLI therefore can't
//! write the files again for a lockfile that pins an older version, though
//! `grimoire verify`, which needs only the recorded hashes, still works.
//!
//! The pack lives in this crate because `cargo publish` packages only files
//! under the crate's own directory, so a build script that read `../../catalog`
//! would break `cargo install`. That is also why the CLI should be installed
//! with `cargo install agent-grimoire --locked`: on Cargo 1.96 to 1.99, a build
//! without `--locked` can take a later version of this crate within the same
//! 0.y, and so newer assets than the CLI version it reports
//! ([`docs/05-release.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#the-minimum-supported-rust-version)).
//!
//! [`Source`] is where a later remote catalog would plug in. ADR-0001 leaves
//! that to a future decision. Such a source would build the same index, so
//! nothing that reads a [`Source`] would change. Today its only identity is
//! [`SourceId::Builtin`](agent_grimoire_model::SourceId::Builtin).

mod asset;
mod code;
mod cross;
mod fs;
mod pack;
mod rules;
mod skill;
mod source;
mod yaml;

// The README's examples are compiled and run as doc tests too, so the
// crates.io page breaks the build when the API changes under it.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

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
