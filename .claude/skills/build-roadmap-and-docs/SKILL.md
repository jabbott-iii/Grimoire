---
name: build-roadmap-and-docs
description: "Build a project's planning and design documents from a description or its code: overview, core documents, roadmap, decision records, specs, and feature list."
argument-hint: "[a project description, or one piece such as \"roadmap\"; empty builds what is missing]"
disable-model-invocation: true
---

# Build the roadmap and the documents

Requested: `$ARGUMENTS`

- **A description of the project,** such as "a 2D game engine in Rust for Linux, Windows, and
  macOS": plan the project from the description, and from its code if it has any.
- **One piece:** `overview`, `core` (the other core documents), `decisions`, `specs`, `roadmap`,
  `features`, or `guide` (the agent guide, the update map, and the status locations): build that
  piece, and any piece it needs that is missing.
- **Empty:** build every piece that is missing, from the repository's code and records.

This skill builds the documents that `roadmap-and-docs.md` describes, in the shapes it gives: the
numbered core documents, a roadmap of milestones and checkable tasks with a backlog, the decision
records, a spec for each part of the system, and the feature list. You survey and draft; the user
decides the goals, the milestones, and the decisions.

Bold names are values from the "Project details" of the adopted rules:

- **Docs folder**, **Core documents**, **Specs**, **Task IDs**, **Environments**, **Definition of
  done**, **Decision log**, **Feature list**, **Status locations**, **Checked documents**,
  **Agent guide**, **Update map**, **Diagrams**, and **Gate evidence** from
  `roadmap-and-docs.md`
- **Git access** and **Changes that travel together** from `commits.md`
- **Check commands** and **Test layout** from the language rules, where they are adopted
- **Approved exceptions** from `dependencies.md`, where it is adopted

The text follows `writing.md`. Where `dependencies.md` or `testing.md` is adopted, the core
documents point to it instead of repeating it. If the repository hasn't adopted
`roadmap-and-docs.md`, say so and stop.

## Ground rules

These hold for the whole run, and override anything a later step seems to allow:

- **The user decides the plan.** The goals, non-goals, budgets, and assumptions, the milestones
  and their order, and which decisions are accepted are the user's, as `roadmap-and-docs.md`
  says. Propose them, and write nothing that depends on them until the user approves.
- **Draft decisions as `Proposed`.** A record is `Accepted` only when the user makes or approves
  the decision in this conversation, and the record says so, with the date.
- **Write only what you can confirm.** Call something built only after you have found it in the
  code and seen its tests pass. Take numbers from a measurement or from the user, and versions
  from the manifests and lock files. Where a fact isn't known, write the question, or the task
  that will settle it, instead of a guess.
- **Keep built and planned apart.** Write what exists in the present tense. For everything else,
  name the task that will build it.
- **Merge; never replace.** Keep every existing document, task ID, and record number. Add what is
  missing, but don't restructure an existing document only to fit the rule's shape. Never
  rewrite a finished task or an accepted record, and leave statements that no longer match the
  code to `/fix-doc-drift`.
- **Give each fact one place.** Put each fact in the document whose job it is, and link to it
  from the others.
- **Build documents, not tools.** When a check the rule expects doesn't exist yet, such as the
  check that keeps the **Feature list** in step with the roadmap, plan it as a roadmap task
  instead of writing it here.
- **Build in order.** Write the pieces in the order of step 5, and check each before the next, so
  a run that stops leaves complete documents and a later run continues from the first missing
  piece.
- **Never put a secret in a document**, not even in an example.

## 1. Set the scope

1. List the pieces that exist and the pieces that are missing, from the **Docs folder** and the
   rest of the "Project details" of `roadmap-and-docs.md`.
2. For one piece, add the missing pieces it needs. The roadmap needs the specs that its tasks
   point to, and every piece needs the records it cites.
3. If every piece already exists, say so, suggest `/fix-doc-drift` for any that may be out of
   date, and stop.
4. Tell the user in one or two sentences what you will build. Then make a task list whose last
   item is "Check and report".

## 2. Survey the repository

