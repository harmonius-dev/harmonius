# VFX Effect Graph Test Cases

Companion test cases for [effect-graph.md](effect-graph.md).

## Unit Tests

### TC-11.6.1.1 Validate Complete Graph

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with Spawn, Initialize, Update, Output context nodes | `validate()` returns `Ok(ValidatedGraph)` | R-11.6.1 |

### TC-11.6.1.2 Validate Missing Context

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph missing Spawn context node | Returns `Err(MissingContext { stage: Spawn })` | R-11.6.1 |

### TC-11.6.1.3 Validate Type Mismatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Edge connecting Float output pin to Vec3 input pin | Returns `Err(TypeMismatch { expected: Vec3, found: Float })` | R-11.6.1 |

### TC-11.6.1.4 Validate Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Cyclic dataflow: A -> B -> C -> A | Returns `Err(CycleDetected)` with cycle path [A, B, C] | R-11.6.1 |

### TC-11.6.1.5 Validate Mobile Node Limit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile tier graph with 33 nodes (limit = 32) | Returns `Err(NodeCountExceeded { count: 33, limit: 32 })` | R-11.6.1 |

### TC-11.6.2.1 Validate Mobile Custom Node

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Per-particle custom node on Mobile tier | Returns `Err(MobilePerParticleCustomNode)` | R-11.6.2 |

### TC-11.6.1.6 Codegen Gravity Update

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with gravity + position integration nodes | Valid HLSL with correct buffer bindings for positions and velocities | R-11.6.1 |

### TC-11.6.1.7 Codegen Noise Operator

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with Noise node | Generated HLSL contains correct noise function call | R-11.6.1 |

### TC-11.6.1.8 Codegen Sample Curve

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with SampleCurve node | HLSL emits texture sample with correct UV mapping | R-11.6.1 |

### TC-11.6.1.9 Codegen Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with Branch node | HLSL emits conditional with both true and false paths | R-11.6.1 |

### TC-11.6.1.10 Topological Sort

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with A -> B -> C dependency chain | Sorted order: [A, B, C] | R-11.6.1 |

### TC-11.6.1.11 Dead Node Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with disconnected node D (not reachable from output) | D removed during flatten pass | R-11.6.1 |

### TC-11.6.3.1 Param Default Value

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Unconnected parameter pin with default=2.0 | Codegen uses 2.0 as constant in HLSL | R-11.6.3 |

### TC-11.6.3.2 Param Override

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | ParamOverride { name: "intensity", value: 0.5 } | Correct cbuffer offset written with 0.5 | R-11.6.3 |

### TC-11.6.5.1 LOD Tier Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter at distance < reduced_distance | LodTier::Full | R-11.6.5 |
| 2 | Emitter at distance between reduced and impostor | LodTier::Reduced | R-11.6.5 |
| 3 | Emitter at distance > cull_distance | LodTier::Culled | R-11.6.5 |

### TC-11.6.5.2 LOD Hysteresis

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Emitter oscillating near reduced_distance threshold | No flicker between Full and Reduced tiers | R-11.6.5 |

### TC-11.6.5.3 Budget Priority Ordering

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Over budget with Low, Medium, High emitters | Low scaled down first, then Medium | R-11.6.5 |

### TC-11.6.5.4 Budget Critical Immune

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Critical-priority effect under budget pressure | Never scaled or culled | R-11.6.5 |

### TC-11.1.1.1 Spawn Shape Coverage

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SpawnShape::Sphere { radius: 5.0 } | All particles within 5.0 units of origin | R-11.1.1 |
| 2 | SpawnShape::Box { half_extents: (2, 3, 4) } | All particles within box bounds | R-11.1.1 |

## Integration Tests

### TC-11.6.1.I1 Compile and Dispatch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile graph end-to-end, dispatch compute kernels | Particle buffer contains expected state after simulation | R-11.6.1 |

### TC-11.6.4.I1 Event Spawn Collision

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Physics collision triggers observer | VfxSpawnEvent creates effect at contact point with correct normal | R-11.6.4 |

### TC-11.6.4.I2 Event Spawn Anim Notify

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Animation notify triggers observer | VFX spawns at bone position with correct velocity | R-11.6.4 |

### TC-11.6.4.I3 Event Spawn Attach

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Spawn effect with attach_to entity | Effect follows parent entity transform | R-11.6.4 |

### TC-11.6.3.I1 Param Data Binding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bind parameter to ECS component, change value | Effect updates within one frame | R-11.6.3 |

### TC-11.6.2.I1 Custom Node Per Emitter

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Custom node via logic graph compiled into effect | Output matches expected values | R-11.6.2 |

### TC-11.6.1.I2 Preview Scrub

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open preview, scrub to t=2.0 | Particle state matches t=2.0 simulation | R-11.6.1 |

### TC-11.6.1.I3 Preview Stats

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Open preview, check stats | Per-emitter alive count and GPU time are non-zero | R-11.6.1 |

### TC-11.6.1.I4 Shader Cache Hit

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile same graph twice | Second compile reads from cache, no DXC invocation | R-11.6.1 |

### TC-11.1.3.I1 Output Sprite Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile sprite output graph | Draw-indirect args produce correct billboard draws | R-11.1.3 |

### TC-11.1.3.I2 Output Mesh Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile mesh output graph | GPU instancing with per-particle transforms | R-11.1.3 |

### TC-11.1.3.I3 Output Ribbon Render

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compile ribbon output graph | Spline geometry connects sequential particles | R-11.1.3 |

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
