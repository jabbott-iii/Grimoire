//! Owned entries in config files such as `.mcp.json`, `.gemini/settings.json`
//! and `.codex/config.toml`.
//!
//! The design is in
//! [`docs/architecture.md` §7.2, Ownership units and merge strategies](https://github.com/jabbott-iii/Grimoire/blob/main/docs/architecture.md#72-ownership-units-and-merge-strategies).
//!
//! An entry key is an RFC 6901 JSON Pointer to what Grimoire owns:
//!
//! - `/mcpServers/docs-index` owns the value of that object member;
//! - `/hooks/PreToolUse|secret-guard#0` owns one element of the array at
//!   `/hooks/PreToolUse`. Elements have no names, so the element is found by
//!   the hash of its value; the text after `|` only keeps keys distinct.
//!
//! A value is a JSON value whatever the file's syntax, and the hash of what is on
//! disk is [`value_hash`] of it read back as JSON. A lock hash therefore means the
//! same thing in every format.
//!
//! # Formats
//!
//! [`EntryFormat::for_path`] decides a file's syntax from its path alone. Removing
//! a locked entry that is no longer wanted has no desired value to look at: the
//! lock records only the path, the unit, the key and the hash. The format must
//! still be known then, so it cannot come from the adapter that wrote the entry.
//!
//! - **JSON**, the default (`.mcp.json`, `.claude/settings.json`, …): strict
//!   RFC 8259. Claude Code reads its settings files as strict JSON
//!   ([`docs/specs.md`, CC-SET](https://github.com/jabbott-iii/Grimoire/blob/main/docs/specs.md#cc-set-settings-permissions-hooks)),
//!   and Grimoire refuses to edit what a client could not read.
//! - **JSONC**, exactly `.gemini/settings.json`: Gemini CLI reads it as
//!   `JSON.parse` of the text with its `//` and `/* */` comments stripped. Comments
//!   are accepted and kept. Everything else a strict reader refuses (trailing
//!   commas, single-quoted strings, hexadecimal numbers, …) is still refused.
//! - **TOML**, any `.toml` file (Codex's `.codex/config.toml`): only object
//!   members can be owned, not array elements. See *TOML* below.
//!
//! In every format a comment is the user's content: a file whose last member is
//! gone but that still holds a comment is not empty, so it is never deleted.
//! Values are hashed without their comments, so a comment the user adds *inside*
//! an owned entry would be lost by rewriting or removing it; `EntryDoc::has_comments`
//! lets the planner make that a conflict instead.
//!
//! # JSON and JSONC
//!
//! Files are edited in place with a concrete syntax tree, so the user's key
//! order and unrelated content survive, and so do comments and whitespace outside
//! the container Grimoire edits. Inserting into a container that is written on one
//! line reflows that container across lines (a jsonc-parser behavior); the values
//! are unchanged, and removing the entry later does not restore the single line.
//! Duplicate keys on Grimoire's path are refused, because readers resolve them
//! differently.
//!
//! # TOML
//!
//! Files are edited with `toml_edit`, which keeps unrelated tables, keys,
//! comments and ordering. A new entry whose value is an object is written as a
//! standard table (`[mcp_servers.docs-index]`) with its keys in the value's
//! order; nested objects become inline tables. Containers Grimoire creates on the
//! way are implicit tables, which print no header of their own. Under a parent
//! the user wrote as dotted keys or as an inline table, the entry takes the same
//! form, because a `[table]` header cannot extend an inline table. Replacing an
//! entry keeps its place, its form, its header line and the comments above it;
//! removing it moves the comments above its header to the next one (see the
//! `toml` module). `null` has no
//! TOML form and is refused, as are integers outside the signed 64-bit range. A
//! datetime or a non-finite float on disk has no JSON form, so it never matches
//! a desired value. Invalid TOML, including duplicate keys and tables defined
//! twice, makes the file unreadable for Grimoire.

use std::fmt;

use agent_grimoire_model::{EntryKey, RelPath, Sha256Digest};
use jsonc_parser::cst::{CstArray, CstInputValue, CstNode, CstObject, CstRootNode};
use jsonc_parser::tokens::Token;
use jsonc_parser::{CollectOptions, CommentCollectionStrategy, ParseOptions};
use serde_json::Value;

mod toml;

/// The syntax of an entry file. It follows from the file's path alone (see the
/// module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryFormat {
    /// Strict JSON (RFC 8259).
    Json,
    /// JSON with `//` and `/* */` comments, and nothing else beyond strict JSON.
    Jsonc,
    /// TOML, where only object members can be owned.
    Toml,
}

