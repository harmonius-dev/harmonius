# Physics Constraints and Joints Test Cases

Companion test cases for [constraints.md](constraints.md).

## Unit Tests

### TC-4.3.1.1 Revolute Joint 5-DOF Lock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create revolute joint between body A and B, apply force on all axes | 5 DOF constrained, free axis rotates | R-4.3.1 |
| 2 | Measure translation on constrained axes | Zero displacement (within solver tolerance) | R-4.3.1 |

### TC-4.3.1.2 Prismatic Joint 5-DOF Lock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create prismatic joint, apply force along slide axis and perpendicular | 5 DOF constrained, free axis slides | R-4.3.1 |
| 2 | Measure rotation on all axes | Zero rotation (within solver tolerance) | R-4.3.1 |

### TC-4.3.1.3 Fixed Joint 6-DOF Lock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create fixed joint, apply force and torque on all axes | All 6 DOF locked, zero relative motion | R-4.3.1 |

### TC-4.3.1.4 Distance Joint Separation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Distance joint with target=5m, apply 100N pulling force for 100 ticks | Separation stays within 1 mm of 5m target | R-4.3.1 |

### TC-4.3.2.1 Spring Joint Equilibrium

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spring joint (stiffness=100, damping=10, rest_length=2m), displace to 3m | Oscillation converges to rest length (2m +/- 1mm) | R-4.3.2 |

### TC-4.3.2.2 Cone-Twist Limits

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cone-twist with 45-deg limit, apply 50 Nm torque for 1000 ticks | Angle does not exceed 45.5 degrees | R-4.3.2 |

### TC-4.3.2.3 6-DOF Per-Axis Lock

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Lock X axis, limit Y to +/-30 deg, free Z; apply torque on all axes | X frozen (0 deg), Y bounded at 30 deg, Z unconstrained | R-4.3.2 |
| 2 | Axes are independent | Constraining X does not affect Z behavior | R-4.3.2 |

### TC-4.3.3.1 Motor Velocity Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Velocity motor target=2 rad/s, max_force=1000N, run 500 ticks | Steady-state angular velocity within 1% of 2 rad/s | R-4.3.3 |

### TC-4.3.3.2 Motor Position Target

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Position motor target=90 deg, from initial 0 deg | Converges to 90 deg within tolerance | R-4.3.3 |

### TC-4.3.3.3 Motor Max Force Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Motor max_force=100N, apply 500N external load | Motor force never exceeds 100N (clamped) | R-4.3.3 |

### TC-4.3.3.4 Angular Limits Clamp

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Revolute with +/-45 deg limits, apply 200 Nm torque | Angle does not exceed 45.5 deg | R-4.3.3 |

### TC-4.3.3.5 Limits Restitution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Revolute with restitution=0.5 at limit, hit limit at 1 rad/s | Bounce velocity approximately 0.5 rad/s | R-4.3.3 |

### TC-4.3.4.1 Break Force Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Breakable joint threshold=1000N, apply 1500N force | Joint despawned within one substep | R-4.3.4 |
| 2 | Apply 800N force (below threshold) | Joint intact | R-4.3.4 |

### TC-4.3.4.2 Break Event Payload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Break a joint, observe JointBroken event | Event contains body_a entity, body_b entity, and force magnitude | R-4.3.4 |

### TC-4.3.4.3 Break Torque Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Joint with break_torque=500 Nm, apply 700 Nm torque | Joint breaks (independent of force threshold) | R-4.3.4 |

### TC-4.3.4.4 Break Varied Directions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply tension exceeding threshold | Joint breaks | R-4.3.4 |
| 2 | Apply compression exceeding threshold | Joint breaks | R-4.3.4 |
| 3 | Apply shear exceeding threshold | Joint breaks | R-4.3.4 |
| 4 | Apply torsion exceeding threshold | Joint breaks | R-4.3.4 |

### TC-4.3.7.1 Warm Start Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run solver with warm start vs cold start on 50-joint chain | Warm-started converges in fewer iterations (at least 30% fewer) | R-4.3.7 |

### TC-4.3.7.2 SI Solver Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Known 2-body revolute configuration, reference impulses computed analytically | SI solver impulses match reference within tolerance | R-4.3.7 |

### TC-4.3.7.3 TGS Solver Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same configuration as SI test | TGS solver impulses and positions match TGS reference | R-4.3.7 |

### TC-4.3.7.4 Solver Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run identical 100-body scenario twice | Bit-identical results between both runs | R-4.3.7 |

### TC-4.3.7.5 Island Partitioning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3 disconnected body groups (10, 20, 15 bodies) | Exactly 3 islands with correct membership counts | R-4.3.7 |

