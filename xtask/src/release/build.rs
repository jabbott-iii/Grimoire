//! `cargo xtask release build --target TRIPLE --out DIR`: builds the CLI and every
//! asset binary for one target with `cargo auditable`, one package at a time,
//! smoke-tests them, and writes their archives (ADR-0009).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::archive::{self, File};
use super::meta::{self, Release};
use super::targets::{self, BIN, PACKAGE, Target};

/// The Cargo profile release binaries are built with (root `Cargo.toml`).
pub(crate) const PROFILE: &str = "dist";

/// The licence files every archive carries, from the repository root.
pub(crate) const LICENSES: [&str; 2] = ["LICENSE-MIT", "LICENSE-APACHE"];

/// The flags every release build gets: build paths remapped so binaries don't embed
/// the runner's directories, and a static CRT on Windows so no VC++ runtime is needed
/// (WINGET research). Both go in one variable: `RUSTFLAGS` (and its encoded form)
/// replace `[target]` rustflags from config files entirely.
pub(crate) fn rustflags(
    target: &Target,
    workspace: &Path,
    cargo_home: Option<&Path>,
) -> Vec<String> {
    let mut flags = vec![format!(
        "--remap-path-prefix={}=/grimoire",
        workspace.display()
    )];
    if let Some(home) = cargo_home {
        flags.push(format!("--remap-path-prefix={}=/cargo", home.display()));
    }
    if target.is_windows() {
        flags.push("-Ctarget-feature=+crt-static".to_owned());
    }
    flags
}

/// Cargo's home as Cargo finds it: `CARGO_HOME`, else `.cargo` in the user's home
/// directory (`USERPROFILE` on Windows, where Git Bash also sets a Unix-style `HOME`).
fn cargo_home() -> Option<PathBuf> {
    let home = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    std::env::var_os("CARGO_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os(home).map(|h| PathBuf::from(h).join(".cargo")))
}

fn host_triple() -> Result<String, String> {
    let out = Command::new("rustc")
        .arg("-vV")
        .output()
        .map_err(|e| format!("cannot run rustc: {e}"))?;
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .find_map(|l| l.strip_prefix("host: "))
        .map(str::to_owned)
        .ok_or_else(|| "rustc -vV printed no host".to_owned())
}

/// Whether binaries for `triple` run on `host`: the same triple, or a static musl
/// build on a glibc host of the same architecture (rustup's Linux hosts are `-gnu`).
fn runs_on(host: &str, triple: &str) -> bool {
    host == triple
        || triple
            .strip_suffix("-unknown-linux-musl")
            .is_some_and(|arch| host == format!("{arch}-unknown-linux-gnu"))
}

#[allow(clippy::print_stderr, reason = "xtask reports progress on stderr")]
fn cargo_build(root: &Path, triple: &str, package: &str, flags: &[String]) -> Result<(), String> {
    eprintln!(
        "release build: cargo auditable build --profile {PROFILE} --target {triple} -p {package} ({} flags)",
        flags.len()
    );
    let status = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")))
        .current_dir(root)
        .args(["auditable", "build", "--locked", "--profile", PROFILE])
        .args(["--target", triple, "-p", package])
        .env("CARGO_ENCODED_RUSTFLAGS", flags.join("\u{1f}"))
        .env_remove("RUSTFLAGS")
        .status()
        .map_err(|e| {
            format!("cannot run `cargo auditable` (install cargo-auditable 0.7.7): {e}")
        })?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("the {triple} build of {package} failed"))
    }
}

#[allow(clippy::print_stderr, reason = "xtask reports progress on stderr")]
pub(crate) fn run(root: &Path, triple: &str, out: &Path) -> Result<(), String> {
    let target = targets::find(triple)?;
    let release = meta::load(root)?;
    let flags = rustflags(target, &release.workspace_root, cargo_home().as_deref());
    // One build per package, so each binary gets exactly the dependency features its own
    // manifest asks for (as `cargo install` would) and its SBOM describes it. A single
    // multi-package build would unify features across the CLI and the asset crates.
    cargo_build(root, triple, PACKAGE, &flags)?;
    for a in &release.assets {
        cargo_build(root, triple, &a.package, &flags)?;
    }
    let bin_dir = release.target_dir.join(triple).join(PROFILE);
    let runnable = runs_on(&host_triple()?, triple);
    // Release runners are native, so a build CI can't run is misconfigured.
    let ci = std::env::var_os("CI").is_some_and(|v| !v.is_empty());
    if !runnable && ci {
        return Err(format!(
            "{triple} binaries can't run on this runner, so they can't be smoke-tested"
        ));
    }
    fs::create_dir_all(out).map_err(|e| format!("cannot create {}: {e}", out.display()))?;
    for (name, version, bin, readme) in archives_of(root, &release) {
        let path = bin_dir.join(target.binary_file(&bin));
        if runnable {
            smoke_test(&path, &bin, &version)?;
        } else {
            eprintln!("release build: {triple} can't run here; skipping the --version check");
        }
        let bytes = package(root, target, &path, &bin, &readme)?;
        let file = target.archive(&name, &version);
        fs::write(out.join(&file), bytes).map_err(|e| format!("cannot write {file}: {e}"))?;
        eprintln!("release build: wrote {file}");
    }
    Ok(())
}

