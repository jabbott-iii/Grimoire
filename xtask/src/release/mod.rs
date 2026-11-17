//! `cargo xtask release …`: the release pipeline's logic, so that
//! `.github/workflows/release.yml` stays thin glue (ADR-0009, ADR-0010).
//!
//! - `plan`: what this run does (dry-run, release or skip) and its matrices.
//! - `build`: build and archive one target's binaries.
//! - `sbom`: a CycloneDX 1.6 SBOM per archive.
//! - `channels`: npm packages, wheels, the Homebrew formula, the Scoop and winget
//!   manifests and the install scripts, from the CLI archives.
//! - `checksums`: `SHA256SUMS`.
//! - `verify`: the finished `dist/` is exactly what the release needs.
//! - `notes`: the GitHub Release text from `CHANGELOG.md`.

mod archive;
mod build;
mod channels;
mod checksums;
mod meta;
mod notes;
mod plan;
mod sbom;
mod targets;
mod zip;

use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Decide what this run of release.yml does and print its outputs.
    Plan {
        /// The GitHub event (`github.event_name`).
        #[arg(long)]
        event: String,
        /// The Git ref (`github.ref`).
        #[arg(long = "ref", value_name = "REF")]
        git_ref: String,
        /// The repository variable `GRIMOIRE_RELEASES` (`enabled` turns releases on).
        #[arg(long, default_value = "")]
        releases_enabled: String,
        /// Append the outputs to this file (`$GITHUB_OUTPUT`) as well as printing them.
        #[arg(long, value_name = "FILE")]
        github_output: Option<PathBuf>,
    },
    /// Build one target's binaries with cargo auditable and write their archives.
    Build {
        /// A release target triple.
        #[arg(long)]
        target: String,
        /// Where to write the archives.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
    },
    /// Write `{archive}.cdx.json` for every archive in DIR.
    Sbom {
        #[arg(long, value_name = "DIR")]
        dist: PathBuf,
    },
    /// Generate every package manager's files from the CLI archives in DIR.
    Channels {
        #[arg(long, value_name = "DIR")]
        dist: PathBuf,
        /// Where npm package directories and winget JSON twins go.
        #[arg(long, value_name = "DIR")]
        work: PathBuf,
        /// Accept a subset of targets (pull request dry-runs).
        #[arg(long)]
        partial: bool,
    },
    /// Write SHA256SUMS for DIR, or check it.
    Checksums {
        #[arg(long, value_name = "DIR")]
        dist: PathBuf,
        /// Fail if SHA256SUMS is missing or stale instead of writing it.
        #[arg(long)]
        check: bool,
    },
    /// Check that DIR holds exactly the release's assets, consistent with each other.
    Verify {
        #[arg(long, value_name = "DIR")]
        dist: PathBuf,
        /// Accept a subset of targets (pull request dry-runs).
        #[arg(long)]
        partial: bool,
    },
    /// Write the GitHub Release notes for VERSION from CHANGELOG.md.
    Notes {
        #[arg(long = "version", value_name = "VERSION")]
        release_version: String,
        #[arg(long, value_name = "FILE")]
        out: PathBuf,
    },
}

