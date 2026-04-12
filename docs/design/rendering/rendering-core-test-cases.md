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

### TC-2.3.10.1 Scene Capture Planar and Cubemap Output

| # | Requirement |
|---|-------------|
| 1 | R-2.3.10    |
| 2 | R-2.3.10    |

1. **#1** — Configure `SceneCaptureComponent` with `CaptureMode::Planar`, resolution `(512, 512)`,
   target texture `T`. Run one frame.
   - **Expected:** `T` contains the scene rendered from the capture view; pixel 0 is non-zero.
2. **#2** — Switch to `CaptureMode::Cubemap`, resolution `(256, 256)`. Run one frame.
   - **Expected:** All 6 cubemap faces of `T` are written; each face has non-zero alpha.

### TC-2.3.12.1 Screen-Space Subsurface Scattering Blur

| # | Requirement |
|---|-------------|
| 1 | R-2.3.12    |
| 2 | R-2.3.12    |

1. **#1** — Render a lit sphere with `ShadingModel::Subsurface`, diffuse profile radius 4 mm.
   - **Expected:** Post-SSS output shows a blurred rim on shaded edge; Laplacian energy drops >30%
     vs pre-SSS buffer.
2. **#2** — Disable SSS via shading model switch to `DefaultLit` on same sphere.
   - **Expected:** Post-SSS pass is skipped; output matches raw lit buffer within 1 ULP.

### TC-2.10.6.1 Per-View Draw Lists Isolation

| # | Requirement |
|---|-------------|
| 1 | R-2.10.6    |
| 2 | R-2.10.6    |

1. **#1** — Register 2 views `V0`, `V1` with different frustums. Build draw lists.
   - **Expected:** `V0.draw_list` and `V1.draw_list` are separate `DrawList` instances with distinct
     `view_id` fields.
2. **#2** — Mutate `V0.draw_list.phase` to `Transparent`; inspect `V1`.
   - **Expected:** `V1.draw_list.phase` unchanged; no aliasing.

### TC-2.10.10.1 Buffer Visualization Modes Strip

| # | Requirement |
|---|-------------|
| 1 | R-2.10.10   |
| 2 | R-2.10.10   |

1. **#1** — Debug build with `BufferViz::BaseColor` enabled; render frame.
   - **Expected:** Final framebuffer contains only albedo channel; no lighting applied.
2. **#2** — Ship build (feature flag off) with same call.
   - **Expected:** `buffer_viz` symbol absent from binary; renderer path does not branch on viz.

### TC-2.4.1.1 Cluster Grid Build and Assignment

| # | Requirement |
|---|-------------|
| 1 | R-2.4.1     |
| 2 | R-2.4.1     |

1. **#1** — Build `ClusterGrid` at tile 64 px for a 1920x1080 view with 24 depth slices.
   - **Expected:** `tile_count_x == 30`, `tile_count_y == 17`, `slice_count == 24`.
2. **#2** — Assign 500 random lights to clusters via compute dispatch.
   - **Expected:** Each affected cluster's light list length > 0; total indices <= 500 * clusters
     touched; no out-of-range writes.

### TC-2.4.2.1 Deferred G-Buffer Layout

| # | Requirement |
|---|-------------|
| 1 | R-2.4.2     |
| 2 | R-2.4.2     |

1. **#1** — Bind G-buffer targets for a deferred view; render one lit mesh.
   - **Expected:** MRT0 contains base color, MRT1 normal + roughness, MRT2 metallic + AO, depth
     attachment written.
2. **#2** — Run fullscreen deferred resolve over the G-buffer.
   - **Expected:** Resolved color matches forward+ reference within PSNR > 40 dB.

### TC-2.4.3.1 BRDF Evaluation Energy Conservation

| # | Requirement |
|---|-------------|
| 1 | R-2.4.3     |
| 2 | R-2.4.3     |

1. **#1** — Evaluate GGX BRDF over the upper hemisphere with roughness 0.5, view at zenith.
   - **Expected:** Integrated reflectance <= 1.0 (no energy gain).
2. **#2** — Flip to grazing view angle 85 degrees.
   - **Expected:** Integrated reflectance still <= 1.0; Fresnel term < 1.0 at normal incidence.

### TC-2.4.10.1 Stochastic Light Sampling Convergence

| # | Requirement |
|---|-------------|
| 1 | R-2.4.10    |
| 2 | R-2.4.10    |

1. **#1** — Place 1024 point lights; sample 2 per pixel stochastically for 64 frames with temporal
   accumulation.
   - **Expected:** Converged image PSNR > 35 dB vs ground-truth all-lights solution.
2. **#2** — Sample 1 per pixel for 1 frame (no accumulation).
   - **Expected:** Variance above no-stochastic baseline but mean error < 10%.

### TC-2.4.11.1 Cascaded Shadow Map Split Computation

| # | Requirement |
|---|-------------|
| 1 | R-2.4.11    |
| 2 | R-2.4.11    |

1. **#1** — Build CSM with 4 cascades, near 0.1 m, far 500 m, logarithmic split factor 0.8.
   - **Expected:** Split distances monotonic; ratio (split[i+1] / split[i]) constant within 5%.
