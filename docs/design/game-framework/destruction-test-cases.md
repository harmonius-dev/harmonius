# Destruction System Test Cases

Companion test cases for [destruction.md](destruction.md).

## Unit Tests

### TC-4.6.1.1 Voronoi Volume Preservation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fracture unit cube into 20 fragments | Sum of fragment volumes within 1% of 1.0 m^3 | R-4.6.1 |

### TC-4.6.1.2 Convex Hull Validity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Generate 20 fragments from Voronoi | All fragments pass convex hull validation | R-4.6.1 |

### TC-4.6.1.3 Platform Fragment Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fracture on Mobile tier | Fragment count <= 8 | R-4.6.1 |
| 2 | Fracture on Desktop tier | Fragment count <= 64 | R-4.6.1 |

### TC-4.6.2.1 DCC Asset Load

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load pre-fractured `.frac` asset | Fragment count, connectivity edges, and joint configs match authored data | R-4.6.2 |

### TC-4.6.3.1 Fracture Activation Threshold

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply damage exceeding threshold (integrity -> 0) | Intact entity despawned; fragment entities spawned | R-4.6.3 |
| 2 | Apply damage below threshold (integrity > 0) | Intact entity remains; no fragments | R-4.6.3 |

### TC-4.6.3.2 Fragment Transform Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Activate fracture; compare fragment positions to asset | All positions within 0.001 m of asset layout | R-4.6.3 |

### TC-4.6.3.3 Activation Budget Per Frame

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Trigger 3 activations on Mobile (max=1) | Only 1 activation per frame; remaining deferred | R-4.6.3 |

### TC-4.6.3.4 Staggered Spawning

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 64-fragment object on Mobile (max 8/activation) | Fragments spawn across 8 frames | R-4.6.3 |

### TC-4.6.4.1 Damage Accumulation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply 3 impacts: 20, 30, 10 to max_integrity=100 | integrity = 40 | R-4.6.4 |

### TC-4.6.4.2 Visual Stage Progression

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-stage health (thresholds: 0.75, 0.50, 0.25); damage to 60% | current_stage = 1 | R-4.6.4 |
| 2 | Damage further to 20% | current_stage = 2 | R-4.6.4 |

### TC-4.6.4.3 Damage From Multiple Types

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Projectile (20) + explosion (30) + melee (10) | integrity reduced by 60 total | R-4.6.4 |

### TC-4.6.5.1 Cascading Collapse

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 3-column arch; break keystone fragment | Unsupported fragments lose joints and fall | R-4.6.5 |

### TC-4.6.5.2 Anchor Connectivity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fragments connected to StructuralAnchor via joints | Fragments remain supported (visited by BFS) | R-4.6.5 |

### TC-4.6.5.3 Mobile Structural Disabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Run structural analysis on Mobile tier | Pre-baked collapse sequence used; no runtime BFS | R-4.6.5 |

### TC-4.6.6.1 Debris TTL Expiration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn debris with TTL=5.0s; advance 5.0s | Debris entity despawned | R-4.6.6 |

### TC-4.6.6.2 Debris Cap Enforcement

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn 500 debris with max_debris=200 | Oldest 300 despawned; 200 remain | R-4.6.6 |

### TC-4.6.6.3 Debris Sleep Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris velocity below threshold for N frames | State transitions to Sleeping; zero sim cost | R-4.6.6 |

### TC-4.6.7.1 Pooling Reduces Allocation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 destructions with pool vs without pool | Pooled allocations >= 80% fewer | R-4.6.7 |

### TC-4.6.7.2 LOD Component Removal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris beyond `particle_distance` | No RigidBody or Collider components | R-4.6.7 |
| 2 | Debris between `lod_distance` and `particle_distance` | Simplified collision shape | R-4.6.7 |

## Integration Tests

### TC-4.6.4.I1 Damage Replication

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Server modifies DamageHealth; client observes | Client receives replicated integrity value; client cannot modify locally | R-4.6.4 |

### TC-4.6.3.I1 Destruction With Cover

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Destroy cover object | Cover points removed from spatial index | R-4.6.3 |

### TC-4.6.3.I2 Destruction Audio

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fracture activation triggers | Destruction audio event emitted | R-4.6.3 |

### TC-4.6.7.I1 Debris Visual Particles

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Debris beyond particle_distance | Rendered as visual particles; no physics simulation | R-4.6.7 |

## Benchmarks

### TC-4.6.NF1.B1 Fracture Activation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Activate fracture (50 fragments) | Wall-clock time | < 2 ms | R-4.6.NF1 |

### TC-4.6.NF3.B1 Structural Analysis

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | BFS traversal (200 nodes) | Wall-clock time | < 0.5 ms | R-4.6.NF3 |

### TC-4.6.NF2.B1 Sustained Destruction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 2000 fragments active, cap 500 | Frame rate | Stable (>= 30 fps) | R-4.6.NF2 |

### TC-4.6.7.B1 Debris Pool Operations

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Acquire/release cycle | Wall-clock time | < 1 us per op | R-4.6.7 |

### TC-4.6.7.B2 Debris LOD Query

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Distance query for 512 debris entities | Wall-clock time | < 0.5 ms | R-4.6.7 |
