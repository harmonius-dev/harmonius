# Physics Foundation Test Cases

Companion test cases for [foundation.md](foundation.md).

## Unit Tests

### TC-4.1.1.1 Symplectic Euler Energy Conservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spring-mass system (k=100, m=1, x0=1), 10,000 steps at dt=0.001 | Total energy drift < 1% of initial energy | R-4.1.1 |

### TC-4.1.1.2 Verlet Position Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Constant acceleration a=9.81, dt=0.01, 100 steps | Position matches analytic x = 0.5*a*t^2 within float epsilon | R-4.1.1 |

### TC-4.1.1.3 Determinism 1000 Frames

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Identical 1000-frame simulation, run twice | Bit-equal state at frame 1000 | R-4.1.1 |

### TC-4.1.2.1 Substep Invocation Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | substeps=4, run 1 tick | Each physics system runs exactly 4 times | R-4.1.2 |
| 2 | substeps=1, run 1 tick | Each physics system runs exactly 1 time | R-4.1.2 |

### TC-4.1.3.1 Restitution Bounce Height

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sphere (mass=1, restitution=1.0) dropped from h=5m onto plane | Rebound height within 1% of 5m (4.95-5.05m) | R-4.1.3 |
| 2 | Sphere with restitution=0.0 | Rebound height near 0 (no bounce) | R-4.1.3 |

### TC-4.1.3.2 Static Friction on Slope

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Box on 30-deg slope, friction coefficient > tan(30) = 0.577 (use 0.7) | Zero displacement over 500 ticks | R-4.1.3 |
| 2 | Same slope, friction=0.4 (below tan(30)) | Box slides downhill | R-4.1.3 |

### TC-4.1.3.3 Material Combine Symmetry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | combine(material_A, material_B) for all combine modes (Average, Min, Max, Multiply) | Results identical to combine(material_B, material_A) | R-4.1.3 |

### TC-4.1.4.1 CCD Prevents Tunneling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 0.1m sphere at 500 m/s toward 0.01m wall, CCD enabled | ContactManifold generated (no tunneling) | R-4.1.4 |

### TC-4.1.4.2 CCD Skips Slow Objects

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | CcdEnabled entity moving at 0.1 m/s | CCD processing skipped (velocity below threshold) | R-4.1.4 |

### TC-4.1.5.1 Island Disjoint Groups

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two groups of 50 bodies each, no contacts between groups | Exactly 2 distinct Island IDs | R-4.1.5 |

### TC-4.1.5.2 Island Merge on Contact

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two separate islands, bodies from each come into contact | Merged into 1 island | R-4.1.5 |

### TC-4.1.5.3 Island Split on Separation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single island, contact link between subgroups removed | Splits into 2 islands with correct membership | R-4.1.5 |

### TC-4.1.6.1 Sleep After Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Body with velocity < sleep_threshold for sleep_delay seconds | Sleeping marker component added | R-4.1.6 |

### TC-4.1.6.2 Wake on Force

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply ExternalForce to sleeping body | Sleeping marker removed within 1 tick | R-4.1.6 |

### TC-4.1.6.3 Wake on Contact

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drop active body onto sleeping body | Sleeping body wakes (Sleeping marker removed) | R-4.1.6 |

### TC-4.2.1.1 Broadphase No False Negatives

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 random colliders, BroadphasePairs vs brute-force O(n^2) | Zero misses (all true overlaps detected by broadphase) | R-4.2.1 |

### TC-4.2.6.1 Layer Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two overlapping entities on non-interacting layers | No ContactManifold generated | R-4.2.6 |

### TC-4.2.6.2 Layer Acceptance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two overlapping entities on interacting layers | ContactManifold generated | R-4.2.6 |

### TC-4.2.2.1 GJK Sphere-Sphere Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sphere A at (0,0,0) r=1, Sphere B at (4,0,0) r=1 | GJK distance = 2.0, matches analytic within 1 mm | R-4.2.2 |

### TC-4.2.2.2 EPA Penetration Depth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Box A at (0,0,0) size 2x2x2, Box B at (1,0,0) size 2x2x2 | Penetration depth = 1.0, matches analytic within 1 mm | R-4.2.2 |

### TC-4.2.7.1 Collision Event Lifecycle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two bodies in contact for 5 frames, then separate | 1 Started, 4 Persisted, 1 Ended events (6 total) | R-4.2.7 |

### TC-4.2.7.2 Same Frame Event Delivery

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger collision at frame N | Event readable at frame N (same frame) | R-4.2.7 |

