# Packaging

These files turn a built `grimoire` into the release's distribution channels: the ways users can install it, such as the install scripts, npm, Homebrew and Scoop.

- `cargo xtask release` (the repository's Rust release commands) reads them, and `.github/workflows/release.yml` runs it.
- The runbook, the step-by-step guide to a release, is [`docs/05-release.md`](../docs/05-release.md).
- The design is [ADR-0009](../docs/adr/0009-release-pipeline.md), the architecture decision record (ADR) for the release pipeline and its distribution channels.

| Path | What | Where it ends up |
|---|---|---|
| `install/install.sh` | Installer template for Linux and macOS, in POSIX `sh` (the portable shell language, with no bash extensions). It downloads with curl only, because wget can't refuse a redirect from HTTPS to HTTP | The release asset `install.sh` |
| `install/install.ps1` | Installer template for Windows, for Windows PowerShell 5.1 and PowerShell 7 | The release asset `install.ps1` |
| `npm/grimoire.js` | The npm launcher, a small Node.js script that starts the real binary. It runs the binary from the matching `@agent-grimoire/<os>-<cpu>` package and forwards `SIGINT`, `SIGTERM` and `SIGHUP` to it | `bin/grimoire.js` in the `agent-grimoire` npm package |
| `homebrew-tap/sync-formula.yml` | Workflow template for the Homebrew tap `jabbott-iii/homebrew-tap` | Copied there by the maintainer |
| `scoop-bucket/sync-manifest.yml` | Workflow template for the Scoop bucket `jabbott-iii/scoop-bucket` | Copied there by the maintainer |

Some of these terms in more detail:

- **npm** is the package manager and registry for JavaScript and Node.js. **PyPI**, the Python Package Index, is the registry for Python packages.
- **The launcher** is needed because npm puts only the top-level package's `bin` (its commands) on `PATH`, so a binary inside a platform package (one of the six `@agent-grimoire/<os>-<cpu>` packages) can't serve `npm i -g` or `npx` on its own. It forwards the signals so that stopping the launcher stops `grimoire` too.
- **A Homebrew tap** and **a Scoop bucket** are separate Git repositories that hold install recipes: formulae for Homebrew (macOS and Linux) and manifests for Scoop (Windows). A workflow is a GitHub Actions automation file.

## What is generated

`cargo xtask release channels` writes every channel file from the built archives.

- **Install scripts.** It renders the two templates, replacing `@@VERSION@@` and `@@SHA256:<target>@@` and nothing else. A target is one platform the binary is built for, and its SHA-256 is the checksum of that target's archive.
  - `install.sh` holds the four Linux and macOS targets, and `install.ps1` the two Windows targets.
  - A target that wasn't built (`--partial`) gets an empty SHA-256, and the script refuses to install it.
  - `channels` fails on any other `@@…@@` key, or if a placeholder is left.
- **The templates are valid scripts as they stand.** The placeholders sit inside single-quoted strings, so the linters (tools that check code for likely mistakes) can check the templates too. Run unrendered, a script refuses to install.
- **npm launcher.** `grimoire.js` is copied as it is.
  - The `package.json` files are generated.
  - A platform package's `package.json` must not set `exports` (the field that limits which of a package's files other code can load). `require.resolve`, which the launcher uses to find the binary, can't find it then.
- **No templates for the rest.**
  - The Homebrew formula (`agent-grimoire.rb`), the Scoop manifest (`agent-grimoire.json`), the winget (Windows Package Manager) manifests, and the npm `package.json` files and READMEs are written by the Rust code in `xtask/`.
  - The npm packages don't carry the root `README.md`; the CLI (command-line interface) archives and the wheels (the Python packages for PyPI) do.

Change a template here, never a rendered file.

## Language policy

Grimoire's code is Rust, so that one toolchain, one lint policy and one test harness cover it. Every file here is one of the exceptions in [ADR-0010](../docs/adr/0010-language-policy-exceptions.md), which gives the reason for each, what it may do and how it is checked.

- Keep these files small, and keep each to its scope.
- Decisions about what a release contains belong in `cargo xtask`, where they are unit-tested. These files only install, launch or sync what a release already holds.

## How CI checks them

CI (continuous integration) runs the verification jobs in `release.yml` on every pull request, as a dry run that builds and checks but publishes nothing, and before every release.

- **`verify-linux`:**
  - ShellCheck v0.11.0 (a linter for shell scripts), pinned by SHA-256, with `--shell=sh`, on `install.sh` as a template and as rendered.
  - PSScriptAnalyzer 1.25.0 (a linter for PowerShell) with its default rules on `install.ps1`.
- **`verify-install`, on every built target:**
  - `install.sh`, or `install.ps1` under both PowerShell 7 and Windows PowerShell 5.1, against a local `python3 -m http.server` (Python's built-in web server). This includes a tampered archive that the script must refuse.
  - The launcher and the platform tarball (the packed npm package) installed with npm, then `npx grimoire --version`.
- **The sync workflow templates** are linted with actionlint and zizmor, two linters for GitHub Actions workflows; zizmor looks for security problems. The templates run only in their own repositories.

To check the templates locally:

```bash
shellcheck --shell=sh packaging/install/install.sh
pwsh -NoProfile -Command 'Invoke-ScriptAnalyzer -Path packaging/install/install.ps1'
node --check packaging/npm/grimoire.js
```
