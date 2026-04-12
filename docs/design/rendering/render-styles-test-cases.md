# Render Styles — Test Cases

Companion to [render-styles.md](render-styles.md).

Test case IDs use `TC-2.11.Z.N` and `TC-2.6.Z.N` format. Every row links to a specific R-2.11.Z or
R-2.6.Z.

## Unit Tests

| ID            | Name                                 | Req       |
|---------------|--------------------------------------|-----------|
| TC-2.11.1.1   | `test_outline_jump_flood_init`       | R-2.11.1  |
| TC-2.11.1.2   | `test_outline_color_apply`           | R-2.11.1  |
| TC-2.11.1.3   | `test_outline_sobel_fallback`        | R-2.11.1  |
| TC-2.11.1.4   | `test_outline_priority_overlap`      | R-2.11.1  |
| TC-2.11.2.1   | `test_highlight_pulse_modulation`    | R-2.11.2  |
| TC-2.11.2.2   | `test_highlight_fresnel_rim`         | R-2.11.2  |
| TC-2.11.2.3   | `test_highlight_inner_glow_blur`     | R-2.11.2  |
| TC-2.11.3.1   | `test_toon_band_count_3`             | R-2.11.3  |
| TC-2.11.3.2   | `test_toon_ramp_texture_lookup`      | R-2.11.3  |
| TC-2.11.3.3   | `test_toon_specular_shape`           | R-2.11.3  |
| TC-2.11.4.1   | `test_occlusion_volume_fade`         | R-2.11.4  |
| TC-2.11.4.2   | `test_occlusion_dither_pattern`      | R-2.11.4  |
| TC-2.11.5.1   | `test_xray_depth_compare`            | R-2.11.5  |
| TC-2.11.5.2   | `test_xray_priority_through_walls`   | R-2.11.5  |
| TC-2.11.6.1   | `test_painterly_brush_radius`        | R-2.11.6  |
| TC-2.11.6.2   | `test_painterly_edge_darken`         | R-2.11.6  |
| TC-2.11.7.1   | `test_pixel_art_resolution_quant`    | R-2.11.7  |
| TC-2.11.7.2   | `test_pixel_art_palette_match`       | R-2.11.7  |
| TC-2.11.7.3   | `test_pixel_art_nearest_neighbor`    | R-2.11.7  |
| TC-2.6.1.1    | `test_taa_jitter_sequence`           | R-2.6.1   |
| TC-2.6.1.2    | `test_taa_history_clamp_disocclude`  | R-2.6.1   |
| TC-2.6.2.1    | `test_fxaa_edge_smooth`              | R-2.6.2   |
| TC-2.6.4.1    | `test_tsr_upscale_psnr`              | R-2.6.4   |
| TC-2.6.5.1    | `test_checkerboard_reconstruct`      | R-2.6.5   |
| TC-2.6.9.1    | `test_smaa_neighborhood_blend`       | R-2.6.9   |

1. **TC-2.11.1.1** `test_outline_jump_flood_init` — Initialize jump-flood seed buffer from a 32x32
   stencil mask. Assert each seed pixel encodes its own coordinate.
   - Input: 32x32 stencil mask with one selected entity
   - Expected: seed buffer values match `(x, y)` for masked pixels, `(MAX, MAX)` elsewhere

2. **TC-2.11.1.2** `test_outline_color_apply` — Selected entity with
   `OutlineColor::RGBA(1, 0, 0, 1)` and width 2. Sample outline pixel adjacent to silhouette. Assert
   color matches.
   - Input: selected entity, outline color red, width 2
   - Expected: outline pixel rgba ≈ `(1, 0, 0, 1)` within 1/255

3. **TC-2.11.1.3** `test_outline_sobel_fallback` — Capability `compute = false`. Build outline pass
   list. Assert Sobel-based pass selected (no jump flood).
   - Input: `Capabilities { compute_shaders: false }`
   - Expected: outline pipeline = `Sobel`, no jump-flood passes registered

4. **TC-2.11.1.4** `test_outline_priority_overlap` — Two outlines overlap; A priority 1 (red), B
   priority 2 (blue). Assert overlap pixel is blue (B wins).
   - Input: two `OutlineRequest` with priorities 1 and 2
   - Expected: overlap pixel matches B's color

5. **TC-2.11.2.1** `test_highlight_pulse_modulation` — Pulse mode with frequency 1 Hz, base 0.5,
   amplitude 0.5. Sample at t=0 and t=0.5 s. Assert intensities ≈ 0.5 and 1.0.
   - Input: `HighlightState { mode: Pulse, freq: 1.0, base: 0.5, amp: 0.5 }`
   - Expected: `intensity(0.0) ≈ 0.5`, `intensity(0.25) ≈ 1.0`

6. **TC-2.11.2.2** `test_highlight_fresnel_rim` — Surface with normal `(0, 0, 1)` viewed from
   `(0, 0, 1)` (parallel) vs `(1, 0, 0)` (perpendicular). Assert rim factor near 0 vs near 1.
   - Input: normal and view vectors
   - Expected: `fresnel(parallel) < 0.05`, `fresnel(perp) > 0.9`

