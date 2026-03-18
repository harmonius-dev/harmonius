# Physics Constraints and Joints Test Cases

Companion test cases for [constraints.md](constraints.md).

## Unit Tests

### TC-4.3.1.1 Revolute Joint 5-DOF Lock

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |
| 2 | R-4.3.1     |

1. **#1** — Create revolute joint between body A and B, apply force on all axes
   - **Expected:** 5 DOF constrained, free axis rotates
2. **#2** — Measure translation on constrained axes
   - **Expected:** Zero displacement (within solver tolerance)

### TC-4.3.1.2 Prismatic Joint 5-DOF Lock

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |
| 2 | R-4.3.1     |

1. **#1** — Create prismatic joint, apply force along slide axis and perpendicular
   - **Expected:** 5 DOF constrained, free axis slides
2. **#2** — Measure rotation on all axes
   - **Expected:** Zero rotation (within solver tolerance)

### TC-4.3.1.3 Fixed Joint 6-DOF Lock

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |

1. **#1** — Create fixed joint, apply force and torque on all axes
   - **Expected:** All 6 DOF locked, zero relative motion

### TC-4.3.1.4 Distance Joint Separation

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |

1. **#1** — Distance joint with target=5m, apply 100N pulling force for 100 ticks
   - **Expected:** Separation stays within 1 mm of 5m target

### TC-4.3.2.1 Spring Joint Equilibrium

| # | Requirement |
|---|-------------|
| 1 | R-4.3.2     |

1. **#1** — Spring joint (stiffness=100, damping=10, rest_length=2m), displace to 3m
   - **Expected:** Oscillation converges to rest length (2m +/- 1mm)

### TC-4.3.2.2 Cone-Twist Limits

| # | Requirement |
|---|-------------|
| 1 | R-4.3.2     |

1. **#1** — Cone-twist with 45-deg limit, apply 50 Nm torque for 1000 ticks
   - **Expected:** Angle does not exceed 45.5 degrees

### TC-4.3.2.3 6-DOF Per-Axis Lock

| # | Requirement |
|---|-------------|
| 1 | R-4.3.2     |
| 2 | R-4.3.2     |

1. **#1** — Lock X axis, limit Y to +/-30 deg, free Z; apply torque on all axes
   - **Expected:** X frozen (0 deg), Y bounded at 30 deg, Z unconstrained
2. **#2** — Axes are independent
   - **Expected:** Constraining X does not affect Z behavior

### TC-4.3.3.1 Motor Velocity Target

| # | Requirement |
|---|-------------|
| 1 | R-4.3.3     |

1. **#1** — Velocity motor target=2 rad/s, max_force=1000N, run 500 ticks
   - **Expected:** Steady-state angular velocity within 1% of 2 rad/s

### TC-4.3.3.2 Motor Position Target

| # | Requirement |
|---|-------------|
| 1 | R-4.3.3     |

1. **#1** — Position motor target=90 deg, from initial 0 deg
   - **Expected:** Converges to 90 deg within tolerance

### TC-4.3.3.3 Motor Max Force Clamp

| # | Requirement |
|---|-------------|
| 1 | R-4.3.3     |

1. **#1** — Motor max_force=100N, apply 500N external load
   - **Expected:** Motor force never exceeds 100N (clamped)

### TC-4.3.3.4 Angular Limits Clamp

| # | Requirement |
|---|-------------|
| 1 | R-4.3.3     |

1. **#1** — Revolute with +/-45 deg limits, apply 200 Nm torque
   - **Expected:** Angle does not exceed 45.5 deg

### TC-4.3.3.5 Limits Restitution

| # | Requirement |
|---|-------------|
| 1 | R-4.3.3     |

1. **#1** — Revolute with restitution=0.5 at limit, hit limit at 1 rad/s
   - **Expected:** Bounce velocity approximately 0.5 rad/s

### TC-4.3.4.1 Break Force Threshold

| # | Requirement |
|---|-------------|
| 1 | R-4.3.4     |
| 2 | R-4.3.4     |

1. **#1** — Breakable joint threshold=1000N, apply 1500N force
   - **Expected:** Joint despawned within one substep
2. **#2** — Apply 800N force (below threshold)
   - **Expected:** Joint intact

### TC-4.3.4.2 Break Event Payload

| # | Requirement |
|---|-------------|
| 1 | R-4.3.4     |

1. **#1** — Break a joint, observe JointBroken event
   - **Expected:** Event contains body_a entity, body_b entity, and force magnitude

### TC-4.3.4.3 Break Torque Threshold

| # | Requirement |
|---|-------------|
| 1 | R-4.3.4     |

1. **#1** — Joint with break_torque=500 Nm, apply 700 Nm torque
   - **Expected:** Joint breaks (independent of force threshold)

### TC-4.3.4.4 Break Varied Directions

| # | Requirement |
|---|-------------|
| 1 | R-4.3.4     |
| 2 | R-4.3.4     |
| 3 | R-4.3.4     |
| 4 | R-4.3.4     |

1. **#1** — Apply tension exceeding threshold
   - **Expected:** Joint breaks
2. **#2** — Apply compression exceeding threshold
   - **Expected:** Joint breaks
3. **#3** — Apply shear exceeding threshold
   - **Expected:** Joint breaks
4. **#4** — Apply torsion exceeding threshold
   - **Expected:** Joint breaks

### TC-4.3.7.1 Warm Start Convergence

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — Run solver with warm start vs cold start on 50-joint chain
   - **Expected:** Warm-started converges in fewer iterations (at least 30% fewer)

### TC-4.3.7.2 SI Solver Correctness

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — Known 2-body revolute configuration, reference impulses computed analytically
   - **Expected:** SI solver impulses match reference within tolerance

