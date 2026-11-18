//! `cargo xtask release image …`: the container image (Phase 6, ADR-0011).
//!
//! The image is the release's static musl binaries on distroless static, built by
//! BuildKit with COPY steps only, so nothing runs during the build and no platform
//! needs emulation. These steps hold the logic, so `release.yml` stays glue:
//!
//! - `base` prints the base image index that the `Dockerfile` pins by digest.
//! - `context` checks that index, as the registry served it, against the pin; unpacks
//!   the Linux binaries from the release archives into a build context; and writes the
//!   `docker buildx build` platform and annotation arguments (`buildx-args`), the image
//!   platforms (`platforms`), the base references whose signatures the workflow verifies
//!   (`base-refs`), `SOURCE_DATE_EPOCH` (the commit time) and a plan for `check`.
//! - `check` checks the OCI layout buildx wrote against that plan: one index holding
//!   exactly the two platforms and their attestations, the annotations and image
//!   configs, the pinned base's layers, and layers on top that hold exactly the archived
//!   binaries, byte for byte. It prints the digests.
//! - `coverage` checks that Grype found Rust packages in every binary (cargo-auditable
//!   data). Grype reports a binary without that data as holding nothing, not as unknown.
//!
//! None of these runs in a job that can mint OIDC tokens or push: there, a dependency's
//! build script could use them (ADR-0011).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};

use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::archive;
use super::checksums::sha256_hex;
use super::meta;
use super::targets::{self, BIN, PACKAGE, REPO_URL};

/// The image, always written in lowercase: an OCI repository name must be, and
/// `github.repository` (`jabbott-iii/Grimoire`) is not (ADR-0004, REGISTRY-P6).
pub(crate) const IMAGE: &str = "ghcr.io/jabbott-iii/grimoire";
/// Where the CLI goes, on the image's `PATH`.
pub(crate) const CLI_DIR: &str = "usr/local/bin";
/// Where asset binaries go: off `PATH`, so `grimoire doctor` in a container never
/// mistakes them for the host's (RUN-P6).
pub(crate) const ASSET_DIR: &str = "opt/grimoire/bin";
/// distroless's `nonroot` user and group.
pub(crate) const USER: &str = "65532:65532";
/// The working directory, where a repository is mounted.
pub(crate) const WORKDIR: &str = "/repo";
/// The workspace licence (root `Cargo.toml`).
pub(crate) const LICENSES: &str = "MIT OR Apache-2.0";
/// GHCR shows at most 512 characters of a description (REGISTRY-P6).
const MAX_DESCRIPTION: usize = 512;

const DOCKERFILE: &str = "Dockerfile";
/// The base index inside `--base`, exactly as the registry served it.
const BASE_INDEX: &str = "index.json";
const PLAN: &str = "image-plan.json";

const OCI_INDEX: &str = "application/vnd.oci.image.index.v1+json";
const OCI_MANIFEST: &str = "application/vnd.oci.image.manifest.v1+json";
const DOCKER_LIST: &str = "application/vnd.docker.distribution.manifest.list.v2+json";
const OCI_LAYER_GZIP: &str = "application/vnd.oci.image.layer.v1.tar+gzip";
const SPDX: &str = "https://spdx.dev/Document";
const SLSA_PROVENANCE: &str = "https://slsa.dev/provenance/v1";
/// The key BuildKit's GitHub Actions provenance context uses for the event payload.
const GHA_PAYLOAD: &str = "github_event_payload";

/// The most a manifest, config or attestation blob may hold.
const MAX_JSON: u64 = 4 << 20;
/// The most a layer may hold, compressed or not (as for release archives).
const MAX_LAYER: u64 = super::zip::MAX_TOTAL;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Print the base image index the Dockerfile pins (`repository@sha256:…`).
    Base,
    /// Write the build context, the buildx arguments and the plan for `check` into OUT.
    Context {
        /// The release assets (the CLI and asset archives).
        #[arg(long, value_name = "DIR")]
        dist: PathBuf,
        /// A directory holding the base index as `index.json`, as the registry served it.
        #[arg(long, value_name = "DIR")]
        base: PathBuf,
        /// A new directory for the results.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
    },
    /// Check the OCI layout buildx wrote against the plan, and print its digests.
    Check {
        /// The directory `context` wrote.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
        /// The OCI layout (`--output type=oci,tar=false`).
        #[arg(long, value_name = "DIR")]
        layout: PathBuf,
        /// The base index and the platform manifests named in `OUT/base-refs`, each as
        /// `<hex digest>.json`.
        #[arg(long, value_name = "DIR")]
        base: PathBuf,
        /// Append the digests to this file (`$GITHUB_OUTPUT`) as well as printing them.
        #[arg(long, value_name = "FILE")]
        github_output: Option<PathBuf>,
    },
    /// Check that Grype saw the base's Debian packages and every binary's Rust packages.
    Coverage {
        /// The directory `context` wrote.
        #[arg(long, value_name = "DIR")]
        out: PathBuf,
        /// Grype's `-o json` output for one platform.
        #[arg(long, value_name = "FILE")]
        report: PathBuf,
        /// Grype's `-o cyclonedx-json` output for the same scan.
        #[arg(long, value_name = "FILE")]
        cdx: PathBuf,
        /// The platform it scanned, such as `linux/amd64`.
        #[arg(long)]
        platform: String,
    },
}

#[allow(clippy::print_stdout, reason = "these steps print results for the log")]
pub(crate) fn run(
    root: &Path,
    at: impl Fn(&Path) -> PathBuf,
    command: Command,
) -> Result<(), String> {
    match command {
        Command::Base => {
            let base = parse_dockerfile(&read_text(&root.join(DOCKERFILE))?)?;
            println!("{}", base.index_ref());
        }
        Command::Context { dist, base, out } => {
            let text = context(root, &at(&dist), &at(&base), &at(&out))?;
            print!("{text}");
        }
        Command::Check {
            out,
            layout,
            base,
            github_output,
        } => {
            let digests = check(&at(&out), &at(&layout), &at(&base))?;
            print!("{digests}");
            if let Some(path) = github_output {
                super::append_output(&path, &digests)?;
            }
        }
        Command::Coverage {
            out,
            report,
            cdx,
            platform,
        } => {
            println!(
                "{}",
                coverage(&at(&out), &at(&report), &at(&cdx), &platform)?
            );
        }
    }
    Ok(())
}

