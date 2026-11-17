//! Scoop: `agent-grimoire.json` for `jabbott-iii/scoop-bucket` (SCOOP and the
//! brew-scoop Phase 5 research in docs/specs.md, ADR-0009). The bucket's sync workflow
//! copies it from each attested release, so it has no `checkver`/`autoupdate`.
//!
//! Scoop's v0.6.0 schema requires `version`, `homepage` and `license`; bucket files use
//! 4-space indentation, no BOM and a final newline.

use serde::Serialize as _;
use serde_json::json;

use super::Input;
use crate::release::targets::{self, BIN, PACKAGE, REPO_URL};

pub(super) fn manifest(input: &Input) -> Result<String, String> {
    let arch = |triple: &str| -> Result<serde_json::Value, String> {
        let t = targets::find(triple)?;
        Ok(json!({
            "url": targets::download_url(&input.version, &t.archive(PACKAGE, &input.version)),
            "hash": input.sha256(triple),
        }))
    };
    let value = json!({
        "version": input.version,
        "description": input.description,
        "homepage": REPO_URL,
        // Scoop separates a choice of licences with `|` (App-Manifests wiki, `license`;
        // `scoop info` splits on `|` and `,`), not SPDX's `OR`.
        "license": "MIT|Apache-2.0",
        "architecture": {
            "64bit": arch("x86_64-pc-windows-msvc")?,
            "arm64": arch("aarch64-pc-windows-msvc")?,
        },
        "bin": format!("{BIN}.exe"),
    });
    let mut out = Vec::new();
    let mut ser = serde_json::Serializer::with_formatter(
        &mut out,
        serde_json::ser::PrettyFormatter::with_indent(b"    "),
    );
    value.serialize(&mut ser).map_err(|e| e.to_string())?;
    let mut text = String::from_utf8(out).map_err(|e| e.to_string())?;
    text.push('\n');
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::super::tests::input;

    #[test]
    fn manifest() {
        insta::assert_snapshot!(super::manifest(&input(false)).unwrap());
    }
}
