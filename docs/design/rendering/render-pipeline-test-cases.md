# Render Pipeline — Test Cases

Companion to [render-pipeline.md](render-pipeline.md).

## `PLAN-rendering-render-pipeline` milestone

TC-2.1.4.3, TC-2.1.5.1, and TC-2.1.6.1 are satisfied by CPU stubs (`MetalBackendStub`,
`D3D12BackendStub`, `VulkanValidationStub`) that keep the bootstrap API stable. Native Metal, D3D12,
and Vulkan validation integration is deferred; numbered items below stay the long-term target.

Test case IDs use `TC-2.2.Z.N` and `TC-2.1.Z.N` format. Every row links to a specific R-2.2.Z,
R-2.1.Z, or GR-X.Z (GPU runtime).

## Unit Tests

| ID            | Name                                 | Req       |
|---------------|--------------------------------------|-----------|
| TC-2.2.1.1    | `test_pass_register_topology`        | R-2.2.1   |
| TC-2.2.1.2    | `test_pass_read_write_dependency`    | R-2.2.1   |
| TC-2.2.2.1    | `test_capability_gate_prune_rt`      | R-2.2.2   |
| TC-2.2.2.2    | `test_fallback_pass_substitution`    | R-2.2.2   |
| TC-2.2.3.1    | `test_virtual_resource_alias`        | R-2.2.3   |
| TC-2.2.3.2    | `test_resource_lifetime_overlap`     | R-2.2.3   |
| TC-2.2.3.3    | `test_aliasing_vram_savings_40pct`   | R-2.2.3a  |
| TC-2.2.4.1    | `test_minimal_barrier_chain`         | R-2.2.4   |
| TC-2.2.4.2    | `test_split_barrier_overlap`         | R-2.2.4   |
| TC-2.2.5.1    | `test_async_compute_queue_assign`    | R-2.2.5   |
| TC-2.2.5.2    | `test_cross_queue_fence_insert`      | R-2.2.5   |
| TC-2.2.6.1    | `test_budget_cull_low_priority`      | R-2.2.6   |
| TC-2.2.7.1    | `test_multi_view_share_culling`      | R-2.2.7   |
| TC-2.2.8.1    | `test_parallel_encode_4_threads`     | R-2.2.8   |
| TC-2.2.9.1    | `test_streaming_placeholder_sub`     | R-2.2.9   |
| TC-2.2.10.1   | `test_graph_recompile_on_change`     | R-2.2.10  |
| TC-2.2.10.2   | `test_graph_no_recompile_param_only` | R-2.2.10  |
| TC-2.2.13.1   | `test_command_buffer_scope_lifetime` | R-2.2.13  |
| TC-2.1.1.1    | `test_unified_alloc_o1`              | GR-1.2    |
| TC-2.1.1.2    | `test_dedicated_heap_alloc`          | GR-1.3    |
| TC-2.1.1.3    | `test_ring_buffer_no_heap_alloc`     | GR-1.5    |
| TC-2.1.2.1    | `test_state_cache_skip_redundant`    | GR-2.1    |
| TC-2.1.2.2    | `test_pipeline_bind_cache`           | GR-2.2    |
| TC-2.1.4.1    | `test_barrier_elide_same_state`      | GR-4.4    |
| TC-2.1.4.2    | `test_traceray_compute_fallback`     | GR-4.6    |
| TC-2.1.1.4    | `test_static_dispatch_no_vtable`     | R-2.1.1   |
| TC-2.1.2.3    | `test_cmd_buf_graphics_compute_copy` | R-2.1.2   |
| TC-2.1.3.1    | `test_pso_invalid_combination`       | R-2.1.3   |
| TC-2.1.4.3    | `test_vulkan_backend_ash_init`      | R-2.1.4   |
| TC-2.1.4.4    | `test_vulkan_validation_zero_errors` | R-2.1.4   |
| TC-2.1.7.1    | `test_heap_suballoc_alignment`       | R-2.1.7   |
| TC-2.1.8.1    | `test_state_tracker_redundant_bind`  | R-2.1.8   |
| TC-2.1.9.1    | `test_barrier_batching_merge`        | R-2.1.9   |
| TC-2.1.10.1   | `test_work_graph_native_dispatch`    | R-2.1.10  |
| TC-2.1.11.1   | `test_cross_backend_emulation`       | R-2.1.11  |
| TC-2.1.12.1   | `test_gpu_perf_query_resolve`        | R-2.1.12  |

