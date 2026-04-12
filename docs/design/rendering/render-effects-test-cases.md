# Render Effects — Test Cases

Companion to [render-effects.md](render-effects.md).

Test case IDs use `TC-2.9.Z.N` and `TC-2.4.Z.N` format. Every row links to a specific R-2.9.Z,
R-2.4.Z, or F-X.Y.Z.

## Unit Tests

| ID            | Name                              | Req       |
|---------------|-----------------------------------|-----------|
| TC-2.9.1.1    | `test_bloom_threshold_filter`     | R-2.9.1   |
| TC-2.9.1.2    | `test_bloom_dual_kawase_mobile`   | R-2.9.1   |
| TC-2.9.1.3    | `test_bloom_gaussian_desktop`     | R-2.9.1   |
| TC-2.9.2.1    | `test_dof_circle_of_confusion`    | R-2.9.2   |
| TC-2.9.2.2    | `test_dof_bokeh_shape_circle`     | R-2.9.2   |
| TC-2.9.3.1    | `test_motion_blur_velocity_dir`   | R-2.9.3   |
| TC-2.9.4.1    | `test_auto_exposure_histogram`    | R-2.9.4   |
| TC-2.9.4.2    | `test_auto_exposure_smoothing`    | R-2.9.4   |
| TC-2.9.5.1    | `test_aces_tone_curve_clamp`      | R-2.9.5   |
| TC-2.9.5.2    | `test_lut_apply_3d`               | R-2.9.5   |
| TC-2.9.6.1    | `test_film_grain_seed`            | R-2.9.6   |
| TC-2.9.6.2    | `test_chromatic_aberration_split` | R-2.9.6   |
| TC-2.9.6.3    | `test_vignette_radial_falloff`    | R-2.9.6   |
| TC-2.9.7.1    | `test_custom_pp_grayscale`        | R-2.9.7   |
| TC-2.9.7.2    | `test_custom_pp_chain_3_passes`   | R-2.9.7   |
| TC-2.9.8.1    | `test_local_exposure_tile_grid`   | R-2.9.8   |
| TC-2.9.10.1   | `test_cavity_normal_sample`       | R-2.9.10  |
| TC-2.9.12.1   | `test_pp_volume_blend_priority`   | R-2.9.12  |
| TC-2.9.13.1   | `test_quality_tier_mobile`        | R-2.9.13  |
| TC-2.4.1.1    | `test_tiled_light_cull_grid`      | R-2.4.1   |
| TC-2.4.10.1   | `test_stochastic_light_sampling`  | R-2.4.10  |
| TC-2.4.11.1   | `test_csm_cascade_split`          | R-2.4.11  |
| TC-2.4.13.1   | `test_gtao_bent_normal`           | R-2.4.13  |
| TC-2.9.9.1    | `test_vignette_power_curve`       | R-2.9.9   |
| TC-2.9.11.1   | `test_local_exposure_tile_bias`   | R-2.9.11  |
| TC-2.9.14.1   | `test_post_graph_compile`         | R-2.9.14  |
| TC-2.5.1.1    | `test_blas_build_from_meshlets`   | R-2.5.1   |
| TC-2.5.2.1    | `test_rt_reflection_hybrid_ssr`   | R-2.5.2   |
| TC-2.5.3.1    | `test_rt_indirect_diffuse_one`    | R-2.5.3   |
| TC-2.5.4.1    | `test_ddgi_probe_irradiance`      | R-2.5.4   |
| TC-2.5.5.1    | `test_path_tracer_accumulation`   | R-2.5.5   |
| TC-2.5.6.1    | `test_rt_subsurface_transmit`     | R-2.5.6   |
| TC-2.5.7.1    | `test_surfel_clipmap_update`      | R-2.5.7   |
| TC-2.5.8.1    | `test_restir_reservoir_merge`     | R-2.5.8   |
| TC-2.5.9.1    | `test_realtime_pt_tier_select`    | R-2.5.9   |
| TC-2.5.10.1   | `test_opacity_micromap_build`     | R-2.5.10  |
| TC-2.5.11.1   | `test_shader_exec_reorder`        | R-2.5.11  |
| TC-2.5.12.1   | `test_denoiser_psnr_above_30`     | R-2.5.12  |
| TC-2.5.13.1   | `test_rt_lens_flare_trace`        | R-2.5.13  |
| TC-2.5.14.1   | `test_voxel_gi_cone_trace`        | R-2.5.14  |
| TC-2.5.15.1   | `test_neural_radiance_cache_hit`  | R-2.5.15  |
| TC-2.5.16.1   | `test_stochastic_ssr_brdf`        | R-2.5.16  |

