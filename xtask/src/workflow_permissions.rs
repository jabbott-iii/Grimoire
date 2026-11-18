//! A job that calls a reusable workflow in this repository must grant every
//! permission that any job in the called workflow asks for.
//!
//! GitHub checks this when a run starts, for every job of the called workflow,
//! including a job whose `if:` would skip it, and refuses the whole run when a
//! job asks for more than the call grants ("The nested job 'image' is requesting
//! 'contents: read, packages: write', but is only allowed 'contents: none,
//! packages: none'"; SIGN-P6 in `docs/specs.md`). zizmor and actionlint don't
//! check it. Without this test, the first sign of a wrong grant is every run
//! of the calling workflow failing to start, pull requests' included, once the
//! change is pushed.
//!
//! The workflows are read as the one block-style shape this repository writes:
//! two-space indentation, bare `key:` names, job ids under a block `jobs:`,
//! each job's keys at indent 4, and `permissions` either `{}` or one
//! `scope: none|read|write` line per scope. A line this reader looks at that
//! has another shape (a quoted key, a space before the colon, flow style, a
//! list item, odd indentation under `jobs:`) fails the test rather than being
//! skipped. Lines deeper than a job's keys, such as steps, aren't read.

use std::collections::BTreeMap;
use std::fmt;

/// One scope's level. A scope a block leaves out is `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
    None,
    Read,
    Write,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::None => "none",
            Self::Read => "read",
            Self::Write => "write",
        })
    }
}

type Permissions = BTreeMap<String, Level>;

#[derive(Debug, Default)]
struct Job {
    id: String,
    /// `None` when the job sets no `permissions`.
    permissions: Option<Permissions>,
    /// The job's `uses:`, for a job that calls a reusable workflow.
    uses: Option<String>,
}

#[derive(Debug, Default)]
struct Workflow {
    permissions: Option<Permissions>,
    jobs: Vec<Job>,
}

/// A line's indentation and its text without a trailing comment, or `None` for
/// a blank or comment-only line. Only ` #` starts a comment, as in YAML.
fn content(line: &str) -> Option<(usize, &str)> {
    let indent = line.len() - line.trim_start_matches(' ').len();
    let text = &line[indent..];
    if text.is_empty() || text.starts_with('#') {
        return None;
    }
    let text = text.find(" #").map_or(text, |i| &text[..i]).trim_end();
    Some((indent, text))
}

/// Splits a `key: value` line whose key is a bare name, as this repository
/// writes them, into the key and its trimmed inline value (`""` when the key
/// ends the line). `None` for anything else: a quoted key, a space before the
/// colon, a list item or flow style.
fn key_value(text: &str) -> Option<(&str, &str)> {
    let (key, rest) = text.split_once(':')?;
    let bare = !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    (bare && (rest.is_empty() || rest.starts_with(' '))).then(|| (key, rest.trim()))
}

/// Reads a `permissions` value: `{}` or `{ }` inline, or the block of
/// `scope: level` lines that starts after `lines[*next]` at `indent`.
fn permissions(
    inline: &str,
    indent: usize,
    lines: &[(usize, usize, &str)],
    next: &mut usize,
) -> Result<Permissions, String> {
    let mut found = Permissions::new();
    if !inline.is_empty() {
        let inner = inline
            .strip_prefix('{')
            .and_then(|v| v.strip_suffix('}'))
            .ok_or_else(|| format!("`permissions: {inline}` isn't `{{}}` or a block"))?;
        if !inner.trim().is_empty() {
            return Err(format!("`permissions: {inline}` isn't `{{}}` or a block"));
        }
        return Ok(found);
    }
    while let Some(&(number, child, text)) = lines.get(*next) {
        if child <= indent {
            break;
        }
        if child != indent + 2 {
            return Err(format!(
                "line {number}: unexpected indentation in `permissions`"
            ));
        }
        let (scope, level) = key_value(text)
            .ok_or_else(|| format!("line {number}: `{text}` isn't `scope: level`"))?;
        let level = match level {
            "none" => Level::None,
            "read" => Level::Read,
            "write" => Level::Write,
            other => return Err(format!("line {number}: unknown level `{other}`")),
        };
        if found.insert(scope.to_owned(), level).is_some() {
            return Err(format!("line {number}: `{scope}` is set twice"));
        }
        *next += 1;
    }
    if found.is_empty() {
        return Err("an empty `permissions:` block".to_owned());
    }
    Ok(found)
}

