//! Property tests: owned JSON entries never disturb the rest of the document.
//!
//! For arbitrary user documents (pretty or compact), adding then removing an
//! owned member or array element restores the user's values exactly (and the
//! bytes, for multi-line containers), a file
//! Grimoire created goes away again, and content the user added inside a
//! container Grimoire created survives.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::{BTreeMap, BTreeSet};

use agent_grimoire_install::apply::next_lock;
use agent_grimoire_install::entry::{element_key, member_key, value_hash};
use agent_grimoire_install::plan::{Content, Desired, Owner, plan};
use agent_grimoire_model::{AssetId, ClientId, EntryKey, Lockfile, RelPath};
use proptest::prelude::*;
use serde_json::{Value, json};

fn leaf() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<i32>().prop_map(|n| json!(n)),
        "[a-z ]{0,8}".prop_map(Value::String),
    ]
}

fn value() -> impl Strategy<Value = Value> {
    leaf().prop_recursive(3, 16, 4, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..4).prop_map(Value::Array),
            prop::collection::btree_map("[a-z]{1,6}", inner, 0..4)
                .prop_map(|m| Value::Object(m.into_iter().collect())),
        ]
    })
}

/// A user's document: arbitrary members, maybe an existing `mcpServers` object
/// and `hooks.PreToolUse` array of their own (possibly empty).
fn document() -> impl Strategy<Value = (Value, bool)> {
    (
        prop::collection::btree_map("[a-z]{1,6}", value(), 0..4),
        prop::option::of(prop::collection::btree_map("[a-z]{1,6}", value(), 0..3)),
        prop::option::of(prop::collection::vec(value(), 0..3)),
        any::<bool>(),
    )
        .prop_map(|(mut top, servers, hooks, pretty)| {
            if let Some(servers) = servers {
                top.insert(
                    "mcpServers".into(),
                    Value::Object(servers.into_iter().collect()),
                );
            }
            if let Some(hooks) = hooks {
                top.insert("hooks".into(), json!({ "PreToolUse": hooks }));
            }
            (Value::Object(top.into_iter().collect()), pretty)
        })
}

fn render(value: &Value, pretty: bool) -> String {
    if pretty {
        serde_json::to_string_pretty(value).unwrap() + "\n"
    } else {
        serde_json::to_string(value).unwrap()
    }
}

fn path() -> RelPath {
    RelPath::new(".mcp.json").unwrap()
}

fn desired(key: EntryKey, value: &Value) -> Vec<Desired> {
    vec![Desired {
        owner: Owner::Asset(AssetId::new("mine").unwrap()),
        clients: [ClientId::ClaudeCode].into(),
        path: path(),
        content: Content::Entry {
            key,
            value: value.clone(),
        },
    }]
}

/// Plans `want` against `lock` with `text` on disk. Returns the new text and the
/// lock that applying it would write, as the CLI does.
fn step(text: Option<&str>, want: Vec<Desired>, lock: &Lockfile) -> (Option<String>, Lockfile) {
    let current = BTreeMap::from([(path(), text.map(|t| t.as_bytes().to_vec()))]);
    let planned = plan(want, lock, &current).unwrap();
    let file = &planned.files[0];
    assert!(file.unreadable.is_none(), "{:?}", file.unreadable);
    assert!(!planned.has_conflicts(), "{planned:?}");
    let meta = if file.units.iter().any(|u| u.hash.is_some()) {
        let template = format!(
            "format = 1\n[[asset]]\nid = \"mine\"\nkind = \"mcp-server\"\nversion = \"1.0.0\"\nsource = \"builtin\"\nmanifest = \"sha256:{}\"\nselected-by = [\"direct\"]\n",
            "0".repeat(64)
        );
        Lockfile::from_toml(&template).unwrap().assets
    } else {
        Vec::new()
    };
    let next = next_lock(lock, meta, &planned, &BTreeSet::new());
    let after = file
        .after
        .as_ref()
        .map(|b| String::from_utf8(b.clone()).unwrap());
    (after, next)
}

