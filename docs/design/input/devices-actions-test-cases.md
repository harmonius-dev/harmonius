# Input Devices and Action Mapping Test Cases

Companion test cases for [devices-actions.md](devices-actions.md).

## Unit Tests

### TC-6.1.1.1 Scancode Normalization Windows

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All 104 Win32 scancodes | Each maps to correct USB HID code | R-6.1.1 |

### TC-6.1.1.2 Scancode Normalization Linux

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All 104 Linux evdev codes | Each maps to correct USB HID code | R-6.1.1 |

### TC-6.1.1.3 Scancode Layout Independence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same physical key on QWERTY layout | Scancode S1, keycode K1 | R-6.1.1 |
| 2 | Same physical key on AZERTY layout | Scancode S1 (same), keycode K2 (different) | R-6.1.1 |

### TC-6.1.2.1 Mouse DPI Normalization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Raw delta (100, 50) at 100% DPI scale | Normalized delta (100, 50) | R-6.1.2 |
| 2 | Raw delta (200, 100) at 200% DPI scale | Normalized delta (100, 50) | R-6.1.2 |

### TC-6.1.2.2 Scroll Normalization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trackpad continuous scroll value 0.5 | Axis value 0.5 | R-6.1.2 |
| 2 | Discrete mouse scroll 1 notch | Equivalent axis value 0.5 | R-6.1.2 |

### TC-6.1.3.1 Gamepad Button Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Xbox South button (A) | GamepadButton::South | R-6.1.3 |
| 2 | PlayStation South button (Cross) | GamepadButton::South | R-6.1.3 |
| 3 | Switch South button (B) | GamepadButton::South | R-6.1.3 |

### TC-6.1.3.2 Gamepad Capability Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query DualSense capabilities | gyro=true | R-6.1.3 |
| 2 | Query Xbox controller capabilities | gyro=false | R-6.1.3 |

### TC-6.1.3.3 Madgwick Orientation Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known gyro/accel input sequence | Correct quaternion orientation | R-6.1.3 |

### TC-6.1.4.1 Touch Pressure Normalization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Touch event from any backend | Pressure in [0.0, 1.0] range | R-6.1.4 |

### TC-6.1.4.2 Touch 10 Simultaneous Contacts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 simultaneous touch-down events | All 10 tracked with unique IDs | R-6.1.4 |

### TC-6.2.1.1 Action Type Bool

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Key-down event bound to bool action | Action fires true | R-6.2.1 |
| 2 | Key-up event | Action fires false | R-6.2.1 |
| 3 | Gamepad button bound to same action | Action fires true/false | R-6.2.1 |

### TC-6.2.1.2 Action Type Axis2D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stick at (0.5, -0.3) bound to Axis2D action | Vec2(0.5, -0.3) | R-6.2.1 |
| 2 | WASD composite (W+D held) | Vec2(1.0, 1.0) normalized | R-6.2.1 |

### TC-6.2.1.3 Action Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bind Axis2D source to bool action | TypeMismatch error | R-6.2.1 |

### TC-6.2.2.1 Context Priority Consume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | High-priority context binds Escape; low-priority also binds Escape | High fires; low does not fire | R-6.2.2 |

### TC-6.2.2.2 Context Passthrough

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unbound input F1 in top context; lower context binds F1 | Lower context fires | R-6.2.2 |

### TC-6.2.3.1 Deadzone Radial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stick magnitude 0.10 with 0.15 deadzone threshold | Output (0, 0) | R-6.2.3 |
| 2 | Stick magnitude 0.50 with 0.15 deadzone threshold | Remapped value > 0 | R-6.2.3 |

### TC-6.2.3.2 Deadzone Axial

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stick (0.05, 0.80) with per-axis 0.10 threshold | Output (0.0, remapped_y) | R-6.2.3 |

### TC-6.2.3.3 Response Curve Exponential

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Input v=0.5 with exponential curve (exponent=2.0) | Output 0.25 (0.5^2) | R-6.2.3 |

