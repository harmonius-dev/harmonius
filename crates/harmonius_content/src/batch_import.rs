//! Cooperative batch import with cancellation.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::metadata::{AssetMetadata, AssetType, MetadataStore};
use crate::progress::ProgressTracker;
use crate::{AssetId, ContentHash, ImportSettings};

/// One entry in a batch.
#[derive(Clone, Debug)]
pub struct ImportEntry {
    /// Logical path for metadata.
    pub path: PathBuf,
    /// Source bytes.
    pub bytes: Vec<u8>,
}

/// Handle to cancel in-flight batch work.
#[derive(Debug)]
pub struct BatchImportHandle {
    cancelled: bool,
    total: usize,
    completed: usize,
    entries: Vec<ImportEntry>,
    progress: ProgressTracker,
}

/// Snapshot after a cancelled or finished batch.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BatchImportState {
    /// How many entries were committed to metadata.
    pub committed_metadata: usize,
}

impl BatchImportHandle {
    /// Start a batch (does not run imports until `tick` is called).
    pub fn new(entries: Vec<ImportEntry>) -> Self {
        let total = entries.len();
        Self {
            cancelled: false,
            total,
            completed: 0,
            entries,
            progress: ProgressTracker::new(),
        }
    }

    /// Request cancellation before remaining entries are committed.
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }

    /// Returns whether cancelled.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Process up to `max` more items unless cancelled.
    pub fn tick(
        &mut self,
        metadata: &mut MetadataStore,
        mut commit: impl FnMut(&ImportEntry) -> AssetMetadata,
    ) -> BatchImportState {
        let mut n = 0usize;
        while self.completed < self.total && n < 1 {
            if self.cancelled {
                break;
            }
            let idx = self.completed;
            let entry = &self.entries[idx];
            let meta = commit(entry);
            metadata.put(meta.asset_id, meta);
            self.progress.on_item_completed(idx);
            self.completed += 1;
            n += 1;
        }
        BatchImportState {
            committed_metadata: metadata.len(),
        }
    }

    /// Run all ticks until done or cancelled; used by tests for fast-forward.
    pub fn run_all(
        &mut self,
        metadata: &mut MetadataStore,
        mut commit: impl FnMut(&ImportEntry) -> AssetMetadata,
    ) -> BatchImportState {
        while self.completed < self.total && !self.cancelled {
            self.tick(metadata, |e| commit(e));
        }
        BatchImportState {
            committed_metadata: metadata.len(),
        }
    }

    /// Borrow progress tracker.
    pub fn progress(&self) -> &ProgressTracker {
        &self.progress
    }

    /// Mutable progress tracker.
    pub fn progress_mut(&mut self) -> &mut ProgressTracker {
        &mut self.progress
    }
}

/// Simulate GC over CAS: remove blobs not referenced by current metadata.
pub fn gc_cas_orphans(
    cas_root: &std::path::Path,
    metadata: &MetadataStore,
) -> std::io::Result<crate::cas::GcResult> {
    let mut referenced = HashSet::new();
    for id in metadata.all_ids() {
        if let Some(m) = metadata.get(id) {
            referenced.insert(m.content_hash);
            if let Some(t) = m.thumbnail_hash {
                referenced.insert(t);
            }
        }
    }
    let mut cas = crate::cas::ContentAddressableStore::new(cas_root.to_path_buf());
    cas.gc(&referenced)
}

/// Build trivial metadata for batch tests.
pub fn simple_meta(id: u64, path: PathBuf, bytes: &[u8]) -> AssetMetadata {
    AssetMetadata {
        asset_id: AssetId(id),
        content_hash: ContentHash::from_data(bytes),
        source_path: path,
        import_settings: ImportSettings::Native,
        tags: vec![],
        asset_type: AssetType::Other,
        version: 1,
        name: String::new(),
        thumbnail_hash: None,
    }
}
