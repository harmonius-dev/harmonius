# R-6.3 — Gesture Requirements

## Discrete Gestures

| ID       | Derived From |
|----------|--------------|
| R-6.3.1  |              |
| R-6.3.1a |              |
| R-6.3.1b |              |

1. **R-6.3.1** — The engine **SHALL** recognize single-tap, double-tap, and triple-tap gestures
   based on configurable inter-tap interval and distance thresholds. Multi-tap recognition **SHALL**
   suppress lower-tap events (single-tap does not fire if double-tap succeeds).
   [F-6.3.1](../../features/input/gestures.md) disambiguation causes accidental activations. fires
   and single-tap does not. Inject one tap and wait past the interval. Assert single-tap fires.
   - **Rationale:** Tap drives target selection; multi-tap drives quick actions; incorrect
   - **Verification:** Unit test: inject two taps within the double-tap interval. Assert double-tap
2. **R-6.3.1a** — The engine **SHALL** recognize long press gestures when a touch contact is held
   past a configurable duration threshold without exceeding a configurable distance tolerance.
   [F-6.3.1](../../features/input/gestures.md) thresholds prevent false positives from slow taps or
   finger drift. fires. Hold for 400 ms. Assert long press does not fire. Hold 600 ms but move
   beyond distance tolerance. Assert long press does not fire.
   - **Rationale:** Long press drives context menus and drag initiation; duration and distance
   - **Verification:** Unit test: hold a touch for 600 ms with a 500 ms threshold. Assert long press
3. **R-6.3.1b** — The engine **SHALL** scale gesture distance thresholds proportionally with display
   DPI, so tap and swipe recognition is consistent across screen sizes and resolutions.
   [F-6.3.1](../../features/input/gestures.md) finger movement covers more pixels. Inject a tap with
   finger drift at 1x threshold on 2x display. Assert tap is still recognized.
   - **Rationale:** A fixed pixel threshold fails on high-DPI mobile screens where the same physical
   - **Verification:** Unit test: at 2x DPI, verify the distance threshold is 2x the base pixel
     value.

## Continuous Gestures

| ID       | Derived From |
|----------|--------------|
| R-6.3.2  |              |
| R-6.3.3  |              |
| R-6.3.3a |              |

1. **R-6.3.2** — The engine **SHALL** recognize linear swipe gestures in cardinal (up, down, left,
   right) and diagonal (4 diagonals) directions, reporting swipe distance and velocity. Swipe
   detection **SHALL** filter out incidental motion below configurable velocity and distance
   thresholds. [F-6.3.2](../../features/input/gestures.md) prevents false swipe triggers during tap
   interactions. with correct distance and velocity. Inject motion below threshold. Assert no swipe
   fires. Inject diagonal swipe. Assert correct diagonal direction is reported.
   - **Rationale:** Swipe drives dodge, UI dismissal, and directional abilities; incidental
     filtering
   - **Verification:** Unit test: inject a rightward swipe above thresholds. Assert swipe-right
     fires
2. **R-6.3.3** — The engine **SHALL** track two-finger pinch (scale factor), rotation (angle delta
   in degrees), and pan (position delta with velocity) as continuous gesture events. Pinch and pan
   **SHALL** be recognizable simultaneously when both fingers move apart while translating.
   [F-6.3.3](../../features/input/gestures.md) force choose-one behavior. (scale > 1.0) and pan
   events fire. Inject two rotating fingers. Assert rotation event with correct angle delta.
   - **Rationale:** Camera zoom + orbit requires simultaneous pinch and pan; separate recognition
     would
   - **Verification:** Unit test: inject two fingers moving apart and translating. Assert both pinch
3. **R-6.3.3a** — The engine **SHALL** report pinch scale factors with at least 1% accuracy relative
   to the physical finger distance ratio. [F-6.3.3](../../features/input/gestures.md) within 1%
   tolerance.
   - **Rationale:** Inaccurate scale factors produce jerky or unpredictable zoom behavior.
   - **Verification:** Unit test: inject fingers at distance 100 then 200. Assert scale factor is
     2.0

## Gesture Engine

| ID       | Derived From |
|----------|--------------|
| R-6.3.4  |              |
| R-6.3.4a |              |
| R-6.3.4b |              |
| R-6.3.5  |              |

1. **R-6.3.4** — The engine **SHALL** implement gesture recognition as state machines with lifecycle
   states: possible, began, changed, ended, cancelled, and failed. State transitions **SHALL**
   follow a defined order (possible -> began -> changed -> ended) and never skip states.
   [F-6.3.4](../../features/input/gestures.md) (e.g., highlight on began, confirm on ended). the
   defined order. Assert no state is skipped.
   - **Rationale:** Well-defined lifecycle states enable reliable gesture tracking and UI feedback
   - **Verification:** Unit test: track a swipe gesture through all states. Assert transitions
     follow
2. **R-6.3.4a** — The engine **SHALL** support conflict resolution strategies between competing
   gesture recognizers: require-failure (tap waits for double-tap timeout), simultaneous recognition
   (pan and pinch together), and priority ordering (higher priority wins).
   [F-6.3.4](../../features/input/gestures.md) produces unpredictable behavior. tap. Assert tap
   fires only after double-tap timeout. (2) Configure pan and pinch as simultaneous. Inject
   two-finger movement. Assert both fire.
   - **Rationale:** Without conflict resolution, ambiguous input (is it a tap or the start of a
     swipe?)
   - **Verification:** Unit test: (1) configure tap to require-failure of double-tap. Inject a
     single
3. **R-6.3.4b** — The engine **SHALL** allow designers to configure conflict resolution strategies
   between gesture recognizers in the visual editor. [F-6.3.4](../../features/input/gestures.md)
   editor. Test at runtime and assert correct disambiguation behavior.
   - **Rationale:** No-code engine constraint; gesture conflict tuning must not require code.
   - **Verification:** Integration test: configure require-failure between tap and double-tap in the
4. **R-6.3.5** — The engine **SHALL** support custom gesture recognizers authored in the visual
   editor by composing primitive recognizers (tap, swipe, hold, pinch) on a timeline with
   sequencing, branching, and configurable distance, angle, and timing thresholds. Custom gestures
   **SHALL** be loadable as data assets at runtime without recompilation.
   [F-6.3.5](../../features/input/gestures.md) anticipated by built-in recognizers; no-code
   constraint requires visual authoring. motion at runtime. Assert the custom gesture fires. Inject
   a linear swipe. Assert it does not fire. Verify the gesture loads as a data asset without engine
   recompilation.
   - **Rationale:** Games require unique gestures (rune shapes, circular swipes) that cannot be
   - **Verification:** Integration test: author a circle swipe gesture in the editor. Inject
     circular

---
