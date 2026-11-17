//! Small text builders shared by adapters: YAML frontmatter and Markdown code spans.

use serde_json::Value;

/// A YAML scalar for `value`. JSON string syntax is valid YAML 1.2 double-quoted
/// syntax, and always quoting avoids YAML reading `true`, `null` or `123` as
/// non-strings.
pub(crate) fn yaml_string(value: &str) -> String {
    Value::String(value.to_owned()).to_string()
}

/// A YAML flow sequence of strings.
pub(crate) fn yaml_list<'a>(values: impl IntoIterator<Item = &'a str>) -> String {
    let items: Vec<String> = values.into_iter().map(yaml_string).collect();
    format!("[{}]", items.join(", "))
}

/// Frontmatter from `(key, rendered value)` pairs, followed by `body`.
pub(crate) fn with_frontmatter(fields: &[(&str, String)], body: &str) -> String {
    let mut out = String::from("---\n");
    for (key, value) in fields {
        out.push_str(key);
        out.push_str(": ");
        out.push_str(value);
        out.push('\n');
    }
    out.push_str("---\n\n");
    out.push_str(body);
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

/// A Markdown code span that shows `text` literally, whatever backticks it holds.
pub(crate) fn code_span(text: &str) -> String {
    let longest = text.split(|c| c != '`').map(str::len).max().unwrap_or(0);
    let fence = "`".repeat(longest + 1);
    if longest == 0 {
        format!("{fence}{text}{fence}")
    } else {
        format!("{fence} {text} {fence}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yaml_values_are_quoted() {
        assert_eq!(yaml_string("true"), "\"true\"");
        assert_eq!(yaml_string("say \"hi\"\n"), "\"say \\\"hi\\\"\\n\"");
        assert_eq!(yaml_list(["**/*.rs", "a"]), "[\"**/*.rs\", \"a\"]");
    }

    #[test]
    fn code_spans_survive_backticks() {
        assert_eq!(code_span("**/*.rs"), "`**/*.rs`");
        assert_eq!(code_span("a`b"), "`` a`b ``");
    }

    #[test]
    fn frontmatter_layout() {
        assert_eq!(
            with_frontmatter(&[("paths", yaml_list(["x"]))], "Body"),
            "---\npaths: [\"x\"]\n---\n\nBody\n"
        );
    }
}