fn read(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

fn read_text(path: &Path) -> Result<String, String> {
    String::from_utf8(read(path)?).map_err(|_| format!("{} is not UTF-8", path.display()))
}

fn write(path: &Path, bytes: &[u8]) -> Result<(), String> {
    fs::write(path, bytes).map_err(|e| format!("cannot write {}: {e}", path.display()))
}

fn parse_json(bytes: &[u8], what: &str) -> Result<Value, String> {
    serde_json::from_slice(bytes).map_err(|e| format!("{what} is not JSON: {e}"))
}

fn is_digest(s: &str) -> bool {
    s.strip_prefix("sha256:").is_some_and(|hex| {
        hex.len() == 64
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

/// The base image, as the `Dockerfile` pins it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Base {
    /// `gcr.io/distroless/static-debian13:nonroot`
    pub(crate) name: String,
    /// `gcr.io/distroless/static-debian13`
    pub(crate) repository: String,
    /// The index digest, `sha256:…`.
    pub(crate) digest: String,
}

impl Base {
    /// `repository@digest`.
    pub(crate) fn index_ref(&self) -> String {
        format!("{}@{}", self.repository, self.digest)
    }
}

/// The first line: BuildKit's build checks fail the build instead of warning.
pub(crate) const CHECK_DIRECTIVE: &str = "# check=error=true";

/// Reads the `Dockerfile`'s one `FROM`, and refuses anything that would make the
/// build run code, pull by tag or parse differently from how this reads it. Only
/// `FROM`, `ARG`, `COPY --chmod=0755 <src> <dst>`, `WORKDIR`, `USER`, `LABEL`,
/// `ENTRYPOINT` and `CMD` are allowed; so no `RUN`, `ADD`, `ONBUILD` or `COPY --from`.
/// Also refused: a `# syntax=` or `# escape=` parser directive, line continuations,
/// heredocs, a base without a digest on the same line, and a first line other than
/// [`CHECK_DIRECTIVE`].
pub(crate) fn parse_dockerfile(text: &str) -> Result<Base, String> {
    if text.lines().next() != Some(CHECK_DIRECTIVE) {
        return Err(format!(
            "Dockerfile:1: the first line must be `{CHECK_DIRECTIVE}`"
        ));
    }
    let mut from = Vec::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        let at = n + 1;
        if let Some(comment) = line.strip_prefix('#') {
            let directive: String = comment
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            if directive.starts_with("syntax=") {
                return Err(format!(
                    "Dockerfile:{at}: no `# syntax=` line: the frontend is the one in the pinned BuildKit image"
                ));
            }
            if directive.starts_with("escape=") {
                return Err(format!(
                    "Dockerfile:{at}: no `# escape=` line: it would change how lines continue"
                ));
            }
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if line.ends_with('\\') {
            return Err(format!("Dockerfile:{at}: no line continuations"));
        }
        if line.contains("<<") {
            return Err(format!("Dockerfile:{at}: no heredocs"));
        }
        let mut words = line.split_whitespace();
        let instruction = words.next().unwrap_or_default().to_ascii_uppercase();
        match instruction.as_str() {
            "ARG" | "WORKDIR" | "USER" | "LABEL" | "ENTRYPOINT" | "CMD" => {}
            "COPY" => {
                // Only `COPY --chmod=0755 <src> <dst>`: no --from (an image pulled by tag),
                // --chown, --link or other flags.
                let args: Vec<&str> = words.collect();
                let ["--chmod=0755", src, dst] = args.as_slice() else {
                    return Err(format!(
                        "Dockerfile:{at}: write COPY as `COPY --chmod=0755 <src> <dst>`"
                    ));
                };
                if src.starts_with('-') || dst.starts_with('-') {
                    return Err(format!(
                        "Dockerfile:{at}: COPY takes one flag, --chmod=0755"
                    ));
                }
            }
            "FROM" => {
                let args: Vec<&str> = words.collect();
                let [reference] = args.as_slice() else {
                    return Err(format!(
                        "Dockerfile:{at}: write the base as `FROM name:tag@sha256:…`, with no flags or stage name"
                    ));
                };
                from.push((at, (*reference).to_owned()));
            }
            other => {
                return Err(format!(
                    "Dockerfile:{at}: {other} is not allowed: the image takes FROM, ARG, COPY, WORKDIR, USER, LABEL, ENTRYPOINT and CMD only"
                ));
            }
        }
    }
    let [(at, reference)] = from.as_slice() else {
        return Err(format!(
            "the Dockerfile must have exactly one FROM, not {}",
            from.len()
        ));
    };
    parse_base(*at, reference)
}

/// `name:tag@sha256:…`, lowercase, with an optional registry port.
fn parse_base(at: usize, reference: &str) -> Result<Base, String> {
    let (name, digest) = reference
        .split_once('@')
        .ok_or_else(|| format!("Dockerfile:{at}: the base must be pinned by digest"))?;
    if !is_digest(digest) {
        return Err(format!(
            "Dockerfile:{at}: `{digest}` is not a sha256 digest"
        ));
    }
    let slash = name.rfind('/').unwrap_or(0);
    let (repository, tag) = match name[slash..].find(':') {
        Some(colon) => (&name[..slash + colon], &name[slash + colon + 1..]),
        None => {
            return Err(format!(
                "Dockerfile:{at}: name the base's tag too (`name:tag@sha256:…`)"
            ));
        }
    };
    let valid = |s: &str, extra: &[u8]| {
        !s.is_empty()
            && s.bytes().all(|b| {
                b.is_ascii_lowercase()
                    || b.is_ascii_digit()
                    || b"._-".contains(&b)
                    || extra.contains(&b)
            })
    };
    // The registry host may carry a port; every other component is a lowercase name.
    let (host, path) = repository.split_once('/').unwrap_or(("", repository));
    if (!host.is_empty() && !valid(host, b":"))
        || !path.split('/').all(|c| valid(c, b""))
        || !valid(tag, b"")
    {
        return Err(format!(
            "Dockerfile:{at}: `{name}` is not a lowercase image name and tag"
        ));
    }
    Ok(Base {
        name: name.to_owned(),
        repository: repository.to_owned(),
        digest: digest.to_owned(),
    })
}

/// The digests of the base index's `linux/{arch}` manifests, for every image
/// architecture, after checking that `bytes` are the pinned index.
pub(crate) fn base_platforms(
    base: &Base,
    bytes: &[u8],
) -> Result<BTreeMap<String, String>, String> {
    let actual = format!("sha256:{}", sha256_hex(bytes));
    if actual != base.digest {
        return Err(format!(
            "the base index has digest {actual}, not the {} the Dockerfile pins",
            base.digest
        ));
    }
    let index = parse_json(bytes, "the base index")?;
    let media = index.get("mediaType").and_then(Value::as_str);
    if media != Some(OCI_INDEX) && media != Some(DOCKER_LIST) {
        return Err("the base is not a multi-platform index".to_owned());
    }
    let manifests = index
        .get("manifests")
        .and_then(Value::as_array)
        .ok_or("the base index lists no manifests")?;
    let mut found = BTreeMap::new();
    for arch in image_archs() {
        let matches: Vec<&str> = manifests
            .iter()
            .filter(|m| {
                let p = m.get("platform");
                let field = |k: &str| p.and_then(|p| p.get(k)).and_then(Value::as_str);
                field("os") == Some("linux")
                    && field("architecture") == Some(arch)
                    // arm64 is published as arm64/v8; BuildKit treats v8 and none alike.
                    && matches!(field("variant"), None | Some("v8" | "8"))
            })
            .filter_map(|m| m.get("digest").and_then(Value::as_str))
            .collect();
        let [digest] = matches.as_slice() else {
            return Err(format!(
                "the base index has {} linux/{arch} manifests, not one",
                matches.len()
            ));
        };
        if !is_digest(digest) {
            return Err(format!(
                "the base's linux/{arch} digest `{digest}` is malformed"
            ));
        }
        found.insert(arch.to_owned(), (*digest).to_owned());
    }
    Ok(found)
}

/// The image's architectures, in [`targets::TARGETS`] order.
fn image_archs() -> impl Iterator<Item = &'static str> {
    targets::TARGETS.iter().filter_map(|t| t.image_arch)
}

/// Refuses anything but a static ELF executable for `arch`: the base has no libc and
/// no dynamic loader, so a dynamically linked binary would not start.
pub(crate) fn check_elf(data: &[u8], arch: &str) -> Result<(), String> {
    let machine: u16 = match arch {
        "amd64" => 62,
        "arm64" => 183,
        other => return Err(format!("no ELF machine known for {other}")),
    };
    let bytes_at = |off: usize, n: usize| {
        off.checked_add(n)
            .and_then(|end| data.get(off..end))
            .ok_or("truncated ELF header")
    };
    let u16_at = |off: usize| bytes_at(off, 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let u32_at =
        |off: usize| bytes_at(off, 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let u64_at = |off: usize| {
        bytes_at(off, 8)
            .map(|b| u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    };
    // 64-bit (class 2), little-endian (data 1).
    if data.get(..6) != Some(b"\x7fELF\x02\x01".as_slice()) {
        return Err("not a 64-bit little-endian ELF file".to_owned());
    }
    if !matches!(u16_at(16)?, 2 | 3) {
        return Err("not an ELF executable".to_owned());
    }
    if u16_at(18)? != machine {
        return Err(format!("built for ELF machine {}, not {arch}", u16_at(18)?));
    }
    let phoff = usize::try_from(u64_at(32)?).map_err(|_| "bad ELF header")?;
    let phentsize = usize::from(u16_at(54)?);
    let phnum = usize::from(u16_at(56)?);
    if phentsize < 4 {
        return Err("bad ELF program header size".to_owned());
    }
    for i in 0..phnum {
        let entry = phoff
            .checked_add(i.checked_mul(phentsize).ok_or("bad ELF header")?)
            .ok_or("bad ELF header")?;
        // PT_INTERP: a dynamic loader, which the base doesn't have.
        if u32_at(entry)? == 3 {
            return Err("dynamically linked (it names a program interpreter)".to_owned());
        }
    }
    Ok(())
}

/// `1970-01-01T00:00:00Z` plus `epoch` seconds, in RFC 3339.
pub(crate) fn rfc3339(epoch: u64) -> String {
    let days = epoch / 86_400;
    let secs = epoch % 86_400;
    // Howard Hinnant's civil_from_days, for days since 1970-01-01.
    let z = days + 719_468;
    let era = z / 146_097;
    let doe = z % 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + u64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        secs / 3600,
        secs % 3600 / 60,
        secs % 60
    )
}

/// What `check` and `coverage` compare the layout and the scan with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Plan {
    pub(crate) version: String,
    pub(crate) index_annotations: BTreeMap<String, String>,
    /// The Debian major version of the base (`13` for `static-debian13`).
    pub(crate) debian: String,
    /// Every directory the image's own layers may hold, with its mode and owner.
    pub(crate) dirs: BTreeMap<String, Owned>,
    pub(crate) platforms: Vec<PlatformPlan>,
}

/// A mode and an owner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Owned {
    pub(crate) mode: u32,
    pub(crate) uid: u64,
    pub(crate) gid: u64,
}

/// The directories BuildKit writes into the image's own layers: the parents of the
/// two binary directories (root-owned), and the working directory, which `WORKDIR`
/// creates as the base's user.
pub(crate) fn expected_dirs() -> Result<BTreeMap<String, Owned>, String> {
    let root = Owned {
        mode: 0o755,
        uid: 0,
        gid: 0,
    };
    let mut dirs = BTreeMap::new();
    for path in [CLI_DIR, ASSET_DIR] {
        let mut prefix = String::new();
        for part in path.split('/') {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(part);
            dirs.insert(prefix.clone(), root);
        }
    }
    let (uid, gid) = USER
        .split_once(':')
        .and_then(|(u, g)| Some((u.parse().ok()?, g.parse().ok()?)))
        .ok_or("USER is not uid:gid")?;
    dirs.insert(
        WORKDIR.trim_start_matches('/').to_owned(),
        Owned {
            mode: 0o755,
            uid,
            gid,
        },
    );
    Ok(dirs)
}

/// The Debian major version in a distroless base name such as
/// `gcr.io/distroless/static-debian13:nonroot`.
pub(crate) fn debian_major(name: &str) -> Result<String, String> {
    let rest = name
        .rfind("debian")
        .map(|i| &name[i + "debian".len()..])
        .unwrap_or_default();
    let major: String = rest.chars().take_while(char::is_ascii_digit).collect();
    if major.is_empty() {
        return Err(format!("`{name}` names no Debian version"));
    }
    Ok(major)
}

/// One platform of the image.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PlatformPlan {
    /// `amd64` or `arm64`.
    pub(crate) architecture: String,
    /// The base's manifest for this platform.
    pub(crate) base_digest: String,
    pub(crate) annotations: BTreeMap<String, String>,
    /// Every file the image adds (path without a leading `/`) and its SHA-256.
    pub(crate) files: BTreeMap<String, String>,
}

impl PlatformPlan {
    fn platform(&self) -> String {
        format!("linux/{}", self.architecture)
    }
}

/// Annotation keys and values.
pub(crate) type Annotations = BTreeMap<String, String>;

/// The annotations every manifest and the index carry, and the per-platform base ones.
pub(crate) fn annotations(
    version: &str,
    description: &str,
    revision: &str,
    epoch: u64,
    base: &Base,
    platforms: &BTreeMap<String, String>,
) -> Result<(Annotations, BTreeMap<String, Annotations>), String> {
    if description.chars().count() > MAX_DESCRIPTION {
        return Err(format!(
            "the description is longer than {MAX_DESCRIPTION} characters"
        ));
    }
    if revision.len() != 40 || !revision.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("`{revision}` is not a full commit SHA"));
    }
    let key = |k: &str| format!("org.opencontainers.image.{k}");
    let mut common = BTreeMap::new();
    for (k, v) in [
        ("created", rfc3339(epoch)),
        ("description", description.to_owned()),
        (
            "documentation",
            format!("{REPO_URL}/blob/v{version}/docs/container.md"),
        ),
        ("licenses", LICENSES.to_owned()),
        ("revision", revision.to_owned()),
        ("source", REPO_URL.to_owned()),
        ("title", BIN.to_owned()),
        ("url", REPO_URL.to_owned()),
        ("version", version.to_owned()),
        ("base.name", base.name.clone()),
    ] {
        common.insert(key(k), v);
    }
    let mut index = common.clone();
    index.insert(key("base.digest"), base.digest.clone());
    let mut per_platform = BTreeMap::new();
    for (arch, digest) in platforms {
        let mut m = common.clone();
        m.insert(key("base.digest"), digest.clone());
        per_platform.insert(arch.clone(), m);
    }
    Ok((index, per_platform))
}

/// The `docker buildx build` arguments `context` writes, one per line: the platforms
/// and every annotation, each at the level it belongs to.
pub(crate) fn buildx_args(plan: &Plan) -> Result<String, String> {
    let mut lines = vec![
        "--platform".to_owned(),
        plan.platforms
            .iter()
            .map(PlatformPlan::platform)
            .collect::<Vec<_>>()
            .join(","),
    ];
    for (k, v) in &plan.index_annotations {
        lines.push("--annotation".to_owned());
        lines.push(format!("index:{k}={v}"));
    }
    for p in &plan.platforms {
        for (k, v) in &p.annotations {
            lines.push("--annotation".to_owned());
            lines.push(format!("manifest[{}]:{k}={v}", p.platform()));
        }
    }
    if let Some(bad) = lines.iter().find(|l| l.contains(['\n', '\r'])) {
        return Err(format!("a buildx argument holds a line break: {bad:?}"));
    }
    let mut text = lines.join("\n");
    text.push('\n');
    Ok(text)
}

fn git_text(root: &Path, args: &[&str]) -> Result<String, String> {
    let out = crate::git::run(root, args)?;
    Ok(String::from_utf8_lossy(&out).trim().to_owned())
}

