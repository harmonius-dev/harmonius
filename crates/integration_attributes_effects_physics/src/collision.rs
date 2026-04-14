//! Collision surface → effect application (`TC-IR-2.6.5.*`).

use crate::active_effects::ActiveEffects;
use crate::registry::{EffectDefinition, RowRef, TableRegistry};
use crate::surface::{SurfaceEffectMap, SurfaceType};
use crate::sync::PhysicsMaterial;

/// One diagnostic line for debug toggles (`log_surface_effect_hits`).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollisionEffectLogEntry {
    /// Surface that produced a hit.
    pub surface: SurfaceType,
    /// Row resolved from [`SurfaceEffectMap`].
    pub row: RowRef,
    /// Effect id applied.
    pub effect_id: u32,
}

/// Append-only log used by tests in place of ECS diagnostics resources.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CollisionEffectLog {
    entries: Vec<CollisionEffectLogEntry>,
}

impl CollisionEffectLog {
    /// Empty log.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Recorded hits.
    #[must_use]
    pub fn entries(&self) -> &[CollisionEffectLogEntry] {
        &self.entries
    }

    fn push(&mut self, surface: SurfaceType, row: RowRef, effect_id: u32) {
        self.entries.push(CollisionEffectLogEntry {
            surface,
            row,
            effect_id,
        });
    }
}

/// Applies a surface-driven effect after a collision contact (`IR-2.6.5`).
///
/// - Missing material → silent skip (`TC-IR-2.6.5.N3`).
/// - Unmapped surface → silent skip (`TC-IR-2.6.5.N1`).
/// - Missing `ActiveEffects` → skip; increments `missing_active_warn_counter` once (`TC-IR-2.6.5.N2`).
#[must_use]
pub fn apply_surface_collision_effect(
    material: Option<&PhysicsMaterial>,
    surface_map: &SurfaceEffectMap,
    registry: &TableRegistry,
    active_effects: Option<&mut ActiveEffects>,
    effect_priority: i32,
    log: &mut CollisionEffectLog,
    missing_active_warn_counter: &mut u32,
) -> bool {
    let Some(mat) = material else {
        return false;
    };
    let Some(row) = surface_map.get(mat.surface_type) else {
        return false;
    };
    let Some(def) = registry.effect(row) else {
        return false;
    };
    let Some(active) = active_effects else {
        if *missing_active_warn_counter == 0 {
            *missing_active_warn_counter += 1;
        }
        return false;
    };
    active.apply(effect_priority, def.id);
    log.push(mat.surface_type, row, def.id);
    true
}

/// Resolves [`EffectDefinition`] for a surface without mutating `ActiveEffects` (`TC-IR-2.6.5.U1`).
#[must_use]
pub fn resolve_surface_effect_row<'a>(
    surface: SurfaceType,
    surface_map: &'a SurfaceEffectMap,
    registry: &'a TableRegistry,
) -> Option<&'a EffectDefinition> {
    let row = surface_map.get(surface)?;
    registry.effect(row)
}
