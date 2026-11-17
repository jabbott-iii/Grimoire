//! Detection rules and the scanner.
//!
//! Every rule matches a credential format that its vendor documents with a
//! fixed, distinctive prefix or structure, so false positives stay rare. There
//! are no entropy heuristics. The sources are recorded in `docs/specs.md`
//! (`SECRET-FORMATS`).

use std::ops::Range;
use std::sync::OnceLock;

use regex::bytes::Regex;
use sha2::{Digest, Sha256};

/// One detection rule.
pub(crate) struct Rule {
    /// Stable identifier, used in reports and tests.
    pub(crate) id: &'static str,
    /// What the rule finds, in a phrase.
    pub(crate) description: &'static str,
    /// The token pattern, matched against raw bytes. A `secret` capture group,
    /// when present, is the secret; otherwise the whole token is.
    pattern: &'static str,
    /// Whether the token must not directly follow a letter or digit. Anything
    /// else may precede it, including `_` and a backslash escape such as the
    /// `\n` in JSON text.
    bounded: bool,
    /// A run of 16 identical bytes starting within this many bytes of the
    /// secret marks a placeholder (`ghp_xxxx…`); `0` turns the check off.
    /// Only the start counts, so padding a real secret does not exempt it.
    placeholder_window: usize,
}

/// The length of a run of one repeated byte that marks a placeholder.
const PLACEHOLDER_RUN: usize = 16;

macro_rules! rule {
    ($id:literal, $description:literal, $pattern:expr, $window:expr) => {
        Rule {
            id: $id,
            description: $description,
            pattern: $pattern,
            bounded: true,
            placeholder_window: $window,
        }
    };
}

/// The rules, in report order. Sources: `SECRET-FORMATS` in `docs/specs.md`.
/// Lengths are minimums wherever a vendor may lengthen its tokens.
pub(crate) const RULES: &[Rule] = &[
    Rule {
        id: "private-key",
        description: "a private key",
        // RFC 7468, plus the OpenSSL, OpenSSH and OpenPGP labels; a body and an
        // END line are required, so a document that only names the header is not flagged.
        pattern: r"-----BEGIN (?:RSA |DSA |EC |SM2 |OPENSSH |ENCRYPTED |PGP )?PRIVATE KEY(?: BLOCK)?-----[\s\S]{64,}?-----END [A-Z0-9 ]{0,20}PRIVATE KEY(?: BLOCK)?-----",
        bounded: false,
        placeholder_window: 0,
    },
    // GitHub: classic personal access and OAuth tokens are 40 characters today.
    rule!(
        "github-token",
        "a GitHub personal access or OAuth token",
        r"gh[op]_[0-9A-Za-z]{36,251}(?-u:\b)",
        40
    ),
    // GitHub's own recommended pattern: installation tokens changed to a longer format in 2026.
    rule!(
        "github-app-token",
        "a GitHub App installation or user access token",
        r"gh[su]_[0-9A-Za-z._-]{36,}",
        40
    ),
    rule!(
        "github-refresh-token",
        "a GitHub App refresh token",
        r"ghr_[0-9A-Za-z_]{36,}",
        40
    ),
    rule!(
        "github-fine-grained-token",
        "a GitHub fine-grained personal access token",
        r"github_pat_[0-9A-Za-z_]{40,}",
        51
    ),
    rule!(
        "gitlab-token",
        "a GitLab personal, project or group access token",
        r"glpat-[0-9A-Za-z_-]{20}[0-9A-Za-z_.-]*",
        26
    ),
    rule!(
        "aws-access-key-id",
        "an AWS access key ID",
        // The IAM API allows 16 to 128 characters; AWS's examples have 16 after the prefix.
        r"(?:AKIA|ASIA)[0-9A-Z]{16,124}(?-u:\b)",
        20
    ),
    // Only after a documented name, since the value has no prefix: the
    // `aws_secret_access_key` file and variable names, and the API's
    // `SecretAccessKey` / SDKs' `secretAccessKey` fields.
    rule!(
        "aws-secret-access-key",
        "an AWS secret access key",
        r#"(?i)(?:aws_?)?secret_?access_?key["']?\s*(?:[:=]|\s)\s*["']?(?P<secret>[A-Za-z0-9/+]{40,})(?:[^A-Za-z0-9/+=]|$)"#,
        40
    ),
    rule!(
        "slack-token",
        "a Slack bot, user or app-level token",
        r"(?:xoxe\.)?(?:xoxb|xoxp|xapp)-[0-9]+-[0-9A-Za-z-]{6,}",
        40
    ),
    rule!(
        "slack-refresh-token",
        "a Slack refresh token",
        r"xoxe-[0-9]+-[0-9A-Za-z-]{20,}",
        40
    ),
    // Fixed length, so the whole URL is checked for placeholders.
    rule!(
        "slack-webhook",
        "a Slack incoming-webhook URL",
        r"https://hooks\.slack\.com/services/T[0-9A-Z]{8,}/B[0-9A-Z]{8,}/[0-9A-Za-z]{24}",
        usize::MAX
    ),
    rule!(
        "stripe-key",
        "a Stripe live secret or restricted key",
        r"(?:sk|rk)_live_[0-9A-Za-z]{24,}",
        32
    ),
    rule!(
        "anthropic-api-key",
        "an Anthropic API key",
        r"sk-ant-(?:api|admin)[0-9]{2}-[0-9A-Za-z_-]{32,}",
        45
    ),
    // Third-party pattern (GitLab's rules): OpenAI's own docs could not be read.
    rule!(
        "openai-api-key",
        "an OpenAI API key",
        r"sk-(?:proj|svcacct|admin)-[0-9A-Za-z_-]{40,}",
        48
    ),
    rule!(
        "npm-token",
        "an npm access token",
        r"npm_[0-9A-Za-z]{36,}(?-u:\b)",
        40
    ),
    // `AgEIcHlwaS5vcmc` is the encoded macaroon header for pypi.org.
    rule!(
        "pypi-token",
        "a PyPI API token",
        r"pypi-AgEIcHlwaS5vcmc[0-9A-Za-z_-]{50,}",
        70
    ),
];

