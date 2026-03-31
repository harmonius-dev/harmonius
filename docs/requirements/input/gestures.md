# R-6.3 — Gesture Requirements

## Discrete Gestures

1. **R-6.3.1** — The engine **SHALL** recognize single-tap, double-tap, and triple-tap gestures
   based on configurable inter-tap interval and distance thresholds. Multi-tap recognition **SHALL**
   suppress lower-tap events (single-tap does not fire if double-tap succeeds).
   - **Rationale:** Tap drives target selection; multi-tap drives quick actions; incorrect
     disambiguation causes accidental activations.
   - **Verification:** Inject two taps within the double-tap interval. Assert double-tap fires and
     single-tap does not. Inject one tap and wait. Assert single-tap fires.

2. **R-6.3.2** — The engine **SHALL** recognize long press gestures when a touch contact is held
   past a configurable duration threshold without exceeding a configurable distance tolerance.
   - **Rationale:** Long press drives context menus and drag initiation; thresholds prevent false
     positives.
   - **Verification:** Hold touch for 600 ms with 500 ms threshold. Assert long press fires. Hold
     400 ms. Assert it does not. Hold 600 ms beyond distance. Assert it does not.

3. **R-6.3.3** — The engine **SHALL** scale gesture distance thresholds proportionally with display
   DPI.
   - **Rationale:** Fixed pixel thresholds fail on high-DPI mobile screens where the same physical
     finger movement covers more pixels.
   - **Verification:** At 2x DPI, verify threshold is 2x base pixel value. Inject tap drift at 1x
     threshold on 2x display. Assert tap recognized.

## Continuous Gestures

4. **R-6.3.4** — The engine **SHALL** recognize linear swipe gestures in 8 directions (cardinal +
   diagonal), reporting distance and velocity. Swipe detection **SHALL** filter out incidental
   motion below configurable thresholds.
   - **Rationale:** Swipe drives dodge, UI dismissal, and directional abilities; filtering prevents
     false triggers during tap interactions.
   - **Verification:** Inject rightward swipe above threshold. Assert swipe-right fires. Inject
     motion below threshold. Assert no swipe fires.

5. **R-6.3.5** — The engine **SHALL** track two-finger pinch (scale factor), rotation (angle delta),
   and pan (position delta with velocity) as continuous gesture events. Pinch and pan **SHALL** be
   recognizable simultaneously.
   - **Rationale:** Camera zoom + orbit requires simultaneous pinch and pan.
   - **Verification:** Inject two fingers moving apart and translating. Assert both pinch and pan
     fire. Inject rotating fingers. Assert rotation with correct angle.

6. **R-6.3.6** — The engine **SHALL** report pinch scale factors with at least 1% accuracy relative
   to the physical finger distance ratio.
   - **Rationale:** Inaccurate scale factors produce jerky zoom behavior.
   - **Verification:** Inject fingers at distance 100 then
     200. Assert scale factor is 2.0 within 1%.

## Gesture Engine

7. **R-6.3.7** — The engine **SHALL** implement gesture recognition as state machines with lifecycle
   states: possible, began, changed, ended, cancelled, and failed. Transitions **SHALL** never skip
   states.
   - **Rationale:** Well-defined lifecycle enables reliable gesture tracking and UI feedback.
   - **Verification:** Track a swipe through all states. Assert transitions follow defined order.

8. **R-6.3.8** — The engine **SHALL** support conflict resolution strategies: require-failure,
   simultaneous recognition, and priority ordering.
   - **Rationale:** Without conflict resolution, ambiguous input produces unpredictable behavior.
   - **Verification:** Configure tap to require-failure of double-tap. Inject single tap. Assert
     fires after timeout. Configure pan+pinch simultaneous. Assert both fire.

9. **R-6.3.9** — The engine **SHALL** allow designers to configure conflict resolution strategies in
   the visual editor.
   - **Rationale:** No-code engine constraint; gesture tuning must not require code.
   - **Verification:** Configure require-failure in editor. Test at runtime. Assert correct
     disambiguation.

10. **R-6.3.10** — The engine **SHALL** support custom gesture recognizers authored in the visual
    editor by composing primitives with sequencing, branching, and configurable thresholds. Custom
    gestures **SHALL** be loadable as data assets without recompilation.
    - **Rationale:** Games require unique gestures that built-in recognizers do not cover.
    - **Verification:** Author a circle swipe in the editor. Inject circular motion. Assert fires.
      Inject linear swipe. Assert it does not. Verify loading as data asset without recompilation.
