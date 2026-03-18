# Physics Foundation Test Cases

Companion test cases for [foundation.md](foundation.md).

## Unit Tests

### TC-4.1.1.1 Symplectic Euler Energy Conservation

| # | Requirement |
|---|-------------|
| 1 | R-4.1.1     |

1. **#1** — Spring-mass system (k=100, m=1, x0=1), 10,000 steps at dt=0.001
   - **Expected:** Total energy drift < 1% of initial energy

### TC-4.1.1.2 Verlet Position Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-4.1.1     |

1. **#1** — Constant acceleration a=9.81, dt=0.01, 100 steps
   - **Expected:** Position matches analytic x = 0.5*a*t^2 within float epsilon

### TC-4.1.1.3 Determinism 1000 Frames

| # | Requirement |
|---|-------------|
| 1 | R-4.1.1     |

1. **#1** — Identical 1000-frame simulation, run twice
   - **Expected:** Bit-equal state at frame 1000

### TC-4.1.2.1 Substep Invocation Count

| # | Requirement |
|---|-------------|
| 1 | R-4.1.2     |
| 2 | R-4.1.2     |

1. **#1** — substeps=4, run 1 tick
   - **Expected:** Each physics system runs exactly 4 times
2. **#2** — substeps=1, run 1 tick
   - **Expected:** Each physics system runs exactly 1 time

### TC-4.1.3.1 Restitution Bounce Height

| # | Requirement |
|---|-------------|
| 1 | R-4.1.3     |
| 2 | R-4.1.3     |

1. **#1** — Sphere (mass=1, restitution=1.0) dropped from h=5m onto plane
   - **Expected:** Rebound height within 1% of 5m (4.95-5.05m)
2. **#2** — Sphere with restitution=0.0
   - **Expected:** Rebound height near 0 (no bounce)

### TC-4.1.3.2 Static Friction on Slope

| # | Requirement |
|---|-------------|
| 1 | R-4.1.3     |
| 2 | R-4.1.3     |

1. **#1** — Box on 30-deg slope, friction coefficient > tan(30) = 0.577 (use 0.7)
   - **Expected:** Zero displacement over 500 ticks
2. **#2** — Same slope, friction=0.4 (below tan(30))
   - **Expected:** Box slides downhill

### TC-4.1.3.3 Material Combine Symmetry

| # | Requirement |
|---|-------------|
| 1 | R-4.1.3     |

1. **#1** — combine(material_A, material_B) for all combine modes (Average, Min, Max, Multiply)
   - **Expected:** Results identical to combine(material_B, material_A)

### TC-4.1.4.1 CCD Prevents Tunneling

| # | Requirement |
|---|-------------|
| 1 | R-4.1.4     |

1. **#1** — 0.1m sphere at 500 m/s toward 0.01m wall, CCD enabled
   - **Expected:** ContactManifold generated (no tunneling)

### TC-4.1.4.2 CCD Skips Slow Objects

| # | Requirement |
|---|-------------|
| 1 | R-4.1.4     |

1. **#1** — CcdEnabled entity moving at 0.1 m/s
   - **Expected:** CCD processing skipped (velocity below threshold)

### TC-4.1.5.1 Island Disjoint Groups

| # | Requirement |
|---|-------------|
| 1 | R-4.1.5     |

1. **#1** — Two groups of 50 bodies each, no contacts between groups
   - **Expected:** Exactly 2 distinct Island IDs

### TC-4.1.5.2 Island Merge on Contact

| # | Requirement |
|---|-------------|
| 1 | R-4.1.5     |

1. **#1** — Two separate islands, bodies from each come into contact
   - **Expected:** Merged into 1 island

### TC-4.1.5.3 Island Split on Separation

| # | Requirement |
|---|-------------|
| 1 | R-4.1.5     |

1. **#1** — Single island, contact link between subgroups removed
   - **Expected:** Splits into 2 islands with correct membership

### TC-4.1.6.1 Sleep After Threshold

| # | Requirement |
|---|-------------|
| 1 | R-4.1.6     |

1. **#1** — Body with velocity < sleep_threshold for sleep_delay seconds
   - **Expected:** Sleeping marker component added

### TC-4.1.6.2 Wake on Force

| # | Requirement |
|---|-------------|
| 1 | R-4.1.6     |

1. **#1** — Apply ExternalForce to sleeping body
   - **Expected:** Sleeping marker removed within 1 tick

### TC-4.1.6.3 Wake on Contact

| # | Requirement |
|---|-------------|
| 1 | R-4.1.6     |

1. **#1** — Drop active body onto sleeping body
   - **Expected:** Sleeping body wakes (Sleeping marker removed)

### TC-4.2.1.1 Broadphase No False Negatives

| # | Requirement |
|---|-------------|
| 1 | R-4.2.1     |

1. **#1** — 1000 random colliders, BroadphasePairs vs brute-force O(n^2)
   - **Expected:** Zero misses (all true overlaps detected by broadphase)

### TC-4.2.6.1 Layer Rejection

| # | Requirement |
|---|-------------|
| 1 | R-4.2.6     |

1. **#1** — Two overlapping entities on non-interacting layers
   - **Expected:** No ContactManifold generated

### TC-4.2.6.2 Layer Acceptance

| # | Requirement |
|---|-------------|
| 1 | R-4.2.6     |

1. **#1** — Two overlapping entities on interacting layers
   - **Expected:** ContactManifold generated

### TC-4.2.2.1 GJK Sphere-Sphere Distance

