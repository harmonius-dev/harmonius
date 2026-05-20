# R-2.14 -- GPU Runtime Requirements

> **Scope:** GPU runtime internals — memory management, state tracking, work graph emulation, and
> feature emulation. For the backend trait interface, see
> [gpu-abstraction-layer.md](gpu-abstraction-layer.md). For the design, see
> [design/rendering/render-pipeline.md](../../design/rendering/render-pipeline.md) and
> [design/rendering/pipeline-state-cache.md](../../design/rendering/pipeline-state-cache.md).

The GPU runtime is the shared services layer between the GPU backend interface and consumers such as
the render graph, asset pipeline, and PSO cache. All requirements below are Vulkan 1.4 on top of
`ash`; the runtime exposes Vulkan idioms (descriptor pools, descriptor sets, timeline semaphores)
consistently across all platforms.

This file replaces the prior `gpu-abstraction.md`, which used a non-conforming `GR-*` ID scheme
without Rationale/Verification. The Legacy ID Mapping table at the bottom maps every `GR-N.M`
reference still found in the corpus to the canonical `R-2.14.*` ID.

## Memory Management

| ID         | Requirement                                                                            |
|------------|----------------------------------------------------------------------------------------|
| R-2.14.1.1 | Unified GPU memory allocator built on the GPU backend interface                        |
| R-2.14.1.2 | O(1) allocation and deallocation with bounded fragmentation                            |
| R-2.14.1.3 | Dedicated allocations for resources requiring dedicated memory                         |
| R-2.14.1.4 | Placed allocations within shared heaps for transient resource aliasing                 |
| R-2.14.1.5 | Per-frame staging and constant-buffer ring with zero heap allocations on the hot path  |
| R-2.14.1.6 | Incremental GPU defragmentation without pipeline stalls                                |
| R-2.14.1.7 | Per-heap-type budget querying with error on over-commitment                            |
| R-2.14.1.8 | Automatic memory heap-type selection based on declared usage flags                     |
| R-2.14.1.9 | Queryable metadata (size, alignment, heap, offset, handle) for any active allocation  |
| R-2.14.1.10| Named allocation pools with configurable max capacity and element size class           |
| R-2.14.1.11| Sparse texture tile binding management with batch bind/unbind operations               |

1. **R-2.14.1.1** — The engine **SHALL** provide a unified GPU memory allocator that owns all GPU
   memory across every backend, built entirely on top of the GPU backend interface without any
   third-party allocator.
   - **Rationale:** A single allocator is required for transient aliasing, fragmentation control,
     and budget enforcement; third-party allocators do not understand the render graph.
   - **Verification:** Allocate from every supported heap type and verify usage tracking matches
     the device-reported `VmaTotalStatistics` equivalent within 1 byte.
2. **R-2.14.1.2** — The engine **SHALL** provide O(1) allocation and deallocation with bounded
   external fragmentation, respecting per-resource alignment requirements.
   - **Rationale:** Real-time allocation cannot afford log or linear behavior on the hot path.
   - **Verification:** Allocate and free 100 000 random-sized blocks. Measure p99 allocation
     latency under 1 µs and external fragmentation under 5%.
3. **R-2.14.1.3** — The engine **SHALL** support dedicated heap allocations for resources that
   the device reports as requiring dedicated memory.
   - **Rationale:** `VK_KHR_dedicated_allocation` requires this for some images.
   - **Verification:** Create an image whose memory requirements report
     `VK_MEMORY_DEDICATED_ALLOCATE` and verify the allocator returns a dedicated allocation.
4. **R-2.14.1.4** — The engine **SHALL** support placed allocations within shared heaps so the
   render graph can alias non-overlapping transient resources to the same backing memory.
   - **Rationale:** Memory aliasing reduces VRAM pressure substantially in heavy scenes.
   - **Verification:** Declare two transient resources with non-overlapping lifetimes; verify
     the allocator returns offsets into the same `VkDeviceMemory`.
