# 6.3 — Gestures

## Discrete Gestures

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-6.3.1 | Tap, Multi-Tap, and Long Press Recognition | Recognize single-tap, double-tap, triple-tap, and long press gestures with configurable time and distance thresholds. Tap gestures drive target selection (tap enemy), double-tap for quick actions (auto-run), and long press for context menus (invite/inspect/trade). Long press also initiates drag for inventory item rearrangement and action bar customization. larger tolerance zones than desktop touchscreens. Haptic feedback should fire at long-press recognition threshold on supported platforms (see F-6.4.1). | R-6.3.1 | F-6.1.4 | Tap distance tolerance should scale with display DPI. Mobile platforms require |

## Continuous Gestures

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-6.3.2 | Swipe Direction Recognition | Recognize linear swipe gestures in cardinal and diagonal directions with velocity and distance reporting. Swipe drives mobile MMO navigation (swipe to dodge), quick-slot ability casting (directional swipe on virtual pad), and UI panel dismissal. recognition. Velocity thresholds should be tunable per-platform to account for different touch surface sizes. | R-6.3.2 | F-6.1.4 | Swipe detection must filter out incidental movement during tap and long press |
| F-6.3.3 | Pinch, Rotate, and Pan Gestures | Track two-finger pinch (scale factor), rotation (angle delta), and single/multi-finger pan (position delta with velocity). Pinch controls camera zoom on mobile MMO clients, rotation enables free camera orbit and map rotation, and pan drives third-person camera orbiting, map scrolling, and drag-and-drop for inventory management. touch platforms, simultaneous pinch and rotate must be disambiguated when fingers move in ambiguous arcs. Pinch and pan may run simultaneously for camera zoom-and-orbit. | R-6.3.3 | F-6.1.4 | macOS trackpad gestures use `NSEvent` magnification and rotation types. On |

## Gesture Engine

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-6.3.4 | Gesture State Machines with Conflict Resolution | Implement gesture recognition as composable state machines with well-defined lifecycle states (possible, began, changed, ended, cancelled, failed). Resolve ambiguity when multiple recognizers match using configurable strategies: require-failure (tap waits for double-tap timeout), simultaneous recognition (pan and pinch together), and priority ordering. Correct conflict resolution prevents frustrating input swallowing in complex MMO touch interfaces. Android's `GestureDetector` patterns but are implemented engine-side for cross-platform consistency. | R-6.3.4 | F-6.3.1, F-6.3.2, F-6.3.3 | Resolution strategies mirror UIKit's `UIGestureRecognizer` delegate model and |
| F-6.3.5 | Custom Gesture Definition | Define custom gesture recognizers visually by composing touch event patterns in the gesture editor. Authors drag primitive recognizers (tap, swipe, hold, pinch) onto a timeline, connect them with sequencing and branching nodes, and set distance, angle, and timing thresholds via property panels. Complex patterns such as drawing rune shapes to cast spells, circular swipes for area-of-effect targeting, or signature emote gestures are authored entirely in the visual editor and evaluated at runtime through the logic graph system (F-15.8.4). support custom gestures but are secondary targets. | R-6.3.5 | F-6.3.4, F-15.8.4 (Logic Graphs) | Primarily used on mobile and tablet. Desktop touchscreen and macOS trackpad |
