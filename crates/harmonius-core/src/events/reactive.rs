//! Archetype-level reactive query stub used by scheduler skip logic tests.

use std::any::TypeId;

use crate::world::World;

/// Tracks whether a component type's archetypes changed since the last mark.
#[derive(Debug)]
pub struct ReactiveQuery {
    watched: TypeId,
    last_change_tick: u64,
}

impl ReactiveQuery {
    /// Builds a reactive query watching `T`.
    pub fn new<T: 'static>() -> Self {
        Self {
            watched: TypeId::of::<T>(),
            last_change_tick: 0,
        }
    }

    /// Returns true when the world's recorded change tick for `T` advanced.
    pub fn has_changed(&self, world: &World, _current_tick: u64) -> bool {
        match world.last_change_tick_for_id(self.watched) {
            Some(t) => t > self.last_change_tick,
            None => false,
        }
    }

    /// Records the latest observed change tick from the world.
    pub fn mark_seen(&mut self, world: &World) {
        if let Some(t) = world.last_change_tick_for_id(self.watched) {
            self.last_change_tick = t;
        }
    }
}