fn parse(text: &str) -> Result<Workflow, String> {
    // YAML also breaks lines at a lone CR, and YAML 1.1 parsers at NEL, LS and
    // PS. `lines()` doesn't, so whatever followed one would be read as part of
    // the line before it, comments included, and skipped.
    for (i, line) in text.lines().enumerate() {
        if line.contains(['\r', '\u{85}', '\u{2028}', '\u{2029}']) {
            return Err(format!(
                "line {}: a line break other than LF or CRLF",
                i + 1
            ));
        }
    }
    let lines: Vec<(usize, usize, &str)> = text
        .lines()
        .enumerate()
        .filter_map(|(i, l)| content(l).map(|(indent, t)| (i + 1, indent, t)))
        .collect();
    let mut workflow = Workflow::default();
    let mut seen_jobs = false;
    let mut in_jobs = false;
    // Set after a job id: the job's first key must follow at indent 4.
    let mut body_expected = false;
    let mut next = 0;
    while let Some(&(number, indent, text)) = lines.get(next) {
        next += 1;
        if text.contains('\t') {
            return Err(format!("line {number}: a tab"));
        }
        if body_expected && indent != 4 {
            return Err(format!(
                "line {number}: a job's keys must start at indent 4"
            ));
        }
        body_expected = false;
        match indent {
            0 => {
                let (key, inline) = key_value(text)
                    .ok_or_else(|| format!("line {number}: `{text}` isn't a top-level `key:`"))?;
                in_jobs = key == "jobs";
                if in_jobs {
                    if seen_jobs || !inline.is_empty() {
                        return Err(format!("line {number}: `jobs` must be one block"));
                    }
                    seen_jobs = true;
                } else if key == "permissions" {
                    if workflow.permissions.is_some() {
                        return Err(format!("line {number}: `permissions` is set twice"));
                    }
                    workflow.permissions = Some(
                        permissions(inline, 0, &lines, &mut next)
                            .map_err(|e| format!("line {number}: {e}"))?,
                    );
                }
            }
            1 | 3 if in_jobs => {
                return Err(format!("line {number}: odd indentation under `jobs`"));
            }
            2 if in_jobs => {
                let (id, _) = key_value(text)
                    .filter(|(_, inline)| inline.is_empty())
                    .ok_or_else(|| format!("line {number}: `{text}` isn't a job id"))?;
                workflow.jobs.push(Job {
                    id: id.to_owned(),
                    ..Job::default()
                });
                body_expected = true;
            }
            4 if in_jobs => {
                let job = workflow
                    .jobs
                    .last_mut()
                    .ok_or_else(|| format!("line {number}: a job key before any job"))?;
                let (key, inline) = key_value(text)
                    .ok_or_else(|| format!("line {number}: `{text}` isn't a job `key:`"))?;
                match key {
                    "permissions" => {
                        if job.permissions.is_some() {
                            return Err(format!("line {number}: `permissions` is set twice"));
                        }
                        job.permissions = Some(
                            permissions(inline, 4, &lines, &mut next)
                                .map_err(|e| format!("line {number}: {e}"))?,
                        );
                    }
                    "uses" => job.uses = Some(inline.to_owned()),
                    _ => {}
                }
            }
            _ => {}
        }
    }
    if body_expected {
        return Err("the last job has no keys".to_owned());
    }
    if workflow.jobs.is_empty() {
        return Err("no jobs".to_owned());
    }
    Ok(workflow)
}

/// The file in `.github/workflows/` that a job-level `uses:` names in this
/// repository, or an error for a reusable workflow elsewhere, whose jobs this
/// check can't read.
fn local_target(uses: &str) -> Result<&str, String> {
    uses.strip_prefix("./.github/workflows/")
        .or_else(|| uses.strip_prefix("$/.github/workflows/"))
        .filter(|file| !file.contains('/') && !file.contains('@'))
        .ok_or_else(|| format!("`uses: {uses}` isn't a workflow in this repository"))
}

