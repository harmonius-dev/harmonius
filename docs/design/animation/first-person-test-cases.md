# First-Person Animation Test Cases

Companion test cases for [first-person.md](first-person.md).

## Unit Tests

### TC-9.6.1.1 Spring Damper Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpringDamper(stiffness=100, damping=1.0, mass=1.0), target=1.0, 120 frames at dt=1/60 | position within 0.01 of 1.0 after 2 seconds | R-9.6.1 |
| 2 | SpringDamper(stiffness=100, damping=1.0, mass=1.0), target=1.0 | No overshoot (position never exceeds 1.0) for critically damped | R-9.6.1 |
| 3 | SpringDamper(stiffness=100, damping=0.5, mass=1.0), target=1.0 | Underdamped oscillation: position exceeds 1.0 at least once | R-9.6.1 |

### TC-9.6.1.2 Spring Damper Impulse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpringDamper at rest, apply_impulse(10.0), 120 frames | Velocity spikes > 0 at frame 1, returns to < 0.01 by frame 120 | R-9.6.1 |
| 2 | SpringDamper at rest, apply_impulse(0.0) | Velocity remains 0.0 | R-9.6.1 |

### TC-9.6.1.3 Spring Damper 3D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpringDamper3D, target=(1,2,3), 120 frames | Each axis within 0.01 of target independently | R-9.6.1 |
| 2 | SpringDamper3D, target=(1,0,0), 120 frames | Y and Z axes remain at 0.0, X converges to 1.0 | R-9.6.1 |

### TC-9.6.1.4 Spring Damper Quaternion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpringDamperQuat, target=90 deg Y rotation, 120 frames | Rotation within 1 degree of target after 2 seconds | R-9.6.1 |
| 2 | SpringDamperQuat, target=180 deg Z rotation | Converges without gimbal lock (no NaN in output) | R-9.6.1 |

### TC-9.6.1.5 Head Bob Frequency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk at constant speed, bob_frequency=2.0 Hz, 60 frames at dt=1/60 | Exactly 2 full oscillation cycles in 1 second | R-9.6.1 |
| 2 | Walk at constant speed, bob_frequency=3.0 Hz, 60 frames | Exactly 3 full oscillation cycles in 1 second | R-9.6.1 |

### TC-9.6.1.6 Landing Impact

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | fall_distance=3.0 m, landing_snap_scale=0.1 | Camera snap magnitude = 0.3 (proportional), recovery to 0 within 60 frames | R-9.6.1 |
| 2 | fall_distance=0.0 m (no fall) | Zero landing snap applied | R-9.6.1 |

### TC-9.6.1.7 Lean Offset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | lean_active=true, lean_direction=1.0, lean_max_offset=0.5 | Camera X offset converges to 0.5 within 30 frames | R-9.6.1 |
| 2 | lean_active=false after being active | Camera X offset returns to 0.0 within 30 frames | R-9.6.1 |

### TC-9.6.1.8 Strafe Tilt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Strafe left at constant speed, tilt_max_degrees=5.0 | Camera roll converges to 5.0 degrees within 30 frames | R-9.6.1 |
| 2 | Stop strafing after tilt | Camera roll returns to 0.0 degrees within 30 frames | R-9.6.1 |

### TC-9.6.1.9 Viewmodel FOV

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | world_fov=110, viewmodel_fov=70 | Viewmodel projection matrix uses 70-degree FOV | R-9.6.1 |
| 2 | world_fov=110, viewmodel_fov=70 | World projection matrix uses 110-degree FOV | R-9.6.1 |

### TC-9.6.2.1 Sway Opposite Direction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mouse delta=(100,0) pixels right | Weapon position.x < 0 (displaced left) | R-9.6.2 |
| 2 | Mouse delta=(0,100) pixels down | Weapon position.y > 0 (displaced up) | R-9.6.2 |

### TC-9.6.2.2 Sway Mass Scaling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | mass=2.0, mouse delta=(50,0), 30 frames | Sway displacement D_heavy | R-9.6.2 |
| 2 | mass=0.5, mouse delta=(50,0), 30 frames | Sway displacement D_light, D_heavy > D_light (more inertia = larger lag) | R-9.6.2 |

### TC-9.6.2.3 Bob Amplitude

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk at constant speed, vertical_amplitude=0.02 | Measured vertical bob peak within 5% of 0.02 | R-9.6.2 |
| 2 | Standing still, vertical_amplitude=0.02 | Zero vertical bob displacement | R-9.6.2 |

### TC-9.6.2.4 Sprint Tilt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | speed=10.0, speed_threshold=8.0, tilt_rotation=(15,5,0) | Weapon rotation converges to (15,5,0) degrees | R-9.6.2 |
| 2 | speed=6.0, speed_threshold=8.0 | Weapon rotation stays at rest (0,0,0) | R-9.6.2 |

