//! Owned entries in JSON config files such as `.mcp.json` (`architecture.md` §7.2).
//!
//! An entry key is an RFC 6901 JSON Pointer to what Grimoire owns:
//!
//! - `/mcpServers/docs-index` owns the value of that object member;
//! - `/hooks/PreToolUse|secret-guard#0` owns one element of the array at
//!   `/hooks/PreToolUse`. Elements have no names, so the element is found by
//!   the hash of its value; the text after `|` only keeps keys distinct.
//!
//! Files are edited in place with a concrete syntax tree, so the user's key
//! order and unrelated content survive, and so do comments and whitespace outside
//! the container Grimoire edits. Inserting into a container that is written on one
//! line reflows that container across lines (a jsonc-parser behaviour); the values
//! are unchanged, and removing the entry later does not restore the single line. Only strict JSON is
//! accepted: Claude Code reads its settings files as strict JSON (`CC-SET`),
//! and Grimoire refuses to edit what a client could not read.

use std::fmt;

use agent_grimoire_model::{EntryKey, Sha256Digest};
use jsonc_parser::ParseOptions;
use jsonc_parser::cst::{CstArray, CstInputValue, CstNode, CstObject, CstRootNode};
use serde_json::Value;

/// Why an entry file cannot be edited.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryError(String);

impl fmt::Display for EntryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for EntryError {}

fn err(message: impl Into<String>) -> EntryError {
    EntryError(message.into())
}

/// Strict JSON. Every field is listed so that a new parser extension, which
/// would default to permissive, fails the build until it is reviewed.
fn strict() -> ParseOptions {
    ParseOptions {
        allow_comments: false,
        allow_loose_object_property_names: false,
        allow_trailing_commas: false,
        allow_missing_commas: false,
        allow_single_quoted_strings: false,
        allow_hexadecimal_numbers: false,
        allow_unary_plus_numbers: false,
        allow_bare_decimal_point_numbers: false,
        allow_non_finite_numbers: false,
        allow_extended_string_escapes: false,
    }
}

/// What an entry key owns.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Target {
    /// The value of the object member at this path.
    Member(Vec<String>),
    /// One element, found by value, of the array at this path.
    Element(Vec<String>),
}

fn escape(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

/// The key of an object member at `path`.
///
/// # Errors
///
/// Fails if the key would be empty or too long for an [`EntryKey`].
pub fn member_key(path: &[&str]) -> Result<EntryKey, agent_grimoire_model::InvalidValue> {
    EntryKey::new(pointer(path))
}

/// The key of an array element at `path`, kept distinct by `label`.
///
/// # Errors
///
/// Fails if the key would be too long for an [`EntryKey`].
pub fn element_key(
    path: &[&str],
    label: &str,
) -> Result<EntryKey, agent_grimoire_model::InvalidValue> {
    EntryKey::new(format!("{}|{label}", pointer(path)))
}

/// For an array-element key, the pointer of its array.
#[must_use]
pub fn element_array(key: &EntryKey) -> Option<String> {
    match parse_key(key) {
        Ok(Target::Element(path)) => Some(pointer(&path)),
        _ => None,
    }
}

/// The pointers of the containers on the way to what `key` owns, outermost
/// first: the parents of a member, or an element's array and its parents.
pub(crate) fn containers(key: &EntryKey) -> Vec<String> {
    let path = match parse_key(key) {
        Ok(Target::Member(path)) => path[..path.len().saturating_sub(1)].to_vec(),
        Ok(Target::Element(path)) => path,
        Err(_) => return Vec::new(),
    };
    (1..=path.len()).map(|n| pointer(&path[..n])).collect()
}

fn parse_key(key: &EntryKey) -> Result<Target, EntryError> {
    let text = key.as_str();
    let (pointer, element) = match text.split_once('|') {
        Some((pointer, _label)) => (pointer, true),
        None => (text, false),
    };
    let Some(rest) = pointer.strip_prefix('/') else {
        return Err(err(format!("entry key `{text}` is not a JSON Pointer")));
    };
    let path: Vec<String> = rest
        .split('/')
        .map(|s| s.replace("~1", "/").replace("~0", "~"))
        .collect();
    Ok(if element {
        Target::Element(path)
    } else {
        Target::Member(path)
    })
}

/// The hash of a JSON value, independent of formatting and key order.
#[must_use]
pub fn value_hash(value: &Value) -> Sha256Digest {
    Sha256Digest::of(canonical(value).as_bytes())
}

fn canonical(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            let parts: Vec<String> = keys
                .into_iter()
                .map(|k| format!("{}:{}", Value::String(k.clone()), canonical(&map[k])))
                .collect();
            format!("{{{}}}", parts.join(","))
        }
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(canonical).collect();
            format!("[{}]", parts.join(","))
        }
        other => other.to_string(),
    }
}