2. **#2** — Render shadow for a depth sample at 50 m.
   - **Expected:** Sample selects cascade index 2; shadow atlas tile sampled matches cascade 2.

### TC-2.4.12.1 PCF 4-Tap Filter Output

| # | Requirement |
|---|-------------|
| 1 | R-2.4.12    |
| 2 | R-2.4.12    |

1. **#1** — Sample shadow map with 4-tap PCF at occluder edge pixel.
   - **Expected:** Filter output is fractional (between 0 and 1), not binary.
2. **#2** — Sample deep in shadow.
   - **Expected:** Filter output == 0.0 within 1/255.

### TC-2.4.13.1 GTAO Bent Normal Output

| # | Requirement |
|---|-------------|
| 1 | R-2.4.13    |
| 2 | R-2.4.13    |

1. **#1** — Run GTAO at half-res over a concave corner scene with 16 samples.
   - **Expected:** Occlusion factor at corner < 0.6; flat wall pixels > 0.95.
2. **#2** — Read GTAO bent normal output buffer.
   - **Expected:** Bent normals deviate from surface normal toward unoccluded direction; length 1.

### TC-2.4.14.1 Virtual Shadow Map Page Allocation

| # | Requirement |
|---|-------------|
| 1 | R-2.4.14    |
| 2 | R-2.4.14    |

1. **#1** — Mark 128 pages as required for a camera frustum; request VSM allocation.
   - **Expected:** `allocated_pages == 128`; all allocated pages have unique physical tile IDs.
2. **#2** — Move camera; mark 64 new pages required and 64 stale.
   - **Expected:** 64 stale pages evicted LRU; 64 new pages allocated in freed slots.

### TC-2.4.15.1 Contact Shadow Ray March Steps

| # | Requirement |
|---|-------------|
| 1 | R-2.4.15    |
| 2 | R-2.4.15    |

1. **#1** — Ray march contact shadows with 16 steps, max distance 0.5 m.
   - **Expected:** Self-shadow appears on 0.2 m gap pair; no shadow on 1.0 m gap pair.
2. **#2** — Disable contact shadows.
   - **Expected:** 0.2 m gap pair shows no contact shadow; only CSM shadow present.

### TC-2.4.16.1 Distance Field Shadow Trace

| # | Requirement |
|---|-------------|
| 1 | R-2.4.16    |
| 2 | R-2.4.16    |

1. **#1** — Trace distance field from light through a sphere SDF of radius 1 m.
   - **Expected:** Shadow factor behind sphere == 0.0; shadow factor at grazing tangent > 0.7.
2. **#2** — Trace from a direction missing the sphere.
   - **Expected:** Shadow factor == 1.0 (unshadowed).

### TC-2.4.17.1 Capsule Shadow Skeletal Mesh

| # | Requirement |
|---|-------------|
| 1 | R-2.4.17    |
| 2 | R-2.4.17    |

1. **#1** — Compute capsule shadow for a single bone capsule (radius 0.1 m, length 1 m) under a
   directional light.
   - **Expected:** Shadow ellipse projected onto ground plane; area within 5% of analytical value.
2. **#2** — Animate bone to rotate 90 degrees.
   - **Expected:** Shadow ellipse rotates correspondingly; area change < 5%.

### TC-2.4.18.1 Moment-Based OIT Resolve

| # | Requirement |
|---|-------------|
| 1 | R-2.4.18    |
| 2 | R-2.4.18    |

1. **#1** — Render 3 overlapping translucent layers at alpha 0.5; run MBOIT resolve.
   - **Expected:** Final alpha matches `1 - (1 - 0.5)^3 == 0.875` within 1/255.
2. **#2** — Swap draw order of the 3 layers.
   - **Expected:** Output identical within 1/255 (order independence).

### TC-2.4.19.1 Volumetric Shadow Map Extinction

| # | Requirement |
|---|-------------|
| 1 | R-2.4.19    |
| 2 | R-2.4.19    |

1. **#1** — Build volumetric shadow for participating media of optical depth 2.0 from a directional
   light.
   - **Expected:** Transmittance at far end == `exp(-2.0)` within 1%.
2. **#2** — Sample midway through the media.
   - **Expected:** Transmittance == `exp(-1.0)` within 1%.

### TC-2.4.20.1 Area Light LTC Energy

| # | Requirement |
|---|-------------|
| 1 | R-2.4.20    |
| 2 | R-2.4.20    |

1. **#1** — Evaluate LTC BRDF for a 1x1 m rect area light over a horizontal plane, roughness 0.5.
   - **Expected:** Integrated irradiance at plane center within 5% of reference Monte Carlo solution
     with 1024 samples.
2. **#2** — Disc area light of equal area.
   - **Expected:** Integrated irradiance within 5% of rect result (equal power conservation).

### TC-2.4.21.1 Sky Light Split-Sum IBL

| # | Requirement |
|---|-------------|
| 1 | R-2.4.21    |
| 2 | R-2.4.21    |

1. **#1** — Prefilter an HDR cubemap for split-sum IBL; sample for roughness 0.0.
   - **Expected:** Sample matches mip 0 of source cubemap within 1/255.
2. **#2** — Sample for roughness 1.0.
   - **Expected:** Sample equals hemisphere average (constant per face) within 1/255.

