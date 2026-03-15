# R-6.3 — Gesture User Stories

## F-6.3.1 Tap, Multi-Tap, and Long Press Recognition

## US-6.3.1.1 Configure Tap Thresholds in Editor

**As a** designer (P-5), **I want to** set inter-tap interval, long press duration, and distance
thresholds in the editor, **so that** gesture recognition timing is tuned per game.

## US-6.3.1.2 Configure DPI-Scaled Distance Tolerance

**As a** designer (P-5), **I want** distance tolerance to scale with display DPI automatically, **so
that** taps are recognized consistently across screen sizes.

## US-6.3.1.3 Verify Single-Tap Recognition

**As an** engine tester (P-27), **I want to** inject touch-down and touch-up within thresholds and
assert a single-tap event fires, **so that** tap recognition is correct.

## US-6.3.1.4 Verify Double-Tap Recognition

**As an** engine tester (P-27), **I want to** inject two taps within the double-tap interval and
assert double-tap fires without single-tap, **so that** multi-tap disambiguation works.

## US-6.3.1.5 Verify Long Press Recognition

**As an** engine tester (P-27), **I want to** hold a touch past the long press threshold and assert
a long press event fires, **so that** long press timing is accurate.

## US-6.3.1.6 Verify DPI Scaling

**As an** engine tester (P-27), **I want to** verify that increasing DPI proportionally increases
the distance threshold, **so that** DPI scaling is correct.

## US-6.3.1.7 Implement Tap and Long Press Recognizers

**As an** engine developer (P-26), **I want to** implement single-tap, double-tap, triple-tap, and
long press gesture recognizers with configurable thresholds and DPI scaling, **so that** discrete
gestures are available.

## US-6.3.1.8 Test Gesture Recognition on Touch Devices

**As a** QA tester (P-19), **I want to** test tap, double-tap, and long press on mobile and tablet
devices, **so that** gesture accuracy is verified across screen sizes.

## US-6.3.1.9 Tap to Select Targets

**As a** player (P-23), **I want to** tap to select targets and long press for context menus, **so
that** touch interactions feel natural.

---

## F-6.3.2 Swipe Direction Recognition

## US-6.3.2.1 Configure Swipe Thresholds in Editor

**As a** designer (P-5), **I want to** set swipe velocity and distance thresholds per platform in
the editor, **so that** swipe sensitivity matches the device.

## US-6.3.2.2 Verify Swipe Direction Recognition

**As an** engine tester (P-27), **I want to** inject a rightward swipe exceeding thresholds and
assert swipe-right fires with correct distance and velocity, **so that** directional recognition is
accurate.

## US-6.3.2.3 Verify Diagonal Swipe Recognition

**As an** engine tester (P-27), **I want to** inject a diagonal swipe and assert the correct
diagonal direction is reported, **so that** 8-directional swipes work.

## US-6.3.2.4 Verify Tap-Swipe Filtering

**As an** engine tester (P-27), **I want to** verify a tap followed by a short drag below threshold
does not trigger a swipe, **so that** incidental motion is filtered.

## US-6.3.2.5 Implement Swipe Recognizer

**As an** engine developer (P-26), **I want to** implement linear swipe recognition in cardinal and
diagonal directions with velocity, distance, and incidental motion filtering, **so that** swipe
gestures are available.

## US-6.3.2.6 Test Swipe Across Screen Sizes

**As a** QA tester (P-19), **I want to** test swipe recognition on phones and tablets of varying
sizes, **so that** swipe thresholds work across devices.

## US-6.3.2.7 Swipe to Dodge and Navigate

**As a** player (P-23), **I want to** swipe to dodge in combat and dismiss UI panels, **so that**
swipe controls are responsive.

---

## F-6.3.3 Pinch, Rotate, and Pan Gestures

## US-6.3.3.1 Configure Pinch Zoom Sensitivity

**As a** designer (P-5), **I want to** set pinch zoom sensitivity and range in the editor, **so
that** camera zoom is tuned per UI context.

## US-6.3.3.2 Verify Pinch Scale Factor

**As an** engine tester (P-27), **I want to** inject two fingers moving apart and assert a pinch
event with scale factor > 1.0, **so that** pinch zoom works.

## US-6.3.3.3 Verify Rotation Angle Delta

**As an** engine tester (P-27), **I want to** inject two rotating fingers and assert a rotation
event with the correct angle delta, **so that** rotation tracking is accurate.

## US-6.3.3.4 Verify Simultaneous Pinch and Pan

**As an** engine tester (P-27), **I want to** inject two fingers moving apart while translating and
assert both pinch and pan events fire with correct values, **so that** simultaneous recognition
works.

## US-6.3.3.5 Implement Pinch, Rotate, and Pan Recognizers

