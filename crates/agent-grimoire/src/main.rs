//! The `grimoire` binary: the library's [`agent_grimoire::run`] with real streams
//! and the catalog embedded in this build.

use std::io::{self, IsTerminal as _};
use std::process::ExitCode;

use agent_grimoire::{Context, Exit, run};
use agent_grimoire_catalog::Source;
use agent_grimoire_model::Version;

#[allow(clippy::print_stderr, reason = "the only way to report a broken build")]
fn main() -> ExitCode {
    let source = match Source::builtin() {
        Ok(source) => source,
        Err(e) => {
            eprintln!("error: this grimoire build is broken: {e}");
            return ExitCode::from(Exit::Failure.code());
        }
    };
    let Ok(version) = Version::parse(env!("CARGO_PKG_VERSION")) else {
        eprintln!("error: this grimoire build has an invalid version");
        return ExitCode::from(Exit::Failure.code());
    };
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(e) => {
            eprintln!("error: cannot read the current directory: {e}");
            return ExitCode::from(Exit::Failure.code());
        }
    };
    let ci = std::env::var_os("CI").is_some_and(|v| !v.is_empty());
    let stdin = io::stdin();
    let interactive = stdin.is_terminal() && !ci;
    let mut input = stdin.lock();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let mut ctx = Context {
        source: &source,
        version,
        cwd,
        stdin: &mut input,
        stdout: &mut stdout,
        stderr: &mut stderr,
        interactive,
        path: std::env::var_os("PATH"),
    };
    ExitCode::from(run(std::env::args_os(), &mut ctx).code())
}
