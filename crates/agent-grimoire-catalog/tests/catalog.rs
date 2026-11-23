//! Catalog validation against generated fixture catalogs.
//!
//! `valid()` builds a catalog with one asset of every kind plus a bundle,
//! which must produce no diagnostics at all. Each other test applies one
//! mutation and asserts on the diagnostic codes it produces.

#![allow(
    clippy::unwrap_used,
    reason = "fixture helpers fail the test loudly by design"
)]

use std::fs;
use std::path::Path;

use agent_grimoire_catalog::{Source, Validation, load};
use agent_grimoire_model::{AssetId, MAX_REL_PATH, RelPath, SourceId};
use tempfile::TempDir;

struct Fixture {
    dir: TempDir,
}

impl Fixture {
    fn empty() -> Self {
        Self {
            dir: tempfile::tempdir().unwrap(),
        }
    }

    fn root(&self) -> &Path {
        self.dir.path()
    }

    fn write(&self, path: &str, contents: &str) -> &Self {
        let path = self.root().join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, contents).unwrap();
        self
    }

    fn remove(&self, path: &str) -> &Self {
        let path = self.root().join(path);
        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
        self
    }

    fn edit(&self, path: &str, f: impl FnOnce(String) -> String) -> &Self {
        let full = self.root().join(path);
        let text = fs::read_to_string(&full).unwrap();
        fs::write(full, f(text)).unwrap();
        self
    }

    fn load(&self) -> Validation {
        load(self.root())
    }

    fn codes(&self) -> Vec<&'static str> {
        self.load().diagnostics.iter().map(|d| d.code).collect()
    }

    fn assert_code(&self, code: &str) {
        let validation = self.load();
        assert!(
            validation.diagnostics.iter().any(|d| d.code == code),
            "expected {code}, got:\n{}",
            render(&validation)
        );
    }
}

