//! World snapshot serialization without `HashMap` iteration (R-13.3.1).

use smallvec::SmallVec;

#[cfg(test)]
use smallvec::smallvec;

use crate::arena::Arena;
use crate::error::LoadError;
use crate::error::SaveError;
use crate::types::SaveContext;
use crate::types::SchemaVersion;

/// One saveable entity in the stand-in world until ECS lands.
#[derive(Clone, Debug)]
pub struct WorldEntity {
    pub stable_id: u64,
    pub contexts: SmallVec<[SaveContext; 2]>,
    pub dirty_tick: u64,
    pub last_saved_tick: u64,
    /// `(type_hash, schema_version, rkyv_bytes)` sorted by `type_hash` for deterministic output.
    pub components: SmallVec<[(u64, SchemaVersion, Vec<u8>); 8]>,
}

/// Minimal world view used by [`SaveSerializer`] tests (TC-13.3.1.*).
#[derive(Clone, Debug, Default)]
pub struct World {
    pub entities: Vec<WorldEntity>,
}

/// Component payload in a deserialized snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentSnapshot {
    pub type_hash: u64,
    pub schema_version: SchemaVersion,
    pub data: Box<[u8]>,
}

/// Entity snapshot emitted by [`SaveDeserializer`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EntitySnapshot {
    pub stable_id: u64,
    pub parent_id: Option<u64>,
    pub components: SmallVec<[ComponentSnapshot; 8]>,
}

/// Serializes [`World`] snapshots using sorted vectors only (no `HashMap`).
#[derive(Clone, Copy, Debug, Default)]
pub struct SaveSerializer;

impl SaveSerializer {
    /// Full-world serialization for entities matching `context`.
    pub fn serialize_world(
        &self,
        world: &World,
        context: SaveContext,
        arena: &mut Arena,
    ) -> Result<Box<[u8]>, SaveError> {
        let mut ids: Vec<u64> = world
            .entities
            .iter()
            .filter(|e| e.contexts.contains(&context))
            .map(|e| e.stable_id)
            .collect();
        ids.sort_unstable();
        ids.dedup();
        let mut out: Vec<u8> = Vec::new();
        for id in ids {
            let entity = world
                .entities
                .iter()
                .find(|e| e.stable_id == id)
                .ok_or_else(|| SaveError::SerializationFailed {
                    entity: id,
                    type_hash: 0,
                    detail: "missing entity".into(),
                })?;
            out.extend_from_slice(&entity.stable_id.to_le_bytes());
            let n = entity.components.len() as u32;
            out.extend_from_slice(&n.to_le_bytes());
            for (th, ver, data) in entity.components.iter() {
                out.extend_from_slice(&th.to_le_bytes());
                out.extend_from_slice(&ver.major.to_le_bytes());
                out.extend_from_slice(&ver.minor.to_le_bytes());
                out.extend_from_slice(&ver.patch.to_le_bytes());
                let len = data.len() as u32;
                out.extend_from_slice(&len.to_le_bytes());
                out.extend_from_slice(data);
            }
        }
        let _ = arena;
        Ok(out.into_boxed_slice())
    }

    /// Incremental save: entities whose `dirty_tick` exceeds `last_saved_tick`.
    pub fn serialize_incremental(
        &self,
        world: &World,
        context: SaveContext,
        arena: &mut Arena,
    ) -> Result<Box<[u8]>, SaveError> {
        let mut filtered = World {
            entities: world
                .entities
                .iter()
                .filter(|e| e.contexts.contains(&context) && e.dirty_tick > e.last_saved_tick)
                .cloned()
                .collect(),
        };
        filtered.entities.sort_by_key(|e| e.stable_id);
        self.serialize_world(&filtered, context, arena)
    }
}

/// Deserializes wire snapshots produced by [`SaveSerializer`].
#[derive(Clone, Copy, Debug, Default)]
pub struct SaveDeserializer;

