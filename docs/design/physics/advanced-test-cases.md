# Advanced Physics Test Cases

Companion test cases for [advanced.md](advanced.md).

## Unit Tests

### TC-4.4.1.1 Ray Cast Sphere Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray origin=(0,0,0), dir=(1,0,0); sphere center=(5,0,0), radius=1 | Hit at pos=(4,0,0), error < 0.1 mm | R-4.4.1 |
| 2 | Hit normal | (−1,0,0), within 0.001 rad of analytical | R-4.4.1 |

### TC-4.4.1.2 Ray Cast Normal Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray tangent to sphere, analytical normal known | Reported normal within 0.001 rad of analytical | R-4.4.1 |

### TC-4.4.1.3 Ray Cast All Shapes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray vs box (axis-aligned, 2x2x2 at origin) | Hit at correct face, distance matches analytical | R-4.4.1 |
| 2 | Ray vs sphere (radius=1 at (5,0,0)) | Hit position matches analytical within 0.1 mm | R-4.4.1 |
| 3 | Ray vs capsule (radius=0.5, half_height=1) | Hit detected, contact valid | R-4.4.1 |
| 4 | Ray vs convex hull (8-vertex cube) | Hit detected, contact valid | R-4.4.1 |
| 5 | Ray vs triangle mesh (flat quad) | Hit detected at surface | R-4.4.1 |
| 6 | Ray vs heightfield (flat at y=0) | Hit at y=0 | R-4.4.1 |

### TC-4.4.1.4 Ray Cast Miss

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray origin=(0,0,0), dir=(0,1,0); no entities in path | `None` returned | R-4.4.1 |

### TC-4.4.NF1.1 Ray Cast 50K Latency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single ray vs 50,000 entities in shared BVH | Completes within 5 us | R-4.4.NF1 |

### TC-4.4.2.1 Shape Cast Ground Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capsule (radius=0.5, half_height=1) swept downward from y=5 toward ground plane at y=0 | Ground detected, contact point at y=0.5 (capsule bottom) | R-4.4.2 |

### TC-4.4.2.2 Shape Cast No Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Capsule swept in direction with no obstacles, max_dist=100 | `None` returned | R-4.4.2 |

### TC-4.4.2.3 Shape Cast Contact Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sphere (radius=1) swept toward box, known analytical contact | Contact point within 0.1 mm of analytical solution | R-4.4.2 |

### TC-4.4.3.1 Overlap Sphere AOE

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 entities, sphere overlap radius=10 at origin, 30 within range | Exactly 30 entities returned, zero false positives, zero false negatives | R-4.4.3 |

### TC-4.4.3.2 Overlap All Shapes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Box overlap (2x2x2) at known position with known entities | Correct entity set returned | R-4.4.3 |
| 2 | Capsule overlap at known position | Correct entity set returned | R-4.4.3 |
| 3 | Convex hull overlap at known position | Correct entity set returned | R-4.4.3 |

### TC-4.4.4.1 Closest Point Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query point (3,0,0), sphere center=(5,0,0) radius=1 | Closest point = (4,0,0), distance = 1.0, within 0.1 mm | R-4.4.4 |
| 2 | Query point (1,1,0), box min=(2,0,0) max=(4,2,2) | Closest point = (2,1,0), within 0.1 mm | R-4.4.4 |
| 3 | Query point (0,5,0), capsule at origin | Closest point within 0.1 mm of analytical | R-4.4.4 |

### TC-4.4.4.2 Closest Point Signed Distance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query point inside sphere (center=(5,0,0), radius=2, point=(5,0,0)) | Signed distance = -2.0 (negative = inside) | R-4.4.4 |
| 2 | Query point outside sphere (point=(8,0,0)) | Signed distance = +1.0 (positive = outside) | R-4.4.4 |

### TC-4.4.5.1 Batch Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 rays, run as batch and individually | Results identical for all 100 rays | R-4.4.5 |

### TC-4.4.NF2.1 Batch Parallel Speedup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 rays on 8 cores, batch vs sequential | Batch >= 4x faster than sequential | R-4.4.NF2 |

### TC-4.4.NF2.2 Batch Scalability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 rays on 4, 8, 16 cores | >= 75% parallel efficiency at each core count | R-4.4.NF2 |

### TC-4.4.6.1 Filter Layers

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ray query excluding layer "debris", 50 debris entities in path | Zero debris entities in results, excluded from narrowphase | R-4.4.6 |

### TC-4.4.6.2 Filter With/Without

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query with With(Enemy) filter, 20 enemies and 80 props in range | Only 20 enemy entities returned | R-4.4.6 |
| 2 | Query with Without(Static) filter | Static entities excluded | R-4.4.6 |

### TC-4.4.6.3 Filter Predicate

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom predicate: health > 0, 60 alive and 40 dead entities | Only 60 alive entities in results | R-4.4.6 |

### TC-4.4.6.4 Filter Combinations

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Layer filter + With filter + predicate combined | Intersection of all three filters, correct results | R-4.4.6 |

### TC-4.5.1.1 Suspension Rest

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1500 kg vehicle, 4 wheels, dropped from 0.5 m | Settles within 1 mm of rest height in 2 s | R-4.5.1 |

