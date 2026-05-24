# Input

How the engine reads player actions from all input devices.

## Topics

- [devices-and-actions](./devices-and-actions.md) — gamepads, keyboards, mice, and action
  mapping.
- [gestures-and-haptics](./gestures-and-haptics.md) — touch, swipes, and controller feedback.
- [specialty-input](./specialty-input.md) — VR controllers and motion-capture rigs.

## Key takeaways

- Input abstraction decouples raw devices (key codes, button IDs) from actions (jump, move),
  enabling rebinding without code changes.
- Analog stick dead zones and normalization prevent stick drift artifacts while preserving
  sensitivity at full deflection.
- Input buffering stores recent inputs for frame-perfect detection (combo windows, replay
  accuracy).
- Gesture recognizers (tap, swipe, pinch) layer on raw touch via state machines distinguishing
  gesture intent via distance/duration thresholds.
- Haptic intensity scales with gameplay impact: light tap for UI, heavy rumble for collision,
  creating tactile feedback hierarchy.

## Integration risks

- Input buffering window size (frames of history) must match prediction horizon; too short window
  misses buffered inputs, too long delays response feel. See [devices-and-actions.md](./devices-and-actions.md)
  for buffer tuning.
- Gesture recognition hysteresis (distance threshold to lock into drag mode) can feel sluggish or
  overly sensitive depending on UI scale. See [gestures-and-haptics.md](./gestures-and-haptics.md)
  for platform-specific tuning.
- Haptic patterns on different platforms (LRA vs ERM motors) feel different; platform-specific
  customization necessary. See [gestures-and-haptics.md](./gestures-and-haptics.md) for
  cross-platform guidance.
- Specialty input (VR, motion capture) requires calibration; poor calibration breaks hand tracking
  or gaze accuracy. See [specialty-input.md](./specialty-input.md) for calibration protocol.
- Raw input mode (bypassing OS queueing) introduces input lag if not synchronized to frame timing.
  See [devices-and-actions.md](./devices-and-actions.md) for sync strategy.
