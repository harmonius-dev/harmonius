# GPU Abstraction Layer Test Cases

Companion test cases for [gpu-abstraction.md](gpu-abstraction.md).

## Unit Tests

### TC-2.1.1.1 Static Dispatch No Vtable

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Compiled backend binary, trait call sites | No indirect calls in assembly at trait sites | R-2.1.1, NFR-2.1.1 |

### TC-2.1.1.2 Buffer Create Destroy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Create buffer | Handle valid | R-2.1.1 |
| 2 | Destroy buffer | Handle invalid | R-2.1.1 |

### TC-2.1.1.3 Texture Create All Formats

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Every Format enum variant | Texture created, no errors | R-2.1.1 |

### TC-2.1.2.1 Cmd Buf Graphics Compute Copy

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | One graphics, one compute, one copy op per cmd buf | Fence signals after submit | R-2.1.2 |

### TC-2.1.2.2 Cmd Buf Type-Safe Binding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bind wrong resource type to slot | Compile-time error | R-2.1.2 |

### TC-2.1.3.1 PSO Invalid Combination

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PSO with invalid blend/depth config | Structured error at creation, not at encoding | R-2.1.3 |

### TC-2.1.3.2 PSO Zero-Cost Encoding

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | PSO bind during encoding | Zero conditional branches vs raw backend | R-2.1.3 |

### TC-2.1.4.1 Metal FFI No ObjC

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Metal FFI boundary signatures | Only C-compatible signatures, no Objective-C selectors | R-2.1.4 |

### TC-2.1.5.1 D3D12 No C++ No Windows-rs

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | D3D12 dependency graph | No C++ translation units, no windows-rs | R-2.1.5 |

### TC-2.1.6.1 Vulkan Validation Zero Errors

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Conformance suite with VK validation layers | Zero validation errors | R-2.1.6 |

### TC-2.1.6.2 Vulkan Loader Runtime

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Vulkan loader linkage | Runtime-discovered, not statically linked | R-2.1.6 |

### TC-2.1.7.1 Sub-Alloc Alignment D3D12

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sub-allocated buffer offsets on D3D12 | All offsets 256 B aligned | R-2.1.7, GR-1.2 |

### TC-2.1.7.2 Sub-Alloc Alignment Vulkan

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sub-allocated resources on Vulkan | Per-resource alignment queries respected | R-2.1.7, GR-1.2 |

### TC-2.1.7.3 Sub-Alloc Alignment Metal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sub-allocated resources on Metal | Page alignment (4096 B) | R-2.1.7, GR-1.2 |

### TC-2.1.8.1 State Tracker Redundant Bind

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Set same pipeline twice | API trace shows single bind call | R-2.1.8, GR-2.2 |

### TC-2.1.8.2 State Tracker Reset On Begin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Call begin() on command buffer | All caches reset to unknown | GR-2.7 |

### TC-2.1.9.1 Barrier Merge

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Three consecutive barriers on same resource | Single merged API call | R-2.1.9 |

### TC-2.1.9.2 Barrier Noop Metal

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Barrier calls on Metal backend | Barrier calls elided | R-2.1.9 |

### TC-2.1.9.3 Split Barrier Overlap

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Split barrier across independent work | GPU overlap verified via capture | R-2.1.9, GR-4.2 |

### TC-2.1.10.1 Work Graph Native D3D12

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Work graph executed on D3D12 native API | Correct output | R-2.1.10, GR-3.2 |

### TC-2.1.10.2 Work Graph Emulated

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Same work graph on emulated path | Output matches native within FP tolerance | R-2.1.10, GR-3.3 |

### TC-2.1.11.1 Emulation No Runtime Branch

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Device without mesh shaders | Emulated path selected at creation, no runtime branches | R-2.1.11, GR-4.1 |

### TC-2.1.12.1 Timestamp Query Readback

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Bracket 5 passes, read back next frame | Non-zero, monotonic timestamps | R-2.1.12 |

### TC-2.1.12.2 Profiling No Stall

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query readback during frame | No GPU idle time introduced | R-2.1.12 |

### TC-GR.1.5.1 Ring Buffer Zero Alloc

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000 ring slices per frame | Zero OS-level heap allocations | GR-1.5 |

