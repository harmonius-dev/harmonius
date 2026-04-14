//! Deterministic fake cook used by integration tests (IR-5.1.1 / IR-5.1.2).

use crate::merkle::cooked_manifest_merkle_root;
use crate::platform::select_texture_format;
use crate::types::{
    AssetId, AssetKind, BuildConfig, CookRequest, CookedEntry, CookedManifest, PlatformProfile,
    TargetPlatform,
};

/// Cook failures (caller-visible).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CookError {
    /// Asset source missing.
    UnknownAsset(AssetId),
}

/// Lightweight stats for assertions.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CookStats {
    pub processed: usize,
}

/// Provides deterministic source bytes per asset id.
pub trait FakeAssetSource {
    /// Returns kind + source payload for `id`.
    fn describe(&self, id: AssetId) -> Option<(AssetKind, Vec<u8>)>;
}

/// Cooks `request` into a manifest using `source` for inputs.
pub fn cook_assets(
    request: &CookRequest,
    source: &impl FakeAssetSource,
) -> Result<(CookedManifest, CookStats), CookError> {
    let mut entries = Vec::with_capacity(request.asset_ids.len());
    let mut stats = CookStats::default();
    for &id in &request.asset_ids {
        let (kind, src) = source.describe(id).ok_or(CookError::UnknownAsset(id))?;
        let baked = bake_asset(
            kind,
            &src,
            &request.profile,
            &BuildConfig {
                target: request.config.target,
            },
        );
        let cas_key = *blake3::hash(&baked).as_bytes();
        entries.push(CookedEntry {
            asset_id: id,
            cas_key,
            size_bytes: baked.len() as u64,
            kind,
        });
        stats.processed += 1;
    }
    let leaves: Vec<[u8; 32]> = entries.iter().map(|e| e.cas_key).collect();
    let blake3_root = cooked_manifest_merkle_root(&leaves);
    Ok((
        CookedManifest {
            platform: request.config.target,
            entries,
            blake3_root,
        },
        stats,
    ))
}

fn bake_asset(
    kind: AssetKind,
    src: &[u8],
    profile: &PlatformProfile,
    config: &BuildConfig,
) -> Vec<u8> {
    match kind {
        AssetKind::Texture => {
            let codec = select_texture_format(profile);
            let mut out = Vec::new();
            out.extend_from_slice(b"TEX:");
            out.push(codec as u8);
            out.push(config.target as u8);
            out.extend_from_slice(src);
            out
        }
        AssetKind::Shader => {
            let mut out = Vec::new();
            out.extend_from_slice(b"SHD:");
            out.push(config.target as u8);
            out.extend_from_slice(src);
            out
        }
        AssetKind::Mesh | AssetKind::Audio => {
            let mut out = Vec::new();
            out.extend_from_slice(b"GEN:");
            out.push(kind as u8);
            out.extend_from_slice(src);
            out
        }
    }
}

/// Integration-test hook to rebuild baked bytes with an explicit target platform.
#[doc(hidden)]
pub fn re_bake_for_test(kind: AssetKind, src: &[u8], platform: TargetPlatform) -> Vec<u8> {
    let profile = PlatformProfile {
        target: platform,
        texture_override: None,
    };
    bake_asset(kind, src, &profile, &BuildConfig { target: platform })
}
