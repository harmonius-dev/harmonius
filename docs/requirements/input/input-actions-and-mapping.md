# R-6.2 — Input Actions & Mapping Requirements

## Action Definitions

| ID       | Derived From |
|----------|--------------|
| R-6.2.1  |              |
| R-6.2.1a |              |

1. **R-6.2.1** — The engine **SHALL** define input actions as strongly typed values supporting
   boolean (pressed/not pressed), axis 1D (scalar float), axis 2D (vector2), and axis 3D (vector3),
   with type enforcement at binding load time that rejects mismatched source-to-action bindings with
   a diagnostic error. [F-6.2.1](../../features/input/input-actions-and-mapping.md) boolean action)
   that causes hard-to-debug gameplay bugs. three produce the same boolean value. Bind an axis 2D
   source to a boolean action and assert a load-time diagnostic error.
   - **Rationale:** Type safety prevents silent value truncation (e.g., binding an axis 2D stick to
     a
   - **Verification:** Unit test: bind a boolean action to keyboard, gamepad, and touch. Assert all
2. **R-6.2.1a** — The engine **SHALL** produce identical `ActionState` values for the same semantic
   input regardless of source device (keyboard, gamepad, touch, VR controller), so gameplay systems
   never branch on device type. [F-6.2.1](../../features/input/input-actions-and-mapping.md)
   expressible in visual graphs. and touch virtual joystick. Push all to max forward. Assert all
   three produce equivalent axis 2D values within 1% tolerance.
   - **Rationale:** The no-code engine requires device-agnostic gameplay logic; device branching is
     not
   - **Verification:** Unit test: bind a "Move" axis 2D action to keyboard WASD, gamepad left stick,

## Input Mapping Contexts

| ID       | Derived From |
|----------|--------------|
| R-6.2.2  |              |
| R-6.2.2a |              |
| R-6.2.2b |              |

1. **R-6.2.2** — The engine **SHALL** maintain mapping contexts on a priority-ordered stack where
   higher-priority contexts are evaluated first. When a context matches an input and its
   `consumes_input` flag is true, the input **SHALL NOT** pass to lower-priority contexts.
   [F-6.2.2](../../features/input/input-actions-and-mapping.md) contexts. binding Escape. Press
   Escape. Assert only UIMenu fires. Pop UIMenu and press Escape again. Assert Combat fires.
   - **Rationale:** Modal overlays (inventory, menus) must capture inputs before combat or movement
   - **Verification:** Unit test: push Combat (priority 10) and UIMenu (priority 20) contexts, both
2. **R-6.2.2a** — The engine **SHALL** pass inputs not bound in the top context through to
   lower-priority contexts, allowing movement inputs to function while a UI overlay captures
   menu-specific keys. [F-6.2.2](../../features/input/input-actions-and-mapping.md) bind movement
   keys. (priority 10) that binds WASD. Press W. Assert OnFoot movement fires. Press Escape. Assert
   UIMenu fires and OnFoot does not.
   - **Rationale:** Players expect to move while an inventory overlay is open if the overlay does
     not
   - **Verification:** Unit test: push UIMenu (priority 20) that binds only Escape, and OnFoot
3. **R-6.2.2b** — The engine **SHALL** support authoring mapping contexts and their bindings
   entirely in the visual editor, with no code required to create, modify, or assign contexts.
   [F-6.2.2](../../features/input/input-actions-and-mapping.md) reload, and verify all bindings are
   intact and functional at runtime.
   - **Rationale:** No-code engine constraint requires all input configuration via visual tools.
   - **Verification:** Integration test: create a mapping context in the editor, add 5 bindings,
     save,

## Modifiers

| ID       | Derived From |
|----------|--------------|
| R-6.2.3  |              |
| R-6.2.3a |              |
| R-6.2.3b |              |

1. **R-6.2.3** — The engine **SHALL** apply composable modifier chains to raw input values in a
   defined order: dead zone, response curve, swizzle, negate, scalar, smoothing, and acceleration.
   Each modifier **SHALL** be configurable per binding in the editor.
   [F-6.2.3](../../features/input/input-actions-and-mapping.md) composable chains avoid
   combinatorial explosion of presets. Feed input magnitude 0.10 and assert output is zero. Feed
   0.50 and verify output matches mathematical composition within 0.1% tolerance.
   - **Rationale:** Different controllers and gameplay contexts require different input processing;
   - **Verification:** Unit test: chain radial dead zone (0.15), exponential curve, and scalar
     (2.0).
2. **R-6.2.3a** — The engine **SHALL** support both axial (per-axis) and radial (magnitude-based)
   dead zone modes with configurable thresholds. Values below the threshold **SHALL** map to zero;
   values above **SHALL** be remapped to [0.0, 1.0].
   [F-6.2.3](../../features/input/input-actions-and-mapping.md) based on stick magnitude. (0, 0).
   Apply to magnitude 0.50. Assert output is remapped to (0.50 - 0.15) / (1.0 - 0.15) = 0.412.
   - **Rationale:** Axial dead zones prevent drift on individual axes; radial dead zones prevent
     drift
   - **Verification:** Unit test: apply radial dead zone 0.15 to input magnitude 0.10. Assert output
     is
3. **R-6.2.3b** — The engine **SHALL** support linear, exponential, and S-curve response curves as
   modifier stages, each parameterized by an exponent or curve factor.
   [F-6.2.3](../../features/input/input-actions-and-mapping.md) both extremes. 0.25. Apply S-curve
   and verify inflection point behavior.
   - **Rationale:** Exponential curves give fine control at low input; S-curves give fine control at
   - **Verification:** Unit test: apply exponential curve (exponent 2.0) to input 0.5. Assert output
     is

## Triggers

| ID       | Derived From |
|----------|--------------|
| R-6.2.4  |              |
| R-6.2.4a |              |

1. **R-6.2.4** — The engine **SHALL** support trigger conditions: pressed (fires on key-down frame
   only), released (fires on key-up frame only), hold (fires after sustained duration), tap (fires
   on release within threshold), pulse (fires on interval while held), chord (fires when all
   required inputs are simultaneously active), and combo (fires on ordered sequence within time
   window). [F-6.2.4](../../features/input/input-actions-and-mapping.md) casts, hold for channels,
   chord for modifier combos). hold -- assert fires after configured duration. (3) tap -- assert
   fires only if release occurs within threshold. (4) chord -- assert fires only when all inputs are
   active. (5) combo -- assert fires only on correct ordered sequence within time window.
   - **Rationale:** Diverse activation patterns are essential for combat systems (tap for instant
   - **Verification:** Unit test per trigger type: (1) pressed -- assert fires on down frame only.
     (2)
2. **R-6.2.4a** — The engine **SHALL** allow designers to set trigger conditions per action binding
   in the visual editor, with preview of trigger timing behavior.
   [F-6.2.4](../../features/input/input-actions-and-mapping.md) the bound key for 600 ms. Assert
   action fires.
   - **Rationale:** No-code engine constraint; designers must tune trigger timing without code.
   - **Verification:** Integration test: set hold trigger with 500 ms duration in editor. Play and
     hold

## Rebinding

| ID       | Derived From |
|----------|--------------|
| R-6.2.5  |              |
| R-6.2.5a |              |
| R-6.2.5b |              |

1. **R-6.2.5** — The engine **SHALL** allow runtime rebinding of any action to any compatible input
   source, with conflict detection that identifies bindings mapped to the same input within the same
   or overlapping mapping contexts, presenting swap, unbind, or cancel resolution options.
   [F-6.2.5](../../features/input/input-actions-and-mapping.md) configurations. conflict is detected
   and resolution options are presented. Select swap and assert bindings are swapped.
   - **Rationale:** Rebinding is a core accessibility requirement; conflict detection prevents
     unusable
   - **Verification:** Unit test: rebind two actions to the same key within one context. Assert
2. **R-6.2.5a** — The engine **SHALL** persist rebinding changes to storage within 100 ms and
   restore all rebindings within 50 ms during startup, surviving crashes and restarts.
   [F-6.2.5](../../features/input/input-actions-and-mapping.md) Restart session. Assert all 20 are
   restored within 50 ms.
   - **Rationale:** Players expect rebindings to survive crashes; fast restore prevents startup
     delay.
   - **Verification:** Integration test: rebind 20 actions. Assert writes complete within 100 ms.
3. **R-6.2.5b** — The engine **SHALL** reject rebinding attempts to platform-reserved keys (PS
   button, Xbox Guide button) with a diagnostic message.
   [F-6.2.5](../../features/input/input-actions-and-mapping.md) rejected with a message naming the
   reserved key.
   - **Rationale:** Platform certification requires reserved keys to remain functional.
   - **Verification:** Unit test: attempt to rebind an action to the PS button. Assert rebinding is

## Button Glyphs

| ID       | Derived From |
|----------|--------------|
| R-6.2.6  |              |
| R-6.2.6a |              |

1. **R-6.2.6** — The engine **SHALL** resolve input action bindings to platform-specific button
   icons (Xbox, PlayStation, Switch, keyboard) and update all displayed glyphs within one frame of
   an active device change. [F-6.2.6](../../features/input/input-actions-and-mapping.md) glyph for
   the "Jump" action updates from "Space" to "A" within one frame. Switch to DualSense. Assert glyph
   updates to "Cross".
   - **Rationale:** Console certification requires correct platform-native button icons at all
     times.
   - **Verification:** Unit test: switch active device from keyboard to Xbox gamepad. Assert
     displayed
2. **R-6.2.6a** — The engine **SHALL** support swappable glyph atlas assets, enabling custom
   controller icon packs without engine modifications.
   [F-6.2.6](../../features/input/input-actions-and-mapping.md) customization. the custom atlas
   instead of the default.
   - **Rationale:** Games need branded or stylized button icons; hard-coded atlases prevent
   - **Verification:** Integration test: load a custom glyph atlas. Assert glyphs resolve to icons
     from

## Recording and Playback

| ID      | Derived From |
|---------|--------------|
| R-6.2.7 |              |

1. **R-6.2.7** — The engine **SHALL** record all input events (device state, action firings,
   timestamps) to a binary stream and play them back deterministically, producing identical game
   state on replay. Playback **SHALL** support speed control (0.5x to 4.0x) and frame stepping.
   [F-6.2.7](../../features/input/input-actions-and-mapping.md) ghost playback. hash. Assert
   identical. Play back at 0.5x and 2.0x. Assert same final state. Frame-step and assert each step
   advances exactly one frame.
   - **Rationale:** Deterministic replay enables automated testing, bug reproduction, and tutorial
   - **Verification:** Integration test: record 30 seconds of input. Play back and compare game
     state

## Combos

| ID       | Derived From |
|----------|--------------|
| R-6.2.8  |              |
| R-6.2.8a |              |

1. **R-6.2.8** — The engine **SHALL** support combo input trees as visual graph assets with
   directional motion sequences (quarter-circle forward, dragon punch, charge inputs), branching
   states, configurable leniency windows, and input buffering. Combo definitions **SHALL** be
   authored in the visual editor. [F-6.2.8](../../features/input/input-actions-and-mapping.md)
   serves the no-code constraint. within window. Assert combo fires. Input with timeout. Assert
   combo resets to root.
   - **Rationale:** Fighting-game-grade combos require structured input recognition; visual
     authoring
   - **Verification:** Unit test: author a quarter-circle-forward
2. **R-6.2.8a** — The engine **SHALL** normalize directional inputs from sticks, D-pad, and WASD to
   identical cardinal and diagonal directions, so combos authored once work across all input
   devices. [F-6.2.8](../../features/input/input-actions-and-mapping.md) method. three trigger the
   same combo.
   - **Rationale:** Players use different devices; combos must feel identical regardless of input
   - **Verification:** Unit test: input a quarter-circle-forward via stick, D-pad, and WASD. Assert
     all

## Input Buffering

| ID       | Derived From |
|----------|--------------|
| R-6.2.9  |              |
| R-6.2.9a |              |

1. **R-6.2.9** — The engine **SHALL** buffer the most recent input during active ability frames with
   a configurable buffer duration (100-200 ms) and execute it when the current action enters a
   cancel window or completes. Cancel windows **SHALL** be defined per ability as frame ranges with
   permitted action categories. [F-6.2.9](../../features/input/input-actions-and-mapping.md) 250 ms.
   Assert dodge executes at 300 ms. Press dodge after buffer expires. Assert it does not execute.
   - **Rationale:** Input buffering enables responsive action combat without frame-perfect timing.
   - **Verification:** Unit test: trigger a 500 ms ability with cancel at 300-500 ms. Press dodge at
2. **R-6.2.9a** — The engine **SHALL** resolve conflicts when multiple inputs are buffered during
   the same window using priority ordering (dodge > attack > movement).
   [F-6.2.9](../../features/input/input-actions-and-mapping.md) should execute. executes (higher
   priority) and attack is discarded.
   - **Rationale:** When multiple inputs arrive during recovery frames, the highest-priority action
   - **Verification:** Unit test: buffer dodge and attack during the same cancel window. Assert
     dodge

## Advanced Filtering

| ID        | Derived From |
|-----------|--------------|
| R-6.2.10  |              |
| R-6.2.10a |              |
| R-6.2.10b |              |

1. **R-6.2.10** — The engine **SHALL** provide a low-pass filter modifier with configurable time
   constant that reduces analog stick jitter by at least 80% without adding more than 16 ms of
   latency. [F-6.2.10](../../features/input/input-actions-and-mapping.md) add perceptible lag. by at
   least 80%. Measure added latency. Assert it does not exceed 16 ms.
   - **Rationale:** Worn controllers produce jitter that degrades camera control; smoothing must not
   - **Verification:** Unit test: feed jittery input with 50 ms smoothing. Assert variance is
     reduced
2. **R-6.2.10a** — The engine **SHALL** support acceleration curves with configurable ramp-up time,
   max multiplier, and decay rate, increasing sensitivity proportional to input velocity.
   [F-6.2.10](../../features/input/input-actions-and-mapping.md) turns. velocity equals 2x base
   sensitivity.
   - **Rationale:** Acceleration enables slow movements for precision and fast movements for quick
   - **Verification:** Unit test: set acceleration at 2x max multiplier. Assert output at max input
3. **R-6.2.10b** — The engine **SHALL** support aim assist with magnetism (crosshair deflection
   toward valid targets), friction (sensitivity reduction over targets), and snap (instant snap on
   ADS activation), each configurable per weapon type and game mode. Aim assist **SHALL** be
   disableable per game mode. [F-6.2.10](../../features/input/input-actions-and-mapping.md)
   disableable for competitive PvP fairness. Disable magnetism. Assert no deflection. Verify aim
   assist is disabled in a competitive game mode.
   - **Rationale:** Gamepad players need aim assist for competitive parity with mouse; it must be
   - **Verification:** Unit test: enable magnetism. Assert crosshair deflects toward a valid target.

## Gamepad UI Navigation

| ID        | Derived From |
|-----------|--------------|
| R-6.2.11  |              |
| R-6.2.11a |              |

1. **R-6.2.11** — The engine **SHALL** support full UI navigability via gamepad using three modes:
   directional focus (D-pad/stick moves focus between widgets), virtual cursor (stick controls a
   software cursor), and action mapping (A/Cross = confirm, B/Circle = back, shoulder = tab switch).
   All mouse-hover-dependent behaviors **SHALL** activate on focus when gamepad is active.
   [F-6.2.11](../../features/input/input-actions-and-mapping.md) requires controller navigation
   without scripting. widget is reachable. Verify tooltips appear on focus (not hover) when gamepad
   is active. Switch to mouse mid-interaction and assert no focus state is lost.
   - **Rationale:** Console certification requires full UI navigability via controller; no-code
     engine
   - **Verification:** Integration test: navigate all UI screens using only a gamepad. Assert every
2. **R-6.2.11a** — The engine **SHALL** support switching between gamepad and mouse mid-interaction
   without losing UI focus state or requiring a restart.
   [F-6.2.11](../../features/input/input-actions-and-mapping.md) element retains focus. Click a
   different element. Assert focus transfers correctly.
   - **Rationale:** Players frequently switch devices; the transition must be invisible.
   - **Verification:** Unit test: navigate to a UI element via gamepad. Switch to mouse. Assert the

---