/// Every permission that `target` asks for beyond what `granted` allows, as
/// one message each, for the call from job `job` of `from`. A job without
/// `permissions` is taken to get the called workflow's top-level ones, or,
/// without those, what the call grants, so it can't ask for more. GitHub's docs
/// don't say so (UNVERIFIED, SIGN-P6); this repository's called jobs all set
/// their own, which the real-file test checks. The top-level block is checked
/// too, even when every job sets its own: whether GitHub checks it then is
/// UNVERIFIED, so this check assumes it does.
fn excess(
    from: &str,
    job: &str,
    name: &str,
    granted: &Permissions,
    target: &Workflow,
) -> Vec<String> {
    let mut asks: Vec<(String, &Permissions)> = Vec::new();
    if let Some(top) = &target.permissions {
        asks.push(("the top-level `permissions`".to_owned(), top));
    }
    for nested in &target.jobs {
        if let Some(own) = &nested.permissions {
            asks.push((format!("job `{}`", nested.id), own));
        }
    }
    let mut problems = Vec::new();
    for (who, asked) in asks {
        for (scope, &level) in asked {
            let allowed = granted.get(scope).copied().unwrap_or(Level::None);
            if level > allowed {
                problems.push(format!(
                    "{from}: job `{job}` calls {name}, where {who} asks for `{scope}: {level}`, but the call grants `{scope}: {allowed}`"
                ));
            }
        }
    }
    problems
}

