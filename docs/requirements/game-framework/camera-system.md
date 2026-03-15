# R-13.25 — 3D Camera System Requirements

## Virtual Camera Framework

### R-13.25.1 Virtual Camera Entity and Priority System

The engine **SHALL** represent each camera behavior as an ECS entity with a `VirtualCamera`
component and a numeric priority, where the highest-priority camera is selected as the active view
source by the Camera Brain.

- **Derived from:** [F-13.25.1](../../features/game-framework/camera-system.md)
- **Rationale:** ECS-based virtual cameras enable data-driven camera management with runtime
  priority changes driven by gameplay events.
- **Verification:** Unit test: create 3 virtual cameras with priorities 1, 5, and 10. Verify the
  brain selects priority 10. Change priority 10 to 0 at runtime and verify the brain switches to
  priority 5.

### R-13.25.2 Camera Brain and Output Controller

The engine **SHALL** provide a per-player camera brain component that monitors active virtual
cameras, selects the highest-priority camera per output channel, drives blended transitions, and
supports configurable update timing (late update, fixed update, manual).

- **Derived from:** [F-13.25.2](../../features/game-framework/camera-system.md)
- **Rationale:** Per-player brains with channel masks enable split-screen without singleton camera
  managers.
- **Verification:** Unit test: assign two brains to different channels with one camera each. Verify
  each brain outputs the correct camera's view independently. Verify fixed-update timing
  synchronizes with physics.

## Position Control Behaviors

### R-13.25.3 Follow (Fixed Offset) Position Control

The engine **SHALL** provide a follow position behavior with six binding modes (world space,
lock-to-target, lock-to-target-no-roll, lock-with-world-up, lock-on-assign, lazy follow) and
per-axis position and angular damping.

- **Derived from:** [F-13.25.3](../../features/game-framework/camera-system.md)
- **Rationale:** Multiple binding modes cover the full range of third-person camera attachment
  styles without requiring custom code per game.
- **Verification:** Unit test: for each of the 6 binding modes, rotate the target 90 degrees and
  verify the camera offset transforms according to the mode's specification. Verify damping smooths
  position transitions over multiple frames.

### R-13.25.4 Orbital Follow Position Control

The engine **SHALL** provide an orbital follow behavior placing the camera on a sphere or three-ring
spline surface around a target, driven by player input axes with configurable ranges, wrapping, and
automatic recentering.

- **Derived from:** [F-13.25.4](../../features/game-framework/camera-system.md)
- **Rationale:** Orbital follow provides the standard third-person game camera with player control
  over viewing angle.
- **Verification:** Unit test: in sphere mode, verify input axes rotate the camera around the target
  at the configured radius. In three-ring mode, verify the camera follows the spline surface defined
  by three radii. Verify automatic recentering activates after the configured wait time.

### R-13.25.5 Third-Person Follow with Shoulder Offset

The engine **SHALL** provide a third-person follow behavior with a four-pivot rig (origin, shoulder,
hand, camera), left/right shoulder blending, vertical arm length, and built-in collision resolution
with separate damping for entering and recovering from collision.

- **Derived from:** [F-13.25.5](../../features/game-framework/camera-system.md)
- **Rationale:** The four-pivot rig with collision resolution is the standard over-the-shoulder
  camera required by action and shooter games.
- **Verification:** Unit test: set shoulder offset to left and verify the camera is positioned left
  of the character. Blend `camera_side` to right and verify smooth transition. Place an obstacle
  between camera and target and verify the camera retracts to maintain visibility.

### R-13.25.6 Hard Lock to Target

The engine **SHALL** provide a position control that copies the tracking target's world position
directly to the camera with no offset or damping.

- **Derived from:** [F-13.25.6](../../features/game-framework/camera-system.md)
- **Rationale:** First-person and cockpit cameras require zero-latency rigid attachment to the
  controlled entity.
- **Verification:** Unit test: move the target to 10 random positions over 10 frames and verify the
  camera position matches exactly each frame with zero offset.

### R-13.25.7 Position Composer (Adaptive Framing)