### TC-2.4.22.1 IES Profile Parse and Sample

| # | Requirement |
|---|-------------|
| 1 | R-2.4.22    |
| 2 | R-2.4.22    |

1. **#1** — Parse valid IESNA LM-63-2002 file with 180 horizontal x 37 vertical candela grid.
   - **Expected:** Resulting `IesProfile` has 180x37 samples; max candela matches header value.
2. **#2** — Sample at `(0, 0)` (nadir).
   - **Expected:** Returns header's nadir candela within 0.1%.

### TC-2.4.23.1 Light Function Gobo Sampling

| # | Requirement |
|---|-------------|
| 1 | R-2.4.23    |
| 2 | R-2.4.23    |

1. **#1** — Attach light function texture (checker pattern) to a spot light; project onto plane.
   - **Expected:** Plane color is modulated by checker pattern; sampled at cell center matches
     texture cell color.
2. **#2** — Detach light function.
   - **Expected:** Plane color equals unmodulated spot light radiance.

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

<!-- THIN: design section lacks detail -->
### TC-2.3.1.I1 Direct Lighting Scene End-To-End Point

| # | Requirement |
|---|-------------|
| 1 | US-2.3.1.1  |

1. **#1** — Build scene with 1 point light, 1 lit sphere; run full frame.
   - **Expected:** Framebuffer lit by point light; luminance at sphere center matches analytic
     `N dot L * color * intensity / (distance^2)` within 5%.

<!-- THIN: design section lacks detail -->
### TC-2.3.1.I2 Direct Lighting Scene End-To-End Spot

| # | Requirement |
|---|-------------|
| 1 | US-2.3.1.2  |

1. **#1** — Build scene with 1 spot light (inner 10 deg, outer 20 deg), 1 plane; render frame.
   - **Expected:** Plane lit within inner cone at full intensity; falloff ring between inner and
     outer angles; zero outside outer angle.

<!-- THIN: design section lacks detail -->
### TC-2.3.1.I3 Direct Lighting Scene End-To-End Directional

| # | Requirement |
|---|-------------|
| 1 | US-2.3.1.3  |

1. **#1** — Build scene with 1 directional light, 1 lit sphere; run full frame.
   - **Expected:** Sphere shaded according to `max(0, dot(N, -L)) * color * intensity`; parallel
     shading direction validated at opposite sphere poles.

<!-- THIN: design section lacks detail -->
### TC-2.3.2.I1 GPU Meshlet Frustum Cull End-To-End

| # | Requirement |
|---|-------------|
| 1 | US-2.3.2.1  |

1. **#1** — Load 10k meshlet mesh; place camera so 30% are in frustum; run cull compute pass.
   - **Expected:** Indirect draw count equal to 30% within 1%; disabled meshlets produce zero
     fragments.

<!-- THIN: design section lacks detail -->
### TC-2.3.2.I2 GPU Meshlet Cull Camera Rotation

| # | Requirement |
|---|-------------|
| 1 | US-2.3.2.2  |

1. **#1** — Rotate camera 90 deg over 60 frames; track indirect draw count.
   - **Expected:** Draw count changes each frame monotonically with frustum overlap; never exceeds
     total meshlet count.

<!-- THIN: design section lacks detail -->
### TC-2.3.3.I1 Meshlet Backface Cull Sphere

| # | Requirement |
|---|-------------|
| 1 | US-2.3.3.1  |

1. **#1** — Render tessellated sphere with normal-cone backface cull; capture draw count.
   - **Expected:** Draw count halved within 10% vs no-cull baseline; visible pixels identical.

<!-- THIN: design section lacks detail -->
### TC-2.3.3.I2 Meshlet Backface Cull Flat Mesh

| # | Requirement |
|---|-------------|
| 1 | US-2.3.3.2  |

1. **#1** — Render a flat quad mesh with normal cone facing camera; then facing away.
   - **Expected:** Facing camera: all meshlets drawn. Facing away: zero meshlets drawn.

<!-- THIN: design section lacks detail -->
### TC-2.3.4.I1 Two-Phase HZB Occlusion E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.3.4.1  |

1. **#1** — Scene with a large occluder wall hiding 500 instances; run two-phase HZB cull.
   - **Expected:** Phase 1 rejects >90% of hidden instances using previous HZB; phase 2 reclaims any
     newly-revealed instances.

<!-- THIN: design section lacks detail -->
### TC-2.3.4.I2 HZB Reveal After Occluder Move

| # | Requirement |
|---|-------------|
| 1 | US-2.3.4.2  |

1. **#1** — Move occluder wall away over 1 frame; rerun phase 2.
   - **Expected:** All 500 previously-hidden instances appear in next frame with no pop-in delay.

<!-- THIN: design section lacks detail -->
### TC-2.3.4.I3 HZB Stress With 1M Instances

| # | Requirement |
|---|-------------|
| 1 | US-2.3.4.3  |

1. **#1** — Scene with 1M instances, 20% occluded; measure cull wall time.
   - **Expected:** Phase 1 + phase 2 total < 1.0 ms GPU time; occluded fraction rejected >= 20%.

