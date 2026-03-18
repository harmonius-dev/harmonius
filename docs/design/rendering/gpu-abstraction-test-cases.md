# GPU Abstraction Layer Test Cases

Companion test cases for [gpu-abstraction.md](gpu-abstraction.md).

## Unit Tests

### TC-2.1.1.1 Static Dispatch No Vtable

| # | Requirement        |
|---|--------------------|
| 1 | R-2.1.1, NFR-2.1.1 |

1. **#1** — Compiled backend binary, trait call sites
   - **Expected:** No indirect calls in assembly at trait sites

### TC-2.1.1.2 Buffer Create Destroy

| # | Requirement |
|---|-------------|
| 1 | R-2.1.1     |
| 2 | R-2.1.1     |

1. **#1** — Create buffer
   - **Expected:** Handle valid
2. **#2** — Destroy buffer
   - **Expected:** Handle invalid

### TC-2.1.1.3 Texture Create All Formats

| # | Requirement |
|---|-------------|
| 1 | R-2.1.1     |

1. **#1** — Every Format enum variant
   - **Expected:** Texture created, no errors

### TC-2.1.2.1 Cmd Buf Graphics Compute Copy

| # | Requirement |
|---|-------------|
| 1 | R-2.1.2     |

1. **#1** — One graphics, one compute, one copy op per cmd buf
   - **Expected:** Fence signals after submit

### TC-2.1.2.2 Cmd Buf Type-Safe Binding

| # | Requirement |
|---|-------------|
| 1 | R-2.1.2     |

1. **#1** — Bind wrong resource type to slot
   - **Expected:** Compile-time error

### TC-2.1.3.1 PSO Invalid Combination

| # | Requirement |
|---|-------------|
| 1 | R-2.1.3     |

1. **#1** — PSO with invalid blend/depth config
   - **Expected:** Structured error at creation, not at encoding

### TC-2.1.3.2 PSO Zero-Cost Encoding

| # | Requirement |
|---|-------------|
| 1 | R-2.1.3     |

1. **#1** — PSO bind during encoding
   - **Expected:** Zero conditional branches vs raw backend

### TC-2.1.4.1 Metal FFI No ObjC

| # | Requirement |
|---|-------------|
| 1 | R-2.1.4     |

1. **#1** — Metal FFI boundary signatures
   - **Expected:** Only C-compatible signatures, no Objective-C selectors

### TC-2.1.5.1 D3D12 No C++ No Windows-rs

| # | Requirement |
|---|-------------|
| 1 | R-2.1.5     |

1. **#1** — D3D12 dependency graph
   - **Expected:** No C++ translation units, no windows-rs

### TC-2.1.6.1 Vulkan Validation Zero Errors

| # | Requirement |
|---|-------------|
| 1 | R-2.1.6     |

1. **#1** — Conformance suite with VK validation layers
   - **Expected:** Zero validation errors

### TC-2.1.6.2 Vulkan Loader Runtime

| # | Requirement |
|---|-------------|
| 1 | R-2.1.6     |

1. **#1** — Vulkan loader linkage
   - **Expected:** Runtime-discovered, not statically linked

### TC-2.1.7.1 Sub-Alloc Alignment D3D12

| # | Requirement     |
|---|-----------------|
| 1 | R-2.1.7, GR-1.2 |

1. **#1** — Sub-allocated buffer offsets on D3D12
   - **Expected:** All offsets 256 B aligned

### TC-2.1.7.2 Sub-Alloc Alignment Vulkan

| # | Requirement     |
|---|-----------------|
| 1 | R-2.1.7, GR-1.2 |

1. **#1** — Sub-allocated resources on Vulkan
   - **Expected:** Per-resource alignment queries respected

### TC-2.1.7.3 Sub-Alloc Alignment Metal

| # | Requirement     |
|---|-----------------|
| 1 | R-2.1.7, GR-1.2 |

1. **#1** — Sub-allocated resources on Metal
   - **Expected:** Page alignment (4096 B)

### TC-2.1.8.1 State Tracker Redundant Bind

| # | Requirement     |
|---|-----------------|
| 1 | R-2.1.8, GR-2.2 |

1. **#1** — Set same pipeline twice
   - **Expected:** API trace shows single bind call

### TC-2.1.8.2 State Tracker Reset On Begin

| # | Requirement |
|---|-------------|
| 1 | GR-2.7      |

1. **#1** — Call begin() on command buffer
   - **Expected:** All caches reset to unknown

### TC-2.1.9.1 Barrier Merge

| # | Requirement |
|---|-------------|
| 1 | R-2.1.9     |

1. **#1** — Three consecutive barriers on same resource
   - **Expected:** Single merged API call

### TC-2.1.9.2 Barrier Noop Metal

| # | Requirement |
|---|-------------|
| 1 | R-2.1.9     |

1. **#1** — Barrier calls on Metal backend
   - **Expected:** Barrier calls elided

### TC-2.1.9.3 Split Barrier Overlap

| # | Requirement     |
|---|-----------------|
| 1 | R-2.1.9, GR-4.2 |

1. **#1** — Split barrier across independent work
   - **Expected:** GPU overlap verified via capture

### TC-2.1.10.1 Work Graph Native D3D12

| # | Requirement      |
|---|------------------|
| 1 | R-2.1.10, GR-3.2 |

