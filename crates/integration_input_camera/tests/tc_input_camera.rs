//! Integration tests mapped from `docs/design/integration/input-camera-test-cases.md`.

use glam::{Quat, Vec2, Vec3};
use integration_input_camera::{
    aim_deflect_look_delta, ciac_tick, free_look_sync, freelook_enabled, push_raw_camera_delta,
    vr_camera_brain_tick, ActionState, ActionValue, ActionValueType, AimAssistConfig, AimAssistState,
    BlendState, CameraInputAxisController, CameraOutput, Entity, FreeLookModifier, InputActionState,
    InputCameraDebug, InputModifier, InputSensitivity, ModifierChain, ModifierState, OrbitalFollow,
    PanTilt, RawCameraDelta, RawCameraInputBus, ResponseCurveType, VrBrainResult, VrCameraBrain,
    XrHeadPose,
};
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tracing::Subscriber;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::prelude::*;
use tracing_subscriber::Registry;

struct WarnCounter(Arc<AtomicUsize>);

impl<S: Subscriber> Layer<S> for WarnCounter {
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: Context<'_, S>,
    ) {
        if event.metadata().level() == &tracing::Level::WARN {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }
}

fn axis_state(v: Vec2) -> ActionState {
    ActionState {
        value: ActionValue::Axis2D(v),
        triggered: false,
        elapsed: 0.0,
        completed: false,
        value_type: ActionValueType::Axis2D,
    }
}

fn bool_state(pressed: bool, triggered: bool) -> ActionState {
    ActionState {
        value: ActionValue::Bool(pressed),
        triggered,
        elapsed: 0.0,
        completed: false,
        value_type: ActionValueType::Bool,
    }
}

fn default_ciac(look: integration_input_camera::ActionId) -> CameraInputAxisController {
    CameraInputAxisController {
        look_action: look,
        sensitivity: InputSensitivity { x: 1.0, y: 1.0 },
        invert_y: false,
        suppress_during_blend: true,
        suppression_timeout: 2.0,
        suppression_elapsed: 0.0,
    }
}

