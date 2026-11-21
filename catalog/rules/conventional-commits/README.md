# conventional-commits

An always-on rule for AI coding agents: the agent's client (the AI coding tool it runs in) loads it in every session, not only when certain files are in play. It tells the agent how to write commit messages in the [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) format.

That format keeps history readable. It also lets release tools that read Conventional Commits, such as release-plz (which opens release pull requests with the next version and changelog), generate changelogs and choose versions from the history.

The rule covers:
- the `type(scope)!: description` header, what `feat` and `fix` mean, and a default list of other types;
- the body, and footers in Git's trailer form (`Key: value` lines at the end of the message, such as `Refs: #88`);
- breaking changes (`!` and `BREAKING CHANGE:`) and reverts;
- how each kind of commit maps to a release under SemVer (semantic versioning, with its major, minor and patch releases).

It leaves out the unreleased draft of the next version of the specification.

The rule only says how a message is written. It never tells the agent to commit. It tells the agent to follow the repository's own commit conventions where those are stricter, for example in `CONTRIBUTING.md`, `AGENTS.md` or a commitlint config (the configuration of a tool that checks commit messages).

## When to use it

- The repository already uses Conventional Commits, or you want agents to start writing them.
- Releases or changelogs are generated from commit messages.
- Agents draft commit messages or pull request titles for you to review.

Skip it if the project uses a different commit style: the rule would contradict it.

## What Grimoire installs

When `agents-md` or `codex` is selected, the rule is written once, as a managed block in `AGENTS.md`. A managed block is a section between Grimoire's marker lines, which Grimoire updates or removes without touching the rest of the file. Writing the rule once means that a client which reads `AGENTS.md` doesn't also get a copy in its own rules folder and read the rule twice.

Every other selected client reads the rule from `AGENTS.md`:
- Claude Code and Gemini CLI through an `@AGENTS.md` import (a line that pulls in the text of `AGENTS.md`) that Grimoire adds to `CLAUDE.md` or `GEMINI.md`;
- Cursor and Copilot directly, because they read `AGENTS.md`.

Otherwise each client gets its own always-on copy:

| Client | Output |
|---|---|
| `claude-code` | `.claude/rules/conventional-commits.md`, with no `paths` frontmatter (the metadata block at the top of the file that would limit the rule to matching files), so Claude Code loads it at the start of every session |
| `cursor` | `.cursor/rules/conventional-commits.mdc` with `alwaysApply: true` |
| `gemini-cli` | A managed block in `GEMINI.md` |
| `copilot` | `.github/instructions/conventional-commits.instructions.md` with `applyTo: "**"` |

`grimoire plan`, which shows what `grimoire apply` would change, reports any client that may still read the rule twice. One example is Copilot CLI, which reads both `AGENTS.md` and the import in `CLAUDE.md`.

## License of the source

The rule is written in Grimoire's own words and links to the specification. The Conventional Commits specification text is licensed CC BY 3.0 (Creative Commons Attribution 3.0) and is not copied here.

## Security notes

- The rule is plain text loaded into the agent's context (the text the model sees) in every session. It grants no tools and runs nothing.
- It can only change how the agent words commit messages. It does not make the agent commit, push or rewrite history.

## Permissions

None. Rules declare no permissions.
