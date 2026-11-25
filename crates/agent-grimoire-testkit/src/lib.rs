//! Fixture catalogs and projects for Grimoire's own tests. Never published.
//!
//! A fixture is a small, known input made for a test. [`FixtureCatalog`]
//! writes valid assets into a temporary catalog directory and packs it into a
//! [`Source`], so tests exercise the same loading code as the embedded catalog.
//! [`Project`] is a temporary user repository to install them into.
//!
//! # Where it fits
//!
//! - **Who uses it.** It is a dev-dependency (a dependency of tests only) of
//!   `agent-grimoire`, the CLI, whose end-to-end tests are in `tests/cli.rs`,
//!   `tests/clients.rs` and `tests/builtin.rs`; of `agent-grimoire-resolve`
//!   and `agent-grimoire-adapters`; and of `xtask`, the repository's own build
//!   tasks. The CLI is a library (`agent_grimoire::run`) plus a thin binary
//!   so that these tests can drive it with a fixture catalog; each test then
//!   inspects the files the CLI wrote in the project directory.
//! - **What it builds on.** `agent-grimoire-catalog`, whose
//!   [`agent_grimoire_catalog::pack`] validates and packs a catalog directory
//!   and whose [`Source`] loads the pack; `agent-grimoire-model`, for the
//!   source's identity; and `tempfile`, for the directories.
//! - **Never published.** Its `Cargo.toml` sets `publish = false`, and the
//!   workspace refers to it by path only, so it has no crates.io or docs.rs
//!   page; `cargo doc -p agent-grimoire-testkit` builds these docs locally. It
//!   exists only for Grimoire's own tests: every crate that uses it lists it
//!   under `[dev-dependencies]` only, so no published crate's library or
//!   binary depends on it. `cargo publish --workspace` skips it at release time
//!   ([`docs/05-release.md`, During the first release run, During the first release run][release-2d]).
//! - **Design.** [`docs/01-architecture.md`, Repository layout, Repository layout][architecture-4]
//!   places the fixtures in this crate (its Phase 2 notes) and explains why
//!   the CLI is a library plus a binary.
//!
//! # The parts
//!
//! - **Catalogs.** [`FixtureCatalog::new`] is an empty catalog, with only the
//!   catalog's own `README.md`. Each builder below writes one complete asset,
//!   or a bundle (a named set of assets, each with a version requirement), and
//!   returns the catalog so that calls can be chained:
//!   - [`FixtureCatalog::rule`], for the `claude-code` and `agents-md` clients; empty
//!     `paths` means always on;
//!   - [`FixtureCatalog::skill`], with a reference file, for `claude-code` and
//!     `agents-md`;
//!   - [`FixtureCatalog::agent`], for `claude-code`, with the permissions its
//!     tools imply: filesystem read for `read` or `search`, filesystem write
//!     for `edit`, a shell for `execute` and the network for `web`;
//!   - [`FixtureCatalog::mcp_server`], for `claude-code`: one read-only
//!     `search` tool, and the given environment variables as required
//!     secrets;
//!   - [`FixtureCatalog::tool`], for `claude-code`: one pre-tool-use hook on
//!     edits;
//!   - [`FixtureCatalog::bundle`], a bundle of `(id, requirement)` members.
//!
//!   [`FixtureCatalog::asset`] writes an asset of any kind from a manifest
//!   fragment. [`FixtureCatalog::write`] and [`FixtureCatalog::edit_manifest`]
//!   change files afterwards, for example to give a test an invalid or unusual
//!   asset. [`FixtureCatalog::source`] packs and loads the catalog.
//! - **Projects.** [`Project::new`] is an empty directory.
//!   [`Project::write`] and [`Project::read`] write and read one file,
//!   [`Project::exists`] says whether a path exists, [`Project::tree`] lists
//!   everything below the root, and [`Project::path`] and [`Project::root`]
//!   give absolute paths.
//!
//! # Example
//!
//! A catalog like the one the CLI's end-to-end tests use, and a project with
//! content of its own:
//!
//! ```
//! use agent_grimoire_model::AssetId;
//! use agent_grimoire_testkit::{FixtureCatalog, Project};
//!
//! let catalog = FixtureCatalog::new();
//! catalog
//!     .rule("commit-style", "1.0.0", &[], "# Commits\n\nWrite Conventional Commits.\n")
//!     .rule("rust-errors", "1.2.0", &["**/*.rs"], "# Errors\n\nUse typed errors.\n")
//!     .skill("adr-writer", "0.3.1")
//!     .agent("security-reviewer", "1.0.0", &["read", "search"])
//!     .tool("secret-guard", "1.0.0")
//!     .mcp_server("docs-index", "2.0.0", &["DOCS_TOKEN"])
//!     .bundle(
//!         "security-baseline",
//!         &[("security-reviewer", "^1"), ("secret-guard", "^1")],
//!     );
//!
//! // Validated and packed, then loaded from the pack like the embedded catalog.
//! let source = catalog.source();
//! assert_eq!(source.assets().len(), 6);
//! let skill = source.asset(&AssetId::new("adr-writer")?).expect("packed");
//! assert_eq!(skill.manifest.version.to_string(), "0.3.1");
//! assert!(source.bundle(&AssetId::new("security-baseline")?).is_some());
//!
//! // A user's repository, with human-written content in a shared file.
//! let project = Project::new();
//! project.write("AGENTS.md", "# Our project\n\nHuman-written guidance.\n");
//! assert_eq!(project.tree(), ["AGENTS.md"]);
//! assert!(!project.exists(".mcp.json"));
//!
//! // A CLI test now calls `agent_grimoire::run` with `&source` and
//! // `project.root()` as its working directory, then reads the files back.
//! assert!(project.read("AGENTS.md").starts_with("# Our project"));
//! # Ok::<(), agent_grimoire_model::InvalidValue>(())
//! ```
//!
//! # What a fixture asset holds
//!
//! Every asset builder (all but [`FixtureCatalog::bundle`], which writes one
//! `bundles/<id>.toml`) writes the files that catalog validation requires of
//! an asset of its kind:
//! - `asset.toml`, with fixed common fields: `format = 1`, the summary
//!   `Fixture <kind> <id>`, the license `MIT OR Apache-2.0`, the maintainer
//!   `example-maintainer`, `maturity = "stable"` and
//!   `min-cli-version = "0.1.0"`;
//! - `README.md`, and a `CHANGELOG.md` with a section for the asset's version;
//! - for rules, skills and agents, `evals/cases.md`;
//! - for tools and MCP servers, a crate: a `Cargo.toml` that names the binary
//!   `grimoire-tool-<id>` or `grimoire-mcp-<id>`, `src/main.rs` and
//!   `tests/cli.rs`. An MCP server's `Cargo.toml` depends on
//!   `agent-grimoire-mcp`, and its test names
//!   `agent_grimoire_mcp::testing::declared_tools_match`, so it passes the
//!   checks that every server serves through that crate. These crates are
//!   never built; the temporary directory is outside the Cargo workspace.
//!
//! # Packed, not read from the directory
//!
//! [`FixtureCatalog::source`] runs [`agent_grimoire_catalog::pack`], which
//! validates the whole catalog first, as `cargo xtask pack-catalog` does for
//! the real one. It then loads the pack with [`Source::from_pack`] and the
//! source identity `builtin`, which is what [`Source::builtin`] does with the
//! pack embedded in the binary. So a fixture catalog passes the same
//! catalog-wide validation as the real one, a test loads it with the same code
//! as a user's `grimoire`, and the `grimoire.lock` the CLI writes in a test
//! names the same source.
//!
//! # Failures panic
//!
//! Fixtures fail the test loudly by design. The methods panic on a filesystem
//! error instead of returning one ([`FixtureCatalog::asset`] also panics on an
//! unknown kind), and [`FixtureCatalog::source`] panics with every diagnostic
//! when the catalog is invalid:
//!
//! ```should_panic
//! use agent_grimoire_testkit::FixtureCatalog;
//!
//! let catalog = FixtureCatalog::new();
//! catalog
//!     .rule("commit-style", "1.0.0", &[], "# Commits\n")
//!     .edit_manifest("rules/commit-style", |toml| {
//!         toml.replace(r#"maturity = "stable""#, r#"maturity = "ripe""#)
//!     });
//! // Panics: "invalid fixture catalog:", then each diagnostic.
//! let _ = catalog.source();
//! ```
//!
//! # Paths and temporary directories
//!
//! - Every path argument uses `/`. Most are relative to the catalog or project
//!   root; [`FixtureCatalog::asset`]'s `files` are relative to the asset's own
//!   directory. [`FixtureCatalog::write`] and the [`Project`] methods split a
//!   path at each `/` and join the parts with the operating system's
//!   separator; [`FixtureCatalog::edit_manifest`] joins its directory as
//!   given.
//! - [`Project::exists`] doesn't follow a final symbolic link: a link counts
//!   as itself, even when its target is missing. [`Project::tree`] lists a
//!   link as an entry and doesn't walk into it. It gives directories a
//!   trailing `/`, and lists depth first, each directory's entries sorted by
//!   name.
//! - Each [`FixtureCatalog`] and [`Project`] deletes its directory when it is
//!   dropped, so keep it alive while a test uses its paths.
//!
//! [architecture-4]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/01-architecture.md#repository-layout
//! [release-2d]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/05-release.md#during-the-first-release-run