<!-- THIN: design section lacks detail -->
### TC-2.3.5.I1 Orthographic Camera 2D Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.3.5.1  |

1. **#1** — Set up orthographic camera; render 3 sprites at z=0, z=10, z=100.
   - **Expected:** All 3 sprites render with identical screen size; no perspective foreshortening.

<!-- THIN: design section lacks detail -->
### TC-2.3.5.I2 Orthographic Shadow Cascade Camera

| # | Requirement |
|---|-------------|
| 1 | US-2.3.5.2  |

1. **#1** — Use orthographic camera for CSM shadow cascade; render shadow map.
   - **Expected:** Shadow depth values linear in world units; cascade bounds match requested
     frustum.

<!-- THIN: design section lacks detail -->
### TC-2.3.6.I1 Perspective Reverse-Z Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.3.6.1  |

1. **#1** — Render scene with reverse-Z perspective; sample depth at near and far objects.
   - **Expected:** Near depth == 1.0, far depth approaches 0.0 (reverse convention); greater
     precision on distant objects vs standard Z.

<!-- THIN: design section lacks detail -->
### TC-2.3.6.I2 Reverse-Z Depth Test Ordering

| # | Requirement |
|---|-------------|
| 1 | US-2.3.6.2  |

1. **#1** — Render 2 overlapping boxes at differing depths under `GREATER` depth func.
   - **Expected:** Nearer box visible; depth test passes with reverse-Z mapping correctly.

<!-- THIN: design section lacks detail -->
### TC-2.3.7.I1 GPU Instancing Batch Compaction

| # | Requirement |
|---|-------------|
| 1 | US-2.3.7.1  |

1. **#1** — Spawn 10k identical instances with 5 materials; compact on GPU.
   - **Expected:** Draw calls reduced to 5; total instance count preserved; rendering identical to
     non-compacted reference.

<!-- THIN: design section lacks detail -->
### TC-2.3.7.I2 GPU Instancing Mixed LOD

| # | Requirement |
|---|-------------|
| 1 | US-2.3.7.2  |

1. **#1** — Spawn 10k instances split across 3 LOD levels.
   - **Expected:** Batch compaction yields 3 indirect draw groups, each with correct LOD mesh ID.

<!-- THIN: design section lacks detail -->
### TC-2.3.7.I3 GPU Instancing Dynamic Spawn

| # | Requirement |
|---|-------------|
| 1 | US-2.3.7.3  |

1. **#1** — Spawn 100 new instances per frame for 60 frames; verify batch compaction tracks.
   - **Expected:** Draw count matches live instance count each frame; no leaks or stale entries.

<!-- THIN: design section lacks detail -->
### TC-2.3.8.I1 Render-To-Texture With Barriers

| # | Requirement |
|---|-------------|
| 1 | US-2.3.8.1  |

1. **#1** — Render scene to offscreen texture T, then sample T in a later pass.
   - **Expected:** Auto-inserted barrier appears between passes; output pixels match direct-render
     reference within 1/255.

<!-- THIN: design section lacks detail -->
### TC-2.3.8.I2 Render-To-Texture Multi-Read

| # | Requirement |
|---|-------------|
| 1 | US-2.3.8.2  |

1. **#1** — Render to T in one pass; sample in 3 later passes.
   - **Expected:** Single write barrier; 3 read bindings. No redundant barriers.

<!-- THIN: design section lacks detail -->
### TC-2.3.9.I1 Dynamic Cubemap Scene Reflection

| # | Requirement |
|---|-------------|
| 1 | US-2.3.9.1  |

1. **#1** — Scene with moving ball near reflective sphere; dynamic cubemap refresh each frame.
   - **Expected:** Sphere reflection shows ball position each frame; 6 faces written per refresh.

<!-- THIN: design section lacks detail -->
### TC-2.3.9.I2 Static Cubemap Baked

| # | Requirement |
|---|-------------|
| 1 | US-2.3.9.2  |

1. **#1** — Load a pre-baked cubemap asset; sample from 6 canonical directions.
   - **Expected:** Samples match asset data within 1/255; cubemap texture memory unchanged across
     frames.

<!-- THIN: design section lacks detail -->
### TC-2.3.10.I1 Planar Scene Capture To Texture

| # | Requirement |
|---|-------------|
| 1 | US-2.3.10.1 |

1. **#1** — Add `SceneCaptureComponent` planar mode to a mirror; render frame.
   - **Expected:** Mirror texture contains reflected scene; view origin matches mirrored camera.

<!-- THIN: design section lacks detail -->
### TC-2.3.10.I2 Cubemap Scene Capture Refresh Interval

| # | Requirement |
|---|-------------|
| 1 | US-2.3.10.2 |

1. **#1** — Cubemap capture with `update_interval = 10` frames; run 30 frames.
   - **Expected:** Capture texture updated exactly 3 times; unchanged between updates.

<!-- THIN: design section lacks detail -->
### TC-2.3.11.I1 DRS Under Heavy Load

| # | Requirement |
|---|-------------|
| 1 | US-2.3.11.1 |

1. **#1** — Simulate 30 ms GPU cost at target 16 ms; run feedback loop 60 frames.
   - **Expected:** Scale converges to roughly sqrt(16/30) within 5 frames; oscillation <5% post
     convergence.