1. **TC-2.9.1.1** `test_bloom_threshold_filter` — Input HDR pixels at luminance 0.5, 1.0, 2.0 with
   threshold 1.0. Assert only the 2.0 pixel contributes; the 0.5 and 1.0 pixels output 0.
   - Input: `BloomParams { threshold: 1.0 }`, three pixel samples
   - Expected: `[0.0, 0.0, (2.0 - 1.0)]` after threshold prefilter

2. **TC-2.9.1.2** `test_bloom_dual_kawase_mobile` — Quality tier `Mobile`. Build bloom pass list.
   Assert dual-kawase variant is selected, 4 down + 4 up passes.
   - Input: `QualityTier::Mobile`, bloom enabled
   - Expected: `passes.len() == 8`, all pass kinds are `DualKawase{Down|Up}`

3. **TC-2.9.1.3** `test_bloom_gaussian_desktop` — Quality tier `HighEnd`. Build bloom pass list.
   Assert wide Gaussian variant is selected with 6 mip levels.
   - Input: `QualityTier::HighEnd`, bloom enabled
   - Expected: `mip_count == 6`, pass kind == `Gaussian`

4. **TC-2.9.2.1** `test_dof_circle_of_confusion` — Pixel at depth 5 m, focus 5 m. Aperture f/2.0.
   Assert CoC ≈ 0. Pixel at depth 10 m. Assert CoC > 0.
   - Input: `DofParams { focus: 5.0, aperture: 2.0, focal: 0.05 }`
   - Expected: `coc(5.0) ≈ 0.0`, `coc(10.0) > 0.0`

5. **TC-2.9.2.2** `test_dof_bokeh_shape_circle` — Set bokeh shape to `Circle`. Sample bokeh kernel
   at 16 angles. Assert all samples lie within unit disk.
   - Input: `BokehShape::Circle`, 16 sample directions
   - Expected: every sample `length() <= 1.0`, max length within `[0.95, 1.0]`

6. **TC-2.9.3.1** `test_motion_blur_velocity_dir` — Pixel velocity buffer entry `Vec2(0.1, 0.0)`.
   Assert blur sample direction is `(1, 0)` and step count matches blur length.
   - Input: velocity sample, `MotionBlurParams { sample_count: 8 }`
   - Expected: sample direction `Vec2(1, 0)`, 8 samples emitted along `+x`

7. **TC-2.9.4.1** `test_auto_exposure_histogram` — 64-bin luminance histogram from a uniform 0.5
   luminance image. Assert peak bin matches log2(0.5) bucket; computed EV100 within 0.1.
   - Input: 256x256 image, all pixels luminance 0.5, `min_ev: -8`, `max_ev: 8`
   - Expected: histogram peak at expected bin, `ev100` within `[-1.1, -0.9]`

8. **TC-2.9.4.2** `test_auto_exposure_smoothing` — Previous `ev100 = 0.0`, target `ev100 = 4.0`,
   adapt rate 1 EV/s, dt = 0.5 s. Assert new ev100 ≈ 0.5.
   - Input: `AutoExposure { adapt_rate: 1.0, prev: 0.0 }`, target 4.0, dt 0.5
   - Expected: `ev100` within `[0.45, 0.55]`

9. **TC-2.9.5.1** `test_aces_tone_curve_clamp` — Apply ACES tone curve to inputs
   `[0.0, 0.18, 1.0, 8.0]`. Assert outputs are monotonically increasing and the 8.0 input maps to ≤
   1.0.
   - Input: array of HDR luminances
   - Expected: outputs sorted ascending, `aces(8.0) <= 1.0`

10. **TC-2.9.5.2** `test_lut_apply_3d` — 32^3 identity LUT applied to RGB `(0.4, 0.6, 0.2)`. Assert
    output equals input within 1/255 tolerance.
    - Input: identity LUT, single pixel
    - Expected: `|out - in| < 1.0/255.0` per channel

