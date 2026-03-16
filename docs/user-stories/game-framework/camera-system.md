# User Stories -- 3D Camera System (13.25)

## Virtual Camera Entity and Priority (F-13.25.1)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-13.25.1.1 | designer (P-5) | **As a** designer (P-5), **I want** each camera behavior to be an ECS entity with a priority value that determines which camera is active, **so that** camera switching is data-driven. |  | F-13.25.1 | R-13.25.1 |
| US-13.25.1.2 | designer (P-5) | **As a** designer (P-5), **I want** priority modifiable at runtime by gameplay events, **so that** cameras activate automatically on triggers, combat, or mounting. |  | F-13.25.1 | R-13.25.1 |
| US-13.25.1.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** virtual cameras to be lightweight descriptions that feed computed parameters to the Camera Brain, **so that** multiple cameras coexist cheaply. |  | F-13.25.1 | R-13.25.1 |
| US-13.25.1.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the highest-priority camera is selected when multiple cameras are active, **so that** priority resolution is correct. |  | F-13.25.1 | R-13.25.1 |
## Camera Brain (F-13.25.2)
| US-13.25.2.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera brain to blend between outgoing and incoming cameras using configurable curves, **so that** transitions feel smooth. |  | F-13.25.2 | R-13.25.2 |
| US-13.25.2.2 | creative director (P-2) | **As a** creative director (P-2), **I want** multiple brains for split-screen, each reading from a different channel mask, **so that** local multiplayer cameras are independent. |  | F-13.25.2 | R-13.25.2 |
| US-13.25.2.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** update timing configurable as late update, fixed update, or manual update, **so that** cameras sync with physics or replays as needed. |  | F-13.25.2 | R-13.25.2 |
| US-13.25.2.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the brain blends position, rotation, FOV, and post-process between cameras, **so that** all parameters transition smoothly. |  | F-13.25.2 | R-13.25.2 |
## Follow (Fixed Offset) (F-13.25.3)
| US-13.25.3.1 | player (P-23) | **As a** player (P-23), **I want** the camera to follow my character at a fixed offset with smooth damping, **so that** movement feels responsive but not jarring. |  | F-13.25.3 | R-13.25.3 |
| US-13.25.3.2 | designer (P-5) | **As a** designer (P-5), **I want** six binding modes (world space, lock-to-target, lock-no-roll, lock-with-world-up, lock-on-assign, lazy follow), **so that** I can tune follow behavior per scenario. |  | F-13.25.3 | R-13.25.3 |
| US-13.25.3.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that per-axis position and angular damping smooth the camera response, **so that** damping parameters apply correctly. |  | F-13.25.3 | R-13.25.3 |
## Orbital Follow (F-13.25.4)
| US-13.25.4.1 | player (P-23) | **As a** player (P-23), **I want** to orbit the camera around my character using input axes, **so that** I control the viewing angle. |  | F-13.25.4 | R-13.25.4 |
| US-13.25.4.2 | designer (P-5) | **As a** designer (P-5), **I want** sphere mode and three-ring mode for orbit shapes, **so that** I can create simple or complex orbit surfaces. |  | F-13.25.4 | R-13.25.4 |
| US-13.25.4.3 | designer (P-5) | **As a** designer (P-5), **I want** automatic recentering with configurable wait and completion time, **so that** the camera returns to a neutral position when idle. |  | F-13.25.4 | R-13.25.4 |
| US-13.25.4.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that axis wrapping loops the orbit without snapping, **so that** continuous rotation is smooth. |  | F-13.25.4 | R-13.25.4 |
## Third-Person Follow with Shoulder Offset (F-13.25.5)
| US-13.25.5.1 | player (P-23) | **As a** player (P-23), **I want** the camera positioned over my character's shoulder with configurable left/right offset, **so that** I have an over-the-shoulder view. |  | F-13.25.5 | R-13.25.5 |
| US-13.25.5.2 | player (P-23) | **As a** player (P-23), **I want** the camera to retract when obstacles occlude the character and extend back when clear, **so that** the view never clips through walls. |  | F-13.25.5 | R-13.25.5 |
| US-13.25.5.3 | designer (P-5) | **As a** designer (P-5), **I want** separate damping rates for collision retraction and recovery, **so that** collision response feels smooth. |  | F-13.25.5 | R-13.25.5 |
| US-13.25.5.4 | engine dev (P-26) | **As a** engine dev (P-26), **I want** collision layer masks and ignore tags for collision checks, **so that** selective geometry participates in camera collision. |  | F-13.25.5 | R-13.25.5 |
| US-13.25.5.5 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the camera radius defines a minimum distance from obstacles, **so that** near-clipping is prevented. |  | F-13.25.5 | R-13.25.5 |
## Hard Lock to Target (F-13.25.6)
| US-13.25.6.1 | designer (P-5) | **As a** designer (P-5), **I want** a position control that copies the target's world position directly, **so that** first-person and cockpit cameras have rigid attachment. |  | F-13.25.6 | R-13.25.6 |
| US-13.25.6.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that no offset or damping is applied, **so that** the lock is truly rigid. |  | F-13.25.6 | R-13.25.6 |
## Position Composer (F-13.25.7)
| US-13.25.7.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera to maintain a desired screen-space position for the target using dead zone, soft zone, and hard limits, **so that** framing adapts to movement. |  | F-13.25.7 | R-13.25.7 |
| US-13.25.7.2 | creative director (P-2) | **As a** creative director (P-2), **I want** screen position parameters for rule-of-thirds placement, **so that** composition follows cinematic principles. |  | F-13.25.7 | R-13.25.7 |
| US-13.25.7.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the target cannot leave the hard limits, **so that** the framing constraint is enforced. |  | F-13.25.7 | R-13.25.7 |
## Spline Dolly (F-13.25.8)
| US-13.25.8.1 | designer (P-5) | **As a** designer (P-5), **I want** to constrain the camera to a spline path with automatic dolly modes (fixed speed, nearest-point-to-target), **so that** I can create cinematic rails. |  | F-13.25.8 | R-13.25.8 |
| US-13.25.8.2 | creative director (P-2) | **As a** creative director (P-2), **I want** Catmull-Rom and Bezier spline support, **so that** I can author smooth camera paths. |  | F-13.25.8 | R-13.25.8 |
| US-13.25.8.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that nearest-point-to-target mode keeps the camera at the closest spline point to the tracked entity, **so that** tracking is correct. |  | F-13.25.8 | R-13.25.8 |
## Rotation Composer (F-13.25.9)
| US-13.25.9.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera to rotate toward the look-at target with dead zone, soft zone, and damping, **so that** aiming adapts smoothly. |  | F-13.25.9 | R-13.25.9 |
| US-13.25.9.2 | designer (P-5) | **As a** designer (P-5), **I want** lookahead time for anticipatory aiming, **so that** the camera leads target movement. |  | F-13.25.9 | R-13.25.9 |
| US-13.25.9.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that rotation control adjusts only rotation and never position, **so that** the behavior separation is correct. |  | F-13.25.9 | R-13.25.9 |
## Hard Look At (F-13.25.10)
| US-13.25.10.1 | designer (P-5) | **As a** designer (P-5), **I want** rigid aim at the look-at target with no damping, **so that** the target is always centered. |  | F-13.25.10 | R-13.25.10 |
| US-13.25.10.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the target remains pixel-centered regardless of camera or target movement, **so that** rigid aim is accurate. |  | F-13.25.10 | R-13.25.10 |
## Pan and Tilt (F-13.25.11)
| US-13.25.11.1 | player (P-23) | **As a** player (P-23), **I want** to rotate the camera via input axes for manual look control, **so that** I can freely look around. |  | F-13.25.11 | R-13.25.11 |
| US-13.25.11.2 | designer (P-5) | **As a** designer (P-5), **I want** tilt clamped to [-90, 90] degrees and configurable recentering, **so that** look boundaries feel natural. |  | F-13.25.11 | R-13.25.11 |
| US-13.25.11.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that automatic recentering activates after the configured wait time, **so that** the timer is correct. |  | F-13.25.11 | R-13.25.11 |
## Rotate with Follow Target (F-13.25.12)
| US-13.25.12.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera to copy the tracking target's rotation, **so that** vehicle and mounted cameras face the vehicle's direction. |  | F-13.25.12 | R-13.25.12 |
| US-13.25.12.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the camera rotation matches the target rotation each frame, **so that** the copy is frame-precise. |  | F-13.25.12 | R-13.25.12 |
## Spring Arm Component (F-13.25.13)
| US-13.25.13.1 | designer (P-5) | **As a** designer (P-5), **I want** a spring arm that retracts on collision and extends back when clear, **so that** the camera does not clip into geometry. |  | F-13.25.13 | R-13.25.13 |
| US-13.25.13.2 | engine dev (P-26) | **As a** engine dev (P-26), **I want** configurable probe size, channel, position lag, and rotation lag, **so that** spring arm behavior is tunable. |  | F-13.25.13 | R-13.25.13 |
| US-13.25.13.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** per-axis control of pitch, roll, and yaw inheritance from parent, **so that** rotation isolation is possible. |  | F-13.25.13 | R-13.25.13 |
| US-13.25.13.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the arm retracts when a wall is between the parent and the arm end, **so that** collision detection works. |  | F-13.25.13 | R-13.25.13 |
## Camera Deoccluder (F-13.25.14)
| US-13.25.14.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera to reposition to maintain line of sight when obstacles intervene, **so that** the player is always visible. |  | F-13.25.14 | R-13.25.14 |
| US-13.25.14.2 | designer (P-5) | **As a** designer (P-5), **I want** three repositioning strategies (pull forward, preserve height, preserve distance), **so that** I can match the game's camera style. |  | F-13.25.14 | R-13.25.14 |
| US-13.25.14.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** separate damping for normal movement and occluded repositioning, **so that** collision response has a different feel. |  | F-13.25.14 | R-13.25.14 |
| US-13.25.14.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that brief obstructions below the minimum occlusion time are ignored, **so that** transient objects do not trigger repositioning. |  | F-13.25.14 | R-13.25.14 |
## Camera Decollider (F-13.25.15)
| US-13.25.15.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera prevented from entering solid geometry with terrain and obstacle resolution, **so that** the camera never clips inside walls. |  | F-13.25.15 | R-13.25.15 |
| US-13.25.15.2 | engine dev (P-26) | **As a** engine dev (P-26), **I want** separate terrain and obstacle collision layers, **so that** different geometry types are handled distinctly. |  | F-13.25.15 | R-13.25.15 |
| US-13.25.15.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that smoothing time holds the camera position briefly to reduce jitter, **so that** the anti-jitter mechanism works. |  | F-13.25.15 | R-13.25.15 |
## Camera Blend System (F-13.25.16)
| US-13.25.16.1 | designer (P-5) | **As a** designer (P-5), **I want** eight blend curve types (cut, ease-in-out, linear, etc.) with configurable duration, **so that** I can style transitions per scenario. |  | F-13.25.16 | R-13.25.16 |
| US-13.25.16.2 | creative director (P-2) | **As a** creative director (P-2), **I want** custom blends assets defining per-camera-pair transition rules with wildcard support, **so that** cinematic transitions are precise. |  | F-13.25.16 | R-13.25.16 |
| US-13.25.16.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that a custom blend for a specific camera pair overrides the default blend, **so that** specificity resolution is correct. |  | F-13.25.16 | R-13.25.16 |
## Camera Mixing (F-13.25.17)
| US-13.25.17.1 | designer (P-5) | **As a** designer (P-5), **I want** up to eight cameras blended simultaneously by weight, **so that** multi-perspective shots are achievable. |  | F-13.25.17 | R-13.25.17 |
| US-13.25.17.2 | creative director (P-2) | **As a** creative director (P-2), **I want** weights animatable via the sequencer, **so that** weighted blends are choreographable. |  | F-13.25.17 | R-13.25.17 |
| US-13.25.17.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that zero-weighted cameras contribute nothing to the blend, **so that** weight normalization is correct. |  | F-13.25.17 | R-13.25.17 |
## Perlin Noise Profiles (F-13.25.18)
| US-13.25.18.1 | designer (P-5) | **As a** designer (P-5), **I want** multi-channel Perlin noise profiles with amplitude and frequency gains, **so that** I can create organic camera shake. |  | F-13.25.18 | R-13.25.18 |
| US-13.25.18.2 | creative director (P-2) | **As a** creative director (P-2), **I want** built-in presets (handheld, breathe, tripod vibration), **so that** common shake styles are ready to use. |  | F-13.25.18 | R-13.25.18 |
| US-13.25.18.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that amplitude gain of zero mutes the noise, **so that** the muting parameter works correctly. |  | F-13.25.18 | R-13.25.18 |
## Camera Impulse System (F-13.25.19)
| US-13.25.19.1 | player (P-23) | **As a** player (P-23), **I want** explosions and impacts to shake my camera proportionally to distance, **so that** environmental events feel physically impactful. |  | F-13.25.19 | R-13.25.19 |
| US-13.25.19.2 | designer (P-5) | **As a** designer (P-5), **I want** impulse sources with direction, strength curve, duration, and propagation radius, **so that** I can tune shake per event type. |  | F-13.25.19 | R-13.25.19 |
| US-13.25.19.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** collision impulse sources that trigger automatically on rigid body collisions, **so that** physics events produce camera feedback. |  | F-13.25.19 | R-13.25.19 |
| US-13.25.19.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that distance attenuation reduces intensity for cameras far from the impulse origin, **so that** falloff is correct. |  | F-13.25.19 | R-13.25.19 |
## Wave Oscillation Shake (F-13.25.20)
| US-13.25.20.1 | designer (P-5) | **As a** designer (P-5), **I want** sinusoidal wave oscillation with per-axis amplitude and frequency, **so that** I can create rocking boat or dream sequence effects. |  | F-13.25.20 | R-13.25.20 |
| US-13.25.20.2 | creative director (P-2) | **As a** creative director (P-2), **I want** blend-in and blend-out times for smooth activation, **so that** wave shake starts and stops gracefully. |  | F-13.25.20 | R-13.25.20 |
| US-13.25.20.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that infinite duration mode produces continuous oscillation, **so that** persistent effects do not expire. |  | F-13.25.20 | R-13.25.20 |
## Composite Shake Patterns (F-13.25.21)
| US-13.25.21.1 | designer (P-5) | **As a** designer (P-5), **I want** to layer Perlin noise, wave oscillation, and sequencer animation into a single composite shake, **so that** complex effects are composable. |  | F-13.25.21 | R-13.25.21 |
| US-13.25.21.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that each layer contributes additively to the final shake, **so that** layer composition is correct. |  | F-13.25.21 | R-13.25.21 |
## Sequencer-Driven Camera Shake (F-13.25.22)
| US-13.25.22.1 | creative director (P-2) | **As a** creative director (P-2), **I want** to keyframe precise shake motion in the sequencer, **so that** scripted explosions have choreographed camera reactions. |  | F-13.25.22 | R-13.25.22 |
| US-13.25.22.2 | designer (P-5) | **As a** designer (P-5), **I want** sequencer shake layerable with procedural noise, **so that** authored and organic shake combine. |  | F-13.25.22 | R-13.25.22 |
| US-13.25.22.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that looped sequencer shake repeats seamlessly, **so that** continuous authored shake is glitch-free. |  | F-13.25.22 | R-13.25.22 |
## State-Driven Camera Switching (F-13.25.23)
| US-13.25.23.1 | designer (P-5) | **As a** designer (P-5), **I want** animation states mapped to virtual cameras with per-mapping wait time and minimum duration, **so that** cameras switch with animation context. |  | F-13.25.23 | R-13.25.23 |
| US-13.25.23.2 | creative director (P-2) | **As a** creative director (P-2), **I want** blending between cameras on state change using custom blend assets, **so that** transitions are cinematic. |  | F-13.25.23 | R-13.25.23 |
| US-13.25.23.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that transient animation states shorter than the wait time do not trigger camera switches, **so that** flickering is prevented. |  | F-13.25.23 | R-13.25.23 |
## Clear Shot (F-13.25.24)
| US-13.25.24.1 | designer (P-5) | **As a** designer (P-5), **I want** automatic best-camera selection based on unobstructed view quality scoring, **so that** the camera always has a clear shot. |  | F-13.25.24 | R-13.25.24 |
| US-13.25.24.2 | creative director (P-2) | **As a** creative director (P-2), **I want** activate-after delay and minimum duration to prevent rapid switching, **so that** cuts are deliberate. |  | F-13.25.24 | R-13.25.24 |
| US-13.25.24.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the camera with the best quality score is selected, **so that** scoring-based selection works. |  | F-13.25.24 | R-13.25.24 |
## Shot Quality Evaluator (F-13.25.25)
| US-13.25.25.1 | designer (P-5) | **As a** designer (P-5), **I want** per-camera quality scores from target visibility and distance, **so that** automatic camera selection uses quantifiable metrics. |  | F-13.25.25 | R-13.25.25 |
| US-13.25.25.2 | engine dev (P-26) | **As a** engine dev (P-26), **I want** quality scores normalized to 0-1 with high-quality bonus multipliers, **so that** scoring is consistent across systems. |  | F-13.25.25 | R-13.25.25 |
| US-13.25.25.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that an occluded camera receives a lower quality score, **so that** occlusion detection affects scoring. |  | F-13.25.25 | R-13.25.25 |
## Camera Sequencer (F-13.25.26)
| US-13.25.26.1 | designer (P-5) | **As a** designer (P-5), **I want** a timed camera playlist with hold duration per camera, **so that** attract-mode flyovers play automatically. |  | F-13.25.26 | R-13.25.26 |
| US-13.25.26.2 | creative director (P-2) | **As a** creative director (P-2), **I want** loop and non-loop modes with blend transitions, **so that** sequenced cameras serve attract screens and showcases. |  | F-13.25.26 | R-13.25.26 |
| US-13.25.26.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that non-loop mode holds the final camera indefinitely, **so that** the playlist terminates correctly. |  | F-13.25.26 | R-13.25.26 |
## Target Group (F-13.25.27)
| US-13.25.27.1 | designer (P-5) | **As a** designer (P-5), **I want** multiple targets aggregated into a single virtual target with group center or weighted average positioning, **so that** multi-target tracking is simple. |  | F-13.25.27 | R-13.25.27 |
| US-13.25.27.2 | player (P-23) | **As a** player (P-23), **I want** the camera to frame all group members naturally, **so that** multi-character scenes keep everyone visible. |  | F-13.25.27 | R-13.25.27 |
| US-13.25.27.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that member weight affects position averaging, **so that** weighted members have more influence on the group center. |  | F-13.25.27 | R-13.25.27 |
## Group Framing Extension (F-13.25.28)
| US-13.25.28.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera to adjust zoom and position to keep all group members framed, **so that** the view adapts to group spread. |  | F-13.25.28 | R-13.25.28 |
| US-13.25.28.2 | creative director (P-2) | **As a** creative director (P-2), **I want** framing modes (horizontal, vertical, best fit) with size adjustment via zoom, dolly, or both, **so that** framing matches the shot style. |  | F-13.25.28 | R-13.25.28 |
| US-13.25.28.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that FOV constraints clamp the zoom range, **so that** extreme zoom is prevented. |  | F-13.25.28 | R-13.25.28 |
## Camera Confiner 2D (F-13.25.29)
| US-13.25.29.1 | designer (P-5) | **As a** designer (P-5), **I want** the camera constrained to a 2D polygon boundary so screen edges stay within bounds, **so that** the 2D camera does not show out-of-bounds areas. |  | F-13.25.29 | R-13.25.29 |
| US-13.25.29.2 | player (P-23) | **As a** player (P-23), **I want** smooth corner transitions with damping, **so that** boundary enforcement does not cause jarring stops. |  | F-13.25.29 | R-13.25.29 |
| US-13.25.29.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the camera cannot show pixels outside the polygon boundary, **so that** confinement is absolute. |  | F-13.25.29 | R-13.25.29 |
## Camera Confiner 3D (F-13.25.30)
| US-13.25.30.1 | designer (P-5) | **As a** designer (P-5), **I want** camera position restricted to a 3D bounding volume with slowing distance, **so that** the camera stays within playable areas. |  | F-13.25.30 | R-13.25.30 |
| US-13.25.30.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that slowing distance creates deceleration near boundaries, **so that** the camera slows rather than hard-clamping. |  | F-13.25.30 | R-13.25.30 |
## Follow Zoom (F-13.25.31)
| US-13.25.31.1 | designer (P-5) | **As a** designer (P-5), **I want** FOV dynamically adjusted to maintain constant on-screen size for the target, **so that** the subject stays consistently sized. |  | F-13.25.31 | R-13.25.31 |
| US-13.25.31.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that min/max FOV constraints clamp the zoom, **so that** extreme FOV values are prevented. |  | F-13.25.31 | R-13.25.31 |
## Auto Focus (F-13.25.32)
| US-13.25.32.1 | designer (P-5) | **As a** designer (P-5), **I want** five focus target modes (look-at, follow, custom, camera-relative, screen center), **so that** depth of field focus is versatile. |  | F-13.25.32 | R-13.25.32 |
| US-13.25.32.2 | creative director (P-2) | **As a** creative director (P-2), **I want** damping on focus transitions, **so that** rack focus feels smooth and cinematic. |  | F-13.25.32 | R-13.25.32 |
| US-13.25.32.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that screen-center mode samples the depth buffer at screen center, **so that** deferred rendering focus works. |  | F-13.25.32 | R-13.25.32 |
## Third-Person Aim Extension (F-13.25.33)
| US-13.25.33.1 | player (P-23) | **As a** player (P-23), **I want** the camera aim ray to account for parallax between camera and character weapon origin, **so that** I hit what my crosshair targets. |  | F-13.25.33 | R-13.25.33 |
| US-13.25.33.2 | designer (P-5) | **As a** designer (P-5), **I want** noise cancellation to stabilize the aim target despite camera shake, **so that** aim is precise during shaky cameras. |  | F-13.25.33 | R-13.25.33 |
| US-13.25.33.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** the aim intersection stored for gameplay systems to query, **so that** firing direction is available to combat. |  | F-13.25.33 | R-13.25.33 |
| US-13.25.33.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that near-side obstructions are correctly handled, **so that** parallax correction is accurate. |  | F-13.25.33 | R-13.25.33 |
## FreeLook Modifier (F-13.25.34)
| US-13.25.34.1 | designer (P-5) | **As a** designer (P-5), **I want** camera settings to adjust based on orbital position (top, middle, bottom), **so that** high and low angles have distinct feels. |  | F-13.25.34 | R-13.25.34 |
| US-13.25.34.2 | tester (P-27) | **As a** tester (P-27), **I want** to verify that easing parameter controls blending smoothness, **so that** the transition between top and bottom is smooth. |  | F-13.25.34 | R-13.25.34 |
## Recomposer (F-13.25.35)
| US-13.25.35.1 | designer (P-5) | **As a** designer (P-5), **I want** animatable tilt, pan, dutch, and zoom overrides on top of procedural camera motion, **so that** sequencer shots can be hand-tuned. |  | F-13.25.35 | R-13.25.35 |
| US-13.25.35.2 | creative director (P-2) | **As a** creative director (P-2), **I want** follow and look-at attachment overrides, **so that** the camera can let go of a target smoothly. |  | F-13.25.35 | R-13.25.35 |
| US-13.25.35.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the processing stage configuration controls when the recomposer applies, **so that** ordering is correct. |  | F-13.25.35 | R-13.25.35 |
## Camera Modifier Stack (F-13.25.36)
| US-13.25.36.1 | designer (P-5) | **As a** designer (P-5), **I want** an extensible modifier pipeline adjusting position, rotation, FOV, and post-process after the brain computes the view, **so that** late-stage tweaks are possible. |  | F-13.25.36 | R-13.25.36 |
| US-13.25.36.2 | engine dev (P-26) | **As a** engine dev (P-26), **I want** built-in modifiers for FOV override, post-process blend, and lens effects, **so that** common modifications are ready to use. |  | F-13.25.36 | R-13.25.36 |
| US-13.25.36.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** custom modifiers registerable as plugin components, **so that** the pipeline is extensible. |  | F-13.25.36 | R-13.25.36 |
| US-13.25.36.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that modifiers run in priority order, **so that** execution ordering is deterministic. |  | F-13.25.36 | R-13.25.36 |
## Camera Input Axis Controller (F-13.25.37)
| US-13.25.37.1 | player (P-23) | **As a** player (P-23), **I want** camera input with configurable acceleration, deceleration, and gain, **so that** camera control feels responsive to my preferences. |  | F-13.25.37 | R-13.25.37 |
| US-13.25.37.2 | designer (P-5) | **As a** designer (P-5), **I want** per-player-index input routing for multiplayer, **so that** each player controls only their camera. |  | F-13.25.37 | R-13.25.37 |
| US-13.25.37.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** input suppression during camera blending and unscaled delta-time during paused time scale, **so that** edge cases do not cause jarring jumps. |  | F-13.25.37 | R-13.25.37 |
| US-13.25.37.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that delta-time compensation produces frame-rate-independent input, **so that** camera speed is consistent across frame rates. |  | F-13.25.37 | R-13.25.37 |
## Cine Camera Properties (F-13.25.38)
| US-13.25.38.1 | creative director (P-2) | **As a** creative director (P-2), **I want** physical camera simulation with sensor size, focal length, aperture, and focus distance, **so that** cinematic shots have realistic optics. |  | F-13.25.38 | R-13.25.38 |
| US-13.25.38.2 | designer (P-5) | **As a** designer (P-5), **I want** lens distortion profiles for barrel and pincushion simulation, **so that** stylized camera looks are possible. |  | F-13.25.38 | R-13.25.38 |
| US-13.25.38.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that aperture controls depth-of-field blur radius, **so that** physical camera parameters drive rendering correctly. |  | F-13.25.38 | R-13.25.38 |
## Camera Rig Presets (F-13.25.39)
| US-13.25.39.1 | designer (P-5) | **As a** designer (P-5), **I want** pre-built dolly, crane, and jib rig entities with keyframeable parameters, **so that** physical camera setups are authoring-ready. |  | F-13.25.39 | R-13.25.39 |
| US-13.25.39.2 | creative director (P-2) | **As a** creative director (P-2), **I want** rig integration with the sequencer, **so that** crane sweeps and dolly tracks are animatable. |  | F-13.25.39 | R-13.25.39 |
| US-13.25.39.3 | tester (P-27) | **As a** tester (P-27), **I want** to verify that the crane arm length constrains camera movement to the configured sweep, **so that** rig constraints are enforced. |  | F-13.25.39 | R-13.25.39 |
## Picture-in-Picture (F-13.25.40)
| US-13.25.40.1 | player (P-23) | **As a** player (P-23), **I want** a secondary camera view rendered as an inset viewport, **so that** I can see rear-view mirrors or security camera feeds. |  | F-13.25.40 | R-13.25.40 |
| US-13.25.40.2 | designer (P-5) | **As a** designer (P-5), **I want** configurable PiP position, size, border style, and resolution scale, **so that** the inset is tunable per use case. |  | F-13.25.40 | R-13.25.40 |
| US-13.25.40.3 | engine dev (P-26) | **As a** engine dev (P-26), **I want** PiP using a separate render target with configurable resolution, **so that** performance cost is controllable. |  | F-13.25.40 | R-13.25.40 |
| US-13.25.40.4 | tester (P-27) | **As a** tester (P-27), **I want** to verify that multiple PiP viewports render simultaneously on desktop, **so that** multi-PiP support works. |  | F-13.25.40 | R-13.25.40 |
