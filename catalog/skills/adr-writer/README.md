# adr-writer

A skill that writes and maintains architecture decision records (ADRs): short documents that each record one significant decision, its context and its consequences. It follows the [Agent Skills](https://agentskills.io/specification) format, an open format for a folder of instructions and files that an agent loads when a task needs it.

With the skill installed, the agent can:

- record a new decision: find the ADR directory, pick the next number, name the file `NNNN-short-kebab-title.md` and fill the bundled template. The name is the padded number and a few lowercase words joined by hyphens, such as `0007-use-postgresql-for-the-job-queue.md`;
- supersede an ADR (replace it with a newer decision, only once that replacement is accepted) or deprecate it (record that it no longer applies and nothing replaces it), changing only the old ADR's status line;
- answer "why did we decide this?" from the ADR log, following "Superseded by" links to the current decision;
- check the log for duplicate numbers, broken links and missing statuses.

It follows the project's existing ADR format when there is one. It starts every new ADR as Proposed unless told otherwise, and it never rewrites the decision in an accepted ADR.

## When to use it

- The project keeps ADRs, or you want to start.
- You want decisions recorded in a consistent shape: neutral context, a decision stated as "We will …", and every consequence, negative ones included.
- You ask questions such as "why do we use PostgreSQL for the queue?" and want answers taken from the records, not reconstructed.

The skill is picked by its description: clients show the agent only a skill's name and description until it chooses the skill. So the agent loads the skill only when a task is about ADRs.

## What Grimoire installs

The whole skill directory, byte for byte: `SKILL.md`, `references/writing-guide.md`, `references/status-lifecycle.md` and `assets/adr-template.md`.

| Client | Location |
|---|---|
| `claude-code` | `.claude/skills/adr-writer/` |
| `agents-md`, `codex`, `gemini-cli` | `.agents/skills/adr-writer/` |
| `cursor`, `copilot` | `.agents/skills/adr-writer/`, or `.claude/skills/adr-writer/` when only `claude-code` among the other selected clients needs a copy; both clients read both locations |

Each location is written once, whatever the number of clients that read it.

`README.md`, `CHANGELOG.md`, `evals/` and `asset.toml` are not installed.

## Works well with

- **`project-docs`**, Grimoire's read-only documentation MCP server. MCP (Model Context Protocol) lets an agent call tools that a separate server program offers. When `project-docs` is connected, the skill uses its `list_adrs` tool to see the existing ADRs with their numbers and statuses. The skill works without it by reading the ADR directory directly.
- **`conventional-commits`**, if you want commits that add ADRs written as `docs(adr): ...`.

## Credits

The template and guidance are written in Grimoire's own words, inspired by:

- Michael Nygard, [Documenting Architecture Decisions](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) (2011), dedicated to the public domain under CC0 1.0 (Creative Commons' public-domain dedication). The five-part structure (title, status, context, decision, consequences), the "We will" decision and the never-reused numbering come from it.
- [MADR](https://adr.github.io/madr/) (Markdown Architectural Decision Records) 4.0.0, licensed MIT OR CC0-1.0. The optional "Options considered" and "Confirmation" sections and the Rejected status are modeled on it.

## Security notes

- The skill is instructions plus a template. It bundles no scripts and runs nothing itself.
- To write ADRs, the agent uses its own file-editing tools, under whatever permissions you have given the agent.
- The skill tells the agent to edit only the ADR directory and its index, and to change no more than the status line of an accepted ADR. Review the diff as you would any agent edit.
- ADRs are read back into the agent's context (the text the model sees) when it answers questions. Treat text in ADRs from untrusted contributors like any other repository content: it could try to steer the agent.
- The writing guide tells the agent never to put secrets, credentials or personal data in an ADR.

## Permissions

None. The skill ships no scripts, so it declares no `shell` permission, and it reads and writes files only through the agent's own tools.