<!-- THIN: design section lacks detail -->
### TC-2.3.11.I2 DRS Under Light Load

| # | Requirement |
|---|-------------|
| 1 | US-2.3.11.2 |

1. **#1** — Simulate 8 ms GPU cost at target 16 ms; run 60 frames.
   - **Expected:** Scale climbs to `max_scale` (1.0) within budget; no upward overshoot.

<!-- THIN: design section lacks detail -->
### TC-2.3.11.I3 DRS Rapid Cost Swings

| # | Requirement |
|---|-------------|
| 1 | US-2.3.11.3 |

1. **#1** — Alternate 8 ms / 24 ms every frame for 60 frames.
   - **Expected:** Scale change rate limited; per-frame delta < 0.1; no oscillation divergence.

<!-- THIN: design section lacks detail -->
### TC-2.3.12.I1 SSS Screen-Space Skin

| # | Requirement |
|---|-------------|
| 1 | US-2.3.12.1 |

1. **#1** — Render a lit head model with subsurface shading and screen-space blur.
   - **Expected:** Back-lit ear rim shows reddish tint; frequency analysis confirms high-freq
     attenuation from diffusion.

<!-- THIN: design section lacks detail -->
### TC-2.3.12.I2 SSS Ray-Traced Path

| # | Requirement |
|---|-------------|
| 1 | US-2.3.12.2 |

1. **#1** — Enable ray-traced SSS path on same head model.
   - **Expected:** Output visually matches SS-SSS ground truth within PSNR > 35 dB; no NaN pixels.

<!-- THIN: design section lacks detail -->
### TC-2.3.13.I1 Alpha Cutout Foliage Shadow

| # | Requirement |
|---|-------------|
| 1 | US-2.3.13.1 |

1. **#1** — Render grass mesh with alpha cutout threshold 0.5 into both color and shadow passes.
   - **Expected:** Color and shadow silhouettes identical; shadow receiver sees gaps between blades.

<!-- THIN: design section lacks detail -->
### TC-2.3.13.I2 Alpha Cutout Threshold Change

| # | Requirement |
|---|-------------|
| 1 | US-2.3.13.2 |

1. **#1** — Change threshold from 0.5 to 0.1 at runtime.
   - **Expected:** More pixels visible; shadow silhouette updates on next frame without reupload.

<!-- THIN: design section lacks detail -->
### TC-2.10.1.I1 Render Proxy Extraction Single Frame

| # | Requirement |
|---|-------------|
| 1 | US-2.10.1.1 |

1. **#1** — Spawn 1000 renderables; run one extraction frame.
   - **Expected:** `ProxyStore.len() == 1000`; all proxies have valid entity mappings; simulation
     world untouched by extractor (no mut).

<!-- THIN: design section lacks detail -->
### TC-2.10.1.I2 Render Proxy Extraction Concurrent

| # | Requirement |
|---|-------------|
| 1 | US-2.10.1.2 |

1. **#1** — Run extraction on dedicated thread while simulation runs on game thread.
   - **Expected:** No data races (ThreadSanitizer clean); snapshot internally consistent.

<!-- THIN: design section lacks detail -->
### TC-2.10.2.I1 SoA Proxy Upload To GPU

| # | Requirement |
|---|-------------|
| 1 | US-2.10.2.1 |

1. **#1** — Upload 10k SoA proxies to a GPU staging buffer.
   - **Expected:** Buffer contents match CPU-side column layout; no interleaving.

<!-- THIN: design section lacks detail -->
### TC-2.10.2.I2 SoA Proxy Column Access Pattern

| # | Requirement |
|---|-------------|
| 1 | US-2.10.2.2 |

1. **#1** — Iterate `transforms[]` column sequentially over 100k proxies.
   - **Expected:** No stride waste; iteration touches only `transforms` memory range (verified via
     page access counter / profiler).

<!-- THIN: design section lacks detail -->
### TC-2.10.3.I1 Dirty Flag Incremental Update Small

| # | Requirement |
|---|-------------|
| 1 | US-2.10.3.1 |

1. **#1** — 10k proxies; flip 5 dirty; incremental GPU upload.
   - **Expected:** Only 5 GPU writes performed (not 10k).

<!-- THIN: design section lacks detail -->
### TC-2.10.3.I2 Dirty Flag Incremental Update Large

| # | Requirement |
|---|-------------|
| 1 | US-2.10.3.2 |

1. **#1** — 10k proxies; flip 9000 dirty; incremental GPU upload.
   - **Expected:** 9000 GPU writes. Output matches full-upload reference within 0 diff.

<!-- THIN: design section lacks detail -->
### TC-2.10.3.I3 Dirty Flag Zero Dirty Fast Path

| # | Requirement |
|---|-------------|
| 1 | US-2.10.3.3 |

1. **#1** — 10k proxies; 0 dirty; incremental upload.
   - **Expected:** Zero GPU writes; zero staging allocations.

<!-- THIN: design section lacks detail -->
### TC-2.10.4.I1 View Camera Registration Main

| # | Requirement |
|---|-------------|
| 1 | US-2.10.4.1 |

1. **#1** — Register main camera with perspective projection.
   - **Expected:** `RenderView` added with `ViewType::MainCamera`, `view_id` unique.

