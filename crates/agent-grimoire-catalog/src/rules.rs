//! Duplicate and near-duplicate detection across rules.
//!
//! Contradictions between rules cannot be detected mechanically; they are
//! handled by declared `conflicts` and the bundle conflict check. This lint
//! catches the mechanical part: the same guidance shipped twice.

use std::collections::BTreeSet;

use agent_grimoire_model::{AssetId, Diagnostic};

/// Words per shingle for near-duplicate detection.
const SHINGLE: usize = 5;

/// The source text of one rule output.
pub(crate) struct RuleText {
    pub(crate) id: AssetId,
    pub(crate) file: String,
    pub(crate) text: String,
}

fn words(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_owned)
        .collect()
}

fn shingles(words: &[String]) -> BTreeSet<&[String]> {
    if words.len() < SHINGLE {
        return BTreeSet::from([words]);
    }
    words.windows(SHINGLE).collect()
}

/// Jaccard similarity of the two shingle sets, as `(intersection, union)`.
fn similarity(a: &BTreeSet<&[String]>, b: &BTreeSet<&[String]>) -> (usize, usize) {
    let inter = a.intersection(b).count();
    (inter, a.len() + b.len() - inter)
}

pub(crate) fn check(rules: &[RuleText], out: &mut Vec<Diagnostic>) {
    let prepared: Vec<(&RuleText, Vec<String>)> =
        rules.iter().map(|r| (r, words(&r.text))).collect();
    let shingled: Vec<BTreeSet<&[String]>> = prepared.iter().map(|(_, w)| shingles(w)).collect();
    for (i, (a, words_a)) in prepared.iter().enumerate() {
        for (j, (b, words_b)) in prepared.iter().enumerate().skip(i + 1) {
            if a.id == b.id {
                continue;
            }
            if words_a == words_b {
                out.push(
                    Diagnostic::error(
                        "rule/duplicate",
                        format!("same text as `{}` ({}); keep one rule", a.id, a.file),
                    )
                    .in_file(b.file.clone()),
                );
                continue;
            }
            let (inter, union) = similarity(&shingled[i], &shingled[j]);
            // Integer comparisons of inter/union against 0.9 and 0.7.
            if union > 0 && inter * 10 >= union * 9 {
                out.push(
                    Diagnostic::error(
                        "rule/near-duplicate",
                        format!(
                            "at least 90% similar to `{}` ({}); merge them",
                            a.id, a.file
                        ),
                    )
                    .in_file(b.file.clone()),
                );
            } else if union > 0 && inter * 10 >= union * 7 {
                out.push(
                    Diagnostic::warning(
                        "rule/near-duplicate",
                        format!(
                            "at least 70% similar to `{}` ({}); check for overlap",
                            a.id, a.file
                        ),
                    )
                    .in_file(b.file.clone()),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(id: &str, text: &str) -> RuleText {
        RuleText {
            id: AssetId::new(id).unwrap(),
            file: format!("rules/{id}/rule.md"),
            text: text.to_owned(),
        }
    }

    const BASE: &str = "Use thiserror for library error types and anyhow only in binaries. \
        Never panic in library code; return a Result with context instead. \
        Prefer explicit error variants over string errors so callers can match on them.";

    #[test]
    fn exact_duplicates_ignore_formatting() {
        let mut out = Vec::new();
        check(
            &[
                rule("a", BASE),
                rule("b", &BASE.to_uppercase().replace(". ", ".\n\n")),
            ],
            &mut out,
        );
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].code, "rule/duplicate");
    }

    #[test]
    fn near_duplicates_are_errors() {
        let mut out = Vec::new();
        let tweaked = format!("{BASE} Always.");
        check(&[rule("a", BASE), rule("b", &tweaked)], &mut out);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].code, "rule/near-duplicate");
        assert!(out[0].is_error());
    }

    #[test]
    fn different_rules_pass() {
        let mut out = Vec::new();
        check(
            &[
                rule("a", BASE),
                rule(
                    "b",
                    "Write commit messages in the imperative mood, under 72 characters.",
                ),
            ],
            &mut out,
        );
        assert!(out.is_empty(), "{out:?}");
    }

    #[test]
    fn same_asset_is_not_compared_with_itself() {
        let mut out = Vec::new();
        check(&[rule("a", BASE), rule("a", BASE)], &mut out);
        assert_eq!(out, Vec::new());
    }
}
