# R-13.25 — 3D Camera System Requirements

## Virtual Camera Framework

1. **R-13.25.1** — The engine **SHALL** provide a data-driven virtual camera framework where each
   camera behavior is an ECS entity with a numeric priority, with the highest-priority camera
   driving the rendered view through a Camera Brain component.
   - **Rationale:** Priority-based ECS camera entities enable data-driven switching without
     singleton managers.
   - **Verification:** Create three virtual cameras with priorities 1, 5, 10. Verify priority 10
     drives the view. Change priority 10 to 0 and verify priority 5 takes over.

2. **R-13.25.2** — The engine **SHALL** support configurable blend curves (cut, ease-in-out, linear,
   custom) between camera transitions, per-camera-pair custom blend rules, and weighted multi-camera
   blending of up to eight cameras.
   - **Rationale:** Smooth blending prevents jarring camera changes; per-pair rules enable unique
     transitions.
   - **Verification:** Trigger a camera transition and verify the configured blend curve
     interpolates position, rotation, FOV, and post-process. Verify custom per-pair rules override
     defaults.

## Position and Rotation Control

3. **R-13.25.3** — The engine **SHALL** provide position control behaviors including fixed-offset
   follow with per-axis damping, orbital follow driven by input, third-person shoulder with
   collision resolution, hard lock, adaptive framing, and spline dolly.
   - **Rationale:** Diverse position behaviors cover all common camera patterns across game genres.
   - **Verification:** Attach each behavior to a virtual camera tracking a moving target. Verify
     fixed-offset maintains the configured offset with damping. Verify orbital responds to input.
     Verify shoulder camera retracts on collision.

4. **R-13.25.4** — The engine **SHALL** provide rotation control behaviors including adaptive aim
   with dead/soft zones, hard look-at, input-driven pan/tilt with recentering, and follow-target
   rotation copy.
   - **Rationale:** Rotation behaviors complement position controls to cover aiming, look-at, and
     free-look patterns.
   - **Verification:** Verify adaptive aim only rotates when the target leaves the dead zone. Verify
     hard look-at centers the target every frame. Verify pan/tilt recenters after the configured
     wait time.

## Collision and Spring Arm

5. **R-13.25.5** — The engine **SHALL** provide a spring arm component that retracts on collision
   and extends when clear, a deoccluder preserving line of sight, and a decollider preventing
   geometry penetration.
   - **Rationale:** Camera collision handling prevents clipping and maintains subject visibility.
   - **Verification:** Place the camera behind a wall and verify the spring arm retracts. Move clear
     and verify extension. Verify the deoccluder repositions when an obstacle crosses the line of
     sight. Verify the decollider pushes the camera out of terrain.

## Camera Shake

6. **R-13.25.6** — The engine **SHALL** support multi-channel Perlin noise profiles, impulse-based
   shake with distance attenuation, wave oscillation, composite patterns, and
   cinematics-editor-driven keyframed shake.
   - **Rationale:** Varied shake types enable organic, event-driven, and precisely authored camera
     effects.
   - **Verification:** Trigger Perlin noise and verify multi-channel output varies across position
     and rotation axes. Trigger an impulse at distance D and verify attenuation reduces amplitude.
     Verify composite pattern layers multiple shake types additively.

## Camera Intelligence

7. **R-13.25.7** — The engine **SHALL** provide state-driven camera switching mapped to animation
   states, a clear-shot manager evaluating shot quality, and a timed camera playlist for
   attract-mode flyovers.
   - **Rationale:** Automatic camera selection reduces manual scripting for common patterns.
   - **Verification:** Map cameras to animation states and verify switching on state change. Occlude
     a camera's target and verify the clear-shot manager selects an unobstructed alternative. Define
     a 3-camera playlist with hold durations and verify timed cycling.

## Group and Extensions

8. **R-13.25.8** — The engine **SHALL** support target group aggregation for multi-target tracking
   with group framing that adjusts zoom/position to keep all members visible.
   - **Rationale:** Multi-target cameras are essential for co-op, party, and cinematic group shots.
   - **Verification:** Create a target group of 4 entities. Spread them apart and verify the camera
     zooms out. Move them together and verify zoom-in.

9. **R-13.25.9** — The engine **SHALL** provide camera extensions: 2D and 3D confiners, follow zoom,
   auto focus, third-person aim with parallax resolution, FreeLook modifier, recomposer, and a
   modifier stack.
   - **Rationale:** Extensions compose with base behaviors without modifying core camera logic.
   - **Verification:** Enable a 2D confiner and verify the camera stays within the polygon. Enable
     auto focus and verify focus distance tracks the target. Enable the aim extension and verify
     parallax correction reports the correct hit point.

## Cinematic Features

10. **R-13.25.10** — The engine **SHALL** support physical camera simulation (sensor size, focal
    length, aperture), pre-built dolly/crane rig entities, and picture-in-picture rendering from a
    secondary camera.
    - **Rationale:** Physical camera properties enable cinematic rendering matching real-world
      lenses.
    - **Verification:** Set sensor size and focal length and verify perspective projection changes.
      Attach a crane rig and verify constrained arm movement. Enable PiP and verify a secondary
      viewport renders at the configured position and resolution.

## Non-Functional Requirements

11. **R-13.25.NF1** — The engine **SHALL** evaluate all active virtual cameras and the camera brain
    within 0.5 ms per frame on a single thread.
    - **Rationale:** Camera evaluation must not consume significant frame budget alongside gameplay
      and rendering.
    - **Verification:** Create 16 active virtual cameras with various behaviors. Measure camera
      evaluation time and verify under 0.5 ms.

12. **R-13.25.NF2** — Camera input response (from input event to camera position/rotation change)
    **SHALL** complete within 1 frame at 60 fps.
    - **Rationale:** Camera lag makes aiming and navigation feel unresponsive.
    - **Verification:** Measure input-to-camera-update latency across 100 frames and verify all
      within 16.67 ms.
