# 13.25 — 3D Camera System

## Virtual Camera Framework

### F-13.25.1 Virtual Camera Entity and Priority System

A data-driven virtual camera framework where each camera behavior is an ECS entity with a
`VirtualCamera` component. Multiple virtual cameras can exist simultaneously; the active camera is
selected by the Camera Brain (F-13.25.2) based on a numeric priority value. Higher priority cameras
override lower ones. Priority can be set statically in data or modified at runtime by gameplay
events (entering a trigger volume, starting combat, mounting a vehicle). Virtual cameras are
lightweight descriptions of desired camera behavior — they do not render independently but feed
computed position, rotation, FOV, and post-process parameters to the Camera Brain for final output.

- **Requirements:** R-13.25.1
- **Dependencies:** F-1.1.1 (ECS), F-1.5.1 (Typed Event Channels)
- **Platform notes:** None

**Gap source:** Unity CineMachine 3 virtual camera priority system; UE5 Gameplay Cameras plugin
camera director/rig selection.

### F-13.25.2 Camera Brain and Output Controller

A singleton-free camera manager component attached to each player's view entity that monitors all
active virtual cameras, selects the highest-priority camera matching the player's output channel,
and drives the final rendered view. The brain computes blended transitions between outgoing and
incoming cameras using configurable blend curves (F-13.25.15). Multiple brains can coexist for
split-screen (each brain reads from a different channel mask). Update timing is configurable: late
update (default), fixed update (for physics sync), or manual update (for replay/sequencer control).

- **Requirements:** R-13.25.2
- **Dependencies:** F-13.25.1, F-2.10.4 (View Setup)
- **Platform notes:** None

**Gap source:** Unity CineMachine Brain; UE5 PlayerCameraManager.

## Position Control Behaviors

### F-13.25.3 Follow (Fixed Offset)

Position control behavior that maintains a fixed offset from a tracking target in a configurable
coordinate space. Six binding modes control how the offset relates to target rotation: world space
(offset ignores rotation), lock-to-target (offset rotates with target), lock-to-target-no-roll,
lock-with-world-up (yaw only), lock-on-assign (snapshot orientation at activation), and lazy follow
(minimizes movement while maintaining distance). Per-axis position damping and angular damping
(quaternion or Euler) smooth the camera's response to target motion.

- **Requirements:** R-13.25.3
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineFollow` with 6 binding modes.

### F-13.25.4 Orbital Follow

Position control behavior that places the camera on a sphere or three-ring surface around the
tracking target, driven by player input axes. In sphere mode, a single radius defines camera
distance; horizontal and vertical axes control yaw and pitch. In three-ring mode, three configurable
orbit radii (top, middle, bottom) define a spline-extruded surface for more complex orbit shapes.
Each axis (horizontal, vertical, radial) supports configurable ranges, wrapping, and automatic
recentering with wait time and completion time. Integrates with the input system (F-6.2.1) for
real-time player control of orbit position. Target offset fine-tunes the orbit center relative to
the tracked entity.

- **Requirements:** R-13.25.4
- **Dependencies:** F-13.25.1, F-6.2.1 (Input Actions)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineOrbitalFollow`; UE5 Gameplay Cameras boom arm node.

### F-13.25.5 Third-Person Follow with Shoulder Offset

Position control behavior for over-the-shoulder and third-person cameras. Uses a four-pivot rig:
origin (target position), shoulder (lateral offset from origin), hand (vertical offset from
shoulder), and camera (behind hand, always faces hand). Shoulder offset places the camera left or
right of the character with a `camera_side` parameter that blends between shoulders. Vertical arm
length controls how much the camera drops or rises relative to the shoulder. Built-in collision
resolution adjusts camera distance when obstacles occlude the target, with separate damping rates
for moving into and recovering from collision. Camera radius defines minimum distance from
obstacles. Collision layer mask and ignore tag filter which geometry participates in collision
checks.

- **Requirements:** R-13.25.5
- **Dependencies:** F-13.25.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineThirdPersonFollow`.

### F-13.25.6 Hard Lock to Target

Position control that copies the tracking target's world position directly to the camera with no
offset or damping. Used for first-person cameras, vehicle cockpit views, or any case where the
camera must be rigidly attached to an entity.

- **Requirements:** R-13.25.6
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineHardLockToTarget`.

### F-13.25.7 Position Composer (Adaptive Framing)

