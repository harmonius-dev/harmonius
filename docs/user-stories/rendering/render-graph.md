# User Stories -- 2.2 Render Graph

## Stories

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-2.2.1.1  | engine developer (P-26) |          |              |
| US-2.2.1.2  | engine tester (P-27)    |          |              |
| US-2.2.2.1  | engine developer (P-26) |          |              |
| US-2.2.2.2  | engine tester (P-27)    |          |              |
| US-2.2.3.1  | engine developer (P-26) |          |              |
| US-2.2.3.2  | engine tester (P-27)    |          |              |
| US-2.2.4.1  | engine developer (P-26) |          |              |
| US-2.2.4.2  | engine tester (P-27)    |          |              |
| US-2.2.5.1  | engine developer (P-26) |          |              |
| US-2.2.5.2  | engine tester (P-27)    |          |              |
| US-2.2.6.1  | engine developer (P-26) |          |              |
| US-2.2.6.2  | engine tester (P-27)    |          |              |
| US-2.2.7.1  | engine developer (P-26) |          |              |
| US-2.2.7.2  | engine tester (P-27)    |          |              |
| US-2.2.8.1  | game designer (P-5)     |          |              |
| US-2.2.8.2  | engine tester (P-27)    |          |              |
| US-2.2.9.1  | engine developer (P-26) |          |              |
| US-2.2.9.2  | engine tester (P-27)    |          |              |
| US-2.2.10.1 | engine developer (P-26) |          |              |
| US-2.2.10.2 | engine tester (P-27)    |          |              |
| US-2.2.11.1 | game developer (P-15)   |          |              |
| US-2.2.11.2 | engine tester (P-27)    |          |              |
| US-2.2.12.1 | engine developer (P-26) |          |              |
| US-2.2.12.2 | engine tester (P-27)    |          |              |
| US-2.2.13.1 | technical artist (P-13) |          |              |
| US-2.2.13.2 | engine tester (P-27)    |          |              |
| US-2.2.14.1 | game designer (P-5)     |          |              |
| US-2.2.14.2 | engine developer (P-26) |          |              |
| US-2.2.14.3 | engine tester (P-27)    |          |              |
| US-2.2.15.1 | engine developer (P-26) |          |              |
| US-2.2.15.2 | game developer (P-15)   |          |              |
| US-2.2.15.3 | engine tester (P-27)    |          |              |

1. **US-2.2.1.1** — I want to declare passes as structs implementing a pass trait with typed
   resource reads and writes
   - **Acceptance:** the graph compiler discovers dependency topology and I never manually insert
     barriers or order passes
2. **US-2.2.1.2** — I want to confirm that pass registration is a CPU-side operation with no GPU
   overhead by measuring GPU timeline during graph construction
   - **Acceptance:** graph setup does not affect frame time
3. **US-2.2.2.1** — I want each pass to declare its required GPU capabilities (mesh shaders, RT,
   compute) so the compiler prunes unsupported passes and activates fallback implementations
   - **Acceptance:** the render graph adapts to any hardware configuration automatically
4. **US-2.2.2.2** — I want to run on a GPU without RT support and verify that all RT-dependent
   passes are pruned and replaced by rasterization fallbacks
   - **Acceptance:** the render graph compiles correctly without RT hardware
5. **US-2.2.3.1** — I want to declare transient resources with virtual handles and have the compiler
   map non-overlapping lifetimes to shared physical allocations
   - **Acceptance:** VRAM usage is minimized through automatic memory sharing
6. **US-2.2.3.2** — I want to confirm that transient resources use MTLStorageModeMemoryless on Metal
   and VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT on Vulkan
   - **Acceptance:** tile-local attachments consume zero external memory bandwidth on mobile
7. **US-2.2.4.1** — I want the compiler to build an interference graph and assign non-overlapping
   transient resources to the same physical memory block
   - **Acceptance:** peak VRAM consumption drops significantly in complex render graphs
8. **US-2.2.4.2** — I want to verify that resource aliasing uses placed resources in heaps on D3D12,
   memory aliasing with explicit invalidation on Vulkan, and MTLHeap makeAliasable on Metal
   - **Acceptance:** aliasing is correct per backend
9. **US-2.2.5.1** — I want the graph compiler to analyze read/write sets and insert the minimal
   barrier set at the latest point, using split barriers where supported
   - **Acceptance:** GPU pipeline stalls are minimized
10. **US-2.2.5.2** — I want to confirm that Metal relies on driver hazard tracking with fences
    emitted only at queue boundaries, while D3D12 and Vulkan use explicit barriers
    - **Acceptance:** barrier strategy is correct per backend
11. **US-2.2.6.1** — I want the compiler to assign passes to graphics, compute, and copy queues
    based on workload type and insert cross-queue fences
    - **Acceptance:** async compute passes overlap with graphics work to maximize GPU utilization
