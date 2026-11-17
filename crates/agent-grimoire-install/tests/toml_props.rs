//! Property tests: owned TOML entries never disturb the rest of the document.
//!
//! For arbitrary user documents (top-level keys, tables, servers of their own as
//! tables, dotted keys or an inline table, comment and blank lines anywhere, LF
//! or CRLF line endings), adding an owned member keeps every other item as it
//! was, a second plan finds nothing to do, a new version replaces it, and
//! removing the member restores the user's bytes exactly. Comment lines the user
//! adds next to Grimoire's entry after it is written survive its removal.

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use agent_grimoire_install::apply::next_lock;
use agent_grimoire_install::entry::member_key;
use agent_grimoire_install::plan::{Action, Content, Desired, Owner, plan};
use agent_grimoire_model::{AssetId, ClientId, Lockfile, RelPath};
use proptest::prelude::*;
use serde_json::{Value, json};
use toml_edit::{DocumentMut, InlineTable, Item, Table};

/// A value Grimoire can write to TOML as an entry: a table (adapters write only
/// tables to TOML, and a plain value would add a header), holding anything but
/// `null`.
fn mine() -> impl Strategy<Value = Value> {
    let leaf = prop_oneof![
        any::<bool>().prop_map(Value::Bool),
        any::<i32>().prop_map(|n| json!(n)),
        "[a-z ]{0,8}".prop_map(Value::String),
    ];
    let value = leaf.prop_recursive(3, 16, 4, |inner| {
        prop_oneof![
            prop::collection::vec(inner.clone(), 0..4).prop_map(Value::Array),
            prop::collection::btree_map("[a-z]{1,6}", inner, 0..4)
                .prop_map(|m| Value::Object(m.into_iter().collect())),
        ]
    });
    prop::collection::btree_map("[a-z]{1,6}", value, 0..4)
        .prop_map(|m| Value::Object(m.into_iter().collect()))
}

fn scalar() -> impl Strategy<Value = toml_edit::Value> {
    prop_oneof![
        any::<bool>().prop_map(toml_edit::Value::from),
        any::<i32>().prop_map(|n| toml_edit::Value::from(i64::from(n))),
        "[a-z ]{0,8}".prop_map(|s| toml_edit::Value::from(s.as_str())),
    ]
}

type Pairs = BTreeMap<String, toml_edit::Value>;

fn table(pairs: Pairs, dotted: bool) -> Table {
    let mut table = Table::new();
    table.set_dotted(dotted);
    for (k, v) in pairs {
        table.insert(&k, Item::Value(v));
    }
    table
}

/// The user's own servers, written as `[mcp_servers.<id>]` tables (0), the same
/// under an explicit `[mcp_servers]` header (1), dotted keys (2) or an inline
/// table (3).
fn servers(servers: BTreeMap<String, Pairs>, form: u8) -> Item {
    if form == 3 {
        let mut parent = InlineTable::new();
        for (name, pairs) in servers {
            parent.insert(
                name,
                toml_edit::Value::InlineTable(table(pairs, false).into_inline_table()),
            );
        }
        return Item::Value(toml_edit::Value::InlineTable(parent));
    }
    let mut parent = Table::new();
    parent.set_implicit(form == 0);
    parent.set_dotted(form == 2);
    for (name, pairs) in servers {
        parent.insert(&name, Item::Table(table(pairs, form == 2)));
    }
    Item::Table(parent)
}

/// A user's TOML file, with comment or blank lines inserted before any line.
fn document() -> impl Strategy<Value = String> {
    let pairs = || prop::collection::btree_map("[a-z]{1,6}", scalar(), 0..3);
    (
        pairs(),
        prop::collection::btree_map("[a-z]{1,6}", pairs(), 0..3),
        prop::option::of((
            prop::collection::btree_map("[a-z]{1,6}", pairs(), 0..3),
            0u8..4,
        )),
        prop::collection::vec(0u8..4, 0..24),
        any::<bool>(),
    )
        .prop_map(|(top, tables, theirs, decorations, trailing)| {
            let mut doc = DocumentMut::new();
            for (k, v) in top {
                doc.insert(&k, Item::Value(v));
            }
            for (name, pairs) in tables {
                doc.insert(&name, Item::Table(table(pairs, false)));
            }
            if let Some((theirs, form)) = theirs {
                doc.insert("mcp_servers", servers(theirs, form));
            }
            let mut text = String::new();
            for (n, line) in doc.to_string().lines().enumerate() {
                match decorations.get(n) {
                    Some(1) => text.push_str("# a comment\n"),
                    Some(2) => text.push('\n'),
                    _ => {}
                }
                text.push_str(line);
                text.push('\n');
            }
            if trailing {
                text.push_str("# the end\n");
            }
            text
        })
}

