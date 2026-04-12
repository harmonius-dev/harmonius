# Rendering Core — Test Cases

Companion to [rendering-core.md](rendering-core.md).

Test case IDs use `TC-2.3.Z.N` and `TC-2.10.Z.N` format. Every row links to a specific R-2.3.Z or
R-2.10.Z.

## Unit Tests

| ID            | Name                                  | Req       |
|---------------|---------------------------------------|-----------|
| TC-2.3.1.1    | `test_unified_light_buffer_write`     | R-2.3.1   |
| TC-2.3.1.2    | `test_forward_deferred_match`         | R-2.3.1   |
| TC-2.3.2.1    | `test_meshlet_frustum_cull`           | R-2.3.2   |
| TC-2.3.2.2    | `test_meshlet_offscreen_excluded`     | R-2.3.2   |
| TC-2.3.3.1    | `test_normal_cone_backface_cull`      | R-2.3.3   |
| TC-2.3.4.1    | `test_hzb_phase1_test`                | R-2.3.4   |
| TC-2.3.4.2    | `test_hzb_phase2_recover`             | R-2.3.4   |
| TC-2.3.5.1    | `test_orthographic_projection_no_fov` | R-2.3.5   |
| TC-2.3.6.1    | `test_reverse_z_infinite_far`         | R-2.3.6   |
| TC-2.3.7.1    | `test_indirect_compact_by_material`   | R-2.3.7   |
| TC-2.3.8.1    | `test_rt_barrier_auto_insert`         | R-2.3.8   |
| TC-2.3.9.1    | `test_cubemap_six_faces_seamless`     | R-2.3.9   |
| TC-2.3.11.1   | `test_dynamic_resolution_converge`    | R-2.3.11  |
| TC-2.3.13.1   | `test_alpha_test_threshold_shadow`    | R-2.3.13  |
| TC-2.3.14.1   | `test_mesh_shader_dispatch`           | R-2.3.14  |
| TC-2.10.1.1   | `test_extract_thread_isolation`       | R-2.10.1  |
| TC-2.10.2.1   | `test_proxy_dirty_incremental`        | R-2.10.2  |
| TC-2.10.3.1   | `test_view_register_4_views`          | R-2.10.3  |
| TC-2.10.4.1   | `test_draw_list_sort_by_material`     | R-2.10.4  |
| TC-2.10.4.2   | `test_sort_key_64bit_layout`          | R-2.10.4a |
| TC-2.10.5.1   | `test_bindless_param_buffer`          | R-2.10.5  |
| TC-2.10.7.1   | `test_render_layer_bitmask_filter`    | R-2.10.7  |
| TC-2.10.8.1   | `test_transform_interpolation_alpha`  | R-2.10.8  |
| TC-2.10.9.1   | `test_ring_buffer_3_frames`           | R-2.10.9  |
| TC-2.4.24.1   | `test_light_probe_sh_sample`          | R-2.4.24  |

1. **TC-2.3.1.1** `test_unified_light_buffer_write` — Insert one point, one spot, one directional
   light. Assert all three appear in the unified light buffer with correct types.
   - Input: 3 `Light` entities
   - Expected: `light_buffer.len() == 3`, types `[Point, Spot, Directional]` present

2. **TC-2.3.1.2** `test_forward_deferred_match` — Render same scene with forward and deferred paths.
   Compare framebuffers. Assert PSNR > 40 dB.
   - Input: same scene fixture, two pipelines
   - Expected: PSNR > 40 dB, no per-pixel max delta > 8/255

3. **TC-2.3.2.1** `test_meshlet_frustum_cull` — 1000 meshlets; camera frustum contains 400. Assert
   exactly 400 pass GPU frustum cull (compared to CPU reference).
   - Input: known meshlet positions, fixed camera
   - Expected: GPU cull result == CPU brute-force result (set equality)

