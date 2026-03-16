# User Stories — 6.1 Device Abstraction

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-6.1.1.1 | engine developer (P-26) | capture per-key press, release, and repeat events with scancode and virtual keycode | keyboard input is reported accurately |  |  |
| US-6.1.1.2 | player (P-23) | WASD movement to work identically on any keyboard layout | physical key positions are consistent |  |  |
| US-6.1.1.3 | engine developer (P-26) | scancodes for layout-independent physical key identification | movement bindings work regardless of locale |  |  |
| US-6.1.1.4 | engine developer (P-26) | virtual keycodes reflecting user locale | chat and text-oriented features work correctly |  |  |
| US-6.1.1.5 | engine developer (P-26) | platform scancode tables normalized to a common enumeration | input is cross-platform |  |  |
| US-6.1.1.6 | engine tester (P-27) | verify key capture works on Windows (WM_KEY), macOS (IOHIDManager), and Linux (evdev) | all backends are validated |  |  |
| US-6.1.1.7 | designer (P-5) | key bindings configurable in settings | players can customize their controls |  |  |
| US-6.1.1.8 | engine tester (P-27) | test key repeat timing | repeat events fire at the correct rate |  |  |
| US-6.1.1.9 | player (P-23) | chat input to produce correct characters for my language | text communication works |  |  |
| US-6.1.1.10 | engine tester (P-27) | test keyboard input on non-Latin layouts (AZERTY, QWERTZ, Cyrillic) | international support is validated |  |  |
| US-6.1.1.11 | engine developer (P-26) | WM_KEYDOWN/UP on Windows, IOHIDManager on macOS, and evdev on Linux | platform-native input is used |  |  |
| US-6.1.1.12 | QA tester (P-19) | verify scancode-to-keycode mapping on each platform | every key produces the correct code |  |  |
| US-6.1.2.1 | engine developer (P-26) | mouse button events (left, right, middle, extended) reported | click input is captured |  |  |
| US-6.1.2.2 | engine developer (P-26) | high-resolution cursor deltas for FPS camera control | mouse look is smooth |  |  |
| US-6.1.2.3 | engine developer (P-26) | absolute cursor position for UI targeting | mouse interaction with UI works |  |  |
| US-6.1.2.4 | player (P-23) | precise mouse aiming for combat | targeting feels responsive and accurate |  |  |
| US-6.1.2.5 | engine developer (P-26) | vertical and horizontal scroll events | zoom, inventory cycling, and scrolling work |  |  |
| US-6.1.2.6 | engine developer (P-26) | macOS trackpad continuous scroll and discrete mouse scroll normalized to a unified axis | scroll behavior is consistent |  |  |
| US-6.1.2.7 | engine developer (P-26) | high-DPI scaling accounted for in cursor position | targeting is pixel-accurate on Retina displays |  |  |
| US-6.1.2.8 | engine tester (P-27) | verify WM_INPUT provides sub-pixel deltas on Windows | high-DPI mouse motion is captured |  |  |
| US-6.1.2.9 | player (P-23) | mouse scroll to cycle through inventory | item selection is quick |  |  |
| US-6.1.2.10 | engine tester (P-27) | test extended mouse buttons (button 4, 5) | extra buttons are usable for bindings |  |  |
| US-6.1.2.11 | designer (P-5) | mouse sensitivity configurable in settings | players tune their aim speed |  |  |
| US-6.1.2.12 | QA tester (P-19) | verify mouse input on Windows, macOS, and Linux | all platforms work correctly |  |  |
| US-6.1.3.1 | engine developer (P-26) | a unified gamepad abstraction over XInput, DualSense, and Switch Pro | all controllers work through one API |  |  |
| US-6.1.3.2 | engine developer (P-26) | buttons, analog sticks, and triggers exposed per controller | all standard inputs are available |  |  |
| US-6.1.3.3 | player (P-23) | gyro aim for precision targeting | motion control enhances gamepad accuracy |  |  |
| US-6.1.3.4 | engine developer (P-26) | gyroscope and accelerometer data exposed per controller | motion input is available |  |  |
| US-6.1.3.5 | designer (P-5) | capability queries for touchpad, adaptive triggers, NFC, and gyro | features are gated by hardware |  |  |
| US-6.1.3.6 | engine developer (P-26) | Madgwick filter deriving orientation from raw gyro and accelerometer | motion input is stable |  |  |
| US-6.1.3.7 | engine tester (P-27) | verify gamepad abstraction on Windows (XInput), macOS (GCController), and Linux (evdev) | all backends work |  |  |
| US-6.1.3.8 | engine tester (P-27) | test DualSense HID report parsing for motion data | PS5 controller motion works |  |  |
| US-6.1.3.9 | player (P-23) | any controller to work immediately when plugged in | I do not need to configure drivers |  |  |
| US-6.1.3.10 | designer (P-5) | gamepad dead zones configurable in settings | stick drift is eliminated |  |  |
| US-6.1.3.11 | engine tester (P-27) | test Switch Pro controller motion data | gyro input works on Switch |  |  |
| US-6.1.3.12 | QA tester (P-19) | verify controller capability queries return correct results per controller type | feature gating is accurate |  |  |
| US-6.1.4.1 | engine developer (P-26) | simultaneous touch contacts tracked with per- finger ID, position, pressure, and area | multi-touch works |  |  |
| US-6.1.4.2 | engine developer (P-26) | pen position, pressure, tilt, and barrel button captured | pen input is available |  |  |
| US-6.1.4.3 | player (P-23) | virtual joystick and gesture controls on mobile | touch-based gameplay works |  |  |
| US-6.1.4.4 | engine developer (P-26) | pressure normalized to 0.0-1.0 across all backends | pressure values are consistent |  |  |
| US-6.1.4.5 | designer (P-5) | touch and gesture recognition running simultaneously | virtual joystick and camera gesture coexist |  |  |
| US-6.1.4.6 | engine developer (P-26) | WM_POINTER on Windows, NSTouch on macOS, and libinput on Linux | touch uses native APIs |  |  |
| US-6.1.4.7 | engine tester (P-27) | verify maximum simultaneous touch contacts per device (typically 5-10) | limits are documented |  |  |
| US-6.1.4.8 | player (P-23) | annotate maps with pen input | creative tools in player housing work |  |  |
| US-6.1.4.9 | engine tester (P-27) | test pen tilt and barrel button events | full pen input is validated |  |  |
| US-6.1.4.10 | designer (P-5) | touch sensitivity configurable per device type | touch input is tuned for different screens |  |  |
| US-6.1.4.11 | QA tester (P-19) | test multi-touch on Windows, macOS, and mobile | touch works across platforms |  |  |
| US-6.1.4.12 | engine tester (P-27) | verify pen input via Windows Ink (WM_POINTER) | pen support works on Windows |  |  |
| US-6.1.5.1 | engine developer (P-26) | input device connection detected at runtime | new controllers are recognized immediately |  |  |
| US-6.1.5.2 | engine developer (P-26) | input device disconnection detected | the system handles mid-game disconnects gracefully |  |  |
| US-6.1.5.3 | player (P-23) | switch between keyboard/mouse and gamepad without restarting | input transitions are seamless |  |  |
| US-6.1.5.4 | player (P-23) | the game to pause or show fallback behavior when my controller disconnects mid-combat | I am not penalized |  |  |
| US-6.1.5.5 | engine developer (P-26) | WM_DEVICECHANGE on Windows, IOHIDManager on macOS, and udev on Linux for hot-plug | detection is native |  |  |
| US-6.1.5.6 | engine developer (P-26) | device enumeration on a background thread | the game loop is not blocked |  |  |
| US-6.1.5.7 | engine tester (P-27) | verify hot-plug detection on Windows, macOS, and Linux | all platforms handle device changes |  |  |
| US-6.1.5.8 | engine tester (P-27) | test rapid connect/disconnect cycles | the system handles edge cases without crashes |  |  |
| US-6.1.5.9 | designer (P-5) | pause-on-disconnect behavior configurable | games choose between pause, AI takeover, or warning overlay |  |  |
| US-6.1.5.10 | player (P-23) | a notification when a device connects or disconnects | I know my input state |  |  |
| US-6.1.5.11 | QA tester (P-19) | verify connection/disconnection notifications appear correctly | player feedback is reliable |  |  |
| US-6.1.5.12 | engine developer (P-26) | all connected devices enumerated on startup | initial device state is known |  |  |
