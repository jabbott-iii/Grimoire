//! `install.sh` and `install.ps1`: the templates in `packaging/install/` with the
//! version and the CLI archives' SHA-256s filled in (ADR-0009, ADR-0010).
//!
//! Placeholders are `@@VERSION@@` and `@@SHA256:{triple}@@`, inside single-quoted
//! strings so the templates lint as they stand. A target `--partial` didn't build
//! gets an empty hash, which the scripts refuse. Any other `@@…@@` is an error.

use super::{Generated, Input, MISSING_SHA256, Out};
use crate::release::targets;

/// Replaces every placeholder in `template`.
pub(crate) fn render(template: &str, input: &Input) -> Result<String, String> {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find("@@") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let end = after
            .find("@@")
            .ok_or_else(|| "an unterminated @@ placeholder".to_owned())?;
        let key = &after[..end];
        let value = if key == "VERSION" {
            input.version.clone()
        } else if let Some(triple) = key.strip_prefix("SHA256:") {
            targets::find(triple)?;
            match input.sha256(triple) {
                MISSING_SHA256 => String::new(),
                hex => hex.to_owned(),
            }
        } else {
            return Err(format!("unknown placeholder @@{key}@@"));
        };
        out.push_str(&value);
        rest = &after[end + 2..];
    }
    out.push_str(rest);
    Ok(out)
}

pub(super) fn generate(input: &Input, g: &mut Generated) -> Result<(), String> {
    for (name, template) in [
        ("install.sh", &input.install_sh),
        ("install.ps1", &input.install_ps1),
    ] {
        let text = render(template, input).map_err(|e| format!("packaging/install/{name}: {e}"))?;
        if !text.contains(&input.version) {
            return Err(format!("packaging/install/{name} has no @@VERSION@@"));
        }
        g.dist.insert(
            name.to_owned(),
            Out {
                bytes: text.into_bytes(),
                mode: 0o755,
            },
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::repo_root;
    use super::super::tests::input;
    use super::*;

    #[test]
    fn placeholders_are_filled_or_refused() {
        let full = input(false);
        let linux = full.sha256("x86_64-unknown-linux-musl").to_owned();
        assert_eq!(
            render(
                "v='@@VERSION@@' h='@@SHA256:x86_64-unknown-linux-musl@@'",
                &full
            )
            .unwrap(),
            format!("v='1.2.3' h='{linux}'")
        );
        let partial = input(true);
        assert_eq!(
            render("'@@SHA256:aarch64-pc-windows-msvc@@'", &partial).unwrap(),
            "''"
        );
        assert!(render("@@NOPE@@", &full).is_err());
        assert!(render("@@SHA256:x86_64-unknown-linux-gnu@@", &full).is_err());
        assert!(render("@@VERSION", &full).is_err());
    }

    /// The real templates name every CLI target exactly once and nothing else.
    #[test]
    fn the_templates_cover_every_target() {
        let root = repo_root();
        for name in ["install.sh", "install.ps1"] {
            let Ok(text) = std::fs::read_to_string(root.join("packaging/install").join(name))
            else {
                panic!("packaging/install/{name} is missing");
            };
            let mut i = input(false);
            let rendered = render(&text, &i).unwrap();
            let relevant: Vec<_> = targets::TARGETS
                .iter()
                .filter(|t| t.is_windows() == (name == "install.ps1"))
                .collect();
            for t in &relevant {
                let hex = i.sha256(t.triple).to_owned();
                assert_eq!(rendered.matches(&hex).count(), 1, "{name}: {}", t.triple);
            }
            i.version = "9.9.9".to_owned();
            assert!(render(&text, &i).unwrap().contains("9.9.9"));
        }
    }
}