Position control behavior that moves the camera to maintain a desired screen-space position for the
tracking target. Defines dead zone (no camera movement), soft zone (gradual camera adjustment with
damping), and hard limits (target cannot leave frame). Screen position parameters place the target
at a configurable point on screen (center, rule-of-thirds, custom offset). Lookahead time and
smoothing predict target movement to reduce lag. Camera distance and dead zone depth control the
Z-axis relationship. Visual guides in the editor show dead zone (clear), soft zone (blue), and hard
limits (red).

- **Requirements:** R-13.25.7
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachinePositionComposer`.

### F-13.25.8 Spline Dolly

Position control behavior that constrains the camera to move along a predefined spline path. Three
position unit modes: knot index, distance from start, and normalized (0-1). Automatic dolly modes:
none (manual control), fixed speed (constant velocity traversal), and nearest-point-to-target
(camera tracks closest spline point to the tracking target). Camera rotation options: default,
follow path tangent, follow path without roll, match tracking target orientation, match target
without roll. Per-axis position damping and angular damping smooth movement along the spline.
Supports Catmull-Rom and Bezier splines.

- **Requirements:** R-13.25.8
- **Dependencies:** F-13.25.1, F-13.5.3 (Camera Rails and Splines)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineSplineDolly`. Harmonius has cinematic spline paths
(F-13.5.3) but lacks a gameplay-time spline dolly usable outside the sequencer.

## Rotation Control Behaviors

### F-13.25.9 Rotation Composer (Adaptive Aim)

Rotation control behavior that rotates the camera to face the look-at target with adaptive
composition rules. Target offset adjusts the aim point in target-local space. Screen position places
the target at a configurable horizontal/vertical position. Dead zone, soft zone, and hard limits
control when and how aggressively the camera re-aims. Damping controls rotational responsiveness.
Lookahead time and smoothing predict target movement for anticipatory aiming. Only controls
rotation, never position.

- **Requirements:** R-13.25.9
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineRotationComposer`.

### F-13.25.10 Hard Look At

Rotation control that keeps the camera rigidly aimed at the look-at target's position with no
damping, dead zone, or composition offset. The target is always centered in frame.

- **Requirements:** R-13.25.10
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineHardLookAt`.

### F-13.25.11 Pan and Tilt (Input-Driven Rotation)

Rotation control behavior that rotates the camera based on player input axes. Configurable reference
frame (parent object, world, tracking target, look-at target). Pan axis (horizontal) has
configurable range, wrap, and center. Tilt axis (vertical) is clamped to [-90, 90] degrees with
optional wrap. Automatic recentering with configurable wait time and completion time, targeting axis
center, tracking target forward, or look-at target forward. Works with the input axis controller
(F-6.2.1) or custom input drivers.

- **Requirements:** R-13.25.11
- **Dependencies:** F-13.25.1, F-6.2.1 (Input Actions)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachinePanTilt`.

### F-13.25.12 Rotate with Follow Target

Rotation control that copies the tracking target's rotation to the camera. Used when the camera
should match the target's facing direction (vehicle cameras, mounted weapon cameras).

- **Requirements:** R-13.25.12
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineRotateWithFollowTarget`.

## Spring Arm and Collision

### F-13.25.13 Spring Arm Component

A positioning component that maintains child entities (typically a camera) at a fixed distance from
a parent, retracting when collision is detected and extending back when clear. Properties: target
arm length (natural distance), probe size (collision sphere radius), probe channel (collision
layer), socket offset (offset at arm end), target offset (offset at arm start in world space).
Collision test uses sphere sweep along the arm direction. Position lag with configurable speed, max
distance, and max time step for sub-stepping across variable frame rates. Rotation lag with
configurable speed. Selective rotation inheritance: per-axis control of pitch, roll, and yaw
inheritance from parent. Option to use pawn control rotation. Debug visualization of lag markers.

- **Requirements:** R-13.25.13
- **Dependencies:** F-1.9.4 (Spatial Query), F-1.1.1 (ECS)
- **Platform notes:** None

**Gap source:** UE5 `USpringArmComponent`.

### F-13.25.14 Camera Deoccluder (Line-of-Sight Preservation)

Extension that post-processes the final camera position to maintain line of sight between camera and
look-at target. Uses physics raycasting to detect obstacles. Three repositioning strategies: pull
camera forward (move along Z-axis ahead of nearest obstacle), preserve camera height (relocate while
maintaining original height), and preserve camera distance (relocate while maintaining original
distance from target). Properties: camera radius (minimum distance from obstacles), distance limit
(max raycast range), damping and damping-when-occluded (separate response speeds), minimum occlusion
time (ignore brief obstructions), ignore tag (exclude specific entities), and obstacle layer mask.

