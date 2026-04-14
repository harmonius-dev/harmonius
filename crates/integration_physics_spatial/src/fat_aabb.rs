//! Fat AABB expansion used to cover intra-frame motion.

use crate::aabb::Aabb;
use crate::math::Vec3;

/// Expands an AABB along the velocity vector to cover motion over `dt_frame`.
///
/// The expansion adds `|velocity| * dt_frame` in each axis direction plus a fixed margin per axis.
#[must_use]
pub fn fatten_aabb_for_velocity(bounds: Aabb, velocity: Vec3, dt_frame: f32, margin: f32) -> Aabb {
    let expand = velocity.mul_scalar(dt_frame);
    let absx = expand.x.abs() + margin;
    let absy = expand.y.abs() + margin;
    let absz = expand.z.abs() + margin;

    Aabb {
        min: Vec3::new(
            bounds.min.x - absx,
            bounds.min.y - absy,
            bounds.min.z - absz,
        ),
        max: Vec3::new(
            bounds.max.x + absx,
            bounds.max.y + absy,
            bounds.max.z + absz,
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tc_ir_3_9_5_u1_fat_aabb_velocity_expand() {
        let base = Aabb {
            min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };
        let velocity = Vec3::new(5.0, 0.0, 0.0);
        let dt_frame = 1.0 / 60.0;
        let margin = 0.1;
        let fat = fatten_aabb_for_velocity(base, velocity, dt_frame, margin);
        let expected = 5.0 * dt_frame + margin;
        assert!((fat.max.x - base.max.x - expected).abs() < 1.0e-4);
    }
}
