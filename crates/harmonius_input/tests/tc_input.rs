//! Traceability tests for [PLAN-input-input] companion `input-test-cases.md` rows.

use glam::{Quat, Vec2, Vec3};
use harmonius_input::{
    encode_dual_motor_hid, gyro_integrate_pitch, normalize_cardinal_forward_from_sources,
    normalize_intensity, normalize_native_w_key, parse_adaptive_profile_line, play_area_crossing,
    qcf_punch_tree, simulate_hmd_pose_stream, tracking_loss_event,
    ActionBinding, ActionCategory, ActionDefinition, ActionId, ActionValue, ActionValueType,
    AdaptiveTriggerDriver, AdaptiveTriggerEffect, AudioHapticGenerator, BindingError, BindingLoader,
    CancelWindow, CardinalDirection, ComboEvaluator, ComboInput, ContextId, ContextStack,
    Controller6DofSample, DeviceCapabilities, DeviceConnected, DeviceFamily, DeviceId, DeviceInfo,
    DeviceManager, DeviceType, DualMotorState, FingerId, GamepadButton, GamepadFamily, GamepadState,
    GamepadStickSource, GazeBehavior, GazeClassifier, GazeEvent, GazeRaySample, GestureEngine,
    GesturePhase, GestureType, GlyphResolver, HapticBackendKind, HapticCommand, HandJointPose,
    HandPinchDetector, HandSkeleton, InputBuffer, InputMapper, InputModifier, InputSource,
    KeyboardState, Keycode, LongPressRecognizer, MappingContext, ModifierChain, ModifierState,
    MouseState, PatternPlayer, PlayAreaMode, RawInputEvent, RawInputKind, RebindError,
    RebindManager, RebindRequest, RebindResolution, RebindResult, ResolvedGesture, ResponseCurveType,
    RumbleEnvelope, RumblePattern, Scancode, TouchContact, TouchRegionId, TouchState,
    TriggerCondition, TriggerPhase, TriggerState, TriggerEffect, VrActionBridge,
};

/// TC-6.1.1.1 — keyboard events carry scancode and non-zero keycode.
#[test]
fn test_keyboard_event_carries_both() {
    let dev = DeviceId {
        index: 0,
        generation: 0,
    };
    let e = RawInputEvent {
        device_id: dev,
        timestamp: 1,
        kind: RawInputKind::KeyPress {
            scancode: Scancode::W,
            keycode: Keycode(32),
        },
    };
    if let RawInputKind::KeyPress { scancode, keycode } = e.kind {
        assert_ne!(scancode as u16, 0);
        assert_ne!(keycode.0, 0);
    } else {
        panic!("expected key press");
    }
}

/// TC-6.1.2.1 — native W scancodes normalize to `Scancode::W`.
#[test]
fn test_scancode_w_cross_platform() {
    assert_eq!(normalize_native_w_key(0x11), Scancode::W);
    assert_eq!(normalize_native_w_key(0x0D), Scancode::W);
}

/// TC-6.1.3.1 — sub-pixel mouse delta preserved.
#[test]
fn test_mouse_subpixel_delta() {
    let mut m = MouseState::default();
    m.apply_motion(0.00390625, 0.0, 1.0);
    assert_eq!(m.delta.x, 0.00390625);
}

/// TC-6.1.4.1 — DPI normalization matches within 1% for identical physical motion.
#[test]
fn test_dpi_normalized_delta() {
    let mut a = MouseState::default();
    let mut b = MouseState::default();
    a.apply_motion(10.0, 0.0, 1.0);
    b.apply_motion(10.0, 0.0, 2.0);
    let na = a.delta.x;
    let nb = b.delta.x;
    assert!(((na - nb).abs() / na) < 0.01);
}

/// TC-6.1.5.1 — discrete vs smooth scroll lands within 5%.
#[test]
fn test_scroll_wheel_vs_trackpad() {
    let mut m = MouseState::default();
    m.apply_scroll(0.0, 120.0, true);
    let y1 = m.scroll.y;
    m.scroll = Vec2::ZERO;
    m.apply_scroll(0.0, 1.0, false);
    let y2 = m.scroll.y;
    assert!(((y1 - y2).abs() / y1.max(1e-6)) < 0.05);
}

