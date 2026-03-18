# Procedural Animation Test Cases

Companion test cases for [procedural.md](procedural.md).

## Unit Tests

### TC-9.3.1.1 Two-Bone IK Reach Target

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Target within reach, chain lengths (1.0, 1.0), target at (1.5, 0, 0)
   - **Expected:** End-effector within 0.01 units of target
2. **#2** — Target at exact reach limit (2.0, 0, 0)
   - **Expected:** End-effector within 0.01 units of target

### TC-9.3.1.2 Two-Bone IK Pole Vector

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Pole vector at (0, 0, 1), target at (1.5, 0, 0)
   - **Expected:** Elbow displaced toward +Z
2. **#2** — Pole vector rotated 90 degrees to (0, 0, -1)
   - **Expected:** Elbow displaced toward -Z

### TC-9.3.1.3 Two-Bone IK Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — Chain lengths (1.0, 1.0), target at (5.0, 0, 0)
   - **Expected:** Chain fully extended toward target, no NaN in joints
2. **#2** — Chain lengths (1.0, 1.0), target at (0.01, 0, 0)
   - **Expected:** Chain folds inward, no joint explosion

### TC-9.3.1.4 Two-Bone IK Weight Zero

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |
| 2 | R-9.3.1     |

1. **#1** — weight=0.0, any target position
   - **Expected:** Output pose identical to input pose (bitwise)
2. **#2** — weight=0.5, target at (1.5, 0, 0)
   - **Expected:** Output pose halfway between input and IK solution

### TC-9.3.2.1 CCD IK 6-Bone Convergence

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |
| 2 | R-9.3.2     |

1. **#1** — 6-bone chain, reachable target, 10 iterations
   - **Expected:** End-effector within 0.05 units of target
2. **#2** — 6-bone chain, reachable target, 3 iterations
   - **Expected:** End-effector closer to target than initial pose

### TC-9.3.2.2 CCD IK Angular Limits

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |
| 2 | R-9.3.2     |

1. **#1** — 6-bone chain, 30-degree per-joint limit, 10 iterations
   - **Expected:** No joint rotation exceeds 30 degrees
2. **#2** — 6-bone chain, 5-degree per-joint limit, target at edge of reach
   - **Expected:** All joints within 5-degree limit

### TC-9.3.2.3 CCD IK Unreachable

| # | Requirement |
|---|-------------|
| 1 | R-9.3.2     |

1. **#1** — 6-bone chain, target outside reach, 30-degree limits
   - **Expected:** Chain extends toward target, no joint exceeds limit

### TC-9.3.3.1 FABRIK 8-Bone

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |
| 2 | R-9.3.3     |

1. **#1** — 8-bone chain, reachable target, 6 iterations
   - **Expected:** End-effector within 0.05 units of target
2. **#2** — 8-bone chain, target at half reach
   - **Expected:** End-effector within 0.05 units, chain naturally curved

### TC-9.3.3.2 FABRIK Multi-End-Effector

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |
| 2 | R-9.3.3     |

1. **#1** — 8-leg spider skeleton, 8 ground targets
   - **Expected:** All 8 leg tips within 0.05 units of their targets
2. **#2** — 4-leg quadruped, 4 targets on uneven terrain
   - **Expected:** All 4 feet within 0.05 units of targets

### TC-9.3.3.3 FABRIK Priority

| # | Requirement |
|---|-------------|
| 1 | R-9.3.3     |

1. **#1** — Two effectors sharing joint, priority 1.0 vs 0.5
   - **Expected:** Higher-priority effector within 0.05, lower may exceed

### TC-9.3.4.1 Ragdoll Blend In

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Activate ragdoll, blend_in_duration=0.5s, dt=1/60
   - **Expected:** blend_weight=1.0 after 30 frames
2. **#2** — Activate ragdoll, blend_in_duration=0.0s
   - **Expected:** blend_weight=1.0 immediately (frame 1)

### TC-9.3.4.2 Ragdoll Recovery

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Trigger recovery, recovery_duration=1.0s, dt=1/60
   - **Expected:** blend_weight=0.0 after 60 frames
2. **#2** — Trigger recovery, recovery_duration=0.3s
   - **Expected:** blend_weight=0.0 after 18 frames

### TC-9.3.4.3 Ragdoll Partial Mask

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |
| 2 | R-9.3.4     |

1. **#1** — Mask=upper_body only, ragdoll active
   - **Expected:** Lower body bones: output == animation pose (delta < 0.001)
2. **#2** — Mask=upper_body only, ragdoll active
   - **Expected:** Upper body bones: output == ragdoll pose

### TC-9.3.4.4 Ragdoll No Discontinuity

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |

1. **#1** — Blend 0.0 to 1.0 over 0.5s at dt=1/60
   - **Expected:** Max frame-to-frame bone position delta < 0.1 units

