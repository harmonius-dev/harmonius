# Material and Animation Editors Test Cases

Companion test cases for [material-animation.md](material-animation.md).

## Unit Tests

### TC-15.3.1.1 Pin Type Validation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Connect `Color` output to `Scalar` input | `Err(PinTypeError::Incompatible { from: Color, to: Scalar })` | R-15.3.1 |
| 2 | Connect `Vec3` output to `Vec3` input | `Ok(())` | R-15.3.1 |
| 3 | Connect to already-connected input pin | `Err(PinTypeError::AlreadyConnected { pin })` | R-15.3.1 |

### TC-15.3.1.2 Graph Cycle Detection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create cycle: A -> B -> C -> A | `compile()` returns `Err(CycleDetected { cycle: [A, B, C] })` | R-15.3.1 |
| 2 | Acyclic graph A -> B -> C -> Output | `compile()` returns `Ok(HlslSource)` | R-15.3.1 |

### TC-15.3.1.3 Graph Compile Simple

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | SampleTexture2D -> SurfaceOutput | `Ok(HlslSource)` with non-empty `pixel_shader` containing `Sample` | R-15.3.1 |

### TC-15.3.1.4 Dead Code Elimination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Graph with 2 unused Multiply nodes not connected to output | compiled HLSL does not contain those multiply operations | R-15.3.1 |

### TC-15.3.2.1 Function Inline

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Inline triplanar mapping function into parent graph | output HLSL matches manually constructed equivalent graph | R-15.3.2 |

### TC-15.3.2.2 Function Propagation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Update function signature (add pin), compile referencing materials | all referencing materials recompile without stale reference errors | R-15.3.2 |

### TC-15.3.3.1 Preview Update Timing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change scalar parameter via tweaker, measure preview update | preview updates within 16 ms (one frame at 60 FPS) | R-15.3.3 |

### TC-15.3.4.1 Parameter No Recompile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change `ScalarParameter` value from `0.5` to `0.8` | uniform buffer updated, no DXC invocation triggered | R-15.3.4 |

### TC-15.3.5.1 Instance Shader Sharing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create 100 `MaterialInstance` from same parent | all 100 return identical `ShaderHandle` from `get_shader_handle()` | R-15.3.5 |

### TC-15.3.5.2 Instance Override Isolation

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Override color on instance, check parent | parent's parameter unchanged, `instance.has_override("color") == true` | R-15.3.5 |
| 2 | `remove_override("color")`, check effective value | effective value reverts to parent default | R-15.3.5 |

### TC-15.3.6.1 Library Search by Tag

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Add tag `"metal"` to 3 materials, search with `MaterialFilter { tags: ["metal"] }` | exactly 3 `MaterialEntry` results | R-15.3.6 |
| 2 | Search with tag `"nonexistent"` | 0 results | R-15.3.6 |

### TC-15.3.6.2 Library Usage Tracking

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Assign material M to mesh asset A | `get_usage(M)` includes asset A in results | R-15.3.6 |

### TC-15.4.1.1 Timeline Keyframe Snap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Move keyframe to time `0.517` at 30 FPS sample rate | snaps to `0.5` (frame 15) | R-15.4.1 |
| 2 | Move keyframe to time `0.034` at 30 FPS | snaps to `0.033` (frame 1) | R-15.4.1 |

### TC-15.4.1.2 Timeline Scrub Performance

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Load 300-bone clip, scrub timeline, measure FPS | FPS > 30 during scrub | R-15.4.1 |

### TC-15.4.2.1 Bezier Curve Continuity

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create 3 keyframes with `Bezier` tangent mode | `evaluate()` produces C1 continuous values at joins (derivative matches) | R-15.4.2 |

### TC-15.4.2.2 Curve Preset Accuracy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Apply `EaseIn` preset, evaluate at t=0.5 | value matches reference ease-in curve within 0.01 tolerance | R-15.4.2 |
| 2 | Apply `Linear` preset, evaluate at t=0.5 | value == 0.5 (midpoint) | R-15.4.2 |

### TC-15.4.3.1 Skeleton Child Chain Selection

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Select `UpperArm` bone in hierarchy with 3 children | `selected_chain()` returns `[UpperArm, LowerArm, Hand, Fingers]` | R-15.4.3 |

### TC-15.4.4.1 Preview Root Motion Trajectory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play root motion walk clip for 2 seconds at 1 m/s | trajectory distance within 0.1m of 2.0m | R-15.4.4 |

### TC-15.4.5.1 Blend Space Center Weights

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 2D blend space with 4 corner samples, crosshair at `(0.5, 0.5)` | all 4 weights approximately equal (0.25 each, within 0.05) | R-15.4.5 |

### TC-15.4.5.2 Blend Space 1D

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1D blend with samples at 0.0 and 1.0, crosshair at `(0.5, 0.0)` | weights `[(0, 0.5), (1, 0.5)]` | R-15.4.5 |

### TC-15.4.6.1 State Machine Reachability

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | State machine with unreachable state S3 (no transitions to it) | `validate()` returns `UnreachableState { state: S3 }` | R-15.4.6 |
| 2 | State machine with no default state | `validate()` returns `NoDefaultState` | R-15.4.6 |

### TC-15.4.6.2 State Machine Transition

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Transition condition: `speed > 0.5`, set `speed = 0.8` | `active_state()` changes from Idle to Walk | R-15.4.6 |
| 2 | Same condition, set `speed = 0.3` | `active_state()` remains Idle | R-15.4.6 |

### TC-15.4.7.1 Retarget Bone Mapping

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Map source `Hips` to target `Hips`, apply T-pose | target pose matches source pose orientation within 1 degree | R-15.4.7 |

### TC-15.4.7.2 Retarget Foot Contact

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Retarget walk cycle, measure foot contact height | foot contact height within 0.02m of ground plane | R-15.4.7 |

## Integration Tests

### TC-15.3.1.I1 Material Compile D3D12

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full pipeline: graph -> HLSL -> DXC -> DXIL | valid DXIL bytecode, renders correct output on D3D12 | R-15.3.1 |

### TC-15.3.1.I2 Material Compile Vulkan

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full pipeline: graph -> HLSL -> DXC -> SPIR-V | valid SPIR-V bytecode, renders correct output on Vulkan | R-15.3.1 |

### TC-15.3.1.I3 Material Compile Metal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Full pipeline: graph -> HLSL -> DXC -> DXIL -> MSC -> MSL | valid MSL source, renders correct output on Metal | R-15.3.1 |

### TC-15.3.3.I1 Material Hot Reload

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Change graph topology in editor, observe viewport | viewport updates within 2 frames, no restart needed | R-15.3.3 |

### TC-15.4.6.I1 State Machine Play

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Play state machine with Idle -> Walk -> Run transitions | correct state transitions match parameter changes | R-15.4.6 |

### TC-15.4.7.I1 Cross-Rig Retarget

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Retarget humanoid walk to quadruped rig | no self-intersection in retargeted mesh | R-15.4.7 |

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

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Memory per `MaterialInstance` variation (uniform buffer only) | bytes | < 256 bytes | R-15.3.5 |

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