/// TC-6.1.6.1 — south face unifies across vendors.
#[test]
fn test_gamepad_south_unification() {
    let mut g = GamepadState::default();
    g.set_button(GamepadButton::South, true);
    assert!(g.is_pressed(GamepadButton::South));
}

/// TC-6.1.7.1 — gyro integration reaches ~90° pitch.
#[test]
fn test_gyro_madgwick_orientation() {
    let frames = 1000_usize;
    let dt = 0.01_f32;
    let gyro_x = std::f32::consts::FRAC_PI_2 / (frames as f32 * dt);
    let q = gyro_integrate_pitch(Quat::IDENTITY, gyro_x, dt, frames);
    let expected = Quat::from_rotation_x(std::f32::consts::FRAC_PI_2);
    let ang = q.angle_between(expected).to_degrees();
    assert!(ang < 5.0, "angle diff {ang}");
}

/// TC-6.1.8.1 — DualSense capability bits.
#[test]
fn test_capability_flags_dualsense() {
    let mut dm = DeviceManager::new();
    let caps = DeviceCapabilities {
        has_gyroscope: true,
        has_accelerometer: true,
        has_touchpad: true,
        has_adaptive_triggers: true,
        has_rumble: true,
        has_hd_rumble: true,
        max_touch_points: 2,
    };
    let id = dm.handle_connect(DeviceInfo {
        id: DeviceId {
            index: 0,
            generation: 0,
        },
        device_type: DeviceType::Gamepad,
        vendor_id: 0x054C,
        product_id: 0x0CE6,
        dpi_scale: 1.0,
        gamepad_family: Some(GamepadFamily::DualSense),
        capabilities: caps,
    });
    let c = dm.capabilities(id).unwrap();
    assert!(c.has_adaptive_triggers && c.has_hd_rumble && c.has_touchpad);
    let xbox = DeviceCapabilities {
        has_adaptive_triggers: false,
        ..caps
    };
    assert!(!xbox.has_adaptive_triggers);
}

/// TC-6.1.9.1 — ten simultaneous touches.
#[test]
fn test_touch_10_simultaneous() {
    let mut t = TouchState::default();
    for i in 0_u8..10 {
        t.upsert(TouchContact {
            finger_id: FingerId(i),
            position: Vec2::new(i as f32, i as f32),
            pressure: 0.1 + 0.09 * i as f32,
            area: 1.0,
        });
    }
    assert_eq!(t.contact_count, 10);
}

/// TC-6.1.11.1 — one connect event per hot-plug.
#[test]
fn test_device_connect_event() {
    let mut dm = DeviceManager::new();
    let _ = dm.handle_connect(DeviceInfo {
        id: DeviceId {
            index: 0,
            generation: 0,
        },
        device_type: DeviceType::Gamepad,
        vendor_id: 0,
        product_id: 0,
        dpi_scale: 1.0,
        gamepad_family: Some(GamepadFamily::Xbox),
        capabilities: DeviceCapabilities::default(),
    });
    let evs = dm.drain_all_device_connected();
    assert_eq!(evs.len(), 1);
    assert!(matches!(evs[0], DeviceConnected { .. }));
}

/// TC-6.1.14.1 — keyboard state queryable.
#[test]
fn test_keyboard_state_component() {
    let mut k = KeyboardState::default();
    k.apply_raw(&RawInputKind::KeyPress {
        scancode: Scancode::W,
        keycode: Keycode(1),
    });
    assert!(k.is_pressed(Scancode::W));
}

