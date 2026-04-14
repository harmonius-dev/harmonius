use glam::{Quat, Vec3};

use crate::animation_params::AnimationParams;
use crate::diagnostics::Diagnostics;
use crate::montage::{ActiveMontage, AssetHandle, MontageInstance, MontageState};
use crate::StringId;

/// Stable identifier for a state-graph node (animation SM).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct StateNodeId(pub u32);

/// Canonical locomotion node used when speed is zero (TC-IR-1.1.4.2).
pub const STATE_IDLE: StateNodeId = StateNodeId(1);
/// Blend node for walk/run strafe (TC-IR-1.1.4.1).
pub const STATE_WALK_RUN_BLEND: StateNodeId = StateNodeId(2);

/// Result of locomotion selection for assertions (TC-IR-1.1.4.x).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocomotionSelection {
    /// Active state node after evaluation.
    pub state: StateNodeId,
    /// Blend factor toward run in `[0,1]` when [`STATE_WALK_RUN_BLEND`] is active.
    pub run_blend: f32,
}

/// Persisted animation state machine component (Phase 6 writer, Phase 4 reader).
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationStateMachine {
    current_state: StateNodeId,
    current_state_name: &'static str,
    clip_duration_sec: f32,
    clip_elapsed_sec: f32,
    transitioning: bool,
    root_motion_translation: Vec3,
    root_motion_rotation: Quat,
    warned_missing_params: bool,
    /// Trigger ids recognized by transition conditions for the stub SM.
    pub known_triggers: Vec<StringId>,
}

impl AnimationStateMachine {
    /// New state machine rooted at idle with a one-second clip horizon.
    #[must_use]
    pub fn new_idle() -> Self {
        Self {
            current_state: STATE_IDLE,
            current_state_name: "idle",
            clip_duration_sec: 1.0,
            clip_elapsed_sec: 0.0,
            transitioning: false,
            root_motion_translation: Vec3::ZERO,
            root_motion_rotation: Quat::IDENTITY,
            warned_missing_params: false,
            known_triggers: Vec::new(),
        }
    }

    /// Configures clip timing for [`AnimationQuery::remaining_time`] tests.
    pub fn set_clip_timing(&mut self, duration_sec: f32, elapsed_sec: f32) {
        self.clip_duration_sec = duration_sec;
        self.clip_elapsed_sec = elapsed_sec;
    }

    /// Forces transition flag for tests (TC-IR-1.1.2.2).
    pub fn set_transitioning(&mut self, value: bool) {
        self.transitioning = value;
    }

    /// Registers a trigger id treated as valid by transition evaluation.
    pub fn register_known_trigger(&mut self, id: StringId) {
        if !self.known_triggers.contains(&id) {
            self.known_triggers.push(id);
        }
    }

    /// Current state node id.
    #[must_use]
    pub fn state_id(&self) -> StateNodeId {
        self.current_state
    }

    /// Human-readable state label for diagnostics.
    #[must_use]
    pub fn state_name(&self) -> &'static str {
        self.current_state_name
    }

    /// Seconds remaining in the active clip window.
    #[must_use]
    pub fn remaining_clip_time(&self) -> f32 {
        (self.clip_duration_sec - self.clip_elapsed_sec).max(0.0)
    }

    /// Whether the state machine reports an in-flight transition blend.
    #[must_use]
    pub fn is_transitioning_flag(&self) -> bool {
        self.transitioning
    }

    /// Translation component accumulated for the last evaluation step.
    #[must_use]
    pub fn root_motion_translation(&self) -> Vec3 {
        self.root_motion_translation
    }

    /// Rotation component accumulated for the last evaluation step.
    #[must_use]
    pub fn root_motion_rotation(&self) -> Quat {
        self.root_motion_rotation
    }

    /// Evaluates locomotion selection from parameters (pure helper for TC-IR-1.1.4.x).
    #[must_use]
    pub fn locomotion_from_params(params: &AnimationParams) -> LocomotionSelection {
        if params.speed <= f32::EPSILON {
            return LocomotionSelection {
                state: STATE_IDLE,
                run_blend: 0.0,
            };
        }
        // Walk/run threshold: blend ramps between speed 1.5 and 3.0.
        let run_blend = ((params.speed - 1.5) / 1.5).clamp(0.0, 1.0);
        LocomotionSelection {
            state: STATE_WALK_RUN_BLEND,
            run_blend,
        }
    }

    /// Single-frame animation evaluation: consumes params, updates SM, validates montages.
    pub fn evaluate_frame(
        &mut self,
        params: Option<&mut AnimationParams>,
        diagnostics: &mut Diagnostics,
        active_montage: &mut Option<ActiveMontage>,
        dt: f32,
    ) {
        let effective = if let Some(p) = params {
            filter_triggers(p, &self.known_triggers, diagnostics);
            p.clone()
        } else {
            if !self.warned_missing_params {
                diagnostics.warn("missing AnimationParams: defaulting to idle parameters");
                self.warned_missing_params = true;
            }
            AnimationParams::default_idle()
        };

        let locomotion = Self::locomotion_from_params(&effective);
        let prev = self.current_state;
        self.current_state = locomotion.state;
        self.current_state_name = if locomotion.state == STATE_IDLE {
            "idle"
        } else {
            "walk_run_blend"
        };
        self.transitioning = prev != self.current_state;

        self.clip_duration_sec = self.clip_duration_sec.max(0.000_1);
        if dt > f32::EPSILON {
            self.clip_elapsed_sec = (self.clip_elapsed_sec + dt).min(self.clip_duration_sec);
        }
        self.root_motion_translation = Vec3::new(effective.speed * dt, 0.0, 0.0);
        self.root_motion_rotation = Quat::from_rotation_y(effective.direction.to_radians() * dt);

        if let Some(m) = active_montage.as_mut() {
            if m.instance.montage.is_invalid() {
                diagnostics.error(format!(
                    "ActiveMontage references missing montage asset id={}",
                    m.instance.montage.id
                ));
                *active_montage = None;
            } else {
                m.instance.tick(dt);
                if m.instance.state == MontageState::Finished {
                    *active_montage = None;
                }
            }
        }
    }
}

/// Filters trigger list against known ids (FM-2: unknown triggers removed with warning).
pub fn filter_triggers(params: &mut AnimationParams, known: &[StringId], diagnostics: &mut Diagnostics) {
    let mut i = 0;
    while i < params.triggers.len() {
        let t = params.triggers[i];
        if known.contains(&t) {
            i += 1;
        } else {
            diagnostics.warn(format!("unrecognized trigger StringId={}", t.0));
            params.triggers.remove(i);
        }
    }
}

/// Inserts a combat montage placeholder (TC-IR-1.1.3.1).
#[must_use]
pub fn combat_montage_placeholder() -> ActiveMontage {
    ActiveMontage {
        instance: MontageInstance {
            montage: AssetHandle::new(1),
            duration_sec: 0.5,
            elapsed_sec: 0.0,
            blend_weight: 1.0,
            state: MontageState::Playing,
            notifies_fired_this_frame: Vec::new(),
        },
    }
}
