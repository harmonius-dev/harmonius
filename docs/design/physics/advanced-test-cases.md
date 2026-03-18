# Advanced Physics Test Cases

Companion test cases for [advanced.md](advanced.md).

## Unit Tests

### TC-4.4.1.1 Ray Cast Sphere Hit

| # | Requirement |
|---|-------------|
| 1 | R-4.4.1     |
| 2 | R-4.4.1     |

1. **#1** — Ray origin=(0,0,0), dir=(1,0,0); sphere center=(5,0,0), radius=1
   - **Expected:** Hit at pos=(4,0,0), error < 0.1 mm
2. **#2** — Hit normal
   - **Expected:** (−1,0,0), within 0.001 rad of analytical

### TC-4.4.1.2 Ray Cast Normal Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-4.4.1     |

1. **#1** — Ray tangent to sphere, analytical normal known
   - **Expected:** Reported normal within 0.001 rad of analytical

### TC-4.4.1.3 Ray Cast All Shapes

| # | Requirement |
|---|-------------|
| 1 | R-4.4.1     |
| 2 | R-4.4.1     |
| 3 | R-4.4.1     |
| 4 | R-4.4.1     |
| 5 | R-4.4.1     |
| 6 | R-4.4.1     |

1. **#1** — Ray vs box (axis-aligned, 2x2x2 at origin)
   - **Expected:** Hit at correct face, distance matches analytical
2. **#2** — Ray vs sphere (radius=1 at (5,0,0))
   - **Expected:** Hit position matches analytical within 0.1 mm
3. **#3** — Ray vs capsule (radius=0.5, half_height=1)
   - **Expected:** Hit detected, contact valid
4. **#4** — Ray vs convex hull (8-vertex cube)
   - **Expected:** Hit detected, contact valid
5. **#5** — Ray vs triangle mesh (flat quad)
   - **Expected:** Hit detected at surface
6. **#6** — Ray vs heightfield (flat at y=0)
   - **Expected:** Hit at y=0

### TC-4.4.1.4 Ray Cast Miss

| # | Requirement |
|---|-------------|
| 1 | R-4.4.1     |

1. **#1** — Ray origin=(0,0,0), dir=(0,1,0); no entities in path
   - **Expected:** `None` returned

### TC-4.4.NF1.1 Ray Cast 50K Latency

| # | Requirement |
|---|-------------|
| 1 | R-4.4.NF1   |

1. **#1** — Single ray vs 50,000 entities in shared BVH
   - **Expected:** Completes within 5 us

### TC-4.4.2.1 Shape Cast Ground Detection

| # | Requirement |
|---|-------------|
| 1 | R-4.4.2     |

1. **#1** — Capsule (radius=0.5, half_height=1) swept downward from y=5 toward ground plane at y=0
   - **Expected:** Ground detected, contact point at y=0.5 (capsule bottom)

### TC-4.4.2.2 Shape Cast No Hit

| # | Requirement |
|---|-------------|
| 1 | R-4.4.2     |

1. **#1** — Capsule swept in direction with no obstacles, max_dist=100
   - **Expected:** `None` returned

### TC-4.4.2.3 Shape Cast Contact Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-4.4.2     |

1. **#1** — Sphere (radius=1) swept toward box, known analytical contact
   - **Expected:** Contact point within 0.1 mm of analytical solution

### TC-4.4.3.1 Overlap Sphere AOE

| # | Requirement |
|---|-------------|
| 1 | R-4.4.3     |

1. **#1** — 100 entities, sphere overlap radius=10 at origin, 30 within range
   - **Expected:** Exactly 30 entities returned, zero false positives, zero false negatives

### TC-4.4.3.2 Overlap All Shapes

| # | Requirement |
|---|-------------|
| 1 | R-4.4.3     |
| 2 | R-4.4.3     |
| 3 | R-4.4.3     |

1. **#1** — Box overlap (2x2x2) at known position with known entities
   - **Expected:** Correct entity set returned
