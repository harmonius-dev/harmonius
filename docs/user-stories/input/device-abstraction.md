# User Stories — 6.1 Device Abstraction

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-6.1.1.1  | engine developer (P-26) |          |              |
| US-6.1.1.2  | player (P-23)           |          |              |
| US-6.1.1.3  | engine developer (P-26) |          |              |
| US-6.1.1.4  | engine developer (P-26) |          |              |
| US-6.1.1.5  | engine developer (P-26) |          |              |
| US-6.1.1.6  | engine tester (P-27)    |          |              |
| US-6.1.1.7  | designer (P-5)          |          |              |
| US-6.1.1.8  | engine tester (P-27)    |          |              |
| US-6.1.1.9  | player (P-23)           |          |              |
| US-6.1.1.10 | engine tester (P-27)    |          |              |
| US-6.1.1.11 | engine developer (P-26) |          |              |
| US-6.1.1.12 | QA tester (P-19)        |          |              |
| US-6.1.2.1  | engine developer (P-26) |          |              |
| US-6.1.2.2  | engine developer (P-26) |          |              |
| US-6.1.2.3  | engine developer (P-26) |          |              |
| US-6.1.2.4  | player (P-23)           |          |              |
| US-6.1.2.5  | engine developer (P-26) |          |              |
| US-6.1.2.6  | engine developer (P-26) |          |              |
| US-6.1.2.7  | engine developer (P-26) |          |              |
| US-6.1.2.8  | engine tester (P-27)    |          |              |
| US-6.1.2.9  | player (P-23)           |          |              |
| US-6.1.2.10 | engine tester (P-27)    |          |              |
| US-6.1.2.11 | designer (P-5)          |          |              |
| US-6.1.2.12 | QA tester (P-19)        |          |              |
| US-6.1.3.1  | engine developer (P-26) |          |              |
| US-6.1.3.2  | engine developer (P-26) |          |              |
| US-6.1.3.3  | player (P-23)           |          |              |
| US-6.1.3.4  | engine developer (P-26) |          |              |
| US-6.1.3.5  | designer (P-5)          |          |              |
| US-6.1.3.6  | engine developer (P-26) |          |              |
| US-6.1.3.7  | engine tester (P-27)    |          |              |
| US-6.1.3.8  | engine tester (P-27)    |          |              |
| US-6.1.3.9  | player (P-23)           |          |              |
| US-6.1.3.10 | designer (P-5)          |          |              |
| US-6.1.3.11 | engine tester (P-27)    |          |              |
| US-6.1.3.12 | QA tester (P-19)        |          |              |
| US-6.1.4.1  | engine developer (P-26) |          |              |
| US-6.1.4.2  | engine developer (P-26) |          |              |
| US-6.1.4.3  | player (P-23)           |          |              |
| US-6.1.4.4  | engine developer (P-26) |          |              |
| US-6.1.4.5  | designer (P-5)          |          |              |
| US-6.1.4.6  | engine developer (P-26) |          |              |
| US-6.1.4.7  | engine tester (P-27)    |          |              |
| US-6.1.4.8  | player (P-23)           |          |              |
| US-6.1.4.9  | engine tester (P-27)    |          |              |
| US-6.1.4.10 | designer (P-5)          |          |              |
| US-6.1.4.11 | QA tester (P-19)        |          |              |
| US-6.1.4.12 | engine tester (P-27)    |          |              |
| US-6.1.5.1  | engine developer (P-26) |          |              |
| US-6.1.5.2  | engine developer (P-26) |          |              |
| US-6.1.5.3  | player (P-23)           |          |              |
| US-6.1.5.4  | player (P-23)           |          |              |
| US-6.1.5.5  | engine developer (P-26) |          |              |
| US-6.1.5.6  | engine developer (P-26) |          |              |
| US-6.1.5.7  | engine tester (P-27)    |          |              |
| US-6.1.5.8  | engine tester (P-27)    |          |              |
| US-6.1.5.9  | designer (P-5)          |          |              |
| US-6.1.5.10 | player (P-23)           |          |              |
| US-6.1.5.11 | QA tester (P-19)        |          |              |
| US-6.1.5.12 | engine developer (P-26) |          |              |

