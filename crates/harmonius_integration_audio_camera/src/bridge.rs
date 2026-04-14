//! Velocity derivation and per-frame listener synchronization.

use glam::{Quat, Vec3};
use log::warn;

use crate::command::{AudioCommand, AudioCommandSink, ListenerId};
use crate::prev::{ListenerPrevPositions, MAX_LOCAL_PLAYERS};

/// World-space forward axis for listener facing (right-handed, −Z forward).
pub const LISTENER_FORWARD: Vec3 = Vec3::NEG_Z;

/// Maximum listener velocity magnitude (m/s) after clamping teleports.
pub const MAX_LISTENER_VELOCITY: f32 = 100.0;

/// One camera brain worth of data consumed by the bridge (ECS-free for tests).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraListenerInput {
    /// Split-screen player index / listener slot.
    pub player_index: u8,
    /// Evaluated camera position in world space.
    pub position: Vec3,
    /// Evaluated camera orientation in world space.
    pub rotation: Quat,
}

/// Debug toggles matching `ListenerDebug` in `docs/design/integration/audio-camera.md`.
///
/// `draw_gizmo` is honored by engine debug-draw integration; this crate does not render.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ListenerDebug {
    /// When true, emit a debug log line per listener update.
    pub log_each_update: bool,
    /// When true, host code should draw listener gizmos (no-op inside this crate).
    pub draw_gizmo: bool,
}

/// Counts non-fatal drops while syncing one frame.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ListenerSyncMetrics {
    /// Positions rejected because any component was NaN.
    pub dropped_nan: u32,
    /// Commands not enqueued because the sink returned an error.
    pub dropped_queue_full: u32,
    /// Successfully enqueued listener updates.
    pub sent: u32,
}

/// Finite backward-difference listener velocity with clamp and non-positive `dt` guards.
#[must_use]
pub fn listener_velocity(prev_world: Option<Vec3>, current_world: Vec3, dt: f32) -> Vec3 {
    if !dt.is_finite() || dt <= 0.0 {
        return Vec3::ZERO;
    }
    let prev_pos = prev_world.unwrap_or(current_world);
    let raw = (current_world - prev_pos) / dt;
    raw.clamp_length_max(MAX_LISTENER_VELOCITY)
}

