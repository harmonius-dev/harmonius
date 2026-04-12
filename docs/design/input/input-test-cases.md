# Input System — Test Cases

Companion to [input.md](input.md).

Test case IDs use `TC-6.G.Z.N` format where G is the group (1=devices, 2=actions, 3=gestures,
4=haptics, 5=VR). Every row links to a specific R-X.Y.Z or F-X.Y.Z.

## Unit Tests

| ID            | Name                                | Req         |
|---------------|-------------------------------------|-------------|
| TC-6.1.1.1    | `test_keyboard_event_carries_both`  | R-6.1.1     |
| TC-6.1.2.1    | `test_scancode_w_cross_platform`    | R-6.1.2     |
| TC-6.1.3.1    | `test_mouse_subpixel_delta`         | R-6.1.3     |
| TC-6.1.4.1    | `test_dpi_normalized_delta`         | R-6.1.4     |
| TC-6.1.5.1    | `test_scroll_wheel_vs_trackpad`     | R-6.1.5     |
| TC-6.1.6.1    | `test_gamepad_south_unification`    | R-6.1.6     |
| TC-6.1.7.1    | `test_gyro_madgwick_orientation`    | R-6.1.7     |
| TC-6.1.8.1    | `test_capability_flags_dualsense`   | R-6.1.8     |
| TC-6.1.9.1    | `test_touch_10_simultaneous`        | R-6.1.9     |
| TC-6.1.11.1   | `test_device_connect_event`         | R-6.1.11    |
| TC-6.1.14.1   | `test_keyboard_state_component`     | R-6.1.14    |
| TC-6.1.15.1   | `test_active_device_switch`         | R-6.1.15    |
| TC-6.2.1.1    | `test_action_type_mismatch_reject`  | R-6.2.1     |
| TC-6.2.2.1    | `test_move_action_three_sources`    | R-6.2.2     |
| TC-6.2.3.1    | `test_context_priority_consume`     | R-6.2.3     |
| TC-6.2.4.1    | `test_unbound_passthrough`          | R-6.2.4     |
| TC-6.2.6.1    | `test_modifier_chain_order`         | R-6.2.6     |
| TC-6.2.7.1    | `test_radial_dead_zone_remap`       | R-6.2.7     |
| TC-6.2.8.1    | `test_exponential_curve_05`         | R-6.2.8     |
| TC-6.2.9.1    | `test_trigger_phases`               | R-6.2.9     |
| TC-6.2.11.1   | `test_rebind_conflict_swap`         | R-6.2.11    |
| TC-6.2.13.1   | `test_reserved_key_reject`          | R-6.2.13    |
| TC-6.2.14.1   | `test_glyph_resolves_xbox`          | R-6.2.14    |
| TC-6.2.17.1   | `test_quarter_circle_combo`         | R-6.2.17    |
| TC-6.2.18.1   | `test_directional_normalize`        | R-6.2.18    |
| TC-6.2.19.1   | `test_input_buffer_cancel_window`   | R-6.2.19    |
| TC-6.2.20.1   | `test_buffer_priority_dodge_first`  | R-6.2.20    |
| TC-6.3.1.1    | `test_double_tap_suppresses_single` | R-6.3.1     |
| TC-6.3.2.1    | `test_long_press_threshold`         | R-6.3.2     |
| TC-6.3.4.1    | `test_swipe_8_directions`           | R-6.3.4     |
| TC-6.3.5.1    | `test_pinch_pan_simultaneous`       | R-6.3.5     |
| TC-6.3.7.1    | `test_gesture_lifecycle_states`     | R-6.3.7     |
| TC-6.4.1.1    | `test_lf_hf_motor_independent`      | R-6.4.1     |
| TC-6.4.2.1    | `test_rumble_pattern_envelope`      | R-6.4.2     |
| TC-6.4.4.1    | `test_adaptive_trigger_xbox_noop`   | R-6.4.4     |
| TC-6.4.7.1    | `test_audio_haptic_band_extract`    | R-6.4.7     |
| TC-6.5.1.1    | `test_hmd_pose_refresh_rate`        | R-6.5.1     |
| TC-6.5.6.1    | `test_vr_action_shared_with_pad`    | R-6.5.6     |
| TC-6.5.8.1    | `test_pinch_gesture_action`         | R-6.5.8     |
| TC-6.5.11.1   | `test_gaze_fixation_classify`       | R-6.5.11    |
| TC-6.2.5.1    | `test_mapping_context_editor_auth`  | R-6.2.5     |
| TC-6.2.10.1   | `test_trigger_condition_editor`     | R-6.2.10    |
| TC-6.4.3.1    | `test_haptic_intensity_normalized`  | R-6.4.3     |
| TC-6.4.5.1    | `test_adaptive_trigger_editor`      | R-6.4.5     |
| TC-6.5.3.1    | `test_tracking_loss_event_1frame`   | R-6.5.3     |
| TC-6.5.4.1    | `test_play_area_modes_boundary`     | R-6.5.4     |
| TC-6.5.5.1    | `test_6dof_controller_components`   | R-6.5.5     |

