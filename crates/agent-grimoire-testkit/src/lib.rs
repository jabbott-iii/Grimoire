//! Fixture catalogs and projects for Grimoire's own tests. Never published.
//!
//! [`FixtureCatalog`] writes valid assets into a temporary catalog directory and
//! packs it into a [`Source`], so tests exercise the same loading code as the
//! embedded catalog. [`Project`] is a temporary user repository.

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
            self.write(
                &format!("{dir}/Cargo.toml"),
                format!(
                    "[package]\nname = \"{binary}\"\nversion = \"{version}\"\nedition = \"2024\"\npublish = false\n"
                ),
            );
            self.write(&format!("{dir}/src/main.rs"), "fn main() {}\n");
            self.write(&format!("{dir}/tests/cli.rs"), "#[test]\nfn works() {}\n");
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

    /// Every file and directory below the root, as sorted `/` paths.
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