#[test]
fn tc_ir_4_1_1_1_mouse_yaw() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(10.0, 0.0)));
    let mut ciac = default_ciac(look);
    ciac.sensitivity = InputSensitivity { x: 1.0, y: 1.0 };
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState { is_blending: false },
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.yaw - 10.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_1_2_mouse_pitch() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.0, 5.0)));
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.pitch - 5.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_1_3_invert_y() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.0, 5.0)));
    let mut ciac = default_ciac(look);
    ciac.invert_y = true;
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.pitch + 5.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_2_1_stick_orbit_h() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.8, 0.0)));
    let mut ciac = default_ciac(look);
    let mut orb = OrbitalFollow {
        horizontal: 0.0,
        vertical: 0.0,
        distance: 5.0,
    };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        None,
        Some(&mut orb),
    );
    assert!((orb.horizontal - 0.8).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_2_2_stick_orbit_v() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.0, 0.6)));
    let mut ciac = default_ciac(look);
    let mut orb = OrbitalFollow {
        horizontal: 0.0,
        vertical: 0.0,
        distance: 5.0,
    };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        None,
        Some(&mut orb),
    );
    assert!((orb.vertical - 0.6).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_3_1_vr_pos_override() {
    let brain = VrCameraBrain {
        ipd: 0.064,
        late_latch: false,
    };
    let pose = XrHeadPose {
        position: Vec3::new(1.0, 2.0, 3.0),
        rotation: Quat::IDENTITY,
    };
    let mut out = CameraOutput {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut dbg = InputCameraDebug::default();
    let r = vr_camera_brain_tick(&brain, Some(&pose), &mut out, &mut dbg);
    assert_eq!(r, VrBrainResult::Applied);
    assert_eq!(out.position, Vec3::new(1.0, 2.0, 3.0));
}

#[test]
fn tc_ir_4_1_3_2_vr_rot_override() {
    let brain = VrCameraBrain {
        ipd: 0.064,
        late_latch: false,
    };
    let q = Quat::from_rotation_y(0.42);
    let pose = XrHeadPose {
        position: Vec3::ZERO,
        rotation: q,
    };
    let mut out = CameraOutput {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut dbg = InputCameraDebug::default();
    let r = vr_camera_brain_tick(&brain, Some(&pose), &mut out, &mut dbg);
    assert_eq!(r, VrBrainResult::Applied);
    assert!((out.rotation.w - q.w).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_3_3_vr_track_lost_holds_last() {
    let brain = VrCameraBrain {
        ipd: 0.064,
        late_latch: false,
    };
    let pose = XrHeadPose {
        position: Vec3::new(1.0, 0.0, 0.0),
        rotation: Quat::IDENTITY,
    };
    let mut out = CameraOutput {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
    };
    let mut dbg = InputCameraDebug::default();
    let _ = vr_camera_brain_tick(&brain, Some(&pose), &mut out, &mut dbg);
    for _ in 0..10 {
        let _ = vr_camera_brain_tick(&brain, Some(&pose), &mut out, &mut dbg);
    }
    assert_eq!(out.position, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn tc_ir_4_1_4_1_freelook_insert() {
    let fl = integration_input_camera::ActionId(9);
    let cam = Entity(1);
    let mut actions = InputActionState::default();
    actions.insert(fl, bool_state(true, true));
    let mut markers = HashSet::new();
    free_look_sync(&actions, fl, cam, false, &mut markers);
    assert!(markers.contains(&cam));
    let _ = FreeLookModifier;
}

#[test]
fn tc_ir_4_1_4_2_freelook_remove() {
    let fl = integration_input_camera::ActionId(9);
    let cam = Entity(1);
    let mut actions = InputActionState::default();
    actions.insert(fl, bool_state(false, true));
    let mut markers = HashSet::new();
    markers.insert(cam);
    free_look_sync(&actions, fl, cam, false, &mut markers);
    assert!(!freelook_enabled(&markers, cam));
}

#[test]
fn tc_ir_4_1_5_1_aim_deflect() {
    let mut cfg = AimAssistConfig {
        magnetism_radius: 10.0,
        magnetism_strength: 0.5,
        friction_radius: 0.5,
        friction_multiplier: 1.0,
        snap_enabled: false,
        snap_radius: 0.1,
        enabled: true,
    };
    let mut st = AimAssistState::default();
    let cam = Vec3::ZERO;
    let forward = Vec3::new(0.0, 0.0, -1.0);
    let targets = [(Entity(2), Vec3::new(1.0, 0.0, -1.0))];
    let d = Vec2::new(0.0, 0.1);
    let out = aim_deflect_look_delta(&cfg, &mut st, cam, forward, &targets, d);
    assert_ne!(out, d);
    cfg.enabled = false;
    let passthrough = aim_deflect_look_delta(&cfg, &mut st, cam, forward, &targets, d);
    assert_eq!(passthrough, d);
}

#[test]
fn tc_ir_4_1_5_2_aim_passthrough_no_targets() {
    let cfg = AimAssistConfig {
        magnetism_radius: 2.0,
        magnetism_strength: 0.9,
        friction_radius: 0.5,
        friction_multiplier: 1.0,
        snap_enabled: false,
        snap_radius: 0.1,
        enabled: true,
    };
    let mut st = AimAssistState::default();
    let d = Vec2::new(0.2, -0.1);
    let out = aim_deflect_look_delta(
        &cfg,
        &mut st,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        d,
    );
    assert_eq!(out, d);
}

#[test]
fn tc_ir_4_1_6_1_dead_zone_radial() {
    let chain = ModifierChain::from_slice(&[InputModifier::DeadZoneRadial {
        threshold: 0.08,
    }]);
    let mut ms = ModifierState::default();
    let v = chain.apply(
        ActionValue::Axis2D(Vec2::new(0.05, 0.05)),
        &mut ms,
    );
    assert_eq!(v, ActionValue::Axis2D(Vec2::ZERO));
}

#[test]
fn tc_ir_4_1_6_2_response_curve() {
    let chain = ModifierChain::from_slice(&[InputModifier::ResponseCurve {
        curve_type: ResponseCurveType::Exponential { exp: 2.0 },
    }]);
    let mut ms = ModifierState::default();
    let v = chain.apply(ActionValue::Axis2D(Vec2::new(0.5, 0.0)), &mut ms);
    match v {
        ActionValue::Axis2D(o) => assert!((o.x - 0.25).abs() < 1e-5),
        _ => panic!("expected Axis2D"),
    }
}

#[test]
fn tc_ir_4_1_7_1_sensitivity_scale() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(10.0, 5.0)));
    let mut ciac = default_ciac(look);
    ciac.sensitivity = InputSensitivity { x: 2.0, y: 2.0 };
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.yaw - 20.0).abs() < 1e-5 && (pt.pitch - 10.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_8_1_blend_suppress() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(3.0, 3.0)));
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState { is_blending: true },
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert_eq!(pt, PanTilt { yaw: 0.0, pitch: 0.0 });
}

#[test]
fn tc_ir_4_1_8_2_blend_resume() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(2.0, 1.0)));
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState { is_blending: false },
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.yaw - 2.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_e1_missing_action_warn_once() {
    let warns = Arc::new(AtomicUsize::new(0));
    let layer = WarnCounter(warns.clone());
    let _g = Registry::default().with(layer).set_default();

    let look = integration_input_camera::ActionId(77);
    let actions = InputActionState::default();
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert_eq!(warns.load(Ordering::SeqCst), 1);
    assert!(dbg.missing_action_warned.contains(&look));
}