- **Requirements:** R-13.25.14
- **Dependencies:** F-13.25.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineDeoccluder`.

### F-13.25.15 Camera Decollider (Geometry Penetration Prevention)

Extension that prevents the camera from entering solid geometry. Two resolution algorithms: terrain
resolution (ray cast downward, push camera above terrain surface) and obstacle resolution (detect
overlapping colliders, push camera toward target to escape). Properties: camera radius, obstacle
layers, terrain layers, damping (return speed), and smoothing time (hold position briefly to reduce
jitter). Distinct from the deoccluder — the decollider prevents the camera itself from clipping into
geometry, while the deoccluder preserves line of sight to the target.

- **Requirements:** R-13.25.15
- **Dependencies:** F-13.25.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineDecollider`.

## Camera Blending and Transitions

### F-13.25.16 Camera Blend System

Configurable blending between virtual cameras when the active camera changes. Eight blend curve
types: cut (instant), ease-in-out (S-curve), ease-in, ease-out, hard-in, hard-out, linear, and
custom (user-defined animation curve). Blend duration in seconds. A custom blends asset defines
per-camera-pair transition rules using camera names with an "any camera" wildcard. Specificity
hierarchy resolves conflicts: exact camera-pair match wins over wildcard matches. Default blend
fallback when no custom blend matches. Blending interpolates position, rotation, FOV, and
post-process settings between outgoing and incoming cameras.

- **Requirements:** R-13.25.16
- **Dependencies:** F-13.25.2
- **Platform notes:** None

**Gap source:** Unity CineMachine blend system with 8 curve types; UE5 `SetViewTargetWithBlend`.

### F-13.25.17 Camera Mixing (Weighted Multi-Camera Blend)

Simultaneous blending of up to eight child virtual cameras using weighted averaging. Each child
contributes its position, rotation, FOV, and post-process settings proportionally to its weight
divided by the total weight sum. Weights are animatable via the sequencer timeline. Used for smooth
multi-perspective shots and gradual camera transitions that combine properties from several
viewpoints.

- **Requirements:** R-13.25.17
- **Dependencies:** F-13.25.1, F-13.5.1 (Sequencer)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineMixingCamera`.

## Camera Shake and Noise

### F-13.25.18 Multi-Channel Perlin Noise Profiles

Procedural camera noise using multi-channel Perlin noise with artist-authored noise profile assets.
Each profile defines noise behavior over time across multiple channels (position X/Y/Z, rotation
pitch/yaw/roll). Runtime parameters: amplitude gain (multiplier on profile intensity, 0 = muted),
frequency gain (multiplier on profile speed), and pivot offset (positional variation corresponding
to rotational noise). Amplitude and frequency gains are animatable for dynamic intensity ramps.
Includes built-in preset profiles (handheld, breathe, tripod vibration) and supports custom profile
authoring. Extends the existing screen shake (F-11.3.1) with structured noise profiles rather than
single-parameter Perlin noise.

- **Requirements:** R-13.25.18
- **Dependencies:** F-11.3.1 (Screen Shake)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineBasicMultiChannelPerlin`; UE5 Perlin noise shake
pattern.

### F-13.25.19 Camera Impulse System

Event-driven camera shake where impulse sources emit signals that propagate outward from a
world-space origin like a shockwave, and impulse listeners on virtual cameras react with positional
and rotational shake. Two source types: manual impulse source (triggered by gameplay events —
explosions, impacts, abilities) and collision impulse source (triggered automatically by rigid body
collisions). Impulse signals have a direction, strength-over-time curve, duration, and propagation
radius. Listener settings control gain, frequency filtering, and signal combination mode (add,
multiply, max). Distance attenuation reduces shake intensity for cameras far from the impulse
origin. Multiple simultaneous impulses are composited additively with configurable amplitude
clamping.

- **Requirements:** R-13.25.19
- **Dependencies:** F-13.25.1, F-1.5.1 (Typed Event Channels)
- **Platform notes:** None

**Gap source:** Unity CineMachine Impulse system (source + listener); UE5 `PlayWorldCameraShake`
with epicenter and attenuation radii.

### F-13.25.20 Wave Oscillation Shake

Camera shake pattern using sinusoidal wave oscillation with smooth periodic motion. Configurable
per-axis amplitude and frequency for location (X, Y, Z), rotation (pitch, yaw, roll), and FOV.
Initial offset type (zero or random) varies the waveform start point. Useful for low-frequency
effects: rocking boats, dream sequences, idle breathing. Blend-in and blend-out times for smooth
activation and deactivation. Duration (finite or infinite for continuous effects).

