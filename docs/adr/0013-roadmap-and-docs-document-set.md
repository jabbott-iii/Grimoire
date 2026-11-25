# ADR-0013: Adopt the roadmap-and-docs document set

- **Status:** Accepted (the maintainer asked for the documentation to be rebuilt to follow the `build-roadmap-and-docs` skill, with anything outside its convention deleted, and approved the blueprint's ten defaults on 2026-11-24)
- **Date:** 2026-11-24
- **Related:** `.claude/rules/roadmap-and-docs.md`, the rule that describes the document set; [ADR-0012](0012-crate-documentation.md), the crate docs, which stay part of the specs

## Context

On 2026-11-24 Grimoire adopted five rules from Doctrina, the maintainer's library of portable rules and skills for coding agents (commit `baf29f0`). One of them, `roadmap-and-docs.md`, describes a set of planning and design documents:
- numbered core documents, from `00-overview.md` to `05-release.md`;
- a roadmap of milestones, each made of tasks with an ID and the commands that check them, followed by a backlog;
- a decision log;
- a spec for each part of the system;
- an optional feature list.

When the rule was adopted, its "Project details" kept Grimoire's existing documents and their names:
- `PLAN.md` (534 lines), a plan in seven phases with no task IDs;
- `docs/architecture.md` (897 lines), the whole design in one file;
- `docs/security-model.md` (517 lines), `docs/versioning.md` (258 lines) and `docs/release.md` (636 lines).

The rule and its skill say to merge into existing documents and never to restructure one only to fit. On 2026-11-24 the maintainer asked instead for the documentation to be recreated in the skill's shape, and for anything outside that shape to be deleted.

Four facts limited what could go:
- `docs/cli.md` and `docs/diagnostics.md` are checked documents: tests fail when they and the code disagree. Deleting them would mean deleting those tests, which `AGENTS.md` forbids.
- `docs/specs.md` is the register that `AGENTS.md`'s first working rule requires. Its entries are cited by 44 files outside Markdown.
- Files in `docs/adr/` are linked from 39 files outside Markdown, and the catalog's own `adr-writer` skill and `project-docs` server expect one file per record.
- No release exists, so no crates.io, docs.rs or PyPI page links to any of these documents yet.

## Decision

**Decision** (the maintainer approved all ten defaults of the proposed blueprint on 2026-11-24):

1. **Six numbered core documents** replace the old design and policy files:

   | Document | Its job | Taken mostly from |
   |---|---|---|
   | `docs/00-overview.md` | Purpose, goals, non-goals, budgets, assumptions, scope, top risks | `README.md`, `architecture.md` §1, `PLAN.md` §7 |
   | `docs/01-architecture.md` | Principles, layers, layout, control flow, the security model | `architecture.md` §2 and §4, `security-model.md` §1–§3 and §7 |
   | `docs/02-dependency-policy.md` | Third-party code, exceptions, build-or-buy register | `deny.toml`, `security-model.md` §6d, [ADR-0010](0010-language-policy-exceptions.md) |
   | `docs/03-platforms.md` | Targets, the order work moves through them, platform invariants | `PLAN.md` §4, `ci.yml` |
   | `docs/04-quality.md` | Test layers and tools, checks, CI, review | `testing.md`, `CONTRIBUTING.md`, `security-model.md` §6d |
   | `docs/05-release.md` | Versioning, build profiles, the release checklist, recovery, installing and verifying | `versioning.md`, `release.md` §2–§6 |

2. **`docs/roadmap.md` replaces `PLAN.md`.** Milestones M1 to M7 are the former Phases 1 to 7, recorded as built, one task for each part they built. M8 is the first public release, planned in full. Task IDs read `M<milestone>.<task>`. The dated decisions in `PLAN.md` become dated notes on the tasks they shaped, and its policy text moves to `05-release.md`. Later work is in the backlog, each item with a trigger.
3. **Specs live in `docs/subsystems/`**, one file for each part: `model`, `catalog`, `resolve`, `install`, `adapters`, `marketplace`, `cli`, `mcp`, `assets`, `xtask`, `release` and `container-image`. The rest of `architecture.md` and `security-model.md` is divided among them.
4. **Kept, outside the rule:**
   - the user reference (`docs/cli.md`, `docs/diagnostics.md`, `docs/configuration.md`, `docs/container.md`), because the rule covers planning and design documents only;
   - `docs/specs.md`, the external-format register;
   - `docs/adr/`, one file per record.
5. **Deleted once their content moved:** `PLAN.md`, `docs/architecture.md`, `docs/security-model.md`, `docs/versioning.md` and `docs/release.md`. Every link to them, in code, schemas, workflows and documents, now points to where the content went. Accepted records had only their links and section references changed, never their decisions.
6. **Tasks are ticked from evidence.** Each M1–M7 task was ticked only when its `Verify` commands passed on Linux in the session that wrote the roadmap. Checks that need macOS, Windows or GitHub were read from the latest CI, Security and Release runs on `main` with `gh run list`, which changes nothing, and each is cited by its run ID.
7. **No budgets and no feature list yet.** Nothing measures performance (`testing.md`'s "Performance budgets" is "none"), so the overview says so and the backlog holds the item. The README's "What Grimoire does and does not do" does the feature list's job.
8. **The rule's general text matches Doctrina again.** Its one local change, `PLAN.md` in its `paths`, is gone, because the roadmap is now in `docs/`.

## Consequences

Good:
- A new contributor can read `00` to `05` in order, then the spec for the part they are changing.
- Every task names its `Verify` commands, so whether it is done can be checked rather than taken on trust.
- Each fact has one place: design in the specs, policy in the core documents, status in the roadmap.

Bad:
- Saved links to the five deleted files break. GitHub doesn't redirect a moved file. No release has linked them, so the break stays inside the repository's history, its issues and pull requests.
- The change is large: 20 new files (this record among them), 5 deleted, and links or references changed in about 90 others. The JSON Schemas are regenerated, because their descriptions come from doc comments that link to the design.
- Older text still says "Phase 4" where the roadmap says M4. The roadmap's milestone titles name their phase, so the two can be matched.

Follow-up work:
- M8.2 (in `docs/roadmap.md`) adds the check that the rule expects and the project lacks: a test that the roadmap's task IDs, `Depends` lines and backlog triggers, and the cited records, hold together. It is in Rust, under the language policy in ADR-0010.

## Alternatives considered

- **Keep the existing names**, as the adoption first did: no links break, but it is what the maintainer asked to change.
- **One file, `docs/decisions.md`, for every record**, in the rule's single-file template: it would reformat twelve accepted records and change about 50 links, and the catalog's ADR tools expect one file per record.
- **`docs/specs/` for the specs**, with the register renamed: two "specs" no longer collide, but 44 more files change.
- **Delete the user reference too:** it would remove the tests that keep `docs/cli.md` and `docs/diagnostics.md` true to the code, which `AGENTS.md` forbids.
- **One first milestone for all the work done so far**, as the skill suggests for existing code: it would lose each phase's dates and reviews, which M1 to M7 keep.

**Revisit when:**
- Doctrina's `roadmap-and-docs.md` changes the document set. Copy the new text, then bring these documents into its shape in a change of their own.
- The first release is published. From then on, crates.io, docs.rs and PyPI pages link to these files, so moving one needs a new record.