Read before you plan. Skip what doesn't exist yet, such as the code of a new project.

- **The documents:** the **Docs folder**, the **Agent guide**, `README.md`, `CONTRIBUTING.md`,
  and any planning notes or issues the user points you to. On GitHub, `gh issue list` reads the
  open issues without changing anything.
- **The parts:** the manifests and the source layout. Each crate, package, module, or app is a
  candidate for a spec. Note what each part depends on, for the architecture's layers.
- **The checks:** the tests, where the **Test layout** says they live, the **Check commands**,
  the full check that the **Definition of done** names, and the CI workflows. Run the checks, so
  you know what passes today.
- **The history:** `git log`, the tags, and the releases, for dates and versions. A decision that
  the code already shows, such as a chosen format, a dropped platform, or a dependency, becomes
  a record for the user to accept.
- **The numbers:** where the project already measures something, such as a benchmark or a size
  check, run it, and keep the number with the hardware it ran on. The user sets budgets against
  these numbers.

Keep a table of what you found: each part, what exists, the tests that cover it, and whether it
is built, partly built, or only planned.

## 3. Ask what only the user can answer

Ask in one message, and propose a default for each question, so the user can accept it in a word.
Ask only the questions whose answers change the documents:

- what the project is, for whom, and the one rule that shapes the rest
- the goals, each one checkable, and the non-goals
- each budget, as a number with its unit and the hardware it is measured on, or which task will
  choose the hardware
- the platforms, and the order work moves through the **Environments**
- how far ahead to plan in full; work beyond that goes to the backlog, with a trigger
- whether milestones end in gates or in free-form exit criteria, which the **Gate evidence**
  records
- which **Core documents** to keep, and whether to keep a **Feature list**
- a value for each value in the "Project details" of `roadmap-and-docs.md` that still reads
  `[FILL IN: …]`

## 4. Propose the blueprint

Show the user the whole plan before you write any document:

1. **Project details:** the proposed value for each one still unfilled.
2. **Documents:** each file to create, and each existing file to extend, with its job.
3. **Overview:** the goals, the non-goals, the budgets, the assumptions, the scope estimate, and
   the top risks.
4. **Decision records:** for each, its title, its decision in one line, and its status:
   `Proposed`, or `Accepted` if the user approves it now. List the decisions you found in the
   code with the rest.
5. **Specs:** one row for each part, with its file, whether it is built or planned, and the
   records that shaped it.
6. **Milestones:** for each, its goal (what a user can see or use when it ends), its tasks by ID
   and title, each task's `Depends`, and its gate or exit criteria. When code already exists,
   propose a first milestone that records what is built, as step 5 describes.
7. **Backlog:** each item, with its trigger.
8. **Features:** each feature, with its tasks, unless the **Feature list** is "none".
9. **Where status and guidance go:** what the **Status locations** will say, the new rows in the
   **Agent guide**, and the entries in the **Update map**.
10. **Tasks for missing checks:** each check the rule expects that the project lacks.

Ask the user to approve the blueprint or change it, and write nothing until they approve. Record
each choice they make, with the date, in the context of its decision record or in a dated note
on its task.

## 5. Write the documents

Write the pieces in this order, each in the shape `roadmap-and-docs.md` gives it. Write only the
pieces in scope, and check each against the blueprint before you start the next.

1. **Project details:** fill the approved values into the copy of `roadmap-and-docs.md` in
   `.claude/rules/`. Change nothing outside its "Project details" section.
2. **The decision records,** in the **Decision log**. Write them before the other documents,
   because those cite them. Append each with the next number. In the context of a record found
   in the code, say where the code shows it.
3. **The overview,** `00-overview.md`. Define the sizes S, M, L, and XL above the scope table.
   Give each non-goal the record that excludes it, and each budget its hardware, or the task
   that will choose it.
4. **The other core documents,** in number order, only those in the **Core documents**.
   - Where `dependencies.md` is adopted, `02-dependency-policy.md` names that rule as the policy
     and keeps only what the rule leaves to the project: the register of build-or-buy
     decisions, and the approved exceptions if the rule's **Approved exceptions** points here.
   - Where `testing.md` is adopted, `04-quality.md` names that rule for how tests are written,
     and keeps the project's own test layers, tools, and checks.