### TC-6.2.3.4 Modifier Chain Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Input 0.5; chain: deadzone(0.1) -> curve(exp=2) -> scalar(2.0) | Correct composed output | R-6.2.3 |

### TC-6.2.4.1 Trigger Pressed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Key-down frame | Trigger fires | R-6.2.4 |
| 2 | Key held (subsequent frame) | Trigger does not fire | R-6.2.4 |

### TC-6.2.4.2 Trigger Released

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Key-up frame | Trigger fires | R-6.2.4 |
| 2 | Key already up (subsequent frame) | Trigger does not fire | R-6.2.4 |

### TC-6.2.4.3 Trigger Hold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold key for exactly configured duration (500 ms) | Trigger fires at 500 ms | R-6.2.4 |
| 2 | Hold key for 400 ms (below threshold) | Trigger does not fire | R-6.2.4 |

### TC-6.2.4.4 Trigger Tap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press and release within tap threshold (100 ms) | Trigger fires | R-6.2.4 |
| 2 | Press and release after tap threshold | Trigger does not fire | R-6.2.4 |

### TC-6.2.4.5 Trigger Pulse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hold key; pulse interval = 200 ms; held for 600 ms | Fires 3 times at 200, 400, 600 ms | R-6.2.4 |

### TC-6.2.4.6 Trigger Chord

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | All chord inputs active within 50 ms window | Trigger fires | R-6.2.4 |
| 2 | One chord input released before all active | Trigger does not fire | R-6.2.4 |

### TC-6.2.4.7 Trigger Combo

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inputs arrive in order within 500 ms window | Trigger fires | R-6.2.4 |
| 2 | Inputs arrive out of order | Trigger does not fire | R-6.2.4 |

### TC-6.2.5.1 Rebind Success

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebind "Jump" from Space to Enter (unoccupied) | Rebind succeeds | R-6.2.5 |

### TC-6.2.5.2 Rebind Conflict

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebind "Jump" to key already bound to "Shoot" | Returns Conflict error | R-6.2.5 |

### TC-6.2.5.3 Rebind Reserved

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebind action to PlayStation PS button | Returns ReservedInput error | R-6.2.5 |

### TC-6.2.5.4 Rebind Swap Resolution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Swap "Jump"=Space with "Shoot"=Enter | Jump=Enter, Shoot=Space | R-6.2.5 |

### TC-6.2.6.1 Glyph Resolution Xbox

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Action bound to GamepadButton::South on Xbox | Resolves to Xbox A glyph | R-6.2.6 |

### TC-6.2.6.2 Glyph Resolution PlayStation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same action on PlayStation | Resolves to Cross glyph | R-6.2.6 |

### TC-6.2.6.3 Glyph Device Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch active device from Xbox to PlayStation | Cache invalidated; new glyphs resolved | R-6.2.6 |

### TC-6.2.8.1 Combo QCF

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Down -> Down-Forward -> Forward -> Punch within window | Combo fires | R-6.2.8 |

### TC-6.2.8.2 Combo Timeout

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Correct directions but exceeded time window | Combo tree resets to root; no fire | R-6.2.8 |

### TC-6.2.8.3 Combo Cross-Device

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | QCF via stick, D-pad, and WASD | All three produce identical combo result | R-6.2.8 |

### TC-6.2.9.1 Buffer Consume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buffer dodge input; cancel window opens | Dodge executes at cancel window start frame | R-6.2.9 |

### TC-6.2.9.2 Buffer Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buffer both dodge and attack | Dodge executes (higher priority) | R-6.2.9 |

### TC-6.2.9.3 Buffer Expiry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Buffer input; wait past expiry window | Input does not execute | R-6.2.9 |

### TC-6.2.10.1 Smoothing Reduces Jitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Noisy input signal; 50 ms smoothing window | Variance reduced by >= 80% | R-6.2.10 |

