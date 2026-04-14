//! Packaged build manifest slice for shipped provenance metadata.

use super::provenance::AssetId;

/// Minimal manifest listing AI-tagged assets included in a build export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PackagedBuildManifest {
    /// Asset ids whose provenance must ship with the build.
    pub provenance_entries: Vec<AssetId>,
}

impl PackagedBuildManifest {
    /// Builds a manifest covering exactly the provided ids (stable sorted).
    pub fn from_tagged_assets(mut assets: Vec<AssetId>) -> Self {
        assets.sort_by_key(|a| a.0);
        Self {
            provenance_entries: assets,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::provenance::AssetId;
    use super::*;

    /// TC-15.7.8.1 — manifest lists every tagged asset slated for export.
    #[test]
    fn tc_15_7_8_1_packaged_provenance_manifest() {
        let ids = vec![AssetId(10), AssetId(2), AssetId(7)];
        let manifest = PackagedBuildManifest::from_tagged_assets(ids);
        assert_eq!(manifest.provenance_entries.len(), 3);
        assert_eq!(
            manifest.provenance_entries,
            vec![AssetId(2), AssetId(7), AssetId(10)]
        );
    }
}
