# 6.3 — Gestures

## Discrete Gestures

### F-6.3.1 Tap, Multi-Tap, and Long Press Recognition

Recognize single-tap, double-tap, triple-tap, and long press gestures with configurable time and
distance thresholds. Tap gestures drive target selection (tap enemy), double-tap for quick actions
(auto-run), and long press for context menus (invite/inspect/trade). Long press also initiates
drag for inventory item rearrangement and action bar customization.

- **Requirements:** R-6.3.1
- **Dependencies:** F-6.1.4
- **Platform notes:** Tap distance tolerance should scale with display DPI. Mobile platforms
  require larger tolerance zones than desktop touchscreens. Haptic feedback should fire at
  long-press recognition threshold on supported platforms (see F-6.4.1).

## Continuous Gestures

### F-6.3.2 Swipe Direction Recognition

Recognize linear swipe gestures in cardinal and diagonal directions with velocity and distance
reporting. Swipe drives mobile MMO navigation (swipe to dodge), quick-slot ability casting
(directional swipe on virtual pad), and UI panel dismissal.

- **Requirements:** R-6.3.2
- **Dependencies:** F-6.1.4
- **Platform notes:** Swipe detection must filter out incidental movement during tap and long
  press recognition. Velocity thresholds should be tunable per-platform to account for different
  touch surface sizes.

### F-6.3.3 Pinch, Rotate, and Pan Gestures

Track two-finger pinch (scale factor), rotation (angle delta), and single/multi-finger pan
(position delta with velocity). Pinch controls camera zoom on mobile MMO clients, rotation enables
free camera orbit and map rotation, and pan drives third-person camera orbiting, map scrolling,
and drag-and-drop for inventory management.

- **Requirements:** R-6.3.3
- **Dependencies:** F-6.1.4
- **Platform notes:** macOS trackpad gestures use `NSEvent` magnification and rotation types.
  On touch platforms, simultaneous pinch and rotate must be disambiguated when fingers move in
  ambiguous arcs. Pinch and pan may run simultaneously for camera zoom-and-orbit.

## Gesture Engine

### F-6.3.4 Gesture State Machines with Conflict Resolution

Implement gesture recognition as composable state machines with well-defined lifecycle states
(possible, began, changed, ended, cancelled, failed). Resolve ambiguity when multiple recognizers
match using configurable strategies: require-failure (tap waits for double-tap timeout),
simultaneous recognition (pan and pinch together), and priority ordering. Correct conflict
resolution prevents frustrating input swallowing in complex MMO touch interfaces.

- **Requirements:** R-6.3.4
- **Dependencies:** F-6.3.1, F-6.3.2, F-6.3.3
- **Platform notes:** Resolution strategies mirror UIKit's `UIGestureRecognizer` delegate model
  and Android's `GestureDetector` patterns but are implemented engine-side for cross-platform
  consistency.

### F-6.3.5 Custom Gesture Definition

Allow gameplay code to define custom gesture recognizers by composing primitive recognizers or
implementing a gesture trait with touch event callbacks. Custom gestures enable MMO-specific
interactions such as drawing rune shapes to cast spells, circular swipes for area-of-effect
abilities, or signature gestures for emotes.

- **Requirements:** R-6.3.5
- **Dependencies:** F-6.3.4
- **Platform notes:** None
