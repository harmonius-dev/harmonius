# R-6.5 — VR and Spatial Input Requirements

## Head Tracking

| ID       | Derived From |
|----------|--------------|
| R-6.5.1  |              |
| R-6.5.1a |              |
| R-6.5.1b |              |
| R-6.5.1c |              |

1. **R-6.5.1** — The engine **SHALL** provide 6DOF head tracking (position and orientation) via
   platform-native VR APIs (OpenXR on PC, OVR on Quest, PSVR2 SDK), exposing `HmdPose` and
   `HmdVelocity` as ECS components updated at the headset's native refresh rate (up to 120 Hz).
   [F-6.5.1](../../features/input/vr-input.md) judder. update every frame. Verify pose data contains
   valid position (meters) and quaternion orientation.
   - **Rationale:** Head tracking is the foundation of VR presence; refresh-rate updates prevent
   - **Verification:** Unit test: read `HmdPose` each frame at 90 Hz. Assert position and
     orientation
2. **R-6.5.1a** — The engine **SHALL** submit late-latching pose updates as close to scanout as
   possible, achieving motion-to-photon latency not exceeding 20 ms at the headset's native refresh
   rate. [F-6.5.1](../../features/input/vr-input.md) between pose sample and display. Assert within
   one frame interval. Assert total motion-to-photon latency does not exceed 20 ms at 90 Hz and 120
   Hz.
   - **Rationale:** Latency above 20 ms causes motion sickness; late-latching minimizes the interval
   - **Verification:** Integration test: measure timestamp delta between last pose update and
     scanout.
3. **R-6.5.1b** — The engine **SHALL** detect tracking loss and emit a tracking-loss ECS event
   within one frame. The response (freeze, warning overlay) **SHALL** be configurable in the visual
   editor. [F-6.5.1](../../features/input/vr-input.md) handling. one frame. Verify configured
   response (freeze or overlay) activates.
   - **Rationale:** Tracking loss during gameplay is disorienting; immediate detection enables
     graceful
   - **Verification:** Unit test: simulate tracking occlusion. Assert tracking-loss event fires
     within
4. **R-6.5.1c** — The engine **SHALL** support room-scale, seated, and standing play area modes with
   configurable guardian/chaperone boundary events, selectable in the visual editor per project.
   [F-6.5.1](../../features/input/vr-input.md) players from hitting walls. event fires. Set seated
   mode. Assert boundary constraints match seated configuration.
   - **Rationale:** Different VR experiences require different play areas; guardian events prevent
   - **Verification:** Unit test: set room-scale mode. Move headset past boundary. Assert guardian

## Motion Controllers

| ID       | Derived From |
|----------|--------------|
| R-6.5.2  |              |
| R-6.5.2a |              |

1. **R-6.5.2** — The engine **SHALL** expose 6DOF controller tracking (position, orientation,
   velocity, angular velocity), button states, analog trigger and thumbstick axes, and capacitive
   touch sensor data as ECS components per hand, via OpenXR interaction profiles, PSVR2 Sense, and
   Quest Touch APIs. [F-6.5.2](../../features/input/vr-input.md) identically to gamepad input (100%
   ECS-based constraint). each frame. Assert all update. Verify capacitive touch is present on
   controllers that support it and absent on those that do not.
   - **Rationale:** Full controller state as ECS components enables gameplay systems to query VR
     input
   - **Verification:** Unit test: read `ControllerPose`, button, trigger, and thumbstick components
2. **R-6.5.2a** — The engine **SHALL** map VR controller inputs to the same typed action system as
   gamepad inputs (F-6.2.1), so that shared input mappings work across flat and VR modes.
   [F-6.5.2](../../features/input/vr-input.md) gameplay. both produce the same `ActionState` value.
   - **Rationale:** Shared mappings prevent duplicated binding configurations and enable cross-mode
   - **Verification:** Unit test: bind a "Fire" action to both gamepad trigger and VR trigger.
     Assert

## Hand Tracking

| ID       | Derived From |
|----------|--------------|
| R-6.5.3  |              |
| R-6.5.3a |              |
| R-6.5.3b |              |
| R-6.5.3c |              |

1. **R-6.5.3** — The engine **SHALL** provide camera-based hand tracking with a 26-joint skeletal
   hand model per hand, exposing joint positions, orientations, and radii as ECS components
   (`HandSkeleton`, `HandJointPose`), updated at a minimum of 30 Hz with per-joint accuracy within 5
   mm. [F-6.5.3](../../features/input/vr-input.md) visible jitter; 5 mm accuracy enables reliable
   pinch and grab detection. frame at >= 30 Hz. Compare engine joint positions against SDK
   reference. Assert < 5 mm RMS error.
   - **Rationale:** 26 joints enable full finger articulation for gesture recognition; 30 Hz
     prevents
   - **Verification:** Integration test: display hand skeleton and verify all 26 joints update each
