//! String newtypes whose values are validated on construction.
//!
//! Each type deserializes through `TryFrom<String>`, so an invalid value is
//! reported by the TOML parser with the span of the offending field.

use std::fmt;

/// A value rejected by one of the constrained types in this crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidValue {
    what: &'static str,
    value: String,
    reason: String,
}

impl InvalidValue {
    pub(crate) fn new(what: &'static str, value: &str, reason: impl Into<String>) -> Self {
        Self {
            what,
            value: value.to_owned(),
            reason: reason.into(),
        }
    }

    /// What kind of value was rejected, e.g. `"asset id"`.
    #[must_use]
    pub fn what(&self) -> &'static str {
        self.what
    }

    /// Why the value was rejected.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for InvalidValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Long values (descriptions) are truncated so diagnostics stay readable.
        const SHOWN: usize = 48;
        let shown: String = self.value.chars().take(SHOWN).collect();
        let ellipsis = if self.value.chars().count() > SHOWN {
            "…"
        } else {
            ""
        };
        write!(
            f,
            "invalid {} {shown:?}{ellipsis}: {}",
            self.what, self.reason
        )
    }
}

impl std::error::Error for InvalidValue {}

/// Defines a `String` newtype validated by `check: fn(&str) -> Result<(), String>`
/// with a hand-written JSON Schema.
macro_rules! constrained_string {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            what: $what:literal,
            check: $check:path,
            schema: $schema:tt $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ::serde::Serialize, ::serde::Deserialize)]
        #[serde(try_from = "String", into = "String")]
        $vis struct $name(String);

        impl $name {
            /// Validates `value` and wraps it.
            ///
            /// # Errors
            ///
            /// Returns [`InvalidValue`](crate::InvalidValue) when `value` breaks the rules of this type.
            pub fn new(value: impl Into<String>) -> Result<Self, $crate::InvalidValue> {
                let value = value.into();
                match $check(&value) {
                    Ok(()) => Ok(Self(value)),
                    Err(reason) => Err($crate::InvalidValue::new($what, &value, reason)),
                }
            }

            /// The validated value.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl ::core::convert::TryFrom<String> for $name {
            type Error = $crate::InvalidValue;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl ::core::convert::From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl ::core::str::FromStr for $name {
            type Err = $crate::InvalidValue;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl ::core::convert::AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl ::core::fmt::Display for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl ::schemars::JsonSchema for $name {
            fn schema_name() -> ::std::borrow::Cow<'static, str> {
                stringify!($name).into()
            }

            fn json_schema(_: &mut ::schemars::SchemaGenerator) -> ::schemars::Schema {
                ::schemars::json_schema!($schema)
            }
        }
    };
}

pub(crate) use constrained_string;

/// Checks a kebab-case identifier: `[a-z0-9-]`, no leading, trailing or doubled `-`.
pub(crate) fn check_kebab(value: &str, max: usize) -> Result<(), String> {
    if value.is_empty() {
        return Err("must not be empty".into());
    }
    let len = value.chars().count();
    if len > max {
        return Err(format!("must be at most {max} characters (got {len})"));
    }
    if let Some(c) = value
        .chars()
        .find(|c| !(c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-'))
    {
        return Err(format!(
            "may contain only lowercase ASCII letters, digits and '-' (found {c:?})"
        ));
    }
    if value.starts_with('-') || value.ends_with('-') {
        return Err("must not start or end with '-'".into());
    }
    if value.contains("--") {
        return Err("must not contain '--'".into());
    }
    Ok(())
}

/// Checks free text: length bounds in characters, and no control characters
/// other than those listed in `allowed_controls`.
pub(crate) fn check_text(value: &str, max: usize, single_line: bool) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err("must not be empty or only whitespace".into());
    }
    let len = value.chars().count();
    if len > max {
        return Err(format!("must be at most {max} characters (got {len})"));
    }
    let allowed: &[char] = if single_line {
        &[]
    } else {
        &['\n', '\r', '\t']
    };
    if let Some(c) = value
        .chars()
        .find(|c| c.is_control() && !allowed.contains(c))
    {
        return Err(if single_line && (c == '\n' || c == '\r') {
            "must be a single line".into()
        } else {
            format!("must not contain the control character {c:?}")
        });
    }
    if single_line && value.trim() != value {
        return Err("must not have leading or trailing whitespace".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_rules() {
        assert!(check_kebab("rust-errors", 64).is_ok());
        assert!(check_kebab("a", 64).is_ok());
        assert!(check_kebab("a1-b2", 64).is_ok());
        assert!(check_kebab("", 64).is_err());
        assert!(check_kebab("-a", 64).is_err());
        assert!(check_kebab("a-", 64).is_err());
        assert!(check_kebab("a--b", 64).is_err());
        assert!(check_kebab("A", 64).is_err());
        assert!(check_kebab("a_b", 64).is_err());
        assert!(check_kebab("é", 64).is_err());
        assert!(check_kebab(&"a".repeat(65), 64).is_err());
        assert!(check_kebab(&"a".repeat(64), 64).is_ok());
    }

    #[test]
    fn text_rules() {
        assert!(check_text("One line", 10, true).is_ok());
        assert!(check_text(" padded", 10, true).is_err());
        assert!(check_text("two\nlines", 20, true).is_err());
        assert!(check_text("two\nlines", 20, false).is_ok());
        assert!(check_text("   ", 20, false).is_err());
        assert!(check_text("bell\u{7}", 20, false).is_err());
        assert!(check_text("ünïcödé", 7, true).is_ok());
        assert!(check_text("ünïcödé!", 7, true).is_err());
    }

    #[test]
    fn invalid_value_display_truncates() {
        let long = "x".repeat(100);
        let err = InvalidValue::new("summary", &long, "too long");
        let shown = err.to_string();
        assert!(shown.contains('…'));
        assert!(shown.len() < 100);
    }
}
