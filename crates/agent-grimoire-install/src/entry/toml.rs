//! Owned entries in TOML files such as Codex's `.codex/config.toml`, edited with
//! `toml_edit` so that everything else in the file keeps its text.
//!
//! An entry is an object member, found by its JSON Pointer through tables, dotted
//! keys and inline tables alike. Array elements cannot be owned in TOML.
//!
//! - **Writing.** A value is converted to TOML first, so a value TOML cannot
//!   hold (`null`, an integer beyond `i64`) changes nothing. Under a standard or
//!   implicit table an object becomes a standard table (`[mcp_servers.id]`), with
//!   nested objects as inline tables. Under dotted keys it becomes dotted keys
//!   (an empty object, which would leave no text, is written `id = {}`), and
//!   inside an inline table an inline table: a `[table]` header cannot extend an
//!   inline table, so the entry follows whatever form its parent has.
//! - **Creating containers.** Missing tables on the way are created as implicit
//!   tables (no header of their own), or as dotted or inline tables under a
//!   parent of that form, and their pointers are returned like the JSON ones.
//!   A plain value can't go into a table without a header of its own: it would
//!   add a `[parent]` header that the user may later rely on, so it is refused.
//!   Grimoire's adapters write only tables to TOML.
//! - **Replacing.** An existing entry keeps its form, its place in the file, its
//!   header line and the comments above it. Comments inside it are replaced with
//!   it, so the planner treats them as an edit (`has_comments`).
//! - **Removing.** Comments above a removed `[header]` belong to whatever comes
//!   before it, such as a setting the user commented out at the end of the
//!   previous table, so they move to the next header, or to the end of the file.
//! - **Pruning.** An empty table Grimoire created goes, unless it now has a
//!   header or comments of the user's.
//! - **Bytes.** A file Grimoire didn't change renders as its original bytes. A
//!   changed one keeps its byte-order mark and its CRLF line endings, so a
//!   `core.autocrlf` checkout isn't rewritten (security model F13).
//! - **Hashing.** What is on disk is read back as JSON and hashed with
//!   [`value_hash`], so integers stay integers and `D == N` holds right after a
//!   write. Datetimes and non-finite floats have no JSON form and never match.

use agent_grimoire_model::{EntryKey, Sha256Digest};
use serde_json::{Map, Number, Value as Json};
use toml_edit::{
    Array, Decor, DocumentMut, InlineTable, Item, RawString, Table, TableLike, TomlError, Value,
};

use super::{EntryError, Target, deepest_first, err, escape, parse_key, pointer, value_hash};

/// A TOML document, edited in place.
pub(crate) struct TomlDoc {
    doc: DocumentMut,
    /// Whether the file existed with content.
    existed: bool,
    /// What the file looked like, to keep its bytes.
    shape: Shape,
    /// Whether anything was edited.
    touched: bool,
}

/// The parts of a file's bytes that `toml_edit` doesn't keep.
struct Shape {
    /// The file as read, returned unchanged when nothing was edited.
    original: Option<String>,
    /// Whether the file started with a byte-order mark.
    bom: bool,
    /// Whether the file used CRLF line endings.
    crlf: bool,
}

impl TomlDoc {
    /// Parses the current file content (`None`: the file is absent). An absent or
    /// blank file starts empty.
    pub(super) fn parse(text: Option<&str>) -> Result<Self, EntryError> {
        let body = text.map(|t| t.strip_prefix('\u{feff}').unwrap_or(t));
        let existed = body.is_some_and(|t| !t.trim().is_empty());
        let doc = match body {
            Some(body) if existed => body.parse::<DocumentMut>().map_err(|e| invalid(body, &e))?,
            _ => DocumentMut::new(),
        };
        Ok(Self {
            doc,
            existed,
            shape: Shape {
                original: text.map(str::to_owned),
                bom: text.is_some_and(|t| t.starts_with('\u{feff}')),
                crlf: body.is_some_and(|t| t.contains("\r\n")),
            },
            touched: false,
        })
    }

    /// The hash of the member's value on disk, read back as JSON.
    pub(super) fn find(&self, key: &EntryKey) -> Result<Option<Sha256Digest>, EntryError> {
        let path = member(key)?;
        Ok(lookup(self.doc.as_item(), &path)?.map(item_hash))
    }

    /// Writes `value` at the member. Returns the pointers of the tables it had to
    /// create.
    pub(super) fn set(&mut self, key: &EntryKey, value: &Json) -> Result<Vec<String>, EntryError> {
        let path = member(key)?;
        let (name, parents) = path
            .split_last()
            .ok_or_else(|| err("an entry key cannot own the whole file"))?;
        let value = to_toml(value, &pointer(&path))?;
        if !parents.is_empty()
            && !matches!(value, Value::InlineTable(_))
            && lookup(self.doc.as_item(), parents)?.is_none_or(header_hidden)
        {
            return Err(err(format!(
                "`{}` is not a table, and a plain value there would add a `[{}]` header to the file; grimoire writes only tables under a table without a header",
                pointer(&path),
                parents.join(".")
            )));
        }
        self.touched = true;
        let mut created = Vec::new();
        let parent = tables_to(self.doc.as_item_mut(), parents, &mut created)?;
        let form = Form::of(parent);
        let table = parent
            .as_table_like_mut()
            .ok_or_else(|| not_table(parents))?;
        match table.get_mut(name).filter(|item| !item.is_none()) {
            Some(existing) => replace(existing, value, form),
            None => {
                table.insert(name, form.entry(value));
            }
        }
        Ok(created)
    }

