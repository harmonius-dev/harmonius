# R-6.1 — Device Abstraction Requirements

## Keyboard and Mouse

### R-6.1.1 Keyboard Input Capture

The engine **SHALL** capture per-key press, release, and repeat events exposing both a
platform-normalized scancode (physical key) and a locale-aware virtual keycode, delivered as ECS
events within the same frame the OS reports them.

- **Derived from:** [F-6.1.1](../../features/input/device-abstraction.md)
- **Rationale:** Scancodes provide layout-independent physical key identity for movement bindings,
  while virtual keycodes reflect the user's locale for text input features.
- **Verification:** Unit test: simulate press, release, and repeat for every key on a standard
  104-key layout. Assert each event carries a valid scancode and virtual keycode. Integration test
  on Windows, macOS, and Linux: press the physical "W" key on QWERTY and AZERTY layouts and verify
  the scancode is identical while the virtual keycode differs.

### R-6.1.2 Mouse Button, Motion, and Scroll Input

The engine **SHALL** report mouse button events (left, right, middle, extended), sub-pixel cursor
deltas, absolute cursor position, and vertical/horizontal scroll values, all normalized for
high-DPI scaling, as ECS events each frame.

- **Derived from:** [F-6.1.2](../../features/input/device-abstraction.md)
- **Rationale:** Delta values drive camera control, absolute position drives UI targeting, and scroll
  drives zoom and inventory cycling; all require high-DPI normalization to behave consistently.
- **Verification:** Unit test: inject synthetic mouse events for each button, delta motion, absolute
  position, and scroll axis; assert all values appear as correctly typed ECS events. Integration
  test: on a high-DPI display, verify delta and position values are scale-normalized by comparing
  against known OS-reported values.

## Gamepad

### R-6.1.3 Unified Gamepad Abstraction

The engine **SHALL** expose a unified gamepad abstraction over XInput, DualSense, and Switch Pro
controllers providing buttons, analog sticks (as 2D axes), triggers (as 1D axes), and motion
sensors (gyroscope, accelerometer) as ECS components, with controller-specific features queryable
through a capability API.

- **Derived from:** [F-6.1.3](../../features/input/device-abstraction.md)
- **Rationale:** A unified abstraction lets gameplay logic reference "primary trigger" without
  branching on controller type, while capability queries expose DualSense-specific features.
- **Verification:** Integration test: connect each supported controller type (Xbox, DualSense,
  Switch Pro). Verify that buttons, sticks, and triggers produce identical ECS component values
  for equivalent physical inputs. Verify capability queries correctly report gyroscope support on
  DualSense and Switch Pro and correctly report absence on Xbox controllers lacking motion sensors.

## Touch and Pen

### R-6.1.4 Multi-Touch and Pen Input

The engine **SHALL** track up to 10 simultaneous touch contacts with per-finger identifier,
position, pressure (normalized 0.0-1.0), and contact area, and **SHALL** capture pen position,
pressure (normalized 0.0-1.0), tilt, and barrel button events, all as ECS events.

- **Derived from:** [F-6.1.4](../../features/input/device-abstraction.md)
- **Rationale:** Concurrent multi-touch enables virtual joysticks and gesture camera control
  simultaneously; pen input supports map annotation and creative tools.
- **Verification:** Unit test: inject 10 simultaneous touch contacts with distinct identifiers and
  pressure values; assert all 10 are tracked with correct data. Verify pressure normalization
  clamps to [0.0, 1.0]. Integration test: on a pen-enabled device, verify position, pressure,
  tilt, and barrel button events are captured with correct values.

## Device Lifecycle

### R-6.1.5 Device Hot-Plug Detection and Enumeration

The engine **SHALL** detect input device connection and disconnection at runtime and emit an ECS
event within one frame of the OS notification, without blocking the game loop.

- **Derived from:** [F-6.1.5](../../features/input/device-abstraction.md)
- **Rationale:** Players must seamlessly switch between keyboard/mouse and gamepad mid-session, and
  the engine must handle mid-gameplay controller disconnects gracefully.
- **Verification:** Integration test: start a session with no gamepad connected, connect a gamepad,
  and assert a connection event is emitted within one frame. Disconnect the gamepad and assert a
  disconnection event is emitted within one frame. Verify the game loop frame time does not spike
  above 1ms during either event.

---

## Non-Functional Requirements

### R-6.1.NF1 Input Polling Latency

The engine **SHALL** poll all connected input devices and deliver raw events to the action
system within 1 ms of the OS reporting the event, measured from OS event timestamp to
action system delivery timestamp.

- **Derived from:** [F-6.1.1](../../features/input/device-abstraction.md),
  [F-6.1.2](../../features/input/device-abstraction.md),
  [F-6.1.3](../../features/input/device-abstraction.md),
  [R-X.2.1](../cross-cutting.md) (Thread Ownership — main thread input processing)
- **Rationale:** Input latency directly degrades responsiveness. Keeping the OS-to-action
  pipeline under 1 ms ensures the input system contributes negligible latency relative to
  frame time.
- **Verification:** Integration test: inject a timestamped OS input event and measure the
  delta to action system delivery. Assert p99 latency is below 1 ms over 10,000 events
  across keyboard, mouse, and gamepad devices.

### R-6.1.NF2 Device Enumeration Throughput

The engine **SHALL** enumerate all connected input devices within 5 ms during startup and
within 2 ms during hot-plug events, without blocking the game loop.

- **Derived from:** [F-6.1.5](../../features/input/device-abstraction.md),
  [R-X.2.1](../cross-cutting.md) (Thread Ownership)
- **Rationale:** Slow enumeration during hot-plug can cause frame hitches if run on the main
  thread. Background enumeration with tight time bounds prevents stalls.
- **Verification:** Integration test: connect 4 input devices simultaneously. Assert all
  devices are enumerated and connection events emitted within 5 ms. Assert game loop frame
  time does not spike above 1 ms during enumeration.
