//! Physics parameter sync driven by attribute aggregates (IR-2.6.1–IR-2.6.4).

use glam::Vec3;

use crate::surface::SurfaceType;

/// Per-entity gravity multiplier written when `gravity_scale != 1.0`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GravityOverride {
    /// Multiplier applied to global gravity for integration.
    pub scale: f32,
}

/// Per-entity Coulomb friction coefficients overriding the base material.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FrictionOverride {
    /// Static friction coefficient μ_s (clamped design range).
    pub static_friction: f32,
    /// Dynamic friction coefficient μ_d (clamped design range).
    pub dynamic_friction: f32,
}

/// Authoring-time physics material (subset used by integration tests).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PhysicsMaterial {
    /// Static friction from asset data.
    pub static_friction: f32,
    /// Dynamic friction from asset data.
    pub dynamic_friction: f32,
    /// Restitution (unused by this integration slice).
    pub restitution: f32,
    /// Density (unused by this integration slice).
    pub density: f32,
    /// Surface tag used for collision-driven effects.
    pub surface_type: SurfaceType,
}

/// Mass snapshot for tests that relate inverse mass to gameplay mass.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MassSample {
    /// Current inverse mass after attribute sync.
    pub inverse: f32,
    /// Inverse mass before modifiers.
    pub base_inverse: f32,
}

impl MassSample {
    /// Effective mass `1 / inverse`.
    #[must_use]
    pub fn mass(self) -> f32 {
        1.0 / self.inverse
    }
}

/// Clamps gravity per failure mode #1 (`TC-IR-2.6.1.N1`).
#[must_use]
pub fn clamp_gravity_scale(scale: f32) -> f32 {
    if scale <= 0.0 {
        0.01
    } else {
        scale
    }
}

/// Clamps mass scale per failure mode #2 (`TC-IR-2.6.2.N1`).
#[must_use]
pub fn clamp_mass_scale(scale: f32) -> f32 {
    if scale <= 0.0 {
        0.001
    } else {
        scale
    }
}

/// Clamps friction scale per failure mode #5 (`TC-IR-2.6.3.N1`).
#[must_use]
pub fn clamp_friction_scale(scale: f32) -> f32 {
    if scale < 0.0 {
        0.0
    } else {
        scale
    }
}

fn near_one(scale: f32) -> bool {
    (scale - 1.0).abs() < 1e-5
}

/// Decides the [`GravityOverride`] component from an evaluated `gravity_scale`.
///
/// `TC-IR-2.6.1.U1`, `TC-IR-2.6.1.N1`, IR-2.6.1.
#[must_use]
pub fn gravity_override_from_scale(gravity_scale: f32) -> Option<GravityOverride> {
    if near_one(gravity_scale) {
        return None;
    }
    let scale = clamp_gravity_scale(gravity_scale);
    Some(GravityOverride { scale })
}

/// Computes inverse mass after `mass_scale` (`TC-IR-2.6.2.U1`, `TC-IR-2.6.2.U2`, IR-2.6.2).
#[must_use]
pub fn mass_inverse_from_scales(base_inverse: f32, mass_scale: f32) -> f32 {
    let scale = clamp_mass_scale(mass_scale);
    base_inverse / scale
}

/// Builds a [`MassSample`] for tests comparing gameplay mass.
#[must_use]
pub fn mass_sample_after_scale(base_inverse: f32, mass_scale: f32) -> MassSample {
    MassSample {
        base_inverse,
        inverse: mass_inverse_from_scales(base_inverse, mass_scale),
    }
}

fn clamp_mu(mu: f32) -> f32 {
    mu.clamp(0.0, 1.0)
}

/// Builds [`FrictionOverride`] from base material coefficients and `friction_scale`.
///
/// Returns `None` when `friction_scale` is ~1.0 (solver uses base asset).  
/// `TC-IR-2.6.3.U1`, `TC-IR-2.6.3.U2`, `TC-IR-2.6.3.N1`, IR-2.6.3.
#[must_use]
pub fn friction_override_from_material(
    material: &PhysicsMaterial,
    friction_scale: f32,
) -> Option<FrictionOverride> {
    if near_one(friction_scale) {
        return None;
    }
    let scale = clamp_friction_scale(friction_scale);
    let sf = clamp_mu(material.static_friction * scale);
    let df = clamp_mu(material.dynamic_friction * scale);
    Some(FrictionOverride {
        static_friction: sf,
        dynamic_friction: df,
    })
}

/// Scales global gravity using an optional override (`TC-IR-2.6.1.1`–`TC-IR-2.6.1.3`).
#[must_use]
pub fn scale_gravity_force(global_g: Vec3, gravity_scale: f32) -> Vec3 {
    let s = if near_one(gravity_scale) {
        1.0
    } else {
        clamp_gravity_scale(gravity_scale)
    };
    global_g * s
}

/// Scales knockback-style impulses from strength-like attributes.
///
/// Companion `TC-IR-2.6.4.1` maps `strength = 50` to `1.5×` base impulse; `TC-IR-2.6.4.2` maps zero
/// strength to zero force using the same linear rule (`factor = strength * 0.03`).
#[must_use]
pub fn scale_knockback_force(base: Vec3, strength_current: f32) -> Vec3 {
    base * (strength_current * 0.03)
}

/// Explains why [`PhysicsMaterial`] was not available for friction sync (`TC-IR-2.6.3.N2`, `N3`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialLookup<'a> {
    /// No material handle on the entity.
    MissingHandle,
    /// Handle present but the asset store has no resident asset yet.
    Unloaded,
    /// Resolved material asset.
    Ready(&'a PhysicsMaterial),
}

/// Resolves friction overrides while counting first-hit warnings separately per failure mode.
#[must_use]
pub fn friction_override_from_lookup(
    lookup: MaterialLookup<'_>,
    friction_scale: f32,
    missing_handle_warns: &mut u32,
    unloaded_warns: &mut u32,
) -> Option<FrictionOverride> {
    match lookup {
        MaterialLookup::MissingHandle => {
            if *missing_handle_warns == 0 {
                *missing_handle_warns += 1;
            }
            None
        }
        MaterialLookup::Unloaded => {
            if *unloaded_warns == 0 {
                *unloaded_warns += 1;
            }
            None
        }
        MaterialLookup::Ready(m) => friction_override_from_material(m, friction_scale),
    }
}