#![allow(
    clippy::unwrap_used,
    clippy::missing_panics_doc,
    reason = "test fixtures fail the test loudly by design"
)]
#![allow(
    clippy::must_use_candidate,
    reason = "builder methods return &Self only so calls can be chained"
)]

use std::fs;
use std::path::{Path, PathBuf};

use agent_grimoire_catalog::Source;
use agent_grimoire_model::SourceId;
use tempfile::TempDir;

/// A temporary catalog directory.
#[derive(Debug)]
pub struct FixtureCatalog {
    dir: TempDir,
}

impl Default for FixtureCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl FixtureCatalog {
    /// An empty catalog.
    #[must_use]
    pub fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("README.md"), "# Fixture catalog\n").unwrap();
        Self { dir }
    }

    /// The catalog directory.
    #[must_use]
    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    /// Writes one file, creating parent directories.
    pub fn write(&self, path: &str, contents: impl AsRef<[u8]>) -> &Self {
        write_file(self.root(), path, contents.as_ref());
        self
    }

    /// Writes a complete asset directory: `asset.toml` from the common fields plus
    /// `manifest`, the given content files, and the required metadata files.
    pub fn asset(
        &self,
        kind: &str,
        id: &str,
        version: &str,
        manifest: &str,
        files: &[(&str, &str)],
    ) -> &Self {
        let kind_dir = match kind {
            "rule" => "rules",
            "skill" => "skills",
            "agent" => "agents",
            "tool" => "tools",
            "mcp-server" => "mcp-servers",
            other => panic!("unknown kind {other}"),
        };
        let dir = format!("{kind_dir}/{id}");
        self.write(
            &format!("{dir}/asset.toml"),
            format!(
                r#"format = 1
id = "{id}"
kind = "{kind}"
version = "{version}"
summary = "Fixture {kind} {id}"
description = "Used by Grimoire's tests."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "stable"
min-cli-version = "0.1.0"
{manifest}"#
            ),
        );
        self.write(
            &format!("{dir}/README.md"),
            format!("# {id}\n\nWhat it does.\n"),
        );
        self.write(
            &format!("{dir}/CHANGELOG.md"),
            format!("# Changelog\n\n## {version}\n\n- Release.\n"),
        );
        if matches!(kind, "tool" | "mcp-server") {
            let binary = if kind == "tool" {
                format!("grimoire-tool-{id}")
            } else {
                format!("grimoire-mcp-{id}")
            };
            // MCP servers must serve through the shared hardening crate.
            let deps = if kind == "mcp-server" {
                "\n[dependencies]\nagent-grimoire-mcp.workspace = true\n"
            } else {
                ""
            };
            self.write(
                &format!("{dir}/Cargo.toml"),
                format!(
                    "[package]\nname = \"{binary}\"\nversion = \"{version}\"\nedition = \"2024\"\npublish = false\n{deps}"
                ),
            );
            self.write(&format!("{dir}/src/main.rs"), "fn main() {}\n");
            // Fixture tests are never built; a server's must name the declared-tools check.
            self.write(
                &format!("{dir}/tests/cli.rs"),
                "#[test]\nfn works() {\n    // agent_grimoire_mcp::testing::declared_tools_match\n}\n",
            );
        } else {
            self.write(&format!("{dir}/evals/cases.md"), "- behaves\n");
        }
        for (path, contents) in files {
            self.write(&format!("{dir}/{path}"), contents);
        }
        self
    }

    /// Rewrites an asset's `asset.toml`, e.g. to change a common field.
    pub fn edit_manifest(&self, dir: &str, f: impl FnOnce(String) -> String) -> &Self {
        let path = self.root().join(dir).join("asset.toml");
        let text = fs::read_to_string(&path).unwrap();
        fs::write(path, f(text)).unwrap();
        self
    }

    /// A rule for `claude-code` and `agents-md`. Empty `paths` means always-on.
    pub fn rule(&self, id: &str, version: &str, paths: &[&str], body: &str) -> &Self {
        let paths = if paths.is_empty() {
            String::new()
        } else {
            let quoted: Vec<String> = paths.iter().map(|p| format!("{p:?}")).collect();
            format!("[rule]\npaths = [{}]\n", quoted.join(", "))
        };
        let table = if paths.is_empty() { "[rule]\n" } else { "" };
        self.asset(
            "rule",
            id,
            version,
            &format!(
                r#"{table}{paths}
[clients.claude-code]
outputs = [{{ src = "rule.md", to = "rules:{id}.md", strategy = "render" }}]

[clients.agents-md]
outputs = [{{ src = "rule.md", to = "instructions", strategy = "block" }}]
"#
            ),
            &[("rule.md", body)],
        )
    }

    /// A skill with a reference file, for `claude-code` and `agents-md`.
    pub fn skill(&self, id: &str, version: &str) -> &Self {
        let outputs = format!(
            r#"outputs = [
  {{ src = "SKILL.md", to = "skills:{id}/SKILL.md", strategy = "copy" }},
  {{ src = "references", to = "skills:{id}/references", strategy = "copy" }},
]"#
        );
        self.asset(
            "skill",
            id,
            version,
            &format!(
                "[skill]\n\n[clients.claude-code]\n{outputs}\n\n[clients.agents-md]\n{outputs}\n"
            ),
            &[
                (
                    "SKILL.md",
                    &format!(
                        "---\nname: {id}\ndescription: Does {id} things. Use when testing.\n---\n# {id}\n\nSee [the notes](references/notes.md).\n"
                    ),
                ),
                ("references/notes.md", "# Notes\n\nDetails.\n"),
            ],
        )
    }

    /// An agent for `claude-code` with the given capabilities.
    pub fn agent(&self, id: &str, version: &str, tools: &[&str]) -> &Self {
        let quoted: Vec<String> = tools.iter().map(|t| format!("{t:?}")).collect();
        let mut permissions = Vec::new();
        if tools.iter().any(|t| matches!(*t, "read" | "search")) && !tools.contains(&"edit") {
            permissions.push("filesystem = \"read\"");
        }
        if tools.contains(&"edit") {
            permissions.push("filesystem = \"write\"");
        }
        if tools.contains(&"execute") {
            permissions.push("shell = true");
        }
        if tools.contains(&"web") {
            permissions.push("network = true");
        }
        self.asset(
            "agent",
            id,
            version,
            &format!(
                r#"[permissions]
{}

[agent]
tools = [{}]

[clients.claude-code]
outputs = [{{ src = "agent.md", to = "agents:{id}.md", strategy = "render" }}]
"#,
                permissions.join("\n"),
                quoted.join(", ")
            ),
            &[(
                "agent.md",
                "You review changes carefully. You never edit files.\n",
            )],
        )
    }

    /// An MCP server for `claude-code` that reads the given environment variables.
    pub fn mcp_server(&self, id: &str, version: &str, env: &[&str]) -> &Self {
        let names: Vec<String> = env.iter().map(|e| format!("{e:?}")).collect();
        let specs: Vec<String> = env
            .iter()
            .map(|e| {
                format!(
                    "{{ name = {e:?}, required = true, secret = true, description = \"Token\" }}"
                )
            })
            .collect();
        let env_permission = if env.is_empty() {
            String::new()
        } else {
            format!("env = [{}]\n", names.join(", "))
        };
        self.asset(
            "mcp-server",
            id,
            version,
            &format!(
                r#"[permissions]
filesystem = "read"
{env_permission}
[mcp-server]
binary = "grimoire-mcp-{id}"
transports = ["stdio", "streamable-http"]
env = [{}]
tools = [{{ name = "search", read-only = true, destructive = false, idempotent = true, open-world = false }}]

[clients.claude-code]
outputs = [{{ to = "mcp", strategy = "entry" }}]
"#,
                specs.join(", ")
            ),
            &[],
        )
    }

    /// A tool with one pre-tool-use hook on edits, for `claude-code`.
    pub fn tool(&self, id: &str, version: &str) -> &Self {
        self.asset(
            "tool",
            id,
            version,
            &format!(
                r#"[permissions]
filesystem = "read"
hooks = true

[tool]
binary = "grimoire-tool-{id}"
hooks = [{{ event = "pre-tool-use", tools = ["edit"], args = ["check", "--stdin"], timeout-secs = 10 }}]

[clients.claude-code]
outputs = [{{ to = "hooks", strategy = "entry" }}]
"#
            ),
            &[],
        )
    }

    /// A bundle of `(id, requirement)` members.
    pub fn bundle(&self, id: &str, members: &[(&str, &str)]) -> &Self {
        let lines: Vec<String> = members
            .iter()
            .map(|(m, r)| format!("{m} = {r:?}"))
            .collect();
        self.write(
            &format!("bundles/{id}.toml"),
            format!(
                "format = 1\nid = \"{id}\"\nversion = \"1.0.0\"\nsummary = \"Fixture bundle\"\ndescription = \"For tests.\"\nmaintainers = [\"example-maintainer\"]\n\n[assets]\n{}\n",
                lines.join("\n")
            ),
        )
    }

    /// Packs the catalog and loads it, the way the embedded catalog is loaded.
    ///
    /// # Panics
    ///
    /// Panics with every diagnostic when the fixture catalog is invalid.
    #[must_use]
    pub fn source(&self) -> Source {
        let bytes = match agent_grimoire_catalog::pack(self.root()) {
            Ok(bytes) => bytes,
            Err(diagnostics) => {
                let text: Vec<String> = diagnostics.iter().map(ToString::to_string).collect();
                panic!("invalid fixture catalog:\n{}", text.join("\n"));
            }
        };
        Source::from_pack(SourceId::Builtin, &bytes).unwrap()
    }
}