impl SaveDeserializer {
    /// Parse binary snapshot into entity/component views.
    pub fn deserialize(
        &self,
        data: &[u8],
        arena: &mut Arena,
    ) -> Result<Vec<EntitySnapshot>, LoadError> {
        let _ = arena;
        let mut out = Vec::new();
        let mut i = 0usize;
        while i < data.len() {
            if i + 8 > data.len() {
                return Err(LoadError::DeserializationFailed {
                    detail: "truncated entity id".into(),
                });
            }
            let stable_id = u64::from_le_bytes(data[i..i + 8].try_into().unwrap());
            i += 8;
            if i + 4 > data.len() {
                return Err(LoadError::DeserializationFailed {
                    detail: "truncated component count".into(),
                });
            }
            let n = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
            i += 4;
            let mut comps = SmallVec::new();
            for _ in 0..n {
                if i + 8 + 6 + 4 > data.len() {
                    return Err(LoadError::DeserializationFailed {
                        detail: "truncated component header".into(),
                    });
                }
                let th = u64::from_le_bytes(data[i..i + 8].try_into().unwrap());
                i += 8;
                let major = u16::from_le_bytes(data[i..i + 2].try_into().unwrap());
                i += 2;
                let minor = u16::from_le_bytes(data[i..i + 2].try_into().unwrap());
                i += 2;
                let patch = u16::from_le_bytes(data[i..i + 2].try_into().unwrap());
                i += 2;
                let len = u32::from_le_bytes(data[i..i + 4].try_into().unwrap()) as usize;
                i += 4;
                if i + len > data.len() {
                    return Err(LoadError::DeserializationFailed {
                        detail: "truncated component payload".into(),
                    });
                }
                let payload = data[i..i + len].to_vec().into_boxed_slice();
                i += len;
                comps.push(ComponentSnapshot {
                    type_hash: th,
                    schema_version: SchemaVersion {
                        major,
                        minor,
                        patch,
                    },
                    data: payload,
                });
            }
            out.push(EntitySnapshot {
                stable_id,
                parent_id: None,
                components: comps,
            });
        }
        Ok(out)
    }
}

/// Simple 2D transform payload (TC-13.3.1.6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform2D {
    pub x: f32,
    pub y: f32,
    pub rotation: f32,
    pub sx: f32,
    pub sy: f32,
}

impl Transform2D {
    /// Lossless binary encoding (not full rkyv yet).
    pub fn to_bytes(self) -> Vec<u8> {
        let mut v = Vec::with_capacity(20);
        v.extend_from_slice(&self.x.to_le_bytes());
        v.extend_from_slice(&self.y.to_le_bytes());
        v.extend_from_slice(&self.rotation.to_le_bytes());
        v.extend_from_slice(&self.sx.to_le_bytes());
        v.extend_from_slice(&self.sy.to_le_bytes());
        v
    }

