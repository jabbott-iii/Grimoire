//! Keeps `docs/cli.md` true to the command line (task M7.2 in `docs/roadmap.md`).
//!
//! The test reads only the structure that `docs/cli.md` defines for it:
//! - one `## grimoire <command>` section per subcommand except `help`, in the
//!   order `grimoire --help` lists them;
//! - in each command section, and in `## Global options`, every table whose
//!   header starts with `Option` or `Argument or option`. The first cell of
//!   each row names its flags and arguments in code spans, such as
//!   `` `-C <DIR>` ``, `` `-h`, `--help` `` or `` `<ID>` ``. A `|` inside a
//!   cell is written `\|`, as GitHub's tables need;
//! - in `## Exit codes`, the table whose header starts with `Code`, one code
//!   per row.
//!
//! Everything else in the file is for people, and the test ignores it. Code
//! fences are skipped.
//!
//! So the test compares the commands and their order, each table's flag and
//! argument names, and the exit codes. It doesn't compare a flag's value
//! placeholder (the `<DIR>` of `-C <DIR>`), the values a row lists as allowed
//! (for `--kind`, `--client` or `--maturity`), the usage lines in the code
//! fences, each command's own exit codes and JSON keys, or any description.
//! Review keeps those true: a new kind, client or maturity, for example, has to
//! be added to `docs/cli.md` by hand.

use std::collections::BTreeSet;

use clap::CommandFactory as _;

use crate::Exit;

/// The reference, as users read it.
const DOC: &str = include_str!("../../../docs/cli.md");

/// The first header cells of the tables that list flags and arguments.
const OPTION_HEADERS: [&str; 2] = ["Option", "Argument or option"];

/// What the reference says, as far as this test reads it.
#[derive(Debug, Default, PartialEq, Eq)]
struct Doc {
    /// Each `## grimoire <name>` section in order, with the flags and
    /// arguments its tables list.
    commands: Vec<(String, BTreeSet<String>)>,
    /// The flags in the `## Global options` table.
    global: BTreeSet<String>,
    /// The codes in the `## Exit codes` table, in order.
    exits: Vec<u8>,
}

/// The `## ` section a line is in.
enum Section {
    Command,
    Global,
    Exits,
    Other,
}

fn parse(text: &str) -> Result<Doc, String> {
    let mut doc = Doc::default();
    let mut section = Section::Other;
    let mut fenced = false;
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.next() {
        if line.trim_start().starts_with("```") {
            fenced = !fenced;
            continue;
        }
        if fenced {
            continue;
        }
        if let Some(heading) = line.strip_prefix("## ") {
            let heading = heading.trim();
            section = if let Some(name) = heading.strip_prefix("grimoire ") {
                doc.commands.push((name.trim().to_owned(), BTreeSet::new()));
                Section::Command
            } else if heading == "Global options" {
                Section::Global
            } else if heading == "Exit codes" {
                Section::Exits
            } else {
                Section::Other
            };
            continue;
        }
        if !line.starts_with('|') {
            continue;
        }
        let header = first_cell(line);
        let mut rows = Vec::new();
        while let Some(row) = lines.next_if(|l| l.starts_with('|')) {
            rows.push(first_cell(row));
        }
        // The first row is the `|---|` delimiter row.
        let body = rows.iter().skip(1);
        let listed = match section {
            Section::Command => doc.commands.last_mut().map(|(_, flags)| flags),
            Section::Global => Some(&mut doc.global),
            Section::Exits | Section::Other => None,
        };
        if let Some(listed) = listed.filter(|_| OPTION_HEADERS.contains(&header.as_str())) {
            for cell in body {
                listed.extend(flags(cell)?);
            }
        } else if matches!(section, Section::Exits) && header == "Code" {
            for cell in body {
                let code = cell
                    .parse()
                    .map_err(|_| format!("`{cell}` in the Exit codes table is not an exit code"))?;
                doc.exits.push(code);
            }
        }
    }
    Ok(doc)
}

/// The first cell of a table row, with `\|` unescaped.
fn first_cell(row: &str) -> String {
    let row = row.trim();
    let mut chars = row.strip_prefix('|').unwrap_or(row).chars();
    let mut cell = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some('|') => cell.push('|'),
                Some(other) => {
                    cell.push('\\');
                    cell.push(other);
                }
                None => cell.push('\\'),
            },
            '|' => break,
            c => cell.push(c),
        }
    }
    cell.trim().to_owned()
}

