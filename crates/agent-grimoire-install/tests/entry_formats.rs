//! Entries in TOML (`.codex/config.toml`) and JSONC (`.gemini/settings.json`)
//! files, through the planner, `apply` and `verify`: the format comes from the
//! path, the user's text survives, and a file Grimoire created goes away again.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::apply::{Options, apply, next_lock, verify};
use agent_grimoire_install::entry::{element_key, member_key};
use agent_grimoire_install::fs::Root;
use agent_grimoire_install::plan::{Action, Content, Desired, Owner, Plan, plan};
use agent_grimoire_model::{AssetId, ClientId, EntryKey, Lockfile, RelPath};
use serde_json::{Value, json};

const CODEX: &str = ".codex/config.toml";
const GEMINI: &str = ".gemini/settings.json";

/// A user's Codex config with comments and a server of their own.
const CODEX_USER: &str = "# Codex project config\nmodel = \"o3\" # pinned\n\n# Servers I added myself\n[mcp_servers.theirs]\ncommand = \"theirs\" # their comment\n\n[profiles.fast]\nmodel = \"o4-mini\"\n";

/// A user's Gemini settings with comments before, inside and after `mcpServers`.
const GEMINI_USER: &str = "// Project settings\n{\n  /* servers */\n  \"mcpServers\": {\n    // theirs comes first\n    \"theirs\": {\"command\": \"x\"} // keep me\n  },\n  \"theme\": \"dark\" /* trailing */\n}\n// the end\n";

fn rel(s: &str) -> RelPath {
    RelPath::new(s).unwrap()
}

fn desired(path: &str, key: EntryKey, value: &Value, client: ClientId) -> Vec<Desired> {
    vec![Desired {
        owner: Owner::Asset(AssetId::new("docs-index").unwrap()),
        clients: [client].into(),
        path: rel(path),
        content: Content::Entry {
            key,
            value: value.clone(),
        },
    }]
}

fn codex(value: &Value) -> Vec<Desired> {
    desired(
        CODEX,
        member_key(&["mcp_servers", "docs-index"]).unwrap(),
        value,
        ClientId::Codex,
    )
}

fn gemini(value: &Value) -> Vec<Desired> {
    desired(
        GEMINI,
        member_key(&["mcpServers", "docs-index"]).unwrap(),
        value,
        ClientId::GeminiCli,
    )
}

fn asset_meta() -> Vec<agent_grimoire_model::LockedAsset> {
    let template = format!(
        "format = 1\n[[asset]]\nid = \"docs-index\"\nkind = \"mcp-server\"\nversion = \"1.0.0\"\nsource = \"builtin\"\nmanifest = \"sha256:{}\"\nselected-by = [\"direct\"]\n",
        "0".repeat(64)
    );
    Lockfile::from_toml(&template).unwrap().assets
}

fn planned(path: &str, text: Option<&str>, want: Vec<Desired>, lock: &Lockfile) -> Plan {
    let current = BTreeMap::from([(rel(path), text.map(|t| t.as_bytes().to_vec()))]);
    plan(want, lock, &current).unwrap()
}

/// Plans `want` against `lock` with `text` on disk. Returns the new text and the
/// lock that applying it would write, as the CLI does.
fn step(
    path: &str,
    text: Option<&str>,
    want: Vec<Desired>,
    lock: &Lockfile,
) -> (Option<String>, Lockfile, Plan) {
    let planned = planned(path, text, want, lock);
    let file = &planned.files[0];
    assert!(file.unreadable.is_none(), "{:?}", file.unreadable);
    assert!(!planned.has_conflicts(), "{planned:?}");
    let meta = if file.units.iter().any(|u| u.hash.is_some()) {
        asset_meta()
    } else {
        Vec::new()
    };
    let next = next_lock(lock, meta, &planned, &BTreeSet::new());
    let after = file
        .after
        .as_ref()
        .map(|b| String::from_utf8(b.clone()).unwrap());
    (after, next, planned)
}

