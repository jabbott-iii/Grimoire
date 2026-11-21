# plain-language-docs

An always-on rule for writing and updating documentation: the agent's client (the AI coding tool it runs in) loads it in every session, not only when certain files are in play. It asks the agent to write so that a second-year computer science student can follow the text, without dropping anything a senior engineer needs.

It applies to READMEs, guides, tutorials, reference pages, design records, doc comments (comments in source code that document it), command help text and changelog entries. It covers four things:
- **Keeping jargon down:** plain phrases where they are just as exact, each unfamiliar term explained the first time it appears, acronyms spelled out on first use, and no insider shorthand such as "footgun" or a bare "ADR-0011".
- **Keeping the meaning:** exact numbers, limits, terms and caveats stay, and a precise sentence that can't be simplified gets an explanation beside it.
- **Helping newer readers:** the reason behind each design choice, one idea at a time, concrete examples, analogies only next to the real explanation, short paragraphs and headings that are easy to skim.
- **A final check as two readers:** the agent rereads its draft as a student, looking for undefined terms and skipped steps, and then as a senior engineer, looking for anything vague, oversimplified or wrong. It also checks American English spelling and grammar.

The rule only shapes the text the agent writes or changes. It tells the agent not to rewrite parts of a document it wasn't asked to touch.

It also tells the agent to follow the repository's own style guide (in `CONTRIBUTING.md` or `AGENTS.md`, for example) wherever that is stricter or more specific. The one exception is spelling: the agent always writes American English.

## When to use it

- The documentation should serve newcomers and experienced engineers alike, for example in an open-source project, or in a team that onboards new people often.
- Agents draft or update READMEs, guides, reference pages or doc comments for you to review.
- The project writes American English.

Skip it if the project writes British English or another variety: the rule keeps to American spelling even when the style guide asks otherwise. Skip it too if the project wants a terse reference style for experts only, which the rule would contradict where the guide isn't stricter.

## What Grimoire installs

When `agents-md` or `codex` is selected, the rule is written once, as a managed block in `AGENTS.md`. A managed block is a section between Grimoire's marker lines, which Grimoire updates or removes without touching the rest of the file. Writing the rule once means that a client which reads `AGENTS.md` doesn't also get a copy in its own rules folder and read the rule twice.

Every other selected client reads the rule from `AGENTS.md`:
- Claude Code and Gemini CLI through an `@AGENTS.md` import (a line that pulls in the text of `AGENTS.md`) that Grimoire adds to `CLAUDE.md` or `GEMINI.md`;
- Cursor and Copilot directly, because they read `AGENTS.md`.

Otherwise each client gets its own always-on copy:

| Client | Output |
|---|---|
| `claude-code` | `.claude/rules/plain-language-docs.md`, with no `paths` frontmatter (the metadata block at the top of the file that would limit the rule to matching files), so Claude Code loads it at the start of every session |
| `cursor` | `.cursor/rules/plain-language-docs.mdc` with `alwaysApply: true` |
| `gemini-cli` | A managed block in `GEMINI.md` |
| `copilot` | `.github/instructions/plain-language-docs.instructions.md` with `applyTo: "**"` |

The rule is always on rather than limited to certain paths, because documentation isn't only in Markdown files: doc comments live in source files, and help text in code.

`grimoire plan`, which shows what `grimoire apply` would change, reports any client that may still read the rule twice. One example is Copilot CLI, which reads both `AGENTS.md` and the import in `CLAUDE.md`.

## Security notes

- The rule is plain text loaded into the agent's context (the text the model sees) in every session. It grants no tools and runs nothing.
- It can only change how the agent words documentation. It doesn't make the agent edit files it wasn't asked to edit.

## Permissions

None. Rules declare no permissions.
