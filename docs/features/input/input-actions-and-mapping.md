# 6.2 — Input Actions & Mapping

## Action Definitions

### F-6.2.1 Typed Input Actions

Define input actions as strongly typed values: boolean (pressed/not pressed), axis 1D (scalar
float), axis 2D (vector2), and axis 3D (vector3). Actions decouple gameplay logic from physical
device inputs, allowing the same "Dodge Roll" bool action or "Move" axis2D action to be triggered
by keyboard, gamepad stick, or touch virtual joystick without changing gameplay code.

- **Requirements:** R-6.2.1
- **Dependencies:** None
- **Platform notes:** None

## Input Mapping Contexts

### F-6.2.2 Input Mapping Contexts with Priority Stacking

Group input-to-action bindings into named mapping contexts (e.g., "OnFoot", "Mounted", "UIMenu",
"VehicleDriving") that can be activated and deactivated at runtime. Contexts are maintained on a
priority-ordered stack where higher-priority contexts consume matching inputs first. This enables
modal overlays (inventory captures Escape before combat) and layered controls (UI on top of
movement) common in MMOs with many distinct control schemes.

- **Requirements:** R-6.2.2
- **Dependencies:** F-6.2.1
- **Platform notes:** None

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
- **Platform notes:** None

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