11. **TC-2.9.6.1** `test_film_grain_seed` — Generate film grain pattern with seed 42 twice. Assert
    byte-equal output. Re-run with seed 43. Assert different output.
    - Input: `FilmGrain { seed: 42 }` then `seed: 43`
    - Expected: pattern_42_a == pattern_42_b, pattern_42 != pattern_43

12. **TC-2.9.6.2** `test_chromatic_aberration_split` — Aberration strength 1.0; sample at screen
    edge. Assert R, G, B sample offsets distinct and increase toward edge.
    - Input: `ChromaticAberration { strength: 1.0 }`, sample at uv `(0.95, 0.5)`
    - Expected: `r_offset.x > g_offset.x > b_offset.x` (or symmetric depending on convention)

13. **TC-2.9.6.3** `test_vignette_radial_falloff` — Sample vignette at uv `(0.5, 0.5)` and
    `(0.0, 0.0)` with strength 1.0. Assert center ≈ 1.0, corner < 0.5.
    - Input: `Vignette { strength: 1.0, falloff: 2.0 }`
    - Expected: `vignette(0.5, 0.5) ≈ 1.0`, `vignette(0.0, 0.0) < 0.5`

14. **TC-2.9.7.1** `test_custom_pp_grayscale` — Build a 1-node grayscale post-process graph; apply
    to a colored image. Assert output channels equal luma value.
    - Input: graph `[ColorInput → DesaturateNode(1.0) → Output]`, `(0.7, 0.3, 0.1)` pixel
    - Expected: output `r == g == b == luma(0.7, 0.3, 0.1)`

15. **TC-2.9.7.2** `test_custom_pp_chain_3_passes` — Chain three custom passes (negate, saturate,
    grayscale). Assert composition order: negate → saturate → grayscale.
    - Input: pipeline with three nodes in declared order
    - Expected: `compiled_passes` order matches declaration; output matches manual composition

16. **TC-2.9.8.1** `test_local_exposure_tile_grid` — 16x16 tile grid downsampled from 1024x1024.
    Bright sky tiles get exposure -1, dark interior tiles get +1. Assert tile classification.
    - Input: top half luminance 4.0, bottom half luminance 0.1
    - Expected: top tiles have negative bias, bottom tiles positive

17. **TC-2.9.10.1** `test_cavity_normal_sample` — Sample normal buffer at concave region; assert
    cavity factor > 0. Sample at convex; assert curvature factor > 0.
    - Input: synthetic normal buffer with concave dimple at center, convex bump at corner
    - Expected: `cavity(center) > 0.5`, `curvature(corner) > 0.5`

18. **TC-2.9.12.1** `test_pp_volume_blend_priority` — Two volumes overlap: A
    `priority=1, bloom=0.5`, B `priority=2, bloom=1.0`. Camera at center. Assert blended result
    favors B.
    - Input: two `PostProcessVolume` entities, camera inside both
    - Expected: blended bloom value within `[0.85, 1.0]` (priority weighted)

19. **TC-2.9.13.1** `test_quality_tier_mobile` — Set `QualityTier::Mobile`. Assert DOF disabled,
    bloom uses dual-kawase, motion blur disabled, custom pp passes capped at 1.
    - Input: tier change to Mobile
    - Expected: `EffectConfig { dof: false, bloom: DualKawase, motion_blur: false, max_pp: 1 }`

20. **TC-2.4.1.1** `test_tiled_light_cull_grid` — 16x16 light grid; insert 100 lights at known
    positions. Assert each tile's light list contains only lights overlapping its frustum.
    - Input: 100 `Light` entities, 16x16 tile grid, known camera frustum
    - Expected: per-tile counts match brute-force overlap; no false positives

21. **TC-2.4.10.1** `test_stochastic_light_sampling` — Importance-sample 8 lights per pixel from a
    set of 1000. Assert sampled light index distribution favors higher-radiance lights.
    - Input: 1000 lights, 1 with radiance 1000, 999 with radiance 1
    - Expected: bright light sampled in > 50% of samples over 10000 trials