    /// Removes the member, if it is there, keeping the comments above its header.
    pub(super) fn remove(&mut self, key: &EntryKey) -> Result<(), EntryError> {
        let path = member(key)?;
        let Some((name, parents)) = path.split_last() else {
            return Ok(());
        };
        let Some(table) =
            lookup_mut(self.doc.as_item_mut(), parents)?.and_then(Item::as_table_like_mut)
        else {
            return Ok(());
        };
        let Some(removed) = table.remove(name) else {
            return Ok(());
        };
        self.touched = true;
        if let Item::Table(t) = &removed
            && has_header(t)
            && let Some(kept) = comment_lines(t.decor().prefix())
        {
            self.relocate(&kept, t.position());
        }
        Ok(())
    }

    /// Puts comment lines that sat above a removed header before the next header
    /// in the document, or at the end of the file.
    fn relocate(&mut self, kept: &str, after: Option<isize>) {
        let mut headers = Vec::new();
        collect_headers(self.doc.as_item(), &mut Vec::new(), &mut headers);
        let next = after.and_then(|after| {
            headers
                .into_iter()
                .filter(|(position, _, _)| *position > after)
                .min_by_key(|(position, _, _)| *position)
        });
        let target = next.and_then(|(_, path, index)| {
            let item = lookup_mut(self.doc.as_item_mut(), &path).ok()??;
            match (item, index) {
                (Item::Table(t), None) => Some(t.decor_mut()),
                (Item::ArrayOfTables(a), Some(i)) => a.get_mut(i).map(Table::decor_mut),
                _ => None,
            }
        });
        if let Some(decor) = target {
            let rest = raw(decor.prefix()).to_owned();
            decor.set_prefix(format!("{kept}{rest}"));
        } else {
            let rest = raw(Some(self.doc.trailing())).to_owned();
            self.doc.set_trailing(format!("{kept}{rest}"));
        }
    }

    /// Whether what the member holds carries comments that writing or removing it
    /// would lose: comments inside it, and, when `deleting`, the comment on its
    /// header line or its own line. Comments above a header are kept anyway.
    pub(super) fn has_comments(&self, key: &EntryKey, deleting: bool) -> Result<bool, EntryError> {
        let path = member(key)?;
        let Some((name, parents)) = path.split_last() else {
            return Ok(false);
        };
        let Some(parent) = lookup(self.doc.as_item(), parents)?.and_then(Item::as_table_like)
        else {
            return Ok(false);
        };
        let Some((key, item)) = parent.get_key_value(name).filter(|(_, i)| !i.is_none()) else {
            return Ok(false);
        };
        let mut found = item_comments(item);
        if deleting {
            found |= match item {
                Item::Table(t) if has_header(t) => commented(t.decor().suffix()),
                Item::Value(v) => {
                    decor_commented(key.leaf_decor()) || commented(v.decor().suffix())
                }
                _ => decor_commented(key.leaf_decor()),
            };
        }
        Ok(found)
    }

    /// Removes each table in `pointers` that is now empty, deepest first. Returns
    /// those still holding something, which stay. An empty one stays too when it
    /// has a header or comments: the user wrote those.
    pub(super) fn prune(&mut self, pointers: &[String]) -> Result<Vec<String>, EntryError> {
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
            let Some(parent) =
                lookup_mut(self.doc.as_item_mut(), parents)?.and_then(Item::as_table_like_mut)
            else {
                continue;
            };
            let Some(item) = parent.get_mut(name).filter(|item| !item.is_none()) else {
                continue;
            };
            let empty = item.as_table_like().is_some_and(TableLike::is_empty)
                || item.as_array().is_some_and(Array::is_empty);
            let users = match &*item {
                Item::Table(t) => has_header(t) || decor_commented(t.decor()),
                Item::Value(v) => decor_commented(v.decor()),
                _ => true,
            };
            if empty && !users {
                parent.remove(name);
                self.touched = true;
            } else {
                kept.push(pointer.clone());
            }
        }
        Ok(kept)
    }

    /// Whether the file existed with content before editing.
    pub(super) fn existed(&self) -> bool {
        self.existed
    }

    /// Whether the file would hold nothing but whitespace: no keys, no tables
    /// with a header, and no comments.
    pub(super) fn is_empty(&self) -> bool {
        self.doc.to_string().trim().is_empty()
    }

    /// The document as bytes: the original bytes when nothing was edited, else the
    /// edited document with the file's byte-order mark and line endings. A file
    /// Grimoire creates ends with a newline.
    pub(super) fn render(&self) -> Vec<u8> {
        if !self.touched
            && let Some(original) = &self.shape.original
        {
            return original.clone().into_bytes();
        }
        let mut text = self.doc.to_string();
        if !self.existed && !text.ends_with('\n') {
            text.push('\n');
        }
        if self.shape.crlf {
            text = crlf(&text);
        }
        if self.shape.bom {
            text.insert(0, '\u{feff}');
        }
        text.into_bytes()
    }
}

/// `text` with every line feed not already after a carriage return turned into
/// CRLF. Multi-line strings parsed from a CRLF file already hold `\r\n`.
fn crlf(text: &str) -> String {
    let mut out = String::with_capacity(text.len() + text.len() / 16);
    let mut previous = '\0';
    for c in text.chars() {
        if c == '\n' && previous != '\r' {
            out.push('\r');
        }
        out.push(c);
        previous = c;
    }
    out
}

/// Whether `table` prints a `[header]` line of its own.
fn has_header(table: &Table) -> bool {
    !table.is_implicit() && !table.is_dotted()
}

fn raw(text: Option<&RawString>) -> &str {
    text.and_then(RawString::as_str).unwrap_or_default()
}

