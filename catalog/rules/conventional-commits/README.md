# conventional-commits

An always-on rule that tells the agent how to write commit messages in the [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) format, so that history stays readable and release tools that read Conventional Commits, such as release-plz, can generate changelogs and choose versions from it.

It covers the `type(scope)!: description` header, what `feat` and `fix` mean, a default list of other types, the body and git-trailer footers, breaking changes (`!` and `BREAKING CHANGE:`), reverts, and how each kind of commit maps to a SemVer release. It leaves out the unreleased draft of the next version of the specification.

The rule only says how a message is written. It never tells the agent to commit, and it tells the agent to follow the repository's own commit conventions (in `CONTRIBUTING.md`, `AGENTS.md` or a commitlint config) where those are stricter.

## When to use it

- The repository already uses Conventional Commits, or you want agents to start writing them.
- Releases or changelogs are generated from commit messages.
- Agents draft commit messages or pull request titles for you to review.

Skip it if the project uses a different commit style: the rule would contradict it.

## What Grimoire installs

When `agents-md` or `codex` is selected, the rule is written once, as a managed block in `AGENTS.md`, and every other selected client reads it from there:
- Claude Code and Gemini CLI through an `@AGENTS.md` import that Grimoire adds to `CLAUDE.md` or `GEMINI.md`;
- Cursor and Copilot directly, because they read `AGENTS.md`.

Otherwise each client gets its own always-on copy:

| Client | Output |
|---|---|
| `claude-code` | `.claude/rules/conventional-commits.md`, with no `paths` frontmatter, so Claude Code loads it at the start of every session |
| `cursor` | `.cursor/rules/conventional-commits.mdc` with `alwaysApply: true` |
| `gemini-cli` | A managed block in `GEMINI.md` |
| `copilot` | `.github/instructions/conventional-commits.instructions.md` with `applyTo: "**"` |

`grimoire plan` reports any client that may still read the rule twice, such as Copilot CLI, which reads both `AGENTS.md` and the import in `CLAUDE.md`.

## License of the source

The rule is written in Grimoire's own words and links to the specification. The Conventional Commits specification text is licensed CC BY 3.0 and is not copied here.

## Security notes

- The rule is plain text loaded into the agent's context in every session. It grants no tools and runs nothing.
- It can only change how the agent words commit messages. It does not make the agent commit, push or rewrite history.

## Permissions

None. Rules declare no permissions.
