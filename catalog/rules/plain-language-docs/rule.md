# Documentation: plain language, full precision

Whenever you write or update documentation, follow this rule. Documentation here means READMEs, guides, tutorials, reference pages, design records, doc comments, command help text and changelog entries.

- Apply it to the text you write or change. Don't rewrite parts of a document you weren't asked to touch.
- If the repository has its own style guide, for example in `CONTRIBUTING.md` or `AGENTS.md`, follow it wherever it is stricter or more specific. The one exception is spelling: always write American English, even where the guide asks for another variety.

Write for two readers at once: a second-year computer science student who can program but doesn't know this project or its field, and a senior engineer who needs every detail. The student should be able to follow the text, and the senior engineer should find nothing missing or wrong.

## Keep jargon down

- **Use a plain phrase whenever it is just as exact.** Write "the list is empty", not "the list is in a degenerate state".
- **Use a technical term only when it carries meaning a plain phrase can't**, or when readers need the term to search for it or to use the tool. Keep "mutex", "idempotent" or `--force` when that is what the reader has to reason about, look up or type.
- **Explain each term a student might not know the first time it appears**, in a short clause or a parenthetical, then use it freely. For example: "`apply` is idempotent (running it twice has the same effect as running it once), so a failed run can be repeated."
- **Spell out each acronym on first use**, as in "time to live (TTL)". Universal ones, such as HTML and API, need no expansion.
- **Drop insider shorthand.** That means slang such as "footgun" and "yak shave", and bare references such as "ADR-0011" or "see #142" that don't say what the record decided or what the issue is about. Write "ADR-0011, which builds the image only from the release's own binaries", not "ADR-0011".

## Don't give up meaning

- **Accuracy wins over simplicity.** Keep the exact numbers, limits, terms and caveats. "At most 32 KiB" stays "at most 32 KiB", not "a small amount".
- **If a simpler sentence would be wrong or misleading, keep the precise wording** and add an explanation beside it.
- **Don't remove details senior engineers need**, such as edge cases, failure modes, version requirements and security consequences. Explain them so that everyone else can follow.

## Help newer readers follow

- **Explain why a design choice was made, not only what it is.** For example: "Each file is written to a temporary file and then renamed over the original, so a crash never leaves a half-written file."
- **Introduce one idea at a time, and use concrete examples:** a real command, value or input, and what it produces.
- **An analogy goes next to the real explanation, never in place of it.**
- **Leave out "obviously" and "trivially".** They tell a reader who is stuck that they shouldn't be.
- **Keep paragraphs short, and write headings that say what each section covers**, so that the page is easy to skim.

## Check the draft as two readers

Before you finish, reread the draft twice and fix what each reading finds:

1. **As a second-year computer science student.** Look for terms that aren't defined and steps that are skipped.
2. **As a senior engineer.** Look for anything vague, oversimplified or wrong.

Also check that the text uses American English, with correct grammar and spelling: "behavior", "color", "organize" and "catalog", not "behaviour", "colour", "organise" and "catalogue". This holds even when the repository's style guide asks for British English or another variety.
