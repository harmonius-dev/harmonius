//! Spring arm, deoccluder, and decollider helpers.

use glam::Vec3;

use crate::ids::LayerMask;

/// Spring arm configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpringArm {
    /// Natural arm distance (meters).
    pub target_length: f32,
    /// Collision sphere radius (meters).
    pub probe_size: f32,
    /// Collision layer mask.
    pub probe_channel: LayerMask,
}

/// Deoccluder configuration.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraDeoccluder {
    /// Minimum distance from obstacles.
    pub camera_radius: f32,
    /// Ignore obstructions shorter than this duration (seconds).
    pub min_occlusion_time: f32,
    /// Obstacle layer mask.
    pub obstacle_layers: LayerMask,
}

/// Deocclusion strategy (engine parity; only pull-forward is evaluated today).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeocclusionStrategy {
    PullForward,
    /// Climb vertically before advancing (reserved).
    PreserveHeight,
    /// Maintain arm length while clearing hits (reserved).
    PreserveDistance,
}

/// Geometry penetration prevention.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraDecollider {
    /// Minimum distance from geometry.
    pub camera_radius: f32,
    /// Obstacle layer mask.
    pub obstacle_layers: LayerMask,
    /// Terrain layer mask.
    pub terrain_layers: LayerMask,
}

/// Computes effective spring arm length after a forward hit.
#[must_use]
pub fn apply_spring_arm_length(arm: &SpringArm, hit_distance: Option<f32>) -> f32 {
    match hit_distance {
        None => arm.target_length,
        Some(hit) => (hit - arm.probe_size).min(arm.target_length).max(0.0),
    }
}

/// Pulls the camera forward when occlusion persists longer than `min_occlusion_time`.
#[must_use]
pub fn apply_deoccluder_pull_forward(
    deoc: &CameraDeoccluder,
    strategy: DeocclusionStrategy,
    occlusion_time: f32,
    forward_pull: f32,
) -> f32 {
    match strategy {
        DeocclusionStrategy::PullForward => {
            if occlusion_time >= deoc.min_occlusion_time {
                forward_pull.max(0.0)
            } else {
                0.0
            }
        }
        DeocclusionStrategy::PreserveHeight | DeocclusionStrategy::PreserveDistance => 0.0,
    }
}

/// Pushes the camera above the terrain surface when penetrating.
#[must_use]
pub fn apply_camera_decollider(
    decollider: &CameraDecollider,
    camera_position: Vec3,
    terrain_height: f32,
) -> Vec3 {
    let min_y = terrain_height + decollider.camera_radius;
    if camera_position.y < min_y {
        Vec3::new(camera_position.x, min_y, camera_position.z)
    } else {
        camera_position
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// TC-13.25.13.1 — spring arm shortens to the hit distance.
    #[test]
    fn tc_13_25_13_1_spring_arm_retraction() {
        let arm = SpringArm {
            target_length: 5.0,
            probe_size: 0.0,
            probe_channel: LayerMask(1),
        };
        let len = apply_spring_arm_length(&arm, Some(3.0));
        assert!((len - 3.0).abs() < 1e-3);
    }

    /// TC-13.25.13.2 — removing the obstacle restores full arm length.
    #[test]
    fn tc_13_25_13_2_spring_arm_extension() {
        let arm = SpringArm {
            target_length: 5.0,
            probe_size: 0.0,
            probe_channel: LayerMask(1),
        };
        let retracted = apply_spring_arm_length(&arm, Some(3.0));
        assert!(retracted < arm.target_length);
        let extended = apply_spring_arm_length(&arm, None);
        assert!((extended - arm.target_length).abs() < 1e-3);
    }

    /// TC-13.25.14.1 — pull-forward activates after the minimum occlusion time.
    #[test]
    fn tc_13_25_14_1_deoccluder_pull_forward() {
        let deoc = CameraDeoccluder {
            camera_radius: 0.2,
            min_occlusion_time: 0.1,
            obstacle_layers: LayerMask(1),
        };
        let pull = apply_deoccluder_pull_forward(&deoc, DeocclusionStrategy::PullForward, 0.2, 0.5);
        assert!((pull - 0.5).abs() < 1e-3);
    }

    /// TC-13.25.14.2 — short occlusions are ignored.
    #[test]
    fn tc_13_25_14_2_deoccluder_min_time() {
        let deoc = CameraDeoccluder {
            camera_radius: 0.2,
            min_occlusion_time: 0.1,
            obstacle_layers: LayerMask(1),
        };
        let pull =
            apply_deoccluder_pull_forward(&deoc, DeocclusionStrategy::PullForward, 0.05, 0.5);
        assert_eq!(pull, 0.0);
    }

    /// TC-13.25.15.1 — decollider lifts the camera above terrain.
    #[test]
    fn tc_13_25_15_1_decollider_terrain() {
        let decollider = CameraDecollider {
            camera_radius: 0.25,
            obstacle_layers: LayerMask(1),
            terrain_layers: LayerMask(2),
        };
        let cam = Vec3::new(0.0, 0.0, 0.0);
        let fixed = apply_camera_decollider(&decollider, cam, 1.0);
        assert!(fixed.y >= 1.0 + decollider.camera_radius - 1e-3);
    }
}