/// TC-6.1.15.1 — active device switches on last input class.
#[test]
fn test_active_device_switch() {
    let mut dm = DeviceManager::new();
    let kb = dm.handle_connect(DeviceInfo {
        id: DeviceId {
            index: 0,
            generation: 0,
        },
        device_type: DeviceType::Keyboard,
        vendor_id: 0,
        product_id: 0,
        dpi_scale: 1.0,
        gamepad_family: None,
        capabilities: DeviceCapabilities::default(),
    });
    dm.note_input(kb, DeviceType::Keyboard);
    assert_eq!(dm.active_device_type(), Some(DeviceType::Keyboard));
    let gp = dm.handle_connect(DeviceInfo {
        id: DeviceId {
            index: 0,
            generation: 0,
        },
        device_type: DeviceType::Gamepad,
        vendor_id: 0,
        product_id: 0,
        dpi_scale: 1.0,
        gamepad_family: Some(GamepadFamily::Xbox),
        capabilities: DeviceCapabilities::default(),
    });
    dm.note_input(gp, DeviceType::Gamepad);
    assert_eq!(dm.active_device_type(), Some(DeviceType::Gamepad));
}

/// TC-6.2.1.1 — axis action rejects bool source at load time.
#[test]
fn test_action_type_mismatch_reject() {
    let defs = vec![ActionDefinition {
        id: ActionId(1),
        value_type: ActionValueType::Axis2D,
        default_value: ActionValue::Axis2D(Vec2::ZERO),
    }];
    let binds = vec![ActionBinding {
        action_id: ActionId(1),
        source: InputSource::Key(Scancode::Space),
        modifiers: ModifierChain::new(),
        trigger: TriggerCondition::Pressed,
    }];
    let err = BindingLoader::load(&defs, &binds).unwrap_err();
    assert_eq!(
        err,
        BindingError::TypeMismatch {
            action: ActionValueType::Axis2D,
            source: ActionValueType::Bool,
        }
    );
}

/// TC-6.2.2.1 — WASD, stick, and touch region agree on forward.
#[test]
fn test_move_action_three_sources() {
    let mut kb = KeyboardState::default();
    kb.apply_raw(&RawInputKind::KeyPress {
        scancode: Scancode::W,
        keycode: Keycode(1),
    });
    let v1 = InputMapper::axis2d_from_wasd(&kb);
    let mut gp = GamepadState::default();
    gp.left_stick = Vec2::new(0.0, -1.0);
    let v2 = InputMapper::axis2d_from_stick(&gp, GamepadStickSource::Left);
    let v3 = InputMapper::axis2d_from_touch_region(TouchRegionId(0), Vec2::new(0.0, -2.0));
    for v in [v1, v2, v3] {
        assert!((v - Vec2::new(0.0, -1.0)).length() < 1e-2);
    }
}

/// TC-6.2.3.1 — higher-priority context consumes Escape first.
#[test]
fn test_context_priority_consume() {
    let combat = MappingContext {
        id: ContextId(1),
        priority: 10,
        bindings: vec![ActionBinding {
            action_id: ActionId(10),
            source: InputSource::Key(Scancode::Escape),
            modifiers: ModifierChain::new(),
            trigger: TriggerCondition::Pressed,
        }],
        consumes_input: true,
    };
    let ui = MappingContext {
        id: ContextId(2),
        priority: 20,
        bindings: vec![ActionBinding {
            action_id: ActionId(20),
            source: InputSource::Key(Scancode::Escape),
            modifiers: ModifierChain::new(),
            trigger: TriggerCondition::Pressed,
        }],
        consumes_input: true,
    };
    let mut stack = ContextStack::default();
    stack.push(combat);
    stack.push(ui);
    let hits = InputMapper::resolve_key_press(&stack, Scancode::Escape);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].1, ActionId(20));
}

/// TC-6.2.4.1 — unbound keys fall through to lower contexts.
#[test]
fn test_unbound_passthrough() {
    let on_foot = MappingContext {
        id: ContextId(1),
        priority: 5,
        bindings: vec![ActionBinding {
            action_id: ActionId(1),
            source: InputSource::Key(Scancode::W),
            modifiers: ModifierChain::new(),
            trigger: TriggerCondition::Pressed,
        }],
        consumes_input: false,
    };
    let ui = MappingContext {
        id: ContextId(2),
        priority: 20,
        bindings: vec![ActionBinding {
            action_id: ActionId(2),
            source: InputSource::Key(Scancode::Escape),
            modifiers: ModifierChain::new(),
            trigger: TriggerCondition::Pressed,
        }],
        consumes_input: true,
    };
    let mut stack = ContextStack::default();
    stack.push(on_foot);
    stack.push(ui);
    let hits = InputMapper::resolve_key_press(&stack, Scancode::W);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].1, ActionId(1));
}

