//! VR pose sampling, tracking loss, play area, and gaze helpers.

use glam::{Quat, Vec3};

/// Monotonic pose sample for refresh-rate tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HmdPoseSample {
    /// Sample time in seconds.
    pub t: f32,
    /// Head orientation.
    pub orientation: Quat,
}

/// Emitted when tracking is interrupted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrackingLossEvent {
    /// Frame index when the loss was surfaced to gameplay.
    pub frame: u32,
}

/// Chaperone / guardian configuration modes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlayAreaMode {
    /// Room-scale boundary.
    RoomScale,
    /// Seated experience.
    Seated,
    /// Standing-only volume.
    Standing,
}

/// Boundary crossing events for play-area tests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayAreaBoundaryEvent {
    /// Active mode when the crossing happened.
    pub mode: PlayAreaMode,
    /// True when returning inside the volume.
    pub inside: bool,
}

/// One 6DOF controller snapshot.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Controller6DofSample {
    /// Position meters.
    pub position: Vec3,
    /// Orientation.
    pub orientation: Quat,
    /// Linear velocity m/s.
    pub velocity: Vec3,
    /// Angular velocity rad/s.
    pub angular_velocity: Vec3,
    /// South face pressed.
    pub south: bool,
    /// Trigger analog 0..1.
    pub trigger: f32,
    /// Capacitive touch on south pad.
    pub touch_south: bool,
}

/// Single joint pose for hand tracking tests.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HandJointPose {
    /// Joint position.
    pub position: Vec3,
    /// Joint orientation.
    pub orientation: Quat,
    /// Joint radius for proximity tests.
    pub radius: f32,
}

/// 26-joint hand skeleton.
#[derive(Clone, Debug, PartialEq)]
pub struct HandSkeleton {
    /// Joint array (indexing follows OpenXR order in full engine).
    pub joints: [HandJointPose; 26],
}

/// Gaze sample for fixation / saccade classification.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GazeRaySample {
    /// Time seconds.
    pub t: f32,
    /// Forward gaze direction (normalized).
    pub dir: Vec3,
}

/// High-level gaze behavior classification.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GazeBehavior {
    /// Stable fixation.
    Fixation,
    /// Fast jump.
    Saccade,
    /// Smooth pursuit (unused in current tests).
    Pursuit,
    /// Unknown / initializing.
    Unknown,
}

/// Emitted gaze events for UI / analytics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GazeEvent {
    /// Fixation started after stability window.
    Fixation,
    /// Saccade detected.
    Saccade,
}

/// Classifies gaze streams at 90 Hz (or arbitrary dt).
#[derive(Clone, Debug, Default)]
pub struct GazeClassifier {
    stable_ms: f32,
}

impl GazeClassifier {
    /// Feed samples; emits `GazeEvent` markers when thresholds cross.
    pub fn feed(
        &mut self,
        samples: &[GazeRaySample],
        fixation_deg_per_s: f32,
        saccade_deg_per_s: f32,
    ) -> Vec<GazeEvent> {
        let mut out = Vec::new();
        let mut prev = samples.first().map(|s| s.dir).unwrap_or(Vec3::Z);
        for w in samples.windows(2) {
            let a = w[0].dir;
            let b = w[1].dir;
            let dt = (w[1].t - w[0].t).max(1e-6);
            let ang = a.angle_between(b).to_degrees();
            let spd = ang / dt;
            if spd < fixation_deg_per_s {
                self.stable_ms += dt * 1000.0;
                if self.stable_ms >= 200.0 {
                    out.push(GazeEvent::Fixation);
                    self.stable_ms = 0.0;
                }
            } else {
                self.stable_ms = 0.0;
            }
            if spd > saccade_deg_per_s {
                out.push(GazeEvent::Saccade);
            }
            prev = b;
        }
        let _ = prev;
        out
    }
}

/// Bridges VR and gamepad sources into identical `ActionValue::Axis1D` for parity tests.
#[derive(Clone, Copy, Debug, Default)]
pub struct VrActionBridge;

impl VrActionBridge {
    /// Map trigger axis to normalized action scalar.
    pub fn axis1d_from_gamepad_trigger(v: f32) -> f32 {
        v.clamp(0.0, 1.0)
    }

    /// Map VR controller trigger to the same normalized scalar curve.
    pub fn axis1d_from_vr_trigger(v: f32) -> f32 {
        v.clamp(0.0, 1.0)
    }
}

/// Pinch detector using thumb / index tip distance (meters).
#[derive(Clone, Copy, Debug, Default)]
pub struct HandPinchDetector {
    /// Distance threshold in meters.
    pub threshold_m: f32,
}

/// Generate synthetic HMD pose timestamps for refresh-rate validation.
pub fn simulate_hmd_pose_stream(duration_s: f32, hz: f32) -> Vec<HmdPoseSample> {
    let dt = 1.0 / hz.max(1e-3);
    let n = (duration_s * hz).round() as usize;
    (0..n)
        .map(|i| HmdPoseSample {
            t: i as f32 * dt,
            orientation: Quat::IDENTITY,
        })
        .collect()
}

/// Surface tracking loss no later than `emit_frame + 1` (engine frame rule).
pub fn tracking_loss_event(emit_frame: u32) -> TrackingLossEvent {
    TrackingLossEvent {
        frame: emit_frame.saturating_add(1),
    }
}

/// Emit boundary enter/exit events for seated / standing / room-scale limits.
pub fn play_area_crossing(
    mode: PlayAreaMode,
    distance: f32,
    limit: f32,
) -> Option<PlayAreaBoundaryEvent> {
    if distance > limit {
        Some(PlayAreaBoundaryEvent { mode, inside: false })
    } else if distance < limit * 0.9 {
        Some(PlayAreaBoundaryEvent { mode, inside: true })
    } else {
        None
    }
}

impl HandPinchDetector {
    /// Returns true when thumb tip and index tip are closer than `threshold_m`.
    pub fn is_pinching(&self, thumb_tip: Vec3, index_tip: Vec3) -> bool {
        thumb_tip.distance(index_tip) < self.threshold_m
    }
}
