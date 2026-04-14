//! Minimal ECS `World` with typed component storage for integration tests.

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};

/// Stable entity identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity(pub u64);

/// Marker for types stored as components.
pub trait Component: Any + Send + Sync + 'static {}

impl<T: Any + Send + Sync + 'static> Component for T {}

trait Storage: Send + Sync {
    fn get(&self, entity: Entity) -> Option<&dyn Any>;
    fn insert(&mut self, entity: Entity, value: Box<dyn Any + Send>) -> bool;
    fn remove(&mut self, entity: Entity) -> bool;
}

#[derive(Default)]
struct MapStorage<T: Component> {
    map: HashMap<Entity, T>,
}

impl<T: Component> Storage for MapStorage<T> {
    fn get(&self, entity: Entity) -> Option<&dyn Any> {
        self.map.get(&entity).map(|c| c as &dyn Any)
    }

    fn insert(&mut self, entity: Entity, value: Box<dyn Any + Send>) -> bool {
        if let Ok(v) = value.downcast::<T>() {
            self.map.insert(entity, *v);
            return true;
        }
        false
    }

    fn remove(&mut self, entity: Entity) -> bool {
        self.map.remove(&entity).is_some()
    }
}

/// Dynamic component storage keyed by `TypeId`.
#[derive(Default)]
pub struct World {
    storages: HashMap<TypeId, Box<dyn Storage>>,
    alive: HashSet<Entity>,
}

impl World {
    /// Empty world.
    pub fn new() -> Self {
        Self::default()
    }

    fn ensure_storage<T: Component>(&mut self) {
        let tid = TypeId::of::<T>();
        self.storages
            .entry(tid)
            .or_insert_with(|| Box::new(MapStorage::<T> { map: HashMap::new() }));
    }

    /// Typed read.
    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T> {
        let tid = TypeId::of::<T>();
        let storage = self.storages.get(&tid)?;
        storage.get(entity)?.downcast_ref()
    }

    /// Ensure storage exists for `T` (call from test setup).
    pub fn ensure_registered<T: Component>(&mut self) {
        self.ensure_storage::<T>();
    }

    /// Typed write (immediate, not deferred).
    pub fn insert<T: Component>(&mut self, entity: Entity, value: T) {
        self.ensure_storage::<T>();
        let tid = TypeId::of::<T>();
        let storage = self.storages.get_mut(&tid).expect("storage exists");
        assert!(storage.insert(entity, Box::new(value)));
        self.alive.insert(entity);
    }

    /// Dynamic insert for command flush (`TypeId` must already have storage).
    pub fn insert_dyn(
        &mut self,
        entity: Entity,
        ty: TypeId,
        value: Box<dyn Any + Send>,
    ) -> bool {
        let Some(storage) = self.storages.get_mut(&ty) else {
            return false;
        };
        if storage.insert(entity, value) {
            self.alive.insert(entity);
            return true;
        }
        false
    }

    /// Dynamic remove for command flush.
    pub fn remove_dyn(&mut self, entity: Entity, ty: TypeId) {
        let Some(storage) = self.storages.get_mut(&ty) else {
            return;
        };
        let _ = storage.remove(entity);
    }

    /// Mark `entity` alive before components attach (spawn path).
    pub fn reserve_entity(&mut self, entity: Entity) {
        self.alive.insert(entity);
    }

    /// Remove one component type from an entity.
    pub fn remove<T: Component>(&mut self, entity: Entity) -> bool {
        let tid = TypeId::of::<T>();
        let Some(storage) = self.storages.get_mut(&tid) else {
            return false;
        };
        storage.remove(entity)
    }

    /// Remove entity and all attached components.
    pub fn despawn(&mut self, entity: Entity) {
        for storage in self.storages.values_mut() {
            let _ = storage.remove(entity);
        }
        self.alive.remove(&entity);
    }

    /// Whether the entity currently has any registered components.
    pub fn contains_entity(&self, entity: Entity) -> bool {
        self.alive.contains(&entity)
    }

    /// Iterate entities marked alive (integration tests).
    pub fn iter_entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.alive.iter().copied()
    }
}