1. **TC-6.1.1.1** `test_keyboard_event_carries_both` — Inject press, release, and repeat events for
   the W key on each platform. Assert each `RawInputEvent` carries both a non-zero `Scancode` and a
   non-zero `Keycode`.
   - Input:
     `RawInputEvent { kind: RawInputKind::KeyPress { scancode: Scancode::W, keycode: ...}, .. }` for
     each phase
   - Expected: `event.scancode != Scancode::Unknown` and `event.keycode.0 != 0` for press, release,
     and repeat

2. **TC-6.1.2.1** `test_scancode_w_cross_platform` — Map the platform-native W scancode through the
   normalization table on Windows (`0x11`), macOS (`0x0D`), Linux (`KEY_W`).
   - Input: three native scancodes for the W key
   - Expected: all map to `Scancode::W`; full 104-key set has zero collisions

3. **TC-6.1.3.1** `test_mouse_subpixel_delta` — Inject a raw mouse delta of `0.00390625` (1/256)
   horizontal. Assert `MouseState.delta.x == 0.00390625` after one device poll.
   - Input: `RawInputKind::MouseMotion { dx: 0.00390625, dy: 0.0 }`
   - Expected: `mouse_state.delta.x` equals `0.00390625` exactly (no truncation)

4. **TC-6.1.4.1** `test_dpi_normalized_delta` — Inject identical raw delta `(10, 0)` once with DPI
   scale 1.0 and once with DPI scale 2.0. Assert normalized values are equal within 1%.
   - Input: same delta, two `DeviceInfo { dpi_scale: 1.0 | 2.0 }`
   - Expected: `abs(normalized_a - normalized_b) / normalized_a < 0.01`

5. **TC-6.1.5.1** `test_scroll_wheel_vs_trackpad` — Inject one mouse wheel notch (raw `120`) and a
   trackpad scroll of equivalent magnitude (raw `1.0` continuous).
   - Input: two `RawInputKind::Scroll` events, one discrete + one continuous
   - Expected: `MouseState.scroll.y` differs by `< 5%` between the two normalized outputs

6. **TC-6.1.6.1** `test_gamepad_south_unification` — Press the south-face button on simulated Xbox,
   DualSense, and Switch Pro devices. Read `GamepadState`.
   - Input: three `DeviceType::Gamepad { family: Xbox | DualSense | SwitchPro }` connected; press
     south on each
   - Expected: each `GamepadState.pressed` set contains `GamepadButton::South`

7. **TC-6.1.7.1** `test_gyro_madgwick_orientation` — Feed 1000 frames of synthesized gyro/accel data
   simulating a 90° pitch rotation. Run Madgwick fusion.
   - Input: `RawInputKind::Motion { gyro: ..., accel: ... }` over 10 s
   - Expected: final quaternion within 5° of `Quat::from_rotation_x(PI/2)`

8. **TC-6.1.8.1** `test_capability_flags_dualsense` — Connect a simulated DualSense device. Query
   `DeviceCapabilities` from `DeviceManager`.
   - Input: `DeviceInfo { vendor: 0x054C, product: 0x0CE6, .. }`
   - Expected: `caps.has_adaptive_triggers == true`, `caps.has_hd_rumble == true`,
     `caps.has_touchpad == true`; on Xbox simulated device, `has_adaptive_triggers == false`