2. **#2** — Capsule overlap at known position
   - **Expected:** Correct entity set returned
3. **#3** — Convex hull overlap at known position
   - **Expected:** Correct entity set returned

### TC-4.4.4.1 Closest Point Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-4.4.4     |
| 2 | R-4.4.4     |
| 3 | R-4.4.4     |

1. **#1** — Query point (3,0,0), sphere center=(5,0,0) radius=1
   - **Expected:** Closest point = (4,0,0), distance = 1.0, within 0.1 mm
2. **#2** — Query point (1,1,0), box min=(2,0,0) max=(4,2,2)
   - **Expected:** Closest point = (2,1,0), within 0.1 mm
3. **#3** — Query point (0,5,0), capsule at origin
   - **Expected:** Closest point within 0.1 mm of analytical

### TC-4.4.4.2 Closest Point Signed Distance

| # | Requirement |
|---|-------------|
| 1 | R-4.4.4     |
| 2 | R-4.4.4     |

1. **#1** — Query point inside sphere (center=(5,0,0), radius=2, point=(5,0,0))
   - **Expected:** Signed distance = -2.0 (negative = inside)
2. **#2** — Query point outside sphere (point=(8,0,0))
   - **Expected:** Signed distance = +1.0 (positive = outside)

### TC-4.4.5.1 Batch Correctness

| # | Requirement |
|---|-------------|
| 1 | R-4.4.5     |

1. **#1** — 100 rays, run as batch and individually
   - **Expected:** Results identical for all 100 rays

### TC-4.4.NF2.1 Batch Parallel Speedup

| # | Requirement |
|---|-------------|
| 1 | R-4.4.NF2   |

1. **#1** — 1000 rays on 8 cores, batch vs sequential
   - **Expected:** Batch >= 4x faster than sequential

### TC-4.4.NF2.2 Batch Scalability

| # | Requirement |
|---|-------------|
| 1 | R-4.4.NF2   |

1. **#1** — 10,000 rays on 4, 8, 16 cores
   - **Expected:** >= 75% parallel efficiency at each core count

### TC-4.4.6.1 Filter Layers

| # | Requirement |
|---|-------------|
| 1 | R-4.4.6     |

1. **#1** — Ray query excluding layer "debris", 50 debris entities in path
   - **Expected:** Zero debris entities in results, excluded from narrowphase

### TC-4.4.6.2 Filter With/Without

| # | Requirement |
|---|-------------|
| 1 | R-4.4.6     |
| 2 | R-4.4.6     |

1. **#1** — Query with With(Enemy) filter, 20 enemies and 80 props in range
   - **Expected:** Only 20 enemy entities returned
2. **#2** — Query with Without(Static) filter
   - **Expected:** Static entities excluded

### TC-4.4.6.3 Filter Predicate

| # | Requirement |
|---|-------------|
| 1 | R-4.4.6     |

1. **#1** — Custom predicate: health > 0, 60 alive and 40 dead entities
   - **Expected:** Only 60 alive entities in results

### TC-4.4.6.4 Filter Combinations

| # | Requirement |
|---|-------------|
| 1 | R-4.4.6     |

1. **#1** — Layer filter + With filter + predicate combined
   - **Expected:** Intersection of all three filters, correct results

### TC-4.5.1.1 Suspension Rest

| # | Requirement |
|---|-------------|
| 1 | R-4.5.1     |

1. **#1** — 1500 kg vehicle, 4 wheels, dropped from 0.5 m
   - **Expected:** Settles within 1 mm of rest height in 2 s

### TC-4.5.NF1.1 Suspension 20 Vehicle Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.5.NF1   |

1. **#1** — 20 vehicles (4 wheels each = 80 wheels)
   - **Expected:** Full suspension pipeline within 4 ms

### TC-4.5.2.1 Pacejka Curve

| # | Requirement |
|---|-------------|
| 1 | R-4.5.2     |

