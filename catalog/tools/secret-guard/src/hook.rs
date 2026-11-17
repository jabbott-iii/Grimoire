//! `secret-guard hook`: the Claude Code command-hook protocol (`CC-P3` in
//! `docs/specs.md`).
//!
//! The event arrives as one JSON object on stdin. A clean event exits 0 and
//! prints nothing, because plain stdout on `UserPromptSubmit` would be added to
//! Claude's context. A secret, or an event that cannot be checked, exits 2,
//! which blocks a `PreToolUse` call or a `UserPromptSubmit` prompt. The reason
//! goes to stderr and never contains the matched value: Claude reads a
//! `PreToolUse` hook's stderr as the tool's error.

use std::path::Path;

use serde_json::Value;

use crate::detect::{self, Finding};

/// The largest hook event read from stdin. Larger events are refused (blocked).
pub(crate) const MAX_EVENT_BYTES: usize = 32 * 1024 * 1024;

/// What the hook tells the client.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    /// Let the event through: exit 0, no output.
    Allow,
    /// Block the event: exit 2 with `stderr`, and `stdout` when set.
    Block {
        /// The reason, for Claude (`PreToolUse`) or the user (`UserPromptSubmit`).
        stderr: String,
        /// Hook JSON for the client, if any.
        stdout: Option<String>,
    },
}

/// Tools that only read local state: their input names files or patterns and
/// sends nothing anywhere, so it is not checked.
const LOCAL_READ_TOOLS: &[&str] = &["Read", "Glob", "Grep", "LS", "NotebookRead"];

/// The largest existing file read to see whether a secret is already in it.
const MAX_EXISTING_BYTES: u64 = 16 * 1024 * 1024;

/// One piece of an event that is scanned, with the label used in reports.
struct Field<'a> {
    label: String,
    text: &'a str,
    /// Whether a secret the target file already holds is allowed here: true
    /// for the content a write puts into the file, never for anything else.
    exemptible: bool,
}

impl<'a> Field<'a> {
    fn new(label: impl Into<String>, text: &'a str) -> Self {
        Self {
            label: label.into(),
            text,
            exemptible: false,
        }
    }

    fn content(label: &str, text: &'a str) -> Self {
        Self {
            exemptible: true,
            ..Self::new(label, text)
        }
    }
}

fn str_at<'a>(input: &'a Value, key: &str) -> Option<&'a str> {
    input.get(key).and_then(Value::as_str)
}

/// Every string in `value`, object keys included, labelled with its JSON Pointer.
fn all_strings<'a>(value: &'a Value, pointer: String, out: &mut Vec<Field<'a>>) {
    match value {
        Value::String(text) => out.push(Field::new(
            if pointer.is_empty() {
                "/".to_owned()
            } else {
                pointer
            },
            text,
        )),
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                all_strings(item, format!("{pointer}/{i}"), out);
            }
        }
        Value::Object(map) => {
            for (key, item) in map {
                let escaped = key.replace('~', "~0").replace('/', "~1");
                let path = format!("{pointer}/{escaped}");
                out.push(Field::new(format!("{path} (its name)"), key));
                all_strings(item, path, out);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}

/// The parts of a tool call's input that carry new content or leave the machine.
///
/// Edits are checked on the text they write, never on the text they replace,
/// so Claude can still remove a secret that is already in a file.
fn tool_fields<'a>(tool: &str, input: &'a Value) -> Vec<Field<'a>> {
    let content = |key: &str| str_at(input, key).map(|text| Field::content(key, text));
    // The file a write targets is part of what lands on disk.
    let target =
        target_key(tool).and_then(|key| str_at(input, key).map(|text| Field::new(key, text)));
    let mut fields: Vec<Field<'a>> = match tool {
        "Write" => content("content").into_iter().collect(),
        "Edit" => content("new_string").into_iter().collect(),
        "NotebookEdit" => content("new_source").into_iter().collect(),
        "Bash" | "PowerShell" => str_at(input, "command")
            .map(|text| Field::new("command", text))
            .into_iter()
            .collect(),
        // Legacy multi-edit: the new text of each edit.
        "MultiEdit" => input
            .get("edits")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, edit)| {
                str_at(edit, "new_string").map(|text| Field {
                    exemptible: true,
                    ..Field::new(format!("edits/{i}/new_string"), text)
                })
            })
            .collect(),
        // Everything else, including MCP tools and web tools, can send its
        // arguments off the machine: check every string.
        _ => {
            let mut out = Vec::new();
            all_strings(input, String::new(), &mut out);
            out
        }
    };
    fields.extend(target);
    fields
}