9. **TC-6.1.9.1** `test_touch_10_simultaneous` — Inject 10 simultaneous `TouchContact` events with
   distinct `FingerId` values, varied positions and pressures.
   - Input: 10 contacts, `pressure ∈ [0.1, 1.0]`, distinct positions
   - Expected: `touch_state.contacts.len() == 10`; each `FingerId` unique; pressures preserved

10. **TC-6.1.11.1** `test_device_connect_event` — Simulate gamepad hot-plug. Run one frame of
    `device_poll_system`.
    - Input: `DeviceManager::on_hotplug(DeviceInfo { .. })` between frames
    - Expected: exactly one `DeviceConnected { device_id }` event in the ECS event queue at the next
      `Update` schedule run

11. **TC-6.1.14.1** `test_keyboard_state_component` — Press the W key, run device poll system, then
    query the `KeyboardState` ECS component on the input singleton entity.
    - Input: `RawInputKind::KeyPress { scancode: Scancode::W, .. }`
    - Expected: `keyboard_state.is_pressed(Scancode::W) == true`; component is reachable via
      `Query<&KeyboardState>`

12. **TC-6.1.15.1** `test_active_device_switch` — Send keyboard input, then in the next frame send
    gamepad input. Read `ActiveDeviceType` resource.
    - Input: keyboard event at `t0`, gamepad event at `t1 = t0 + 1 frame`
    - Expected: `ActiveDeviceType` is `Keyboard` after `t0`; switches to `Gamepad` within the same
      frame as the gamepad event

13. **TC-6.2.1.1** `test_action_type_mismatch_reject` — Define an action `Move` of type
    `ActionValueType::Axis2D`. Attempt to bind a boolean keyboard scancode to it.
    - Input: `ActionDefinition { value_type: ActionValueType::Axis2D }`, source
      `InputSource::Key(Scancode::Space)`
    - Expected: `BindingLoader::load()` returns
      `Err(BindingError::TypeMismatch { action: ..., source: ... })`

14. **TC-6.2.2.1** `test_move_action_three_sources` — Bind `Move` (Axis2D) to WASD, gamepad left
    stick, and a virtual touch joystick. Push each to maximum forward.
    - Input: keyboard W down, gamepad `axis_y = -1.0`, touch joystick `(0, -1)`
    - Expected: all three resolve to `ActionValue::Axis2D(Vec2::new(0.0, -1.0))` within `1e-2`

15. **TC-6.2.3.1** `test_context_priority_consume` — Push `Combat` (priority 10,
    `consumes_input: true`) and `UIMenu` (priority 20, `consumes_input: true`) both binding
    `Scancode::Escape`. Press Escape.
    - Input: stack `[Combat(10), UIMenu(20)]`, both bind Escape
    - Expected: only `UIMenu` action fires; `Combat` action does not fire

16. **TC-6.2.4.1** `test_unbound_passthrough` — Push `UIMenu` (binds only Escape) and `OnFoot`
    (binds WASD). Press W.
    - Input: stack `[OnFoot, UIMenu]`, press W
    - Expected: `OnFoot::Move::Forward` fires; UIMenu binding receives no event

17. **TC-6.2.6.1** `test_modifier_chain_order` — Build a `ModifierChain` with
    `[DeadZone(radial, 0.15), ResponseCurve(Exponential { exp: 2.0 }), Scalar(2.0)]`. Feed `0.10`
    and `0.50`.
    - Input: two raw scalar inputs through identical chain
    - Expected: `0.10 -> 0.0` (dead zone);
      `0.50 -> 2.0 * pow((0.50 - 0.15) / (1.0 - 0.15), 2.0) ≈ 0.339 ± 0.001`

18. **TC-6.2.7.1** `test_radial_dead_zone_remap` — Apply radial dead zone `0.15` to a 2D input with
    magnitude `0.50`.
    - Input: `Vec2::new(0.50, 0.0)`, radial dead zone threshold `0.15`
    - Expected: output magnitude == `(0.50 - 0.15) / (1.0 - 0.15) ≈ 0.4118` along the same direction