/// Whether decor text holds a comment. Decor is only whitespace and comments.
fn commented(text: Option<&RawString>) -> bool {
    raw(text).contains('#')
}

fn decor_commented(decor: &Decor) -> bool {
    commented(decor.prefix()) || commented(decor.suffix())
}

/// The comment lines in a header's prefix, without the blank lines after the
/// last one (Grimoire's own separator); `None` when there is no comment.
fn comment_lines(prefix: Option<&RawString>) -> Option<String> {
    let text = raw(prefix);
    if !text.contains('#') {
        return None;
    }
    let last = text.trim_end().len();
    let end = text[last..].find('\n').map_or(text.len(), |n| last + n + 1);
    Some(text[..end].to_owned())
}

/// Every `[header]` in the document with its position, its path, and its index
/// when it is one table of an array of tables.
fn collect_headers(
    item: &Item,
    path: &mut Vec<String>,
    out: &mut Vec<(isize, Vec<String>, Option<usize>)>,
) {
    match item {
        Item::Table(table) => {
            if has_header(table)
                && !path.is_empty()
                && let Some(position) = table.position()
            {
                out.push((position, path.clone(), None));
            }
            for (key, child) in table {
                path.push(key.to_owned());
                collect_headers(child, path, out);
                path.pop();
            }
        }
        Item::ArrayOfTables(tables) => {
            for (index, table) in tables.iter().enumerate() {
                if let Some(position) = table.position() {
                    out.push((position, path.clone(), Some(index)));
                }
            }
        }
        Item::None | Item::Value(_) => {}
    }
}

/// Whether anything inside `item` carries a comment.
fn item_comments(item: &Item) -> bool {
    match item {
        Item::None => false,
        Item::Value(value) => value_comments(value),
        Item::Table(table) => table_comments(table),
        Item::ArrayOfTables(tables) => tables
            .iter()
            .any(|t| decor_commented(t.decor()) || table_comments(t)),
    }
}

fn table_comments(table: &Table) -> bool {
    table.iter().any(|(name, child)| {
        let key = table.key(name);
        key.is_some_and(|k| decor_commented(k.leaf_decor()) || decor_commented(k.dotted_decor()))
            || match child {
                Item::Table(t) => decor_commented(t.decor()) || table_comments(t),
                Item::Value(v) => decor_commented(v.decor()) || value_comments(v),
                other => item_comments(other),
            }
    })
}

fn value_comments(value: &Value) -> bool {
    match value {
        Value::Array(array) => {
            commented(Some(array.trailing()))
                || array
                    .iter()
                    .any(|v| decor_commented(v.decor()) || value_comments(v))
        }
        Value::InlineTable(table) => {
            commented(Some(table.trailing()))
                || table.iter().any(|(name, v)| {
                    table.key(name).is_some_and(|k| {
                        decor_commented(k.leaf_decor()) || decor_commented(k.dotted_decor())
                    }) || decor_commented(v.decor())
                        || value_comments(v)
                })
        }
        _ => false,
    }
}

/// The path of a member key; array elements cannot be owned in TOML.
fn member(key: &EntryKey) -> Result<Vec<String>, EntryError> {
    match parse_key(key)? {
        Target::Member(path) => Ok(path),
        Target::Element(_) => Err(err(format!(
            "entry key `{}` owns an array element, which grimoire does not support in TOML files",
            key.as_str()
        ))),
    }
}

fn not_table<S: AsRef<str>>(path: &[S]) -> EntryError {
    err(format!("`{}` is not a TOML table", pointer(path)))
}

/// Whether `item` is a table without a printed `[header]`: an implicit table
/// with no values of its own, only tables with headers of their own.
fn header_hidden(item: &Item) -> bool {
    matches!(item, Item::Table(t) if t.is_implicit() && !t.is_dotted() && t.get_values().is_empty())
}

/// Why `text` is not valid TOML, on one line.
fn invalid(text: &str, e: &TomlError) -> EntryError {
    let message = e.message().trim().replace('\n', "; ");
    let place = e
        .span()
        .and_then(|span| text.get(..span.start))
        .map(|before| {
            let line = before.matches('\n').count() + 1;
            let column = before
                .rsplit('\n')
                .next()
                .unwrap_or_default()
                .chars()
                .count()
                + 1;
            format!(" at line {line}, column {column}")
        })
        .unwrap_or_default();
    err(format!("it is not valid TOML ({message}{place})"))
}

/// Follows `path` through tables and inline tables. `Ok(None)` when a key is absent.
fn lookup<'a>(root: &'a Item, path: &[String]) -> Result<Option<&'a Item>, EntryError> {
    let mut current = root;
    for (n, segment) in path.iter().enumerate() {
        let table = current
            .as_table_like()
            .ok_or_else(|| not_table(&path[..n]))?;
        match table.get(segment).filter(|item| !item.is_none()) {
            Some(item) => current = item,
            None => return Ok(None),
        }
    }
    Ok(Some(current))
}

/// [`lookup`], for editing.
fn lookup_mut<'a>(root: &'a mut Item, path: &[String]) -> Result<Option<&'a mut Item>, EntryError> {
    let mut current = root;
    for (n, segment) in path.iter().enumerate() {
        let table = current
            .as_table_like_mut()
            .ok_or_else(|| not_table(&path[..n]))?;
        match table.get_mut(segment).filter(|item| !item.is_none()) {
            Some(item) => current = item,
            None => return Ok(None),
        }
    }
    Ok(Some(current))
}

