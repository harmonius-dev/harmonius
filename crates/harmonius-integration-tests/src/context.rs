//! `ExecutionContext` passed to codegen'd graph functions.

use crate::arena::ThreadArena;
use crate::commands::{CommandSegment, EntityCommands};
use crate::events::{DebugBridge, EventWriter};
use crate::world::{Component, Entity, World};

/// Sandbox passed into each `GraphFn` invocation.
pub struct ExecutionContext<'w> {
    /// Entity hosting this graph instance.
    pub entity: Entity,
    /// Read-only ECS world.
    pub world: &'w World,
    /// Per-thread deferred writes.
    pub commands: &'w mut CommandSegment,
    /// Event sink.
    pub events: &'w EventWriter,
    /// Frame counter.
    pub frame: u64,
    /// Delta time seconds.
    pub delta_time: f32,
    /// Instruction budget for this step.
    pub instruction_budget: u32,
    /// Scratch arena.
    pub arena: &'w ThreadArena,
    /// Optional debug bridge.
    pub debug: Option<&'w DebugBridge>,
}

impl<'w> ExecutionContext<'w> {
    /// Read component on the graph entity.
    pub fn read<T: Component>(&self) -> Option<&T> {
        self.world.get::<T>(self.entity)
    }

    /// Read component on another entity.
    pub fn read_entity<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.world.get::<T>(entity)
    }

    /// Queue a deferred component write.
    pub fn write<T: Component>(&mut self, entity: Entity, value: T) {
        let _ = self.commands.insert(entity, value);
    }

    /// Queue a deferred spawn.
    pub fn spawn(&mut self) -> EntityCommands<'_> {
        self.commands.spawn()
    }
}