5. **The specs,** one for each part, in the **Specs** folder.
   - For a built part, describe the code as it is, and check every type, function, and file
     name you write against the code. Above each sketch of built code, name the task that built
     it.
   - For a planned part, describe the design in the future tense, with the task that builds it.
   - In `Tests`, name the test that checks each invariant, or the task that will add it.
6. **The roadmap,** `roadmap.md` in the **Docs folder**:
   1. Start with the "How to use this file" list.
   2. When code already exists, record it as the approved first milestone: one task for each
      built part or feature, with a dated `Note:` saying it records work done before the
      roadmap, and a `Verify` line that runs the tests covering it. Tick a task only when its
      `Verify` commands pass in this session; otherwise leave it unticked, and note what fails.
   3. Then the approved milestones. Each task's `Do` points to a spec section, its `Accept`
      lists results someone can observe, and its `Verify` gives exact commands with their
      expected results. A `Verify` command exists now, or a task in its `Depends` builds it.
      A task depends only on tasks listed above it.
   4. Add the tasks for missing checks where they are first needed.
   5. End each milestone with its gate where the **Gate evidence** isn't "none", and with exit
      criteria otherwise. `/phase-gates plan` can regroup the tasks into phases later.
   6. End the roadmap with the backlog. Give every item a trigger that someone could observe.
7. **The feature list,** unless the **Feature list** is "none": one entry for each feature, with
   its tasks and the ticked ones among them. Write what the ticked tasks built in the present
   tense, and the rest in the future tense.
8. **The guide, the map, and the status:** add every new document to the **Agent guide**, list in
   the **Update map** which documents each kind of change updates, and write the same status in
   every one of the **Status locations**, matching the roadmap's ticks.

Unless the **Diagrams** value is "none", leave the diagrams to `/draw-diagrams`.

## 6. Check the documents

First, check that the roadmap and the records hold together. This command prints:

- each task that is used twice, depends on a task listed after it, or is ticked while a task it
  depends on isn't
- each backlog item without a trigger
- each record that is numbered out of order or has an unknown status
- each task or record that a document cites but that doesn't exist

Replace the first two paths with the roadmap and the **Decision log**, the two patterns with the
**Task IDs** format and the record ID format, and the rest with every document that cites a task
or a record:

