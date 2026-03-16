# User Stories — 6.1 Device Abstraction

## US-6.1.1.1 Capture Key Press and Release Events

**As an** engine developer (P-26), **I want to** capture per-key press, release, and repeat events
with scancode and virtual keycode, **so that** keyboard input is reported accurately.

## US-6.1.1.2 Use WASD on Any Keyboard Layout

**As a** player (P-23), **I want** WASD movement to work identically on any keyboard layout,
**so that** physical key positions are consistent.

## US-6.1.1.3 Provide Scancodes for Layout-Independent Input

**As an** engine developer (P-26), **I want** scancodes for layout-independent physical key
identification, **so that** movement bindings work regardless of locale.

## US-6.1.1.4 Provide Virtual Keycodes for Text Input

**As an** engine developer (P-26), **I want** virtual keycodes reflecting user locale, **so that**
chat and text-oriented features work correctly.

## US-6.1.1.5 Normalize Scancode Tables Across Platforms

**As an** engine developer (P-26), **I want** platform scancode tables normalized to a common
enumeration, **so that** input is cross-platform.

## US-6.1.1.6 Verify Key Capture on All Platforms

**As an** engine tester (P-27), **I want to** verify key capture works on Windows (WM_KEY), macOS
(IOHIDManager), and Linux (evdev), **so that** all backends are validated.

## US-6.1.1.7 Configure Key Bindings in Settings

**As a** designer (P-5), **I want** key bindings configurable in settings, **so that** players can
customize their controls.

## US-6.1.1.8 Test Key Repeat Behavior

**As an** engine tester (P-27), **I want to** test key repeat timing, **so that** repeat events fire
at the correct rate.

## US-6.1.1.9 Type in Chat with Correct Characters

**As a** player (P-23), **I want** chat input to produce correct characters for my language,
**so that** text communication works.

## US-6.1.1.10 Test Non-Latin Keyboard Layouts

**As an** engine tester (P-27), **I want to** test keyboard input on non-Latin layouts (AZERTY,
QWERTZ, Cyrillic), **so that** international support is validated.

## US-6.1.1.11 Use Standard Platform APIs

**As an** engine developer (P-26), **I want** WM_KEYDOWN/UP on Windows, IOHIDManager on macOS, and
evdev on Linux, **so that** platform-native input is used.

## US-6.1.1.12 Verify Scancode-Keycode Mapping

**As a** QA tester (P-19), **I want to** verify scancode-to-keycode mapping on each platform,
**so that** every key produces the correct code.

## US-6.1.2.1 Report Mouse Button Events

**As an** engine developer (P-26), **I want** mouse button events (left, right, middle, extended)
reported, **so that** click input is captured.

## US-6.1.2.2 Report High-Resolution Cursor Deltas

**As an** engine developer (P-26), **I want** high-resolution cursor deltas for FPS camera control,
**so that** mouse look is smooth.

## US-6.1.2.3 Report Absolute Cursor Position

**As an** engine developer (P-26), **I want** absolute cursor position for UI targeting, **so that**
mouse interaction with UI works.

## US-6.1.2.4 Aim Precisely with Mouse

**As a** player (P-23), **I want** precise mouse aiming for combat, **so that** targeting feels
responsive and accurate.

## US-6.1.2.5 Report Vertical and Horizontal Scroll

**As an** engine developer (P-26), **I want** vertical and horizontal scroll events, **so that**
zoom, inventory cycling, and scrolling work.

## US-6.1.2.6 Normalize Trackpad and Mouse Scroll

**As an** engine developer (P-26), **I want** macOS trackpad continuous scroll and discrete mouse
scroll normalized to a unified axis, **so that** scroll behavior is consistent.

## US-6.1.2.7 Account for High-DPI Scaling

**As an** engine developer (P-26), **I want** high-DPI scaling accounted for in cursor position,
**so that** targeting is pixel-accurate on Retina displays.

## US-6.1.2.8 Verify Raw Input on Windows