/// `cargo xtask release image context`.
fn context(root: &Path, dist: &Path, base_dir: &Path, out: &Path) -> Result<String, String> {
    let release = meta::load(root)?;
    let base = parse_dockerfile(&read_text(&root.join(DOCKERFILE))?)?;
    let platforms = base_platforms(&base, &read(&base_dir.join(BASE_INDEX))?)?;
    if out.exists() {
        return Err(format!("{} already exists", out.display()));
    }
    let revision = git_text(root, &["rev-parse", "HEAD"])?;
    let epoch: u64 = git_text(root, &["log", "-1", "--format=%ct", "HEAD"])?
        .parse()
        .map_err(|_| "cannot read the commit time".to_owned())?;
    let (index_annotations, mut per_platform) = annotations(
        &release.version,
        &release.description,
        &revision,
        epoch,
        &base,
        &platforms,
    )?;

    let mut plan = Plan {
        version: release.version.clone(),
        index_annotations,
        debian: debian_major(&base.name)?,
        dirs: expected_dirs()?,
        platforms: Vec::new(),
    };
    for target in targets::TARGETS {
        let Some(arch) = target.image_arch else {
            continue;
        };
        let dir = out.join("context/linux").join(arch);
        let mut files = BTreeMap::new();
        let mut bins = vec![(
            target.archive(PACKAGE, &release.version),
            BIN,
            "cli",
            CLI_DIR,
        )];
        for a in &release.assets {
            bins.push((
                target.archive(&a.bin, &a.version),
                a.bin.as_str(),
                "assets",
                ASSET_DIR,
            ));
        }
        for (name, bin, sub, dest) in bins {
            let bytes = read(&dist.join(&name))?;
            let members = archive::read_canonical(target.format, &bytes)
                .map_err(|e| format!("{name}: {e}"))?;
            let data = &archive::member(&members, bin)
                .map_err(|e| format!("{name}: {e}"))?
                .data;
            check_elf(data, arch).map_err(|e| format!("{name}: {bin}: {e}"))?;
            let to = dir.join(sub);
            fs::create_dir_all(&to).map_err(|e| format!("cannot create {}: {e}", to.display()))?;
            write(&to.join(bin), data)?;
            files.insert(format!("{dest}/{bin}"), sha256_hex(data));
        }
        plan.platforms.push(PlatformPlan {
            architecture: arch.to_owned(),
            base_digest: platforms
                .get(arch)
                .cloned()
                .ok_or_else(|| format!("no base digest for {arch}"))?,
            annotations: per_platform
                .remove(arch)
                .ok_or_else(|| format!("no annotations for {arch}"))?,
            files,
        });
    }

    write(&out.join("buildx-args"), buildx_args(&plan)?.as_bytes())?;
    let list: String = plan.platforms.iter().map(|p| p.platform() + "\n").collect();
    write(&out.join("platforms"), list.as_bytes())?;
    write(
        &out.join("source-date-epoch"),
        format!("{epoch}\n").as_bytes(),
    )?;
    let mut refs = format!("{}\n", base.index_ref());
    for p in &plan.platforms {
        let _ = writeln!(refs, "{}@{}", base.repository, p.base_digest);
    }
    write(&out.join("base-refs"), refs.as_bytes())?;
    let json = serde_json::to_vec_pretty(&plan).map_err(|e| e.to_string())?;
    write(&out.join(PLAN), &json)?;
    Ok(format!(
        "image context: {} platforms, {} binaries each, base {}\n{refs}",
        plan.platforms.len(),
        plan.platforms.first().map_or(0, |p| p.files.len()),
        base.index_ref()
    ))
}

fn load_plan(out: &Path) -> Result<Plan, String> {
    serde_json::from_slice(&read(&out.join(PLAN))?)
        .map_err(|e| format!("{}: {e}", out.join(PLAN).display()))
}

/// A content-addressed blob store: an OCI layout's `blobs/sha256`, or a directory of
/// `<hex>.json` files. Every read checks the size limit and the digest.
struct Blobs {
    dir: PathBuf,
    suffix: &'static str,
}

impl Blobs {
    fn get(&self, digest: &str, limit: u64) -> Result<Vec<u8>, String> {
        if !is_digest(digest) {
            return Err(format!("`{digest}` is not a sha256 digest"));
        }
        let hex = &digest["sha256:".len()..];
        let path = self.dir.join(format!("{hex}{}", self.suffix));
        let meta = fs::symlink_metadata(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        if !meta.is_file() {
            return Err(format!("{} is not a regular file", path.display()));
        }
        if meta.len() > limit {
            return Err(format!("{} is larger than {limit} bytes", path.display()));
        }
        let bytes = read(&path)?;
        if sha256_hex(&bytes) != hex {
            return Err(format!("{} does not match its digest", path.display()));
        }
        Ok(bytes)
    }

    /// The blob a descriptor names, checking its size field too.
    fn descriptor(&self, desc: &Value, limit: u64) -> Result<Vec<u8>, String> {
        let digest = desc
            .get("digest")
            .and_then(Value::as_str)
            .ok_or("a descriptor has no digest")?;
        let bytes = self.get(digest, limit)?;
        if desc.get("size").and_then(Value::as_u64) != Some(bytes.len() as u64) {
            return Err(format!("{digest}: the descriptor's size is wrong"));
        }
        Ok(bytes)
    }
}

fn str_map(value: Option<&Value>) -> BTreeMap<String, String> {
    value
        .and_then(Value::as_object)
        .map(|o| {
            o.iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or_default().to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

fn field<'a>(v: &'a Value, path: &[&str]) -> Option<&'a Value> {
    path.iter().try_fold(v, |v, k| v.get(k))
}

fn diff(
    what: &str,
    expected: &BTreeMap<String, String>,
    actual: &BTreeMap<String, String>,
) -> Result<(), String> {
    if expected == actual {
        return Ok(());
    }
    let mut lines = vec![format!("{what} differ from the plan:")];
    for (k, v) in expected {
        match actual.get(k) {
            Some(a) if a == v => {}
            Some(a) => lines.push(format!("  {k}: expected {v:?}, found {a:?}")),
            None => lines.push(format!("  {k}: missing")),
        }
    }
    for k in actual.keys().filter(|k| !expected.contains_key(*k)) {
        lines.push(format!("  {k}: unexpected"));
    }
    Err(lines.join("\n"))
}

/// A regular file in a layer.
#[derive(Debug)]
struct LayerFile {
    path: String,
    owned: Owned,
    data: Vec<u8>,
}

/// What a layer holds.
#[derive(Debug, Default)]
struct LayerEntries {
    files: Vec<LayerFile>,
    dirs: Vec<(String, Owned)>,
}

/// Reads a gzipped layer tarball strictly enough that no other reader can see
/// something else in it, as Go's gzip and tar (Docker, containerd) would:
/// - exactly one gzip member and nothing after it (Go reads every member);
/// - the uncompressed bytes hash to `diff_id`, which the runtime checks too;
/// - only directories and regular files, each with a plain header: no PAX or GNU
///   extension records (they can rename an entry or add xattrs, such as file
///   capabilities), and a directory's size field is 0 (Go skips no data for it);
/// - no unsafe paths or whiteouts, and nothing but zeros after the archive's end.
fn layer_entries(bytes: &[u8], diff_id: &str) -> Result<LayerEntries, String> {
    let mut gz = flate2::bufread::GzDecoder::new(bytes);
    let mut raw = Vec::new();
    (&mut gz)
        .take(MAX_LAYER + 1)
        .read_to_end(&mut raw)
        .map_err(|e| format!("not a gzip stream: {e}"))?;
    if raw.len() as u64 > MAX_LAYER {
        return Err(format!("the layer holds more than {MAX_LAYER} bytes"));
    }
    if !gz.into_inner().is_empty() {
        return Err("the layer has data after its gzip member".to_owned());
    }
    if format!("sha256:{}", sha256_hex(&raw)) != diff_id {
        return Err("the layer does not match its diff_id".to_owned());
    }
    let mut archive = tar::Archive::new(std::io::Cursor::new(raw.as_slice()));
    let mut found = LayerEntries::default();
    for entry in archive
        .entries()
        .map_err(|e| format!("not a tarball: {e}"))?
    {
        let mut entry = entry.map_err(|e| format!("corrupt layer: {e}"))?;
        if entry
            .pax_extensions()
            .map_err(|e| format!("corrupt layer: {e}"))?
            .is_some()
            || entry.path_bytes() != entry.header().path_bytes()
        {
            return Err("a layer entry uses PAX or GNU extension records".to_owned());
        }
        let path = entry
            .path()
            .map_err(|e| format!("bad layer path: {e}"))?
            .to_str()
            .ok_or("non-UTF-8 layer path")?
            .trim_start_matches("./")
            .trim_end_matches('/')
            .to_owned();
        if path.is_empty()
            || path.starts_with('/')
            || path
                .split('/')
                .any(|c| c.is_empty() || c == "." || c == ".." || c.starts_with(".wh."))
        {
            return Err(format!("layer path `{path}` is not allowed"));
        }
        let header = entry.header();
        let owned = Owned {
            mode: header.mode().map_err(|e| format!("{path}: {e}"))? & 0o7777,
            uid: header.uid().map_err(|e| format!("{path}: {e}"))?,
            gid: header.gid().map_err(|e| format!("{path}: {e}"))?,
        };
        let kind = header.entry_type();
        if kind.is_dir() {
            if header.size().map_err(|e| format!("{path}: {e}"))? != 0 {
                return Err(format!("layer directory `{path}` has a size"));
            }
            found.dirs.push((path, owned));
            continue;
        }
        if !kind.is_file() {
            return Err(format!(
                "layer entry `{path}` is not a regular file or directory"
            ));
        }
        let mut data = Vec::new();
        entry
            .read_to_end(&mut data)
            .map_err(|e| format!("{path}: {e}"))?;
        found.files.push(LayerFile { path, owned, data });
    }
    let end = usize::try_from(archive.into_inner().position()).unwrap_or(usize::MAX);
    if !raw
        .get(end..)
        .is_some_and(|rest| rest.iter().all(|b| *b == 0))
    {
        return Err("the layer has data after the end of its tarball".to_owned());
    }
    Ok(found)
}

/// `cargo xtask release image check`: returns the `key=value` lines it prints.
fn check(out: &Path, layout: &Path, base_dir: &Path) -> Result<String, String> {
    let plan = load_plan(out)?;
    let blobs = Blobs {
        dir: layout.join("blobs/sha256"),
        suffix: "",
    };
    let base_blobs = Blobs {
        dir: base_dir.to_path_buf(),
        suffix: ".json",
    };

    let oci_layout = parse_json(&read(&layout.join("oci-layout"))?, "oci-layout")?;
    if oci_layout.get("imageLayoutVersion").and_then(Value::as_str) != Some("1.0.0") {
        return Err("not an OCI image layout 1.0.0".to_owned());
    }
    let outer = parse_json(&read(&layout.join("index.json"))?, "index.json")?;
    let entries = outer
        .get("manifests")
        .and_then(Value::as_array)
        .ok_or("index.json lists no manifests")?;
    let [top_desc] = entries.as_slice() else {
        return Err(format!(
            "index.json must list exactly one image index, not {}",
            entries.len()
        ));
    };
    if top_desc.get("mediaType").and_then(Value::as_str) != Some(OCI_INDEX) {
        return Err("index.json does not point at an OCI image index".to_owned());
    }
    let index_digest = top_desc
        .get("digest")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let index = parse_json(&blobs.descriptor(top_desc, MAX_JSON)?, "the image index")?;
    if index.get("mediaType").and_then(Value::as_str) != Some(OCI_INDEX) {
        return Err("the image index has the wrong media type".to_owned());
    }
    diff(
        "the index annotations",
        &plan.index_annotations,
        &str_map(index.get("annotations")),
    )?;
    let manifests = index
        .get("manifests")
        .and_then(Value::as_array)
        .ok_or("the image index lists no manifests")?;
    if manifests.len() != 2 * plan.platforms.len() {
        return Err(format!(
            "the image index lists {} manifests, not {} (each platform and its attestations)",
            manifests.len(),
            2 * plan.platforms.len()
        ));
    }

    let mut text = format!("index-digest={index_digest}\n");
    for p in &plan.platforms {
        let digest = check_platform(&blobs, &base_blobs, manifests, &plan, p)?;
        let _ = writeln!(text, "{}-digest={digest}", p.architecture);
    }
    Ok(text)
}

/// One platform: its manifest, config, layers and attestations. Returns its digest.
fn check_platform(
    blobs: &Blobs,
    base_blobs: &Blobs,
    manifests: &[Value],
    plan: &Plan,
    p: &PlatformPlan,
) -> Result<String, String> {
    let platform = p.platform();
    let found: Vec<&Value> = manifests
        .iter()
        .filter(|m| {
            field(m, &["platform", "os"]).and_then(Value::as_str) == Some("linux")
                && field(m, &["platform", "architecture"]).and_then(Value::as_str)
                    == Some(&p.architecture)
                && field(m, &["platform", "variant"]).is_none()
        })
        .collect();
    let [desc] = found.as_slice() else {
        return Err(format!(
            "the index has {} {platform} manifests, not one",
            found.len()
        ));
    };
    let digest = desc
        .get("digest")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let manifest = parse_json(&blobs.descriptor(desc, MAX_JSON)?, &platform)?;
    if manifest.get("mediaType").and_then(Value::as_str) != Some(OCI_MANIFEST) {
        return Err(format!("{platform}: not an OCI image manifest"));
    }
    diff(
        &format!("the {platform} manifest annotations"),
        &p.annotations,
        &str_map(manifest.get("annotations")),
    )?;
    let base = parse_json(
        &base_blobs.get(&p.base_digest, MAX_JSON)?,
        "a base manifest",
    )?;
    let base_config = parse_json(
        &base_blobs.descriptor(
            base.get("config").ok_or("a base manifest has no config")?,
            MAX_JSON,
        )?,
        "a base config",
    )?;
    let diff_ids = check_config(blobs, &manifest, &base_config, p)?;
    check_layers(blobs, &manifest, &base, &diff_ids, plan, p)?;
    check_attestation(blobs, manifests, &digest, &platform)?;
    Ok(digest)
}
/// The platform's one attestation manifest must hold an SPDX SBOM and SLSA v1 provenance.
fn check_attestation(
    blobs: &Blobs,
    manifests: &[Value],
    digest: &str,
    platform: &str,
) -> Result<(), String> {
    let attestations: Vec<&Value> = manifests
        .iter()
        .filter(|m| {
            field(m, &["platform", "os"]).and_then(Value::as_str) == Some("unknown")
                && field(m, &["platform", "architecture"]).and_then(Value::as_str)
                    == Some("unknown")
                && field(m, &["annotations", "vnd.docker.reference.type"]).and_then(Value::as_str)
                    == Some("attestation-manifest")
                && field(m, &["annotations", "vnd.docker.reference.digest"]).and_then(Value::as_str)
                    == Some(digest)
        })
        .collect();
    let [att_desc] = attestations.as_slice() else {
        return Err(format!(
            "{platform}: {} attestation manifests, not one",
            attestations.len()
        ));
    };
    let att = parse_json(
        &blobs.descriptor(att_desc, MAX_JSON)?,
        "an attestation manifest",
    )?;
    if field(&att, &["subject", "digest"]).and_then(Value::as_str) != Some(digest) {
        return Err(format!(
            "{platform}: the attestation manifest's subject is not the image"
        ));
    }
    let types: BTreeSet<&str> = att
        .get("layers")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|l| {
            field(l, &["annotations", "in-toto.io/predicate-type"]).and_then(Value::as_str)
        })
        .collect();
    if types != BTreeSet::from([SPDX, SLSA_PROVENANCE]) {
        return Err(format!(
            "{platform}: the attestations are {types:?}, not an SPDX SBOM and SLSA v1 provenance"
        ));
    }
    // The public provenance must not carry the GitHub event payload, which BuildKit's
    // docker-container driver adds unless `provenance-add-gha=false` (BUILD-P6).
    for layer in att
        .get("layers")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let bytes = blobs.descriptor(layer, MAX_LAYER)?;
        if bytes
            .windows(GHA_PAYLOAD.len())
            .any(|w| w == GHA_PAYLOAD.as_bytes())
        {
            return Err(format!(
                "{platform}: the provenance holds the GitHub event payload; create the builder with provenance-add-gha=false"
            ));
        }
    }
    Ok(())
}

