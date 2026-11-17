# adr-writer

An [Agent Skills](https://agentskills.io/specification) skill that writes and maintains architecture decision records (ADRs): short documents that each record one significant decision, its context and its consequences.

With the skill installed, the agent can:

- record a new decision: find the ADR directory, pick the next number, name the file `NNNN-short-kebab-title.md` and fill the bundled template;
- supersede or deprecate an ADR, changing only the old ADR's status line, and only once the replacement is accepted;
- answer "why did we decide this?" from the ADR log, following "Superseded by" links to the current decision;
- check the log for duplicate numbers, broken links and missing statuses.

It follows the project's existing ADR format when there is one, starts every new ADR as Proposed unless told otherwise, and never rewrites the decision in an accepted ADR.

## When to use it

- The project keeps ADRs, or you want to start.
- You want decisions recorded in a consistent shape: neutral context, a "We will" decision, and every consequence, negative ones included.
- You ask questions such as "why do we use PostgreSQL for the queue?" and want answers taken from the records, not reconstructed.

The skill is picked by its description, so the agent loads it only when a task is about ADRs.

## What Grimoire installs

The whole skill directory, byte for byte: `SKILL.md`, `references/writing-guide.md`, `references/status-lifecycle.md` and `assets/adr-template.md`.

| Client | Location |
|---|---|
| `claude-code` | `.claude/skills/adr-writer/` |
| `agents-md` | `.agents/skills/adr-writer/` |

`README.md`, `CHANGELOG.md`, `evals/` and `asset.toml` are not installed.

## Works well with

- **`project-docs`**, Grimoire's read-only documentation MCP server. When it is connected, the skill uses its `list_adrs` tool to see the existing ADRs with their numbers and statuses. The skill works without it by reading the ADR directory directly.
- **`conventional-commits`**, if you want commits that add ADRs written as `docs(adr): ...`.

## Credits

The template and guidance are written in Grimoire's own words, inspired by:

- Michael Nygard, [Documenting Architecture Decisions](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) (2011), dedicated to the public domain under CC0 1.0. The five-part structure (title, status, context, decision, consequences), the "We will" decision and the never-reused numbering come from it.
- [MADR](https://adr.github.io/madr/) (Markdown Architectural Decision Records) 4.0.0, licensed MIT OR CC0-1.0. The optional "Options considered" and "Confirmation" sections and the Rejected status are modelled on it.

## Security notes

- The skill is instructions plus a template. It bundles no scripts and runs nothing itself.
- To write ADRs the agent uses its own file-editing tools, under whatever permissions you have given the agent. The skill tells it to edit only the ADR directory and its index, and to change no more than the status line of an accepted ADR. Review the diff as you would any agent edit.
- ADRs are read back into the agent's context when it answers questions. Treat text in ADRs from untrusted contributors like any other repository content: it could try to steer the agent.
- The writing guide tells the agent never to put secrets, credentials or personal data in an ADR.

## Permissions

None. The skill ships no scripts, so it declares no `shell` permission, and it reads and writes files only through the agent's own tools.
