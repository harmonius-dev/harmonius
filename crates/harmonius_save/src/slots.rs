//! Save slot layout: separate `.meta`, `.save`, `.thumb` files (R-13.3.4).

use std::path::Path;
use std::path::PathBuf;

use crate::error::SaveError;
use crate::types::SaveSlotMeta;
use crate::types::SlotId;
use crate::vfs::VirtualFileSystem;

/// Manages save slot metadata and paths under `save_dir`.
#[derive(Debug)]
pub struct SaveSlotManager {
    slots: Vec<SaveSlotMeta>,
    max_slots: u32,
    autosave_rotation: u32,
    autosave_cursor: u32,
    save_dir: PathBuf,
}

fn slot_prefix(id: SlotId) -> String {
    format!("slot_{:02}", id.0)
}

fn parse_slot_meta_filename(name: &str) -> Option<SlotId> {
    let name = name.strip_suffix(".meta")?;
    let id = name.strip_prefix("slot_")?;
    let n: u32 = id.parse().ok()?;
    Some(SlotId(n))
}

impl SaveSlotManager {
    /// New manager rooted at `save_dir`.
    pub fn new(max_slots: u32, autosave_rotation: u32, save_dir: impl Into<PathBuf>) -> Self {
        Self {
            slots: Vec::new(),
            max_slots,
            autosave_rotation,
            autosave_cursor: 0,
            save_dir: save_dir.into(),
        }
    }

    /// Relative path to the `.meta` file for a slot.
    pub fn meta_path(&self, id: SlotId) -> PathBuf {
        self.save_dir.join(format!("{}.meta", slot_prefix(id)))
    }

    /// Relative path to the `.save` archive for a slot.
    pub fn save_path(&self, id: SlotId) -> PathBuf {
        self.save_dir.join(format!("{}.save", slot_prefix(id)))
    }

    /// Relative path to the thumbnail blob for a slot.
    pub fn thumb_path(&self, id: SlotId) -> PathBuf {
        self.save_dir.join(format!("{}.thumb", slot_prefix(id)))
    }

    /// Cached slot metadata (may be stale vs disk until refreshed).
    pub fn list_slots(&self) -> &[SaveSlotMeta] {
        &self.slots
    }

    /// Refresh `slots` from `*.meta` files on disk.
    pub fn refresh_from_disk(&mut self, vfs: &dyn VirtualFileSystem) -> Result<(), SaveError> {
        let names = vfs
            .read_dir_names(&self.save_dir)
            .map_err(SaveError::IoFailed)?;
        let mut metas = Vec::new();
        for n in names {
            if let Some(id) = parse_slot_meta_filename(&n) {
                let path = self.save_dir.join(&n);
                let bytes = vfs.read_file(&path).map_err(SaveError::IoFailed)?;
                let meta: SaveSlotMeta =
                    serde_json::from_slice(&bytes).map_err(|e| SaveError::SerializationFailed {
                        entity: id.0 as u64,
                        type_hash: 0,
                        detail: e.to_string(),
                    })?;
                metas.push(meta);
            }
        }
        metas.sort_by_key(|m| m.id.0);
        self.slots = metas;
        Ok(())
    }

