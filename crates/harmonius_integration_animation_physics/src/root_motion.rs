//! Root motion delta buffering and composition.

use glam::{Quat, Vec3};

/// Translation + rotation delta extracted from animation (Phase 6).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootMotionDelta {
    /// Linear delta in parent/world space (per design consumption order).
    pub translation: Vec3,
    /// Orientation delta as a quaternion.
    pub rotation: Quat,
}

impl RootMotionDelta {
    /// Identity delta (no motion).
    pub const fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
}

/// Composes two deltas as rigid transforms: `first` then `second`.
///
/// Rotation uses quaternion multiplication; translation applies `first.rotation` to
/// `second.translation` before adding `first.translation`.
#[must_use]
pub fn compose_root_motion(first: RootMotionDelta, second: RootMotionDelta) -> RootMotionDelta {
    RootMotionDelta {
        translation: first.translation + first.rotation * second.translation,
        rotation: (first.rotation * second.rotation).normalize(),
    }
}

/// Tracks which physics frame a buffered delta belongs to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RootMotionFrame {
    /// Monotonic frame counter when the delta was produced (animation Phase 6).
    pub produced_frame: u64,
}

impl RootMotionFrame {
    /// Frame marker for tests and hosts without a scheduler yet.
    pub const fn new(produced_frame: u64) -> Self {
        Self { produced_frame }
    }
}

/// Result of applying buffered root motion once.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RootMotionApplyOutcome {
    /// Movement written to character controllers.
    pub desired_direction: Vec3,
    /// Non-negative speed scalar along `desired_direction`.
    pub desired_speed: f32,
    /// External linear impulse for dynamic bodies (when not using CC).
    pub external_linear: Vec3,
    /// Whether a sleeping body was explicitly woken before apply.
    pub woke_body: bool,
    /// Whether the delta was discarded (static / invalid body).
    pub discarded: bool,
}

/// Buffers a delta at animation Phase 6 and consumes it at the next frame Phase 5 start.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RootMotionPipeline {
    /// Engine frame counter.
    pub frame: u64,
    buffered: Option<(RootMotionFrame, RootMotionDelta)>,
}

impl RootMotionPipeline {
    /// Records a delta produced at the current `frame` (end of animation Phase 6).
    pub fn buffer_from_animation(&mut self, delta: RootMotionDelta) {
        self.buffered = Some((RootMotionFrame::new(self.frame), delta));
    }

    /// Advances the global frame counter (end of simulation tick).
    pub fn advance_frame(&mut self) {
        self.frame = self.frame.saturating_add(1);
    }

    /// Consumes a buffered delta at the start of Phase 5 when it was produced on the prior frame.
    pub fn consume_at_phase5_start(&mut self) -> Option<RootMotionDelta> {
        let (tag, delta) = self.buffered.take()?;
        (tag.produced_frame.saturating_add(1) == self.frame).then_some(delta)
    }
}

impl RootMotionApplyOutcome {
    /// Cleared delta after a successful controller apply.
    pub fn character_applied(direction: Vec3, speed: f32) -> Self {
        Self {
            desired_direction: direction,
            desired_speed: speed,
            external_linear: Vec3::ZERO,
            woke_body: false,
            discarded: false,
        }
    }

    /// Dynamic body path using external impulse.
    pub fn dynamic_force(linear: Vec3, woke_body: bool) -> Self {
        Self {
            desired_direction: Vec3::ZERO,
            desired_speed: 0.0,
            external_linear: linear,
            woke_body,
            discarded: false,
        }
    }

    /// Static body or invalid target: caller should log and drop delta.
    pub fn discarded() -> Self {
        Self {
            desired_direction: Vec3::ZERO,
            desired_speed: 0.0,
            external_linear: Vec3::ZERO,
            woke_body: false,
            discarded: true,
        }
    }
}
