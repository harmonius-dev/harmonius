//! Sparse component storage for headless integration tests.

use std::collections::HashMap;

use crate::model::{
    Collider, CompoundCollider, Entity, PhysicsMaterialHandle, RigidBody, SleepTimer, Sleeping,
};

#[derive(Clone, Debug, Default)]
struct EntityRecord {
    collider: Option<Collider>,
    compound: Option<CompoundCollider>,
    material: PhysicsMaterialHandle,
    rigid_body: Option<RigidBody>,
    sleeping: Option<Sleeping>,
    sleep_timer: SleepTimer,
}

/// Minimal ECS-style world backing editor command tests.
#[derive(Debug, Default)]
pub struct World {
    entities: HashMap<Entity, EntityRecord>,
    /// Restitution keyed by `PhysicsMaterialHandle.0` (TC-IR-5.4.7.1).
    pub material_restitution: HashMap<u32, f32>,
    /// Problem-panel style strings (deterministic harness only).
    pub diagnostics: Vec<String>,
}

impl World {
    /// Creates an empty world.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts or replaces the `Collider` component.
    pub fn insert_collider(&mut self, entity: Entity, collider: Collider) {
        let r = self.entities.entry(entity).or_default();
        r.collider = Some(collider);
    }

    /// Inserts or replaces the `CompoundCollider` component.
    pub fn insert_compound(&mut self, entity: Entity, compound: CompoundCollider) {
        let r = self.entities.entry(entity).or_default();
        r.compound = Some(compound);
    }

    /// Sets rigid-body marker.
    pub fn insert_rigid_body(&mut self, entity: Entity) {
        self.entities.entry(entity).or_default().rigid_body = Some(RigidBody);
    }

    /// Adds sleeping marker (and optional timer) for debug coloring tests.
    pub fn insert_sleeping(&mut self, entity: Entity) {
        let r = self.entities.entry(entity).or_default();
        r.sleeping = Some(Sleeping);
    }

    /// Sets `SleepTimer` ticks (test harness for TC-IR-5.4.N7).
    pub fn set_sleep_timer_ticks(&mut self, entity: Entity, ticks: u32) {
        if let Some(r) = self.entities.get_mut(&entity) {
            r.sleep_timer.ticks = ticks;
        }
    }

    /// Borrows `Collider` mutably when present.
    pub fn collider_mut(&mut self, entity: Entity) -> Option<&mut Collider> {
        self.entities.get_mut(&entity)?.collider.as_mut()
    }

    /// Borrows `CompoundCollider` mutably when present.
    pub fn compound_mut(&mut self, entity: Entity) -> Option<&mut CompoundCollider> {
        self.entities.get_mut(&entity)?.compound.as_mut()
    }

    /// Reads `Collider` when present.
    #[must_use]
    pub fn collider(&self, entity: Entity) -> Option<&Collider> {
        self.entities.get(&entity)?.collider.as_ref()
    }

    /// Reads `CompoundCollider` when present.
    #[must_use]
    pub fn compound(&self, entity: Entity) -> Option<&CompoundCollider> {
        self.entities.get(&entity)?.compound.as_ref()
    }

    /// Writes physics material handle (creates record if missing).
    pub fn set_physics_material(&mut self, entity: Entity, material: PhysicsMaterialHandle) {
        self.entities.entry(entity).or_default().material = material;
    }

    /// Reads physics material handle (default `0` when missing).
    #[must_use]
    pub fn physics_material(&self, entity: Entity) -> PhysicsMaterialHandle {
        self.entities
            .get(&entity)
            .map(|r| r.material)
            .unwrap_or(PhysicsMaterialHandle(0))
    }

    /// True when entity has `Sleeping` marker.
    #[must_use]
    pub fn is_sleeping(&self, entity: Entity) -> bool {
        self.entities
            .get(&entity)
            .is_some_and(|r| r.sleeping.is_some())
    }

    /// Returns sleep timer ticks (0 when absent).
    #[must_use]
    pub fn sleep_timer_ticks(&self, entity: Entity) -> u32 {
        self.entities
            .get(&entity)
            .map(|r| r.sleep_timer.ticks)
            .unwrap_or(0)
    }

    /// Editor selection path: force-wake sleeping rigid bodies.
    pub fn editor_select_wake(&mut self, entity: Entity) {
        let Some(r) = self.entities.get_mut(&entity) else {
            return;
        };
        if r.sleeping.take().is_some() {
            r.sleep_timer = SleepTimer::default();
        }
    }

    /// Records a diagnostic line (degenerate hull, layer warnings, etc.).
    pub fn log_diagnostic(&mut self, message: impl Into<String>) {
        self.diagnostics.push(message.into());
    }

    /// Stores `PhysicsMaterial` scalar for a handle (harness asset table).
    pub fn register_material_restitution(
        &mut self,
        handle: PhysicsMaterialHandle,
        restitution: f32,
    ) {
        self.material_restitution.insert(handle.0, restitution);
    }

    /// Restitution for the entity's assigned material (0 when unknown).
    #[must_use]
    pub fn restitution_for_entity(&self, entity: Entity) -> f32 {
        let h = self.physics_material(entity);
        self.material_restitution.get(&h.0).copied().unwrap_or(0.0)
    }
}