5. **R-2.14.1.5** — The engine **SHALL** provide per-frame staging and constant-buffer ring
   allocation with zero heap allocations on the hot path.
   - **Rationale:** Per-frame uploads must not allocate; ring buffers are the standard approach.
   - **Verification:** Trace per-frame allocator calls and verify zero `vkAllocateMemory` calls
     after warm-up.
6. **R-2.14.1.6** — The engine **SHALL** perform incremental GPU defragmentation without
   pipeline stalls.
   - **Rationale:** Long-running sessions accumulate fragmentation; stall-the-world compaction
     is unacceptable for interactive applications.
   - **Verification:** Run a 24-hour soak test with random alloc/free cycles; verify
     fragmentation stays bounded and no frame exceeds budget.
7. **R-2.14.1.7** — The engine **SHALL** provide per-heap-type budget querying and usage
   tracking, returning a typed error on over-commitment.
   - **Rationale:** Apps must respond to memory pressure before the device runs out.
   - **Verification:** Set a budget below the device limit; verify allocation past the budget
     returns `EngineError::OutOfMemory` with the requested heap type.
8. **R-2.14.1.8** — The engine **SHALL** select the memory heap type automatically from declared
   usage flags.
   - **Rationale:** Picking heap type by hand is error-prone and platform-dependent.
   - **Verification:** Allocate a vertex buffer with `DEVICE_LOCAL` usage and verify the chosen
     heap is device-local on every supported platform.
9. **R-2.14.1.9** — The engine **SHALL** expose queryable metadata (size, alignment, heap type,
   offset, handle) for any active allocation.
   - **Rationale:** Profilers and the editor need allocation introspection.
   - **Verification:** Query metadata for 10 active allocations; verify every field matches the
     allocator's internal records.
10. **R-2.14.1.10** — The engine **SHALL** support named allocation pools with configurable
    maximum capacity and element size class.
    - **Rationale:** Subsystems (terrain, particles, UI) want partitioned budgets.
    - **Verification:** Create a 64 MiB "particles" pool; verify the budget is enforced when the
      pool fills.
11. **R-2.14.1.11** — The engine **SHALL** manage sparse texture tile bindings with batch
    bind/unbind operations.
    - **Rationale:** Virtual texturing requires efficient sparse residency updates.
    - **Verification:** Bind 1 000 tiles in a single batch and verify the API issues one
      `vkQueueBindSparse` call.

## State Tracking

| ID         | Requirement                                                                              |
|------------|------------------------------------------------------------------------------------------|
| R-2.14.2.1 | Wrapper that suppresses redundant API calls by comparing against cached state            |
| R-2.14.2.2 | Pipeline-binding cache per bind point                                                    |
| R-2.14.2.3 | Bound descriptor-set / descriptor-pool cache                                             |
| R-2.14.2.4 | Viewport, scissor, and blend-constant cache                                              |
| R-2.14.2.5 | Push-constant write cache                                                                |
| R-2.14.2.6 | Per-resource layout/access cache for minimum-barrier computation                         |
| R-2.14.2.7 | All caches reset to invalid on `command_buffer_begin`                                    |

1. **R-2.14.2.1** — The engine **SHALL** wrap the backend command interface with a state-tracking
   layer that suppresses redundant API calls by comparing requested state against cached state.
   - **Rationale:** Drivers do not always elide redundant binds; doing it CPU-side is cheap.
   - **Verification:** Issue 1 000 identical `cmd_bind_pipeline` calls; verify only the first
     reaches the backend.
2. **R-2.14.2.2** — The engine **SHALL** cache currently bound pipeline handles per bind point
   (graphics, compute, ray tracing) and skip redundant binds.
   - **Rationale:** Pipeline switches are expensive on most drivers.
   - **Verification:** Trace `vkCmdBindPipeline` calls; assert the count equals the count of
     unique handles bound in the frame.