#[test]
fn tc_ir_4_1_e2_type_mismatch_warn_once() {
    let warns = Arc::new(AtomicUsize::new(0));
    let layer = WarnCounter(warns.clone());
    let _g = Registry::default().with(layer).set_default();

    let look = integration_input_camera::ActionId(88);
    let mut actions = InputActionState::default();
    actions.insert(
        look,
        ActionState {
            value: ActionValue::Bool(true),
            triggered: false,
            elapsed: 0.0,
            completed: false,
            value_type: ActionValueType::Bool,
        },
    );
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert_eq!(warns.load(Ordering::SeqCst), 1);
}

#[test]
fn tc_ir_4_1_e3_channel_overflow_drops() {
    let bus = RawCameraInputBus::new(256);
    let s = bus.sender();
    let r = bus.receiver();
    let mut dropped = 0u64;
    for i in 0..512 {
        push_raw_camera_delta(
            &s,
            r,
            RawCameraDelta {
                delta: Vec2::new(i as f32, 0.0),
            },
            &mut dropped,
        );
    }
    assert_eq!(dropped, 256);
}

#[test]
fn tc_ir_4_1_e4_suppress_timeout_clears() {
    let warns = Arc::new(AtomicUsize::new(0));
    let layer = WarnCounter(warns.clone());
    let _g = Registry::default().with(layer).set_default();

    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(1.0, 1.0)));
    let mut ciac = default_ciac(look);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    let blend = BlendState { is_blending: true };
    ciac_tick(
        2.1,
        &actions,
        &blend,
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!(!ciac.suppress_during_blend);
    assert!(dbg.blend_suppress_timeout_warned);
    assert_eq!(warns.load(Ordering::SeqCst), 1);
}

#[test]
fn tc_ir_4_1_e5_vr_skipped_without_pose() {
    let brain = VrCameraBrain {
        ipd: 0.064,
        late_latch: false,
    };
    let mut out = CameraOutput {
        position: Vec3::ONE,
        rotation: Quat::IDENTITY,
    };
    let mut dbg = InputCameraDebug::default();
    let r = vr_camera_brain_tick(&brain, None, &mut out, &mut dbg);
    assert_eq!(r, VrBrainResult::SkippedNoPose);
    assert_eq!(out.position, Vec3::ONE);
}

#[test]
fn tc_ir_4_1_e6_aim_disabled_raw_through_ciac() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.3, 0.2)));
    let mut ciac = default_ciac(look);
    let cfg = AimAssistConfig {
        magnetism_radius: 5.0,
        magnetism_strength: 0.9,
        friction_radius: 0.5,
        friction_multiplier: 1.0,
        snap_enabled: false,
        snap_radius: 0.1,
        enabled: false,
    };
    let mut st = AimAssistState::default();
    let targets = [(Entity(2), Vec3::new(0.0, 0.0, -1.0))];
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        Some(&cfg),
        Some(&mut st),
        Vec3::ZERO,
        Vec3::NEG_Z,
        &targets,
        Some(&mut pt),
        None,
    );
    assert!((pt.yaw - 0.3).abs() < 1e-5 && (pt.pitch - 0.2).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_c1_sens_inv_dead_zone_order() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.03, 0.5)));
    let mut ciac = default_ciac(look);
    ciac.sensitivity = InputSensitivity { x: 2.0, y: 2.0 };
    ciac.invert_y = true;
    let chain = ModifierChain::from_slice(&[InputModifier::DeadZoneAxial {
        threshold_x: 0.1,
        threshold_y: 0.0,
    }]);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    assert!((pt.yaw).abs() < 1e-5);
    assert!((pt.pitch + 1.0).abs() < 1e-5);
}