- **Requirements:** R-13.25.20
- **Dependencies:** F-11.3.1 (Screen Shake)
- **Platform notes:** None

**Gap source:** UE5 wave oscillation shake pattern.

### F-13.25.21 Composite Shake Patterns

Shake pattern that layers multiple shake types (Perlin noise, wave oscillation, sequencer-driven
animation) into a single composite effect. Each layer runs independently with its own parameters and
contributes additively to the final shake. Allows creating varied, complex shake effects that
combine the organic feel of Perlin noise with the predictability of wave oscillation and the
precision of authored animation.

- **Requirements:** R-13.25.21
- **Dependencies:** F-13.25.18, F-13.25.20
- **Platform notes:** None

**Gap source:** UE5 composite camera shake pattern.

### F-13.25.22 Sequencer-Driven Camera Shake

Camera shake pattern driven by a camera animation sequence asset authored in the sequencer
(F-13.5.1). Provides full artistic control over shake motion by keyframing position and rotation
offsets. Used for precisely choreographed shake effects (scripted explosions, cutscene impacts)
where procedural noise lacks the necessary precision. Animation sequences can be looped, blended
in/out, and layered with procedural shake.

- **Requirements:** R-13.25.22
- **Dependencies:** F-13.5.1 (Sequencer), F-11.3.1 (Screen Shake)
- **Platform notes:** None

**Gap source:** UE5 sequence camera shake pattern.

## Camera Intelligence

### F-13.25.23 State-Driven Camera Switching

A camera manager that monitors an animation state machine on a target entity and automatically
activates a mapped virtual camera when the animation state changes. State-to-camera mapping
associates animation states with virtual cameras. Per-mapping wait time delays activation to avoid
flickering on transient states. Per-mapping minimum duration prevents premature deactivation. When
multiple cameras map to the same state, the highest priority camera is selected. Blending between
cameras on state change uses configurable default and custom blend assets. Standby update frequency
controls how non-active cameras are updated.

- **Requirements:** R-13.25.23
- **Dependencies:** F-13.25.1, F-9.4.1 (Animation State Machine)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineStateDrivenCamera`.

### F-13.25.24 Clear Shot (Automatic Best-Camera Selection)

A camera manager that evaluates shot quality among child virtual cameras and automatically activates
the one with the best unobstructed view. Quality evaluation uses physics raycasting to detect target
occlusion and optionally scores target distance against optimal range. When multiple cameras score
equally, either the highest-priority camera or a random selection is used. Properties:
activate-after delay (seconds before switching), minimum duration (prevent rapid switching), default
blend, and custom blends. Child cameras use the shot quality evaluator extension (F-13.25.25) and/or
the deoccluder (F-13.25.14) to compute their quality scores.

- **Requirements:** R-13.25.24
- **Dependencies:** F-13.25.1, F-13.25.25
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineClearShot`.

### F-13.25.25 Shot Quality Evaluator

Extension that scores a virtual camera's shot quality based on target visibility (raycast occlusion
check) and target distance (near/far limits, optimal distance). Quality score is a normalized 0-1
value. High- quality shots receive a configurable bonus multiplier. Occlusion detection uses layer
masks and ignore tags. Used by the Clear Shot manager (F-13.25.24) and available to gameplay logic
for custom camera selection.

- **Requirements:** R-13.25.25
- **Dependencies:** F-13.25.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineShotQualityEvaluator`.

### F-13.25.26 Camera Sequencer (Timed Camera Playlist)

A camera manager that plays through an ordered list of child virtual cameras with configurable hold
duration per camera. Transitions between cameras use the blend system (F-13.25.16). Loop mode
restarts the sequence from the beginning; non-loop mode holds the final camera indefinitely. Used
for attract-mode flyovers, automated showcase sequences, and simple timed camera progressions
without requiring the full sequencer timeline.

- **Requirements:** R-13.25.26
- **Dependencies:** F-13.25.1, F-13.25.16
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineSequencerCamera`.

## Group and Multi-Target

### F-13.25.27 Target Group (Multi-Target Aggregation)

Aggregates multiple tracking targets into a single virtual target for camera tracking. Two position
modes: group center (axis-aligned bounding box center) and group average (weighted average of member
positions). Rotation mode: manual or group average. Each member has a weight (influence on
averaging) and radius (size for bounding box computation). The target group entity is assigned as
the tracking target of any virtual camera, enabling multi-target tracking without specialized
per-camera logic.