/// The image config: exactly the fields the Dockerfile sets, on the base's `Env`, with
/// the base's `diff_ids` first. Returns the `diff_ids`.
fn check_config(
    blobs: &Blobs,
    manifest: &Value,
    base_config: &Value,
    p: &PlatformPlan,
) -> Result<Vec<String>, String> {
    let platform = p.platform();
    let desc = manifest.get("config").ok_or("a manifest has no config")?;
    let config = parse_json(&blobs.descriptor(desc, MAX_JSON)?, "an image config")?;
    let keys = |v: Option<&Value>| -> BTreeSet<String> {
        v.and_then(Value::as_object)
            .map(|o| o.keys().cloned().collect())
            .unwrap_or_default()
    };
    let text = |path: &[&str]| field(&config, path).and_then(Value::as_str);
    let list = |v: &Value, path: &[&str]| -> Option<Vec<String>> {
        field(v, path).and_then(Value::as_array).map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect()
        })
    };
    let mut problems = Vec::new();
    let top = keys(Some(&config));
    let allowed_top = [
        "architecture",
        "config",
        "created",
        "history",
        "os",
        "rootfs",
    ];
    if !top.iter().all(|k| allowed_top.contains(&k.as_str()))
        || !["architecture", "config", "os", "rootfs"]
            .iter()
            .all(|k| top.contains(*k))
    {
        problems.push(format!("its top-level fields are {top:?}"));
    }
    let inner = keys(config.get("config"));
    let required = ["Cmd", "Entrypoint", "Env", "Labels", "User", "WorkingDir"];
    if !inner
        .iter()
        .all(|k| required.contains(&k.as_str()) || k == "ArgsEscaped")
        || !required.iter().all(|k| inner.contains(*k))
    {
        problems.push(format!("its config fields are {inner:?}"));
    }
    if field(&config, &["config", "ArgsEscaped"]).is_some_and(|v| v != &Value::Bool(true)) {
        problems.push("ArgsEscaped is not true".to_owned());
    }
    if text(&["os"]) != Some("linux") || text(&["architecture"]) != Some(&p.architecture) {
        problems.push("its platform is wrong".to_owned());
    }
    if text(&["config", "User"]) != Some(USER) {
        problems.push(format!(
            "User is {:?}, not {USER}",
            text(&["config", "User"])
        ));
    }
    if text(&["config", "WorkingDir"]) != Some(WORKDIR) {
        problems.push(format!("WorkingDir is not {WORKDIR}"));
    }
    if list(&config, &["config", "Entrypoint"]) != Some(vec![format!("/{CLI_DIR}/{BIN}")]) {
        problems.push(format!("Entrypoint is not /{CLI_DIR}/{BIN}"));
    }
    if list(&config, &["config", "Cmd"]) != Some(vec!["--help".to_owned()]) {
        problems.push("Cmd is not --help".to_owned());
    }
    // The base's Env, unchanged: a PATH that reached /opt/grimoire/bin would make the
    // asset binaries look installed to `grimoire doctor` (RUN-P6).
    let env = list(&config, &["config", "Env"]);
    if env.is_none() || env != list(base_config, &["config", "Env"]) {
        problems.push("Env is not the base's".to_owned());
    }
    if str_map(field(&config, &["config", "Labels"]))
        != BTreeMap::from([(
            "org.opencontainers.image.source".to_owned(),
            REPO_URL.to_owned(),
        )])
    {
        problems.push("its labels are not exactly org.opencontainers.image.source".to_owned());
    }
    if text(&["rootfs", "type"]) != Some("layers") {
        problems.push("rootfs.type is not layers".to_owned());
    }
    let diff_ids = list(&config, &["rootfs", "diff_ids"]).unwrap_or_default();
    let base_ids = list(base_config, &["rootfs", "diff_ids"]).unwrap_or_default();
    let layers = manifest
        .get("layers")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    if diff_ids.len() != layers
        || !diff_ids.iter().all(|d| is_digest(d))
        || base_ids.is_empty()
        || !diff_ids.starts_with(&base_ids)
    {
        problems.push("its diff_ids are not the base's followed by one per layer".to_owned());
    }
    if problems.is_empty() {
        Ok(diff_ids)
    } else {
        Err(format!("{platform} image config: {}", problems.join("; ")))
    }
}

/// The layers: first exactly the verified base's, byte for byte, then layers that hold
/// only the planned directories and exactly the archived binaries.
fn check_layers(
    blobs: &Blobs,
    manifest: &Value,
    base: &Value,
    diff_ids: &[String],
    plan: &Plan,
    p: &PlatformPlan,
) -> Result<(), String> {
    let platform = p.platform();
    let descriptors = |m: &Value| -> Vec<Value> {
        m.get("layers")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default()
    };
    let base_layers = descriptors(base);
    let layers = descriptors(manifest);
    if base_layers.is_empty() || layers.len() <= base_layers.len() {
        return Err(format!(
            "{platform}: the image doesn't start with the pinned base's {} layers",
            base_layers.len()
        ));
    }
    for (image, base) in layers.iter().zip(&base_layers) {
        if image.get("digest") != base.get("digest") || image.get("size") != base.get("size") {
            return Err(format!(
                "{platform}: the image's first layers are not exactly the pinned base's"
            ));
        }
        // The bytes in the layout are what Grype scans and crane pushes.
        blobs.descriptor(image, MAX_LAYER)?;
    }
    let mut seen = BTreeSet::new();
    for (i, desc) in layers.iter().enumerate().skip(base_layers.len()) {
        if desc.get("mediaType").and_then(Value::as_str) != Some(OCI_LAYER_GZIP) {
            return Err(format!("{platform}: a layer is not a gzipped tarball"));
        }
        let bytes = blobs.descriptor(desc, MAX_LAYER)?;
        let diff_id = diff_ids.get(i).map_or("", String::as_str);
        let found = layer_entries(&bytes, diff_id).map_err(|e| format!("{platform}: {e}"))?;
        for (path, owned) in found.dirs {
            if plan.dirs.get(&path) != Some(&owned) {
                return Err(format!(
                    "{platform}: the image adds directory `/{path}` ({:o}, {}:{}), which the plan doesn't list",
                    owned.mode, owned.uid, owned.gid
                ));
            }
        }
        for LayerFile { path, owned, data } in found.files {
            let Some(expected) = p.files.get(&path) else {
                return Err(format!(
                    "{platform}: the image adds `/{path}`, which is not a release binary"
                ));
            };
            if owned.mode != 0o755 || owned.uid != 0 || owned.gid != 0 {
                return Err(format!(
                    "{platform}: /{path} is not mode 0755 owned by root"
                ));
            }
            if &sha256_hex(&data) != expected {
                return Err(format!("{platform}: /{path} is not the archived binary"));
            }
            if !seen.insert(path.clone()) {
                return Err(format!("{platform}: /{path} is in more than one layer"));
            }
        }
    }
    let missing: Vec<&String> = p.files.keys().filter(|f| !seen.contains(*f)).collect();
    if !missing.is_empty() {
        return Err(format!("{platform}: the image lacks {missing:?}"));
    }
    Ok(())
}