/// Resolves a user-given path against the repository root (`cargo xtask` runs there).
fn at(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

#[allow(clippy::print_stdout, reason = "plan prints its outputs for the log")]
pub(crate) fn run(root: &Path, command: Command) -> Result<ExitCode, String> {
    match command {
        Command::Plan {
            event,
            git_ref,
            releases_enabled,
            github_output,
        } => {
            let release = meta::load(root)?;
            plan::check_limits(&release)?;
            let tag = release.tag();
            let mut mode = plan::mode(&event, &git_ref, releases_enabled.trim(), || {
                plan::remote_tag_exists(root, &tag)
            })?;
            // A release needs its CHANGELOG.md section, which the merged release PR adds.
            // Without it, any other push to main would release the version early.
            let changelog = std::fs::read_to_string(root.join("CHANGELOG.md")).unwrap_or_default();
            if mode == plan::Mode::Release && notes::section(&changelog, &release.version).is_none()
            {
                println!(
                    "::warning::CHANGELOG.md has no section for {}; release mode waits for the release PR, so this run is a dry-run",
                    release.version
                );
                mode = plan::Mode::DryRun;
            }
            let text = plan::outputs(&release, mode);
            print!("{text}");
            if let Some(path) = github_output {
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(&path)
                    .and_then(|mut f| f.write_all(text.as_bytes()))
                    .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
            }
        }
        Command::Build { target, out } => build::run(root, &target, &at(root, &out))?,
        Command::Sbom { dist } => sbom::run(root, &at(root, &dist))?,
        Command::Channels {
            dist,
            work,
            partial,
        } => channels::run(root, &at(root, &dist), &at(root, &work), partial)?,
        Command::Checksums { dist, check } => {
            if !checksums::run(&at(root, &dist), check)? {
                return Ok(ExitCode::FAILURE);
            }
        }
        Command::Verify { dist, partial } => channels::verify(root, &at(root, &dist), partial)?,
        Command::Notes {
            release_version,
            out,
        } => notes::run(root, &release_version, &at(root, &out))?,
    }
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::targets::{self, BIN, Format, PACKAGE, REPO_URL};

    fn root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or(Path::new("."))
    }

    /// `[package.metadata.binstall]` in the CLI's manifest must name, for every host
    /// cargo-binstall may run on, an archive `release build` really makes, in its format,
    /// with the binary where `bin-dir` says (BINSTALL in docs/specs.md).
    #[test]
    fn binstall_metadata_names_the_release_archives() {
        let manifest: toml::Table =
            std::fs::read_to_string(root().join("crates/agent-grimoire/Cargo.toml"))
                .unwrap()
                .parse()
                .unwrap();
        let meta = &manifest["package"]["metadata"]["binstall"];
        let field = |host: &str, key: &str| -> String {
            meta.get("overrides")
                .and_then(|o| o.get(host))
                .and_then(|o| o.get(key))
                .or_else(|| meta.get(key))
                .and_then(toml::Value::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        let version = "1.2.3";
        let mut hosts: Vec<(String, &targets::Target)> = targets::TARGETS
            .iter()
            .map(|t| (t.triple.to_owned(), t))
            .collect();
        // glibc hosts get the static musl builds.
        for arch in ["x86_64", "aarch64"] {
            let musl = targets::find(&format!("{arch}-unknown-linux-musl")).unwrap();
            hosts.push((format!("{arch}-unknown-linux-gnu"), musl));
        }
        for (host, target) in hosts {
            let render = |s: &str| {
                s.replace("{ repo }", REPO_URL)
                    .replace("{ version }", version)
                    .replace("{ target }", &host)
                    .replace("{ bin }", BIN)
                    .replace("{ binary-ext }", target.exe_suffix())
            };
            let url = render(&field(&host, "pkg-url"));
            assert!(!url.contains('{'), "{host}: unrendered template in {url}");
            assert_eq!(
                url,
                targets::download_url(version, &target.archive(PACKAGE, version)),
                "{host}"
            );
            let fmt = match target.format {
                Format::TarGz => "tgz",
                Format::Zip => "zip",
            };
            assert_eq!(field(&host, "pkg-fmt"), fmt, "{host}");
            assert_eq!(
                render(&field(&host, "bin-dir")),
                target.binary_file(BIN),
                "{host}"
            );
        }
        assert_eq!(
            meta["disabled-strategies"].as_array().map(Vec::len),
            Some(1),
            "quick-install stays disabled"
        );
    }

    /// Every published crate carries both licence files, identical to the root ones.
    #[test]
    fn published_crates_carry_the_licence_files() {
        let release = super::meta::load(root()).unwrap();
        for name in &release.crates {
            for licence in super::build::LICENSES {
                let copy = std::fs::read(root().join("crates").join(name).join(licence))
                    .unwrap_or_else(|_| panic!("crates/{name}/{licence} is missing"));
                assert_eq!(
                    copy,
                    std::fs::read(root().join(licence)).unwrap(),
                    "crates/{name}/{licence}"
                );
            }
        }
    }
}
