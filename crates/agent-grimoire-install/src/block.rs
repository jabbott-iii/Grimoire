//! Managed blocks in shared Markdown files such as `AGENTS.md`.
//!
//! The design is in
//! [`docs/subsystems/install.md`, Ownership units and merge strategies](https://github.com/jabbott-iii/Grimoire/blob/main/docs/subsystems/install.md#ownership-units-and-merge-strategies).
//!
//! A block is the text between two marker lines:
//!
//! ```text
//! <!-- grimoire:begin rust-errors -->
//! ...owned content...
//! <!-- grimoire:end rust-errors -->
//! ```
//!
//! Everything outside blocks belongs to the user and is preserved byte for byte.
//! Markers inside fenced code blocks of the user's text are ignored, so a file
//! that documents Grimoire's markers is not misread. Any other line starting with
//! `<!-- grimoire:` that is not a well-formed marker, an unterminated or nested
//! block, or a repeated name makes the file unparseable: Grimoire then refuses to
//! touch it rather than guess what it owns.

use std::fmt::{self, Write as _};

/// The prefix shared by both marker kinds.
pub const MARKER_PREFIX: &str = "<!-- grimoire:";
const BEGIN: &str = "<!-- grimoire:begin ";
const END: &str = "<!-- grimoire:end ";
const CLOSE: &str = " -->";

/// Why a file's managed blocks cannot be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockError {
    /// 1-based line number.
    pub line: usize,
    /// What is wrong.
    pub reason: String,
}

impl fmt::Display for BlockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}: {}", self.line, self.reason)
    }
}

impl std::error::Error for BlockError {}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Segment {
    /// User text, kept verbatim.
    Text(String),
    /// A managed block: its name and the body between the marker lines.
    Block { name: String, body: String },
}

/// A Markdown document split into user text and managed blocks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    segments: Vec<Segment>,
    /// The line ending used for lines Grimoire writes: `\r\n` when the file already uses it.
    newline: &'static str,
    /// The user's text ends inside an unclosed code fence, which would swallow an appended block.
    open_fence: bool,
    /// The file started with a UTF-8 byte order mark, which is written back.
    bom: bool,
}

/// A block body in the form Grimoire stores and hashes: `\n` line endings and
/// exactly one trailing newline.
#[must_use]
pub fn normalize_body(body: &str) -> String {
    let lf = body.replace("\r\n", "\n");
    let mut out = lf.trim_end_matches('\n').to_owned();
    out.push('\n');
    out
}

/// What a marker line is.
enum Marker<'a> {
    Begin(&'a str),
    End(&'a str),
}

/// Parses a marker line (without its line ending). `Ok(None)` means "not a marker".
fn marker(line: &str) -> Result<Option<Marker<'_>>, String> {
    // Markers count only at column 0, where Grimoire writes them: an indented
    // marker-like line is user text, such as an example in an indented code block.
    let line = line.trim_end();
    if !line.starts_with(MARKER_PREFIX) {
        return Ok(None);
    }
    let parsed = if let Some(rest) = line.strip_prefix(BEGIN) {
        rest.strip_suffix(CLOSE).map(Marker::Begin)
    } else if let Some(rest) = line.strip_prefix(END) {
        rest.strip_suffix(CLOSE).map(Marker::End)
    } else {
        None
    };
    match parsed {
        Some(Marker::Begin(name) | Marker::End(name)) if !is_name(name) => {
            Err(format!("`{line}` has an invalid block name"))
        }
        Some(m) => Ok(Some(m)),
        None => Err(format!(
            "`{line}` looks like a Grimoire marker but is not `<!-- grimoire:begin NAME -->` or `<!-- grimoire:end NAME -->`"
        )),
    }
}

/// Block names are asset ids, or Grimoire's reserved owner name.
fn is_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= 64
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !name.starts_with('-')
        && !name.ends_with('-')
        && !name.contains("--")
}

/// Tracks `CommonMark` fenced code blocks in user text.
#[derive(Default)]
struct Fence {
    open: Option<(char, usize)>,
}

