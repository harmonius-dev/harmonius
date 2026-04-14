//! Named shelves for editor WIP snapshots (**TC-15.10.7.1**, R-15.10.7).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// Opaque shelf identifier returned by [`ShelfManager::create`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ShelfId(pub u64);

type WorkSnapshot = BTreeMap<String, Vec<u8>>;
type ShelfTable = BTreeMap<ShelfId, WorkSnapshot>;

/// In-memory shelf table keyed by [`ShelfId`].
#[derive(Debug)]
pub struct ShelfManager {
    work: Arc<Mutex<WorkSnapshot>>,
    shelves: Arc<Mutex<ShelfTable>>,
    next_id: AtomicU64,
}

impl ShelfManager {
    /// Empty manager with no materialized files.
    pub fn new() -> Self {
        Self {
            work: Arc::new(Mutex::new(BTreeMap::new())),
            shelves: Arc::new(Mutex::new(BTreeMap::new())),
            next_id: AtomicU64::new(1),
        }
    }

    /// Upsert a logical file in the active working snapshot.
    pub fn write_file(&self, path: impl Into<String>, bytes: Vec<u8>) {
        let mut guard = self.work.lock().expect("work lock");
        guard.insert(path.into(), bytes);
    }

    /// Read a file from the active working snapshot when present.
    pub fn read_file(&self, path: &str) -> Option<Vec<u8>> {
        let guard = self.work.lock().expect("work lock");
        guard.get(path).cloned()
    }

    /// Capture the current working snapshot and assign a new [`ShelfId`].
    pub fn create(&self, _name: &str) -> ShelfId {
        let snap = self.work.lock().expect("work lock").clone();
        let id = ShelfId(self.next_id.fetch_add(1, Ordering::Relaxed));
        self.shelves
            .lock()
            .expect("shelf lock")
            .insert(id, snap);
        id
    }

    /// Restore shelf `id` into the active working snapshot.
    pub fn apply(&self, id: ShelfId) -> Result<(), ShelfError> {
        let snap = self
            .shelves
            .lock()
            .expect("shelf lock")
            .get(&id)
            .cloned()
            .ok_or(ShelfError::UnknownShelf)?;
        let mut work = self.work.lock().expect("work lock");
        *work = snap;
        Ok(())
    }
}

impl Default for ShelfManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Shelf lookup failures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShelfError {
    /// No shelf exists for the requested [`ShelfId`].
    UnknownShelf,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **TC-15.10.7.1** — Shelf create and apply round-trips WIP changes.
    #[test]
    fn tc_15_10_7_1_shelf_round_trip() {
        let mgr = ShelfManager::new();
        mgr.write_file("scene.bin", vec![1, 2, 3]);
        let sid = mgr.create("wip-a");
        mgr.write_file("scene.bin", vec![9, 9, 9]);
        assert_eq!(mgr.read_file("scene.bin"), Some(vec![9, 9, 9]));
        mgr.apply(sid).expect("apply");
        assert_eq!(mgr.read_file("scene.bin"), Some(vec![1, 2, 3]));
    }
}
