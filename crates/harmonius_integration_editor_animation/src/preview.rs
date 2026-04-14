//! In-memory preview commands from the editor into the animation worker.

use crate::clip::{AnimationClip, AnimationClipTable};
use crate::handle::Handle;
use crate::ids::{Entity, ParameterId};

/// High-level preview intent for a single entity.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimPreviewCommand {
    /// Target entity in the editor world.
    pub entity: Entity,
    /// Concrete preview operation.
    pub action: PreviewAction,
}

/// Preview operations routed through the event bridge (in-memory only).
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewAction {
    /// Begin playback of a clip at the given speed multiplier.
    Play {
        /// Clip to evaluate.
        clip: Handle<AnimationClip>,
        /// Scalar speed multiplier applied to timeline advance.
        speed: f32,
    },
    /// Pause ongoing preview playback.
    Pause,
    /// Apply the pose at normalized time within the current frame (no extra frame delay).
    Scrub {
        /// Normalized time in `[0.0, 1.0]`.
        normalized_time: f32,
    },
    /// Update a float blend parameter by stable engine id (no string lookup).
    SetBlendParam {
        /// Parameter slot index.
        param: ParameterId,
        /// New parameter value.
        value: f32,
    },
    /// Advance simulation by an integer tick delta for stepping.
    StepFrame {
        /// Tick delta relative to the current pose clock.
        delta_ticks: i32,
    },
}

/// Pose classification after applying a preview action with failure-mode policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewPoseKind {
    /// A clip-driven pose should be shown.
    Animated,
    /// Invalid clip or missing data: show authoring T-pose placeholder.
    TPose,
    /// Blend space had no samples: bind pose fallback.
    BindPose,
}

/// Structured outcome for deterministic tests and editor UI routing.
#[derive(Debug, PartialEq)]
pub struct PreviewApplyOutcome {
    /// Which pose bucket the viewport should display.
    pub pose: PreviewPoseKind,
    /// Human-readable warnings mirroring production logging text.
    pub warnings: Vec<&'static str>,
}

/// Emitted when a clip handle does not match a live slot.
pub const WARN_INVALID_CLIP_HANDLE: &str = "invalid clip handle for preview";

/// Emitted when a 2D blend space has zero samples.
pub const WARN_BLEND_SPACE_EMPTY: &str = "blend space has no samples; clamped to bind pose";

/// Applies failure-mode policy for preview actions (TC-IR-5.3.F1 / F2 slices).
#[must_use]
pub fn apply_preview_action(
    action: &PreviewAction,
    clips: &AnimationClipTable,
    blend_space_sample_count: usize,
) -> PreviewApplyOutcome {
    match action {
        PreviewAction::Play { clip, speed: _ } => {
            if clips.is_valid(*clip) {
                PreviewApplyOutcome {
                    pose: PreviewPoseKind::Animated,
                    warnings: Vec::new(),
                }
            } else {
                PreviewApplyOutcome {
                    pose: PreviewPoseKind::TPose,
                    warnings: vec![WARN_INVALID_CLIP_HANDLE],
                }
            }
        }
        PreviewAction::Pause => PreviewApplyOutcome {
            pose: PreviewPoseKind::Animated,
            warnings: Vec::new(),
        },
        PreviewAction::Scrub { .. } => PreviewApplyOutcome {
            pose: PreviewPoseKind::Animated,
            warnings: Vec::new(),
        },
        PreviewAction::SetBlendParam { param: _, value: _ } => {
            if blend_space_sample_count == 0 {
                PreviewApplyOutcome {
                    pose: PreviewPoseKind::BindPose,
                    warnings: vec![WARN_BLEND_SPACE_EMPTY],
                }
            } else {
                PreviewApplyOutcome {
                    pose: PreviewPoseKind::Animated,
                    warnings: Vec::new(),
                }
            }
        }
        PreviewAction::StepFrame { .. } => PreviewApplyOutcome {
            pose: PreviewPoseKind::Animated,
            warnings: Vec::new(),
        },
    }
}
