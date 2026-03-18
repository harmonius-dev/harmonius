# First-Person Animation Test Cases

Companion test cases for [first-person.md](first-person.md).

## Unit Tests

### TC-9.6.1.1 Spring Damper Convergence

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |
| 3 | R-9.6.1     |

1. **#1** — SpringDamper(stiffness=100, damping=1.0, mass=1.0), target=1.0, 120 frames at dt=1/60
   - **Expected:** position within 0.01 of 1.0 after 2 seconds
2. **#2** — SpringDamper(stiffness=100, damping=1.0, mass=1.0), target=1.0
   - **Expected:** No overshoot (position never exceeds 1.0) for critically damped
3. **#3** — SpringDamper(stiffness=100, damping=0.5, mass=1.0), target=1.0
   - **Expected:** Underdamped oscillation: position exceeds 1.0 at least once

### TC-9.6.1.2 Spring Damper Impulse

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — SpringDamper at rest, apply_impulse(10.0), 120 frames
   - **Expected:** Velocity spikes > 0 at frame 1, returns to < 0.01 by frame 120
2. **#2** — SpringDamper at rest, apply_impulse(0.0)
   - **Expected:** Velocity remains 0.0

### TC-9.6.1.3 Spring Damper 3D

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — SpringDamper3D, target=(1,2,3), 120 frames
   - **Expected:** Each axis within 0.01 of target independently
2. **#2** — SpringDamper3D, target=(1,0,0), 120 frames
   - **Expected:** Y and Z axes remain at 0.0, X converges to 1.0

### TC-9.6.1.4 Spring Damper Quaternion

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — SpringDamperQuat, target=90 deg Y rotation, 120 frames
   - **Expected:** Rotation within 1 degree of target after 2 seconds
2. **#2** — SpringDamperQuat, target=180 deg Z rotation
   - **Expected:** Converges without gimbal lock (no NaN in output)

### TC-9.6.1.5 Head Bob Frequency

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — Walk at constant speed, bob_frequency=2.0 Hz, 60 frames at dt=1/60
   - **Expected:** Exactly 2 full oscillation cycles in 1 second
2. **#2** — Walk at constant speed, bob_frequency=3.0 Hz, 60 frames
   - **Expected:** Exactly 3 full oscillation cycles in 1 second

### TC-9.6.1.6 Landing Impact

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — fall_distance=3.0 m, landing_snap_scale=0.1
   - **Expected:** Camera snap magnitude = 0.3 (proportional), recovery to 0 within 60 frames
2. **#2** — fall_distance=0.0 m (no fall)
   - **Expected:** Zero landing snap applied

### TC-9.6.1.7 Lean Offset

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — lean_active=true, lean_direction=1.0, lean_max_offset=0.5
   - **Expected:** Camera X offset converges to 0.5 within 30 frames
2. **#2** — lean_active=false after being active
   - **Expected:** Camera X offset returns to 0.0 within 30 frames

### TC-9.6.1.8 Strafe Tilt

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — Strafe left at constant speed, tilt_max_degrees=5.0
   - **Expected:** Camera roll converges to 5.0 degrees within 30 frames
2. **#2** — Stop strafing after tilt
   - **Expected:** Camera roll returns to 0.0 degrees within 30 frames

### TC-9.6.1.9 Viewmodel FOV

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — world_fov=110, viewmodel_fov=70
   - **Expected:** Viewmodel projection matrix uses 70-degree FOV
2. **#2** — world_fov=110, viewmodel_fov=70
   - **Expected:** World projection matrix uses 110-degree FOV

### TC-9.6.2.1 Sway Opposite Direction

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — Mouse delta=(100,0) pixels right
   - **Expected:** Weapon position.x < 0 (displaced left)
2. **#2** — Mouse delta=(0,100) pixels down
   - **Expected:** Weapon position.y > 0 (displaced up)