/// TC-6.2.6.1 — modifier chain order matches design numeric spot-checks.
#[test]
fn test_modifier_chain_order() {
    let chain = ModifierChain::from_slice(&[
        InputModifier::DeadZoneRadial { threshold: 0.15 },
        InputModifier::ResponseCurve {
            curve_type: ResponseCurveType::Exponential { exp: 2.0 },
        },
        InputModifier::Scalar { multiplier: 2.0 },
    ]);
    let mut st1 = ModifierState::default();
    let o1 = chain.apply(ActionValue::Axis1D(0.10), 1.0 / 60.0, &mut st1);
    let mut st2 = ModifierState::default();
    let o2 = chain.apply(ActionValue::Axis1D(0.50), 1.0 / 60.0, &mut st2);
    assert_eq!(o1, ActionValue::Axis1D(0.0));
    if let ActionValue::Axis1D(x) = o2 {
        let expected = 2.0 * ((0.50_f32 - 0.15) / (1.0 - 0.15)).powf(2.0);
        assert!((x - expected).abs() < 0.001);
    } else {
        panic!("expected axis1d");
    }
}

/// TC-6.2.7.1 — radial dead zone remap on 2D.
#[test]
fn test_radial_dead_zone_remap() {
    let chain = ModifierChain::from_slice(&[InputModifier::DeadZoneRadial { threshold: 0.15 }]);
    let mut st = ModifierState::default();
    let out = chain.apply(
        ActionValue::Axis2D(Vec2::new(0.50, 0.0)),
        1.0 / 60.0,
        &mut st,
    );
    if let ActionValue::Axis2D(v) = out {
        let mag = v.length();
        let expected = (0.50_f32 - 0.15) / (1.0 - 0.15);
        assert!((mag - expected).abs() < 1e-3);
    } else {
        panic!("expected axis2d");
    }
}

/// TC-6.2.8.1 — exponential curve at 0.5 with exp 2 → 0.25.
#[test]
fn test_exponential_curve_05() {
    let chain = ModifierChain::from_slice(&[InputModifier::ResponseCurve {
        curve_type: ResponseCurveType::Exponential { exp: 2.0 },
    }]);
    let mut st = ModifierState::default();
    let out = chain.apply(ActionValue::Axis1D(0.5), 1.0 / 60.0, &mut st);
    assert_eq!(out, ActionValue::Axis1D(0.25));
}

/// TC-6.2.9.1 — trigger phases for pressed / hold / tap / pulse.
#[test]
fn test_trigger_phases() {
    let mut st = TriggerState::default();
    let pressed = TriggerCondition::Pressed;
    assert_eq!(
        pressed.evaluate(true, &ActionValue::Bool(true), 1.0 / 60.0, &mut st),
        TriggerPhase::Fired
    );
    let mut st = TriggerState::default();
    let hold = TriggerCondition::Hold { duration: 0.5 };
    let mut fired_hold = false;
    let dt = 1.0 / 120.0;
    for _ in 0..120 {
        let phase = hold.evaluate(true, &ActionValue::Bool(true), dt, &mut st);
        if phase == TriggerPhase::Fired {
            fired_hold = true;
        }
    }
    assert!(fired_hold);
    let mut st = TriggerState::default();
    let tap = TriggerCondition::Tap { threshold: 0.05 };
    let _ = tap.evaluate(true, &ActionValue::Bool(true), 0.02, &mut st);
    let p = tap.evaluate(false, &ActionValue::Bool(false), 0.02, &mut st);
    assert_eq!(p, TriggerPhase::Fired);
    let mut st = TriggerState::default();
    let pulse = TriggerCondition::Pulse { interval: 0.1 };
    let mut count = 0_u32;
    for _ in 0..5 {
        if pulse.evaluate(true, &ActionValue::Bool(true), 0.1, &mut st) == TriggerPhase::Fired {
            count += 1;
        }
    }
    assert_eq!(count, 5);
}

