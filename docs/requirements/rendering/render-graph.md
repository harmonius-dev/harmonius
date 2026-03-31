# R-2.2 -- Render Graph Requirements

## Pass Declaration and Resources

1. **R-2.2.1** — The engine **SHALL** allow render passes to be declared as structs implementing a
   pass trait that specifies resource reads, writes, and queue requirements, with the graph compiler
   discovering dependency topology automatically.
   - **Rationale:** Declarative pass registration decouples pass authoring from scheduling, enabling
     automatic dependency resolution.
   - **Verification:** Register two passes with a shared resource. Verify the compiler produces the
     correct execution order without manual ordering.

2. **R-2.2.2** — The engine **SHALL** gate each pass on declared GPU capabilities, pruning passes
   that exceed the device feature set and falling back to registered alternative implementations.
   - **Rationale:** Capability gating ensures the pipeline degrades gracefully across hardware
     generations.
   - **Verification:** Register an RT pass with a non-RT fallback. Run on hardware without RT.
     Verify the RT pass is pruned and the fallback executes.

3. **R-2.2.3** — The engine **SHALL** allow passes to declare virtual resource handles with format,
   dimensions, and usage flags, mapping them to physical allocations that share backing memory when
   lifetimes do not overlap.
   - **Rationale:** Virtual resources with aliasing minimize VRAM consumption without manual
     management.
   - **Verification:** Declare two non-overlapping transient resources. Verify they share the same
     physical memory. On mobile, verify memoryless storage is used.

## Synchronization and Scheduling

4. **R-2.2.4** — The engine **SHALL** insert the minimal barrier set between passes based on
   resource read/write analysis, using split barriers when transitions overlap independent work.
   - **Rationale:** Minimal barriers reduce GPU stalls while guaranteeing correctness.
   - **Verification:** Compile a graph with three sequential passes sharing a resource. Verify
     barrier count equals the theoretical minimum. On Metal, verify fences only at queue boundaries.

5. **R-2.2.5** — The engine **SHALL** assign passes to graphics, compute, and copy queues with
   automatic cross-queue fence insertion and topological ordering stable across frames.
   - **Rationale:** Multi-queue scheduling maximizes GPU utilization; stable ordering avoids
     pipeline bubbles.
   - **Verification:** Register a compute pass and graphics pass with shared output. Verify the
     compute pass runs on the async compute queue with a fence before the graphics consumer.

6. **R-2.2.6** — The engine **SHALL** cull lowest-priority passes when total estimated frame cost
   exceeds the target budget, using historical GPU timing for cost estimates.
   - **Rationale:** Budget culling enables graceful quality degradation at runtime.
   - **Verification:** Set a 16 ms budget. Register passes totaling 20 ms. Verify the
     lowest-priority passes are culled and total stays under budget.

## Execution and Multi-View

7. **R-2.2.7** — The engine **SHALL** instantiate a single render graph multiple times with per-view
   parameter overrides for split-screen, VR stereo, shadow cascades, and reflection probes, sharing
   computation where possible.
   - **Rationale:** Multi-view execution avoids redundant culling and lighting work across views.
   - **Verification:** Register 4 views sharing a culling pass. Verify culling executes once and
     fans out to 4 per-view draw passes.

8. **R-2.2.8** — The engine **SHALL** encode independent passes on separate threads using secondary
   command buffers to reduce CPU submission latency.
   - **Rationale:** Parallel encoding utilizes available CPU cores during frame submission.
   - **Verification:** Measure encoding latency with 1 and 4 threads. Verify multi-threaded encoding
     is at least 2x faster with 4+ passes.

9. **R-2.2.9** — The engine **SHALL** substitute placeholder resources or skip dependent passes when
   streamed assets are unavailable, preventing GPU stalls during world traversal.
   - **Rationale:** Streaming integration avoids hitches when assets are still in flight.
   - **Verification:** Start a scene with missing streamed textures. Verify placeholder resources
     appear. Verify no GPU idle time in the frame trace.

## Compilation and Diagnostics

10. **R-2.2.10** — The engine **SHALL** compile the render graph once and cache the result,
    recompiling only when the pass set or device capabilities change.
    - **Rationale:** Cached compilation eliminates per-frame graph overhead.
    - **Verification:** Compile the graph. Change a material parameter. Verify no recompilation. Add
      a new pass. Verify recompilation occurs.

11. **R-2.2.11** — The engine **SHALL** provide a diagnostic overlay and export tool visualizing the
    compiled graph as a DAG with per-pass GPU timing, resource lifetimes, and barrier placement.
    - **Rationale:** Visual diagnostics accelerate rendering pipeline debugging.
    - **Verification:** Enable the overlay. Verify per-pass timing matches GPU profiler results.
      Export the graph and verify it opens in the offline analysis tool.

## Task Graph Integration

12. **R-2.2.12** — Render passes **SHALL** compile into task graph nodes (F-14.3.14) scheduled
    alongside ECS systems, physics, audio, and input phases, with CPU-side work overlapping GPU
    submission from the previous phase.
    - **Rationale:** Unified scheduling maximizes CPU-GPU overlap and eliminates idle time between
      phases.
    - **Verification:** Profile a frame with rendering and simulation. Verify CPU extraction
      overlaps with GPU submission from the prior frame.

13. **R-2.2.13** — GPU command buffers **SHALL** be borrowed via scoped APIs enforcing encoding
    lifetime at compile time, preventing use-after-submit.
    - **Rationale:** Compile-time lifetime enforcement eliminates dangling command buffer bugs.
    - **Verification:** Attempt to use a command buffer outside its encoding scope. Verify compile
      failure.