1. **#1** — Sweep slip angle 0-20 degrees, standard tire parameters
   - **Expected:** Lateral force within 5% of Pacejka reference curve at each angle

### TC-4.5.2.2 Surface Friction

| # | Requirement |
|---|-------------|
| 1 | R-4.5.2     |

1. **#1** — Vehicle on asphalt (friction=0.9), then on ice (friction=0.1)
   - **Expected:** Grip reduces measurably on ice vs asphalt

### TC-4.5.3.1 Torque Distribution

| # | Requirement |
|---|-------------|
| 1 | R-4.5.3     |

1. **#1** — RWD vehicle, engine_torque=300, gear_ratio=3.0
   - **Expected:** Sum of rear wheel torques = 300 * 3.0 = 900, within 1%

### TC-4.5.3.2 Differential Types

| # | Requirement |
|---|-------------|
| 1 | R-4.5.3     |
| 2 | R-4.5.3     |
| 3 | R-4.5.3     |

1. **#1** — Open differential, one wheel on ice
   - **Expected:** All torque to slipping wheel
2. **#2** — Locked differential, one wheel on ice
   - **Expected:** Equal torque split (50/50)
3. **#3** — LSD differential, one wheel on ice
   - **Expected:** Partial transfer to gripping wheel

### TC-4.5.4.1 Anti-Roll Effectiveness

| # | Requirement |
|---|-------------|
| 1 | R-4.5.4     |

1. **#1** — 0.5g lateral acceleration, with anti-roll bars vs without
   - **Expected:** >= 30% less body roll with bars

### TC-4.5.5.1 Tracked Vehicle Turning Radius

| # | Requirement |
|---|-------------|
| 1 | R-4.5.5     |

1. **#1** — Track length=5m, left_speed=5 m/s, right_speed=3 m/s
   - **Expected:** Turning radius within 10% of expected (R = L*(R+L)/(2*(L-R)))

### TC-4.5.6.1 Hover Altitude Stabilization

| # | Requirement |
|---|-------------|
| 1 | R-4.5.6     |

1. **#1** — 4 repulsors at target height=2m, vehicle dropped from 3m
   - **Expected:** Stabilizes within 5% of 2m (1.9-2.1m) in 5 s

### TC-4.5.6.2 Hover Velocity at Equilibrium

| # | Requirement |
|---|-------------|
| 1 | R-4.5.6     |

1. **#1** — Hover vehicle at equilibrium altitude
   - **Expected:** Vertical velocity < 0.01 m/s

### TC-4.5.7.1 Vehicle Replication Divergence

| # | Requirement |
|---|-------------|
| 1 | R-4.5.7     |

1. **#1** — Server-authoritative vehicle, client at 100 ms latency
   - **Expected:** Client-server position divergence < 10 cm

### TC-4.5.NF2.1 Vehicle Bandwidth

| # | Requirement |
|---|-------------|
| 1 | R-4.5.NF2   |

1. **#1** — Delta-compressed vehicle state per tick
   - **Expected:** Payload < 256 bytes/vehicle

### TC-4.6.1.1 Voronoi Fragment Volume

| # | Requirement |
|---|-------------|
| 1 | R-4.6.1     |

1. **#1** — 1x1x1 cube fractured into 20 Voronoi fragments
   - **Expected:** Sum of fragment volumes within 1% of 1.0

### TC-4.6.2.1 Pre-Fractured Load

| # | Requirement |
|---|-------------|
| 1 | R-4.6.2     |

1. **#1** — 15-fragment pre-fractured asset
   - **Expected:** Loads and spawns all fragments in 1 frame

### TC-4.6.3.1 Fracture Trigger

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Apply damage > fracture threshold to intact entity
   - **Expected:** Intact entity despawned, fragment entities spawned

### TC-4.6.3.2 Fracture Fragment Positions

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Activate fracture on known asset
   - **Expected:** Fragment transforms match asset layout exactly

### TC-4.6.NF1.1 Fracture 50 Fragment Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.6.NF1   |

