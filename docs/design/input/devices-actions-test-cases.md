# Input Devices and Action Mapping Test Cases

Companion test cases for [devices-actions.md](devices-actions.md).

## Unit Tests

### TC-6.1.1.1 Scancode Normalization Windows

| # | Requirement |
|---|-------------|
| 1 | R-6.1.1     |

1. **#1** — All 104 Win32 scancodes
   - **Expected:** Each maps to correct USB HID code

### TC-6.1.1.2 Scancode Normalization Linux

| # | Requirement |
|---|-------------|
| 1 | R-6.1.1     |

1. **#1** — All 104 Linux evdev codes
   - **Expected:** Each maps to correct USB HID code

### TC-6.1.1.3 Scancode Layout Independence

| # | Requirement |
|---|-------------|
| 1 | R-6.1.1     |
| 2 | R-6.1.1     |

1. **#1** — Same physical key on QWERTY layout
   - **Expected:** Scancode S1, keycode K1
2. **#2** — Same physical key on AZERTY layout
   - **Expected:** Scancode S1 (same), keycode K2 (different)

### TC-6.1.2.1 Mouse DPI Normalization

| # | Requirement |
|---|-------------|
| 1 | R-6.1.2     |
| 2 | R-6.1.2     |

1. **#1** — Raw delta (100, 50) at 100% DPI scale
   - **Expected:** Normalized delta (100, 50)
2. **#2** — Raw delta (200, 100) at 200% DPI scale
   - **Expected:** Normalized delta (100, 50)

### TC-6.1.2.2 Scroll Normalization

| # | Requirement |
|---|-------------|
| 1 | R-6.1.2     |
| 2 | R-6.1.2     |

1. **#1** — Trackpad continuous scroll value 0.5
   - **Expected:** Axis value 0.5
2. **#2** — Discrete mouse scroll 1 notch
   - **Expected:** Equivalent axis value 0.5

### TC-6.1.3.1 Gamepad Button Mapping

| # | Requirement |
|---|-------------|
| 1 | R-6.1.3     |
| 2 | R-6.1.3     |
| 3 | R-6.1.3     |

1. **#1** — Xbox South button (A)
   - **Expected:** GamepadButton::South
2. **#2** — PlayStation South button (Cross)
   - **Expected:** GamepadButton::South
3. **#3** — Switch South button (B)
   - **Expected:** GamepadButton::South

### TC-6.1.3.2 Gamepad Capability Query

| # | Requirement |
|---|-------------|
| 1 | R-6.1.3     |
| 2 | R-6.1.3     |

1. **#1** — Query DualSense capabilities
   - **Expected:** gyro=true
2. **#2** — Query Xbox controller capabilities
   - **Expected:** gyro=false

### TC-6.1.3.3 Madgwick Orientation Filter

| # | Requirement |
|---|-------------|
| 1 | R-6.1.3     |

1. **#1** — Known gyro/accel input sequence
   - **Expected:** Correct quaternion orientation

### TC-6.1.4.1 Touch Pressure Normalization

| # | Requirement |
|---|-------------|
| 1 | R-6.1.4     |

1. **#1** — Touch event from any backend
   - **Expected:** Pressure in [0.0, 1.0] range

### TC-6.1.4.2 Touch 10 Simultaneous Contacts

| # | Requirement |
|---|-------------|
| 1 | R-6.1.4     |

1. **#1** — 10 simultaneous touch-down events
   - **Expected:** All 10 tracked with unique IDs

### TC-6.2.1.1 Action Type Bool

| # | Requirement |
|---|-------------|
| 1 | R-6.2.1     |
| 2 | R-6.2.1     |
| 3 | R-6.2.1     |

1. **#1** — Key-down event bound to bool action
   - **Expected:** Action fires true
2. **#2** — Key-up event
   - **Expected:** Action fires false
3. **#3** — Gamepad button bound to same action
   - **Expected:** Action fires true/false

### TC-6.2.1.2 Action Type Axis2D

| # | Requirement |
|---|-------------|
| 1 | R-6.2.1     |
| 2 | R-6.2.1     |

1. **#1** — Stick at (0.5, -0.3) bound to Axis2D action
   - **Expected:** Vec2(0.5, -0.3)
2. **#2** — WASD composite (W+D held)
   - **Expected:** Vec2(1.0, 1.0) normalized

### TC-6.2.1.3 Action Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-6.2.1     |

1. **#1** — Bind Axis2D source to bool action
   - **Expected:** TypeMismatch error

### TC-6.2.2.1 Context Priority Consume

| # | Requirement |
|---|-------------|
| 1 | R-6.2.2     |

1. **#1** — High-priority context binds Escape; low-priority also binds Escape
   - **Expected:** High fires; low does not fire

### TC-6.2.2.2 Context Passthrough

| # | Requirement |
|---|-------------|
| 1 | R-6.2.2     |

1. **#1** — Unbound input F1 in top context; lower context binds F1
   - **Expected:** Lower context fires

### TC-6.2.3.1 Deadzone Radial

| # | Requirement |
|---|-------------|
| 1 | R-6.2.3     |
| 2 | R-6.2.3     |