/// `(archive name, version, binary, README path)` for the CLI and each asset binary.
fn archives_of(root: &Path, release: &Release) -> Vec<(String, String, String, PathBuf)> {
    let mut list = vec![(
        PACKAGE.to_owned(),
        release.version.clone(),
        BIN.to_owned(),
        root.join("README.md"),
    )];
    for a in &release.assets {
        list.push((
            a.bin.clone(),
            a.version.clone(),
            a.bin.clone(),
            a.dir.join("README.md"),
        ));
    }
    list
}

/// Runs `{bin} --version` and expects `{bin} {version}`.
fn smoke_test(path: &Path, bin: &str, version: &str) -> Result<(), String> {
    let out = Command::new(path)
        .arg("--version")
        .output()
        .map_err(|e| format!("cannot run {}: {e}", path.display()))?;
    let printed = String::from_utf8_lossy(&out.stdout);
    let expected = format!("{bin} {version}");
    if out.status.success() && printed.trim() == expected {
        Ok(())
    } else {
        Err(format!(
            "{} --version printed {:?} (exit {:?}); expected {expected:?}",
            path.display(),
            printed.trim(),
            out.status.code()
        ))
    }
}

/// The archive for one binary: the binary (mode 0755), both licences and a README.
pub(crate) fn package(
    root: &Path,
    target: &Target,
    binary: &Path,
    bin: &str,
    readme: &Path,
) -> Result<Vec<u8>, String> {
    let read = |p: &Path| fs::read(p).map_err(|e| format!("cannot read {}: {e}", p.display()));
    let mut files = vec![
        File {
            name: target.binary_file(bin),
            mode: 0o755,
            data: read(binary)?,
        },
        File {
            name: "README.md".to_owned(),
            mode: 0o644,
            data: read(readme)?,
        },
    ];
    for l in LICENSES {
        files.push(File {
            name: l.to_owned(),
            mode: 0o644,
            data: read(&root.join(l))?,
        });
    }
    archive::write(target.format, &files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustflags_remap_paths_and_link_the_crt_statically_on_windows() {
        let linux = targets::find("x86_64-unknown-linux-musl").unwrap();
        let flags = rustflags(linux, Path::new("/w"), Some(Path::new("/h/.cargo")));
        assert_eq!(
            flags,
            [
                "--remap-path-prefix=/w=/grimoire",
                "--remap-path-prefix=/h/.cargo=/cargo"
            ]
        );
        let win = targets::find("aarch64-pc-windows-msvc").unwrap();
        assert!(
            rustflags(win, Path::new("/w"), None)
                .contains(&"-Ctarget-feature=+crt-static".to_owned())
        );
    }

    #[test]
    fn musl_binaries_run_on_glibc_hosts_of_the_same_architecture() {
        assert!(runs_on(
            "x86_64-unknown-linux-gnu",
            "x86_64-unknown-linux-musl"
        ));
        assert!(runs_on(
            "aarch64-unknown-linux-gnu",
            "aarch64-unknown-linux-musl"
        ));
        assert!(runs_on("aarch64-apple-darwin", "aarch64-apple-darwin"));
        assert!(!runs_on(
            "x86_64-unknown-linux-gnu",
            "aarch64-unknown-linux-musl"
        ));
        assert!(!runs_on("aarch64-apple-darwin", "x86_64-apple-darwin"));
    }

    #[test]
    fn archives_hold_the_binary_licences_and_readme() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for (f, body) in [
            ("LICENSE-MIT", "mit"),
            ("LICENSE-APACHE", "apache"),
            ("README.md", "readme"),
            ("grimoire.exe", "MZ"),
        ] {
            fs::write(root.join(f), body).unwrap();
        }
        let win = targets::find("x86_64-pc-windows-msvc").unwrap();
        let bytes = package(
            root,
            win,
            &root.join("grimoire.exe"),
            BIN,
            &root.join("README.md"),
        )
        .unwrap();
        let files = archive::read(win.format, &bytes).unwrap();
        let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(
            names,
            ["LICENSE-APACHE", "LICENSE-MIT", "README.md", "grimoire.exe"]
        );
        assert_eq!(archive::member(&files, "grimoire.exe").unwrap().mode, 0o755);
    }
}
