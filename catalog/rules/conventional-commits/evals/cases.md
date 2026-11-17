# Evaluation cases

Manual evaluation prompts for `conventional-commits`. Run each in a scratch repository with the rule installed, and compare the drafted message with the expected behaviour. The agent should draft the message only; none of these prompts asks it to commit.

| # | Setup | Prompt | Expected |
|---|---|---|---|
| 1 | A staged change that adds a `--json` flag to a `list` command in `src/cli.rs` | "Write a commit message for the staged change." | A single header such as `feat(cli): add a --json flag to the list command`: `feat` type, a noun scope, imperative description, no trailing body unless it adds information. |
| 2 | A staged change that fixes an off-by-one error reported in issue 88 | "Draft the commit message. It fixes issue 88." | `fix:` or `fix(scope):` header, a body after one blank line explaining the cause, and a footer such as `Refs: #88` or `Fixes #88` after another blank line. |
| 3 | A staged change that renames a public configuration key | "Write the commit message for this." | Marks the change as breaking with `!` before the colon, a `BREAKING CHANGE:` footer in upper case that says how to migrate, or both. |
| 4 | Commit `3f9c2e1` introduced a regression | "Write a message for a commit that reverts 3f9c2e1." | A `revert:` header describing what is undone and a `Refs: 3f9c2e1` footer. |
| 5 | `CONTRIBUTING.md` says every commit needs a scope from the list `core`, `cli`, `site`; the staged change edits `src/cli.rs` | "Write a commit message for the staged change." | Follows the repository's stricter convention: a header with the `(cli)` scope, never an unlisted scope or a missing one. |
| 6 | A staged change that only reformats whitespace and also fixes a typo in an error message | "Write the commit message." | Uses one type per commit and suggests splitting into `style:` and `fix:` commits, or picks the more significant type and explains the choice; it does not run `git commit`. |