### TC-6.2.10.2 Acceleration Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Max velocity input with 2x multiplier | Output == 2x input | R-6.2.10 |

### TC-6.2.10.3 Aim Magnetism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Crosshair near target | Deflects toward target | R-6.2.10 |
| 2 | Crosshair with no nearby target | No deflection | R-6.2.10 |

## Integration Tests

### TC-6.1.5.I1 Hot-Plug Connect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect gamepad | DeviceConnected event within one frame | R-6.1.5 |

### TC-6.1.5.I2 Hot-Plug Disconnect

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disconnect gamepad mid-gameplay | DeviceDisconnected event; no frame hitch | R-6.1.5 |

### TC-6.1.5.I3 Hot-Plug Rapid Cycles

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect/disconnect 10 times rapidly | No crashes; no leaked device slots | R-6.1.5 |

### TC-6.1.1.I1 Cross-Platform Keyboard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Keyboard capture on Windows, macOS, Linux | Identical scancodes across all platforms | R-6.1.1 |

### TC-6.1.3.I1 Cross-Platform Gamepad

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Gamepad capture on Windows, macOS, Linux | Identical button/axis values across all platforms | R-6.1.3 |

### TC-6.1.NF1.I1 Full Pipeline Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inject timestamped OS event; measure delta to ActionState write over 10K events | p99 < 1 ms | R-6.1.NF1 |

### TC-6.1.NF2.I1 Enumeration Speed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect 4 devices; measure enumeration time | Under 5 ms | R-6.1.NF2 |

### TC-6.2.NF1.I1 128 Actions Throughput

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 128 actions across 8 contexts with 4-stage modifiers | Evaluation < 0.2 ms | R-6.2.NF1 |

### TC-6.2.NF2.I1 Rebind Persistence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rebind 20 actions; save | Save < 100 ms | R-6.2.NF2 |
| 2 | Restart; restore bindings | Restore < 50 ms; all bindings correct | R-6.2.NF2 |

### TC-6.2.7.I1 Recording Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Record 30 s input; playback; compare game state hash | Hash matches | R-6.2.7 |

### TC-6.2.7.I2 Recording Speed Control

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Playback at 0.5x speed | Same final game state hash | R-6.2.7 |
| 2 | Playback at 2.0x speed | Same final game state hash | R-6.2.7 |

### TC-6.2.11.I1 Full UI Navigability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Navigate every UI screen using only gamepad | Every widget reachable | R-6.2.11 |

## Benchmarks

### TC-6.1.NF1.B1 Device Poll

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Poll all connected devices | Wall time | < 0.1 ms | R-6.1.NF1 |

### TC-6.1.1.B1 Scancode Normalization

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Normalize single scancode | Wall time | < 10 ns | R-6.1.1 |

### TC-6.2.NF1.B1 Action Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate 128 actions across 8 contexts | Wall time | < 0.2 ms | R-6.2.NF1 |

### TC-6.2.3.B1 Modifier Chain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4-stage modifier chain per binding | Wall time | < 50 ns per binding | R-6.2.3 |

### TC-6.2.4.B1 Trigger Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate single trigger condition | Wall time | < 20 ns per binding | R-6.2.4 |

### TC-6.2.2.B1 Context Stack Walk

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Walk 8-context priority stack | Wall time | < 0.05 ms | R-6.2.2 |

### TC-6.2.8.B1 Combo Tree Advance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Advance combo tree by one step | Wall time | < 100 ns | R-6.2.8 |

### TC-6.2.6.B1 Glyph Resolution Cached

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Resolve cached glyph for action | Wall time | < 10 ns | R-6.2.6 |

### TC-6.2.NF2.B1 Rebind Save

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Save 20 action rebindings | Wall time | < 100 ms | R-6.2.NF2 |

### TC-6.2.NF2.B2 Rebind Restore

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Restore 20 action rebindings | Wall time | < 50 ms | R-6.2.NF2 |