/// The `tool_input` key naming the file a write tool changes.
fn target_key(tool: &str) -> Option<&'static str> {
    match tool {
        "Write" | "Edit" | "MultiEdit" => Some("file_path"),
        "NotebookEdit" => Some("notebook_path"),
        _ => None,
    }
}

/// `label`, unless it holds a secret itself (an object key or a path), which a
/// report must not repeat.
fn safe(label: &str) -> &str {
    if detect::scan(label.as_bytes()).is_empty() {
        label
    } else {
        "a value whose name or path holds a secret"
    }
}

fn describe(findings: &[(String, Finding)]) -> String {
    findings
        .iter()
        .map(|(label, f)| {
            format!(
                "  - {} ({}) in {} at line {}, column {} [{}]",
                f.description,
                f.rule,
                safe(label),
                f.line,
                f.column,
                f.fingerprint
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn scan_fields(fields: &[Field<'_>], existing: Option<&[u8]>) -> Vec<(String, Finding)> {
    fields
        .iter()
        .flat_map(|field| {
            detect::scan(field.text.as_bytes())
                .into_iter()
                .filter(move |f| {
                    // A secret the file already holds is not newly exposed by
                    // rewriting the file, as when editing `.env`.
                    let value = &field.text.as_bytes()[f.span.clone()];
                    !(field.exemptible && existing.is_some_and(|bytes| contains(bytes, value)))
                })
                .map(|f| (field.label.clone(), f))
        })
        .collect()
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    memchr::memmem::find(haystack, needle).is_some()
}

/// The current content of the file a write targets: a regular file (never a
/// symbolic link), up to [`MAX_EXISTING_BYTES`]. `None` when there is none.
fn existing_file(path: &str) -> Option<Vec<u8>> {
    let path = Path::new(path);
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.is_file() || meta.len() > MAX_EXISTING_BYTES {
        return None;
    }
    std::fs::read(path).ok()
}

fn refuse(reason: &str) -> Outcome {
    Outcome::Block {
        stderr: format!(
            "secret-guard could not check this event, so it was blocked: {reason}. \
             If the hook is misconfigured, fix it or remove it from .claude/settings.json \
             (with Grimoire: `grimoire remove secret-guard && grimoire apply`)."
        ),
        stdout: None,
    }
}

/// The outcome when the event could not even be read: blocked, so the guard
/// fails closed.
pub(crate) fn decide_unreadable(reason: &str) -> Outcome {
    refuse(reason)
}

/// Decides one hook event.
pub(crate) fn decide(event: &[u8]) -> Outcome {
    if event.len() > MAX_EVENT_BYTES {
        return refuse(&format!(
            "the event is larger than {} MiB",
            MAX_EVENT_BYTES / (1024 * 1024)
        ));
    }
    let event: Value = match serde_json::from_slice(event) {
        Ok(v @ Value::Object(_)) => v,
        Ok(_) => return refuse("the event is not a JSON object"),
        Err(e) => return refuse(&format!("the event is not valid JSON ({e})")),
    };
    match str_at(&event, "hook_event_name") {
        Some("PreToolUse") => pre_tool_use(&event),
        Some("UserPromptSubmit") => user_prompt_submit(&event),
        // Nothing to check. Exit 2 would be wrong here: on `Stop` it tells
        // Claude to keep working.
        Some(_) => Outcome::Allow,
        None => refuse("the event has no hook_event_name"),
    }
}

fn pre_tool_use(event: &Value) -> Outcome {
    let Some(tool) = str_at(event, "tool_name") else {
        return refuse("the PreToolUse event has no tool_name");
    };
    if LOCAL_READ_TOOLS.contains(&tool) {
        return Outcome::Allow;
    }
    let input = event.get("tool_input").unwrap_or(&Value::Null);
    let target_path = target_key(tool).and_then(|key| str_at(input, key));
    let existing = target_path.and_then(existing_file);
    let findings = scan_fields(&tool_fields(tool, input), existing.as_deref());
    if findings.is_empty() {
        return Outcome::Allow;
    }
    let target = target_path
        .map(|p| format!(" to {}", safe(p)))
        .unwrap_or_default();
    Outcome::Block {
        stderr: format!(
            "secret-guard blocked this {tool} call{target}: it contains what looks like a secret.\n{}\n\
             Remove the secret value. Read it from an environment variable or a secret store at run time instead, \
             and never print, write or send it. Do not obfuscate, split or encode the value to get past this check.",
            describe(&findings)
        ),
        stdout: None,
    }
}

fn user_prompt_submit(event: &Value) -> Outcome {
    let Some(prompt) = str_at(event, "prompt") else {
        return refuse("the UserPromptSubmit event has no prompt");
    };
    let findings = scan_fields(&[Field::new("your prompt", prompt)], None);
    if findings.is_empty() {
        return Outcome::Allow;
    }
    Outcome::Block {
        stderr: format!(
            "secret-guard blocked your prompt: it contains what looks like a secret.\n{}\n\
             Remove the secret and send the prompt again. It was not sent to the model, \
             but Claude Code keeps the text in its local transcript and prompt history.",
            describe(&findings)
        ),
        // Without this, Claude Code repeats the prompt in its block message.
        stdout: Some(
            r#"{"hookSpecificOutput":{"hookEventName":"UserPromptSubmit","suppressOriginalPrompt":true}}"#
                .to_owned(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::detect::samples::github;

    fn token() -> String {
        github(1)
    }

    fn pre(tool: &str, input: &Value) -> Outcome {
        decide(
            json!({
                "session_id": "s",
                "hook_event_name": "PreToolUse",
                "tool_name": tool,
                "tool_input": input,
                "tool_use_id": "t",
                "an_unknown_field": {"is": "ignored"},
            })
            .to_string()
            .as_bytes(),
        )
    }

    fn blocked(outcome: &Outcome) -> &str {
        match outcome {
            Outcome::Block { stderr, .. } => stderr,
            Outcome::Allow => panic!("expected a block"),
        }
    }

    #[test]
    fn writes_commands_and_edits_with_secrets_are_blocked() {
        for (tool, input) in [
            (
                "Write",
                json!({"file_path": "/p/config.py", "content": format!("KEY = \"{}\"\n", token())}),
            ),
            (
                "Edit",
                json!({"file_path": "/p/a.rs", "old_string": "x", "new_string": token(), "replace_all": false}),
            ),
            (
                "NotebookEdit",
                json!({"notebook_path": "/p/n.ipynb", "new_source": token()}),
            ),
            (
                "Bash",
                json!({"command": format!("curl -H 'Authorization: token {}' https://x", token())}),
            ),
            ("PowerShell", json!({"command": token()})),
            (
                "MultiEdit",
                json!({"edits": [{"old_string": "a", "new_string": token()}]}),
            ),
            (
                "WebFetch",
                json!({"url": format!("https://x.example/?t={}", token()), "prompt": "p"}),
            ),
            (
                "mcp__slack__post_message",
                json!({"channel": "c", "text": {"nested": [token()]}}),
            ),
        ] {
            let outcome = pre(tool, &input);
            let stderr = blocked(&outcome);
            assert!(stderr.contains("github-token"), "{tool}: {stderr}");
            assert!(
                !stderr.contains(&token()),
                "{tool}: the value is never echoed"
            );
        }
    }

    #[test]
    fn reports_name_the_field_and_the_target() {
        let outcome = pre(
            "Write",
            &json!({"file_path": "/p/config.py", "content": format!("a\nKEY = \"{}\"\n", token())}),
        );
        let stderr = blocked(&outcome);
        assert!(stderr.contains("Write call to /p/config.py"), "{stderr}");
        assert!(
            stderr.contains("in content at line 2, column 8"),
            "{stderr}"
        );
        let outcome = pre("mcp__x__y", &json!({"a": {"b/c": [token()]}}));
        assert!(
            blocked(&outcome).contains("in /a/b~1c/0 at"),
            "JSON Pointer labels"
        );
    }

    #[test]
    fn removing_a_secret_and_reading_files_are_allowed() {
        assert_eq!(
            pre(
                "Edit",
                &json!({"file_path": "/p/a.rs", "old_string": token(), "new_string": "env::var(\"TOKEN\")?"})
            ),
            Outcome::Allow
        );
        assert_eq!(pre("Grep", &json!({"pattern": token()})), Outcome::Allow);
        assert_eq!(
            pre("Read", &json!({"file_path": "/p/a.rs"})),
            Outcome::Allow
        );
        assert_eq!(
            pre(
                "Write",
                &json!({"file_path": "/p/a.rs", "content": "fn main() {}"})
            ),
            Outcome::Allow
        );
    }

    #[test]
    fn prompts_with_secrets_are_blocked_without_echoing_them() {
        let outcome = decide(
            json!({"hook_event_name": "UserPromptSubmit", "prompt": format!("use {} please", token())})
                .to_string()
                .as_bytes(),
        );
        let Outcome::Block { stderr, stdout } = outcome else {
            panic!("blocked")
        };
        assert!(stderr.contains("your prompt"), "{stderr}");
        assert!(!stderr.contains(&token()));
        let json: Value = serde_json::from_str(&stdout.unwrap()).unwrap();
        assert_eq!(json["hookSpecificOutput"]["suppressOriginalPrompt"], true);
        let clean = decide(br#"{"hook_event_name":"UserPromptSubmit","prompt":"hello"}"#);
        assert_eq!(clean, Outcome::Allow);
    }

    #[test]
    fn events_that_cannot_be_checked_are_blocked() {
        for event in [
            &b"not json"[..],
            b"[1, 2]",
            br#"{"tool_name": "Write"}"#,
            br#"{"hook_event_name": "PreToolUse"}"#,
            br#"{"hook_event_name": "UserPromptSubmit"}"#,
        ] {
            let outcome = decide(event);
            assert!(
                blocked(&outcome).contains("could not check this event"),
                "{}",
                String::from_utf8_lossy(event)
            );
        }
        let huge = vec![b' '; MAX_EVENT_BYTES + 1];
        assert!(blocked(&decide(&huge)).contains("larger than"));
    }

    #[test]
    fn other_events_pass() {
        for event in [
            format!(r#"{{"hook_event_name": "Stop", "x": "{}"}}"#, token()),
            format!(
                r#"{{"hook_event_name": "PostToolUse", "tool_name": "Write", "tool_input": {{"content": "{}"}}}}"#,
                token()
            ),
        ] {
            assert_eq!(decide(event.as_bytes()), Outcome::Allow, "{event}");
        }
    }

    #[test]
    fn rewriting_a_file_that_already_holds_the_secret_is_allowed() {
        let dir = tempfile::tempdir().unwrap();
        let env = dir.path().join(".env");
        std::fs::write(&env, format!("TOKEN={}\n", token())).unwrap();
        let path = env.to_str().unwrap();
        let rewrite =
            json!({"file_path": path, "content": format!("TOKEN={}\nDEBUG=1\n", token())});
        assert_eq!(pre("Write", &rewrite), Outcome::Allow);
        let edit = json!({"file_path": path, "old_string": "DEBUG=0", "new_string": format!("X={}", token())});
        assert_eq!(pre("Edit", &edit), Outcome::Allow);
        // A different secret is still new.
        let other = github(2);
        let new =
            json!({"file_path": path, "content": format!("TOKEN={}\nOTHER={other}\n", token())});
        assert!(blocked(&pre("Write", &new)).contains("github-token"));
        // The exemption is only for the file being written, not for commands.
        assert!(
            blocked(&pre(
                "Bash",
                &json!({"command": format!("echo {}", token()), "file_path": path})
            ))
            .contains("github-token")
        );
    }

    #[test]
    fn secrets_in_names_and_paths_are_found_and_never_repeated() {
        let key = token();
        let other = github(3);
        let outcome = pre("mcp__kv__set", &json!({ "entries": { key.clone(): "v" } }));
        let stderr = blocked(&outcome);
        assert!(stderr.contains("github-token"), "{stderr}");
        assert!(!stderr.contains(&key), "a key is never echoed: {stderr}");
        let outcome = pre("mcp__kv__set", &json!({ key.clone(): other.clone() }));
        let stderr = blocked(&outcome);
        assert!(
            !stderr.contains(&key) && !stderr.contains(&other),
            "{stderr}"
        );
        let path = format!("/tmp/{key}.txt");
        let outcome = pre("Write", &json!({ "file_path": path, "content": "plain" }));
        let stderr = blocked(&outcome);
        assert!(!stderr.contains(&key), "a path is never echoed: {stderr}");
    }

    #[test]
    fn only_the_tool_s_own_target_unlocks_the_existing_file_exemption() {
        let dir = tempfile::tempdir().unwrap();
        let env = dir.path().join(".env");
        std::fs::write(&env, format!("TOKEN={}\n", token())).unwrap();
        let notebook = dir.path().join("public.ipynb");
        // A NotebookEdit's target is notebook_path; a decoy file_path is ignored.
        let decoy = json!({
            "notebook_path": notebook.to_str().unwrap(),
            "file_path": env.to_str().unwrap(),
            "new_source": token(),
        });
        assert!(blocked(&pre("NotebookEdit", &decoy)).contains("github-token"));
        // A secret in the path is never exempt, even if the file holds it.
        let path = dir.path().join(format!("{}.env", token()));
        std::fs::write(&path, token()).unwrap();
        let rewrite = json!({ "file_path": path.to_str().unwrap(), "content": token() });
        assert!(blocked(&pre("Write", &rewrite)).contains("github-token"));
    }
}