impl EntryFormat {
    /// The format of the entry file at `path`: TOML for a `.toml` extension, JSONC
    /// for exactly `.gemini/settings.json`, and strict JSON for everything else.
    #[must_use]
    pub fn for_path(path: &RelPath) -> Self {
        let extension = match path.file_name().rsplit_once('.') {
            Some((stem, extension)) if !stem.is_empty() => extension,
            _ => "",
        };
        if extension.eq_ignore_ascii_case("toml") {
            EntryFormat::Toml
        } else if path.as_str() == ".gemini/settings.json" {
            EntryFormat::Jsonc
        } else {
            EntryFormat::Json
        }
    }
}

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

/// Strict JSON plus comments, for JSONC files.
fn with_comments() -> ParseOptions {
    ParseOptions {
        allow_comments: true,
        ..strict()
    }
}

/// Checks `source` as a JSONC reader that strips comments and then parses strict
/// JSON would (Gemini CLI: `JSON.parse` after `strip-json-comments`).
///
/// jsonc-parser finds the comments, which it tells apart from `//` inside
/// strings; they are blanked out and the rest must be RFC 8259 JSON for
/// `serde_json`. jsonc-parser alone would also accept some things `JSON.parse`
/// refuses, such as raw control characters in strings.
fn check_jsonc(source: &str) -> Result<(), EntryError> {
    let invalid = |e: &dyn fmt::Display| err(format!("it is not valid JSONC ({e})"));
    let parsed = jsonc_parser::parse_to_ast(
        source,
        &CollectOptions {
            comments: CommentCollectionStrategy::AsTokens,
            tokens: true,
        },
        &with_comments(),
    )
    .map_err(|e| invalid(&e))?;
    let mut stripped = String::with_capacity(source.len());
    let mut done = 0;
    for token in parsed.tokens.iter().flatten() {
        let range = token.range;
        let line = matches!(token.token, Token::CommentLine(_));
        if !line && !matches!(token.token, Token::CommentBlock(_)) {
            continue;
        }
        // jsonc-parser also ends a line comment at a lone `\r`, strip-json-comments
        // does not: the two would disagree about what follows.
        if line
            && source.as_bytes().get(range.end) == Some(&b'\r')
            && source.as_bytes().get(range.end + 1) != Some(&b'\n')
        {
            return Err(err(
                "it is not valid JSONC (a `//` comment ends in a lone carriage return, which JSONC readers disagree about)",
            ));
        }
        let (Some(before), Some(comment)) = (
            source.get(done..range.start),
            source.get(range.start..range.end),
        ) else {
            return Err(invalid(&"comment boundaries are not character boundaries"));
        };
        stripped.push_str(before);
        // strip-json-comments blanks only non-whitespace (`replace(/\S/g, ' ')`),
        // so whitespace inside a comment reaches `JSON.parse`, which refuses all
        // but space, tab, CR and LF. Keep JavaScript's whitespace as it is, so
        // serde_json refuses the same characters. Rust's set adds U+0085 and
        // lacks U+FEFF, unlike JavaScript's `\s`.
        stripped.extend(comment.chars().map(|c| {
            if (c.is_whitespace() && c != '\u{85}') || c == '\u{feff}' {
                c
            } else {
                ' '
            }
        }));
        done = range.end;
    }
    stripped.push_str(source.get(done..).unwrap_or_default());
    serde_json::from_str::<Value>(&stripped).map_err(|e| invalid(&e))?;
    Ok(())
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

/// An entry file, edited in place: one API over every [`EntryFormat`].
pub(crate) enum EntryDoc {
    /// JSON or JSONC.
    Json(JsonDoc),
    /// TOML.
    Toml(Box<toml::TomlDoc>),
}

impl EntryDoc {
    /// Parses the current file content (`None`: the file is absent) as `format`.
    pub(crate) fn parse(format: EntryFormat, text: Option<&str>) -> Result<Self, EntryError> {
        match format {
            EntryFormat::Json => JsonDoc::parse(text, false).map(Self::Json),
            EntryFormat::Jsonc => JsonDoc::parse(text, true).map(Self::Json),
            EntryFormat::Toml => toml::TomlDoc::parse(text).map(|d| Self::Toml(Box::new(d))),
        }
    }

    /// The hash of what the key owns on disk: the member's value, or the element
    /// whose hash matches `desired` (preferred) or `locked`.
    pub(crate) fn find(
        &self,
        key: &EntryKey,
        locked: Option<&Sha256Digest>,
        desired: Option<&Sha256Digest>,
    ) -> Result<Option<Sha256Digest>, EntryError> {
        match self {
            Self::Json(doc) => doc.find(key, locked, desired),
            Self::Toml(doc) => doc.find(key),
        }
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
        match self {
            Self::Json(doc) => doc.set(key, locked, value),
            Self::Toml(doc) => doc.set(key, value),
        }
    }

    /// Removes what the key owns: the member, or the element whose hash is `disk`.
    pub(crate) fn remove(
        &mut self,
        key: &EntryKey,
        disk: Option<&Sha256Digest>,
    ) -> Result<(), EntryError> {
        match self {
            Self::Json(doc) => doc.remove(key, disk),
            Self::Toml(doc) => doc.remove(key),
        }
    }

    /// Whether what the key owns carries comments that writing it, or removing
    /// it when `deleting`, would lose. Values are hashed without their comments,
    /// so the planner asks this before it rewrites or removes an entry.
    pub(crate) fn has_comments(&self, key: &EntryKey, deleting: bool) -> Result<bool, EntryError> {
        match self {
            Self::Json(doc) => doc.has_comments(key),
            Self::Toml(doc) => doc.has_comments(key, deleting),
        }
    }

    /// Removes each container in `pointers` that is now empty, deepest first.
    /// Returns those still holding something, which stay.
    pub(crate) fn prune(&mut self, pointers: &[String]) -> Result<Vec<String>, EntryError> {
        match self {
            Self::Json(doc) => doc.prune(pointers),
            Self::Toml(doc) => doc.prune(pointers),
        }
    }

    /// Whether the file existed with content before editing.
    pub(crate) fn existed(&self) -> bool {
        match self {
            Self::Json(doc) => doc.existed,
            Self::Toml(doc) => doc.existed(),
        }
    }

    /// Whether nothing is left in the file: no members, and no comments.
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            Self::Json(doc) => doc.is_empty(),
            Self::Toml(doc) => doc.is_empty(),
        }
    }

    /// The document as bytes.
    pub(crate) fn render(&self) -> Vec<u8> {
        match self {
            Self::Json(doc) => doc.render(),
            Self::Toml(doc) => doc.render(),
        }
    }
}

