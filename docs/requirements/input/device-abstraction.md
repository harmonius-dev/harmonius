# R-6.1 — Device Abstraction Requirements

## Keyboard and Mouse

### R-6.1.1 Keyboard Input Capture

The engine **SHALL** capture per-key press, release, and repeat events using platform-native APIs
(WM_KEYDOWN/WM_KEYUP on Windows, IOHIDManager on macOS, evdev on Linux), delivering each event as an
ECS event carrying both a layout-independent scancode and a locale-aware virtual keycode.

- **Derived from:**
  [F-6.1.1](../../features/input/device-abstraction.md)
- **Rationale:** Scancodes enable WASD bindings that work on any keyboard layout; virtual keycodes
  enable correct text display. Platform-native capture avoids winit dependency.
- **Verification:** Unit test: simulate press, release, and repeat for 104 keys on each platform.
  Assert every event carries a valid scancode and keycode. Verify scancodes are identical on QWERTY
  and AZERTY for the same physical key.

### R-6.1.1a Scancode Normalization

The engine **SHALL** normalize platform-specific scancodes to a common USB HID enumeration covering
at least 256 key positions, such that the same physical key produces the same scancode value on
Windows, macOS, and Linux.

- **Derived from:**
  [F-6.1.1](../../features/input/device-abstraction.md)
- **Rationale:** Cross-platform bindings require a single scancode space; per-platform tables break
  portable input assets.
- **Verification:** Unit test: map the W key scancode on all three platforms and assert they produce
  the same enum value. Verify the full 104-key set maps without collisions.

### R-6.1.2 Mouse Button, Motion, and Scroll Input

The engine **SHALL** capture mouse button events (left, right, middle, Button4, Button5), sub-pixel
cursor deltas via platform raw input APIs (WM_INPUT on Windows, CGEvent on macOS, evdev on Linux),
absolute cursor position, and vertical/horizontal scroll, delivering each as an ECS event per frame.

- **Derived from:**
  [F-6.1.2](../../features/input/device-abstraction.md)
- **Rationale:** Sub-pixel deltas are required for smooth FPS camera control; absolute position
  drives UI targeting; scroll drives zoom and cycling.
- **Verification:** Unit test: inject button, motion, and scroll events on each platform. Assert all
  event fields are populated. Benchmark: measure delta resolution and verify sub-pixel precision (at
  least 1/256 pixel).

### R-6.1.2a High-DPI Mouse Normalization

The engine **SHALL** normalize mouse deltas and cursor positions for high-DPI displays, producing
consistent values regardless of OS DPI scaling factor.

- **Derived from:**
  [F-6.1.2](../../features/input/device-abstraction.md)
- **Rationale:** Without DPI normalization, mouse sensitivity varies across monitors and scaling
  settings, breaking user experience.
- **Verification:** Unit test: inject identical raw deltas at 100% and 200% DPI scaling. Assert
  normalized delta values are equal within 1% tolerance.

### R-6.1.2b Unified Scroll Normalization

The engine **SHALL** normalize trackpad continuous scroll and discrete mouse wheel scroll into
unified floating-point axis values, so both produce the same semantic range.

- **Derived from:**
  [F-6.1.2](../../features/input/device-abstraction.md)
- **Rationale:** Games must treat scroll identically whether from a notched mouse wheel or a smooth
  trackpad.
- **Verification:** Unit test: inject one mouse wheel notch and a trackpad scroll of equivalent
  magnitude. Assert both produce the same normalized value within 5% tolerance.

## Gamepad

### R-6.1.3 Unified Gamepad Abstraction

The engine **SHALL** provide a unified gamepad abstraction exposing buttons, analog sticks, and
triggers for Xbox (via XInput/WGI on Windows), DualSense (via HID on all platforms), and Switch Pro
(via HID on all platforms), such that gameplay systems receive identical `GamepadButton` and
`GamepadAxis` values regardless of controller type.

- **Derived from:**
  [F-6.1.3](../../features/input/device-abstraction.md)
- **Rationale:** Gameplay systems must be device-agnostic; per-controller branches in game logic are
  not maintainable.