22. **TC-2.4.11.1** `test_csm_cascade_split` — 4-cascade CSM, far plane 200, lambda 0.5. Assert
    splits match the standard PSSM equation within 0.1 m.
    - Input: `near=0.1`, `far=200`, `cascades=4`, `lambda=0.5`
    - Expected: split distances within tolerance of PSSM-Slice formula

23. **TC-2.4.13.1** `test_gtao_bent_normal` — Compute GTAO at a flat horizontal surface. Assert bent
    normal is straight up `(0, 1, 0)` and AO factor ≈ 1.0 (no occlusion).
    - Input: synthetic depth/normal buffer of a flat plane
    - Expected: `bent_normal ≈ Vec3(0, 1, 0)`, `ao >= 0.95`

24. **TC-2.9.9.1** `test_vignette_power_curve` — Sample radial vignette with power curve exponent
    2.0 at uv `(0.0, 0.0)` and `(0.5, 0.5)`. Assert curve shape.
    - Input: `Vignette { falloff: Power(2.0), strength: 1.0 }`
    - Expected: `v(0.0,0.0) < 0.25`, `v(0.5,0.5) == 1.0`, monotonic decrease from center

25. **TC-2.9.11.1** `test_local_exposure_tile_bias` — Per-tile local exposure bias computed from a
    16x16 tile histogram; tile with high average luminance gets negative EV bias.
    - Input: 1024x1024 luminance field with bright top-left quadrant
    - Expected: top-left tile biases negative; bottom-right tiles near zero; center smooth blend

26. **TC-2.9.14.1** `test_post_graph_compile` — Build a 3-node post-process graph via the node
    editor DSL and compile. Assert compiled pass order matches topological sort.
    - Input: graph `[A → B → C]` with one forked sampler input
    - Expected: compiled pass order `[A, B, C]`, fork sampler bound once, no redundant copies

27. **TC-2.5.1.1** `test_blas_build_from_meshlets` — Build a BLAS from a 1k-triangle meshlet cluster
    with compaction. Assert BLAS is valid and compacted size < raw size.
    - Input: 1000 triangle mesh, `BlasBuildFlags::COMPACTION`
    - Expected: valid BLAS handle, `compacted_size < raw_size * 0.75`

28. **TC-2.5.2.1** `test_rt_reflection_hybrid_ssr` — Pixel with SSR hit falls back to RT when SSR
    occluded. Assert correct source per pixel.
    - Input: scene with partial occluder in front of reflective surface
    - Expected: occluded pixels use RT ray, unoccluded use SSR; sum image matches reference

29. **TC-2.5.3.1** `test_rt_indirect_diffuse_one` — Trace one-bounce indirect diffuse on a Cornell
    box scene. Assert color bleed from red wall onto floor.
    - Input: Cornell fixture, single bounce
    - Expected: floor near red wall samples red channel > floor near white wall

30. **TC-2.5.4.1** `test_ddgi_probe_irradiance` — Populate a 4x4x4 DDGI grid and sample irradiance
    at probe center. Assert radiance roughly matches direct probe integration.
    - Input: uniform emissive environment, 64 probes
    - Expected: sampled irradiance within 5% of analytic expected

31. **TC-2.5.5.1** `test_path_tracer_accumulation` — Run 64 samples per pixel on a static scene.
    Assert variance decreases as 1/N.
    - Input: fixed seed, static scene, N samples in `[1, 4, 16, 64]`
    - Expected: measured variance ratio within 20% of theoretical 1/N

32. **TC-2.5.6.1** `test_rt_subsurface_transmit` — Trace subsurface transmission through a 1 cm
    thick skin slab. Assert transmitted color matches Beer-Lambert expectation.
    - Input: planar slab, skin material, single point light behind
    - Expected: exit radiance within 10% of `exp(-sigma_t * 1 cm)` per channel

33. **TC-2.5.7.1** `test_surfel_clipmap_update` — Spawn surfels in view, step the clipmap update
    pass, verify stale surfels evicted from outer clipmap level.
    - Input: 1000 surfels, 3 clipmap levels
    - Expected: outer-level surfels evicted after N frames; inner level stable

34. **TC-2.5.8.1** `test_restir_reservoir_merge` — Merge two spatially adjacent reservoirs with
    known samples. Assert merged reservoir weight sum is correct.
    - Input: two reservoirs with `(sample, weight)` pairs
    - Expected: merged `w_sum == a.w_sum + b.w_sum`, selected sample follows distribution

