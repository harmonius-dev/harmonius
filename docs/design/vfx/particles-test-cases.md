# GPU Particle System Test Cases

Companion test cases for [particles.md](particles.md).

## Unit Tests

### TC-11.1.1.1 Emit Point Shape

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Point emitter, spawn 100 particles
   - **Expected:** All 100 positions at emitter origin

### TC-11.1.1.2 Emit Sphere Surface

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Sphere surface emitter, radius=5.0, spawn 1000
   - **Expected:** All distances from origin within [4.9, 5.1]

### TC-11.1.1.3 Emit Box Volume

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Box emitter, half_extents=(2,3,4), spawn 1000
   - **Expected:** All positions within [-2,2] x [-3,3] x [-4,4]

### TC-11.1.1.4 Emit Cone

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Cone emitter, angle=30deg, radius=1.0, spawn 500
   - **Expected:** All directions within 30-degree cone

### TC-11.1.1.5 Emit Mesh Surface

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Mesh surface emitter with cube mesh
   - **Expected:** Particles spawn on cube faces

### TC-11.1.1.6 Emit Skinned Mesh

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Skinned mesh emitter at animated pose
   - **Expected:** Particles at animated vertex positions

### TC-11.1.1.7 Free List Allocation

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Emit 100, kill 50, emit 50 more
   - **Expected:** Free-list recycles 50 indices

### TC-11.1.1.8 Indirect Dispatch Args

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — 500 alive particles after simulation
   - **Expected:** Dispatch args thread count = ceil(500/256)

### TC-11.1.2.1 Gravity Module

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Particle velocity=(0,0,0), gravity=(0,-9.81,0), dt=1/60
   - **Expected:** velocity.y = -0.1635

### TC-11.1.2.2 Curl Noise Module

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Particle in curl noise field
   - **Expected:** Velocity displaced, divergence-free motion

### TC-11.1.2.3 Drag Module

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Particle velocity=10.0, drag=0.5, dt=1.0
   - **Expected:** Velocity = 10 * (1 - 0.5 * 1.0) = 5.0

### TC-11.1.2.4 Color Over Life

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — age=0.5, lifetime=1.0, gradient red-to-blue
   - **Expected:** Color = gradient sample at t=0.5

### TC-11.1.2.5 Size Over Life

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — age=0.0, size curve start=1.0 end=0.0
   - **Expected:** Size = 1.0 at birth, 0.0 at death

### TC-11.1.2.6 Depth Buffer Collision

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Particle hits depth buffer surface, restitution=0.5
   - **Expected:** Velocity reflected, magnitude * 0.5

### TC-11.1.2.7 SDF Collision

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |
| 2 | R-11.1.2    |

1. **#1** — Particle hits SDF surface on desktop
   - **Expected:** Bounces off SDF
2. **#2** — SDF collision on mobile config
   - **Expected:** Disabled, particle passes through

### TC-11.1.2.8 Fused Module Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Emitter with gravity + drag + color modules
   - **Expected:** All execute in single compute dispatch

### TC-11.1.3.1 Sprite Billboard

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Sprite particle viewed from 4 camera angles
   - **Expected:** Quad faces camera at each angle

### TC-11.1.3.2 Flipbook Animation

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — 8-frame flipbook at 10 FPS, elapsed=0.35s
   - **Expected:** Frame index = 3

### TC-11.1.3.3 Soft Depth Fade

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Particle intersecting depth buffer
   - **Expected:** Alpha fades near intersection

### TC-11.1.3.4 Ribbon Connectivity

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — 10 sequential ribbon particles
   - **Expected:** Segments connect without gaps

### TC-11.1.3.5 Ribbon Catmull Rom

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — 4 ribbon control points
   - **Expected:** Subdivided geometry follows Catmull-Rom spline

### TC-11.1.3.6 Mesh Particle Instancing

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — 100 mesh particles with unique transforms
   - **Expected:** Each rendered with correct per-instance transform

### TC-11.1.4.1 LOD Tier Transitions

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — Move emitter through 4 distance thresholds
   - **Expected:** Transitions Full -> Reduced -> Impostor -> Culled

### TC-11.1.4.2 LOD Hysteresis

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — Emitter oscillating at LOD boundary
   - **Expected:** No tier oscillation