fn render(v: &Validation) -> String {
    v.diagnostics
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

const COMMON: &str = r#"format = 1
version = "1.0.0"
summary = "A fixture asset"
description = "Used by the catalog tests."
license = "MIT OR Apache-2.0"
maintainers = ["example-maintainer"]
maturity = "stable"
min-cli-version = "0.1.0"
"#;

fn metadata(f: &Fixture, dir: &str) {
    f.write(&format!("{dir}/README.md"), "# Fixture\n\nWhat it does.\n")
        .write(
            &format!("{dir}/CHANGELOG.md"),
            "# Changelog\n\n## 1.0.0\n\n- Initial release.\n",
        );
}

fn valid() -> Fixture {
    let f = Fixture::empty();
    f.write("README.md", "# Catalog\n");

    // Rule.
    let dir = "rules/rust-errors";
    f.write(
        &format!("{dir}/asset.toml"),
        &format!(
            r#"id = "rust-errors"
kind = "rule"
{COMMON}
[rule]
paths = ["**/*.rs"]

[clients.claude-code]
outputs = [{{ src = "rule.md", to = "rules:rust-errors.md", strategy = "render" }}]

[clients.agents-md]
outputs = [{{ src = "rule.md", to = "instructions", strategy = "block" }}]
"#
        ),
    )
    .write(
        &format!("{dir}/rule.md"),
        "Use typed errors in libraries and attach context in binaries.\n",
    )
    .write(
        &format!("{dir}/evals/cases.md"),
        "- returns a typed error\n",
    );
    metadata(&f, dir);

    // Skill.
    let dir = "skills/adr-writer";
    f.write(
        &format!("{dir}/asset.toml"),
        &format!(
            r#"id = "adr-writer"
kind = "skill"
{COMMON}
[permissions]
filesystem = "write"
shell = true

[clients.claude-code]
outputs = [
  {{ src = "SKILL.md", to = "skills:adr-writer/SKILL.md", strategy = "copy" }},
  {{ src = "references", to = "skills:adr-writer/references", strategy = "copy" }},
  {{ src = "scripts", to = "skills:adr-writer/scripts", strategy = "copy" }},
]

[clients.agents-md]
outputs = [
  {{ src = "SKILL.md", to = "skills:adr-writer/SKILL.md", strategy = "copy" }},
  {{ src = "references", to = "skills:adr-writer/references", strategy = "copy" }},
  {{ src = "scripts", to = "skills:adr-writer/scripts", strategy = "copy" }},
]
"#
        ),
    )
    .write(
        &format!("{dir}/SKILL.md"),
        "---\nname: adr-writer\ndescription: Writes Architecture Decision Records. Use when a significant decision needs recording.\nlicense: MIT OR Apache-2.0\nmetadata:\n  author: example\n---\n# Writing an ADR\n\n1. Copy [the template](references/template.md).\n2. Run `scripts/next-number.sh` for the next number.\n",
    )
    .write(&format!("{dir}/references/template.md"), "# ADR-NNNN: Title\n")
    .write(&format!("{dir}/scripts/next-number.sh"), "#!/bin/sh\necho 0001\n")
    .write(&format!("{dir}/evals/cases.md"), "- writes an ADR\n");
    metadata(&f, dir);

    // Agent.
    let dir = "agents/security-reviewer";
    f.write(
        &format!("{dir}/asset.toml"),
        &format!(
            r#"id = "security-reviewer"
kind = "agent"
{COMMON}
[permissions]
filesystem = "read"

[agent]
tools = ["read", "search"]

[clients.claude-code]
outputs = [{{ src = "agent.md", to = "agents:security-reviewer.md", strategy = "render" }}]
"#
        ),
    )
    .write(
        &format!("{dir}/agent.md"),
        "You review diffs for security problems. You never edit files.\n",
    )
    .write(
        &format!("{dir}/evals/cases.md"),
        "- flags a hard-coded token\n",
    );
    metadata(&f, dir);

    // MCP server.
    let dir = "mcp-servers/docs-index";
    f.write(
        &format!("{dir}/asset.toml"),
        &format!(
            r#"id = "docs-index"
kind = "mcp-server"
{COMMON}
[permissions]
filesystem = "read"

[mcp-server]
binary = "grimoire-mcp-docs-index"
transports = ["stdio", "streamable-http"]
tools = [{{ name = "search_docs", read-only = true, destructive = false, idempotent = true, open-world = false }}]

[clients.claude-code]
outputs = [{{ to = "mcp", strategy = "entry" }}]
"#
        ),
    )
    .write(
        &format!("{dir}/Cargo.toml"),
        "[package]\nname = \"grimoire-mcp-docs-index\"\nversion = \"1.0.0\"\nedition = \"2024\"\npublish = false\n\n[dependencies]\nagent-grimoire-mcp.workspace = true\n",
    )
    .write(&format!("{dir}/src/main.rs"), "fn main() {}\n")
    .write(
        &format!("{dir}/tests/protocol.rs"),
        "#[test]\nfn lists_tools() {\n    // agent_grimoire_mcp::testing::declared_tools_match\n}\n",
    );
    metadata(&f, dir);

    // Tool with a hook.
    let dir = "tools/secret-guard";
    f.write(
        &format!("{dir}/asset.toml"),
        &format!(
            r#"id = "secret-guard"
kind = "tool"
{COMMON}
[permissions]
filesystem = "read"
hooks = true

[tool]
binary = "grimoire-tool-secret-guard"
hooks = [{{ event = "pre-tool-use", tools = ["edit"], args = ["check", "--stdin"], timeout-secs = 10 }}]

[clients.claude-code]
outputs = [{{ to = "hooks", strategy = "entry" }}]
"#
        ),
    )
    .write(
        &format!("{dir}/Cargo.toml"),
        "[package]\nname = \"grimoire-tool-secret-guard\"\nversion = \"1.0.0\"\nedition = \"2024\"\npublish = false\n",
    )
    .write(&format!("{dir}/src/main.rs"), "fn main() {}\n")
    .write(&format!("{dir}/tests/cli.rs"), "#[test]\nfn blocks_secrets() {}\n");
    metadata(&f, dir);

    // Bundle.
    f.write(
        "bundles/security-baseline.toml",
        r#"format = 1
id = "security-baseline"
version = "1.0.0"
summary = "Security essentials"
description = "A reviewer agent and a secret guard."
maintainers = ["example-maintainer"]

[assets]
security-reviewer = "^1"
secret-guard = "^1"
"#,
    );
    f
}

#[test]
fn valid_catalog_has_no_diagnostics() {
    let f = valid();
    let v = f.load();
    assert!(
        v.diagnostics.is_empty(),
        "unexpected diagnostics:\n{}",
        render(&v)
    );
    assert_eq!(v.catalog.assets().len(), 5);
    assert_eq!(v.catalog.bundles().len(), 1);
}

#[test]
fn empty_catalog_is_valid() {
    let f = Fixture::empty();
    assert_eq!(f.load().diagnostics, Vec::new());
}

#[test]
fn missing_root_is_reported() {
    let v = load(Path::new("/definitely/not/a/catalog"));
    assert!(v.has_errors());
    assert_eq!(v.diagnostics[0].code, "catalog/unreadable");
}

#[test]
fn layout_errors() {
    let f = valid();
    f.write("stray.txt", "x");
    f.assert_code("catalog/unexpected-entry");

    let f = valid();
    f.write("rules/Bad_Name/asset.toml", "x");
    f.assert_code("catalog/asset-dir-name");

    let f = valid();
    f.write("rules/empty-dir/README.md", "x");
    f.assert_code("catalog/manifest-missing");
}

#[test]
fn manifest_must_parse_strictly() {
    let f = valid();
    f.edit("rules/rust-errors/asset.toml", |t| {
        format!("surprise = true\n{t}")
    });
    f.assert_code("catalog/manifest-parse");
    let v = f.load();
    let d = v
        .diagnostics
        .iter()
        .find(|d| d.code == "catalog/manifest-parse")
        .unwrap();
    assert!(d.message.contains("surprise"), "{}", d.message);
}

#[test]
fn id_and_kind_must_match_directories() {
    let f = valid();
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("id = \"rust-errors\"", "id = \"other\"")
    });
    f.assert_code("catalog/id-mismatch");

    let f = valid();
    fs::create_dir_all(f.root().join("skills")).unwrap();
    fs::rename(
        f.root().join("rules/rust-errors"),
        f.root().join("skills/rust-errors"),
    )
    .unwrap();
    f.assert_code("catalog/kind-mismatch");
}

#[test]
fn metadata_files_are_required() {
    let f = valid();
    f.remove("rules/rust-errors/README.md");
    f.assert_code("catalog/metadata");

    let f = valid();
    f.write(
        "rules/rust-errors/CHANGELOG.md",
        "# Changelog\n\n## 0.9.0\n",
    );
    f.assert_code("catalog/changelog");

    let f = valid();
    f.remove("rules/rust-errors/evals");
    f.assert_code("catalog/tests");

    let f = valid();
    f.remove("tools/secret-guard/tests");
    f.assert_code("catalog/tests");
}