### TC-4.2.8.1 Trigger Oneshot

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity enters oneshot trigger volume | TriggerEnter fires once, trigger disables after | R-4.2.8 |

### TC-4.2.8.2 Trigger No Contact Force

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Entity passes through trigger volume at velocity (5,0,0) | Velocity unchanged after passing through (no contact force) | R-4.2.8 |

### TC-4.1.8.1 Slope Rejection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Character controller on 50-deg slope, max_slope=45 deg | Character slides (slope rejected) | R-4.1.8 |
| 2 | Same controller on 40-deg slope | Character stands (slope accepted) | R-4.1.8 |

### TC-4.1.8.2 Step Climbing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Step height=0.3m, step_limit=0.35m | Character climbs step | R-4.1.8 |
| 2 | Step height=0.4m, step_limit=0.35m | Character blocked | R-4.1.8 |

### TC-4.1.8.3 Ground Detection via BVH

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Character controller ground detection shape cast | Uses shared BVH (not separate structure) | R-4.1.8 |

### TC-4.1.9.1 Platform Passenger Drift

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Character on 5 m/s moving platform for 10 s | Drift < 1 cm/s relative to platform surface | R-4.1.9 |

### TC-4.1.10.1 Ground Smoothing EMA

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Walk over surface with 5 cm triangle seams | Vertical oscillation < 1 mm peak-to-peak | R-4.1.10 |

## Integration Tests

### TC-4.1.NF3.I1 Cross-Platform Determinism

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-step simulation on Windows, macOS, Linux | Serialized state is bit-equal across all 3 platforms | R-4.1.NF3 |

### TC-4.1.5.I1 Parallel Island Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100-body scene, parallel island solve vs serial solve | Identical results between parallel and serial | R-4.1.5 |

### TC-4.1.7.I1 Zone Migration Momentum

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Body at constant velocity (10,0,0) crosses zone boundary | Velocity preserved within 0.1% after migration | R-4.1.7 |

### TC-4.1.7.I2 Zone Migration Contacts

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Stack of 5 bodies crosses zone boundary together | Contact relationships preserved, stack does not collapse | R-4.1.7 |

### TC-4.1.NF1.I1 Full Pipeline 2000 Bodies

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2000 active bodies, 4 substeps, measure wall time | Total physics tick < 4 ms on min-spec hardware | R-4.1.NF1 |

### TC-4.1.NF4.I1 200 Character Controllers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200 character controllers on varied terrain | Total system time < 20 ms (0.1 ms per controller) | R-4.1.NF4 |

### TC-4.1.4.I1 CCD Budget Desktop 256

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 256 CCD-enabled entities on desktop | CCD system completes within 0.5 ms | R-4.1.4 |

### TC-4.1.6.I1 Sleeping Reduces Cost 80 Percent

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 sleeping bodies vs 10,000 active bodies | >= 80% tick cost reduction for sleeping scenario | R-4.1.6 |

## Benchmarks

### TC-4.1.1.B1 Integration 2000 Bodies

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2000 bodies, symplectic Euler integration | Total time | < 1 ms | R-4.1.1 |

### TC-4.2.NF1.B1 Broadphase 50K AABBs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50,000 AABBs, broadphase pair detection | Total time | < 1 ms | R-4.2.NF1 |

### TC-4.2.NF2.B1 Narrowphase 10K Pairs

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 primitive collision pairs | Total time | < 2 ms | R-4.2.NF2 |

### TC-4.1.NF1.B1 Constraint Solver

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2000 bodies, 4 substeps | Total solve time | < 2 ms | R-4.1.NF1 |

### TC-4.1.4.B1 CCD 256 Entities

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 CCD entities on desktop | Total time | < 0.5 ms | R-4.1.4 |

### TC-4.1.5.B1 Island Computation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1024 islands, union-find computation | Total time | < 0.5 ms | R-4.1.5 |

### TC-4.1.6.B1 Sleep Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10,000 entities, evaluate sleep criteria | Total time | < 0.1 ms | R-4.1.6 |

### TC-4.1.NF4.B1 Character Controller

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single character controller tick | Per-entity time | < 0.1 ms | R-4.1.NF4 |

### TC-4.1.NF1.B2 Full Physics Tick

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2000 bodies, 4 substeps, full pipeline | Total wall time | < 4 ms | R-4.1.NF1 |

### TC-4.1.NF2.B1 Memory Per Rigid Body

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Measure memory per rigid body entity | Per-entity memory | <= 256 bytes | R-4.1.NF2 |