19. **TC-6.2.8.1** `test_exponential_curve_05` — Apply `ResponseCurve::Exponential { exp: 2.0 }` to
    scalar input `0.5`.
    - Input: `value = 0.5`, exponent `2.0`
    - Expected: output == `0.25` exactly

20. **TC-6.2.9.1** `test_trigger_phases` — Test each `TriggerCondition` variant. Pressed: assert
    fires only on the down frame. Hold: assert fires after configured duration. Tap: assert fires on
    quick press-release. Pulse: assert fires periodically.
    - Input: simulated key state transitions over 60 frames
    - Expected: `Pressed -> Started` exactly once; `Hold(500ms) -> Performed` exactly once after 500
      ms; `Pulse(100ms) -> Performed` 5 times over 500 ms

21. **TC-6.2.11.1** `test_rebind_conflict_swap` — Two actions `Jump` (Space) and `Crouch` (C). Issue
    `RebindRequest { action: Jump, new_source: Key(C), resolution: Swap }`.
    - Input: existing bindings `[Jump=Space, Crouch=C]`; rebind Jump to C
    - Expected: result == `RebindResult::Swapped`; final bindings `[Jump=C, Crouch=Space]`

22. **TC-6.2.13.1** `test_reserved_key_reject` — Attempt to rebind any action to the PS button on
    DualSense.
    - Input: `RebindRequest { new_source: GamepadButton(GamepadButton::Home), .. }`
    - Expected: `RebindResult::Rejected { reason: RebindError::ReservedKey }`

23. **TC-6.2.14.1** `test_glyph_resolves_xbox` — Set `ActiveDeviceType = Gamepad(Xbox)`. Resolve
    glyph for the `Jump` action bound to gamepad south.
    - Input: `GlyphResolver::resolve(action_id, DeviceFamily::Xbox)`
    - Expected: `ResolvedGlyph { atlas_id: ..., sprite_name: "xbox_a", .. }`; switching to
      `Keyboard` returns `"key_space"`

24. **TC-6.2.17.1** `test_quarter_circle_combo` — Define a `ComboTree` for QCF (down → down-forward
    → forward → punch) with leniency window 200 ms.
    - Input: directional sequence within window followed by punch
    - Expected: `ComboEvaluator::poll()` emits `ResolvedCombo("qcf_punch")`; sequence with last
      input outside window resets to root

25. **TC-6.2.18.1** `test_directional_normalize` — Apply `ComboInput::Direction(Forward)` from stick
    at `(0, 1)`, D-pad up, and W key.
    - Input: three different sources representing "forward"
    - Expected: all normalize to `ComboInput::Direction(CardinalDir::Forward)`

26. **TC-6.2.19.1** `test_input_buffer_cancel_window` — Trigger an ability with cancel window
    `[300, 500] ms`. Press dodge at `250 ms`.
    - Input: ability start `t=0`, dodge press `t=250 ms`
    - Expected: `InputBuffer::execute_at(300 ms)` fires the dodge; ability cancelled

27. **TC-6.2.20.1** `test_buffer_priority_dodge_first` — Buffer dodge and attack inputs in the same
    recovery window.
    - Input: attack press at `t=240 ms`, dodge press at `t=260 ms`, cancel window opens at `300 ms`
    - Expected: dodge executes first (priority `Dodge > Attack`); attack discarded

28. **TC-6.3.1.1** `test_double_tap_suppresses_single` — Inject two taps within
    `inter_tap_ms = 300`. Wait `400 ms`.
    - Input: two `Tap` recognizer inputs at `t=0` and `t=200 ms`
    - Expected: `ResolvedGesture::DoubleTap` event fires; zero `ResolvedGesture::Tap` events

29. **TC-6.3.2.1** `test_long_press_threshold` — Hold a touch for 600 ms with threshold 500 ms.
    Repeat with 400 ms hold and a 600 ms hold that drifts past distance tolerance.
    - Input: three test cases
    - Expected: case 1 fires `LongPress`; case 2 does not; case 3 does not (distance fail)

30. **TC-6.3.4.1** `test_swipe_8_directions` — For each of the 8 cardinal/diagonal directions,
    inject a swipe above the velocity threshold.
    - Input: 8 swipe traces, each at `velocity = 1500 px/s`
    - Expected: 8 distinct `SwipeDirection` enum values are emitted, one per input