### TC-4.5.NF1.1 Suspension 20 Vehicle Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20 vehicles (4 wheels each = 80 wheels) | Full suspension pipeline within 4 ms | R-4.5.NF1 |

### TC-4.5.2.1 Pacejka Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sweep slip angle 0-20 degrees, standard tire parameters | Lateral force within 5% of Pacejka reference curve at each angle | R-4.5.2 |

### TC-4.5.2.2 Surface Friction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vehicle on asphalt (friction=0.9), then on ice (friction=0.1) | Grip reduces measurably on ice vs asphalt | R-4.5.2 |

### TC-4.5.3.1 Torque Distribution

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | RWD vehicle, engine_torque=300, gear_ratio=3.0 | Sum of rear wheel torques = 300 * 3.0 = 900, within 1% | R-4.5.3 |

### TC-4.5.3.2 Differential Types

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open differential, one wheel on ice | All torque to slipping wheel | R-4.5.3 |
| 2 | Locked differential, one wheel on ice | Equal torque split (50/50) | R-4.5.3 |
| 3 | LSD differential, one wheel on ice | Partial transfer to gripping wheel | R-4.5.3 |

### TC-4.5.4.1 Anti-Roll Effectiveness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 0.5g lateral acceleration, with anti-roll bars vs without | >= 30% less body roll with bars | R-4.5.4 |

### TC-4.5.5.1 Tracked Vehicle Turning Radius

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Track length=5m, left_speed=5 m/s, right_speed=3 m/s | Turning radius within 10% of expected (R = L*(R+L)/(2*(L-R))) | R-4.5.5 |

### TC-4.5.6.1 Hover Altitude Stabilization

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 repulsors at target height=2m, vehicle dropped from 3m | Stabilizes within 5% of 2m (1.9-2.1m) in 5 s | R-4.5.6 |

### TC-4.5.6.2 Hover Velocity at Equilibrium

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hover vehicle at equilibrium altitude | Vertical velocity < 0.01 m/s | R-4.5.6 |

### TC-4.5.7.1 Vehicle Replication Divergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server-authoritative vehicle, client at 100 ms latency | Client-server position divergence < 10 cm | R-4.5.7 |

### TC-4.5.NF2.1 Vehicle Bandwidth

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Delta-compressed vehicle state per tick | Payload < 256 bytes/vehicle | R-4.5.NF2 |

### TC-4.6.1.1 Voronoi Fragment Volume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1x1x1 cube fractured into 20 Voronoi fragments | Sum of fragment volumes within 1% of 1.0 | R-4.6.1 |

### TC-4.6.2.1 Pre-Fractured Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 15-fragment pre-fractured asset | Loads and spawns all fragments in 1 frame | R-4.6.2 |

### TC-4.6.3.1 Fracture Trigger

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply damage > fracture threshold to intact entity | Intact entity despawned, fragment entities spawned | R-4.6.3 |

### TC-4.6.3.2 Fracture Fragment Positions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate fracture on known asset | Fragment transforms match asset layout exactly | R-4.6.3 |

### TC-4.6.NF1.1 Fracture 50 Fragment Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate 50-fragment fracture | Completes within 2 ms | R-4.6.NF1 |

### TC-4.6.4.1 Progressive Damage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-stage DamageHealth (thresholds at 75%, 50%, 25%) | Stages trigger in order at correct thresholds within 5% | R-4.6.4 |

### TC-4.6.4.2 Damage Replication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | DamageHealth component modified on server | Replicated to clients via ECS replication | R-4.6.4 |

### TC-4.6.5.1 Structural Collapse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Break keystone joint in arch structure | Unsupported fragments above keystone fall due to gravity | R-4.6.5 |

### TC-4.6.NF3.1 Structural Analysis 200 Nodes

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 200-node structural graph, traverse after break | Traversal completes within 0.5 ms | R-4.6.NF3 |

### TC-4.6.NF2.1 Debris Cap Enforcement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris cap=200, spawn 500 fragments | Active debris never exceeds 200 | R-4.6.NF2 |

### TC-4.6.6.1 Debris TTL Expiry

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris with TTL=5 s, advance 5 s | Debris despawned within 1 frame of expiry | R-4.6.6 |

### TC-4.6.NF2.2 Debris Sustained Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2000 fragments spawned over 30 s, cap=500 | Stable frame rates throughout | R-4.6.NF2 |

### TC-4.6.7.1 Debris Pooling Efficiency

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 fracture activations with pooling vs without | Pooling reduces allocations by >= 80% | R-4.6.7 |

### TC-4.6.7.2 Debris LOD Removal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris fragment beyond max LOD distance | No RigidBody or Collider components present | R-4.6.7 |

### TC-4.7.1.1 XPBD Convergence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10x10 cloth grid, 10 solver iterations | All constraints satisfied within 1% of rest length | R-4.7.1 |

### TC-4.7.NF1.1 XPBD Single Instance Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 20x20 cloth, 10 iterations | Completes within 0.5 ms | R-4.7.NF1 |