### TC-4.3.7.3 TGS Solver Correctness

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — Same configuration as SI test
   - **Expected:** TGS solver impulses and positions match TGS reference

### TC-4.3.7.4 Solver Determinism

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — Run identical 100-body scenario twice
   - **Expected:** Bit-identical results between both runs

### TC-4.3.7.5 Island Partitioning

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — 3 disconnected body groups (10, 20, 15 bodies)
   - **Expected:** Exactly 3 islands with correct membership counts

### TC-4.3.7.6 Island Incremental Update

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |
| 2 | R-4.3.7     |

1. **#1** — Add joint connecting two islands
   - **Expected:** Islands merge into one (correct count)
2. **#2** — Remove joint splitting island
   - **Expected:** Island splits into two (correct membership)

### TC-4.3.5.1 Ragdoll Activation

| # | Requirement |
|---|-------------|
| 1 | R-4.3.5     |
| 2 | R-4.3.5     |

1. **#1** — Activate ragdoll on 20-bone skeleton
   - **Expected:** All joint entities spawned with correct types (revolute, cone-twist) and limits
2. **#2** — Joint count
   - **Expected:** Equals bone_count - 1 (each bone-to-parent connection)

### TC-4.3.5.2 Ragdoll Deactivation

| # | Requirement |
|---|-------------|
| 1 | R-4.3.5     |

1. **#1** — Deactivate active ragdoll
   - **Expected:** All joint entities and ragdoll body entities despawned

### TC-4.3.6.1 Chain Spawn

| # | Requirement |
|---|-------------|
| 1 | R-4.3.6     |

1. **#1** — Spawn 32-segment chain
   - **Expected:** 32 body entities + 31 joint entities, correct sequential connectivity

### TC-4.3.8.1 Limb Health Zero Severs

| # | Requirement |
|---|-------------|
| 1 | R-4.3.8     |

1. **#1** — Reduce LimbHealth to 0 on arm joint
   - **Expected:** Joint despawned, JointSevered event fires with correct limb entity

### TC-4.3.1.5 Constraint Row Count

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |
| 2 | R-4.3.1     |
| 3 | R-4.3.1     |
| 4 | R-4.3.1     |

1. **#1** — Revolute joint
   - **Expected:** 5 constraint rows
2. **#2** — Prismatic joint
   - **Expected:** 5 constraint rows
3. **#3** — Fixed joint
   - **Expected:** 6 constraint rows
4. **#4** — Distance joint
   - **Expected:** 1 constraint row

## Integration Tests

### TC-4.3.1.I1 Core Joint Drift

| # | Requirement |
|---|-------------|
| 1 | R-4.3.1     |
| 2 | R-4.3.1     |

1. **#1** — Two 1 kg bodies with revolute joint, apply forces 500 ticks at 8 iterations
   - **Expected:** Drift below 1 mm
2. **#2** — Same test with prismatic, fixed, distance joints
   - **Expected:** Drift below 1 mm for each type

### TC-4.3.7.I1 TGS Drift Reduction

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |
| 2 | R-4.3.7     |

1. **#1** — 10-body chain, 1000 ticks, SI solver at 8 iterations
   - **Expected:** Measure end-to-end drift
2. **#2** — Same chain, TGS solver at 8 iterations
   - **Expected:** TGS drift at least 30% lower than SI drift

### TC-4.3.5.I1 Ragdoll Stability

| # | Requirement |
|---|-------------|
| 1 | R-4.3.5     |

1. **#1** — Activate ragdoll, simulate 100 ticks with gravity
   - **Expected:** No constraint violation exceeds 5 mm

### TC-4.3.5.I2 Ragdoll on Slopes

| # | Requirement |
|---|-------------|
| 1 | R-4.3.5     |

1. **#1** — Ragdoll on 30-degree slope for 5 s
   - **Expected:** No jitter or tunneling through ground

### TC-4.3.6.I1 Chain Stability 60 Seconds

| # | Requirement |
|---|-------------|
| 1 | R-4.3.6     |

1. **#1** — 32-segment chain, 60 s simulation, 4 substeps
   - **Expected:** Separation below 1 mm, energy gain below 1%/s

### TC-4.3.6.I2 Chain Collision

| # | Requirement |
|---|-------------|
| 1 | R-4.3.6     |

1. **#1** — Chain draped over cylindrical obstacle
   - **Expected:** Collision response keeps chain on surface, no penetration

### TC-4.3.6.I3 Chain Extreme Tension

| # | Requirement |
|---|-------------|
| 1 | R-4.3.6     |

1. **#1** — Apply 10,000N tension to 32-segment chain
   - **Expected:** Chain does not explode or produce NaN positions

### TC-4.3.7.I2 Parallel Island Solve

| # | Requirement |
|---|-------------|
| 1 | R-4.3.7     |

1. **#1** — 16 independent islands, solve in parallel
   - **Expected:** All islands solved correctly, results match serial solve

### TC-4.3.5.I3 Ragdoll to Animation

| # | Requirement |
|---|-------------|
| 1 | R-4.3.5     |

1. **#1** — Transition ragdoll back to animation blend
   - **Expected:** Smooth blend with no visible snapping or popping

### TC-4.3.9.I1 Prosthetic Attachment

| # | Requirement |
|---|-------------|
| 1 | R-4.3.9     |

1. **#1** — Sever limb, attach prosthetic entity
   - **Expected:** Constraints restored, prosthetic functions as joint

### TC-4.3.8.I1 Severance Locomotion

| # | Requirement |
|---|-------------|
| 1 | R-4.3.8     |

1. **#1** — Sever leg joint on animated character
   - **Expected:** Skeleton adapts locomotion (limp or fall)

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