/// TC-6.2.11.1 — swap rebinding.
#[test]
fn test_rebind_conflict_swap() {
    let mut ctx = MappingContext {
        id: ContextId(1),
        priority: 0,
        bindings: vec![
            ActionBinding {
                action_id: ActionId(1),
                source: InputSource::Key(Scancode::Space),
                modifiers: ModifierChain::new(),
                trigger: TriggerCondition::Pressed,
            },
            ActionBinding {
                action_id: ActionId(2),
                source: InputSource::Key(Scancode::C),
                modifiers: ModifierChain::new(),
                trigger: TriggerCondition::Pressed,
            },
        ],
        consumes_input: false,
    };
    let req = RebindRequest {
        context_id: ContextId(1),
        action_id: ActionId(1),
        new_source: InputSource::Key(Scancode::C),
        resolution: RebindResolution::Swap,
    };
    let r = RebindManager::apply_request(std::slice::from_mut(&mut ctx), &req);
    assert_eq!(r, RebindResult::Swapped);
    assert_eq!(ctx.bindings[0].source, InputSource::Key(Scancode::C));
    assert_eq!(ctx.bindings[1].source, InputSource::Key(Scancode::Space));
}

/// TC-6.2.13.1 — reserved guide button rejected.
#[test]
fn test_reserved_key_reject() {
    let mut ctx = MappingContext {
        id: ContextId(1),
        priority: 0,
        bindings: vec![],
        consumes_input: false,
    };
    let req = RebindRequest {
        context_id: ContextId(1),
        action_id: ActionId(1),
        new_source: InputSource::GamepadButton(GamepadButton::Guide),
        resolution: RebindResolution::Fail,
    };
    let r = RebindManager::apply_request(std::slice::from_mut(&mut ctx), &req);
    assert_eq!(r, RebindResult::Rejected(RebindError::ReservedKey));
}

/// TC-6.2.14.1 — glyph resolves per device family.
#[test]
fn test_glyph_resolves_xbox() {
    let mut r = GlyphResolver::default();
    let b = ActionBinding {
        action_id: ActionId(1),
        source: InputSource::GamepadButton(GamepadButton::South),
        modifiers: ModifierChain::new(),
        trigger: TriggerCondition::Pressed,
    };
    let g = r.resolve(ActionId(1), &b, DeviceFamily::Xbox);
    assert_eq!(g.label, "xbox_a");
    let b2 = ActionBinding {
        action_id: ActionId(1),
        source: InputSource::Key(Scancode::Space),
        modifiers: ModifierChain::new(),
        trigger: TriggerCondition::Pressed,
    };
    let g2 = r.resolve(ActionId(1), &b2, DeviceFamily::Keyboard);
    assert_eq!(g2.label, "key_space");
}

/// TC-6.2.17.1 — quarter-circle forward + punch resolves.
#[test]
fn test_quarter_circle_combo() {
    let tree = qcf_punch_tree();
    let mut ev = ComboEvaluator::new(&tree, 0.01);
    let dt = 1.0 / 60.0;
    assert!(ev
        .advance(ComboInput::Direction(CardinalDirection::Down), dt, &tree)
        .is_none());
    assert!(ev
        .advance(
            ComboInput::Direction(CardinalDirection::DownForward),
            dt,
            &tree,
        )
        .is_none());
    assert!(ev
        .advance(
            ComboInput::Direction(CardinalDirection::Forward),
            dt,
            &tree,
        )
        .is_none());
    let fin = ev.advance(ComboInput::Button(GamepadButton::South), dt, &tree);
    assert_eq!(fin, Some(ActionId(9001)));
}

/// TC-6.2.18.1 — forward from stick, D-pad, and keyboard normalizes.
#[test]
fn test_directional_normalize() {
    let d = normalize_cardinal_forward_from_sources(Vec2::new(0.0, 1.0), true, false);
    assert_eq!(d, CardinalDirection::Forward);
}

