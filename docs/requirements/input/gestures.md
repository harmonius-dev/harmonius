# R-6.3 — Gesture Requirements

## Discrete Gestures

### R-6.3.1 Tap, Multi-Tap, and Long Press Recognition

The engine **SHALL** recognize single-tap, double-tap, triple-tap, and long press gestures with
configurable time thresholds (inter-tap interval, long press duration) and distance thresholds
(scaled by display DPI), emitting each recognized gesture as a distinct ECS event.

- **Derived from:** [F-6.3.1](../../features/input/gestures.md)
- **Rationale:** Tap drives target selection, double-tap drives quick actions, and long press
  initiates context menus and drag operations; DPI-scaled thresholds ensure consistency across
  devices.
- **Verification:** Unit test: inject a touch-down and touch-up within the tap time threshold and
  distance threshold; assert a single-tap event fires. Inject two taps within the double-tap
  interval; assert a double-tap event fires and no single-tap event fires. Hold a touch for longer
  than the long press threshold; assert a long press event fires. Verify that increasing DPI
  proportionally increases the distance threshold.

## Continuous Gestures

### R-6.3.2 Swipe Direction Recognition

The engine **SHALL** recognize linear swipe gestures in cardinal and diagonal directions, reporting
swipe direction, distance, and velocity, while filtering out incidental motion during tap and long
press recognition.

- **Derived from:** [F-6.3.2](../../features/input/gestures.md)
- **Rationale:** Directional swipes drive dodge input, quick-slot ability casting, and UI panel
  dismissal on touch platforms.
- **Verification:** Unit test: inject a rightward swipe exceeding the velocity and distance
  thresholds; assert a swipe-right event fires with correct distance and velocity values. Inject
  a slow, short movement below thresholds; assert no swipe event fires. Inject a diagonal swipe;
  assert the correct diagonal direction is reported. Verify a tap followed by a short drag below
  the swipe threshold does not trigger a swipe.

### R-6.3.3 Pinch, Rotate, and Pan Gestures

The engine **SHALL** track two-finger pinch (reporting scale factor), rotation (reporting angle
delta in degrees), and single/multi-finger pan (reporting position delta and velocity), supporting
simultaneous pinch and pan recognition.

- **Derived from:** [F-6.3.3](../../features/input/gestures.md)
- **Rationale:** Pinch controls camera zoom, rotation enables free camera orbit, and pan drives
  camera orbiting and map scrolling; simultaneous recognition prevents gesture conflicts.
- **Verification:** Unit test: inject two fingers moving apart symmetrically; assert a pinch event
  fires with a scale factor greater than 1.0. Inject two fingers rotating; assert a rotation event
  fires with the correct angle delta. Inject two fingers moving apart while also translating;
  assert both pinch and pan events fire simultaneously with correct values.

## Gesture Engine

### R-6.3.4 Gesture State Machines with Conflict Resolution

The engine **SHALL** implement gesture recognition as composable state machines with lifecycle
states (possible, began, changed, ended, cancelled, failed) and resolve ambiguity between
simultaneous recognizers using configurable strategies: require-failure, simultaneous recognition,
and priority ordering.

- **Derived from:** [F-6.3.4](../../features/input/gestures.md)
- **Rationale:** Without conflict resolution, overlapping recognizers (tap vs. double-tap, pan vs.
  swipe) swallow inputs unpredictably, causing frustrating touch interactions.
- **Verification:** Unit test: configure tap to require-failure of double-tap. Inject a single tap;
  assert the tap event fires only after the double-tap timeout expires. Inject a double-tap; assert
  only the double-tap event fires and the single-tap recognizer transitions to failed. Configure
  pan and pinch for simultaneous recognition; inject two-finger movement and assert both fire.
  Verify all lifecycle state transitions follow the defined state machine ordering.

### R-6.3.5 Custom Gesture Definition via Visual Editor

The engine **SHALL** allow authors to define custom gesture recognizers by composing primitive
recognizers (tap, swipe, hold, pinch) in the visual gesture editor, connecting them with sequencing
and branching nodes, and evaluating them at runtime through the logic graph system without
requiring any code.

- **Derived from:** [F-6.3.5](../../features/input/gestures.md)
- **Rationale:** Complex gesture patterns (rune shapes, circular swipes for area targeting) must be
  authorable visually to support the no-code engine constraint.
- **Verification:** Integration test: author a "circle swipe" gesture in the visual editor by
  connecting swipe nodes in a circular sequence with angle and timing thresholds. Inject a circular
  touch motion; assert the custom gesture fires. Inject a linear swipe; assert the custom gesture
  does not fire. Verify the gesture definition is a data asset loadable at runtime without
  recompilation.

---

## Non-Functional Requirements

### R-6.3.NF1 Gesture Recognition Latency

The engine **SHALL** recognize discrete gestures (tap, double-tap, long press) within 1 frame
of the gesture completion event, and continuous gestures (swipe, pinch, pan) within 2 frames
of the initial touch movement exceeding the recognition threshold.

- **Derived from:** [F-6.3.1](../../features/input/gestures.md),
  [F-6.3.4](../../features/input/gestures.md)
- **Rationale:** Gesture recognition delay directly impacts perceived touch responsiveness.
  Delayed tap recognition causes frustrating input lag on mobile and touchscreen platforms.
- **Verification:** Unit test: inject a tap gesture and measure frames from touch-up to tap
  event emission. Assert recognition occurs within 1 frame. Inject a swipe exceeding the
  velocity threshold and measure frames from threshold crossing to swipe event. Assert
  recognition occurs within 2 frames.
