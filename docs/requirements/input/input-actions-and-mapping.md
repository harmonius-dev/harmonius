# R-6.2 — Input Actions & Mapping Requirements

## Action Definitions

### R-6.2.1 Typed Input Actions

The engine **SHALL** define input actions as strongly typed values in four variants -- boolean
(pressed/released), axis 1D (scalar float), axis 2D (vector2), and axis 3D (vector3) -- decoupling
gameplay logic from physical device bindings so that the same action fires identically from any
bound device.

- **Derived from:** [F-6.2.1](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Typed actions decouple gameplay systems from hardware, enabling controller, keyboard,
  and touch to trigger the same logic without branching on device type.
- **Verification:** Unit test: bind a boolean action to a keyboard key, a gamepad button, and a touch
  tap. Fire each binding and assert the action produces the same boolean value. Repeat for axis 1D
  (trigger, mouse delta, touch drag), axis 2D (stick, WASD, virtual joystick), and axis 3D (VR
  controller position). Verify type mismatches produce a compile-time or load-time error.

## Input Mapping Contexts

### R-6.2.2 Input Mapping Contexts with Priority Stacking

The engine **SHALL** support named mapping contexts (e.g., "OnFoot", "UIMenu") maintained on a
priority-ordered stack, where higher-priority contexts consume matching inputs first and
lower-priority contexts receive only unconsumed inputs.

- **Derived from:** [F-6.2.2](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Modal control schemes (inventory captures Escape before combat) require priority-based
  input consumption to prevent actions from leaking across incompatible gameplay states.
- **Verification:** Unit test: push "Combat" and "UIMenu" contexts where both bind Escape. Activate
  "UIMenu" at higher priority and press Escape; assert only the UIMenu action fires. Pop "UIMenu"
  and press Escape again; assert the Combat action fires. Verify that inputs not bound in the
  top context pass through to lower contexts.

## Modifiers

### R-6.2.3 Input Value Modifiers

The engine **SHALL** apply configurable modifier chains to raw input values before delivery to
actions, supporting at minimum: dead zone (axial and radial), response curve (linear, exponential,
S-curve), swizzle, negate, and scalar modifiers, composable in author-defined order.

- **Derived from:** [F-6.2.3](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Stick feel varies across controller models and player preference; modifier chains
  enable per-player tuning without changing gameplay code.
- **Verification:** Unit test: apply a radial dead zone of 0.15 to an axis 2D input of magnitude
  0.10 and assert the output is (0.0, 0.0). Apply magnitude 0.50 and assert the output is
  remapped to exclude the dead zone range. Chain dead zone, exponential curve, and scalar 2.0;
  verify the output matches the expected mathematical composition.

## Triggers

### R-6.2.4 Input Trigger Conditions

The engine **SHALL** support configurable trigger conditions on actions: pressed (single frame on
key-down), released (single frame on key-up), hold (fires after sustained duration), tap (press
and release within threshold), pulse (repeating at interval while held), chord (multiple
simultaneous inputs), and combo (ordered sequence within time window).

- **Derived from:** [F-6.2.4](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Different ability types need distinct activation patterns -- instant casts use tap,
  channeled abilities use hold, extended action bars use chord, and ability chains use combo.
- **Verification:** Unit test per trigger type: (1) pressed fires on the key-down frame only;
  (2) released fires on the key-up frame only; (3) hold fires after exactly the configured
  duration; (4) tap fires only if release occurs within threshold; (5) pulse fires at configured
  interval; (6) chord fires only when all required inputs are active simultaneously; (7) combo
  fires only when inputs arrive in order within the time window.

## Rebinding

### R-6.2.5 Runtime Key Rebinding with Conflict Detection

The engine **SHALL** allow players to rebind any action to any compatible input at runtime through
the visual settings UI, detect binding conflicts within overlapping mapping contexts, present
resolution options (swap, unbind previous, cancel), and persist rebindings across sessions.

- **Derived from:** [F-6.2.5](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Rebinding is a core accessibility requirement; players with many action bar slots
  must freely customize layouts, and conflicts must be surfaced to avoid broken bindings.
- **Verification:** Integration test: rebind "Jump" from Space to F in the settings UI. Assert F
  now fires Jump. Rebind another action to F; assert a conflict dialog appears with swap/unbind
  options. Choose swap; assert bindings are exchanged. Restart the session and verify rebindings
  persist from storage. Verify platform-reserved keys (PS button, Guide button) are rejected.

## Button Glyphs

### R-6.2.6 Platform-Aware Button Glyph Resolution

The engine **SHALL** resolve each input action's current binding to the correct platform-specific
button icon and update all displayed glyphs within one frame of an active input device change.

- **Derived from:** [F-6.2.6](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Console certification mandates correct platform-native button icons; players
  switching between keyboard and gamepad expect immediate visual feedback.
- **Verification:** Integration test: display a prompt bound to the "Confirm" action. Switch from
  keyboard to an Xbox gamepad; assert the glyph updates to the A button icon within one frame.
  Switch to a DualSense; assert the glyph updates to the Cross icon. Switch to keyboard; assert
  the glyph updates to the bound key label. Verify custom glyph atlas assets load correctly.

## Input Recording

### R-6.2.7 Input Recording and Playback

The engine **SHALL** record all input events (device states, action firings, frame timestamps) to a
binary stream and support deterministic playback with speed control (slow-mo, fast-forward) and
frame-stepping, producing identical game state when replayed under the same initial conditions.
Input recording captures raw input events for testing, tutorials, and bug reproduction. For full
game state recording and spectator replay, see the networking replay system (R-8.6.1). Input
recordings can be used as input sources for the replay system's deterministic playback mode.

- **Derived from:** [F-6.2.7](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Deterministic input replay enables automated test regression, tutorial ghost
  playback, and reproducible bug reports.
- **Verification:** Integration test: record a 30-second input sequence, play it back, and compare
  final game state (entity positions, health values) against the original recording; assert
  exact match. Verify playback at 0.5x and 2.0x speed produces the same final state. Verify
  frame-stepping advances exactly one frame per step.

## Advanced Combo Input

### R-6.2.8 Combo Input Trees and Directional Sequences

The engine **SHALL** recognize directional motion sequences (quarter-circle, dragon punch, charge,
360), branching combo trees authored as visual graph assets, and multi-button simultaneous presses
with configurable leniency windows, normalizing directional inputs across device types (stick,
D-pad, WASD).

- **Derived from:** [F-6.2.8](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Action combat requires fighting-game-grade input recognition where combo definitions
  are data-driven visual assets editable without code.
- **Verification:** Unit test: define a quarter-circle-forward + button combo with a 200ms leniency
  window. Input the sequence within window; assert combo fires. Input with incorrect direction;
  assert combo does not fire. Input with correct direction but exceeding the timing window; assert
  combo does not fire and state resets to root. Verify identical behavior with stick, D-pad, and
  WASD inputs.

### R-6.2.9 Input Buffering and Ability Cancel Windows

The engine **SHALL** buffer the most recent input during an ability's active frames and execute it
when the ability enters a cancellable window or completes, with a configurable buffer duration and
priority rules resolving conflicts among multiple buffered inputs.

- **Derived from:** [F-6.2.9](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Input buffering provides responsive action-game feel, letting players queue their
  next action during recovery frames without frame-perfect timing.
- **Verification:** Unit test: trigger ability A (500ms duration, cancel window at 300-500ms) and
  press dodge at 250ms. Assert dodge executes at exactly 300ms (cancel window start). Press both
  dodge and attack at 250ms; assert dodge executes (higher priority). Assert no buffered input
  executes if it arrives after the buffer duration expires.

## Advanced Filtering

### R-6.2.10 Input Smoothing, Acceleration, and Aim Assist

The engine **SHALL** provide composable input processing stages for smoothing (low-pass filter with
configurable time constant), acceleration (velocity-proportional sensitivity with ramp-up time, max
multiplier, and decay rate), and aim assist (target magnetism, friction, and snap) that are
individually toggleable per input action and configurable per weapon type and game mode.

- **Derived from:** [F-6.2.10](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Advanced input processing eliminates jitter from worn controllers, provides natural
  camera feel, and bridges the precision gap between mouse and gamepad aiming.
- **Verification:** Unit test: apply smoothing with a 50ms time constant to a jittery axis input;
  assert output variance is reduced by at least 80% without exceeding 16ms added latency.
  Apply acceleration with 2x max multiplier; assert output at max input velocity equals 2x the
  base sensitivity. Enable aim assist magnetism; assert crosshair deflects toward a valid target
  within the configured radius and does not deflect when no target is present.

## Gamepad UI Navigation

### R-6.2.11 Controller-Driven UI Interaction

The engine **SHALL** support full UI navigation via gamepad and keyboard without requiring mouse or
touch input, providing directional focus navigation, virtual cursor mode, and context-sensitive
action mapping, with all hover-dependent UI behaviors activating on focus when a gamepad is active.

- **Derived from:** [F-6.2.11](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Console certification requires full controller navigability of all UI screens;
  virtual cursor mode handles complex UIs like world maps that cannot be easily focus-navigated.
- **Verification:** Integration test: navigate through a multi-screen UI (main menu, inventory,
  settings, skill tree) using only a gamepad. Assert every interactive widget is reachable via
  D-pad/stick. Verify virtual cursor mode moves a software cursor proportionally to stick input.
  Switch from gamepad to mouse mid-interaction; assert the transition is seamless with no lost
  focus state. Verify tooltips appear on focus rather than hover when gamepad is active.

---

## Non-Functional Requirements

### R-6.2.NF1 Action Evaluation Throughput

The engine **SHALL** evaluate all active input actions (modifier chains, trigger conditions,
context priority resolution) within 0.2 ms per frame for up to 128 simultaneously active
actions across all mapping contexts.

- **Derived from:** [F-6.2.1](../../features/input/input-actions-and-mapping.md),
  [F-6.2.3](../../features/input/input-actions-and-mapping.md),
  [R-X.1.1](../cross-cutting.md) (Frame Time Budget)
- **Rationale:** The action system runs on the main thread every frame. Its cost must be
  negligible relative to the frame budget to avoid impacting gameplay logic throughput.
- **Verification:** Benchmark: configure 128 actions across 8 stacked contexts with 4-stage
  modifier chains. Process a frame with all inputs active. Assert total evaluation time is
  below 0.2 ms on minimum-spec hardware.

### R-6.2.NF2 Rebinding Persistence Latency

The engine **SHALL** persist rebinding changes to storage within 100 ms of the player
confirming the rebind, and restore all rebindings from storage within 50 ms during session
startup.

- **Derived from:** [F-6.2.5](../../features/input/input-actions-and-mapping.md)
- **Rationale:** Players expect rebinding changes to survive unexpected crashes. Fast restore
  prevents perceptible delay during session startup.
- **Verification:** Integration test: rebind 20 actions and measure time from confirmation
  to storage write completion. Assert all writes complete within 100 ms. Restart the session
  and measure time from startup to rebinding restoration. Assert restoration completes within
  50 ms.