fn input(value: &Value) -> CstInputValue {
    match value {
        Value::Null => CstInputValue::Null,
        Value::Bool(b) => CstInputValue::Bool(*b),
        Value::Number(n) => CstInputValue::Number(n.to_string()),
        Value::String(s) => CstInputValue::String(s.clone()),
        Value::Array(items) => CstInputValue::Array(items.iter().map(input).collect()),
        Value::Object(map) => {
            CstInputValue::Object(map.iter().map(|(k, v)| (k.clone(), input(v))).collect())
        }
    }
}

fn node_hash(node: &CstNode) -> Option<Sha256Digest> {
    node.to_serde_value().as_ref().map(value_hash)
}

/// A JSON document whose root is an object, edited in place.
pub(crate) struct EntryDoc {
    /// Kept alive for as long as any node is used: the tree holds weak parent links.
    root: CstRootNode,
    /// Whether the file existed with content.
    existed: bool,
}

impl EntryDoc {
    /// Parses the current file content (`None`: the file is absent).
    pub(crate) fn parse(text: Option<&str>) -> Result<Self, EntryError> {
        let existed = text.is_some_and(|t| !t.trim().is_empty());
        let source = if existed {
            text.unwrap_or_default()
        } else {
            "{}\n"
        };
        // jsonc-parser 0.34.0 collects comments as tokens when building a CST and
        // then skips its `allow_comments` check (`scan_handling_comments`), so
        // strictness is checked first with serde_json, an RFC 8259 parser.
        serde_json::from_str::<Value>(source)
            .map_err(|e| err(format!("it is not valid JSON ({e})")))?;
        let root = CstRootNode::parse(source, &strict())
            .map_err(|e| err(format!("it is not valid JSON ({e})")))?;
        match root.value() {
            Some(node) if node.as_object().is_some() => {}
            _ => return Err(err("its top level is not a JSON object")),
        }
        Ok(Self { root, existed })
    }

    fn object(&self) -> Result<CstObject, EntryError> {
        self.root
            .object_value()
            .ok_or_else(|| err("its top level is not a JSON object"))
    }

    /// Follows `path` through objects. `Ok(None)` when a member is absent.
    fn walk(&self, path: &[String]) -> Result<Option<CstNode>, EntryError> {
        let mut current: CstNode = self.object()?.into();
        let mut shown = String::new();
        for segment in path {
            let object = current
                .as_object()
                .ok_or_else(|| err(format!("`{shown}` is not a JSON object")))?;
            shown.push('/');
            shown.push_str(&escape(segment));
            let Some(prop) = unique(&object, segment, &shown)? else {
                return Ok(None);
            };
            current = prop
                .value()
                .ok_or_else(|| err(format!("`{shown}` has no value")))?;
        }
        Ok(Some(current))
    }

    /// The hash of what the key owns on disk: the member's value, or the element
    /// whose hash matches `desired` (preferred) or `locked`.
    pub(crate) fn find(
        &self,
        key: &EntryKey,
        locked: Option<&Sha256Digest>,
        desired: Option<&Sha256Digest>,
    ) -> Result<Option<Sha256Digest>, EntryError> {
        match parse_key(key)? {
            Target::Member(path) => Ok(self.walk(&path)?.as_ref().and_then(node_hash)),
            Target::Element(path) => {
                let Some(array) = self.array(&path)? else {
                    return Ok(None);
                };
                let hashes: Vec<Sha256Digest> =
                    array.elements().iter().filter_map(node_hash).collect();
                Ok([desired, locked]
                    .into_iter()
                    .flatten()
                    .find(|h| hashes.contains(h))
                    .cloned())
            }
        }
    }

    fn array(&self, path: &[String]) -> Result<Option<CstArray>, EntryError> {
        match self.walk(path)? {
            None => Ok(None),
            Some(node) => node
                .as_array()
                .map(Some)
                .ok_or_else(|| err(format!("`{}` is not a JSON array", pointer(path)))),
        }
    }

