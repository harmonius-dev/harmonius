use smallvec::SmallVec;

use crate::StringId;

/// Parameters written by AI (Phase 4) and read by animation evaluation (Phase 6).
#[allow(clippy::struct_excessive_bools)] // Mirrors integration design field set (IR-1.1.4).
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationParams {
    /// Locomotion scalar used for blend spaces.
    pub speed: f32,
    /// Locomotion facing in degrees (design uses direction for 2D blend spaces).
    pub direction: f32,
    /// Whether the controller considers the entity supported by collision.
    pub is_grounded: bool,
    /// Whether the entity is in a lowered posture stance.
    pub is_crouching: bool,
    /// Whether a jump impulse is active this frame.
    pub is_jumping: bool,
    /// Whether the entity is airborne without upward impulse.
    pub is_falling: bool,
    /// Aim pitch offset in degrees.
    pub aim_pitch: f32,
    /// Aim yaw offset in degrees.
    pub aim_yaw: f32,
    /// One-shot transition triggers consumed by the animation state machine.
    pub triggers: SmallVec<[StringId; 4]>,
}

impl AnimationParams {
    /// Idle defaults used when [`AnimationParams`] is missing (FM-1).
    #[must_use]
    pub fn default_idle() -> Self {
        Self {
            speed: 0.0,
            direction: 0.0,
            is_grounded: true,
            is_crouching: false,
            is_jumping: false,
            is_falling: false,
            aim_pitch: 0.0,
            aim_yaw: 0.0,
            triggers: SmallVec::new(),
        }
    }
}
