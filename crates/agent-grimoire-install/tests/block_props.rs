//! Property tests: managed blocks never disturb the user's text.

use agent_grimoire_install::block::Document;
use proptest::prelude::*;

/// Lines that look like Markdown a user might write, including fences, comments
/// and CRLF, but never a line that starts with a Grimoire marker.
fn user_line() -> impl Strategy<Value = String> {
    prop_oneof![
        "[a-zA-Z0-9 #*_>-]{0,30}",
        Just("```".to_owned()),
        Just("~~~ text".to_owned()),
        Just("<!-- a user comment -->".to_owned()),
        Just(String::new()),
        "    [a-z]{0,10}",
    ]
}

fn user_text() -> impl Strategy<Value = String> {
    (
        prop::collection::vec(user_line(), 0..12),
        any::<bool>(),
        any::<bool>(),
    )
        .prop_map(|(lines, crlf, trailing)| {
            let nl = if crlf { "\r\n" } else { "\n" };
            let mut text = lines.join(nl);
            if trailing && !text.is_empty() {
                text.push_str(nl);
            }
            text
        })
}

fn body() -> impl Strategy<Value = String> {
    prop::collection::vec("[a-zA-Z0-9 `#.-]{0,20}", 1..6).prop_map(|l| l.join("\n"))
}

fn name() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9]{0,6}"
}

proptest! {
    #[test]
    fn add_then_remove_restores_the_text(text in user_text(), name in name(), body in body()) {
        let mut doc = Document::parse(&text).unwrap();
        prop_assert_eq!(doc.render(), text.clone());
        if doc.set(&name, &body).is_err() {
            // Only an unclosed fence at the end may refuse, and nothing changes.
            prop_assert_eq!(doc.render(), text);
            return Ok(());
        }
        let written = doc.render();
        let mut reread = Document::parse(&written).unwrap();
        let expected = body.trim_end_matches('\n').to_owned() + "\n";
        prop_assert_eq!(reread.get(&name).map(|b| b.replace("\r\n", "\n")), Some(expected));
        prop_assert!(reread.remove(&name));
        let restored = reread.render();
        let nl = if text.contains("\r\n") { "\r\n" } else { "\n" };
        if text.is_empty() || text.ends_with('\n') {
            prop_assert_eq!(restored, text);
        } else {
            prop_assert_eq!(restored, text + nl);
        }
    }

    #[test]
    fn blocks_are_independent(text in user_text(), a in body(), b in body(), b2 in body()) {
        let mut doc = Document::parse(&text).unwrap();
        if doc.set("alpha", &a).is_err() {
            return Ok(());
        }
        doc.set("beta", &b).unwrap();
        let before = Document::parse(&doc.render()).unwrap();
        let alpha = before.get("alpha").map(str::to_owned);
        let mut after = before.clone();
        after.set("beta", &b2).unwrap();
        prop_assert!(after.remove("beta"));
        let after = Document::parse(&after.render()).unwrap();
        prop_assert_eq!(after.get("alpha").map(str::to_owned), alpha);
        prop_assert_eq!(after.names(), vec!["alpha"]);
    }
}