#[test]
fn sources_must_exist_and_not_be_metadata() {
    let f = valid();
    f.remove("rules/rust-errors/rule.md");
    f.assert_code("catalog/src-missing");

    let f = valid();
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace(
            "src = \"rule.md\", to = \"instructions\"",
            "src = \"README.md\", to = \"instructions\"",
        )
    });
    f.assert_code("catalog/src-metadata");

    let f = valid();
    f.write("rules/rust-errors/rule.md", "   \n");
    f.assert_code("catalog/src-text");
}

#[cfg(unix)]
#[test]
fn symlinks_and_unportable_names_are_rejected() {
    let f = valid();
    std::os::unix::fs::symlink("/etc/passwd", f.root().join("rules/rust-errors/evals/link"))
        .unwrap();
    f.assert_code("catalog/symlink");

    let f = valid();
    f.write("rules/rust-errors/evals/aux.md", "x");
    f.assert_code("catalog/path");
}

/// The pack stores each file under its path from the catalog root, and loading
/// reads that path back as a `RelPath`. So `load` must refuse a file whose full
/// path is too long, even when its path inside the asset is not.
#[test]
fn paths_from_the_catalog_root_must_fit_in_the_pack() {
    // `references` is installed, so every file in it is packed.
    let prefix = "skills/adr-writer/references/";
    let long = |len: usize| format!("{prefix}{}.md", "a".repeat(len - prefix.len() - 3));

    let f = valid();
    let at_limit = long(MAX_REL_PATH);
    f.write(&at_limit, "# Long\n");
    let v = f.load();
    assert!(v.diagnostics.is_empty(), "{}", render(&v));
    let bytes = agent_grimoire_catalog::pack(f.root()).unwrap();
    let source = Source::from_pack(SourceId::Builtin, &bytes).unwrap();
    let rel = RelPath::new(at_limit.strip_prefix("skills/adr-writer/").unwrap()).unwrap();
    let skill = source.asset(&AssetId::new("adr-writer").unwrap()).unwrap();
    assert_eq!(skill.file(&rel), Some(&b"# Long\n"[..]));

    let f = valid();
    let over = long(MAX_REL_PATH + 1);
    f.write(&over, "# Long\n");
    let v = f.load();
    assert!(
        v.diagnostics
            .iter()
            .any(|d| d.code == "catalog/path" && d.file.as_deref() == Some(over.as_str())),
        "expected catalog/path on {over}, got:\n{}",
        render(&v)
    );
    assert!(agent_grimoire_catalog::pack(f.root()).is_err());
}

#[test]
fn oversized_files_are_rejected() {
    let f = valid();
    let big = "x".repeat(usize::try_from(agent_grimoire_catalog::MAX_FILE_BYTES).unwrap() + 1);
    f.write("rules/rust-errors/evals/big.txt", &big);
    f.assert_code("catalog/file-size");
}

#[test]
fn rules_must_not_duplicate_each_other() {
    let f = valid();
    let dir = "rules/rust-errors-copy";
    f.write(
        &format!("{dir}/asset.toml"),
        &fs::read_to_string(f.root().join("rules/rust-errors/asset.toml"))
            .unwrap()
            .replace("rust-errors", "rust-errors-copy"),
    )
    .write(
        &format!("{dir}/rule.md"),
        "USE TYPED ERRORS in libraries, and attach context in binaries!\n",
    )
    .write(&format!("{dir}/evals/cases.md"), "- x\n");
    metadata(&f, dir);
    f.assert_code("rule/duplicate");
}

#[test]
fn rules_that_run_long_are_reported() {
    let f = valid();
    f.write(
        "rules/rust-errors/rule.md",
        &"A distinct line.\n".repeat(250),
    );
    let v = f.load();
    assert!(
        v.diagnostics
            .iter()
            .any(|d| d.code == "rule/length" && !d.is_error()),
        "{}",
        render(&v)
    );
    f.write(
        "rules/rust-errors/rule.md",
        &"A distinct line.\n".repeat(501),
    );
    f.assert_code("rule/length");
    assert!(f.load().has_errors());
}

#[test]
fn skill_frontmatter_rules() {
    let skill = "skills/adr-writer/SKILL.md";

    let f = valid();
    f.edit(skill, |t| t.replace("name: adr-writer", "name: other-name"));
    f.assert_code("skill/name");

    let f = valid();
    f.edit(skill, |t| {
        t.replace(
            "license: MIT OR Apache-2.0\n",
            "license: MIT OR Apache-2.0\nunknown-key: x\n",
        )
    });
    f.assert_code("skill/frontmatter");

    let f = valid();
    f.edit(skill, |t| {
        t.replace(
            "license: MIT OR Apache-2.0\n",
            "license: MIT OR Apache-2.0\nallowed-tools: Bash(git:*)\n",
        )
    });
    f.assert_code("skill/allowed-tools");

    let f = valid();
    f.edit(skill, |t| {
        t.replacen(
            "description: Writes",
            &format!("description: {}", "x".repeat(1100)),
            1,
        )
    });
    f.assert_code("skill/description");

    let f = valid();
    f.edit(skill, |t| t.replacen("---\n", "", 1));
    f.assert_code("skill/frontmatter");

    let f = valid();
    f.edit(skill, |t| {
        t.replace(
            "metadata:\n  author: example\n",
            "metadata:\n  count: [1, 2]\n",
        )
    });
    f.assert_code("skill/frontmatter");

    let f = valid();
    f.edit(skill, |t| {
        t.replace("name: adr-writer\n", "name: adr-writer\nname: adr-writer\n")
    });
    f.assert_code("skill/frontmatter");

    let f = valid();
    f.edit(skill, |t| format!("{t}{}", "More detail.\n".repeat(500)));
    f.assert_code("skill/length");
}

