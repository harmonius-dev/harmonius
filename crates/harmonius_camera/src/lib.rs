//! Virtual camera selection and evaluation helpers for Harmonius.
//!
//! This crate implements deterministic, test-first camera logic described in
//! `docs/design/game-framework/camera.md`.

#![deny(clippy::all)]
#![forbid(unsafe_code)]

pub mod blend;
pub mod brain;
pub mod cinematic;
pub mod collision;
pub mod extensions;
pub mod group;
pub mod ids;
pub mod input;
pub mod intelligence;
pub mod mixer;
pub mod orbital;
pub mod position;
pub mod rotation;
pub mod shake;
pub mod third_person;
pub mod types;

pub use blend::{
    evaluate_blend_curve, resolve_custom_blend, BlendCurve, BlendDefinition, BlendRule,
    CustomBlends,
};
pub use brain::{
    select_highest_priority_camera, CameraBrain, CameraUpdateMode, FixedUpdateBrainClock,
};
pub use cinematic::{
    apply_picture_in_picture_viewport, compute_cine_vertical_fov_degrees, crane_rig_position,
    dolly_rig_position, CraneRig, DollyRig, NormalizedViewport,
};
pub use collision::{
    apply_camera_decollider, apply_deoccluder_pull_forward, apply_spring_arm_length,
    CameraDecollider, CameraDeoccluder, DeocclusionStrategy, SpringArm,
};
pub use extensions::{
    apply_auto_focus_distance, apply_camera_confiner_2d_boundary, apply_camera_confiner_3d_slowing,
    apply_follow_zoom_fov, apply_free_look_yaw_clamp, apply_modifier_stack_order,
    apply_recomposer_fov_override, apply_third_person_aim_ray_origin, CameraConfiner2D,
    CameraConfiner3D, CameraModifierStep, FollowZoom, FreeLookModifier, RecomposerState,
    ThirdPersonAim,
};
pub use group::{
    evaluate_group_framing_fov, evaluate_shot_quality_with_occlusion, group_bounding_box_center,
    GroupFraming, TargetGroup, TargetGroupMember,
};
pub use ids::{Entity, LayerMask};
pub use input::integrate_camera_input_frame_independent;
pub use intelligence::{
    evaluate_camera_sequencer, evaluate_clear_shot_child, evaluate_state_driven_camera,
    CameraSequencer, ClearShot, SequencerEntry, StateCameraMapping, StateDrivenCamera,
};
pub use mixer::mix_camera_positions;
pub use orbital::{
    evaluate_orbital_follow, evaluate_orbital_recenter, OrbitMode, OrbitalFollow, RecenterConfig,
};
pub use position::{
    evaluate_follow_step, evaluate_hard_lock_position, evaluate_position_composer_correction,
    evaluate_spline_dolly_nearest, evaluate_spline_dolly_speed, HardLockToTarget, PositionComposer,
    SplineDollySample, SplinePoint,
};
pub use rotation::{
    evaluate_hard_look_at_rotation, evaluate_pan_tilt_clamped_rotation,
    evaluate_rotate_with_target, evaluate_rotation_composer_delta, HardLookAt, PanTilt,
    PanTiltReference, RotateWithFollowTarget, RotationComposer,
};
pub use shake::{
    evaluate_cinematic_shake_at, evaluate_composite_shake, evaluate_impulse_listener_response,
    evaluate_perlin_noise_offset, evaluate_perlin_with_gain, evaluate_wave_oscillation_position,
    CinematicShakeClip, CompositeShakeLayer, ImpulseListener, ImpulseSource, PerlinNoiseProfile,
    WaveOscillation,
};
pub use third_person::{
    evaluate_third_person_collision_retraction, evaluate_third_person_shoulder, ThirdPersonFollow,
};
pub use types::{
    CameraOutput, CameraProjection, Follow, FollowBindingMode, ScreenRect, VirtualCamera,
};