1. **#1** — Work graph executed on D3D12 native API
   - **Expected:** Correct output

### TC-2.1.10.2 Work Graph Emulated

| # | Requirement      |
|---|------------------|
| 1 | R-2.1.10, GR-3.3 |

1. **#1** — Same work graph on emulated path
   - **Expected:** Output matches native within FP tolerance

### TC-2.1.11.1 Emulation No Runtime Branch

| # | Requirement      |
|---|------------------|
| 1 | R-2.1.11, GR-4.1 |

1. **#1** — Device without mesh shaders
   - **Expected:** Emulated path selected at creation, no runtime branches

### TC-2.1.12.1 Timestamp Query Readback

| # | Requirement |
|---|-------------|
| 1 | R-2.1.12    |

1. **#1** — Bracket 5 passes, read back next frame
   - **Expected:** Non-zero, monotonic timestamps

### TC-2.1.12.2 Profiling No Stall

| # | Requirement |
|---|-------------|
| 1 | R-2.1.12    |

1. **#1** — Query readback during frame
   - **Expected:** No GPU idle time introduced

### TC-GR.1.5.1 Ring Buffer Zero Alloc

| # | Requirement |
|---|-------------|
| 1 | GR-1.5      |

1. **#1** — 1000 ring slices per frame
   - **Expected:** Zero OS-level heap allocations

### TC-GR.1.3.1 Committed Alloc

| # | Requirement |
|---|-------------|
| 1 | GR-1.3      |

1. **#1** — Large texture, committed alloc
   - **Expected:** Dedicated heap assigned

### TC-GR.1.4.1 Placed Alloc Aliasing

| # | Requirement |
|---|-------------|
| 1 | GR-1.4      |

1. **#1** — Two placed resources in same heap, overlapping offsets
   - **Expected:** Valid aliasing

### TC-GR.1.6.1 Defragment Reduces Waste

| # | Requirement |
|---|-------------|
| 1 | GR-1.6      |

1. **#1** — Fragmented heap, run `defragment_step()`
   - **Expected:** Fragmentation reduced

### TC-GR.1.7.1 Budget Query

| # | Requirement |
|---|-------------|
| 1 | GR-1.7      |

1. **#1** — Query GPU memory budget
   - **Expected:** Non-zero values matching expected VRAM size

### TC-GR.2.5.1 Push Constant Dedup

| # | Requirement |
|---|-------------|
| 1 | GR-2.5      |

1. **#1** — Identical push constants written twice
   - **Expected:** Single API call emitted

### TC-2.1.0.1 Fence Async No Spin

| # | Requirement |
|---|-------------|
| 1 | F-2.1.1     |

1. **#1** — Await fence completion
   - **Expected:** No CPU spin-wait; worker resumes other tasks

## Integration Tests

### TC-2.1.1.I1 Cross Backend Image Diff

| # | Requirement |
|---|-------------|
| 1 | R-2.1.1     |

1. **#1** — Reference scene on Metal, D3D12, Vulkan
   - **Expected:** Output images pixel-identical within threshold

### TC-2.1.1.I2 10K Draws Overhead

| # | Requirement |
|---|-------------|
| 1 | NFR-2.1.1   |

1. **#1** — 10K-draw benchmark, abstraction vs raw API
   - **Expected:** < 5% CPU overhead

### TC-2.1.1.I3 10K Draws Alloc Count

| # | Requirement |
|---|-------------|
| 1 | NFR-2.1.2   |

1. **#1** — 10K draws, count OS GPU allocs per frame
   - **Expected:** < 64 allocations

### TC-2.1.8.I1 State Tracker Reduction

| # | Requirement |
|---|-------------|
| 1 | NFR-2.1.3   |

1. **#1** — 1000-draw cmd buf, with vs without tracker
   - **Expected:** >= 20% API call reduction

### TC-2.1.8.I2 State Tracker Memory

| # | Requirement |
|---|-------------|
| 1 | NFR-2.1.3   |

1. **#1** — State tracker memory per cmd buf
   - **Expected:** <= 64 KB

### TC-2.1.0.I1 Shader Compile All Targets

| # | Requirement |
|---|-------------|
| 1 | F-2.1.1     |

1. **#1** — Sample HLSL compiled to DXIL, SPIR-V, metallib
   - **Expected:** Non-empty bytecode for all targets

### TC-2.1.1.I4 Swapchain Resize

| # | Requirement |
|---|-------------|
| 1 | F-2.1.1     |

1. **#1** — Resize window
   - **Expected:** Swapchain recreates, presents correctly

### TC-2.1.0.I2 Reactor Fence Integration

| # | Requirement |
|---|-------------|
| 1 | F-2.1.1     |

1. **#1** — Submit GPU work, await fence via reactor
   - **Expected:** Completion detected at poll()

### TC-2.1.3.I1 PSO Cache Warm Mobile

| # | Requirement |
|---|-------------|
| 1 | US-2.1.3.2  |

1. **#1** — Mobile, warm PSO cache at load
   - **Expected:** No hitching during draw

### TC-2.1.11.I1 Mesh Shader Emulation Visual

| # | Requirement |
|---|-------------|
| 1 | GR-4.1      |

1. **#1** — Scene on capable and incapable HW
   - **Expected:** Visual match >= 40 dB PSNR

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