#[test]
fn skill_references_and_payload() {
    let skill = "skills/adr-writer/SKILL.md";

    let f = valid();
    f.edit(skill, |t| {
        format!("{t}\nSee [missing](references/nope.md).\n")
    });
    f.assert_code("skill/reference-missing");

    let f = valid();
    f.edit(skill, |t| {
        format!("{t}\nSee [outside](../../rules/rust-errors/rule.md).\n")
    });
    f.assert_code("skill/reference");

    let f = valid();
    f.write("skills/adr-writer/notes.md", "private notes\n");
    f.edit(skill, |t| format!("{t}\nSee [notes](notes.md).\n"));
    let codes = f.codes();
    assert!(
        codes.contains(&"skill/reference-not-installed"),
        "{codes:?}"
    );
    assert!(codes.contains(&"skill/extra-file"), "{codes:?}");

    let f = valid();
    f.write("skills/adr-writer/scripts/undocumented.sh", "#!/bin/sh\n");
    f.assert_code("skill/script-undocumented");

    let f = valid();
    f.edit("skills/adr-writer/asset.toml", |t| {
        t.replace("shell = true\n", "")
    });
    f.assert_code("skill/script-permission");

    let f = valid();
    f.write("skills/adr-writer/assets/logo.txt", "logo\n");
    f.assert_code("skill/payload");

    let f = valid();
    f.write(
        "skills/adr-writer/references/deep.md",
        "See [deeper](more.md).\n",
    )
    .write("skills/adr-writer/references/more.md", "x\n");
    f.edit(skill, |t| format!("{t}\nSee [deep](references/deep.md).\n"));
    f.assert_code("skill/reference-depth");
}

#[test]
fn code_assets_are_checked_as_crates() {
    let f = valid();
    f.edit("mcp-servers/docs-index/Cargo.toml", |t| {
        t.replace("version = \"1.0.0\"", "version = \"2.0.0\"")
    });
    f.assert_code("crate/version");

    let f = valid();
    f.edit("mcp-servers/docs-index/Cargo.toml", |t| {
        t.replace("publish = false\n", "")
    });
    f.assert_code("crate/publish");

    let f = valid();
    f.edit("mcp-servers/docs-index/Cargo.toml", |t| {
        t.replace("grimoire-mcp-docs-index", "other-name")
    });
    f.assert_code("crate/name");

    let f = valid();
    f.write("tools/secret-guard/src/bin/extra.rs", "fn main() {}\n");
    f.assert_code("crate/bin");

    let f = valid();
    f.remove("tools/secret-guard/Cargo.toml");
    f.assert_code("crate/missing");
}

#[test]
fn servers_use_the_hardening_crate_and_no_asset_uses_core_crates() {
    let f = valid();
    f.edit("mcp-servers/docs-index/Cargo.toml", |t| {
        t.replace("agent-grimoire-mcp.workspace = true\n", "")
    });
    f.assert_code("crate/hardening");

    // Only a normal dependency counts.
    let f = valid();
    f.edit("mcp-servers/docs-index/Cargo.toml", |t| {
        t.replace("[dependencies]", "[dev-dependencies]")
    });
    f.assert_code("crate/hardening");

    for dependency in [
        "[dependencies]\nagent-grimoire-model.workspace = true\n",
        "[dev-dependencies]\nagent-grimoire-catalog = { path = \"../../../crates/agent-grimoire-catalog\" }\n",
        "[target.'cfg(unix)'.dependencies]\nalias = { package = \"agent-grimoire-install\", path = \"x\" }\n",
    ] {
        let f = valid();
        f.edit("tools/secret-guard/Cargo.toml", |t| {
            format!("{t}\n{dependency}")
        });
        f.assert_code("crate/core-dependency");
    }
}

#[test]
fn manifest_rules_surface_through_the_catalog() {
    let f = valid();
    f.edit("mcp-servers/docs-index/asset.toml", |t| {
        t.replace("[\"stdio\", \"streamable-http\"]", "[\"stdio\"]")
    });
    f.assert_code("manifest/mcp-transports");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[\"read\", \"search\"]", "[\"read\", \"execute\"]")
    });
    f.assert_code("manifest/agent-permissions");
}

#[test]
fn dependencies_are_resolved_catalog_wide() {
    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nnot-there = \"^1\"\n\n[agent]")
    });
    f.assert_code("catalog/requires-missing");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nrust-errors = \"^2\"\n\n[agent]")
    });
    f.assert_code("catalog/requires-version");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nadr-writer = \"^1\"\n\n[agent]")
    });
    f.edit("skills/adr-writer/asset.toml", |t| {
        t.replace(
            "[permissions]",
            "[requires]\nsecurity-reviewer = \"^1\"\n\n[permissions]",
        )
    });
    f.assert_code("catalog/requires-cycle");
    assert_eq!(
        f.codes()
            .iter()
            .filter(|c| **c == "catalog/requires-cycle")
            .count(),
        1
    );
}

#[test]
fn agent_mcp_references_must_resolve() {
    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace(
            "[\"read\", \"search\"]",
            "[\"read\", \"mcp:docs-index/missing_tool\"]",
        )
        .replace("[agent]", "[requires]\ndocs-index = \"^1\"\n\n[agent]")
    });
    f.assert_code("catalog/agent-mcp-tool");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[\"read\", \"search\"]", "[\"read\", \"mcp:rust-errors\"]")
            .replace("[agent]", "[requires]\nrust-errors = \"^1\"\n\n[agent]")
    });
    f.assert_code("catalog/agent-mcp");
}

