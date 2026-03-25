# User Stories -- 2.1 GPU Abstraction Layer

## Stories

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-2.1.1.1  | engine developer (P-26) |          |              |
| US-2.1.1.2  | engine tester (P-27)    |          |              |
| US-2.1.2.1  | engine developer (P-26) |          |              |
| US-2.1.2.2  | engine tester (P-27)    |          |              |
| US-2.1.3.1  | engine developer (P-26) |          |              |
| US-2.1.3.2  | engine tester (P-27)    |          |              |
| US-2.1.4.1  | engine developer (P-26) |          |              |
| US-2.1.4.2  | engine tester (P-27)    |          |              |
| US-2.1.5.1  | engine developer (P-26) |          |              |
| US-2.1.5.2  | engine tester (P-27)    |          |              |
| US-2.1.6.1  | engine developer (P-26) |          |              |
| US-2.1.6.2  | engine tester (P-27)    |          |              |
| US-2.1.7.1  | engine developer (P-26) |          |              |
| US-2.1.7.2  | engine tester (P-27)    |          |              |
| US-2.1.8.1  | engine developer (P-26) |          |              |
| US-2.1.8.2  | engine tester (P-27)    |          |              |
| US-2.1.9.1  | engine developer (P-26) |          |              |
| US-2.1.9.2  | engine tester (P-27)    |          |              |
| US-2.1.10.1 | engine developer (P-26) |          |              |
| US-2.1.10.2 | engine tester (P-27)    |          |              |
| US-2.1.11.1 | engine developer (P-26) |          |              |
| US-2.1.11.2 | engine tester (P-27)    |          |              |
| US-2.1.12.1 | technical artist (P-13) |          |              |
| US-2.1.12.2 | engine tester (P-27)    |          |              |
| US-2.1.12.3 | engine developer (P-26) |          |              |
| US-2.1.13.1 | technical artist (P-13) |          |              |
| US-2.1.13.2 | engine developer (P-26) |          |              |
| US-2.1.13.3 | engine tester (P-27)    |          |              |
| US-2.1.14.1 | engine developer (P-26) |          |              |
| US-2.1.14.2 | engine developer (P-26) |          |              |
| US-2.1.14.3 | engine tester (P-27)    |          |              |
| US-2.1.14.4 | QA tester (P-19)        |          |              |

1. **US-2.1.1.1** — I want a unified GPU backend trait with associated types for device, command
   buffer, pipeline state, and resource handles using static dispatch via generics
   - **Acceptance:** I can write rendering code once without vtable overhead or backend-specific
     branching
2. **US-2.1.1.2** — I want to run the full rendering test suite on each backend and diff the output
   images
   - **Acceptance:** I can confirm all three backends produce identical results from the same
     trait-based rendering code
3. **US-2.1.2.1** — I want a trait-based command buffer abstraction supporting graphics, compute,
   and copy operations with type-safe resource binding
   - **Acceptance:** I can encode rendering work independently and submit ordered batches without
     unsafe casts
4. **US-2.1.2.2** — I want to trace command buffer submission and verify that Metal uses
   MTLCommandBuffer, D3D12 uses ID3D12GraphicsCommandList, and Vulkan uses VkCommandBuffer
   - **Acceptance:** the abstraction maps correctly to native APIs
5. **US-2.1.3.1** — I want pipeline state objects pre-validated at creation time covering shaders,
   vertex layout, blend state, and depth-stencil configuration
   - **Acceptance:** command buffer encoding has zero runtime validation cost
6. **US-2.1.3.2** — I want to confirm that pipeline state caches are warmed at load time on
   tile-based mobile GPUs to avoid hitching during gameplay
   - **Acceptance:** PSO creation does not cause frame drops on mobile
7. **US-2.1.4.1** — I want the Metal GPU backend exposed as a Swift library with @_cdecl
   C-compatible functions consumed by Rust through `extern "C"` declarations
   - **Acceptance:** Metal functionality is available without Objective-C or C++ in the FFI boundary
8. **US-2.1.4.2** — I want to confirm that the Metal backend compiles only on macOS and iOS targets
   and is excluded from Windows and Linux builds
   - **Acceptance:** platform gating is correct
9. **US-2.1.5.1** — I want the D3D12 backend implemented in pure Rust using `windows-rs` COM
   bindings with safe wrappers managing COM reference counting
   - **Acceptance:** D3D12 is accessed without C++ dependencies
10. **US-2.1.5.2** — I want to confirm that the D3D12 backend compiles only on Windows targets and
    is excluded from macOS and Linux builds
    - **Acceptance:** platform gating is correct
11. **US-2.1.6.1** — I want a Vulkan backend using `ash` with RAII lifetime management and
    validation layers enabled in debug builds
    - **Acceptance:** runtime Vulkan errors are caught during development
