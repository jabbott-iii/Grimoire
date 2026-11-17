//! winget: the three manifests (schema 1.28.0) for `jabbott-iii.agent-grimoire`, which
//! the maintainer submits to microsoft/winget-pkgs by hand (WINGET and the winget
//! Phase 5 research in docs/specs.md, ADR-0009).
//!
//! - zip archive with a portable `grimoire.exe`, linked as `agent-grimoire` (the
//!   maintainer's choice: `grimoire` is CatfinityStudios.Grimoire's alias).
//! - The binaries link the CRT statically, so there is no `VCRedist` dependency.
//! - Each file gets a JSON twin under `winget-json/` with the same fields, so CI can
//!   validate it against the 1.28.0 JSON schemas without converting YAML.

use std::fmt::Write as _;

use serde_json::{Map, Value, json};

use super::{Generated, Input, Out, SHORT_DESCRIPTION};
use crate::release::targets::{self, BIN, PACKAGE, REPO_URL};

pub(crate) const IDENTIFIER: &str = "jabbott-iii.agent-grimoire";
const ALIAS: &str = "agent-grimoire";
const MANIFEST_VERSION: &str = "1.28.0";

/// The manifests' file names and types.
const FILES: [(&str, &str); 3] = [
    ("", "version"),
    (".installer", "installer"),
    (".locale.en-US", "defaultLocale"),
];

/// A YAML scalar: plain when that is unambiguous, otherwise single-quoted.
fn scalar(s: &str) -> String {
    let plain = !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b" ._-/:+()".contains(&b))
        && !s.starts_with([' ', '-', ':', '.'])
        && !s.ends_with([' ', ':'])
        && !s.contains(": ")
        && !s.contains(" #")
        && s.parse::<f64>().is_err()
        && !matches!(
            s.to_ascii_lowercase().as_str(),
            "true" | "false" | "yes" | "no" | "null" | "on" | "off" | "~"
        );
    if plain {
        s.to_owned()
    } else {
        format!("'{}'", s.replace('\'', "''"))
    }
}

/// Renders a manifest object as YAML: string values, arrays of strings and arrays of
/// flat objects, which is all these manifests use.
fn yaml(map: &Map<String, Value>) -> Result<String, String> {
    let mut out = String::new();
    for (k, v) in map {
        match v {
            Value::String(s) => {
                let _ = writeln!(out, "{k}: {}", scalar(s));
            }
            Value::Array(items) => {
                let _ = writeln!(out, "{k}:");
                for item in items {
                    match item {
                        Value::String(s) => {
                            let _ = writeln!(out, "- {}", scalar(s));
                        }
                        Value::Object(o) => {
                            for (i, (k2, v2)) in o.iter().enumerate() {
                                let lead = if i == 0 { "- " } else { "  " };
                                match v2 {
                                    Value::String(s) => {
                                        let _ = writeln!(out, "{lead}{k2}: {}", scalar(s));
                                    }
                                    Value::Array(inner) => {
                                        let _ = writeln!(out, "{lead}{k2}:");
                                        for x in inner {
                                            let Value::Object(o3) = x else {
                                                return Err("unsupported winget value".to_owned());
                                            };
                                            for (j, (k3, v3)) in o3.iter().enumerate() {
                                                let lead3 = if j == 0 { "  - " } else { "    " };
                                                let s = v3
                                                    .as_str()
                                                    .ok_or("unsupported winget value")?;
                                                let _ = writeln!(out, "{lead3}{k3}: {}", scalar(s));
                                            }
                                        }
                                    }
                                    _ => return Err("unsupported winget value".to_owned()),
                                }
                            }
                        }
                        _ => return Err("unsupported winget value".to_owned()),
                    }
                }
            }
            _ => return Err("unsupported winget value".to_owned()),
        }
    }
    Ok(out)
}