- **Verification:** Integration test: connect Xbox, DualSense, and Switch Pro controllers. Press the
  south-face button on each. Assert all three produce `GamepadButton::South` with identical
  `pressed = true` state.

### R-6.1.3a Gamepad Motion Sensor Support

The engine **SHALL** expose gyroscope and accelerometer data as ECS events on controllers that
support motion sensors (DualSense, Switch Pro), and **SHALL** derive orientation via Madgwick sensor
fusion. Controllers without motion sensors (Xbox) **SHALL** report no motion events.

- **Derived from:**
  [F-6.1.3](../../features/input/device-abstraction.md)
- **Rationale:** Gyro aim enables precision targeting; sensor fusion provides stable orientation
  from noisy raw data.
- **Verification:** Unit test: read gyro/accel from DualSense and verify 3-axis data is present.
  Read from Xbox and verify no motion events are emitted. Verify Madgwick fusion produces a stable
  quaternion within 5 degrees of ground truth.

### R-6.1.3b Gamepad Capability Queries

The engine **SHALL** expose per-device capability flags (has_gyroscope, has_accelerometer,
has_touchpad, has_adaptive_triggers, has_nfc, has_rumble, has_hd_rumble) queryable at runtime, so
that gameplay and editor systems can adapt features to the connected controller.

- **Derived from:**
  [F-6.1.3](../../features/input/device-abstraction.md)
- **Rationale:** Features like adaptive triggers and HD rumble are hardware-specific; capability
  queries prevent errors on unsupported controllers.
- **Verification:** Unit test: query capabilities on DualSense and assert has_adaptive_triggers is
  true. Query on Xbox and assert has_adaptive_triggers is false.

## Touch and Pen

### R-6.1.4 Multi-Touch Input

The engine **SHALL** track up to 10 simultaneous touch contacts with per-finger ID, position,
pressure (normalized to [0.0, 1.0]), and contact area, using WM_POINTER on Windows, NSTouch on
macOS, and libinput on Linux.

- **Derived from:**
  [F-6.1.4](../../features/input/device-abstraction.md)
- **Rationale:** Multi-touch is required for mobile virtual joysticks and gesture recognition;
  10-point tracking covers all production hardware.
- **Verification:** Unit test: inject 10 simultaneous touch contacts and assert all 10 are tracked
  with correct finger IDs, positions, and pressure values.

### R-6.1.4a Pen Input Capture

The engine **SHALL** capture pen position, pressure (normalized to [0.0, 1.0]), tilt (2D angle), and
barrel button events as ECS events, using WM_POINTER on Windows, NSEvent tablet events on macOS, and
libinput tablet tools on Linux.

- **Derived from:**
  [F-6.1.4](../../features/input/device-abstraction.md)
- **Rationale:** Pen input supports creative tools and map annotation in the editor and in-game.
- **Verification:** Unit test: inject pen move with pressure 0.5 and tilt (10, 20). Assert event
  fields match. Verify barrel button press and release produce correct events.

## Device Lifecycle

### R-6.1.5 Device Hot-Plug Detection

The engine **SHALL** detect connection and disconnection of input devices at runtime using
platform-native APIs (WM_DEVICECHANGE on Windows, IOHIDManager on macOS, udev on Linux), emitting
`DeviceConnected` and `DeviceDisconnected` ECS events within one frame of the OS notification.

- **Derived from:**
  [F-6.1.5](../../features/input/device-abstraction.md)
- **Rationale:** Players switch devices mid-session; the engine must detect and adapt without
  restarting.
- **Verification:** Integration test: connect a gamepad at runtime and assert `DeviceConnected`
  event fires within one frame. Disconnect and assert `DeviceDisconnected` fires within one frame.

### R-6.1.5a Non-Blocking Hot-Plug Detection

The engine **SHALL** perform hot-plug enumeration on a background thread or async task, ensuring
device connection and disconnection do not block the game loop or cause frame time spikes exceeding
1 ms.

- **Derived from:**
  [F-6.1.5](../../features/input/device-abstraction.md)
- **Rationale:** Blocking the game loop for device enumeration causes visible hitches during
  gameplay.