3. **R-2.14.2.3** — The engine **SHALL** cache the currently bound descriptor sets per set index
   and skip redundant binds.
   - **Rationale:** Descriptor binds are frequent in bindless and per-material flows.
   - **Verification:** Bind the same descriptor set twice; verify only one
     `vkCmdBindDescriptorSets` reaches the backend.
4. **R-2.14.2.4** — The engine **SHALL** cache viewport, scissor, and blend-constant state and
   suppress identical re-issues.
   - **Rationale:** Dynamic state is common in UI and post-processing.
   - **Verification:** Set the same viewport twice; verify one `vkCmdSetViewport` reaches the
     backend.
5. **R-2.14.2.5** — The engine **SHALL** cache the most recently written push-constant data and
   skip identical writes.
   - **Rationale:** Push constants are written frequently; identical writes are wasted bandwidth.
   - **Verification:** Issue identical push-constant writes 100 times; assert one
     `vkCmdPushConstants` reaches the backend.
6. **R-2.14.2.6** — The engine **SHALL** maintain a per-resource layout and access state cache
   to compute the minimum barrier set between operations.
   - **Rationale:** Conservative barriers are wasteful; tracking enables exact transitions.
   - **Verification:** Compile a graph with three sequential reads of one image; verify zero
     barriers between reads and one barrier on the first write afterwards.
7. **R-2.14.2.7** — The engine **SHALL** reset every cache slot to "invalid" on
   `command_buffer_begin`.
   - **Rationale:** Command buffers are reusable; stale cache entries cause correctness bugs.
   - **Verification:** Begin a recycled command buffer; verify the next bind issues a backend
     call regardless of prior contents.

## Work Graph Runtime

| ID         | Requirement                                                                              |
|------------|------------------------------------------------------------------------------------------|
| R-2.14.3.1 | Native vs CPU-emulated work-graph path selection at runtime                              |
| R-2.14.3.2 | Render-graph plan translates to native GPU work-graph with self-scheduling               |
| R-2.14.3.3 | CPU-emulated path produces results indistinguishable from the native path                |
| R-2.14.3.4 | Single execution API hides path selection                                                |
| R-2.14.3.5 | Cached translated programs reused until the execution plan changes                       |
| R-2.14.3.6 | Per-frame bindings, constants, and activation flags injected before dispatch             |
| R-2.14.3.7 | Timestamp insertion on both native and emulated paths                                    |
| R-2.14.3.8 | Emulated path replicates native synchronization via barriers and timeline semaphores     |
| R-2.14.3.9 | Native work-graph backing memory allocated from the unified allocator                    |

1. **R-2.14.3.1** — The engine **SHALL** select between native GPU work graphs and CPU
   emulation automatically based on declared device capabilities.
   - **Rationale:** Vulkan device support for work graphs is uneven; the runtime must abstract.
   - **Verification:** Run on a device without `VK_NV_device_generated_commands_compute` and
     verify the emulated path is chosen.
2. **R-2.14.3.2** — The engine **SHALL** translate the render-graph execution plan to a GPU
   work-graph program with GPU self-scheduling on capable backends.
   - **Rationale:** GPU-driven dispatch reduces CPU submission overhead.
   - **Verification:** Compile a 100-pass plan and verify the GPU program executes with one
     CPU submission.
3. **R-2.14.3.3** — The engine **SHALL** record CPU-side command buffers for the emulated path
   that produce results indistinguishable from the native path.
   - **Rationale:** Output parity is required for testing and platform consistency.
   - **Verification:** Run the same plan on native and emulated paths; bit-compare attachments.
4. **R-2.14.3.4** — The engine **SHALL** expose a single execution API that accepts a compiled
   plan plus device capabilities and chooses the path internally.
   - **Rationale:** Callers must not branch on backend.
   - **Verification:** Call the execute API on two devices and verify the call site is identical.