### TC-9.6.3.1 Recoil Non-Repetitive

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire 10 shots, randomization_range=(0.8,1.2) | Each kick differs from previous by >= 10% in rotation or translation | R-9.6.3 |
| 2 | Fire 10 shots, randomization_range=(1.0,1.0) (no randomization) | All kicks identical | R-9.6.3 |

### TC-9.6.3.2 Recoil Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fire single shot, wait 60 frames at dt=1/60 | kick_position within 0.01 of (0,0,0) | R-9.6.3 |
| 2 | Fire single shot, wait 0 frames | kick_position at peak displacement | R-9.6.3 |

### TC-9.6.3.3 ADS Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Press ADS, transition_duration=0.3s, dt=1/60 | blend_alpha=1.0 after 18 frames | R-9.6.3 |
| 2 | Release ADS after fully aimed, transition_duration=0.3s | blend_alpha=0.0 after 18 frames | R-9.6.3 |

### TC-9.6.3.4 ADS Sway Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ADS active, sway_multiplier=0.3, mouse delta=(50,0) | Sway amplitude = 30% of hip-fire sway | R-9.6.3 |
| 2 | ADS active, sway_multiplier=0.0 | Zero sway displacement | R-9.6.3 |

### TC-9.6.3.5 ADS Sight Switch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | In ADS, switch sight from index 0 to index 1 | active_sight_index=1, position moves to sights[1].position | R-9.6.3 |
| 2 | In ADS with fov_override=4x on sight 1 | Viewmodel FOV interpolates to sight fov_override | R-9.6.3 |

### TC-9.6.4.1 Equip Holster Sequence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch weapon from equipped state | Phase sequence: Equipped->Holstering->Holstered->Drawing->Equipped | R-9.6.4 |
| 2 | Switch weapon, verify timing | Holster completes in holster_duration, draw completes in draw_duration | R-9.6.4 |

### TC-9.6.4.2 Inspect Rotation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger inspect, duration=2.0s | Weapon rotates through rotation_curve, returns to rest after 2.0s | R-9.6.4 |
| 2 | Interrupt inspect with fire input | Inspect canceled, weapon returns to hip position | R-9.6.4 |

### TC-9.6.4.3 Dual Wield Alternating

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DualFireMode::Alternating, fire 4 shots | Fire sequence: Right, Left, Right, Left | R-9.6.4 |
| 2 | DualFireMode::Alternating, fire 1 shot | Only right hand fires | R-9.6.4 |

### TC-9.6.4.4 Dual Wield Simultaneous

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DualFireMode::Simultaneous, fire 1 input | Both hands fire on same frame | R-9.6.4 |
| 2 | DualFireMode::Simultaneous, fire 3 inputs | 6 total shots (3 per hand) | R-9.6.4 |

### TC-9.6.4.5 Dual Wield Independent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DualFireMode::Independent, primary trigger pressed | Only right hand fires | R-9.6.4 |
| 2 | DualFireMode::Independent, secondary trigger pressed | Only left hand fires | R-9.6.4 |
| 3 | Both triggers pressed simultaneously | Both hands fire independently | R-9.6.4 |

## Integration Tests

### TC-9.6.1.I1 Camera with Controller

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk character for 300 frames | Head-bob oscillation synchronized with locomotion gait cycle | R-9.6.1 |
| 2 | Character steps off 1 m ledge | Landing snap proportional to fall distance, recovery within 30 frames | R-9.6.1 |

### TC-9.6.2.I1 Full Weapon Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk, aim, fire 10 shots, release ADS over 300 frames | All spring outputs finite (no NaN, no Inf in any transform) | R-9.6.2, R-9.6.3 |
| 2 | Walk, aim, fire 10 shots, release ADS | Final viewmodel position = hip_position (all springs at rest) after 5s | R-9.6.2, R-9.6.3 |

### TC-9.6.4.I1 Dual Wield Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DualWield enabled, 60 frames | Exactly 2 viewmodel draw calls per frame | R-9.6.4 |
| 2 | DualWield enabled, mouse input applied | Each weapon has independent sway displacement | R-9.6.4 |

### TC-9.6.3.I1 Scope RTT Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile config, ADS with magnified optic | ScopeResolution::Half, RTT active at blend_alpha=1.0 | R-9.6.3 |

### TC-9.6.3.I2 Scope RTT Desktop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Desktop config, ADS with magnified optic | ScopeResolution::Full, RTT active at blend_alpha=1.0 | R-9.6.3 |

### TC-9.6.2.I2 Weapon Data Asset

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load weapon asset with sway_stiffness=200, mass=1.5 | WeaponSwayState springs initialized with stiffness=200, mass=1.5 | R-9.6.2 |
| 2 | Modify sway_stiffness in asset from 200 to 100 | Weapon feel changes without code recompilation | R-9.6.2 |

### TC-9.6.2.I3 No-Code Tuning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Modify bob_amplitude from 0.02 to 0.04 in visual editor | Measured bob doubles without code change | R-9.6.2 |

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
