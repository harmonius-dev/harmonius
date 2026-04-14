//! Minimal synchronous import coordinator for cache + processor accounting tests.

use std::path::Path;

use crate::import_cache::{CacheKey, ImportCache};
use crate::metadata::{AssetMetadata, AssetType, MetadataStore};
use crate::native_format::{import_native_asset, NativeImportOutput};
use crate::{AssetId, ImportError, ImportResult, ImportSettings};

/// Coordinates CAS, metadata, and import cache for deterministic tests.
#[derive(Debug)]
pub struct ImportCoordinator {
    /// Content-addressable store.
    pub cas: crate::cas::ContentAddressableStore,
    /// Metadata rows.
    pub metadata: MetadataStore,
    /// Import cache.
    pub cache: ImportCache,
    /// Increments on each processor run (cache miss path).
    pub processor_invocations: u32,
    next_id: u64,
}

impl ImportCoordinator {
    /// New coordinator rooted at `cas_root`.
    pub fn new(cas_root: std::path::PathBuf) -> Self {
        Self {
            cas: crate::cas::ContentAddressableStore::new(cas_root),
            metadata: MetadataStore::new(),
            cache: ImportCache::default(),
            processor_invocations: 0,
            next_id: 1,
        }
    }

    /// Import native bytes from `path` with `settings`, using `source` as cache key material.
    pub fn import_native_bytes(
        &mut self,
        path: &Path,
        source: &[u8],
        settings: ImportSettings,
    ) -> Result<ImportResult, ImportError> {
        let key = CacheKey::from_source_and_settings(source, &settings);
        if let Some(artifact) = self.cache.lookup(&key) {
            return Ok(ImportResult::CacheHit {
                artifact_key: artifact,
            });
        }
        let out = import_native_asset(source, path)?;
        self.processor_invocations += 1;
        let NativeImportOutput { content_hash, .. } = out;
        self.cas
            .store(content_hash, &source[40..])
            .map_err(|e| ImportError::Io {
                path: path.to_path_buf(),
                message: e.to_string(),
            })?;
        let id = AssetId(self.next_id);
        self.next_id += 1;
        self.cache.insert(key, content_hash);
        self.metadata.put(
            id,
            AssetMetadata {
                asset_id: id,
                content_hash,
                source_path: path.to_path_buf(),
                import_settings: settings.clone(),
                tags: vec![],
                asset_type: AssetType::Other,
                version: 1,
                name: path.display().to_string(),
                thumbnail_hash: None,
            },
        );
        Ok(ImportResult::Imported {
            asset_id: id,
            content_hash,
        })
    }
}
