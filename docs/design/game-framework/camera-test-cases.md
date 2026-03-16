# Camera System Test Cases

Companion test cases for [camera.md](camera.md).

## Unit Tests

### TC-13.25.1.1 Priority Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 cameras: priority 1, 5, 10 | Brain selects camera with priority 10 | R-13.25.1 |
| 2 | Change priority 10 to 0 | Brain selects camera with priority 5 | R-13.25.1 |

### TC-13.25.2.1 Channel Mask Filter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 brains (mask 0x1, 0x2); 2 cameras (mask 0x1, 0x2) | Each brain sees only its channel camera | R-13.25.2 |

### TC-13.25.2.2 Fixed Update Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Brain in FixedUpdate mode | Updates sync with physics timestep | R-13.25.2 |

### TC-13.25.3.1 Follow 6 Binding Modes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Rotate target 90 deg in each binding mode | Offset transforms correctly per mode | R-13.25.3 |

### TC-13.25.3.2 Follow Damping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move target; observe camera over 10 frames | Position smooths toward target over time | R-13.25.3 |

### TC-13.25.4.1 Orbital Sphere Mode

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Input horizontal axis +1.0 for 60 frames | Camera orbits at configured radius | R-13.25.4 |

### TC-13.25.4.2 Orbital Three Ring

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Orbit vertically through three rings | Radius interpolates between top/middle/bottom | R-13.25.4 |

### TC-13.25.4.3 Orbital Recentering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Idle for wait_time seconds | Recentering begins; completes in completion_time | R-13.25.4 |

### TC-13.25.5.1 Third Person Shoulder Blend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blend camera_side from 0.0 to 1.0 | Smooth lateral transition | R-13.25.5 |

### TC-13.25.5.2 Third Person Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wall between camera and target | Camera retracts to wall distance | R-13.25.5 |

### TC-13.25.6.1 Hard Lock Zero Offset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at (10, 5, 3); evaluate camera | Camera position = (10, 5, 3) exactly | R-13.25.6 |

### TC-13.25.7.1 Position Composer Dead Zone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target screen position inside dead zone | Zero camera movement | R-13.25.7 |

### TC-13.25.7.2 Position Composer Hard Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at screen edge (hard limit) | Immediate position correction | R-13.25.7 |

### TC-13.25.8.1 Spline Dolly Fixed Speed

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | AutoDolly::FixedSpeed at speed 2.0 | Camera traverses at constant 2.0 u/s | R-13.25.8 |

### TC-13.25.8.2 Spline Dolly Nearest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target moves along spline | Camera tracks nearest spline point | R-13.25.8 |

### TC-13.25.9.1 Rotation Composer Dead Zone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Look-at target in dead zone | No rotation change | R-13.25.9 |

### TC-13.25.10.1 Hard Look At Centered

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at various positions | Target centered in frame each time | R-13.25.10 |

### TC-13.25.11.1 Pan Tilt Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Tilt input beyond 90 degrees | Tilt clamped at 90 degrees | R-13.25.11 |

### TC-13.25.12.1 Rotate With Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target rotates 45 degrees | Camera rotation matches | R-13.25.12 |

### TC-13.25.13.1 Spring Arm Retraction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Obstacle at 3 m on 5 m arm | Camera distance = 3 m | R-13.25.13 |

### TC-13.25.13.2 Spring Arm Extension

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Remove obstacle from 3 m | Camera extends back to 5 m target length | R-13.25.13 |

### TC-13.25.14.1 Deoccluder Pull Forward

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Obstacle between camera and target | Camera pulls forward past obstacle | R-13.25.14 |

### TC-13.25.14.2 Deoccluder Min Time

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Obstruction for 0.05s (min_time=0.1s) | Obstruction ignored | R-13.25.14 |

### TC-13.25.15.1 Decollider Terrain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera position below terrain surface | Camera pushed above surface | R-13.25.15 |