### TC-4.3.7.6 Island Incremental Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add joint connecting two islands | Islands merge into one (correct count) | R-4.3.7 |
| 2 | Remove joint splitting island | Island splits into two (correct membership) | R-4.3.7 |

### TC-4.3.5.1 Ragdoll Activation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate ragdoll on 20-bone skeleton | All joint entities spawned with correct types (revolute, cone-twist) and limits | R-4.3.5 |
| 2 | Joint count | Equals bone_count - 1 (each bone-to-parent connection) | R-4.3.5 |

### TC-4.3.5.2 Ragdoll Deactivation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Deactivate active ragdoll | All joint entities and ragdoll body entities despawned | R-4.3.5 |

### TC-4.3.6.1 Chain Spawn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 32-segment chain | 32 body entities + 31 joint entities, correct sequential connectivity | R-4.3.6 |

### TC-4.3.8.1 Limb Health Zero Severs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reduce LimbHealth to 0 on arm joint | Joint despawned, JointSevered event fires with correct limb entity | R-4.3.8 |

### TC-4.3.1.5 Constraint Row Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Revolute joint | 5 constraint rows | R-4.3.1 |
| 2 | Prismatic joint | 5 constraint rows | R-4.3.1 |
| 3 | Fixed joint | 6 constraint rows | R-4.3.1 |
| 4 | Distance joint | 1 constraint row | R-4.3.1 |

## Integration Tests

### TC-4.3.1.I1 Core Joint Drift

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two 1 kg bodies with revolute joint, apply forces 500 ticks at 8 iterations | Drift below 1 mm | R-4.3.1 |
| 2 | Same test with prismatic, fixed, distance joints | Drift below 1 mm for each type | R-4.3.1 |

### TC-4.3.7.I1 TGS Drift Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10-body chain, 1000 ticks, SI solver at 8 iterations | Measure end-to-end drift | R-4.3.7 |
| 2 | Same chain, TGS solver at 8 iterations | TGS drift at least 30% lower than SI drift | R-4.3.7 |

### TC-4.3.5.I1 Ragdoll Stability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate ragdoll, simulate 100 ticks with gravity | No constraint violation exceeds 5 mm | R-4.3.5 |

### TC-4.3.5.I2 Ragdoll on Slopes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ragdoll on 30-degree slope for 5 s | No jitter or tunneling through ground | R-4.3.5 |

### TC-4.3.6.I1 Chain Stability 60 Seconds

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 32-segment chain, 60 s simulation, 4 substeps | Separation below 1 mm, energy gain below 1%/s | R-4.3.6 |

### TC-4.3.6.I2 Chain Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Chain draped over cylindrical obstacle | Collision response keeps chain on surface, no penetration | R-4.3.6 |

### TC-4.3.6.I3 Chain Extreme Tension

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 10,000N tension to 32-segment chain | Chain does not explode or produce NaN positions | R-4.3.6 |

### TC-4.3.7.I2 Parallel Island Solve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 16 independent islands, solve in parallel | All islands solved correctly, results match serial solve | R-4.3.7 |

### TC-4.3.5.I3 Ragdoll to Animation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition ragdoll back to animation blend | Smooth blend with no visible snapping or popping | R-4.3.5 |

### TC-4.3.9.I1 Prosthetic Attachment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever limb, attach prosthetic entity | Constraints restored, prosthetic functions as joint | R-4.3.9 |

### TC-4.3.8.I1 Severance Locomotion

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sever leg joint on animated character | Skeleton adapts locomotion (limp or fall) | R-4.3.8 |

## Benchmarks

### TC-4.3.NF1.B1 Solver Throughput

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 joints, 8 iterations | Total solve time | < 4 ms | R-4.3.NF1 |

### TC-4.3.NF1.B2 Constraint Rows Per Millisecond

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Mixed joint types, steady state | Rows processed per ms | >= 5000 | R-4.3.NF1 |

### TC-4.3.NF2.B1 Ragdoll Activation 8x20

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 8 ragdolls (20 bones each) activated simultaneously | Total time | < 4 ms | R-4.3.NF2 |

### TC-4.3.NF2.B2 Single Ragdoll Activation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single 20-bone ragdoll activation | Total time | < 0.5 ms | R-4.3.NF2 |

### TC-4.3.7.B1 Island Build

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000 bodies, build island graph | Total time | < 0.5 ms | R-4.3.7 |

### TC-4.3.7.B2 Warm Start Apply

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 joints, apply warm start impulses | Total time | < 0.2 ms | R-4.3.7 |

### TC-4.3.4.B1 Break Detection

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 500 breakable joints, evaluate thresholds | Total time | < 0.1 ms | R-4.3.4 |

### TC-4.3.6.B1 Chain Spawn

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Spawn 32-segment chain (32 bodies + 31 joints) | Total time | < 0.3 ms | R-4.3.6 |
