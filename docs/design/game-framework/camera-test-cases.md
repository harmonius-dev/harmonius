# Camera System Test Cases

Companion test cases for [camera.md](camera.md).

## Unit Tests

### TC-13.25.1.1 Priority Selection

| # | Requirement |
|---|-------------|
| 1 | R-13.25.1   |
| 2 | R-13.25.1   |

1. **#1** — 3 cameras: priority 1, 5, 10
   - **Expected:** Brain selects camera with priority 10
2. **#2** — Change priority 10 to 0
   - **Expected:** Brain selects camera with priority 5

### TC-13.25.2.1 Channel Mask Filter

| # | Requirement |
|---|-------------|
| 1 | R-13.25.2   |

1. **#1** — 2 brains (mask 0x1, 0x2); 2 cameras (mask 0x1, 0x2)
   - **Expected:** Each brain sees only its channel camera

### TC-13.25.2.2 Fixed Update Timing

| # | Requirement |
|---|-------------|
| 1 | R-13.25.2   |

1. **#1** — Brain in FixedUpdate mode
   - **Expected:** Updates sync with physics timestep

### TC-13.25.3.1 Follow 6 Binding Modes

| # | Requirement |
|---|-------------|
| 1 | R-13.25.3   |

1. **#1** — Rotate target 90 deg in each binding mode
   - **Expected:** Offset transforms correctly per mode

### TC-13.25.3.2 Follow Damping

| # | Requirement |
|---|-------------|
| 1 | R-13.25.3   |

1. **#1** — Move target; observe camera over 10 frames
   - **Expected:** Position smooths toward target over time

### TC-13.25.4.1 Orbital Sphere Mode

| # | Requirement |
|---|-------------|
| 1 | R-13.25.4   |

1. **#1** — Input horizontal axis +1.0 for 60 frames
   - **Expected:** Camera orbits at configured radius

### TC-13.25.4.2 Orbital Three Ring

| # | Requirement |
|---|-------------|
| 1 | R-13.25.4   |

1. **#1** — Orbit vertically through three rings
   - **Expected:** Radius interpolates between top/middle/bottom

### TC-13.25.4.3 Orbital Recentering

| # | Requirement |
|---|-------------|
| 1 | R-13.25.4   |

1. **#1** — Idle for wait_time seconds
   - **Expected:** Recentering begins; completes in completion_time

### TC-13.25.5.1 Third Person Shoulder Blend

| # | Requirement |
|---|-------------|
| 1 | R-13.25.5   |

1. **#1** — Blend camera_side from 0.0 to 1.0
   - **Expected:** Smooth lateral transition

### TC-13.25.5.2 Third Person Collision

| # | Requirement |
|---|-------------|
| 1 | R-13.25.5   |

1. **#1** — Wall between camera and target
   - **Expected:** Camera retracts to wall distance

### TC-13.25.6.1 Hard Lock Zero Offset

| # | Requirement |
|---|-------------|
| 1 | R-13.25.6   |

1. **#1** — Target at (10, 5, 3); evaluate camera
   - **Expected:** Camera position = (10, 5, 3) exactly

### TC-13.25.7.1 Position Composer Dead Zone

| # | Requirement |
|---|-------------|
| 1 | R-13.25.7   |

1. **#1** — Target screen position inside dead zone
   - **Expected:** Zero camera movement

### TC-13.25.7.2 Position Composer Hard Limit

| # | Requirement |
|---|-------------|
| 1 | R-13.25.7   |

1. **#1** — Target at screen edge (hard limit)
   - **Expected:** Immediate position correction

### TC-13.25.8.1 Spline Dolly Fixed Speed

| # | Requirement |
|---|-------------|
| 1 | R-13.25.8   |

1. **#1** — AutoDolly::FixedSpeed at speed 2.0
   - **Expected:** Camera traverses at constant 2.0 u/s

### TC-13.25.8.2 Spline Dolly Nearest

| # | Requirement |
|---|-------------|
| 1 | R-13.25.8   |

1. **#1** — Target moves along spline
   - **Expected:** Camera tracks nearest spline point

### TC-13.25.9.1 Rotation Composer Dead Zone

| # | Requirement |
|---|-------------|
| 1 | R-13.25.9   |

1. **#1** — Look-at target in dead zone
   - **Expected:** No rotation change

