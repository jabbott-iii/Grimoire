# agent-grimoire-catalog

Catalog loading and catalog-wide validation for Grimoire. The catalog is the collection of assets Grimoire can install, one directory per asset under `catalog/` in the Grimoire repository. This crate reads that directory and checks it, including the checks that need more than one document, such as whether each dependency exists. It then packs the checked catalog into one file, `catalog.pack`, which is built into the CLI, and loads that file back into the in-memory index that the CLI installs from.

It is part of [Grimoire](https://github.com/jabbott-iii/Grimoire): a catalog of AI-agent assets (rules, skills, agents, tools and MCP servers, where MCP is the Model Context Protocol) and `grimoire`, the command-line interface (CLI) that installs the assets a project selects. It builds on `agent-grimoire-model`, which holds the strict types for `asset.toml` and bundle files and the checks that need only one document. The CLI (`agent-grimoire`) loads the built-in catalog through this crate when it starts. `agent-grimoire-resolve`, which works out which assets and versions to install, and `agent-grimoire-adapters`, which writes each client's files, read assets from that index. The repository's own `cargo xtask validate` and `cargo xtask pack-catalog` commands run this crate's checks and write its pack.

## Status

Before 1.0, and internal to the CLI. It is published only so that `cargo install agent-grimoire` can build the CLI from crates.io, the Rust package registry. Its Rust API changes whenever the CLI needs it to, so don't depend on it: to install assets, use the CLI ([Install](https://github.com/jabbott-iii/Grimoire#install) in the Grimoire README).

A change that breaks it is still marked breaking, so that it ships as a new 0.y version and Cargo doesn't build an older CLI against it. The CLI depends on its libraries with Cargo requirements such as `0.1.0`, which accept every later 0.1.z. A break released as 0.1.z could therefore let Cargo build an older CLI against a library it doesn't compile with, and a new 0.y (0.2.0 after 0.1.z, for example) is outside those requirements.

The catalog is built into this crate, so install the CLI with `cargo install agent-grimoire --locked`. `--locked` makes Cargo use the `Cargo.lock` published with the CLI, which holds the version of this crate that the release was built with. On Cargo 1.96 to 1.99, a build without `--locked` can take a later version of this crate within the same 0.y, and so newer assets than the CLI version it reports.

What it does:

- `load` reads a catalog directory and returns a `Validation`: the `Catalog` of assets and bundles that parsed, and every diagnostic, sorted by file. A diagnostic is an error or a warning with a stable code, such as `catalog/symlink`. `load` doesn't stop at the first problem: it reports every one it finds, and an asset whose `asset.toml` is missing or doesn't parse is left out of the `Catalog` while the others are still checked. It checks:
  - file hygiene: no symbolic links (symlinks) and no special files such as named pipes; paths that are portable across Linux, macOS and Windows, and no two that differ only in letter case; at most 1 MiB (1,048,576 bytes) per file, 2,000 entries per asset and 12 levels of nesting;
  - each asset's `asset.toml`, `README.md`, `CHANGELOG.md` (with an entry for the current version) and `evals/` or `tests/`, and every source file its outputs name;
  - each kind's own rules: a rule stays under 500 lines (with a warning from 200) and holds nothing that would break the managed block it is installed in (the marked section Grimoire owns in a shared Markdown file such as `AGENTS.md`); an agent's prompt is at most 30,000 characters; skills are checked as below; and tools and MCP servers, which are Rust crates, follow the catalog's crate rules;
  - across the catalog: unique ids, dependencies that exist and form no cycle, deprecations, bundle members and conflicts, and rules that duplicate or nearly duplicate each other.
- Native Agent Skills checks: skills are checked against the Agent Skills specification (the open format that skills follow) in Rust, with the specification's stricter rules, rather than with its Python reference validator, `skills-ref`, which is "intended for demonstration purposes only". Where the specification only recommends, Grimoire is deliberately stricter: unknown frontmatter keys (in the YAML block between `---` lines that opens `SKILL.md`) are errors, `allowed-tools` is refused because it pre-approves tools outside Grimoire's permission consent, every bundled script must be referenced from `SKILL.md`, a skill with scripts must declare the `shell` permission, and `SKILL.md` must stay under 500 lines.
- `pack` runs `load` and, only when it finds no error, packs the catalog. The pack is deterministic (the same catalog always gives the same bytes) and holds only these files: each asset's `asset.toml`, `README.md` and `CHANGELOG.md`, the files its outputs install, and every bundle. `cargo xtask pack-catalog` writes it to `catalog.pack` in this crate, and continuous integration (CI) runs `cargo xtask pack-catalog --check`, which fails when that file is out of date.
- `Source`, the index the CLI installs from. It holds every bundle and, for each asset, its manifest, its files and a SHA-256 digest (a hash) of its manifest file, which `grimoire.lock` records. `Source::builtin` loads the pack built into this crate, and `Source::from_pack` loads any pack. Both parse every manifest and bundle strictly and check each one again on its own, so a damaged pack fails when it is loaded rather than at install time. The checks across the catalog aren't run again: `pack` ran them before the catalog was packed.

Not yet, as [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md) records:

- **A remote catalog.** ADR-0001 chose the embedded catalog for the first version and left a signed index fetched over the network for a later decision. The only source is the built-in one.
- **Older asset versions.** A pack holds one version of each asset, so a newer CLI can't write the files again for a lockfile that pins an older version. `grimoire verify` still works, because it needs only the recorded hashes.

## How the CLI uses it

When `grimoire` starts, it loads the built-in catalog with `Source::builtin` and passes the `Source` to every command. If the pack doesn't load, the CLI reports that its build is broken and exits with code 1.

```rust
use agent_grimoire_catalog::Source;
use agent_grimoire_model::{AssetId, Kind, RelPath, SourceId};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The catalog built into this crate. Each manifest and bundle in it is
    // parsed and checked again here, on its own.
    let source = Source::builtin()?;
    assert_eq!(source.id(), SourceId::Builtin); // `source = "builtin"` in grimoire.lock

    // One asset, by id: its manifest, and the files it installs.
    let id = AssetId::new("plain-language-docs")?;
    let asset = source.asset(&id).ok_or("not in the catalog")?;
    assert_eq!(asset.manifest.kind, Kind::Rule);
    let rule = asset.file(&RelPath::new("rule.md")?).ok_or("no rule.md")?;
    println!("{id} {}: {} bytes", asset.manifest.version, rule.len());

    // A bundle names assets that a project can select together. `load`
    // checked that each one exists before the catalog was packed.
    let bundle = source.bundle(&AssetId::new("rust-cli")?).ok_or("no such bundle")?;
    assert!(bundle.assets.keys().all(|member| source.asset(member).is_some()));
    Ok(())
}
```

## Documentation

- API reference: https://docs.rs/agent-grimoire-catalog
- Design: [`docs/architecture.md` §6.4, Catalog validation](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#64-catalog-validation-cargo-xtask-validate-also-a-ci-gate), the checks per asset, per kind and across the catalog; [ADR-0001, Catalog delivery](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md), which builds the catalog into the binary, behind a `Source`, so that installing needs no network code and the catalog is covered by the binary's own checksums and signatures; and [`docs/specs.md`, SKILLS](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#skills-agent-skills-specification), the Agent Skills specification as verified at its source.
- Versioning: [`docs/versioning.md`](https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md). Grimoire versions its crates together.
- Grimoire: https://github.com/jabbott-iii/Grimoire

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.