The engine **SHALL** provide a position composer that moves the camera to maintain a desired
screen-space position for the target, with dead zone, soft zone, hard limits, and lookahead
prediction.

- **Derived from:** [F-13.25.7](../../features/game-framework/camera-system.md)
- **Rationale:** Screen-space composition rules keep the target framed consistently regardless of
  world-space motion, essential for 2.5D and side-scrolling cameras.
- **Verification:** Unit test: move the target within the dead zone and verify zero camera movement.
  Move into the soft zone and verify gradual camera adjustment. Move past hard limits and verify
  immediate camera correction to keep the target on screen.

### R-13.25.8 Spline Dolly Position Control

The engine **SHALL** provide a spline dolly behavior constraining camera movement along Catmull-Rom
or Bezier spline paths with three position unit modes (knot, distance, normalized), automatic dolly
modes (none, fixed speed, nearest-point-to-target), and configurable rotation.

- **Derived from:** [F-13.25.8](../../features/game-framework/camera-system.md)
- **Rationale:** Spline-constrained cameras enable cinematic tracking shots and guided camera paths
  usable at gameplay time, not only in the sequencer.
- **Verification:** Unit test: create a 4-knot spline. In fixed-speed mode, verify the camera
  traverses at constant velocity. In nearest-point mode, move the target and verify the camera
  tracks the closest spline point. Verify all three position unit modes produce correct positions.

## Rotation Control Behaviors

### R-13.25.9 Rotation Composer (Adaptive Aim)

The engine **SHALL** provide a rotation composer that aims the camera at a look-at target with
screen-space composition (dead zone, soft zone, hard limits), target offset, damping, and lookahead
prediction, controlling only rotation.

- **Derived from:** [F-13.25.9](../../features/game-framework/camera-system.md)
- **Rationale:** Decoupling rotation from position control enables independent composition of aim
  and follow behaviors.
- **Verification:** Unit test: move the look-at target within the dead zone and verify no rotation
  change. Move into the soft zone and verify gradual re-aim. Verify position remains unchanged
  throughout.

### R-13.25.10 Hard Look At

The engine **SHALL** provide a rotation control that rigidly aims the camera at the look-at target's
position with no damping, dead zone, or composition offset.

- **Derived from:** [F-13.25.10](../../features/game-framework/camera-system.md)
- **Rationale:** Simple rigid aim is the baseline rotation behavior for cameras that must always
  center the target.
- **Verification:** Unit test: move the target to 10 random positions and verify the camera forward
  vector points exactly at the target each frame.

### R-13.25.11 Pan and Tilt (Input-Driven Rotation)

The engine **SHALL** provide an input-driven rotation behavior with configurable reference frame,
horizontal pan with range and wrap, vertical tilt clamped to [-90, 90] degrees, and automatic
recentering with configurable wait and completion time.

- **Derived from:** [F-13.25.11](../../features/game-framework/camera-system.md)
- **Rationale:** Input-driven rotation is required for first-person and free-look cameras where the
  player directly controls view direction.
- **Verification:** Unit test: send horizontal input and verify pan rotation. Send vertical input
  and verify tilt clamps at 90 degrees. Stop input and verify recentering activates after the
  configured wait time and completes within the configured duration.

### R-13.25.12 Rotate with Follow Target

The engine **SHALL** provide a rotation control that copies the tracking target's rotation to the
camera for vehicle and mounted-weapon cameras.

- **Derived from:** [F-13.25.12](../../features/game-framework/camera-system.md)
- **Rationale:** Vehicle and mounted cameras must match the target entity's facing direction without
  additional aim logic.
- **Verification:** Unit test: rotate the target through yaw, pitch, and roll and verify the camera
  rotation matches exactly each frame.

## Spring Arm and Collision

### R-13.25.13 Spring Arm Component

The engine **SHALL** provide a spring arm component that maintains a child entity at a target
distance, retracts via sphere sweep on collision, extends when clear, and supports per-axis rotation
inheritance, position/rotation lag, and sub-stepping for variable frame rates.

- **Derived from:** [F-13.25.13](../../features/game-framework/camera-system.md)
- **Rationale:** Spring arm collision avoidance prevents the camera from clipping through geometry,
  which is the most common third-person camera defect.
