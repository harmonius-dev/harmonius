# 6.1 — Device Abstraction

## Keyboard and Mouse

| ID      | Feature                                |
|---------|----------------------------------------|
| F-6.1.1 | Keyboard Input Capture                 |
| F-6.1.2 | Mouse Button, Motion, and Scroll Input |

1. **F-6.1.1** — Capture per-key press, release, and repeat events with full scancode and virtual
   keycode support. Scancodes provide layout-independent physical key identification critical for
   WASD movement bindings, while virtual keycodes reflect the user's locale for text-oriented
   features like chat.
   - **Platform:** Windows uses `WM_KEYDOWN`/`WM_KEYUP` with `MapVirtualKey`; macOS uses
     `IOHIDManager` or `CGEvent` taps; Linux uses `evdev` or `libinput`. Scancode tables differ
     across platforms and must be normalized to a common enumeration.
2. **F-6.1.2** — Report mouse button events (left, right, middle, extended), high-resolution cursor
   deltas, absolute position, and vertical/horizontal scroll. Delta values drive FPS-style camera
   control, absolute position drives targeting and UI interaction, and scroll drives zoom, inventory
   cycling, and action bar navigation. High-DPI scaling must be accounted for. macOS trackpads
   report continuous scroll with momentum while discrete mice report notched scroll — normalize both
   into a unified axis value.
   - **Platform:** Windows provides raw input via `WM_INPUT` for sub-pixel deltas; macOS uses
     `CGEvent` with `deltaX`/`deltaY`; Linux uses `evdev` relative axes.

## Gamepad

| ID      | Feature                     |
|---------|-----------------------------|
| F-6.1.3 | Unified Gamepad Abstraction |

1. **F-6.1.3** — Provide a unified gamepad abstraction over XInput (Xbox), DualSense (PlayStation
   5), and Switch Pro controllers exposing buttons, analog sticks, triggers, and motion sensors
   (gyroscope, accelerometer). Gyro-aim enables precision targeting. Controller-specific features
   (DualSense touchpad, adaptive triggers, Switch NFC) are accessible through capability queries.
   Sensor fusion (Madgwick filter) derives orientation from raw gyro and accelerometer readings.
   - **Platform:** Windows uses XInput and Windows.Gaming.Input; macOS uses `GCController`; Linux
     uses `evdev`. DualSense and Switch Pro motion data require platform-specific HID report
     parsing.

## Touch and Pen

| ID      | Feature                   |
|---------|---------------------------|
| F-6.1.4 | Multi-Touch and Pen Input |

1. **F-6.1.4** — Track simultaneous touch contacts with per-finger identifiers, position, pressure,
   and area. Additionally capture pen position, pressure, tilt, and barrel button events. Touch
   enables mobile clients to support virtual joysticks and gesture-based camera control
   concurrently, while pen input supports map annotation and creative tools. Pressure normalization
   to 0.0-1.0 is required across all backends. Maximum simultaneous touch contacts vary by hardware
   (typically 5-10 points).
   - **Platform:** Windows uses `WM_POINTER` for both touch and pen (Windows Ink); macOS uses
     `NSTouch`/`NSEvent` tablet events; Linux uses `libinput` multitouch slots and tablet tool
     events.

## Device Lifecycle

| ID      | Feature                                   |
|---------|-------------------------------------------|
| F-6.1.5 | Device Hot-Plug Detection and Enumeration |

1. **F-6.1.5** — Detect connection and disconnection of input devices at runtime and notify the
   input system immediately. Players must seamlessly switch between keyboard/mouse and gamepad
   without restarting, and the engine must gracefully handle mid-session controller disconnects with
   appropriate pause or fallback behavior. Enumeration must run on a background thread to avoid
   blocking the game loop.
   - **Platform:** Windows uses `WM_DEVICECHANGE` and `RegisterDeviceNotification`; macOS uses
     `IOHIDManager` matching/removal callbacks; Linux uses `udev` monitor or `inotify` on
     `/dev/input/`.