impl Fence {
    /// Updates the state for `line` and reports whether the line is inside (or opens or closes) a fence.
    fn step(&mut self, line: &str) -> bool {
        let indent = line.len() - line.trim_start_matches(' ').len();
        let rest = &line[indent..];
        let fence = (indent <= 3)
            .then(|| rest.chars().next())
            .flatten()
            .filter(|c| *c == '`' || *c == '~')
            .map(|c| (c, rest.chars().take_while(|x| *x == c).count()))
            .filter(|(_, n)| *n >= 3);
        match (self.open, fence) {
            // CommonMark: a backtick fence's info string cannot contain a backtick,
            // so a line such as "```` ``` ```` text" is a code span, not a fence.
            (None, Some(('`', n))) if rest[n..].contains('`') => false,
            (None, Some(f)) => {
                self.open = Some(f);
                true
            }
            (Some((c, n)), Some((c2, n2)))
                if c == c2 && n2 >= n && rest.trim_end().chars().all(|x| x == c) =>
            {
                self.open = None;
                true
            }
            (Some(_), _) => true,
            (None, None) => false,
        }
    }
}

impl Document {
    /// Splits `text` into user text and managed blocks.
    ///
    /// # Errors
    ///
    /// Fails on malformed, unterminated, nested, mismatched or repeated markers.
    pub fn parse(text: &str) -> Result<Self, BlockError> {
        let (bom, text) = match text.strip_prefix('\u{feff}') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        let newline = if text.contains("\r\n") { "\r\n" } else { "\n" };
        let mut segments = Vec::new();
        let mut user = String::new();
        let mut open: Option<(String, String, usize)> = None;
        let mut fence = Fence::default();
        let mut names = std::collections::BTreeSet::new();
        for (index, raw) in text.split_inclusive('\n').enumerate() {
            let line_no = index + 1;
            let line = raw.trim_end_matches(['\n', '\r']);
            if let Some((name, body, start)) = &mut open {
                match marker(line).map_err(|reason| BlockError {
                    line: line_no,
                    reason,
                })? {
                    None => body.push_str(raw),
                    Some(Marker::End(end)) if end == name => {
                        segments.push(Segment::Block {
                            name: std::mem::take(name),
                            body: std::mem::take(body),
                        });
                        open = None;
                    }
                    Some(Marker::End(end)) => {
                        return Err(BlockError {
                            line: line_no,
                            reason: format!(
                                "`{end}` ends a block, but `{name}` (line {start}) is open"
                            ),
                        });
                    }
                    Some(Marker::Begin(_)) => {
                        return Err(BlockError {
                            line: line_no,
                            reason: format!("a block starts inside block `{name}` (line {start})"),
                        });
                    }
                }
                continue;
            }
            if fence.step(line) {
                user.push_str(raw);
                continue;
            }
            match marker(line).map_err(|reason| BlockError {
                line: line_no,
                reason,
            })? {
                None => user.push_str(raw),
                Some(Marker::Begin(name)) => {
                    if !names.insert(name.to_owned()) {
                        return Err(BlockError {
                            line: line_no,
                            reason: format!("block `{name}` appears more than once"),
                        });
                    }
                    if !user.is_empty() {
                        segments.push(Segment::Text(std::mem::take(&mut user)));
                    }
                    open = Some((name.to_owned(), String::new(), line_no));
                }
                Some(Marker::End(name)) => {
                    return Err(BlockError {
                        line: line_no,
                        reason: format!("block `{name}` ends but never began"),
                    });
                }
            }
        }
        if let Some((name, _, start)) = open {
            return Err(BlockError {
                line: start,
                reason: format!("block `{name}` is never closed"),
            });
        }
        if !user.is_empty() {
            segments.push(Segment::Text(user));
        }
        Ok(Self {
            segments,
            newline,
            open_fence: fence.open.is_some(),
            bom,
        })
    }

