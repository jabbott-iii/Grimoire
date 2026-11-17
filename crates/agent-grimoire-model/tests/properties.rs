//! Property tests for the safety-critical value types.

use agent_grimoire_model::{AssetId, Capability, Destination, McpToolName, RelPath};
use proptest::prelude::*;

/// An independent statement of the asset-id rules, to cross-check the implementation.
fn is_valid_id(s: &str) -> bool {
    const RESERVED: [&str; 5] = [
        "claude-",
        "anthropic-",
        "anthropics-",
        "cc-plugin-",
        "grimoire-",
    ];
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
        && !RESERVED.iter().any(|p| s.starts_with(p))
        && s != "synced"
        && s != "grimoire"
}

fn safe_component() -> impl Strategy<Value = String> {
    "[a-z0-9_][a-z0-9_.-]{0,10}[a-z0-9_]".prop_filter("not a Windows device name", |c| {
        let stem = c.split('.').next().unwrap_or_default();
        !matches!(
            stem.to_ascii_uppercase().as_str(),
            "CON" | "PRN" | "AUX" | "NUL"
        ) && !(stem.len() == 4
            && (stem.to_ascii_uppercase().starts_with("COM")
                || stem.to_ascii_uppercase().starts_with("LPT")))
    })
}

proptest! {
    #[test]
    fn asset_id_matches_the_rules(s in "[a-z0-9-]{0,70}|claude-[a-z]{1,5}|grimoire(-[a-z]{1,5})?|synced|[A-Za-z0-9_ .-]{1,10}") {
        prop_assert_eq!(AssetId::new(s.clone()).is_ok(), is_valid_id(&s));
    }

    #[test]
    fn accepted_paths_are_confined(s in "\\PC{0,40}") {
        if let Ok(path) = RelPath::new(s.clone()) {
            prop_assert_eq!(path.as_str(), s.as_str());
            prop_assert!(s.is_ascii());
            prop_assert!(!s.starts_with('/'));
            prop_assert!(!s.contains('\\'));
            prop_assert!(!s.contains(':'));
            for component in path.components() {
                prop_assert!(!component.is_empty());
                prop_assert!(component != "." && component != "..");
            }
        }
    }

    #[test]
    fn traversal_is_always_rejected(prefix in proptest::collection::vec(safe_component(), 0..4),
                                    suffix in proptest::collection::vec(safe_component(), 0..4)) {
        let mut parts = prefix;
        parts.push("..".to_owned());
        parts.extend(suffix);
        prop_assert!(RelPath::new(parts.join("/")).is_err());
    }

    #[test]
    fn safe_component_lists_are_accepted(parts in proptest::collection::vec(safe_component(), 1..6)) {
        let joined = parts.join("/");
        prop_assert!(RelPath::new(joined.clone()).is_ok(), "{}", joined);
    }

    #[test]
    fn capabilities_round_trip(server in "[a-z0-9]{1,8}(-[a-z0-9]{1,8}){0,2}",
                               tool in proptest::option::of("[A-Za-z0-9_.-]{1,20}")) {
        let cap = Capability::Mcp {
            server: AssetId::new(server).unwrap(),
            tool: tool.map(|t| McpToolName::new(t).unwrap()),
        };
        prop_assert_eq!(Capability::parse(&cap.to_string()).unwrap(), cap);
    }

    #[test]
    fn destinations_round_trip(parts in proptest::collection::vec(safe_component(), 1..4),
                               anchor in prop_oneof!["rules", "skills", "agents"]) {
        let text = format!("{anchor}:{}", parts.join("/"));
        let parsed = Destination::parse(&text).unwrap();
        prop_assert_eq!(parsed.to_string(), text);
    }
}