    /// Decode [`Transform2D::to_bytes`] output.
    pub fn from_bytes(data: &[u8]) -> Option<Self> {
        if data.len() != 20 {
            return None;
        }
        Some(Self {
            x: f32::from_le_bytes(data[0..4].try_into().ok()?),
            y: f32::from_le_bytes(data[4..8].try_into().ok()?),
            rotation: f32::from_le_bytes(data[8..12].try_into().ok()?),
            sx: f32::from_le_bytes(data[12..16].try_into().ok()?),
            sy: f32::from_le_bytes(data[16..20].try_into().ok()?),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_world() -> SaveContext {
        SaveContext::World
    }

    /// TC-13.3.1.1 Full-world serialization for 50 entities.
    #[test]
    fn tc_13_3_1_1_full_world_fifty_entities() {
        let mut world = World::default();
        for i in 0..50 {
            world.entities.push(WorldEntity {
                stable_id: i,
                contexts: smallvec![SaveContext::Character],
                dirty_tick: 0,
                last_saved_tick: 0,
                components: SmallVec::new(),
            });
        }
        let ser = SaveSerializer;
        let mut arena = Arena::new();
        let bytes = ser
            .serialize_world(&world, SaveContext::Character, &mut arena)
            .unwrap();
        let de = SaveDeserializer;
        let snaps = de.deserialize(&bytes, &mut arena).unwrap();
        assert_eq!(snaps.len(), 50);
    }

    /// TC-13.3.1.2 Dirty-only serialization counts dirty entities.
    #[test]
    fn tc_13_3_1_2_incremental_ten_dirty() {
        let mut world = World::default();
        for i in 0..50 {
            world.entities.push(WorldEntity {
                stable_id: i,
                contexts: smallvec![SaveContext::World],
                dirty_tick: if i < 10 { 2 } else { 0 },
                last_saved_tick: 0,
                components: SmallVec::new(),
            });
        }
        let ser = SaveSerializer;
        let mut arena = Arena::new();
        let bytes = ser
            .serialize_incremental(&world, SaveContext::World, &mut arena)
            .unwrap();
        let snaps = SaveDeserializer::default()
            .deserialize(&bytes, &mut arena)
            .unwrap();
        assert_eq!(snaps.len(), 10);
    }

    /// TC-13.3.1.3 Roundtrip for a single test component payload.
    #[test]
    fn tc_13_3_1_3_codegen_roundtrip_placeholder() {
        let mut world = World::default();
        world.entities.push(WorldEntity {
            stable_id: 1,
            contexts: smallvec![ctx_world()],
            dirty_tick: 0,
            last_saved_tick: 0,
            components: smallvec![(
                0xDEAD,
                SchemaVersion {
                    major: 1,
                    minor: 0,
                    patch: 0,
                },
                vec![1, 2, 3],
            )],
        });
        let mut arena = Arena::new();
        let bytes = SaveSerializer
            .serialize_world(&world, SaveContext::World, &mut arena)
            .unwrap();
        let snaps = SaveDeserializer::default()
            .deserialize(&bytes, &mut arena)
            .unwrap();
        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].components[0].data.as_ref(), [1, 2, 3]);
    }

    /// TC-13.3.1.4 Stable ordering across runs (sorted entity ids and type hashes).
    #[test]
    fn tc_13_3_1_4_deterministic_ordering() {
        let mut world = World::default();
        for id in [3u64, 1, 2] {
            world.entities.push(WorldEntity {
                stable_id: id,
                contexts: smallvec![SaveContext::World],
                dirty_tick: 0,
                last_saved_tick: 0,
                components: smallvec![
                    (
                        2,
                        SchemaVersion {
                            major: 1,
                            minor: 0,
                            patch: 0,
                        },
                        vec![2],
                    ),
                    (
                        1,
                        SchemaVersion {
                            major: 1,
                            minor: 0,
                            patch: 0,
                        },
                        vec![1],
                    ),
                ],
            });
        }
        world.entities.sort_by_key(|e| e.stable_id);
        for e in &mut world.entities {
            e.components.sort_by_key(|c| c.0);
        }
        let mut arena = Arena::new();
        let a = SaveSerializer
            .serialize_world(&world, SaveContext::World, &mut arena)
            .unwrap();
        arena.reset();
        let b = SaveSerializer
            .serialize_world(&world, SaveContext::World, &mut arena)
            .unwrap();
        assert_eq!(a.as_ref(), b.as_ref());
    }

    /// TC-13.3.1.5 Arena reset clears growth between saves.
    #[test]
    fn tc_13_3_1_5_arena_reset_between_saves() {
        let mut arena = Arena::new();
        arena.push_bytes(&[0u8; 128]);
        let first = arena.len();
        arena.reset();
        assert_eq!(arena.len(), 0);
        arena.push_bytes(&[0u8; 128]);
        assert_eq!(arena.len(), first);
    }

    /// TC-13.3.1.6 Transform2D roundtrip.
    #[test]
    fn tc_13_3_1_6_transform2d_roundtrip() {
        let t = Transform2D {
            x: 1.0,
            y: -2.5,
            rotation: 0.25,
            sx: 1.0,
            sy: 2.0,
        };
        let b = t.to_bytes();
        assert_eq!(Transform2D::from_bytes(&b).unwrap(), t);
    }
}
