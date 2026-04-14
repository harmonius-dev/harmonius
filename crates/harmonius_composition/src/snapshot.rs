//! Save / load helpers using `rkyv` (R-16.5.7).

use bevy_ecs::prelude::*;

use crate::components::{
    AttributeSet, CraftingState, DirectedGraphInstance, Effect, InventoryState, QuestState,
    ScheduleState,
};

/// Errors from snapshot IO.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotError {
    /// Entity missing expected component.
    MissingComponent(&'static str),
    /// Archive decode failed.
    Decode,
}

/// Serializable quest + graph slice for save games.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug, PartialEq))]
pub struct CompositionSnapshot {
    /// Active quest objective index.
    pub quest_active: u32,
    /// Quest objective count.
    pub quest_objectives: u32,
    /// Graph current node.
    pub graph_node: u32,
    /// Graph node count.
    pub graph_nodes: u32,
    /// Graph edge count.
    pub graph_edges: u32,
}

impl CompositionSnapshot {
    /// Build from live ECS state on `entity`.
    pub fn capture(world: &World, entity: Entity) -> Result<Self, SnapshotError> {
        let q = world
            .entity(entity)
            .get::<QuestState>()
            .ok_or(SnapshotError::MissingComponent("QuestState"))?;
        let g = world
            .entity(entity)
            .get::<DirectedGraphInstance>()
            .ok_or(SnapshotError::MissingComponent("DirectedGraphInstance"))?;
        Ok(Self {
            quest_active: q.active_objective,
            quest_objectives: q.objectives,
            graph_node: g.current_node,
            graph_nodes: g.nodes,
            graph_edges: g.edges,
        })
    }

    /// Apply captured state back onto `entity`.
    pub fn apply(self, world: &mut World, entity: Entity) -> Result<(), SnapshotError> {
        {
            let mut e = world.entity_mut(entity);
            if let Some(mut q) = e.get_mut::<QuestState>() {
                q.active_objective = self.quest_active;
                q.objectives = self.quest_objectives;
            } else {
                return Err(SnapshotError::MissingComponent("QuestState"));
            }
        }
        {
            let mut e = world.entity_mut(entity);
            if let Some(mut g) = e.get_mut::<DirectedGraphInstance>() {
                g.current_node = self.graph_node;
                g.nodes = self.graph_nodes;
                g.edges = self.graph_edges;
            } else {
                return Err(SnapshotError::MissingComponent("DirectedGraphInstance"));
            }
        }
        Ok(())
    }
}

/// Serialize a composition snapshot to bytes.
pub fn save_snapshot(world: &World, entity: Entity) -> Result<Vec<u8>, SnapshotError> {
    let snap = CompositionSnapshot::capture(world, entity)?;
    rkyv::to_bytes::<rkyv::rancor::Error>(&snap)
        .map_err(|_| SnapshotError::Decode)
        .map(|v| v.into_vec())
}

/// Deserialize and apply a snapshot.
pub fn load_snapshot(world: &mut World, entity: Entity, bytes: &[u8]) -> Result<(), SnapshotError> {
    let archived = rkyv::access::<rkyv::Archived<CompositionSnapshot>, rkyv::rancor::Error>(bytes)
        .map_err(|_| SnapshotError::Decode)?;
    let snap = rkyv::deserialize::<CompositionSnapshot, rkyv::rancor::Error>(archived)
        .map_err(|_| SnapshotError::Decode)?;
    snap.apply(world, entity)
}

/// Serialize inventory + attribute + effect bundle.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug, PartialEq))]
pub struct InventoryEffectSnapshot {
    /// Attack stat on inventory state.
    pub attack: f32,
    /// Serialized attribute map (sorted keys in BTreeMap order on wire via vec).
    pub attrs: Vec<(String, f32)>,
    /// Optional effect.
    pub effect_name: Option<String>,
    pub effect_mag: Option<f32>,
}

impl InventoryEffectSnapshot {
    /// Capture from `entity`.
    pub fn capture(world: &World, entity: Entity) -> Result<Self, SnapshotError> {
        let inv = world
            .entity(entity)
            .get::<InventoryState>()
            .ok_or(SnapshotError::MissingComponent("InventoryState"))?;
        let attrs = world
            .entity(entity)
            .get::<AttributeSet>()
            .ok_or(SnapshotError::MissingComponent("AttributeSet"))?;
        let eff = world.entity(entity).get::<Effect>();
        Ok(Self {
            attack: inv.attack,
            attrs: attrs.values.iter().map(|(k, v)| (k.clone(), *v)).collect(),
            effect_name: eff.map(|e| e.name.clone()),
            effect_mag: eff.map(|e| e.magnitude),
        })
    }

