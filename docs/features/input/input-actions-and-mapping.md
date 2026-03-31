# 6.2 — Input Actions & Mapping

## Action Definitions

| ID      | Feature             |
|---------|---------------------|
| F-6.2.1 | Typed Input Actions |

1. **F-6.2.1** — Define input actions as strongly typed values: boolean (pressed/not pressed), axis
   1D (scalar float), axis 2D (vector2), and axis 3D (vector3). Actions decouple gameplay logic from
   physical device inputs, allowing the same action to be triggered by keyboard, gamepad stick, or
   touch virtual joystick without changing gameplay logic.
   - **Platform:** Mobile uses touch virtual joystick as default axis2D source. Default action
     bindings differ per platform (touch, gamepad, keyboard/mouse).

## Input Mapping Contexts

| ID      | Feature                                       |
|---------|-----------------------------------------------|
| F-6.2.2 | Input Mapping Contexts with Priority Stacking |

1. **F-6.2.2** — Group input-to-action bindings into named mapping contexts (e.g., "OnFoot",
   "Mounted", "UIMenu", "VehicleDriving") that can be activated and deactivated at runtime. Contexts
   are maintained on a priority-ordered stack where higher-priority contexts consume matching inputs
   first. Each platform provides default context sets matching its primary input device.
   - **Deps:** F-6.2.1
   - **Platform:** Mobile ships with touch-specific contexts (virtual joystick, gesture areas).

## Modifiers

| ID      | Feature               |
|---------|-----------------------|
| F-6.2.3 | Input Value Modifiers |

1. **F-6.2.3** — Apply configurable modifier chains to raw input values before they reach actions.
   Supported modifiers include dead zones (axial and radial), response curves (linear, exponential,
   S-curve), swizzle (remap axes), negate (invert axes), and scalar (sensitivity multiplier).
   Response curve presets should be provided for common use cases (camera aim, vehicle steering,
   menu scrolling).
   - **Deps:** F-6.2.1
   - **Platform:** Dead zone defaults should vary by controller type — Xbox controllers typically
     need larger dead zones than DualSense.

## Triggers

| ID      | Feature                  |
|---------|--------------------------|
| F-6.2.4 | Input Trigger Conditions |

1. **F-6.2.4** — Define trigger conditions that control when an action fires: pressed (single
   frame), released (on key-up), hold (sustained for duration), tap (press and release within
   threshold), pulse (repeating on interval while held), chord (multiple inputs simultaneously), and
   combo (ordered sequence within time window).
   - **Deps:** F-6.2.1
   - **Platform:** Touch input on mobile uses longer tap/hold thresholds to accommodate finger
     imprecision. Chord triggers are limited on touch (max 2 simultaneous).

## Rebinding

| ID      | Feature                                       |
|---------|-----------------------------------------------|
| F-6.2.5 | Runtime Key Rebinding with Conflict Detection |

1. **F-6.2.5** — Allow players to rebind any action to any compatible input at runtime. Detect
   binding conflicts within the same or overlapping mapping contexts and present resolution options
   (swap, unbind previous, cancel). Rebindings are serialized to persistent storage and restored on
   session start. Rebinding must respect platform reserved keys (PS button, Guide button). Conflict
   detection must account for modifier keys and context priority to avoid false positives.
   - **Deps:** F-6.2.2, F-6.2.4
   - **Platform:** Platform-specific default bindings should be provided (gamepad defaults differ
     between Xbox and PlayStation button labels).

## Button Glyphs

| ID      | Feature                                |
|---------|----------------------------------------|
| F-6.2.6 | Platform-Aware Button Glyph Resolution |

