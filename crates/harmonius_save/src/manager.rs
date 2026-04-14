//! Save manager façade: events + slot coordination (R-13.3.3).

use crate::arena::Arena;
use crate::error::SaveError;
use crate::migration::MigrationRegistry;
use crate::pipeline::SavePipeline;
use crate::serialize::SaveSerializer;
use crate::serialize::World;
use crate::slots::SaveSlotManager;
use crate::types::IoPriority;
use crate::types::SaveConfig;
use crate::types::SaveContext;
use crate::types::SaveEvent;
use crate::types::SaveSlotMeta;
use crate::types::SaveType;
use crate::types::SchemaVersion;
use crate::types::SlotId;
use crate::vfs::VirtualFileSystem;

/// Coordinates serialization, pipeline writes, and slot metadata (design subset).
#[derive(Debug)]
pub struct SaveManager {
    /// Slot bookkeeping and paths.
    pub slots: SaveSlotManager,
    /// Compression / encryption pipeline.
    pub pipeline: SavePipeline,
    /// Per-component migrations (wired on load in future milestones).
    pub migration: MigrationRegistry,
    pub config: SaveConfig,
    pub autosave_timer: f64,
    pub current_schema: SchemaVersion,
    /// Emitted events for tests and future ECS wiring.
    pub events: Vec<SaveEvent>,
}

impl SaveManager {
    /// Construct a manager rooted at `save_dir`.
    pub fn new(
        slots: SaveSlotManager,
        pipeline: SavePipeline,
        migration: MigrationRegistry,
        config: SaveConfig,
        current_schema: SchemaVersion,
    ) -> Self {
        Self {
            slots,
            pipeline,
            migration,
            config,
            autosave_timer: 0.0,
            current_schema,
            events: Vec::new(),
        }
    }

    /// Checkpoint save into `slot` using `context` (emits [`SaveEvent::SaveStarted`]).
    pub fn checkpoint_save(
        &mut self,
        world: &World,
        context: SaveContext,
        slot: SlotId,
        vfs: &dyn VirtualFileSystem,
        arena: &mut Arena,
    ) -> Result<(), SaveError> {
        self.events.push(SaveEvent::SaveStarted { slot });
        let ser = SaveSerializer;
        let bytes = ser.serialize_world(world, context, arena)?;
        let path = self.slots.save_path(slot);
        self.pipeline
            .write(slot, &bytes, IoPriority::Critical, vfs, &path)?;
        let mut meta = SaveSlotMeta::fixture(slot, "checkpoint");
        meta.save_type = SaveType::Checkpoint;
        meta.schema_version = self.current_schema;
        self.slots.update_meta(slot, meta.clone(), vfs)?;
        self.events.push(SaveEvent::SaveComplete { slot, meta });
        Ok(())
    }

    /// Autosave using rotating slot from [`SaveSlotManager::next_autosave_slot`].
    pub fn autosave(
        &mut self,
        world: &World,
        vfs: &dyn VirtualFileSystem,
        arena: &mut Arena,
    ) -> Result<(), SaveError> {
        let slot = self.slots.next_autosave_slot();
        self.events.push(SaveEvent::AutosaveTriggered { slot });
        self.checkpoint_save(world, SaveContext::World, slot, vfs, arena)
    }

    /// Advance autosave timer; returns true when an autosave should trigger.
    pub fn tick_autosave(&mut self, dt: f64) -> bool {
        self.autosave_timer += dt;
        let interval = self.config.autosave_interval_secs as f64;
        if interval > 0.0 && self.autosave_timer >= interval {
            self.autosave_timer = 0.0;
            true
        } else {
            false
        }
    }

    /// Read-only slot manager reference.
    pub fn slots(&self) -> &SaveSlotManager {
        &self.slots
    }

    /// Mutable slot manager reference.
    pub fn slots_mut(&mut self) -> &mut SaveSlotManager {
        &mut self.slots
    }
}

/// Test helper: build a default [`SaveManager`] over `save_dir`.
#[cfg(test)]
pub(crate) fn test_manager(
    vfs_root: std::path::PathBuf,
    save_dir: std::path::PathBuf,
) -> (SaveManager, crate::vfs::StdVirtualFileSystem) {
    use crate::types::CompressionMode;
    use crate::types::EncryptionConfig;
    use crate::types::KeySource;

    let vfs = crate::vfs::StdVirtualFileSystem::new(vfs_root);
    let slots = SaveSlotManager::new(8, 3, save_dir);
    let pipeline = SavePipeline::new(
        CompressionMode::Lz4,
        EncryptionConfig {
            key_source: KeySource::FixedKey,
        },
        SchemaVersion {
            major: 1,
            minor: 0,
            patch: 0,
        },
        Some([3u8; 32]),
    );
    let cfg = SaveConfig {
        max_slots: 8,
        autosave_rotation: 3,
        autosave_interval_secs: 1,
        local_compression: CompressionMode::Lz4,
        save_dir: ".".into(),
    };
    let mgr = SaveManager::new(
        slots,
        pipeline,
        MigrationRegistry::new(),
        cfg,
        SchemaVersion {
            major: 1,
            minor: 0,
            patch: 0,
        },
    );
    (mgr, vfs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    /// TC-13.3.3.1 Checkpoint emits `SaveStarted` and writes slot path.
    #[test]
    fn tc_13_3_3_1_checkpoint_events() {
        let dir = tempdir().unwrap();
        let (mut mgr, vfs) = test_manager(dir.path().to_path_buf(), PathBuf::from("."));
        let slot = SlotId(3);
        let meta = SaveSlotMeta::fixture(slot, "c");
        mgr.slots.update_meta(slot, meta.clone(), &vfs).unwrap();
        let world = World::default();
        let mut arena = Arena::new();
        mgr.checkpoint_save(&world, SaveContext::World, slot, &vfs, &mut arena)
            .unwrap();
        assert!(matches!(mgr.events[0], SaveEvent::SaveStarted { .. }));
        assert!(matches!(mgr.events[1], SaveEvent::SaveComplete { .. }));
        let p = mgr.slots.save_path(slot);
        let bytes = vfs.read_file(&p).unwrap();
        assert!(!bytes.is_empty());
    }
}
