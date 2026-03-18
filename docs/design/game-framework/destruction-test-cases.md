# Destruction System Test Cases

Companion test cases for [destruction.md](destruction.md).

## Unit Tests

### TC-4.6.1.1 Voronoi Volume Preservation

| # | Requirement |
|---|-------------|
| 1 | R-4.6.1     |

1. **#1** — Fracture unit cube into 20 fragments
   - **Expected:** Sum of fragment volumes within 1% of 1.0 m^3

### TC-4.6.1.2 Convex Hull Validity

| # | Requirement |
|---|-------------|
| 1 | R-4.6.1     |

1. **#1** — Generate 20 fragments from Voronoi
   - **Expected:** All fragments pass convex hull validation

### TC-4.6.1.3 Platform Fragment Cap

| # | Requirement |
|---|-------------|
| 1 | R-4.6.1     |
| 2 | R-4.6.1     |

1. **#1** — Fracture on Mobile tier
   - **Expected:** Fragment count <= 8
2. **#2** — Fracture on Desktop tier
   - **Expected:** Fragment count <= 64

### TC-4.6.2.1 DCC Asset Load

| # | Requirement |
|---|-------------|
| 1 | R-4.6.2     |

1. **#1** — Load pre-fractured `.frac` asset
   - **Expected:** Fragment count, connectivity edges, and joint configs match authored data

### TC-4.6.3.1 Fracture Activation Threshold

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |
| 2 | R-4.6.3     |

1. **#1** — Apply damage exceeding threshold (integrity -> 0)
   - **Expected:** Intact entity despawned; fragment entities spawned
2. **#2** — Apply damage below threshold (integrity > 0)
   - **Expected:** Intact entity remains; no fragments

### TC-4.6.3.2 Fragment Transform Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Activate fracture; compare fragment positions to asset
   - **Expected:** All positions within 0.001 m of asset layout

### TC-4.6.3.3 Activation Budget Per Frame

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Trigger 3 activations on Mobile (max=1)
   - **Expected:** Only 1 activation per frame; remaining deferred

### TC-4.6.3.4 Staggered Spawning

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — 64-fragment object on Mobile (max 8/activation)
   - **Expected:** Fragments spawn across 8 frames

### TC-4.6.4.1 Damage Accumulation

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |

1. **#1** — Apply 3 impacts: 20, 30, 10 to max_integrity=100
   - **Expected:** integrity = 40

### TC-4.6.4.2 Visual Stage Progression

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |
| 2 | R-4.6.4     |

1. **#1** — 3-stage health (thresholds: 0.75, 0.50, 0.25); damage to 60%
   - **Expected:** current_stage = 1
2. **#2** — Damage further to 20%
   - **Expected:** current_stage = 2

### TC-4.6.4.3 Damage From Multiple Types

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |

1. **#1** — Projectile (20) + explosion (30) + melee (10)
   - **Expected:** integrity reduced by 60 total

### TC-4.6.5.1 Cascading Collapse

| # | Requirement |
|---|-------------|
| 1 | R-4.6.5     |

1. **#1** — 3-column arch; break keystone fragment
   - **Expected:** Unsupported fragments lose joints and fall

### TC-4.6.5.2 Anchor Connectivity

| # | Requirement |
|---|-------------|
| 1 | R-4.6.5     |

1. **#1** — Fragments connected to StructuralAnchor via joints
   - **Expected:** Fragments remain supported (visited by BFS)

### TC-4.6.5.3 Mobile Structural Disabled

| # | Requirement |
|---|-------------|
| 1 | R-4.6.5     |

1. **#1** — Run structural analysis on Mobile tier
   - **Expected:** Pre-baked collapse sequence used; no runtime BFS

### TC-4.6.6.1 Debris TTL Expiration

| # | Requirement |
|---|-------------|
| 1 | R-4.6.6     |

1. **#1** — Spawn debris with TTL=5.0s; advance 5.0s
   - **Expected:** Debris entity despawned

### TC-4.6.6.2 Debris Cap Enforcement

| # | Requirement |
|---|-------------|
| 1 | R-4.6.6     |

1. **#1** — Spawn 500 debris with max_debris=200
   - **Expected:** Oldest 300 despawned; 200 remain

### TC-4.6.6.3 Debris Sleep Transition

| # | Requirement |
|---|-------------|
| 1 | R-4.6.6     |

1. **#1** — Debris velocity below threshold for N frames
   - **Expected:** State transitions to Sleeping; zero sim cost

### TC-4.6.7.1 Pooling Reduces Allocation

| # | Requirement |
|---|-------------|
| 1 | R-4.6.7     |

1. **#1** — 10 destructions with pool vs without pool
   - **Expected:** Pooled allocations >= 80% fewer

### TC-4.6.7.2 LOD Component Removal

| # | Requirement |
|---|-------------|
| 1 | R-4.6.7     |
| 2 | R-4.6.7     |

1. **#1** — Debris beyond `particle_distance`
   - **Expected:** No RigidBody or Collider components
2. **#2** — Debris between `lod_distance` and `particle_distance`
   - **Expected:** Simplified collision shape

## Integration Tests

### TC-4.6.4.I1 Damage Replication

| # | Requirement |
|---|-------------|
| 1 | R-4.6.4     |

1. **#1** — Server modifies DamageHealth; client observes
   - **Expected:** Client receives replicated integrity value; client cannot modify locally

### TC-4.6.3.I1 Destruction With Cover

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Destroy cover object
   - **Expected:** Cover points removed from spatial index

### TC-4.6.3.I2 Destruction Audio

| # | Requirement |
|---|-------------|
| 1 | R-4.6.3     |

1. **#1** — Fracture activation triggers
   - **Expected:** Destruction audio event emitted

### TC-4.6.7.I1 Debris Visual Particles

| # | Requirement |
|---|-------------|
| 1 | R-4.6.7     |

1. **#1** — Debris beyond particle_distance
   - **Expected:** Rendered as visual particles; no physics simulation

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