**As an** engine tester (P-27), **I want to** verify WM_INPUT provides sub-pixel deltas on Windows,
**so that** high-DPI mouse motion is captured.

## US-6.1.2.9 Scroll Through Inventory

**As a** player (P-23), **I want** mouse scroll to cycle through inventory, **so that** item
selection is quick.

## US-6.1.2.10 Test Extended Mouse Buttons

**As an** engine tester (P-27), **I want to** test extended mouse buttons (button 4, 5), **so that**
extra buttons are usable for bindings.

## US-6.1.2.11 Configure Mouse Sensitivity in Settings

**As a** designer (P-5), **I want** mouse sensitivity configurable in settings, **so that** players
tune their aim speed.

## US-6.1.2.12 Verify Mouse Input Across Platforms

**As a** QA tester (P-19), **I want to** verify mouse input on Windows, macOS, and Linux,
**so that** all platforms work correctly.

## US-6.1.3.1 Provide Unified Gamepad Abstraction

**As an** engine developer (P-26), **I want** a unified gamepad abstraction over XInput, DualSense,
and Switch Pro, **so that** all controllers work through one API.

## US-6.1.3.2 Expose Buttons, Sticks, and Triggers

**As an** engine developer (P-26), **I want** buttons, analog sticks, and triggers exposed per
controller, **so that** all standard inputs are available.

## US-6.1.3.3 Support Gyro Aim on Compatible Controllers

**As a** player (P-23), **I want** gyro aim for precision targeting, **so that** motion control
enhances gamepad accuracy.

## US-6.1.3.4 Expose Motion Sensors

**As an** engine developer (P-26), **I want** gyroscope and accelerometer data exposed per
controller, **so that** motion input is available.

## US-6.1.3.5 Query Controller Capabilities

**As a** designer (P-5), **I want** capability queries for touchpad, adaptive triggers, NFC, and
gyro, **so that** features are gated by hardware.

## US-6.1.3.6 Apply Madgwick Sensor Fusion

**As an** engine developer (P-26), **I want** Madgwick filter deriving orientation from raw gyro and
accelerometer, **so that** motion input is stable.

## US-6.1.3.7 Verify Controller Abstraction Per Platform

**As an** engine tester (P-27), **I want to** verify gamepad abstraction on Windows (XInput), macOS
(GCController), and Linux (evdev), **so that** all backends work.

## US-6.1.3.8 Test DualSense HID Report Parsing

**As an** engine tester (P-27), **I want to** test DualSense HID report parsing for motion data,
**so that** PS5 controller motion works.

## US-6.1.3.9 Play with Any Controller Seamlessly

**As a** player (P-23), **I want** any controller to work immediately when plugged in, **so that** I
do not need to configure drivers.

## US-6.1.3.10 Configure Gamepad Dead Zones in Settings

**As a** designer (P-5), **I want** gamepad dead zones configurable in settings, **so that** stick
drift is eliminated.

## US-6.1.3.11 Test Switch Pro Motion Input

**As an** engine tester (P-27), **I want to** test Switch Pro controller motion data, **so that**
gyro input works on Switch.

## US-6.1.3.12 Verify Controller Feature Gating

**As a** QA tester (P-19), **I want to** verify controller capability queries return correct results
per controller type, **so that** feature gating is accurate.

## US-6.1.4.1 Track Simultaneous Touch Contacts

**As an** engine developer (P-26), **I want** simultaneous touch contacts tracked with per- finger
ID, position, pressure, and area, **so that** multi-touch works.

## US-6.1.4.2 Capture Pen Position and Pressure

**As an** engine developer (P-26), **I want** pen position, pressure, tilt, and barrel button
captured, **so that** pen input is available.

## US-6.1.4.3 Use Touch Controls on Mobile

**As a** player (P-23), **I want** virtual joystick and gesture controls on mobile, **so that**
touch-based gameplay works.

## US-6.1.4.4 Normalize Pressure to 0-1 Range

**As an** engine developer (P-26), **I want** pressure normalized to 0.0-1.0 across all backends,
**so that** pressure values are consistent.

