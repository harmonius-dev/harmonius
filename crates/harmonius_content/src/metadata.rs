//! In-memory metadata store with simple search.

use std::collections::HashMap;
use std::path::PathBuf;

use crate::{AssetId, ContentHash, ImportSettings};

/// Discriminant for stored assets.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AssetType {
    /// 2D texture.
    Texture,
    /// 3D mesh.
    Mesh,
    /// PBR material.
    Material,
    /// Other / unspecified.
    Other,
}

/// Per-asset metadata row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssetMetadata {
    /// Asset id.
    pub asset_id: AssetId,
    /// Content hash of primary blob.
    pub content_hash: ContentHash,
    /// Original source path string (for tests).
    pub source_path: PathBuf,
    /// Import settings snapshot.
    pub import_settings: ImportSettings,
    /// Free-form tags.
    pub tags: Vec<String>,
    /// Asset type for faceted search.
    pub asset_type: AssetType,
    /// Logical version counter.
    pub version: u32,
    /// Display name for full-text search.
    pub name: String,
    /// Optional thumbnail CAS hash.
    pub thumbnail_hash: Option<ContentHash>,
}

/// Search predicates.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SearchFilter {
    /// Substring match on `name` or any `tag` (case-sensitive per tests).
    Text(String),
    /// Require asset type and tag simultaneously.
    Facet {
        /// Required asset type.
        asset_type: AssetType,
        /// Required tag substring.
        tag: String,
    },
}

/// Sorted map semantics simulated with `BTreeMap` order by id for stable `query` results.
#[derive(Debug, Default)]
pub struct MetadataStore {
    entries: HashMap<AssetId, AssetMetadata>,
}

impl MetadataStore {
    /// New empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Lookup by id.
    pub fn get(&self, id: AssetId) -> Option<&AssetMetadata> {
        self.entries.get(&id)
    }

    /// Insert or replace metadata.
    pub fn put(&mut self, id: AssetId, meta: AssetMetadata) {
        self.entries.insert(id, meta);
    }

    /// Remove a row.
    pub fn remove(&mut self, id: AssetId) -> bool {
        self.entries.remove(&id).is_some()
    }

    /// Return ids matching `filter`.
    pub fn query(&self, filter: &SearchFilter) -> Vec<AssetId> {
        let mut ids: Vec<AssetId> = self
            .entries
            .iter()
            .filter_map(|(id, m)| {
                let ok = match filter {
                    SearchFilter::Text(q) => {
                        m.name.contains(q.as_str()) || m.tags.iter().any(|t| t.contains(q.as_str()))
                    }
                    SearchFilter::Facet { asset_type, tag } => {
                        m.asset_type == *asset_type
                            && m.tags.iter().any(|t| t.contains(tag.as_str()))
                    }
                };
                if ok {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        ids.sort_by_key(|a| a.0);
        ids
    }

    /// All known ids.
    pub fn all_ids(&self) -> Vec<AssetId> {
        let mut v: Vec<_> = self.entries.keys().copied().collect();
        v.sort_by_key(|a| a.0);
        v
    }

    /// Count of rows.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
