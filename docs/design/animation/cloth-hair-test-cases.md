# Cloth & Hair Animation Test Cases

Companion test cases for [cloth-hair.md](cloth-hair.md).

## Unit Tests

### TC-9.5.1.1 Distance Constraint Rest Length

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two particles 2.0 units apart, rest_length=1.0, 10 solver iterations | Particle distance within 1% of 1.0 (0.99-1.01 units) | R-9.5.1 |
| 2 | Two particles at rest_length=1.0, compliance=0.0 (rigid), 5 iterations | Particle distance exactly 1.0 units | R-9.5.1 |
| 3 | Two particles 0.5 units apart, rest_length=1.0, 10 iterations | Particle distance within 1% of 1.0 | R-9.5.1 |

### TC-9.5.1.2 Bending Constraint Rest Angle

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Four particles forming 90-degree dihedral, rest_angle=90 deg, 10 iterations | Dihedral angle within 2 degrees of 90 | R-9.5.1 |
| 2 | Four particles at 45-degree dihedral, rest_angle=90 deg, 10 iterations | Dihedral angle within 2 degrees of 90 | R-9.5.1 |

### TC-9.5.1.3 Cloth Self-Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100-vertex cloth plane folded in half, collision_radius=0.01 | All pairwise distances > 0.01 (no penetration) | R-9.5.1 |
| 2 | 100-vertex cloth draped over sphere, self-collision enabled | Zero particle-particle penetrations after 60 frames | R-9.5.1 |

### TC-9.5.1.4 Cloth Wind Response

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hanging cloth panel, wind=(10,0,0) m/s, 30 frames | All particle X positions > initial X positions | R-9.5.1 |
| 2 | Hanging cloth panel, wind=(0,0,0) m/s, 30 frames | Particle X positions unchanged from gravity-only baseline | R-9.5.1 |

### TC-9.5.2.1 Strand Gravity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 guide strands, gravity=(0,-9.81,0), 60 frames | All strand tip Y < root Y | R-9.5.2 |
| 2 | 100 guide strands, gravity=(0,0,0), 60 frames | Strand tip Y unchanged from initial | R-9.5.2 |

### TC-9.5.2.2 Strand Stretch Constraint

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10-segment strand, rest_length=1.0 per segment, gravity applied, 60 frames | Total strand length within 1% of 10.0 | R-9.5.2 |
| 2 | 10-segment strand, stretch_compliance=0.0 (rigid) | Total strand length exactly 10.0 | R-9.5.2 |

### TC-9.5.2.3 Strand Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Collision capsule radius=0.1 at strand midpoint, 30 frames | All particles distance from capsule axis >= 0.1 | R-9.5.2 |
| 2 | No collision capsule, strand falls through | Strand tip passes below capsule position | R-9.5.2 |

### TC-9.5.3.1 Card Spring Physics

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Card hair group, impulse=(1,0,0), spring_stiffness=50, spring_damping=5, 30 frames | Peak displacement at frame 2-5, amplitude < 10% of peak at frame 30 | R-9.5.3 |
| 2 | Card hair group, no impulse, 30 frames | Zero displacement across all frames | R-9.5.3 |

### TC-9.5.3.2 Card Anisotropic Specular

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Light at 0 deg azimuth, card tangent along Y | Specular highlight centered on Y-axis reflection | R-9.5.3 |
| 2 | Light rotated 90 deg around card | Specular highlight shifts 90 deg along anisotropic direction | R-9.5.3 |

### TC-9.5.4.1 Hair LOD Tier Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera at 5 m, strand_max_distance=15 | HairLodTier::FullStrands | R-9.5.4 |
| 2 | Camera at 20 m, strand_max_distance=15, cluster_max_distance=40 | HairLodTier::SimplifiedClusters | R-9.5.4 |
| 3 | Camera at 50 m, cluster_max_distance=40, card_max_distance=100 | HairLodTier::CardBased | R-9.5.4 |
| 4 | Camera at 200 m, card_max_distance=100 | HairLodTier::Shell | R-9.5.4 |

### TC-9.5.4.2 Hair LOD Hysteresis

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera moves 19 m to 21 m, hysteresis=3.0, strand_max_distance=20 | No tier change (stays FullStrands) | R-9.5.4 |
| 2 | Camera moves 19 m to 24 m, hysteresis=3.0, strand_max_distance=20 | Tier changes to SimplifiedClusters | R-9.5.4 |
| 3 | Camera moves 24 m to 20 m, hysteresis=3.0, strand_max_distance=20 | No tier change (stays SimplifiedClusters) | R-9.5.4 |