### TC-11.1.4.3 Budget Cap Enforced

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — Spawn emitters exceeding platform budget
   - **Expected:** Total alive never exceeds global_max

### TC-11.1.4.4 Budget Priority Cull

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — Budget exceeded with Low and High emitters
   - **Expected:** Low-priority culled first

### TC-11.1.4.5 Radix Sort Correctness

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — 1000 particles at random distances
   - **Expected:** Sorted alive list in back-to-front order

### TC-11.1.5.1 Sub Emitter Death

| # | Requirement |
|---|-------------|
| 1 | R-11.1.5    |

1. **#1** — Parent dies with Death sub-emitter trigger
   - **Expected:** Children spawn at parent death position

### TC-11.1.5.2 Sub Emitter Collision

| # | Requirement |
|---|-------------|
| 1 | R-11.1.5    |

1. **#1** — Parent collides with Collision sub-emitter trigger
   - **Expected:** Children spawn at collision position

### TC-11.1.5.3 Sub Emitter Inherit Velocity

| # | Requirement |
|---|-------------|
| 1 | R-11.1.5    |

1. **#1** — Parent velocity=(10,0,0), inherit_velocity=0.5
   - **Expected:** Child initial velocity includes (5,0,0)

### TC-11.1.5.4 Sub Emitter Depth Limit

| # | Requirement |
|---|-------------|
| 1 | R-11.1.5    |
| 2 | R-11.1.5    |

1. **#1** — Sub-emitter chain on mobile
   - **Expected:** Max depth = 1
2. **#2** — Sub-emitter chain on desktop
   - **Expected:** Max depth = 4

### TC-11.1.6.1 Particle Light Injection

| # | Requirement |
|---|-------------|
| 1 | R-11.1.6    |

1. **#1** — Emitter with ParticleLightConfig, 10 alive
   - **Expected:** Lights appear in clustered light buffer

### TC-11.1.6.2 Particle Light Cap

| # | Requirement |
|---|-------------|
| 1 | R-11.1.6    |

1. **#1** — Emitter max_lights=8, 100 alive particles
   - **Expected:** Light count <= 8

### TC-11.1.1.9 Warm Up Particle Count

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Warm-up duration=2.0, spawn_rate=100/s
   - **Expected:** Alive count approximately 200

## Integration Tests

### TC-11.1.1.I1 Async Compute Overlap

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Particle simulation on async compute queue
   - **Expected:** Overlaps with previous frame's graphics work

### TC-11.1.1.I2 Graphics Queue Fallback

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Disable async compute
   - **Expected:** Simulation runs correctly on graphics queue

### TC-11.1.4.I1 40 Player Raid Budget

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — 40 emitters with full modules
   - **Expected:** Budget caps particles, frame rate stable

### TC-11.1.2.I1 Effect Graph Compile

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Effect graph with all module types
   - **Expected:** Compiles to valid fused shader

### TC-11.1.4.I2 Platform Tier Configs

| # | Requirement |
|---|-------------|
| 1 | R-11.1.4    |

1. **#1** — Check each PlatformTier
   - **Expected:** Budget, LOD distances, module availability match spec

### TC-11.1.5.I1 Cascading Sub Emitters

| # | Requirement |
|---|-------------|
| 1 | R-11.1.5    |

1. **#1** — Firework: burst -> death sub-emit -> sparks
   - **Expected:** Full chain executes without error

### TC-11.1.3.I1 Ribbon Moving Emitter

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Ribbon trail on moving entity
   - **Expected:** Trail follows without discontinuities

### TC-11.1.1.I3 Warm Up Visual Parity

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |

1. **#1** — Compare warm-up result with real-time simulation at t=2.0
   - **Expected:** Visual parity within tolerance

### TC-11.1.2.I2 Depth Collision All Angles

| # | Requirement |
|---|-------------|
| 1 | R-11.1.2    |

1. **#1** — Depth-buffer collision from 8 camera angles
   - **Expected:** No tunneling at any angle

### TC-11.1.6.I1 Particle Lights Raid

| # | Requirement |
|---|-------------|
| 1 | R-11.1.6    |

1. **#1** — 40 emitters with lights
   - **Expected:** Per-tile cap enforced, lighting cost bounded

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
