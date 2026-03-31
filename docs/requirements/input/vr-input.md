# R-6.5 — VR and Spatial Input Requirements

## Head Tracking

1. **R-6.5.1** — The engine **SHALL** provide 6DOF head tracking via platform-native VR APIs
   (OpenXR, OVR, PSVR2 SDK), exposing `HmdPose` and `HmdVelocity` as ECS components updated at the
   headset's native refresh rate (up to 120 Hz).
   - **Rationale:** Head tracking is the foundation of VR presence; refresh-rate updates prevent
     judder.
   - **Verification:** Read `HmdPose` each frame at 90 Hz. Assert position and orientation update
     every frame.

2. **R-6.5.2** — The engine **SHALL** submit late-latching pose updates close to scanout, achieving
   motion-to-photon latency not exceeding 20 ms.
   - **Rationale:** Latency above 20 ms causes motion sickness.
   - **Verification:** Measure timestamp delta between last pose update and scanout. Assert within
     one frame interval. Assert total latency under 20 ms.

3. **R-6.5.3** — The engine **SHALL** detect tracking loss and emit a tracking-loss ECS event within
   one frame. The response **SHALL** be configurable in the visual editor.
   - **Rationale:** Tracking loss is disorienting; immediate detection enables graceful handling.
   - **Verification:** Simulate tracking occlusion. Assert event fires within one frame. Verify
     configured response activates.

4. **R-6.5.4** — The engine **SHALL** support room-scale, seated, and standing play area modes with
   configurable guardian/chaperone boundary events.
   - **Rationale:** Different VR experiences require different play areas; guardian events prevent
     players from hitting walls.
   - **Verification:** Set room-scale mode. Move past boundary. Assert guardian event fires. Set
     seated mode. Assert constraints match configuration.

## Motion Controllers

5. **R-6.5.5** — The engine **SHALL** expose 6DOF controller tracking (position, orientation,
   velocity, angular velocity), button states, analog axes, and capacitive touch as ECS components
   per hand.
   - **Rationale:** Full controller state as ECS components enables gameplay systems to query VR
     input identically to gamepad input.
   - **Verification:** Read `ControllerPose`, button, trigger, and thumbstick components each frame.
     Assert all update. Verify capacitive touch on supporting controllers.

6. **R-6.5.6** — The engine **SHALL** map VR controller inputs to the same typed action system as
   gamepad inputs (F-6.2.1), so shared mappings work across flat and VR modes.
   - **Rationale:** Shared mappings prevent duplicated binding configurations.
   - **Verification:** Bind "Fire" to both gamepad trigger and VR trigger. Assert both produce the
     same `ActionState`.

## Hand Tracking

7. **R-6.5.7** — The engine **SHALL** provide camera-based hand tracking with a 26-joint skeletal
   hand model per hand, updated at minimum 30 Hz with per-joint accuracy within 5 mm.
   - **Rationale:** 26 joints enable full finger articulation; 30 Hz prevents visible jitter.
   - **Verification:** Display hand skeleton and verify 26 joints update each frame at >= 30 Hz.
     Compare against SDK reference. Assert < 5 mm RMS error.

8. **R-6.5.8** — The engine **SHALL** recognize predefined hand gestures (pinch, grab, point,
   thumbs-up, open palm) and expose them as input actions through the typed action system (F-6.2.1).
   - **Rationale:** Common gestures must be recognized out of the box for interaction.
   - **Verification:** Perform pinch gesture. Assert pinch action fires. Perform grab. Assert grab
     fires.

9. **R-6.5.9** — The engine **SHALL** support custom hand gestures authored in the logic graph using
   joint angle and distance thresholds, loadable as data assets.
   - **Rationale:** Games need unique gestures; no-code constraint requires visual authoring.
   - **Verification:** Author thumbs-up in the logic graph editor. Perform gesture. Assert custom
     action fires.

10. **R-6.5.10** — The engine **SHALL** automatically switch between controller and hand tracking
    when controllers are picked up or put down, within 1 second.
    - **Rationale:** Manual mode switching breaks immersion.
    - **Verification:** Hold controllers and verify controller tracking active. Release controllers.
      Assert hand tracking activates within 1 second.

## Eye Tracking

11. **R-6.5.11** — The engine **SHALL** expose gaze direction, fixation point, and pupil data as ECS
    components (`GazeRay`), updated each frame. The engine **SHALL** classify gaze events as
    fixation or saccade.
    - **Rationale:** Gaze data drives foveated rendering, UI interaction, and gameplay mechanics.
    - **Verification:** Read `GazeRay` each frame. Assert valid direction. Fixate 500 ms. Assert
      fixation event. Rapid movement. Assert saccade event.

12. **R-6.5.12** — The engine **SHALL** support gaze-based UI selection (look-and-select) where UI
    elements highlight on gaze intersection, configurable in the visual editor.
    - **Rationale:** Gaze selection is primary UI interaction in some VR experiences; no-code
      constraint requires visual configuration.
    - **Verification:** Look at UI element. Assert highlight appears. Look away. Assert highlight
      removed.

13. **R-6.5.13** — The engine **SHALL** expose `GazeRay` data in a format consumable by the foveated
    rendering system.
    - **Rationale:** Foveated rendering reduces GPU load by 30-50%.
    - **Verification:** Verify the rendering system reads `GazeRay` each frame and adjusts shading
      detail.

## VR Haptics

14. **R-6.5.14** — The engine **SHALL** provide per-hand haptic feedback on VR controllers with
    amplitude, frequency, and duration control, supporting continuous vibration, impulse events, and
    spatially-driven haptics. Patterns **SHALL** be authored as visual editor assets.
    - **Rationale:** Per-hand haptics enable asymmetric feedback; spatial haptics reinforce
      presence.
    - **Verification:** Trigger impulse at 0.8 amplitude and 150 Hz on right controller. Assert
      output matches. Trigger different haptics on left and right. Assert independence.

15. **R-6.5.15** — The engine **SHALL** abstract VR haptic output across OpenXR, PSVR2 Sense, and
    Quest Touch APIs.
    - **Rationale:** A common authoring format avoids per-platform asset duplication.
    - **Verification:** Play the same pattern on Oculus Touch and PSVR2 Sense. Assert both produce
      haptic output matching authored parameters.