### TC-9.3.5.1 Look-At 45 Degrees

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |
| 2 | R-9.3.5     |

1. **#1** — Target 45 degrees right of forward
   - **Expected:** Head yaw within 1 degree of 45
2. **#2** — Target 45 degrees left of forward
   - **Expected:** Head yaw within 1 degree of -45

### TC-9.3.5.2 Look-At Clamp

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |
| 2 | R-9.3.5     |

1. **#1** — Target 120 degrees right, max_yaw=90 degrees
   - **Expected:** Head yaw clamped at exactly 90 degrees
2. **#2** — Target 200 degrees right, max_yaw=90 degrees
   - **Expected:** Head yaw clamped at 90 degrees

### TC-9.3.5.3 Aim Alignment

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |

1. **#1** — Aim target at (10, 5, 0), weapon forward=+X
   - **Expected:** Weapon direction within 2 degrees of target direction

### TC-9.3.5.4 Aim Preserves Lower Body

| # | Requirement |
|---|-------------|
| 1 | R-9.3.5     |

1. **#1** — Aim constraint active on upper body
   - **Expected:** Lower body bones differ from base pose by < 0.001 units

### TC-9.3.7.1 Foot Placement Stairs

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |
| 2 | R-9.3.7     |

1. **#1** — Walk across 20 cm stair steps, 120 frames
   - **Expected:** No foot penetration > 1 cm, no foot float > 2 cm
2. **#2** — Walk across 10 cm stair steps
   - **Expected:** No penetration > 1 cm

### TC-9.3.7.2 Foot Placement Slope

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |
| 2 | R-9.3.7     |

1. **#1** — Walk 30-degree slope uphill, 120 frames
   - **Expected:** Foot contacts match slope surface within 2 cm
2. **#2** — Walk 30-degree slope downhill
   - **Expected:** Stride length adapts (shorter than flat ground)

### TC-9.3.7.3 Foot Placement Disabled

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |

1. **#1** — foot_placement_enabled=false, walk 60 frames
   - **Expected:** Zero raycasts issued to spatial index

### TC-9.3.8.1 Gait Biped Walk

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Biped at walk speed (1.5 m/s)
   - **Expected:** Alternating foot phase pattern: L-R-L-R
2. **#2** — Biped at walk speed
   - **Expected:** Each foot phase duration within 10% of gait period/2

### TC-9.3.8.2 Gait Quadruped Trot-Gallop

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Quadruped, speed increases from 2 to 8 m/s
   - **Expected:** Gait transitions from trot to gallop
2. **#2** — Quadruped at trot speed
   - **Expected:** Diagonal pairs move in sync

### TC-9.3.8.3 Gait Hexapod Tripod

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |

1. **#1** — Hexapod at walk speed
   - **Expected:** Tripod gait: 3 legs down, 3 legs up, alternating

### TC-9.3.8.4 Gait All ECS

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |
| 2 | R-9.3.8     |

1. **#1** — Query LocomotionProfile from ECS world
   - **Expected:** Component exists and is queryable
2. **#2** — Query GaitState and FootGroup from ECS world
   - **Expected:** Both components exist and are queryable

### TC-9.3.9.1 Physics Balance Upright

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |
| 2 | R-9.3.9     |

1. **#1** — Level ground, PID active, 120 frames
   - **Expected:** Torso angle within 2 degrees of upright
2. **#2** — Level ground, PID gains=(100, 10, 5)
   - **Expected:** Stable balance, no oscillation

### TC-9.3.9.2 Physics Stumble Recovery

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — Lateral impulse of 500 N*s applied
   - **Expected:** Stumble detected within 3 frames, recovery within 60 frames

### TC-9.3.9.3 Physics Slope Lean

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — 20-degree slope, walking uphill
   - **Expected:** Forward lean angle > 5 degrees

### TC-9.3.10.1 Attach Socket

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Attach entity to hand socket, 60 frames
   - **Expected:** Attached entity position within 0.01 units of socket each frame
2. **#2** — Detach entity from socket
   - **Expected:** Entity position no longer follows socket

### TC-9.3.10.2 Dismember Spawns Ragdoll

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Sever quadruped front-right leg
   - **Expected:** Detached entity has RagdollBlend component
2. **#2** — Sever quadruped front-right leg
   - **Expected:** Parent entity missing front-right bone subtree

### TC-9.3.10.3 Dismember Gait Adapt

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |
| 2 | R-9.3.10    |

1. **#1** — Sever quadruped front-right leg
   - **Expected:** Gait switches to three-legged pattern
2. **#2** — Sever hexapod leg
   - **Expected:** Gait adapts to 5-leg pattern

### TC-9.3.10.4 Dismember ECS Commands

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |

1. **#1** — Sever limb, trace command buffer
   - **Expected:** All dismemberment operations use ECS commands (not direct mutation)