31. **TC-6.3.5.1** `test_pinch_pan_simultaneous` — Inject two-finger trace where fingers move apart
    (pinch) and the centroid translates (pan).
    - Input: f1 at `(100, 100) -> (50, 100)`, f2 at `(200, 100) -> (250, 200)`
    - Expected: both `ResolvedGesture::Pinch { scale: ~2.0 }` and
      `ResolvedGesture::Pan { delta: ~(0, 50) }` fire in the same frame

32. **TC-6.3.7.1** `test_gesture_lifecycle_states` — Track a swipe through its full lifecycle.
    - Input: touch begin → move (above threshold) → end
    - Expected: `GesturePhase` transitions in order `Possible -> Began -> Changed -> Ended`; no
      skipped state

33. **TC-6.4.1.1** `test_lf_hf_motor_independent` — Set `DualMotorState { low: 0.8, high: 0.2 }`.
    Inspect the HID output report sent to the platform driver.
    - Input: `DualMotorState { low: 0.8, high: 0.2 }`
    - Expected: HID report bytes for low motor encode `0.8`; high motor encode `0.2`; ranges
      independent

34. **TC-6.4.2.1** `test_rumble_pattern_envelope` — Define `RumblePattern` with attack 100 ms,
    sustain 200 ms, decay 100 ms, peak 1.0. Run `PatternPlayer::tick` over 500 ms.
    - Input: `RumbleEnvelope { attack: 100, sustain: 200, decay: 100, peak: 1.0 }`
    - Expected: motor amplitude curve matches envelope within 5 ms timing tolerance; peak reached at
      `t = 100 ms`; zero at `t = 400 ms`

35. **TC-6.4.4.1** `test_adaptive_trigger_xbox_noop` — Apply
    `TriggerEffect::Resistance { strength: 0.5 }` to L2 on a simulated Xbox device.
    - Input: Xbox `DeviceCapabilities { has_adaptive_triggers: false }`
    - Expected: `AdaptiveTriggerDriver::apply` returns `Ok(())`; zero HID output bytes written; no
      panic, no error

36. **TC-6.4.7.1** `test_audio_haptic_band_extract` — Feed a 100 Hz sine at amplitude 0.8 into
    `AudioHapticGenerator`. Then feed a 5 kHz sine.
    - Input: two test signals, 1 s each at 48 kHz
    - Expected: 100 Hz output amplitude in `[0.6, 0.96]`; 5 kHz output amplitude `< 0.05`;
      audio-to-haptic latency `< 10 ms`

37. **TC-6.5.1.1** `test_hmd_pose_refresh_rate` — Run VR runtime in headless mode at 90 Hz for 1
    second. Capture every `HmdPose` write timestamp.
    - Input: simulated OpenXR runtime, 90 Hz refresh
    - Expected: 90 ± 2 pose updates within the 1 s window; intervals within `±2 ms` of `11.11 ms`

38. **TC-6.5.6.1** `test_vr_action_shared_with_pad` — Bind `Fire` to both `GamepadAxis::TriggerR`
    and VR right-controller trigger. Pull each.
    - Input: gamepad trigger value `0.7`; VR controller trigger value `0.7`
    - Expected: both produce `ActionState { value: ActionValue::Axis1D(0.7), .. }`; gameplay
      branches receive identical state

39. **TC-6.5.8.1** `test_pinch_gesture_action` — Perform a hand-tracking pinch (thumb tip to index
    tip distance < 1 cm). Read action state.
    - Input: 26-joint hand pose with pinch geometry
    - Expected: `ActionState` for `pinch_action` is `Bool(true)`; releasing the pinch returns
      `Bool(false)` within one frame

40. **TC-6.5.11.1** `test_gaze_fixation_classify` — Feed 500 ms of stable gaze (angular velocity <
    30°/s), then a saccade (angular velocity > 100°/s).
    - Input: stream of `GazeRay` samples at 90 Hz
    - Expected: one `GazeEvent::Fixation` after 200 ms of stability; one `GazeEvent::Saccade` when
      angular velocity crosses threshold

