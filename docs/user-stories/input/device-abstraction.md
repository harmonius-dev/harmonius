# User Stories -- 6.1 Device Abstraction

## US-6.1.1 Use Any Keyboard Layout
**As a** player, **I want** the engine to capture keyboard events with both physical
scancodes and locale-aware virtual keycodes, **so that** my WASD movement works identically
on QWERTY and AZERTY layouts while chat and text input respect my keyboard locale.

## US-6.1.2 Aim Precisely with Mouse
**As a** player, **I want** high-resolution mouse deltas, absolute position, and scroll
values normalized for high-DPI scaling, **so that** camera control and UI targeting feel
consistent regardless of my display resolution or DPI settings.

## US-6.1.3 Use Any Major Gamepad
**As a** player, **I want** a unified gamepad abstraction that works with Xbox, DualSense,
and Switch Pro controllers including gyroscope and accelerometer data, **so that** I can
use my preferred controller with full feature support and gyro-aim.

## US-6.1.4 Play on Touchscreen Devices
**As a** player, **I want** multi-touch and pen input with per-finger tracking, pressure,
and area reporting, **so that** virtual joysticks, gesture-based camera control, and
creative tools work simultaneously on touchscreen devices.

## US-6.1.5 Switch Controllers Without Restarting
**As a** player, **I want** the engine to detect controller connection and disconnection
instantly at runtime, **so that** I can switch between keyboard/mouse and gamepad mid-game
without restarting and see appropriate pause behavior on unexpected disconnects.