/// A temporary user repository.
#[derive(Debug)]
pub struct Project {
    dir: TempDir,
}

impl Default for Project {
    fn default() -> Self {
        Self::new()
    }
}

impl Project {
    /// An empty project directory.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
        }
    }

    /// The project root.
    #[must_use]
    pub fn root(&self) -> &Path {
        self.dir.path()
    }

    /// The absolute path of `rel`.
    #[must_use]
    pub fn path(&self, rel: &str) -> PathBuf {
        rel.split('/')
            .fold(self.root().to_path_buf(), |p, part| p.join(part))
    }

    /// Writes one file, creating parent directories.
    pub fn write(&self, rel: &str, contents: impl AsRef<[u8]>) -> &Self {
        write_file(self.root(), rel, contents.as_ref());
        self
    }

    /// Reads a file as UTF-8.
    #[must_use]
    pub fn read(&self, rel: &str) -> String {
        fs::read_to_string(self.path(rel)).unwrap()
    }

    /// Whether `rel` exists (without following a final symlink).
    #[must_use]
    pub fn exists(&self, rel: &str) -> bool {
        fs::symlink_metadata(self.path(rel)).is_ok()
    }

    /// Every file and directory below the root, as `/`-separated paths, with a
    /// trailing `/` on each directory. The walk is depth first: each directory's
    /// entries are sorted by name, and a directory's contents come right after
    /// it. So `a/`, `a/b` and `a.txt` come in that order, which a string sort of
    /// the full paths would not give (it puts `a.txt` first, because `.` sorts
    /// before `/`).
    #[must_use]
    pub fn tree(&self) -> Vec<String> {
        fn walk(dir: &Path, prefix: &str, out: &mut Vec<String>) {
            let mut entries: Vec<_> = fs::read_dir(dir).unwrap().map(|e| e.unwrap()).collect();
            entries.sort_by_key(fs::DirEntry::file_name);
            for entry in entries {
                let name = entry.file_name().into_string().unwrap();
                let rel = if prefix.is_empty() {
                    name
                } else {
                    format!("{prefix}/{name}")
                };
                let meta = fs::symlink_metadata(entry.path()).unwrap();
                if meta.is_dir() {
                    out.push(format!("{rel}/"));
                    walk(&entry.path(), &rel, out);
                } else {
                    out.push(rel);
                }
            }
        }
        let mut out = Vec::new();
        walk(self.root(), "", &mut out);
        out
    }
}

fn write_file(root: &Path, rel: &str, contents: &[u8]) {
    let path = rel
        .split('/')
        .fold(root.to_path_buf(), |p, part| p.join(part));
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, contents).unwrap();
}
