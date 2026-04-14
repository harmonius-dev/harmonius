//! Animation ↔ physics integration helpers: ragdoll asset layout, root-motion composition,
//! velocity limits, and swing-cone clamps. Matches `docs/design/integration/animation-physics.md`.
//!
//! Design name mapping (stub crate): `BonePaletteGpu` rows → `&[Mat4]` palette slices;
//! [`DesiredMovement`](CharacterController) ↔ [`CharacterController`]; [`RagdollActive`] ↔
//! [`RagdollTransition`]; dynamic impulses → [`RootMotionApplyOutcome`] plus
//! [`ExternalForce`].

#![deny(clippy::all)]
#![deny(unsafe_code)]

mod kinematics;
mod ragdoll;
mod root_motion;
mod sim;

pub use kinematics::{
    clamp_joint_twist, clamp_linear_angular_velocity, clamp_orientation_swing, clamp_to_swing_cone,
    LinearAngular,
};
pub use ragdoll::{
    BoneIndex, ColliderShape, Handle, RagdollBone, RagdollConstraint, RagdollDef, RagdollDefStore,
};
pub use root_motion::{
    compose_root_motion, root_rotation_delta_to_angular_impulse, ExternalForce,
    RootMotionApplyOutcome, RootMotionDelta, RootMotionFrame, RootMotionPipeline,
};
pub use sim::{
    animation_eval_policy, apply_root_motion_for_body, blend_ragdoll_pose, bone_collider_sync,
    inherit_root_linear_velocity, init_ragdoll_isometries_from_palette, ragdoll_recovery_fallback,
    recovery_blend_weight, AnimationEvalPolicy, BodyKind, BoneColliderSyncOutcome,
    CharacterController, LogSink, RagdollInitLog, RagdollInitOutcome, RagdollRef,
    RagdollTransition, WeaponColliderState,
};

/// Design `RagdollActive` component (same fields as [`RagdollTransition`]).
pub type RagdollActive = RagdollTransition;

/// Preferred alias for [`Handle<RagdollDef>`](Handle) in integration call sites.
pub type RagdollDefHandle = Handle<RagdollDef>;

/// Design `DesiredMovement` (direction + speed) maps to this controller stub.
pub type DesiredMovement = CharacterController;