/// Whether a JSON node or anything inside it is a comment.
fn node_comments(node: &CstNode) -> bool {
    node.is_comment() || node.children().iter().any(node_comments)
}

/// Pointers in deepest-first order without duplicates, for pruning.
fn deepest_first(pointers: &[String]) -> Vec<&String> {
    let mut ordered: Vec<&String> = pointers.iter().collect();
    ordered.sort_by_key(|p| std::cmp::Reverse(p.matches('/').count()));
    ordered.dedup();
    ordered
}

/// A JSON or JSONC document whose root is an object, edited in place.
pub(crate) struct JsonDoc {
    /// Kept alive for as long as any node is used: the tree holds weak parent links.
    root: CstRootNode,
    /// Whether the file existed with content.
    existed: bool,
    /// Whether comments are allowed (JSONC).
    jsonc: bool,
}

impl JsonDoc {
    /// Parses the current file content (`None`: the file is absent), allowing
    /// comments when `jsonc`.
    fn parse(text: Option<&str>, jsonc: bool) -> Result<Self, EntryError> {
        let existed = text.is_some_and(|t| !t.trim().is_empty());
        let source = if existed {
            text.unwrap_or_default()
        } else {
            "{}\n"
        };
        let what = if jsonc { "JSONC" } else { "JSON" };
        // jsonc-parser 0.34.0 collects comments as tokens when building a CST and
        // then skips its `allow_comments` check (`scan_handling_comments`), so
        // strictness is checked first with serde_json, an RFC 8259 parser.
        if jsonc {
            check_jsonc(source)?;
        } else {
            serde_json::from_str::<Value>(source)
                .map_err(|e| err(format!("it is not valid JSON ({e})")))?;
        }
        let options = if jsonc { with_comments() } else { strict() };
        let root = CstRootNode::parse(source, &options)
            .map_err(|e| err(format!("it is not valid {what} ({e})")))?;
        match root.value() {
            Some(node) if node.as_object().is_some() => {}
            _ => return Err(err("its top level is not a JSON object")),
        }
        Ok(Self {
            root,
            existed,
            jsonc,
        })
    }

