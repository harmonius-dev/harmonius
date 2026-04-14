//! Attributes/effects ↔ physics integration helpers (design `attributes-effects-physics`).
//!
//! Pure transforms and small runtime structs back the IR-2.6.x behaviors described in the
//! Harmonius integration design companion test-case table (`TC-IR-2.6.*`).

#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod active_effects;
mod collision;
mod registry;
mod surface;
mod sync;
mod throughput;

pub use active_effects::{ActiveEffectEntry, ActiveEffects};
pub use collision::{
    apply_surface_collision_effect, resolve_surface_effect_row, CollisionEffectLog,
    CollisionEffectLogEntry,
};
pub use registry::{EffectDefinition, RowRef, TableRegistry};
pub use surface::{SurfaceEffectMap, SurfaceType};
pub use sync::{
    clamp_friction_scale, clamp_gravity_scale, clamp_mass_scale, friction_override_from_lookup,
    friction_override_from_material, gravity_override_from_scale, mass_inverse_from_scales,
    mass_sample_after_scale, scale_gravity_force, scale_knockback_force, FrictionOverride,
    GravityOverride, MassSample, MaterialLookup, PhysicsMaterial,
};
pub use throughput::{
    bench_friction_batch, bench_gravity_force_batch, bench_gravity_override_churn,
    bench_mass_inverse_batch, idle_sync_touch_count, sync_gravity_batch,
};