5. **R-2.14.3.5** — The engine **SHALL** cache translated work-graph programs and re-translate
   only when the execution plan changes.
   - **Rationale:** Translation is expensive; plan stability is the common case.
   - **Verification:** Submit the same plan ten times and verify translation runs once.
6. **R-2.14.3.6** — The engine **SHALL** inject per-frame resource bindings, constants, and
   activation flags before dispatch.
   - **Rationale:** Plan compilation is amortized; per-frame data is not.
   - **Verification:** Change one constant frame to frame and verify the change reaches the GPU.
7. **R-2.14.3.7** — The engine **SHALL** insert GPU timestamps on both native and emulated
   paths.
   - **Rationale:** Profiling parity is required across paths.
   - **Verification:** Run the profiler on both paths and verify equivalent timestamp coverage.
8. **R-2.14.3.8** — The engine **SHALL** replicate native work-graph synchronization on the
   emulated path using explicit `vkCmdPipelineBarrier2` calls and timeline semaphores.
   - **Rationale:** Correctness must not depend on the native path's implicit synchronization.
   - **Verification:** Run a producer-consumer plan; verify hazards are caught by validation
     layers on both paths.
9. **R-2.14.3.9** — The engine **SHALL** allocate backing memory for native work-graph programs
   from the unified allocator and reuse it across frames.
   - **Rationale:** No separate allocator should manage work-graph memory.
   - **Verification:** Trace `vkAllocateMemory` during native work-graph execution; assert calls
     come from the unified allocator only.

## Feature Emulation

| ID         | Requirement                                                                                  |
|------------|----------------------------------------------------------------------------------------------|
| R-2.14.4.1 | Per-operation native-vs-emulated path selection from device capabilities                     |
| R-2.14.4.2 | Deferred begin-barrier with combined immediate barrier at split-end on unsupported backends  |
| R-2.14.4.3 | Multiple barriers at the same scope merged into one backend call                              |
| R-2.14.4.4 | Barriers elided for resources already in the target state                                     |
| R-2.14.4.5 | Queue-ownership transfer barriers elided on unified-memory architectures                     |
| R-2.14.4.6 | `vkCmdTraceRaysKHR` translated to compute dispatches with inline ray queries when needed      |
| R-2.14.4.7 | SBT replacement with flat material data buffer accessed by bindless indexing                  |
| R-2.14.4.8 | Capability-based selection of native RT vs compute fallback pipeline pairs                   |
| R-2.14.4.9 | Acceleration-structure binding adapted between RT pipeline layout and compute SRV access     |

1. **R-2.14.4.1** — The engine **SHALL** select between native and emulated implementations of
   each feature per operation based on declared device capabilities.
   - **Rationale:** Per-operation selection avoids requiring all-or-nothing device support.
   - **Verification:** Run a feature audit on a partial-RT device and verify mixed
     native/emulated selection occurs.
2. **R-2.14.4.2** — The engine **SHALL** defer the begin half of split barriers and combine the
   end half with the immediate barrier on backends that do not support
   `VK_KHR_synchronization2` split barriers.
   - **Rationale:** Correctness must not depend on optional features.
   - **Verification:** Disable the synchronization2 capability and verify barrier order is
     preserved by validation layers.
3. **R-2.14.4.3** — The engine **SHALL** merge multiple barriers issued at the same
   synchronization scope into a single backend API call.
   - **Rationale:** Multiple `vkCmdPipelineBarrier2` calls at the same scope is wasteful.
   - **Verification:** Issue four barriers at the same scope; verify one
     `vkCmdPipelineBarrier2` reaches the backend.
4. **R-2.14.4.4** — The engine **SHALL** elide barriers for resources already in the target
   state.
   - **Rationale:** Redundant barriers are wasted GPU sync.
   - **Verification:** Issue a no-op transition; verify zero `vkCmdPipelineBarrier2` calls.
