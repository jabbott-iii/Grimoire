---
name: adr-writer
description: Use this skill when recording an architecture or technical decision as an architecture decision record (ADR), when superseding or deprecating an existing ADR, when someone asks why a past decision was made and the project keeps ADRs, or when keeping an ADR log in order (numbering, statuses, links between records). It finds the ADR directory, picks the next number, fills a template with value-neutral context, a "We will" decision and every consequence, and changes statuses without rewriting accepted decisions.
license: MIT OR Apache-2.0
---

# ADR writer

An architecture decision record (ADR) captures one significant decision: the forces that led to it, what was decided, and what follows from it. A project's ADRs together form its decision log. Use this skill to write a new ADR, to change the status of an existing one, to answer questions from the log, and to keep the log consistent.

## Rules that always apply

- **One decision per ADR.** If the user describes several independent decisions, write one ADR for each.
- **Numbers are permanent.** They are sequential, never reused and never changed, whatever happens to the ADR later.
- **Accepted ADRs are not rewritten.** Never change the context, decision or consequences of an accepted ADR. To change a decision, write a new ADR that supersedes it, and once the new ADR is accepted, change only the old ADR's status line.
- **Follow the project's existing format.** If the project's ADRs use their own headings, status values or file names, match them. Use the bundled template only when the project has no format of its own.
- **Do not invent facts.** Take the context, options and consequences from what the user tells you and from the repository. When something important is unknown, ask. If you cannot ask, write the gap down in the ADR as an open question rather than guessing.
- **New ADRs start as Proposed** unless the user says the decision has already been accepted. A user who plainly tells you to supersede an ADR has made that decision (see section 4).
- **Touch only the ADR log.** Create and edit files only in the ADR directory and its index.
- **Do not commit.** Write and edit the files only, unless the user asks for more.

## 1. Find the ADR directory

1. If the project-docs MCP server is connected and its `list_adrs` tool is available, call it first. It lists the existing ADRs with their numbers and statuses, so you can skip most of the searching below.
2. Otherwise, look for an existing directory, in this order: `docs/adr/`, `doc/adr/`, `docs/decisions/`, `adr/`. Also check `README.md`, `CONTRIBUTING.md` and `AGENTS.md` for a stated location.
3. If several directories hold ADRs, ask the user which one is current.
4. If none exists, ask the user where ADRs should live. If they have no preference, or you cannot ask, create `docs/adr/` and say that you did.

## 2. Pick the number and file name

1. List every file in the ADR directory whose name starts with a number. Count all of them, whatever their status.
2. Take the highest number and add 1. If there are no ADRs yet, start at 1. If an index or another document mentions a higher ADR number than any file has (the file was deleted), continue after that number instead: numbers are never reused.
3. Zero-pad the number to four digits (`0001`, `0012`, `0107`). If the project already pads to a different width, match it.
4. Name the file `NNNN-short-kebab-title.md`: the padded number, a hyphen, then a few lower-case words joined by hyphens, for example `0007-use-postgresql-for-the-job-queue.md`.
5. Never fill a gap in the numbering and never renumber existing files, even if numbers are missing. Other documents and commits refer to ADRs by number.

## 3. Write a new ADR

1. Read [references/writing-guide.md](references/writing-guide.md) now, before you write the Context, Decision and Consequences sections. It explains what each section must and must not contain.
2. Read [assets/adr-template.md](assets/adr-template.md) and copy it to the new file. Use it only if the project has no format of its own (see the rules above).
3. Fill in every part, replacing all guidance text:
   - **Title:** `# ADR-NNNN: ` followed by a short noun phrase that names the decision.
   - **Status:** `Proposed`, unless the user told you otherwise.
   - **Date:** today's date as `YYYY-MM-DD`.
   - **Related:** the related ADRs, each as a relative Markdown link to its file, such as `[ADR-0003](0003-release-tooling.md)`, with a few words on how it relates. Delete the line if there are none.
   - **Context:** the forces and constraints, described neutrally, without arguing for the outcome.
   - **Options considered:** each realistic option with its main pros and cons. Delete the section if only one option was ever on the table.
   - **Decision:** full sentences in the active voice, starting with "We will".
   - **Consequences:** every consequence, positive, negative and neutral, including the costs and the follow-up work.
   - **Confirmation:** how anyone can check that the decision is being followed. Delete the section if there is no practical check.
