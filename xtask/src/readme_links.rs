//! The root `README.md` may link only to `https://` URLs, `mailto:` addresses
//! and in-page `#fragment`s.
//!
//! The same file is the `agent-grimoire` crate's page on crates.io, which
//! resolves a relative link against `crates/agent-grimoire/` on the default
//! branch; the long description of the PyPI wheels, which rewrites no links;
//! and a plain file in every CLI release archive, which holds no `docs/`
//! (README-RENDER-P7 in `docs/specs.md`). A relative link works on GitHub and
//! breaks in all three. The library crates' own READMEs link their
//! `LICENSE-*` files relatively on purpose, so `crate_docs` checks those
//! READMEs with that exception, through [`link_targets`] and [`allowed`].
//!
//! The README is parsed with pulldown-cmark, as the catalog's Markdown is, with
//! GitHub's extensions on. Every link and image counts, including reference
//! links and autolinks; an email autolink, `<x@example.com>`, counts as its
//! `mailto:` address. So do `href`, `src` and `srcset` attributes in raw HTML,
//! and bare `http://` and `www.` URLs that GitHub turns into links: anywhere in
//! a run of text where one starts the run or follows whitespace, `*`, `_`, `~`
//! or `(` (the "Autolinks (extension)" section of the GFM spec, 0.29-gfm,
//! <https://github.github.com/gfm/>). Every `<!--` in raw HTML starts a
//! comment, inside another tag's HTML too. Code spans and code blocks don't
//! count.

use pulldown_cmark::{Event, LinkType, Options, Parser, Tag, TagEnd};

use crate::release::plan::{before_first_release, is_marker};

/// Whether the root README may link to `target`.
pub(crate) fn allowed(target: &str) -> bool {
    target.starts_with('#') || target.starts_with("https://") || target.starts_with("mailto:")
}

/// What the README holds outside code: `(line, target)` for every link target,
/// and `(line, text)` for every HTML comment.
#[derive(Debug, Default)]
struct Scan {
    links: Vec<(usize, String)>,
    comments: Vec<(usize, String)>,
}

/// `(line, target)` for every link target in `markdown`, counted as the module
/// docs say.
pub(crate) fn link_targets(markdown: &str) -> Vec<(usize, String)> {
    scan(markdown).links
}

fn scan(markdown: &str) -> Scan {
    let line = |offset: usize| markdown[..offset].matches('\n').count() + 1;
    let options = Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_GFM;
    let mut found = Scan::default();
    let mut in_code = false;
    let mut in_link = 0_usize;
    // pulldown-cmark splits a run of text at `_`, `*` and other delimiters, so
    // the pieces are joined again before bare URLs are looked for.
    let mut run: Option<(usize, String)> = None;
    for (event, range) in Parser::new_ext(markdown, options).into_offset_iter() {
        let at = line(range.start);
        // A link's own text, such as an autolink's, is already counted.
        if let Event::Text(text) = &event
            && !in_code
            && in_link == 0
        {
            run.get_or_insert_with(|| (at, String::new()))
                .1
                .push_str(text);
            continue;
        }
        end_run(&mut run, &mut found.links);
        match event {
            // pulldown-cmark leaves out the `mailto:` that GitHub links to.
            Event::Start(Tag::Link {
                link_type: LinkType::Email,
                dest_url,
                ..
            }) => {
                in_link += 1;
                found.links.push((at, format!("mailto:{dest_url}")));
            }
            Event::Start(Tag::Link { dest_url, .. } | Tag::Image { dest_url, .. }) => {
                in_link += 1;
                found.links.push((at, dest_url.into_string()));
            }
            Event::End(TagEnd::Link | TagEnd::Image) => in_link = in_link.saturating_sub(1),
            Event::Start(Tag::CodeBlock(_)) => in_code = true,
            Event::End(TagEnd::CodeBlock) => in_code = false,
            Event::Html(html) | Event::InlineHtml(html) => {
                let (comments, rest) = split_comments(&html);
                for (offset, comment) in comments {
                    let lines = html[..offset].matches('\n').count();
                    found.comments.push((at + lines, comment));
                }
                for target in attributes(&rest) {
                    found.links.push((at, target));
                }
            }
            _ => {}
        }
    }
    end_run(&mut run, &mut found.links);
    found
}

/// Adds the bare URLs in `run`, a run of text and its line, to `links`.
fn end_run(run: &mut Option<(usize, String)>, links: &mut Vec<(usize, String)>) {
    if let Some((line, text)) = run.take() {
        links.extend(bare_urls(&text).into_iter().map(|url| (line, url)));
    }
}

/// The comments in a piece of raw HTML, each with its byte offset, and the HTML
/// without them. A comment runs from `<!--` to the next `-->` (`<!-->` and
/// `<!--->` are whole comments), or to the end of the piece.
fn split_comments(html: &str) -> (Vec<(usize, String)>, String) {
    let mut comments = Vec::new();
    let mut rest = String::new();
    let mut from = 0;
    while let Some(start) = html[from..].find("<!--").map(|i| i + from) {
        rest.push_str(&html[from..start]);
        let end = html[start + 2..]
            .find("-->")
            .map_or(html.len(), |i| start + 2 + i + 3);
        comments.push((start, html[start..end].trim_end().to_owned()));
        from = end;
    }
    rest.push_str(&html[from..]);
    (comments, rest)
}

