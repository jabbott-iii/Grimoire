//! `SemVer` versions and requirements, serialized as strings.

use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;

use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};

use crate::InvalidValue;

/// A `SemVer` 2.0.0 version without build metadata, e.g. `1.2.3` or `1.2.3-rc.1`.
///
/// Because build metadata is refused, the derived ordering is `SemVer` precedence.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Version(semver::Version);

impl Version {
    /// Parses a version.
    ///
    /// # Errors
    ///
    /// Fails when `value` is not a valid `SemVer` 2.0.0 version.
    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        let version = semver::Version::parse(value)
            .map_err(|e| InvalidValue::new("version", value, e.to_string()))?;
        if !version.build.is_empty() {
            return Err(InvalidValue::new(
                "version",
                value,
                "build metadata (`+...`) is not allowed: it does not change SemVer precedence",
            ));
        }
        Ok(Self(version))
    }

    /// The underlying `semver` value.
    #[must_use]
    pub fn semver(&self) -> &semver::Version {
        &self.0
    }
}

impl From<semver::Version> for Version {
    fn from(value: semver::Version) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for Version {
    type Error = InvalidValue;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<Version> for String {
    fn from(value: Version) -> Self {
        value.0.to_string()
    }
}

impl FromStr for Version {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Version({})", self.0)
    }
}

impl JsonSchema for Version {
    fn schema_name() -> Cow<'static, str> {
        "Version".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "minLength": 5,
            "description": "A SemVer 2.0.0 version without build metadata, e.g. \"1.2.3\"."
        })
    }
}

/// A Cargo-style `SemVer` requirement, e.g. `^1.2` or `>=1, <3`.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct VersionReq(semver::VersionReq);

impl VersionReq {
    /// Parses a requirement.
    ///
    /// # Errors
    ///
    /// Fails when `value` is not a valid requirement.
    pub fn parse(value: &str) -> Result<Self, InvalidValue> {
        semver::VersionReq::parse(value)
            .map(Self)
            .map_err(|e| InvalidValue::new("version requirement", value, e.to_string()))
    }

    /// Whether `version` satisfies this requirement.
    #[must_use]
    pub fn matches(&self, version: &Version) -> bool {
        self.0.matches(version.semver())
    }

    /// The underlying `semver` value.
    #[must_use]
    pub fn semver(&self) -> &semver::VersionReq {
        &self.0
    }
}

impl TryFrom<String> for VersionReq {
    type Error = InvalidValue;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(&value)
    }
}

impl From<VersionReq> for String {
    fn from(value: VersionReq) -> Self {
        value.0.to_string()
    }
}

impl FromStr for VersionReq {
    type Err = InvalidValue;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl fmt::Display for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl fmt::Debug for VersionReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "VersionReq({})", self.0)
    }
}

impl JsonSchema for VersionReq {
    fn schema_name() -> Cow<'static, str> {
        "VersionReq".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "minLength": 1,
            "description": "A Cargo-style SemVer requirement, e.g. \"^1.2\", \">=1, <3\" or \"*\"."
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn versions_and_requirements() {
        let v = Version::parse("1.4.0").unwrap();
        assert!(VersionReq::parse("^1").unwrap().matches(&v));
        assert!(!VersionReq::parse("^2").unwrap().matches(&v));
        assert!(VersionReq::parse("*").unwrap().matches(&v));
        assert!(Version::parse("1.4").is_err());
        assert!(Version::parse("1.0.0+build.1").is_err());
        assert!(Version::parse("1.0.0-rc.1").unwrap() < Version::parse("1.0.0").unwrap());
        assert!(VersionReq::parse("not a req").is_err());
        assert_eq!(String::from(v), "1.4.0");
        assert!(Version::parse("1.0.0").unwrap() < Version::parse("1.0.1").unwrap());
    }
}