/// The flags and arguments an options-table cell names: `--long`, `-s`, and
/// `<NAME>` for `` `<NAME>...` `` or `` `[NAME]` ``.
fn flags(cell: &str) -> Result<Vec<String>, String> {
    let spans: Vec<&str> = cell.split('`').skip(1).step_by(2).collect();
    if spans.is_empty() {
        return Err(format!(
            "`{cell}`: an options table's first column must name its flags or arguments in backticks"
        ));
    }
    spans
        .into_iter()
        .map(|span| {
            let span = span.trim();
            if span.starts_with('-') {
                Ok(span.split_whitespace().next().unwrap_or(span).to_owned())
            } else if let Some(rest) = span.strip_prefix(['<', '[']) {
                let end = rest
                    .find(['>', ']'])
                    .ok_or_else(|| format!("`{span}` has no closing bracket"))?;
                Ok(format!("<{}>", &rest[..end]))
            } else {
                Err(format!("`{span}` is neither a flag nor an argument"))
            }
        })
        .collect()
}

/// The flags and arguments of `cmd`, written as the doc writes them. With
/// `own`, the global options and `--help`, which every command has, are left
/// out: they belong in the Global options table.
fn expected(cmd: &clap::Command, own: bool) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for arg in cmd.get_arguments() {
        if own && (arg.is_global_set() || arg.get_id().as_str() == "help") {
            continue;
        }
        if arg.is_positional() {
            let name = arg
                .get_value_names()
                .and_then(<[_]>::first)
                .map_or_else(|| arg.get_id().as_str().to_uppercase(), ToString::to_string);
            names.insert(format!("<{name}>"));
        }
        if let Some(long) = arg.get_long() {
            names.insert(format!("--{long}"));
        }
        if let Some(short) = arg.get_short() {
            names.insert(format!("-{short}"));
        }
    }
    names
}

/// Every difference between `cli` (already built) with its exit codes, and `doc`.
fn differences(cli: &clap::Command, exits: &[u8], doc: &Doc) -> Vec<String> {
    let mut problems = Vec::new();
    let commands: Vec<&clap::Command> = cli
        .get_subcommands()
        .filter(|c| c.get_name() != "help")
        .collect();
    for cmd in &commands {
        let name = cmd.get_name();
        match doc.commands.iter().find(|(n, _)| n == name) {
            Some((_, listed)) => compare(
                &mut problems,
                &format!("\"## grimoire {name}\""),
                &expected(cmd, true),
                listed,
            ),
            None => problems.push(format!(
                "`grimoire {name}` has no \"## grimoire {name}\" section"
            )),
        }
    }
    for (name, _) in &doc.commands {
        if !commands.iter().any(|c| c.get_name() == name) {
            problems.push(format!(
                "\"## grimoire {name}\" names no command grimoire has"
            ));
        }
    }
    let documented: Vec<&str> = doc
        .commands
        .iter()
        .map(|(n, _)| n.as_str())
        .filter(|n| commands.iter().any(|c| c.get_name() == *n))
        .collect();
    let listed: Vec<&str> = commands
        .iter()
        .map(|c| c.get_name())
        .filter(|n| documented.contains(n))
        .collect();
    if documented != listed {
        problems.push(format!(
            "the command sections are in the order {documented:?}, not {listed:?} as `grimoire --help` lists them"
        ));
    }
    compare(
        &mut problems,
        "\"## Global options\"",
        &expected(cli, false),
        &doc.global,
    );
    if doc.exits != exits {
        problems.push(format!(
            "the Exit codes table lists {:?}, but the CLI returns {exits:?}",
            doc.exits
        ));
    }
    problems
}

fn compare(
    problems: &mut Vec<String>,
    table: &str,
    code: &BTreeSet<String>,
    doc: &BTreeSet<String>,
) {
    for missing in code.difference(doc) {
        problems.push(format!("{table} does not list `{missing}`"));
    }
    for extra in doc.difference(code) {
        problems.push(format!(
            "{table} lists `{extra}`, which the code does not have"
        ));
    }
}

