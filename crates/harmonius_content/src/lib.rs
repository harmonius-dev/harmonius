//! Asset pipeline building blocks: native import header, CAS, metadata, caching,
//! dependency invalidation, hot-reload helpers, and binary asset format I/O.
//!
//! This crate implements the behaviors covered by `docs/design/content-pipeline/asset-pipeline-test-cases.md`
//! for Harmonius plan `PLAN-content-pipeline-asset-pipeline`.

#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![allow(clippy::too_many_lines)]

pub mod asset_binary;
pub mod audio_flac;
pub mod audio_wav;
pub mod auto_resolve;
pub mod batch_import;
pub mod bundle;
pub mod cas;
pub mod coordinator;
pub mod data_table;
pub mod dep_graph;
pub mod editor_sync;
pub mod file_watch;
pub mod handle_table;
pub mod headless_batch;
pub mod import_cache;
pub mod logic_graph;
pub mod material_mapper;
pub mod metadata;
pub mod native_format;
pub mod pipeline;
pub mod progress;
pub mod shader_reload;
pub mod structural_diff;
pub mod subasset;
pub mod texture_decode;
pub mod three_way_merge;
pub mod thumbnail;
pub mod ui_reload;
pub mod validation;
pub mod version_store;
pub mod visual_inspector;

pub use asset_binary::{AssetError, AssetHeader, AssetReader, AssetWriter, ASSET_MAGIC, FORMAT_VERSION};
pub use audio_flac::parse_flac_loop_comments;
pub use audio_wav::{parse_wav_with_cue, WavMeta};
pub use auto_resolve::{auto_resolve_lww, auto_resolve_union_tags, AutoResolution, ResolutionStrategy};
pub use batch_import::{BatchImportHandle, BatchImportState, ImportEntry};
pub use bundle::{BundleDecodeStats, BundleReader, BundleWriter};
pub use cas::{ContentAddressableStore, GcResult, StoreResult};
pub use coordinator::ImportCoordinator;
pub use data_table::{edit_data_table_cell, structural_diff_table_cells, DataTable};
pub use dep_graph::DependencyGraph;
pub use editor_sync::{EditorMessage, EditorSync, MaterialRuntime};
pub use file_watch::{AssetChange, DebouncedWatcher};
pub use handle_table::{HandleTable, PendingSwap, SwapScheduler};
pub use headless_batch::run_headless_batch_twice;
pub use import_cache::{CacheKey, ImportCache};
pub use logic_graph::{hot_reload_logic_graph, LogicGraphInstance, LogicGraphSpec};
pub use material_mapper::{
    translate_gltf_pbr_material, HarMaterialDesc, HarTextureSlot, ImportedMaterial, MaterialSource,
};
pub use metadata::{AssetMetadata, AssetType, MetadataStore, SearchFilter};
pub use native_format::{import_native_asset, NativeImportOutput};
pub use pipeline::{run_pipeline, PipelineError};
pub use progress::{ImportProgressEvent, ProgressTracker};
pub use shader_reload::ShaderReloader;
pub use structural_diff::{diff_mesh_assets, DiffResult, MeshAssetSummary};
pub use subasset::{partial_subasset_reimport, CompositeAsset, SubassetEdit};
pub use texture_decode::{
    decode_exr_linear, decode_png_srgb, write_exr_linear_fixture, ColorSpace, DecodedTexture,
};
pub use three_way_merge::{three_way_merge_disjoint_graphs, MergeResult};
pub use thumbnail::{generate_thumbnail_for_mesh_import, ThumbnailJob};
pub use ui_reload::{reload_ui_panel_style, PanelState, UiDocument};
pub use validation::{import_with_optional_tags_validation, validate_native_version, ValidationWarning};
pub use version_store::VersionStore;
pub use visual_inspector::{visual_inspector_fields, InspectorRow, WidgetKind};

use std::path::PathBuf;

/// Stable asset identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AssetId(pub u64);

/// BLAKE3 content hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContentHash {
    /// Raw BLAKE3 digest.
    pub bytes: [u8; 32],
}

impl ContentHash {
    /// Hash of `data` using BLAKE3.
    pub fn from_data(data: &[u8]) -> Self {
        Self {
            bytes: *blake3::hash(data).as_bytes(),
        }
    }

    /// Hex encoding of the digest.
    pub fn hex(&self) -> String {
        self.bytes.iter().fold(String::new(), |mut s, b| {
            use std::fmt::Write as _;
            let _ = write!(s, "{b:02x}");
            s
        })
    }

    /// Two-byte prefix for sharded CAS paths.
    pub fn prefix(&self) -> [u8; 2] {
        [self.bytes[0], self.bytes[1]]
    }
}

/// Texture compression setting (cache key component).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextureCompression {
    /// BC7 block compression.
    Bc7,
    /// ASTC block compression.
    Astc,
}

/// Import settings affecting cache keys and processors.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ImportSettings {
    /// Native Harmonius interchange tests.
    Native,
    /// Texture import with compression choice.
    Texture {
        compression: TextureCompression,
        generate_mips: bool,
    },
}

/// Result of a single-file import through the coordinator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportResult {
    /// Newly imported and stored.
    Imported {
        asset_id: AssetId,
        content_hash: ContentHash,
    },
    /// Import cache hit; no processor work.
    CacheHit {
        artifact_key: ContentHash,
    },
}

/// Import failures for native and coordinated paths.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportError {
    /// Header hash does not match BLAKE3(body).
    HashMismatch {
        expected: ContentHash,
        actual: ContentHash,
    },
    /// Validation failed with structured diagnostics.
    ValidationFailed {
        path: PathBuf,
        offset: u64,
        suggestion: Option<String>,
    },
}

#[cfg(test)]
mod tc;
