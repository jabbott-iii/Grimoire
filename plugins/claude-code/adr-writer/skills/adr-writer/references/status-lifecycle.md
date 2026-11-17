# ADR statuses and their lifecycle

Read this before you change the status of an ADR, supersede or deprecate one, or decide whether a new decision affects an older ADR.

If the project already uses its own status values or wording (for example lower-case values in YAML front matter), keep using them, and map the steps below onto them.

## Statuses

| Status | Meaning | Can the ADR's content change? |
|---|---|---|
| Proposed | Written and under discussion; not yet agreed | Yes. Revise it freely while it is discussed |
| Accepted | Agreed and in force | No. Only its status line may change later |
| Rejected | Considered and decided against | No. Keep the file as a record of what was turned down |
| Deprecated | No longer applies, and nothing replaces it | No, apart from the status line |
| Superseded by ADR-NNNN | Replaced by the newer ADR named in the status | No, apart from the status line |

## Allowed transitions

| From | To | When |
|---|---|---|
| Proposed | Accepted | The people who own the decision agree to it |
| Proposed | Rejected | They decide against it. If it proposed to supersede an older ADR, the older ADR stays as it is |
| Accepted | Superseded by ADR-NNNN | A newer ADR that replaces the decision has been accepted |
| Accepted | Deprecated | The decision stops applying and no new decision replaces it, for example because the component it governs was removed |

Rejected, Deprecated and Superseded are final. Any other change, such as reopening a rejected decision, is made by writing a new ADR. Because Superseded is final, mark an ADR superseded only by an Accepted ADR, never by one that is still Proposed.

Change a status only when the user, or the project's documented process, has made that decision. Never accept, reject or deprecate an ADR on your own initiative.

## Status line wording

With the bundled template, the status is the first item under the title. Write it exactly like one of these:

```markdown
- **Status:** Proposed
- **Status:** Accepted
- **Status:** Accepted (2026-11-20)
- **Status:** Rejected
- **Status:** Deprecated (the export service was removed)
- **Status:** Superseded by [ADR-0009](0009-use-sqlite-for-the-job-queue.md)
```

- Add the date of a change in parentheses only if the project already records dates that way.
- Leave the ADR's Date line alone. It records when the ADR was written.
- A deprecation should say why, in a few words.

## Numbers

- Every ADR keeps its number for ever, whatever its status.
- Numbers are never reused, even for a rejected ADR or one whose file was deleted.
- Never renumber files to close gaps.

## Superseding an ADR, step by step

Only an Accepted ADR supersedes another. A proposal replaces nothing until it is accepted, and marking the old ADR early would leave the log naming a proposal as the decision in force, with no way back because Superseded is final.

1. Settle the new ADR's status.
   - It is Accepted if the user says the replacement is agreed, or plainly tells you to supersede the old ADR: that instruction is the decision.
   - It is Proposed if the user is only exploring or proposing a replacement.
   - If you cannot tell which, ask.
2. Write the new ADR with the next free number, or update it if it already exists as a proposal. Explain in its Context what changed since the old decision was made.
3. In the new ADR's Related line, link the old ADR:
   - `Supersedes [ADR-0004](0004-use-redis-for-the-job-queue.md)` when the new ADR is Accepted;
   - `Proposes to supersede [ADR-0004](0004-use-redis-for-the-job-queue.md)` while it is Proposed.
4. If the new ADR is Accepted, change only the old ADR's status line to `Superseded by ADR-NNNN`, linked to the new file. Do not touch any other line: not the context, the decision, the consequences or the related links. If the new ADR is Proposed, leave the old ADR unchanged, and tell the user it will be marked superseded once the replacement is accepted.
5. If the project keeps an index of ADRs, update the entries that changed.

### When the proposal is decided

- If it is accepted, change its status to Accepted and change `Proposes to supersede` to `Supersedes` in its Related line. Then do steps 4 and 5.
- If it is rejected, change its status to Rejected and leave the old ADR as it is. The old decision stays in force.

## When a new decision only partly overlaps an older one

Ask the user whether the older decision still holds as written.

- If it no longer holds, the new ADR supersedes the older one, following the steps above: the older ADR is marked superseded once the new one is accepted.
- If it still holds and the new ADR only adds to it or narrows it, leave the older ADR unchanged and link to it from the new ADR's Related line.

## Following a chain

When an ADR is superseded, follow its "Superseded by" link, and keep following until you reach an ADR that is not superseded.

- If that ADR is Accepted, it is the current decision. The older ADRs explain how the project got there.
- If it is Deprecated, no decision on the topic is in force any more.
- If it is Proposed or Rejected, it is not in force: the ADR before it was marked superseded too early, and holds the last decision that was accepted. Tell the user the log is inconsistent, and do not present the proposal or the rejected ADR as the current decision. Do not change the statuses yourself; let the user decide.

An ADR that says it proposes to supersede another is a pending proposal. Mention it when you describe the decision it would replace, but do not treat it as in force.
