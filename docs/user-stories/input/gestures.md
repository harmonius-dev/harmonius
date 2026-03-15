# User Stories -- 6.3 Gestures

## US-6.3.1 Use Tap and Long Press on Touch
**As a** player, **I want** the engine to recognize single-tap, double-tap, triple-tap, and
long press gestures with DPI-scaled thresholds, **so that** I can select targets with tap,
trigger quick actions with double-tap, and open context menus with long press on any touch
device.

## US-6.3.2 Swipe to Dodge and Cast
**As a** player, **I want** directional swipe recognition with velocity and distance
reporting, **so that** I can dodge by swiping in a direction and cast abilities via
directional swipes on a virtual pad.

## US-6.3.3 Zoom and Orbit with Pinch and Pan
**As a** player, **I want** simultaneous pinch-to-zoom, rotation, and pan gestures, **so
that** I can zoom the camera while orbiting with two fingers on touch devices without one
gesture blocking the other.

## US-6.3.4 Never Have Gestures Swallow Inputs
**As an** input designer, **I want** gesture recognizers implemented as composable state
machines with conflict resolution (require-failure, simultaneous, priority), **so that**
overlapping gestures (tap vs. double-tap, pan vs. swipe) are resolved predictably without
swallowing valid inputs.

## US-6.3.5 Author Custom Gestures Visually
**As an** input designer, **I want** to define custom gesture recognizers by composing
primitives (tap, swipe, hold, pinch) in a visual editor with timeline and branching nodes,
**so that** rune-drawing spells and circular area-targeting swipes are authored without code.
