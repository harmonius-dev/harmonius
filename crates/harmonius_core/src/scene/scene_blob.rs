//! Minimal scene snapshot format for deterministic tests.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Entity, SceneError, Transform, World};

/// Serializable scene snapshot using stable entity indices.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    records: Vec<SceneRecord>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SceneRecord {
    transform: Transform,
    parent: Option<usize>,
}

impl Scene {
    /// Builds a scene from explicit records (indices are local to the snapshot).
    #[must_use]
    pub fn from_records(records: Vec<(Transform, Option<usize>)>) -> Self {
        Self {
            records: records
                .into_iter()
                .map(|(transform, parent)| SceneRecord { transform, parent })
                .collect(),
        }
    }

    /// Validates parent pointers before serialization.
    pub fn validate(&self) -> Result<(), SceneError> {
        let parents: Vec<Option<usize>> = self.records.iter().map(|r| r.parent).collect();
        detect_cycle(&parents)
    }

    /// Serializes to JSON bytes.
    pub fn to_json_bytes(&self) -> Result<Vec<u8>, SceneError> {
        self.validate()?;
        serde_json::to_vec(self).map_err(|_| SceneError::Serialization)
    }

    /// Deserializes from JSON bytes.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, SceneError> {
        let scene: Scene = serde_json::from_slice(bytes).map_err(|_| SceneError::Serialization)?;
        scene.validate()?;
        Ok(scene)
    }

    /// Returns the number of entities in the snapshot.
    #[must_use]
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Returns `true` when the snapshot contains no entities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
}

/// Maps snapshot entity indices to runtime [`Entity`] handles after a spawn.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EntityMap {
    map: HashMap<usize, Entity>,
}

impl EntityMap {
    /// Inserts a mapping entry.
    pub fn insert(&mut self, scene_index: usize, world_entity: Entity) {
        self.map.insert(scene_index, world_entity);
    }

    /// Looks up a spawned entity.
    #[must_use]
    pub fn get(&self, scene_index: usize) -> Option<Entity> {
        self.map.get(&scene_index).copied()
    }

    /// Number of mapped entities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Returns `true` when no mappings exist.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

/// Opaque identifier for a spawned scene instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SceneInstanceId(pub u64);

/// Spawns [`Scene`] snapshots into a [`World`].
#[derive(Debug, Default)]
pub struct SceneSpawner {
    next_instance: u64,
}

impl SceneSpawner {
    /// Spawns all entities from `scene` as children of `parent`, remapping roots to the parent.
    pub fn spawn_as_child(
        &mut self,
        scene: &Scene,
        world: &mut World,
        parent: Entity,
    ) -> Result<EntityMap, SceneError> {
        let map = self.spawn(scene, world)?;
        for index in 0..scene.records.len() {
            if scene.records[index].parent.is_none() {
                let child = *map.map.get(&index).ok_or(SceneError::Serialization)?;
                world.commands_mut().set_parent(child, parent);
            }
        }
        world.flush_hierarchy_commands()?;
        Ok(map)
    }

    /// Spawns all entities from `scene` into `world`, returning the index remap.
    pub fn spawn(&mut self, scene: &Scene, world: &mut World) -> Result<EntityMap, SceneError> {
        scene.validate()?;
        let mut map = EntityMap::default();
        for (index, record) in scene.records.iter().enumerate() {
            let entity = world.spawn_transform(record.transform);
            map.insert(index, entity);
        }
        for (index, record) in scene.records.iter().enumerate() {
            if let Some(parent_index) = record.parent {
                let child = *map.map.get(&index).ok_or(SceneError::Serialization)?;
                let parent = *map
                    .map
                    .get(&parent_index)
                    .ok_or(SceneError::Serialization)?;
                world.commands_mut().set_parent(child, parent);
            }
        }
        world.flush_hierarchy_commands()?;
        self.next_instance = self.next_instance.wrapping_add(1);
        Ok(map)
    }
}

fn detect_cycle(parents: &[Option<usize>]) -> Result<(), SceneError> {
    let mut color = vec![0u8; parents.len()];
    for i in 0..parents.len() {
        if color[i] != 0 {
            continue;
        }
        dfs_parent_graph(i, parents, &mut color)?;
    }
    Ok(())
}

fn dfs_parent_graph(
    node: usize,
    parents: &[Option<usize>],
    color: &mut [u8],
) -> Result<(), SceneError> {
    color[node] = 1;
    if let Some(p) = parents.get(node).copied().flatten() {
        if p < parents.len() {
            match color[p] {
                1 => return Err(SceneError::CyclicHierarchy),
                0 => dfs_parent_graph(p, parents, color)?,
                _ => {}
            }
        }
    }
    color[node] = 2;
    Ok(())
}
