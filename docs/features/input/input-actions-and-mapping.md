# 6.2 — Input Actions & Mapping

## Action Definitions

### F-6.2.1 Typed Input Actions

Define input actions as strongly typed values: boolean (pressed/not pressed), axis 1D (scalar
float), axis 2D (vector2), and axis 3D (vector3). Actions decouple gameplay logic from physical
device inputs, allowing the same "Dodge Roll" bool action or "Move" axis2D action to be triggered
by keyboard, gamepad stick, or touch virtual joystick without changing gameplay code.

- **Requirements:** R-6.2.1
- **Dependencies:** None
- **Platform notes:** Mobile uses touch virtual joystick as default axis2D source. Default
  action bindings differ per platform (touch, gamepad, keyboard/mouse).

## Input Mapping Contexts

### F-6.2.2 Input Mapping Contexts with Priority Stacking

Group input-to-action bindings into named mapping contexts (e.g., "OnFoot", "Mounted", "UIMenu",
"VehicleDriving") that can be activated and deactivated at runtime. Contexts are maintained on a
priority-ordered stack where higher-priority contexts consume matching inputs first. This enables
modal overlays (inventory captures Escape before combat) and layered controls (UI on top of
movement) common in MMOs with many distinct control schemes.

- **Requirements:** R-6.2.2
- **Dependencies:** F-6.2.1
- **Platform notes:** Mobile ships with touch-specific contexts (virtual joystick, gesture
  areas). Each platform provides default context sets matching its primary input device.

## Modifiers

### F-6.2.3 Input Value Modifiers

Apply configurable modifier chains to raw input values before they reach actions. Supported
modifiers include dead zones (axial and radial), response curves (linear, exponential, S-curve),
swizzle (remap axes), negate (invert axes), and scalar (sensitivity multiplier). Modifiers are
essential for tuning stick feel across different gamepads and providing per-player sensitivity
options common in MMO settings menus.

- **Requirements:** R-6.2.3
- **Dependencies:** F-6.2.1
- **Platform notes:** Dead zone defaults should vary by controller type — Xbox controllers
  typically need larger dead zones than DualSense. Response curve presets should be provided
  for common use cases (camera aim, vehicle steering, menu scrolling).

## Triggers

### F-6.2.4 Input Trigger Conditions

Define trigger conditions that control when an action fires: pressed (single frame), released
(on key-up), hold (sustained for duration), tap (press and release within threshold), pulse
(repeating on interval while held), chord (multiple inputs simultaneously), and combo (ordered
sequence within time window). An MMO action bar needs tap for instant casts, hold for channeled
abilities, chord (Shift+1) for extended action bar pages, and combos for ability chains.

- **Requirements:** R-6.2.4
- **Dependencies:** F-6.2.1
- **Platform notes:** Touch input on mobile uses longer tap/hold thresholds to accommodate
  finger imprecision. Chord triggers are limited on touch (max 2 simultaneous).

## Rebinding

### F-6.2.5 Runtime Key Rebinding with Conflict Detection

Allow players to rebind any action to any compatible input at runtime. Detect binding conflicts
within the same or overlapping mapping contexts and present resolution options (swap, unbind
previous, cancel). Rebindings are serialized to persistent storage and restored on session start.
Rebinding is a core accessibility requirement — MMO players with dozens of action bar slots must
customize their layouts freely.

- **Requirements:** R-6.2.5
- **Dependencies:** F-6.2.2, F-6.2.4
- **Platform notes:** Platform-specific default bindings should be provided (gamepad defaults
  differ between Xbox and PlayStation button labels). Rebinding must respect platform reserved
  keys (PS button, Guide button). Conflict detection must account for modifier keys and context
  priority to avoid false positives in non-overlapping contexts.

## Button Glyphs

### F-6.2.6 Platform-Aware Button Glyph Resolution

Automatically resolve input action bindings to the correct platform-specific button icons for
display in UI prompts, tutorials, and control overlays. When the active input device changes
(keyboard to gamepad to VR controller), all displayed button glyphs update instantly. The glyph
resolver maps each input action to the bound physical input, then to the platform-specific icon
atlas (Xbox buttons, PlayStation symbols, Switch buttons, keyboard keys, VR controller
diagrams). Glyph atlases are swappable assets supporting custom controller icon packs. The UI
widget system (F-10.1.7) binds glyph images reactively to the active device.

- **Requirements:** R-6.2.6
- **Dependencies:** F-6.2.1 (Input Actions), F-6.2.4 (Input Trigger Conditions),
  F-10.1.7 (Reactive Data Binding)
- **Platform notes:** Console certification requires showing correct platform-native button
  icons at all times.

## Advanced Combo Input

### F-6.2.8 Combo Input Trees and Directional Sequences