### TC-9.5.4.3 Hair LOD Blend

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | LOD transition triggered, blend_duration_sec=0.5, dt=1/60 | blend_alpha reaches 1.0 after 30 frames | R-9.5.4 |
| 2 | LOD transition triggered, blend_duration_sec=1.0, dt=1/60 | blend_alpha=0.5 at frame 30, blend_alpha=1.0 at frame 60 | R-9.5.4 |

### TC-9.5.5.1 Collision Proxy Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skeleton pose changed, capsule on upper_arm bone | Capsule world position matches bone world transform within 1 frame | R-9.5.5 |
| 2 | Skeleton at rest pose | All capsule positions match bone rest transforms | R-9.5.5 |

### TC-9.5.5.2 Collision Friction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cloth sliding on capsule, friction=0.8, initial tangent velocity=1.0 | Tangent velocity < 0.2 after 10 frames | R-9.5.5 |
| 2 | Cloth sliding on capsule, friction=0.0 | Tangent velocity unchanged after contact | R-9.5.5 |

### TC-9.5.6.1 Wind Field Sampling Coherence

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hair and foliage at position (10,0,10), shared wind field | Both sample identical wind vector (bitwise equal) | R-9.5.6 |
| 2 | Hair at (10,0,10), foliage at (20,0,20), wind field with spatial variation | Each samples its own position's wind vector correctly | R-9.5.6 |

### TC-9.5.6.2 Wind Drag Proportional

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Wind speed=5 m/s, drag_coefficient=1.0, 30 frames | Strand tip displacement D1 | R-9.5.6 |
| 2 | Wind speed=10 m/s, drag_coefficient=1.0, 30 frames | Strand tip displacement within 20% of 2*D1 | R-9.5.6 |

## Integration Tests

### TC-9.5.1.I1 Cloth on Animated Character

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-vertex cloak on walking character, 300 frames | Zero cloth-body penetrations across all frames | R-9.5.1, R-9.5.5 |
| 2 | 1000-vertex cloak on running character, 300 frames | Cloth follows character, max distance from body < 2.0 units | R-9.5.1, R-9.5.5 |

### TC-9.5.2.I1 Strand Hair Head Turn

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Character head rotates 90 degrees over 30 frames | Hair swings with peak delay at frame 10-20, no capsule penetrations | R-9.5.2 |
| 2 | Character head rotates 90 degrees, collision capsules on neck and shoulders | Zero strand particles inside any capsule radius | R-9.5.2 |

### TC-9.5.4.I1 LOD Flythrough

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Camera flies from 5 m to 200 m over 300 frames, per-frame screenshots | Zero visible popping artifacts between LOD tiers | R-9.5.4 |
| 2 | Camera oscillates at LOD boundary for 120 frames | No rapid tier oscillation (max 2 transitions total) | R-9.5.4 |

### TC-9.5.6.I1 Wind Coherence Across Systems

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Hair, cloth, foliage, particles at same position, wind=(10,0,0) m/s | All four systems deflect in +X direction | R-9.5.6 |
| 2 | Wind direction changed from +X to -Z mid-frame | All systems respond to new direction within 2 frames | R-9.5.6 |

### TC-9.5.1.I2 Platform Cloth Disabled

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile platform config, ClothGarment spawned | ClothGarment.enabled=false, baked animation active | R-9.5.1 |
| 2 | Desktop platform config, ClothGarment spawned | ClothGarment.enabled=true, PBD simulation active | R-9.5.1 |

### TC-9.5.2.I2 Platform Strand Gating

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Switch platform config, hair entity spawned | HairLodState.current_tier = CardBased (never FullStrands) | R-9.5.2 |
| 2 | Desktop platform config, hero character at 5 m | HairLodState.current_tier = FullStrands | R-9.5.2 |

### TC-9.5.3.I1 Card Count Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile platform config | card_count in range 8-16 | R-9.5.3 |
| 2 | Switch platform config | card_count in range 16-32 | R-9.5.3 |
| 3 | Desktop platform config | card_count in range 32-64 | R-9.5.3 |

### TC-9.5.5.I1 Proxy Count Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile platform config | proxy count = 0 | R-9.5.5 |
| 2 | Switch platform config | proxy count in range 4-6 | R-9.5.5 |
| 3 | Desktop platform config | proxy count in range 8-12 | R-9.5.5 |

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

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Card hair vs strand hair, same character | Relative GPU cost | Card at least 5x faster | R-9.5.3 |

### TC-9.5.4.B1 LOD Transition Frame Spike

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | LOD tier transition during flythrough | Frame time delta | Zero spike (< 0.1 ms increase) | R-9.5.4 |

### TC-9.5.6.B1 Wind Field Sampling

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Wind field texture sample per entity | Per-entity cost | < 0.01 ms | R-9.5.6 |