4. If the project keeps an index of ADRs (for example a `README.md` or `index.md` in the ADR directory), add the new ADR to it in the same style as the existing entries.
5. Re-read the ADR as a developer who joins the project a year from now. They should understand why the decision was made without asking anyone.

## 4. Change a status or supersede an ADR

Read [references/status-lifecycle.md](references/status-lifecycle.md) before you change any status, supersede or deprecate an ADR, or decide whether an older ADR is affected by a new one. It lists the allowed statuses and transitions, with the exact wording.

Only an Accepted ADR supersedes another. A proposal replaces nothing until it is accepted. To supersede an ADR:

1. Settle the new ADR's status. It is Accepted if the user says the replacement is agreed, or plainly tells you to supersede the old ADR. It is Proposed if the user is only exploring or proposing a replacement. If you cannot tell which, ask.
2. Write the new ADR as in section 3, or update it if it already exists as a proposal. In its Related line, link the old ADR: `Supersedes` followed by the link when the new ADR is Accepted, `Proposes to supersede` followed by the link while it is Proposed.
3. If the new ADR is Accepted, change only the old ADR's status line, so that it reads `Superseded by ADR-NNNN` with the new number, linked to the new file. Leave every other line of the old ADR exactly as it was. If the new ADR is Proposed, leave the old ADR untouched, and tell the user it will be marked superseded once the replacement is accepted.
4. If the project keeps an index, update the entries that changed.

When a proposal to supersede is decided later: if it is accepted, set its status to Accepted, change `Proposes to supersede` to `Supersedes`, then do steps 3 and 4. If it is rejected, set its status to Rejected and leave the old ADR as it is; the old decision stays in force.

Change a status only when the user, or the project's documented process, has made that decision. Never accept your own proposal.

## 5. Answer "why did we decide this?"

1. Find the ADRs on the topic: use `list_adrs` when it is available, otherwise search the ADR directory for the relevant terms.
2. Read the matching ADRs in full. If one is superseded, follow the "Superseded by" links to the newest ADR, and read the older ones only for history. Only an Accepted ADR is in force. If the chain ends at a Proposed or Rejected ADR, the log is inconsistent: say so, and do not present that ADR as the current decision. If a Proposed ADR proposes to supersede the one you found, mention it as pending.
3. Answer from the Context and Decision sections. Cite each ADR by number and file path, and say if the decision you describe is no longer current.
4. If no ADR covers the question, say so plainly. Do not reconstruct a rationale from memory or guesswork. Offer to record the decision as a new ADR if the user can supply the reasons.

## 6. Keep the log in order

When asked to review or tidy the ADR log, check for the following and report what you find:

- duplicate numbers, or files that do not follow the project's naming pattern;
- ADRs without a status, or with a status the project does not use;
- "Superseded by" lines that point to a missing ADR or to one that is still Proposed or was Rejected, and superseding ADRs that do not link back to the ADR they replace;
- an index that is missing ADRs or lists ones that no longer exist;
- broken relative links between ADRs.

Fix only what is safe and changes no record of a decision: a missing or stale index entry, or a broken link in an ADR that is still Proposed. Report everything else, including problems inside accepted ADRs, duplicate numbers and disputed statuses, and let the user decide. Never renumber ADRs, and never edit an accepted ADR beyond its status line.

## When you finish

Tell the user the path, number, title and status of each ADR you created, and list every change you made to other ADRs or to the index.
