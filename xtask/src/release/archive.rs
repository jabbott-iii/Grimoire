//! Release archives: flat `.tar.gz` (Linux, macOS) and `.zip` (Windows), with
//! sorted members, a fixed 1980-01-01 timestamp and owner 0, so the same binaries
//! always give the same archive bytes (ADR-0009).

use std::io::Read as _;

use flate2::Compression;
use flate2::read::GzDecoder;

use super::targets::Format;
use super::zip;

/// One file in an archive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct File {
    pub(crate) name: String,
    pub(crate) mode: u32,
    pub(crate) data: Vec<u8>,
}

/// 1980-01-01T00:00:00Z, the earliest time a zip can hold; tarballs use it too.
pub(crate) const EPOCH: u64 = 315_532_800;
/// The most [`read`] unpacks from a tarball in all (as for zips).
const MAX_TOTAL: u64 = zip::MAX_TOTAL;
/// The most entries [`read`] accepts from a tarball.
const MAX_ENTRIES: usize = 4096;

/// Writes `files` (sorted by name) as an archive of `format`.
pub(crate) fn write(format: Format, files: &[File]) -> Result<Vec<u8>, String> {
    let mut files = files.to_vec();
    files.sort_by(|a, b| a.name.cmp(&b.name));
    if files.windows(2).any(|w| w[0].name == w[1].name) {
        return Err("duplicate archive member".to_owned());
    }
    match format {
        Format::Zip => zip::write(
            &files
                .into_iter()
                .map(|f| zip::Member {
                    name: f.name,
                    mode: f.mode,
                    data: f.data,
                })
                .collect::<Vec<_>>(),
        ),
        Format::TarGz => {
            let gz = flate2::GzBuilder::new()
                .mtime(0)
                .write(Vec::new(), Compression::best());
            let mut tar = tar::Builder::new(gz);
            tar.mode(tar::HeaderMode::Deterministic);
            for f in &files {
                if f.name.contains('/') || !f.name.is_ascii() || f.name.is_empty() {
                    return Err(format!("archives are flat; bad member name {:?}", f.name));
                }
                let mut header = tar::Header::new_ustar();
                header
                    .set_path(&f.name)
                    .map_err(|e| format!("{}: {e}", f.name))?;
                header.set_size(f.data.len() as u64);
                header.set_mode(f.mode & 0o7777);
                header.set_mtime(EPOCH);
                header.set_uid(0);
                header.set_gid(0);
                header.set_entry_type(tar::EntryType::Regular);
                header.set_cksum();
                tar.append(&header, f.data.as_slice())
                    .map_err(|e| format!("cannot write {}: {e}", f.name))?;
            }
            tar.into_inner()
                .and_then(flate2::write::GzEncoder::finish)
                .map_err(|e| format!("cannot finish the tarball: {e}"))
        }
    }
}

/// Reads every regular file of an archive of `format`. Anything other than a regular
/// file or a directory is refused (release archives never contain links), and so are
/// duplicate names and more than [`MAX_TOTAL`] bytes in all.
pub(crate) fn read(format: Format, bytes: &[u8]) -> Result<Vec<File>, String> {
    let files = read_any(format, bytes)?;
    let mut names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|w| w[0] == w[1]) {
        return Err("duplicate archive member".to_owned());
    }
    Ok(files)
}

/// Reads an archive that `cargo xtask release build` made, and refuses it unless
/// [`write`] gives back exactly the same bytes. Extra gzip members, trailing data,
/// nested or duplicate names and odd headers can then never make one tool see a
/// different binary from another.
pub(crate) fn read_canonical(format: Format, bytes: &[u8]) -> Result<Vec<File>, String> {
    let files = read(format, bytes)?;
    if write(format, &files)? != bytes {
        return Err("not an archive as `cargo xtask release build` writes it".to_owned());
    }
    Ok(files)
}

fn read_any(format: Format, bytes: &[u8]) -> Result<Vec<File>, String> {
    match format {
        Format::Zip => Ok(zip::read(bytes)?
            .into_iter()
            .filter(|m| !m.name.ends_with('/'))
            .map(|m| File {
                name: m.name,
                mode: m.mode,
                data: m.data,
            })
            .collect()),
        Format::TarGz => {
            let mut archive = tar::Archive::new(GzDecoder::new(bytes));
            let mut files = Vec::new();
            let mut total: u64 = 0;
            for entry in archive
                .entries()
                .map_err(|e| format!("not a tarball: {e}"))?
            {
                let mut entry = entry.map_err(|e| format!("corrupt tarball: {e}"))?;
                let kind = entry.header().entry_type();
                if kind.is_dir() {
                    continue;
                }
                if !kind.is_file() {
                    return Err("tarball contains something other than files".to_owned());
                }
                let name = entry
                    .path()
                    .map_err(|e| format!("bad tarball path: {e}"))?
                    .to_str()
                    .ok_or("non-UTF-8 tarball path")?
                    .to_owned();
                let mode = entry.header().mode().map_err(|e| format!("{name}: {e}"))?;
                total = total.saturating_add(entry.size());
                if total > MAX_TOTAL || files.len() >= MAX_ENTRIES {
                    return Err("the tarball is too large".to_owned());
                }
                let mut data = Vec::new();
                entry
                    .by_ref()
                    .take(MAX_TOTAL)
                    .read_to_end(&mut data)
                    .map_err(|e| format!("{name}: {e}"))?;
                files.push(File { name, mode, data });
            }
            Ok(files)
        }
    }
}

/// The contents of the member called `name`.
pub(crate) fn member<'a>(files: &'a [File], name: &str) -> Result<&'a File, String> {
    files
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| format!("the archive has no `{name}`"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<File> {
        vec![
            File {
                name: "grimoire".to_owned(),
                mode: 0o755,
                data: b"\x7fELF fake binary".to_vec(),
            },
            File {
                name: "LICENSE-MIT".to_owned(),
                mode: 0o644,
                data: b"MIT".to_vec(),
            },
        ]
    }

    #[test]
    fn both_formats_round_trip_sorted_and_deterministic() {
        for format in [Format::TarGz, Format::Zip] {
            let bytes = write(format, &sample()).unwrap();
            assert_eq!(write(format, &sample()).unwrap(), bytes, "{format:?}");
            let files = read(format, &bytes).unwrap();
            let names: Vec<&str> = files.iter().map(|f| f.name.as_str()).collect();
            assert_eq!(names, ["LICENSE-MIT", "grimoire"], "{format:?}");
            let bin = member(&files, "grimoire").unwrap();
            assert_eq!(bin.mode, 0o755);
            assert_eq!(bin.data, b"\x7fELF fake binary");
        }
    }

    #[test]
    fn only_canonical_archives_are_accepted_where_it_matters() {
        for format in [Format::TarGz, Format::Zip] {
            let bytes = write(format, &sample()).unwrap();
            assert!(read_canonical(format, &bytes).is_ok(), "{format:?}");
            let mut extra = bytes.clone();
            if format == Format::TarGz {
                // A second gzip member that GNU tar would also extract.
                extra.extend_from_slice(&write(format, &sample()).unwrap());
            } else {
                extra.extend_from_slice(b"trailing");
            }
            assert!(read_canonical(format, &extra).is_err(), "{format:?}");
        }
    }

    #[test]
    fn tarballs_are_flat_and_unique() {
        let mut nested = sample();
        nested[0].name = "dir/grimoire".to_owned();
        assert!(write(Format::TarGz, &nested).is_err());
        let mut dup = sample();
        dup[1].name = "grimoire".to_owned();
        assert!(write(Format::Zip, &dup).is_err());
    }
}