- **Verification:** Unit test: set arm length to 5 m. Place an obstacle at 3 m along the arm and
  verify the camera retracts to the obstacle surface. Remove the obstacle and verify the camera
  extends back to 5 m with configured lag speed. Verify rotation inheritance per axis.

### R-13.25.14 Camera Deoccluder (Line-of-Sight Preservation)

The engine **SHALL** provide a deoccluder extension that post-processes camera position to maintain
line of sight to the look-at target, with three repositioning strategies (pull forward, preserve
height, preserve distance), separate damping rates, and minimum occlusion time.

- **Derived from:** [F-13.25.14](../../features/game-framework/camera-system.md)
- **Rationale:** Line-of-sight preservation ensures the player can always see their character, which
  is a fundamental usability requirement for third-person cameras.
- **Verification:** Unit test: for each strategy, place an obstacle between camera and target and
  verify the camera repositions to restore line of sight. Verify brief obstructions shorter than
  minimum occlusion time are ignored. Verify damping-when-occluded differs from standard damping.

### R-13.25.15 Camera Decollider (Geometry Penetration Prevention)

The engine **SHALL** provide a decollider extension that prevents the camera from penetrating solid
geometry using terrain resolution (ray cast downward, push above surface) and obstacle resolution
(detect overlap, push toward target), with damping and smoothing.

- **Derived from:** [F-13.25.15](../../features/game-framework/camera-system.md)
- **Rationale:** Decollision prevents the camera from showing the inside of geometry, which breaks
  immersion and can expose hidden areas.
- **Verification:** Unit test: position the camera inside a solid volume and verify the decollider
  pushes it outside within one frame. Position the camera below terrain and verify it pushes above.
  Verify smoothing time prevents jitter when the camera oscillates near a surface.

## Camera Blending and Transitions

### R-13.25.16 Camera Blend System

The engine **SHALL** provide camera blending with eight blend curve types (cut, ease-in-out,
ease-in, ease-out, hard-in, hard-out, linear, custom), per-camera-pair custom blend assets with
wildcard matching and specificity-based conflict resolution, interpolating position, rotation, FOV,
and post-process settings.

- **Derived from:** [F-13.25.16](../../features/game-framework/camera-system.md)
- **Rationale:** Per-pair blend rules enable cinematic transitions tailored to specific camera
  changes without a one-size-fits-all approach.
- **Verification:** Unit test: define a custom blend asset with an exact pair rule and a wildcard
  rule. Trigger both transitions and verify the exact pair rule takes precedence. Verify all 8 curve
  types produce distinct interpolation curves over a 1-second blend.

### R-13.25.17 Camera Mixing (Weighted Multi-Camera Blend)

The engine **SHALL** blend up to eight child virtual cameras simultaneously using per-camera
weights, interpolating position, rotation, FOV, and post-process settings proportionally, with
weights animatable via the sequencer timeline.

- **Derived from:** [F-13.25.17](../../features/game-framework/camera-system.md)
- **Rationale:** Weighted multi-camera blending enables smooth multi-perspective shots and gradual
  transitions that cannot be achieved with binary camera switching.
- **Verification:** Unit test: set up 3 cameras with weights 1, 2, 1. Verify the output position is
  the weighted average. Animate one weight from 0 to 1 and verify a smooth transition into the
  blend.

## Camera Shake and Noise

### R-13.25.18 Multi-Channel Perlin Noise Profiles

The engine **SHALL** provide multi-channel Perlin noise camera shake with artist-authored noise
profile assets defining per-channel (position X/Y/Z, rotation pitch/yaw/roll) behavior, runtime
amplitude and frequency gain multipliers, and built-in preset profiles.

- **Derived from:** [F-13.25.18](../../features/game-framework/camera-system.md)
- **Rationale:** Structured noise profiles produce organic camera motion (handheld feel) that simple
  single-parameter shake cannot achieve.
- **Verification:** Unit test: load a handheld preset profile and verify all 6 channels produce
  non-zero output. Set amplitude gain to 0 and verify all channels output zero. Verify frequency
  gain 2x doubles the oscillation rate compared to 1x.