1. **US-6.1.1.1** — capture per-key press, release, and repeat events with scancode and virtual
   keycode
   - **Acceptance:** keyboard input is reported accurately
2. **US-6.1.1.2** — WASD movement to work identically on any keyboard layout
   - **Acceptance:** physical key positions are consistent
3. **US-6.1.1.3** — scancodes for layout-independent physical key identification
   - **Acceptance:** movement bindings work regardless of locale
4. **US-6.1.1.4** — virtual keycodes reflecting user locale
   - **Acceptance:** chat and text-oriented features work correctly
5. **US-6.1.1.5** — platform scancode tables normalized to a common enumeration
   - **Acceptance:** input is cross-platform
6. **US-6.1.1.6** — verify key capture works on Windows (WM_KEY), macOS (IOHIDManager), and Linux
   (evdev)
   - **Acceptance:** all backends are validated
7. **US-6.1.1.7** — key bindings configurable in settings
   - **Acceptance:** players can customize their controls
8. **US-6.1.1.8** — test key repeat timing
   - **Acceptance:** repeat events fire at the correct rate
9. **US-6.1.1.9** — chat input to produce correct characters for my language
   - **Acceptance:** text communication works
10. **US-6.1.1.10** — test keyboard input on non-Latin layouts (AZERTY, QWERTZ, Cyrillic)
    - **Acceptance:** international support is validated
11. **US-6.1.1.11** — WM_KEYDOWN/UP on Windows, IOHIDManager on macOS, and evdev on Linux
    - **Acceptance:** platform-native input is used
12. **US-6.1.1.12** — verify scancode-to-keycode mapping on each platform
    - **Acceptance:** every key produces the correct code
13. **US-6.1.2.1** — mouse button events (left, right, middle, extended) reported
    - **Acceptance:** click input is captured
14. **US-6.1.2.2** — high-resolution cursor deltas for FPS camera control
    - **Acceptance:** mouse look is smooth
15. **US-6.1.2.3** — absolute cursor position for UI targeting
    - **Acceptance:** mouse interaction with UI works
16. **US-6.1.2.4** — precise mouse aiming for combat
    - **Acceptance:** targeting feels responsive and accurate
17. **US-6.1.2.5** — vertical and horizontal scroll events
    - **Acceptance:** zoom, inventory cycling, and scrolling work
18. **US-6.1.2.6** — macOS trackpad continuous scroll and discrete mouse scroll normalized to a
    unified axis
    - **Acceptance:** scroll behavior is consistent
19. **US-6.1.2.7** — high-DPI scaling accounted for in cursor position
    - **Acceptance:** targeting is pixel-accurate on Retina displays
20. **US-6.1.2.8** — verify WM_INPUT provides sub-pixel deltas on Windows
    - **Acceptance:** high-DPI mouse motion is captured
21. **US-6.1.2.9** — mouse scroll to cycle through inventory
    - **Acceptance:** item selection is quick
22. **US-6.1.2.10** — test extended mouse buttons (button 4, 5)
    - **Acceptance:** extra buttons are usable for bindings
23. **US-6.1.2.11** — mouse sensitivity configurable in settings
    - **Acceptance:** players tune their aim speed
24. **US-6.1.2.12** — verify mouse input on Windows, macOS, and Linux
    - **Acceptance:** all platforms work correctly
25. **US-6.1.3.1** — a unified gamepad abstraction over XInput, DualSense, and Switch Pro
    - **Acceptance:** all controllers work through one API
26. **US-6.1.3.2** — buttons, analog sticks, and triggers exposed per controller
    - **Acceptance:** all standard inputs are available
27. **US-6.1.3.3** — gyro aim for precision targeting
    - **Acceptance:** motion control enhances gamepad accuracy
28. **US-6.1.3.4** — gyroscope and accelerometer data exposed per controller
    - **Acceptance:** motion input is available
29. **US-6.1.3.5** — capability queries for touchpad, adaptive triggers, NFC, and gyro
    - **Acceptance:** features are gated by hardware
