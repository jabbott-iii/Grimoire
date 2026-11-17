//! The packed catalog embedded in the `grimoire` binary (ADR-0001).
//!
//! A pack is a deterministic, binary-safe list of files: the magic line, then for
//! each file, sorted by path, a little-endian `u32` path length, the path (ASCII,
//! `/`-separated, relative to the catalog root), a little-endian `u32` content
//! length and the content. It holds only what installation needs: each asset's
//! `asset.toml`, `README.md` and `CHANGELOG.md`, the files its outputs install,
//! and every bundle.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use agent_grimoire_model::{Diagnostic, RelPath, has_errors};

use crate::fs::{MAX_FILE_BYTES, read_bytes, walk};

/// Identifies a pack and its format version.
const MAGIC: &[u8] = b"grimoire catalog pack v1\n";
/// Most files a pack may hold.
const MAX_FILES: usize = 100_000;
/// Longest path inside a pack: a kind directory, an asset id and a relative path.
const MAX_PATH: usize = 300;
/// Files every asset ships with besides its installed outputs.
const ASSET_FILES: [&str; 3] = ["asset.toml", "README.md", "CHANGELOG.md"];

/// Validates the catalog at `root` and packs it.
///
/// # Errors
///
/// Returns every diagnostic when validation finds an error, so an invalid
/// catalog can never be embedded.
pub fn pack(root: &Path) -> Result<Vec<u8>, Vec<Diagnostic>> {
    let validation = crate::load(root);
    if validation.has_errors() {
        return Err(validation.diagnostics);
    }
    let mut problems = Vec::new();
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    for asset in validation.catalog.assets().values() {
        let dir = os_path(root, &asset.dir);
        let tree = walk(&dir, &asset.dir, &mut problems);
        let mut wanted: BTreeSet<RelPath> = ASSET_FILES
            .iter()
            .filter_map(|name| RelPath::new(*name).ok())
            .collect();
        for outputs in asset.manifest.clients.values() {
            for src in outputs.outputs.iter().filter_map(|o| o.src.as_ref()) {
                wanted.extend(tree.files_under(src).cloned());
            }
        }
        for rel in wanted {
            let shown = format!("{}/{rel}", asset.dir);
            match read_bytes(&os_path(&dir, rel.as_str())) {
                Ok(bytes) => {
                    files.insert(shown, bytes);
                }
                Err(e) => {
                    problems.push(
                        Diagnostic::error("pack/read", format!("`{rel}` {e}")).in_file(shown),
                    );
                }
            }
        }
    }
    for bundle in validation.catalog.bundles().values() {
        match read_bytes(&os_path(root, &bundle.file)) {
            Ok(bytes) => {
                files.insert(bundle.file.clone(), bytes);
            }
            Err(e) => problems.push(Diagnostic::error("pack/read", e).in_file(bundle.file.clone())),
        }
    }
    if has_errors(&problems) {
        return Err(problems);
    }
    encode(&files).map_err(|e| vec![Diagnostic::error("pack/limits", e)])
}

fn os_path(root: &Path, rel: &str) -> PathBuf {
    rel.split('/')
        .fold(root.to_path_buf(), |path, part| path.join(part))
}

fn encode(files: &BTreeMap<String, Vec<u8>>) -> Result<Vec<u8>, String> {
    if files.len() > MAX_FILES {
        return Err(format!("a pack may hold at most {MAX_FILES} files"));
    }
    let mut out = MAGIC.to_vec();
    for (path, data) in files {
        let path_len = u32::try_from(path.len()).map_err(|_| format!("`{path}` is too long"))?;
        let data_len = u32::try_from(data.len()).map_err(|_| format!("`{path}` is too large"))?;
        out.extend_from_slice(&path_len.to_le_bytes());
        out.extend_from_slice(path.as_bytes());
        out.extend_from_slice(&data_len.to_le_bytes());
        out.extend_from_slice(data);
    }
    Ok(out)
}

/// Reads a pack back into `path → content`.
pub(crate) fn decode(bytes: &[u8]) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut rest = bytes
        .strip_prefix(MAGIC)
        .ok_or("not a Grimoire catalog pack (bad magic or unsupported version)")?;
    let mut files: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    while !rest.is_empty() {
        if files.len() >= MAX_FILES {
            return Err(format!("holds more than {MAX_FILES} files"));
        }
        let path = take_chunk(&mut rest, MAX_PATH)?;
        let path = std::str::from_utf8(path)
            .ok()
            .filter(|p| RelPath::new(*p).is_ok())
            .ok_or("holds a path that is not a portable relative path")?
            .to_owned();
        let limit = usize::try_from(MAX_FILE_BYTES).unwrap_or(usize::MAX);
        let data = take_chunk(&mut rest, limit)?.to_vec();
        if files
            .last_key_value()
            .is_some_and(|(last, _)| *last >= path)
        {
            return Err(format!("is not sorted at `{path}`"));
        }
        files.insert(path, data);
    }
    Ok(files)
}

/// Splits one length-prefixed chunk off the front of `rest`.
fn take_chunk<'a>(rest: &mut &'a [u8], limit: usize) -> Result<&'a [u8], String> {
    let (len, tail) = rest.split_first_chunk::<4>().ok_or("is truncated")?;
    let len = usize::try_from(u32::from_le_bytes(*len)).map_err(|_| "is malformed")?;
    if len > limit {
        return Err(format!(
            "holds a {len}-byte chunk, over the {limit}-byte limit"
        ));
    }
    if tail.len() < len {
        return Err("is truncated".to_owned());
    }
    let (chunk, tail) = tail.split_at(len);
    *rest = tail;
    Ok(chunk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_rejections() {
        let mut files = BTreeMap::new();
        files.insert("a/b.txt".to_owned(), b"hello".to_vec());
        files.insert("bundles/x.toml".to_owned(), vec![0, 159, 255]);
        let bytes = encode(&files).unwrap();
        assert_eq!(decode(&bytes).unwrap(), files);
        assert_eq!(decode(MAGIC).unwrap(), BTreeMap::new());

        assert!(decode(b"nope").is_err());
        assert!(
            decode(&bytes[..bytes.len() - 1]).is_err(),
            "truncated content"
        );
        let mut unsafe_path = MAGIC.to_vec();
        unsafe_path.extend_from_slice(&5u32.to_le_bytes());
        unsafe_path.extend_from_slice(b"../x\0");
        unsafe_path.extend_from_slice(&0u32.to_le_bytes());
        assert!(decode(&unsafe_path).is_err());

        // Out of order: encode two entries by hand in reverse.
        let mut unsorted = MAGIC.to_vec();
        for path in ["b", "a"] {
            unsorted.extend_from_slice(&1u32.to_le_bytes());
            unsorted.extend_from_slice(path.as_bytes());
            unsorted.extend_from_slice(&0u32.to_le_bytes());
        }
        assert!(decode(&unsorted).is_err());
    }
}