### R-13.25.19 Camera Impulse System

The engine **SHALL** provide an event-driven impulse system where impulse sources emit directional
shake signals with strength-over-time curves and propagation radius, and impulse listeners on
virtual cameras react with distance-attenuated positional and rotational shake.

- **Derived from:** [F-13.25.19](../../features/game-framework/camera-system.md)
- **Rationale:** Distance-attenuated impulses produce physically plausible shake where nearby
  explosions are felt more strongly than distant ones.
- **Verification:** Unit test: emit an impulse at origin with radius 50 m. Place two cameras at 10 m
  and 40 m. Verify the closer camera receives higher shake amplitude. Place a camera at 60 m and
  verify it receives no shake. Verify two simultaneous impulses composite additively.

### R-13.25.20 Wave Oscillation Shake

The engine **SHALL** provide sinusoidal wave oscillation camera shake with per-axis amplitude and
frequency for position, rotation, and FOV, configurable initial offset, blend-in/out times, and
finite or infinite duration.

- **Derived from:** [F-13.25.20](../../features/game-framework/camera-system.md)
- **Rationale:** Sinusoidal oscillation produces smooth periodic motion suitable for low-frequency
  effects like rocking boats or breathing idle animations.
- **Verification:** Unit test: configure a 1 Hz sine wave on pitch with amplitude 5 degrees. Verify
  the output oscillates between -5 and +5 degrees at 1 Hz. Verify blend-in ramps amplitude from 0 to
  full over the configured duration.

### R-13.25.21 Composite Shake Patterns

The engine **SHALL** support layering multiple shake types (Perlin noise, wave oscillation,
sequencer-driven) into a single composite pattern where each layer contributes additively.

- **Derived from:** [F-13.25.21](../../features/game-framework/camera-system.md)
- **Rationale:** Composite patterns enable complex shake effects combining organic noise with
  predictable oscillation and authored animation.
- **Verification:** Unit test: create a composite with a Perlin layer and a wave layer. Verify the
  output equals the sum of both layers' individual outputs each frame.

### R-13.25.22 Sequencer-Driven Camera Shake

The engine **SHALL** support camera shake driven by keyframed animation sequences from the
sequencer, with loop, blend-in/out, and layering with procedural shake.

- **Derived from:** [F-13.25.22](../../features/game-framework/camera-system.md)
- **Rationale:** Authored shake provides precise artistic control for scripted moments where
  procedural noise lacks choreographic precision.
- **Verification:** Unit test: author a 1-second shake sequence with known keyframes. Play it and
  verify the camera offsets match the keyframed values at each sample point. Layer with a Perlin
  noise shake and verify additive composition.

## Camera Intelligence

### R-13.25.23 State-Driven Camera Switching

The engine **SHALL** provide a camera manager that maps animation states to virtual cameras,
automatically activating the mapped camera on state transitions with per-mapping wait time, minimum
duration, and configurable blend assets.

- **Derived from:** [F-13.25.23](../../features/game-framework/camera-system.md)
- **Rationale:** Animation-state-driven switching enables automatic camera behavior changes (idle,
  run, combat, climb) without manual gameplay scripting.
- **Verification:** Unit test: map 3 animation states to 3 cameras. Transition between states and
  verify the correct camera activates. Verify wait time delays activation. Verify minimum duration
  prevents premature deactivation on rapid state changes.

### R-13.25.24 Clear Shot (Automatic Best-Camera Selection)

The engine **SHALL** provide a clear shot camera manager that evaluates child cameras for target
visibility and distance, activating the camera with the best unobstructed view, with activate-after
delay, minimum duration, and custom blend rules.

- **Derived from:** [F-13.25.24](../../features/game-framework/camera-system.md)
- **Rationale:** Automatic best-camera selection reduces manual camera scripting for scenes with
  multiple viewpoints and moving obstacles.
- **Verification:** Unit test: set up 3 child cameras viewing a target. Occlude camera 1's view and
  verify the manager switches to the highest-scoring unoccluded camera. Verify activate- after delay
  prevents switching for brief occlusions.

### R-13.25.25 Shot Quality Evaluator