#[test]
fn deprecations_need_an_existing_replacement() {
    let f = valid();
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"\ndeprecated = { replacement = \"gone\", reason = \"Superseded.\" }")
    });
    f.assert_code("catalog/replacement-missing");

    let f = valid();
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"\ndeprecated = { replacement = \"adr-writer\", reason = \"Superseded.\" }")
    });
    f.assert_code("catalog/replacement-kind");
}

#[test]
fn bundle_rules() {
    let f = valid();
    f.edit("bundles/security-baseline.toml", |t| {
        t.replace(
            "secret-guard = \"^1\"",
            "secret-guard = \"^1\"\nnot-there = \"^1\"",
        )
    });
    f.assert_code("bundle/member-missing");

    let f = valid();
    f.edit("bundles/security-baseline.toml", |t| {
        t.replace("secret-guard = \"^1\"", "secret-guard = \"^9\"")
    });
    f.assert_code("bundle/member-version");

    let f = valid();
    fs::rename(
        f.root().join("bundles/security-baseline.toml"),
        f.root().join("bundles/other.toml"),
    )
    .unwrap();
    f.assert_code("catalog/bundle-id");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[conflicts]\nsecret-guard = { version = \"*\", reason = \"They disagree.\" }\n\n[agent]")
    });
    f.assert_code("bundle/conflict");

    let f = valid();
    f.write(
        "bundles/rust-errors.toml",
        &fs::read_to_string(f.root().join("bundles/security-baseline.toml"))
            .unwrap()
            .replace("security-baseline", "rust-errors"),
    );
    f.assert_code("catalog/duplicate-id");
}

#[test]
fn duplicate_ids_across_kinds_are_rejected() {
    let f = valid();
    let src = f.root().join("rules/rust-errors");
    let dst = f.root().join("agents/rust-errors");
    copy_dir(&src, &dst);
    f.assert_code("catalog/duplicate-id");
}

fn copy_dir(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for entry in fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let to = dst.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &to);
        } else {
            fs::copy(entry.path(), to).unwrap();
        }
    }
}

#[test]
fn diagnostics_render_stably() {
    let f = valid();
    f.write("stray.txt", "x");
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"")
    });
    f.remove("agents/security-reviewer/README.md");
    insta::assert_snapshot!(render(&f.load()));
}

fn assert_warning(f: &Fixture, code: &str) {
    let v = f.load();
    assert!(
        v.diagnostics
            .iter()
            .any(|d| d.code == code && !d.is_error()),
        "expected warning {code}, got:\n{}",
        render(&v)
    );
}

fn write_bytes(f: &Fixture, path: &str, bytes: &[u8]) {
    let path = f.root().join(path);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}

#[test]
fn reserved_and_unportable_ids_are_rejected() {
    let f = valid();
    f.write("rules/synced/asset.toml", "x");
    f.assert_code("catalog/asset-dir-name");

    let f = valid();
    f.write("rules/rust-errors/evals/caf\u{e9}.md", "x");
    f.assert_code("catalog/path");
}

#[test]
fn case_insensitive_collisions_are_rejected() {
    let f = valid();
    f.write("skills/adr-writer/references/Template.md", "# Other\n");
    // On case-insensitive filesystems the second name replaces the first; there is nothing to detect.
    let both = fs::read_dir(f.root().join("skills/adr-writer/references"))
        .unwrap()
        .count()
        == 2;
    if both {
        f.assert_code("catalog/path-collision");
    }
}

#[test]
fn non_utf8_and_empty_metadata_are_rejected() {
    let f = valid();
    f.write("rules/rust-errors/README.md", "   \n");
    f.assert_code("catalog/metadata");

    let f = valid();
    write_bytes(&f, "rules/rust-errors/README.md", b"\xff\xfe not utf-8");
    f.assert_code("catalog/metadata");

    let f = valid();
    write_bytes(&f, "rules/rust-errors/rule.md", b"\xff\xfe not utf-8");
    f.assert_code("catalog/src-text");
}

#[test]
fn layout_entries_inside_kind_and_bundle_dirs() {
    let f = valid();
    f.write("rules/stray.md", "x");
    f.assert_code("catalog/unexpected-entry");

    let f = valid();
    f.write("bundles/notes.txt", "x");
    f.assert_code("catalog/unexpected-entry");
}

#[test]
fn bundles_parse_strictly_and_ids_are_unique() {
    let f = valid();
    f.write(
        "bundles/broken.toml",
        "format = 1\nid = \"broken\"\nunknown = true\n",
    );
    f.assert_code("catalog/bundle-parse");

    let f = valid();
    let text = fs::read_to_string(f.root().join("bundles/security-baseline.toml")).unwrap();
    f.write("bundles/another.toml", &text);
    let codes = f.codes();
    assert!(codes.contains(&"catalog/bundle-id"), "{codes:?}");
    assert!(codes.contains(&"catalog/duplicate-id"), "{codes:?}");

    // Reads are capped at the file-size limit for bundles too.
    let f = valid();
    let big = "#".repeat(usize::try_from(agent_grimoire_catalog::MAX_FILE_BYTES).unwrap() + 1);
    f.write("bundles/huge.toml", &big);
    let v = f.load();
    let d = v
        .diagnostics
        .iter()
        .find(|d| d.code == "catalog/bundle-parse")
        .expect("bundle-parse");
    assert!(d.message.contains("limit"), "{}", d.message);
}