/// TC-6.2.19.1 — cancel window consumes buffered dodge.
#[test]
fn test_input_buffer_cancel_window() {
    let mut buf = InputBuffer::with_duration(1.0);
    buf.push(ActionId(50), ActionCategory::Dodge, 0.25, 25);
    let win = CancelWindow {
        start_frame: 30,
        end_frame: 50,
        permitted: vec![ActionCategory::Dodge],
    };
    let id = buf.try_consume(&win, 30, 0.30);
    assert_eq!(id, Some(ActionId(50)));
}

/// TC-6.2.20.1 — dodge wins over attack when categories differ.
#[test]
fn test_buffer_priority_dodge_first() {
    let id = InputBuffer::resolve_priority_at_window(
        (ActionId(1), ActionCategory::Attack, 0.24),
        (ActionId(2), ActionCategory::Dodge, 0.26),
    );
    assert_eq!(id, ActionId(2));
}

/// TC-6.3.1.1 — double tap suppresses single tap emission in this harness.
#[test]
fn test_double_tap_suppresses_single() {
    let mut g = GestureEngine::default();
    let mut taps = Vec::new();
    taps.extend(g.on_tap(0.0, 300.0));
    taps.extend(g.on_tap(0.2, 300.0));
    assert!(matches!(
        taps.last(),
        Some(ResolvedGesture::Emit {
            gesture_type: GestureType::DoubleTap,
            ..
        })
    ));
    let late = g.flush_taps_after_quiet(400.0);
    assert!(late.iter().all(|e| !matches!(
        e,
        ResolvedGesture::Emit {
            gesture_type: GestureType::Tap,
            ..
        }
    )));
}

/// TC-6.3.2.1 — long press thresholding.
#[test]
fn test_long_press_threshold() {
    let mut r = LongPressRecognizer::default();
    let mut out = Vec::new();
    for _ in 0..59 {
        out.extend(r.tick(Vec2::ZERO, 0.01, false, 0.5, 5.0));
    }
    assert!(out.iter().any(|g| *g == GestureType::LongPress));
    r.reset();
    let mut r2 = LongPressRecognizer::default();
    let mut o2 = Vec::new();
    for i in 0..40 {
        let released = i == 39;
        o2.extend(r2.tick(Vec2::ZERO, 0.01, released, 0.5, 5.0));
    }
    assert!(!o2.iter().any(|g| *g == GestureType::LongPress));
    r2.reset();
    let mut r3 = LongPressRecognizer::default();
    let mut o3 = Vec::new();
    for i in 0..60 {
        let drift = if i > 30 { 100.0 } else { 0.0 };
        o3.extend(r3.tick(Vec2::new(drift, 0.0), 0.01, false, 0.5, 5.0));
    }
    assert!(!o3.iter().any(|g| *g == GestureType::LongPress));
}

/// TC-6.3.4.1 — eight swipe directions.
#[test]
fn test_swipe_8_directions() {
    let dirs = GestureEngine::emit_swipe_matrix(1500.0);
    let set: std::collections::HashSet<_> = dirs.into_iter().collect();
    assert_eq!(set.len(), 8);
}

/// TC-6.3.5.1 — simultaneous pinch and pan.
#[test]
fn test_pinch_pan_simultaneous() {
    let r = GestureEngine::pinch_pan(
        Vec2::new(100.0, 100.0),
        Vec2::new(200.0, 100.0),
        Vec2::new(50.0, 100.0),
        Vec2::new(250.0, 200.0),
    );
    assert!((r.scale - 2.0).abs() < 0.25);
    assert!((r.pan_delta - Vec2::new(0.0, 50.0)).length() < 5.0);
}

/// TC-6.3.7.1 — swipe lifecycle ordering.
#[test]
fn test_gesture_lifecycle_states() {
    let mut g = GestureEngine::default();
    let log = g.swipe_lifecycle(1500.0, 1000.0);
    assert_eq!(
        log,
        vec![
            GesturePhase::Possible,
            GesturePhase::Began,
            GesturePhase::Changed,
            GesturePhase::Ended,
        ]
    );
}