1. **#1** — Activate 50-fragment fracture
   - **Expected:** Completes within 2 ms

### TC-4.6.4.1 Progressive Damage

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |

1. **#1** — 3-stage DamageHealth (thresholds at 75%, 50%, 25%)
   - **Expected:** Stages trigger in order at correct thresholds within 5%

### TC-4.6.4.2 Damage Replication

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |

1. **#1** — DamageHealth component modified on server
   - **Expected:** Replicated to clients via ECS replication

### TC-4.6.5.1 Structural Collapse

| # | Requirement |
|---|-------------|
| 1 | R-4.6.5     |

1. **#1** — Break keystone joint in arch structure
   - **Expected:** Unsupported fragments above keystone fall due to gravity

### TC-4.6.NF3.1 Structural Analysis 200 Nodes

| # | Requirement |
|---|-------------|
| 1 | R-4.6.NF3   |

1. **#1** — 200-node structural graph, traverse after break
   - **Expected:** Traversal completes within 0.5 ms

### TC-4.6.NF2.1 Debris Cap Enforcement

| # | Requirement |
|---|-------------|
| 1 | R-4.6.NF2   |

1. **#1** — Debris cap=200, spawn 500 fragments
   - **Expected:** Active debris never exceeds 200

### TC-4.6.6.1 Debris TTL Expiry

| # | Requirement |
|---|-------------|
| 1 | R-4.6.6     |

1. **#1** — Debris with TTL=5 s, advance 5 s
   - **Expected:** Debris despawned within 1 frame of expiry

### TC-4.6.NF2.2 Debris Sustained Load

| # | Requirement |
|---|-------------|
| 1 | R-4.6.NF2   |

1. **#1** — 2000 fragments spawned over 30 s, cap=500
   - **Expected:** Stable frame rates throughout

### TC-4.6.7.1 Debris Pooling Efficiency

| # | Requirement |
|---|-------------|
| 1 | R-4.6.7     |

1. **#1** — 100 fracture activations with pooling vs without
   - **Expected:** Pooling reduces allocations by >= 80%

### TC-4.6.7.2 Debris LOD Removal

| # | Requirement |
|---|-------------|
| 1 | R-4.6.7     |

1. **#1** — Debris fragment beyond max LOD distance
   - **Expected:** No RigidBody or Collider components present

### TC-4.7.1.1 XPBD Convergence

| # | Requirement |
|---|-------------|
| 1 | R-4.7.1     |

1. **#1** — 10x10 cloth grid, 10 solver iterations
   - **Expected:** All constraints satisfied within 1% of rest length

### TC-4.7.NF1.1 XPBD Single Instance Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.7.NF1   |

1. **#1** — 20x20 cloth, 10 iterations
   - **Expected:** Completes within 0.5 ms

### TC-4.7.2.1 Cloth Attachment

| # | Requirement |
|---|-------------|
| 1 | R-4.7.2     |

1. **#1** — Cloth attached to skeleton bone, bone moves
   - **Expected:** Attached particles within 0.1 mm of bone position

### TC-4.7.2.2 Cloth Gravity

| # | Requirement |
|---|-------------|
| 1 | R-4.7.2     |

1. **#1** — Cloth with 4 corners attached, center free
   - **Expected:** Free particles fall under gravity, attached corners remain fixed

### TC-4.7.NF1.2 Cloth 8 Instance Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.7.NF1   |

1. **#1** — 8 cloth instances (20x20 each)
   - **Expected:** Total solve time within 4 ms

### TC-4.7.NF2.1 Cloth Memory Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.7.NF2   |

1. **#1** — Single 20x20 cloth instance
   - **Expected:** Buffer size <= 64 KB

### TC-4.7.3.1 Self Collision

| # | Requirement |
|---|-------------|
| 1 | R-4.7.3     |

1. **#1** — Cloth draped over sphere, particles settling
   - **Expected:** No non-adjacent particles closer than cloth thickness