<!-- THIN: design section lacks detail -->
### TC-2.10.4.I2 View Camera Register Shadow

| # | Requirement |
|---|-------------|
| 1 | US-2.10.4.2 |

1. **#1** — Register shadow cascade view with orthographic projection.
   - **Expected:** `RenderView` added with `ViewType::ShadowCascade`; projection orthographic.

<!-- THIN: design section lacks detail -->
### TC-2.10.5.I1 Multi-View From Single Snapshot

| # | Requirement |
|---|-------------|
| 1 | US-2.10.5.1 |

1. **#1** — Render 4 views (main, shadow, reflection, ui) from one extracted snapshot.
   - **Expected:** Snapshot extracted exactly once; 4 distinct draw lists produced from same proxy
     store.

<!-- THIN: design section lacks detail -->
### TC-2.10.5.I2 Multi-View VR Stereo

| # | Requirement |
|---|-------------|
| 1 | US-2.10.5.2 |

1. **#1** — Register 2 VR eye views sharing one snapshot; render.
   - **Expected:** Both eye buffers populated; single extraction; no duplicate state setup.

<!-- THIN: design section lacks detail -->
### TC-2.10.6.I1 Per-View Draw List Sort Main

| # | Requirement |
|---|-------------|
| 1 | US-2.10.6.1 |

1. **#1** — Main view generates 5000 draws mixed opaque/transparent; sort.
   - **Expected:** Opaque sorted front-to-back by depth; transparent sorted back-to-front; ordering
     matches sort key.

<!-- THIN: design section lacks detail -->
### TC-2.10.6.I2 Per-View Draw List Sort Shadow

| # | Requirement |
|---|-------------|
| 1 | US-2.10.6.2 |

1. **#1** — Shadow view: sort only by material (depth irrelevant).
   - **Expected:** Draws grouped by material; depth field ignored in sort order.

<!-- THIN: design section lacks detail -->
### TC-2.10.7.I1 GPU Compute Batch Compaction

| # | Requirement |
|---|-------------|
| 1 | US-2.10.7.1 |

1. **#1** — 10k surviving draws; run compaction compute pass.
   - **Expected:** Compacted indirect buffer contains contiguous draws grouped by key; total
     instance count preserved.

<!-- THIN: design section lacks detail -->
### TC-2.10.7.I2 GPU Compute Batch Compaction Empty

| # | Requirement |
|---|-------------|
| 1 | US-2.10.7.2 |

1. **#1** — Zero surviving draws; run compaction.
   - **Expected:** Indirect buffer size == 0; no GPU dispatches issued.

<!-- THIN: design section lacks detail -->
### TC-2.10.8.I1 Bindless Material Parameter Bind

| # | Requirement |
|---|-------------|
| 1 | US-2.10.8.1 |

1. **#1** — 1000 draws with distinct material parameter indices; bind once.
   - **Expected:** 0 descriptor set changes per draw; parameter indices used in shader to fetch
     per-draw data.

<!-- THIN: design section lacks detail -->
### TC-2.10.8.I2 Bindless Material Hot Swap

| # | Requirement |
|---|-------------|
| 1 | US-2.10.8.2 |

1. **#1** — Replace material parameter buffer slot 5 with new data mid-frame.
   - **Expected:** Next draw using slot 5 samples updated data; other slots unaffected.

<!-- THIN: design section lacks detail -->
### TC-2.10.9.I1 Debug Draw API Runtime

| # | Requirement |
|---|-------------|
| 1 | US-2.10.9.1 |

1. **#1** — Call `debug_draw::line(...)` 100 times in debug build; render.
   - **Expected:** 100 line segments visible in debug overlay buffer.

<!-- THIN: design section lacks detail -->
### TC-2.10.9.I2 Debug Draw Stripped Ship Build

| # | Requirement |
|---|-------------|
| 1 | US-2.10.9.2 |

1. **#1** — Ship build with `debug_draw::line` symbol usage.
   - **Expected:** Symbol absent from binary; calls compiled to no-op; zero runtime cost.

<!-- THIN: design section lacks detail -->
### TC-2.10.10.I1 Buffer Viz Base Color Mode

| # | Requirement |
|---|-------------|
| 1 | US-2.10.10.1 |

1. **#1** — Toggle viz mode to `BaseColor` in dev build; render.
   - **Expected:** Output is albedo-only; no lighting or shadows.

<!-- THIN: design section lacks detail -->
### TC-2.10.10.I2 Buffer Viz Normal Mode

| # | Requirement |
|---|-------------|
| 1 | US-2.10.10.2 |

1. **#1** — Toggle viz mode to `WorldNormal`; render.
   - **Expected:** Output color encodes world-space normals; RGB channels in `[-1, 1]` mapped to
     `[0, 1]`.

<!-- THIN: design section lacks detail -->
### TC-2.4.1.I1 Forward Plus 500 Lights E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.4.1.1  |

1. **#1** — Scene with 500 point lights; render with forward+ path.
   - **Expected:** All lights contribute; cluster assignment dispatches succeed; shading matches
     reference deferred within PSNR > 40 dB.