/// Whether a match is a documented example or a placeholder rather than a
/// credential: AWS's documentation examples all end in `EXAMPLE` or
/// `EXAMPLEKEY`, and documentation often writes `XXXX…` or `0000…`.
fn is_example(rule: &Rule, value: &[u8]) -> bool {
    if rule.id.starts_with("aws-")
        && (value.ends_with(b"EXAMPLE") || value.ends_with(b"EXAMPLEKEY"))
    {
        return true;
    }
    let window = &value[..value.len().min(rule.placeholder_window)];
    window
        .windows(PLACEHOLDER_RUN)
        .any(|w| w.iter().all(|b| *b == w[0]))
}

/// One likely secret in a text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finding {
    /// The rule that matched.
    pub(crate) rule: &'static str,
    /// What it found.
    pub(crate) description: &'static str,
    /// 1-based line.
    pub(crate) line: usize,
    /// 1-based column, counted in characters.
    pub(crate) column: usize,
    /// `sha256:` and the first 12 hex digits of the matched bytes: enough to
    /// tell findings apart and to match them against a known value, never
    /// enough to recover the secret.
    pub(crate) fingerprint: String,
    /// Where the match is, in bytes. Used to compare a match with existing
    /// content; never printed.
    pub(crate) span: Range<usize>,
}

