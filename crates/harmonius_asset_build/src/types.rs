//! Shared integration types (`CookRequest`, `CookedManifest`, …) per
//! `docs/design/integration/asset-pipeline-build-deploy.md`.

use rkyv::{Archive, Deserialize, Serialize};

/// Stable asset identifier (see asset-pipeline design).
#[derive(
    Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(transparent)]
pub struct AssetId(pub u64);

/// Runtime asset classification for cooked entries.
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AssetKind {
    Texture = 0,
    Shader = 1,
    Mesh = 2,
    Audio = 3,
}

/// Build/deploy target platform (see `build-deploy` design).
#[derive(Archive, Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TargetPlatform {
    Windows = 0,
    MacOS = 1,
    IOS = 2,
    Linux = 3,
    Android = 4,
    ConsoleA = 5,
    ConsoleB = 6,
}

/// Build-time target selection (subset used by integration tests).
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct BuildConfig {
    /// Primary target for this build request.
    pub target: TargetPlatform,
}

/// Per-platform processing parameters (texture codecs, etc.).
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct PlatformProfile {
    /// Target used to derive defaults when fields are unset.
    pub target: TargetPlatform,
    /// Explicit texture codec override for tests (optional semantics).
    pub texture_override: Option<TargetPlatform>,
}

/// Request from the build system to cook a batch of assets.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct CookRequest {
    pub config: BuildConfig,
    pub profile: PlatformProfile,
    pub asset_ids: Vec<AssetId>,
}

/// One row in a [`CookedManifest`].
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct CookedEntry {
    pub asset_id: AssetId,
    /// BLAKE3 hash of baked bytes in the CAS.
    pub cas_key: [u8; 32],
    pub size_bytes: u64,
    pub kind: AssetKind,
}

/// Output manifest from a cook pass.
#[derive(Archive, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[repr(C, align(16))]
pub struct CookedManifest {
    pub platform: TargetPlatform,
    pub entries: Vec<CookedEntry>,
    /// Merkle root over `cas_key` leaves (binary tree, odd leaf promoted).
    pub blake3_root: [u8; 32],
}
