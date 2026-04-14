use glam::{Quat, Vec3};

use crate::montage::{notify_id, ActiveMontage};
use crate::state_machine::StateNodeId;
use crate::AnimationStateMachine;

/// Static query helper for AI systems (IR-1.1.2).
#[derive(Debug)]
pub struct AnimationQuery;

impl AnimationQuery {
    /// Current state node name (persisted SM component).
    #[must_use]
    pub fn current_state(sm: &AnimationStateMachine) -> &'static str {
        sm.state_name()
    }

    /// Remaining time in the current clip segment.
    #[must_use]
    pub fn remaining_time(sm: &AnimationStateMachine) -> f32 {
        sm.remaining_clip_time()
    }

    /// Whether `state` matches the active node.
    #[must_use]
    pub fn is_in_state(sm: &AnimationStateMachine, state: StateNodeId) -> bool {
        sm.state_id() == state
    }

    /// Whether a graph transition is actively blending.
    #[must_use]
    pub fn is_transitioning(sm: &AnimationStateMachine) -> bool {
        sm.is_transitioning_flag()
    }

    /// Root motion delta for the last evaluated frame (translation, rotation).
    #[must_use]
    pub fn root_motion_delta(sm: &AnimationStateMachine) -> (Vec3, Quat) {
        (
            sm.root_motion_translation(),
            sm.root_motion_rotation(),
        )
    }

    /// Whether a montage notify fired this frame (string compare via [`StringId`] hash of `name`).
    #[must_use]
    pub fn montage_notify_fired(montage: &ActiveMontage, notify_name: &str) -> bool {
        let needle = notify_id(notify_name);
        montage
            .instance
            .notifies_fired_this_frame
            .iter()
            .any(|n| notify_id(n) == needle)
    }
}