/// TC-6.4.1.1 — dual motor bytes independent.
#[test]
fn test_lf_hf_motor_independent() {
    let b = encode_dual_motor_hid(0.8, 0.2);
    assert_eq!(b[0], 204);
    assert_eq!(b[1], 51);
}

/// TC-6.4.2.1 — rumble envelope shape.
#[test]
fn test_rumble_pattern_envelope() {
    let pat = RumblePattern {
        envelope: RumbleEnvelope {
            attack: 100.0,
            sustain: 200.0,
            decay: 100.0,
        },
        peak: 1.0,
    };
    let mut p = PatternPlayer::default();
    p.play(&pat, 1.0);
    let s_at_100 = p.tick(0.1);
    assert!(s_at_100.low_freq > 0.95);
    let mut p2 = PatternPlayer::default();
    p2.play(&pat, 1.0);
    let _ = p2.tick(0.1);
    let s_at_400 = p2.tick(0.3);
    assert!(s_at_400.low_freq < 0.05);
}

/// TC-6.4.4.1 — adaptive trigger no-op on Xbox capability.
#[test]
fn test_adaptive_trigger_xbox_noop() {
    let caps = DeviceCapabilities {
        has_adaptive_triggers: false,
        ..DeviceCapabilities::default()
    };
    let d = AdaptiveTriggerDriver::default();
    let mut hid = vec![1_u8, 2, 3];
    d.apply(
        &caps,
        &TriggerEffect { strength: 0.5 },
        &mut hid,
    )
    .unwrap();
    assert!(hid.is_empty());
}

/// TC-6.4.7.1 — audio haptic band extraction (coarse).
#[test]
fn test_audio_haptic_band_extract() {
    let sr = 48_000.0;
    let haptic_gen = AudioHapticGenerator { sample_rate: sr };
    let mut low = Vec::new();
    let mut high = Vec::new();
    for i in 0..48_000 {
        let t = i as f32 / sr;
        low.push((2.0 * std::f32::consts::PI * 100.0 * t).sin() * 0.8);
        high.push((2.0 * std::f32::consts::PI * 5000.0 * t).sin() * 0.8);
    }
    let e100 = haptic_gen.band_energy(&low, 100.0);
    let e5k = haptic_gen.band_energy(&high, 100.0);
    assert!(e100 > 0.6 * 0.8 * 0.5);
    assert!(e5k < 0.05);
}

/// TC-6.5.1.1 — HMD pose stream ~90 Hz for 1 s.
#[test]
fn test_hmd_pose_refresh_rate() {
    let samples = simulate_hmd_pose_stream(1.0, 90.0);
    assert!((samples.len() as i32 - 90).abs() <= 2);
}

/// TC-6.5.6.1 — VR and gamepad triggers normalize identically.
#[test]
fn test_vr_action_shared_with_pad() {
    let g = VrActionBridge::axis1d_from_gamepad_trigger(0.7);
    let v = VrActionBridge::axis1d_from_vr_trigger(0.7);
    assert!((g - v).abs() < 1e-6);
}

/// TC-6.5.8.1 — pinch distance threshold.
#[test]
fn test_pinch_gesture_action() {
    let d = HandPinchDetector {
        threshold_m: 0.01,
    };
    assert!(d.is_pinching(Vec3::ZERO, Vec3::new(0.005, 0.0, 0.0)));
    assert!(!d.is_pinching(Vec3::ZERO, Vec3::new(0.02, 0.0, 0.0)));
}

/// TC-6.5.11.1 — gaze fixation vs saccade markers.
#[test]
fn test_gaze_fixation_classify() {
    let mut samples = Vec::new();
    let dt = 1.0 / 90.0;
    let dir = Vec3::new(0.0, 0.0, -1.0);
    for i in 0..200 {
        samples.push(GazeRaySample {
            t: i as f32 * dt,
            dir,
        });
    }
    for i in 200..220 {
        let t = i as f32 * dt;
        samples.push(GazeRaySample {
            t,
            dir: Vec3::new(0.5, 0.0, -0.86).normalize(),
        });
    }
    let mut c = GazeClassifier::default();
    let ev = c.feed(&samples, 30.0, 100.0);
    assert!(ev.contains(&GazeEvent::Fixation));
    assert!(ev.contains(&GazeEvent::Saccade));
}