35. **TC-2.5.9.1** `test_realtime_pt_tier_select` — Quality tier `Ultra` selects 2 bounces + NRD;
    `Mid` selects 1 bounce + simpler filter. Assert selector picks correct config.
    - Input: tier in `{Ultra, High, Mid, Low}`
    - Expected: configs match spec table; Low disables path tracing entirely

36. **TC-2.5.10.1** `test_opacity_micromap_build` — Build opacity micromap for an alpha-tested leaf
    mesh. Assert opaque/transparent/unknown counts match texture.
    - Input: leaf quad with known alpha mask
    - Expected: OMM classification counts match manual grid evaluation

37. **TC-2.5.11.1** `test_shader_exec_reorder` — Trace path with diverging material branches; assert
    SER reduces effective divergence vs. non-SER baseline.
    - Input: fixture scene with two materials, traced with SER on/off
    - Expected: SER-on divergence metric < SER-off metric; visual output identical

38. **TC-2.5.12.1** `test_denoiser_psnr_above_30` — Run neural denoiser on 1 spp input. Assert PSNR
    vs. reference > 30 dB.
    - Input: 1 spp noisy image + G-buffer inputs
    - Expected: `psnr(denoised, reference) > 30.0`

39. **TC-2.5.13.1** `test_rt_lens_flare_trace` — Trace a ray through lens element list for a single
    bright light source. Assert ghost positions match optical prediction.
    - Input: 5-element lens list, light at screen center
    - Expected: ghost positions within 2 px of reference

40. **TC-2.5.14.1** `test_voxel_gi_cone_trace` — Cone trace one cone through a voxelized Cornell
    box. Assert accumulated radiance is non-zero and matches low-res reference.
    - Input: 128^3 voxel grid, one 45-degree cone
    - Expected: cone sample within 15% of brute-force reference

41. **TC-2.5.15.1** `test_neural_radiance_cache_hit` — Query NRC at a point with cached training
    data. Assert return radiance matches cached prediction.
    - Input: trained NRC, query point within training distribution
    - Expected: query result within 10% of oracle; cache hit logged

42. **TC-2.5.16.1** `test_stochastic_ssr_brdf` — Importance sample SSR rays weighted by microfacet
    BRDF. Assert sample distribution matches BRDF lobe.
    - Input: rough surface, 256 samples
    - Expected: sample histogram correlates with `D_ggx(alpha=0.3)` within 10%

## Integration Tests

| ID          | Name                            | Req      |
|-------------|---------------------------------|----------|
| TC-2.9.I.1  | `test_full_post_chain`          | R-2.9.7  |
| TC-2.9.I.2  | `test_volume_camera_traversal`  | R-2.9.12 |
| TC-2.9.I.3  | `test_hdr10_output`             | R-2.9.5  |
| TC-2.4.I.1  | `test_500_lights_tiled`         | R-2.4.1  |
| TC-2.4.I.2  | `test_csm_4_cascades_render`    | R-2.4.11 |
| TC-2.4.I.3  | `test_pbr_material_match`       | R-2.4.3  |
| TC-2.4.I.4  | `test_decals_dbuffer_blend`     | R-2.4.8  |

1. **TC-2.9.I.1** `test_full_post_chain` — Render scene with bloom, DOF, motion blur, exposure,
   ACES, vignette enabled. Assert each effect runs in order and final output is non-NaN.
   - Input: full effect stack at HighEnd tier
   - Expected: all passes execute, no NaN/Inf in final image, frame time < 6 ms

2. **TC-2.9.I.2** `test_volume_camera_traversal` — Camera moves from outside volume A through
   overlap region into volume B. Assert smooth interpolation of blended parameters.
   - Input: two overlapping volumes with different bloom thresholds
   - Expected: parameter trajectory monotonic across overlap region

3. **TC-2.9.I.3** `test_hdr10_output` — Configure HDR10 swap chain. Render bright HDR scene. Assert
   output color space is `BT2020-PQ` and peak luminance metadata correct.
   - Input: HDR10 enabled, scene with peak HDR pixels
   - Expected: swap chain format `R10G10B10A2`, color space `HDR10`, metadata written