Fighting-game-grade combo input recognition supporting directional motion sequences (quarter-circle
forward, dragon punch, charge inputs, 360 motions), branching combo trees where the next valid
input depends on the current combo state, and multi-button simultaneous presses with configurable
leniency windows. Combo definitions are visual graph assets authored in the editor — each node
represents an input step with a timing window, and edges define valid transitions. The system
supports input buffering: inputs pressed during an ability's active frames are queued and executed
when the current action completes or enters a cancel window (F-6.2.9). Combo counter tracks the
current chain length for UI display and damage scaling. Failed combos (wrong input or timeout)
reset to the root state. Directional inputs are normalized across device types — stick directions,
D-pad, and WASD all map to the same cardinal/diagonal inputs.

- **Requirements:** R-6.2.8
- **Dependencies:** F-6.2.4 (Input Triggers), F-6.2.1 (Typed Actions)
- **Platform notes:** Touch combo input uses swipe directions mapped to cardinal/diagonal.
  Leniency windows are wider on touch (150 ms vs. 100 ms) for finger imprecision.

### F-6.2.9 Input Buffering and Ability Cancel Windows

Buffer the next input during the active frames of a current ability or animation, executing it
as soon as the current action enters a cancellable window or completes. Cancel windows are
defined per ability animation as frame ranges where specific action categories (dodge, jump,
special, any) are permitted to interrupt. The buffer stores the most recent buffered input with
a configurable buffer duration (typically 100-200ms). Priority rules resolve conflicts when
multiple inputs are buffered: dodge cancels take priority over attack chains, which take priority
over movement. The system integrates with the ability activation system (F-13.10.2) — abilities
declare which cancel categories they belong to and which categories can cancel them. This enables
responsive action game feel where players can queue their next action during recovery frames
without frame-perfect timing.

- **Requirements:** R-6.2.9
- **Dependencies:** F-6.2.8, F-13.10.2 (Ability Activation), F-9.1.9 (Animation Events)
- **Platform notes:** Mobile uses a longer default buffer duration (200 ms vs. 100 ms) to
  compensate for touch input latency and imprecision.

## Advanced Filtering

### F-6.2.10 Input Smoothing, Acceleration, and Aim Assist

Advanced input processing beyond basic modifiers. **Smoothing**: low-pass filter on analog stick
axes with configurable time constant to reduce jitter from worn controllers without adding
perceptible latency. **Acceleration**: mouse and stick acceleration curves that increase
sensitivity proportional to input velocity — slow movements are precise, fast movements cover
more ground. Acceleration is configurable with ramp-up time, max multiplier, and decay rate.
**Aim assist** (gamepad only): target magnetism (crosshair pulled toward nearest valid target
when close), friction (sensitivity reduction when crosshair passes over a target), and snap
(instant snap to nearest target on ADS activation). Aim assist strength is configurable per
weapon type and game mode (disabled in competitive PvP). All processing stages are composable
in the modifier chain (F-6.2.3) and individually toggleable per input action.

- **Requirements:** R-6.2.10
- **Dependencies:** F-6.2.3 (Input Modifiers), F-1.9.4 (Spatial Query for aim assist targets)
- **Platform notes:** Aim assist is standard on console platforms and typically disabled by default
  on PC with mouse input. Platform certification may require aim assist availability.

## Gamepad UI Navigation

### F-6.2.11 Controller-Driven UI Interaction

Comprehensive gamepad and keyboard navigation of the UI widget system (F-10.1.8) without
requiring a mouse or touch input. Navigation modes: **directional focus** (D-pad/stick moves
focus between widgets using the focus system's spatial navigation), **virtual cursor** (stick
controls a software cursor rendered as an overlay, simulating mouse movement for complex UIs
like world maps and skill trees), and **action mapping** (A/Cross = confirm, B/Circle = back,
shoulder buttons = tab switch, triggers = scroll). Context-sensitive button prompts update
dynamically as focus changes. The system handles complex widget patterns: scroll views advance
with D-pad when focused, sliders adjust with left/right, dropdown menus open with confirm and
navigate with D-pad, and radial menus use stick angle for selection. When a gamepad is the active
device, all mouse-hover-dependent UI behaviors (tooltips, highlights) activate on focus instead.
Mixed input is supported — switching between gamepad and mouse mid-interaction is seamless.

- **Requirements:** R-6.2.11
- **Dependencies:** F-6.2.1 (Input Actions), F-10.1.8 (Focus and Navigation), F-6.2.6 (Glyphs)
- **Platform notes:** Console certification requires full UI navigability via controller. The
  virtual cursor mode satisfies this requirement for UIs that cannot be easily focus-navigated.

## Recording

### F-6.2.7 Input Recording and Playback

Record all input events (device state snapshots, action firings, timestamps) to a binary stream
for deterministic playback. Recordings capture the complete input state per frame, enabling
automated testing (replay a recorded input sequence and compare game state), tutorial ghost
playback (show a recorded player's inputs as a ghost character), and bug reproduction (attach
input recording to bug reports). Playback supports speed control (slow-mo, fast-forward) and
frame-stepping. Recordings are stored as lightweight assets that reference the input mapping
context used during capture.

- **Requirements:** R-6.2.7
- **Dependencies:** F-6.2.1 (Input Actions), F-1.4.1 (Binary Serialization)
- **Platform notes:** Recordings are cross-platform at the action level (not raw device
  level). Mobile storage uses compressed recordings to limit file size.