proptest! {
    #[test]
    fn member_round_trip((doc, pretty) in document(), mine in value()) {
        prop_assume!(doc.get("mcpServers").is_none_or(|s| s.get("mine").is_none()));
        let text = render(&doc, pretty);
        let want = desired(member_key(&["mcpServers", "mine"]).unwrap(), &mine);
        let (written, lock) = step(Some(&text), want.clone(), &Lockfile::new());
        let written = written.unwrap();
        let parsed: Value = serde_json::from_str(&written).unwrap();
        prop_assert_eq!(&parsed["mcpServers"]["mine"], &mine);
        for (k, v) in doc.as_object().unwrap() {
            if k != "mcpServers" {
                prop_assert_eq!(&parsed[k], v);
            }
        }
        // A second plan finds nothing to do.
        let (again, same) = step(Some(&written), want, &lock);
        prop_assert_eq!(again.as_deref(), Some(written.as_str()));
        prop_assert_eq!(&same, &lock);
        // Removing it restores the user's document exactly.
        let (removed, empty) = step(Some(&written), Vec::new(), &lock);
        prop_assert!(empty.assets.is_empty());
        let restored: Value = serde_json::from_str(removed.as_deref().unwrap()).unwrap();
        prop_assert_eq!(&restored, &doc);
        if pretty && doc.get("mcpServers").is_some_and(|s| s.as_object().is_some_and(|o| !o.is_empty())) {
            prop_assert_eq!(removed.as_deref(), Some(text.as_str()));
        }
    }

    #[test]
    fn element_round_trip((doc, pretty) in document(), mine in value()) {
        let theirs = doc.pointer("/hooks/PreToolUse").and_then(Value::as_array).cloned().unwrap_or_default();
        prop_assume!(!theirs.iter().any(|v| value_hash(v) == value_hash(&mine)));
        let text = render(&doc, pretty);
        let want = desired(element_key(&["hooks", "PreToolUse"], "mine#0").unwrap(), &mine);
        let (written, lock) = step(Some(&text), want, &Lockfile::new());
        let written = written.unwrap();
        let parsed: Value = serde_json::from_str(&written).unwrap();
        let array = parsed.pointer("/hooks/PreToolUse").and_then(Value::as_array).unwrap();
        prop_assert_eq!(array.len(), theirs.len() + 1);
        prop_assert_eq!(&array[..theirs.len()], &theirs[..]);
        let (removed, _) = step(Some(&written), Vec::new(), &lock);
        let restored: Value = serde_json::from_str(removed.as_deref().unwrap()).unwrap();
        prop_assert_eq!(&restored, &doc);
    }

    #[test]
    fn a_file_grimoire_created_goes_away(mine in value()) {
        let want = desired(member_key(&["mcpServers", "mine"]).unwrap(), &mine);
        let (written, lock) = step(None, want, &Lockfile::new());
        prop_assert!(written.is_some());
        let (removed, _) = step(written.as_deref(), Vec::new(), &lock);
        prop_assert_eq!(removed, None);
    }

    #[test]
    fn user_content_added_later_survives_removal(mine in value(), theirs in value()) {
        let want = desired(member_key(&["mcpServers", "mine"]).unwrap(), &mine);
        let (written, lock) = step(None, want, &Lockfile::new());
        // The user adds their own server inside the container Grimoire created.
        let mut doc: Value = serde_json::from_str(&written.unwrap()).unwrap();
        doc["mcpServers"]["theirs"] = theirs.clone();
        let text = serde_json::to_string_pretty(&doc).unwrap();
        let (removed, _) = step(Some(&text), Vec::new(), &lock);
        let restored: Value = serde_json::from_str(removed.as_deref().unwrap()).unwrap();
        prop_assert_eq!(restored, json!({"mcpServers": {"theirs": theirs}}));
    }
}