fn server() -> Value {
    json!({"command": "grimoire-mcp-docs-index", "env_vars": ["DOCS_TOKEN"], "startup_timeout_sec": 20})
}

#[test]
fn codex_toml_entry_is_added_kept_and_removed() {
    let (written, lock, first) = step(CODEX, Some(CODEX_USER), codex(&server()), &Lockfile::new());
    assert_eq!(first.files[0].units[0].action, Action::Create);
    let written = written.unwrap();
    let expected = CODEX_USER.replace(
        "\n[profiles.fast]",
        "\n[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-docs-index\"\nenv_vars = [\"DOCS_TOKEN\"]\nstartup_timeout_sec = 20\n\n[profiles.fast]",
    );
    assert_eq!(written, expected);
    // A second plan finds nothing to do: the TOML read back hashes like the JSON value.
    let (again, same, second) = step(CODEX, Some(&written), codex(&server()), &lock);
    assert_eq!(again.as_deref(), Some(written.as_str()));
    assert_eq!(second.files[0].units[0].action, Action::Unchanged);
    assert_eq!(same, lock);
    // A new version updates the table in place.
    let newer =
        json!({"command": "grimoire-mcp-docs-index", "env_vars": ["DOCS_TOKEN", "DOCS_URL"]});
    let (updated, lock, third) = step(CODEX, Some(&written), codex(&newer), &lock);
    assert_eq!(third.files[0].units[0].action, Action::Update);
    let updated = updated.unwrap();
    assert!(
        updated.contains("[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-docs-index\"\nenv_vars = [\"DOCS_TOKEN\", \"DOCS_URL\"]\n\n[profiles.fast]"),
        "{updated}"
    );
    // Removing it restores the user's file exactly. The lock records no value,
    // so the format comes from the path alone.
    let (removed, empty, fourth) = step(CODEX, Some(&updated), Vec::new(), &lock);
    assert_eq!(fourth.files[0].units[0].action, Action::Delete);
    assert_eq!(empty.assets, Vec::new());
    assert_eq!(removed.as_deref(), Some(CODEX_USER));
}

#[test]
fn a_codex_config_grimoire_created_goes_away() {
    let (written, lock, _) = step(CODEX, None, codex(&server()), &Lockfile::new());
    let written = written.unwrap();
    assert_eq!(
        written,
        "[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-docs-index\"\nenv_vars = [\"DOCS_TOKEN\"]\nstartup_timeout_sec = 20\n"
    );
    let record = &lock.assets[0].outputs[0];
    assert!(record.created_file);
    assert_eq!(
        record
            .created_keys
            .iter()
            .map(EntryKey::as_str)
            .collect::<Vec<_>>(),
        ["/mcp_servers"]
    );
    let (removed, _, _) = step(CODEX, Some(&written), Vec::new(), &lock);
    assert_eq!(removed, None);
}

#[test]
fn a_toml_file_refuses_array_elements_and_null() {
    let key = element_key(&["hooks", "PreToolUse"], "docs-index#0").unwrap();
    let elements = planned(
        CODEX,
        None,
        desired(CODEX, key, &json!({"matcher": "Bash"}), ClientId::Codex),
        &Lockfile::new(),
    );
    let reason = elements.files[0].unreadable.as_deref().unwrap();
    assert!(reason.contains("array element"), "{reason}");
    let null = planned(
        CODEX,
        Some(CODEX_USER),
        codex(&json!({"command": null})),
        &Lockfile::new(),
    );
    let reason = null.files[0].unreadable.as_deref().unwrap();
    assert!(reason.contains("is null"), "{reason}");
    assert_eq!(null.files[0].after.as_deref(), Some(CODEX_USER.as_bytes()));
    let invalid = planned(
        CODEX,
        Some("a = 1\na = 2\n"),
        codex(&server()),
        &Lockfile::new(),
    );
    let reason = invalid.files[0].unreadable.as_deref().unwrap();
    assert!(
        reason.starts_with("it is not valid TOML (duplicate key"),
        "{reason}"
    );
}

