//! Minimal ECS-style `World` harness for integration tests.

use crate::bindings_system::BlackboardBindings;
use crate::blackboard::Blackboard;
use crate::cache::AiTableCache;
use crate::component_store::ComponentStore;
use crate::events::EntityEventQueue;
use crate::ids::TableId;
use crate::table::{DatabaseRow, TableRegistry};
use crate::trace::AiDataTraceFlag;
use crate::TableReloaded;

/// Stable entity identifier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct EntityId(pub u32);

#[derive(Debug)]
struct EntityState {
    id: EntityId,
    row: Option<DatabaseRow>,
    bb: Blackboard,
    bindings: Option<BlackboardBindings>,
}

/// Lightweight world owning registry, entities, caches, and reload queue.
#[derive(Debug)]
pub struct World {
    /// Global table registry resource.
    pub registry: TableRegistry,
    /// Reload events queued at `FrameEnd`.
    pub reload_events: EntityEventQueue<TableReloaded>,
    /// Trace + warning buffer.
    pub trace: AiDataTraceFlag,
    /// `AiTableCache` components keyed by entity.
    pub caches: ComponentStore<AiTableCache>,
    entities: Vec<EntityState>,
    next_id: u32,
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

impl World {
    /// Empty world.
    pub fn new() -> Self {
        Self {
            registry: TableRegistry::default(),
            reload_events: EntityEventQueue::default(),
            trace: AiDataTraceFlag::new(),
            caches: ComponentStore::default(),
            entities: Vec::new(),
            next_id: 1,
        }
    }

    /// Spawns an entity with optional `DatabaseRow` / bindings / blackboard.
    pub fn spawn(
        &mut self,
        row: Option<DatabaseRow>,
        bindings: Option<BlackboardBindings>,
        bb: Blackboard,
    ) -> EntityId {
        let id = EntityId(self.next_id);
        self.next_id += 1;
        let cache = if let Some(r) = &row {
            AiTableCache::new(r.table)
        } else {
            AiTableCache::new(TableId(0))
        };
        self.caches.insert(id, cache);
        self.entities.push(EntityState {
            id,
            row,
            bb,
            bindings,
        });
        id
    }

    fn index(&self, entity: EntityId) -> Option<usize> {
        self.entities.iter().position(|s| s.id == entity)
    }

    /// Borrows `Blackboard` for `entity`.
    pub fn blackboard_mut(&mut self, entity: EntityId) -> Option<&mut Blackboard> {
        let i = self.index(entity)?;
        Some(&mut self.entities[i].bb)
    }

    /// Borrows `DatabaseRow` for `entity`.
    pub fn database_row(&self, entity: EntityId) -> Option<&DatabaseRow> {
        let i = self.index(entity)?;
        self.entities[i].row.as_ref()
    }

    /// Borrows `BlackboardBindings` for `entity`.
    pub fn bindings(&self, entity: EntityId) -> Option<&BlackboardBindings> {
        let i = self.index(entity)?;
        self.entities[i].bindings.as_ref()
    }

    /// Borrows `AiTableCache` for `entity`.
    pub fn cache_mut(&mut self, entity: EntityId) -> Option<&mut AiTableCache> {
        self.caches.get_mut(entity)
    }
}