    /// Creates the objects along `path` and returns the innermost one, recording
    /// the pointer of every object it had to create in `created`.
    fn objects_to(
        &self,
        path: &[String],
        created: &mut Vec<String>,
    ) -> Result<CstObject, EntryError> {
        let mut object = self.object()?;
        let mut shown = String::new();
        for segment in path {
            shown.push('/');
            shown.push_str(&escape(segment));
            object = if let Some(prop) = unique(&object, segment, &shown)? {
                prop.value()
                    .and_then(|v| v.as_object())
                    .ok_or_else(|| err(format!("`{shown}` is not a JSON object")))?
            } else {
                created.push(shown.clone());
                object
                    .append(segment, CstInputValue::Object(Vec::new()))
                    .value()
                    .and_then(|v| v.as_object())
                    .ok_or_else(|| err(format!("cannot create `{shown}`")))?
            };
        }
        Ok(object)
    }

    /// Writes `value` where the key points. For elements, replaces the element
    /// matching `locked`, or appends. Returns the pointers of the containers it
    /// had to create.
    pub(crate) fn set(
        &mut self,
        key: &EntryKey,
        locked: Option<&Sha256Digest>,
        value: &Value,
    ) -> Result<Vec<String>, EntryError> {
        let mut created = Vec::new();
        match parse_key(key)? {
            Target::Member(path) => {
                let (name, parents) = path
                    .split_last()
                    .ok_or_else(|| err("an entry key cannot own the whole file"))?;
                let object = self.objects_to(parents, &mut created)?;
                match unique(&object, name, &pointer(&path))? {
                    Some(prop) => prop.set_value(input(value)),
                    None => {
                        object.append(name, input(value));
                    }
                }
            }
            Target::Element(path) => {
                let (name, parents) = path
                    .split_last()
                    .ok_or_else(|| err("an entry key cannot own the whole file"))?;
                let object = self.objects_to(parents, &mut created)?;
                let array = if let Some(prop) = unique(&object, name, &pointer(&path))? {
                    prop.value()
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| err(format!("`{}` is not a JSON array", pointer(&path))))?
                } else {
                    created.push(pointer(&path));
                    object.array_value_or_set(name)
                };
                let existing = locked.and_then(|l| {
                    array
                        .elements()
                        .into_iter()
                        .find(|e| node_hash(e).as_ref() == Some(l))
                });
                match existing {
                    Some(node) => {
                        replace(&node, input(value));
                    }
                    None => {
                        array.append(input(value));
                    }
                }
            }
        }
        Ok(created)
    }

    /// Removes what the key owns: the member, or the element whose hash is `disk`.
    pub(crate) fn remove(
        &mut self,
        key: &EntryKey,
        disk: Option<&Sha256Digest>,
    ) -> Result<(), EntryError> {
        match parse_key(key)? {
            Target::Member(path) => {
                if let Some((name, parents)) = path.split_last()
                    && let Some(object) = self.walk(parents)?.and_then(|n| n.as_object())
                    && let Some(prop) = unique(&object, name, &pointer(&path))?
                {
                    prop.remove();
                }
            }
            Target::Element(path) => {
                if let Some(array) = self.array(&path)?
                    && let Some(node) = array
                        .elements()
                        .into_iter()
                        .find(|e| node_hash(e).as_ref() == disk)
                {
                    node.remove();
                }
            }
        }
        Ok(())
    }

    /// Removes each container in `pointers` that is now empty, deepest first.
    /// Returns those still holding something, which stay.
    pub(crate) fn prune(&mut self, pointers: &[String]) -> Result<Vec<String>, EntryError> {
        let mut ordered: Vec<&String> = pointers.iter().collect();
        ordered.sort_by_key(|p| std::cmp::Reverse(p.matches('/').count()));
        ordered.dedup();
        let mut kept = Vec::new();
        for pointer in ordered {
            let Ok(Target::Member(path)) =
                parse_key(&EntryKey::new(pointer.as_str()).map_err(|e| err(e.to_string()))?)
            else {
                continue;
            };
            let Some((name, parents)) = path.split_last() else {
                continue;
            };
            let Some(parent) = self.walk(parents)?.and_then(|n| n.as_object()) else {
                continue;
            };
            let Some(prop) = unique(&parent, name, pointer)? else {
                continue;
            };
            let empty = prop.value().is_some_and(|v| {
                v.as_object().is_some_and(|o| o.properties().is_empty())
                    || v.as_array().is_some_and(|a| a.elements().is_empty())
            });
            if empty {
                prop.remove();
            } else {
                kept.push(pointer.clone());
            }
        }
        Ok(kept)
    }

    /// Whether the file existed with content before editing.
    pub(crate) fn existed(&self) -> bool {
        self.existed
    }

    /// Whether the top-level object has no members left.
    pub(crate) fn is_empty(&self) -> bool {
        self.root
            .object_value()
            .is_some_and(|o| o.properties().is_empty())
    }

    /// The document as bytes.
    pub(crate) fn render(&self) -> Vec<u8> {
        let mut text = self.root.to_string();
        if !self.existed && !text.ends_with('\n') {
            text.push('\n');
        }
        text.into_bytes()
    }
}

