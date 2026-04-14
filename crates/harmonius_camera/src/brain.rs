//! Camera brain selection and update cadence helpers.

use crate::blend::BlendDefinition;
use crate::ids::Entity;
use crate::types::VirtualCamera;

/// Per-player camera brain configuration.
///
/// `custom_blends` stores a numeric asset-handle placeholder until the asset pipeline wires in
/// `blend::CustomBlends` data from disk.
#[derive(Clone, Debug, PartialEq)]
pub struct CameraBrain {
    /// Channel mask to filter virtual cameras.
    pub channel_mask: u32,
    /// Render layer bitmask forwarded from the active virtual camera.
    pub render_layers: u32,
    /// Update timing mode.
    pub update_mode: CameraUpdateMode,
    /// Default blend when no custom blend matches.
    pub default_blend: BlendDefinition,
    /// Optional asset handle placeholder for custom blends.
    pub custom_blends: Option<u64>,
}

/// Update cadence for a camera brain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CameraUpdateMode {
    /// After simulation, before render.
    LateUpdate,
    /// Synchronized with physics fixed timestep.
    FixedUpdate,
    /// Driven externally (replay, sequencer).
    Manual,
}

/// Selects the highest-priority virtual camera visible to `brain_channel_mask`.
///
/// Cameras must overlap the brain mask (`camera.channel_mask & brain_channel_mask != 0`). Ties on
/// priority prefer the higher underlying [`Entity`] id (tuple field `0`) for deterministic ordering.
#[must_use]
pub fn select_highest_priority_camera(
    brain_channel_mask: u32,
    cameras: &[(Entity, VirtualCamera)],
) -> Option<Entity> {
    cameras
        .iter()
        .filter(|(_, cam)| cam.channel_mask & brain_channel_mask != 0)
        .max_by(|(ea, a), (eb, b)| a.priority.cmp(&b.priority).then_with(|| ea.0.cmp(&eb.0)))
        .map(|(entity, _)| *entity)
}

/// Accumulator that emits fixed-timestep ticks for [`CameraUpdateMode::FixedUpdate`].
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FixedUpdateBrainClock {
    accumulator: f32,
}

impl FixedUpdateBrainClock {
    /// Creates a clock with zero accumulated time.
    pub const fn new() -> Self {
        Self { accumulator: 0.0 }
    }

    /// Adds `delta_time` and returns how many fixed steps occurred for `fixed_dt`.
    ///
    /// Emits at most one million steps per call so pathological frame times cannot stall the main
    /// thread; excess time remains in `accumulator` for subsequent frames (spiral-of-death guard).
    pub fn consume(&mut self, delta_time: f32, fixed_dt: f32) -> u32 {
        if fixed_dt <= 0.0 {
            return 0;
        }
        self.accumulator += delta_time;
        let mut steps = 0_u32;
        while self.accumulator >= fixed_dt {
            self.accumulator -= fixed_dt;
            steps += 1;
            if steps > 1_000_000 {
                break;
            }
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blend::{BlendCurve, BlendDefinition};

    fn sample_cameras() -> [(Entity, VirtualCamera); 3] {
        [
            (
                Entity(1),
                VirtualCamera {
                    priority: 1,
                    channel_mask: 0xFFFF,
                    render_layers: 1,
                    tracking_target: None,
                    look_at_target: None,
                },
            ),
            (
                Entity(2),
                VirtualCamera {
                    priority: 5,
                    channel_mask: 0xFFFF,
                    render_layers: 1,
                    tracking_target: None,
                    look_at_target: None,
                },
            ),
            (
                Entity(3),
                VirtualCamera {
                    priority: 10,
                    channel_mask: 0xFFFF,
                    render_layers: 1,
                    tracking_target: None,
                    look_at_target: None,
                },
            ),
        ]
    }

    /// TC-13.25.1.1 #1 — three cameras choose highest priority.
    #[test]
    fn tc_13_25_1_1_selects_highest_priority() {
        let cams = sample_cameras();
        let selected = select_highest_priority_camera(0x1, &cams);
        assert_eq!(selected, Some(Entity(3)));
    }

    /// TC-13.25.1.1 #2 — lowering the winner re-selects the next highest.
    #[test]
    fn tc_13_25_1_1_priority_change_reselects() {
        let mut cams = sample_cameras();
        cams[2].1.priority = 0;
        let selected = select_highest_priority_camera(0x1, &cams);
        assert_eq!(selected, Some(Entity(2)));
    }

    /// TC-13.25.2.1 — channel masks isolate cameras per brain.
    #[test]
    fn tc_13_25_2_1_channel_mask_filter() {
        let cams = [
            (
                Entity(10),
                VirtualCamera {
                    priority: 1,
                    channel_mask: 0x1,
                    render_layers: 1,
                    tracking_target: None,
                    look_at_target: None,
                },
            ),
            (
                Entity(11),
                VirtualCamera {
                    priority: 1,
                    channel_mask: 0x2,
                    render_layers: 1,
                    tracking_target: None,
                    look_at_target: None,
                },
            ),
        ];
        assert_eq!(select_highest_priority_camera(0x1, &cams), Some(Entity(10)));
        assert_eq!(select_highest_priority_camera(0x2, &cams), Some(Entity(11)));
    }

    /// TC-13.25.2.2 — fixed update cadence tracks the physics timestep.
    #[test]
    fn tc_13_25_2_2_fixed_update_syncs_with_physics_dt() {
        let mut clock = FixedUpdateBrainClock::new();
        assert_eq!(clock.consume(0.016, 0.05), 0);
        assert_eq!(clock.consume(0.040, 0.05), 1);
        assert_eq!(clock.consume(0.001, 0.05), 0);
    }

    #[test]
    fn brain_struct_round_trip() {
        let brain = CameraBrain {
            channel_mask: 0x1,
            render_layers: 0xFF,
            update_mode: CameraUpdateMode::FixedUpdate,
            default_blend: BlendDefinition {
                curve: BlendCurve::EaseInOut,
                duration: 0.25,
            },
            custom_blends: None,
        };
        assert_eq!(brain.update_mode, CameraUpdateMode::FixedUpdate);
    }
}
