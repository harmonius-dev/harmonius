# User Stories — 6.3 Gestures

## US-6.3.1.1 Recognize Single Tap Gesture

**As an** engine developer (P-26), **I want** single-tap gesture recognition with configurable
distance threshold, **so that** target selection works on touch.

## US-6.3.1.2 Recognize Double-Tap Gesture

**As an** engine developer (P-26), **I want** double-tap recognition with configurable timing,
**so that** quick actions (auto-run) work.

## US-6.3.1.3 Recognize Long Press Gesture

**As an** engine developer (P-26), **I want** long press recognition, **so that** context menus
(invite, inspect, trade) activate on sustained touch.

## US-6.3.1.4 Tap to Select Targets

**As a** player (P-23), **I want to** tap enemies to select them, **so that** target selection is
intuitive on touch.

## US-6.3.1.5 Scale Tap Distance with Display DPI

**As an** engine developer (P-26), **I want** tap distance tolerance scaled with display DPI,
**so that** mobile vs desktop touchscreens are accommodated.

## US-6.3.1.6 Trigger Haptic on Long Press Recognition

**As an** engine developer (P-26), **I want** haptic feedback at long-press threshold on supported
platforms, **so that** the player knows the gesture was recognized.

## US-6.3.1.7 Configure Gesture Thresholds in Editor

**As a** designer (P-5), **I want** tap timing and distance thresholds configurable in the editor,
**so that** gesture sensitivity is tunable.

## US-6.3.1.8 Long Press for Context Menu

**As a** player (P-23), **I want** long press to open a context menu on objects, **so that**
interact options are accessible.

## US-6.3.1.9 Recognize Triple-Tap

**As an** engine developer (P-26), **I want** triple-tap recognition, **so that** text selection and
special actions work.

## US-6.3.1.10 Verify Gesture Timing Thresholds

**As an** engine tester (P-27), **I want to** verify tap, double-tap, and long press timing
thresholds, **so that** recognition is accurate.

## US-6.3.1.11 Initiate Drag with Long Press

**As a** player (P-23), **I want** long press to initiate drag for inventory rearrangement,
**so that** item management works on touch.

## US-6.3.1.12 Test Gesture Recognition on Various DPIs

**As a** QA tester (P-19), **I want to** test gesture recognition on screens of different DPI,
**so that** tolerance scaling is validated.

## US-6.3.2.1 Recognize Cardinal Swipe Directions

**As an** engine developer (P-26), **I want** linear swipe recognition in cardinal and diagonal
directions, **so that** directional input works on touch.

## US-6.3.2.2 Report Swipe Velocity and Distance

**As an** engine developer (P-26), **I want** swipe velocity and distance reported, **so that**
gesture intensity is available.

## US-6.3.2.3 Swipe to Dodge on Mobile

**As a** player (P-23), **I want** swiping to trigger dodge in the swipe direction, **so that**
touch combat feels responsive.

## US-6.3.2.4 Use Directional Swipe for Quick-Slot Casting

**As a** designer (P-5), **I want** directional swipe on virtual pad to cast quick-slot abilities,
**so that** ability activation is fast.

## US-6.3.2.5 Filter Swipe from Tap Recognition

**As an** engine developer (P-26), **I want** swipe detection to filter out incidental movement
during tap and long press, **so that** gestures do not conflict.

## US-6.3.2.6 Configure Velocity Thresholds Per Platform

**As a** designer (P-5), **I want** swipe velocity thresholds tunable per platform, **so that**
different touch surfaces are accommodated.

## US-6.3.2.7 Verify Swipe Direction Accuracy

**As an** engine tester (P-27), **I want to** verify swipe direction detection is accurate for all 8
directions, **so that** directional input is reliable.

## US-6.3.2.8 Dismiss UI Panels with Swipe

**As a** player (P-23), **I want** swiping to dismiss UI panels, **so that** panel management is
quick.