#[test]
fn deprecation_and_unknown_conflict_warnings() {
    let deprecate = |t: String| {
        t.replace(
            "maturity = \"stable\"",
            "maturity = \"deprecated\"\ndeprecated = { replacement = \"security-reviewer\", reason = \"Superseded.\" }",
        )
    };
    // A deprecated member and dependency are warnings, not errors.
    let f = valid();
    f.edit("tools/secret-guard/asset.toml", |t| {
        t.replace(
            "maturity = \"stable\"",
            "maturity = \"deprecated\"\ndeprecated = { replacement = \"secret-guard-v2\", reason = \"Superseded.\" }",
        )
    });
    let dir = "tools/secret-guard-v2";
    copy_dir(&f.root().join("tools/secret-guard"), &f.root().join(dir));
    f.edit(&format!("{dir}/asset.toml"), |t| {
        t.replace("id = \"secret-guard\"", "id = \"secret-guard-v2\"")
            .replace("grimoire-tool-secret-guard", "grimoire-tool-secret-guard-v2")
            .replace("maturity = \"deprecated\"\ndeprecated = { replacement = \"secret-guard-v2\", reason = \"Superseded.\" }", "maturity = \"stable\"")
    });
    f.edit(&format!("{dir}/Cargo.toml"), |t| {
        t.replace(
            "grimoire-tool-secret-guard",
            "grimoire-tool-secret-guard-v2",
        )
    });
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nsecret-guard = \"^1\"\n\n[agent]")
    });
    assert_warning(&f, "bundle/member-deprecated");
    assert_warning(&f, "catalog/requires-deprecated");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", deprecate);
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"\ndeprecated = { replacement = \"security-reviewer\", reason = \"x\" }")
    });
    f.assert_code("catalog/replacement-kind");

    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace(
            "[agent]",
            "[conflicts]\nlong-gone = { version = \"*\", reason = \"Old.\" }\n\n[agent]",
        )
    });
    assert_warning(&f, "catalog/conflicts-unknown");
}

#[test]
fn deprecated_replacements_are_warned_about() {
    let f = valid();
    let dir = "rules/rust-errors-v2";
    f.write(
        &format!("{dir}/asset.toml"),
        &fs::read_to_string(f.root().join("rules/rust-errors/asset.toml"))
            .unwrap()
            .replace("rust-errors", "rust-errors-v2"),
    )
    .write(
        &format!("{dir}/rule.md"),
        "A completely different rule about logging levels and spans.\n",
    )
    .write(&format!("{dir}/evals/cases.md"), "- x\n");
    metadata(&f, dir);
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"\ndeprecated = { replacement = \"rust-errors-v2\", reason = \"New.\" }")
    });
    f.edit(&format!("{dir}/asset.toml"), |t| {
        t.replace("maturity = \"stable\"", "maturity = \"deprecated\"\ndeprecated = { replacement = \"rust-errors\", reason = \"Loop.\" }")
    });
    assert_warning(&f, "catalog/replacement-deprecated");
}

#[test]
fn conflicts_across_dependency_closures() {
    // security-reviewer requires adr-writer and rust-errors; adr-writer conflicts with rust-errors.
    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace(
            "[agent]",
            "[requires]\nadr-writer = \"^1\"\nrust-errors = \"^1\"\n\n[agent]",
        )
    });
    f.edit("skills/adr-writer/asset.toml", |t| {
        t.replace("[permissions]", "[conflicts]\nrust-errors = { version = \"*\", reason = \"They disagree.\" }\n\n[permissions]")
    });
    f.assert_code("catalog/uninstallable");

    // A bundle conflicts through a member's dependency.
    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nrust-errors = \"^1\"\n\n[agent]")
    });
    f.edit("tools/secret-guard/asset.toml", |t| {
        t.replace("[permissions]", "[conflicts]\nrust-errors = { version = \"*\", reason = \"They disagree.\" }\n\n[permissions]")
    });
    f.assert_code("bundle/conflict");
}

#[test]
fn agent_prompts_and_block_sources_are_checked() {
    let f = valid();
    f.write("agents/security-reviewer/agent.md", &"x".repeat(30_001));
    f.assert_code("agent/length");

    let f = valid();
    f.write("rules/rust-errors/parts/one.md", "Part one.\n");
    f.edit("rules/rust-errors/asset.toml", |t| {
        t.replace(
            "src = \"rule.md\", to = \"instructions\"",
            "src = \"parts\", to = \"instructions\"",
        )
    });
    f.assert_code("catalog/src-type");
}

#[test]
fn crate_rules_for_code_assets() {
    let cargo = "mcp-servers/docs-index/Cargo.toml";
    for (edit, code) in [
        (
            Box::new(|_: String| "not [valid toml".to_owned()) as Box<dyn Fn(String) -> String>,
            "crate/parse",
        ),
        (
            Box::new(|_: String| "[lib]\nname = \"x\"\n".to_owned()),
            "crate/package",
        ),
        (
            Box::new(|t: String| t.replace("version = \"1.0.0\"", "version.workspace = true")),
            "crate/version",
        ),
        (
            Box::new(|t: String| format!("{t}\n[[bin]]\nname = \"something-else\"\n")),
            "crate/bin",
        ),
        (
            Box::new(|t: String| {
                format!(
                    "{t}\n[[bin]]\nname = \"grimoire-mcp-docs-index\"\npath = \"../../../outside.rs\"\n"
                )
            }),
            "crate/path",
        ),
        (
            Box::new(|t: String| {
                t.replace(
                    "publish = false",
                    "publish = false\nbuild = \"../build.rs\"",
                )
            }),
            "crate/path",
        ),
    ] {
        let f = valid();
        f.edit(cargo, edit);
        f.assert_code(code);
    }
    let f = valid();
    f.remove("mcp-servers/docs-index/src/main.rs");
    f.assert_code("crate/main");
}