7. **TC-2.11.2.3** `test_highlight_inner_glow_blur` — Stencil-masked Gaussian blur on a 16x16
   region. Assert energy preserved (sum of weights ≈ 1.0).
   - Input: 16x16 stencil region, sigma 2.0
   - Expected: total weight ≈ 1.0 ± 0.001

8. **TC-2.11.3.1** `test_toon_band_count_3` — Quality `Mobile`, lighting NdotL = 0.7. Assert
   quantized output matches the middle band of a 3-band ramp.
   - Input: 3 bands at `[0.33, 0.66, 1.0]`, NdotL 0.7
   - Expected: quantized output == `0.66`

9. **TC-2.11.3.2** `test_toon_ramp_texture_lookup` — Custom ramp texture with three colors. Sample
   with NdotL `[0.1, 0.5, 0.9]`. Assert sampled colors match ramp at expected u.
   - Input: ramp texture, three NdotL samples
   - Expected: each sample matches ramp color at its u

10. **TC-2.11.3.3** `test_toon_specular_shape` — Specular factor over threshold creates circular
    highlight; below threshold suppressed. Assert step function behavior.
    - Input: `spec_threshold = 0.8`, samples `[0.5, 0.85, 0.9]`
    - Expected: `[0.0, 1.0, 1.0]` after step

11. **TC-2.11.4.1** `test_occlusion_volume_fade` — Roof entity inside `OcclusionVolume`; camera
    moves under it. Assert roof alpha fades from 1.0 to 0.0 over fade duration.
    - Input: `OcclusionVolume { mode: Volume, fade: 0.5 }`, scripted camera
    - Expected: alpha trajectory monotonically decreasing within 0.5 s

12. **TC-2.11.4.2** `test_occlusion_dither_pattern` — Dissolve dither pattern at 50% reveal. Assert
    checker mask matches reference Bayer dither.
    - Input: 8x8 Bayer matrix, threshold 0.5
    - Expected: pattern matches reference Bayer mask exactly

13. **TC-2.11.5.1** `test_xray_depth_compare` — Entity behind wall (depth > wall depth) with
    `XRayVisible` component. Assert silhouette pass uses `Greater` depth function.
    - Input: entity with XRayVisible, depth state
    - Expected: pipeline depth_compare == `Greater`, silhouette renders

14. **TC-2.11.5.2** `test_xray_priority_through_walls` — Two occluded entities, priorities 1 and 3,
    both behind walls. Assert only priority ≥ 2 entity renders silhouette.
    - Input: cutoff `min_priority = 2`
    - Expected: priority-3 entity drawn, priority-1 not drawn

15. **TC-2.11.6.1** `test_painterly_brush_radius` — Set brush radius 5 px on a noise-textured image.
    Assert local color cluster size ≈ brush radius.
    - Input: `PainterlyParams { brush_radius: 5.0 }`, synthetic input
    - Expected: clustered region width ≈ 5 px ± 1

16. **TC-2.11.6.2** `test_painterly_edge_darken` — Edge pixels detected via Sobel; assert their
    output luminance is multiplied by `edge_darken_factor`.
    - Input: edge mask, factor 0.5
    - Expected: edge pixel luma == 0.5 * input luma

17. **TC-2.11.7.1** `test_pixel_art_resolution_quant` — Render 1920x1080 source to 160x144 pixel art
    target. Assert internal RT dimensions match.
    - Input: `PixelArtParams { internal_res: (160, 144) }`
    - Expected: render target size == `(160, 144)`

18. **TC-2.11.7.2** `test_pixel_art_palette_match` — Quantize input pixel `(0.6, 0.4, 0.2)` to a
    16-color palette. Assert output equals nearest palette entry by Euclidean RGB distance.
    - Input: 16-color palette, sample pixel
    - Expected: output == nearest palette entry verified by brute-force scan

19. **TC-2.11.7.3** `test_pixel_art_nearest_neighbor` — Upscale 160x144 to 1920x1080 with
    nearest-neighbor filter. Assert each output pixel exactly matches its source pixel block.
    - Input: 160x144 input, 12x integer upscale
    - Expected: each 12x output region uniformly equals source pixel

20. **TC-2.6.1.1** `test_taa_jitter_sequence` — Frames 0..7 read TAA jitter offsets. Assert 8-step
    Halton sequence matches reference.
    - Input: `TaaJitter::halton(8)`
    - Expected: offsets match precomputed Halton(2,3) sequence

21. **TC-2.6.1.2** `test_taa_history_clamp_disocclude` — Disocclusion mask flagged for a pixel.
    Assert TAA neighborhood-clamps history sample to current frame's range.
    - Input: pixel with `disocclusion_flag = true`, history outside `[0, 1]`
    - Expected: clamped history within current 3x3 min/max range

22. **TC-2.6.2.1** `test_fxaa_edge_smooth` — Synthetic black/white step edge. Run FXAA. Assert edge
    pixel luma is intermediate (not 0 or 1).
    - Input: vertical step edge, FXAA pass
    - Expected: edge pixel luma in `(0.1, 0.9)`

