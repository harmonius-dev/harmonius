//! ECS components used by composition definitions and recipes.

use std::collections::BTreeMap;

use bevy_ecs::prelude::*;

/// Runtime meter bound from [`crate::MeterDefinition`](super::MeterDefinition).
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Meter {
    /// Opaque binding cookie matching the issuing handle.
    pub binding_gen: u64,
    pub value: f32,
    pub min: f32,
    pub max: f32,
}

/// Attribute storage with deterministic key order.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct AttributeSet {
    /// Opaque binding generation.
    pub binding_gen: u64,
    pub values: BTreeMap<String, f32>,
}

/// Container layout kinds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContainerLayout {
    /// Fixed grid rows × columns.
    Grid {
        /// Row count.
        rows: u32,
        /// Column count.
        cols: u32,
    },
}

/// Item stack identifier for container slots.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ItemId(pub u64);

/// Runtime container.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Container {
    pub binding_gen: u64,
    pub layout: ContainerLayout,
    pub capacity: u32,
    /// Slot contents; `None` is empty.
    pub slots: Vec<Option<ItemId>>,
}

/// Directed graph instance summary (topology counts).
#[derive(Component, Clone, Debug, PartialEq)]
pub struct DirectedGraphInstance {
    pub binding_gen: u64,
    pub nodes: u32,
    pub edges: u32,
    pub current_node: u32,
}

/// Active gameplay effect marker (minimal for tests).
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Effect {
    pub name: String,
    pub magnitude: f32,
}

/// Quest progression marker for integration tests.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct QuestState {
    pub active_objective: u32,
    pub objectives: u32,
}

/// Ability caster state.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct AbilityState {
    pub mana: f32,
}

/// Optional single-target focus for [`AbilityState`] resolution during [`crate::step_frame`].
#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct AbilityCastTarget {
    /// Entity receiving ability effects when a cast is requested.
    pub target: Entity,
}

/// Inventory + equipment linkage.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct InventoryState {
    pub attack: f32,
}

/// Crafting counters for herbs/vials/potions.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct CraftingState {
    pub herbs: u32,
    pub vials: u32,
    pub potions: u32,
}

/// Stealth / detection log index.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct StealthState {
    pub detected: bool,
}

/// NPC schedule destination.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct Destination {
    pub target: Entity,
}

/// Timeline progress for schedule recipe tests.
#[derive(Component, Clone, Debug, PartialEq)]
pub struct ScheduleState {
    /// Elapsed ticks since install.
    pub elapsed: u64,
    /// Tick at which the keyframe fires.
    pub fire_at: u64,
    /// Whether the keyframe callback ran.
    pub fired: bool,
}