## US-6.1.4.5 Support Touch and Gesture Simultaneously

**As a** designer (P-5), **I want** touch and gesture recognition running simultaneously,
**so that** virtual joystick and camera gesture coexist.

## US-6.1.4.6 Use Platform-Native Touch APIs

**As an** engine developer (P-26), **I want** WM_POINTER on Windows, NSTouch on macOS, and libinput
on Linux, **so that** touch uses native APIs.

## US-6.1.4.7 Verify Touch Contact Limits

**As an** engine tester (P-27), **I want to** verify maximum simultaneous touch contacts per device
(typically 5-10), **so that** limits are documented.

## US-6.1.4.8 Annotate Maps with Pen

**As a** player (P-23), **I want to** annotate maps with pen input, **so that** creative tools in
player housing work.

## US-6.1.4.9 Test Pen Tilt and Barrel Button

**As an** engine tester (P-27), **I want to** test pen tilt and barrel button events, **so that**
full pen input is validated.

## US-6.1.4.10 Configure Touch Sensitivity

**As a** designer (P-5), **I want** touch sensitivity configurable per device type, **so that**
touch input is tuned for different screens.

## US-6.1.4.11 Test Touch on Multiple Platforms

**As a** QA tester (P-19), **I want to** test multi-touch on Windows, macOS, and mobile, **so that**
touch works across platforms.

## US-6.1.4.12 Verify Pen Input on Windows Ink

**As an** engine tester (P-27), **I want to** verify pen input via Windows Ink (WM_POINTER),
**so that** pen support works on Windows.

## US-6.1.5.1 Detect Device Connection at Runtime

**As an** engine developer (P-26), **I want** input device connection detected at runtime,
**so that** new controllers are recognized immediately.

## US-6.1.5.2 Detect Device Disconnection at Runtime

**As an** engine developer (P-26), **I want** input device disconnection detected, **so that** the
system handles mid-game disconnects gracefully.

## US-6.1.5.3 Switch Between Keyboard and Gamepad Seamlessly

**As a** player (P-23), **I want to** switch between keyboard/mouse and gamepad without restarting,
**so that** input transitions are seamless.

## US-6.1.5.4 Handle Mid-Combat Disconnects Gracefully

**As a** player (P-23), **I want** the game to pause or show fallback behavior when my controller
disconnects mid-combat, **so that** I am not penalized.

## US-6.1.5.5 Use Platform-Native Hot-Plug APIs

**As an** engine developer (P-26), **I want** WM_DEVICECHANGE on Windows, IOHIDManager on macOS, and
udev on Linux for hot-plug, **so that** detection is native.

## US-6.1.5.6 Run Enumeration on Background Thread

**As an** engine developer (P-26), **I want** device enumeration on a background thread, **so that**
the game loop is not blocked.

## US-6.1.5.7 Verify Hot-Plug on All Platforms

**As an** engine tester (P-27), **I want to** verify hot-plug detection on Windows, macOS, and
Linux, **so that** all platforms handle device changes.

## US-6.1.5.8 Test Rapid Connect-Disconnect Cycles

**As an** engine tester (P-27), **I want to** test rapid connect/disconnect cycles, **so that** the
system handles edge cases without crashes.

## US-6.1.5.9 Configure Pause on Disconnect

**As a** designer (P-5), **I want** pause-on-disconnect behavior configurable, **so that** games
choose between pause, AI takeover, or warning overlay.

## US-6.1.5.10 Show Device Connection Notification

**As a** player (P-23), **I want** a notification when a device connects or disconnects, **so that**
I know my input state.

## US-6.1.5.11 Test Notification During Gameplay

**As a** QA tester (P-19), **I want to** verify connection/disconnection notifications appear
correctly, **so that** player feedback is reliable.

## US-6.1.5.12 Enumerate All Connected Devices on Startup

**As an** engine developer (P-26), **I want** all connected devices enumerated on startup,
**so that** initial device state is known.