30. **US-6.1.3.6** — Madgwick filter deriving orientation from raw gyro and accelerometer
    - **Acceptance:** motion input is stable
31. **US-6.1.3.7** — verify gamepad abstraction on Windows (XInput), macOS (GCController), and Linux
    (evdev)
    - **Acceptance:** all backends work
32. **US-6.1.3.8** — test DualSense HID report parsing for motion data
    - **Acceptance:** PS5 controller motion works
33. **US-6.1.3.9** — any controller to work immediately when plugged in
    - **Acceptance:** I do not need to configure drivers
34. **US-6.1.3.10** — gamepad dead zones configurable in settings
    - **Acceptance:** stick drift is eliminated
35. **US-6.1.3.11** — test Switch Pro controller motion data
    - **Acceptance:** gyro input works on Switch
36. **US-6.1.3.12** — verify controller capability queries return correct results per controller
    type
    - **Acceptance:** feature gating is accurate
37. **US-6.1.4.1** — simultaneous touch contacts tracked with per- finger ID, position, pressure,
    and area
    - **Acceptance:** multi-touch works
38. **US-6.1.4.2** — pen position, pressure, tilt, and barrel button captured
    - **Acceptance:** pen input is available
39. **US-6.1.4.3** — virtual joystick and gesture controls on mobile
    - **Acceptance:** touch-based gameplay works
40. **US-6.1.4.4** — pressure normalized to 0.0-1.0 across all backends
    - **Acceptance:** pressure values are consistent
41. **US-6.1.4.5** — touch and gesture recognition running simultaneously
    - **Acceptance:** virtual joystick and camera gesture coexist
42. **US-6.1.4.6** — WM_POINTER on Windows, NSTouch on macOS, and libinput on Linux
    - **Acceptance:** touch uses native APIs
43. **US-6.1.4.7** — verify maximum simultaneous touch contacts per device (typically 5-10)
    - **Acceptance:** limits are documented
44. **US-6.1.4.8** — annotate maps with pen input
    - **Acceptance:** creative tools in player housing work
45. **US-6.1.4.9** — test pen tilt and barrel button events
    - **Acceptance:** full pen input is validated
46. **US-6.1.4.10** — touch sensitivity configurable per device type
    - **Acceptance:** touch input is tuned for different screens
47. **US-6.1.4.11** — test multi-touch on Windows, macOS, and mobile
    - **Acceptance:** touch works across platforms
48. **US-6.1.4.12** — verify pen input via Windows Ink (WM_POINTER)
    - **Acceptance:** pen support works on Windows
49. **US-6.1.5.1** — input device connection detected at runtime
    - **Acceptance:** new controllers are recognized immediately
50. **US-6.1.5.2** — input device disconnection detected
    - **Acceptance:** the system handles mid-game disconnects gracefully
51. **US-6.1.5.3** — switch between keyboard/mouse and gamepad without restarting
    - **Acceptance:** input transitions are seamless
52. **US-6.1.5.4** — the game to pause or show fallback behavior when my controller disconnects
    mid-combat
    - **Acceptance:** I am not penalized
53. **US-6.1.5.5** — WM_DEVICECHANGE on Windows, IOHIDManager on macOS, and udev on Linux for
    hot-plug
    - **Acceptance:** detection is native
54. **US-6.1.5.6** — device enumeration on a background thread
    - **Acceptance:** the game loop is not blocked
55. **US-6.1.5.7** — verify hot-plug detection on Windows, macOS, and Linux
    - **Acceptance:** all platforms handle device changes
56. **US-6.1.5.8** — test rapid connect/disconnect cycles
    - **Acceptance:** the system handles edge cases without crashes
57. **US-6.1.5.9** — pause-on-disconnect behavior configurable
    - **Acceptance:** games choose between pause, AI takeover, or warning overlay
58. **US-6.1.5.10** — a notification when a device connects or disconnects
    - **Acceptance:** I know my input state
59. **US-6.1.5.11** — verify connection/disconnection notifications appear correctly
    - **Acceptance:** player feedback is reliable
60. **US-6.1.5.12** — all connected devices enumerated on startup
    - **Acceptance:** initial device state is known
