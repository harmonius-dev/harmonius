# GPU Particle System Test Cases

Companion test cases for [particles.md](particles.md).

## Unit Tests

### TC-11.1.1.1 Emit Point Shape

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Point emitter, spawn 100 particles | All 100 positions at emitter origin | R-11.1.1 |

### TC-11.1.1.2 Emit Sphere Surface

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sphere surface emitter, radius=5.0, spawn 1000 | All distances from origin within [4.9, 5.1] | R-11.1.1 |

### TC-11.1.1.3 Emit Box Volume

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Box emitter, half_extents=(2,3,4), spawn 1000 | All positions within [-2,2] x [-3,3] x [-4,4] | R-11.1.1 |

### TC-11.1.1.4 Emit Cone

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cone emitter, angle=30deg, radius=1.0, spawn 500 | All directions within 30-degree cone | R-11.1.1 |

### TC-11.1.1.5 Emit Mesh Surface

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mesh surface emitter with cube mesh | Particles spawn on cube faces | R-11.1.1 |

### TC-11.1.1.6 Emit Skinned Mesh

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Skinned mesh emitter at animated pose | Particles at animated vertex positions | R-11.1.1 |

### TC-11.1.1.7 Free List Allocation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emit 100, kill 50, emit 50 more | Free-list recycles 50 indices | R-11.1.1 |

### TC-11.1.1.8 Indirect Dispatch Args

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 500 alive particles after simulation | Dispatch args thread count = ceil(500/256) | R-11.1.1 |

### TC-11.1.2.1 Gravity Module

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle velocity=(0,0,0), gravity=(0,-9.81,0), dt=1/60 | velocity.y = -0.1635 | R-11.1.2 |

### TC-11.1.2.2 Curl Noise Module

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle in curl noise field | Velocity displaced, divergence-free motion | R-11.1.2 |

### TC-11.1.2.3 Drag Module

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle velocity=10.0, drag=0.5, dt=1.0 | Velocity = 10 * (1 - 0.5 * 1.0) = 5.0 | R-11.1.2 |

### TC-11.1.2.4 Color Over Life

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | age=0.5, lifetime=1.0, gradient red-to-blue | Color = gradient sample at t=0.5 | R-11.1.2 |

### TC-11.1.2.5 Size Over Life

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | age=0.0, size curve start=1.0 end=0.0 | Size = 1.0 at birth, 0.0 at death | R-11.1.2 |

### TC-11.1.2.6 Depth Buffer Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle hits depth buffer surface, restitution=0.5 | Velocity reflected, magnitude * 0.5 | R-11.1.2 |

### TC-11.1.2.7 SDF Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle hits SDF surface on desktop | Bounces off SDF | R-11.1.2 |
| 2 | SDF collision on mobile config | Disabled, particle passes through | R-11.1.2 |

### TC-11.1.2.8 Fused Module Dispatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter with gravity + drag + color modules | All execute in single compute dispatch | R-11.1.2 |

### TC-11.1.3.1 Sprite Billboard

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sprite particle viewed from 4 camera angles | Quad faces camera at each angle | R-11.1.3 |

### TC-11.1.3.2 Flipbook Animation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 8-frame flipbook at 10 FPS, elapsed=0.35s | Frame index = 3 | R-11.1.3 |

### TC-11.1.3.3 Soft Depth Fade

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle intersecting depth buffer | Alpha fades near intersection | R-11.1.3 |

### TC-11.1.3.4 Ribbon Connectivity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10 sequential ribbon particles | Segments connect without gaps | R-11.1.3 |

### TC-11.1.3.5 Ribbon Catmull Rom

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 4 ribbon control points | Subdivided geometry follows Catmull-Rom spline | R-11.1.3 |

### TC-11.1.3.6 Mesh Particle Instancing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 100 mesh particles with unique transforms | Each rendered with correct per-instance transform | R-11.1.3 |

### TC-11.1.4.1 LOD Tier Transitions

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move emitter through 4 distance thresholds | Transitions Full -> Reduced -> Impostor -> Culled | R-11.1.4 |

### TC-11.1.4.2 LOD Hysteresis

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter oscillating at LOD boundary | No tier oscillation | R-11.1.4 |

### TC-11.1.4.3 Budget Cap Enforced

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn emitters exceeding platform budget | Total alive never exceeds global_max | R-11.1.4 |

### TC-11.1.4.4 Budget Priority Cull

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Budget exceeded with Low and High emitters | Low-priority culled first | R-11.1.4 |