fn path() -> RelPath {
    RelPath::new(".codex/config.toml").unwrap()
}

fn desired(value: &Value) -> Vec<Desired> {
    vec![Desired {
        owner: Owner::Asset(AssetId::new("mine").unwrap()),
        clients: [ClientId::Codex].into(),
        path: path(),
        content: Content::Entry {
            key: member_key(&["mcp_servers", "mine"]).unwrap(),
            value: value.clone(),
        },
    }]
}

/// Plans `want` against `lock` with `text` on disk. Returns the new text, the
/// lock that applying it would write, and the first unit's action.
fn step(
    text: Option<&str>,
    want: Vec<Desired>,
    lock: &Lockfile,
) -> (Option<String>, Lockfile, Action) {
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
    (after, next, file.units[0].action)
}

/// Comment lines added just above Grimoire's `[mcp_servers.mine]` header, where
/// a user might comment out a setting of the table before it. `None` when the
/// entry has no header of its own (a dotted or inline parent) or `lines` is 0.
fn note(written: &str, lines: u8) -> Option<String> {
    let at = written.find("[mcp_servers.mine]").filter(|_| lines > 0)?;
    let mut added = String::new();
    for n in 0..lines {
        writeln!(added, "# user note {n}").unwrap();
    }
    Some(format!("{}{added}{}", &written[..at], &written[at..]))
}

proptest! {
    #[test]
    fn member_round_trip(
        text in document(),
        mine in mine(),
        newer in mine(),
        crlf in any::<bool>(),
        lines in 0u8..3,
    ) {
        let text = if crlf { text.replace('\n', "\r\n") } else { text };
        let (written, lock, action) = step(Some(&text), desired(&mine), &Lockfile::new());
        prop_assert_eq!(action, Action::Create);
        let written = written.unwrap();
        // Every item of the user's stays as it was.
        let before: DocumentMut = text.parse().unwrap();
        let after: DocumentMut = written.parse().unwrap();
        for (k, item) in before.iter() {
            if k == "mcp_servers" {
                let theirs = item.as_table_like().unwrap();
                for (name, server) in theirs.iter() {
                    let (now, was) = (after["mcp_servers"][name].to_string(), server.to_string());
                    // Spacing before an inline table's `}` may move to Grimoire's entry.
                    prop_assert_eq!(now.trim(), was.trim());
                }
            } else {
                prop_assert_eq!(after[k].to_string(), item.to_string());
            }
        }
        // A second plan finds nothing to do: what was written hashes as desired.
        let (again, same, action) = step(Some(&written), desired(&mine), &lock);
        prop_assert_eq!(action, Action::Unchanged);
        prop_assert_eq!(again.as_deref(), Some(written.as_str()));
        prop_assert_eq!(&same, &lock);
        // A new version replaces it in place, and is then found as written.
        let (updated, lock, action) = step(Some(&written), desired(&newer), &lock);
        let updated = updated.unwrap();
        if mine != newer {
            prop_assert_eq!(action, Action::Update);
        }
        let (_, _, action) = step(Some(&updated), desired(&newer), &lock);
        prop_assert_eq!(action, Action::Unchanged);
        if text.contains("\r\n") {
            prop_assert!(!updated.replace("\r\n", "").contains('\n'), "only CRLF");
        }
        // Removing it restores the user's bytes; a blank file Grimoire filled goes away.
        let (removed, empty, _) = step(Some(&updated), Vec::new(), &lock);
        prop_assert!(empty.assets.is_empty());
        if text.trim().is_empty() {
            prop_assert_eq!(removed, None);
        } else {
            prop_assert_eq!(removed.as_deref(), Some(text.as_str()));
        }
        // Notes the user adds above Grimoire's header survive its removal.
        let eol = if crlf { "\r\n" } else { "\n" };
        if let Some(noted) = note(&updated.replace("\r\n", "\n"), lines) {
            let noted = noted.replace('\n', eol);
            let (removed, _, action) = step(Some(&noted), Vec::new(), &lock);
            prop_assert_eq!(action, Action::Delete);
            let removed = removed.unwrap_or_default();
            for n in 0..lines {
                let line = format!("# user note {n}");
                prop_assert!(removed.contains(&line), "{} lost:\n{}", line, removed);
            }
        }
    }
}