    /// Whether the member's value holds comments, which writing or removing it
    /// would lose. Strict JSON has none; JSONC entries are always members.
    fn has_comments(&self, key: &EntryKey) -> Result<bool, EntryError> {
        if !self.jsonc {
            return Ok(false);
        }
        match parse_key(key)? {
            Target::Member(path) => Ok(self.walk(&path)?.is_some_and(|n| node_comments(&n))),
            Target::Element(_) => Ok(false),
        }
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

    fn find(
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

    fn set(
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

    fn remove(&mut self, key: &EntryKey, disk: Option<&Sha256Digest>) -> Result<(), EntryError> {
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

    fn prune(&mut self, pointers: &[String]) -> Result<Vec<String>, EntryError> {
        let mut kept = Vec::new();
        for pointer in deepest_first(pointers) {
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

    /// Whether the top-level object has no members and no comments left. With no
    /// members there are no strings, so any `/` starts a JSONC comment.
    fn is_empty(&self) -> bool {
        self.root
            .object_value()
            .is_some_and(|o| o.properties().is_empty())
            && !self.root.to_string().contains('/')
    }

    /// The document as bytes.
    fn render(&self) -> Vec<u8> {
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

    fn json(text: Option<&str>) -> Result<EntryDoc, EntryError> {
        EntryDoc::parse(EntryFormat::Json, text)
    }

    fn jsonc(text: Option<&str>) -> Result<EntryDoc, EntryError> {
        EntryDoc::parse(EntryFormat::Jsonc, text)
    }

    fn rendered(doc: &EntryDoc) -> String {
        String::from_utf8(doc.render()).unwrap()
    }

    #[test]
    fn formats_follow_the_path() {
        let format = |p: &str| EntryFormat::for_path(&RelPath::new(p).unwrap());
        assert_eq!(format(".codex/config.toml"), EntryFormat::Toml);
        assert_eq!(format("x/Y.TOML"), EntryFormat::Toml);
        assert_eq!(format(".gemini/settings.json"), EntryFormat::Jsonc);
        assert_eq!(format("sub/.gemini/settings.json"), EntryFormat::Json);
        assert_eq!(format(".gemini/settings.json5"), EntryFormat::Json);
        assert_eq!(format(".mcp.json"), EntryFormat::Json);
        assert_eq!(format(".claude/settings.json"), EntryFormat::Json);
        assert_eq!(
            format(".toml"),
            EntryFormat::Json,
            "a dotfile, not an extension"
        );
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
        let mut doc = json(Some(text)).unwrap();
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
        let mut doc = json(Some(&written)).unwrap();
        let hash = doc.find(&k, None, None).unwrap();
        assert!(hash.is_some());
        doc.remove(&k, hash.as_ref()).unwrap();
        assert_eq!(String::from_utf8(doc.render()).unwrap(), text);
    }

    #[test]
    fn creates_and_deletes_a_new_file() {
        let mut doc = json(None).unwrap();
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
        let mut doc = json(Some(&written)).unwrap();
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
        let mut doc = json(Some("{\"a\": {}, \"b\": {\"c\": 1}, \"d\": []}")).unwrap();
        let kept = doc
            .prune(&["/b".to_owned(), "/d".to_owned(), "/missing".to_owned()])
            .unwrap();
        assert_eq!(kept, ["/b"]);
        let value: Value = serde_json::from_str(&rendered(&doc)).unwrap();
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
        let mut doc = json(Some(&text)).unwrap();
        let k = element_key(&["hooks", "PreToolUse"], "mine#0").unwrap();
        let v1 = json!({"matcher": "Edit", "hooks": [{"type": "command", "command": "mine", "args": []}]});
        let v2 = json!({"matcher": "Edit|Write", "hooks": [{"type": "command", "command": "mine", "args": []}]});
        assert_eq!(doc.set(&k, None, &v1).unwrap(), Vec::<String>::new());
        assert_eq!(
            doc.find(&k, None, Some(&value_hash(&v1))).unwrap(),
            Some(value_hash(&v1))
        );
        doc.set(&k, Some(&value_hash(&v1)), &v2).unwrap();
        let value = serde_json::from_str::<Value>(&rendered(&doc)).unwrap();
        assert_eq!(
            value["hooks"]["PreToolUse"].as_array().unwrap().len(),
            2,
            "replaced, not appended"
        );
        assert_eq!(doc.find(&k, Some(&value_hash(&v1)), None).unwrap(), None);
        doc.remove(&k, Some(&value_hash(&v2))).unwrap();
        let value = serde_json::from_str::<Value>(&rendered(&doc)).unwrap();
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
            assert!(json(Some(bad)).is_err(), "{bad}");
        }
        let doc = json(Some("{\"mcpServers\": [], \"x\": 1}")).unwrap();
        assert!(doc.find(&key("/mcpServers/a"), None, None).is_err());
        let dup = json(Some("{\"mcpServers\": {}, \"mcpServers\": {}}")).unwrap();
        assert!(dup.find(&key("/mcpServers/a"), None, None).is_err());
    }

    /// A Gemini-style settings file: comments before, inside and after the
    /// object Grimoire edits, and a `//` inside a string that is not a comment.
    const COMMENTED: &str = "// Project settings\n{\n  /* servers */\n  \"mcpServers\": {\n    // theirs comes first\n    \"theirs\": {\"command\": \"x\"} // keep me\n  },\n  \"url\": \"https://example.com/a\" /* trailing */\n}\n// the end\n";

    #[test]
    fn strict_json_still_refuses_comments() {
        for bad in [
            COMMENTED,
            "{} // after",
            "/* before */ {}",
            "{\"a\": /* inside */ 1}",
        ] {
            let e = json(Some(bad)).err().unwrap();
            assert!(e.to_string().starts_with("it is not valid JSON ("), "{e}");
            assert!(jsonc(Some(bad)).is_ok(), "{bad}");
        }
    }

    #[test]
    fn jsonc_keeps_comments_around_the_edit() {
        let mut doc = jsonc(Some(COMMENTED)).unwrap();
        let k = key("/mcpServers/mine");
        assert_eq!(doc.find(&k, None, None).unwrap(), None);
        let mine = json!({"command": "grimoire-mcp-x", "env": {"A": "$A"}});
        assert_eq!(doc.set(&k, None, &mine).unwrap(), Vec::<String>::new());
        let written = rendered(&doc);
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
        // What Gemini CLI would read: the text with its comments stripped.
        let mut doc = jsonc(Some(&written)).unwrap();
        assert_eq!(doc.find(&k, None, None).unwrap(), Some(value_hash(&mine)));
        assert_eq!(
            doc.find(&key("/url"), None, None).unwrap(),
            Some(value_hash(&json!("https://example.com/a")))
        );
        doc.remove(&k, Some(&value_hash(&mine))).unwrap();
        assert_eq!(rendered(&doc), COMMENTED);
    }

    #[test]
    fn jsonc_refuses_everything_but_comments() {
        for bad in [
            "{\"a\": 1,}",
            "{\"a\": [1,]}",
            "{'a': 1}",
            "{\"a\": 'b'}",
            "{a: 1}",
            "{\"a\": 0x1F}",
            "{\"a\": +1}",
            "{\"a\": .5}",
            "{\"a\": NaN}",
            "{\"a\": \"\\x41\"}",
            "{\"a\": 1 \"b\": 2}",
            "{\"a\": \"raw\ttab\"}",
            "{\"a\": 01}",
            "{} {}",
            "{} /* unterminated",
            "// a comment ended by a lone CR\r{}",
            "[1, 2]",
        ] {
            let e = jsonc(Some(bad)).err();
            assert!(e.is_some(), "{bad:?} was accepted");
            let e = e.unwrap().to_string();
            assert!(
                e.starts_with("it is not valid JSONC (")
                    || e == "its top level is not a JSON object",
                "{bad:?}: {e}"
            );
        }
        // CRLF line endings end `//` comments for every reader.
        assert!(jsonc(Some("// note\r\n{}\r\n")).is_ok());
        let dup = jsonc(Some("{\"mcpServers\": {}, /* again */ \"mcpServers\": {}}")).unwrap();
        assert!(dup.find(&key("/mcpServers/a"), None, None).is_err());
    }

    #[test]
    fn jsonc_comments_are_content() {
        let mut doc = jsonc(Some("{\n  \"mine\": 1\n}\n// my note\n")).unwrap();
        doc.remove(&key("/mine"), None).unwrap();
        assert!(!doc.is_empty(), "the comment is the user's");
        let mut doc = jsonc(Some("{\n  \"mine\": 1\n}\n")).unwrap();
        doc.remove(&key("/mine"), None).unwrap();
        assert!(doc.is_empty());
    }
}
