# R-6.1 — Device Abstraction User Stories

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

**As a** player (P-23), **I want** mouse aiming to be smooth and responsive at any DPI setting, **so
that** camera control feels natural.

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
and verify buttons, sticks, and triggers produce identical ECS values for equivalent inputs, **so
that** the unified abstraction is correct.

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

**As an** engine tester (P-27), **I want to** verify touch and pen pressure is normalized to [0.0,
1.0] across all platforms, **so that** pressure input is consistent.

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

**As a** player (P-23), **I want** touch controls to work responsively with multiple fingers, **so
that** mobile gameplay is smooth.

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
WM_DEVICECHANGE on Windows, IOHIDManager on macOS, and udev on Linux on a background thread, **so
that** hot-plug runs without blocking the game loop.

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