### TC-4.7.2.1 Cloth Attachment

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth attached to skeleton bone, bone moves | Attached particles within 0.1 mm of bone position | R-4.7.2 |

### TC-4.7.2.2 Cloth Gravity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth with 4 corners attached, center free | Free particles fall under gravity, attached corners remain fixed | R-4.7.2 |

### TC-4.7.NF1.2 Cloth 8 Instance Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8 cloth instances (20x20 each) | Total solve time within 4 ms | R-4.7.NF1 |

### TC-4.7.NF2.1 Cloth Memory Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Single 20x20 cloth instance | Buffer size <= 64 KB | R-4.7.NF2 |

### TC-4.7.3.1 Self Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth draped over sphere, particles settling | No non-adjacent particles closer than cloth thickness | R-4.7.3 |

### TC-4.7.4.1 Two-Way Coupling

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 kg cloth resting on 1 kg rigid body | ExternalForce on rigid body is non-zero (cloth pushes body) | R-4.7.4 |

### TC-4.7.5.1 Wind Proportional Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wind strength=10; measure displacement. Double to strength=20 | Displacement increases >= 50% | R-4.7.5 |

### TC-4.7.5.2 Wind Shared Field

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth reads wind from shared wind texture | Reads shared texture, not WindSource component directly | R-4.7.5 |

### TC-4.7.NF3.1 Wind 16 Source Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 16 wind sources contributing to field | Wind field computation within 0.2 ms | R-4.7.NF3 |

### TC-4.7.6.1 Cloth Tearing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply force exceeding tear threshold on constraint | Cloth splits at constraint, particles preserved on both sides | R-4.7.6 |

### TC-4.7.7.1 Cloth LOD Cost Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth at LOD=0 (full) vs LOD=1 (reduced) | >= 50% cost reduction at LOD=1 | R-4.7.7 |

### TC-4.7.7.2 Cloth LOD Zero Sim

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth beyond max LOD distance | Zero solver invocations for this instance | R-4.7.7 |

### TC-4.7.7.3 Cloth LOD Smooth Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move camera across LOD boundary back and forth | No visible popping during transitions | R-4.7.7 |

### TC-4.8.1.1 SPH Incompressibility

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1,000 SPH particles in sealed box, 500 ticks | Density within 5% of rest density for all particles | R-4.8.1 |

### TC-4.8.NF1.1 SPH 50K Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 50,000 SPH particles on GPU | GPU compute within 4 ms | R-4.8.NF1 |

### TC-4.8.2.1 FLIP Energy Conservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sealed box, no external forces, 1000 ticks | Energy loss < 10% per second | R-4.8.2 |

### TC-4.8.3.1 Eulerian Divergence Free

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64x64x64 grid, pressure solve | Residual divergence < 1e-4 | R-4.8.3 |

### TC-4.8.4.1 Surface Mesh Watertight

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 particles, marching cubes reconstruction | Mesh has no boundary edges (watertight) | R-4.8.4 |

### TC-4.8.4.2 Surface Reconstruction Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10,000 particles surface reconstruction | Completes within 4 ms | R-4.8.4 |

### TC-4.8.5.1 Water Tile Seam

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two adjacent ocean tiles | Edge gap < 1 mm | R-4.8.5 |

### TC-4.8.NF3.1 Water 1km Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1 km x 1 km ocean surface | GPU compute within 0.5 ms | R-4.8.NF3 |

### TC-4.8.6.1 Neutral Buoyancy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Body with density equal to fluid density | Acceleration < 0.01 m/s^2 (neutral buoyancy) | R-4.8.6 |

### TC-4.8.6.2 Buoyancy 64 Body Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64 buoyant bodies in water | Completes within physics budget | R-4.8.6 |

### TC-4.8.7.1 Splash Displacement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 kg sphere dropped at 5 m/s into water | Peak displacement >= 10x rest particle spacing | R-4.8.7 |

### TC-4.8.7.2 Fluid Deceleration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sphere falling through fluid vs freefall | Sphere in fluid decelerates (takes longer to fall same distance) | R-4.8.7 |

### TC-4.8.NF2.1 Fluid Memory Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 fluid volumes at max configuration | GPU memory <= 128 MB total | R-4.8.NF2 |

## Integration Tests

### TC-4.4.5.I1 Batch vs Sequential Equivalence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 rays batch and 1000 rays sequential against same scene | Identical results for every ray | R-4.4.5 |

### TC-4.5.1.I1 Vehicle Full Pipeline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drive vehicle over varied terrain for 60 s | No jitter, suspension stable, frame rate maintained | R-4.5.1 |

### TC-4.6.3.I1 Fracture Chain Reaction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Destroy keystone in multi-object structure | Cascade fracture propagates, debris capped, stable frame rate | R-4.6.3 |

### TC-4.7.2.I1 Cloth on Animated Character

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth cape on running character for 10 s | No cloth explosion, attachment holds, collisions work | R-4.7.2 |

### TC-4.8.1.I1 Fluid Rigid Body Interaction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Drop 10 rigid bodies into fluid volume | Buoyancy, drag, splash effects all active, stable simulation | R-4.8.1 |

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
