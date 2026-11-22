# Grimoire

[![M8ven Score](https://m8ven.ai/badge/mcp/jabbott-iii-grimoire-ord61h)](https://m8ven.ai/mcp/jabbott-iii-grimoire-ord61h?s=readme)

Grimoire is a catalog of AI-agent assets and a command-line tool (CLI), `grimoire`, that installs only the assets a project selects. The assets are rules, skills, agents, tools and MCP (Model Context Protocol) servers: the instructions and programs that AI coding assistants use. [The catalog](#the-catalog) explains each kind.

Each AI coding assistant that reads these assets is a *client*. Grimoire writes each asset in the files and folders that each of the project's clients expects: Claude Code, OpenAI Codex, Cursor, Gemini CLI, GitHub Copilot, or any client that reads `AGENTS.md`. It records everything it writes, so it can check, update and remove exactly that content.

<!-- before-first-release: before the first release, delete everything from this line to the closing /before-first-release line, in a pull request merged ahead of the release PR (docs/release.md §2 C). -->
> [!IMPORTANT]
> No version of Grimoire has been released yet, so the install commands below don't work yet. Until the first release, build the CLI from the latest commit on `main`, with Rust 1.96 or later:
>
> ```bash
> cargo install --locked --git https://github.com/jabbott-iii/Grimoire agent-grimoire
> ```
>
> Or clone the repository and run `cargo install --locked --path crates/agent-grimoire` in it. In a clone, rustup (the Rust toolchain installer) uses the toolchain that `rust-toolchain.toml` pins, 1.100.0, and installs it if it is missing. The `--git` command uses the toolchain you already have.

<!-- /before-first-release -->

## What Grimoire does and does not do

- **It writes only inside your project.** The project root is the directory you run `grimoire` in, or the one `-C DIR` names. `grimoire.toml`, the project's Grimoire configuration, lives there. Grimoire refuses any path that would lead outside the root. It also refuses any symbolic link at or above a path it manages, because writing through a link could put content somewhere you didn't expect.
- **Its catalog is built into the binary.** Grimoire has no network code: it downloads nothing and sends nothing. Release binaries are signed and have build provenance, a signed record of how they were built (see [Verifying a download](#verifying-a-download)).
- **It never runs asset content.** Installing copies files, or renders them (writes each client's version of them). Grimoire runs no install hooks (scripts that run during installation). It never starts the tool and MCP server binaries it configures: `grimoire doctor` only looks for them on `PATH`, the list of directories your shell searches for commands.
- **You consent to permissions.** Each asset declares what it needs: `filesystem-read`, `filesystem-write`, `network`, `shell`, `hooks`, and `env:NAME` for each environment variable it reads. `grimoire.toml` records what you accepted, asset by asset. If a new version of an asset needs more, Grimoire stops with exit code 3 until you accept it.
- **It locks what it wrote.** `grimoire.lock`, the lock file, records each asset's version and the SHA-256 hash of every file, block and entry Grimoire owns. A block is a marked section of a shared file such as `AGENTS.md`, and an entry is one key or array element in a configuration file such as `.mcp.json`. `grimoire verify` checks the hashes.
- **It doesn't overwrite your edits unasked.** If you edit content Grimoire manages, `plan` reports a conflict, and `apply` writes nothing until you choose `--force` (overwrite your edit) or `--write-new` (write `<path>.new` beside it).
- **It removes only its own content.** When an asset leaves `grimoire.toml`, `apply` deletes only what Grimoire wrote for it. `grimoire uninstall` removes all of Grimoire's content, and the lock file.
- **It stores no secrets.** MCP server entries refer to environment variables by name.

It doesn't install the tool and MCP server binaries that assets configure (see [The catalog](#the-catalog)). It also can't check whether a client has loaded or trusted what it wrote.

Clients also read each other's files, so content installed for one client can reach another. `plan` warns when an agent, a hook or an MCP server, or in some cases an `AGENTS.md` rule, reaches a client it wasn't installed for. Other rules and skills that another client reads aren't reported. The cross-client leaks table in [docs/security-model.md §3a][cross-client] lists every location Grimoire writes that another client is documented as reading, and whether `plan` warns about it.

The threat model, which says what Grimoire defends against, how, and the gaps it knows about, is in [docs/security-model.md][security-model].

[security-model]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/security-model.md
[cross-client]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/security-model.md#3a-cross-client-leaks

## Install

The package is `agent-grimoire` in every package manager, and the command is `grimoire`, except with winget (below). Prebuilt binaries cover Linux, macOS and Windows, on x86-64 and arm64. Every channel that installs a prebuilt binary installs the same one. The Linux builds are statically linked with musl (a C standard library), so they don't depend on the system's own C library.

### Cargo

```bash
cargo install --locked agent-grimoire
```

This builds Grimoire from source with Cargo, Rust's build tool and package manager. It needs Rust 1.96 or later; with an older Rust, Cargo stops with an error.

Keep `--locked`. On Cargo 1.96 to 1.99, it makes Cargo use the `Cargo.lock` file published with the crate (a Rust package). That file holds the dependency versions that continuous integration (CI) checks on Rust 1.96. Without `--locked`, those versions of Cargo choose the dependency versions again, and may pick ones that need a newer Rust. From Cargo 1.100, Cargo uses the published lock file by default, and `--locked` changes nothing.

To install the prebuilt binary instead, use cargo-binstall, a Cargo add-on that installs prebuilt binaries:

```bash
cargo binstall agent-grimoire
```

cargo-binstall downloads the release archive for your platform. It checks the crate's checksum, but not the archive's: it relies on HTTPS to deliver the archive unchanged. To verify the archive yourself, see [Verifying a download](#verifying-a-download).

### npm

npm is the package manager for Node.js.

```bash
npm install -g agent-grimoire
npx agent-grimoire --version   # or run it without installing
```

This needs Node 18 or later. The package's launcher, a small JavaScript program, runs the binary from a platform package, `@agent-grimoire/<os>-<cpu>`, that npm installs with it.

### PyPI

PyPI is the Python Package Index. pipx and uv each install a command-line program in an environment of its own.

```bash
pipx install agent-grimoire
uv tool install agent-grimoire
```

Use either one, or `pip install agent-grimoire` in a virtual environment. The wheel (Python's package format) holds the binary and no Python code.

### Homebrew

```bash
brew install jabbott-iii/tap/agent-grimoire
```

A tap is a repository of Homebrew formulae (Homebrew's install recipes), here a third-party one. Homebrew 6 and later require trust for third-party taps, and installing by the fully qualified name trusts just this formula. To use the short name instead:

```bash
brew tap jabbott-iii/tap
brew trust --formula jabbott-iii/tap/agent-grimoire
brew install agent-grimoire
```

### Scoop

Scoop is a command-line installer for Windows. A bucket is a Git repository of Scoop manifests.

```powershell
scoop bucket add jabbott-iii https://github.com/jabbott-iii/scoop-bucket
scoop install jabbott-iii/agent-grimoire
```

Scoop needs Git to add the bucket.

### winget

winget is the Windows Package Manager.

```powershell
winget install jabbott-iii.agent-grimoire
```

With winget, the command is `agent-grimoire`, because the command name `grimoire` belongs to another winget package. Open a new shell after installing.

### Install scripts

On Linux and macOS, `install.sh` needs curl (on Alpine Linux, run `apk add curl` first). It uses curl only, because wget can't be made to refuse a redirect from HTTPS to plain HTTP.

```bash
curl --proto '=https' --tlsv1.2 -fsSLO https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.sh
# Verify it (see below) and read it, then:
sh install.sh                   # installs to ~/.local/bin
sh install.sh --to ~/bin        # or set GRIMOIRE_INSTALL_DIR
```

On Windows:

```powershell
# Windows PowerShell 5.1 may not offer TLS 1.2, which GitHub requires; this line adds it (harmless in PowerShell 7).
[Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor [Net.SecurityProtocolType]::Tls12
Invoke-WebRequest https://github.com/jabbott-iii/Grimoire/releases/latest/download/install.ps1 -OutFile install.ps1 -UseBasicParsing
# Verify it (see below) and read it, then:
powershell -ExecutionPolicy Bypass -File .\install.ps1                    # installs to %LOCALAPPDATA%\Programs\grimoire\bin
powershell -ExecutionPolicy Bypass -File .\install.ps1 -InstallDir C:\tools  # or set GRIMOIRE_INSTALL_DIR
```

- Each script installs the version it was released with.
- Before it extracts anything, it checks the archive's SHA-256 against the value written into the script at release time.
- Neither script changes `PATH` or a shell startup file.
- To pin a version, download the script from `https://github.com/jabbott-iii/Grimoire/releases/download/vX.Y.Z/` instead.

A script is a release file like any other, so verify it before you run it, as [docs/release.md §4, Installing][release-install] describes.

### Container image

```bash
docker pull ghcr.io/jabbott-iii/grimoire@sha256:<digest>
```

Take the digest from the release notes. The digest is a SHA-256 hash that names one exact image. The image holds the CLI and every tool and MCP server binary in the catalog, for linux/amd64 and linux/arm64. [docs/container.md][container] shows how to verify the image and run it on a mounted repository.

### Verifying a download

Every file in a release, except the `.sigstore.json` bundles, comes with:
- a build provenance attestation, a signed statement of how the file was built;
- a cosign bundle (`<file>.sigstore.json`), which holds the file's signature and what is needed to check it. cosign is Sigstore's signing tool.

`SHA256SUMS` lists every release file except the `.sigstore.json` bundles and itself. Every archive also has a CycloneDX SBOM (software bill of materials): a list, in the CycloneDX format, of the software components inside it.

The commands to check them, with the flags that pin who signed, are in [docs/release.md §5, Verifying a download][release-verify].

[release-install]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/release.md#4-installing-users
[release-verify]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/release.md#5-verifying-a-download
[container]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/container.md

## Quick start

In the root of your project:

```bash
grimoire init --client claude-code --client agents-md
grimoire add conventional-commits
grimoire add security-reviewer --allow filesystem-read
grimoire plan
grimoire apply
grimoire verify
```

- `init` writes `grimoire.toml` for the clients you name. Without `--client`, it detects the clients the project already uses from files such as `CLAUDE.md`, `.cursor/` or `GEMINI.md`, and stops with exit code 2 if it finds none.
- `add` checks that the new selection resolves, that is, that the catalog's versions, dependencies, conflicts and clients allow it. Then it records the selection in `grimoire.toml`. It writes nothing else; `add --apply` also applies the change.
- `--allow` accepts an asset's permissions, which `grimoire show <id>` lists. On a terminal, `add` asks you instead. What you accept is recorded next to the asset, here as `allow = ["filesystem-read"]`.
- With `--no-input` or `--json`, when the `CI` environment variable is set to a non-empty value, or when standard input isn't a terminal, `add` doesn't ask. A permission you haven't accepted is then an error (exit code 3) that names the `--allow` value to pass, and `grimoire.toml` stays as it was.
- `plan` shows what `apply` would create, update or delete, with diffs and warnings. It writes nothing.
- `apply` writes the files and `grimoire.lock`. When there is nothing to do, it changes nothing, so it is safe to run again.
- `verify` checks every output recorded in `grimoire.lock` against the files on disk.

In this project, `plan` lists (warnings and diffs left out):

```text
  create    .claude/agents/security-reviewer.md  security-reviewer (claude-code)
  create    AGENTS.md [block conventional-commits]  conventional-commits (claude-code, agents-md)
  create    CLAUDE.md [block grimoire]  grimoire (claude-code)
plan: 3 to create, 0 to update, 0 to delete, 0 conflicting, 0 unchanged
```

The rule is written into `AGENTS.md` once, for both clients, as a block between Grimoire's marker lines. Claude Code reads it through a block in `CLAUDE.md` that imports `@AGENTS.md`. `plan` also warns (`plan/cross-client-agent`) that Cursor and GitHub Copilot load agents from `.claude/agents/` too.

Commit `grimoire.toml` and `grimoire.lock` together with the files Grimoire wrote.

Later:
- `grimoire remove <id>` takes an asset out of `grimoire.toml`, and the next `apply` deletes what Grimoire wrote for it.
- The catalog is part of the binary, so a newer `grimoire` can carry newer asset versions. While `grimoire.lock` pins a version that the binary doesn't have, `plan` and `apply` stop with exit code 3. `grimoire update` moves the lock file to the binary's versions and applies them.
- `grimoire doctor` reports each client's installed outputs, what each client needs before it reads them (trusting the project, for example), and whether tool and MCP server binaries are on `PATH`.

### In CI

```bash
grimoire plan --check
grimoire verify
```

- `plan --check` exits with code 5 when `grimoire.toml`, `grimoire.lock` and the files disagree, so that `apply` would change something. It exits with code 4 when `apply` would stop on a conflict.
- `verify` exits with code 5 when content Grimoire wrote no longer matches the lock file, and with code 3 when there is no lock file.
- Grimoire never prompts when `CI` is set to a non-empty value.
- `--json` prints exactly one JSON document on standard output (stdout) instead of the human-readable output ([docs/cli.md][cli-json]).
- Use the `grimoire` version the rest of the project uses. Another version can carry other asset versions, and `plan --check` then stops with exit code 3 until someone runs `grimoire update`.

Every exit code is in [docs/cli.md][cli-exit].

[cli-json]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md#json-output
[cli-exit]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md#exit-codes

## Clients

`grimoire init --client <id>` takes these IDs. Each client gets the kinds of asset it can host, in its own layout:

| Client ID | Client | What Grimoire writes |
|---|---|---|
| `claude-code` | Claude Code | Rules in `.claude/rules/`, skills in `.claude/skills/`, agents in `.claude/agents/`, MCP servers in `.mcp.json`, tool hooks in `.claude/settings.json`. When rules go into `AGENTS.md`, a block in `CLAUDE.md` imports it |
| `agents-md` | Any client that reads `AGENTS.md` | Rules as blocks in `AGENTS.md`, skills in `.agents/skills/` |
| `codex` | OpenAI Codex | Rules in `AGENTS.md`, skills in `.agents/skills/`, MCP servers in `.codex/config.toml` |
| `cursor` | Cursor | Rules in `.cursor/rules/`, or none of its own when `AGENTS.md` has them; skills; read-only agents in `.cursor/agents/`; MCP servers in `.cursor/mcp.json` |
| `gemini-cli` | Gemini CLI | Rules as blocks in `GEMINI.md`, or a block there that imports `AGENTS.md`; skills in `.agents/skills/`; agents in `.gemini/agents/`; MCP servers in `.gemini/settings.json` |
| `copilot` | GitHub Copilot | Rules in `.github/instructions/`, or none of its own when `AGENTS.md` has them; skills; agents in `.github/agents/`; MCP servers in `.mcp.json` |

- Cursor and Copilot get their skills in `.agents/skills/` or `.claude/skills/`, whichever the other selected clients already use.
- Tool hooks are commands that a client runs automatically at set points, such as before a tool call. They are written only for Claude Code, because the other clients can't block an action when a hook fails.
- Agents are never written for Codex, because a Codex agent can't be limited to its tools. Cursor gets only agents whose tools are exactly `read` and `search`, which it runs read-only. [ADR-0008][adr-0008], the decision on what Grimoire writes for Codex, Cursor, Gemini CLI and Copilot, gives the details.
- Some clients read project files only after you trust the project; `grimoire doctor` says which.
- Clients read each other's files. `plan` warns when an agent, a hook or an MCP server, or in some cases an `AGENTS.md` rule, reaches a client it wasn't installed for. Other rules and skills that another client reads aren't reported. See the cross-client leaks in [docs/security-model.md §3a][cross-client] and the client read matrix in [docs/architecture.md §8.2][architecture-clients].

`grimoire.toml`, `grimoire.lock`, permissions and clients are described in [docs/configuration.md][configuration].

[adr-0008]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0008-phase-4-client-adapters.md
[architecture-clients]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#82-client-read-matrix-and-cross-client-planning
[configuration]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/configuration.md

## The catalog

| Kind | What it is | What Grimoire installs |
|---|---|---|
| `rule` | Markdown instructions a client loads without being asked, always or for matching files | A rule file, or a block in a shared instruction file |
| `skill` | A directory in the Agent Skills format (a `SKILL.md` file and the files it uses), which the model loads when its description fits the task | A byte-for-byte copy of the directory |
| `agent` | A subagent (a separate agent that the main agent can hand a task to), with its own prompt and an explicit list of tools | A subagent definition |
| `tool` | A Rust command-line program, which can be wired into a client as hooks | The client wiring only, such as hook entries |
| `mcp-server` | A Rust MCP server, a program that offers the client extra tools or data over the Model Context Protocol | An entry in the client's MCP configuration |

A bundle is a named, versioned list of assets: `grimoire add bundle:<id>` selects one, and `grimoire add tag:<tag>` selects every asset with that tag. Each asset has a maturity: `experimental`, `stable` or `deprecated`.

```bash
grimoire list                # every asset and bundle
grimoire list --kind skill   # or --tag, --client, --maturity
grimoire list <text>         # search ids, summaries, descriptions and tags
grimoire show <id>           # description, license, permissions, outputs per client, changes
```

Grimoire configures tools and MCP servers, but never installs their binaries. Each release has an archive for each of them, and the container image holds them all.

Some tools' hooks fail closed: when the hook can't run, the client blocks the action instead of allowing it. Such a tool blocks the client's actions while its binary is missing, so install the binary first. `plan` warns until the binary is on `PATH`.

To add or change an asset, read [catalog/README.md][catalog].

[catalog]: https://github.com/jabbott-iii/Grimoire/blob/main/catalog/README.md

## Documentation

- [docs/cli.md][cli]: the commands, global options, exit codes, JSON output and environment variables.
- [docs/diagnostics.md][diagnostics]: every diagnostic code a user can see. Every error or warning Grimoire reports has a stable code, such as `plan/cross-client-agent`.
- [docs/configuration.md][configuration]: `grimoire.toml`, `grimoire.lock`, permissions and clients.
- [docs/versioning.md][versioning]: the versioning and deprecation policy.
- [docs/security-model.md][security-model]: the threat model.
- [docs/container.md][container]: the container image, and running the CLI and MCP servers from it.
- [docs/release.md][release]: installing, verifying a download, and how a release is made.
- [docs/architecture.md][architecture]: the design.
- [docs/adr/][adr]: the architecture decision records (ADRs), which give the reasons behind the design.

[cli]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/cli.md
[diagnostics]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/diagnostics.md
[versioning]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md
[release]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/release.md
[architecture]: https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md
[adr]: https://github.com/jabbott-iii/Grimoire/tree/main/docs/adr

## Contributing

Read [CONTRIBUTING.md][contributing] before you open an issue or a pull request. Everyone who takes part is expected to follow the [code of conduct][conduct].

[contributing]: https://github.com/jabbott-iii/Grimoire/blob/main/CONTRIBUTING.md
[conduct]: https://github.com/jabbott-iii/Grimoire/blob/main/CODE_OF_CONDUCT.md

## Security

Report a vulnerability privately, through GitHub's form, not in a public issue:

https://github.com/jabbott-iii/Grimoire/security/advisories/new

The policy is in [SECURITY.md][security].

[security]: https://github.com/jabbott-iii/Grimoire/blob/main/SECURITY.md

## License

Grimoire is licensed under either the [MIT License][license-mit] or the [Apache License, Version 2.0][license-apache], at your option (`MIT OR Apache-2.0`). Each catalog asset declares its own license, which `grimoire show` prints.

Unless you say otherwise, a contribution you submit for inclusion in Grimoire is licensed under both, as above, with no additional terms or conditions (Apache License 2.0, section 5). No sign-off or Developer Certificate of Origin (DCO) is needed.

[license-mit]: https://github.com/jabbott-iii/Grimoire/blob/main/LICENSE-MIT
[license-apache]: https://github.com/jabbott-iii/Grimoire/blob/main/LICENSE-APACHE