<!-- THIN: design section lacks detail -->
### TC-2.4.1.I2 Forward Plus Cluster Overflow Handling

| # | Requirement |
|---|-------------|
| 1 | US-2.4.1.2  |

1. **#1** — Concentrate 1024 lights in a single tile; exceed per-tile cap.
   - **Expected:** Overflow indicator set; overflow lights resolved via fallback path; no clip
     artifacts.

<!-- THIN: design section lacks detail -->
### TC-2.4.2.I1 Deferred Path Large Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.2.1  |

1. **#1** — Complex scene with 200 lights; render via deferred.
   - **Expected:** G-buffer populated; final lit result matches forward+ reference within PSNR > 40
     dB.

<!-- THIN: design section lacks detail -->
### TC-2.4.2.I2 Deferred Path Transparency Fallback

| # | Requirement |
|---|-------------|
| 1 | US-2.4.2.2  |

1. **#1** — Scene mixing opaque (deferred) and transparent (forward) draws.
   - **Expected:** Opaque lit by deferred resolve; transparent composited on top using forward
     shading; correct ordering.

<!-- THIN: design section lacks detail -->
### TC-2.4.10.I1 Stochastic Many-Light Sampling Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.10.1 |

1. **#1** — Scene with 2000 lights; sample 4 per pixel stochastically; accumulate 32 frames.
   - **Expected:** Noise-free output PSNR > 35 dB vs exhaustive reference.

<!-- THIN: design section lacks detail -->
### TC-2.4.10.I2 Stochastic Sampling Moving Camera

| # | Requirement |
|---|-------------|
| 1 | US-2.4.10.2 |

1. **#1** — Move camera during stochastic integration for 60 frames.
   - **Expected:** No history ghosting; disocclusion clears history correctly.

<!-- THIN: design section lacks detail -->
### TC-2.4.11.I1 CSM 4-Cascade Rendering

| # | Requirement |
|---|-------------|
| 1 | US-2.4.11.1 |

1. **#1** — Directional light with 4 cascades; render scene with near/far shadows.
   - **Expected:** All 4 cascades populated; cascade boundary transitions smooth; no visible splits.

<!-- THIN: design section lacks detail -->
### TC-2.4.11.I2 CSM Temporal Stabilization

| # | Requirement |
|---|-------------|
| 1 | US-2.4.11.2 |

1. **#1** — Move camera 1 m per frame for 60 frames.
   - **Expected:** Shadow map texel alignment temporally stable; < 5% shimmer metric.

<!-- THIN: design section lacks detail -->
### TC-2.4.12.I1 PCF Soft Shadow Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.12.1 |

1. **#1** — Render scene with PCF 5x5 filter on CSM.
   - **Expected:** Shadow edges soft; no acne on lit surfaces.

<!-- THIN: design section lacks detail -->
### TC-2.4.12.I2 PCSS Penumbra Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.12.2 |

1. **#1** — Render scene with PCSS (contact hardening).
   - **Expected:** Penumbra widens with distance from occluder; contact hard, far soft.

<!-- THIN: design section lacks detail -->
### TC-2.4.13.I1 SSAO Scene Enable

| # | Requirement |
|---|-------------|
| 1 | US-2.4.13.1 |

1. **#1** — Enable SSAO tier; render indoor scene.
   - **Expected:** Corner crevices darkened; flat surfaces unchanged; cost under 0.5 ms.

<!-- THIN: design section lacks detail -->
### TC-2.4.13.I2 GTAO Scene Enable

| # | Requirement |
|---|-------------|
| 1 | US-2.4.13.2 |

1. **#1** — Enable GTAO tier; render same scene.
   - **Expected:** Higher quality darkening than SSAO; bent normals available for indirect lighting.

<!-- THIN: design section lacks detail -->
### TC-2.4.14.I1 VSM Active Camera Frustum

| # | Requirement |
|---|-------------|
| 1 | US-2.4.14.1 |

1. **#1** — Enable VSM; render a 10-square-km terrain with moving camera.
   - **Expected:** Only pages within frustum allocated; texture memory usage < 256 MB.

<!-- THIN: design section lacks detail -->
### TC-2.4.14.I2 VSM Page Thrash Resistance

| # | Requirement |
|---|-------------|
| 1 | US-2.4.14.2 |

1. **#1** — Rotate camera 180 deg over 10 frames.
   - **Expected:** Pages evicted/re-allocated smoothly; no rendering spikes > 5 ms.

<!-- THIN: design section lacks detail -->
### TC-2.4.15.I1 Contact Shadow Small Object

| # | Requirement |
|---|-------------|
| 1 | US-2.4.15.1 |

1. **#1** — Enable contact shadows for a scene with small props near the ground.
   - **Expected:** Small shadows visible even when CSM resolution too low to capture them.

<!-- THIN: design section lacks detail -->
### TC-2.4.15.I2 Contact Shadow Cost Budget

| # | Requirement |
|---|-------------|
| 1 | US-2.4.15.2 |

1. **#1** — Measure GPU cost of contact shadow pass for 1080p.
   - **Expected:** Pass cost <= 0.3 ms.

<!-- THIN: design section lacks detail -->
### TC-2.4.16.I1 Distance Field Shadow Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.16.1 |