4. **TC-2.4.I.1** `test_500_lights_tiled` — Render 500 dynamic point lights using tiled deferred.
   Assert per-tile light list capped, frame time scales sub-linearly vs 100-light baseline.
   - Input: 500 `Light` entities scattered uniformly
   - Expected: render succeeds, frame time < 1.5x of 100-light baseline (sub-linear)

5. **TC-2.4.I.2** `test_csm_4_cascades_render` — Sun light with 4 CSM cascades. Render scene with
   shadows. Assert no cascade seam artifacts; depth bias prevents shadow acne.
   - Input: directional light, 4 cascades, scene fixture
   - Expected: shadow buffer valid in all cascades, edge sample test shows no seams

6. **TC-2.4.I.3** `test_pbr_material_match` — Render reference PBR sphere set. Compare against
   golden image. Assert PSNR > 40 dB.
   - Input: 8 spheres with varying metallic/roughness
   - Expected: image PSNR vs golden > 40 dB, no per-pixel max delta > 4/255

7. **TC-2.4.I.4** `test_decals_dbuffer_blend` — Project decal onto curved surface with normal
   blending. Assert DBuffer pass writes before lighting and decal contributes to final shading.
   - Input: scene with curved mesh, projected decal entity
   - Expected: DBuffer ordered before lighting; decal visible with correct normals

## Benchmarks

| ID         | Benchmark                        | Target    | Req      |
|------------|----------------------------------|-----------|----------|
| TC-2.9.B.1 | Bloom (1080p, 6 mips)            | < 1.5 ms  | R-2.9.1  |
| TC-2.9.B.2 | DOF (1080p, full)                | < 2.0 ms  | R-2.9.2  |
| TC-2.9.B.3 | ACES + LUT (1080p)               | < 0.4 ms  | R-2.9.5  |
| TC-2.9.B.4 | Auto exposure histogram (1080p)  | < 0.5 ms  | R-2.9.4  |
| TC-2.4.B.1 | Tiled light cull (500 lights)    | < 0.6 ms  | R-2.4.1  |
| TC-2.4.B.2 | CSM 4 cascades (1k draws)        | < 3.0 ms  | R-2.4.11 |

1. **TC-2.9.B.1** — Full bloom chain at 1080p with 6 mip down/up passes. Wall time on reference GPU.
   Verifies the wide Gaussian variant fits within target on HighEnd tier.

2. **TC-2.9.B.2** — Full DOF (CoC + near/far blur) at 1080p with bokeh shape circle. Wall time on
   reference GPU. Verifies cinematic DOF at desktop quality.

3. **TC-2.9.B.3** — ACES tone map combined with 32^3 3D LUT lookup at 1080p. Single fullscreen pass.
   Wall time.

4. **TC-2.9.B.4** — Compute-shader luminance histogram + average + adaptation step at 1080p. Wall
   time.

5. **TC-2.4.B.1** — Tiled light culling for 500 lights against a 16x16 tile grid. Compute pass wall
   time.

6. **TC-2.4.B.2** — Render shadow maps for 4 CSM cascades over 1000 draws. Total GPU pass time.

## Sub-story and Variant Trace

The upstream design lists the following refined sub-stories and letter-variant stories. Each is
covered by the parent-ID TC rows above; a regression in any parent TC constitutes a regression
against the listed sub-story or variant.

- US-2.5.1.1
- US-2.5.1.2
- US-2.5.10.1
- US-2.5.10.2
- US-2.5.11.1
- US-2.5.11.2
- US-2.5.12.1
- US-2.5.12.2
- US-2.5.13.1
- US-2.5.13.2
- US-2.5.14.1
- US-2.5.14.2
- US-2.5.15.1
- US-2.5.15.2
- US-2.5.16.1
- US-2.5.16.2
- US-2.5.2.1
- US-2.5.2.2
- US-2.5.2.3
- US-2.5.3.1
- US-2.5.3.2
- US-2.5.4.1
- US-2.5.4.2
- US-2.5.5.1
- US-2.5.5.2
- US-2.5.6.1
- US-2.5.6.2
- US-2.5.7.1
- US-2.5.7.2
- US-2.5.8.1
- US-2.5.8.2
- US-2.5.9.1
- US-2.5.9.2
