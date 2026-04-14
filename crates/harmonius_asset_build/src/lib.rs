//! Asset pipeline ↔ build/deploy integration primitives (IR-5.1.x).
#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod bundle;
mod cas;
mod channels;
mod cook;
mod delta;
mod incremental;
mod merkle;
mod platform;
mod shader;
mod types;

pub use bundle::{build_bundles, BundleEntry, BundleSet};
pub use cas::{
    parse_cas_key, CasError, CasStore, ConcurrentCas, LruCas, MalformedCasKeyError, MemoryCas,
};
pub use channels::{build_requests, bundle_completions, cook_completions, ChannelSizes};
pub use cook::{cook_assets, re_bake_for_test, CookError, CookStats, FakeAssetSource};
pub use delta::{
    build_delta_patch, detect_tampered_root, full_v2_wire_size, merkle_over_entries, DeltaError,
    DeltaPatch,
};
pub use incremental::IncrementalCache;
pub use merkle::{cooked_manifest_merkle_root, verify_cooked_manifest_root};
pub use platform::{select_texture_format, TextureCompression};
pub use shader::{ShaderArtifactKind, ShaderCompileCache, ShaderCompileKey};
pub use types::{
    AssetId, AssetKind, BuildConfig, CookRequest, CookedEntry, CookedManifest, PlatformProfile,
    TargetPlatform,
};