1. **TC-2.2.1.1** `test_pass_register_topology` — Register two passes A (writes X) and B (reads X)
   in arbitrary order. Compile graph. Assert execution order is `[A, B]`.
   - Input: `Graph::add(A); Graph::add(B); Graph::compile()`
   - Expected: `compiled.order == [A, B]`, no manual ordering required

2. **TC-2.2.1.2** `test_pass_read_write_dependency` — Three passes A→B→C with chained reads/writes.
   Assert topological order is unique and matches the chain.
   - Input: A writes X; B reads X writes Y; C reads Y writes Z
   - Expected: `order == [A, B, C]`

3. **TC-2.2.2.1** `test_capability_gate_prune_rt` — Register pass requiring `RayTracing`. Compile
   for device without RT. Assert pass is pruned and graph compiles successfully.
   - Input: `Pass { caps: RayTracing }`, `Capabilities { ray_tracing: false }`
   - Expected: pass absent from compiled order, no error

4. **TC-2.2.2.2** `test_fallback_pass_substitution` — Register RT pass with registered fallback.
   Compile for non-RT device. Assert fallback is substituted in the compiled order.
   - Input: `Pass::with_fallback(rt_pass, fallback_pass)`
   - Expected: `compiled.order` contains `fallback_pass`, not `rt_pass`

5. **TC-2.2.3.1** `test_virtual_resource_alias` — Two transient resources with non-overlapping
   lifetimes; assert they map to the same physical allocation.
   - Input: `R1` lifetime `[0..2]`, `R2` lifetime `[3..5]`
   - Expected: `physical_for(R1) == physical_for(R2)`

6. **TC-2.2.3.2** `test_resource_lifetime_overlap` — Two transient resources with overlapping
   lifetimes; assert they get distinct physical allocations.
   - Input: `R1` lifetime `[0..3]`, `R2` lifetime `[2..5]`
   - Expected: `physical_for(R1) != physical_for(R2)`

7. **TC-2.2.3.3** `test_aliasing_vram_savings_40pct` — Compile graph with 10 transient targets.
   Compare aliased VRAM versus naive sum. Assert savings ≥ 40%.
   - Input: 10 transient resources with mixed lifetimes (typical render graph fixture)
   - Expected: `1 - aliased_size / naive_size >= 0.40`

8. **TC-2.2.4.1** `test_minimal_barrier_chain` — Three passes share a single texture in
   write→read→read pattern. Assert exactly one barrier is inserted between write and first read.
   - Input: A writes T; B reads T; C reads T
   - Expected: `barriers.len() == 1`, between A and B; none between B and C

9. **TC-2.2.4.2** `test_split_barrier_overlap` — Pass A writes T; passes B and C run in parallel,
   neither touches T; pass D reads T. Assert split barrier issued at A end and resolved at D.
   - Input: A→[B,C parallel]→D, only A and D touch T
   - Expected: `begin_barrier(A_end)`, `end_barrier(D_begin)`

10. **TC-2.2.5.1** `test_async_compute_queue_assign` — Pass declares `Queue::Compute`. Graphics pass
    consumes its output. Assert compute pass scheduled on async compute queue.
    - Input: compute pass + graphics pass with shared resource
    - Expected: compute pass `queue == AsyncCompute`, graphics pass `queue == Graphics`

11. **TC-2.2.5.2** `test_cross_queue_fence_insert` — Async compute writes texture; graphics reads
    it. Assert a queue fence is inserted between submission groups.
    - Input: same as TC-2.2.5.1
    - Expected: `fences.len() == 1`, signal on compute queue, wait on graphics queue

12. **TC-2.2.6.1** `test_budget_cull_low_priority` — Register passes with costs `[8, 6, 4, 4]` ms
    and priorities `[1, 1, 0, 0]`. Budget 16 ms. Assert lowest-priority passes culled.
    - Input: 4 passes summing 22 ms, budget 16 ms
    - Expected: priority-0 passes pruned, total cost ≤ 16 ms