/// Creates the tables along `path` and returns the innermost one, recording the
/// pointer of every table it had to create in `created`.
fn tables_to<'a>(
    root: &'a mut Item,
    path: &[String],
    created: &mut Vec<String>,
) -> Result<&'a mut Item, EntryError> {
    let mut current = root;
    for (n, segment) in path.iter().enumerate() {
        let form = Form::of(current);
        let table = current
            .as_table_like_mut()
            .ok_or_else(|| not_table(&path[..n]))?;
        current = match table.entry(segment) {
            toml_edit::Entry::Occupied(entry) if !entry.get().is_none() => entry.into_mut(),
            toml_edit::Entry::Occupied(entry) => {
                created.push(pointer(&path[..=n]));
                let item = entry.into_mut();
                *item = form.container();
                item
            }
            toml_edit::Entry::Vacant(entry) => {
                created.push(pointer(&path[..=n]));
                entry.insert(form.container())
            }
        };
    }
    if !current.is_table_like() {
        return Err(not_table(path));
    }
    Ok(current)
}

/// How a table holds what is inserted into it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Form {
    /// A standard or implicit table: an object inside gets its own `[header]`.
    Table,
    /// A table written as dotted keys (`a.b = 1`): what is inside is dotted too.
    Dotted,
    /// An inline table: everything inside is inline.
    Inline,
}

impl Form {
    fn of(item: &Item) -> Self {
        match item {
            Item::Table(table) if table.is_dotted() => Form::Dotted,
            Item::Table(_) => Form::Table,
            _ => Form::Inline,
        }
    }

    /// An empty table to create inside a parent of this form.
    fn container(self) -> Item {
        match self {
            Form::Table => {
                let mut table = Table::new();
                table.set_implicit(true);
                Item::Table(table)
            }
            Form::Dotted => {
                let mut table = Table::new();
                table.set_dotted(true);
                Item::Table(table)
            }
            Form::Inline => Item::Value(Value::InlineTable(InlineTable::new())),
        }
    }

    /// `value` as a new entry inside a parent of this form: an object becomes a
    /// table of the parent's kind, anything else a plain value.
    fn entry(self, value: Value) -> Item {
        match (self, value) {
            (Form::Table, Value::InlineTable(inline)) => Item::Table(inline.into_table()),
            (Form::Dotted, Value::InlineTable(inline)) => dotted(inline),
            (_, value) => Item::Value(value),
        }
    }
}

/// An object as dotted keys. An empty one would leave no text at all, so it
/// stays an inline table (`id = {}`).
fn dotted(inline: InlineTable) -> Item {
    if inline.is_empty() {
        return Item::Value(Value::InlineTable(inline));
    }
    let mut table = inline.into_table();
    table.set_dotted(true);
    Item::Table(table)
}

/// Replaces an existing entry in place, keeping its form, its position in the
/// file and the comments attached to it.
fn replace(existing: &mut Item, value: Value, form: Form) {
    match (existing, value) {
        (existing @ Item::Table(_), Value::InlineTable(inline))
            if existing.as_table().is_some_and(Table::is_dotted) =>
        {
            *existing = dotted(inline);
        }
        (Item::Table(old), Value::InlineTable(inline)) => {
            let mut table = inline.into_table();
            table.set_position(old.position());
            *table.decor_mut() = old.decor().clone();
            *old = table;
        }
        (Item::Value(old), value) => {
            let decor = old.decor().clone();
            *old = value;
            *old.decor_mut() = decor;
        }
        (existing, value) => *existing = form.entry(value),
    }
}

/// `value` as TOML. `at` is its pointer, for errors.
fn to_toml(value: &Json, at: &str) -> Result<Value, EntryError> {
    Ok(match value {
        Json::Null => {
            return Err(err(format!("`{at}` is null, which TOML cannot express")));
        }
        Json::Bool(b) => Value::from(*b),
        Json::Number(n) => {
            if let Some(i) = n.as_i64() {
                Value::from(i)
            } else if n.is_f64()
                && let Some(f) = n.as_f64()
            {
                Value::from(f)
            } else {
                return Err(err(format!(
                    "`{at}` is {n}, which is beyond the 64-bit signed integers TOML can express"
                )));
            }
        }
        Json::String(s) => Value::from(s.as_str()),
        Json::Array(items) => {
            let mut array = Array::new();
            for (n, item) in items.iter().enumerate() {
                array.push(to_toml(item, &format!("{at}/{n}"))?);
            }
            Value::Array(array)
        }
        Json::Object(map) => {
            let mut table = InlineTable::new();
            for (k, v) in map {
                table.insert(k, to_toml(v, &format!("{at}/{}", escape(k)))?);
            }
            Value::InlineTable(table)
        }
    })
}

/// The hash of an item read back as JSON. An item with no JSON form gets a hash
/// no JSON value has (canonical JSON never starts with NUL), so it never matches.
fn item_hash(item: &Item) -> Sha256Digest {
    match item_json(item) {
        Some(value) => value_hash(&value),
        None => Sha256Digest::of(format!("\0toml {item}").as_bytes()),
    }
}

/// An item as JSON; `None` if it holds a datetime or a non-finite float.
fn item_json(item: &Item) -> Option<Json> {
    match item {
        Item::None => None,
        Item::Value(value) => value_json(value),
        Item::Table(table) => table_json(table),
        Item::ArrayOfTables(tables) => tables
            .iter()
            .map(table_json)
            .collect::<Option<Vec<_>>>()
            .map(Json::Array),
    }
}

fn table_json(table: &Table) -> Option<Json> {
    table
        .iter()
        .map(|(k, v)| Some((k.to_owned(), item_json(v)?)))
        .collect::<Option<Map<_, _>>>()
        .map(Json::Object)
}

