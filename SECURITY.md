# Security policy

## Reporting a vulnerability

Report it privately, with GitHub's form: https://github.com/jabbott-iii/Grimoire/security/advisories/new

- Don't describe it in a public issue, pull request or discussion.
- The form needs a signed-in GitHub account. It opens a private advisory that only you, the maintainer and the people the maintainer adds to it can see.
- Say which Grimoire version you used (`grimoire --version`), how you installed it, your operating system, and the client and asset involved, if any. Give the steps to reproduce the problem, or a proof of concept, and its impact. The form also asks for a summary.
- Keep the details private until the advisory is published.

**Without a GitHub account.** The form needs an account, and so does opening an issue. If you can't use one, ask someone who has an account to open an issue for you with the "Ask for a private contact" form, at https://github.com/jabbott-iii/Grimoire/issues/new/choose. The issue is public and asks the maintainer for a private channel, so it must say nothing about the problem itself: no description, no affected version and no proof of concept.

## What to expect

Grimoire has one maintainer, who handles every report.
- **Acknowledgement within 7 days**, in the advisory.
- **Updates in the advisory**, as comments you can reply to.
- **No fixed deadline for the fix.** It depends on the problem, and one person does the work.
- **Publication when the fix is released:** the advisory is published once a release with the fix is out.

## What happens to a report

The steps, as GitHub documents them, except step 4, which is Grimoire's own:
1. **Triage.** The report arrives as a proposed advisory. GitHub adds you to it as a collaborator and credits you.
2. **Accept or close.** The maintainer accepts it as a draft advisory, which is still private, or closes it with a comment that says why.
3. **Fix.** The fix is made in a temporary private fork that GitHub creates for the advisory, where you can review and test it. GitHub runs no CI in that fork.
4. **Release.** The commit that brings the fix to `main` names the advisory's GHSA ID (`GHSA-xxxx-xxxx-xxxx`) in its first line. release-plz writes the release's `CHANGELOG.md` section from the commit messages, and the release notes are built from that section, so they name the advisory. Before merging the release PR, the maintainer checks that its `CHANGELOG.md` section names the GHSA ID.
5. **CVE.** The maintainer can request a CVE ID through GitHub, which is a CVE Numbering Authority. GitHub usually reviews a request within 72 hours, and publishes the CVE after the advisory.
6. **Credit.** Everyone credited is invited to accept. Your username appears in the published advisory only if you accept, so you can decline.
7. **Publication.** Once the release with the fix is out, the maintainer publishes the advisory, and GitHub deletes the temporary fork. Anyone can then read the advisory and the accepted credits; only its collaborators can see the discussion. GitHub reviews a published advisory for the GitHub Advisory Database, which can take up to 72 hours, and may then send Dependabot alerts.

## Which packages an advisory names

An advisory lists each affected package, with its affected and patched versions, in the ecosystems the GitHub Advisory Database supports:
- **Rust (crates.io):** `agent-grimoire`, and whichever of `agent-grimoire-model`, `-catalog`, `-resolve`, `-install` and `-adapters` are affected;
- **npm:** `agent-grimoire` and the `@agent-grimoire/*` platform packages;
- **pip (PyPI):** `agent-grimoire`.

The database has no ecosystem for Homebrew, Scoop, winget, install scripts or container images. The asset binaries (`grimoire-tool-*` and `grimoire-mcp-*`) are published only in the release archives and the container image, so no package in the database covers them either. Whichever way you installed Grimoire, don't count on a Dependabot alert to tell you about a fix. The release notes of the release with the fix name the advisory (step 4 above). To hear about fixes, watch the repository's releases (Watch → Custom → Releases), or check https://github.com/jabbott-iii/Grimoire/security/advisories.

## Supported versions

Only the latest release gets security fixes. A fix ships in a new release: a published release is never changed.
- **The CLI:** upgrade to the release with the fix.
- **An asset:** its fixed version ships in that release too, but upgrading `grimoire` doesn't change your project. Run `grimoire update <id>`, which moves the pin in `grimoire.lock` and applies the fixed content. Until then, the project keeps the old content.
- **A tool or MCP server:** Grimoire never installs their binaries, so also install the binary from that release's archives or container image. A rebuilt binary can keep its asset version, and so its `--version` (§4 of the versioning policy, https://github.com/jabbott-iii/Grimoire/blob/main/docs/versioning.md). Identify the fixed one by the release it comes from and its archive's SHA-256 in that release's `SHA256SUMS`, or for the image, by the digest in that release's notes.

<!-- before-first-release: before the first release, delete everything from this line to the closing /before-first-release line, in a pull request merged ahead of the release PR (docs/release.md §2 C). -->
**No release exists yet.** Until the first one, report problems in the `main` branch.
<!-- /before-first-release -->

## Scope

In scope:
- the `grimoire` CLI and its library crates under `crates/`, including `agent-grimoire-mcp`, the shared hardening crate that every MCP server in the catalog uses;
- the catalog under `catalog/`: each asset's content, the tool and MCP server binaries built from it, and the Claude Code marketplace generated from it;
- the release artifacts: the archives, `SHA256SUMS`, signatures, attestations and SBOMs; the install scripts; the crates.io, npm and PyPI packages; the Homebrew formula and the Scoop and winget manifests; and the container image `ghcr.io/jabbott-iii/grimoire`;
- the workflows and scripts under `.github/` that build, sign and publish them, and the templates under `packaging/`.

Report these elsewhere:
- **rmcp**, the Rust MCP SDK that the MCP servers are built on: to [modelcontextprotocol/rust-sdk](https://github.com/modelcontextprotocol/rust-sdk), through its own security policy.
- **Other dependencies:** to the project concerned. If Grimoire ships an affected version, or uses a dependency in a way that exposes its users, report that here as well.
- **Client tools**, such as Claude Code, Codex, Cursor, Gemini CLI and GitHub Copilot: to their vendors.
- **What a client does with a file Grimoire wrote** is the client's behaviour, so it goes to the client's vendor, unless Grimoire wrote the wrong thing. If Grimoire grants more than an asset declares or the project accepted, or writes outside the project root, report it here.

## Threat model

The security model, https://github.com/jabbott-iii/Grimoire/blob/main/docs/security-model.md, describes what Grimoire defends against, how, and the gaps it knows about. A report that a documented gap is worse than described is in scope.
