# R-6.1 — Device Abstraction Requirements

## Keyboard and Mouse

1. **R-6.1.1** — The engine **SHALL** capture per-key press, release, and repeat events using
   platform-native APIs, delivering each event as an ECS event carrying both a layout-independent
   scancode and a locale-aware virtual keycode.
   - **Rationale:** Scancodes enable WASD bindings that work on any keyboard layout; virtual
     keycodes enable correct text display.
   - **Verification:** Simulate press, release, and repeat for 104 keys on each platform. Assert
     every event carries a valid scancode and keycode.

2. **R-6.1.2** — The engine **SHALL** normalize platform-specific scancodes to a common USB HID
   enumeration covering at least 256 key positions, such that the same physical key produces the
   same scancode value on Windows, macOS, and Linux.
   - **Rationale:** Cross-platform bindings require a single scancode space; per-platform tables
     break portable input assets.
   - **Verification:** Map the W key scancode on all three platforms and assert they produce the
     same enum value. Verify the full 104-key set maps without collisions.

3. **R-6.1.3** — The engine **SHALL** capture mouse button events (left, right, middle, Button4,
   Button5), sub-pixel cursor deltas via platform raw input APIs, absolute cursor position, and
   vertical/horizontal scroll, delivering each as an ECS event per frame.
   - **Rationale:** Sub-pixel deltas are required for smooth FPS camera control; absolute position
     drives UI targeting; scroll drives zoom and cycling.
   - **Verification:** Inject button, motion, and scroll events on each platform. Assert all event
     fields are populated. Verify sub-pixel precision (at least 1/256 pixel).

4. **R-6.1.4** — The engine **SHALL** normalize mouse deltas and cursor positions for high-DPI
   displays, producing consistent values regardless of OS DPI scaling factor.
   - **Rationale:** Without DPI normalization, mouse sensitivity varies across monitors and scaling
     settings.
   - **Verification:** Inject identical raw deltas at 100% and 200% DPI scaling. Assert normalized
     delta values are equal within 1% tolerance.

5. **R-6.1.5** — The engine **SHALL** normalize trackpad continuous scroll and discrete mouse wheel
   scroll into unified floating-point axis values.
   - **Rationale:** Games must treat scroll identically whether from a notched mouse wheel or smooth
     trackpad.
   - **Verification:** Inject one mouse wheel notch and a trackpad scroll of equivalent magnitude.
     Assert both produce the same normalized value within 5%.

## Gamepad

6. **R-6.1.6** — The engine **SHALL** provide a unified gamepad abstraction exposing buttons, analog
   sticks, and triggers for Xbox (XInput/WGI), DualSense (HID), and Switch Pro (HID), such that
   gameplay systems receive identical `GamepadButton` and `GamepadAxis` values regardless of
   controller type.
   - **Rationale:** Gameplay systems must be device-agnostic; per-controller branches in game logic
     are not maintainable.
   - **Verification:** Connect Xbox, DualSense, and Switch Pro. Press the south-face button on each.
     Assert all three produce `GamepadButton::South`.

7. **R-6.1.7** — The engine **SHALL** expose gyroscope and accelerometer data as ECS events on
   controllers that support motion sensors, and derive orientation via Madgwick sensor fusion.
   Controllers without motion sensors **SHALL** report no motion events.
   - **Rationale:** Gyro aim enables precision targeting; sensor fusion provides stable orientation
     from noisy raw data.
   - **Verification:** Read gyro/accel from DualSense and verify 3-axis data. Read from Xbox and
     verify no motion events. Verify Madgwick produces a stable quaternion within 5 degrees of
     ground truth.

8. **R-6.1.8** — The engine **SHALL** expose per-device capability flags (has_gyroscope,
   has_accelerometer, has_touchpad, has_adaptive_triggers, has_nfc, has_rumble, has_hd_rumble)
   queryable at runtime.
   - **Rationale:** Features like adaptive triggers and HD rumble are hardware-specific; capability
     queries prevent errors on unsupported controllers.
   - **Verification:** Query capabilities on DualSense and assert has_adaptive_triggers is true.
     Query on Xbox and assert it is false.

## Touch and Pen

9. **R-6.1.9** — The engine **SHALL** track up to 10 simultaneous touch contacts with per-finger ID,
   position, pressure (normalized to [0.0, 1.0]), and contact area, using WM_POINTER on Windows,
   NSTouch on macOS, and libinput on Linux.
   - **Rationale:** Multi-touch is required for mobile virtual joysticks and gesture recognition.
   - **Verification:** Inject 10 simultaneous touch contacts and assert all 10 are tracked with
     correct finger IDs, positions, and pressure values.

10. **R-6.1.10** — The engine **SHALL** capture pen position, pressure (normalized to [0.0, 1.0]),
    tilt, and barrel button events as ECS events.
    - **Rationale:** Pen input supports creative tools and map annotation.
    - **Verification:** Inject pen move with pressure 0.5 and tilt (10, 20). Assert event fields
      match. Verify barrel button events.

## Device Lifecycle

11. **R-6.1.11** — The engine **SHALL** detect connection and disconnection of input devices at
    runtime using platform-native APIs, emitting `DeviceConnected` and `DeviceDisconnected` ECS
    events within one frame.
    - **Rationale:** Players switch devices mid-session; the engine must detect and adapt without
      restarting.
    - **Verification:** Connect a gamepad at runtime and assert `DeviceConnected` fires within one
      frame. Disconnect and assert `DeviceDisconnected` fires.

12. **R-6.1.12** — The engine **SHALL** perform hot-plug enumeration on a background thread or async
    task, ensuring device events do not block the game loop or cause frame time spikes exceeding 1
    ms.
    - **Rationale:** Blocking the game loop for device enumeration causes visible hitches during
      gameplay.
    - **Verification:** Connect and disconnect 100 times during active gameplay. Assert no frame
      exceeds 1 ms above baseline.

13. **R-6.1.13** — The engine **SHALL** enumerate all connected input devices within 5 ms at
    startup, producing `DeviceInfo` records with device type, vendor ID, product ID, name, and
    capabilities.
    - **Rationale:** Fast startup enumeration prevents input delay at launch.
    - **Verification:** Connect 4 devices, call enumerate, and assert all 4 are returned within 5
      ms.

## ECS Integration

14. **R-6.1.14** — The engine **SHALL** store all input device state (keyboard pressed keys, mouse
    position/deltas, gamepad axes/buttons, touch contacts) as ECS components, updated once per frame
    by the device poll system.
    - **Rationale:** All game state must live in the ECS (100% ECS-based constraint). External input
      state breaks query composition and parallel scheduling.
    - **Verification:** Press a key, read `KeyboardState` component, assert the key is marked
      pressed. Verify `MouseState`, `GamepadState`, and `TouchState` are queryable.

15. **R-6.1.15** — The engine **SHALL** track the most recently used input device and expose an
    `ActiveDeviceType` resource in the ECS world, updated automatically when input is received from
    a different device type.
    - **Rationale:** Glyph resolution and UI navigation mode depend on knowing the active device
      type.
    - **Verification:** Send keyboard input, assert active device is Keyboard. Send gamepad input,
      assert it switches to Gamepad within the same frame.