12. **US-2.1.6.2** — I want to confirm that the Vulkan loader is discovered at runtime on Windows
    and Linux, and that MoltenVK is never used on macOS
    - **Acceptance:** Vulkan initialization follows the specified platform strategy
13. **US-2.1.7.1** — I want a GPU heap sub-allocator that carves typed buffer and texture regions
    from pre-allocated memory blocks
    - **Acceptance:** thousands of per-draw constant uploads use offset-based binding from a handful
      of OS allocations
14. **US-2.1.7.2** — I want to verify that sub-allocations respect 256-byte alignment on D3D12,
    variable alignment on Vulkan, and page alignment on Metal
    - **Acceptance:** GPU memory access is correctly aligned on each backend
15. **US-2.1.8.1** — I want CPU-side shadow state tracking to filter redundant pipeline, binding,
    and render target transitions before encoding
    - **Acceptance:** driver overhead is minimized during high-frequency draw submission
16. **US-2.1.8.2** — I want to measure draw submission cost with and without state tracking on
    mobile (where per-change overhead is higher) and desktop
    - **Acceptance:** I can confirm the state tracker provides measurable improvement on all
      platforms
17. **US-2.1.9.1** — I want automatic barrier batching, merging, and split barrier insertion within
    command buffers
    - **Acceptance:** consecutive transitions are coalesced and GPU pipeline stalls are reduced
18. **US-2.1.9.2** — I want to confirm that barrier optimization is a no-op on Metal where the
    driver handles hazard tracking, while D3D12 and Vulkan emit explicit barriers
    - **Acceptance:** per-backend barrier behavior is correct
19. **US-2.1.10.1** — I want work graph support where GPU nodes dispatch work to subsequent nodes
    without CPU involvement
    - **Acceptance:** GPU-driven rendering pipelines avoid per-dispatch CPU latency
20. **US-2.1.10.2** — I want to verify that work graphs use native D3D12 API on Windows and
    compute-based emulation via indirect dispatch chains on Vulkan and Metal
    - **Acceptance:** the feature works consistently across all backends
21. **US-2.1.11.1** — I want a cross-backend feature emulation layer that selects emulated paths at
    device creation time based on capability queries
    - **Acceptance:** mesh shaders, work graphs, and enhanced barriers work on all backends without
      runtime branching
22. **US-2.1.11.2** — I want to run on Vulkan drivers that lack mesh shader support and verify that
    the emulation path produces correct output matching native mesh shader results
    - **Acceptance:** emulated features are visually identical to native
23. **US-2.1.12.1** — I want per-pass GPU timestamps and pipeline statistics visible in the editor's
    GPU profiler
    - **Acceptance:** I can identify which rendering passes are most expensive and optimize art
      content accordingly
24. **US-2.1.12.2** — I want to confirm that timestamp queries use MTLCounterSampleBuffer on Metal,
    ID3D12QueryHeap on D3D12, and vkCmdWriteTimestamp on Vulkan
    - **Acceptance:** GPU profiling data is accurate on each platform
25. **US-2.1.12.3** — I want GPU timestamp query results read back one frame after submission rather
    than the same frame
    - **Acceptance:** profiling does not introduce GPU stalls or pipeline bubbles
26. **US-2.1.13.1** — I want CPU work graph emulation via indirect dispatch so that GPU-driven
    effects (particle culling, mesh LOD selection, indirect draws) produce identical results on
    Metal and Vulkan as on D3D12
    - **Acceptance:** I author effects once and they work everywhere
27. **US-2.1.13.2** — I want emulated work graph nodes to appear as `TaskNode` entries in the
    unified game loop graph
    - **Acceptance:** work-graph-style GPU scheduling is scheduled alongside CPU work without a
      separate dispatch path
28. **US-2.1.13.3** — I want tests that execute a work graph on D3D12 natively and on Metal/Vulkan
    via emulation, comparing output within floating-point tolerance
    - **Acceptance:** cross-backend correctness is verified in CI
29. **US-2.1.14.1** — I want all GPU resources referenced through generational `Handle<T>` indices
    that detect use-after-free at runtime
    - **Acceptance:** memory bugs from stale resource references are impossible in the public API
30. **US-2.1.14.2** — I want no raw GPU pointers (device addresses, mapped memory pointers) to
    appear in any public API type
    - **Acceptance:** the entire rendering interface is memory-safe by construction
31. **US-2.1.14.3** — I want compile tests that attempt to pass a `Handle<Buffer>` where
    `Handle<Texture>` is expected and assert the code fails to compile
    - **Acceptance:** handle type safety is verified in CI
32. **US-2.1.14.4** — I want runtime detection of stale GPU handles (generation mismatch after
    resource free) with a structured error rather than undefined behavior
    - **Acceptance:** use-after-free bugs are caught immediately during testing
