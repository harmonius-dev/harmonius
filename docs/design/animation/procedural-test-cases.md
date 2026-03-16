# Procedural Animation Test Cases

Companion test cases for [procedural.md](procedural.md).

## Unit Tests

### TC-9.3.1.1 Two-Bone IK Reach Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target within reach, chain lengths (1.0, 1.0), target at (1.5, 0, 0) | End-effector within 0.01 units of target | R-9.3.1 |
| 2 | Target at exact reach limit (2.0, 0, 0) | End-effector within 0.01 units of target | R-9.3.1 |

### TC-9.3.1.2 Two-Bone IK Pole Vector

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Pole vector at (0, 0, 1), target at (1.5, 0, 0) | Elbow displaced toward +Z | R-9.3.1 |
| 2 | Pole vector rotated 90 degrees to (0, 0, -1) | Elbow displaced toward -Z | R-9.3.1 |

### TC-9.3.1.3 Two-Bone IK Unreachable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chain lengths (1.0, 1.0), target at (5.0, 0, 0) | Chain fully extended toward target, no NaN in joints | R-9.3.1 |
| 2 | Chain lengths (1.0, 1.0), target at (0.01, 0, 0) | Chain folds inward, no joint explosion | R-9.3.1 |

### TC-9.3.1.4 Two-Bone IK Weight Zero

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | weight=0.0, any target position | Output pose identical to input pose (bitwise) | R-9.3.1 |
| 2 | weight=0.5, target at (1.5, 0, 0) | Output pose halfway between input and IK solution | R-9.3.1 |

### TC-9.3.2.1 CCD IK 6-Bone Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 6-bone chain, reachable target, 10 iterations | End-effector within 0.05 units of target | R-9.3.2 |
| 2 | 6-bone chain, reachable target, 3 iterations | End-effector closer to target than initial pose | R-9.3.2 |

### TC-9.3.2.2 CCD IK Angular Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 6-bone chain, 30-degree per-joint limit, 10 iterations | No joint rotation exceeds 30 degrees | R-9.3.2 |
| 2 | 6-bone chain, 5-degree per-joint limit, target at edge of reach | All joints within 5-degree limit | R-9.3.2 |

### TC-9.3.2.3 CCD IK Unreachable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 6-bone chain, target outside reach, 30-degree limits | Chain extends toward target, no joint exceeds limit | R-9.3.2 |

### TC-9.3.3.1 FABRIK 8-Bone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8-bone chain, reachable target, 6 iterations | End-effector within 0.05 units of target | R-9.3.3 |
| 2 | 8-bone chain, target at half reach | End-effector within 0.05 units, chain naturally curved | R-9.3.3 |

### TC-9.3.3.2 FABRIK Multi-End-Effector

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8-leg spider skeleton, 8 ground targets | All 8 leg tips within 0.05 units of their targets | R-9.3.3 |
| 2 | 4-leg quadruped, 4 targets on uneven terrain | All 4 feet within 0.05 units of targets | R-9.3.3 |

### TC-9.3.3.3 FABRIK Priority

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two effectors sharing joint, priority 1.0 vs 0.5 | Higher-priority effector within 0.05, lower may exceed | R-9.3.3 |

### TC-9.3.4.1 Ragdoll Blend In

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate ragdoll, blend_in_duration=0.5s, dt=1/60 | blend_weight=1.0 after 30 frames | R-9.3.4 |
| 2 | Activate ragdoll, blend_in_duration=0.0s | blend_weight=1.0 immediately (frame 1) | R-9.3.4 |

### TC-9.3.4.2 Ragdoll Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger recovery, recovery_duration=1.0s, dt=1/60 | blend_weight=0.0 after 60 frames | R-9.3.4 |
| 2 | Trigger recovery, recovery_duration=0.3s | blend_weight=0.0 after 18 frames | R-9.3.4 |

### TC-9.3.4.3 Ragdoll Partial Mask

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mask=upper_body only, ragdoll active | Lower body bones: output == animation pose (delta < 0.001) | R-9.3.4 |
| 2 | Mask=upper_body only, ragdoll active | Upper body bones: output == ragdoll pose | R-9.3.4 |

### TC-9.3.4.4 Ragdoll No Discontinuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Blend 0.0 to 1.0 over 0.5s at dt=1/60 | Max frame-to-frame bone position delta < 0.1 units | R-9.3.4 |

### TC-9.3.5.1 Look-At 45 Degrees

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target 45 degrees right of forward | Head yaw within 1 degree of 45 | R-9.3.5 |
| 2 | Target 45 degrees left of forward | Head yaw within 1 degree of -45 | R-9.3.5 |

### TC-9.3.5.2 Look-At Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Target 120 degrees right, max_yaw=90 degrees | Head yaw clamped at exactly 90 degrees | R-9.3.5 |
| 2 | Target 200 degrees right, max_yaw=90 degrees | Head yaw clamped at 90 degrees | R-9.3.5 |

### TC-9.3.5.3 Aim Alignment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Aim target at (10, 5, 0), weapon forward=+X | Weapon direction within 2 degrees of target direction | R-9.3.5 |

### TC-9.3.5.4 Aim Preserves Lower Body

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Aim constraint active on upper body | Lower body bones differ from base pose by < 0.001 units | R-9.3.5 |

### TC-9.3.7.1 Foot Placement Stairs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk across 20 cm stair steps, 120 frames | No foot penetration > 1 cm, no foot float > 2 cm | R-9.3.7 |
| 2 | Walk across 10 cm stair steps | No penetration > 1 cm | R-9.3.7 |