fn value_json(value: &Value) -> Option<Json> {
    Some(match value {
        Value::String(s) => Json::String(s.value().clone()),
        Value::Integer(i) => Json::from(*i.value()),
        Value::Float(f) => Json::Number(Number::from_f64(*f.value())?),
        Value::Boolean(b) => Json::Bool(*b.value()),
        Value::Datetime(_) => return None,
        Value::Array(array) => Json::Array(array.iter().map(value_json).collect::<Option<_>>()?),
        Value::InlineTable(table) => Json::Object(
            table
                .iter()
                .map(|(k, v)| Some((k.to_owned(), value_json(v)?)))
                .collect::<Option<_>>()?,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::element_key;
    use serde_json::json;

    fn key(s: &str) -> EntryKey {
        EntryKey::new(s).unwrap()
    }

    fn parse(text: Option<&str>) -> TomlDoc {
        TomlDoc::parse(text).unwrap()
    }

    fn rendered(doc: &TomlDoc) -> String {
        String::from_utf8(doc.render()).unwrap()
    }

    fn mine() -> Json {
        json!({"command": "grimoire-mcp-x", "env_vars": ["A", "B"]})
    }

    const MINE: &str =
        "[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-x\"\nenv_vars = [\"A\", \"B\"]\n";

    /// A user's Codex config with comments everywhere and a server of their own.
    const USER: &str = "# Codex project config\nmodel = \"o3\" # pinned\n\n[profiles.fast]\nmodel = \"o4-mini\"\n\n# Servers I added myself\n[mcp_servers.theirs]\ncommand = \"theirs\" # their comment\nargs = [\"--x\"]\n\n[tail] # last\nz = 2\n";

    #[test]
    fn creates_a_file_and_removes_it_again() {
        let mut doc = parse(None);
        assert!(!doc.existed());
        let k = key("/mcp_servers/docs-index");
        assert_eq!(doc.find(&k).unwrap(), None);
        assert_eq!(doc.set(&k, &mine()).unwrap(), ["/mcp_servers"]);
        let written = rendered(&doc);
        assert_eq!(
            written, MINE,
            "a standard table, keys in order, final newline"
        );
        let mut doc = parse(Some(&written));
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&mine())));
        doc.remove(&k).unwrap();
        assert_eq!(
            doc.prune(&["/mcp_servers".to_owned()]).unwrap(),
            Vec::<String>::new()
        );
        assert!(doc.is_empty());
    }

    #[test]
    fn a_blank_file_starts_empty() {
        let mut doc = parse(Some("  \n\n"));
        assert!(!doc.existed());
        doc.set(&key("/mcp_servers/docs-index"), &mine()).unwrap();
        assert_eq!(rendered(&doc), MINE);
    }

    #[test]
    fn the_rest_of_the_file_survives_byte_for_byte() {
        let mut doc = parse(Some(USER));
        let k = key("/mcp_servers/docs-index");
        assert_eq!(
            doc.set(&k, &mine()).unwrap(),
            Vec::<String>::new(),
            "`mcp_servers` already existed, implicitly"
        );
        let written = rendered(&doc);
        let expected = USER.replace("\n[tail] # last", &format!("\n{MINE}\n[tail] # last"));
        assert_eq!(
            written, expected,
            "next to the user's server, nothing else touched"
        );
        let mut doc = parse(Some(&written));
        doc.remove(&k).unwrap();
        assert_eq!(doc.prune(&[]).unwrap(), Vec::<String>::new());
        assert_eq!(rendered(&doc), USER);
    }

    #[test]
    fn a_new_parent_goes_at_the_end_and_is_pruned() {
        let user = "# top\nmodel = \"o3\"\n\n[profiles.fast] # fast\nmodel = \"o4-mini\"\n# trailing comment\n";
        let mut doc = parse(Some(user));
        let k = key("/mcp_servers/docs-index");
        assert_eq!(doc.set(&k, &mine()).unwrap(), ["/mcp_servers"]);
        let written = rendered(&doc);
        assert!(
            written.starts_with(
                "# top\nmodel = \"o3\"\n\n[profiles.fast] # fast\nmodel = \"o4-mini\"\n"
            ),
            "{written}"
        );
        assert!(written.contains(MINE), "{written}");
        let mut doc = parse(Some(&written));
        doc.remove(&k).unwrap();
        assert_eq!(
            doc.prune(&["/mcp_servers".to_owned()]).unwrap(),
            Vec::<String>::new()
        );
        assert_eq!(rendered(&doc), user);
        assert!(!doc.is_empty());
    }

    #[test]
    fn a_created_table_with_user_content_stays() {
        let mut doc = parse(None);
        let k = key("/mcp_servers/docs-index");
        doc.set(&k, &mine()).unwrap();
        let written = format!(
            "{}\n[mcp_servers.theirs]\ncommand = \"t\"\n",
            rendered(&doc)
        );
        let mut doc = parse(Some(&written));
        doc.remove(&k).unwrap();
        assert_eq!(
            doc.prune(&["/mcp_servers".to_owned()]).unwrap(),
            ["/mcp_servers"]
        );
        assert_eq!(
            rendered(&doc),
            "\n[mcp_servers.theirs]\ncommand = \"t\"\n",
            "the user's text, blank line included"
        );
        assert!(!doc.is_empty());
    }

    #[test]
    fn comments_alone_are_content() {
        let mut doc = parse(Some("# keep this note\n"));
        assert!(doc.existed());
        let k = key("/mcp_servers/docs-index");
        doc.set(&k, &mine()).unwrap();
        let mut doc = parse(Some(&rendered(&doc)));
        doc.remove(&k).unwrap();
        doc.prune(&["/mcp_servers".to_owned()]).unwrap();
        assert_eq!(rendered(&doc), "# keep this note\n");
        assert!(!doc.is_empty(), "the comment is the user's");
    }

    #[test]
    fn plain_values_never_add_a_header() {
        // `mcp_servers` exists only through `[mcp_servers.a]`, so it has no header.
        let user = "x = 1\n\n[mcp_servers.a]\ncommand = \"a\"\n";
        let mut doc = parse(Some(user));
        let k = key("/mcp_servers/mine");
        let e = doc.set(&k, &json!(false)).unwrap_err().to_string();
        assert!(e.contains("would add a `[mcp_servers]` header"), "{e}");
        assert_eq!(rendered(&doc), user, "nothing changed");
        // The same where the table doesn't exist yet.
        let mut doc = parse(Some("x = 1\n"));
        assert!(doc.set(&key("/new/mine"), &json!(1)).is_err());
        // A table needs no header from its parent; a top-level value none at all.
        let mut doc = parse(Some(user));
        assert_eq!(doc.set(&k, &mine()).unwrap(), Vec::<String>::new());
        let mut doc = parse(Some(user));
        doc.set(&key("/top"), &json!(2)).unwrap();
        assert!(
            rendered(&doc).starts_with("x = 1\ntop = 2\n"),
            "{}",
            rendered(&doc)
        );
        // Under a header the user wrote, a plain value is fine.
        let mut doc = parse(Some("[mcp_servers]\ntimeout = 5\n"));
        doc.set(&key("/mcp_servers/other"), &json!(1)).unwrap();
    }

    #[test]
    fn untouched_files_keep_their_bytes() {
        let k = key("/mcp_servers/docs-index");
        for original in [
            "model = \"o3\"\r\n\r\n[mcp_servers.docs-index]\r\ncommand = \"grimoire-mcp-x\"\r\nenv_vars = [\"A\", \"B\"]\r\n",
            "\u{feff}[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-x\"\nenv_vars = [\"A\", \"B\"]\n",
            "[mcp_servers.docs-index]\ncommand = \"grimoire-mcp-x\"\nenv_vars = [\"A\", \"B\"]",
        ] {
            let mut doc = parse(Some(original));
            assert_eq!(
                doc.find(&k).unwrap(),
                Some(value_hash(&mine())),
                "{original:?}"
            );
            assert_eq!(doc.prune(&[]).unwrap(), Vec::<String>::new());
            assert_eq!(
                rendered(&doc),
                original,
                "nothing edited, nothing rewritten"
            );
        }
    }

    #[test]
    fn edited_files_keep_their_line_endings_and_bom() {
        let k = key("/mcp_servers/docs-index");
        let user = "\u{feff}# mine\r\nmodel = \"o3\"\r\ns = \"\"\"\r\nmulti\r\nline\"\"\"\r\n";
        let mut doc = parse(Some(user));
        doc.set(&k, &mine()).unwrap();
        let written = rendered(&doc);
        assert!(written.starts_with('\u{feff}'), "{written:?}");
        assert!(
            !written.replace("\r\n", "").contains('\n'),
            "only CRLF: {written:?}"
        );
        assert!(!written.contains("\r\r"), "{written:?}");
        let mut doc = parse(Some(&written));
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&mine())));
        doc.remove(&k).unwrap();
        doc.prune(&["/mcp_servers".to_owned()]).unwrap();
        assert_eq!(rendered(&doc), user, "removing restores the user's bytes");
    }

    #[test]
    fn comments_above_a_removed_header_stay() {
        let k = key("/mcp_servers/docs-index");
        // A setting the user commented out at the end of the table before ours.
        let user = "model = \"o3\"\n\n[profiles.fast]\nmodel = \"o4\"\n";
        let mut doc = parse(Some(user));
        doc.set(&k, &mine()).unwrap();
        let written = rendered(&doc).replace(
            "model = \"o4\"\n",
            "model = \"o4\"\n# model_reasoning_effort = \"high\"\n",
        );
        let mut doc = parse(Some(&written));
        assert!(!doc.has_comments(&k, true).unwrap(), "not inside the entry");
        doc.remove(&k).unwrap();
        doc.prune(&["/mcp_servers".to_owned()]).unwrap();
        assert_eq!(
            rendered(&doc),
            "model = \"o3\"\n\n[profiles.fast]\nmodel = \"o4\"\n# model_reasoning_effort = \"high\"\n"
        );
        // A note directly above the header moves to the next header.
        let user = "[mcp_servers.docs-index]\ncommand = \"x\"\n\n[tail]\nz = 2\n";
        let noted = format!("# my note\n{user}");
        let mut doc = parse(Some(&noted));
        doc.remove(&k).unwrap();
        assert_eq!(rendered(&doc), "# my note\n\n[tail]\nz = 2\n");
    }

    #[test]
    fn comments_inside_an_entry_are_reported() {
        let k = key("/mcp_servers/docs-index");
        let plain = "[mcp_servers.docs-index]\ncommand = \"x\"\nenv_vars = [\"A\"]\n";
        let doc = parse(Some(plain));
        assert!(!doc.has_comments(&k, false).unwrap());
        assert!(!doc.has_comments(&k, true).unwrap());
        for inner in [
            "[mcp_servers.docs-index]\n# inner note\ncommand = \"x\"\n",
            "[mcp_servers.docs-index]\ncommand = \"x\" # why\n",
            "[mcp_servers.docs-index]\nenv_vars = [\n  \"A\", # first\n]\n",
        ] {
            let doc = parse(Some(inner));
            assert!(doc.has_comments(&k, false).unwrap(), "{inner}");
            assert!(doc.has_comments(&k, true).unwrap(), "{inner}");
        }
        // The header line's comment survives an update but not a removal.
        let doc = parse(Some("[mcp_servers.docs-index] # mine\ncommand = \"x\"\n"));
        assert!(!doc.has_comments(&k, false).unwrap());
        assert!(doc.has_comments(&k, true).unwrap());
        // So does a comment on a dotted or inline entry's own line.
        let doc = parse(Some(
            "mcp_servers.docs-index = { command = \"x\" } # mine\n",
        ));
        assert!(!doc.has_comments(&k, false).unwrap());
        assert!(doc.has_comments(&k, true).unwrap());
        // Comments above a header are moved on removal, so they don't count.
        let doc = parse(Some(&format!("# above\n{plain}")));
        assert!(!doc.has_comments(&k, true).unwrap());
    }

    #[test]
    fn a_header_the_user_added_stays() {
        let k = key("/mcp_servers/docs-index");
        let mut doc = parse(Some("model = \"o3\"\n"));
        assert_eq!(doc.set(&k, &mine()).unwrap(), ["/mcp_servers"]);
        let written = rendered(&doc).replace(
            "\n[mcp_servers.docs-index]",
            "\n# ---- MCP servers ----\n[mcp_servers] # see wiki\n\n[mcp_servers.docs-index]",
        );
        let mut doc = parse(Some(&written));
        doc.remove(&k).unwrap();
        assert_eq!(
            doc.prune(&["/mcp_servers".to_owned()]).unwrap(),
            ["/mcp_servers"],
            "kept, though empty"
        );
        assert_eq!(
            rendered(&doc),
            "model = \"o3\"\n\n# ---- MCP servers ----\n[mcp_servers] # see wiki\n"
        );
    }

    #[test]
    fn replacing_keeps_place_and_comments() {
        let user = "# my servers\n[mcp_servers.docs-index] # managed by grimoire\ncommand = \"old\"\n\n[tail]\nz = 2\n";
        let mut doc = parse(Some(user));
        let k = key("/mcp_servers/docs-index");
        assert_eq!(
            doc.find(&k).unwrap(),
            Some(value_hash(&json!({"command": "old"})))
        );
        assert_eq!(doc.set(&k, &mine()).unwrap(), Vec::<String>::new());
        assert_eq!(
            rendered(&doc),
            "# my servers\n[mcp_servers.docs-index] # managed by grimoire\ncommand = \"grimoire-mcp-x\"\nenv_vars = [\"A\", \"B\"]\n\n[tail]\nz = 2\n"
        );
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&mine())));
        // A plain value keeps its trailing comment.
        let mut doc = parse(Some("[mcp_servers]\ntimeout = 5 # seconds\n"));
        doc.set(&key("/mcp_servers/timeout"), &json!(10)).unwrap();
        assert_eq!(rendered(&doc), "[mcp_servers]\ntimeout = 10 # seconds\n");
    }

    #[test]
    fn an_inline_table_parent_gets_an_inline_entry() {
        let user = "mcp_servers = { theirs = { command = \"x\" } }\nmodel = \"o3\"\n";
        let mut doc = parse(Some(user));
        let k = key("/mcp_servers/docs-index");
        assert_eq!(doc.set(&k, &mine()).unwrap(), Vec::<String>::new());
        let written = rendered(&doc);
        assert!(
            written.contains(
                "docs-index = { command = \"grimoire-mcp-x\", env_vars = [\"A\", \"B\"] } }"
            ),
            "{written}"
        );
        assert!(!written.contains("[mcp_servers"), "{written}");
        let mut doc = parse(Some(&written));
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&mine())));
        doc.remove(&k).unwrap();
        assert_eq!(rendered(&doc), user);
        // Tables created inside an inline table are inline too.
        let mut doc = parse(Some("a = {}\n"));
        assert_eq!(doc.set(&key("/a/b/c"), &json!({"d": 1})).unwrap(), ["/a/b"]);
        let written = rendered(&doc);
        assert_eq!(written, "a = { b = { c = { d = 1 } } }\n");
        let mut doc = parse(Some(&written));
        doc.remove(&key("/a/b/c")).unwrap();
        doc.prune(&["/a/b".to_owned()]).unwrap();
        assert_eq!(rendered(&doc), "a = {}\n");
    }

    #[test]
    fn a_dotted_parent_gets_dotted_keys() {
        let user = "mcp_servers.theirs.command = \"x\"\nmodel = \"o3\"\n\n[tail]\nz = 2\n";
        let mut doc = parse(Some(user));
        let k = key("/mcp_servers/docs-index");
        assert_eq!(doc.set(&k, &mine()).unwrap(), Vec::<String>::new());
        let written = rendered(&doc);
        assert_eq!(
            written,
            "mcp_servers.theirs.command = \"x\"\nmcp_servers.docs-index.command = \"grimoire-mcp-x\"\nmcp_servers.docs-index.env_vars = [\"A\", \"B\"]\nmodel = \"o3\"\n\n[tail]\nz = 2\n"
        );
        let mut doc = parse(Some(&written));
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&mine())));
        doc.remove(&k).unwrap();
        assert_eq!(rendered(&doc), user);
        // An empty object as dotted keys would leave no text, so it is inline.
        for start in [user, written.as_str()] {
            let mut doc = parse(Some(start));
            doc.set(&k, &json!({})).unwrap();
            let written = rendered(&doc);
            assert!(
                written.contains("mcp_servers.docs-index = {}\n"),
                "{written}"
            );
            let doc = parse(Some(&written));
            assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&json!({}))));
        }
    }

    #[test]
    fn hashes_survive_the_round_trip() {
        let value = json!({
            "command": "grimoire-mcp-x",
            "startup_timeout_sec": 20,
            "ratio": 1.5,
            "whole": 2.0,
            "negative": -7,
            "enabled": true,
            "env": {"A": "1", "nested": {"deep": [1, "two", false]}},
            "args": [],
            "headers": [{"name": "x"}, {"name": "y"}],
        });
        let k = key("/mcp_servers/docs-index");
        let mut doc = parse(None);
        doc.set(&k, &value).unwrap();
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&value)));
        let written = rendered(&doc);
        assert!(written.contains("startup_timeout_sec = 20\n"), "{written}");
        assert!(written.contains("whole = 2.0\n"), "{written}");
        let doc = parse(Some(&written));
        assert_eq!(doc.find(&k).unwrap(), Some(value_hash(&value)));
        // An integer is not a float, on either side.
        let ints = parse(Some("[t]\nn = 20\n"));
        let floats = parse(Some("[t]\nn = 20.0\n"));
        assert_eq!(
            ints.find(&key("/t/n")).unwrap(),
            Some(value_hash(&json!(20)))
        );
        assert_eq!(
            floats.find(&key("/t/n")).unwrap(),
            Some(value_hash(&json!(20.0)))
        );
        assert_ne!(value_hash(&json!(20)), value_hash(&json!(20.0)));
        // An array of tables reads back as an array of objects.
        let aot = parse(Some("[[t.list]]\na = 1\n[[t.list]]\na = 2\n"));
        assert_eq!(
            aot.find(&key("/t/list")).unwrap(),
            Some(value_hash(&json!([{"a": 1}, {"a": 2}])))
        );
    }

    #[test]
    fn values_without_a_json_form_never_match() {
        let doc = parse(Some(
            "[mcp_servers.x]\nwhen = 1979-05-27T07:32:00Z\n[mcp_servers.y]\nn = nan\n",
        ));
        let x = doc.find(&key("/mcp_servers/x")).unwrap().unwrap();
        assert_ne!(x, value_hash(&json!({"when": "1979-05-27T07:32:00Z"})));
        let y = doc.find(&key("/mcp_servers/y")).unwrap().unwrap();
        assert_ne!(x, y);
    }

    #[test]
    fn refuses_values_toml_cannot_hold() {
        let k = key("/mcp_servers/docs-index");
        for (bad, at) in [
            (json!(null), "`/mcp_servers/docs-index` is null"),
            (
                json!({"env": {"A": null}}),
                "`/mcp_servers/docs-index/env/A` is null",
            ),
            (
                json!({"args": ["a", null]}),
                "`/mcp_servers/docs-index/args/1` is null",
            ),
            (
                json!({"n": u64::MAX}),
                "`/mcp_servers/docs-index/n` is 18446744073709551615",
            ),
        ] {
            let mut doc = parse(Some(USER));
            let e = doc.set(&k, &bad).unwrap_err().to_string();
            assert!(e.starts_with(at), "{e}");
            assert_eq!(rendered(&doc), USER, "nothing changed");
        }
    }

    #[test]
    fn refuses_array_elements() {
        let k = element_key(&["hooks", "PreToolUse"], "mine#0").unwrap();
        let mut doc = parse(None);
        for e in [
            doc.find(&k).unwrap_err(),
            doc.set(&k, &json!({"a": 1})).unwrap_err(),
            doc.remove(&k).unwrap_err(),
        ] {
            assert!(e.to_string().contains("array element"), "{e}");
        }
    }

    #[test]
    fn refuses_files_it_cannot_edit_safely() {
        for (bad, why) in [
            (
                "a = \n",
                "it is not valid TOML (string values must be quoted",
            ),
            (
                "a = 1\na = 2\n",
                "it is not valid TOML (duplicate key at line 2, column 1)",
            ),
            (
                "[a]\nx = 1\n[a]\ny = 2\n",
                "it is not valid TOML (duplicate key at line 3",
            ),
            ("[mcp_servers\n", "it is not valid TOML ("),
            (
                "a.b = 1\n[a]\nc = 1\n",
                "it is not valid TOML (duplicate key",
            ),
        ] {
            let e = TomlDoc::parse(Some(bad)).err().unwrap().to_string();
            assert!(e.starts_with(why), "{bad:?}: {e}");
        }
        let k = key("/mcp_servers/docs-index");
        for not_a_table in [
            "mcp_servers = 1\n",
            "mcp_servers = [1]\n",
            "mcp_servers = \"x\"\n",
            "[[mcp_servers]]\nx = 1\n",
        ] {
            let mut doc = parse(Some(not_a_table));
            for e in [doc.find(&k).unwrap_err(), doc.set(&k, &mine()).unwrap_err()] {
                assert_eq!(
                    e.to_string(),
                    "`/mcp_servers` is not a TOML table",
                    "{not_a_table:?}"
                );
            }
            assert_eq!(rendered(&doc), not_a_table, "nothing changed");
        }
        let mut deep = parse(Some("[a]\nb = 1\n"));
        assert_eq!(
            deep.set(&key("/a/b/c"), &json!(1)).unwrap_err().to_string(),
            "`/a/b` is not a TOML table"
        );
        assert!(
            deep.find(&key("/a/b")).unwrap().is_some(),
            "the value itself may be anything"
        );
    }
}