4. **TC-2.3.2.2** `test_meshlet_offscreen_excluded` — Camera moved so 50% of meshlets are
   off-screen. Assert excluded meshlets do not appear in indirect draw buffer.
   - Input: same dataset, camera moved
   - Expected: indirect draw count ≈ 50% of total

5. **TC-2.3.3.1** `test_normal_cone_backface_cull` — Sphere meshlets with normal cones; camera
   facing one side. Assert ~50% of meshlets are cone-rejected.
   - Input: tessellated sphere, camera at `+Z`
   - Expected: cone-cull rejects approximately half (within 10%)

6. **TC-2.3.4.1** `test_hzb_phase1_test` — Wall hides 100 meshlets. Phase 1 reads previous-frame
   HZB. Assert all 100 meshlets are rejected by HZB.
   - Input: wall + 100 occluded meshlets, prior frame HZB available
   - Expected: phase-1 visible count == 0 for the 100 occluded meshlets

7. **TC-2.3.4.2** `test_hzb_phase2_recover` — Camera translates to reveal previously hidden
   meshlets. Phase 2 re-tests against current HZB. Assert revealed meshlets appear in same frame.
   - Input: prior HZB hides 100 meshlets; camera reveals them this frame
   - Expected: phase-2 visible count == 100, no pop-in

8. **TC-2.3.5.1** `test_orthographic_projection_no_fov` — Build orthographic projection with
   `(left, right, bottom, top, near, far)`. Project a point at `(1, 0, -10)`. Assert no
   foreshortening: x-coordinate scales linearly with depth.
   - Input: orthographic matrix, point fixture
   - Expected: projected x at depth 10 == projected x at depth 100

9. **TC-2.3.6.1** `test_reverse_z_infinite_far` — Build reverse-Z infinite-far projection. Project
   points at depth 1 m and 10 km. Assert both produce distinct, monotonic NDC z values.
   - Input: reverse-Z infinite far matrix
   - Expected: `ndc_z(1m) > ndc_z(10km)`, both within `[0, 1]`

10. **TC-2.3.7.1** `test_indirect_compact_by_material` — 10000 instances across 5 materials. Compact
    into indirect draws. Assert exactly 5 indirect draw groups produced.
    - Input: 10000 instances, 5 materials
    - Expected: `indirect_groups.len() == 5`, total instance count == 10000

11. **TC-2.3.8.1** `test_rt_barrier_auto_insert` — Pass A writes texture T; pass B samples T.
    Compile graph. Assert a `ColorAttachment → ShaderRead` barrier is inserted between them.
    - Input: two passes sharing T
    - Expected: barrier list contains exactly one transition for T

12. **TC-2.3.9.1** `test_cubemap_six_faces_seamless` — Render six cubemap faces. Sample at face
    boundary. Assert no visible seam (color delta < 1/255 across boundary).
    - Input: dynamic cubemap, scene fixture
    - Expected: edge sample delta < 1/255 per channel

13. **TC-2.3.11.1** `test_dynamic_resolution_converge` — Spike GPU cost above target; tick the
    feedback loop 60 times. Assert resolution converges and stays within `[min, max]` bounds.
    - Input: target 16 ms, actual 22 ms initially, bounds `[0.5, 1.0]`
    - Expected: converged scale within bounds, oscillation amplitude < 0.05 after warmup

14. **TC-2.3.13.1** `test_alpha_test_threshold_shadow` — Alpha-test foliage with threshold 0.5.
    Render shadow map. Assert shadow silhouette matches alpha cutout, not opaque bounds.
    - Input: foliage texture with alpha values
    - Expected: shadow pixels missing where `alpha < 0.5`

15. **TC-2.3.14.1** `test_mesh_shader_dispatch` — Capability `mesh_shaders = true`. Assert mesh
    shader dispatch path is used; capability `false`, indirect draw fallback used.
    - Input: same scene, two capability sets
    - Expected: mesh shader path emits `dispatch_mesh`; fallback emits `draw_indexed_indirect`