**As an** engine developer (P-26), **I want to** implement two-finger pinch, rotation, and pan
tracking with simultaneous recognition support, **so that** multi-touch gestures are available.

## US-6.3.3.6 Test Multi-Touch on Various Devices

**As a** QA tester (P-19), **I want to** test pinch, rotate, and pan on phones, tablets, and macOS
trackpads, **so that** multi-touch works across input surfaces.

## US-6.3.3.7 Pinch to Zoom the Camera

**As a** player (P-23), **I want to** pinch to zoom the camera and rotate to orbit, **so that**
mobile camera control is intuitive.

---

## F-6.3.4 Gesture State Machines with Conflict Resolution

## US-6.3.4.1 Configure Conflict Resolution Strategy

**As a** designer (P-5), **I want to** set conflict resolution strategies (require-failure,
simultaneous, priority) between gesture recognizers in the editor, **so that** gesture ambiguity is
resolved per use case.

## US-6.3.4.2 Verify Require-Failure Strategy

**As an** engine tester (P-27), **I want to** configure tap to require-failure of double-tap, inject
a single tap, and assert tap fires only after double-tap timeout, **so that** require-failure works.

## US-6.3.4.3 Verify Simultaneous Recognition Strategy

**As an** engine tester (P-27), **I want to** configure pan and pinch for simultaneous recognition,
inject two-finger movement, and assert both fire, **so that** simultaneous recognition works.

## US-6.3.4.4 Verify Lifecycle State Transitions

**As an** engine tester (P-27), **I want to** verify all lifecycle states (possible, began, changed,
ended, cancelled, failed) transition in the defined order, **so that** state machine correctness is
confirmed.

## US-6.3.4.5 Implement Gesture State Machine Engine

**As an** engine developer (P-26), **I want to** implement composable gesture state machines with
lifecycle states and require-failure, simultaneous, and priority resolution, **so that** gesture
conflicts are resolved correctly.

## US-6.3.4.6 Test Gesture Conflict Edge Cases

**As a** QA tester (P-19), **I want to** test ambiguous gesture combinations (tap vs double- tap,
pan vs swipe) on touch devices, **so that** conflicts resolve as configured.

## US-6.3.4.7 Use Touch Without Accidental Gesture Triggers

**As a** player (P-23), **I want** taps, swipes, and multi-touch gestures to not conflict with each
other, **so that** touch input is reliable.

---

## F-6.3.5 Custom Gesture Definition

## US-6.3.5.1 Author Custom Gestures in Visual Editor

**As a** designer (P-5), **I want to** compose custom gesture recognizers by dragging primitive
recognizers onto a timeline and connecting them with sequencing and branching nodes, **so that**
complex gestures (rune shapes, circular swipes) are authored visually.

## US-6.3.5.2 Configure Gesture Thresholds Visually

**As a** designer (P-5), **I want to** set distance, angle, and timing thresholds via property
panels in the gesture editor, **so that** gesture sensitivity is tuned without code.

## US-6.3.5.3 Verify Custom Circle Swipe Gesture

**As an** engine tester (P-27), **I want to** author a circle swipe gesture, inject circular motion,
and assert it fires, then inject a linear swipe and assert it does not, **so that** custom gesture
evaluation is correct.

## US-6.3.5.4 Verify Gesture Data Asset Loading

**As an** engine tester (P-27), **I want to** verify custom gesture definitions load as data assets
at runtime without recompilation, **so that** gestures are hot-reloadable.

## US-6.3.5.5 Implement Custom Gesture Evaluation

**As an** engine developer (P-26), **I want to** implement runtime evaluation of custom gesture
definitions authored in the visual editor, evaluated through the logic graph system, **so that**
arbitrary gesture patterns are supported.

## US-6.3.5.6 Test Custom Gestures on Touch Devices

**As a** QA tester (P-19), **I want to** test custom gestures (rune drawing, circular swipes) on
mobile and tablet, **so that** custom recognition works on target devices.

## US-6.3.5.7 Draw Rune Shapes to Cast Spells

**As a** player (P-23), **I want to** draw rune shapes on the screen to cast spells, **so that**
spellcasting feels interactive and unique.

---

## Non-Functional Requirements

### R-6.3.NF1 Gesture Recognition Latency

The engine **SHALL** recognize discrete gestures (tap, double-tap, long press) within 1 frame of the
gesture completion event, and continuous gestures (swipe, pinch, pan) within 2 frames.

- **Derived from:** F-6.3.1, F-6.3.4
- **Rationale:** Gesture recognition delay directly impacts perceived touch responsiveness.
- **Verification:** Unit test: inject a tap and measure frames from touch-up to event. Assert
  recognition within 1 frame. Inject a swipe and measure frames from threshold crossing. Assert
  recognition within 2 frames.