12. **US-2.2.6.2** — I want to verify that D3D12 and Vulkan use explicit queue families and Metal
    uses shared and private command queues
    - **Acceptance:** multi-queue scheduling maps correctly to each backend's queue model
13. **US-2.2.7.1** — I want topological sort to produce a stable execution order across frames for
    passes with no data dependency
    - **Acceptance:** GPU pipeline bubbles from reordering are avoided
14. **US-2.2.7.2** — I want to confirm that passes with no mutual data dependency are identified as
    parallelizable in the topological sort output
    - **Acceptance:** the parallel encoding system (F-2.2.10) has correct input
15. **US-2.2.8.1** — I want the render graph to automatically cull lowest-priority passes when
    estimated frame cost exceeds the target budget
    - **Acceptance:** the game maintains stable frame rate without manual per-platform pass
      configuration
16. **US-2.2.8.2** — I want to verify that budget culling targets 16-33ms on mobile, 16ms docked /
    33ms handheld on Switch, 16ms on desktop, and 8-16ms on high-end
    - **Acceptance:** pass culling thresholds are correct per platform
17. **US-2.2.9.1** — I want a single render graph instantiated with per-view parameter overrides for
    split-screen, VR stereo, shadow cascades, and reflection probes
    - **Acceptance:** shared passes execute once and fan out to view-specific passes
18. **US-2.2.9.2** — I want to verify max 4 views on mobile, 8 on Switch, configurable dozens on
    desktop, and unlimited with VR stereo on high-end
    - **Acceptance:** multi-view rendering stays within platform compute budgets
19. **US-2.2.10.1** — I want the graph compiler to distribute independent pass encoding across a
    thread pool using secondary command buffers
    - **Acceptance:** CPU submission latency is reduced by parallel encoding
20. **US-2.2.10.2** — I want to confirm that parallel encoding uses command list bundles on D3D12,
    secondary command buffers on Vulkan, and parallel render command encoders on Metal
    - **Acceptance:** encoding parallelism uses the correct native mechanism
21. **US-2.2.11.1** — I want render passes to use placeholder resources when streamed assets are
    unavailable
    - **Acceptance:** world traversal does not stall while textures, meshes, or acceleration
      structures load in the background
22. **US-2.2.11.2** — I want to verify streaming pools of 256-512MB on mobile, 1GB on Switch, 2-4GB
    on desktop, and 8+GB on high-end
    - **Acceptance:** streaming memory stays within per-platform budgets
23. **US-2.2.12.1** — I want the full render graph compiled into a flat command schedule once per
    topology change rather than every frame
    - **Acceptance:** per-frame parameter updates do not trigger recompilation overhead
24. **US-2.2.12.2** — I want to change pass parameters (viewport size, quality settings) without
    triggering graph recompilation, and verify recompilation fires only when passes are added or
    removed
    - **Acceptance:** recompilation frequency is minimized
25. **US-2.2.13.1** — I want a runtime diagnostic overlay showing the compiled graph as a DAG with
    per-pass GPU timing, resource lifetimes, and barrier placement
    - **Acceptance:** I can identify bottleneck passes and inefficient resource usage visually
26. **US-2.2.13.2** — I want to confirm that the render graph diagnostic overlay and logging are
    compile-time disabled in shipping builds
    - **Acceptance:** debug visualization has zero cost in production
27. **US-2.2.14.1** — I want render passes to integrate with the same scheduling graph as gameplay
    systems (ECS, physics, audio)
    - **Acceptance:** rendering dependencies on CPU work are explicit and the scheduler can overlap
      preparation with submission
28. **US-2.2.14.2** — I want render node dependencies on culling and scene traversal expressed as
    typed edges in the game loop graph
    - **Acceptance:** the scheduler automatically overlaps CPU rendering prep with GPU work from the
      previous phase
29. **US-2.2.14.3** — I want to compile a game loop graph containing render passes and assert that
    each render pass appears as a `TaskNode` in the compiled `TaskGraph`
    - **Acceptance:** render graph integration with the unified scheduler is verified in CI
30. **US-2.2.15.1** — I want command buffer references scoped to the encoding lifetime so the
    compiler rejects any attempt to store or return them beyond the render pass node's execution
    - **Acceptance:** use-after-submit bugs are caught at compile time
31. **US-2.2.15.2** — I want scoped GPU command encoding that ties command buffer validity to the
    render pass execution scope
    - **Acceptance:** I can encode commands safely without worrying about lifetime management
32. **US-2.2.15.3** — I want a compile test that attempts to return a command buffer reference from
    a render pass callback and asserts the code fails with a lifetime error
    - **Acceptance:** encoding scope safety is verified in CI
