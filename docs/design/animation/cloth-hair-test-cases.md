# Cloth & Hair Animation Test Cases

Companion test cases for [cloth-hair.md](cloth-hair.md).

## Unit Tests

### TC-9.5.1.1 Distance Constraint Rest Length

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |
| 3 | R-9.5.1     |

1. **#1** — Two particles 2.0 units apart, rest_length=1.0, 10 solver iterations
   - **Expected:** Particle distance within 1% of 1.0 (0.99-1.01 units)
2. **#2** — Two particles at rest_length=1.0, compliance=0.0 (rigid), 5 iterations
   - **Expected:** Particle distance exactly 1.0 units
3. **#3** — Two particles 0.5 units apart, rest_length=1.0, 10 iterations
   - **Expected:** Particle distance within 1% of 1.0

### TC-9.5.1.2 Bending Constraint Rest Angle

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |

1. **#1** — Four particles forming 90-degree dihedral, rest_angle=90 deg, 10 iterations
   - **Expected:** Dihedral angle within 2 degrees of 90
2. **#2** — Four particles at 45-degree dihedral, rest_angle=90 deg, 10 iterations
   - **Expected:** Dihedral angle within 2 degrees of 90

### TC-9.5.1.3 Cloth Self-Collision

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |

1. **#1** — 100-vertex cloth plane folded in half, collision_radius=0.01
   - **Expected:** All pairwise distances > 0.01 (no penetration)
2. **#2** — 100-vertex cloth draped over sphere, self-collision enabled
   - **Expected:** Zero particle-particle penetrations after 60 frames

### TC-9.5.1.4 Cloth Wind Response

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |

1. **#1** — Hanging cloth panel, wind=(10,0,0) m/s, 30 frames
   - **Expected:** All particle X positions > initial X positions
2. **#2** — Hanging cloth panel, wind=(0,0,0) m/s, 30 frames
   - **Expected:** Particle X positions unchanged from gravity-only baseline

### TC-9.5.2.1 Strand Gravity

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — 100 guide strands, gravity=(0,-9.81,0), 60 frames
   - **Expected:** All strand tip Y < root Y
2. **#2** — 100 guide strands, gravity=(0,0,0), 60 frames
   - **Expected:** Strand tip Y unchanged from initial

### TC-9.5.2.2 Strand Stretch Constraint

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — 10-segment strand, rest_length=1.0 per segment, gravity applied, 60 frames
   - **Expected:** Total strand length within 1% of 10.0
2. **#2** — 10-segment strand, stretch_compliance=0.0 (rigid)
   - **Expected:** Total strand length exactly 10.0

### TC-9.5.2.3 Strand Collision

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — Collision capsule radius=0.1 at strand midpoint, 30 frames
   - **Expected:** All particles distance from capsule axis >= 0.1
2. **#2** — No collision capsule, strand falls through
   - **Expected:** Strand tip passes below capsule position

### TC-9.5.3.1 Card Spring Physics

| # | Requirement |
|---|-------------|
| 1 | R-9.5.3     |
| 2 | R-9.5.3     |

1. **#1** — Card hair group, impulse=(1,0,0), spring_stiffness=50, spring_damping=5, 30 frames
   - **Expected:** Peak displacement at frame 2-5, amplitude < 10% of peak at frame 30
2. **#2** — Card hair group, no impulse, 30 frames
   - **Expected:** Zero displacement across all frames

### TC-9.5.3.2 Card Anisotropic Specular

| # | Requirement |
|---|-------------|
| 1 | R-9.5.3     |
| 2 | R-9.5.3     |

1. **#1** — Light at 0 deg azimuth, card tangent along Y
   - **Expected:** Specular highlight centered on Y-axis reflection
2. **#2** — Light rotated 90 deg around card
   - **Expected:** Specular highlight shifts 90 deg along anisotropic direction

### TC-9.5.4.1 Hair LOD Tier Selection

| # | Requirement |
|---|-------------|
| 1 | R-9.5.4     |
| 2 | R-9.5.4     |
| 3 | R-9.5.4     |
| 4 | R-9.5.4     |

1. **#1** — Camera at 5 m, strand_max_distance=15
   - **Expected:** HairLodTier::FullStrands
