//! The `agents-md` adapter: the portable target read by many clients.
//!
//! | Anchor | Path | Written as |
//! |---|---|---|
//! | `instructions` | `AGENTS.md` | a managed block per rule (`AGENTSMD`) |
//! | `skills:` | `.agents/skills/<path>` | a byte-for-byte copy (`SKILLS`) |
//!
//! `AGENTS.md` has no path scoping, so a path-scoped rule's block starts with a
//! line saying which files it applies to (`architecture.md` §3.1).

use agent_grimoire_install::plan::Content;
use agent_grimoire_model::{Anchor, Diagnostic, OutputSpec, RelPath, Strategy};
use agent_grimoire_resolve::Resolved;

use crate::text::code_span;
use crate::{Report, claude_code, source_text};

/// The shared instruction file.
pub const AGENTS_MD: &str = "AGENTS.md";
const SKILLS: &str = ".agents/skills";

/// Renders one declared output.
pub(crate) fn render(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Vec<(RelPath, Content)> {
    match (spec.to.anchor(), spec.strategy) {
        (Anchor::Instructions, Strategy::Block) => {
            block(resolved, spec, report).into_iter().collect()
        }
        (Anchor::Skills, Strategy::Copy) => claude_code::copy(resolved, spec, SKILLS),
        (anchor, _) => {
            report.errors.push(Diagnostic::error(
                "adapter/unsupported",
                format!(
                    "agents-md cannot write `{}` to `{anchor}` this way",
                    resolved.id()
                ),
            ));
            Vec::new()
        }
    }
}

fn block(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let paths: Vec<String> = resolved
        .asset
        .manifest
        .rule
        .as_ref()
        .map(|r| r.paths.iter().map(|g| code_span(g.as_str())).collect())
        .unwrap_or_default();
    let text = if paths.is_empty() {
        body
    } else {
        format!(
            "_Applies when working on files matching {}._\n\n{body}",
            paths.join(", ")
        )
    };
    Some((RelPath::new(AGENTS_MD).ok()?, Content::Block(text)))
}