41. **TC-6.2.5.1** `test_mapping_context_editor_auth` — Author a mapping context with two bindings
    entirely through the visual editor graph and load it into the input system.
    - Input: editor-saved context file with bindings `{Move -> GamepadLStick, Jump -> South}`
    - Expected: context loads at runtime with zero source-code changes; both actions produce values
      when the corresponding devices fire

42. **TC-6.2.10.1** `test_trigger_condition_editor` — Configure trigger conditions (hold 250 ms,
    double-tap 200 ms) per action binding in the visual editor and preview timing.
    - Input: `ActionBinding { trigger: TriggerRule::Hold(250ms) }`,
      `ActionBinding { trigger: TriggerRule::DoubleTap(200ms) }`
    - Expected: editor preview displays timing graph; at runtime, hold fires after 250 ms and
      double-tap fires only when two taps arrive within 200 ms

43. **TC-6.4.3.1** `test_haptic_intensity_normalized` — Issue `HapticCommand { intensity: 0.5 }` to
    each supported backend and read back the device-native command.
    - Input: intensity 0.5 for Xbox, DualSense, Switch, Steam Input backends
    - Expected: each backend normalizes the input to its native scale (0-255 or 0.0-1.0) such that
      0.5 maps to the half-scale native value on every backend

44. **TC-6.4.5.1** `test_adaptive_trigger_editor` — Author an adaptive trigger effect per trigger
    (Feedback, Weapon, Vibration) in the visual editor and apply at runtime.
    - Input: editor-saved trigger profiles for left and right trigger
    - Expected: profiles parse into `AdaptiveTriggerEffect` enum variants; at runtime DualSense
      trigger responds according to configured effect

45. **TC-6.5.3.1** `test_tracking_loss_event_1frame` — Simulate VR tracking loss at frame N.
    - Input: platform tracking-lost signal dispatched during frame N
    - Expected: `TrackingLossEvent` emitted no later than frame N+1; configured editor response
      (pause/blackout) triggered

46. **TC-6.5.4.1** `test_play_area_modes_boundary` — Configure play area in room-scale, seated, and
    standing modes and trip the guardian/chaperone boundary in each.
    - Input: three `PlayAreaMode` configurations, HMD position crossing boundary
    - Expected: each mode produces correct boundary event (enter/exit); mode-specific limits applied

47. **TC-6.5.5.1** `test_6dof_controller_components` — Connect a 6DOF controller and sample
    position, orientation, velocity, angular velocity, button states, analog axes, and capacitive
    touch.
    - Input: simulated controller sample stream
    - Expected: each datum exposed through dedicated ECS components; values match simulated input
      stream within tolerance

## Integration Tests

| ID            | Name                              | Req         |
|---------------|-----------------------------------|-------------|
| TC-6.1.I.1    | `test_hotplug_no_frame_spike`     | R-6.1.12    |
| TC-6.2.I.1    | `test_rebind_persist_restore`     | R-6.2.12    |
| TC-6.2.I.2    | `test_replay_deterministic`       | R-6.2.16    |
| TC-6.2.I.3    | `test_glyph_swap_one_frame`       | R-6.2.14    |
| TC-6.3.I.1    | `test_dpi_scaled_threshold`       | R-6.3.3     |
| TC-6.4.I.1    | `test_haptic_observer_dispatch`   | R-6.4.10    |
| TC-6.5.I.1    | `test_motion_to_photon_under_20ms`| R-6.5.2     |
| TC-6.5.I.2    | `test_controller_to_hand_swap`    | R-6.5.10    |

1. **TC-6.1.I.1** `test_hotplug_no_frame_spike` — Connect and disconnect a simulated gamepad 100
   times during active gameplay. Profile per-frame time.
   - Input: 100 hot-plug cycles over 10 s of normal gameplay
   - Expected: no frame exceeds baseline + 1 ms; enumeration runs on background task; zero dropped
     frames

2. **TC-6.2.I.1** `test_rebind_persist_restore` — Rebind 20 actions; trigger
   `RebindManager::save()`. Restart engine; trigger `RebindManager::load()`.
   - Input: 20 `RebindRequest` operations followed by save and reload
   - Expected: save completes within 100 ms; load completes within 50 ms; all 20 bindings restored
     exactly