fn compiled() -> &'static [(&'static Rule, Regex)] {
    static COMPILED: OnceLock<Vec<(&'static Rule, Regex)>> = OnceLock::new();
    COMPILED.get_or_init(|| {
        RULES
            .iter()
            .filter_map(|rule| {
                // No look-behind in `regex`: a bounded token consumes the byte
                // before it, or a two-byte escape, unless it starts the text.
                let pattern = if rule.bounded {
                    format!(r"(?:^|[^0-9A-Za-z]|\\[A-Za-z])(?P<token>{})", rule.pattern)
                } else {
                    format!("(?P<token>{})", rule.pattern)
                };
                Regex::new(&pattern).ok().map(|re| (rule, re))
            })
            .collect()
    })
}

/// Whether every rule's pattern compiles. Checked by the tests; a rule that
/// failed to compile would otherwise be skipped silently.
#[cfg(test)]
pub(crate) fn all_rules_compile() -> bool {
    compiled().len() == RULES.len()
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    bytes.iter().fold(String::new(), |mut out, b| {
        let _ = write!(out, "{b:02x}");
        out
    })
}

/// Finds every likely secret in `text`, ordered by position.
pub(crate) fn scan(text: &[u8]) -> Vec<Finding> {
    let mut hits: Vec<(Range<usize>, &'static Rule)> = Vec::new();
    for (rule, re) in compiled() {
        for caps in re.captures_iter(text) {
            let Some(m) = caps.name("secret").or_else(|| caps.name("token")) else {
                continue;
            };
            if !is_example(rule, m.as_bytes()) {
                hits.push((m.range(), rule));
            }
        }
    }
    hits.sort_by(|a, b| (a.0.start, a.1.id).cmp(&(b.0.start, b.1.id)));
    let mut cursor = Cursor::default();
    hits.into_iter()
        .map(|(span, rule)| {
            let (line, column) = cursor.advance(text, span.start);
            let digest = hex(&Sha256::digest(&text[span.clone()]));
            Finding {
                rule: rule.id,
                description: rule.description,
                line,
                column,
                fingerprint: format!("sha256:{}", &digest[..12]),
                span,
            }
        })
        .collect()
}

/// Line and column positions computed in one pass over sorted offsets.
struct Cursor {
    offset: usize,
    line: usize,
    column: usize,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

impl Cursor {
    /// The 1-based line and character column of byte `offset`, which is at or
    /// after the previous one.
    fn advance(&mut self, text: &[u8], offset: usize) -> (usize, usize) {
        let segment = &text[self.offset..offset];
        let chars = |bytes: &[u8]| String::from_utf8_lossy(bytes).chars().count();
        match memchr::memrchr(b'\n', segment) {
            Some(last) => {
                self.line += memchr::memchr_iter(b'\n', segment).count();
                self.column = 1 + chars(&segment[last + 1..]);
            }
            None => self.column += chars(segment),
        }
        self.offset = offset;
        (self.line, self.column)
    }
}

#[cfg(test)]
pub(crate) mod samples {
    //! Synthetic credentials, assembled at run time so that no literal in this
    //! repository looks like a real secret to a scanner.

    /// `n` copies of a character.
    pub(crate) fn run(c: char, n: usize) -> String {
        std::iter::repeat_n(c, n).collect()
    }

    /// `n` varied characters from `alphabet`, deterministic per `seed`: shaped
    /// like a random token, but plainly synthetic.
    pub(crate) fn body(n: usize, seed: u32, alphabet: &str) -> String {
        let chars: Vec<char> = alphabet.chars().collect();
        let mut state = seed.wrapping_mul(2_654_435_761).wrapping_add(1);
        (0..n)
            .map(|_| {
                state = state.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                chars[(state >> 16) as usize % chars.len()]
            })
            .collect()
    }

    /// Base62.
    pub(crate) const B62: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    /// Uppercase and digits.
    pub(crate) const UPPER: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    /// A GitHub-shaped classic token.
    pub(crate) fn github(seed: u32) -> String {
        format!("ghp_{}", body(36, seed, B62))
    }

    /// One synthetic value per rule, keyed by rule id.
    pub(crate) fn every_rule() -> Vec<(&'static str, String)> {
        vec![
            (
                "private-key",
                format!(
                    "-----BEGIN {k}PRIVATE KEY-----\n{}\n{}\n-----END {k}PRIVATE KEY-----",
                    body(64, 1, B62),
                    body(20, 2, B62),
                    k = "RSA "
                ),
            ),
            ("github-token", github(3)),
            (
                "github-app-token",
                format!(
                    "ghs_{}_{}.{}.{}",
                    body(6, 4, "0123456789"),
                    body(40, 5, B62),
                    body(30, 6, B62),
                    body(20, 7, B62)
                ),
            ),
            ("github-refresh-token", format!("ghr_{}", body(76, 8, B62))),
            (
                "github-fine-grained-token",
                format!("github_pat_{}_{}", body(22, 9, B62), body(59, 10, B62)),
            ),
            ("gitlab-token", format!("glpat-{}", body(20, 11, B62))),
            ("aws-access-key-id", format!("AKIA{}", body(16, 12, UPPER))),
            (
                "aws-secret-access-key",
                format!("aws_secret_access_key = {}", body(40, 13, B62)),
            ),
            (
                "slack-token",
                format!(
                    "xoxb-{}-{}-{}",
                    body(11, 14, "0123456789"),
                    body(13, 15, "0123456789"),
                    body(24, 16, B62)
                ),
            ),
            (
                "slack-refresh-token",
                format!("xoxe-1-{}", body(40, 17, B62)),
            ),
            (
                "slack-webhook",
                format!(
                    "https://hooks.slack.com/services/T{}/B{}/{}",
                    body(9, 18, UPPER),
                    body(11, 19, UPPER),
                    body(24, 20, B62)
                ),
            ),
            ("stripe-key", format!("sk_live_{}", body(24, 21, B62))),
            (
                "anthropic-api-key",
                format!("sk-ant-api03-{}", body(95, 22, B62)),
            ),
            ("openai-api-key", format!("sk-proj-{}", body(60, 23, B62))),
            ("npm-token", format!("npm_{}", body(36, 24, B62))),
            (
                "pypi-token",
                format!("pypi-AgEIcHlwaS5vcmc{}", body(60, 25, B62)),
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::samples::{B62, UPPER, body, every_rule, github, run};
    use super::*;

    #[test]
    fn rules_compile_and_have_unique_ids() {
        assert!(all_rules_compile());
        let mut ids: Vec<&str> = RULES.iter().map(|r| r.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), RULES.len());
    }

    #[test]
    fn every_rule_finds_its_sample() {
        let samples = every_rule();
        assert_eq!(samples.len(), RULES.len(), "one sample per rule");
        for (rule, value) in samples {
            let text = format!("let x = \"{value}\";\n");
            let found = scan(text.as_bytes());
            assert_eq!(
                found.iter().map(|f| f.rule).collect::<Vec<_>>(),
                [rule],
                "{rule}: {found:?}"
            );
        }
    }

    #[test]
    fn positions_are_one_based_and_count_characters() {
        let text = format!("first line\n  é {}\n", github(1));
        let found = scan(text.as_bytes());
        assert_eq!((found[0].line, found[0].column), (2, 5));
    }

    #[test]
    fn secrets_in_named_groups_point_at_the_value() {
        let text = format!("aws_secret_access_key={}", body(40, 3, B62));
        let found = scan(text.as_bytes());
        assert_eq!(found[0].column, "aws_secret_access_key=".len() + 1);
        assert_eq!(found[0].span.len(), 40);
    }

    #[test]
    fn fingerprints_identify_without_revealing() {
        let value = github(2);
        let found = scan(value.as_bytes());
        assert!(found[0].fingerprint.starts_with("sha256:"));
        assert_eq!(found[0].fingerprint.len(), "sha256:".len() + 12);
        assert!(!found[0].fingerprint.contains("ghp_"));
    }

    #[test]
    fn near_misses_examples_and_placeholders_are_not_reported() {
        for text in [
            // Too short, the wrong alphabet, or glued to more word characters.
            format!("ghp_{}", body(20, 1, B62)),
            format!("AKIA{}", body(16, 2, "abcdefghijklmnop")),
            format!("npm_{}", body(10, 3, B62)),
            format!("xghp_{}", body(36, 4, B62)),
            format!("{}_suffix", github(5)),
            // Test-mode and publishable keys are not live secrets.
            format!("sk_test_{}", body(24, 6, B62)),
            format!("pk_live_{}", body(24, 7, B62)),
            // Placeholders and vendor documentation examples.
            format!("ghp_{}", run('x', 36)),
            format!(
                "https://hooks.slack.com/services/T00000000/B00000000/{}",
                run('X', 24)
            ),
            "xoxb-your-token".to_owned(),
            format!("AKIA{}{}", "IOSFODNN7", "EXAMPLE"),
            format!(
                "aws_secret_access_key = {}{}",
                body(30, 8, B62),
                "EXAMPLEKEY"
            ),
            // A private-key header with no key, as in documentation.
            "-----BEGIN RSA PRIVATE KEY-----".to_owned(),
            "-----BEGIN PUBLIC KEY-----".to_owned(),
            "-----BEGIN CERTIFICATE-----".to_owned(),
            // Prefix-shaped words.
            "the skeleton key".to_owned(),
            format!("sk-ant-{}", body(40, 9, B62)),
            format!("aws_access_key_id = AKIA{}", body(12, 10, UPPER)),
        ] {
            assert_eq!(scan(text.as_bytes()), Vec::new(), "{text}");
        }
    }

    #[test]
    fn invalid_utf8_is_scanned_too() {
        let mut text = vec![0xff, 0xfe, b' '];
        text.extend_from_slice(github(6).as_bytes());
        assert_eq!(scan(&text).len(), 1);
    }

    #[test]
    fn aws_secret_keys_ending_in_slash_or_plus_are_found() {
        for last in ['/', '+', 'a'] {
            let key = format!("{}{last}", body(39, 30, B62));
            for text in [
                format!("aws_secret_access_key = {key}\n"),
                format!("AWS_SECRET_ACCESS_KEY=\"{key}\""),
                format!("aws_secret_access_key={key}"),
            ] {
                let found = scan(text.as_bytes());
                assert_eq!(found.len(), 1, "{text}");
                assert_eq!(found[0].span.len(), 40, "{text}");
            }
        }
    }

    #[test]
    fn aws_api_and_sdk_field_names_are_recognised() {
        let key = body(40, 31, B62);
        for text in [
            format!(r#"{{"Credentials": {{"SecretAccessKey": "{key}"}}}}"#),
            format!(r#"credentials: {{ secretAccessKey: "{key}" }}"#),
        ] {
            assert_eq!(scan(text.as_bytes()).len(), 1, "{text}");
        }
    }

    #[test]
    fn padding_a_real_token_does_not_make_it_a_placeholder() {
        let pad = run('A', 16);
        for (rule, value) in every_rule() {
            if rule == "private-key" || rule == "slack-webhook" {
                continue;
            }
            let padded = format!("{value}{pad}");
            assert!(!scan(padded.as_bytes()).is_empty(), "{rule}: {padded}");
        }
        // A real placeholder is still recognised.
        assert_eq!(scan(format!("ghs_{}", run('x', 40)).as_bytes()), Vec::new());
        assert_eq!(
            scan(format!("sk-ant-api03-{}", run('X', 40)).as_bytes()),
            Vec::new()
        );
    }

    #[test]
    fn longer_github_and_npm_tokens_are_found() {
        for text in [
            format!("ghp_{}", body(50, 32, B62)),
            format!("gho_{}", body(60, 33, B62)),
            format!("npm_{}", body(40, 34, B62)),
        ] {
            assert_eq!(scan(text.as_bytes()).len(), 1, "{text}");
        }
    }

    #[test]
    fn tokens_after_underscores_and_escapes_are_found() {
        let token = github(35);
        let key_id = format!("AKIA{}", body(16, 36, UPPER));
        for text in [
            format!(r#"{{"log": "created:\n{token}\nid:\n{key_id}\n"}}"#),
            format!("MY_TOKEN_{token}"),
            format!("printf 'token:\\t{token}'"),
        ] {
            assert!(!scan(text.as_bytes()).is_empty(), "{text}");
        }
        assert_eq!(
            scan(format!("x{token}").as_bytes()),
            Vec::new(),
            "still not after a letter"
        );
    }

    #[test]
    fn many_findings_are_positioned_in_linear_time() {
        let line = format!("ASIA{}\n", body(16, 37, UPPER));
        let text = line.repeat(50_000);
        let started = std::time::Instant::now();
        let found = scan(text.as_bytes());
        assert_eq!(found.len(), 50_000);
        assert_eq!((found[49_999].line, found[49_999].column), (50_000, 1));
        assert!(
            started.elapsed() < std::time::Duration::from_secs(20),
            "{:?}",
            started.elapsed()
        );
    }
}