/// The bare `http://` and `www.` URLs in a run of text that GitHub would link:
/// those that start the run or follow whitespace, `*`, `_`, `~` or `(`. Each
/// ends at whitespace or `<`, without trailing punctuation or an unmatched `)`.
fn bare_urls(text: &str) -> Vec<String> {
    let mut starts: Vec<usize> = text
        .match_indices("http://")
        .chain(text.match_indices("www."))
        .map(|(i, _)| i)
        .filter(|&i| {
            text[..i]
                .chars()
                .next_back()
                .is_none_or(|c| c.is_whitespace() || matches!(c, '*' | '_' | '~' | '('))
        })
        .collect();
    starts.sort_unstable();
    starts
        .into_iter()
        .map(|i| {
            let end = text[i..]
                .find(|c: char| c.is_whitespace() || c == '<')
                .map_or(text.len(), |e| i + e);
            let mut url = &text[i..end];
            loop {
                let trimmed = url.trim_end_matches(['?', '!', '.', ',', ':', '*', '_', '~']);
                url = match trimmed.strip_suffix(')') {
                    Some(shorter)
                        if trimmed.matches(')').count() > trimmed.matches('(').count() =>
                    {
                        shorter
                    }
                    _ => break trimmed.to_owned(),
                };
            }
        })
        .collect()
}

