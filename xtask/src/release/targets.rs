//! The release targets and every name derived from them (PLAN.md §4, ADR-0009).

/// The GitHub repository. The maintainer kept this spelling (PLAN.md C8); trusted
/// publishers and cosign identities match it exactly, case included.
pub(crate) const REPO_URL: &str = "https://github.com/jabbott-iii/Grimoire";
/// The CLI package and the name it has on every registry (ADR-0004).
pub(crate) const PACKAGE: &str = "agent-grimoire";
/// The installed binary.
pub(crate) const BIN: &str = "grimoire";
/// The npm scope of the platform packages (ADR-0004).
pub(crate) const NPM_SCOPE: &str = "@agent-grimoire";

/// Archive format of a target's release archives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Format {
    TarGz,
    Zip,
}

impl Format {
    pub(crate) fn extension(self) -> &'static str {
        match self {
            Format::TarGz => "tar.gz",
            Format::Zip => "zip",
        }
    }
}

/// One release target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Target {
    /// The Rust target triple.
    pub(crate) triple: &'static str,
    /// The GitHub-hosted runner label that builds it natively (RUNNERS).
    pub(crate) runner: &'static str,
    /// npm's `process.platform`.
    pub(crate) npm_os: &'static str,
    /// npm's `process.arch`.
    pub(crate) npm_cpu: &'static str,
    /// Wheel platform tags, most specific first (PEP 425 compressed tag set).
    pub(crate) wheel_platforms: &'static [&'static str],
    /// The format of its archives.
    pub(crate) format: Format,
    /// Built on pull requests too: one target per OS, plus arm64 Linux so every pull
    /// request builds the two-platform container image (the maintainer's choice, ADR-0011).
    pub(crate) dry_run: bool,
    /// The container image architecture it provides (`linux/<arch>`), if any.
    pub(crate) image_arch: Option<&'static str>,
}

impl Target {
    pub(crate) fn is_windows(&self) -> bool {
        self.triple.ends_with("-windows-msvc")
    }

    /// `.exe` on Windows, nothing elsewhere.
    pub(crate) fn exe_suffix(&self) -> &'static str {
        if self.is_windows() { ".exe" } else { "" }
    }

    /// The name of a binary inside this target's archives.
    pub(crate) fn binary_file(&self, bin: &str) -> String {
        format!("{bin}{}", self.exe_suffix())
    }

    /// `agent-grimoire-{version}-{triple}.{ext}`, or `{bin}-{version}-{triple}.{ext}`
    /// for an asset binary.
    pub(crate) fn archive(&self, name: &str, version: &str) -> String {
        format!(
            "{name}-{version}-{}.{}",
            self.triple,
            self.format.extension()
        )
    }

    /// `agent-grimoire-{os}-{cpu}`: the platform package's name without its scope,
    /// which is also its directory name and `npm pack`'s tarball prefix.
    pub(crate) fn npm_dir(&self) -> String {
        format!("{PACKAGE}-{}-{}", self.npm_os, self.npm_cpu)
    }

    /// `@agent-grimoire/{os}-{cpu}`.
    pub(crate) fn npm_package(&self) -> String {
        format!("{NPM_SCOPE}/{}-{}", self.npm_os, self.npm_cpu)
    }
}

