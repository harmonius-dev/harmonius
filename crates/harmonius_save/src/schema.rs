//! Schema registry: sorted entries, no `HashMap` (R-13.3.2 / design `SchemaRegistry`).

use crate::arena::Arena;
use crate::error::LoadError;
use crate::error::SaveError;
use crate::types::SchemaVersion;

/// Stable component identity for save format negotiation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SaveTypeId(pub u64);

/// Placeholder world handle until ECS lands.
#[derive(Debug, Default)]
pub struct World;

/// Placeholder entity id until ECS lands.
#[derive(Clone, Copy, Debug)]
pub struct Entity(pub u64);

/// Placeholder command buffer until ECS lands.
#[derive(Debug, Default)]
pub struct CommandBuffer;

/// Codegen'd descriptor for one format version of a component type.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct SchemaEntry {
    pub type_id: SaveTypeId,
    pub version: SchemaVersion,
    pub serialize: fn(&World, Entity, &mut Arena) -> Result<Box<[u8]>, SaveError>,
    pub deserialize: fn(&[u8], &mut CommandBuffer, Entity) -> Result<(), LoadError>,
}

impl std::fmt::Debug for SchemaEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SchemaEntry")
            .field("type_id", &self.type_id)
            .field("version", &self.version)
            .field("serialize", &"<fn>")
            .field("deserialize", &"<fn>")
            .finish()
    }
}

fn entry_key(e: &SchemaEntry) -> (SaveTypeId, SchemaVersion) {
    (e.type_id, e.version)
}

/// Sorted `Vec` keyed by `(SaveTypeId, SchemaVersion)` — binary search only.
#[derive(Debug, Default)]
pub struct SchemaRegistry {
    entries: Vec<SchemaEntry>,
}

impl SchemaRegistry {
    /// Register a codegen'd schema entry.
    pub fn register(&mut self, entry: SchemaEntry) {
        let key = entry_key(&entry);
        match self.entries.binary_search_by_key(&key, entry_key) {
            Ok(i) => self.entries[i] = entry,
            Err(i) => self.entries.insert(i, entry),
        }
    }

    /// Look up the entry for a specific version.
    pub fn get(&self, type_id: SaveTypeId, version: SchemaVersion) -> Option<&SchemaEntry> {
        let key = (type_id, version);
        self.entries
            .binary_search_by_key(&key, entry_key)
            .ok()
            .map(|i| &self.entries[i])
    }

    /// Highest registered version for a type.
    pub fn highest(&self, type_id: SaveTypeId) -> Option<SchemaVersion> {
        self.entries
            .iter()
            .filter(|e| e.type_id == type_id)
            .map(|e| e.version)
            .max()
    }

    /// All registered versions for a type (sorted).
    pub fn versions(&self, type_id: SaveTypeId) -> Vec<SchemaVersion> {
        let mut v: Vec<SchemaVersion> = self
            .entries
            .iter()
            .filter(|e| e.type_id == type_id)
            .map(|e| e.version)
            .collect();
        v.sort();
        v.dedup();
        v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn noop_ser(_: &World, _: Entity, _: &mut Arena) -> Result<Box<[u8]>, SaveError> {
        Ok(Box::from([]))
    }

    fn noop_de(_: &[u8], _: &mut CommandBuffer, _: Entity) -> Result<(), LoadError> {
        Ok(())
    }

    fn entry(type_id: u64, major: u16) -> SchemaEntry {
        SchemaEntry {
            type_id: SaveTypeId(type_id),
            version: SchemaVersion {
                major,
                minor: 0,
                patch: 0,
            },
            serialize: noop_ser,
            deserialize: noop_de,
        }
    }

    /// TC-13.3.2.6-style per-component version table: multiple versions coexist sorted.
    #[test]
    fn tc_13_3_2_6_version_table_sorted() {
        let mut reg = SchemaRegistry::default();
        reg.register(entry(1, 1));
        reg.register(entry(1, 3));
        reg.register(entry(1, 2));
        let vers = reg.versions(SaveTypeId(1));
        assert_eq!(
            vers,
            vec![
                SchemaVersion {
                    major: 1,
                    minor: 0,
                    patch: 0
                },
                SchemaVersion {
                    major: 2,
                    minor: 0,
                    patch: 0
                },
                SchemaVersion {
                    major: 3,
                    minor: 0,
                    patch: 0
                },
            ]
        );
        assert_eq!(
            reg.highest(SaveTypeId(1)),
            Some(SchemaVersion {
                major: 3,
                minor: 0,
                patch: 0
            })
        );
    }
}