2. **#2** — Camera at 20 m, strand_max_distance=15, cluster_max_distance=40
   - **Expected:** HairLodTier::SimplifiedClusters
3. **#3** — Camera at 50 m, cluster_max_distance=40, card_max_distance=100
   - **Expected:** HairLodTier::CardBased
4. **#4** — Camera at 200 m, card_max_distance=100
   - **Expected:** HairLodTier::Shell

### TC-9.5.4.2 Hair LOD Hysteresis

| # | Requirement |
|---|-------------|
| 1 | R-9.5.4     |
| 2 | R-9.5.4     |
| 3 | R-9.5.4     |

1. **#1** — Camera moves 19 m to 21 m, hysteresis=3.0, strand_max_distance=20
   - **Expected:** No tier change (stays FullStrands)
2. **#2** — Camera moves 19 m to 24 m, hysteresis=3.0, strand_max_distance=20
   - **Expected:** Tier changes to SimplifiedClusters
3. **#3** — Camera moves 24 m to 20 m, hysteresis=3.0, strand_max_distance=20
   - **Expected:** No tier change (stays SimplifiedClusters)

### TC-9.5.4.3 Hair LOD Blend

| # | Requirement |
|---|-------------|
| 1 | R-9.5.4     |
| 2 | R-9.5.4     |

1. **#1** — LOD transition triggered, blend_duration_sec=0.5, dt=1/60
   - **Expected:** blend_alpha reaches 1.0 after 30 frames
2. **#2** — LOD transition triggered, blend_duration_sec=1.0, dt=1/60
   - **Expected:** blend_alpha=0.5 at frame 30, blend_alpha=1.0 at frame 60

### TC-9.5.5.1 Collision Proxy Update

| # | Requirement |
|---|-------------|
| 1 | R-9.5.5     |
| 2 | R-9.5.5     |

1. **#1** — Skeleton pose changed, capsule on upper_arm bone
   - **Expected:** Capsule world position matches bone world transform within 1 frame
2. **#2** — Skeleton at rest pose
   - **Expected:** All capsule positions match bone rest transforms

### TC-9.5.5.2 Collision Friction

| # | Requirement |
|---|-------------|
| 1 | R-9.5.5     |
| 2 | R-9.5.5     |

1. **#1** — Cloth sliding on capsule, friction=0.8, initial tangent velocity=1.0
   - **Expected:** Tangent velocity < 0.2 after 10 frames
2. **#2** — Cloth sliding on capsule, friction=0.0
   - **Expected:** Tangent velocity unchanged after contact

### TC-9.5.6.1 Wind Field Sampling Coherence

| # | Requirement |
|---|-------------|
| 1 | R-9.5.6     |
| 2 | R-9.5.6     |

1. **#1** — Hair and foliage at position (10,0,10), shared wind field
   - **Expected:** Both sample identical wind vector (bitwise equal)
2. **#2** — Hair at (10,0,10), foliage at (20,0,20), wind field with spatial variation
   - **Expected:** Each samples its own position's wind vector correctly

### TC-9.5.6.2 Wind Drag Proportional

| # | Requirement |
|---|-------------|
| 1 | R-9.5.6     |
| 2 | R-9.5.6     |

1. **#1** — Wind speed=5 m/s, drag_coefficient=1.0, 30 frames
   - **Expected:** Strand tip displacement D1
2. **#2** — Wind speed=10 m/s, drag_coefficient=1.0, 30 frames
   - **Expected:** Strand tip displacement within 20% of 2*D1

## Integration Tests

### TC-9.5.1.I1 Cloth on Animated Character

| # | Requirement      |
|---|------------------|
| 1 | R-9.5.1, R-9.5.5 |
| 2 | R-9.5.1, R-9.5.5 |

1. **#1** — 1000-vertex cloak on walking character, 300 frames
   - **Expected:** Zero cloth-body penetrations across all frames
2. **#2** — 1000-vertex cloak on running character, 300 frames
   - **Expected:** Cloth follows character, max distance from body < 2.0 units

### TC-9.5.2.I1 Strand Hair Head Turn

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — Character head rotates 90 degrees over 30 frames
   - **Expected:** Hair swings with peak delay at frame 10-20, no capsule penetrations
