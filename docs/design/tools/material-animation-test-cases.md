# Material and Animation Editors Test Cases

Companion test cases for [material-animation.md](material-animation.md).

## Unit Tests

### TC-15.3.1.1 Pin Type Validation

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |
| 2 | R-15.3.1    |
| 3 | R-15.3.1    |

1. **#1** — Connect `Color` output to `Scalar` input
   - **Expected:** `Err(PinTypeError::Incompatible { from: Color, to: Scalar })`
2. **#2** — Connect `Vec3` output to `Vec3` input
   - **Expected:** `Ok(())`
3. **#3** — Connect to already-connected input pin
   - **Expected:** `Err(PinTypeError::AlreadyConnected { pin })`

### TC-15.3.1.2 Graph Cycle Detection

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |
| 2 | R-15.3.1    |

1. **#1** — Create cycle: A -> B -> C -> A
   - **Expected:** `compile()` returns `Err(CycleDetected { cycle: [A, B, C] })`
2. **#2** — Acyclic graph A -> B -> C -> Output
   - **Expected:** `compile()` returns `Ok(HlslSource)`

### TC-15.3.1.3 Graph Compile Simple

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |

1. **#1** — SampleTexture2D -> SurfaceOutput
   - **Expected:** `Ok(HlslSource)` with non-empty `pixel_shader` containing `Sample`

### TC-15.3.1.4 Dead Code Elimination

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |

1. **#1** — Graph with 2 unused Multiply nodes not connected to output
   - **Expected:** compiled HLSL does not contain those multiply operations

### TC-15.3.2.1 Function Inline

| # | Requirement |
|---|-------------|
| 1 | R-15.3.2    |

1. **#1** — Inline triplanar mapping function into parent graph
   - **Expected:** output HLSL matches manually constructed equivalent graph

### TC-15.3.2.2 Function Propagation

| # | Requirement |
|---|-------------|
| 1 | R-15.3.2    |

1. **#1** — Update function signature (add pin), compile referencing materials
   - **Expected:** all referencing materials recompile without stale reference errors

### TC-15.3.3.1 Preview Update Timing

| # | Requirement |
|---|-------------|
| 1 | R-15.3.3    |

1. **#1** — Change scalar parameter via tweaker, measure preview update
   - **Expected:** preview updates within 16 ms (one frame at 60 FPS)

### TC-15.3.4.1 Parameter No Recompile

| # | Requirement |
|---|-------------|
| 1 | R-15.3.4    |

1. **#1** — Change `ScalarParameter` value from `0.5` to `0.8`
   - **Expected:** uniform buffer updated, no DXC invocation triggered

### TC-15.3.5.1 Instance Shader Sharing

| # | Requirement |
|---|-------------|
| 1 | R-15.3.5    |

1. **#1** — Create 100 `MaterialInstance` from same parent
   - **Expected:** all 100 return identical `ShaderHandle` from `get_shader_handle()`

### TC-15.3.5.2 Instance Override Isolation

| # | Requirement |
|---|-------------|
| 1 | R-15.3.5    |
| 2 | R-15.3.5    |

1. **#1** — Override color on instance, check parent
   - **Expected:** parent's parameter unchanged, `instance.has_override("color") == true`
2. **#2** — `remove_override("color")`, check effective value
   - **Expected:** effective value reverts to parent default

### TC-15.3.6.1 Library Search by Tag

| # | Requirement |
|---|-------------|
| 1 | R-15.3.6    |
| 2 | R-15.3.6    |

1. **#1** — Add tag `"metal"` to 3 materials, search with `MaterialFilter { tags: ["metal"] }`
   - **Expected:** exactly 3 `MaterialEntry` results
2. **#2** — Search with tag `"nonexistent"`
   - **Expected:** 0 results

### TC-15.3.6.2 Library Usage Tracking

| # | Requirement |
|---|-------------|
| 1 | R-15.3.6    |

1. **#1** — Assign material M to mesh asset A
   - **Expected:** `get_usage(M)` includes asset A in results

### TC-15.4.1.1 Timeline Keyframe Snap

| # | Requirement |
|---|-------------|
| 1 | R-15.4.1    |
| 2 | R-15.4.1    |

1. **#1** — Move keyframe to time `0.517` at 30 FPS sample rate
   - **Expected:** snaps to `0.5` (frame 15)
2. **#2** — Move keyframe to time `0.034` at 30 FPS
   - **Expected:** snaps to `0.033` (frame 1)

### TC-15.4.1.2 Timeline Scrub Performance

| # | Requirement |
|---|-------------|
| 1 | R-15.4.1    |

1. **#1** — Load 300-bone clip, scrub timeline, measure FPS
   - **Expected:** FPS > 30 during scrub

### TC-15.4.2.1 Bezier Curve Continuity

| # | Requirement |
|---|-------------|
| 1 | R-15.4.2    |

1. **#1** — Create 3 keyframes with `Bezier` tangent mode
   - **Expected:** `evaluate()` produces C1 continuous values at joins (derivative matches)

### TC-15.4.2.2 Curve Preset Accuracy

| # | Requirement |
|---|-------------|
| 1 | R-15.4.2    |
| 2 | R-15.4.2    |

1. **#1** — Apply `EaseIn` preset, evaluate at t=0.5
   - **Expected:** value matches reference ease-in curve within 0.01 tolerance
2. **#2** — Apply `Linear` preset, evaluate at t=0.5
   - **Expected:** value == 0.5 (midpoint)

### TC-15.4.3.1 Skeleton Child Chain Selection

