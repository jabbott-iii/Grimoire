# Writing an ADR

Read this before you write the Context, Decision and Consequences of an ADR.

## Who you are writing for

Write for a developer who joins the project after the decision was made and was not part of the discussion. They need to understand what the situation was, what was chosen and what it costs, so that they can work with the decision or make an informed case for changing it.

- Write in full sentences and short paragraphs, as if you were explaining the decision to that person. Lists are fine for options and consequences, but make each item a complete sentence.
- Keep the whole ADR to one or two pages. Link to longer material (issues, benchmarks, design documents, specifications) instead of copying it in.
- Use plain words. Define any term or abbreviation the reader may not know.
- Never include secrets, credentials, internal hostnames that must stay private, or personal data.

## Title

A short noun phrase that names the decision, such as "Job queue storage" or "PostgreSQL for the job queue". It is the ADR's name in every index and link, so make it specific enough to tell it apart from the other records.

## Context

Describe the situation that forces a decision, and nothing that presupposes the outcome.

- State the problem or need, and why it must be decided now.
- List the forces at play: technical constraints, requirements, deadlines, cost, team skills, existing systems, organisational or legal constraints. Forces often pull against each other; say where they do.
- Be value-neutral. Present facts and constraints, not arguments for the option that was chosen. A reader should be able to imagine a different decision from the same context.
- Say what is out of scope, if that helps the reader.
- Give the source of each important fact: an issue, a measurement, a document, or the person who supplied it.

A good test: if the Context already reads like a sales pitch for one option, it is not neutral yet.

## Options considered

Include this section when more than one realistic option was weighed.

- Describe each option briefly and fairly, with its main advantages and drawbacks.
- Include doing nothing, or keeping the current approach, when that was a real choice.
- Do not invent options that nobody considered just to make the list longer.

## Decision

State what will be done, in full sentences, in the active voice, starting with "We will".

- Be specific enough that someone can tell whether the decision is being followed. "We will store jobs in a PostgreSQL table with row-level locking" can be checked; "We will use a robust queue" cannot.
- Say where the decision applies and where it does not.
- State the decision firmly. If it is provisional, say so, and say what would cause it to be revisited.
- If the reasons for the choice are not obvious from the Context and Options, give them here in a sentence or two.

## Consequences

Describe what happens after the decision is applied: all of it, not only the benefits.

- **Positive:** what becomes easier, cheaper, safer or possible.
- **Negative:** what becomes harder, slower or more expensive, the risks taken on, and the options given up. Every real decision has some. An ADR that lists none has not been thought through.
- **Neutral:** changes that are neither good nor bad but that people need to know about, such as new conventions, migrations or ownership changes.
- **Follow-up work:** tasks the decision creates, and any condition or date at which it should be reviewed.

Consequences affect the team, the users and future decisions, not only the code. Include them all.

## Confirmation

If there is a practical way to check that the decision is followed, describe it: a test, a lint rule, a CI check, a review checklist item or a measurement. Delete the section if there is none.

## Open questions

If something important is still unknown when you write the ADR, say so in the section where it matters, as an explicit open question, and tell the user. Do not fill the gap with a plausible guess. Resolve every open question before the ADR is accepted.

## Before you hand it over

- The title names one decision.
- The Context is neutral and gives the source of its key facts.
- The Decision starts with "We will" and can be checked.
- The Consequences include at least one negative consequence, or explain why there is none.
- Every guidance line from the template has been replaced or deleted.