### TC-9.3.7.2 Foot Placement Slope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk 30-degree slope uphill, 120 frames | Foot contacts match slope surface within 2 cm | R-9.3.7 |
| 2 | Walk 30-degree slope downhill | Stride length adapts (shorter than flat ground) | R-9.3.7 |

### TC-9.3.7.3 Foot Placement Disabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | foot_placement_enabled=false, walk 60 frames | Zero raycasts issued to spatial index | R-9.3.7 |

### TC-9.3.8.1 Gait Biped Walk

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Biped at walk speed (1.5 m/s) | Alternating foot phase pattern: L-R-L-R | R-9.3.8 |
| 2 | Biped at walk speed | Each foot phase duration within 10% of gait period/2 | R-9.3.8 |

### TC-9.3.8.2 Gait Quadruped Trot-Gallop

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Quadruped, speed increases from 2 to 8 m/s | Gait transitions from trot to gallop | R-9.3.8 |
| 2 | Quadruped at trot speed | Diagonal pairs move in sync | R-9.3.8 |

### TC-9.3.8.3 Gait Hexapod Tripod

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hexapod at walk speed | Tripod gait: 3 legs down, 3 legs up, alternating | R-9.3.8 |

### TC-9.3.8.4 Gait All ECS

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query LocomotionProfile from ECS world | Component exists and is queryable | R-9.3.8 |
| 2 | Query GaitState and FootGroup from ECS world | Both components exist and are queryable | R-9.3.8 |

### TC-9.3.9.1 Physics Balance Upright

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Level ground, PID active, 120 frames | Torso angle within 2 degrees of upright | R-9.3.9 |
| 2 | Level ground, PID gains=(100, 10, 5) | Stable balance, no oscillation | R-9.3.9 |

### TC-9.3.9.2 Physics Stumble Recovery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lateral impulse of 500 N*s applied | Stumble detected within 3 frames, recovery within 60 frames | R-9.3.9 |

### TC-9.3.9.3 Physics Slope Lean

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20-degree slope, walking uphill | Forward lean angle > 5 degrees | R-9.3.9 |

### TC-9.3.10.1 Attach Socket

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Attach entity to hand socket, 60 frames | Attached entity position within 0.01 units of socket each frame | R-9.3.10 |
| 2 | Detach entity from socket | Entity position no longer follows socket | R-9.3.10 |

### TC-9.3.10.2 Dismember Spawns Ragdoll

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever quadruped front-right leg | Detached entity has RagdollBlend component | R-9.3.10 |
| 2 | Sever quadruped front-right leg | Parent entity missing front-right bone subtree | R-9.3.10 |

### TC-9.3.10.3 Dismember Gait Adapt

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever quadruped front-right leg | Gait switches to three-legged pattern | R-9.3.10 |
| 2 | Sever hexapod leg | Gait adapts to 5-leg pattern | R-9.3.10 |

### TC-9.3.10.4 Dismember ECS Commands

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever limb, trace command buffer | All dismemberment operations use ECS commands (not direct mutation) | R-9.3.10 |

### TC-9.3.11.1 Debug Vis Foot Targets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | foot_debug_vis=true, walk 30 frames | DebugDraw contains foot target markers each frame | R-9.3.11 |
| 2 | foot_debug_vis=false | Zero foot markers in DebugDraw | R-9.3.11 |

### TC-9.3.11.2 Debug Vis IK Chains

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ik_debug_vis=true, IK active | Bone axes rendered for each IK chain joint | R-9.3.11 |

### TC-9.3.11.3 Debug Vis Per Entity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Toggle debug vis off for entity A, on for entity B | Entity A: zero overlays, Entity B: overlays present | R-9.3.11 |

## Integration Tests

### TC-9.3.1.I1 Pipeline Order

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full procedural pipeline, 1 frame | Systems execute: look-at, foot, IK, ragdoll, secondary (in order) | R-9.3.1 through R-9.3.11 |

### TC-9.3.1.I2 500 Two-Bone GPU

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 two-bone IK chains in single GPU dispatch | All end-effectors within 0.01 units of targets | R-9.3.1 |

### TC-9.3.1.I3 IK After State Machine

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk animation + hand IK target | IK modifies the blended animation pose, not raw clip data | R-9.3.1 |

### TC-9.3.7.I1 Foot Placement Batch Raycast

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 characters with foot placement active | Single batch raycast to shared BVH (not 100 individual queries) | R-9.3.7 |

### TC-9.3.4.I1 Ragdoll Physics Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full ragdoll with physics step, 60 frames | All bones respond to gravity (Y decreasing) | R-9.3.4 |

### TC-9.3.8.I1 Locomotion All Topologies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Biped, quadruped, hexapod on same terrain | Each uses correct gait pattern for its topology | R-9.3.8 |

### TC-9.3.9.I1 Physics to Animated Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition physics->animated over 0.3s | Max frame-to-frame bone delta < 0.1 units (no visible pop) | R-9.3.9 |

### TC-9.3.10.I1 Dismember Runtime Full

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever wing at runtime | Ragdoll spawns for detached part, locomotion adapts | R-9.3.10 |

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

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 characters, all procedural features active | Total CPU time | < 2.0 ms | R-9.3.1 through R-9.3.11 |

## Shipping Build Verification

### TC-9.3.11.S1 Debug Stripped

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile shipping build, inspect binary | `locomotion_diagnostics_system` absent from binary | R-9.3.11 |
| 2 | Compile shipping build, inspect binary | `LocomotionDebugVis` and `LocomotionMetrics` types absent | R-9.3.11 |