#[test]
fn more_skill_frontmatter_rules() {
    let skill = "skills/adr-writer/SKILL.md";
    for (from, to, code) in [
        (
            "license: MIT OR Apache-2.0\n",
            "license: MIT OR Apache-2.0\ncompatibility: \"\"\n",
            "skill/compatibility",
        ),
        (
            "license: MIT OR Apache-2.0\n",
            &format!(
                "license: MIT OR Apache-2.0\ncompatibility: {}\n",
                "x".repeat(501)
            ) as &str,
            "skill/compatibility",
        ),
        (
            "license: MIT OR Apache-2.0\n",
            "license: \"  \"\n",
            "skill/license",
        ),
        (
            "metadata:\n  author: example\n",
            "metadata:\n  \"\": example\n",
            "skill/metadata",
        ),
        (
            "metadata:\n  author: example\n",
            "metadata: ~\n",
            "skill/frontmatter",
        ),
    ] {
        let f = valid();
        f.edit(skill, |t| t.replace(from, to));
        f.assert_code(code);
    }

    let f = valid();
    f.edit(skill, |t| {
        t.split_inclusive("---\n").take(2).collect::<String>()
    });
    f.assert_code("skill/body");

    let f = valid();
    f.remove(skill);
    f.assert_code("skill/missing");

    let f = valid();
    f.edit(skill, |t| format!("{t}\n{}\n", "word ".repeat(4_500)));
    assert_warning(&f, "skill/length");
}

#[test]
fn script_documentation_and_mentions() {
    let skill = "skills/adr-writer/SKILL.md";
    // Mentioning the directory alone does not document each script.
    let f = valid();
    f.edit(skill, |t| {
        t.replace(
            "Run `scripts/next-number.sh` for the next number.",
            "See the `scripts/` folder.",
        )
    });
    f.assert_code("skill/script-undocumented");

    // The spec's own styles count: plain prose, a fenced command, a ./ prefix.
    for mention in [
        "Run the numbering script:\nscripts/next-number.sh",
        "Run:\n\n```sh\nsh ./scripts/next-number.sh --dir docs/adr\n```",
        "Run `sh ./scripts/next-number.sh`.",
    ] {
        let f = valid();
        f.edit(skill, |t| {
            t.replace(
                "2. Run `scripts/next-number.sh` for the next number.",
                mention,
            )
        });
        let v = f.load();
        assert_eq!(v.diagnostics, Vec::new(), "{mention:?}:\n{}", render(&v));
    }

    // Globs and placeholders are not paths; a missing output path is only a warning.
    let f = valid();
    f.edit(skill, |t| format!("{t}\nAny of `scripts/*.sh` work. Write to `references/<topic>.md` and `assets/out.png`.\n"));
    let v = f.load();
    assert!(!v.has_errors(), "{}", render(&v));
    assert_warning(&f, "skill/reference-missing");
}

#[test]
fn skill_link_forms() {
    let skill = "skills/adr-writer/SKILL.md";
    // A directory link with a trailing slash resolves.
    let f = valid();
    f.edit(skill, |t| {
        format!("{t}\nBrowse [the references](references/).\n")
    });
    let v = f.load();
    assert_eq!(v.diagnostics, Vec::new(), "{}", render(&v));

    // file: URLs and raw HTML pointing outside the skill are rejected.
    for link in [
        "[passwd](file:///etc/passwd)",
        "<img src=\"../../rules/rust-errors/rule.md\">",
    ] {
        let f = valid();
        f.edit(skill, |t| format!("{t}\n{link}\n"));
        f.assert_code("skill/reference");
    }

    // A top-level file that every client installs may be linked.
    let f = valid();
    f.write("skills/adr-writer/GUIDE.md", "# Guide\n");
    f.edit("skills/adr-writer/asset.toml", |t| {
        t.replace(
            "  {{ src = \"scripts\", to = \"skills:adr-writer/scripts\", strategy = \"copy\" }},\n]".replace("{{", "{").replace("}}", "}").as_str(),
            "  { src = \"scripts\", to = \"skills:adr-writer/scripts\", strategy = \"copy\" },\n  { src = \"GUIDE.md\", to = \"skills:adr-writer/GUIDE.md\", strategy = \"copy\" },\n]",
        )
    });
    f.edit(skill, |t| format!("{t}\nRead [the guide](GUIDE.md).\n"));
    let v = f.load();
    assert_eq!(v.diagnostics, Vec::new(), "{}", render(&v));
}

#[cfg(unix)]
#[test]
fn symlinks_anywhere_in_the_catalog_are_rejected() {
    use std::os::unix::fs::symlink;
    // A symlinked asset directory.
    let f = valid();
    symlink(
        f.root().join("rules/rust-errors"),
        f.root().join("rules/linked"),
    )
    .unwrap();
    f.assert_code("catalog/symlink");
    // A symlink at the catalog root and in bundles/.
    let f = valid();
    symlink("/etc", f.root().join("etc")).unwrap();
    f.assert_code("catalog/symlink");
    let f = valid();
    symlink(
        f.root().join("bundles/security-baseline.toml"),
        f.root().join("bundles/alias.toml"),
    )
    .unwrap();
    f.assert_code("catalog/symlink");
}

#[cfg(unix)]
#[test]
fn special_files_are_rejected() {
    let f = valid();
    let socket = f.root().join("rules/rust-errors/evals/socket");
    let _listener = std::os::unix::net::UnixListener::bind(&socket).unwrap();
    f.assert_code("catalog/special-file");
}