The engine **SHALL** provide a shot quality evaluator extension that scores virtual cameras on a
normalized 0-1 scale based on target visibility (raycast occlusion) and distance (near/far limits,
optimal distance), with layer mask and ignore tag filtering.

- **Derived from:** [F-13.25.25](../../features/game-framework/camera-system.md)
- **Rationale:** Quantified shot quality enables automated camera selection and is queryable by
  custom gameplay camera logic.
- **Verification:** Unit test: place a camera with clear line of sight at optimal distance and
  verify score near 1.0. Occlude the target and verify score drops. Move the camera outside the
  near/far limits and verify score decreases proportionally.

### R-13.25.26 Camera Sequencer (Timed Camera Playlist)

The engine **SHALL** provide a camera sequencer that plays an ordered list of child cameras with
per-camera hold duration, blend transitions, and loop/hold-final modes.

- **Derived from:** [F-13.25.26](../../features/game-framework/camera-system.md)
- **Rationale:** Timed camera playlists enable attract-mode flyovers and showcase sequences without
  requiring the full sequencer timeline.
- **Verification:** Unit test: create a playlist of 3 cameras with 2-second holds. Verify cameras
  activate in order at the correct times. In loop mode, verify the sequence restarts. In non-loop
  mode, verify the final camera holds indefinitely.

## Group and Multi-Target

### R-13.25.27 Target Group (Multi-Target Aggregation)

The engine **SHALL** aggregate multiple tracking targets into a single virtual target with two
position modes (bounding box center, weighted average), per-member weight and radius, and
assignability as the tracking target of any virtual camera.

- **Derived from:** [F-13.25.27](../../features/game-framework/camera-system.md)
- **Rationale:** Multi-target aggregation enables cameras to track groups of entities (party
  members, combatants) without per-camera multi-target logic.
- **Verification:** Unit test: create a target group with 3 members. In group-center mode, verify
  the position equals the AABB center. In weighted-average mode, assign weights 1, 2, 1 and verify
  the position is the weighted average. Add a member and verify the group updates.

### R-13.25.28 Group Framing Extension

The engine **SHALL** provide a group framing extension that adjusts zoom and position to keep all
target group members on screen, with framing mode (horizontal, vertical, best fit), size adjustment
(zoom, dolly, dolly-then-zoom), and configurable screen-space occupancy.

- **Derived from:** [F-13.25.28](../../features/game-framework/camera-system.md)
- **Rationale:** Automatic group framing keeps all relevant entities visible as they spread apart or
  converge, essential for local multiplayer and group combat cameras.
- **Verification:** Unit test: spread 3 group members apart and verify the camera adjusts FOV or
  distance to keep all members within the configured screen-space occupancy. Move members closer and
  verify the camera zooms in. Verify framing respects min/max FOV constraints.

## Camera Extensions

### R-13.25.29 Camera Confiner 2D

The engine **SHALL** constrain the 2D camera within a polygon boundary by computing a secondary
polygon from camera view size and aspect ratio, with damping, slowing distance, and cache
invalidation on polygon change.

- **Derived from:** [F-13.25.29](../../features/game-framework/camera-system.md)
- **Rationale:** Hard boundary enforcement prevents the 2D camera from revealing areas outside the
  playable level.
- **Verification:** Unit test: define a rectangular boundary. Move the camera toward an edge and
  verify it stops before the screen edge exceeds the boundary. Verify slowing distance creates
  gradual deceleration near edges. Modify the polygon and verify the cache invalidates.

### R-13.25.30 Camera Confiner 3D

The engine **SHALL** restrict 3D camera position to a bounding volume with a slowing distance
deceleration zone at the boundary for smooth stops.

- **Derived from:** [F-13.25.30](../../features/game-framework/camera-system.md)
- **Rationale:** 3D confinement keeps the camera within playable areas, arena boundaries, or
  interior spaces.
- **Verification:** Unit test: define a box volume. Move the camera toward the boundary and verify
  it decelerates within the slowing distance and cannot exit the volume.

### R-13.25.31 Follow Zoom (Constant Screen-Size Framing)

