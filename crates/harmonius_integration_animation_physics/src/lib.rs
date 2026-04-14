//! Animation ↔ physics integration helpers: ragdoll asset layout, root-motion composition,
//! velocity limits, and swing-cone clamps. Matches `docs/design/integration/animation-physics.md`.

#![deny(clippy::all)]
#![deny(unsafe_code)]
// Public field docs are tracked in design docs; keep crate-level narration concise.
#![allow(missing_docs)]

mod kinematics;
mod ragdoll;
mod root_motion;
mod sim;

pub use kinematics::{
    clamp_linear_angular_velocity, clamp_orientation_swing, clamp_to_swing_cone, LinearAngular,
};
pub use ragdoll::{
    BoneIndex, ColliderShape, Handle, RagdollBone, RagdollConstraint, RagdollDef, RagdollDefStore,
};
pub use root_motion::{
    compose_root_motion, RootMotionApplyOutcome, RootMotionDelta, RootMotionFrame,
    RootMotionPipeline,
};
pub use sim::{
    animation_eval_policy, apply_root_motion_for_body, bone_collider_sync,
    inherit_root_linear_velocity, init_ragdoll_isometries_from_palette, ragdoll_recovery_fallback,
    recovery_blend_weight, AnimationEvalPolicy, BodyKind, BoneColliderSyncOutcome,
    CharacterController, LogSink, RagdollInitLog, RagdollInitOutcome, RagdollRef,
    RagdollTransition, WeaponColliderState,
};
