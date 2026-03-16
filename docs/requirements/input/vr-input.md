# R-6.5 — VR and Spatial Input Requirements

## Head Tracking

### R-6.5.1 6DOF Head-Mounted Display Tracking

The engine **SHALL** provide 6DOF head tracking (position and orientation) via platform-native VR
APIs (OpenXR on PC, OVR on Quest, PSVR2 SDK), exposing `HmdPose` and `HmdVelocity` as ECS components
updated at the headset's native refresh rate (up to 120 Hz).

- **Derived from:**
  [F-6.5.1](../../features/input/vr-input.md)
- **Rationale:** Head tracking is the foundation of VR presence; refresh-rate updates prevent
  judder.
- **Verification:** Unit test: read `HmdPose` each frame at 90 Hz. Assert position and orientation
  update every frame. Verify pose data contains valid position (meters) and quaternion orientation.

### R-6.5.1a Late-Latching Pose Updates

The engine **SHALL** submit late-latching pose updates as close to scanout as possible, achieving
motion-to-photon latency not exceeding 20 ms at the headset's native refresh rate.

- **Derived from:**
  [F-6.5.1](../../features/input/vr-input.md)
- **Rationale:** Latency above 20 ms causes motion sickness; late-latching minimizes the interval
  between pose sample and display.
- **Verification:** Integration test: measure timestamp delta between last pose update and scanout.
  Assert within one frame interval. Assert total motion-to-photon latency does not exceed 20 ms at
  90 Hz and 120 Hz.

### R-6.5.1b Tracking Loss Detection

The engine **SHALL** detect tracking loss and emit a tracking-loss ECS event within one frame. The
response (freeze, warning overlay) **SHALL** be configurable in the visual editor.

- **Derived from:**
  [F-6.5.1](../../features/input/vr-input.md)
- **Rationale:** Tracking loss during gameplay is disorienting; immediate detection enables graceful
  handling.
- **Verification:** Unit test: simulate tracking occlusion. Assert tracking-loss event fires within
  one frame. Verify configured response (freeze or overlay) activates.

### R-6.5.1c Play Area Modes and Guardian Boundaries

The engine **SHALL** support room-scale, seated, and standing play area modes with configurable
guardian/chaperone boundary events, selectable in the visual editor per project.

- **Derived from:**
  [F-6.5.1](../../features/input/vr-input.md)
- **Rationale:** Different VR experiences require different play areas; guardian events prevent
  players from hitting walls.
- **Verification:** Unit test: set room-scale mode. Move headset past boundary. Assert guardian
  event fires. Set seated mode. Assert boundary constraints match seated configuration.

## Motion Controllers

### R-6.5.2 6DOF Motion Controller Input

The engine **SHALL** expose 6DOF controller tracking (position, orientation, velocity, angular
velocity), button states, analog trigger and thumbstick axes, and capacitive touch sensor data as
ECS components per hand, via OpenXR interaction profiles, PSVR2 Sense, and Quest Touch APIs.

- **Derived from:**
  [F-6.5.2](../../features/input/vr-input.md)
- **Rationale:** Full controller state as ECS components enables gameplay systems to query VR input
  identically to gamepad input (100% ECS-based constraint).
- **Verification:** Unit test: read `ControllerPose`, button, trigger, and thumbstick components
  each frame. Assert all update. Verify capacitive touch is present on controllers that support it
  and absent on those that do not.

### R-6.5.2a Unified VR and Gamepad Action Mapping

The engine **SHALL** map VR controller inputs to the same typed action system as gamepad inputs
(F-6.2.1), so that shared input mappings work across flat and VR modes.

- **Derived from:**
  [F-6.5.2](../../features/input/vr-input.md)
- **Rationale:** Shared mappings prevent duplicated binding configurations and enable cross-mode
  gameplay.
- **Verification:** Unit test: bind a "Fire" action to both gamepad trigger and VR trigger. Assert
  both produce the same `ActionState` value.

## Hand Tracking

### R-6.5.3 26-Joint Skeletal Hand Tracking

The engine **SHALL** provide camera-based hand tracking with a 26-joint skeletal hand model per
hand, exposing joint positions, orientations, and radii as ECS components (`HandSkeleton`,
`HandJointPose`), updated at a minimum of 30 Hz with per-joint accuracy within 5 mm.