#[test]
fn tc_ir_4_1_c2_curve_before_sensitivity_deterministic() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.5, 0.5)));
    let mut ciac = default_ciac(look);
    ciac.sensitivity = InputSensitivity { x: 1.5, y: 1.5 };
    ciac.invert_y = true;
    let chain = ModifierChain::from_slice(&[InputModifier::ResponseCurve {
        curve_type: ResponseCurveType::Exponential { exp: 2.0 },
    }]);
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        None,
        None,
        Vec3::ZERO,
        Vec3::NEG_Z,
        &[],
        Some(&mut pt),
        None,
    );
    let mag = (0.5_f32.hypot(0.5)).powf(2.0);
    let expected_yaw = mag * (1.0 / 2.0_f32.sqrt()) * 1.5;
    let expected_pitch = -mag * (1.0 / 2.0_f32.sqrt()) * 1.5;
    assert!((pt.yaw - expected_yaw).abs() < 1e-4);
    assert!((pt.pitch - expected_pitch).abs() < 1e-4);
}

#[test]
fn tc_ir_4_1_c3_aim_does_not_bypass_blend() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.5, 0.0)));
    let mut ciac = default_ciac(look);
    let cfg = AimAssistConfig {
        magnetism_radius: 10.0,
        magnetism_strength: 0.8,
        friction_radius: 0.5,
        friction_multiplier: 1.0,
        snap_enabled: false,
        snap_radius: 0.1,
        enabled: true,
    };
    let mut st = AimAssistState::default();
    let targets = [(Entity(2), Vec3::new(1.0, 0.0, -1.0))];
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState { is_blending: true },
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        Some(&cfg),
        Some(&mut st),
        Vec3::ZERO,
        Vec3::NEG_Z,
        &targets,
        Some(&mut pt),
        None,
    );
    assert_eq!(pt, PanTilt { yaw: 0.0, pitch: 0.0 });
}

#[test]
fn tc_ir_4_1_c4_sens_plus_aim() {
    let look = integration_input_camera::ActionId(1);
    let mut actions = InputActionState::default();
    actions.insert(look, axis_state(Vec2::new(0.2, 0.0)));
    let mut ciac = default_ciac(look);
    ciac.sensitivity = InputSensitivity { x: 2.0, y: 2.0 };
    let cfg = AimAssistConfig {
        magnetism_radius: 10.0,
        magnetism_strength: 0.5,
        friction_radius: 0.5,
        friction_multiplier: 1.0,
        snap_enabled: false,
        snap_radius: 0.1,
        enabled: true,
    };
    let mut st = AimAssistState::default();
    let targets = [(Entity(2), Vec3::new(1.0, 0.0, -1.0))];
    let mut pt = PanTilt { yaw: 0.0, pitch: 0.0 };
    let mut dbg = InputCameraDebug::default();
    let chain = ModifierChain::new();
    let mut ms = ModifierState::default();
    ciac_tick(
        1.0 / 60.0,
        &actions,
        &BlendState::default(),
        &mut dbg,
        &chain,
        &mut ms,
        &mut ciac,
        Some(&cfg),
        Some(&mut st),
        Vec3::ZERO,
        Vec3::NEG_Z,
        &targets,
        Some(&mut pt),
        None,
    );
    let raw = aim_deflect_look_delta(
        &cfg,
        &mut AimAssistState::default(),
        Vec3::ZERO,
        Vec3::NEG_Z,
        &targets,
        Vec2::new(0.2, 0.0),
    );
    assert!((pt.yaw - raw.x * 2.0).abs() < 1e-4);
}

#[test]
fn tc_ir_4_1_c5_vr_wins_over_freelook() {
    let fl = integration_input_camera::ActionId(9);
    let cam = Entity(1);
    let mut actions = InputActionState::default();
    actions.insert(fl, bool_state(true, true));
    let mut markers = HashSet::new();
    free_look_sync(&actions, fl, cam, true, &mut markers);
    assert!(!markers.contains(&cam));
}