/// Every call from `workflows` to a reusable workflow in the same set that
/// grants less than a called job asks for. A calling job must set
/// `permissions` itself or inherit its workflow's: GitHub's default token
/// permissions are a repository setting, which this check can't read. A
/// calling job without its own is taken to pass its workflow's top-level
/// block; GitHub's docs say only "the default permissions" (UNVERIFIED,
/// SIGN-P6). Both of this repository's calling jobs set their own.
fn problems(workflows: &BTreeMap<String, Workflow>) -> Vec<String> {
    let mut problems = Vec::new();
    for (file, workflow) in workflows {
        for job in &workflow.jobs {
            let Some(uses) = &job.uses else { continue };
            let target = match local_target(uses) {
                Ok(target) => target,
                Err(e) => {
                    problems.push(format!("{file}: job `{}`: {e}", job.id));
                    continue;
                }
            };
            let Some(granted) = job.permissions.as_ref().or(workflow.permissions.as_ref()) else {
                problems.push(format!(
                    "{file}: job `{}` calls {target} without `permissions`, at the job or the top",
                    job.id
                ));
                continue;
            };
            match workflows.get(target) {
                Some(called) => problems.extend(excess(file, &job.id, target, granted, called)),
                None => problems.push(format!(
                    "{file}: job `{}` calls {target}, which doesn't exist",
                    job.id
                )),
            }
        }
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(files: &[(&str, &str)]) -> BTreeMap<String, Workflow> {
        files
            .iter()
            .map(|(name, text)| ((*name).to_owned(), parse(text).unwrap()))
            .collect()
    }

    const CALLER: &str = "\
name: Release
on: push
permissions: {}
jobs:
  build:
    runs-on: ubuntu-24.04
    permissions:
      contents: read # a comment
    steps:
      - run: |
          echo 'permissions: write-all'
  sign-files:
    if: always()
    permissions:
      id-token: write
      attestations: write
    uses: ./.github/workflows/sign.yml # zizmor: ignore[self-repository]
    with:
      target: files
";

    /// sign.yml as it was in `a052cad`, whose `image` job GitHub refused.
    const BOTH: &str = "\
on:
  workflow_call:
permissions: {}
env:
  permissions: x
jobs:
  files:
    if: inputs.target == 'files'
    permissions:
      id-token: write
      attestations: write
  image:
    if: inputs.target == 'image'
    permissions:
      contents: read
      packages: write
      id-token: write
      attestations: write
";

    #[test]
    fn refuses_a_called_job_that_asks_for_more_than_the_call_grants() {
        let found = problems(&set(&[("release.yml", CALLER), ("sign.yml", BOTH)]));
        assert_eq!(
            found,
            [
                "release.yml: job `sign-files` calls sign.yml, where job `image` asks for `contents: read`, but the call grants `contents: none`",
                "release.yml: job `sign-files` calls sign.yml, where job `image` asks for `packages: write`, but the call grants `packages: none`",
            ]
        );
    }

    #[test]
    fn accepts_a_call_that_grants_every_job_what_it_asks_for() {
        let files_only = BOTH.split("  image:").next().unwrap();
        assert_eq!(
            problems(&set(&[("release.yml", CALLER), ("sign.yml", files_only)])),
            Vec::<String>::new()
        );
    }

    #[test]
    fn the_called_workflows_top_level_permissions_count() {
        // A job without permissions gets these, and the block is checked even when
        // every job sets its own.
        for jobs in ["  one:\n    runs-on: x\n", "  one:\n    permissions: {}\n"] {
            let called =
                format!("on: workflow_call\npermissions:\n  packages: read\njobs:\n{jobs}");
            let found = problems(&set(&[("release.yml", CALLER), ("sign.yml", &called)]));
            assert_eq!(found.len(), 1, "{found:?}");
            assert!(
                found[0].contains("the top-level `permissions` asks for `packages: read`"),
                "{found:?}"
            );
        }
        let inherits = "on: workflow_call\njobs:\n  one:\n    runs-on: x\n";
        assert_eq!(
            problems(&set(&[("release.yml", CALLER), ("sign.yml", inherits)])),
            Vec::<String>::new()
        );
    }

    #[test]
    fn a_read_grant_doesnt_cover_write() {
        let caller = CALLER.replace(
            "      attestations: write\n    uses",
            "      attestations: read\n    uses",
        );
        let found = problems(&set(&[("release.yml", &caller), ("sign.yml", BOTH)]));
        assert!(
            found.iter().any(|p| p.contains("`attestations: write`")
                && p.contains("grants `attestations: read`")),
            "{found:?}"
        );
    }

    #[test]
    fn calls_it_cant_check_are_problems() {
        let remote = CALLER.replace(
            "./.github/workflows/sign.yml",
            "octo/repo/.github/workflows/x.yml@v1",
        );
        let found = problems(&set(&[("release.yml", &remote)]));
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(
            found[0].contains("isn't a workflow in this repository"),
            "{found:?}"
        );
        let missing = problems(&set(&[("release.yml", CALLER)]));
        assert!(missing[0].contains("doesn't exist"), "{missing:?}");
        let ungranted = CALLER.replace("permissions: {}\n", "");
        let ungranted = ungranted.replace(
            "    permissions:\n      id-token: write\n      attestations: write\n",
            "",
        );
        let found = problems(&set(&[("release.yml", &ungranted), ("sign.yml", BOTH)]));
        assert!(found[0].contains("without `permissions`"), "{found:?}");
    }

    #[test]
    fn shapes_it_doesnt_know_are_errors() {
        // Each is otherwise a valid one-job workflow, so the error comes from the
        // shape named, not from a missing job.
        let job = "jobs:\n  one:\n    runs-on: x\n";
        for (shape, error) in [
            (
                format!("permissions: read-all\n{job}"),
                "isn't `{}` or a block",
            ),
            (
                format!("permissions: {{ contents: read }}\n{job}"),
                "isn't `{}` or a block",
            ),
            (
                format!("permissions:\n{job}"),
                "an empty `permissions:` block",
            ),
            (
                "jobs:\n  one:\n    permissions:\n      contents: maybe\n".to_owned(),
                "unknown level",
            ),
            (
                "jobs:\n  one:\n    permissions:\n        contents: read\n".to_owned(),
                "unexpected indentation",
            ),
            (
                "jobs:\n  one:\n    permissions:\n      contents: read\n      contents: write\n"
                    .to_owned(),
                "set twice",
            ),
            (
                "jobs:\n  one:\n    permissions:\n      \"contents\": read\n".to_owned(),
                "isn't `scope: level`",
            ),
            (
                "jobs:\n  one two:\n    runs-on: x\n".to_owned(),
                "isn't a job id",
            ),
            (
                "jobs:\n  \"one\":\n    runs-on: x\n".to_owned(),
                "isn't a job id",
            ),
            (
                "jobs:\n    permissions: {}\n".to_owned(),
                "a job key before any job",
            ),
            // A called job's permissions hidden from a lenient reader.
            (
                "jobs:\n  one:\n      permissions:\n        packages: write\n".to_owned(),
                "must start at indent 4",
            ),
            (
                "jobs:\n  one:\n     runs-on: x\n".to_owned(),
                "must start at indent 4",
            ),
            (
                "jobs:\n   one:\n    runs-on: x\n".to_owned(),
                "odd indentation",
            ),
            (
                "jobs:\n  one:\n    \"permissions\":\n      packages: write\n".to_owned(),
                "isn't a job `key:`",
            ),
            (
                "jobs:\n  one:\n    permissions :\n      packages: write\n".to_owned(),
                "isn't a job `key:`",
            ),
            (
                "jobs:\n  one:\n    - runs-on: x\n".to_owned(),
                "isn't a job `key:`",
            ),
            (
                "\"jobs\":\n  one:\n    runs-on: x\n".to_owned(),
                "isn't a top-level `key:`",
            ),
            (
                "jobs: { one: { permissions: { packages: write } } }\n".to_owned(),
                "must be one block",
            ),
            (format!("{job}{job}"), "must be one block"),
            ("jobs:\n  one:\n".to_owned(), "the last job has no keys"),
            ("on: push\n".to_owned(), "no jobs"),
            // A lone CR, or NEL, LS or PS, would hide what follows it.
            (
                format!("{job}  two:\r    permissions:\r      packages: write\n"),
                "a line break other than",
            ),
            (format!("# note\r{job}"), "a line break other than"),
            (
                format!("{job}  two:\u{85}    runs-on: x\n"),
                "a line break other than",
            ),
            (
                format!("{job}  two:\u{2028}    runs-on: x\n"),
                "a line break other than",
            ),
        ] {
            match parse(&shape) {
                Ok(_) => panic!("{shape:?} parsed"),
                Err(e) => assert!(e.contains(error), "{shape:?}: {e}"),
            }
        }
        assert!(parse(job).is_ok());
        assert!(parse(&job.replace('\n', "\r\n")).is_ok(), "CRLF");
    }

    /// The check itself, over every workflow in `.github/workflows/`.
    #[test]
    fn every_reusable_workflow_call_grants_what_its_jobs_ask_for() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(".github/workflows");
        let mut workflows = BTreeMap::new();
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if !std::path::Path::new(&name)
                .extension()
                .is_some_and(|e| e.eq_ignore_ascii_case("yml") || e.eq_ignore_ascii_case("yaml"))
            {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let workflow = parse(&text).unwrap_or_else(|e| panic!("{name}: {e}"));
            workflows.insert(name, workflow);
        }
        // The set the check is for. If one goes, this test should change with it.
        for (file, job) in [("release.yml", "sign-files"), ("release.yml", "sign-image")] {
            assert!(
                workflows[file]
                    .jobs
                    .iter()
                    .any(|j| j.id == job && j.uses.is_some()),
                "{file} has no reusable-workflow job `{job}`"
            );
        }
        // Every job of a called workflow sets its own `permissions`, as this
        // repository's workflows do. A job read without them would ask for
        // nothing, so this also shows the reader found a block in every job it
        // read. (A job it didn't read at all would need a shape `parse` refuses.)
        for workflow in workflows.values() {
            for job in &workflow.jobs {
                let Some(target) = job.uses.as_deref().and_then(|u| local_target(u).ok()) else {
                    continue;
                };
                let Some(called) = workflows.get(target) else {
                    continue;
                };
                for nested in &called.jobs {
                    assert!(
                        nested.permissions.is_some(),
                        "{target}: job `{}` sets no `permissions`",
                        nested.id
                    );
                }
            }
        }
        let found = problems(&workflows);
        assert_eq!(found, Vec::<String>::new(), "{}", found.join("\n"));
    }
}