    /// Create a new slot with JSON metadata written to `slot_NN.meta`.
    pub fn create_slot(
        &mut self,
        name: &str,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<SlotId, SaveError> {
        if self.slots.len() >= self.max_slots as usize {
            return Err(SaveError::SlotLimitReached {
                max: self.max_slots,
            });
        }
        let id = SlotId(
            self.slots
                .iter()
                .map(|s| s.id.0)
                .max()
                .map(|m| m + 1)
                .unwrap_or(1),
        );
        let meta = SaveSlotMeta::fixture(id, name);
        self.write_meta(&meta, vfs)?;
        self.slots.push(meta);
        self.slots.sort_by_key(|m| m.id.0);
        Ok(id)
    }

    fn write_meta(
        &self,
        meta: &SaveSlotMeta,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<(), SaveError> {
        let path = self.meta_path(meta.id);
        let bytes =
            serde_json::to_vec_pretty(meta).map_err(|e| SaveError::SerializationFailed {
                entity: meta.id.0 as u64,
                type_hash: 0,
                detail: e.to_string(),
            })?;
        vfs.write_atomic(&path, &bytes)
            .map_err(SaveError::IoFailed)?;
        Ok(())
    }

    /// Delete slot files from disk and drop cached metadata.
    pub fn delete_slot(
        &mut self,
        id: SlotId,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<(), SaveError> {
        vfs.remove_file(&self.meta_path(id))
            .map_err(SaveError::IoFailed)?;
        vfs.remove_file(&self.save_path(id))
            .map_err(SaveError::IoFailed)?;
        vfs.remove_file(&self.thumb_path(id))
            .map_err(SaveError::IoFailed)?;
        self.slots.retain(|s| s.id != id);
        Ok(())
    }

    /// Copy slot `src` into a new slot named `dst` (transactional on `vfs`).
    pub fn copy_slot(
        &mut self,
        src: SlotId,
        dst_name: &str,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<SlotId, SaveError> {
        if self.slots.len() >= self.max_slots as usize {
            return Err(SaveError::SlotLimitReached {
                max: self.max_slots,
            });
        }
        let bytes = vfs
            .read_file(&self.meta_path(src))
            .map_err(SaveError::IoFailed)?;
        let mut meta: SaveSlotMeta =
            serde_json::from_slice(&bytes).map_err(|e| SaveError::SerializationFailed {
                entity: src.0 as u64,
                type_hash: 0,
                detail: e.to_string(),
            })?;
        let new_id = SlotId(
            self.slots
                .iter()
                .map(|s| s.id.0)
                .chain(std::iter::once(src.0))
                .max()
                .map(|m| m + 1)
                .unwrap_or(1),
        );
        meta.id = new_id;
        meta.name = dst_name.to_string();
        self.write_meta(&meta, vfs)?;
        vfs.copy_atomic(&self.save_path(src), &self.save_path(new_id))
            .map_err(SaveError::IoFailed)?;
        self.slots.push(meta);
        self.slots.sort_by_key(|m| m.id.0);
        Ok(new_id)
    }

    /// Export `.save` bytes to `path`.
    pub fn export_slot(
        &self,
        id: SlotId,
        path: &Path,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<(), SaveError> {
        vfs.copy_atomic(&self.save_path(id), path)
            .map_err(SaveError::IoFailed)
    }

    /// Import a `.save` archive from `path` into a fresh slot.
    pub fn import_slot(
        &mut self,
        path: &Path,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<SlotId, SaveError> {
        let id = self.create_slot("imported", vfs)?;
        vfs.copy_atomic(path, &self.save_path(id))
            .map_err(SaveError::IoFailed)?;
        Ok(id)
    }

    /// Next rotating autosave slot index in `[0, autosave_rotation)`.
    pub fn next_autosave_slot(&mut self) -> SlotId {
        let id = self.autosave_cursor % self.autosave_rotation.max(1);
        self.autosave_cursor = self.autosave_cursor.wrapping_add(1);
        SlotId(id)
    }

    /// Replace cached metadata for `id`.
    pub fn update_meta(
        &mut self,
        id: SlotId,
        meta: SaveSlotMeta,
        vfs: &dyn VirtualFileSystem,
    ) -> Result<(), SaveError> {
        if meta.id != id {
            return Err(SaveError::SerializationFailed {
                entity: id.0 as u64,
                type_hash: 0,
                detail: "meta.id mismatch".into(),
            });
        }
        self.write_meta(&meta, vfs)?;
        if let Some(s) = self.slots.iter_mut().find(|s| s.id == id) {
            *s = meta;
        } else {
            self.slots.push(meta);
            self.slots.sort_by_key(|m| m.id.0);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vfs::StdVirtualFileSystem;
    use tempfile::tempdir;

    /// TC-13.3.4.1 Save slot metadata fields roundtrip through JSON meta file.
    #[test]
    fn tc_13_3_4_1_meta_fields_populated() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        mgr.create_slot("alpha", &vfs).unwrap();
        mgr.refresh_from_disk(&vfs).unwrap();
        let m = &mgr.list_slots()[0];
        assert_eq!(m.name, "alpha");
        assert_eq!(m.real_world_date, 1_700_000_000);
        assert_eq!(
            m.engine_version,
            crate::types::SchemaVersion {
                major: 0,
                minor: 2,
                patch: 0
            }
        );
        assert_eq!(m.save_type, crate::types::SaveType::Manual);
        assert_eq!(m.platform, crate::types::PlatformId(1));
    }

    /// TC-13.3.4.2 Meta file separate from save archive.
    #[test]
    fn tc_13_3_4_2_meta_and_save_separate_files() {
        let dir = tempdir().unwrap();
        let root = dir.path().to_path_buf();
        let vfs = StdVirtualFileSystem::new(root.clone());
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        let id = mgr.create_slot("s", &vfs).unwrap();
        vfs.write_atomic(&mgr.save_path(id), b"SAVEBYTES").unwrap();
        let names = vfs.read_dir_names(Path::new(".")).unwrap();
        assert!(names.iter().any(|n| n.ends_with(".meta")));
        assert!(names.iter().any(|n| n.ends_with(".save")));
    }

    /// TC-13.3.4.3 Thumbnail not read when loading meta only.
    #[test]
    fn tc_13_3_4_3_thumb_not_read_for_meta_only() {
        use crate::vfs::CountingVirtualFileSystem;
        let dir = tempdir().unwrap();
        let inner = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let vfs = CountingVirtualFileSystem::new(inner);
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        let id = mgr.create_slot("s", &vfs).unwrap();
        vfs.write_atomic(&mgr.thumb_path(id), b"THUMB").unwrap();
        vfs.read_count.set(0);
        mgr.refresh_from_disk(&vfs).unwrap();
        assert_eq!(vfs.read_count.get(), 1);
    }

    /// TC-13.3.4.5 Slot delete removes files.
    #[test]
    fn tc_13_3_4_5_delete_slot() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        let id = mgr.create_slot("x", &vfs).unwrap();
        mgr.delete_slot(id, &vfs).unwrap();
        mgr.refresh_from_disk(&vfs).unwrap();
        assert!(mgr.list_slots().is_empty());
    }

    /// TC-13.3.4.6 Export and import roundtrip `.save` bytes.
    #[test]
    fn tc_13_3_4_6_export_import() {
        let dir = tempdir().unwrap();
        let vfs = StdVirtualFileSystem::new(dir.path().to_path_buf());
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        let id1 = mgr.create_slot("a", &vfs).unwrap();
        vfs.write_atomic(&mgr.save_path(id1), b"PAYLOAD").unwrap();
        let export_path = PathBuf::from("out.save");
        mgr.export_slot(id1, &export_path, &vfs).unwrap();
        let id3 = mgr.import_slot(&export_path, &vfs).unwrap();
        let got = vfs.read_file(&mgr.save_path(id3)).unwrap();
        assert_eq!(got, b"PAYLOAD");
    }

    /// TC-13.3.3.2 Autosave rotation sequence.
    #[test]
    fn tc_13_3_3_2_autosave_rotation() {
        let _dir = tempdir().unwrap();
        let mut mgr = SaveSlotManager::new(8, 3, PathBuf::from("."));
        let seq: Vec<u32> = (0..5).map(|_| mgr.next_autosave_slot().0).collect();
        assert_eq!(seq, vec![0, 1, 2, 0, 1]);
    }
}