- **Verification:** Integration test: connect and disconnect a device 100 times during active
  gameplay. Assert no frame exceeds 1 ms above baseline. Profile to confirm enumeration runs on a
  non-main thread.

### R-6.1.5b Startup Device Enumeration

The engine **SHALL** enumerate all connected input devices within 5 ms at startup, producing
`DeviceInfo` records with device type, vendor ID, product ID, name, and capabilities.

- **Derived from:**
  [F-6.1.5](../../features/input/device-abstraction.md)
- **Rationale:** Fast startup enumeration prevents input delay at launch.
- **Verification:** Integration test: connect 4 devices. Call enumerate at startup and assert all 4
  are returned within 5 ms.

## ECS Integration

### R-6.1.6 Input State as ECS Components

The engine **SHALL** store all input device state (keyboard pressed keys, mouse position/deltas,
gamepad axes/buttons, touch contacts) as ECS components and resources, updated once per frame by the
device poll system.

- **Derived from:** [F-6.1.1](../../features/input/device-abstraction.md),
  [F-6.1.2](../../features/input/device-abstraction.md),
  [F-6.1.3](../../features/input/device-abstraction.md),
  [F-6.1.4](../../features/input/device-abstraction.md)
- **Rationale:** All game state must live in the ECS (100% ECS-based constraint). External input
  state breaks query composition and parallel scheduling.
- **Verification:** Unit test: press a key, read `KeyboardState` component, assert the key is marked
  pressed. Verify `MouseState`, `GamepadState`, and `TouchState` components are queryable from ECS
  systems.

### R-6.1.7 Active Device Tracking

The engine **SHALL** track the most recently used input device and expose an `ActiveDeviceType`
resource in the ECS world, updated automatically when input is received from a different device
type.

- **Derived from:** [F-6.1.5](../../features/input/device-abstraction.md),
  [F-6.1.3](../../features/input/device-abstraction.md)
- **Rationale:** Glyph resolution and UI navigation mode depend on knowing the active device type.
- **Verification:** Unit test: send keyboard input, assert active device is Keyboard. Send gamepad
  input, assert active device switches to Gamepad within the same frame.

---

## User Stories

## F-6.1.1 Keyboard Input Capture

## US-6.1.1.1 Capture Key Press and Release Events

**As a** game developer (P-15 -- via designer P-5), **I want** keyboard press, release, and repeat
events delivered as ECS events with scancode and virtual keycode, **so that** input is available in
the same frame it occurs.

## US-6.1.1.2 Configure Keyboard Bindings in Editor

**As a** designer (P-5), **I want to** view and configure keyboard bindings in the input settings
panel, **so that** default key mappings are set without code.

## US-6.1.1.3 Verify Scancode Layout Independence

**As an** engine tester (P-27), **I want to** press the physical W key on QWERTY and AZERTY layouts
and verify scancodes are identical while virtual keycodes differ, **so that** layout- independent
binding is confirmed.

## US-6.1.1.4 Verify All 104 Keys Generate Events

**As an** engine tester (P-27), **I want to** simulate press, release, and repeat for every key on a
104-key layout and assert each carries valid scancode and keycode, **so that** full keyboard
coverage is verified.

## US-6.1.1.5 Verify Cross-Platform Keyboard Support

**As an** engine tester (P-27), **I want to** verify keyboard input works on Windows, macOS, and
Linux using platform-native APIs, **so that** cross-platform coverage is confirmed.

## US-6.1.1.6 Implement Keyboard Capture Layer

**As an** engine developer (P-26), **I want to** implement keyboard capture using WM_KEYDOWN/
WM_KEYUP on Windows, IOHIDManager on macOS, and evdev on Linux, normalizing scancodes to a common
enumeration, **so that** keyboard input is cross-platform.

## US-6.1.1.7 Test Keyboard Responsiveness

**As a** QA tester (P-19), **I want to** verify keyboard input latency is imperceptible across all
platforms, **so that** typing and movement feel responsive.

## US-6.1.1.8 Type and Move with Keyboard

**As a** player (P-23), **I want** keyboard controls to work immediately regardless of my keyboard
layout, **so that** I can play the game on any keyboard.