5. **R-2.14.4.5** — The engine **SHALL** elide queue-ownership transfer barriers on
   unified-memory architectures (UMA mobile, integrated GPUs).
   - **Rationale:** Ownership transfer is a no-op when memory is shared.
   - **Verification:** Run on an integrated GPU and verify zero ownership-transfer barriers
     in the trace.
6. **R-2.14.4.6** — The engine **SHALL** translate `vkCmdTraceRaysKHR` to compute-shader
   dispatches that use inline ray queries (`SPV_KHR_ray_query`) when dedicated ray-tracing
   pipelines are unavailable.
   - **Rationale:** Inline ray queries are widely supported even when full RT pipelines are not.
   - **Verification:** Disable `VK_KHR_ray_tracing_pipeline` capability; verify trace results
     match the native path within 1 ULP.
7. **R-2.14.4.7** — The engine **SHALL** replace the shader binding table with a flat
   material-data buffer indexed by bindless resource access on the emulated path.
   - **Rationale:** Compute shaders cannot use the SBT; bindless is the standard substitute.
   - **Verification:** Render a 100-material scene and verify bindless lookups produce parity
     output with the SBT path.
8. **R-2.14.4.8** — The engine **SHALL** register and select native RT and compute-fallback
   pipeline pairs by declared capability.
   - **Rationale:** Caller code references one logical pipeline; selection is automatic.
   - **Verification:** Register a pair and run on RT and non-RT devices; verify the selection
     is correct on each.
9. **R-2.14.4.9** — The engine **SHALL** adapt acceleration-structure binding between the RT
   pipeline descriptor layout and the compute-shader resource view layout transparently.
   - **Rationale:** Caller code references one acceleration structure regardless of path.
   - **Verification:** Bind the same acceleration structure on both paths; verify both paths
     render correctly.

## Legacy ID Mapping

The following table maps the legacy `GR-N.M` IDs (prior `gpu-abstraction.md`) to canonical
`R-2.14.*` IDs so existing references in design and test docs can be updated incrementally.

| Legacy | Canonical    |
|--------|--------------|
| GR-1.1  | R-2.14.1.1  |
| GR-1.2  | R-2.14.1.2  |
| GR-1.3  | R-2.14.1.3  |
| GR-1.4  | R-2.14.1.4  |
| GR-1.5  | R-2.14.1.5  |
| GR-1.6  | R-2.14.1.6  |
| GR-1.7  | R-2.14.1.7  |
| GR-1.8  | R-2.14.1.8  |
| GR-1.9  | R-2.14.1.9  |
| GR-1.10 | R-2.14.1.10 |
| GR-1.11 | R-2.14.1.11 |
| GR-2.1  | R-2.14.2.1  |
| GR-2.2  | R-2.14.2.2  |
| GR-2.3  | R-2.14.2.3  |
| GR-2.4  | R-2.14.2.4  |
| GR-2.5  | R-2.14.2.5  |
| GR-2.6  | R-2.14.2.6  |
| GR-2.7  | R-2.14.2.7  |
| GR-3.1  | R-2.14.3.1  |
| GR-3.2  | R-2.14.3.2  |
| GR-3.3  | R-2.14.3.3  |
| GR-3.4  | R-2.14.3.4  |
| GR-3.5  | R-2.14.3.5  |
| GR-3.6  | R-2.14.3.6  |
| GR-3.7  | R-2.14.3.7  |
| GR-3.8  | R-2.14.3.8  |
| GR-3.9  | R-2.14.3.9  |
| GR-4.1  | R-2.14.4.1  |
| GR-4.2  | R-2.14.4.2  |
| GR-4.3  | R-2.14.4.3  |
| GR-4.4  | R-2.14.4.4  |
| GR-4.5  | R-2.14.4.5  |
| GR-4.6  | R-2.14.4.6  |
| GR-4.7  | R-2.14.4.7  |
| GR-4.8  | R-2.14.4.8  |
| GR-4.9  | R-2.14.4.9  |