### TC-4.7.4.1 Two-Way Coupling

| # | Requirement |
|---|-------------|
| 1 | R-4.7.4     |

1. **#1** — 10 kg cloth resting on 1 kg rigid body
   - **Expected:** ExternalForce on rigid body is non-zero (cloth pushes body)

### TC-4.7.5.1 Wind Proportional Response

| # | Requirement |
|---|-------------|
| 1 | R-4.7.5     |

1. **#1** — Wind strength=10; measure displacement. Double to strength=20
   - **Expected:** Displacement increases >= 50%

### TC-4.7.5.2 Wind Shared Field

| # | Requirement |
|---|-------------|
| 1 | R-4.7.5     |

1. **#1** — Cloth reads wind from shared wind texture
   - **Expected:** Reads shared texture, not WindSource component directly

### TC-4.7.NF3.1 Wind 16 Source Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.7.NF3   |

1. **#1** — 16 wind sources contributing to field
   - **Expected:** Wind field computation within 0.2 ms

### TC-4.7.6.1 Cloth Tearing

| # | Requirement |
|---|-------------|
| 1 | R-4.7.6     |

1. **#1** — Apply force exceeding tear threshold on constraint
   - **Expected:** Cloth splits at constraint, particles preserved on both sides

### TC-4.7.7.1 Cloth LOD Cost Reduction

| # | Requirement |
|---|-------------|
| 1 | R-4.7.7     |

1. **#1** — Cloth at LOD=0 (full) vs LOD=1 (reduced)
   - **Expected:** >= 50% cost reduction at LOD=1

### TC-4.7.7.2 Cloth LOD Zero Sim

| # | Requirement |
|---|-------------|
| 1 | R-4.7.7     |

1. **#1** — Cloth beyond max LOD distance
   - **Expected:** Zero solver invocations for this instance

### TC-4.7.7.3 Cloth LOD Smooth Transition

| # | Requirement |
|---|-------------|
| 1 | R-4.7.7     |

1. **#1** — Move camera across LOD boundary back and forth
   - **Expected:** No visible popping during transitions

### TC-4.8.1.1 SPH Incompressibility

| # | Requirement |
|---|-------------|
| 1 | R-4.8.1     |

1. **#1** — 1,000 SPH particles in sealed box, 500 ticks
   - **Expected:** Density within 5% of rest density for all particles

### TC-4.8.NF1.1 SPH 50K Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.8.NF1   |

1. **#1** — 50,000 SPH particles on GPU
   - **Expected:** GPU compute within 4 ms

### TC-4.8.2.1 FLIP Energy Conservation

| # | Requirement |
|---|-------------|
| 1 | R-4.8.2     |

1. **#1** — Sealed box, no external forces, 1000 ticks
   - **Expected:** Energy loss < 10% per second

### TC-4.8.3.1 Eulerian Divergence Free

| # | Requirement |
|---|-------------|
| 1 | R-4.8.3     |

1. **#1** — 64x64x64 grid, pressure solve
   - **Expected:** Residual divergence < 1e-4

### TC-4.8.4.1 Surface Mesh Watertight

| # | Requirement |
|---|-------------|
| 1 | R-4.8.4     |

1. **#1** — 10,000 particles, marching cubes reconstruction
   - **Expected:** Mesh has no boundary edges (watertight)

### TC-4.8.4.2 Surface Reconstruction Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.8.4     |

1. **#1** — 10,000 particles surface reconstruction
   - **Expected:** Completes within 4 ms

### TC-4.8.5.1 Water Tile Seam

| # | Requirement |
|---|-------------|
| 1 | R-4.8.5     |

1. **#1** — Two adjacent ocean tiles
   - **Expected:** Edge gap < 1 mm

### TC-4.8.NF3.1 Water 1km Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.8.NF3   |

1. **#1** — 1 km x 1 km ocean surface
   - **Expected:** GPU compute within 0.5 ms

### TC-4.8.6.1 Neutral Buoyancy