16. **TC-2.10.1.1** `test_extract_thread_isolation` — Run `RenderExtract` while spawning a thread
    that mutates ECS components. Assert extract sees a consistent immutable snapshot.
    - Input: 1000 entities, mutator thread, extract thread
    - Expected: extracted snapshot internally consistent, no torn reads

17. **TC-2.10.2.1** `test_proxy_dirty_incremental` — 10000 proxies; mark 10 dirty; run incremental
    upload. Assert only 10 proxies are written to the GPU buffer.
    - Input: 10000 entities, 10 dirty
    - Expected: `gpu_writes == 10`

18. **TC-2.10.3.1** `test_view_register_4_views` — Register 4 views (main, shadow, reflection,
    minimap). Assert all 4 are present in the view registry with their projection matrices.
    - Input: 4 `View` registrations
    - Expected: `views.len() == 4`, projections match input

19. **TC-2.10.4.1** `test_draw_list_sort_by_material` — Build draw list with mixed materials. Sort.
    Assert draws of the same material are contiguous in the sorted list.
    - Input: 100 draws with 5 distinct materials interleaved
    - Expected: sorted list groups all material-K draws into runs

20. **TC-2.10.4.2** `test_sort_key_64bit_layout` — Encode sort key from
    `(translucency, phase, pipeline, material, depth)`. Decode. Assert all fields round-trip
    exactly.
    - Input: known field values
    - Expected: decoded fields == input; bit allocation matches doc

21. **TC-2.10.5.1** `test_bindless_param_buffer` — Render 1000 draws with bindless material indices.
    Assert all draws share one descriptor set; per-draw param indices distinct.
    - Input: 1000 draws, bindless material descriptor
    - Expected: descriptor set switches == 0, param buffer length == 1000

22. **TC-2.10.7.1** `test_render_layer_bitmask_filter` — Entity layer `bit 2`, camera mask `bit 2`.
    Assert visible. Change camera mask to `bit 3`. Assert invisible.
    - Input: entity + camera with configurable mask
    - Expected: visibility matches mask overlap (`mask_a & mask_b != 0`)

23. **TC-2.10.8.1** `test_transform_interpolation_alpha` — Previous transform at `(0, 0, 0)`,
    current at `(10, 0, 0)`. Alpha 0.5. Assert interpolated position is `(5, 0, 0)`.
    - Input: two transforms, alpha 0.5
    - Expected: `lerp_pos == Vec3(5, 0, 0)`

24. **TC-2.10.9.1** `test_ring_buffer_3_frames` — Ring-buffer per-frame uniform allocator with 3
    frames in flight. Allocate frame 0, 1, 2, 3. Assert frame 3 reuses frame 0's slot.
    - Input: ring of size 3, 4 frames
    - Expected: `slot(0) == slot(3)`, no overlap with current GPU-in-flight slot

25. **TC-2.4.24.1** `test_light_probe_sh_sample` — Place SH irradiance probe; sample from a nearby
    surface point. Assert sampled irradiance matches the probe's stored SH coefficients.
    - Input: known SH coefficients, sample direction
    - Expected: sampled value matches analytic SH evaluation within tolerance

## Integration Tests

| ID           | Name                            | Req       |
|--------------|---------------------------------|-----------|
| TC-2.3.I.1   | `test_100k_meshlets_render`     | R-2.3.7   |
| TC-2.3.I.2   | `test_dynamic_res_stress`       | R-2.3.11  |
| TC-2.3.I.3   | `test_cubemap_reflection_probe` | R-2.3.9   |
| TC-2.10.I.1  | `test_extract_simulate_overlap` | R-2.10.1  |
| TC-2.10.I.2  | `test_split_screen_2_views`     | R-2.10.3  |
| TC-2.10.I.3  | `test_100k_proxy_incremental`   | R-2.10.2  |
| TC-2.10.I.4  | `test_render_layer_split_view`  | R-2.10.7  |