/// `cargo xtask release image coverage`: Grype must have seen the base's Debian
/// packages and the Rust packages of every binary. A scan that sees neither reports
/// nothing, which would pass the gate.
fn coverage(out: &Path, report: &Path, cdx: &Path, platform: &str) -> Result<String, String> {
    let plan = load_plan(out)?;
    let p = plan
        .platforms
        .iter()
        .find(|p| p.platform() == platform)
        .ok_or_else(|| format!("the image has no {platform}"))?;
    let report = parse_json(&read(report)?, "the Grype JSON report")?;
    let cdx = parse_json(&read(cdx)?, "the CycloneDX report")?;
    let debian = debian_coverage(&report, &cdx, &plan.debian)?;
    let rust = rust_coverage(&cdx, p.files.keys().map(|f| format!("/{f}")))?;
    Ok(format!("{debian}\n{rust}"))
}

/// Grype detected Debian `major` and catalogued at least one Debian package.
pub(crate) fn debian_coverage(report: &Value, cdx: &Value, major: &str) -> Result<String, String> {
    let name = field(report, &["distro", "name"]).and_then(Value::as_str);
    let version = field(report, &["distro", "version"])
        .and_then(Value::as_str)
        .unwrap_or_default();
    if name != Some("debian") || !(version == major || version.starts_with(&format!("{major}."))) {
        return Err(format!(
            "Grype detected the distribution as {name:?} {version:?}, not Debian {major}: it would match no Debian package"
        ));
    }
    let debs = cdx
        .get("components")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|c| {
            c.get("purl")
                .and_then(Value::as_str)
                .is_some_and(|p| p.starts_with("pkg:deb/debian/"))
        })
        .count();
    if debs == 0 {
        return Err("Grype found no Debian packages in the image".to_owned());
    }
    Ok(format!("Debian {version}: {debs} packages"))
}