```bash
python3 - docs/roadmap.md docs/decisions.md 'M[0-9]+\.[0-9]+' 'ADR-[0-9]{4}' \
  docs/*.md README.md <<'EOF'
import os, re, sys
roadmap, log, task_id, record_id, *docs = sys.argv[1:]
problems = []

def lines(f):
    return enumerate(open(f, encoding="utf-8").read().splitlines(), 1)

# Tasks in roadmap order, with their Depends lines.
tasks, current = {}, None
for n, line in lines(roadmap):
    m = re.match(r"\s*- \[([ x])\] \*\*(" + task_id + r") — ", line)
    if m:
        current = m.group(2)
        if current in tasks:
            problems.append(f"{roadmap}:{n}: {current} is already used on line "
                            f"{tasks[current]['line']}")
            current = None
        else:
            tasks[current] = {"ticked": m.group(1) == "x", "line": n, "deps": []}
        continue
    if not line.startswith(" "):
        current = None
    d = re.match(r"\s+- Depends: (.*)", line)
    if d and current:
        tasks[current]["deps"] = [(n, dep) for dep in re.findall(task_id, d.group(1))]
order = list(tasks)
for task in order:
    for n, dep in tasks[task]["deps"]:
        if dep not in tasks:
            continue  # reported below, as a citation
        if order.index(dep) > order.index(task):
            problems.append(f"{roadmap}:{n}: {task} depends on {dep}, which comes after it")
        elif tasks[task]["ticked"] and not tasks[dep]["ticked"]:
            problems.append(f"{roadmap}:{n}: {task} is ticked, but {dep} isn't")

# Every backlog row has a trigger.
rows = None
for n, line in lines(roadmap):
    if line.startswith("## "):
        rows = 0 if line.startswith("## Backlog") else None
    elif rows is not None and line.startswith("|"):
        rows += 1
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if rows > 2 and re.fullmatch(r"(?i)(|tbd|(when|if) needed\.?)", cells[-1]):
            problems.append(f"{roadmap}:{n}: backlog item \"{cells[0]}\" has no trigger")

# Decision records are numbered in order, with no gaps, and each has a status.
records, last = {}, None
for n, line in lines(log):
    m = re.match(r"## (" + record_id + r") — ", line)
    if m:
        current, number = m.group(1), int(re.findall(r"\d+", m.group(1))[-1])
        if current in records:
            problems.append(f"{log}:{n}: {current} is already used on line "
                            f"{records[current]['line']}")
        elif last and number != last[1] + 1:
            problems.append(f"{log}:{n}: {current} follows {last[0]}; use the next number")
        records.setdefault(current, {"line": n, "status": None})
        last = (current, number)
    s = re.match(r"- \*\*Status:\*\* (.*)", line)
    if s and last:
        records[last[0]]["status"] = s.group(1)
        if not re.match(r"Proposed|Accepted|Superseded by ", s.group(1)):
            problems.append(f"{log}:{n}: {last[0]} has the status \"{s.group(1)}\"")
for rid, record in records.items():
    if record["status"] is None:
        problems.append(f"{log}:{record['line']}: {rid} has no Status line")

# Every task and record that a document cites exists.
for f in dict.fromkeys([roadmap, log, *docs]):
    if not os.path.isfile(f):
        problems.append(f"{f}: not found")
        continue
    for n, line in lines(f):
        for t in re.findall(task_id, line):
            if t not in tasks:
                problems.append(f"{f}:{n}: cites {t}, which isn't in the roadmap")
        for r in re.findall(record_id, line):
            if r not in records:
                problems.append(f"{f}:{n}: cites {r}, which isn't in the decision log")

print("\n".join(problems) or "Roadmap and decision records OK")
sys.exit(1 if problems else 0)
EOF
```

It prints `Roadmap and decision records OK` when it finds nothing. Otherwise it prints one line
for each problem, such as these, and exits with status 1:

```text
docs/roadmap.md:15: M1.2 depends on M1.4, which comes after it
docs/roadmap.md:33: backlog item "Gamepads" has no trigger
docs/decisions.md:8: ADR-0003 follows ADR-0001; use the next number
docs/00-overview.md:5: cites M2.1, which isn't in the roadmap
```

Then:

1. Check each path, command, type, and feature that the documents call built against the
   repository. Anything you can't find is planned, and its sentence names the task that builds
   it.
2. Run the checks for any **Checked documents** you wrote, and the **Feature list**'s check if
   it exists.
3. Do the steps in "Before finishing" in `roadmap-and-docs.md` and `writing.md`, including the
   two-reader check.

## 7. Commit only as the project allows

Follow the **Git access** level in `commits.md`, and leave the changes uncommitted unless the user
asked for a commit and that level allows it. Either way, end the report with copy-ready commit
messages, as `commits.md` describes. Give the filled-in "Project details" a commit of their own.
The documents share a second commit with their rows in the **Agent guide** and the **Update
map**, and with the **Status locations**, unless the **Changes that travel together** say
otherwise.

## 8. Report

End with a summary that contains:

- **Documents:** each one created or extended, and what it now covers.
- **Decisions:** each question you asked, the user's answer, and where you recorded it, and each
  record left `Proposed`.
- **First milestone:** for existing code, each task you ticked, and each you left unticked, with
  what fails.
- **Not confirmed:** each fact left as a question or a task, such as a budget whose hardware
  isn't chosen.
- **Checks:** each command you ran, with its real result.
- **Next:** `/phase-gates plan` if the milestones need gates, `/draw-diagrams` if the
  **Diagrams** aren't "none", and `/next-task` with the first unblocked task.
- **Commit messages:** in `text` blocks, as `commits.md` describes, with no attribution lines.