13. **TC-2.2.7.1** `test_multi_view_share_culling` — 4 views share one cull pass. Compile graph.
    Assert cull pass executes once and 4 per-view draw passes consume its output.
    - Input: 4 views, shared cull pass and per-view draw passes
    - Expected: `cull_pass.exec_count == 1`, draw passes count `== 4`

14. **TC-2.2.8.1** `test_parallel_encode_4_threads` — Encode eight independent passes on one thread,
    then the same count split across four scoped threads with a start barrier.
    - Input: deterministic CPU work units per pass, no cross-pass dependencies
    - Expected: both paths report eight completed units; parallel path uses four threads (CI-safe)

15. **TC-2.2.9.1** `test_streaming_placeholder_sub` — Pass references texture not yet loaded.
    Compile. Assert placeholder texture substituted; consuming pass not skipped.
    - Input: pass with `streaming_handle` in pending state
    - Expected: physical resource is the placeholder texture, no compile error

16. **TC-2.2.10.1** `test_graph_recompile_on_change` — Compile graph (cache hit count 1). Add a new
    pass. Compile again. Assert recompile occurred (cache hit count still 1, new entry added).
    - Input: graph compile, then `add_pass(new_pass)`, then compile
    - Expected: `recompile_count == 1`, `cache_entries == 2`

17. **TC-2.2.10.2** `test_graph_no_recompile_param_only` — Compile graph; mutate a material
    parameter only. Compile again. Assert cache hit; no recompilation.
    - Input: compile, then param change, then compile
    - Expected: `recompile_count == 0`, `cache_hits == 2`

18. **TC-2.2.13.1** `test_command_buffer_scope_lifetime` — Borrow command buffer in a scope; try to
    use it outside. Assert compile-time error (verified via `trybuild` or doctests).
    - Input: `with_encoder(|enc| { ... })`, attempt to keep `enc` alive after scope
    - Expected: code does not compile (lifetime error)

19. **TC-2.1.1.1** `test_unified_alloc_o1` — Allocate and free 10000 buffers from the unified
    allocator. Assert per-op time bounded (no growth with allocation count).
    - Input: 10000 alternating `alloc(1024)` / `free(handle)` calls
    - Expected: max time per op < 5 µs after warm-up; no fragmentation growth

20. **TC-2.1.1.2** `test_dedicated_heap_alloc` — Allocate a 256 MB texture marked
    `requires_dedicated_heap`. Assert a dedicated heap is created and not shared.
    - Input: `AllocDesc { size: 256 << 20, dedicated: true }`
    - Expected: `heap.allocations.len() == 1`, heap is private to this allocation

21. **TC-2.1.1.3** `test_ring_buffer_no_heap_alloc` — Allocate per-frame staging from the ring 200
    times in a frame. Assert zero heap allocations after warm-up.
    - Input: 200 `ring_alloc(64)` calls in a single frame
    - Expected: `global_alloc_counter == 0`, ring head wraps modulo size

22. **TC-2.1.2.1** `test_state_cache_skip_redundant` — Bind same pipeline twice in a row. Assert
    second bind is skipped (cache hit).
    - Input: `bind(pso_a); bind(pso_a)`
    - Expected: backend `set_pipeline` called once

23. **TC-2.1.2.2** `test_pipeline_bind_cache` — Bind pipeline A then B then A. Assert backend sees
    three distinct binds (A, B, A) — alternating, no false dedup.
    - Input: `bind(A); bind(B); bind(A)`
    - Expected: backend `set_pipeline` called 3 times

24. **TC-2.1.4.1** `test_barrier_elide_same_state` — Issue barrier requesting current state. Assert
    no backend barrier is emitted (elided).
    - Input: resource already in `ShaderRead`, request `ShaderRead`
    - Expected: backend `resource_barrier` not called

25. **TC-2.1.4.2** `test_traceray_compute_fallback` — Capability gate selects compute fallback.
    Submit `TraceRays` call. Assert compute dispatch is issued, not RT pipeline call.
    - Input: device `ray_tracing: false`, code calls `trace_rays(width, height, 1)`
    - Expected: `dispatch(width/8, height/8, 1)` issued via compute

