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
use agent_grimoire_model::{Anchor, ClientId, OutputSpec, RelPath, Strategy};
use agent_grimoire_resolve::Resolved;

use crate::text::code_span;
use crate::{Report, copy_tree, no_destination, rule_paths, source_text};

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
        (Anchor::Skills, Strategy::Copy) => copy_tree(resolved, spec, SKILLS),
        (anchor, _) => {
            no_destination(report, ClientId::AgentsMd, resolved, anchor);
            Vec::new()
        }
    }
}

/// A rule's `AGENTS.md` block. Codex writes the same block.
pub(crate) fn block(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<(RelPath, Content)> {
    Some((
        RelPath::new(AGENTS_MD).ok()?,
        Content::Block(block_text(resolved, spec, report)?),
    ))
}

/// A rule as block text: the rule, after a scope line when it is path-scoped,
/// because shared instruction files have no path scoping (`architecture.md` §3.1).
pub(crate) fn block_text(
    resolved: &Resolved<'_>,
    spec: &OutputSpec,
    report: &mut Report,
) -> Option<String> {
    let src = spec.src.as_ref()?;
    let body = source_text(resolved, src, report)?;
    let paths: Vec<String> = rule_paths(resolved).into_iter().map(code_span).collect();
    Some(if paths.is_empty() {
        body
    } else {
        format!(
            "_Applies when working on files matching {}._\n\n{body}",
            paths.join(", ")
        )
    })
}