1. **TC-2.3.I.1** `test_100k_meshlets_render` — Render 100k meshlet instances with cull, compact,
   and indirect draw. Assert frame succeeds and draw call count < 200.
   - Input: 100k meshlet scene fixture
   - Expected: `draw_calls < 200`, no GPU validation errors

2. **TC-2.3.I.2** `test_dynamic_res_stress` — Run a heavy stress scene for 5 s. Assert dynamic
   resolution converges and frame time stays within 10% of target.
   - Input: stress scene, 5 s wall time
   - Expected: per-frame time within `[14.4, 17.6] ms` (target 16)

3. **TC-2.3.I.3** `test_cubemap_reflection_probe` — Render dynamic cubemap reflection probe in a
   scene with moving objects. Assert reflections update each frame; no stale faces.
   - Input: scene with moving cube + reflection probe
   - Expected: probe content changes per frame, all 6 faces written

4. **TC-2.10.I.1** `test_extract_simulate_overlap` — Profile a frame. Assert simulation systems
   continue executing while `RenderExtract` runs on a separate thread.
   - Input: simulation + extract on separate threads
   - Expected: GPU trace shows overlap > 70% of extract duration

5. **TC-2.10.I.2** `test_split_screen_2_views` — Two cameras with side-by-side viewports. Assert
   both render correctly from a single extracted snapshot.
   - Input: 2 `Camera` entities, shared scene
   - Expected: framebuffer has 2 distinct viewports populated, no duplicate extract

6. **TC-2.10.I.3** `test_100k_proxy_incremental` — 100k proxies with 50 dirty per frame for 1000
   frames. Assert per-frame upload bandwidth is proportional to dirty count, not total count.
   - Input: 100k entities, 50 mutated each frame
   - Expected: average bandwidth per frame ≈ `50 * sizeof(Proxy)`

7. **TC-2.10.I.4** `test_render_layer_split_view` — Player 1 camera mask `bit 0`; Player 2 mask
   `bit 1`. Entities tagged per player. Assert each viewport sees only its tagged entities.
   - Input: split-screen setup, layer-tagged entities
   - Expected: per-viewport rendered counts match per-player entity counts

## Benchmarks

| ID           | Benchmark                       | Target    | Req      |
|--------------|---------------------------------|-----------|----------|
| TC-2.3.B.1   | Meshlet cull (100k meshlets)    | < 0.8 ms  | R-2.3.2  |
| TC-2.3.B.2   | HZB build (1080p)               | < 0.4 ms  | R-2.3.4  |
| TC-2.3.B.3   | Indirect compact (10k draws)    | < 0.3 ms  | R-2.3.7  |
| TC-2.10.B.1  | Render extract (10k entities)   | < 1.0 ms  | R-2.10.1 |
| TC-2.10.B.2  | Draw list sort (10k draws)      | < 0.5 ms  | R-2.10.4 |
| TC-2.10.B.3  | Proxy incremental (50 dirty)    | < 0.1 ms  | R-2.10.2 |

1. **TC-2.3.B.1** — GPU compute pass culling 100k meshlets via frustum + normal cone tests. Wall
   time on reference GPU.

2. **TC-2.3.B.2** — Hierarchical Z-buffer construction at 1080p (full mip chain). Wall time measured
   via GPU timestamps.

3. **TC-2.3.B.3** — Indirect draw compaction over 10k surviving draws into per-material indirect
   buffers. Wall time.

4. **TC-2.10.B.1** — Run `RenderExtract` over 10k entities with `Renderable` queries. Wall time on a
   single extraction thread.

5. **TC-2.10.B.2** — Radix-sort 10k draw entries by 64-bit sort key. Wall time including key
   construction.

6. **TC-2.10.B.3** — Incremental proxy update with 50 dirty proxies out of 100k. Wall time for the
   upload-staging step only.
