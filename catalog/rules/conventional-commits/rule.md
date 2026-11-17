# Commit messages: Conventional Commits

Whenever you write a commit message, or a pull request title that will become one, use the [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/) format described here. This rule is about how a message is worded, not about when to commit: follow your other instructions on that.

If the repository documents its own commit conventions, for example in `CONTRIBUTING.md`, `AGENTS.md` or a commitlint configuration, follow them wherever they are stricter or more specific: their types, scopes, line lengths, sign-off or issue-reference footers.

## Header

The first line is the header:

```text
type(scope)!: description
```

- **type** (required) is a noun naming the kind of change.
  - `feat` means the commit adds a new feature.
  - `fix` means the commit fixes a bug.
- **(scope)** (optional) is a noun in parentheses naming the part of the codebase the change touches, such as `(parser)` or `(api)`.
- **!** (optional) marks a breaking change. It goes straight before the colon.
- A colon and a single space come next, then the **description**: a short summary written in the imperative ("add", not "added" or "adds"). Keep it to one line.

Only `feat` and `fix` have a meaning in the specification. Any other type is a project convention. When the repository has no list of its own, use these:

| Type | Use it for |
|---|---|
| `build` | the build system, packaging or dependencies |
| `chore` | maintenance that fits no other type |
| `ci` | CI configuration and scripts |
| `docs` | documentation only |
| `perf` | a change that makes the code faster or leaner |
| `refactor` | restructuring that neither fixes a bug nor adds a feature |
| `revert` | undoing earlier commits |
| `style` | formatting only, with no change in meaning |
| `test` | adding or correcting tests |

Pick one type per commit. If a change needs two, it is usually two commits.

## Body and footers

- Leave one blank line after the header, then write the optional body: free-form paragraphs that explain what changed and why.
- Leave one blank line before the optional footers. Each footer is a git-trailer-style line: a token, then `: ` (or ` #` before an issue number), then a value, as in `Refs: #142` or `Reviewed-by: Sam Lee`. Tokens use `-` in place of spaces; the one exception is `BREAKING CHANGE`.

## Breaking changes

A commit of any type is breaking when it makes an incompatible change. Mark it with `!` before the colon, with a `BREAKING CHANGE: ` footer that says what breaks and how to migrate, or with both. Write `BREAKING CHANGE` in upper case; `BREAKING-CHANGE` means the same. If you use only `!`, make the description say what breaks.

## Releases

With semantic versioning, `fix` leads to a PATCH release, `feat` to a MINOR release, and any breaking change to a MAJOR release, whatever its type. Other types have no effect on the version unless they are breaking.

## Reverts

The specification leaves reverts open. Use the `revert` type, describe what is being undone, and list the reverted commits' SHAs in a `Refs:` footer, for example `Refs: 3f9c2e1, 8b04d7a`.

## Examples

```text
feat(cli): add a --json flag to the list command
```

```text
fix(parser): reject an empty scope in the header

An empty pair of parentheses was accepted as a scope and produced an
empty section in the changelog.

Refs: #142
```

```text
feat(api)!: remove the v1 endpoints

BREAKING CHANGE: requests to /v1/ now return 404. Move clients to the
equivalent /v2/ endpoints before upgrading.
```