### TC-9.3.11.1 Debug Vis Foot Targets

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |
| 2 | R-9.3.11    |

1. **#1** — foot_debug_vis=true, walk 30 frames
   - **Expected:** DebugDraw contains foot target markers each frame
2. **#2** — foot_debug_vis=false
   - **Expected:** Zero foot markers in DebugDraw

### TC-9.3.11.2 Debug Vis IK Chains

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |

1. **#1** — ik_debug_vis=true, IK active
   - **Expected:** Bone axes rendered for each IK chain joint

### TC-9.3.11.3 Debug Vis Per Entity

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |

1. **#1** — Toggle debug vis off for entity A, on for entity B
   - **Expected:** Entity A: zero overlays, Entity B: overlays present

## Integration Tests

### TC-9.3.1.I1 Pipeline Order

| # | Requirement              |
|---|--------------------------|
| 1 | R-9.3.1 through R-9.3.11 |

1. **#1** — Full procedural pipeline, 1 frame
   - **Expected:** Systems execute: look-at, foot, IK, ragdoll, secondary (in order)

### TC-9.3.1.I2 500 Two-Bone GPU

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |

1. **#1** — 500 two-bone IK chains in single GPU dispatch
   - **Expected:** All end-effectors within 0.01 units of targets

### TC-9.3.1.I3 IK After State Machine

| # | Requirement |
|---|-------------|
| 1 | R-9.3.1     |

1. **#1** — Walk animation + hand IK target
   - **Expected:** IK modifies the blended animation pose, not raw clip data

### TC-9.3.7.I1 Foot Placement Batch Raycast

| # | Requirement |
|---|-------------|
| 1 | R-9.3.7     |

1. **#1** — 100 characters with foot placement active
   - **Expected:** Single batch raycast to shared BVH (not 100 individual queries)

### TC-9.3.4.I1 Ragdoll Physics Integration

| # | Requirement |
|---|-------------|
| 1 | R-9.3.4     |

1. **#1** — Full ragdoll with physics step, 60 frames
   - **Expected:** All bones respond to gravity (Y decreasing)

### TC-9.3.8.I1 Locomotion All Topologies

| # | Requirement |
|---|-------------|
| 1 | R-9.3.8     |

1. **#1** — Biped, quadruped, hexapod on same terrain
   - **Expected:** Each uses correct gait pattern for its topology

### TC-9.3.9.I1 Physics to Animated Transition

| # | Requirement |
|---|-------------|
| 1 | R-9.3.9     |

1. **#1** — Transition physics->animated over 0.3s
   - **Expected:** Max frame-to-frame bone delta < 0.1 units (no visible pop)

### TC-9.3.10.I1 Dismember Runtime Full

| # | Requirement |
|---|-------------|
| 1 | R-9.3.10    |

1. **#1** — Sever wing at runtime
   - **Expected:** Ragdoll spawns for detached part, locomotion adapts

## Benchmarks

### TC-9.3.1.B1 Two-Bone IK GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 two-bone chains, single GPU dispatch | GPU time | < 0.5 ms | US-9.3.1.2 |

### TC-9.3.2.B1 CCD IK GPU

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 chains, 8 iterations, GPU dispatch | GPU time | < 1.0 ms | US-9.3.2.1 |

### TC-9.3.3.B1 FABRIK

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 chains, 6 iterations | CPU time | < 0.8 ms | US-9.3.3.1 |

### TC-9.3.7.B1 Foot Placement Raycasts

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 characters, batch raycast | CPU time | < 0.3 ms | US-9.3.7.2 |

### TC-9.3.4.B1 Ragdoll Blend

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50 characters with ragdoll blend | CPU time | < 0.2 ms | US-9.3.4.1 |

### TC-9.3.5.B1 Look-At and Aim

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200 look-at + aim constraints | CPU time | < 0.2 ms | US-9.3.5.1 |

### TC-9.3.8.B1 Locomotion System

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 creatures with procedural locomotion | CPU time | < 0.5 ms | US-9.3.8.1 |

### TC-9.3.11.B1 Full Pipeline

| # | Metric         | Target   | Requirement              |
|---|----------------|----------|--------------------------|
| 1 | Total CPU time | < 2.0 ms | R-9.3.1 through R-9.3.11 |

1. **1** — 100 characters, all procedural features active

## Shipping Build Verification

### TC-9.3.11.S1 Debug Stripped

| # | Requirement |
|---|-------------|
| 1 | R-9.3.11    |
| 2 | R-9.3.11    |

1. **#1** — Compile shipping build, inspect binary
   - **Expected:** `locomotion_diagnostics_system` absent from binary
2. **#2** — Compile shipping build, inspect binary
   - **Expected:** `LocomotionDebugVis` and `LocomotionMetrics` types absent