3. **TC-6.2.I.2** `test_replay_deterministic` — Record 30 s of gameplay input via `InputRecorder`.
   Play back via `InputPlayback`. Hash final game state.
   - Input: 30 s of mixed keyboard/gamepad/touch input
   - Expected: `state_hash(replay) == state_hash(original)`; speed control 0.5x..4.0x produces
     correct timing; frame stepping advances exactly one input frame at a time

4. **TC-6.2.I.3** `test_glyph_swap_one_frame` — Switch the `ActiveDeviceType` from `Keyboard` to
   `Gamepad(Xbox)` mid-frame. Capture displayed glyphs.
   - Input: keyboard input at `t-1`; gamepad input at `t`
   - Expected: every glyph displayed at frame `t+1` resolves to the Xbox atlas; zero stale keyboard
     glyphs

5. **TC-6.3.I.1** `test_dpi_scaled_threshold` — Configure tap drift threshold = 8 px at base DPI.
   Run gesture engine on a simulated 2x DPI display.
   - Input: tap drift of 16 px on a 2x DPI display
   - Expected: tap is recognized; on a 1x display the same drift would not be (threshold scales
     proportionally)

6. **TC-6.4.I.1** `test_haptic_observer_dispatch` — Register an ECS observer on `HitEvent`
   triggering a `FeedbackProfile`. Emit `HitEvent` from a gameplay system.
   - Input: ECS observer wired to profile `"impact_heavy"`; emit `HitEvent`
   - Expected: profile plays on the active controller; rumble + adaptive trigger + HD haptic layers
     all activate on DualSense; falls back to rumble-only on Xbox

7. **TC-6.5.I.1** `test_motion_to_photon_under_20ms` — Capture timestamp of last `HmdPose`
   submission and the scanout time on the headset compositor for 1000 frames.
   - Input: VR scene running at 90 Hz with late-latching enabled
   - Expected: `mean(scanout - pose_submit) < 20 ms`; `p99 < one frame interval (11.11 ms)`

8. **TC-6.5.I.2** `test_controller_to_hand_swap` — Begin with controllers held; release them. Wait.
   Pick them up.
   - Input: simulated grip-release events on both Touch controllers
   - Expected: `ActiveTrackingMode` switches to `Hands` within 1 s of release; switches back to
     `Controllers` within 1 s of pickup

## Benchmarks

| ID           | Benchmark                              | Target     | Req          |
|--------------|----------------------------------------|------------|--------------|
| TC-6.1.B.1   | Device enumeration (4 devices)         | < 5 ms     | R-6.1.13     |
| TC-6.2.B.1   | Modifier chain (10 stages)             | < 1 us     | R-6.2.6      |
| TC-6.2.B.2   | Action resolve per frame (50 actions)  | < 50 us    | R-6.2.2      |
| TC-6.3.B.1   | Gesture recognition (10 fingers)       | < 100 us   | R-6.3.7      |
| TC-6.5.B.1   | Hand tracking joint solve (26 joints)  | < 1 ms     | R-6.5.7      |

1. **TC-6.1.B.1** — `DeviceManager::enumerate_all` over 4 connected devices (keyboard, mouse, Xbox
   gamepad, DualSense). Wall time from call to populated `Vec<DeviceInfo>`. Assert `< 5 ms` on
   reference hardware. Measured with `criterion`.

2. **TC-6.2.B.1** — `ModifierChain::process` over a chain of 10 modifier stages (dead zone, curve,
   swizzle, negate, scalar, smoothing, acceleration,...). Per-call wall time. Assert
   `< 1 microsecond`. No allocation.

3. **TC-6.2.B.2** — Resolve all actions in a context stack of 4 contexts containing 50 total
   bindings, with mixed keyboard, gamepad, and touch sources. Wall time per frame. Assert
   `< 50 microseconds`.

4. **TC-6.3.B.1** — Process one gesture engine update with 10 active fingers and all built-in
   recognizers (tap, swipe, pinch, rotate, pan, long-press) registered. Wall time per frame. Assert
   `< 100 microseconds`.

5. **TC-6.5.B.1** — Solve 26-joint hand skeleton from a synthesized depth image. Wall time per hand.
   Assert `< 1 ms`; per-joint accuracy `< 5 mm` RMS against ground truth.
