//! Sub-emitter spawn rules (`TC-11.1.5.*`).

use glam::Vec3;

/// Spawns child burst at parent death position (returns local offset 0 for this helper).
#[must_use]
pub fn sub_emit_spawn_position_on_death(parent_world: Vec3) -> Vec3 {
    parent_world
}

/// Spawns child burst at collision contact position.
#[must_use]
pub fn sub_emit_spawn_position_on_collision(contact: Vec3) -> Vec3 {
    contact
}

/// Applies inherit factor to parent velocity for child initial velocity.
#[must_use]
pub fn sub_emit_inherit_velocity(parent_velocity: Vec3, inherit_factor: f32) -> Vec3 {
    parent_velocity * inherit_factor
}

/// Maximum sub-emitter recursion depth by platform tier (`TC-11.1.5.4`).
#[must_use]
pub fn sub_emitter_max_depth(mobile_tier: bool) -> u32 {
    if mobile_tier {
        1
    } else {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `TC-11.1.5.1` — children at parent death position.
    #[test]
    fn tc_11_1_5_1_sub_emitter_death() {
        let p = Vec3::new(3.0, 2.0, 1.0);
        assert_eq!(sub_emit_spawn_position_on_death(p), p);
    }

    /// `TC-11.1.5.2` — children at collision contact.
    #[test]
    fn tc_11_1_5_2_sub_emitter_collision() {
        let c = Vec3::new(0.0, 1.0, 0.0);
        assert_eq!(sub_emit_spawn_position_on_collision(c), c);
    }

    /// `TC-11.1.5.3` — inherit half of parent velocity.
    #[test]
    fn tc_11_1_5_3_sub_emitter_inherit_velocity() {
        let v = Vec3::new(10.0, 0.0, 0.0);
        let out = sub_emit_inherit_velocity(v, 0.5);
        assert_eq!(out, Vec3::new(5.0, 0.0, 0.0));
    }

    /// `TC-11.1.5.4` — depth limits per platform.
    #[test]
    fn tc_11_1_5_4_sub_emitter_depth_limit() {
        assert_eq!(sub_emitter_max_depth(true), 1);
        assert_eq!(sub_emitter_max_depth(false), 4);
    }
}