### TC-13.25.16.1 Blend 8 Curves

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger blend with each of 8 curve types | All 8 produce distinct interpolation curves | R-13.25.16 |

### TC-13.25.16.2 Blend Custom Pair

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom rule for cameras A->B; default for wildcard | Custom rule overrides default | R-13.25.16 |

### TC-13.25.17.1 Mixer Weighted Average

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 cameras at weights 1,2,1 | Output = weighted average of positions | R-13.25.17 |

### TC-13.25.17.2 Mixer Zero Weight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera with weight 0.0 | Contributes nothing to output | R-13.25.17 |

### TC-13.25.18.1 Perlin Mute

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Perlin shake with amplitude_gain=0 | Zero output displacement | R-13.25.18 |

### TC-13.25.18.2 Perlin Frequency Gain

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | frequency_gain=2.0 vs 1.0 | Double oscillation rate | R-13.25.18 |

### TC-13.25.19.1 Impulse Distance Attenuation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Listener at 5m vs 10m from source | Closer listener receives stronger shake | R-13.25.19 |

### TC-13.25.19.2 Impulse Outside Radius

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Listener beyond impulse radius | Zero shake received | R-13.25.19 |

### TC-13.25.20.1 Wave Oscillation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1 Hz sine wave; sample at 0.25s | Output = amplitude (peak of sine) | R-13.25.20 |

### TC-13.25.23.1 State Driven Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animation state changes to "combat" | Mapped combat camera activates | R-13.25.23 |

### TC-13.25.24.1 Clear Shot Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 child cameras with quality scores 0.3, 0.8, 0.5 | Camera with score 0.8 selected | R-13.25.24 |

### TC-13.25.26.1 Sequencer Playlist

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 cameras with hold times 2s, 3s, 1s | Play in order with correct hold durations | R-13.25.26 |

### TC-13.25.27.1 Target Group BBox

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 members at (0,0,0), (10,0,0), (0,10,0) | Group center = AABB center (5,5,0) | R-13.25.27 |

### TC-13.25.28.1 Group Framing Spread

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Members spread from 10m to 50m apart | FOV widens to frame all members | R-13.25.28 |

### TC-13.25.29.1 Confiner 2D Boundary

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target near polygon edge | Camera view stays within polygon | R-13.25.29 |

### TC-13.25.31.1 Follow Zoom FOV Adjust

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target at 50m; target_width=5.0 | FOV adjusts to maintain 5m screen width | R-13.25.31 |

### TC-13.25.37.1 Input Frame Independence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same input at 30 fps and 120 fps | Same final camera position | R-13.25.37 |

### TC-13.25.38.1 Cine Camera FOV

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Super 35 sensor + 50mm focal length | Vertical FOV = 2*atan(12.0/50.0) ~= 27.0 deg | R-13.25.38 |

## Integration Tests

### TC-NFR-13.25.1.I1 Split Screen 4 Brains

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 brains with cameras, blending, collision | Total evaluation < 4 ms | NFR-13.25.1 |

### TC-NFR-13.25.2.I1 Blend Smoothness 30fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1-second blend at 30 fps | No position jump > 0.01 units | NFR-13.25.2 |

### TC-NFR-13.25.2.I2 Blend Smoothness 120fps

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1-second blend at 120 fps | No rotation jump > 0.1 degrees | NFR-13.25.2 |

### TC-13.25.40.I1 PiP Multiple Viewports

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2 PiP viewports on desktop | Both render simultaneously | R-13.25.40 |

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

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Maximum inter-frame rotation jump during blend | Rotation delta | < 0.1 degrees | NFR-13.25.2 |

### TC-13.25.19.B1 Impulse Dispatch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10 sources, 4 listeners | Wall-clock time | < 0.1 ms | R-13.25.19 |

### TC-13.25.28.B1 Group Framing

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 group members | Wall-clock time | < 0.2 ms | R-13.25.28 |