---

## F-6.1.2 Mouse Button, Motion, and Scroll Input

## US-6.1.2.1 Capture Mouse Button Events

**As a** designer (P-5), **I want** mouse button events (left, right, middle, extended) reported as
ECS events each frame, **so that** clicking is usable for gameplay and UI.

## US-6.1.2.2 Capture Sub-Pixel Mouse Deltas

**As an** engine developer (P-26), **I want to** capture high-resolution sub-pixel mouse deltas via
raw input on each platform, **so that** FPS camera control is smooth and precise.

## US-6.1.2.3 Normalize Mouse Input for High-DPI

**As an** engine developer (P-26), **I want to** normalize mouse deltas and position for high-DPI
displays, **so that** mouse behavior is consistent across DPI settings.

## US-6.1.2.4 Verify High-DPI Normalization

**As an** engine tester (P-27), **I want to** verify delta and position values are correctly
scale-normalized on high-DPI displays, **so that** DPI scaling is handled.

## US-6.1.2.5 Verify Scroll Input Normalization

**As an** engine tester (P-27), **I want to** verify trackpad continuous scroll and discrete mouse
scroll are normalized into unified axis values, **so that** scroll input is consistent.

## US-6.1.2.6 Test Mouse on All Platforms

**As a** QA tester (P-19), **I want to** verify mouse input works correctly on Windows, macOS, and
Linux, **so that** cross-platform mouse support is confirmed.

## US-6.1.2.7 Aim Precisely with Mouse

**As a** player (P-23), **I want** mouse aiming to be smooth and responsive at any DPI setting,
**so that** camera control feels natural.

---

## F-6.1.3 Unified Gamepad Abstraction

## US-6.1.3.1 Configure Gamepad Bindings in Editor

**As a** designer (P-5), **I want to** configure gamepad button, stick, and trigger bindings in the
input settings panel, **so that** default gamepad mappings are set without code.

## US-6.1.3.2 Query Controller Capabilities

**As a** designer (P-5), **I want to** query controller capabilities (gyroscope, touchpad, adaptive
triggers) in the editor, **so that** I know what features are available per controller type.

## US-6.1.3.3 Verify Unified Abstraction Across Controllers

**As an** engine tester (P-27), **I want to** connect Xbox, DualSense, and Switch Pro controllers
and verify buttons, sticks, and triggers produce identical ECS values for equivalent inputs,
**so that** the unified abstraction is correct.

## US-6.1.3.4 Verify Gyroscope and Accelerometer Data

**As an** engine tester (P-27), **I want to** verify gyroscope data is available on DualSense and
Switch Pro and correctly absent on Xbox controllers, **so that** capability queries are accurate.

## US-6.1.3.5 Verify Cross-Platform Gamepad APIs

**As an** engine tester (P-27), **I want to** verify gamepad input uses XInput/WGI on Windows,
GCController on macOS, and evdev on Linux, **so that** platform API compliance is confirmed.

## US-6.1.3.6 Implement Unified Gamepad Layer

**As an** engine developer (P-26), **I want to** implement the unified gamepad abstraction over
XInput, DualSense, and Switch Pro with platform-specific HID parsing and Madgwick sensor fusion,
**so that** all controllers are unified.

## US-6.1.3.7 Test All Supported Controller Types

**As a** QA tester (P-19), **I want to** test every supported controller type on every platform,
**so that** gamepad compatibility is comprehensive.

## US-6.1.3.8 Play with Any Controller

**As a** player (P-23), **I want** my preferred controller (Xbox, PlayStation, Switch) to work
automatically, **so that** I can play with whatever controller I have.

---

## F-6.1.4 Multi-Touch and Pen Input

## US-6.1.4.1 Configure Touch Input Zones in Editor

**As a** designer (P-5), **I want to** define touch input zones (virtual joystick area, button area)
in the editor, **so that** mobile controls are authored visually.

## US-6.1.4.2 Track Simultaneous Touch Contacts

**As an** engine developer (P-26), **I want to** track up to 10 simultaneous touch contacts with
per-finger ID, position, pressure, and area, **so that** multi-touch gestures and virtual joysticks
work.