/// Pushes [`AudioCommand::UpdateListener`] for each unique `player_index` in slice order.
///
/// When multiple [`CameraListenerInput`] rows share the same `player_index`, only the first row
/// produces a command (deterministic “first in iteration order wins”).
///
/// `debug.draw_gizmo` is carried for API parity with the ECS design; only
/// `debug.log_each_update` affects behavior inside this helper.
pub fn camera_listener_sync_frame(
    inputs: &[CameraListenerInput],
    prev: &mut ListenerPrevPositions,
    delta_seconds: f32,
    debug: ListenerDebug,
    sink: &mut impl AudioCommandSink,
) -> ListenerSyncMetrics {
    let mut metrics = ListenerSyncMetrics::default();
    let mut seen = [false; MAX_LOCAL_PLAYERS];

    for input in inputs {
        let idx = input.player_index as usize;
        if idx >= MAX_LOCAL_PLAYERS {
            continue;
        }
        if seen[idx] {
            continue;
        }
        seen[idx] = true;

        if input.position.is_nan() {
            warn!(
                "NaN listener position; dropped update for player {}",
                input.player_index
            );
            metrics.dropped_nan += 1;
            continue;
        }

        let prev_sample = prev.get(input.player_index);
        let velocity = listener_velocity(prev_sample, input.position, delta_seconds);
        let cmd = AudioCommand::UpdateListener {
            listener_id: ListenerId(input.player_index),
            position: input.position,
            orientation: input.rotation,
            velocity,
        };

        if debug.log_each_update {
            log::debug!(
                "listener {} pos={:?} vel={:?}",
                input.player_index,
                input.position,
                velocity,
            );
        }

        match sink.try_send(cmd) {
            Ok(()) => {
                metrics.sent += 1;
            }
            Err(_) => {
                warn!(
                    "Audio MPSC queue full; dropped listener update for player {}",
                    input.player_index
                );
                metrics.dropped_queue_full += 1;
            }
        }
        // Match design: advance `prev` even when the sink is full so the next frame’s velocity
        // does not spike from a stale prior sample.
        prev.set(input.player_index, input.position);
    }

    metrics
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::AudioCommand;

    struct RecordingSink {
        commands: Vec<AudioCommand>,
        capacity: usize,
    }

    impl RecordingSink {
        fn new(capacity: usize) -> Self {
            Self {
                commands: Vec::new(),
                capacity,
            }
        }
    }

    impl AudioCommandSink for RecordingSink {
        fn try_send(&mut self, cmd: AudioCommand) -> Result<(), AudioCommand> {
            if self.commands.len() >= self.capacity {
                return Err(cmd);
            }
            self.commands.push(cmd);
            Ok(())
        }
    }

    #[test]
    fn tc_ir_1_7_2_u1_forward_axis_const() {
        assert_eq!(LISTENER_FORWARD, Vec3::NEG_Z);
    }

    #[test]
    fn tc_ir_1_7_2_1_identity_rot_forward() {
        let forward = Quat::IDENTITY.mul_vec3(LISTENER_FORWARD);
        assert!((forward - Vec3::new(0.0, 0.0, -1.0)).length() < 1e-5);
    }

    #[test]
    fn tc_ir_1_7_3_u1_velocity_clamp_cap() {
        let raw = Vec3::X * 500.0;
        let dt = 1.0;
        let v = listener_velocity(Some(Vec3::ZERO), raw * dt + Vec3::ZERO, dt);
        assert!((v.length() - MAX_LISTENER_VELOCITY).abs() < 1e-3);
    }

    #[test]
    fn tc_ir_1_7_3_u2_velocity_below_cap() {
        let dt = 1.0;
        let delta = Vec3::X * 50.0 * dt;
        let v = listener_velocity(Some(Vec3::ZERO), delta, dt);
        assert!((v.length() - 50.0).abs() < 1e-4);
    }

    #[test]
    fn tc_ir_1_7_3_u3_backward_difference() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::X;
        let dt = 0.5;
        let v = listener_velocity(Some(p0), p1, dt);
        assert!((v - Vec3::new(2.0, 0.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn tc_ir_1_7_1_1_listener_gets_cam_pos() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let pos = Vec3::new(5.0, 0.0, 0.0);
        let inputs = [CameraListenerInput {
            player_index: 0,
            position: pos,
            rotation: Quat::IDENTITY,
        }];
        camera_listener_sync_frame(
            &inputs,
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { position, .. } => {
                assert_eq!(*position, pos);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_1_2_listener_tracks_movement() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let dt = 1.0;
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        sink.commands.clear();
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::X,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { position, .. } => {
                assert_eq!(*position, Vec3::X);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_1_3_no_active_brain_retains_prev_slot() {
        let mut prev = ListenerPrevPositions::new();
        prev.set(0, Vec3::new(1.0, 2.0, 3.0));
        let mut sink = RecordingSink::new(8);
        camera_listener_sync_frame(
            &[],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert!(sink.commands.is_empty());
        assert_eq!(prev.get(0), Some(Vec3::new(1.0, 2.0, 3.0)));
    }

    #[test]
    fn tc_ir_1_7_3_1_velocity_from_delta() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let dt = 0.016;
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        sink.commands.clear();
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::X,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { velocity, .. } => {
                let expected = 1.0 / dt;
                assert!((velocity.length() - expected).abs() < 0.1);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_3_2_zero_vel_when_static() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let p = Vec3::new(3.0, 1.0, -2.0);
        let dt = 1.0 / 60.0;
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: p,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        sink.commands.clear();
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: p,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { velocity, .. } => {
                assert_eq!(*velocity, Vec3::ZERO);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_3_3_teleport_clamps_vel() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let dt = 1.0 / 60.0;
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        sink.commands.clear();
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::X * 1000.0,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { velocity, .. } => {
                assert!((velocity.length() - MAX_LISTENER_VELOCITY).abs() < 0.01);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_3_4_zero_dt_skips_vel() {
        let mut prev = ListenerPrevPositions::new();
        prev.set(0, Vec3::ZERO);
        let mut sink = RecordingSink::new(8);
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::X,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            0.0,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { velocity, .. } => {
                assert_eq!(*velocity, Vec3::ZERO);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_4_1_two_listeners_split() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let inputs = [
            CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            },
            CameraListenerInput {
                player_index: 1,
                position: Vec3::ONE,
                rotation: Quat::IDENTITY,
            },
        ];
        camera_listener_sync_frame(
            &inputs,
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert_eq!(sink.commands.len(), 2);
    }

    #[test]
    fn tc_ir_1_7_4_2_separate_listener_ids() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let inputs = [
            CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: Quat::IDENTITY,
            },
            CameraListenerInput {
                player_index: 1,
                position: Vec3::ONE,
                rotation: Quat::IDENTITY,
            },
        ];
        camera_listener_sync_frame(
            &inputs,
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        let ids: Vec<u8> = sink
            .commands
            .iter()
            .map(|c| match c {
                AudioCommand::UpdateListener { listener_id, .. } => listener_id.0,
            })
            .collect();
        assert_eq!(ids, vec![0, 1]);
    }

    #[test]
    fn tc_ir_1_7_5_1_cutscene_listener_at_cine_pos() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let cine = Vec3::new(0.0, 10.0, 0.0);
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: cine,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { position, .. } => {
                assert_eq!(*position, cine);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_5_2_exit_snaps_back_to_gameplay() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let dt = 1.0 / 60.0;
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::Y * 50.0,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        sink.commands.clear();
        let gp = Vec3::new(1.0, 0.0, 0.0);
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: gp,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            dt,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { position, .. } => {
                assert_eq!(*position, gp);
            }
        }
    }

    #[test]
    fn tc_ir_1_7_1_n1_no_camera_brain_no_cmd() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        camera_listener_sync_frame(
            &[],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert!(sink.commands.is_empty());
    }

    #[test]
    fn tc_ir_1_7_1_n2_prev_clear_slot_models_despawn() {
        let mut prev = ListenerPrevPositions::new();
        prev.set(0, Vec3::ONE);
        prev.clear(0);
        assert_eq!(prev.get(0), None);
    }

    #[test]
    fn tc_ir_1_7_3_n1_zero_delta_no_nan() {
        let v = listener_velocity(Some(Vec3::ZERO), Vec3::X, 0.0);
        assert_eq!(v, Vec3::ZERO);
        assert!(!v.is_nan());
    }

    #[test]
    fn tc_ir_1_7_3_n2_negative_dt_zero_velocity() {
        let v = listener_velocity(Some(Vec3::ZERO), Vec3::X, -0.001);
        assert_eq!(v, Vec3::ZERO);
    }

    #[test]
    fn tc_ir_1_7_3_n3_nan_position_dropped() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let m = camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::splat(f32::NAN),
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert!(sink.commands.is_empty());
        assert_eq!(m.dropped_nan, 1);
    }

    #[test]
    fn tc_ir_1_7_4_n2_mpsc_queue_full_drops_update() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(0);
        let pos = Vec3::ONE;
        let m = camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: pos,
                rotation: Quat::IDENTITY,
            }],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert_eq!(m.dropped_queue_full, 1);
        assert_eq!(m.sent, 0);
        assert_eq!(prev.get(0), Some(pos));
    }

    #[test]
    fn tc_ir_1_7_5_n1_two_active_same_player_first_wins() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let inputs = [
            CameraListenerInput {
                player_index: 0,
                position: Vec3::X,
                rotation: Quat::IDENTITY,
            },
            CameraListenerInput {
                player_index: 0,
                position: Vec3::Y * 99.0,
                rotation: Quat::IDENTITY,
            },
        ];
        camera_listener_sync_frame(
            &inputs,
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        assert_eq!(sink.commands.len(), 1);
        match &sink.commands[0] {
            AudioCommand::UpdateListener { position, .. } => {
                assert_eq!(*position, Vec3::X);
            }
        }
    }

    /// TC-IR-1.7.2.2 in this slice: orientation reaches the command; ear-level HRTF checks belong
    /// with the mixer once device-backed tests exist.
    #[test]
    fn tc_ir_1_7_2_2_orientation_propagates_to_command() {
        let mut prev = ListenerPrevPositions::new();
        let mut sink = RecordingSink::new(8);
        let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        camera_listener_sync_frame(
            &[CameraListenerInput {
                player_index: 0,
                position: Vec3::ZERO,
                rotation: rot,
            }],
            &mut prev,
            1.0 / 60.0,
            ListenerDebug::default(),
            &mut sink,
        );
        match &sink.commands[0] {
            AudioCommand::UpdateListener { orientation, .. } => {
                assert_eq!(*orientation, rot);
            }
        }
    }
}