2. **#2** — Character head rotates 90 degrees, collision capsules on neck and shoulders
   - **Expected:** Zero strand particles inside any capsule radius

### TC-9.5.4.I1 LOD Flythrough

| # | Requirement |
|---|-------------|
| 1 | R-9.5.4     |
| 2 | R-9.5.4     |

1. **#1** — Camera flies from 5 m to 200 m over 300 frames, per-frame screenshots
   - **Expected:** Zero visible popping artifacts between LOD tiers
2. **#2** — Camera oscillates at LOD boundary for 120 frames
   - **Expected:** No rapid tier oscillation (max 2 transitions total)

### TC-9.5.6.I1 Wind Coherence Across Systems

| # | Requirement |
|---|-------------|
| 1 | R-9.5.6     |
| 2 | R-9.5.6     |

1. **#1** — Hair, cloth, foliage, particles at same position, wind=(10,0,0) m/s
   - **Expected:** All four systems deflect in +X direction
2. **#2** — Wind direction changed from +X to -Z mid-frame
   - **Expected:** All systems respond to new direction within 2 frames

### TC-9.5.1.I2 Platform Cloth Disabled

| # | Requirement |
|---|-------------|
| 1 | R-9.5.1     |
| 2 | R-9.5.1     |

1. **#1** — Mobile platform config, ClothGarment spawned
   - **Expected:** ClothGarment.enabled=false, baked animation active
2. **#2** — Desktop platform config, ClothGarment spawned
   - **Expected:** ClothGarment.enabled=true, PBD simulation active

### TC-9.5.2.I2 Platform Strand Gating

| # | Requirement |
|---|-------------|
| 1 | R-9.5.2     |
| 2 | R-9.5.2     |

1. **#1** — Switch platform config, hair entity spawned
   - **Expected:** HairLodState.current_tier = CardBased (never FullStrands)
2. **#2** — Desktop platform config, hero character at 5 m
   - **Expected:** HairLodState.current_tier = FullStrands

### TC-9.5.3.I1 Card Count Budget

| # | Requirement |
|---|-------------|
| 1 | R-9.5.3     |
| 2 | R-9.5.3     |
| 3 | R-9.5.3     |

1. **#1** — Mobile platform config
   - **Expected:** card_count in range 8-16
2. **#2** — Switch platform config
   - **Expected:** card_count in range 16-32
3. **#3** — Desktop platform config
   - **Expected:** card_count in range 32-64

### TC-9.5.5.I1 Proxy Count Budget

| # | Requirement |
|---|-------------|
| 1 | R-9.5.5     |
| 2 | R-9.5.5     |
| 3 | R-9.5.5     |

1. **#1** — Mobile platform config
   - **Expected:** proxy count = 0
2. **#2** — Switch platform config
   - **Expected:** proxy count in range 4-6
3. **#3** — Desktop platform config
   - **Expected:** proxy count in range 8-12

## Benchmarks

### TC-9.5.1.B1 Cloth PBD GPU Time

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1000-vertex panel, desktop GPU | GPU time | < 0.5 ms | R-9.5.1 |
| 2 | 16 simultaneous panels, desktop GPU | Total GPU time | < 1.0 ms | US-9.5.1.1 |

### TC-9.5.2.B1 Guide Strand Simulation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 256 guide strands, desktop GPU | GPU time | < 1.0 ms | US-9.5.2.2 |
| 2 | 4096 render strands interpolation | GPU time | < 0.3 ms | R-9.5.2 |
| 3 | Hair OIT compositing pass | GPU time | < 0.5 ms | R-9.5.2 |

### TC-9.5.3.B1 Card vs Strand Cost

| # |
|---|
| 1 |

1. **1** — Card hair vs strand hair, same character
   - **Metric:** Relative GPU cost
   - **Target:** Card at least 5x faster
   - **Requirement:** R-9.5.3

### TC-9.5.4.B1 LOD Transition Frame Spike

| # |
|---|
| 1 |

1. **1** — LOD tier transition during flythrough
   - **Metric:** Frame time delta
   - **Target:** Zero spike (< 0.1 ms increase)
   - **Requirement:** R-9.5.4

### TC-9.5.6.B1 Wind Field Sampling

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Wind field texture sample per entity | Per-entity cost | < 0.01 ms | R-9.5.6 |