/// Counts Rust packages per binary path, and fails if a binary has none.
pub(crate) fn rust_coverage(
    report: &Value,
    paths: impl Iterator<Item = String>,
) -> Result<String, String> {
    let mut counts: BTreeMap<String, usize> = paths.map(|p| (p, 0)).collect();
    for component in report
        .get("components")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        if !component
            .get("purl")
            .and_then(Value::as_str)
            .is_some_and(|p| p.starts_with("pkg:cargo/"))
        {
            continue;
        }
        let locations: BTreeSet<&str> = component
            .get("properties")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter(|prop| {
                prop.get("name")
                    .and_then(Value::as_str)
                    .is_some_and(|n| n.starts_with("syft:location:") && n.ends_with(":path"))
            })
            .filter_map(|prop| prop.get("value").and_then(Value::as_str))
            .collect();
        for location in locations {
            if let Some(n) = counts.get_mut(location) {
                *n += 1;
            }
        }
    }
    let empty: Vec<&String> = counts
        .iter()
        .filter(|(_, n)| **n == 0)
        .map(|(p, _)| p)
        .collect();
    if !empty.is_empty() {
        return Err(format!(
            "Grype found no Rust packages in {empty:?}: without cargo-auditable data the scan can't see their dependencies"
        ));
    }
    Ok(counts
        .iter()
        .map(|(p, n)| format!("{p}: {n} Rust packages"))
        .collect::<Vec<_>>()
        .join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PIN: &str = "sha256:25ac8f0c3b2334a9eb05452266bd4697864a8c8ef0ef71e5e6480d7f92868329";

    fn root() -> &'static Path {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or(Path::new("."))
    }

    fn elf(machine: u16, interp: bool) -> Vec<u8> {
        let mut data = vec![0u8; 64 + 56 * 2];
        data[..6].copy_from_slice(b"\x7fELF\x02\x01");
        data[16..18].copy_from_slice(&3u16.to_le_bytes());
        data[18..20].copy_from_slice(&machine.to_le_bytes());
        data[32..40].copy_from_slice(&64u64.to_le_bytes());
        data[54..56].copy_from_slice(&56u16.to_le_bytes());
        data[56..58].copy_from_slice(&2u16.to_le_bytes());
        data[64..68].copy_from_slice(&1u32.to_le_bytes()); // PT_LOAD
        let second: u32 = if interp { 3 } else { 2 };
        data[120..124].copy_from_slice(&second.to_le_bytes());
        data
    }

    /// The repository's Dockerfile follows the rules `context` enforces, and pins
    /// distroless static's nonroot image by digest.
    #[test]
    fn the_dockerfile_is_copy_only_and_pinned() {
        let text = fs::read_to_string(root().join(DOCKERFILE)).unwrap();
        let base = parse_dockerfile(&text).unwrap();
        assert_eq!(base.name, "gcr.io/distroless/static-debian13:nonroot");
        assert_eq!(base.repository, "gcr.io/distroless/static-debian13");
        assert!(is_digest(&base.digest), "{}", base.digest);
        assert!(
            text.lines()
                .next()
                .unwrap()
                .starts_with("# check=error=true")
        );
        for line in [
            format!("USER {USER}"),
            format!("WORKDIR {WORKDIR}"),
            format!("ENTRYPOINT [\"/{CLI_DIR}/{BIN}\"]"),
            format!("LABEL org.opencontainers.image.source=\"{REPO_URL}\""),
            format!("COPY --chmod=0755 ${{TARGETOS}}/${{TARGETARCH}}/cli/ /{CLI_DIR}/"),
            format!("COPY --chmod=0755 ${{TARGETOS}}/${{TARGETARCH}}/assets/ /{ASSET_DIR}/"),
        ] {
            assert!(text.lines().any(|l| l == line), "Dockerfile lacks `{line}`");
        }
    }

    #[test]
    fn dockerfiles_that_run_code_or_float_are_refused() {
        let df = |body: &str| format!("{CHECK_DIRECTIVE}\n{body}");
        assert!(
            parse_dockerfile(&df(&format!(
                "FROM a.io/b/c:d@{PIN}\nARG A\nCOPY --chmod=0755 x /y\nWORKDIR /w\nUSER 1:1\nLABEL a=b\nENTRYPOINT [\"/x\"]\nCMD [\"-h\"]\n"
            )))
            .is_ok()
        );
        // Without the first line, and with it anywhere but first.
        assert!(parse_dockerfile(&format!("FROM a.io/b/c:d@{PIN}\n")).is_err());
        assert!(
            parse_dockerfile(&format!("# hi\n{CHECK_DIRECTIVE}\nFROM a.io/b/c:d@{PIN}\n")).is_err()
        );
        for bad in [
            format!("# syntax=docker/dockerfile:1\nFROM a.io/b/c:d@{PIN}\n"),
            format!("#  Syntax = docker/dockerfile:1\nFROM a.io/b/c:d@{PIN}\n"),
            format!("# escape=`\nFROM a.io/b/c:d@{PIN}\nCOPY a `\n  /b\n"),
            format!("FROM a.io/b/c:d@{PIN}\nRUN true\n"),
            format!("FROM a.io/b/c:d@{PIN}\nadd x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nonbuild COPY x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY a \\\n  /b\n"),
            "FROM a.io/b/c:d\n".to_owned(),
            format!("FROM a.io/b/c@{PIN}\n"),
            format!("FROM --platform=linux/amd64 a.io/b/c:d@{PIN}\n"),
            format!("FROM a.io/b/c:d@{PIN} AS base\n"),
            format!("FROM a.io/b/c:d@{PIN}\nFROM a.io/b/c:d@{PIN}\n"),
            format!("FROM a.io/B/c:d@{PIN}\n"),
            "FROM a.io/b/c:d@sha256:1234\n".to_owned(),
            "COPY --chmod=0755 x /y\n".to_owned(),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --from=alpine:3 /x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --chmod=0755 --from=alpine:3 /x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --chown=0:0 x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --chmod=0777 x /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --chmod=0755 x y /z\n"),
            format!("FROM a.io/b/c:d@{PIN}\nCOPY --chmod=0755 <<EOF /y\n"),
            format!("FROM a.io/b/c:d@{PIN}\nENV PATH=/opt/grimoire/bin\n"),
            format!("FROM a.io/b/c:d@{PIN}\nHEALTHCHECK CMD [\"/x\"]\n"),
            format!("FROM a.io/b/c:d@{PIN}\nEXPOSE 80\n"),
            format!("FROM a.io/b/c:d@{PIN}\nVOLUME /data\n"),
            format!("FROM a.io/b/c:d@{PIN}\nSHELL [\"/bin/sh\"]\n"),
            format!("FROM a.io/b/c:d@{PIN}\nSTOPSIGNAL SIGKILL\n"),
        ] {
            assert!(parse_dockerfile(&df(&bad)).is_err(), "accepted:\n{bad}");
        }
        let base = parse_dockerfile(&df(&format!("FROM registry:5000/b/c:d@{PIN}\n"))).unwrap();
        assert_eq!(base.repository, "registry:5000/b/c");
        assert_eq!(base.index_ref(), format!("registry:5000/b/c@{PIN}"));
    }

    fn index(entries: &[(&str, &str, Option<&str>, char)]) -> Vec<u8> {
        let manifests: Vec<Value> = entries
            .iter()
            .map(|(os, arch, variant, c)| {
                let mut platform = serde_json::json!({"os": os, "architecture": arch});
                if let Some(v) = variant {
                    platform["variant"] = Value::from(*v);
                }
                serde_json::json!({
                    "mediaType": OCI_MANIFEST,
                    "digest": format!("sha256:{}", c.to_string().repeat(64)),
                    "size": 1,
                    "platform": platform,
                })
            })
            .collect();
        serde_json::to_vec(&serde_json::json!({"schemaVersion": 2, "mediaType": OCI_INDEX, "manifests": manifests}))
            .unwrap()
    }

    #[test]
    fn the_base_index_must_be_the_pinned_one_with_both_platforms() {
        let bytes = index(&[
            ("linux", "amd64", None, 'a'),
            ("linux", "arm64", Some("v8"), 'b'),
            ("linux", "arm", Some("v7"), 'c'),
        ]);
        let base = Base {
            name: "x/y:z".to_owned(),
            repository: "x/y".to_owned(),
            digest: format!("sha256:{}", sha256_hex(&bytes)),
        };
        let found = base_platforms(&base, &bytes).unwrap();
        assert_eq!(found["amd64"], format!("sha256:{}", "a".repeat(64)));
        assert_eq!(found["arm64"], format!("sha256:{}", "b".repeat(64)));
        let other = Base {
            digest: PIN.to_owned(),
            ..base.clone()
        };
        assert!(
            base_platforms(&other, &bytes)
                .unwrap_err()
                .contains("not the")
        );
        let one = index(&[("linux", "amd64", None, 'a')]);
        let pinned = Base {
            digest: format!("sha256:{}", sha256_hex(&one)),
            ..base
        };
        assert!(base_platforms(&pinned, &one).is_err());
    }

    #[test]
    fn only_static_elf_executables_for_the_platform_are_accepted() {
        assert!(check_elf(&elf(62, false), "amd64").is_ok());
        assert!(check_elf(&elf(183, false), "arm64").is_ok());
        assert!(
            check_elf(&elf(62, false), "arm64")
                .unwrap_err()
                .contains("not arm64")
        );
        assert!(
            check_elf(&elf(62, true), "amd64")
                .unwrap_err()
                .contains("dynamically linked")
        );
        assert!(check_elf(b"#!/bin/sh\n", "amd64").is_err());
        assert!(check_elf(&elf(62, false)[..40], "amd64").is_err());
        // A program header table offset near the end of the address space.
        let mut far = elf(62, false);
        far[32..40].copy_from_slice(&(u64::MAX - 1).to_le_bytes());
        assert!(check_elf(&far, "amd64").is_err());
    }

    #[test]
    fn rfc3339_dates() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(rfc3339(1_794_957_430), "2026-11-17T23:17:10Z");
        assert_eq!(rfc3339(4_107_542_399), "2100-02-28T23:59:59Z");
    }

    fn plan() -> Plan {
        let base = Base {
            name: "gcr.io/distroless/static-debian13:nonroot".to_owned(),
            repository: "gcr.io/distroless/static-debian13".to_owned(),
            digest: PIN.to_owned(),
        };
        let platforms = BTreeMap::from([
            ("amd64".to_owned(), format!("sha256:{}", "a".repeat(64))),
            ("arm64".to_owned(), format!("sha256:{}", "b".repeat(64))),
        ]);
        let (index_annotations, mut per) = annotations(
            "1.2.3",
            "Install things (rules, skills), safely",
            &"f".repeat(40),
            1_794_957_430,
            &base,
            &platforms,
        )
        .unwrap();
        Plan {
            version: "1.2.3".to_owned(),
            index_annotations,
            debian: "13".to_owned(),
            dirs: expected_dirs().unwrap(),
            platforms: ["amd64", "arm64"]
                .iter()
                .map(|a| PlatformPlan {
                    architecture: (*a).to_owned(),
                    base_digest: platforms[*a].clone(),
                    annotations: per.remove(*a).unwrap(),
                    files: BTreeMap::from([(format!("{CLI_DIR}/{BIN}"), "0".repeat(64))]),
                })
                .collect(),
        }
    }

    #[test]
    fn annotations_and_buildx_arguments() {
        let plan = plan();
        let key = |k: &str| format!("org.opencontainers.image.{k}");
        assert_eq!(
            plan.index_annotations[&key("created")],
            "2026-11-17T23:17:10Z"
        );
        assert_eq!(plan.index_annotations[&key("base.digest")], PIN);
        assert_eq!(plan.index_annotations[&key("licenses")], LICENSES);
        assert_eq!(
            plan.index_annotations[&key("documentation")],
            "https://github.com/jabbott-iii/Grimoire/blob/v1.2.3/docs/container.md"
        );
        assert_eq!(
            plan.platforms[1].annotations[&key("base.digest")],
            format!("sha256:{}", "b".repeat(64))
        );
        let args = buildx_args(&plan).unwrap();
        let lines: Vec<&str> = args.lines().collect();
        assert_eq!(lines[..2], ["--platform", "linux/amd64,linux/arm64"]);
        assert!(lines.contains(
            &"index:org.opencontainers.image.description=Install things (rules, skills), safely"
        ));
        assert!(lines.contains(&"manifest[linux/arm64]:org.opencontainers.image.base.digest=sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"));
        assert_eq!(
            lines.iter().filter(|l| **l == "--annotation").count(),
            3 * 11
        );
        assert!(
            annotations(
                "1",
                &"x".repeat(513),
                &"f".repeat(40),
                0,
                &Base {
                    name: String::new(),
                    repository: String::new(),
                    digest: PIN.to_owned()
                },
                &BTreeMap::new()
            )
            .is_err()
        );
        assert!(
            annotations(
                "1",
                "d",
                "abc",
                0,
                &Base {
                    name: String::new(),
                    repository: String::new(),
                    digest: PIN.to_owned()
                },
                &BTreeMap::new()
            )
            .is_err()
        );
    }

    /// The workspace licence is what the image declares.
    #[test]
    fn the_licence_annotation_matches_the_workspace() {
        let manifest: toml::Table = fs::read_to_string(root().join("Cargo.toml"))
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            manifest["workspace"]["package"]["license"].as_str(),
            Some(LICENSES)
        );
    }

    /// The image tools and the signing identities are pinned in four workflows; they must
    /// agree, and name the image this module checks. The release files are signed by
    /// sign.yml and the image by sign-image.yml, each with its own identity (SIGN-P6).
    #[test]
    fn the_image_pins_agree_across_workflows() {
        let value = |file: &str, key: &str| -> String {
            let text = fs::read_to_string(root().join(".github/workflows").join(file)).unwrap();
            let prefix = format!("{key}: ");
            let found: Vec<String> = text
                .lines()
                .filter_map(|l| l.trim().strip_prefix(&prefix))
                .map(|v| v.trim_matches('"').to_owned())
                .collect();
            assert_eq!(found.len(), 1, "{file}: {key} is set {} times", found.len());
            found[0].clone()
        };
        let image = ["IMAGE", "CRANE_VERSION", "CRANE_SHA256_X64"];
        let caller = ["COSIGN_ISSUER", "COSIGN_REPOSITORY", "COSIGN_REF"];
        let scans = [
            "GRYPE_VERSION",
            "GRYPE_SHA256",
            "SHELLCHECK_VERSION",
            "SHELLCHECK_SHA256",
        ];
        for key in image.iter().chain(&caller).chain(&scans) {
            assert_eq!(
                value("release.yml", key),
                value("security.yml", key),
                "{key} differs"
            );
        }
        for key in image.iter().chain(&caller) {
            assert_eq!(
                value("release.yml", key),
                value("sign-image.yml", key),
                "{key} differs"
            );
        }
        for key in caller {
            assert_eq!(
                value("release.yml", key),
                value("sign.yml", key),
                "{key} differs"
            );
        }
        assert_eq!(value("release.yml", "IMAGE"), IMAGE);
        // Signatures are made, and accepted, only from the two signing workflows called on
        // main (SIGN-P6): sign.yml for the release files, which release.yml's publish job
        // checks, and sign-image.yml for the image, which security.yml's scan checks.
        let identity = |file: &str| format!("{REPO_URL}/.github/workflows/{file}@refs/heads/main");
        assert_eq!(value("sign.yml", "COSIGN_IDENTITY"), identity("sign.yml"));
        assert_eq!(
            value("release.yml", "COSIGN_IDENTITY"),
            identity("sign.yml")
        );
        assert_eq!(
            value("sign-image.yml", "COSIGN_IDENTITY"),
            identity("sign-image.yml")
        );
        assert_eq!(
            value("security.yml", "COSIGN_IDENTITY"),
            identity("sign-image.yml")
        );
        assert_eq!(
            value("sign-image.yml", "COSIGN_REPOSITORY"),
            "jabbott-iii/Grimoire"
        );
        assert_eq!(value("sign-image.yml", "COSIGN_REF"), "refs/heads/main");
        assert_eq!(
            value("sign-image.yml", "SIGNER_WORKFLOW"),
            "jabbott-iii/Grimoire/.github/workflows/sign-image.yml@refs/heads/main"
        );
        for key in ["BUILDKIT_IMAGE", "SBOM_SCANNER_IMAGE"] {
            let pin = value("release.yml", key);
            let digest = pin.split_once('@').map(|(_, d)| d).unwrap_or_default();
            assert!(is_digest(digest), "{key} is not pinned by digest: {pin}");
        }
    }

    /// What's wrong with a `.grype.yaml`, read as the one block-style shape this
    /// repository allows. Grype ignores unknown keys silently (checked with v0.121.3):
    /// a misspelt `fail-on-severity` turns the gate off, and a misspelt key inside an
    /// ignore rule drops that criterion, so the rule matches more than it says.
    #[allow(
        clippy::too_many_lines,
        reason = "one line-by-line reader for the one shape .grype.yaml may take"
    )]
    fn grype_config_problems(text: &str) -> Vec<String> {
        #[derive(Default)]
        struct Rule {
            keys: BTreeMap<String, String>,
            package: BTreeMap<String, String>,
        }
        fn finish(rule: Option<Rule>, problems: &mut Vec<String>) {
            let Some(rule) = rule else { return };
            let value = |m: &BTreeMap<String, String>, k: &str| {
                m.get(k)
                    .map(|v| v.trim_matches('"').trim().to_owned())
                    .unwrap_or_default()
            };
            let id = value(&rule.keys, "vulnerability");
            if !(id.starts_with("GHSA-") || id.starts_with("CVE-")) {
                problems.push(format!(
                    "a rule's vulnerability `{id}` is not a GHSA or CVE id"
                ));
            }
            if value(&rule.package, "name").is_empty() {
                problems.push(format!("rule {id} names no package"));
            }
            if value(&rule.keys, "reason").is_empty() {
                problems.push(format!("rule {id} gives no reason"));
            }
        }
        let mut problems = Vec::new();
        let mut section = "";
        let mut rule: Option<Rule> = None;
        let mut in_package = false;
        let mut top = Vec::new();
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            if line != "ignore: []" && line.contains(['{', '[']) {
                problems.push(format!("flow style: `{line}`"));
                continue;
            }
            let indent = line.len() - line.trim_start().len();
            let (key, value) = trimmed
                .trim_start_matches("- ")
                .split_once(':')
                .map_or((trimmed, ""), |(k, v)| (k.trim(), v.trim()));
            match indent {
                0 => {
                    finish(rule.take(), &mut problems);
                    top.push(key.to_owned());
                    section = match key {
                        "db" => "db",
                        "ignore" => "ignore",
                        _ => "",
                    };
                    let expected = match key {
                        "fail-on-severity" => Some("medium"),
                        "only-fixed" | "check-for-app-update" => Some("false"),
                        "db" => Some(""),
                        "ignore" => None,
                        _ => {
                            problems.push(format!("unknown key `{key}`"));
                            None
                        }
                    };
                    if expected.is_some_and(|e| e != value) {
                        problems.push(format!("`{line}`"));
                    }
                }
                2 if section == "db" => {
                    if !matches!(
                        (key, value),
                        ("validate-age", "true") | ("max-allowed-built-age", "120h")
                    ) {
                        problems.push(format!("db: `{trimmed}`"));
                    }
                }
                2 if section == "ignore" && trimmed.starts_with("- ") => {
                    finish(rule.take(), &mut problems);
                    rule = Some(Rule::default());
                    in_package = false;
                    if let Some(r) = rule.as_mut() {
                        r.keys.insert(key.to_owned(), value.to_owned());
                    }
                }
                4 if rule.is_some() => {
                    in_package = key == "package";
                    if let Some(r) = rule.as_mut() {
                        r.keys.insert(key.to_owned(), value.to_owned());
                    }
                }
                6 if in_package => {
                    if let Some(r) = rule.as_mut() {
                        r.package.insert(key.to_owned(), value.to_owned());
                    }
                }
                _ => problems.push(format!("unexpected line `{line}`")),
            }
            if let Some(r) = &rule {
                let rule_keys = ["vulnerability", "package", "reason", "include-aliases"];
                if let Some(k) = r.keys.keys().find(|k| !rule_keys.contains(&k.as_str())) {
                    problems.push(format!("unknown rule key `{k}`"));
                }
                if let Some(k) = r
                    .package
                    .keys()
                    .find(|k| !["name", "version"].contains(&k.as_str()))
                {
                    problems.push(format!("unknown package key `{k}`"));
                }
            }
        }
        finish(rule, &mut problems);
        if top
            != [
                "fail-on-severity",
                "only-fixed",
                "check-for-app-update",
                "db",
                "ignore",
            ]
        {
            problems.push(format!("top-level keys {top:?}"));
        }
        problems.dedup();
        problems
    }

    #[test]
    fn the_grype_config_has_only_known_keys_and_complete_rules() {
        let text = fs::read_to_string(root().join(".grype.yaml")).unwrap();
        assert_eq!(grype_config_problems(&text), Vec::<String>::new());
        let with = |rules: &str| text.replace("ignore: []", &format!("ignore:\n{rules}"));
        let good = "  - vulnerability: GHSA-wcg3-cvx6-7396\n    package:\n      name: time\n    reason: \"No fix yet (#1).\"\n";
        assert_eq!(grype_config_problems(&with(good)), Vec::<String>::new());
        for bad in [
            text.replace("fail-on-severity", "fail-on-severty"),
            text.replace("fail-on-severity: medium", "fail-on-severity: high"),
            text.replace("only-fixed: false", "only-fixed: true"),
            text.replace("validate-age: true", "validate-agee: true"),
            text.replace(
                "ignore: []",
                "ignore: [{vulnerability: CVE-1, package: {name: x}}]",
            ),
            with(&good.replace("vulnerability:", "vulnerabilty:")),
            with(&good.replace("package:", "pakage:")),
            with(&good.replace("name: time", "nmae: time")),
            with(&good.replace("    reason: \"No fix yet (#1).\"\n", "")),
            with(&good.replace("reason: \"No fix yet (#1).\"", "reason: \"\"")),
            with(&good.replace("      name: time\n", "")),
            with(&format!("{good}    fix-state: unknown\n")),
        ] {
            assert!(!grype_config_problems(&bad).is_empty(), "accepted:\n{bad}");
        }
    }

    #[test]
    fn coverage_needs_rust_packages_in_every_binary() {
        let report = serde_json::json!({"components": [
            {"purl": "pkg:cargo/serde@1.0.0", "properties": [{"name": "syft:location:0:path", "value": "/usr/local/bin/grimoire"}]},
            {"purl": "pkg:deb/debian/tzdata@1", "properties": [{"name": "syft:location:0:path", "value": "/opt/grimoire/bin/x"}]},
        ]});
        let paths = || {
            [
                "/usr/local/bin/grimoire".to_owned(),
                "/opt/grimoire/bin/x".to_owned(),
            ]
            .into_iter()
        };
        let err = rust_coverage(&report, paths()).unwrap_err();
        assert!(
            err.contains("/opt/grimoire/bin/x") && !err.contains("/usr/local/bin/grimoire\""),
            "{err}"
        );
        assert!(rust_coverage(&report, ["/usr/local/bin/grimoire".to_owned()].into_iter()).is_ok());
    }

    type Entry<'a> = (&'a str, u32, u64, &'a [u8], tar::EntryType);

    fn header(path: &str, mode: u32, owner: u64, size: u64, kind: tar::EntryType) -> tar::Header {
        let mut h = tar::Header::new_ustar();
        // Raw names: the tar crate itself refuses `..`, which these tests need.
        h.as_ustar_mut().unwrap().name[..path.len()].copy_from_slice(path.as_bytes());
        h.set_mode(mode);
        h.set_uid(owner);
        h.set_gid(owner);
        h.set_size(size);
        h.set_entry_type(kind);
        h.set_cksum();
        h
    }

    /// An uncompressed tarball, ended by its two zero blocks.
    fn tar_raw(entries: &[Entry<'_>]) -> Vec<u8> {
        let mut tar = tar::Builder::new(Vec::new());
        for (path, mode, owner, data, kind) in entries {
            let h = header(path, *mode, *owner, data.len() as u64, *kind);
            tar.append(&h, *data).unwrap();
        }
        tar.into_inner().unwrap()
    }

    fn gzip(raw: &[u8]) -> Vec<u8> {
        use std::io::Write as _;
        let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gz.write_all(raw).unwrap();
        gz.finish().unwrap()
    }

    fn digest_of(bytes: &[u8]) -> String {
        format!("sha256:{}", sha256_hex(bytes))
    }

    /// A PAX extended header record, `<len> <key>=<value>\n`.
    fn pax_record(key: &str, value: &str) -> Vec<u8> {
        let body = format!(" {key}={value}\n");
        let mut len = body.len() + 1;
        while format!("{len}{body}").len() != len {
            len += 1;
        }
        format!("{len}{body}").into_bytes()
    }

    #[test]
    fn layers_are_read_strictly() {
        let dir = (
            "usr/local/bin/",
            0o755,
            0,
            &b""[..],
            tar::EntryType::Directory,
        );
        let bin = (
            "usr/local/bin/grimoire",
            0o755,
            0,
            &b"bin"[..],
            tar::EntryType::Regular,
        );
        let raw = tar_raw(&[dir, bin]);
        let found = layer_entries(&gzip(&raw), &digest_of(&raw)).unwrap();
        assert_eq!(found.files.len(), 1);
        assert_eq!(found.files[0].path, "usr/local/bin/grimoire");
        assert_eq!(
            found.dirs,
            [(
                "usr/local/bin".to_owned(),
                Owned {
                    mode: 0o755,
                    uid: 0,
                    gid: 0
                }
            )]
        );
        // The diff_id the runtime checks.
        assert!(
            layer_entries(&gzip(&raw), &digest_of(b"other"))
                .unwrap_err()
                .contains("diff_id")
        );
        // A second gzip member, which Go's reader would also unpack.
        let mut two = gzip(&raw);
        two.extend(gzip(&tar_raw(&[(
            "usr/local/bin/grimoire",
            0o755,
            0,
            b"evil",
            tar::EntryType::Regular,
        )])));
        assert!(
            layer_entries(&two, &digest_of(&raw))
                .unwrap_err()
                .contains("after its gzip member")
        );
    }

    #[test]
    fn layers_with_records_other_readers_see_differently_are_refused() {
        let raw = tar_raw(&[(
            "usr/local/bin/grimoire",
            0o755,
            0,
            b"bin",
            tar::EntryType::Regular,
        )]);
        // A directory whose size field hides a file header from the tar crate but not from Go.
        let mut hidden = header(
            "usr/local/bin/grimoire",
            0o755,
            0,
            4,
            tar::EntryType::Regular,
        )
        .as_bytes()
        .to_vec();
        hidden.extend(b"evil");
        hidden.resize(1024, 0);
        let mut tar = tar::Builder::new(Vec::new());
        tar.append(
            &header("usr/local/bin/", 0o755, 0, 1024, tar::EntryType::Directory),
            hidden.as_slice(),
        )
        .unwrap();
        let sized = tar.into_inner().unwrap();
        assert!(
            layer_entries(&gzip(&sized), &digest_of(&sized))
                .unwrap_err()
                .contains("has a size")
        );
        // A PAX record giving the binary file capabilities.
        let xattr = pax_record("SCHILY.xattr.security.capability", "x");
        let mut tar = tar::Builder::new(Vec::new());
        tar.append(
            &header("pax", 0o644, 0, xattr.len() as u64, tar::EntryType::XHeader),
            xattr.as_slice(),
        )
        .unwrap();
        tar.append(
            &header(
                "usr/local/bin/grimoire",
                0o755,
                0,
                3,
                tar::EntryType::Regular,
            ),
            &b"bin"[..],
        )
        .unwrap();
        let pax = tar.into_inner().unwrap();
        assert!(
            layer_entries(&gzip(&pax), &digest_of(&pax))
                .unwrap_err()
                .contains("extension records")
        );
        // A GNU long name that renames the entry.
        let name = b"usr/local/bin/grimoire\0";
        let mut tar = tar::Builder::new(Vec::new());
        tar.append(
            &header(
                "././@LongLink",
                0o644,
                0,
                name.len() as u64,
                tar::EntryType::GNULongName,
            ),
            &name[..],
        )
        .unwrap();
        tar.append(
            &header("decoy", 0o755, 0, 3, tar::EntryType::Regular),
            &b"bin"[..],
        )
        .unwrap();
        let long = tar.into_inner().unwrap();
        assert!(layer_entries(&gzip(&long), &digest_of(&long)).is_err());
        // Bytes after the end of the tarball.
        let mut trailing = raw.clone();
        trailing.extend(b"more");
        assert!(
            layer_entries(&gzip(&trailing), &digest_of(&trailing))
                .unwrap_err()
                .contains("after the end")
        );
        for bad in [
            tar_raw(&[(
                "usr/local/bin/.wh.grimoire",
                0o644,
                0,
                b"",
                tar::EntryType::Regular,
            )]),
            tar_raw(&[(
                "usr/local/bin/../../etc/passwd",
                0o644,
                0,
                b"x",
                tar::EntryType::Regular,
            )]),
            tar_raw(&[("usr/local/bin/sh", 0o777, 0, b"", tar::EntryType::Symlink)]),
        ] {
            assert!(layer_entries(&gzip(&bad), &digest_of(&bad)).is_err());
        }
    }

    #[test]
    fn coverage_needs_debian_and_its_packages() {
        let cdx = serde_json::json!({"components": [{"purl": "pkg:deb/debian/tzdata@1"}]});
        let report = serde_json::json!({"distro": {"name": "debian", "version": "13.7"}});
        assert!(debian_coverage(&report, &cdx, "13").is_ok());
        assert!(debian_coverage(&report, &cdx, "12").is_err());
        assert!(
            debian_coverage(
                &serde_json::json!({"distro": {"name": "", "version": ""}}),
                &cdx,
                "13"
            )
            .is_err()
        );
        assert!(debian_coverage(&serde_json::json!({}), &cdx, "13").is_err());
        assert!(debian_coverage(&report, &serde_json::json!({"components": []}), "13").is_err());
        assert_eq!(
            debian_major("gcr.io/distroless/static-debian13:nonroot").unwrap(),
            "13"
        );
        assert!(debian_major("gcr.io/distroless/static:nonroot").is_err());
    }

    /// A small OCI layout shaped like the one BuildKit writes, and the plan that matches
    /// it, so `check` can be tested against one change at a time.
    struct Fixture {
        plan: Plan,
        index_annotations: Annotations,
        archs: Vec<ArchParts>,
        extra_manifest: bool,
    }

    struct ArchParts {
        arch: &'static str,
        base_layer: Vec<u8>,
        base_config: Value,
        /// Raw tarballs of the image's own layers.
        layers: Vec<Vec<u8>>,
        /// The image config; `rootfs` is filled in from the layers unless already set.
        config: Value,
        annotations: Annotations,
        predicates: Vec<&'static str>,
        provenance: Vec<u8>,
        /// Make a base layer blob in the layout differ from the base's.
        swap_base_blob: bool,
    }

    const ENV: [&str; 2] = [
        "PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
        "SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt",
    ];

    /// Every regular file in `layers`, with its SHA-256: what `context` would plan.
    fn files_in(layers: &[Vec<u8>]) -> BTreeMap<String, String> {
        let mut files = BTreeMap::new();
        for raw in layers {
            let mut archive = tar::Archive::new(raw.as_slice());
            for entry in archive.entries().unwrap() {
                let mut entry = entry.unwrap();
                if entry.header().entry_type().is_file() {
                    let path = entry.path().unwrap().to_str().unwrap().to_owned();
                    let mut data = Vec::new();
                    entry.read_to_end(&mut data).unwrap();
                    files.insert(path, sha256_hex(&data));
                }
            }
        }
        files
    }

    fn fixture() -> Fixture {
        let plan = plan();
        let archs: Vec<ArchParts> = plan
            .platforms
            .iter()
            .map(|p| {
                let arch: &'static str = if p.architecture == "amd64" { "amd64" } else { "arm64" };
                let os_release = format!("ID=debian\nARCH={arch}\n").into_bytes();
                let base_layer = tar_raw(&[
                    ("etc/", 0o755, 0, b"", tar::EntryType::Directory),
                    ("etc/os-release", 0o644, 0, &os_release, tar::EntryType::Regular),
                ]);
                let root = |path| (path, 0o755, 0, &b""[..], tar::EntryType::Directory);
                let cli = format!("cli for {arch}").into_bytes();
                let asset = format!("asset for {arch}").into_bytes();
                let layers = vec![
                    tar_raw(&[root("usr/"), root("usr/local/"), root("usr/local/bin/"), ("usr/local/bin/grimoire", 0o755, 0, &cli, tar::EntryType::Regular)]),
                    tar_raw(&[root("opt/"), root("opt/grimoire/"), root("opt/grimoire/bin/"), ("opt/grimoire/bin/grimoire-tool-x", 0o755, 0, &asset, tar::EntryType::Regular)]),
                    tar_raw(&[("repo/", 0o755, 65532, b"", tar::EntryType::Directory)]),
                ];
                ArchParts {
                    arch,
                    base_config: serde_json::json!({
                        "architecture": arch, "os": "linux",
                        "config": {"Env": ENV, "User": "65532"},
                        "rootfs": {"type": "layers", "diff_ids": [digest_of(&base_layer)]},
                    }),
                    base_layer,
                    layers,
                    config: serde_json::json!({
                        "architecture": arch, "os": "linux", "created": "2026-11-17T23:17:10Z", "history": [],
                        "config": {
                            "User": USER, "Env": ENV, "Entrypoint": [format!("/{CLI_DIR}/{BIN}")],
                            "Cmd": ["--help"], "WorkingDir": WORKDIR, "ArgsEscaped": true,
                            "Labels": {"org.opencontainers.image.source": REPO_URL},
                        },
                    }),
                    annotations: p.annotations.clone(),
                    predicates: vec![SPDX, SLSA_PROVENANCE],
                    provenance: br#"{"predicate":{"buildDefinition":{}}}"#.to_vec(),
                    swap_base_blob: false,
                }
            })
            .collect();
        let mut plan = plan;
        for (a, p) in archs.iter().zip(plan.platforms.iter_mut()) {
            p.files = files_in(&a.layers);
        }
        Fixture {
            index_annotations: plan.index_annotations.clone(),
            plan,
            archs,
            extra_manifest: false,
        }
    }

    /// Writes the fixture and returns (out, layout, base) directories.
    #[allow(
        clippy::too_many_lines,
        reason = "writes every blob of a two-platform layout, in the order BuildKit does"
    )]
    fn write_fixture(dir: &Path, mut f: Fixture) -> (PathBuf, PathBuf, PathBuf) {
        let (out, layout, base) = (dir.join("out"), dir.join("layout"), dir.join("base"));
        for d in [&out, &base, &layout.join("blobs/sha256")] {
            fs::create_dir_all(d).unwrap();
        }
        let put = |bytes: &[u8], media: &str| -> Value {
            fs::write(layout.join("blobs/sha256").join(sha256_hex(bytes)), bytes).unwrap();
            serde_json::json!({"mediaType": media, "digest": digest_of(bytes), "size": bytes.len()})
        };
        let json = |v: &Value| serde_json::to_vec(v).unwrap();
        let mut entries = Vec::new();
        let mut attestations = Vec::new();
        for (a, p) in f.archs.iter_mut().zip(f.plan.platforms.iter_mut()) {
            let base_blob = gzip(&a.base_layer);
            let base_desc = serde_json::json!({"mediaType": OCI_LAYER_GZIP, "digest": digest_of(&base_blob), "size": base_blob.len()});
            if a.swap_base_blob {
                let mut other = base_blob.clone();
                other.push(0);
                fs::write(
                    layout.join("blobs/sha256").join(sha256_hex(&base_blob)),
                    &other,
                )
                .unwrap();
            } else {
                put(&base_blob, OCI_LAYER_GZIP);
            }
            let base_config = json(&a.base_config);
            fs::write(
                base.join(format!("{}.json", sha256_hex(&base_config))),
                &base_config,
            )
            .unwrap();
            let base_manifest = json(&serde_json::json!({
                "schemaVersion": 2, "mediaType": OCI_MANIFEST,
                "config": {"mediaType": "application/vnd.oci.image.config.v1+json", "digest": digest_of(&base_config), "size": base_config.len()},
                "layers": [base_desc],
            }));
            fs::write(
                base.join(format!("{}.json", sha256_hex(&base_manifest))),
                &base_manifest,
            )
            .unwrap();
            p.base_digest = digest_of(&base_manifest);
            p.annotations.insert(
                "org.opencontainers.image.base.digest".to_owned(),
                p.base_digest.clone(),
            );
            a.annotations.insert(
                "org.opencontainers.image.base.digest".to_owned(),
                p.base_digest.clone(),
            );
            let mut layer_descs = vec![base_desc];
            let mut diff_ids = vec![digest_of(&a.base_layer)];
            for raw in &a.layers {
                layer_descs.push(put(&gzip(raw), OCI_LAYER_GZIP));
                diff_ids.push(digest_of(raw));
            }
            if a.config.get("rootfs").is_none() {
                a.config["rootfs"] = serde_json::json!({"type": "layers", "diff_ids": diff_ids});
            }
            let config = put(&json(&a.config), "application/vnd.oci.image.config.v1+json");
            let manifest = json(&serde_json::json!({
                "schemaVersion": 2, "mediaType": OCI_MANIFEST, "config": config, "layers": layer_descs,
                "annotations": a.annotations,
            }));
            let mut desc = put(&manifest, OCI_MANIFEST);
            let att_layers: Vec<Value> = a
                .predicates
                .iter()
                .map(|t| {
                    let body = if *t == SLSA_PROVENANCE {
                        a.provenance.clone()
                    } else {
                        b"{}".to_vec()
                    };
                    let mut d = put(&body, "application/vnd.in-toto+json");
                    d["annotations"] = serde_json::json!({"in-toto.io/predicate-type": t});
                    d
                })
                .collect();
            let empty = put(b"{}", "application/vnd.oci.empty.v1+json");
            let att = json(&serde_json::json!({
                "schemaVersion": 2, "mediaType": OCI_MANIFEST, "config": empty, "layers": att_layers,
                "subject": desc.clone(),
            }));
            let mut att_desc = put(&att, OCI_MANIFEST);
            att_desc["platform"] = serde_json::json!({"os": "unknown", "architecture": "unknown"});
            att_desc["annotations"] = serde_json::json!({
                "vnd.docker.reference.type": "attestation-manifest",
                "vnd.docker.reference.digest": desc["digest"],
            });
            desc["platform"] = serde_json::json!({"os": "linux", "architecture": a.arch});
            entries.push(desc);
            attestations.push(att_desc);
        }
        entries.extend(attestations);
        if f.extra_manifest {
            let extra = entries[0].clone();
            entries.push(extra);
        }
        let index = json(&serde_json::json!({
            "schemaVersion": 2, "mediaType": OCI_INDEX, "manifests": entries, "annotations": f.index_annotations,
        }));
        let top = put(&index, OCI_INDEX);
        fs::write(
            layout.join("index.json"),
            json(&serde_json::json!({"schemaVersion": 2, "manifests": [top]})),
        )
        .unwrap();
        fs::write(
            layout.join("oci-layout"),
            br#"{"imageLayoutVersion":"1.0.0"}"#,
        )
        .unwrap();
        fs::write(out.join(PLAN), serde_json::to_vec_pretty(&f.plan).unwrap()).unwrap();
        (out, layout, base)
    }

    fn check_fixture(tweak: impl FnOnce(&mut Fixture)) -> Result<String, String> {
        let dir = tempfile::tempdir().unwrap();
        let mut f = fixture();
        tweak(&mut f);
        let (out, layout, base) = write_fixture(dir.path(), f);
        check(&out, &layout, &base)
    }

    #[test]
    fn check_passes_a_layout_shaped_like_buildkits() {
        let text = check_fixture(|_| {}).unwrap();
        assert!(text.starts_with("index-digest=sha256:"), "{text}");
        assert!(
            text.contains("\namd64-digest=sha256:") && text.contains("\narm64-digest=sha256:"),
            "{text}"
        );
    }

    fn refused(name: &str, tweak: impl FnOnce(&mut Fixture)) {
        assert!(check_fixture(tweak).is_err(), "check accepted: {name}");
    }

    #[test]
    fn check_refuses_changed_annotations_manifests_and_attestations() {
        refused("index annotation", |f| {
            f.index_annotations.insert("x".into(), "y".into());
        });
        refused("manifest annotation", |f| {
            f.archs[0]
                .annotations
                .remove("org.opencontainers.image.version");
        });
        refused("extra manifest", |f| f.extra_manifest = true);
        refused("no SBOM", |f| f.archs[0].predicates = vec![SLSA_PROVENANCE]);
        refused("event payload", |f| {
            f.archs[1].provenance = br#"{"github_event_payload":{}}"#.to_vec();
        });
    }

    #[test]
    fn check_refuses_a_changed_config() {
        refused("Env", |f| {
            f.archs[0].config["config"]["Env"] = serde_json::json!(["PATH=/opt/grimoire/bin"]);
        });
        refused("OnBuild", |f| {
            f.archs[0].config["config"]["OnBuild"] = serde_json::json!(["ADD x /"]);
        });
        refused("Healthcheck", |f| {
            f.archs[1].config["config"]["Healthcheck"] = serde_json::json!({});
        });
        refused("top-level key", |f| {
            f.archs[0].config["variant"] = Value::from("v8");
        });
        refused("User", |f| {
            f.archs[0].config["config"]["User"] = Value::from("0");
        });
        refused("diff_ids", |f| {
            f.archs[0].config["rootfs"] =
                serde_json::json!({"type": "layers", "diff_ids": [PIN, PIN, PIN, PIN]});
        });
        refused("base config", |f| {
            f.archs[0].base_config["rootfs"]["diff_ids"] = serde_json::json!([PIN]);
        });
    }

    #[test]
    fn check_refuses_changed_layers() {
        refused("base layer bytes", |f| f.archs[0].swap_base_blob = true);
        refused("unplanned file", |f| {
            f.archs[0].layers.push(tar_raw(&[(
                "etc/passwd",
                0o755,
                0,
                b"x",
                tar::EntryType::Regular,
            )]));
        });
        refused("unplanned directory", |f| {
            f.archs[0].layers.push(tar_raw(&[(
                "etc/",
                0o777,
                65532,
                b"",
                tar::EntryType::Directory,
            )]));
        });
        refused("directory owner", |f| {
            f.archs[1].layers[2] = tar_raw(&[("repo/", 0o755, 0, b"", tar::EntryType::Directory)]);
        });
        refused("file mode", |f| {
            f.archs[0].layers[0] = tar_raw(&[(
                "usr/local/bin/grimoire",
                0o4755,
                0,
                b"cli for amd64",
                tar::EntryType::Regular,
            )]);
        });
    }

    #[test]
    fn check_refuses_other_binaries_and_missing_ones() {
        let dir = tempfile::tempdir().unwrap();
        let (out, layout, base) = write_fixture(dir.path(), fixture());
        let mut plan = load_plan(&out).unwrap();
        plan.platforms[0]
            .files
            .insert(format!("{CLI_DIR}/{BIN}"), sha256_hex(b"other"));
        fs::write(out.join(PLAN), serde_json::to_vec(&plan).unwrap()).unwrap();
        assert!(
            check(&out, &layout, &base)
                .unwrap_err()
                .contains("not the archived binary")
        );
        let mut plan = load_plan(&out).unwrap();
        plan.platforms[0]
            .files
            .insert(format!("{CLI_DIR}/{BIN}"), sha256_hex(b"cli for amd64"));
        plan.platforms[1]
            .files
            .insert(format!("{ASSET_DIR}/missing"), "0".repeat(64));
        fs::write(out.join(PLAN), serde_json::to_vec(&plan).unwrap()).unwrap();
        assert!(check(&out, &layout, &base).unwrap_err().contains("lacks"));
    }
}
