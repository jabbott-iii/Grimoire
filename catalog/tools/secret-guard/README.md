# secret-guard

`grimoire-tool-secret-guard` finds likely secrets and keeps them out of files, commands, outgoing requests and prompts.

It matches only credential formats with a distinctive prefix or structure, so it rarely flags ordinary text: private keys (with their body and END line), and GitHub, GitLab, AWS, Slack, Stripe (live keys), Anthropic, OpenAI, npm and PyPI tokens. Run `grimoire-tool-secret-guard rules` for the list.

- **No entropy heuristics and no generic `password=` rules.** They produce far more false positives than a blocking hook can afford.
- **Examples and placeholders are skipped:** AWS documentation examples (they end in `EXAMPLE`) and values that start with a run of one repeated character (`ghp_XXXX…`). Only the start counts, so padding a real token does not hide it.
- **Not covered:** Google API keys (Firebase projects commit them legitimately), Stripe test and publishable keys, JWTs and database URLs.
- **No inline allow comment.** A marker such as `gitleaks:allow` would let the model wave its own output through.

## What Grimoire installs

For Claude Code, two hooks in `.claude/settings.json`, both in exec form (no shell):

| Event | What is checked |
|---|---|
| `PreToolUse`, every tool | The content the call writes, runs or sends: `Write` content, `Edit` and `NotebookEdit` new text and the path they write to, `Bash` and `PowerShell` commands, and every string of web and MCP tool arguments, argument names included. Local reads (`Read`, `Grep`, `Glob`) are skipped. The text an `Edit` replaces is never checked, so removing a secret is always allowed. |
| `UserPromptSubmit` | The prompt, before it reaches the model. |

A blocked tool call returns a reason to Claude naming the rule, the field, the line and column, and a short SHA-256 fingerprint. The secret itself is never repeated, not even when it is an argument name or part of a file path. A blocked prompt shows the same reason to you; the prompt is not sent to the model, but Claude Code still keeps it in the local transcript and prompt history.

A secret that the target file already contains is not reported in the content a `Write`, `Edit` or `NotebookEdit` puts into that file, so editing a `.env` file that holds credentials still works. To check, the hook reads the current file (a regular file of at most 16 MiB, never through a symbolic link). Events with any other name always pass; only an event with no `hook_event_name` is blocked as uncheckable.

**The hooks fail closed.** They use Claude Code's `onFailure: "block"`: if the binary is missing from `PATH`, crashes or takes longer than 30 seconds, the tool call or prompt is blocked. Install the binary before you run `grimoire apply`, and remove the hooks (`grimoire remove secret-guard && grimoire apply`) before you uninstall it. `grimoire plan` and `grimoire apply` warn while the binary is not on `PATH` (`plan/hook-binary-missing`), and `grimoire doctor` checks it.

Needs Claude Code 2.1.295 or later (`onFailure`; exec-form `args` arrived in 2.1.139).

**If the hooks block everything** (the binary went missing, or Claude Code was started from an app whose `PATH` does not include it), start Claude Code once with all hooks off, then fix the binary or remove the asset:

```bash
claude --settings '{"disableAllHooks": true}'
```

## Installing the binary

Grimoire never installs binaries. Until release builds exist (Phase 5), build it from this repository:

```bash
cargo install --locked --path catalog/tools/secret-guard
```

## Using it directly

```bash
grimoire-tool-secret-guard scan src/ config/       # files and directories
git diff --cached | grimoire-tool-secret-guard scan  # standard input
grimoire-tool-secret-guard scan --json .           # one JSON document on stdout
```

Directories are walked without following symbolic links or entering `.git`. Binary files, files over 16 MiB and special files are skipped and counted.

| Command | Exit status |
|---|---|
| `scan` | 0 nothing found, 1 secrets found, 2 usage error, 3 an input could not be read |
| `hook` | 0 allow, 2 block (a secret was found, or the event could not be checked) |
| `rules` | 0 |

## Security notes

- **Best effort, not a boundary.** Keep a repository secret scanner (such as gitleaks) in CI as well. Ways around the hooks:
  - Claude Code runs project hooks only after you trust the workspace in interactive sessions; `-p` and SDK runs execute them without asking.
  - `disableAllHooks`, `--settings`, a managed `allowManagedHooksOnly` setting, or a Claude Code mod can turn them off or approve a call they blocked.
  - Files you mention with `@` in a prompt are added without a tool call, and the prompt hook sees only what you typed.
  - A secret can reach a file indirectly, for example through a script that Claude writes and then runs.
  - A tool call blocked for a secret means the model already produced that secret: it is in the conversation and the local transcript.
- **Prompt injection.** The block message tells the model not to obfuscate, split or encode a value to get past the check. A determined adversary can still try; the hook only catches formats it knows.
- **Other clients.** Cursor and GitHub Copilot also run hooks from `.claude/settings.json`; `grimoire plan` warns about this. Their hook input may differ from Claude Code's. An event that is not JSON, or has no event name, is blocked, but one named differently (Copilot uses names such as `preToolUse`) passes unchecked.
- **Limits.** A hook event over 32 MiB is blocked rather than scanned. Existing files over 16 MiB are not read for the `.env` exemption above.
- **Fingerprints** are the first 12 hex digits of the SHA-256 of the matched text. They let you tell findings apart without revealing the value.

## Permissions

- `filesystem = "read"`: `scan` reads the paths you give it, and `hook` reads the current content of the file a write targets.
- `hooks = true`: it registers the two hooks above.