    /// Restore components on `entity`.
    pub fn apply(self, world: &mut World, entity: Entity) -> Result<(), SnapshotError> {
        let mut map = std::collections::BTreeMap::new();
        for (k, v) in self.attrs {
            map.insert(k, v);
        }
        world.entity_mut(entity).insert(InventoryState {
            attack: self.attack,
        });
        let gen = crate::definitions::allocate_generation(world);
        world.entity_mut(entity).insert(AttributeSet {
            binding_gen: gen,
            values: map,
        });
        if let (Some(n), Some(m)) = (self.effect_name, self.effect_mag) {
            world.entity_mut(entity).insert(Effect {
                name: n,
                magnitude: m,
            });
        } else {
            world.entity_mut(entity).remove::<Effect>();
        }
        Ok(())
    }
}

/// Save inventory + attributes + optional effect.
pub fn save_inventory_snapshot(world: &World, entity: Entity) -> Result<Vec<u8>, SnapshotError> {
    let s = InventoryEffectSnapshot::capture(world, entity)?;
    rkyv::to_bytes::<rkyv::rancor::Error>(&s)
        .map_err(|_| SnapshotError::Decode)
        .map(|v| v.into_vec())
}

/// Load inventory snapshot.
pub fn load_inventory_snapshot(
    world: &mut World,
    entity: Entity,
    bytes: &[u8],
) -> Result<(), SnapshotError> {
    let archived =
        rkyv::access::<rkyv::Archived<InventoryEffectSnapshot>, rkyv::rancor::Error>(bytes)
            .map_err(|_| SnapshotError::Decode)?;
    let s = rkyv::deserialize::<InventoryEffectSnapshot, rkyv::rancor::Error>(archived)
        .map_err(|_| SnapshotError::Decode)?;
    s.apply(world, entity)
}

/// Crafting progress snapshot.
#[derive(Clone, Debug, rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)]
#[rkyv(derive(Debug, PartialEq))]
pub struct CraftingProgressSnapshot {
    pub herbs: u32,
    pub vials: u32,
    pub potions: u32,
    pub timeline_elapsed: u64,
}

impl CraftingProgressSnapshot {
    /// Capture crafting + schedule progress on `entity`.
    pub fn capture(world: &World, entity: Entity) -> Result<Self, SnapshotError> {
        let c = world
            .entity(entity)
            .get::<CraftingState>()
            .ok_or(SnapshotError::MissingComponent("CraftingState"))?;
        let s = world
            .entity(entity)
            .get::<ScheduleState>()
            .ok_or(SnapshotError::MissingComponent("ScheduleState"))?;
        Ok(Self {
            herbs: c.herbs,
            vials: c.vials,
            potions: c.potions,
            timeline_elapsed: s.elapsed,
        })
    }

    /// Restore crafting + schedule.
    pub fn apply(self, world: &mut World, entity: Entity) -> Result<(), SnapshotError> {
        world.entity_mut(entity).insert(CraftingState {
            herbs: self.herbs,
            vials: self.vials,
            potions: self.potions,
        });
        world.entity_mut(entity).insert(ScheduleState {
            elapsed: self.timeline_elapsed,
            fire_at: 540,
            fired: false,
        });
        Ok(())
    }
}

/// Save crafting progress.
pub fn save_crafting_snapshot(world: &World, entity: Entity) -> Result<Vec<u8>, SnapshotError> {
    let s = CraftingProgressSnapshot::capture(world, entity)?;
    rkyv::to_bytes::<rkyv::rancor::Error>(&s)
        .map_err(|_| SnapshotError::Decode)
        .map(|v| v.into_vec())
}

/// Load crafting snapshot.
pub fn load_crafting_snapshot(
    world: &mut World,
    entity: Entity,
    bytes: &[u8],
) -> Result<(), SnapshotError> {
    let archived =
        rkyv::access::<rkyv::Archived<CraftingProgressSnapshot>, rkyv::rancor::Error>(bytes)
            .map_err(|_| SnapshotError::Decode)?;
    let s = rkyv::deserialize::<CraftingProgressSnapshot, rkyv::rancor::Error>(archived)
        .map_err(|_| SnapshotError::Decode)?;
    s.apply(world, entity)
}