/// Every release target, in a fixed order.
pub(crate) const TARGETS: &[Target] = &[
    Target {
        triple: "x86_64-unknown-linux-musl",
        runner: "ubuntu-24.04",
        npm_os: "linux",
        npm_cpu: "x64",
        wheel_platforms: &[
            "manylinux_2_17_x86_64",
            "manylinux2014_x86_64",
            "musllinux_1_2_x86_64",
        ],
        format: Format::TarGz,
        dry_run: true,
        image_arch: Some("amd64"),
    },
    Target {
        triple: "aarch64-unknown-linux-musl",
        runner: "ubuntu-24.04-arm",
        npm_os: "linux",
        npm_cpu: "arm64",
        wheel_platforms: &[
            "manylinux_2_17_aarch64",
            "manylinux2014_aarch64",
            "musllinux_1_2_aarch64",
        ],
        format: Format::TarGz,
        dry_run: true,
        image_arch: Some("arm64"),
    },
    Target {
        triple: "x86_64-apple-darwin",
        runner: "macos-15-intel",
        npm_os: "darwin",
        npm_cpu: "x64",
        wheel_platforms: &["macosx_10_12_x86_64"],
        format: Format::TarGz,
        dry_run: false,
        image_arch: None,
    },
    Target {
        triple: "aarch64-apple-darwin",
        runner: "macos-15",
        npm_os: "darwin",
        npm_cpu: "arm64",
        wheel_platforms: &["macosx_11_0_arm64"],
        format: Format::TarGz,
        dry_run: true,
        image_arch: None,
    },
    Target {
        triple: "x86_64-pc-windows-msvc",
        runner: "windows-2025",
        npm_os: "win32",
        npm_cpu: "x64",
        wheel_platforms: &["win_amd64"],
        format: Format::Zip,
        dry_run: true,
        image_arch: None,
    },
    Target {
        triple: "aarch64-pc-windows-msvc",
        runner: "windows-11-arm",
        npm_os: "win32",
        npm_cpu: "arm64",
        wheel_platforms: &["win_arm64"],
        format: Format::Zip,
        dry_run: false,
        image_arch: None,
    },
];

/// The target with this triple.
pub(crate) fn find(triple: &str) -> Result<&'static Target, String> {
    TARGETS.iter().find(|t| t.triple == triple).ok_or_else(|| {
        let known: Vec<&str> = TARGETS.iter().map(|t| t.triple).collect();
        format!(
            "`{triple}` is not a release target (expected one of {})",
            known.join(", ")
        )
    })
}

/// Which targets a mode builds.
pub(crate) fn for_mode(dry_run: bool) -> impl Iterator<Item = &'static Target> {
    TARGETS.iter().filter(move |t| !dry_run || t.dry_run)
}

/// `https://github.com/jabbott-iii/Grimoire/releases/download/v{version}/{file}`.
pub(crate) fn download_url(version: &str, file: &str) -> String {
    format!("{REPO_URL}/releases/download/v{version}/{file}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_stable_and_ascii() {
        let linux = find("x86_64-unknown-linux-musl").unwrap();
        assert_eq!(
            linux.archive(PACKAGE, "1.2.3"),
            "agent-grimoire-1.2.3-x86_64-unknown-linux-musl.tar.gz"
        );
        assert_eq!(linux.npm_package(), "@agent-grimoire/linux-x64");
        assert_eq!(linux.npm_dir(), "agent-grimoire-linux-x64");
        let windows = find("aarch64-pc-windows-msvc").unwrap();
        assert_eq!(
            windows.archive("grimoire-tool-x", "0.1.0"),
            "grimoire-tool-x-0.1.0-aarch64-pc-windows-msvc.zip"
        );
        assert_eq!(windows.binary_file(BIN), "grimoire.exe");
        for t in TARGETS {
            let name = t.archive(PACKAGE, "0.1.0");
            assert!(
                name.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
                "{name}"
            );
        }
    }

    #[test]
    fn dry_run_targets_and_unique_npm_packages() {
        // One target per OS, plus arm64 Linux for the two-platform image.
        let dry: Vec<&str> = for_mode(true).map(|t| t.triple).collect();
        assert_eq!(
            dry,
            [
                "x86_64-unknown-linux-musl",
                "aarch64-unknown-linux-musl",
                "aarch64-apple-darwin",
                "x86_64-pc-windows-msvc"
            ]
        );
        let images: Vec<&str> = TARGETS.iter().filter_map(|t| t.image_arch).collect();
        assert_eq!(images, ["amd64", "arm64"]);
        assert!(
            TARGETS
                .iter()
                .all(|t| t.image_arch.is_none() || (t.dry_run && t.triple.ends_with("-linux-musl")))
        );
        assert_eq!(for_mode(false).count(), 6);
        let mut npm: Vec<String> = TARGETS.iter().map(Target::npm_package).collect();
        npm.sort();
        npm.dedup();
        assert_eq!(npm.len(), TARGETS.len());
        assert!(find("x86_64-unknown-linux-gnu").is_err());
    }
}
