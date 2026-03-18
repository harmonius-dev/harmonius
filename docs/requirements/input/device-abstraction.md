# R-6.1 — Device Abstraction Requirements

## Keyboard and Mouse

| ID       | Derived From |
|----------|--------------|
| R-6.1.1  |              |
| R-6.1.1a |              |
| R-6.1.2  |              |
| R-6.1.2a |              |
| R-6.1.2b |              |

1. **R-6.1.1** — The engine **SHALL** capture per-key press, release, and repeat events using
   platform-native APIs (WM_KEYDOWN/WM_KEYUP on Windows, IOHIDManager on macOS, evdev on Linux),
   delivering each event as an ECS event carrying both a layout-independent scancode and a
   locale-aware virtual keycode. [F-6.1.1](../../features/input/device-abstraction.md) enable
   correct text display. Platform-native capture avoids winit dependency. Assert every event carries
   a valid scancode and keycode. Verify scancodes are identical on QWERTY and AZERTY for the same
   physical key.
   - **Rationale:** Scancodes enable WASD bindings that work on any keyboard layout; virtual
     keycodes
   - **Verification:** Unit test: simulate press, release, and repeat for 104 keys on each platform.
2. **R-6.1.1a** — The engine **SHALL** normalize platform-specific scancodes to a common USB HID
   enumeration covering at least 256 key positions, such that the same physical key produces the
   same scancode value on Windows, macOS, and Linux.
   [F-6.1.1](../../features/input/device-abstraction.md) portable input assets. the same enum value.
   Verify the full 104-key set maps without collisions.
   - **Rationale:** Cross-platform bindings require a single scancode space; per-platform tables
     break
   - **Verification:** Unit test: map the W key scancode on all three platforms and assert they
     produce
3. **R-6.1.2** — The engine **SHALL** capture mouse button events (left, right, middle, Button4,
   Button5), sub-pixel cursor deltas via platform raw input APIs (WM_INPUT on Windows, CGEvent on
   macOS, evdev on Linux), absolute cursor position, and vertical/horizontal scroll, delivering each
   as an ECS event per frame. [F-6.1.2](../../features/input/device-abstraction.md) drives UI
   targeting; scroll drives zoom and cycling. event fields are populated. Benchmark: measure delta
   resolution and verify sub-pixel precision (at least 1/256 pixel).
   - **Rationale:** Sub-pixel deltas are required for smooth FPS camera control; absolute position
   - **Verification:** Unit test: inject button, motion, and scroll events on each platform. Assert
     all
4. **R-6.1.2a** — The engine **SHALL** normalize mouse deltas and cursor positions for high-DPI
   displays, producing consistent values regardless of OS DPI scaling factor.
   [F-6.1.2](../../features/input/device-abstraction.md) settings, breaking user experience.
   normalized delta values are equal within 1% tolerance.
   - **Rationale:** Without DPI normalization, mouse sensitivity varies across monitors and scaling
   - **Verification:** Unit test: inject identical raw deltas at 100% and 200% DPI scaling. Assert
5. **R-6.1.2b** — The engine **SHALL** normalize trackpad continuous scroll and discrete mouse wheel
   scroll into unified floating-point axis values, so both produce the same semantic range.
   [F-6.1.2](../../features/input/device-abstraction.md) trackpad. magnitude. Assert both produce
   the same normalized value within 5% tolerance.
   - **Rationale:** Games must treat scroll identically whether from a notched mouse wheel or a
     smooth
   - **Verification:** Unit test: inject one mouse wheel notch and a trackpad scroll of equivalent

## Gamepad

| ID       | Derived From |
|----------|--------------|
| R-6.1.3  |              |
| R-6.1.3a |              |
| R-6.1.3b |              |

1. **R-6.1.3** — The engine **SHALL** provide a unified gamepad abstraction exposing buttons, analog
   sticks, and triggers for Xbox (via XInput/WGI on Windows), DualSense (via HID on all platforms),
   and Switch Pro (via HID on all platforms), such that gameplay systems receive identical
   `GamepadButton` and `GamepadAxis` values regardless of controller type.
   [F-6.1.3](../../features/input/device-abstraction.md) not maintainable. south-face button on
   each. Assert all three produce `GamepadButton::South` with identical `pressed = true` state.
   - **Rationale:** Gameplay systems must be device-agnostic; per-controller branches in game logic
     are
   - **Verification:** Integration test: connect Xbox, DualSense, and Switch Pro controllers. Press
     the
2. **R-6.1.3a** — The engine **SHALL** expose gyroscope and accelerometer data as ECS events on
   controllers that support motion sensors (DualSense, Switch Pro), and **SHALL** derive orientation
   via Madgwick sensor fusion. Controllers without motion sensors (Xbox) **SHALL** report no motion
   events. [F-6.1.3](../../features/input/device-abstraction.md) from noisy raw data. Read from Xbox
   and verify no motion events are emitted. Verify Madgwick fusion produces a stable quaternion
   within 5 degrees of ground truth.
   - **Rationale:** Gyro aim enables precision targeting; sensor fusion provides stable orientation
   - **Verification:** Unit test: read gyro/accel from DualSense and verify 3-axis data is present.