- **Requirements:** R-13.25.27
- **Dependencies:** F-1.1.1 (ECS)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineTargetGroup`.

### F-13.25.28 Group Framing Extension

Extension for virtual cameras tracking a target group that adjusts zoom and position to keep all
group members framed. Framing mode: horizontal, vertical, or horizontal-and-vertical (best fit).
Size adjustment (perspective cameras): zoom only (FOV change), dolly only (reposition camera), or
dolly-then-zoom (move first, then adjust FOV). Lateral adjustment: change position or change
rotation. Framing size controls screen-space occupancy (0 = no group visible, 1 = group fills
screen). Center offset, damping, and FOV/dolly/ortho-size range constraints fine-tune behavior.

- **Requirements:** R-13.25.28
- **Dependencies:** F-13.25.27, F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineGroupFraming`.

## Camera Extensions

### F-13.25.29 Camera Confiner 2D

Extension that constrains the camera position so screen edges stay within a 2D polygon boundary.
Computes a secondary polygon based on camera view size and aspect ratio, then restricts the camera
transform to that boundary. Properties: bounding shape (2D polygon collider), damping (smooth corner
transitions), slowing distance (deceleration zone near edges), max window size (performance
optimization), and padding (minimum area for degenerate polygons). Cache invalidation required when
polygon points change. Extends the existing 2D camera (F-10.5.9) with hard boundary enforcement.

- **Requirements:** R-13.25.29
- **Dependencies:** F-10.5.9 (2D Camera)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineConfiner2D`.

### F-13.25.30 Camera Confiner 3D

Extension that restricts camera position to a 3D bounding volume. The camera cannot move outside the
defined volume. Slowing distance creates a deceleration zone at the boundary for smooth stops rather
than hard clamping. Used for keeping the camera within playable areas, arena boundaries, or interior
spaces.

- **Requirements:** R-13.25.30
- **Dependencies:** F-13.25.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineConfiner3D`.

### F-13.25.31 Follow Zoom (Constant Screen-Size Framing)

Extension that dynamically adjusts the camera's field of view to maintain a constant on-screen size
for the tracking target regardless of distance. Properties: target width (world-space size to
maintain), damping (zoom responsiveness), min/max FOV (allowable zoom range). Useful for tracking
shots where the subject should remain consistently sized in frame as the camera-to-target distance
varies.

- **Requirements:** R-13.25.31
- **Dependencies:** F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineFollowZoom`.

### F-13.25.32 Auto Focus

Extension that controls the camera's focus distance for depth-of-field effects. Five focus target
modes: look-at target, follow target, custom target, camera-relative (manual focus distance), and
screen center (depth buffer sample at screen center for deferred rendering). Properties: focus depth
offset, damping (focus transition speed), and auto-detection radius (screen-center mode). Integrates
with the post-processing depth-of-field system (F-2.9.2).

- **Requirements:** R-13.25.32
- **Dependencies:** F-13.25.1, F-2.9.2 (Depth of Field)
- **Platform notes:** Screen-center mode requires deferred rendering with depth buffer access.

**Gap source:** Unity CineMachine `CinemachineAutoFocus`.

### F-13.25.33 Third-Person Aim Extension

Extension that detects what the camera is aiming at by projecting a ray from camera position along
its forward axis to find the first intersection. The intersection point is stored for gameplay
systems to query (firing direction, interaction target). Handles the parallax problem in
third-person shooters where the character's weapon origin differs from the camera position by
computing what the player would actually hit from their character's position, accounting for
near-side obstructions. Noise cancellation stabilizes the aim target on screen despite camera shake,
preserving positional noise while eliminating rotational noise jitter.

- **Requirements:** R-13.25.33
- **Dependencies:** F-13.25.5, F-1.9.4 (Spatial Query)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineThirdPersonAim`.

### F-13.25.34 FreeLook Modifier

Extension that dynamically adjusts camera settings based on the orbital position of a FreeLook
camera (top, middle, bottom). Modifiable parameters: FOV, noise amplitude/frequency, damping,
composition alignment, and camera distance. The middle value (from the base virtual camera) serves
as reference; top and bottom values define the range. Easing parameter controls blending smoothness
across the center point (0 = linear, 1 = smooth curve).