#[test]
fn gemini_jsonc_entry_keeps_comments() {
    let (written, lock, _) = step(
        GEMINI,
        Some(GEMINI_USER),
        gemini(&server()),
        &Lockfile::new(),
    );
    let written = written.unwrap();
    for comment in [
        "// Project settings\n",
        "/* servers */",
        "// theirs comes first",
        "// keep me",
        "/* trailing */",
        "// the end\n",
    ] {
        assert!(written.contains(comment), "{comment} in {written}");
    }
    assert!(written.contains("\"docs-index\": {"), "{written}");
    let (again, same, second) = step(GEMINI, Some(&written), gemini(&server()), &lock);
    assert_eq!(again.as_deref(), Some(written.as_str()));
    assert_eq!(second.files[0].units[0].action, Action::Unchanged);
    assert_eq!(same, lock);
    let (removed, _, _) = step(GEMINI, Some(&written), Vec::new(), &lock);
    assert_eq!(removed.as_deref(), Some(GEMINI_USER));
}

#[test]
fn only_the_gemini_path_accepts_comments() {
    let want = desired(
        ".mcp.json",
        member_key(&["mcpServers", "docs-index"]).unwrap(),
        &server(),
        ClientId::ClaudeCode,
    );
    let strict = planned(".mcp.json", Some(GEMINI_USER), want, &Lockfile::new());
    let reason = strict.files[0].unreadable.as_deref().unwrap();
    assert!(reason.starts_with("it is not valid JSON ("), "{reason}");
    let trailing = planned(
        GEMINI,
        Some("{\"mcpServers\": {},}\n"),
        gemini(&server()),
        &Lockfile::new(),
    );
    let reason = trailing.files[0].unreadable.as_deref().unwrap();
    assert!(reason.starts_with("it is not valid JSONC ("), "{reason}");
}

#[test]
fn apply_and_verify_use_the_format_of_the_path() {
    let tmp = tempfile::tempdir().unwrap();
    let root = Root::open(tmp.path()).unwrap();
    std::fs::create_dir_all(tmp.path().join(".codex")).unwrap();
    std::fs::create_dir_all(tmp.path().join(".gemini")).unwrap();
    std::fs::write(tmp.path().join(CODEX), CODEX_USER).unwrap();
    std::fs::write(tmp.path().join(GEMINI), GEMINI_USER).unwrap();
    let mut want = codex(&server());
    want.extend(gemini(&server()));
    let current = BTreeMap::from([
        (rel(CODEX), Some(CODEX_USER.as_bytes().to_vec())),
        (rel(GEMINI), Some(GEMINI_USER.as_bytes().to_vec())),
    ]);
    let planned = plan(want, &Lockfile::new(), &current).unwrap();
    assert!(!planned.has_conflicts(), "{planned:?}");
    apply(&root, &planned, Options::default()).unwrap();
    let lock = next_lock(&Lockfile::new(), asset_meta(), &planned, &BTreeSet::new());
    assert_eq!(verify(&root, &lock).unwrap(), Vec::new());
    // An edit to the owned TOML table is drift; an edit elsewhere is not.
    let codex = std::fs::read_to_string(tmp.path().join(CODEX)).unwrap();
    std::fs::write(
        tmp.path().join(CODEX),
        codex.replace("model = \"o3\"", "model = \"o4\""),
    )
    .unwrap();
    assert_eq!(verify(&root, &lock).unwrap(), Vec::new());
    std::fs::write(
        tmp.path().join(CODEX),
        codex.replace("startup_timeout_sec = 20", "startup_timeout_sec = 30"),
    )
    .unwrap();
    let mismatches = verify(&root, &lock).unwrap();
    assert_eq!(mismatches.len(), 1, "{mismatches:?}");
    assert_eq!(mismatches[0].path, rel(CODEX));
    assert_eq!(mismatches[0].reason, "edited since grimoire wrote it");
}