1. **F-6.2.6** — Automatically resolve input action bindings to the correct platform-specific button
   icons for display in UI prompts, tutorials, and control overlays. When the active input device
   changes, all displayed button glyphs update instantly. The glyph resolver maps each input action
   to the bound physical input, then to the platform-specific icon atlas. Glyph atlases are
   swappable assets supporting custom controller icon packs. The UI widget system (F-10.1.7) binds
   glyph images reactively to the active device.
   - **Deps:** F-6.2.1, F-6.2.4, F-10.1.7
   - **Platform:** Console certification requires showing correct platform-native button icons at
     all times.

## Advanced Combo Input

| ID      | Feature                                     |
|---------|---------------------------------------------|
| F-6.2.8 | Combo Input Trees and Directional Sequences |
| F-6.2.9 | Input Buffering and Ability Cancel Windows  |

1. **F-6.2.8** — Fighting-game-grade combo input recognition supporting directional motion sequences
   (quarter-circle forward, dragon punch, charge inputs, 360 motions), branching combo trees, and
   multi-button simultaneous presses with configurable leniency windows. Combo definitions are
   visual graph assets authored in the editor. The system supports input buffering and a combo
   counter. Directional inputs are normalized across device types.
   - **Deps:** F-6.2.4, F-6.2.1
   - **Platform:** Touch combo input uses swipe directions mapped to cardinal/diagonal. Leniency
     windows are wider on touch (150 ms vs. 100 ms) for finger imprecision.
2. **F-6.2.9** — Buffer the next input during the active frames of a current ability or animation,
   executing it as soon as the current action enters a cancellable window or completes. Cancel
   windows are defined per ability animation as frame ranges where specific action categories are
   permitted to interrupt. Priority rules resolve conflicts when multiple inputs are buffered.
   - **Deps:** F-6.2.8, F-9.1.9 (Animation Events)
   - **Platform:** Mobile uses a longer default buffer duration (200 ms vs. 100 ms) to compensate
     for touch input latency and imprecision.

## Advanced Filtering

| ID       | Feature                                       |
|----------|-----------------------------------------------|
| F-6.2.10 | Input Smoothing, Acceleration, and Aim Assist |

1. **F-6.2.10** — Advanced input processing beyond basic modifiers. **Smoothing**: low-pass filter
   on analog stick axes to reduce jitter. **Acceleration**: sensitivity increases proportional to
   input velocity. **Aim assist** (gamepad only): target magnetism, friction, and snap. Aim assist
   strength is configurable per weapon type and game mode (disabled in competitive PvP). All stages
   are composable in the modifier chain (F-6.2.3).
   - **Deps:** F-6.2.3, F-1.9.4 (Spatial Query)
   - **Platform:** Aim assist is standard on console and typically disabled by default on PC with
     mouse input.

## Gamepad UI Navigation

| ID       | Feature                          |
|----------|----------------------------------|
| F-6.2.11 | Controller-Driven UI Interaction |

1. **F-6.2.11** — Comprehensive gamepad and keyboard navigation of the UI widget system (F-10.1.8)
   without requiring a mouse or touch input. Navigation modes: **directional focus**,
   **virtual cursor**, and **action mapping** (A/Cross = confirm, B/Circle = back, shoulder buttons
   = tab switch, triggers = scroll). When a gamepad is the active device, all mouse-hover-dependent
   UI behaviors activate on focus instead. Mixed input is supported.
   - **Deps:** F-6.2.1, F-10.1.8, F-6.2.6
   - **Platform:** Console certification requires full UI navigability via controller.

## Recording

| ID      | Feature                      |
|---------|------------------------------|
| F-6.2.7 | Input Recording and Playback |

1. **F-6.2.7** — Record all input events (device state snapshots, action firings, timestamps) to a
   binary stream for deterministic playback. Playback supports speed control (slow-mo, fast-forward)
   and frame-stepping. Recordings are stored as lightweight assets that reference the input mapping
   context used during capture. Mobile storage uses compressed recordings to limit file size.
   - **Deps:** F-6.2.1, F-1.4.1 (Binary Serialization)
   - **Platform:** Recordings are cross-platform at the action level (not raw device level).