| # | Requirement |
|---|-------------|
| 1 | R-4.8.6     |

1. **#1** — Body with density equal to fluid density
   - **Expected:** Acceleration < 0.01 m/s^2 (neutral buoyancy)

### TC-4.8.6.2 Buoyancy 64 Body Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.8.6     |

1. **#1** — 64 buoyant bodies in water
   - **Expected:** Completes within physics budget

### TC-4.8.7.1 Splash Displacement

| # | Requirement |
|---|-------------|
| 1 | R-4.8.7     |

1. **#1** — 10 kg sphere dropped at 5 m/s into water
   - **Expected:** Peak displacement >= 10x rest particle spacing

### TC-4.8.7.2 Fluid Deceleration

| # | Requirement |
|---|-------------|
| 1 | R-4.8.7     |

1. **#1** — Sphere falling through fluid vs freefall
   - **Expected:** Sphere in fluid decelerates (takes longer to fall same distance)

### TC-4.8.NF2.1 Fluid Memory Budget

| # | Requirement |
|---|-------------|
| 1 | R-4.8.NF2   |

1. **#1** — 4 fluid volumes at max configuration
   - **Expected:** GPU memory <= 128 MB total

## Integration Tests

### TC-4.4.5.I1 Batch vs Sequential Equivalence

| # | Requirement |
|---|-------------|
| 1 | R-4.4.5     |

1. **#1** — 1000 rays batch and 1000 rays sequential against same scene
   - **Expected:** Identical results for every ray

### TC-4.5.1.I1 Vehicle Full Pipeline

| # | Requirement |
|---|-------------|
| 1 | R-4.5.1     |

1. **#1** — Drive vehicle over varied terrain for 60 s
   - **Expected:** No jitter, suspension stable, frame rate maintained

### TC-4.6.3.I1 Fracture Chain Reaction

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Destroy keystone in multi-object structure
   - **Expected:** Cascade fracture propagates, debris capped, stable frame rate

### TC-4.7.2.I1 Cloth on Animated Character

| # | Requirement |
|---|-------------|
| 1 | R-4.7.2     |

1. **#1** — Cloth cape on running character for 10 s
   - **Expected:** No cloth explosion, attachment holds, collisions work

### TC-4.8.1.I1 Fluid Rigid Body Interaction

| # | Requirement |
|---|-------------|
| 1 | R-4.8.1     |

1. **#1** — Drop 10 rigid bodies into fluid volume
   - **Expected:** Buoyancy, drag, splash effects all active, stable simulation

## Benchmarks

### TC-4.4.NF1.B1 Single Ray Cast

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single ray vs 50,000 entities | Latency | < 5 us | R-4.4.NF1 |

### TC-4.4.NF2.B1 Batch 1K Rays

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1,000 rays on 8 cores | Speedup vs sequential | >= 4x | R-4.4.NF2 |

### TC-4.5.NF1.B1 Vehicle Pipeline

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20 vehicles (4 wheels each) | Total pipeline time | < 4 ms | R-4.5.NF1 |

### TC-4.6.NF1.B1 Fracture Activation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50-fragment fracture activation | Total time | < 2 ms | R-4.6.NF1 |

### TC-4.6.NF3.B1 Structural Analysis

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 200-node structural graph traversal | Total time | < 0.5 ms | R-4.6.NF3 |

### TC-4.7.NF1.B1 Cloth Single Instance

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 20x20 cloth, 10 iterations | Solve time | < 0.5 ms | R-4.7.NF1 |

### TC-4.7.NF3.B1 Wind Field

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 16 wind sources | Field computation time | < 0.2 ms | R-4.7.NF3 |

### TC-4.8.NF1.B1 SPH 50K Particles

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 50,000 SPH particles | GPU compute time | < 4 ms | R-4.8.NF1 |

### TC-4.8.NF3.B1 Water Surface 1km

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1 km x 1 km ocean surface | GPU compute time | < 0.5 ms | R-4.8.NF3 |