/// Comments are not part of a value's hash, so one the user writes inside
/// Grimoire's entry would be lost by an update or removal: that is a conflict.
#[test]
fn comments_inside_an_entry_are_conflicts() {
    use agent_grimoire_install::plan::Conflict;
    let newer = json!({"command": "grimoire-mcp-docs-index", "env_vars": ["DOCS_TOKEN", "B"]});
    // TOML.
    let (written, lock, _) = step(CODEX, Some(CODEX_USER), codex(&server()), &Lockfile::new());
    let commented = written.unwrap().replace(
        "env_vars = [\"DOCS_TOKEN\"]",
        "env_vars = [\"DOCS_TOKEN\"] # why this one",
    );
    for (want, what) in [
        (codex(&server()), "unchanged"),
        (codex(&newer), "update"),
        (Vec::new(), "delete"),
    ] {
        let p = planned(CODEX, Some(&commented), want, &lock);
        let action = p.files[0].units[0].action;
        if what == "unchanged" {
            assert_eq!(
                action,
                Action::Unchanged,
                "nothing to lose when nothing changes"
            );
        } else {
            assert_eq!(action, Action::Conflict(Conflict::Commented), "{what}");
        }
    }
    // JSONC.
    let (written, lock, _) = step(
        GEMINI,
        Some(GEMINI_USER),
        gemini(&server()),
        &Lockfile::new(),
    );
    let written = written.unwrap();
    let commented = written.replacen(
        "\"command\": \"grimoire-mcp-docs-index\"",
        "// my note\n      \"command\": \"grimoire-mcp-docs-index\"",
        1,
    );
    assert_ne!(commented, written);
    for want in [gemini(&newer), Vec::new()] {
        let p = planned(GEMINI, Some(&commented), want, &lock);
        assert_eq!(
            p.files[0].units[0].action,
            Action::Conflict(Conflict::Commented)
        );
    }
}

/// A `core.autocrlf` checkout of a file Grimoire edited is not a change, and an
/// edit keeps the file's CRLF line endings.
#[test]
fn crlf_codex_configs_are_not_rewritten() {
    let (written, lock, _) = step(CODEX, Some(CODEX_USER), codex(&server()), &Lockfile::new());
    let crlf = written.unwrap().replace('\n', "\r\n");
    let (again, _, p) = step(CODEX, Some(&crlf), codex(&server()), &lock);
    assert_eq!(p.files[0].units[0].action, Action::Unchanged);
    assert!(!p.has_changes(), "a CRLF checkout is not drift");
    assert_eq!(again.as_deref(), Some(crlf.as_str()));
    let (removed, _, _) = step(CODEX, Some(&crlf), Vec::new(), &lock);
    assert_eq!(
        removed.as_deref(),
        Some(CODEX_USER.replace('\n', "\r\n").as_str())
    );
}

/// Whitespace in a comment reaches `JSON.parse` after strip-json-comments, which
/// refuses all but space, tab, CR and LF (`EDIT-P4`).
#[test]
fn jsonc_comments_with_whitespace_json_refuses_are_refused() {
    for bad in [
        '\u{a0}', '\u{c}', '\u{b}', '\u{2028}', '\u{feff}', '\u{3000}',
    ] {
        let text = format!("{{\n  // see{bad}docs\n  \"theme\": \"dark\"\n}}\n");
        let p = planned(GEMINI, Some(&text), gemini(&server()), &Lockfile::new());
        assert!(p.files[0].unreadable.is_some(), "{bad:?} should be refused");
    }
    // U+0085 is whitespace to Rust but not to JavaScript, so it is blanked.
    let text = "{\n  // see\u{85}docs\n  \"theme\": \"dark\"\n}\n";
    let p = planned(GEMINI, Some(text), gemini(&server()), &Lockfile::new());
    assert!(
        p.files[0].unreadable.is_none(),
        "{:?}",
        p.files[0].unreadable
    );
}
