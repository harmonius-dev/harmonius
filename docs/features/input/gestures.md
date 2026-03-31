# 6.3 — Gestures

## Discrete Gestures

| ID      | Feature                                    |
|---------|--------------------------------------------|
| F-6.3.1 | Tap, Multi-Tap, and Long Press Recognition |

1. **F-6.3.1** — Recognize single-tap, double-tap, triple-tap, and long press gestures with
   configurable time and distance thresholds. Long press initiates drag for item rearrangement and
   action bar customization. Haptic feedback fires at long-press recognition threshold on supported
   platforms (see F-6.4.1).
   - **Deps:** F-6.1.4
   - **Platform:** Tap distance tolerance should scale with display DPI. Mobile platforms require
     larger tolerance zones than desktop touchscreens.

## Continuous Gestures

| ID      | Feature                         |
|---------|---------------------------------|
| F-6.3.2 | Swipe Direction Recognition     |
| F-6.3.3 | Pinch, Rotate, and Pan Gestures |

1. **F-6.3.2** — Recognize linear swipe gestures in cardinal and diagonal directions with velocity
   and distance reporting. Velocity thresholds should be tunable per-platform to account for
   different touch surface sizes.
   - **Deps:** F-6.1.4
   - **Platform:** Swipe detection must filter out incidental movement during tap and long press
     recognition.
2. **F-6.3.3** — Track two-finger pinch (scale factor), rotation (angle delta), and
   single/multi-finger pan (position delta with velocity). On touch platforms, simultaneous pinch
   and rotate must be disambiguated when fingers move in ambiguous arcs. Pinch and pan may run
   simultaneously for camera zoom-and-orbit.
   - **Deps:** F-6.1.4
   - **Platform:** macOS trackpad gestures use `NSEvent` magnification and rotation types.

## Gesture Engine

| ID      | Feature                                         |
|---------|-------------------------------------------------|
| F-6.3.4 | Gesture State Machines with Conflict Resolution |
| F-6.3.5 | Custom Gesture Definition                       |

1. **F-6.3.4** — Implement gesture recognition as composable state machines with well-defined
   lifecycle states (possible, began, changed, ended, cancelled, failed). Resolve ambiguity when
   multiple recognizers match using configurable strategies: require-failure (tap waits for
   double-tap timeout), simultaneous recognition (pan and pinch together), and priority ordering.
   - **Deps:** F-6.3.1, F-6.3.2, F-6.3.3
   - **Platform:** Resolution strategies mirror UIKit's `UIGestureRecognizer` delegate model and
     Android's `GestureDetector` patterns but are implemented engine-side for cross-platform
     consistency.
2. **F-6.3.5** — Define custom gesture recognizers visually by composing touch event patterns in the
   gesture editor. Authors drag primitive recognizers (tap, swipe, hold, pinch) onto a timeline,
   connect them with sequencing and branching nodes, and set distance, angle, and timing thresholds
   via property panels. Complex patterns such as drawing rune shapes or circular swipes are authored
   entirely in the visual editor and evaluated at runtime through the logic graph system (F-15.8.4).
   - **Deps:** F-6.3.4, F-15.8.4 (Logic Graphs)
   - **Platform:** Primarily used on mobile and tablet. Desktop touchscreen and macOS trackpad
     support custom gestures but are secondary targets.