## US-6.1.4.3 Capture Pen Input

**As an** engine developer (P-26), **I want to** capture pen position, pressure, tilt, and barrel
button events, **so that** creative tools and map annotation are supported.

## US-6.1.4.4 Verify Pressure Normalization

**As an** engine tester (P-27), **I want to** verify touch and pen pressure is normalized to
[0.0, 1.0] across all platforms, **so that** pressure input is consistent.

## US-6.1.4.5 Verify 10-Point Multi-Touch

**As an** engine tester (P-27), **I want to** inject 10 simultaneous touch contacts and assert all
10 are tracked with correct data, **so that** full multi-touch is supported.

## US-6.1.4.6 Implement Touch and Pen Capture

**As an** engine developer (P-26), **I want to** implement multi-touch and pen capture using
WM_POINTER on Windows, NSTouch on macOS, and libinput on Linux, **so that** touch and pen are
cross-platform.

## US-6.1.4.7 Test Touch and Pen on All Platforms

**As a** QA tester (P-19), **I want to** test multi-touch and pen input on all supported platforms,
**so that** touch device compatibility is verified.

## US-6.1.4.8 Use Touch Controls on Mobile

**As a** player (P-23), **I want** touch controls to work responsively with multiple fingers,
**so that** mobile gameplay is smooth.

---

## F-6.1.5 Device Hot-Plug Detection and Enumeration

## US-6.1.5.1 Configure Hot-Plug Behavior in Editor

**As a** designer (P-5), **I want to** configure what happens on controller disconnect (pause, show
reconnect prompt) in the editor, **so that** disconnect behavior is authored without code.

## US-6.1.5.2 Verify Hot-Plug Event Timing

**As an** engine tester (P-27), **I want to** connect and disconnect a gamepad and assert events are
emitted within one frame without blocking the game loop, **so that** hot-plug is responsive.

## US-6.1.5.3 Verify No Frame Time Spike on Hot-Plug

**As an** engine tester (P-27), **I want to** verify game loop frame time does not spike above 1ms
during device connection or disconnection, **so that** hot-plug does not cause hitches.

## US-6.1.5.4 Verify Startup Enumeration Speed

**As an** engine tester (P-27), **I want to** connect 4 devices simultaneously and verify all are
enumerated within 5ms at startup, **so that** enumeration is fast.

## US-6.1.5.5 Implement Hot-Plug Detection

**As an** engine developer (P-26), **I want to** implement device hot-plug detection using
WM_DEVICECHANGE on Windows, IOHIDManager on macOS, and udev on Linux on a background thread,
**so that** hot-plug runs without blocking the game loop.

## US-6.1.5.6 Test Controller Disconnect During Gameplay

**As a** QA tester (P-19), **I want to** disconnect a controller during active gameplay and verify
the game responds appropriately (pause, prompt), **so that** the disconnect experience is graceful.

## US-6.1.5.7 Switch Between Keyboard and Gamepad Seamlessly

**As a** player (P-23), **I want to** switch between keyboard/mouse and gamepad at any time without
restarting, **so that** device switching is seamless.

---

## Non-Functional Requirements

### R-6.1.NF1 Input Polling Latency

The engine **SHALL** poll all connected input devices and deliver raw events to the action system
within 1 ms of the OS reporting the event.

- **Derived from:** F-6.1.1, F-6.1.2, F-6.1.3
- **Rationale:** Input latency directly degrades responsiveness. Keeping the OS-to-action pipeline
  under 1 ms ensures negligible latency contribution.
- **Verification:** Integration test: inject a timestamped OS event and measure delta to action
  system delivery. Assert p99 latency is below 1 ms over 10,000 events.

### R-6.1.NF2 Device Enumeration Throughput

The engine **SHALL** enumerate all connected input devices within 5 ms during startup and within 2
ms during hot-plug events, without blocking the game loop.

- **Derived from:** F-6.1.5
- **Rationale:** Slow enumeration during hot-plug can cause frame hitches.
- **Verification:** Integration test: connect 4 devices simultaneously. Assert enumeration and
  events complete within 5 ms. Assert game loop frame time does not spike above 1 ms.