The engine **SHALL** dynamically adjust camera FOV to maintain a constant on-screen size for the
tracking target, with configurable target width, damping, and min/max FOV range.

- **Derived from:** [F-13.25.31](../../features/game-framework/camera-system.md)
- **Rationale:** Constant screen-size framing keeps the subject consistently sized regardless of
  distance changes, important for tracking shots.
- **Verification:** Unit test: move the target from 10 m to 50 m distance. Verify the camera adjusts
  FOV to maintain the target's screen-space width within tolerance. Verify FOV clamps at the
  configured min/max values.

### R-13.25.32 Auto Focus

The engine **SHALL** control camera focus distance for depth-of-field effects with five target modes
(look-at target, follow target, custom target, camera-relative, screen center), focus depth offset,
and damping.

- **Derived from:** [F-13.25.32](../../features/game-framework/camera-system.md)
- **Rationale:** Automatic focus tracking produces cinematic depth-of-field without manual focus
  distance keyframing.
- **Verification:** Unit test: in look-at-target mode, move the target and verify focus distance
  tracks the target's distance from the camera. In camera-relative mode, set a manual distance and
  verify focus is fixed. Verify damping smooths focus transitions.

### R-13.25.33 Third-Person Aim Extension

The engine **SHALL** project a ray from the camera along its forward axis to detect aim targets,
resolve third-person parallax between camera and weapon origin, and cancel rotational noise while
preserving positional noise for aim stability during camera shake.

- **Derived from:** [F-13.25.33](../../features/game-framework/camera-system.md)
- **Rationale:** Parallax resolution ensures third-person shooter players hit what the crosshair
  indicates, not what the character's weapon physically points at.
- **Verification:** Unit test: aim at a target with a lateral offset between camera and weapon
  origin. Verify the computed hit point matches the camera crosshair, not the weapon ray. Apply
  camera shake and verify the aim target remains stable on screen (rotational noise cancelled).

### R-13.25.34 FreeLook Modifier

The engine **SHALL** dynamically adjust camera settings (FOV, noise amplitude/frequency, damping,
composition, distance) based on the FreeLook orbital position (top, middle, bottom) with
configurable easing.

- **Derived from:** [F-13.25.34](../../features/game-framework/camera-system.md)
- **Rationale:** Orbit-dependent settings enable different camera feels at different viewing angles
  (wider FOV looking down, tighter FOV at eye level).
- **Verification:** Unit test: orbit to the top position and verify FOV matches the configured top
  value. Orbit to the bottom and verify FOV matches the bottom value. Verify the easing parameter
  controls blending smoothness through the center point.

### R-13.25.35 Recomposer (Timeline Composition Override)

The engine **SHALL** apply animatable composition overrides (tilt, pan, dutch, zoom scale,
follow/look-at attachment) as a final post-processing stage on the camera output, with configurable
processing order.

- **Derived from:** [F-13.25.35](../../features/game-framework/camera-system.md)
- **Rationale:** Post-hoc composition overrides enable hand-tuned adjustments on top of procedural
  camera motion without modifying the underlying behavior.
- **Verification:** Unit test: apply a 10-degree tilt override and verify the output rotation
  differs from the base camera by exactly 10 degrees. Animate dutch from 0 to 15 degrees and verify
  smooth roll transition. Set follow attachment to 0 and verify the camera releases the follow
  target.

### R-13.25.36 Camera Modifier Stack

The engine **SHALL** provide an extensible modifier pipeline on the camera brain that applies
modifiers in priority order to adjust position, rotation, FOV, clip planes, and post-process
settings, with built-in FOV override, post-process blend, and lens effect modifiers.

- **Derived from:** [F-13.25.36](../../features/game-framework/camera-system.md)
- **Rationale:** A modifier pipeline enables gameplay systems (sprint FOV, ADS zoom, hit effects) to
  layer camera adjustments without modifying virtual camera definitions.
- **Verification:** Unit test: add two modifiers at priorities 1 and 2. Verify they execute in
  priority order. Apply a FOV override modifier during sprint and verify FOV smoothly transitions to
  the sprint value and back.

## Input Integration

### R-13.25.37 Camera Input Axis Controller

