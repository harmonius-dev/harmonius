# VFX Effect Graph Test Cases

Companion test cases for [effect-graph.md](effect-graph.md).

## Unit Tests

### TC-11.6.1.1 Validate Complete Graph

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with Spawn, Initialize, Update, Output context nodes
   - **Expected:** `validate()` returns `Ok(ValidatedGraph)`

### TC-11.6.1.2 Validate Missing Context

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph missing Spawn context node
   - **Expected:** Returns `Err(MissingContext { stage: Spawn })`

### TC-11.6.1.3 Validate Type Mismatch

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Edge connecting Float output pin to Vec3 input pin
   - **Expected:** Returns `Err(TypeMismatch { expected: Vec3, found: Float })`

### TC-11.6.1.4 Validate Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Cyclic dataflow: A -> B -> C -> A
   - **Expected:** Returns `Err(CycleDetected)` with cycle path [A, B, C]

### TC-11.6.1.5 Validate Mobile Node Limit

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Mobile tier graph with 33 nodes (limit = 32)
   - **Expected:** Returns `Err(NodeCountExceeded { count: 33, limit: 32 })`

### TC-11.6.2.1 Validate Mobile Custom Node

| # | Requirement |
|---|-------------|
| 1 | R-11.6.2    |

1. **#1** — Per-particle custom node on Mobile tier
   - **Expected:** Returns `Err(MobilePerParticleCustomNode)`

### TC-11.6.1.6 Codegen Gravity Update

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with gravity + position integration nodes
   - **Expected:** Valid HLSL with correct buffer bindings for positions and velocities

### TC-11.6.1.7 Codegen Noise Operator

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with Noise node
   - **Expected:** Generated HLSL contains correct noise function call

### TC-11.6.1.8 Codegen Sample Curve

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with SampleCurve node
   - **Expected:** HLSL emits texture sample with correct UV mapping

### TC-11.6.1.9 Codegen Branch

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with Branch node
   - **Expected:** HLSL emits conditional with both true and false paths

### TC-11.6.1.10 Topological Sort

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with A -> B -> C dependency chain
   - **Expected:** Sorted order: [A, B, C]

### TC-11.6.1.11 Dead Node Elimination

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Graph with disconnected node D (not reachable from output)
   - **Expected:** D removed during flatten pass

### TC-11.6.3.1 Param Default Value

| # | Requirement |
|---|-------------|
| 1 | R-11.6.3    |

1. **#1** — Unconnected parameter pin with default=2.0
   - **Expected:** Codegen uses 2.0 as constant in HLSL

### TC-11.6.3.2 Param Override

| # | Requirement |
|---|-------------|
| 1 | R-11.6.3    |

1. **#1** — ParamOverride { name: "intensity", value: 0.5 }
   - **Expected:** Correct cbuffer offset written with 0.5

### TC-11.6.5.1 LOD Tier Selection

| # | Requirement |
|---|-------------|
| 1 | R-11.6.5    |
| 2 | R-11.6.5    |
| 3 | R-11.6.5    |

1. **#1** — Emitter at distance < reduced_distance
   - **Expected:** LodTier::Full
2. **#2** — Emitter at distance between reduced and impostor
   - **Expected:** LodTier::Reduced
3. **#3** — Emitter at distance > cull_distance
   - **Expected:** LodTier::Culled

### TC-11.6.5.2 LOD Hysteresis

| # | Requirement |
|---|-------------|
| 1 | R-11.6.5    |

1. **#1** — Emitter oscillating near reduced_distance threshold
   - **Expected:** No flicker between Full and Reduced tiers

### TC-11.6.5.3 Budget Priority Ordering

| # | Requirement |
|---|-------------|
| 1 | R-11.6.5    |

1. **#1** — Over budget with Low, Medium, High emitters
   - **Expected:** Low scaled down first, then Medium

### TC-11.6.5.4 Budget Critical Immune

| # | Requirement |
|---|-------------|
| 1 | R-11.6.5    |

1. **#1** — Critical-priority effect under budget pressure
   - **Expected:** Never scaled or culled

### TC-11.1.1.1 Spawn Shape Coverage

| # | Requirement |
|---|-------------|
| 1 | R-11.1.1    |
| 2 | R-11.1.1    |

