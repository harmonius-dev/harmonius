//! Fixed simulation to variable render interpolation helpers.

use glam::Vec3;

use crate::recomposer::Recomposer;

/// Interpolates authoritative simulation samples for render evaluation.
///
/// When `had_fixed_tick` is false, the previous render snapshot is reused so
/// variable frames without a completed fixed step do not hitch.
#[must_use]
pub fn interpolate_recomposer_overrides(
    previous_render: &Recomposer,
    latest_sim: &Recomposer,
    alpha: f32,
    had_fixed_tick: bool,
) -> Recomposer {
    if !had_fixed_tick {
        return *previous_render;
    }
    let alpha = alpha.clamp(0.0, 1.0);
    Recomposer {
        position_offset: previous_render
            .position_offset
            .lerp(latest_sim.position_offset, alpha),
        rotation_offset: lerp_euler_degrees(
            previous_render.rotation_offset,
            latest_sim.rotation_offset,
            alpha,
        ),
        fov_delta: lerp_scalar(previous_render.fov_delta, latest_sim.fov_delta, alpha),
        dutch: lerp_scalar(previous_render.dutch, latest_sim.dutch, alpha),
        blend_weight: lerp_scalar(
            previous_render.blend_weight,
            latest_sim.blend_weight,
            alpha,
        ),
    }
}

fn lerp_euler_degrees(from: Vec3, to: Vec3, alpha: f32) -> Vec3 {
    from.lerp(to, alpha)
}

fn lerp_scalar(from: f32, to: f32, alpha: f32) -> f32 {
    from + (to - from) * alpha
}