### TC-GR.1.3.1 Committed Alloc

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Large texture, committed alloc | Dedicated heap assigned | GR-1.3 |

### TC-GR.1.4.1 Placed Alloc Aliasing

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Two placed resources in same heap, overlapping offsets | Valid aliasing | GR-1.4 |

### TC-GR.1.6.1 Defragment Reduces Waste

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Fragmented heap, run `defragment_step()` | Fragmentation reduced | GR-1.6 |

### TC-GR.1.7.1 Budget Query

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Query GPU memory budget | Non-zero values matching expected VRAM size | GR-1.7 |

### TC-GR.2.5.1 Push Constant Dedup

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Identical push constants written twice | Single API call emitted | GR-2.5 |

### TC-2.1.0.1 Fence Async No Spin

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Await fence completion | No CPU spin-wait; worker resumes other tasks | F-2.1.1 |

## Integration Tests

### TC-2.1.1.I1 Cross Backend Image Diff

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Reference scene on Metal, D3D12, Vulkan | Output images pixel-identical within threshold | R-2.1.1 |

### TC-2.1.1.I2 10K Draws Overhead

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K-draw benchmark, abstraction vs raw API | < 5% CPU overhead | NFR-2.1.1 |

### TC-2.1.1.I3 10K Draws Alloc Count

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 10K draws, count OS GPU allocs per frame | < 64 allocations | NFR-2.1.2 |

### TC-2.1.8.I1 State Tracker Reduction

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | 1000-draw cmd buf, with vs without tracker | >= 20% API call reduction | NFR-2.1.3 |

### TC-2.1.8.I2 State Tracker Memory

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | State tracker memory per cmd buf | <= 64 KB | NFR-2.1.3 |

### TC-2.1.0.I1 Shader Compile All Targets

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Sample HLSL compiled to DXIL, SPIR-V, metallib | Non-empty bytecode for all targets | F-2.1.1 |

### TC-2.1.1.I4 Swapchain Resize

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Resize window | Swapchain recreates, presents correctly | F-2.1.1 |

### TC-2.1.0.I2 Reactor Fence Integration

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Submit GPU work, await fence via reactor | Completion detected at poll() | F-2.1.1 |

### TC-2.1.3.I1 PSO Cache Warm Mobile

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Mobile, warm PSO cache at load | No hitching during draw | US-2.1.3.2 |

### TC-2.1.11.I1 Mesh Shader Emulation Visual

| # | Input | Expected Output | Requirement |
|---|-------|-----------------|-------------|
| 1 | Scene on capable and incapable HW | Visual match >= 40 dB PSNR | GR-4.1 |

## Benchmarks

### TC-2.1.1.B1 Abstraction Overhead

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 10K draws | CPU overhead vs raw API | < 5% | NFR-2.1.1 |

### TC-2.1.7.B1 Sub-Allocation Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Sub-allocation | Amortized time | O(1) | NFR-2.1.2 |

### TC-2.1.7.B2 OS GPU Allocs Per Frame

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Full frame rendering | OS GPU allocs | < 64 | NFR-2.1.2 |

### TC-2.1.8.B1 State Tracker API Call Reduction

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 1K draws | API call reduction | >= 20% | NFR-2.1.3 |

### TC-2.1.8.B2 State Tracker Memory

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per command buffer | Memory usage | <= 64 KB | NFR-2.1.3 |

### TC-GR.1.5.B1 Ring Buffer Alloc

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Hot path ring alloc | Heap allocations | 0 | GR-1.5 |

### TC-2.1.0.B1 Shader Compile HLSL to DXIL

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per shader | Compile time | < 100 ms | F-2.1.1 |

### TC-2.1.0.B2 Shader Compile DXIL to Metallib

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Per shader | Compile time | < 50 ms | F-2.1.1 |

### TC-2.1.9.B1 Barrier Batch

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | 100 barriers | API calls | Single call | R-2.1.9 |

### TC-2.1.0.B3 Fence Async Wakeup Latency

| # | Scenario | Metric | Target | Requirement |
|---|----------|--------|--------|-------------|
| 1 | Fence completion wakeup | Latency | < 1 frame (16 ms @ 60 fps) | F-2.1.1 |