### TC-9.6.2.2 Sway Mass Scaling

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — mass=2.0, mouse delta=(50,0), 30 frames
   - **Expected:** Sway displacement D_heavy
2. **#2** — mass=0.5, mouse delta=(50,0), 30 frames
   - **Expected:** Sway displacement D_light, D_heavy > D_light (more inertia = larger lag)

### TC-9.6.2.3 Bob Amplitude

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — Walk at constant speed, vertical_amplitude=0.02
   - **Expected:** Measured vertical bob peak within 5% of 0.02
2. **#2** — Standing still, vertical_amplitude=0.02
   - **Expected:** Zero vertical bob displacement

### TC-9.6.2.4 Sprint Tilt

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — speed=10.0, speed_threshold=8.0, tilt_rotation=(15,5,0)
   - **Expected:** Weapon rotation converges to (15,5,0) degrees
2. **#2** — speed=6.0, speed_threshold=8.0
   - **Expected:** Weapon rotation stays at rest (0,0,0)

### TC-9.6.3.1 Recoil Non-Repetitive

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — Fire 10 shots, randomization_range=(0.8,1.2)
   - **Expected:** Each kick differs from previous by >= 10% in rotation or translation
2. **#2** — Fire 10 shots, randomization_range=(1.0,1.0) (no randomization)
   - **Expected:** All kicks identical

### TC-9.6.3.2 Recoil Recovery

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — Fire single shot, wait 60 frames at dt=1/60
   - **Expected:** kick_position within 0.01 of (0,0,0)
2. **#2** — Fire single shot, wait 0 frames
   - **Expected:** kick_position at peak displacement

### TC-9.6.3.3 ADS Transition

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — Press ADS, transition_duration=0.3s, dt=1/60
   - **Expected:** blend_alpha=1.0 after 18 frames
2. **#2** — Release ADS after fully aimed, transition_duration=0.3s
   - **Expected:** blend_alpha=0.0 after 18 frames

### TC-9.6.3.4 ADS Sway Reduction

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — ADS active, sway_multiplier=0.3, mouse delta=(50,0)
   - **Expected:** Sway amplitude = 30% of hip-fire sway
2. **#2** — ADS active, sway_multiplier=0.0
   - **Expected:** Zero sway displacement

### TC-9.6.3.5 ADS Sight Switch

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |
| 2 | R-9.6.3     |

1. **#1** — In ADS, switch sight from index 0 to index 1
   - **Expected:** active_sight_index=1, position moves to sights[1].position
2. **#2** — In ADS with fov_override=4x on sight 1
   - **Expected:** Viewmodel FOV interpolates to sight fov_override

### TC-9.6.4.1 Equip Holster Sequence

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — Switch weapon from equipped state
   - **Expected:** Phase sequence: Equipped->Holstering->Holstered->Drawing->Equipped
2. **#2** — Switch weapon, verify timing
   - **Expected:** Holster completes in holster_duration, draw completes in draw_duration

### TC-9.6.4.2 Inspect Rotation

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — Trigger inspect, duration=2.0s
   - **Expected:** Weapon rotates through rotation_curve, returns to rest after 2.0s
2. **#2** — Interrupt inspect with fire input
   - **Expected:** Inspect canceled, weapon returns to hip position

### TC-9.6.4.3 Dual Wield Alternating

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — DualFireMode::Alternating, fire 4 shots
   - **Expected:** Fire sequence: Right, Left, Right, Left
2. **#2** — DualFireMode::Alternating, fire 1 shot
   - **Expected:** Only right hand fires

### TC-9.6.4.4 Dual Wield Simultaneous

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — DualFireMode::Simultaneous, fire 1 input
   - **Expected:** Both hands fire on same frame
2. **#2** — DualFireMode::Simultaneous, fire 3 inputs
   - **Expected:** 6 total shots (3 per hand)

### TC-9.6.4.5 Dual Wield Independent

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |
| 3 | R-9.6.4     |

