# Packaging

These files turn a built `grimoire` into the release's distribution channels. `cargo xtask release` reads them, and `.github/workflows/release.yml` runs it. The runbook is [`docs/release.md`](../docs/release.md), and the design is [ADR-0009](../docs/adr/0009-release-pipeline.md).

| Path | What | Where it ends up |
|---|---|---|
| `install/install.sh` | Installer template for Linux and macOS, in POSIX `sh`. It downloads with curl only | The release asset `install.sh` |
| `install/install.ps1` | Installer template for Windows, for Windows PowerShell 5.1 and PowerShell 7 | The release asset `install.ps1` |
| `npm/grimoire.js` | The npm launcher. It runs the binary from the matching `@agent-grimoire/<os>-<cpu>` package and forwards `SIGINT`, `SIGTERM` and `SIGHUP` to it | `bin/grimoire.js` in the `agent-grimoire` npm package |
| `homebrew-tap/sync-formula.yml` | Workflow template for `jabbott-iii/homebrew-tap` | Copied there by the maintainer |
| `scoop-bucket/sync-manifest.yml` | Workflow template for `jabbott-iii/scoop-bucket` | Copied there by the maintainer |

## What is generated

`cargo xtask release channels` writes every channel file from the built archives.

- **Install scripts.** It renders the two templates, replacing `@@VERSION@@` and `@@SHA256:<target>@@` and nothing else.
  - `install.sh` holds the four Linux and macOS targets, and `install.ps1` the two Windows targets.
  - A target that wasn't built (`--partial`) gets an empty SHA-256, and the script refuses to install it.
  - `channels` fails on any other `@@…@@` key, or if a placeholder is left.
- **The templates are valid scripts as they stand.** The placeholders sit inside single-quoted strings, so the linters can check the templates too. Run unrendered, a script refuses to install.
- **npm launcher.** `grimoire.js` is copied as it is. The `package.json` files are generated, and must not set `exports` in a platform package: `require.resolve` can't find the binary then.
- **No templates for the rest.** The Homebrew formula (`agent-grimoire.rb`), the Scoop manifest (`agent-grimoire.json`), the winget manifests, and the npm `package.json` files and READMEs are written by the Rust code in `xtask/`. The npm packages don't carry the root `README.md`; the CLI archives and the wheels do.

Change a template here, never a rendered file.

## Language policy

Grimoire's code is Rust. Every file here is one of the exceptions in [ADR-0010](../docs/adr/0010-language-policy-exceptions.md), which gives the reason for each, what it may do and how it is checked.

- Keep these files small, and keep each to its scope.
- Decisions about what a release contains belong in `cargo xtask`, where they are unit-tested. These files only install, launch or sync what a release already holds.

## How CI checks them

The verification jobs in `release.yml` run on every pull request (dry-run) and before every release.

- **`verify-linux`:**
  - ShellCheck v0.11.0, pinned by SHA-256, with `--shell=sh`, on `install.sh` as a template and as rendered.
  - PSScriptAnalyzer 1.25.0 with its default rules on `install.ps1`.
- **`verify-install`, on every built target:**
  - `install.sh`, or `install.ps1` under both PowerShell 7 and Windows PowerShell 5.1, against a local `python3 -m http.server`, including a tampered archive that the script must refuse.
  - The launcher and the platform tarball installed with npm, then `npx grimoire --version`.
- **The sync workflow templates** are linted with actionlint and zizmor. They run only in their own repositories.

To check the templates locally:

```bash
shellcheck --shell=sh packaging/install/install.sh
pwsh -NoProfile -Command 'Invoke-ScriptAnalyzer -Path packaging/install/install.ps1'
node --check packaging/npm/grimoire.js
```
