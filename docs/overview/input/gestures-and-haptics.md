# Gestures and Haptics

Multi-touch gesture recognition and haptic feedback response.

## What it covers

- Gesture recognition: tap, long-press, swipe, pinch-zoom, two-finger rotate.
- Drag detection: distinguishing drag from tap based on distance and duration.
- Pinch-to-zoom: simultaneous two-finger motion scaling.
- Two-finger rotation: computing rotation angle from two-touch movement.
- Haptic feedback: vibration motors, haptic patterns.
- Haptic intensity: scaling vibration strength with impact magnitude.
- Per-platform haptic options: linear resonant actuators (LRA), eccentric rotating mass (ERM),
  immersion haptic libraries.
- Haptic sequencing: chaining multiple vibration patterns.
- Pressure-sensitive input: detecting touch force on capable devices.
- Palm rejection: ignoring palm touches while stylus or multi-touch is active.

## Concepts

### Gesture Recognizers

Gesture recognizers interpret raw touch data into high-level gestures. A tap is a single touch
down-up within a short time and distance threshold. A long-press is sustained contact for a duration.
A swipe is a rapid directional drag. Pinch-zoom measures distance between two fingers; closing
(distance decreasing) is zoom-out, opening is zoom-in. Two-finger rotate computes the angle
between fingers' initial and current positions.

### Drag and Distinguishing Gestures

Drags are ambiguous: they could be the start of a swipe or accidental motion during a tap. The
recognizer uses hysteresis: if motion exceeds a distance threshold (e.g., 10 pixels), lock into
drag mode. If not, and time is within tap threshold, recognize as tap. This prevents accidental
drags from becoming taps.

### Haptic Feedback

Haptic feedback provides tactile response: collision impacts vibrate, UI selection emits a subtle
pulse. Platform-specific haptic engines (Apple Haptics on iOS, Android Vibrator API) define haptic
capabilities. Patterns chain multiple vibration profiles: a "burst" pulses rapidly; a "thud"
vibrates low-frequency. Intensity scales vibration strength from 0 (off) to 1 (maximum).

### Pressure and Advanced Touch

Pressure-sensitive inputs (stylus, newer phones) report touch force in addition to position and
area. Brush sizes scale with pressure; pen input distinguishes light sketches from firm strokes.
Palm rejection prevents accidental touches from the palm or wrist when stylus is in use, ignoring
those touches entirely.

## How it fits

- See [devices-and-actions.md](./devices-and-actions.md) for raw input and action mapping.
- See [../ui/widgets-and-framework.md](../ui/widgets-and-framework.md) for UI gesture
  handling.
- Integrates with [../game-framework/gameplay-features.md](../game-framework/gameplay-features.md)
  for game-specific gesture patterns.
