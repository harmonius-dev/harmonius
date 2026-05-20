# 2.1 — GPU Abstraction Layer

## Backend Trait Interface

| ID      | Feature                    |
|---------|----------------------------|
| F-2.1.1 | GPU Backend Trait          |
| F-2.1.2 | Command Buffer Abstraction |
| F-2.1.3 | Pipeline State Abstraction |

1. **F-2.1.1** — Defines the top-level Rust trait that all GPU backends implement, using associated
   types for device, command buffer, pipeline state, and resource handles. Static dispatch via
   generics eliminates vtable overhead across the entire rendering pipeline.
   - **Platform:** The Vulkan backend provides a concrete type that satisfies the trait bounds.
2. **F-2.1.2** — Trait-based command buffer encoding supporting graphics, compute, and copy
   operations with type-safe resource binding. Command buffers are recorded independently and
   submitted as ordered batches to a queue.
   - **Deps:** F-2.1.1
   - **Platform:** Maps to `VkCommandBuffer` via `ash`.
3. **F-2.1.3** — Unified pipeline state object covering shader stages, vertex layout, blend state,
   depth-stencil configuration, and rasterizer settings. The abstraction pre-validates combinations
   at creation time so that command buffer encoding is zero-cost.
   - **Deps:** F-2.1.1
   - **Platform:** Pipeline state cache warmed at load time to avoid hitching on tile-based GPUs
     where PSO creation is expensive.

## Vulkan Backend

| ID      | Feature                               |
|---------|---------------------------------------|
| F-2.1.4 | Vulkan Backend (`ash`)                |

1. **F-2.1.4** — Vulkan GPU backend implemented via the `ash` Rust crate on all platforms. Vulkan
   types and functions are accessed through safe Rust wrappers that implement the backend trait.
   - **Deps:** F-2.1.1
   - **Platform:** Windows, macOS, Linux, iOS, and Android.

## GPU Runtime

| ID       | Feature                               |
|----------|---------------------------------------|
| F-2.1.7  | Memory Sub-Allocation                 |
| F-2.1.8  | GPU State Tracking                    |
| F-2.1.9  | Barrier Optimization                  |
| F-2.1.10 | Work Graph Support                    |
| F-2.1.11 | Cross-Backend Feature Emulation       |
| F-2.1.12 | GPU Performance Queries and Profiling |

1. **F-2.1.7** — GPU heap sub-allocator that carves typed buffer and texture regions from large
   pre-allocated memory blocks. Reduces the number of OS-level allocations from thousands to a
   handful per frame, supporting offset-based binding for per-draw constant data at MMO-scale draw
   counts.
   - **Deps:** F-2.1.1
   - **Platform:** Allocation strategy respects Vulkan alignment requirements (256 B for uniform
     buffers, variable for storage, page-aligned for mapped staging).
2. **F-2.1.8** — CPU-side shadow state tracker that records current pipeline, binding, and render
   target state per command buffer. Redundant state transitions are filtered before encoding,
   minimizing driver overhead during high-frequency draw submission.
   - **Deps:** F-2.1.2
   - **Platform:** Especially important on mobile where driver overhead per state change is
     proportionally higher due to lower CPU clocks.
3. **F-2.1.9** — Automatic resource barrier batching and reordering within a command buffer.
   Consecutive barriers targeting the same resource are merged, and split barriers are inserted when
   a resource transition can overlap with independent work, reducing GPU pipeline stalls.
   - **Deps:** F-2.1.8
   - **Platform:** Maps to `VkMemoryBarrier2` / `VkBufferMemoryBarrier2` / `VkImageMemoryBarrier2`.
4. **F-2.1.10** — Backend-level support for GPU work graphs enabling GPU-driven dispatch without CPU
   round-trips. Work graph nodes are declared as shader entry points with input/output records, and
   the runtime manages backing memory and dispatch.
   - **Deps:** F-2.1.1, F-2.1.7
   - **Platform:** Native when `VK_KHR_work_graphs` (or successor) is available; otherwise
     compute-based emulation via indirect dispatch chains.
5. **F-2.1.11** — Runtime feature emulation layer that provides consistent API behavior when a
   device lacks native support for a capability. Emulated paths are selected at device creation
   time based on capability queries, with no runtime branching in the hot path.
   - **Deps:** F-2.1.1
   - **Platform:** Emulated features include work graphs on older drivers and mesh shaders when
     `VK_EXT_mesh_shader` is unavailable.
6. **F-2.1.12** — Hardware timestamp queries, pipeline statistics, and occupancy counters exposed
   through the GPU abstraction layer. Each render pass can be bracketed with begin/end timestamp
   queries whose results are read back one frame later to avoid stalls. The profiling API reports
   per-pass GPU time, vertex/fragment invocation counts, and bandwidth consumption. Results feed
   into the editor's GPU profiler (F-15.5.3) and the render graph's diagnostic overlay (F-2.2.13).
   - **Deps:** F-2.1.1
   - **Platform:** `vkCmdWriteTimestamp` with `VK_QUERY_TYPE_TIMESTAMP` and
     `VK_QUERY_TYPE_PIPELINE_STATISTICS`.

## Unified Game Loop Integration

| ID       | Feature                   |
|----------|---------------------------|
| F-2.1.13 | CPU Work Graph Emulation  |
| F-2.1.14 | Safe GPU Resource Handles |
| F-2.1.15 | Shader Compilation Pipeline |

1. **F-2.1.13** — CPU-side emulation of GPU work graphs via indirect dispatch chains in the task
   graph. Producer nodes enqueue work items into intermediate buffers; consumer nodes read from
   those buffers and dispatch additional GPU work via `vkCmdDispatchIndirect`. The emulation path
   integrates with the unified game loop graph (F-14.3.14) as task graph nodes.
   - **Deps:** F-2.1.10, F-2.1.11, F-14.3.14
   - **Platform:** Native work graphs when supported; otherwise `vkCmdDispatchIndirect` emulation.
2. **F-2.1.14** — All GPU resources (buffers, textures, samplers, pipeline states, acceleration
   structures) are referenced through generational `Handle<T>` indices. Each handle contains a
   type-safe index and a generation counter that detects use-after-free at runtime. No raw GPU
   pointers appear in any public API type.
   - **Deps:** F-2.1.1, F-2.1.7
   - **Platform:** Generational handles are a Rust-side abstraction. The Vulkan backend translates
     handles to `VkBuffer`, `VkImage`, and related Vulkan types internally.
3. **F-2.1.15** — GLSL shader sources are compiled to SPIR-V artifacts via the `naga` CLI
   subprocess during asset processing. Compiled artifacts are cached keyed by source hash, include
   set, and target profile. Shipping builds ship precompiled SPIR-V and never invoke shader
   compilers at runtime.
   - **Deps:** F-2.1.1, F-2.1.3
   - **Platform:** All platforms consume SPIR-V produced by `naga`. Compilers run as CLI
     subprocesses; no runtime compilation in shipping builds.