The engine **SHALL** bridge player input to camera axis parameters with per-player-index routing,
acceleration/deceleration tuning, gain multiplier, delta-time compensation, input suppression during
blending, and unscaled delta-time support for paused time scale.

- **Derived from:** [F-13.25.37](../../features/game-framework/camera-system.md)
- **Rationale:** Frame-rate-independent input processing with blend suppression prevents jarring
  camera jumps during transitions and paused states.
- **Verification:** Unit test: send identical input at 30 fps and 120 fps and verify camera rotation
  matches within tolerance (frame-rate independence). Trigger a camera blend and verify input is
  suppressed during the blend duration. Pause time scale and verify unscaled delta-time mode still
  processes input.

## Cinematic Camera Features

### R-13.25.38 Cine Camera Properties

The engine **SHALL** provide physical camera simulation with real-world parameters (sensor size,
focal length, aperture, focus distance, filmback) driving depth-of-field, exposure, and projection,
with lens distortion profiles for barrel/pincushion simulation.

- **Derived from:** [F-13.25.38](../../features/game-framework/camera-system.md)
- **Rationale:** Physical camera parameters enable cinematographers to apply real-world lens
  knowledge for cinematic rendering.
- **Verification:** Unit test: set sensor size to Super 35 and focal length to 50 mm. Verify the
  computed vertical FOV matches the expected value. Apply a barrel distortion profile and verify
  rendered output exhibits radial distortion.

### R-13.25.39 Camera Rig Presets (Dolly, Crane, Jib)

The engine **SHALL** provide pre-built camera rig entities (dolly on track, crane/jib on arm) with
configurable arm length, pivot height, sweep range, and keyframeable rig parameters integrated with
the sequencer.

- **Derived from:** [F-13.25.39](../../features/game-framework/camera-system.md)
- **Rationale:** Pre-built rigs simulate physical camera equipment, enabling cinematic camera
  workflows without custom scripting.
- **Verification:** Unit test: place a crane rig and animate arm length from 2 m to 8 m. Verify the
  camera sweeps along the expected arc. Verify dolly rig constrains movement to the track path.

### R-13.25.40 Picture-in-Picture

The engine **SHALL** render secondary camera views into viewport insets with configurable position,
size, border, opacity, resolution scale, and support for multiple simultaneous PiP viewports.

- **Derived from:** [F-13.25.40](../../features/game-framework/camera-system.md)
- **Rationale:** PiP viewports enable rear-view mirrors, security camera feeds, minimaps, and
  spectator views without full-screen camera switching.
- **Verification:** Integration test: create two PiP viewports at different screen positions with
  different virtual cameras. Verify both render correctly at the configured resolution scale. Verify
  opacity and border style are applied. Verify the main view is unaffected.

## Non-Functional Requirements

### NFR-13.25.1 Camera System Frame Budget

Total camera system processing (virtual camera evaluation, brain selection, blending, collision,
extensions, modifier stack) **SHALL** complete within 1ms per brain per frame. With split-screen (up
to 4 brains), total camera budget **SHALL NOT** exceed 4ms.

- **Rationale:** Camera processing runs every frame and must not consume a significant portion of
  the frame budget, especially in split-screen scenarios.
- **Verification:** Configure 4 camera brains with active virtual cameras, blending, collision, and
  extensions. Measure total camera processing time. Verify it stays under 4ms.

### NFR-13.25.2 Camera Blend Smoothness

Camera blends **SHALL** produce output positions, rotations, and FOV values with no visible
discontinuities (position jumps > 0.01 world units or rotation jumps > 0.1 degrees between
consecutive frames). Blend curves **SHALL** be evaluated at sub-frame precision using interpolation,
not per-frame stepping.

- **Rationale:** Visible camera jumps during blends cause motion sickness and break immersion.
  Sub-frame evaluation ensures smooth transitions at all frame rates.
- **Verification:** Trigger a 1-second blend between two cameras at 30 fps and 120 fps. Record
  per-frame output values. Verify no position delta exceeds 0.01 units and no rotation delta exceeds
  0.1 degrees between consecutive frames at either frame rate.