2. **R-6.5.3a** — The engine **SHALL** recognize predefined hand gestures (pinch, grab, point,
   thumbs-up, open palm) and expose them as input actions through the typed action system (F-6.2.1).
   [F-6.5.3](../../features/input/vr-input.md) point at targets). grab gesture. Assert the grab
   action fires.
   - **Rationale:** Common gestures must be recognized out of the box for interaction (grab objects,
   - **Verification:** Unit test: perform a pinch gesture. Assert the pinch action fires. Perform a
3. **R-6.5.3b** — The engine **SHALL** support custom hand gestures authored in the visual logic
   graph using joint angle and distance thresholds, loadable as data assets without recompilation.
   [F-6.5.3](../../features/input/vr-input.md) built-in recognizers do not cover; no-code constraint
   requires visual authoring. the gesture. Assert the custom action fires.
   - **Rationale:** Games need unique gestures (thumbs-up for emotes, custom spell casting) that
   - **Verification:** Integration test: author a thumbs-up gesture in the logic graph editor.
     Perform
4. **R-6.5.3c** — The engine **SHALL** automatically switch between controller tracking and hand
   tracking when controllers are picked up or put down, without requiring manual mode selection.
   [F-6.5.3](../../features/input/vr-input.md) controllers. Assert hand tracking activates
   automatically within 1 second.
   - **Rationale:** Players expect seamless transitions; manual mode switching breaks immersion.
   - **Verification:** Unit test: hold controllers and verify controller tracking is active. Release

## Eye Tracking

| ID       | Derived From |
|----------|--------------|
| R-6.5.4  |              |
| R-6.5.4a |              |
| R-6.5.4b |              |

1. **R-6.5.4** — The engine **SHALL** expose gaze direction, fixation point, and pupil data as ECS
   components (`GazeRay`), updated each frame via platform-native eye tracking APIs (OpenXR, PSVR2,
   Tobii). The engine **SHALL** classify gaze events as fixation (sustained gaze) or saccade (rapid
   eye movement). [F-6.5.4](../../features/input/vr-input.md) vector. Fixate for 500 ms. Assert
   fixation event fires. Perform rapid eye movement. Assert saccade event fires.
   - **Rationale:** Gaze data drives foveated rendering, UI interaction, and gameplay mechanics.
   - **Verification:** Unit test: read `GazeRay` each frame. Assert it contains a valid direction
2. **R-6.5.4a** — The engine **SHALL** support gaze-based UI selection (look-and-select) where UI
   elements highlight when the gaze ray intersects them, configurable in the visual editor.
   [F-6.5.4](../../features/input/vr-input.md) constraint requires visual configuration. away.
   Assert highlight is removed.
   - **Rationale:** Gaze selection is the primary UI interaction mode in some VR experiences;
     no-code
   - **Verification:** Unit test: look at a UI element. Assert gaze-based selection highlights it.
     Look
3. **R-6.5.4b** — The engine **SHALL** expose `GazeRay` data in a format consumable by the foveated
   rendering system, enabling GPU shading detail allocation based on gaze direction.
   [F-6.5.4](../../features/input/vr-input.md) lower detail; it requires per-frame gaze data.
   adjusts shading detail based on gaze direction.
   - **Rationale:** Foveated rendering reduces GPU load by 30-50% by rendering peripheral vision at
   - **Verification:** Integration test: verify the rendering system reads `GazeRay` each frame and

## VR Haptics

| ID       | Derived From |
|----------|--------------|
| R-6.5.5  |              |
| R-6.5.5a |              |
| R-6.5.5b |              |

1. **R-6.5.5** — The engine **SHALL** provide per-hand haptic feedback on VR controllers with
   amplitude, frequency, and duration control, supporting continuous vibration, impulse events, and
   spatially-driven haptics (intensity varies with proximity to virtual objects). Patterns **SHALL**
   be authored as assets in the visual editor. [F-6.5.5](../../features/input/vr-input.md) spatial
   haptics reinforce presence. controller. Assert output matches parameters. Trigger different
   haptics on left and right simultaneously. Assert independence. Move controller toward a virtual
   object. Assert intensity increases with proximity.
   - **Rationale:** Per-hand haptics enable asymmetric feedback (sword in right, shield in left);
   - **Verification:** Unit test: trigger an impulse at 0.8 amplitude and 150 Hz on the right
2. **R-6.5.5a** — The engine **SHALL** allow authoring VR haptic patterns (amplitude, frequency,
   duration curves) as visual editor assets, triggerable through the input action system or gameplay
   ECS events. [F-6.5.5](../../features/input/vr-input.md) ramp-up. Play at runtime. Assert the
   pattern sustains for the configured duration with the authored amplitude curve.
   - **Rationale:** No-code engine constraint; VR haptic patterns must be authorable without code.
   - **Verification:** Integration test: create a continuous vibration pattern in the editor with
3. **R-6.5.5b** — The engine **SHALL** abstract VR haptic output across OpenXR, PSVR2 Sense, and
   Quest Touch APIs, converting authored patterns to platform-specific output at the backend.
   [F-6.5.5](../../features/input/vr-input.md) Assert both produce haptic output matching the
   authored parameters.
   - **Rationale:** A common authoring format avoids per-platform haptic asset duplication.
   - **Verification:** Integration test: play the same haptic pattern on Oculus Touch and PSVR2
     Sense.

---