1. **#1** — Enable DF shadows; render heavy foliage scene.
   - **Expected:** DF shadows cover scene beyond CSM range; no shimmering on static surfaces.

<!-- THIN: design section lacks detail -->
### TC-2.4.16.I2 Distance Field Update Dynamic Object

| # | Requirement |
|---|-------------|
| 1 | US-2.4.16.2 |

1. **#1** — Translate a mesh with DF representation per frame.
   - **Expected:** Shadow follows object; DF data re-sampled, no stale shadow.

<!-- THIN: design section lacks detail -->
### TC-2.4.17.I1 Capsule Shadow Character

| # | Requirement |
|---|-------------|
| 1 | US-2.4.17.1 |

1. **#1** — Skeletal character with 20 bone capsules cast shadow under directional light.
   - **Expected:** Capsule shadows project below character; match shape.

<!-- THIN: design section lacks detail -->
### TC-2.4.17.I2 Capsule Shadow Animation

| # | Requirement |
|---|-------------|
| 1 | US-2.4.17.2 |

1. **#1** — Animate character walk cycle; observe capsule shadows.
   - **Expected:** Shadows follow bone poses; no lag or jitter.

<!-- THIN: design section lacks detail -->
### TC-2.4.18.I1 MBOIT Transparent Foliage

| # | Requirement |
|---|-------------|
| 1 | US-2.4.18.1 |

1. **#1** — Render dense foliage with translucent leaves using MBOIT.
   - **Expected:** Blended output order-independent; no per-draw sorting needed.

<!-- THIN: design section lacks detail -->
### TC-2.4.18.I2 MBOIT Quality Comparison

| # | Requirement |
|---|-------------|
| 1 | US-2.4.18.2 |

1. **#1** — Compare MBOIT vs sorted alpha blending reference.
   - **Expected:** PSNR > 35 dB; structural differences < 5% pixel area.

<!-- THIN: design section lacks detail -->
### TC-2.4.19.I1 Volumetric Shadow Smoke

| # | Requirement |
|---|-------------|
| 1 | US-2.4.19.1 |

1. **#1** — Scene with volumetric smoke volume; run volumetric shadow pass.
   - **Expected:** Smoke self-shadows correctly; light shafts visible; transmittance monotonic.

<!-- THIN: design section lacks detail -->
### TC-2.4.19.I2 Volumetric Shadow Fog

| # | Requirement |
|---|-------------|
| 1 | US-2.4.19.2 |

1. **#1** — Uniform fog scene with directional light.
   - **Expected:** Fog attenuates light contribution in shadowed regions; output matches analytic
     Beer-Lambert solution within 5%.

<!-- THIN: design section lacks detail -->
### TC-2.4.20.I1 Area Light Rect Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.20.1 |

1. **#1** — Rectangular area light above a floor plane; render via LTC.
   - **Expected:** Floor illumination matches Monte Carlo reference within 5%; no energy gain at
     grazing angles.

<!-- THIN: design section lacks detail -->
### TC-2.4.20.I2 Area Light Sphere Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.4.20.2 |

1. **#1** — Spherical area light adjacent to lit sphere.
   - **Expected:** Highlight matches sphere shape; falloff physically correct.

<!-- THIN: design section lacks detail -->
### TC-2.4.21.I1 Sky Light IBL Outdoor

| # | Requirement |
|---|-------------|
| 1 | US-2.4.21.1 |

1. **#1** — Enable sky light from HDR cubemap in outdoor scene.
   - **Expected:** Diffuse & specular IBL contributions match baked reference within PSNR > 40 dB.

<!-- THIN: design section lacks detail -->
### TC-2.4.21.I2 Sky Light Update Time-of-Day

| # | Requirement |
|---|-------------|
| 1 | US-2.4.21.2 |

1. **#1** — Swap cubemap for different time-of-day.
   - **Expected:** Scene lighting updates next frame; no stale probe data.

<!-- THIN: design section lacks detail -->
### TC-2.4.22.I1 IES Profile Streetlamp

| # | Requirement |
|---|-------------|
| 1 | US-2.4.22.1 |

1. **#1** — Assign IES profile to streetlamp point light; render.
   - **Expected:** Ground illumination pattern matches IES candela distribution within 5%.

<!-- THIN: design section lacks detail -->
### TC-2.4.22.I2 IES Profile Swap

| # | Requirement |
|---|-------------|
| 1 | US-2.4.22.2 |

1. **#1** — Swap IES profile at runtime.
   - **Expected:** New profile takes effect next frame; no caching stale profile.

<!-- THIN: design section lacks detail -->
### TC-2.4.23.I1 Light Function Projector Cookie

| # | Requirement |
|---|-------------|
| 1 | US-2.4.23.1 |

1. **#1** — Spot light with cookie texture (Batman signal) onto cloud layer.
   - **Expected:** Cookie projected onto cloud plane; pattern matches texture orientation.

<!-- THIN: design section lacks detail -->
### TC-2.4.23.I2 Light Function Animated Cookie

| # | Requirement |
|---|-------------|
| 1 | US-2.4.23.2 |

1. **#1** — Animate cookie texture UVs over time.
   - **Expected:** Projection animates in sync; no texture filtering artifacts.

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