| # | Requirement |
|---|-------------|
| 1 | R-15.4.3    |

1. **#1** — Select `UpperArm` bone in hierarchy with 3 children
   - **Expected:** `selected_chain()` returns `[UpperArm, LowerArm, Hand, Fingers]`

### TC-15.4.4.1 Preview Root Motion Trajectory

| # | Requirement |
|---|-------------|
| 1 | R-15.4.4    |

1. **#1** — Play root motion walk clip for 2 seconds at 1 m/s
   - **Expected:** trajectory distance within 0.1m of 2.0m

### TC-15.4.5.1 Blend Space Center Weights

| # | Requirement |
|---|-------------|
| 1 | R-15.4.5    |

1. **#1** — 2D blend space with 4 corner samples, crosshair at `(0.5, 0.5)`
   - **Expected:** all 4 weights approximately equal (0.25 each, within 0.05)

### TC-15.4.5.2 Blend Space 1D

| # | Requirement |
|---|-------------|
| 1 | R-15.4.5    |

1. **#1** — 1D blend with samples at 0.0 and 1.0, crosshair at `(0.5, 0.0)`
   - **Expected:** weights `[(0, 0.5), (1, 0.5)]`

### TC-15.4.6.1 State Machine Reachability

| # | Requirement |
|---|-------------|
| 1 | R-15.4.6    |
| 2 | R-15.4.6    |

1. **#1** — State machine with unreachable state S3 (no transitions to it)
   - **Expected:** `validate()` returns `UnreachableState { state: S3 }`
2. **#2** — State machine with no default state
   - **Expected:** `validate()` returns `NoDefaultState`

### TC-15.4.6.2 State Machine Transition

| # | Requirement |
|---|-------------|
| 1 | R-15.4.6    |
| 2 | R-15.4.6    |

1. **#1** — Transition condition: `speed > 0.5`, set `speed = 0.8`
   - **Expected:** `active_state()` changes from Idle to Walk
2. **#2** — Same condition, set `speed = 0.3`
   - **Expected:** `active_state()` remains Idle

### TC-15.4.7.1 Retarget Bone Mapping

| # | Requirement |
|---|-------------|
| 1 | R-15.4.7    |

1. **#1** — Map source `Hips` to target `Hips`, apply T-pose
   - **Expected:** target pose matches source pose orientation within 1 degree

### TC-15.4.7.2 Retarget Foot Contact

| # | Requirement |
|---|-------------|
| 1 | R-15.4.7    |

1. **#1** — Retarget walk cycle, measure foot contact height
   - **Expected:** foot contact height within 0.02m of ground plane

## Integration Tests

### TC-15.3.1.I1 Material Compile D3D12

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |

1. **#1** — Full pipeline: graph -> HLSL -> DXC -> DXIL
   - **Expected:** valid DXIL bytecode, renders correct output on D3D12

### TC-15.3.1.I2 Material Compile Vulkan

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |

1. **#1** — Full pipeline: graph -> HLSL -> DXC -> SPIR-V
   - **Expected:** valid SPIR-V bytecode, renders correct output on Vulkan

### TC-15.3.1.I3 Material Compile Metal

| # | Requirement |
|---|-------------|
| 1 | R-15.3.1    |

1. **#1** — Full pipeline: graph -> HLSL -> DXC -> DXIL -> MSC -> MSL
   - **Expected:** valid MSL source, renders correct output on Metal

### TC-15.3.3.I1 Material Hot Reload

| # | Requirement |
|---|-------------|
| 1 | R-15.3.3    |

1. **#1** — Change graph topology in editor, observe viewport
   - **Expected:** viewport updates within 2 frames, no restart needed

### TC-15.4.6.I1 State Machine Play

| # | Requirement |
|---|-------------|
| 1 | R-15.4.6    |

1. **#1** — Play state machine with Idle -> Walk -> Run transitions
   - **Expected:** correct state transitions match parameter changes

### TC-15.4.7.I1 Cross-Rig Retarget

| # | Requirement |
|---|-------------|
| 1 | R-15.4.7    |

1. **#1** — Retarget humanoid walk to quadruped rig
   - **Expected:** no self-intersection in retargeted mesh

## Benchmarks

### TC-15.3.1.B1 Material Graph Compile

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Compile 50-node material graph to HLSL | latency | < 100 ms | R-15.3.1 |

### TC-15.3.1.B2 DXC HLSL to DXIL

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | DXC compile generated HLSL to DXIL | latency | < 500 ms | R-15.3.1 |

### TC-15.3.4.B1 Parameter Update Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Uniform buffer write for scalar parameter change | latency | < 1 ms | R-15.3.4 |

### TC-15.3.3.B1 Preview Refresh

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full preview refresh after graph topology change | latency | < 16 ms | R-15.3.3 |

### TC-15.3.5.B1 Instance Memory

| # | Metric | Target      | Requirement |
|---|--------|-------------|-------------|
| 1 | bytes  | < 256 bytes | R-15.3.5    |

1. **1** — Memory per `MaterialInstance` variation (uniform buffer only)

### TC-15.4.1.B1 Timeline Scrub

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Scrub timeline with 300-bone clip | frame rate | > 30 FPS | R-15.4.1 |

### TC-15.4.5.B1 Blend Space Evaluation

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Evaluate 2D blend space with 9 samples | latency | < 0.5 ms | R-15.4.5 |

### TC-15.4.6.B1 State Machine Step

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Single state machine evaluation step | latency | < 0.1 ms | R-15.4.6 |