### TC-13.25.10.1 Hard Look At Centered

| # | Requirement |
|---|-------------|
| 1 | R-13.25.10  |

1. **#1** — Target at various positions
   - **Expected:** Target centered in frame each time

### TC-13.25.11.1 Pan Tilt Clamp

| # | Requirement |
|---|-------------|
| 1 | R-13.25.11  |

1. **#1** — Tilt input beyond 90 degrees
   - **Expected:** Tilt clamped at 90 degrees

### TC-13.25.12.1 Rotate With Target

| # | Requirement |
|---|-------------|
| 1 | R-13.25.12  |

1. **#1** — Target rotates 45 degrees
   - **Expected:** Camera rotation matches

### TC-13.25.13.1 Spring Arm Retraction

| # | Requirement |
|---|-------------|
| 1 | R-13.25.13  |

1. **#1** — Obstacle at 3 m on 5 m arm
   - **Expected:** Camera distance = 3 m

### TC-13.25.13.2 Spring Arm Extension

| # | Requirement |
|---|-------------|
| 1 | R-13.25.13  |

1. **#1** — Remove obstacle from 3 m
   - **Expected:** Camera extends back to 5 m target length

### TC-13.25.14.1 Deoccluder Pull Forward

| # | Requirement |
|---|-------------|
| 1 | R-13.25.14  |

1. **#1** — Obstacle between camera and target
   - **Expected:** Camera pulls forward past obstacle

### TC-13.25.14.2 Deoccluder Min Time

| # | Requirement |
|---|-------------|
| 1 | R-13.25.14  |

1. **#1** — Obstruction for 0.05s (min_time=0.1s)
   - **Expected:** Obstruction ignored

### TC-13.25.15.1 Decollider Terrain

| # | Requirement |
|---|-------------|
| 1 | R-13.25.15  |

1. **#1** — Camera position below terrain surface
   - **Expected:** Camera pushed above surface

### TC-13.25.16.1 Blend 8 Curves

| # | Requirement |
|---|-------------|
| 1 | R-13.25.16  |

1. **#1** — Trigger blend with each of 8 curve types
   - **Expected:** All 8 produce distinct interpolation curves

### TC-13.25.16.2 Blend Custom Pair

| # | Requirement |
|---|-------------|
| 1 | R-13.25.16  |

1. **#1** — Custom rule for cameras A->B; default for wildcard
   - **Expected:** Custom rule overrides default

### TC-13.25.17.1 Mixer Weighted Average

| # | Requirement |
|---|-------------|
| 1 | R-13.25.17  |

1. **#1** — 3 cameras at weights 1,2,1
   - **Expected:** Output = weighted average of positions

### TC-13.25.17.2 Mixer Zero Weight

| # | Requirement |
|---|-------------|
| 1 | R-13.25.17  |

1. **#1** — Camera with weight 0.0
   - **Expected:** Contributes nothing to output

### TC-13.25.18.1 Perlin Mute

| # | Requirement |
|---|-------------|
| 1 | R-13.25.18  |

1. **#1** — Perlin shake with amplitude_gain=0
   - **Expected:** Zero output displacement

### TC-13.25.18.2 Perlin Frequency Gain

| # | Requirement |
|---|-------------|
| 1 | R-13.25.18  |

1. **#1** — frequency_gain=2.0 vs 1.0
   - **Expected:** Double oscillation rate

### TC-13.25.19.1 Impulse Distance Attenuation

| # | Requirement |
|---|-------------|
| 1 | R-13.25.19  |

1. **#1** — Listener at 5m vs 10m from source
   - **Expected:** Closer listener receives stronger shake

### TC-13.25.19.2 Impulse Outside Radius

| # | Requirement |
|---|-------------|
| 1 | R-13.25.19  |

1. **#1** — Listener beyond impulse radius
   - **Expected:** Zero shake received

### TC-13.25.20.1 Wave Oscillation

| # | Requirement |
|---|-------------|
| 1 | R-13.25.20  |

1. **#1** — 1 Hz sine wave; sample at 0.25s
   - **Expected:** Output = amplitude (peak of sine)

### TC-13.25.23.1 State Driven Mapping

| # | Requirement |
|---|-------------|
| 1 | R-13.25.23  |