3. **R-6.1.3b** — The engine **SHALL** expose per-device capability flags (has_gyroscope,
   has_accelerometer, has_touchpad, has_adaptive_triggers, has_nfc, has_rumble, has_hd_rumble)
   queryable at runtime, so that gameplay and editor systems can adapt features to the connected
   controller. [F-6.1.3](../../features/input/device-abstraction.md) queries prevent errors on
   unsupported controllers. true. Query on Xbox and assert has_adaptive_triggers is false.
   - **Rationale:** Features like adaptive triggers and HD rumble are hardware-specific; capability
   - **Verification:** Unit test: query capabilities on DualSense and assert has_adaptive_triggers
     is

## Touch and Pen

| ID       | Derived From |
|----------|--------------|
| R-6.1.4  |              |
| R-6.1.4a |              |

1. **R-6.1.4** — The engine **SHALL** track up to 10 simultaneous touch contacts with per-finger ID,
   position, pressure (normalized to [0.0, 1.0]), and contact area, using WM_POINTER on Windows,
   NSTouch on macOS, and libinput on Linux. [F-6.1.4](../../features/input/device-abstraction.md)
   10-point tracking covers all production hardware. with correct finger IDs, positions, and
   pressure values.
   - **Rationale:** Multi-touch is required for mobile virtual joysticks and gesture recognition;
   - **Verification:** Unit test: inject 10 simultaneous touch contacts and assert all 10 are
     tracked
2. **R-6.1.4a** — The engine **SHALL** capture pen position, pressure (normalized to [0.0, 1.0]),
   tilt (2D angle), and barrel button events as ECS events, using WM_POINTER on Windows, NSEvent
   tablet events on macOS, and libinput tablet tools on Linux.
   [F-6.1.4](../../features/input/device-abstraction.md) fields match. Verify barrel button press
   and release produce correct events.
   - **Rationale:** Pen input supports creative tools and map annotation in the editor and in-game.
   - **Verification:** Unit test: inject pen move with pressure 0.5 and tilt (10, 20). Assert event

## Device Lifecycle

| ID       | Derived From |
|----------|--------------|
| R-6.1.5  |              |
| R-6.1.5a |              |
| R-6.1.5b |              |

1. **R-6.1.5** — The engine **SHALL** detect connection and disconnection of input devices at
   runtime using platform-native APIs (WM_DEVICECHANGE on Windows, IOHIDManager on macOS, udev on
   Linux), emitting `DeviceConnected` and `DeviceDisconnected` ECS events within one frame of the OS
   notification. [F-6.1.5](../../features/input/device-abstraction.md) restarting. event fires
   within one frame. Disconnect and assert `DeviceDisconnected` fires within one frame.
   - **Rationale:** Players switch devices mid-session; the engine must detect and adapt without
   - **Verification:** Integration test: connect a gamepad at runtime and assert `DeviceConnected`
2. **R-6.1.5a** — The engine **SHALL** perform hot-plug enumeration on a background thread or async
   task, ensuring device connection and disconnection do not block the game loop or cause frame time
   spikes exceeding 1 ms. [F-6.1.5](../../features/input/device-abstraction.md) gameplay. gameplay.
   Assert no frame exceeds 1 ms above baseline. Profile to confirm enumeration runs on a non-main
   thread.
   - **Rationale:** Blocking the game loop for device enumeration causes visible hitches during
   - **Verification:** Integration test: connect and disconnect a device 100 times during active
3. **R-6.1.5b** — The engine **SHALL** enumerate all connected input devices within 5 ms at startup,
   producing `DeviceInfo` records with device type, vendor ID, product ID, name, and capabilities.
   [F-6.1.5](../../features/input/device-abstraction.md) are returned within 5 ms.
   - **Rationale:** Fast startup enumeration prevents input delay at launch.
   - **Verification:** Integration test: connect 4 devices. Call enumerate at startup and assert all
     4

## ECS Integration

| ID      | Derived From                                           |
|---------|--------------------------------------------------------|
| R-6.1.6 | [F-6.1.1](../../features/input/device-abstraction.md), |
| R-6.1.7 | [F-6.1.5](../../features/input/device-abstraction.md), |

1. **R-6.1.6** — The engine **SHALL** store all input device state (keyboard pressed keys, mouse
   position/deltas, gamepad axes/buttons, touch contacts) as ECS components and resources, updated
   once per frame by the device poll system. [F-6.1.2](../../features/input/device-abstraction.md),
   [F-6.1.3](../../features/input/device-abstraction.md),
   [F-6.1.4](../../features/input/device-abstraction.md) state breaks query composition and parallel
   scheduling. pressed. Verify `MouseState`, `GamepadState`, and `TouchState` components are
   queryable from ECS systems.
   - **Rationale:** All game state must live in the ECS (100% ECS-based constraint). External input
   - **Verification:** Unit test: press a key, read `KeyboardState` component, assert the key is
     marked
2. **R-6.1.7** — The engine **SHALL** track the most recently used input device and expose an
   `ActiveDeviceType` resource in the ECS world, updated automatically when input is received from a
   different device type. [F-6.1.3](../../features/input/device-abstraction.md) input, assert active
   device switches to Gamepad within the same frame.
   - **Rationale:** Glyph resolution and UI navigation mode depend on knowing the active device
     type.
   - **Verification:** Unit test: send keyboard input, assert active device is Keyboard. Send
     gamepad

---