26. **TC-2.1.1.4** `test_static_dispatch_no_vtable` — Confirm `GpuBackend` impls use generics, not
    trait objects. Verify via `cargo asm` or `trybuild` that no `dyn GpuBackend` appears in API.
    - Input: full backend trait surface; compile a fixture using `VulkanBackend` directly
    - Expected: zero `dyn GpuBackend` references in generated symbols; calls inlined

27. **TC-2.1.2.3** `test_cmd_buf_graphics_compute_copy` — Open one command buffer; record a graphics
    pass, a compute dispatch, and a copy. Submit. Assert all three execute on the queue.
    - Input: `CommandBuffer` with `draw`, `dispatch`, `copy_buffer` calls
    - Expected: queue executes all three, GPU validation reports no errors

28. **TC-2.1.3.1** `test_pso_invalid_combination` — Build a PSO with mismatched vertex shader output
    and fragment shader input signatures. Assert pre-validation fails with a typed error.
    - Input: `PipelineDesc { vs, fs }` with incompatible interfaces
    - Expected: `PsoBuildError::SignatureMismatch`, no GPU object created

29. **TC-2.1.4.3** `test_vulkan_backend_ash_init` — Initialize `VulkanBackend` via `ash`,
    enumerate devices, and create a default queue on each supported OS.
    - Input: `VulkanBackend::new(VulkanConfig::default())`
    - Expected: device count >= 1, default queue valid, no validation errors

30. **TC-2.1.4.4** `test_vulkan_validation_zero_errors` — Initialize Vulkan with validation layers,
    create instance + device + queue, run a no-op submit. Assert zero validation errors.
    - Input: `VulkanBackend::new(VulkanConfig { validation: true, .. })`
    - Expected: validation callback recorded 0 messages

31. **TC-2.1.7.1** `test_heap_suballoc_alignment` — Sub-allocate buffers requiring alignments
    `[16, 256, 65536]` from a single heap. Assert each returned offset matches alignment.
    - Input: `heap.suballoc(size, align)` for each alignment
    - Expected: `offset % align == 0` for all three; no overlap between allocations

32. **TC-2.1.8.1** `test_state_tracker_redundant_bind` — Bind the same vertex buffer 100 times.
    Assert the state tracker filters all but the first to the backend.
    - Input: 100 `bind_vertex_buffer(vb, 0)` calls in a row
    - Expected: backend `set_vertex_buffer` invoked exactly once

33. **TC-2.1.9.1** `test_barrier_batching_merge` — Issue 4 barriers on 4 distinct resources at the
    same point. Assert the backend receives a single batched barrier call.
    - Input: 4 resources transition `ShaderRead → RenderTarget`
    - Expected: backend `resource_barrier(&[r1, r2, r3, r4])` invoked once

34. **TC-2.1.10.1** `test_work_graph_native_dispatch` — On a device with native work graph support,
    submit a work graph with 3 nodes. Assert all nodes execute and produce expected outputs.
    - Input: `WorkGraph { nodes: [a, b, c], edges: [(a,b),(b,c)] }`
    - Expected: outputs match reference; on unsupported device, emulated path used

35. **TC-2.1.11.1** `test_cross_backend_emulation` — Run mesh shader pipeline on a backend lacking
    native mesh shaders. Assert compute-emulation path is selected and produces same output.
    - Input: mesh pipeline desc, backend `mesh_shaders: false`
    - Expected: emulated compute path used; output framebuffer matches native within tolerance

36. **TC-2.1.12.1** `test_gpu_perf_query_resolve` — Insert a timestamp query around a draw, submit,
    and resolve. Assert returned timestamps are monotonic and non-zero.
    - Input: `cmd.write_timestamp(q0); cmd.draw(...); cmd.write_timestamp(q1)`
    - Expected: `t1 > t0`, both > 0, units in nanoseconds per backend convention

## Integration Tests