1. **#1** — DualFireMode::Independent, primary trigger pressed
   - **Expected:** Only right hand fires
2. **#2** — DualFireMode::Independent, secondary trigger pressed
   - **Expected:** Only left hand fires
3. **#3** — Both triggers pressed simultaneously
   - **Expected:** Both hands fire independently

## Integration Tests

### TC-9.6.1.I1 Camera with Controller

| # | Requirement |
|---|-------------|
| 1 | R-9.6.1     |
| 2 | R-9.6.1     |

1. **#1** — Walk character for 300 frames
   - **Expected:** Head-bob oscillation synchronized with locomotion gait cycle
2. **#2** — Character steps off 1 m ledge
   - **Expected:** Landing snap proportional to fall distance, recovery within 30 frames

### TC-9.6.2.I1 Full Weapon Pipeline

| # | Requirement      |
|---|------------------|
| 1 | R-9.6.2, R-9.6.3 |
| 2 | R-9.6.2, R-9.6.3 |

1. **#1** — Walk, aim, fire 10 shots, release ADS over 300 frames
   - **Expected:** All spring outputs finite (no NaN, no Inf in any transform)
2. **#2** — Walk, aim, fire 10 shots, release ADS
   - **Expected:** Final viewmodel position = hip_position (all springs at rest) after 5s

### TC-9.6.4.I1 Dual Wield Render

| # | Requirement |
|---|-------------|
| 1 | R-9.6.4     |
| 2 | R-9.6.4     |

1. **#1** — DualWield enabled, 60 frames
   - **Expected:** Exactly 2 viewmodel draw calls per frame
2. **#2** — DualWield enabled, mouse input applied
   - **Expected:** Each weapon has independent sway displacement

### TC-9.6.3.I1 Scope RTT Mobile

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |

1. **#1** — Mobile config, ADS with magnified optic
   - **Expected:** ScopeResolution::Half, RTT active at blend_alpha=1.0

### TC-9.6.3.I2 Scope RTT Desktop

| # | Requirement |
|---|-------------|
| 1 | R-9.6.3     |

1. **#1** — Desktop config, ADS with magnified optic
   - **Expected:** ScopeResolution::Full, RTT active at blend_alpha=1.0

### TC-9.6.2.I2 Weapon Data Asset

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |
| 2 | R-9.6.2     |

1. **#1** — Load weapon asset with sway_stiffness=200, mass=1.5
   - **Expected:** WeaponSwayState springs initialized with stiffness=200, mass=1.5
2. **#2** — Modify sway_stiffness in asset from 200 to 100
   - **Expected:** Weapon feel changes without code recompilation

### TC-9.6.2.I3 No-Code Tuning

| # | Requirement |
|---|-------------|
| 1 | R-9.6.2     |

1. **#1** — Modify bob_amplitude from 0.02 to 0.04 in visual editor
   - **Expected:** Measured bob doubles without code change

## Benchmarks

### TC-9.6.1.B1 Camera Rig Performance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 5 camera springs evaluated per frame | CPU time | < 0.05 ms | R-9.6.1 |

### TC-9.6.2.B1 Single Weapon Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sway + bob + recoil + ADS per weapon | CPU time | < 0.05 ms | R-9.6.2, R-9.6.3 |

### TC-9.6.4.B1 Dual Wield Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2x weapon + fire routing | CPU time | < 0.1 ms | R-9.6.4 |

### TC-9.6.1.B2 Viewmodel Render Single

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single viewmodel rendering | Draw calls | 1 draw call | R-9.6.1 |

### TC-9.6.4.B2 Viewmodel Render Dual

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Dual viewmodel rendering | Draw calls | 2 draw calls | R-9.6.4 |

### TC-9.6.3.B1 Scope RTT

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Scope RTT full-res, desktop | GPU time | < 1.0 ms | R-9.6.3 |
| 2 | Scope RTT half-res, mobile | GPU time | < 0.5 ms | R-9.6.3 |
