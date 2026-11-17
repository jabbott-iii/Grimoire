//! Strict YAML for Agent Skills frontmatter.
//!
//! serde-saphyr was chosen after a verified comparison (Phase 1 research,
//! 2026-11-16). It is pure safe Rust with no `RustSec` advisories, and it is the
//! only serde YAML crate that rejects duplicate keys at every nesting level.
//! The options below also refuse YAML 1.1-style coercion, so an unquoted
//! `123`, `true` or `~` is not silently accepted as a string. They also refuse
//! merge keys and custom tags. A resource budget is on by default, which stops
//! alias bombs.

use std::collections::BTreeMap;

use serde::Deserializer;
use serde::de::DeserializeOwned;

pub(crate) fn from_str<T: DeserializeOwned>(yaml: &str) -> Result<T, String> {
    if yaml.trim().is_empty() {
        return Err("the frontmatter is empty; it needs at least `name` and `description`".into());
    }
    // `Options` is non-exhaustive, so it is built from the default and adjusted.
    let mut options = serde_saphyr::Options::default();
    options.no_schema = true;
    options.strict_booleans = true;
    options.merge_keys = serde_saphyr::MergeKeyPolicy::Error;
    options.reject_unsupported_tags = true;
    options.reject_tagged_field_names = true;
    serde_saphyr::from_str_with_options(yaml, options).map_err(|e| e.to_string())
}

/// For `Option<String>` fields: absent means `None`, but an explicit null
/// (`key: ~` or an empty `key:`) is an error rather than silently `None`.
pub(crate) fn some_string<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    <String as serde::Deserialize>::deserialize(deserializer).map(Some)
}

/// Like [`some_string`], for the `metadata` string-to-string map.
pub(crate) fn some_map<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<BTreeMap<String, String>>, D::Error> {
    <BTreeMap<String, String> as serde::Deserialize>::deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Doc {
        name: String,
        #[serde(default, deserialize_with = "some_string")]
        license: Option<String>,
        #[serde(default, deserialize_with = "some_map")]
        metadata: Option<BTreeMap<String, String>>,
    }

    #[test]
    fn strict_parsing() {
        let ok: Doc = from_str("name: x\nlicense: MIT\nmetadata:\n  a: b\n").unwrap();
        assert_eq!(ok.name, "x");
        assert_eq!(ok.metadata.unwrap()["a"], "b");
        assert_eq!(ok.license.as_deref(), Some("MIT"));
        assert!(from_str::<Doc>("name: x\n").unwrap().license.is_none());

        for bad in [
            "name: x\nname: y\n",                   // duplicate top-level key
            "name: x\nmetadata:\n  a: b\n  a: c\n", // duplicate nested key
            "name: 123\n",                          // no implicit typing
            "name: true\n",
            "name: x\nlicense: ~\n", // explicit null for an optional field
            "name: x\nlicense:\n",
            "name: x\nmetadata: ~\n", // explicit null map
            "name: x\nmetadata:\n",
            "name: x\nmetadata:\n  a: 1\n", // non-string value
            "name: x\nother: y\n",          // unknown field
            "name: !custom x\n",            // custom tag
            "- name\n",                     // root is not a mapping
            "name: x\n---\nname: y\n",      // multiple documents
            "",
        ] {
            assert!(from_str::<Doc>(bad).is_err(), "should reject {bad:?}");
        }
    }
}
