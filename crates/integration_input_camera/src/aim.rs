//! Aim assist deflection for look deltas.

use crate::components::Entity;
use glam::{Vec2, Vec3};

/// Aim assist configuration (mirrors input subsystem fields used by integration).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AimAssistConfig {
    /// Radius for target magnetism (world units).
    pub magnetism_radius: f32,
    /// Strength of pull toward target (0..1).
    pub magnetism_strength: f32,
    /// Radius for friction slowdown near targets.
    pub friction_radius: f32,
    /// Look speed multiplier when inside friction.
    pub friction_multiplier: f32,
    /// Whether snap-to-target is enabled.
    pub snap_enabled: bool,
    /// Snap activation radius.
    pub snap_radius: f32,
    /// Master enable/disable for aim assist.
    pub enabled: bool,
}

/// Mutable aim assist state tracked per entity.
#[derive(Clone, Debug, PartialEq)]
pub struct AimAssistState {
    /// Currently locked target entity, if any.
    pub sticky_target: Option<Entity>,
    /// Time since last target acquisition.
    pub lock_timer: f32,
}

impl Default for AimAssistState {
    fn default() -> Self {
        Self {
            sticky_target: None,
            lock_timer: 0.0,
        }
    }
}

/// Deflect a 2D look delta toward the closest target within `magnetism_radius`.
///
/// `camera_forward` is a unit vector in world space; targets are world positions.
/// When `config.enabled` is false or no targets qualify, returns `delta` unchanged.
pub fn aim_deflect_look_delta(
    config: &AimAssistConfig,
    state: &mut AimAssistState,
    camera_position: Vec3,
    camera_forward: Vec3,
    targets: &[(Entity, Vec3)],
    delta: Vec2,
) -> Vec2 {
    if !config.enabled || targets.is_empty() {
        return delta;
    }

    let mut best: Option<(Entity, Vec3, f32)> = None;
    for (e, p) in targets {
        let d = *p - camera_position;
        let dist = d.length();
        if dist <= config.magnetism_radius
            && dist > 1e-6
            && best.is_none_or(|(_, _, bd)| dist < bd)
        {
            best = Some((*e, *p, dist));
        }
    }

    let Some((entity, target_pos, dist)) = best else {
        state.sticky_target = None;
        return delta;
    };

    state.sticky_target = Some(entity);

    let friction_scale = if dist < config.friction_radius {
        config.friction_multiplier
    } else {
        1.0
    };

    let to_target = (target_pos - camera_position).normalize_or_zero();
    let world_up = Vec3::Y;
    let right = camera_forward.cross(world_up).normalize_or_zero();
    let up = right.cross(camera_forward).normalize_or_zero();
    let desired = Vec2::new(to_target.dot(right), to_target.dot(up));
    if desired.length_squared() <= 1e-12 {
        return delta;
    }
    let desired_n = desired.normalize();
    let delta_n = if delta.length_squared() <= 1e-12 {
        Vec2::ZERO
    } else {
        delta.normalize()
    };
    let blended = (delta_n * (1.0 - config.magnetism_strength)
        + desired_n * config.magnetism_strength)
        .normalize_or_zero();
    let mut out = blended * delta.length() * friction_scale;
    if config.snap_enabled && dist < config.snap_radius {
        out = desired_n * delta.length() * friction_scale;
    }
    out
}