| # | Requirement |
|---|-------------|
| 1 | R-4.2.2     |

1. **#1** — Sphere A at (0,0,0) r=1, Sphere B at (4,0,0) r=1
   - **Expected:** GJK distance = 2.0, matches analytic within 1 mm

### TC-4.2.2.2 EPA Penetration Depth

| # | Requirement |
|---|-------------|
| 1 | R-4.2.2     |

1. **#1** — Box A at (0,0,0) size 2x2x2, Box B at (1,0,0) size 2x2x2
   - **Expected:** Penetration depth = 1.0, matches analytic within 1 mm

### TC-4.2.7.1 Collision Event Lifecycle

| # | Requirement |
|---|-------------|
| 1 | R-4.2.7     |

1. **#1** — Two bodies in contact for 5 frames, then separate
   - **Expected:** 1 Started, 4 Persisted, 1 Ended events (6 total)

### TC-4.2.7.2 Same Frame Event Delivery

| # | Requirement |
|---|-------------|
| 1 | R-4.2.7     |

1. **#1** — Trigger collision at frame N
   - **Expected:** Event readable at frame N (same frame)

### TC-4.2.8.1 Trigger Oneshot

| # | Requirement |
|---|-------------|
| 1 | R-4.2.8     |

1. **#1** — Entity enters oneshot trigger volume
   - **Expected:** TriggerEnter fires once, trigger disables after

### TC-4.2.8.2 Trigger No Contact Force

| # | Requirement |
|---|-------------|
| 1 | R-4.2.8     |

1. **#1** — Entity passes through trigger volume at velocity (5,0,0)
   - **Expected:** Velocity unchanged after passing through (no contact force)

### TC-4.1.8.1 Slope Rejection

| # | Requirement |
|---|-------------|
| 1 | R-4.1.8     |
| 2 | R-4.1.8     |

1. **#1** — Character controller on 50-deg slope, max_slope=45 deg
   - **Expected:** Character slides (slope rejected)
2. **#2** — Same controller on 40-deg slope
   - **Expected:** Character stands (slope accepted)

### TC-4.1.8.2 Step Climbing

| # | Requirement |
|---|-------------|
| 1 | R-4.1.8     |
| 2 | R-4.1.8     |

1. **#1** — Step height=0.3m, step_limit=0.35m
   - **Expected:** Character climbs step
2. **#2** — Step height=0.4m, step_limit=0.35m
   - **Expected:** Character blocked

### TC-4.1.8.3 Ground Detection via BVH

| # | Requirement |
|---|-------------|
| 1 | R-4.1.8     |

1. **#1** — Character controller ground detection shape cast
   - **Expected:** Uses shared BVH (not separate structure)

### TC-4.1.9.1 Platform Passenger Drift

| # | Requirement |
|---|-------------|
| 1 | R-4.1.9     |

1. **#1** — Character on 5 m/s moving platform for 10 s
   - **Expected:** Drift < 1 cm/s relative to platform surface

### TC-4.1.10.1 Ground Smoothing EMA

| # | Requirement |
|---|-------------|
| 1 | R-4.1.10    |

1. **#1** — Walk over surface with 5 cm triangle seams
   - **Expected:** Vertical oscillation < 1 mm peak-to-peak

## Integration Tests

### TC-4.1.NF3.I1 Cross-Platform Determinism

| # | Requirement |
|---|-------------|
| 1 | R-4.1.NF3   |

1. **#1** — 1000-step simulation on Windows, macOS, Linux
   - **Expected:** Serialized state is bit-equal across all 3 platforms

### TC-4.1.5.I1 Parallel Island Correctness

| # | Requirement |
|---|-------------|
| 1 | R-4.1.5     |

1. **#1** — 100-body scene, parallel island solve vs serial solve
   - **Expected:** Identical results between parallel and serial

### TC-4.1.7.I1 Zone Migration Momentum

| # | Requirement |
|---|-------------|
| 1 | R-4.1.7     |

1. **#1** — Body at constant velocity (10,0,0) crosses zone boundary
   - **Expected:** Velocity preserved within 0.1% after migration

### TC-4.1.7.I2 Zone Migration Contacts

| # | Requirement |
|---|-------------|
| 1 | R-4.1.7     |

1. **#1** — Stack of 5 bodies crosses zone boundary together
   - **Expected:** Contact relationships preserved, stack does not collapse

### TC-4.1.NF1.I1 Full Pipeline 2000 Bodies

| # | Requirement |
|---|-------------|
| 1 | R-4.1.NF1   |

1. **#1** — 2000 active bodies, 4 substeps, measure wall time
   - **Expected:** Total physics tick < 4 ms on min-spec hardware

### TC-4.1.NF4.I1 200 Character Controllers

| # | Requirement |
|---|-------------|
| 1 | R-4.1.NF4   |

1. **#1** — 200 character controllers on varied terrain
   - **Expected:** Total system time < 20 ms (0.1 ms per controller)

### TC-4.1.4.I1 CCD Budget Desktop 256

| # | Requirement |
|---|-------------|
| 1 | R-4.1.4     |

1. **#1** — 256 CCD-enabled entities on desktop
   - **Expected:** CCD system completes within 0.5 ms

### TC-4.1.6.I1 Sleeping Reduces Cost 80 Percent

| # | Requirement |
|---|-------------|
| 1 | R-4.1.6     |

1. **#1** — 10,000 sleeping bodies vs 10,000 active bodies
   - **Expected:** >= 80% tick cost reduction for sleeping scenario

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