23. **TC-2.6.4.1** `test_tsr_upscale_psnr` — Render 540p internal, upscale to 1080p with TSR.
    Compare against native 1080p reference. Assert PSNR > 35 dB.
    - Input: scene fixture, TSR quality preset
    - Expected: PSNR > 35 dB

24. **TC-2.6.5.1** `test_checkerboard_reconstruct` — Checkerboard pattern at half rate; assert
    reconstructed pattern matches native within 30 dB PSNR.
    - Input: 1080p output via checkerboard, native 1080p reference
    - Expected: PSNR > 30 dB

25. **TC-2.6.9.1** `test_smaa_neighborhood_blend` — SMAA edge detection, blending weight, and
    neighborhood blend run sequentially on a stair-step edge. Assert edge smoother than FXAA.
    - Input: stair-step input image
    - Expected: SMAA result has lower per-pixel error vs reference than FXAA

## Integration Tests

| ID           | Name                            | Req       |
|--------------|---------------------------------|-----------|
| TC-2.11.I.1  | `test_outline_full_scene`       | R-2.11.1  |
| TC-2.11.I.2  | `test_toon_lighting_full_pipe`  | R-2.11.3  |
| TC-2.11.I.3  | `test_pixel_art_full_pipeline`  | R-2.11.7  |
| TC-2.6.I.1   | `test_taa_motion_no_ghost`      | R-2.6.1   |
| TC-2.6.I.2   | `test_dlss_fallback_to_tsr`     | R-2.6.6   |
| TC-2.6.I.3   | `test_frame_gen_latency`        | R-2.6.7   |

1. **TC-2.11.I.1** `test_outline_full_scene` — Scene with 50 entities, 10 selected. Render with
   outline pass enabled. Assert 10 outlines visible, no false positives.
   - Input: scene fixture with 10 selected entities
   - Expected: outline pixel count > 0 only around selected entities

2. **TC-2.11.I.2** `test_toon_lighting_full_pipe` — Render full toon pipeline with directional
   light, ramp texture, and rim. Assert frame is non-NaN and bands quantized.
   - Input: toon material, 3-band ramp, directional sun
   - Expected: histogram of luminance shows discrete bands; no NaN

3. **TC-2.11.I.3** `test_pixel_art_full_pipeline` — Render at 320x180 internal, palette 32, then
   nearest-neighbor upscale to 1920x1080. Assert output pixel-blocks uniform and palette-bound.
   - Input: scene, pixel-art preset
   - Expected: 6x6 output blocks uniform; all pixels in palette set

4. **TC-2.6.I.1** `test_taa_motion_no_ghost` — Pan camera quickly across a scene. Assert TAA
   ghosting score (history-based blur) below threshold.
   - Input: scripted camera pan, TAA enabled
   - Expected: ghosting metric < 0.05 (per-pixel history reject rate threshold)

5. **TC-2.6.I.2** `test_dlss_fallback_to_tsr` — Disable DLSS SDK at runtime. Assert upscaler
   automatically falls back to TSR without error.
   - Input: `Upscaler::DLSS` requested, SDK absent
   - Expected: active upscaler == `TSR`, render succeeds

6. **TC-2.6.I.3** `test_frame_gen_latency` — Enable frame gen at 30 fps base. Measure end-to-end
   input-to-display latency. Assert ≤ baseline + 5 ms.
   - Input: scripted input events, frame gen on
   - Expected: `latency_with_fg - latency_no_fg <= 5 ms`

## Benchmarks

| ID           | Benchmark                       | Target    | Req       |
|--------------|---------------------------------|-----------|-----------|
| TC-2.11.B.1  | Outline pass (1080p)            | < 0.6 ms  | R-2.11.1  |
| TC-2.11.B.2  | Toon shading (1080p, 1k draws)  | < 1.5 ms  | R-2.11.3  |
| TC-2.11.B.3  | Pixel art quantize (320x180)    | < 0.2 ms  | R-2.11.7  |
| TC-2.6.B.1   | TAA resolve (1080p)             | < 0.8 ms  | R-2.6.1   |
| TC-2.6.B.2   | TSR upscale 540p→1080p          | < 1.2 ms  | R-2.6.4   |

1. **TC-2.11.B.1** — Full outline pipeline (mask → jump flood → composite) at 1080p with 10 selected
   entities. Wall time on reference GPU.

2. **TC-2.11.B.2** — Toon shading pass for 1000 draws at 1080p with 3-band ramp and shaped specular.
   Wall time.

3. **TC-2.11.B.3** — Resolution quantization, 16-color palette match, and nearest-neighbor upscale
   for a 320x180 frame. Wall time.

4. **TC-2.6.B.1** — TAA resolve pass at 1080p including history sampling, neighborhood clamp, and
   accumulation. Wall time.

5. **TC-2.6.B.2** — TSR upscale from 540p internal to 1080p output, quality preset. Wall time.