fn manifests(input: &Input) -> Result<[Map<String, Value>; 3], String> {
    let v = &input.version;
    let base = |kind: &str, mut m: Map<String, Value>| {
        let mut head = Map::new();
        head.insert("PackageIdentifier".into(), IDENTIFIER.into());
        head.insert("PackageVersion".into(), v.clone().into());
        head.append(&mut m);
        head.insert("ManifestType".into(), kind.into());
        head.insert("ManifestVersion".into(), MANIFEST_VERSION.into());
        head
    };
    let obj = |v: Value| match v {
        Value::Object(m) => m,
        _ => Map::new(),
    };
    let installer = |triple: &str, arch: &str| -> Result<Value, String> {
        let t = targets::find(triple)?;
        Ok(json!({
            "Architecture": arch,
            "InstallerUrl": targets::download_url(v, &t.archive(PACKAGE, v)),
            "InstallerSha256": input.sha256(triple).to_ascii_uppercase(),
        }))
    };
    Ok([
        base("version", obj(json!({"DefaultLocale": "en-US"}))),
        base(
            "installer",
            obj(json!({
                "InstallerType": "zip",
                "NestedInstallerType": "portable",
                "NestedInstallerFiles": [{"RelativeFilePath": format!("{BIN}.exe"), "PortableCommandAlias": ALIAS}],
                "Commands": [ALIAS],
                "Installers": [
                    installer("x86_64-pc-windows-msvc", "x64")?,
                    installer("aarch64-pc-windows-msvc", "arm64")?,
                ],
            })),
        ),
        base(
            "defaultLocale",
            obj(json!({
                "PackageLocale": "en-US",
                "Publisher": "jabbott-iii",
                "PublisherUrl": "https://github.com/jabbott-iii",
                "PackageName": PACKAGE,
                "PackageUrl": REPO_URL,
                "License": "MIT OR Apache-2.0",
                "ShortDescription": SHORT_DESCRIPTION,
                "Description": input.description,
                "Moniker": ALIAS,
                "Tags": ["ai", "agents", "mcp", "skills", "cli"],
                "ReleaseNotesUrl": format!("{REPO_URL}/releases/tag/v{v}"),
            })),
        ),
    ])
}

pub(super) fn generate(input: &Input, g: &mut Generated) -> Result<(), String> {
    for ((suffix, kind), map) in FILES.iter().zip(manifests(input)?) {
        let name = format!("{IDENTIFIER}{suffix}");
        let text = format!(
            "# Generated by `cargo xtask release channels` for {PACKAGE} {}.\n\
# yaml-language-server: $schema=https://aka.ms/winget-manifest.{kind}.{MANIFEST_VERSION}.schema.json\n\n{}",
            input.version,
            yaml(&map)?
        );
        g.dist.insert(format!("{name}.yaml"), Out::text(text));
        let json = serde_json::to_string_pretty(&Value::Object(map)).map_err(|e| e.to_string())?;
        g.work
            .insert(format!("winget-json/{name}.json"), Out::text(json + "\n"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::tests::input;
    use super::*;

    #[test]
    fn manifests_and_json_twins() {
        let mut g = Generated::default();
        generate(&input(false), &mut g).unwrap();
        for (suffix, _) in FILES {
            let name = format!("{IDENTIFIER}{suffix}");
            let yaml = String::from_utf8(g.dist[&format!("{name}.yaml")].bytes.clone()).unwrap();
            insta::assert_snapshot!(format!("winget{suffix}"), yaml);
            assert!(g.work.contains_key(&format!("winget-json/{name}.json")));
        }
    }

    #[test]
    fn scalars_are_quoted_when_yaml_would_reinterpret_them() {
        assert_eq!(scalar("1.2.3"), "1.2.3");
        assert_eq!(scalar("1.10"), "'1.10'");
        assert_eq!(scalar("yes"), "'yes'");
        assert_eq!(scalar("a: b"), "'a: b'");
        assert_eq!(scalar("it's"), "'it''s'");
        assert_eq!(
            scalar("Install only (rules, skills)"),
            "'Install only (rules, skills)'"
        );
        assert_eq!(scalar("https://github.com/x"), "https://github.com/x");
    }
}
