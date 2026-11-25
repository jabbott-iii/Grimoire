# Subsystem: `xtask`

`xtask` is the repository's automation: a Rust program in `xtask/`, run as `cargo xtask <command>`, that generates the checked-in files Grimoire builds from other sources, validates the catalog, scaffolds new assets, and runs the release pipeline's steps. It guarantees that every generated file can be rebuilt from its source and checked with `--check`, so continuous integration (CI) fails when someone edits a generated file by hand or forgets to regenerate one. Its tests also guard the repository itself: the root `README.md`'s links, the shape of each crate's docs, and the permissions that reusable workflows are granted. [ADR-0010](../adr/0010-language-policy-exceptions.md) (Rust is the default for all code, with a short list of exceptions) is why it is Rust rather than shell scripts, and [ADR-0001](../adr/0001-catalog-delivery.md) (the catalog is embedded in the binary) is why `pack-catalog` exists.

`xtask` is a workspace member, never published, with no crate docs of its own: its commands are listed in `xtask/src/main.rs` and in "Everyday commands" in [`../../AGENTS.md`](../../AGENTS.md) ([ADR-0012](../adr/0012-crate-documentation.md), crate documentation, leaves it out).

## Why repository automation in Rust

A repository like this one needs code that isn't part of the product: generators, a catalog linter, a release builder. Written as shell or Python scripts, that code would sit outside Rust's lints and test harness, and would need a second toolchain on every contributor's machine and CI runner. As an `xtask` crate it uses the same model and catalog crates as the CLI, so a generated schema or a validation rule can't drift from the types the CLI parses. The cost is build time: the first `cargo xtask` run compiles the crate and its dependencies.

Every generator writes atomically (through a temporary file in the same directory, then a rename), and its `--check` mode compares what it would write with what is on disk and changes nothing. That is the pattern `AGENTS.md` names: generated files are checked in, with a `--check` mode in CI.

## The commands

Built in M1.3, M1.5, M2.5, M3.6, M4.7 and M5.1 as described here:

| Command | What it does | Task |
|---|---|---|
| `schema [--check]` | Writes or checks the JSON Schemas under `schemas/` | M1.3 |
| `validate [--catalog DIR] [--base REV]` | Validates the catalog. With `--base`, also requires version bumps for assets changed since the merge base with `REV` | M1.5 |
| `pack-catalog [--check]` | Writes or checks the catalog pack embedded in the binary | M2.5 |
| `new <kind> <id> --maintainer LOGIN [MEMBER]...` | Scaffolds an asset, or a bundle of existing assets, that validates as generated | M3.6 |
| `gen-marketplace [--check]` | Writes or checks the Claude Code plugin marketplace | M4.7 |
| `release <…>` | The release pipeline's steps (below, and [`release.md`](release.md)) | M5.1 |

- **Where it runs.** Each command finds the repository root at run time: the nearest ancestor of the current directory that holds `xtask/Cargo.toml` and a `Cargo.toml` with `[workspace]`. So a binary built from one checkout never validates or rewrites another.
- **`--catalog` is confined.** The directory is resolved against the root, and refused if it is the root itself, outside the repository, or missing.
- **Output and exit status.** Diagnostics go to stderr. The exit status is 0 on success, 1 for a validation failure or stale files, and 2 for a usage error.
- **In CI.** The `catalog` job of `.github/workflows/ci.yml` runs `schema --check`, `validate`, `pack-catalog --check`, `gen-marketplace --check` and `validate --base`, each as `cargo --locked xtask …`, so a stale `Cargo.lock` fails instead of being rewritten.

## Generated artifacts

| Artifact | Generator | CI check |
|---|---|---|
| `schemas/*.schema.json` (asset, bundle, config, lock) | `cargo xtask schema` (schemars) | `--check` |
| Packed builtin catalog | `cargo xtask pack-catalog` | `--check` ([ADR-0001](../adr/0001-catalog-delivery.md)) |
| `.claude-plugin/marketplace.json` + `plugins/claude-code/<name>/` | `cargo xtask gen-marketplace` (Phase 4) | `--check`, and `claude plugin validate --strict` on the marketplace and each plugin |