| ID          | Name                            | Req      |
|-------------|---------------------------------|----------|
| TC-2.2.I.1  | `test_full_graph_compile`       | R-2.2.1  |
| TC-2.2.I.2  | `test_aliasing_full_pipeline`   | R-2.2.3a |
| TC-2.2.I.3  | `test_streaming_world_traverse` | R-2.2.9  |
| TC-2.2.I.4  | `test_async_compute_overlap`    | R-2.2.5  |
| TC-2.2.I.5  | `test_diagnostic_overlay`       | R-2.2.11 |
| TC-2.1.I.1  | `test_vulkan_backend_parity`       | GR-3.3   |
| TC-2.1.I.2  | `test_dedicated_alloc_24h`      | GR-1.6   |

1. **TC-2.2.I.1** `test_full_graph_compile` — Compile a 30-pass production graph with shadows,
   GBuffer, lighting, post-processing, and UI. Assert no errors and order is valid.
   - Input: full production render graph fixture
   - Expected: compile succeeds, all dependencies satisfied, capability checks pass

2. **TC-2.2.I.2** `test_aliasing_full_pipeline` — Compile production graph and measure transient
   memory. Assert aliasing achieves at least 40% VRAM savings.
   - Input: same as TC-2.2.I.1
   - Expected: `aliased_vram / naive_vram <= 0.6`

3. **TC-2.2.I.3** `test_streaming_world_traverse` — Travel through streamed world for 30 s. Assert
   no GPU stalls; placeholders substituted for in-flight assets.
   - Input: scripted camera path crossing streaming boundaries
   - Expected: zero GPU idle gaps in trace, no validation errors

4. **TC-2.2.I.4** `test_async_compute_overlap` — Run lighting tile cull on async queue while shadow
   draws on graphics. Assert overlap visible in GPU timeline.
   - Input: split scheduling for cull (compute) vs shadow (graphics)
   - Expected: GPU trace shows overlapping execution windows on both queues

5. **TC-2.2.I.5** `test_diagnostic_overlay` — Enable graph diagnostic overlay; render frame. Assert
   overlay draws DAG with per-pass timings; export DOT file is valid.
   - Input: `enable_graph_overlay()`, single frame
   - Expected: overlay rendered, DOT file parses, timings non-zero

6. **TC-2.1.I.1** `test_vulkan_backend_parity` — Run identical render graph on Vulkan
   backends. Compare hash of final framebuffer. Assert pixel-identical within tolerance.
   - Input: same scene, two backends
   - Expected: PSNR > 50 dB (allowing minor rounding)

7. **TC-2.1.I.2** `test_dedicated_alloc_24h` — Run randomized allocation/free workload for 24 hours
   of simulated frames. Assert no fragmentation collapse, no leaks.
   - Input: 24h worth of randomized GPU memory ops
   - Expected: heap headroom remains > 10%, leak count == 0

## Benchmarks

| ID          | Benchmark                       | Target    | Req     |
|-------------|---------------------------------|-----------|---------|
| TC-2.2.B.1  | Graph compile (30 passes)       | < 2 ms    | R-2.2.10 |
| TC-2.2.B.2  | Barrier compute (30 passes)     | < 0.5 ms  | R-2.2.4 |
| TC-2.2.B.3  | Aliasing color (50 resources)   | < 1 ms    | R-2.2.3a |
| TC-2.2.B.4  | Parallel encode (16 passes)     | < 1.5 ms  | R-2.2.8 |
| TC-2.1.B.1  | Allocator alloc (4 KB)          | < 200 ns  | GR-1.2  |
| TC-2.1.B.2  | State cache miss check          | < 50 ns   | GR-2.1  |

1. **TC-2.2.B.1** — Compile a 30-pass production graph from cold (no cache). Wall time for
   topological sort + barrier insertion + aliasing.

2. **TC-2.2.B.2** — Compute minimal barriers across 30 passes with shared resources. Wall time for
   barrier-insertion phase only.

3. **TC-2.2.B.3** — Run interference-graph coloring for 50 transient resources. Wall time of the
   coloring algorithm.

4. **TC-2.2.B.4** — Encode 16 independent passes on 4 worker threads. Wall time for full submission
   record.

5. **TC-2.1.B.1** — Single 4 KB allocation from the unified allocator (warm cache). Wall time per op
   measured with `criterion`.

6. **TC-2.1.B.2** — One state-cache check that results in a cache miss (different value). Wall time
   of the comparison + update.