## US-6.3.2.9 Test Swipe vs Tap Disambiguation

**As an** engine tester (P-27), **I want to** test swipe vs tap disambiguation, **so that** gestures
do not misfire.

## US-6.3.2.10 Use Swipe for Navigation

**As a** designer (P-5), **I want** swipe for navigation shortcuts, **so that** mobile UI flow is
efficient.

## US-6.3.2.11 Detect Diagonal Swipes

**As an** engine developer (P-26), **I want** diagonal swipe detection, **so that** 8- directional
input is available.

## US-6.3.2.12 Test Swipe on Various Touch Surfaces

**As a** QA tester (P-19), **I want to** test swipe recognition on phone, tablet, and trackpad,
**so that** all surfaces work correctly.

## US-6.3.3.1 Track Pinch Scale Factor

**As an** engine developer (P-26), **I want** two-finger pinch tracked with scale factor,
**so that** camera zoom on mobile works.

## US-6.3.3.2 Track Rotation Angle Delta

**As an** engine developer (P-26), **I want** two-finger rotation tracked with angle delta,
**so that** camera orbit and map rotation work.

## US-6.3.3.3 Track Pan Position and Velocity

**As an** engine developer (P-26), **I want** single/multi-finger pan tracked with position delta
and velocity, **so that** camera orbiting and scrolling work.

## US-6.3.3.4 Pinch to Zoom Camera

**As a** player (P-23), **I want** pinching to zoom the camera on mobile, **so that** view distance
is controllable.

## US-6.3.3.5 Rotate Map with Two Fingers

**As a** player (P-23), **I want** two-finger rotation to rotate the map, **so that** map
orientation is adjustable.

## US-6.3.3.6 Support Simultaneous Pinch and Pan

**As an** engine developer (P-26), **I want** pinch and pan to run simultaneously, **so that**
camera zoom-and-orbit is a single gesture.

## US-6.3.3.7 Use NSEvent for macOS Trackpad Gestures

**As an** engine developer (P-26), **I want** macOS trackpad gestures via NSEvent magnification and
rotation, **so that** trackpad input is native.

## US-6.3.3.8 Disambiguate Pinch from Rotate

**As an** engine developer (P-26), **I want** pinch and rotate disambiguated when finger arcs are
ambiguous, **so that** gestures do not conflict.

## US-6.3.3.9 Drag Inventory Items with Pan

**As a** player (P-23), **I want** panning to drag inventory items on touch, **so that** item
management is intuitive.

## US-6.3.3.10 Verify Pinch Accuracy

**As an** engine tester (P-27), **I want to** verify pinch scale factor accuracy, **so that** zoom
corresponds to finger spread.

## US-6.3.3.11 Configure Pan Sensitivity

**As a** designer (P-5), **I want** pan sensitivity configurable, **so that** camera orbit speed is
tunable.

## US-6.3.3.12 Test Multi-Touch Gestures on Various Devices

**As a** QA tester (P-19), **I want to** test pinch, rotate, and pan on phone, tablet, and trackpad,
**so that** all devices work correctly.

## US-6.3.4.1 Implement Gesture State Machines

**As an** engine developer (P-26), **I want** gesture recognition as composable state machines with
lifecycle states (possible, began, changed, ended, cancelled, failed), **so that** gesture lifecycle
is well-defined.

## US-6.3.4.2 Implement Require-Failure Resolution

**As an** engine developer (P-26), **I want** require-failure strategy (tap waits for double- tap
timeout), **so that** ambiguous gestures are resolved.

## US-6.3.4.3 Implement Simultaneous Recognition

**As an** engine developer (P-26), **I want** simultaneous recognition (pan and pinch together),
**so that** combined gestures work.

## US-6.3.4.4 Use Touch Without Input Swallowing

**As a** player (P-23), **I want** touch input to work without gestures eating each other,
**so that** controls are reliable.

## US-6.3.4.5 Implement Priority Ordering