1. **#1** — SpawnShape::Sphere { radius: 5.0 }
   - **Expected:** All particles within 5.0 units of origin
2. **#2** — SpawnShape::Box { half_extents: (2, 3, 4) }
   - **Expected:** All particles within box bounds

## Integration Tests

### TC-11.6.1.I1 Compile and Dispatch

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Compile graph end-to-end, dispatch compute kernels
   - **Expected:** Particle buffer contains expected state after simulation

### TC-11.6.4.I1 Event Spawn Collision

| # | Requirement |
|---|-------------|
| 1 | R-11.6.4    |

1. **#1** — Physics collision triggers observer
   - **Expected:** VfxSpawnEvent creates effect at contact point with correct normal

### TC-11.6.4.I2 Event Spawn Anim Notify

| # | Requirement |
|---|-------------|
| 1 | R-11.6.4    |

1. **#1** — Animation notify triggers observer
   - **Expected:** VFX spawns at bone position with correct velocity

### TC-11.6.4.I3 Event Spawn Attach

| # | Requirement |
|---|-------------|
| 1 | R-11.6.4    |

1. **#1** — Spawn effect with attach_to entity
   - **Expected:** Effect follows parent entity transform

### TC-11.6.3.I1 Param Data Binding

| # | Requirement |
|---|-------------|
| 1 | R-11.6.3    |

1. **#1** — Bind parameter to ECS component, change value
   - **Expected:** Effect updates within one frame

### TC-11.6.2.I1 Custom Node Per Emitter

| # | Requirement |
|---|-------------|
| 1 | R-11.6.2    |

1. **#1** — Custom node via logic graph compiled into effect
   - **Expected:** Output matches expected values

### TC-11.6.1.I2 Preview Scrub

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Open preview, scrub to t=2.0
   - **Expected:** Particle state matches t=2.0 simulation

### TC-11.6.1.I3 Preview Stats

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Open preview, check stats
   - **Expected:** Per-emitter alive count and GPU time are non-zero

### TC-11.6.1.I4 Shader Cache Hit

| # | Requirement |
|---|-------------|
| 1 | R-11.6.1    |

1. **#1** — Compile same graph twice
   - **Expected:** Second compile reads from cache, no DXC invocation

### TC-11.1.3.I1 Output Sprite Render

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Compile sprite output graph
   - **Expected:** Draw-indirect args produce correct billboard draws

### TC-11.1.3.I2 Output Mesh Render

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Compile mesh output graph
   - **Expected:** GPU instancing with per-particle transforms

### TC-11.1.3.I3 Output Ribbon Render

| # | Requirement |
|---|-------------|
| 1 | R-11.1.3    |

1. **#1** — Compile ribbon output graph
   - **Expected:** Spline geometry connects sequential particles

## Benchmarks

### TC-11.6.1.B1 Graph Validation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate 128-node graph | Time | < 1 ms | R-11.6.1 |

### TC-11.6.1.B2 HLSL Codegen

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Generate HLSL for 128-node graph | Time | < 10 ms | R-11.6.1 |

### TC-11.6.1.B3 Full Compile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Validate + codegen + DXC compile | Time | < 500 ms | R-11.6.1 |

### TC-11.6.1.B4 Shader Cache Lookup

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Look up compiled shader by content hash | Time | < 100 us | R-11.6.1 |

### TC-11.6.4.B1 Spawn System 100 Events

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Process 100 VfxSpawnEvents | Time | < 500 us | R-11.6.4 |

### TC-11.6.5.B1 LOD Update 1000 Emitters

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update LOD for 1000 emitters | Time | < 200 us | R-11.6.5 |

### TC-11.6.5.B2 Budget Enforcement 1000 Emitters

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Enforce budget across 1000 emitters | Time | < 100 us | R-11.6.5 |

### TC-11.1.1.B1 GPU Spawn Kernel

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Spawn 10K particles | GPU time | < 0.1 ms | R-11.1.1 |

### TC-11.1.1.B2 GPU Update Kernel 100K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update 100K particles | GPU time | < 0.5 ms | R-11.1.1 |

### TC-11.1.1.B3 GPU Update Kernel 500K

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Update 500K particles | GPU time | < 2.0 ms | R-11.1.1 |