1. **#1** — Animation state changes to "combat"
   - **Expected:** Mapped combat camera activates

### TC-13.25.24.1 Clear Shot Selection

| # | Requirement |
|---|-------------|
| 1 | R-13.25.24  |

1. **#1** — 3 child cameras with quality scores 0.3, 0.8, 0.5
   - **Expected:** Camera with score 0.8 selected

### TC-13.25.26.1 Sequencer Playlist

| # | Requirement |
|---|-------------|
| 1 | R-13.25.26  |

1. **#1** — 3 cameras with hold times 2s, 3s, 1s
   - **Expected:** Play in order with correct hold durations

### TC-13.25.27.1 Target Group BBox

| # | Requirement |
|---|-------------|
| 1 | R-13.25.27  |

1. **#1** — 3 members at (0,0,0), (10,0,0), (0,10,0)
   - **Expected:** Group center = AABB center (5,5,0)

### TC-13.25.28.1 Group Framing Spread

| # | Requirement |
|---|-------------|
| 1 | R-13.25.28  |

1. **#1** — Members spread from 10m to 50m apart
   - **Expected:** FOV widens to frame all members

### TC-13.25.29.1 Confiner 2D Boundary

| # | Requirement |
|---|-------------|
| 1 | R-13.25.29  |

1. **#1** — Target near polygon edge
   - **Expected:** Camera view stays within polygon

### TC-13.25.31.1 Follow Zoom FOV Adjust

| # | Requirement |
|---|-------------|
| 1 | R-13.25.31  |

1. **#1** — Target at 50m; target_width=5.0
   - **Expected:** FOV adjusts to maintain 5m screen width

### TC-13.25.37.1 Input Frame Independence

| # | Requirement |
|---|-------------|
| 1 | R-13.25.37  |

1. **#1** — Same input at 30 fps and 120 fps
   - **Expected:** Same final camera position

### TC-13.25.38.1 Cine Camera FOV

| # | Requirement |
|---|-------------|
| 1 | R-13.25.38  |

1. **#1** — Super 35 sensor + 50mm focal length
   - **Expected:** Vertical FOV = 2*atan(12.0/50.0) ~= 27.0 deg

## Integration Tests

### TC-NFR-13.25.1.I1 Split Screen 4 Brains

| # | Requirement |
|---|-------------|
| 1 | NFR-13.25.1 |

1. **#1** — 4 brains with cameras, blending, collision
   - **Expected:** Total evaluation < 4 ms

### TC-NFR-13.25.2.I1 Blend Smoothness 30fps

| # | Requirement |
|---|-------------|
| 1 | NFR-13.25.2 |

1. **#1** — 1-second blend at 30 fps
   - **Expected:** No position jump > 0.01 units

### TC-NFR-13.25.2.I2 Blend Smoothness 120fps

| # | Requirement |
|---|-------------|
| 1 | NFR-13.25.2 |

1. **#1** — 1-second blend at 120 fps
   - **Expected:** No rotation jump > 0.1 degrees

### TC-13.25.40.I1 PiP Multiple Viewports

| # | Requirement |
|---|-------------|
| 1 | R-13.25.40  |

1. **#1** — 2 PiP viewports on desktop
   - **Expected:** Both render simultaneously

## Benchmarks

### TC-NFR-13.25.1.B1 Camera Evaluation Per Brain

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single brain evaluation | Wall-clock time | < 1 ms | NFR-13.25.1 |

### TC-NFR-13.25.1.B2 4-Brain Split Screen

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 4 brains with full pipeline | Wall-clock time | < 4 ms | NFR-13.25.1 |

### TC-NFR-13.25.2.B1 Blend Position Delta

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Maximum inter-frame position jump during blend | Position delta | < 0.01 units | NFR-13.25.2 |

### TC-NFR-13.25.2.B2 Blend Rotation Delta

| # | Metric         | Target        | Requirement |
|---|----------------|---------------|-------------|
| 1 | Rotation delta | < 0.1 degrees | NFR-13.25.2 |

1. **1** — Maximum inter-frame rotation jump during blend

### TC-13.25.19.B1 Impulse Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10 sources, 4 listeners | Wall-clock time | < 0.1 ms | R-13.25.19 |

### TC-13.25.28.B1 Group Framing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 group members | Wall-clock time | < 0.2 ms | R-13.25.28 |
