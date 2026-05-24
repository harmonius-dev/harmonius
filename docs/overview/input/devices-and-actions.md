# Devices and Actions

Input hardware abstraction, event generation, and action mapping.

## What it covers

- Keyboard input: key down, key up events with modifier keys (shift, control, alt).
- Mouse input: position, button clicks, scroll wheels, raw input options.
- Gamepad input: analog sticks, triggers, buttons with dead zones.
- Touch input: multi-touch, pressure, and gesture recognition.
- Input binding: mapping raw input to abstract actions (jump, move, aim).
- Action state queries: "is jump pressed", "move direction vector".
- Input history: buffering inputs across several frames for replay or combo detection.
- Input remapping: user-configurable key and button bindings per platform.
- Raw input alternatives: circumventing OS input queueing for competitive gaming.
- Platform-specific input: native keyboard layout detection for international text entry.

## Concepts

### Hardware Abstraction

Input devices include keyboard, mouse, gamepad, and touch. The engine abstracts these into
normalized input types: key codes, analog axes, button IDs, and multi-touch points. Keyboard reports
key down/up events; gamepad reports analog stick positions (X, Y), trigger values (0–1), and button
states. Touch reports finger IDs, positions, and pressures.

### Action Mapping and State

Raw input (key codes) maps to abstract actions (jump, move, fire) via a binding table. The engine
queries action state: "is jump pressed" returns a boolean; "move direction" returns a vector. This
decoupling allows remapping controls without changing game code.

### Dead Zones and Normalization

Gamepad analog sticks have dead zones near center to ignore stick drift. Stick values normalize to
[–1, 1] range after applying dead zone. Triggers normalize to [0, 1]. This enables consistent
behavior across different hardware.

### Input Buffering and History

Input history stores recent raw input and action states, enabling frame-perfect input detection for
combo systems or replay. Buffering recent input across several frames helps fighters detect tight
timing windows and frame-data analysis tools.

### User Configuration and Platform Variants

Input bindings are user-configurable; the engine loads from a config file at startup and saves
changes. Different platforms have different input layout conventions (WASD on PC, L-stick on
gamepad, QWERTY vs AZERTY keyboards). Input mapping accounts for these and provides
localization-aware key labels.

## How it fits

- See [gestures-and-haptics.md](./gestures-and-haptics.md) for multi-touch gestures and haptic
  feedback.
- See [../game-framework/camera-and-controls.md](../game-framework/camera-and-controls.md) for
  control scheme implementation.
- Integrates with [../ui/widgets-and-framework.md](../ui/widgets-and-framework.md) for UI
  input handling.