### TC-11.1.4.5 Radix Sort Correctness

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 particles at random distances | Sorted alive list in back-to-front order | R-11.1.4 |

### TC-11.1.5.1 Sub Emitter Death

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent dies with Death sub-emitter trigger | Children spawn at parent death position | R-11.1.5 |

### TC-11.1.5.2 Sub Emitter Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent collides with Collision sub-emitter trigger | Children spawn at collision position | R-11.1.5 |

### TC-11.1.5.3 Sub Emitter Inherit Velocity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Parent velocity=(10,0,0), inherit_velocity=0.5 | Child initial velocity includes (5,0,0) | R-11.1.5 |

### TC-11.1.5.4 Sub Emitter Depth Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sub-emitter chain on mobile | Max depth = 1 | R-11.1.5 |
| 2 | Sub-emitter chain on desktop | Max depth = 4 | R-11.1.5 |

### TC-11.1.6.1 Particle Light Injection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter with ParticleLightConfig, 10 alive | Lights appear in clustered light buffer | R-11.1.6 |

### TC-11.1.6.2 Particle Light Cap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter max_lights=8, 100 alive particles | Light count <= 8 | R-11.1.6 |

### TC-11.1.1.9 Warm Up Particle Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Warm-up duration=2.0, spawn_rate=100/s | Alive count approximately 200 | R-11.1.1 |

## Integration Tests

### TC-11.1.1.I1 Async Compute Overlap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Particle simulation on async compute queue | Overlaps with previous frame's graphics work | R-11.1.1 |

### TC-11.1.1.I2 Graphics Queue Fallback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Disable async compute | Simulation runs correctly on graphics queue | R-11.1.1 |

### TC-11.1.4.I1 40 Player Raid Budget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 40 emitters with full modules | Budget caps particles, frame rate stable | R-11.1.4 |

### TC-11.1.2.I1 Effect Graph Compile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Effect graph with all module types | Compiles to valid fused shader | R-11.1.2 |

### TC-11.1.4.I2 Platform Tier Configs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Check each PlatformTier | Budget, LOD distances, module availability match spec | R-11.1.4 |

### TC-11.1.5.I1 Cascading Sub Emitters

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Firework: burst -> death sub-emit -> sparks | Full chain executes without error | R-11.1.5 |

### TC-11.1.3.I1 Ribbon Moving Emitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Ribbon trail on moving entity | Trail follows without discontinuities | R-11.1.3 |

### TC-11.1.1.I3 Warm Up Visual Parity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compare warm-up result with real-time simulation at t=2.0 | Visual parity within tolerance | R-11.1.1 |

### TC-11.1.2.I2 Depth Collision All Angles

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Depth-buffer collision from 8 camera angles | No tunneling at any angle | R-11.1.2 |

### TC-11.1.6.I1 Particle Lights Raid

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 40 emitters with lights | Per-tile cap enforced, lighting cost bounded | R-11.1.6 |

## Benchmarks

### TC-11.1.1.B1 Simulate 1M Particles

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Simulate 1M particles (fused modules) | GPU time | < 2 ms | R-11.1.1 |

### TC-11.1.1.B2 Emit 100K Particles Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Emit 100K particles in one frame | GPU time | < 0.5 ms | R-11.1.1 |

### TC-11.1.4.B1 Radix Sort 500K Particles

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | GPU radix sort 500K particles | GPU time | < 1 ms | R-11.1.4 |

### TC-11.1.1.B3 CPU Overhead Per Emitter

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per-emitter CPU setup overhead | CPU time | < 5 us | R-11.1.1 |

### TC-11.1.1.B4 Warm Up 1 Second

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Warm-up 1 second at 60 fps step rate | GPU time | < 50 ms | R-11.1.1 |

### TC-11.1.3.B1 Sprite Draw 200K Particles

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 200K sprite particles | GPU time | < 2 ms | R-11.1.3 |

### TC-11.1.3.B2 Ribbon Draw 10K Segments

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 10K ribbon segments | GPU time | < 0.5 ms | R-11.1.3 |

### TC-11.1.3.B3 Mesh Instanced Draw 50K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Render 50K mesh particles (instanced) | GPU time | < 1 ms | R-11.1.3 |

### TC-11.1.4.B2 Budget Rebalance 100 Emitters

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Rebalance budget across 100 emitters | CPU time | < 100 us | R-11.1.4 |

### TC-11.1.4.B3 LOD Evaluation 100 Emitters

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate LOD for 100 emitters | CPU time | < 50 us | R-11.1.4 |
