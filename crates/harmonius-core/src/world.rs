//! Minimal `World` and identifiers used by events and plugin tests.

use std::any::{Any, TypeId};
use std::collections::BTreeMap;

/// Stable identifier for an [`Entity`].
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Entity(pub u64);

/// Identifier for an independent ECS world instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct WorldId(pub u32);

/// Component lifecycle signal used by event-dispatch observers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentEvent {
    /// A component was added to an entity.
    Added,
    /// A component was removed from an entity.
    Removed,
    /// A component value was mutated.
    Mutated,
    /// Internal table bookkeeping (unused in these tests).
    TableCreated,
    /// State machine transition bookkeeping (unused in these tests).
    StateTransition,
}

/// Lightweight world state shared by event observers and plugin tests.
#[derive(Debug)]
pub struct World {
    id: WorldId,
    archetype_change_tick: u64,
    archetype_change_by_type: BTreeMap<TypeId, u64>,
    resources: BTreeMap<TypeId, Box<dyn Any + Send>>,
    observer_log: Vec<(ComponentEvent, Entity, TypeId)>,
}

impl World {
    /// Creates an empty world with the given id.
    pub fn new(id: WorldId) -> Self {
        Self {
            id,
            archetype_change_tick: 0,
            archetype_change_by_type: BTreeMap::new(),
            resources: BTreeMap::new(),
            observer_log: Vec::new(),
        }
    }

    /// Returns this world's id.
    pub fn id(&self) -> WorldId {
        self.id
    }

    /// Monotonic tick used by reactive queries.
    pub fn current_tick(&self) -> u64 {
        self.archetype_change_tick
    }

    /// Marks the archetype set for `T` as changed at the next tick boundary.
    pub fn bump_component_archetype<T: 'static>(&mut self) {
        self.archetype_change_tick = self.archetype_change_tick.saturating_add(1);
        let id = TypeId::of::<T>();
        self.archetype_change_by_type
            .insert(id, self.archetype_change_tick);
    }

    /// Last tick at which `T`'s matching archetypes changed.
    pub fn last_change_tick_for<T: 'static>(&self) -> Option<u64> {
        self.last_change_tick_for_id(TypeId::of::<T>())
    }

    /// Last tick at which the given component type's archetypes changed.
    pub fn last_change_tick_for_id(&self, id: TypeId) -> Option<u64> {
        self.archetype_change_by_type.get(&id).copied()
    }

    /// Inserts a resource singleton.
    pub fn insert_resource<R: Any + Send + 'static>(&mut self, resource: R) {
        self.resources.insert(TypeId::of::<R>(), Box::new(resource));
    }

    /// Borrows a resource if present.
    pub fn get_resource<R: Any + Send + 'static>(&self) -> Option<&R> {
        self.resources
            .get(&TypeId::of::<R>())
            .and_then(|b| b.downcast_ref())
    }

    /// Borrows a resource mutably if present.
    pub fn get_resource_mut<R: Any + Send + 'static>(&mut self) -> Option<&mut R> {
        self.resources
            .get_mut(&TypeId::of::<R>())
            .and_then(|b| b.downcast_mut())
    }

    /// Records an observer callback invocation for deterministic tests.
    pub fn log_observer_event(&mut self, event: ComponentEvent, entity: Entity, component: TypeId) {
        self.observer_log.push((event, entity, component));
    }

    /// Returns recorded observer dispatches (test and diagnostics hook).
    pub fn observer_events(&self) -> &[(ComponentEvent, Entity, TypeId)] {
        &self.observer_log
    }
}