Release files and the container image are generated too, but at release time and never checked in: [`release.md`](release.md) and [`container-image.md`](container-image.md) describe them.

### `schemas/`

`schema` renders one JSON Schema per document type that `agent_grimoire_model::schema::documents` lists: `asset.schema.json` (`asset.toml`), `bundle.schema.json`, `grimoire.schema.json` (`grimoire.toml`) and `grimoire-lock.schema.json` (`grimoire.lock`). It uses schemars, the library that derives a schema from the Rust types, and writes each as pretty JSON with a trailing newline. A leftover `*.schema.json` from a renamed or removed type is stale too: `schema` deletes it, and `schema --check` reports it.

The schemas' descriptions come from the model types' doc comments, which link to the design documents. So a doc comment change in `agent-grimoire-model` needs `cargo xtask schema` as well, and `schema --check` fails until it runs. The types themselves are in [`model.md`](model.md).

### `catalog.pack`

`pack-catalog` packs `catalog/` with `agent_grimoire_catalog::pack` into `crates/agent-grimoire-catalog/catalog.pack`, which the catalog crate embeds with `include_bytes!`. The pack must sit inside that crate's directory so that `cargo publish` packages it. An invalid catalog isn't packed: the command prints the diagnostics and fails. The pack's format, and how the CLI loads it, are in [`catalog.md`, "The embedded pack and `Source`"](catalog.md#the-embedded-pack-and-source).

A change under `catalog/` therefore needs `cargo xtask pack-catalog`, or the CLI ships the old catalog. CI's `pack-catalog --check` catches a stale pack on a pull request, and `cargo xtask release plan` refuses a release whose checked-in pack doesn't match `catalog/` ([`release.md`](release.md); added in M7.5).

### The marketplace

`gen-marketplace` writes `.claude-plugin/marketplace.json` at the repository root and one plugin under `plugins/claude-code/<id>/` for every eligible asset and bundle, from `catalog/`. Which assets are eligible, how plugins are named and versioned, and what their READMEs say are in [`marketplace.md`](marketplace.md).
- It touches only the marketplace's own files: it rewrites the ones whose content changed, deletes the ones it no longer generates, and `--check` writes nothing.
- It refuses symbolic links, and fails when a name breaks Claude Code's reserved-name rules or an agent names an MCP server that isn't a plugin.
- After `gen-marketplace --check`, CI's `plugins` job installs a pinned, checksum-verified Claude Code and runs `claude plugin validate --strict` on the marketplace and on each plugin directory, with no secrets.

## `validate`

`validate` loads the catalog with `agent_grimoire_catalog::load` and prints every diagnostic, sorted, with the file it belongs to, then a summary line: `catalog: N assets, M bundles; E errors, W warnings`. It fails when there is an error. Three groups of checks run:

1. **The catalog checks** in `agent-grimoire-catalog`: per asset, per kind and catalog-wide. They are listed in [`catalog.md`, "Catalog validation"](catalog.md#catalog-validation).
2. **Rendering for every client** (`xtask/src/render.rs`, Phase 4), when the catalog has no errors. Every asset is rendered for each client it declares, through the real adapters, so a refusal only an adapter makes fails here rather than in a user's `grimoire apply`. Examples are a Cursor agent whose tools aren't exactly `read` and `search`, a comma in a Cursor glob, and an MCP tool name a client would rename.
3. **Version bumps**, with `--base REV` (`xtask/src/bumps.rs`). An asset needs a higher version whenever a file that is installed or built changes: `asset.toml` and anything else in its directory except `README.md`, `CHANGELOG.md`, `evals/` and `tests/`. The lockfile records hashes of installed content, so a silent content change under an unchanged version would break reproducibility.
   - The comparison is against `git merge-base REV HEAD`, with `git diff --no-renames`, so renaming or moving a file counts as removing one file and adding another, in each asset involved. Files that git doesn't track yet count as changes too.
   - A version may never go down. Removing an asset is allowed only if it was deprecated at the base, and renaming an asset counts as removing it.
   - A bundle's changes need a bump too, and removing a bundle warns.
   - The catalog checks already require a CHANGELOG entry for the current version.

   The versioning policy behind these rules is in [`../05-release.md`, "Asset versions"](../05-release.md#asset-versions). A change to an asset's binary that comes from outside its directory, such as a change to `agent-grimoire-mcp`, isn't seen; review checks that rule ([`../05-release.md`, "Tool and MCP server binaries (not checked)"](../05-release.md#tool-and-mcp-server-binaries-not-checked)).

## `new`

`new <kind> <id> --maintainer LOGIN` scaffolds a new asset under `catalog/<kind>/<id>/`, or, for `bundle`, a `catalog/bundles/<id>.toml` that lists the given members. The kinds are `rule`, `skill`, `agent`, `tool`, `mcp-server` and `bundle`. The id must be a valid asset id and unique across every asset and bundle, members must exist, and nothing is overwritten.

Built in M3.6 as described here. The templates are in `xtask/templates/<kind>/`, compiled into `xtask` with `include_str!`, and follow the exemplars ([`assets.md`](assets.md)):

| Kind | Files |
|---|---|
| `rule` | `asset.toml`, `rule.md`, `README.md`, `CHANGELOG.md`, `evals/cases.md` |
| `skill` | `asset.toml`, `SKILL.md`, `README.md`, `CHANGELOG.md`, `evals/cases.md` |
| `agent` | `asset.toml`, `agent.md`, `README.md`, `CHANGELOG.md`, `evals/cases.md` |
| `tool` | `asset.toml`, `Cargo.toml`, `src/main.rs`, `tests/cli.rs`, `README.md`, `CHANGELOG.md` |
| `mcp-server` | `asset.toml`, `Cargo.toml`, `src/main.rs`, `tests/protocol.rs`, `README.md`, `CHANGELOG.md` |

The placeholders are `@@ID@@` (the kebab-case id), `@@SNAKE@@` (the id with `_`), `@@BINARY@@` (`grimoire-tool-<id>` or `grimoire-mcp-<id>`) and `@@MAINTAINER@@` (a GitHub login). Each template validates cleanly as generated, so a new asset starts from a passing state. The MCP server template serves through `agent-grimoire-mcp` and requires `--no-auth` to serve HTTP without authorization, like `project-docs`. `new` runs `rustfmt` on the Rust files it writes, because line wrapping depends on the id's length, and ends by telling the author to fill in the content and run `validate` and `pack-catalog`.

CI's `scaffold` job generates a probe tool and a probe MCP server, then checks their formatting, lints and tests them, and validates the catalog with them in it. It runs cargo without `--locked`, because the probe crates are new to `Cargo.lock`.

## `release`

`release` holds the release pipeline's logic, so that `.github/workflows/release.yml` stays thin glue: `plan`, `build`, `sbom`, `channels`, `checksums`, `verify` and `notes` (M5.1), and `image base`, `image context`, `image check` and `image coverage` for the container image (M6.2). [`release.md`](release.md) describes the pipeline and those commands, and [`container-image.md`](container-image.md) the image.

## Checks that guard the repository

Three test modules in `xtask/src/`, compiled only for tests (`#[cfg(test)]`), check files that no Rust type parses. `cargo test --workspace --locked` runs them with the rest. [`../04-quality.md`, "Checked documents"](../04-quality.md#checked-documents) lists them among the project's checks.

### The root README's links (`readme_links.rs`)

Built in M7.2 as described here. The root `README.md` may link only to `https://` URLs, `mailto:` addresses and in-page `#fragment`s. The same file is the `agent-grimoire` crate's page on crates.io, which resolves a relative link against `crates/agent-grimoire/`; the long description of the PyPI wheels, which rewrites no links; and a plain file in every CLI archive, which holds no `docs/` (`README-RENDER-P7` in [`../specs.md`](../specs.md)). A relative link works on GitHub and breaks in all three.
- The README is parsed with pulldown-cmark, as the catalog's Markdown is, with GitHub's extensions on.
- Every link and image counts, including reference links and autolinks; an email autolink counts as its `mailto:` address. So do `href`, `src` and `srcset` attributes in raw HTML, and bare `http://` and `www.` URLs that GitHub turns into links: anywhere in a run of text where one starts the run or follows whitespace, `*`, `_`, `~` or `(`.
- Code spans and code blocks don't count. Every `<!--` in raw HTML starts a comment, and the only comments the README may hold are the release markers (the paired `before-first-release` lines that `cargo xtask release plan` checks).

The library crates' own READMEs link their `LICENSE-*` files relatively on purpose, so `crate_docs.rs` checks those READMEs with that one exception, through the same parser.

### The crate docs and READMEs (`crate_docs.rs`)

Built in M7.4 as described here. Every crate under `crates/` documents itself in the shape [ADR-0012](../adr/0012-crate-documentation.md) gives:
- **Crate docs.** The `//!` lines at the top of each crate's `src/lib.rs` open with the sections `# Where it fits`, `# The parts` and `# Example`, in that order. More sections may follow. A heading inside a code block isn't a section.
- **A README of its own** for each published crate other than `agent-grimoire`. It starts with `# <package name>`; has the sections `## Status`, `## How the CLI uses it`, `## Platforms` (optional), `## Documentation` and `## License`, in that order, and no other `##` section; links to the crate's docs.rs page; holds at least one Rust example with no lines hidden with `# `, which `src/lib.rs` runs as a doc test through a `#[cfg(doctest)]` item; and links only to absolute URLs, fragments and its own `LICENSE-APACHE` and `LICENSE-MIT`.
- **The workspace README.** Only `agent-grimoire` uses the repository's README as its crates.io page (`readme.workspace = true`); any other published crate needs its own.

The checks are structural: they can't tell whether the words are true. Review covers that.

### Reusable-workflow permissions (`workflow_permissions.rs`)

Built in M7.5 as described here, after the Release workflow failed to start on GitHub on 2026-11-18. A job that calls a reusable workflow in this repository must grant every permission that any job in the called workflow asks for. GitHub checks this when a run starts, for every job of the called workflow, including a job whose `if:` would skip it, and refuses the whole run when a job asks for more than the call grants (`SIGN-P6` in [`../specs.md`](../specs.md)). zizmor and actionlint don't check it, so without this test the first sign of a wrong grant is every run of the calling workflow failing to start once the change is pushed.

The workflows are read as the one block-style shape this repository writes: two-space indentation, bare `key:` names, job ids under a block `jobs:`, each job's keys at indent 4, and `permissions` either `{}` or one `scope: none|read|write` line per scope. A line in another shape that the reader looks at fails the test rather than being skipped. A called workflow's top-level `permissions` count for its jobs, and a `read` grant doesn't cover `write`.

## Invariants

1. Each generator's output on disk equals what it would generate now: `schema --check`, `pack-catalog --check` and `gen-marketplace --check` exit 0 on a clean checkout and change nothing.
2. `schemas/` holds exactly the four schema files; a stale extra one fails `schema --check`.
3. `pack-catalog` never packs an invalid catalog.
4. `gen-marketplace` writes only the marketplace's files, refuses symbolic links, gives a plugin a new version whenever its files change, and refuses an agent whose MCP server isn't a plugin.
5. `validate` fails on any asset that an adapter would refuse for a client the asset declares.
6. With `--base`, any change to an installed or built file of an asset needs a higher version, counting renames, moves and untracked files; versions never go down; and an asset may be removed only after it was deprecated at the base.
7. Every scaffold validates as generated, every placeholder is replaced, ids are unique, nothing is overwritten, and a bundle's members must exist.
8. `--catalog` can't name the repository root, a missing directory, or a directory outside the repository.
9. The root `README.md` links only to `https://`, `mailto:` and its own fragments, and its only comments are release markers.
10. Every crate under `crates/` has crate docs in the ADR-0012 shape, and every published library crate a README in that shape.
11. Every call to a reusable workflow in `.github/workflows/` grants what every job of the called workflow asks for.

## Tests

| Invariant | Tests |
|---|---|
| 1 | CI's `catalog` and `plugins` jobs, which run the three `--check` commands on every pull request; `check_and_write_touch_only_the_marketplace` (`xtask/src/marketplace.rs`). No unit test runs `schema --check` itself |
| 2 | No unit test; `schema --check` in CI |
| 3 | No test in `xtask`; the pack's own tests, such as `the_embedded_pack_loads` in `crates/agent-grimoire-catalog/src/source.rs`, are in [`catalog.md`](catalog.md#tests) |
| 4 | `the_builtin_catalog_generates`, `agents_depend_on_their_servers_and_name_plugin_tools`, `an_agent_whose_server_is_no_plugin_is_refused`, `plugin_versions_change_with_content`, `check_and_write_touch_only_the_marketplace`, `symbolic_links_are_refused` (`xtask/src/marketplace.rs`) |
| 5 | `adapter_refusals_fail_validation` (`xtask/src/render.rs`) |
| 6 | `unchanged_catalog_passes`, `content_change_requires_a_bump`, `untracked_content_counts_as_a_change`, `metadata_changes_need_no_bump`, `versions_must_not_go_down`, `removal_requires_prior_deprecation`, `renaming_an_asset_counts_as_removing_it`, `moving_a_file_between_assets_bumps_both`, `removing_a_deprecated_asset_is_allowed`, `bundle_changes_need_a_bump_and_removal_warns`, `option_like_bases_are_refused`, `a_missing_base_is_a_clear_error` (`xtask/src/bumps.rs`) |
| 7 | `every_template_validates_as_generated`, `placeholders_are_all_replaced`, `ids_are_unique_and_nothing_is_overwritten`, `bundles_need_existing_members`, `kinds_parse` (`xtask/src/new.rs`); CI's `scaffold` job, which also builds, lints and tests the generated crates |
| 8 | `catalog_paths_are_normalized_and_confined` (`xtask/src/main.rs`) |
| 9 | `root_readme_links_are_absolute`, `root_readme_comments_are_release_markers`, and the parser's own tests: `allows_https_mailto_and_fragments`, `finds_relative_links_images_and_references`, `finds_html_attributes_and_bare_urls`, `skips_code_and_collects_comments`, `finds_bare_urls_at_githubs_autolink_boundaries`, `reads_srcset_and_email_autolinks`, `finds_comments_inside_other_html`, `reports_lines_with_crlf` (`xtask/src/readme_links.rs`) |
| 10 | `every_crate_is_documented_in_the_shape`, and `accepts_the_shape`, `crate_docs_need_the_sections_in_order`, `a_heading_inside_a_code_block_is_not_a_section`, `readme_needs_its_title_sections_reference_and_example`, `readme_links_must_be_absolute_except_the_licenses` (`xtask/src/crate_docs.rs`) |
| 11 | `every_reusable_workflow_call_grants_what_its_jobs_ask_for`, and `refuses_a_called_job_that_asks_for_more_than_the_call_grants`, `accepts_a_call_that_grants_every_job_what_it_asks_for`, `the_called_workflows_top_level_permissions_count`, `a_read_grant_doesnt_cover_write`, `calls_it_cant_check_are_problems`, `shapes_it_doesnt_know_are_errors` (`xtask/src/workflow_permissions.rs`) |

The tests of the `release` subcommands are listed in [`release.md`](release.md) and [`container-image.md`](container-image.md).