/// The property `name` of `object`, refusing duplicates (which JSON readers resolve differently).
fn unique(
    object: &CstObject,
    name: &str,
    shown: &str,
) -> Result<Option<jsonc_parser::cst::CstObjectProp>, EntryError> {
    let matches: Vec<_> = object
        .properties()
        .into_iter()
        .filter(|p| p.decoded_name().as_deref() == Some(name))
        .collect();
    if matches.len() > 1 {
        return Err(err(format!("`{shown}` appears more than once")));
    }
    Ok(matches.into_iter().next())
}

fn replace(node: &CstNode, value: CstInputValue) {
    if let Some(object) = node.as_object() {
        object.replace_with(value);
    } else if let Some(array) = node.as_array() {
        array.replace_with(value);
    } else if let Some(s) = node.as_string_lit() {
        s.replace_with(value);
    } else if let Some(n) = node.as_number_lit() {
        n.replace_with(value);
    } else if let Some(b) = node.as_boolean_lit() {
        b.replace_with(value);
    } else if let Some(n) = node.as_null_keyword() {
        n.replace_with(value);
    }
}

fn pointer<S: AsRef<str>>(path: &[S]) -> String {
    let mut out = String::new();
    for segment in path {
        out.push('/');
        out.push_str(&escape(segment.as_ref()));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn key(s: &str) -> EntryKey {
        EntryKey::new(s).unwrap()
    }

    #[test]
    fn keys_escape_and_parse() {
        assert_eq!(member_key(&["a/b", "c~d"]).unwrap().as_str(), "/a~1b/c~0d");
        assert_eq!(
            parse_key(&key("/a~1b/c~0d")).unwrap(),
            Target::Member(vec!["a/b".into(), "c~d".into()])
        );
        assert_eq!(
            parse_key(&element_key(&["hooks", "PreToolUse"], "x#0").unwrap()).unwrap(),
            Target::Element(vec!["hooks".into(), "PreToolUse".into()])
        );
        assert!(parse_key(&key("nope")).is_err());
    }

    #[test]
    fn hashes_ignore_order_and_formatting() {
        assert_eq!(
            value_hash(&json!({"a": 1, "b": [1, {"c": null}]})),
            value_hash(&json!({"b": [1, {"c": null}], "a": 1}))
        );
        assert_ne!(value_hash(&json!({"a": 1})), value_hash(&json!({"a": "1"})));
    }

    #[test]
    fn member_set_and_remove_preserve_the_rest() {
        let text = "{\n  \"keep\": true,\n  \"mcpServers\": {\n    \"theirs\": {\"command\": \"x\"}\n  }\n}\n";
        let mut doc = EntryDoc::parse(Some(text)).unwrap();
        let k = key("/mcpServers/mine");
        assert_eq!(doc.find(&k, None, None).unwrap(), None);
        assert_eq!(
            doc.set(
                &k,
                None,
                &json!({"type": "stdio", "command": "grimoire-mcp-x"})
            )
            .unwrap(),
            Vec::<String>::new(),
            "mcpServers already existed"
        );
        let written = String::from_utf8(doc.render()).unwrap();
        let value: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(value["mcpServers"]["mine"]["command"], "grimoire-mcp-x");
        assert_eq!(value["mcpServers"]["theirs"]["command"], "x");
        assert!(written.contains("\"keep\": true"));
        let mut doc = EntryDoc::parse(Some(&written)).unwrap();
        let hash = doc.find(&k, None, None).unwrap();
        assert!(hash.is_some());
        doc.remove(&k, hash.as_ref()).unwrap();
        assert_eq!(String::from_utf8(doc.render()).unwrap(), text);
    }

    #[test]
    fn creates_and_deletes_a_new_file() {
        let mut doc = EntryDoc::parse(None).unwrap();
        assert!(!doc.existed());
        let k = key("/mcpServers/mine");
        assert_eq!(
            doc.set(&k, None, &json!({"command": "c"})).unwrap(),
            ["/mcpServers"]
        );
        let written = String::from_utf8(doc.render()).unwrap();
        assert!(written.ends_with('\n'));
        let value: Value = serde_json::from_str(&written).unwrap();
        assert_eq!(value, json!({"mcpServers": {"mine": {"command": "c"}}}));
        let mut doc = EntryDoc::parse(Some(&written)).unwrap();
        let hash = doc.find(&k, None, None).unwrap();
        doc.remove(&k, hash.as_ref()).unwrap();
        assert_eq!(
            doc.prune(&["/mcpServers".to_owned()]).unwrap(),
            Vec::<String>::new()
        );
        assert!(doc.is_empty());
    }

    #[test]
    fn only_listed_empty_containers_are_pruned() {
        let mut doc = EntryDoc::parse(Some("{\"a\": {}, \"b\": {\"c\": 1}, \"d\": []}")).unwrap();
        let kept = doc
            .prune(&["/b".to_owned(), "/d".to_owned(), "/missing".to_owned()])
            .unwrap();
        assert_eq!(kept, ["/b"]);
        let value: Value = serde_json::from_str(&doc.root.to_string()).unwrap();
        assert_eq!(
            value,
            json!({"a": {}, "b": {"c": 1}}),
            "the unlisted empty `a` stays"
        );
    }

    #[test]
    fn elements_are_found_by_value() {
        let theirs =
            json!({"matcher": "Bash", "hooks": [{"type": "command", "command": "theirs"}]});
        let text = format!("{{\"hooks\": {{\"PreToolUse\": [{theirs}]}}}}");
        let mut doc = EntryDoc::parse(Some(&text)).unwrap();
        let k = element_key(&["hooks", "PreToolUse"], "mine#0").unwrap();
        let v1 = json!({"matcher": "Edit", "hooks": [{"type": "command", "command": "mine", "args": []}]});
        let v2 = json!({"matcher": "Edit|Write", "hooks": [{"type": "command", "command": "mine", "args": []}]});
        assert_eq!(doc.set(&k, None, &v1).unwrap(), Vec::<String>::new());
        assert_eq!(
            doc.find(&k, None, Some(&value_hash(&v1))).unwrap(),
            Some(value_hash(&v1))
        );
        doc.set(&k, Some(&value_hash(&v1)), &v2).unwrap();
        let value = serde_json::from_str::<Value>(&doc.root.to_string()).unwrap();
        assert_eq!(
            value["hooks"]["PreToolUse"].as_array().unwrap().len(),
            2,
            "replaced, not appended"
        );
        assert_eq!(doc.find(&k, Some(&value_hash(&v1)), None).unwrap(), None);
        doc.remove(&k, Some(&value_hash(&v2))).unwrap();
        let value = serde_json::from_str::<Value>(&doc.root.to_string()).unwrap();
        assert_eq!(value, json!({"hooks": {"PreToolUse": [theirs]}}));
    }

    #[test]
    fn refuses_what_a_strict_reader_would_refuse() {
        for bad in [
            "{ // comment\n}",
            "{\"a\": 1,}",
            "[1, 2]",
            "{'a': 1}",
            "nope",
        ] {
            assert!(EntryDoc::parse(Some(bad)).is_err(), "{bad}");
        }
        let doc = EntryDoc::parse(Some("{\"mcpServers\": [], \"x\": 1}")).unwrap();
        assert!(doc.find(&key("/mcpServers/a"), None, None).is_err());
        let dup = EntryDoc::parse(Some("{\"mcpServers\": {}, \"mcpServers\": {}}")).unwrap();
        assert!(dup.find(&key("/mcpServers/a"), None, None).is_err());
    }
}