    /// The body of block `name`, if present.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&str> {
        self.segments.iter().find_map(|s| match s {
            Segment::Block { name: n, body } if n == name => Some(body.as_str()),
            _ => None,
        })
    }

    /// The names of every block, in file order.
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        self.segments
            .iter()
            .filter_map(|s| match s {
                Segment::Block { name, .. } => Some(name.as_str()),
                Segment::Text(_) => None,
            })
            .collect()
    }

    /// Sets block `name` to `body`, which is normalized to end with one newline.
    ///
    /// An existing block is replaced where it is. A new block goes just before the
    /// first block whose name sorts after it, else just after the last block, else
    /// at the end of the file after a blank line.
    ///
    /// # Errors
    ///
    /// Refuses to append to a file whose text ends inside an unclosed code fence:
    /// the block would become part of the code block.
    pub fn set(&mut self, name: &str, body: &str) -> Result<(), BlockError> {
        let body = self.normalize(body);
        if let Some(Segment::Block { body: existing, .. }) = self
            .segments
            .iter_mut()
            .find(|s| matches!(s, Segment::Block { name: n, .. } if n == name))
        {
            *existing = body;
            return Ok(());
        }
        let block = Segment::Block {
            name: name.to_owned(),
            body,
        };
        let blocks: Vec<(usize, &str)> = self
            .segments
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                Segment::Block { name, .. } => Some((i, name.as_str())),
                Segment::Text(_) => None,
            })
            .collect();
        if let Some((index, _)) = blocks.iter().find(|(_, n)| *n > name) {
            self.segments.insert(*index, block);
        } else if let Some((index, _)) = blocks.last() {
            self.segments.insert(index + 1, block);
        } else {
            if self.open_fence {
                return Err(BlockError {
                    line: self.render().lines().count(),
                    reason: "the file ends inside an unclosed code fence; close it so Grimoire can add its block after it".to_owned(),
                });
            }
            let nl = self.newline;
            match self.segments.last_mut() {
                Some(Segment::Text(text)) if !text.is_empty() => {
                    if !text.ends_with('\n') {
                        text.push_str(nl);
                    }
                    text.push_str(nl);
                }
                _ => {}
            }
            self.segments.push(block);
        }
        Ok(())
    }

    /// Removes block `name`. Returns whether it existed.
    ///
    /// The blank line that [`Document::set`] put before a block appended at the end
    /// of the file is removed with it, so adding then removing a block restores the
    /// original text.
    pub fn remove(&mut self, name: &str) -> bool {
        let Some(index) = self
            .segments
            .iter()
            .position(|s| matches!(s, Segment::Block { name: n, .. } if n == name))
        else {
            return false;
        };
        self.segments.remove(index);
        let at_end = index == self.segments.len();
        if at_end
            && index > 0
            && let Some(Segment::Text(text)) = self.segments.get_mut(index - 1)
        {
            let nl = self.newline;
            let doubled = format!("{nl}{nl}");
            if text.ends_with(&doubled) {
                text.truncate(text.len() - nl.len());
            } else if text == nl {
                text.clear();
            }
            if text.is_empty() {
                self.segments.remove(index - 1);
            }
        }
        true
    }

    /// Whether the document holds nothing but whitespace.
    #[must_use]
    pub fn is_blank(&self) -> bool {
        self.segments
            .iter()
            .all(|s| matches!(s, Segment::Text(t) if t.trim().is_empty()))
    }

    /// The document as text.
    #[must_use]
    pub fn render(&self) -> String {
        let nl = self.newline;
        let mut out = String::new();
        if self.bom {
            out.push('\u{feff}');
        }
        for segment in &self.segments {
            match segment {
                Segment::Text(text) => out.push_str(text),
                Segment::Block { name, body } => {
                    if !out.is_empty() && !out.ends_with('\n') && out != "\u{feff}" {
                        out.push_str(nl);
                    }
                    let _ = write!(out, "{BEGIN}{name}{CLOSE}{nl}");
                    out.push_str(body);
                    let _ = write!(out, "{END}{name}{CLOSE}{nl}");
                }
            }
        }
        out
    }

    /// Converts `body` to this document's line ending and makes it end with exactly one.
    fn normalize(&self, body: &str) -> String {
        let normalized = normalize_body(body);
        if self.newline == "\n" {
            normalized
        } else {
            normalized.replace('\n', self.newline)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_blocks_and_user_text() {
        let text = "# Mine\n\n<!-- grimoire:begin a -->\nA\n<!-- grimoire:end a -->\nafter\n";
        let doc = Document::parse(text).unwrap();
        assert_eq!(doc.get("a"), Some("A\n"));
        assert_eq!(doc.names(), ["a"]);
        assert_eq!(doc.render(), text);
    }

    #[test]
    fn inserts_in_name_order_and_appends_after_a_blank_line() {
        let mut doc = Document::parse("# Mine\n").unwrap();
        doc.set("m", "M").unwrap();
        assert_eq!(
            doc.render(),
            "# Mine\n\n<!-- grimoire:begin m -->\nM\n<!-- grimoire:end m -->\n"
        );
        doc.set("z", "Z\n\n").unwrap();
        doc.set("a", "A").unwrap();
        assert_eq!(doc.names(), ["a", "m", "z"]);
        doc.set("m", "M2").unwrap();
        assert_eq!(doc.get("m"), Some("M2\n"));
        assert!(doc.remove("a") && doc.remove("m") && doc.remove("z"));
        assert!(!doc.remove("z"));
        assert_eq!(doc.render(), "# Mine\n");
    }

    #[test]
    fn crlf_files_get_crlf_blocks() {
        let mut doc = Document::parse("one\r\ntwo\r\n").unwrap();
        doc.set("a", "x\ny").unwrap();
        assert_eq!(
            doc.render(),
            "one\r\ntwo\r\n\r\n<!-- grimoire:begin a -->\r\nx\r\ny\r\n<!-- grimoire:end a -->\r\n"
        );
    }

    #[test]
    fn empty_file_round_trips() {
        let mut doc = Document::parse("").unwrap();
        doc.set("a", "A").unwrap();
        assert_eq!(
            doc.render(),
            "<!-- grimoire:begin a -->\nA\n<!-- grimoire:end a -->\n"
        );
        doc.remove("a");
        assert_eq!(doc.render(), "");
        assert!(doc.is_blank());
    }

    #[test]
    fn bodies_normalize_to_one_trailing_newline() {
        assert_eq!(normalize_body("X"), "X\n");
        assert_eq!(normalize_body("X\r\n\r\n\n"), "X\n");
        let mut doc = Document::parse("").unwrap();
        doc.set("a", "X\n\n").unwrap();
        assert_eq!(doc.get("a"), Some(normalize_body("X\n\n").as_str()));
    }

    #[test]
    fn indented_markers_and_backtick_spans_are_user_text() {
        let text = "Example:\n\n    <!-- grimoire:begin example -->\n    Rule.\n    <!-- grimoire:end example -->\n";
        let doc = Document::parse(text).unwrap();
        assert_eq!(doc.names(), Vec::<&str>::new());
        assert_eq!(doc.render(), text);
        let span =
            "Write ```` ``` ```` fences.\n<!-- grimoire:begin a -->\nA\n<!-- grimoire:end a -->\n";
        assert_eq!(Document::parse(span).unwrap().names(), ["a"]);
    }

    #[test]
    fn a_byte_order_mark_is_kept() {
        let text = "\u{feff}<!-- grimoire:begin a -->\nA\n<!-- grimoire:end a -->\n";
        let mut doc = Document::parse(text).unwrap();
        assert_eq!(doc.get("a"), Some("A\n"));
        assert_eq!(doc.render(), text);
        doc.set("b", "B").unwrap();
        assert!(doc.render().starts_with('\u{feff}'));
    }

    #[test]
    fn markers_in_fences_are_user_text() {
        let text = "```md\n<!-- grimoire:begin a -->\n```\n~~~~\n<!-- grimoire:typo -->\n~~~~\n";
        let doc = Document::parse(text).unwrap();
        assert_eq!(doc.names(), Vec::<&str>::new());
        assert_eq!(doc.render(), text);
    }

    #[test]
    fn refuses_to_append_inside_an_open_fence() {
        let mut doc = Document::parse("intro\n```\ncode\n").unwrap();
        assert!(doc.set("a", "A").is_err());
        assert_eq!(doc.render(), "intro\n```\ncode\n");
    }

    #[test]
    fn malformed_markers_are_refused() {
        for bad in [
            "<!-- grimoire:begin a -->\nno end\n",
            "<!-- grimoire:end a -->\n",
            "<!-- grimoire:begin a -->\n<!-- grimoire:begin b -->\n",
            "<!-- grimoire:begin a -->\n<!-- grimoire:end b -->\n",
            "<!-- grimoire:begin a -->\n<!-- grimoire:end a -->\n<!-- grimoire:begin a -->\n<!-- grimoire:end a -->\n",
            "<!-- grimoire:begin Bad_Name -->\n<!-- grimoire:end Bad_Name -->\n",
            "<!-- grimoire:start a -->\n",
        ] {
            assert!(Document::parse(bad).is_err(), "{bad:?}");
        }
    }
}