- **Derived from:**
  [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** 26 joints enable full finger articulation for gesture recognition; 30 Hz prevents
  visible jitter; 5 mm accuracy enables reliable pinch and grab detection.
- **Verification:** Integration test: display hand skeleton and verify all 26 joints update each
  frame at >= 30 Hz. Compare engine joint positions against SDK reference. Assert < 5 mm RMS error.

### R-6.5.3a Predefined Hand Gesture Recognition

The engine **SHALL** recognize predefined hand gestures (pinch, grab, point, thumbs-up, open palm)
and expose them as input actions through the typed action system (F-6.2.1).

- **Derived from:**
  [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** Common gestures must be recognized out of the box for interaction (grab objects,
  point at targets).
- **Verification:** Unit test: perform a pinch gesture. Assert the pinch action fires. Perform a
  grab gesture. Assert the grab action fires.

### R-6.5.3b Custom Hand Gesture Definition

The engine **SHALL** support custom hand gestures authored in the visual logic graph using joint
angle and distance thresholds, loadable as data assets without recompilation.

- **Derived from:**
  [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** Games need unique gestures (thumbs-up for emotes, custom spell casting) that
  built-in recognizers do not cover; no-code constraint requires visual authoring.
- **Verification:** Integration test: author a thumbs-up gesture in the logic graph editor. Perform
  the gesture. Assert the custom action fires.

### R-6.5.3c Controller-Hand Auto-Switching

The engine **SHALL** automatically switch between controller tracking and hand tracking when
controllers are picked up or put down, without requiring manual mode selection.

- **Derived from:**
  [F-6.5.3](../../features/input/vr-input.md)
- **Rationale:** Players expect seamless transitions; manual mode switching breaks immersion.
- **Verification:** Unit test: hold controllers and verify controller tracking is active. Release
  controllers. Assert hand tracking activates automatically within 1 second.

## Eye Tracking

### R-6.5.4 Eye Tracking and Gaze Input

The engine **SHALL** expose gaze direction, fixation point, and pupil data as ECS components
(`GazeRay`), updated each frame via platform-native eye tracking APIs (OpenXR, PSVR2, Tobii). The
engine **SHALL** classify gaze events as fixation (sustained gaze) or saccade (rapid eye movement).

- **Derived from:**
  [F-6.5.4](../../features/input/vr-input.md)
- **Rationale:** Gaze data drives foveated rendering, UI interaction, and gameplay mechanics.
- **Verification:** Unit test: read `GazeRay` each frame. Assert it contains a valid direction
  vector. Fixate for 500 ms. Assert fixation event fires. Perform rapid eye movement. Assert saccade
  event fires.

### R-6.5.4a Gaze-Based UI Selection

The engine **SHALL** support gaze-based UI selection (look-and-select) where UI elements highlight
when the gaze ray intersects them, configurable in the visual editor.

- **Derived from:**
  [F-6.5.4](../../features/input/vr-input.md)
- **Rationale:** Gaze selection is the primary UI interaction mode in some VR experiences; no-code
  constraint requires visual configuration.
- **Verification:** Unit test: look at a UI element. Assert gaze-based selection highlights it. Look
  away. Assert highlight is removed.

### R-6.5.4b Foveated Rendering Integration

The engine **SHALL** expose `GazeRay` data in a format consumable by the foveated rendering system,
enabling GPU shading detail allocation based on gaze direction.

- **Derived from:**
  [F-6.5.4](../../features/input/vr-input.md)
- **Rationale:** Foveated rendering reduces GPU load by 30-50% by rendering peripheral vision at
  lower detail; it requires per-frame gaze data.
- **Verification:** Integration test: verify the rendering system reads `GazeRay` each frame and
  adjusts shading detail based on gaze direction.

## VR Haptics

### R-6.5.5 Per-Hand VR Controller Haptics

The engine **SHALL** provide per-hand haptic feedback on VR controllers with amplitude, frequency,
and duration control, supporting continuous vibration, impulse events, and spatially-driven haptics
(intensity varies with proximity to virtual objects). Patterns **SHALL** be authored as assets in
the visual editor.

- **Derived from:**
  [F-6.5.5](../../features/input/vr-input.md)
- **Rationale:** Per-hand haptics enable asymmetric feedback (sword in right, shield in left);
  spatial haptics reinforce presence.
- **Verification:** Unit test: trigger an impulse at 0.8 amplitude and 150 Hz on the right
  controller. Assert output matches parameters. Trigger different haptics on left and right
  simultaneously. Assert independence. Move controller toward a virtual object. Assert intensity
  increases with proximity.

### R-6.5.5a VR Haptic Pattern Authoring

The engine **SHALL** allow authoring VR haptic patterns (amplitude, frequency, duration curves) as
visual editor assets, triggerable through the input action system or gameplay ECS events.

- **Derived from:**
  [F-6.5.5](../../features/input/vr-input.md)
- **Rationale:** No-code engine constraint; VR haptic patterns must be authorable without code.
- **Verification:** Integration test: create a continuous vibration pattern in the editor with
  ramp-up. Play at runtime. Assert the pattern sustains for the configured duration with the
  authored amplitude curve.

### R-6.5.5b Cross-Platform VR Haptic Abstraction

The engine **SHALL** abstract VR haptic output across OpenXR, PSVR2 Sense, and Quest Touch APIs,
converting authored patterns to platform-specific output at the backend.

- **Derived from:**
  [F-6.5.5](../../features/input/vr-input.md)
- **Rationale:** A common authoring format avoids per-platform haptic asset duplication.
- **Verification:** Integration test: play the same haptic pattern on Oculus Touch and PSVR2 Sense.
  Assert both produce haptic output matching the authored parameters.

---

## User Stories

## F-6.5.1 Head-Mounted Display Tracking

## US-6.5.1.1 Configure Play Area Mode

**As a** designer (P-5), **I want to** select room-scale, seated, or standing play area modes with
configurable guardian boundaries in the editor, **so that** VR play space is defined per project.

## US-6.5.1.2 Configure Tracking Loss Response

**As a** designer (P-5), **I want to** configure the tracking loss response (freeze, warning
overlay) in the editor, **so that** loss-of-tracking behavior is authored without code.

## US-6.5.1.3 Verify 6DOF Head Tracking Rate

**As an** engine tester (P-27), **I want to** read `HmdPose` each frame and assert position and
orientation update at the headset's native refresh rate, **so that** tracking runs at full speed.

## US-6.5.1.4 Verify Late-Latching Pose Timing

**As an** engine tester (P-27), **I want to** measure timestamp delta between last pose update and
scanout and assert it is within one frame interval, **so that** late-latching minimizes latency.

## US-6.5.1.5 Verify Tracking Loss Detection

**As an** engine tester (P-27), **I want to** occlude tracking and assert a tracking-loss event
fires within one frame, **so that** loss detection is responsive.

## US-6.5.1.6 Verify Guardian Boundary Events

**As an** engine tester (P-27), **I want to** move the headset past the play area boundary and
verify a guardian event fires, **so that** boundary protection works.

## US-6.5.1.7 Implement HMD Tracking Layer

**As an** engine developer (P-26), **I want to** implement 6DOF head tracking via OpenXR on PC, OVR
on Quest, and PSVR2 SDK, with late-latching pose updates and tracking loss detection, **so that**
HMD tracking is cross-platform.

## US-6.5.1.8 Test HMD Tracking on All Supported Headsets

**As a** QA tester (P-19), **I want to** test head tracking on PCVR, Quest, and PSVR2, **so that**
tracking quality is verified across headsets.

## US-6.5.1.9 Look Around Freely in VR

**As a** player (P-23), **I want** my head movements to track smoothly and accurately, **so that**
VR feels natural and not nauseating.

---

## F-6.5.2 Motion Controller Input

## US-6.5.2.1 Configure VR Controller Bindings in Editor

**As a** designer (P-5), **I want to** bind VR controller buttons, triggers, and sticks to game
actions in the editor, sharing bindings with gamepad where possible, **so that** VR input uses the
standard action system.

## US-6.5.2.2 Verify 6DOF Controller Tracking

**As an** engine tester (P-27), **I want to** verify `ControllerPose`, button, trigger, and
thumbstick ECS components update each frame, **so that** controller tracking is complete.

## US-6.5.2.3 Verify Shared Action Mapping

**As an** engine tester (P-27), **I want to** bind a Fire action to both gamepad trigger and VR
trigger and assert the same action fires from either, **so that** action mapping is unified.

## US-6.5.2.4 Verify Capacitive Touch Sensors

**As an** engine tester (P-27), **I want to** verify capacitive touch data is exposed on controllers
that support it and absent on those that do not, **so that** capability detection works.

## US-6.5.2.5 Implement Motion Controller Input

**As an** engine developer (P-26), **I want to** implement 6DOF controller tracking with buttons,
triggers, thumbsticks, and capacitive touch via OpenXR interaction profiles, **so that** VR
controllers are fully supported.

## US-6.5.2.6 Test Controllers on All VR Platforms

**As a** QA tester (P-19), **I want to** test Oculus Touch, Index, PSVR2 Sense, and Quest Touch
controllers, **so that** all supported controllers work.

## US-6.5.2.7 Interact with the VR World Using Controllers

**As a** player (P-23), **I want** my hand controllers to track precisely and respond to button
presses instantly, **so that** VR interaction is natural.

---

## F-6.5.3 Hand Tracking and Skeletal Input

## US-6.5.3.1 Configure Hand Gesture Actions in Editor

**As a** designer (P-5), **I want to** bind predefined hand gestures (pinch, grab, point) to game
actions in the editor, **so that** hand tracking drives gameplay without code.

## US-6.5.3.2 Author Custom Hand Gestures in Logic Graph

**As a** designer (P-5), **I want to** author custom hand gestures using joint angle thresholds in
the visual logic graph, **so that** game-specific gestures are data-driven.

## US-6.5.3.3 Verify 26-Joint Skeletal Tracking

**As an** engine tester (P-27), **I want to** display a hand and verify all 26 joints of
`HandSkeleton` update each frame with valid positions, **so that** full skeletal tracking works.

## US-6.5.3.4 Verify Predefined Gesture Recognition

**As an** engine tester (P-27), **I want to** perform a pinch gesture and assert the pinch action
fires, **so that** built-in gesture recognition works.

## US-6.5.3.5 Verify Custom Gesture Recognition

**As an** engine tester (P-27), **I want to** author a thumbs-up gesture in the logic graph, perform
it, and assert the action fires, **so that** custom hand gestures work.

## US-6.5.3.6 Verify Controller/Hand Auto-Switch

**As an** engine tester (P-27), **I want to** hold controllers and verify controller tracking, then
release and verify automatic switch to hand tracking, **so that** auto-switching works.

## US-6.5.3.7 Implement Hand Tracking System

**As an** engine developer (P-26), **I want to** implement camera-based hand tracking with 26-joint
skeleton, predefined gesture recognition, and auto-switching between controllers and hands,
**so that** hand tracking is available.

## US-6.5.3.8 Test Hand Tracking on Supported Headsets

**As a** QA tester (P-19), **I want to** test hand tracking on Quest and PC (via OpenXR),
**so that** hand tracking works on all supported headsets.

## US-6.5.3.9 Grab and Interact Without Controllers

**As a** player (P-23), **I want to** use my hands to grab, point, and interact in VR without
needing controllers, **so that** VR interaction is natural and controller-free.

---

## F-6.5.4 Eye Tracking and Gaze Input

## US-6.5.4.1 Configure Gaze-Based UI Interaction

**As a** designer (P-5), **I want to** enable gaze-based UI selection (look-and-select) in the
editor, **so that** eye tracking drives UI interaction.

## US-6.5.4.2 Configure Gaze-Driven Gameplay

**As a** designer (P-5), **I want to** bind gaze direction to gameplay mechanics (enemies react to
being looked at) in the editor, **so that** eye tracking enhances gameplay.

## US-6.5.4.3 Verify Gaze Ray Updates Each Frame

**As an** engine tester (P-27), **I want to** read `GazeRay` each frame and assert it contains a
valid direction vector, **so that** eye tracking data is available.

## US-6.5.4.4 Verify Gaze-Based Selection

**As an** engine tester (P-27), **I want to** look at a UI element and assert gaze-based selection
highlights it, **so that** look-and-select works.

## US-6.5.4.5 Verify Fixation Detection

**As an** engine tester (P-27), **I want to** fixate on a point for 500ms and assert a fixation
event fires, **so that** fixation detection works.

## US-6.5.4.6 Verify Saccade Detection

**As an** engine tester (P-27), **I want to** perform a rapid eye movement and assert a saccade
event fires, **so that** saccade classification works.

## US-6.5.4.7 Verify Foveated Rendering Integration

**As an** engine tester (P-27), **I want to** verify `GazeRay` data is consumable by the foveated
rendering system, **so that** GPU optimization uses eye tracking.

## US-6.5.4.8 Implement Eye Tracking System

**As an** engine developer (P-26), **I want to** implement eye tracking via OpenXR and platform SDKs
exposing gaze direction, fixation, saccade detection, and pupil data as ECS components, **so that**
eye tracking is available for rendering and interaction.

## US-6.5.4.9 Test Eye Tracking on Supported Headsets

**As a** QA tester (P-19), **I want to** test eye tracking on PSVR2, Quest Pro, and PC headsets with
Tobii, **so that** tracking works on all supported hardware.

## US-6.5.4.10 Select UI Elements by Looking

**As a** player (P-23), **I want to** select menu items and interact with the world by looking at
them, **so that** VR interaction uses natural gaze.

---

## F-6.5.5 VR Controller Haptics

## US-6.5.5.1 Author VR Haptic Patterns in Editor

**As a** designer (P-5), **I want to** create VR controller haptic patterns as visual editor assets
with amplitude, frequency, and duration, **so that** VR tactile feedback is data-driven.

## US-6.5.5.2 Configure Spatial Haptics

**As a** designer (P-5), **I want to** configure spatially-driven haptics where vibration intensity
varies with proximity to objects, **so that** VR haptics reinforce spatial awareness.

## US-6.5.5.3 Verify Impulse Haptic Output

**As an** engine tester (P-27), **I want to** trigger an impulse at 0.8 amplitude and 150Hz on the
right controller and verify output matches parameters, **so that** impulse haptics work.

## US-6.5.5.4 Verify Continuous Haptic Duration

**As an** engine tester (P-27), **I want to** trigger a continuous pattern and verify it sustains
for the configured duration, **so that** sustained haptics work.

## US-6.5.5.5 Verify Proximity-Based Haptic Scaling

**As an** engine tester (P-27), **I want to** move a controller toward a virtual object and verify
haptic intensity increases with proximity, **so that** spatial haptics work.

## US-6.5.5.6 Verify Per-Hand Asymmetric Haptics

**As an** engine tester (P-27), **I want to** trigger different haptics on left and right
controllers simultaneously and verify independence, **so that** per-hand channels work.

## US-6.5.5.7 Implement VR Controller Haptics

**As an** engine developer (P-26), **I want to** implement per-hand haptic feedback via OpenXR and
platform SDKs with continuous, impulse, and spatially-driven patterns, **so that** VR haptics are
available.

## US-6.5.5.8 Test VR Haptics on All Supported Controllers

**As a** QA tester (P-19), **I want to** test haptic patterns on Oculus Touch, Index, PSVR2 Sense,
and Quest Touch controllers, **so that** VR haptic compatibility is verified.

## US-6.5.5.9 Feel Objects When Reaching Near Them

**As a** player (P-23), **I want** my controllers to vibrate when I reach near objects in VR,
**so that** I can feel the virtual world.

---

## Non-Functional Requirements

### R-6.5.NF1 Motion-to-Photon Latency

The engine **SHALL** achieve motion-to-photon latency not exceeding 20 ms at the headset's native
refresh rate, using late-latching pose updates.

- **Derived from:** F-6.5.1
- **Rationale:** Motion-to-photon latency above 20 ms causes motion sickness in VR.
- **Verification:** Integration test: measure timestamp delta between pose and scanout. Assert it
  does not exceed 20 ms at 90 Hz and 120 Hz.

### R-6.5.NF2 Hand Tracking Joint Update Rate

The engine **SHALL** update the 26-joint skeletal hand model at a minimum of 30 Hz with per-joint
position accuracy within 5 mm.

- **Derived from:** F-6.5.3
- **Rationale:** Below 30 Hz causes visible jitter and gesture failures. 5 mm accuracy ensures
  reliable gesture recognition.
- **Verification:** Integration test: track a hand for 10 seconds. Assert joint updates at >= 30 Hz.
  Compare engine positions against SDK values. Assert < 5 mm RMS error.