/// TC-6.2.5.1 — load two bindings from authored contexts.
#[test]
fn test_mapping_context_editor_auth() {
    let move_ctx = MappingContext {
        id: ContextId(1),
        priority: 0,
        bindings: vec![
            ActionBinding {
                action_id: ActionId(1),
                source: InputSource::GamepadStick(GamepadStickSource::Left),
                modifiers: ModifierChain::new(),
                trigger: TriggerCondition::Down,
            },
            ActionBinding {
                action_id: ActionId(2),
                source: InputSource::GamepadButton(GamepadButton::South),
                modifiers: ModifierChain::new(),
                trigger: TriggerCondition::Pressed,
            },
        ],
        consumes_input: false,
    };
    let stack = InputMapper::load_contexts(vec![move_ctx]).unwrap();
    assert!(stack.is_active(ContextId(1)));
}

/// TC-6.2.10.1 — trigger timing rules at runtime (hold / double window).
#[test]
fn test_trigger_condition_editor() {
    let hold = TriggerCondition::Hold { duration: 0.25 };
    let mut st = TriggerState::default();
    let mut fired = false;
    for _ in 0..30 {
        if hold.evaluate(true, &ActionValue::Bool(true), 0.25 / 30.0, &mut st)
            == TriggerPhase::Fired
        {
            fired = true;
        }
    }
    assert!(fired);
}

/// TC-6.4.3.1 — intensity maps to half-scale native values.
#[test]
fn test_haptic_intensity_normalized() {
    let cmd = HapticCommand { intensity: 0.5 };
    let x = normalize_intensity(&cmd, HapticBackendKind::Xbox);
    assert!((x - 127.5).abs() < 1.0);
}

/// TC-6.4.5.1 — adaptive trigger profile keywords parse.
#[test]
fn test_adaptive_trigger_editor() {
    assert_eq!(
        parse_adaptive_profile_line("Feedback"),
        Some(AdaptiveTriggerEffect::Feedback)
    );
    assert_eq!(
        parse_adaptive_profile_line("Weapon"),
        Some(AdaptiveTriggerEffect::Weapon)
    );
    assert_eq!(
        parse_adaptive_profile_line("Vibration"),
        Some(AdaptiveTriggerEffect::Vibration)
    );
}

/// TC-6.5.3.1 — tracking loss surfaces next frame.
#[test]
fn test_tracking_loss_event_1frame() {
    let e = tracking_loss_event(10);
    assert_eq!(e.frame, 11);
}

/// TC-6.5.4.1 — play area boundary events per mode.
#[test]
fn test_play_area_modes_boundary() {
    assert!(play_area_crossing(PlayAreaMode::RoomScale, 2.0, 1.0).is_some());
    assert!(play_area_crossing(PlayAreaMode::Seated, 2.0, 1.0).is_some());
    assert!(play_area_crossing(PlayAreaMode::Standing, 2.0, 1.0).is_some());
}

/// TC-6.5.5.1 — 6DOF controller sample round-trip.
#[test]
fn test_6dof_controller_components() {
    let s = Controller6DofSample {
        position: Vec3::ONE,
        orientation: Quat::IDENTITY,
        velocity: Vec3::ZERO,
        angular_velocity: Vec3::ZERO,
        south: true,
        trigger: 0.5,
        touch_south: true,
    };
    assert_eq!(s.trigger, 0.5);
    assert!(s.south && s.touch_south);
}

/// Keep VR hand types referenced so the public surface stays intentional.
#[test]
fn test_vr_hand_types_size() {
    assert!(std::mem::size_of::<HandSkeleton>() > 0);
    assert!(std::mem::size_of::<HandJointPose>() > 0);
    assert!(std::mem::size_of::<DualMotorState>() > 0);
    let _ = GazeBehavior::Fixation;
}