- **Requirements:** R-13.25.34
- **Dependencies:** F-13.25.4
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineFreeLookModifier`.

### F-13.25.35 Recomposer (Timeline Composition Override)

Extension that applies a final compositional tweak to the camera output, particularly useful in
sequencer workflows. Animatable properties: tilt (vertical rotation), pan (horizontal rotation),
dutch (Z-axis roll), zoom scale, follow attachment (damping override, 0 = let go of follow target),
and look-at attachment (damping override, 0 = let go of look-at target). Processing stage
configurable: after body, after aim, after noise, or final (default). Enables hand-tuned adjustments
on top of procedural or recorded camera motion.

- **Requirements:** R-13.25.35
- **Dependencies:** F-13.25.1, F-13.5.1 (Sequencer)
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineRecomposer`.

### F-13.25.36 Camera Modifier Stack

Extensible modifier pipeline applied by the camera brain after computing the final view. Modifiers
run in priority order and can adjust position, rotation, FOV, near/far clip planes, and post-process
settings. Built-in modifiers include: FOV override (smooth FOV transitions for sprint/ADS),
post-process blend (apply camera-specific post-process profiles), and lens effects (vignette, film
grain intensity). Custom modifiers can be authored as plugin components. Modifiers are stateful and
associated with a specific camera brain instance.

- **Requirements:** R-13.25.36
- **Dependencies:** F-13.25.2, F-2.9.1 (Post-Processing)
- **Platform notes:** None

**Gap source:** UE5 `UCameraModifier` pipeline in `APlayerCameraManager`.

## Input Integration

### F-13.25.37 Camera Input Axis Controller

Component that bridges player input to camera axis parameters (orbital position, pan/tilt, zoom).
Compatible with the input action system (F-6.2.1) and supports both single-player and multiplayer
(per-player-index input routing). Responsiveness tuning: acceleration time, deceleration time, gain
multiplier, and delta-time compensation for frame-rate independence. Input suppression during camera
blending prevents jarring jumps. Unscaled delta-time option for input processing during paused time
scale. Recursive scanning discovers all input-capable camera behaviors on the entity.

- **Requirements:** R-13.25.37
- **Dependencies:** F-6.2.1 (Input Actions), F-13.25.1
- **Platform notes:** None

**Gap source:** Unity CineMachine `CinemachineInputAxisController`; UE5 pawn control rotation input
pipeline.

## Cinematic Camera Features

### F-13.25.38 Cine Camera Properties

Physical camera simulation with real-world lens and body parameters for cinematic rendering.
Properties: sensor size (Super 35, full frame, custom), focal length (lens zoom), aperture (f-stop
for depth of field), focus distance (manual or auto-focus), and filmback settings. These properties
drive the rendering pipeline's depth-of-field, exposure, and perspective projection. Used by the
sequencer for cinematographic workflows and available at runtime for stylized game cameras. Supports
lens distortion profiles for barrel/pincushion distortion simulation.

- **Requirements:** R-13.25.38
- **Dependencies:** F-2.9.2 (Depth of Field), F-2.10.4 (View Setup)
- **Platform notes:** None

**Gap source:** UE5 `CineCameraActor` with physical camera body and lens properties.

### F-13.25.39 Camera Rig Presets (Dolly, Crane, Jib)

Pre-built camera rig entities that simulate physical camera equipment. Camera dolly: camera on a
track with smooth lateral/forward movement. Camera crane/jib: camera on an arm with configurable arm
length, pivot height, and sweep range. Each rig constrains camera movement to physically plausible
paths and integrates with the sequencer for animated operation. Rig parameters (arm length, track
speed, pivot angle) are keyframeable.

- **Requirements:** R-13.25.39
- **Dependencies:** F-13.25.8, F-13.5.1 (Sequencer)
- **Platform notes:** None

**Gap source:** UE5 Camera Rig Dolly and Camera Rig Crane actors.

### F-13.25.40 Picture-in-Picture

Renders a secondary camera view into a viewport inset within the main view. The PiP viewport has
configurable position, size, border style, and opacity. The secondary camera can be any virtual
camera (rear-view mirror, security camera feed, minimap camera, spectator view). PiP rendering uses
a separate render target with configurable resolution scale for performance control. Multiple PiP
viewports are supported simultaneously.

- **Requirements:** R-13.25.40
- **Dependencies:** F-2.10.5 (Multi-View Rendering), F-13.25.2
- **Platform notes:** Mobile limits PiP to one viewport at quarter resolution. Desktop supports
  multiple PiP viewports at configurable resolution.

**Gap source:** UE5 scene capture to render target for PiP; common game engine requirement for
racing (rear mirror), stealth (security cam), and spectator modes.
