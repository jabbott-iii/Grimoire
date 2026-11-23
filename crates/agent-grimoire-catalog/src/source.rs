//! Where catalog data comes from.
//!
//! [ADR-0001](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0001-catalog-delivery.md)
//! chose to embed the catalog in the binary for v1, behind this abstraction,
//! so that a remote catalog can be added later without changing the lockfile
//! format.
//!
//! A [`Source`] is the in-memory index the resolver and installer work from: every
//! asset's manifest, its manifest digest and the files it installs, plus every
//! bundle. v1 builds it only from the pack embedded in the binary. A later remote
//! source would build the same index, so nothing downstream changes.

use std::collections::BTreeMap;
use std::fmt;

use agent_grimoire_model::{
    AssetId, AssetManifest, Bundle, Kind, RelPath, Sha256Digest, SourceId, has_errors, validate,
};

use crate::pack;

/// The catalog packed by `cargo xtask pack-catalog` and embedded in this build.
const BUILTIN: &[u8] = include_bytes!("../catalog.pack");

/// A source that failed to load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceError(String);

impl fmt::Display for SourceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for SourceError {}

/// A loaded catalog source.
#[derive(Debug)]
pub struct Source {
    id: SourceId,
    assets: BTreeMap<AssetId, SourceAsset>,
    bundles: BTreeMap<AssetId, Bundle>,
}

/// One asset in a source.
#[derive(Debug)]
pub struct SourceAsset {
    /// The parsed manifest.
    pub manifest: AssetManifest,
    /// Digest of the manifest file's bytes.
    pub digest: Sha256Digest,
    files: BTreeMap<RelPath, Vec<u8>>,
}

impl SourceAsset {
    /// A file of the asset, by path relative to the asset directory.
    #[must_use]
    pub fn file(&self, path: &RelPath) -> Option<&[u8]> {
        self.files.get(path).map(Vec::as_slice)
    }

    /// The files at or below `prefix`, which may name a file or a directory, sorted by path.
    pub fn files_under<'a>(
        &'a self,
        prefix: &'a RelPath,
    ) -> impl Iterator<Item = (&'a RelPath, &'a [u8])> + 'a {
        self.files
            .iter()
            .filter(move |(path, _)| path.starts_with(prefix))
            .map(|(path, data)| (path, data.as_slice()))
    }

    /// The asset's `README.md`, when it is valid UTF-8.
    #[must_use]
    pub fn readme(&self) -> Option<&str> {
        self.text("README.md")
    }

    /// The asset's `CHANGELOG.md`, when it is valid UTF-8.
    #[must_use]
    pub fn changelog(&self) -> Option<&str> {
        self.text("CHANGELOG.md")
    }

    fn text(&self, name: &str) -> Option<&str> {
        let path = RelPath::new(name).ok()?;
        std::str::from_utf8(self.file(&path)?).ok()
    }
}

impl Source {
    /// The catalog embedded in this build of Grimoire.
    ///
    /// # Errors
    ///
    /// Fails only if the embedded pack is corrupt, which CI prevents.
    pub fn builtin() -> Result<Self, SourceError> {
        Self::from_pack(SourceId::Builtin, BUILTIN)
    }

    /// Loads a pack produced by [`crate::pack`].
    ///
    /// Every manifest and bundle is parsed strictly and validated again, so a
    /// damaged pack fails here rather than at install time.
    ///
    /// # Errors
    ///
    /// Fails when the pack is malformed or holds an invalid document.
    pub fn from_pack(id: SourceId, bytes: &[u8]) -> Result<Self, SourceError> {
        let err = |message: String| SourceError(format!("catalog pack {message}"));
        let files = pack::decode(bytes).map_err(err)?;
        let mut grouped: BTreeMap<(Kind, AssetId), BTreeMap<RelPath, Vec<u8>>> = BTreeMap::new();
        let mut bundles = BTreeMap::new();
        for (path, data) in files {
            let mut parts = path.splitn(3, '/');
            let (first, second, rest) = (parts.next(), parts.next(), parts.next());
            if let (Some("bundles"), Some(name), None) = (first, second, rest) {
                let text = std::str::from_utf8(&data)
                    .map_err(|_| err(format!("holds a non-UTF-8 bundle `{path}`")))?;
                let bundle = Bundle::from_toml(text)
                    .map_err(|e| err(format!("holds an invalid bundle `{path}`: {e}")))?;
                if has_errors(&validate::bundle(&bundle))
                    || name.strip_suffix(".toml") != Some(bundle.id.as_str())
                {
                    return Err(err(format!("holds an invalid bundle `{path}`")));
                }
                bundles.insert(bundle.id.clone(), bundle);
                continue;
            }
            let located = first
                .and_then(Kind::from_catalog_dir)
                .zip(second.and_then(|s| AssetId::new(s).ok()))
                .zip(rest.and_then(|r| RelPath::new(r).ok()));
            let Some(((kind, asset), rel)) = located else {
                return Err(err(format!("holds an unexpected path `{path}`")));
            };
            grouped.entry((kind, asset)).or_default().insert(rel, data);
        }
        let manifest_path = RelPath::new("asset.toml").map_err(|e| err(e.to_string()))?;
        let mut assets = BTreeMap::new();
        for ((kind, id), files) in grouped {
            let raw = files
                .get(&manifest_path)
                .ok_or_else(|| err(format!("has no manifest for `{id}`")))?;
            let text = std::str::from_utf8(raw)
                .map_err(|_| err(format!("has a non-UTF-8 manifest for `{id}`")))?;
            let manifest = AssetManifest::from_toml(text)
                .map_err(|e| err(format!("has an invalid manifest for `{id}`: {e}")))?;
            if manifest.id != id
                || manifest.kind != kind
                || has_errors(&validate::manifest(&manifest))
            {
                return Err(err(format!("has an invalid manifest for `{id}`")));
            }
            let digest = Sha256Digest::of(raw);
            if assets.contains_key(&id) || bundles.contains_key(&id) {
                return Err(err(format!("defines `{id}` twice")));
            }
            assets.insert(
                id,
                SourceAsset {
                    manifest,
                    digest,
                    files,
                },
            );
        }
        Ok(Self {
            id,
            assets,
            bundles,
        })
    }

    /// The identity recorded in the lock as `source`.
    #[must_use]
    pub fn id(&self) -> SourceId {
        self.id
    }

    /// Every asset, by id.
    #[must_use]
    pub fn assets(&self) -> &BTreeMap<AssetId, SourceAsset> {
        &self.assets
    }

    /// One asset.
    #[must_use]
    pub fn asset(&self, id: &AssetId) -> Option<&SourceAsset> {
        self.assets.get(id)
    }

    /// Every bundle, by id.
    #[must_use]
    pub fn bundles(&self) -> &BTreeMap<AssetId, Bundle> {
        &self.bundles
    }

    /// One bundle.
    #[must_use]
    pub fn bundle(&self, id: &AssetId) -> Option<&Bundle> {
        self.bundles.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_embedded_pack_loads() {
        Source::builtin().unwrap();
    }
}