/// The values of `href` and `src` attributes in a piece of raw HTML, and the
/// URLs in `srcset` attributes.
fn attributes(html: &str) -> Vec<String> {
    let lower = html.to_ascii_lowercase();
    let mut values = Vec::new();
    for name in ["href", "src", "srcset"] {
        let mut from = 0;
        while let Some(i) = lower[from..].find(name).map(|i| i + from) {
            from = i + name.len();
            // An attribute name follows whitespace, so `data-href` doesn't count.
            let before_ok = i > 0 && lower.as_bytes()[i - 1].is_ascii_whitespace();
            let rest = html[from..].trim_start();
            if !before_ok || !rest.starts_with('=') {
                continue;
            }
            let rest = rest[1..].trim_start();
            let value = match rest.chars().next() {
                Some(q @ ('"' | '\'')) => rest[1..].split(q).next().unwrap_or_default(),
                _ => rest
                    .split(|c: char| c.is_whitespace() || c == '>')
                    .next()
                    .unwrap_or_default(),
            };
            if name == "srcset" {
                // Each candidate is a URL, then an optional `1x` or `100w`.
                values.extend(
                    value
                        .split(',')
                        .filter_map(|c| c.split_whitespace().next())
                        .map(str::to_owned),
                );
            } else {
                values.push(value.to_owned());
            }
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    /// `(line, target)` for each target the root README may not use.
    fn bad(markdown: &str) -> Vec<(usize, String)> {
        scan(markdown)
            .links
            .into_iter()
            .filter(|(_, t)| !allowed(t))
            .collect()
    }

    fn one(line: usize, target: &str) -> Vec<(usize, String)> {
        vec![(line, target.to_owned())]
    }

    #[test]
    fn allows_https_mailto_and_fragments() {
        let text = "[a](https://example.com/x) [b](#install) [c](mailto:x@example.com)\n\
                    <https://example.com> ![d](https://example.com/d.png)\n";
        assert_eq!(bad(text), []);
        assert_eq!(scan(text).links.len(), 5);
    }

    #[test]
    fn finds_relative_links_images_and_references() {
        assert_eq!(bad("See [the CLI](docs/cli.md)."), one(1, "docs/cli.md"));
        assert_eq!(bad("x\n\n![logo](logo.png)"), one(3, "logo.png"));
        assert_eq!(bad("[a](/docs/a.md)"), one(1, "/docs/a.md"));
        assert_eq!(bad("[a](../a.md \"title\")"), one(1, "../a.md"));
        assert_eq!(
            bad("[cli][ref]\n\n[ref]: docs/cli.md\n"),
            one(1, "docs/cli.md")
        );
        assert_eq!(
            bad("[ref]\n\n[ref]: <docs/x y.md>\n"),
            one(1, "docs/x y.md")
        );
        assert_eq!(bad("<http://example.com>"), one(1, "http://example.com"));
        assert_eq!(bad("| a |\n|---|\n| [b](b.md) |\n"), one(3, "b.md"));
        assert_eq!(bad("> [!NOTE]\n> [b](b.md)\n"), one(2, "b.md"));
    }

    #[test]
    fn finds_html_attributes_and_bare_urls() {
        assert_eq!(bad("<a href=\"docs/a.md\">a</a>"), one(1, "docs/a.md"));
        assert_eq!(bad("<p>\n<img src='x.png'>\n</p>\n"), one(2, "x.png"));
        assert_eq!(bad("<a HREF=x.md>x</a>"), one(1, "x.md"));
        assert_eq!(bad("<a data-href=\"x\" href=\"https://e.com\">"), []);
        assert_eq!(
            bad("Go to http://example.com now"),
            one(1, "http://example.com")
        );
        assert_eq!(bad("or www.example.com"), one(1, "www.example.com"));
    }

    #[test]
    fn skips_code_and_collects_comments() {
        assert_eq!(bad("`[a](docs/cli.md)` and `<a href=\"x\">`"), []);
        assert_eq!(
            bad("```bash\ncat [a](a.md) http://x\n```\n[b](b.md)\n"),
            one(4, "b.md")
        );
        assert_eq!(bad("    [a](a.md)\n"), []);
        assert_eq!(bad("<!-- [a](a.md) -->\n\n[c](c.md)"), one(3, "c.md"));
        let comments = scan("a <!-- one -->\n\n<!-- two -->\n\n`<!-- code -->`\n").comments;
        assert_eq!(
            comments,
            [
                (1, "<!-- one -->".to_owned()),
                (3, "<!-- two -->".to_owned())
            ]
        );
    }

    #[test]
    fn finds_bare_urls_at_githubs_autolink_boundaries() {
        assert_eq!(
            bad("See the site (http://example.com) now."),
            one(1, "http://example.com")
        );
        assert_eq!(bad("*www.example.com*"), one(1, "www.example.com"));
        assert_eq!(
            bad("Read http://example.com/a_(b). Then (www.b.example)."),
            [
                (1, "http://example.com/a_(b)".to_owned()),
                (1, "www.b.example".to_owned())
            ]
        );
        assert_eq!(
            bad("http://www.example.com"),
            one(1, "http://www.example.com")
        );
        // GitHub links none of these.
        assert_eq!(bad("a,http://example.com xwww.example.com"), []);
    }

    #[test]
    fn reads_srcset_and_email_autolinks() {
        assert_eq!(
            bad("<img srcset=\"logo.png 1x, https://e.com/l.png 2x\">"),
            one(1, "logo.png")
        );
        assert_eq!(
            bad("<picture>\n<source srcset=\"dark.png\">\n</picture>\n"),
            one(2, "dark.png")
        );
        let text = "Mail <security@example.com>.";
        assert_eq!(bad(text), []);
        assert_eq!(scan(text).links, one(1, "mailto:security@example.com"));
    }

    #[test]
    fn finds_comments_inside_other_html() {
        let comments = |text: &str| scan(text).comments;
        assert_eq!(
            comments("<p><!-- hidden note --></p>"),
            one(1, "<!-- hidden note -->")
        );
        assert_eq!(
            comments("x\n\n<details><summary>s</summary><!-- x --></details>\n"),
            one(3, "<!-- x -->")
        );
        // A link inside a comment doesn't count, and `<!-->` is a whole comment.
        let text = "<p><!-- <a href=\"a.md\"> --><!--> <a href=\"b.md\">b</a></p>";
        assert_eq!(bad(text), one(1, "b.md"));
        assert_eq!(
            comments(text),
            [
                (1, "<!-- <a href=\"a.md\"> -->".to_owned()),
                (1, "<!-->".to_owned())
            ]
        );
    }

    #[test]
    fn reports_lines_with_crlf() {
        let text = "Prose.\r\n\r\n[a](café.md) and [b](b.md)\r\n";
        assert_eq!(
            bad(text),
            [(3, "café.md".to_owned()), (3, "b.md".to_owned())]
        );
    }

    fn root_readme() -> String {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../README.md");
        std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
    }

    /// The root README is published where relative links break (see the module
    /// docs), so it links only to absolute `https://` URLs, `mailto:` addresses
    /// and headings in itself.
    #[test]
    fn root_readme_links_are_absolute() {
        let found: Vec<String> = bad(&root_readme())
            .into_iter()
            .map(|(line, target)| format!("README.md:{line}: `{target}`"))
            .collect();
        assert!(
            found.is_empty(),
            "README.md may link only to https://, mailto: or #fragment targets. Use \
             https://github.com/jabbott-iii/Grimoire/blob/main/<path> (tree/main for a \
             directory) for a repository file:\n{}",
            found.join("\n")
        );
    }

    /// GitHub, crates.io and PyPI hide HTML comments, so the only comments the
    /// README may hold are the release markers (`release::plan::is_marker`), each
    /// on a line of its own, where `cargo xtask release plan` finds them.
    #[test]
    fn root_readme_comments_are_release_markers() {
        let text = root_readme();
        let comments = scan(&text).comments;
        for (line, comment) in &comments {
            let whole = text.lines().nth(line - 1).unwrap_or_default().trim();
            assert!(
                is_marker(comment) && whole == comment,
                "README.md:{line}: the only HTML comments allowed are `<!-- before-first-release: … -->` and `<!-- /before-first-release -->` lines of their own: {comment}"
            );
        }
        let guarded = before_first_release(&[("README.md", text.clone())]);
        assert_eq!(
            guarded.len(),
            comments.len(),
            "every marker must be one the release guard finds: {guarded:?}"
        );
    }
}
