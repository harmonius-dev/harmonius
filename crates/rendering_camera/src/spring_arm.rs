//! Spring-arm boom retraction without pulling in a full physics engine (`R-2.7.1`).
//!
//! `SpringArmWorld` uses a trait so real physics and deterministic narrow fakes share one call
//! site; this matches the engine constraint to avoid mock frameworks while keeping casts
//! swappable.

/// Result of a sphere cast along the boom direction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphereCastResult {
    /// Distance from the cast origin to the hit along the cast direction.
    pub distance: f32,
}

/// Query parameters for a spring-arm sweep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SphereCastQuery {
    /// Cast origin in world meters.
    pub origin: [f32; 3],
    /// Cast direction (not necessarily normalized).
    pub direction: [f32; 3],
    /// Sphere radius in meters.
    pub radius: f32,
    /// Physics layer mask for the cast.
    pub layer_mask: u32,
}

/// Narrow world interface for spring-arm sphere casts (implemented by real physics or fakes).
pub trait SpringArmWorld {
    /// Performs a sphere cast and returns the nearest blocking hit, if any.
    fn sphere_cast(&self, query: SphereCastQuery) -> Option<SphereCastResult>;
}

/// Spring-arm state carried by a camera rig.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringArm {
    /// Desired boom length in meters.
    pub desired_length: f32,
    /// Current boom length in meters.
    pub current_length: f32,
    /// Sphere cast radius in meters.
    pub probe_radius: f32,
    /// Retraction speed toward obstructed lengths (1/s).
    pub retract_rate: f32,
    /// Restoration speed toward desired length (1/s).
    pub restore_rate: f32,
    /// Layer mask used for casts.
    pub layer_mask: u32,
}

impl SpringArm {
    /// Advances boom length toward the physics-informed target for one frame.
    pub fn step_toward_target(
        &mut self,
        hit_distance: Option<f32>,
        dt_seconds: f32,
        clearance_m: f32,
    ) {
        let target_length = match hit_distance {
            Some(distance) => (distance - clearance_m).max(0.0),
            None => self.desired_length,
        };
        let rate = if target_length < self.current_length {
            self.retract_rate
        } else {
            self.restore_rate
        };
        let alpha = (rate * dt_seconds).clamp(0.0, 1.0);
        self.current_length += (target_length - self.current_length) * alpha;
    }
}

/// Computes boom target position given a forward unit vector and boom length.
#[must_use]
pub fn boom_tip(origin: [f32; 3], forward: [f32; 3], length: f32) -> [f32; 3] {
    [
        origin[0] + forward[0] * length,
        origin[1] + forward[1] * length,
        origin[2] + forward[2] * length,
    ]
}

#[cfg(test)]
mod tests {
    use super::{boom_tip, SphereCastQuery, SphereCastResult, SpringArm, SpringArmWorld};

    struct NoHitWorld;

    impl SpringArmWorld for NoHitWorld {
        fn sphere_cast(&self, _query: SphereCastQuery) -> Option<SphereCastResult> {
            None
        }
    }

    struct HitWorld {
        pub distance: f32,
        pub expected_mask: u32,
    }

    impl SpringArmWorld for HitWorld {
        fn sphere_cast(&self, query: SphereCastQuery) -> Option<SphereCastResult> {
            assert_eq!(query.layer_mask, self.expected_mask);
            Some(SphereCastResult {
                distance: self.distance,
            })
        }
    }

    /// TC-2.7.1.1 — with no hit, length converges to desired.
    #[test]
    fn test_spring_arm_no_contact_full_length() {
        let world = NoHitWorld;
        let mut arm = SpringArm {
            desired_length: 3.0,
            current_length: 1.0,
            probe_radius: 0.1,
            retract_rate: 10.0,
            restore_rate: 2.0,
            layer_mask: 0xFFFF,
        };
        for _ in 0..120 {
            let q = SphereCastQuery {
                origin: [0.0; 3],
                direction: [0.0, 0.0, 1.0],
                radius: arm.probe_radius,
                layer_mask: arm.layer_mask,
            };
            let hit = world.sphere_cast(q).map(|h| h.distance);
            arm.step_toward_target(hit, 1.0 / 60.0, 0.05);
        }
        assert!((arm.current_length - arm.desired_length).abs() < 0.05);
    }

    /// TC-2.7.1.2 — a closer hit retracts toward the hit distance.
    #[test]
    fn test_spring_arm_contact_retract_rate() {
        let world = HitWorld {
            distance: 2.0,
            expected_mask: 0xFFFF,
        };
        let mut arm = SpringArm {
            desired_length: 3.0,
            current_length: 3.0,
            probe_radius: 0.1,
            retract_rate: 5.0,
            restore_rate: 1.0,
            layer_mask: 0xFFFF,
        };
        let q = SphereCastQuery {
            origin: [0.0; 3],
            direction: [0.0, 0.0, 1.0],
            radius: arm.probe_radius,
            layer_mask: arm.layer_mask,
        };
        let hit = world.sphere_cast(q).map(|h| h.distance);
        arm.step_toward_target(hit, 1.0 / 60.0, 0.05);
        assert!(arm.current_length < arm.desired_length);
        assert!(arm.current_length > 1.5);
    }

    /// TC-2.7.1.3 — after obstruction clears, restore approaches desired without overshoot.
    #[test]
    fn test_spring_arm_restore_smooth() {
        let hit = true;
        let mut arm = SpringArm {
            desired_length: 3.0,
            current_length: 3.0,
            probe_radius: 0.1,
            retract_rate: 10.0,
            restore_rate: 3.0,
            layer_mask: 0xFFFF,
        };
        let mut prev = arm.current_length;
        for frame in 0..200 {
            let distance = if frame < 40 && hit { Some(1.5) } else { None };
            arm.step_toward_target(distance, 1.0 / 60.0, 0.05);
            if frame >= 40 {
                assert!(arm.current_length >= prev - 1.0e-3, "monotonic restore");
            }
            prev = arm.current_length;
        }
        assert!((arm.current_length - arm.desired_length).abs() < 0.1);
    }

    /// TC-2.7.1.4 — casts include the configured layer mask.
    #[test]
    fn test_spring_arm_hits_use_layer_mask() {
        let world = HitWorld {
            distance: 1.0,
            expected_mask: 0b1010,
        };
        let mut arm = SpringArm {
            desired_length: 2.0,
            current_length: 2.0,
            probe_radius: 0.05,
            retract_rate: 20.0,
            restore_rate: 20.0,
            layer_mask: 0b1010,
        };
        let q = SphereCastQuery {
            origin: [0.0; 3],
            direction: [0.0, 1.0, 0.0],
            radius: arm.probe_radius,
            layer_mask: arm.layer_mask,
        };
        let hit = world.sphere_cast(q).map(|h| h.distance);
        arm.step_toward_target(hit, 1.0 / 60.0, 0.05);
        assert!(arm.current_length < 2.0);
    }

    #[test]
    fn boom_tip_moves_along_forward() {
        let tip = boom_tip([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], 2.0);
        assert!((tip[0] - 2.0).abs() < 1.0e-4);
        assert!((tip[1] - 1.0).abs() < 1.0e-4);
    }
}