/// Every [`Exit`], in code order. The match has no wildcard arm, so a new
/// variant fails to compile here until it is added to the chain (and to
/// `docs/cli.md`).
fn exits() -> Vec<Exit> {
    let mut all = Vec::new();
    let mut next = Some(Exit::Ok);
    while let Some(exit) = next {
        all.push(exit);
        next = match exit {
            Exit::Ok => Some(Exit::Failure),
            Exit::Failure => Some(Exit::Usage),
            Exit::Usage => Some(Exit::Invalid),
            Exit::Invalid => Some(Exit::Conflict),
            Exit::Conflict => Some(Exit::Check),
            Exit::Check => Some(Exit::Refused),
            Exit::Refused => None,
        };
    }
    all
}

#[test]
fn docs_cli_md_matches_the_command_line() {
    let mut cli = crate::cli::Cli::command();
    cli.build();
    let codes: Vec<u8> = exits().into_iter().map(Exit::code).collect();
    let doc = match parse(DOC) {
        Ok(doc) => doc,
        Err(e) => panic!("docs/cli.md: {e}"),
    };
    let problems = differences(&cli, &codes, &doc);
    assert!(
        problems.is_empty(),
        "docs/cli.md differs from the command line; update the doc with the code:\n- {}",
        problems.join("\n- ")
    );
}

#[test]
fn the_parser_reads_only_its_own_structure() {
    let text = r"# Title

| Option | Meaning |
|---|---|
| `--before-any-section` | Ignored. |

## Global options

| Option | Meaning |
|---|---|
| `-C <DIR>` | A directory. |
| `-h`, `--help` | Help. |

## grimoire add

```text
## grimoire not-a-section
| Option | Meaning |
| `--in-a-fence` | Ignored. |
```

| Argument or option | Meaning |
|---|---|
| `<ID\|bundle:ID>...` | What to add, with an escaped `\|`. |
| `--allow <ALLOW>` | Permissions. |

| Key | Meaning |
|---|---|
| `added` | Not an options table. |

## grimoire verify

## Exit codes

| Code | Meaning |
|---|---|
| 0 | Fine. |
| 2 | Usage. |
";
    let doc = parse(text).unwrap();
    let set = |names: &[&str]| names.iter().map(ToString::to_string).collect();
    assert_eq!(
        doc,
        Doc {
            commands: vec![
                ("add".into(), set(&["<ID|bundle:ID>", "--allow"])),
                ("verify".into(), BTreeSet::new()),
            ],
            global: set(&["-C", "-h", "--help"]),
            exits: vec![0, 2],
        }
    );

    let mut cli = clap::Command::new("grimoire")
        .arg(clap::Arg::new("dir").short('C').global(true))
        .subcommand(
            clap::Command::new("add")
                .arg(clap::Arg::new("specs").value_name("ID|bundle:ID"))
                .arg(
                    clap::Arg::new("apply")
                        .long("apply")
                        .action(clap::ArgAction::SetTrue),
                ),
        )
        .subcommand(clap::Command::new("plan"));
    cli.build();
    assert_eq!(
        differences(&cli, &[0, 1, 2], &doc),
        [
            "\"## grimoire add\" does not list `--apply`",
            "\"## grimoire add\" lists `--allow`, which the code does not have",
            "`grimoire plan` has no \"## grimoire plan\" section",
            "\"## grimoire verify\" names no command grimoire has",
            "the Exit codes table lists [0, 2], but the CLI returns [0, 1, 2]",
        ]
    );

    let swapped = Doc {
        commands: vec![
            ("plan".into(), BTreeSet::new()),
            ("add".into(), set(&["<ID|bundle:ID>", "--apply"])),
        ],
        global: set(&["-C", "-h", "--help"]),
        exits: vec![0],
    };
    assert_eq!(
        differences(&cli, &[0], &swapped),
        [
            "the command sections are in the order [\"plan\", \"add\"], not [\"add\", \"plan\"] as `grimoire --help` lists them"
        ]
    );

    for bad in [
        "## Global options\n\n| Option | Meaning |\n|---|---|\n| --json | No backticks. |\n",
        "## Global options\n\n| Option | Meaning |\n|---|---|\n| `json` | Not a flag. |\n",
        "## Exit codes\n\n| Code | Meaning |\n|---|---|\n| zero | Not a number. |\n",
    ] {
        assert!(parse(bad).is_err(), "{bad}");
    }
}
