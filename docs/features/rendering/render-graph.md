# 2.2 — Render Graph

## Pass Declaration

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.1 | Declarative Pass Registration | Passes are declared as structs implementing a pass trait, specifying their resource reads, writes, and queue requirements. The graph compiler discovers the full dependency topology from these declarations without requiring manual ordering. | R-2.2.1 | None | All platforms: full quality. Pass registration is CPU-side with no GPU cost. |
| F-2.2.2 | Capability Gating | Each pass declares the GPU capabilities it requires (e.g., mesh shaders, ray tracing, compute). The graph compiler prunes passes whose requirements exceed the current device's feature set, automatically falling back to alternative pass implementations registered for lower tiers. | R-2.2.2 | F-2.2.1 | Capability sets are queried from the GPU abstraction layer at startup and remain fixed for the session lifetime. |

## Resource Management

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.3 | Transient Resource Declaration | Passes declare virtual resource handles with format, dimensions, and usage flags. The graph compiler maps virtual handles to physical allocations, allowing resources that do not overlap in lifetime to share the same backing memory. | R-2.2.3 | F-2.2.1, F-2.1.7 | Mobile: transient resources use memoryless storage on Metal (MTLStorageModeMemoryless) and lazily-allocated memory on Vulkan (VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT) for zero-bandwidth tile-local attachments. All platforms: full quality. |
| F-2.2.4 | Resource Aliasing | Automatic memory aliasing of transient resources whose lifetimes do not overlap within a frame. The compiler builds an interference graph from pass execution order and assigns non-overlapping resources to the same physical memory block, reducing peak VRAM consumption. | R-2.2.4 | F-2.2.3 | D3D12 uses placed resources in heaps. Vulkan uses memory aliasing with explicit invalidation. Metal relies on MTLHeap makeAliasable. |

## Barriers and Synchronization

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.5 | Automatic Barrier Insertion | The graph compiler analyzes resource read/write sets across passes and inserts the minimal set of barriers required for correct execution. Barriers are placed at the latest possible point and split across passes when the backend supports split barriers. | R-2.2.5 | F-2.2.3, F-2.1.9 | Metal relies on driver-level hazard tracking; the compiler emits fences only at queue boundaries. |

## Queue Assignment and Scheduling

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.6 | Multi-Queue Scheduling | The compiler assigns passes to graphics, compute, and copy queues based on declared workload type and dependency constraints. Cross-queue synchronization fences are inserted automatically. Async compute passes overlap with graphics work to maximize GPU utilization. | R-2.2.6 | F-2.2.1, F-2.2.5 | Queue family selection is backend-specific. Metal uses shared and private command queues. D3D12 and Vulkan use explicit queue families. |
| F-2.2.7 | Pass Ordering and Topological Sort | Passes are topologically sorted by resource dependencies to determine execution order. Passes with no data dependency between them are candidates for parallel encoding or async queue assignment. Stable ordering is maintained across frames to avoid GPU pipeline bubbles. | R-2.2.7 | F-2.2.1 | All platforms: full quality. CPU-side operation with no GPU cost. |
| F-2.2.8 | Budget Culling | The graph compiler assigns each pass a cost estimate based on historical GPU timing data. When the total estimated frame cost exceeds the target budget, lowest-priority passes are culled from the graph before compilation, enabling graceful quality degradation at scale. | R-2.2.8 | F-2.2.1, F-2.2.7 | Mobile: aggressive budget targets (16-33 ms); many optional passes culled by default. Switch: 16 ms docked (60 fps) / 33 ms handheld (30 fps) budgets. Desktop: 16 ms target. High-end: 8-16 ms target allowing more optional passes. |

## Execution

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.9 | Multi-View Execution | A single logical render graph is instantiated multiple times with per-view parameter overrides for split-screen, VR stereo, shadow cascades, and reflection probe renders. Shared passes (e.g., culling, lighting) execute once and fan out to view-specific passes. | R-2.2.9 | F-2.2.1, F-2.2.7 | Mobile: max 4 views (main + 2-3 shadow cascades); no VR stereo. Switch: max 8 views. Desktop: configurable; dozens of concurrent views. High-end: unlimited views; VR stereo with single-pass instanced rendering. |
| F-2.2.10 | Parallel Command Encoding | Independent passes within the same queue are encoded on separate threads using secondary command buffers. The graph compiler identifies parallelism from the dependency graph and distributes encoding work across a thread pool, reducing CPU submission latency. | R-2.2.10 | F-2.2.7, F-2.1.2 | D3D12 uses command list bundles. Vulkan uses secondary command buffers. Metal uses parallel render command encoders. |
| F-2.2.11 | Streaming Integration | Passes that depend on streamed resources (textures, meshes, acceleration structures) declare fallback behavior for unavailable assets. The graph compiler substitutes placeholder resources or skips dependent passes until streaming completes, avoiding stalls during world traversal. | R-2.2.11 | F-2.2.3 | Mobile: aggressive streaming pool limits (256-512 MB); more frequent fallback to placeholders. Switch: 1 GB streaming pool. Desktop: 2-4 GB pool. High-end: 8+ GB pool with acceleration structure streaming. |

## Graph Compilation and Diagnostics

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.12 | Graph Compilation and Caching | The full render graph is compiled once when the pass set or device capabilities change. The compiled graph encodes barrier placement, resource aliasing, queue assignment, and execution order into a flat command schedule. Recompilation is triggered only by topology changes, not per-frame parameter updates. | R-2.2.12 | F-2.2.5, F-2.2.6, F-2.2.7, F-2.2.4 | All platforms: full quality. Compiled graph topology is simpler on mobile/Switch due to fewer active passes (many optional passes pruned). |
| F-2.2.13 | Render Graph Diagnostics | Runtime diagnostic overlay and logging that visualizes the compiled graph as a DAG with per-pass GPU timing, resource lifetimes, barrier placement, and queue assignment. Captured frame graphs can be exported for offline analysis. | R-2.2.13 | F-2.2.12 | All platforms: full quality. Debug overlay disabled in shipping builds. Mobile profiling uses Metal GPU Capture or Vulkan validation layers. |

## Unified Game Loop Integration

| ID | Feature | Description | Requirements | Dependencies | Platform Notes |
|----|---------|-------------|--------------|--------------|----------------|
| F-2.2.14 | Render Graph as Task Graph Phase | Render passes compile into `TaskNode` entries within the unified game loop graph (F-14.3.14). The render graph compiler emits nodes that the parent `TaskGraph` schedules alongside ECS systems, physics, audio, and input phases. Render node dependencies on CPU-side work (culling, scene traversal) are expressed as typed edges in the game loop graph, enabling the scheduler to overlap CPU rendering prep with GPU submission from the previous phase. | R-2.2.14 | F-2.2.12, F-14.3.14, F-14.3.3 | None. Task graph integration is a CPU-side scheduling concern with no GPU-specific behavior. |
| F-2.2.15 | Safe GPU Command Encoding | GPU command buffers are borrowed via scoped APIs that enforce encoding lifetime at compile time. A command buffer reference cannot outlive the encoding scope that created it, preventing use-after-submit and dangling command buffer references. Scoped encoding uses the same lifetime mechanism as scoped task execution (F-14.3.8), tying command buffer validity to the render pass node's execution scope within the task graph. | R-2.2.15 | F-2.2.14, F-2.1.2, F-14.3.8 | None. Encoding scope safety is enforced by Rust lifetimes at compile time with no runtime cost. |