#[test]
fn nesting_and_entry_limits() {
    let f = valid();
    let deep = (0..13)
        .map(|i| format!("d{i}"))
        .collect::<Vec<_>>()
        .join("/");
    f.write(&format!("rules/rust-errors/evals/{deep}/x.md"), "x");
    f.assert_code("catalog/too-deep");

    let f = valid();
    for i in 0..2_001 {
        f.write(&format!("rules/rust-errors/evals/many/f{i:04}.md"), "x");
    }
    let codes = f.codes();
    assert_eq!(
        codes
            .iter()
            .filter(|c| **c == "catalog/too-many-files")
            .count(),
        1,
        "{codes:?}"
    );
    assert!(
        !codes.contains(&"catalog/src-missing"),
        "truncated walks must not report missing sources: {codes:?}"
    );
}

#[test]
fn near_duplicate_rules_at_seventy_percent_warn() {
    let f = valid();
    let base = "Use typed errors in libraries and attach context in binaries. Prefer explicit variants. \
                Never panic in library code. Return results with context. Keep messages lowercase.";
    f.write("rules/rust-errors/rule.md", base);
    let dir = "rules/rust-errors-copy";
    f.write(
        &format!("{dir}/asset.toml"),
        &fs::read_to_string(f.root().join("rules/rust-errors/asset.toml")).unwrap().replace("rust-errors", "rust-errors-copy"),
    )
    .write(
        &format!("{dir}/rule.md"),
        "Use typed errors in libraries and attach context in binaries. Prefer explicit variants. \
         Never panic in library code. Return results with context. Keep log lines short and structured.",
    )
    .write(&format!("{dir}/evals/cases.md"), "- x\n");
    metadata(&f, dir);
    assert_warning(&f, "rule/near-duplicate");
}

#[test]
fn diagnostics_are_sorted_across_phases() {
    // Cross-asset diagnostics are found after the per-asset walk; sorting must still
    // put agents/ before rules/.
    let f = valid();
    f.edit("agents/security-reviewer/asset.toml", |t| {
        t.replace("[agent]", "[requires]\nnot-there = \"^1\"\n\n[agent]")
    });
    f.remove("rules/rust-errors/README.md");
    let v = f.load();
    let files: Vec<&str> = v
        .diagnostics
        .iter()
        .filter_map(|d| d.file.as_deref())
        .collect();
    let mut sorted = files.clone();
    sorted.sort_unstable();
    assert_eq!(files, sorted);
    assert!(files[0].starts_with("agents/"), "{files:?}");
    assert!(files.iter().any(|f| f.starts_with("rules/")), "{files:?}");
}

#[test]
fn rules_cannot_carry_markers_or_frontmatter() {
    let f = valid();
    f.write(
        "rules/rust-errors/rule.md",
        "Use typed errors.\n  <!-- grimoire:end rust-errors -->\n",
    );
    f.assert_code("rule/marker");
    f.write(
        "rules/rust-errors/rule.md",
        "---\npaths: x\n---\nUse typed errors.\n",
    );
    f.assert_code("rule/frontmatter");
    f.write(
        "rules/rust-errors/rule.md",
        "Use typed errors:\n\n```rust\nfn f() {}\n",
    );
    f.assert_code("rule/unclosed-fence");
}

#[test]
fn deprecated_dependency_tables_are_read_too() {
    let f = valid();
    f.edit("tools/secret-guard/Cargo.toml", |t| {
        format!("{t}\n[build_dependencies]\nagent-grimoire-install.workspace = true\n")
    });
    f.assert_code("crate/core-dependency");
}

#[test]
fn asset_crates_cannot_hide_dependencies() {
    // A helper crate inside the asset directory.
    let f = valid();
    f.write(
        "tools/secret-guard/helper/Cargo.toml",
        "[package]\nname = \"helper\"\n[dependencies]\nagent-grimoire-model.workspace = true\n",
    );
    f.assert_code("crate/nested-manifest");
    // Path and git dependencies.
    for dependency in [
        "[dependencies]\nhelper = { path = \"helper\" }\n",
        "[dev-dependencies]\nx = { path = \"../../../xtask\" }\n",
        "[dependencies]\ny = { git = \"https://example.com/y\", rev = \"abc\" }\n",
    ] {
        let f = valid();
        f.edit("tools/secret-guard/Cargo.toml", |t| {
            format!("{t}\n{dependency}")
        });
        f.assert_code("crate/path-dependency");
    }
}

#[test]
fn servers_serve_through_the_hardening_crate_and_test_their_declared_tools() {
    let f = valid();
    f.write(
        "mcp-servers/docs-index/src/main.rs",
        "fn main() { let _ = rmcp::transport::stdio(); }\n",
    );
    f.assert_code("crate/raw-transport");
    let f = valid();
    f.write(
        "mcp-servers/docs-index/tests/protocol.rs",
        "#[test]\nfn lists_tools() {}\n",
    );
    f.assert_code("crate/declared-tools-test");
}

/// A skill folder holding `.claude-plugin/plugin.json` loads in Claude Code as a
/// plugin with its own hooks and servers, so no skill may contain one.
#[test]
fn skills_may_not_contain_a_plugin_manifest() {
    let f = valid();
    f.write(
        "skills/adr-writer/references/deep/.claude-plugin/plugin.json",
        "{\"name\": \"sneaky\"}\n",
    );
    f.write(
        "skills/adr-writer/references/deep/.claude-plugin/hooks.json",
        "{}\n",
    );
    let found: Vec<_> = f
        .load()
        .diagnostics
        .into_iter()
        .filter(|d| d.code == "skill/plugin-manifest")
        .collect();
    assert_eq!(found.len(), 1, "once per folder: {found:?}");
    assert_eq!(
        found[0].file.as_deref(),
        Some("skills/adr-writer/references/deep/.claude-plugin")
    );
}
