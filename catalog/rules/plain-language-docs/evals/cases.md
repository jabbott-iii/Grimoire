# Evaluation cases

Manual evaluation prompts for `plain-language-docs`. Run each in a scratch repository with the rule installed, and compare the agent's text with the expected behavior.

| # | Setup | Prompt | Expected |
|---|---|---|---|
| 1 | `docs/config.md` says "Setting `retries = 0` is a footgun; we yak-shaved the defaults in #142." | "Tidy up this paragraph." | No "footgun" or "yak-shaved". It says what goes wrong with `retries = 0` (one failed request fails the whole run), and what #142 changed, or asks what it changed. It doesn't leave a bare "#142". |
| 2 | A module that caches DNS (Domain Name System) answers for the TTL each record carries, and renders an HTML report | "Write the README section on caching." | "Time to live (TTL)" is spelled out the first time it appears, and then "TTL" is used freely. HTML isn't expanded. |
| 3 | A server that refuses request bodies over 1 MiB with status 413 | "Document the request size limit. Keep it simple for beginners." | Keeps "1 MiB" and "413" exactly, and explains 413 beside it (the HTTP status for "Payload Too Large"). Never "a large request" or "an error". |
| 4 | `docs/adr/0011-container-image.md` decides to build the image only from the release's binaries. `docs/container.md` says "See ADR-0011." | "Update container.md to explain where the image's binaries come from." | Says what ADR-0011 decided, in a clause next to the reference, and why: the image holds the same binaries the release signed. |
| 5 | A config option `lock_timeout` that defaults to 30 seconds | "Document `lock_timeout`." | Gives the exact default (30 seconds) and the reason for it, with a concrete example of a value and its effect. No "obviously" or "trivially". |
| 6 | `docs/guide.md` uses "behaviour", "colour" and "organise" in a paragraph the prompt asks about | "Fix the grammar in the second paragraph of docs/guide.md." | Fixes the grammar and changes the spellings to "behavior", "color" and "organize" in that paragraph only. The rest of the file is unchanged. |
| 7 | `README.md` has a typo in its first line, and later sections full of jargon | "Fix the typo in the first line of README.md." | Fixes the typo only. It doesn't rewrite the later sections, though it may mention them. |
| 8 | A retry loop with exponential backoff: 100 ms, doubling, at most 5 attempts | "Write the doc comment for this function." | Explains exponential backoff on first use (each wait is twice the one before), keeps 100 ms, the doubling and the 5-attempt limit exactly, and says what happens after the last attempt. |
| 9 | `CONTRIBUTING.md` says "Write British English" and "Wrap lines at 80 characters." | "Write a short README for the `parser` module." | Wraps lines at 80 characters, as the repository's guide asks, but spells in American English ("behavior", "color"). The guide takes precedence except on spelling. |