**As an** engine developer (P-26), **I want** priority ordering between conflicting recognizers,
**so that** higher-priority gestures win.

## US-6.3.4.6 Configure Conflict Resolution in Editor

**As a** designer (P-5), **I want** conflict resolution strategies configurable in the editor,
**so that** gesture behavior is tunable.

## US-6.3.4.7 Verify Gesture Lifecycle States

**As an** engine tester (P-27), **I want to** verify all lifecycle states fire correctly,
**so that** recognizer state machines are accurate.

## US-6.3.4.8 Test Require-Failure Timing

**As an** engine tester (P-27), **I want to** test require-failure with various timings, **so that**
tap vs double-tap resolves correctly.

## US-6.3.4.9 Experience Reliable Multi-Gesture UI

**As a** player (P-23), **I want** simultaneous pinch-and-pan to work reliably, **so that** camera
control feels natural.

## US-6.3.4.10 Implement Cross-Platform Consistency

**As an** engine developer (P-26), **I want** gesture resolution consistent across platforms,
**so that** behavior is predictable everywhere.

## US-6.3.4.11 Test Conflicting Gesture Pairs

**As a** QA tester (P-19), **I want to** test all conflicting gesture pairs and verify resolution,
**so that** no input is swallowed incorrectly.

## US-6.3.4.12 Test Cancelled and Failed States

**As an** engine tester (P-27), **I want to** test gesture cancellation and failure paths,
**so that** error states are handled cleanly.

## US-6.3.5.1 Define Custom Gesture Recognizers Visually

**As a** designer (P-5), **I want to** define custom gesture recognizers in the visual gesture
editor, **so that** novel gestures are authored without code.

## US-6.3.5.2 Compose from Primitive Recognizers

**As a** designer (P-5), **I want to** compose custom gestures from tap, swipe, hold, and pinch
primitives, **so that** complex patterns are built from simple parts.

## US-6.3.5.3 Draw Rune Shapes to Cast Spells

**As a** player (P-23), **I want to** draw rune shapes on screen to cast spells, **so that**
gesture-based magic feels immersive.

## US-6.3.5.4 Evaluate Custom Gestures via Logic Graphs

**As an** engine developer (P-26), **I want** custom gestures evaluated at runtime through the logic
graph system, **so that** recognition is data-driven.

## US-6.3.5.5 Set Thresholds via Property Panels

**As a** designer (P-5), **I want** distance, angle, and timing thresholds set via property panels,
**so that** custom gesture sensitivity is tunable.

## US-6.3.5.6 Author Circular Swipe for AoE Targeting

**As a** designer (P-5), **I want** circular swipe gestures for area-of-effect targeting,
**so that** spatial abilities use touch input.

## US-6.3.5.7 Test Custom Gesture Recognition Accuracy

**As an** engine tester (P-27), **I want to** test custom gesture recognition accuracy, **so that**
authored patterns are detected reliably.

## US-6.3.5.8 Connect Primitives with Sequencing Nodes

**As an** engine developer (P-26), **I want** primitive recognizers connected with sequencing and
branching nodes, **so that** multi-step gestures are composable.

## US-6.3.5.9 Use Emote Gestures for Social Interaction

**As a** player (P-23), **I want** signature emote gestures on touch, **so that** social expression
uses custom input.

## US-6.3.5.10 Support Desktop Touchscreen Gestures

**As an** engine developer (P-26), **I want** custom gestures supported on desktop touchscreen and
macOS trackpad, **so that** non-mobile touch is covered.

## US-6.3.5.11 Test Custom Gestures on All Touch Devices

**As a** QA tester (P-19), **I want to** test custom gestures on phone, tablet, and trackpad,
**so that** all touch devices work.

## US-6.3.5.12 Verify Logic Graph Integration

**As an** engine tester (P-27), **I want to** verify custom gesture evaluation through logic graphs,
**so that** data-driven recognition works correctly.