1. **#1** — Stick magnitude 0.10 with 0.15 deadzone threshold
   - **Expected:** Output (0, 0)
2. **#2** — Stick magnitude 0.50 with 0.15 deadzone threshold
   - **Expected:** Remapped value > 0

### TC-6.2.3.2 Deadzone Axial

| # | Requirement |
|---|-------------|
| 1 | R-6.2.3     |

1. **#1** — Stick (0.05, 0.80) with per-axis 0.10 threshold
   - **Expected:** Output (0.0, remapped_y)

### TC-6.2.3.3 Response Curve Exponential

| # | Requirement |
|---|-------------|
| 1 | R-6.2.3     |

1. **#1** — Input v=0.5 with exponential curve (exponent=2.0)
   - **Expected:** Output 0.25 (0.5^2)

### TC-6.2.3.4 Modifier Chain Order

| # | Requirement |
|---|-------------|
| 1 | R-6.2.3     |

1. **#1** — Input 0.5; chain: deadzone(0.1) -> curve(exp=2) -> scalar(2.0)
   - **Expected:** Correct composed output

### TC-6.2.4.1 Trigger Pressed

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — Key-down frame
   - **Expected:** Trigger fires
2. **#2** — Key held (subsequent frame)
   - **Expected:** Trigger does not fire

### TC-6.2.4.2 Trigger Released

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — Key-up frame
   - **Expected:** Trigger fires
2. **#2** — Key already up (subsequent frame)
   - **Expected:** Trigger does not fire

### TC-6.2.4.3 Trigger Hold

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — Hold key for exactly configured duration (500 ms)
   - **Expected:** Trigger fires at 500 ms
2. **#2** — Hold key for 400 ms (below threshold)
   - **Expected:** Trigger does not fire

### TC-6.2.4.4 Trigger Tap

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — Press and release within tap threshold (100 ms)
   - **Expected:** Trigger fires
2. **#2** — Press and release after tap threshold
   - **Expected:** Trigger does not fire

### TC-6.2.4.5 Trigger Pulse

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |

1. **#1** — Hold key; pulse interval = 200 ms; held for 600 ms
   - **Expected:** Fires 3 times at 200, 400, 600 ms

### TC-6.2.4.6 Trigger Chord

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — All chord inputs active within 50 ms window
   - **Expected:** Trigger fires
2. **#2** — One chord input released before all active
   - **Expected:** Trigger does not fire

### TC-6.2.4.7 Trigger Combo

| # | Requirement |
|---|-------------|
| 1 | R-6.2.4     |
| 2 | R-6.2.4     |

1. **#1** — Inputs arrive in order within 500 ms window
   - **Expected:** Trigger fires
2. **#2** — Inputs arrive out of order
   - **Expected:** Trigger does not fire

### TC-6.2.5.1 Rebind Success

| # | Requirement |
|---|-------------|
| 1 | R-6.2.5     |

1. **#1** — Rebind "Jump" from Space to Enter (unoccupied)
   - **Expected:** Rebind succeeds

### TC-6.2.5.2 Rebind Conflict

| # | Requirement |
|---|-------------|
| 1 | R-6.2.5     |

1. **#1** — Rebind "Jump" to key already bound to "Shoot"
   - **Expected:** Returns Conflict error

### TC-6.2.5.3 Rebind Reserved

| # | Requirement |
|---|-------------|
| 1 | R-6.2.5     |

1. **#1** — Rebind action to PlayStation PS button
   - **Expected:** Returns ReservedInput error

### TC-6.2.5.4 Rebind Swap Resolution

| # | Requirement |
|---|-------------|
| 1 | R-6.2.5     |

1. **#1** — Swap "Jump"=Space with "Shoot"=Enter
   - **Expected:** Jump=Enter, Shoot=Space

### TC-6.2.6.1 Glyph Resolution Xbox

| # | Requirement |
|---|-------------|
| 1 | R-6.2.6     |

1. **#1** — Action bound to GamepadButton::South on Xbox
   - **Expected:** Resolves to Xbox A glyph

### TC-6.2.6.2 Glyph Resolution PlayStation

| # | Requirement |
|---|-------------|
| 1 | R-6.2.6     |

1. **#1** — Same action on PlayStation
   - **Expected:** Resolves to Cross glyph

### TC-6.2.6.3 Glyph Device Switch

| # | Requirement |
|---|-------------|
| 1 | R-6.2.6     |

1. **#1** — Switch active device from Xbox to PlayStation
   - **Expected:** Cache invalidated; new glyphs resolved

### TC-6.2.8.1 Combo QCF

| # | Requirement |
|---|-------------|
| 1 | R-6.2.8     |

1. **#1** — Down -> Down-Forward -> Forward -> Punch within window
   - **Expected:** Combo fires

### TC-6.2.8.2 Combo Timeout

| # | Requirement |
|---|-------------|
| 1 | R-6.2.8     |

1. **#1** — Correct directions but exceeded time window
   - **Expected:** Combo tree resets to root; no fire

### TC-6.2.8.3 Combo Cross-Device

| # | Requirement |
|---|-------------|
| 1 | R-6.2.8     |

1. **#1** — QCF via stick, D-pad, and WASD
   - **Expected:** All three produce identical combo result

### TC-6.2.9.1 Buffer Consume

| # | Requirement |
|---|-------------|
| 1 | R-6.2.9     |

1. **#1** — Buffer dodge input; cancel window opens
   - **Expected:** Dodge executes at cancel window start frame

### TC-6.2.9.2 Buffer Priority

| # | Requirement |
|---|-------------|
| 1 | R-6.2.9     |

1. **#1** — Buffer both dodge and attack
   - **Expected:** Dodge executes (higher priority)

### TC-6.2.9.3 Buffer Expiry

| # | Requirement |
|---|-------------|
| 1 | R-6.2.9     |

1. **#1** — Buffer input; wait past expiry window
   - **Expected:** Input does not execute

### TC-6.2.10.1 Smoothing Reduces Jitter

| # | Requirement |
|---|-------------|
| 1 | R-6.2.10    |

1. **#1** — Noisy input signal; 50 ms smoothing window
   - **Expected:** Variance reduced by >= 80%

### TC-6.2.10.2 Acceleration Scaling

| # | Requirement |
|---|-------------|
| 1 | R-6.2.10    |

1. **#1** — Max velocity input with 2x multiplier
   - **Expected:** Output == 2x input

### TC-6.2.10.3 Aim Magnetism

| # | Requirement |
|---|-------------|
| 1 | R-6.2.10    |
| 2 | R-6.2.10    |

1. **#1** — Crosshair near target
   - **Expected:** Deflects toward target
2. **#2** — Crosshair with no nearby target
   - **Expected:** No deflection

## Integration Tests

### TC-6.1.5.I1 Hot-Plug Connect

| # | Requirement |
|---|-------------|
| 1 | R-6.1.5     |

1. **#1** — Connect gamepad
   - **Expected:** DeviceConnected event within one frame

### TC-6.1.5.I2 Hot-Plug Disconnect

| # | Requirement |
|---|-------------|
| 1 | R-6.1.5     |

1. **#1** — Disconnect gamepad mid-gameplay
   - **Expected:** DeviceDisconnected event; no frame hitch

### TC-6.1.5.I3 Hot-Plug Rapid Cycles

| # | Requirement |
|---|-------------|
| 1 | R-6.1.5     |

1. **#1** — Connect/disconnect 10 times rapidly
   - **Expected:** No crashes; no leaked device slots

### TC-6.1.1.I1 Cross-Platform Keyboard

| # | Requirement |
|---|-------------|
| 1 | R-6.1.1     |

1. **#1** — Keyboard capture on Windows, macOS, Linux
   - **Expected:** Identical scancodes across all platforms

### TC-6.1.3.I1 Cross-Platform Gamepad

| # | Requirement |
|---|-------------|
| 1 | R-6.1.3     |

1. **#1** — Gamepad capture on Windows, macOS, Linux
   - **Expected:** Identical button/axis values across all platforms

### TC-6.1.NF1.I1 Full Pipeline Latency

| # | Requirement |
|---|-------------|
| 1 | R-6.1.NF1   |

1. **#1** — Inject timestamped OS event; measure delta to ActionState write over 10K events
   - **Expected:** p99 < 1 ms

### TC-6.1.NF2.I1 Enumeration Speed

| # | Requirement |
|---|-------------|
| 1 | R-6.1.NF2   |

1. **#1** — Connect 4 devices; measure enumeration time
   - **Expected:** Under 5 ms

### TC-6.2.NF1.I1 128 Actions Throughput

| # | Requirement |
|---|-------------|
| 1 | R-6.2.NF1   |

1. **#1** — 128 actions across 8 contexts with 4-stage modifiers
   - **Expected:** Evaluation < 0.2 ms

### TC-6.2.NF2.I1 Rebind Persistence

| # | Requirement |
|---|-------------|
| 1 | R-6.2.NF2   |
| 2 | R-6.2.NF2   |

1. **#1** — Rebind 20 actions; save
   - **Expected:** Save < 100 ms
2. **#2** — Restart; restore bindings
   - **Expected:** Restore < 50 ms; all bindings correct

### TC-6.2.7.I1 Recording Determinism

| # | Requirement |
|---|-------------|
| 1 | R-6.2.7     |

1. **#1** — Record 30 s input; playback; compare game state hash
   - **Expected:** Hash matches

### TC-6.2.7.I2 Recording Speed Control

| # | Requirement |
|---|-------------|
| 1 | R-6.2.7     |
| 2 | R-6.2.7     |

1. **#1** — Playback at 0.5x speed
   - **Expected:** Same final game state hash
2. **#2** — Playback at 2.0x speed
   - **Expected:** Same final game state hash

### TC-6.2.11.I1 Full UI Navigability

| # | Requirement |
|---|-------------|
| 1 | R-6.2.11    |

1. **#1** — Navigate every UI screen using only gamepad
   - **Expected:** Every widget reachable

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
