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

### TC-2.6.3.1 FXAA Single-Pass Luma Filter

| # | Requirement |
|---|-------------|
| 1 | R-2.6.3     |
| 2 | R-2.6.3     |

1. **#1** — Apply FXAA to a 1-pixel-thick diagonal white line on black.
   - **Expected:** Line luminance redistributes across 2-3 px; step-edge frequency reduced.
2. **#2** — Apply FXAA to flat color region.
   - **Expected:** Output unchanged within 1/255 (FXAA respects flat regions).

### TC-2.6.8.1 Latency Reduction Submission Sync

| # | Requirement |
|---|-------------|
| 1 | R-2.6.8     |
| 2 | R-2.6.8     |

1. **#1** — Enable latency-reduction submission sync; profile command submission vs GPU end.
   - **Expected:** Submission-to-present delta reduced by >= 1 frame time vs sync-off baseline.
2. **#2** — Disable latency reduction.
   - **Expected:** Delta matches baseline.

### TC-2.7.1.1 Procedural Sky Atmosphere LUT

| # | Requirement |
|---|-------------|
| 1 | R-2.7.1     |
| 2 | R-2.7.1     |

1. **#1** — Compute atmosphere transmittance LUT for sun at zenith; sample at (0, 0).
   - **Expected:** Transmittance in `[0.7, 1.0]` (low atmosphere depth).
2. **#2** — Sample LUT at grazing sun angle.
   - **Expected:** Transmittance decreases monotonically toward 0.

### TC-2.7.2.1 Volumetric Fog Froxel Density

| # | Requirement |
|---|-------------|
| 1 | R-2.7.2     |
| 2 | R-2.7.2     |

1. **#1** — Fill froxel grid (160x90x64) with constant density 1.0.
   - **Expected:** All froxels sample density 1.0; shader reads match CPU data.
2. **#2** — Raymarch from camera through grid.
   - **Expected:** Integrated extinction proportional to travel distance.

### TC-2.7.3.1 Procedural Clouds Temporal Reprojection

| # | Requirement |
|---|-------------|
| 1 | R-2.7.3     |
| 2 | R-2.7.3     |

1. **#1** — Render clouds at quarter-res; reproject to full-res via motion vectors.
   - **Expected:** Full-res cloud output PSNR > 30 dB vs per-frame full-res reference.
2. **#2** — Large camera motion in one frame.
   - **Expected:** History rejected; next 4 frames re-converge without persistent ghost.

### TC-2.7.4.1 God Rays Screen-Space Pass

| # | Requirement |
|---|-------------|
| 1 | R-2.7.4     |
| 2 | R-2.7.4     |

1. **#1** — Screen-space god rays pass from sun behind a masked occluder.
   - **Expected:** Radial streaks emanate from sun position; intensity falls off with distance.
2. **#2** — Sun off-screen.
   - **Expected:** No streaks; pass early-outs.

### TC-2.7.5.1 Height Fog Analytical Integration

| # | Requirement |
|---|-------------|
| 1 | R-2.7.5     |
| 2 | R-2.7.5     |

1. **#1** — Evaluate exponential height fog integral between two points.
   - **Expected:** Result matches closed-form formula within 1%.
2. **#2** — Both points at zero altitude.
   - **Expected:** Integral equals density * distance.

### TC-2.7.6.1 FFT Ocean Height Field

| # | Requirement |
|---|-------------|
| 1 | R-2.7.6     |
| 2 | R-2.7.6     |

1. **#1** — Run 256-point FFT ocean spectrum; sample height field at t=0 s.
   - **Expected:** Mean height == 0; standard deviation matches Hs input parameter within 5%.
2. **#2** — Sample at t=1 s.
   - **Expected:** Height field non-identical to t=0; spectral energy preserved.

### TC-2.7.7.1 Heterogeneous Volume OpenVDB Load

| # | Requirement |
|---|-------------|
| 1 | R-2.7.7     |
| 2 | R-2.7.7     |

1. **#1** — Load OpenVDB file; query density at known voxel.
   - **Expected:** Returned density matches file content within 1/256.
2. **#2** — Sample outside volume bounds.
   - **Expected:** Density == 0.

### TC-2.7.8.1 Voxel Clouds SDF Acceleration

| # | Requirement |
|---|-------------|
| 1 | R-2.7.8     |
| 2 | R-2.7.8     |

1. **#1** — Build SDF for a sphere voxel cloud; query SDF at cloud surface.
   - **Expected:** SDF value near 0 at surface (within voxel size).
2. **#2** — Raymarch clouds using SDF stride.
   - **Expected:** Step count at least 2x smaller than fixed-stride baseline.

### TC-2.7.9.1 Breaking Waves Coons Surface

| # | Requirement |
|---|-------------|
| 1 | R-2.7.9     |
| 2 | R-2.7.9     |

1. **#1** — Evaluate Coons patch for wave crest control curve set.
   - **Expected:** Patch passes through boundary curves; C0 continuity preserved.
2. **#2** — Sample patch normal at center.
   - **Expected:** Normal length == 1; orientation matches expected wave up-vector.

### TC-2.7.10.1 Weather State Machine Transition

| # | Requirement |
|---|-------------|
| 1 | R-2.7.10    |
| 2 | R-2.7.10    |

1. **#1** — Start in `Clear`, request `Rain`, step state machine over 10 s.
   - **Expected:** `transition_progress` ramps 0 -> 1; final state == `Rain`; wetness > 0.5.
2. **#2** — Interrupt transition at t=5 s with `Snow` request.
   - **Expected:** Transition restarts toward `Snow`; no NaN or stuck state.

### TC-2.8.1.1 Strand Hair Marschner BSDF

| # | Requirement |
|---|-------------|
| 1 | R-2.8.1     |
| 2 | R-2.8.1     |

1. **#1** — Evaluate Marschner R lobe for a hair strand with view at 30 deg.
   - **Expected:** Lobe peak at predicted angle within 1 deg; energy normalized.
2. **#2** — Evaluate TT and TRT lobes.
   - **Expected:** Both positive; combined R + TT + TRT energy <= 1.0.

### TC-2.8.2.1 Card Hair Layered Alpha

| # | Requirement |
|---|-------------|
| 1 | R-2.8.2     |
| 2 | R-2.8.2     |

1. **#1** — Render 5 card strips with alpha textures; composite.
   - **Expected:** Output alpha matches `1 - prod(1 - alpha_i)` within 1/255.
2. **#2** — Disable card hair rendering.
   - **Expected:** Output transparent in card regions.

### TC-2.8.3.1 Hair LOD Cross-Fade

| # | Requirement |
|---|-------------|
| 1 | R-2.8.3     |
| 2 | R-2.8.3     |

1. **#1** — Trigger LOD cross-fade from Strand to Card at distance 8 m.
   - **Expected:** Cross-fade blends both methods over 0.5 s; no pop in silhouette.
2. **#2** — Trigger distant Card to MeshProxy transition.
   - **Expected:** Cross-fade smooth; camera shake doesn't cause LOD thrash.

### TC-2.8.4.1 Eye Layer Composition

| # | Requirement |
|---|-------------|
| 1 | R-2.8.4     |
| 2 | R-2.8.4     |

1. **#1** — Render eye with cornea IOR 1.376, iris depth 0.5 mm.
   - **Expected:** Iris appears shifted by refraction; cornea reflection visible at rim.
2. **#2** — Rotate camera 90 deg.
   - **Expected:** Iris parallax visible; sclera color unchanged.

### TC-2.8.5.1 Cloth Fuzz Sheen BRDF

| # | Requirement |
|---|-------------|
| 1 | R-2.8.5     |
| 2 | R-2.8.5     |

1. **#1** — Evaluate cloth sheen BRDF at grazing angle.
   - **Expected:** Sheen intensity > diffuse contribution.
2. **#2** — Evaluate at normal incidence.
   - **Expected:** Sheen contribution minimal; diffuse dominates.

### TC-2.8.6.1 Skin SSS Burley Diffusion

| # | Requirement |
|---|-------------|
| 1 | R-2.8.6     |
| 2 | R-2.8.6     |

1. **#1** — Apply Burley diffusion profile to a point light source on skin.
   - **Expected:** Irradiance diffused radially matching reference profile within 5%.
2. **#2** — Use `PreintegratedLut` profile on same setup.
   - **Expected:** Output matches LUT-based reference within 1/255.

### TC-2.8.7.1 Hair Compute Software Rasterizer

| # | Requirement |
|---|-------------|
| 1 | R-2.8.7     |
| 2 | R-2.8.7     |

1. **#1** — Rasterize 100k sub-pixel hair strands via compute shader.
   - **Expected:** Output coverage buffer has non-zero pixels; visible strands count matches
     analytic coverage within 5%.
2. **#2** — Compare to hardware raster at strand width 2 px.
   - **Expected:** Output PSNR > 35 dB vs hardware reference.

### TC-2.8.8.1 Peach Fuzz Screen-Space Layer

| # | Requirement |
|---|-------------|
| 1 | R-2.8.8     |
| 2 | R-2.8.8     |

1. **#1** — Render close-up face with peach fuzz at screen-size threshold 10 px.
   - **Expected:** Fuzz silhouette visible at face rim; no fuzz on background pixels.
2. **#2** — Move camera back beyond threshold.
   - **Expected:** Fuzz pass skipped; no visible fuzz.

### TC-2.8.9.1 Biometric Skin Model

| # | Requirement |
|---|-------------|
| 1 | R-2.8.9     |
| 2 | R-2.8.9     |

1. **#1** — Compose biometric skin with melanin_scale=1.0.
   - **Expected:** Skin base color matches reference dark-skin calibration within dE < 3.
2. **#2** — Set melanin_scale=0.1, evaluate blood_map influence.
   - **Expected:** Base color matches lighter calibration; redness from blood_map visible.

### TC-11.2.1.1 Deferred Decal G-Buffer Modification

| # | Requirement |
|---|-------------|
| 1 | R-11.2.1    |
| 2 | R-11.2.1    |

1. **#1** — Project deferred decal onto G-buffer with normal override channel.
   - **Expected:** Normal buffer under decal bounds replaced with decal normal; outside bounds
     unchanged.
2. **#2** — Blend mode `Multiply`.
   - **Expected:** Base color multiplied; no alpha channel write.

### TC-11.2.2.1 Mesh Decal Tangent-Space Normal

| # | Requirement |
|---|-------------|
| 1 | R-11.2.2    |
| 2 | R-11.2.2    |

1. **#1** — Project mesh decal onto a curved surface; sample tangent-space normal.
   - **Expected:** Normal correctly transformed by surface TBN; no z-fighting.
2. **#2** — Update surface deformation.
   - **Expected:** Decal follows surface; no visual drift.

### TC-11.2.3.1 Decal Atlas LRU Eviction

| # | Requirement |
|---|-------------|
| 1 | R-11.2.3    |
| 2 | R-11.2.3    |

1. **#1** — Fill atlas with 256 decals; request 257th.
   - **Expected:** Least-recently-used decal evicted; new decal allocated.
2. **#2** — Touch decal 0, then request 258th.
   - **Expected:** Decal 0 retained; decal 1 evicted.

### TC-11.2.4.1 Decal Priority Layering

| # | Requirement |
|---|-------------|
| 1 | R-11.2.4    |
| 2 | R-11.2.4    |

1. **#1** — Two overlapping decals, priorities 1 and 5.
   - **Expected:** Priority-5 decal drawn on top; priority-1 blended below.
2. **#2** — Decal fade_params expiration.
   - **Expected:** Decal alpha fades over lifetime; removed at t >= lifetime.

### TC-2.12.1.1 Glass Refraction IOR

| # | Requirement |
|---|-------------|
| 1 | R-2.12.1    |
| 2 | R-2.12.1    |

1. **#1** — Render glass sphere with IOR 1.5 over checkered background.
   - **Expected:** Background refracted; shift matches Snell's law within 2 px at sphere rim.
2. **#2** — Raise IOR to 2.4 (diamond).
   - **Expected:** Larger refraction shift; total internal reflection visible at grazing.

### TC-2.12.2.1 Glass Dispersion Chromatic

| # | Requirement |
|---|-------------|
| 1 | R-2.12.2    |
| 2 | R-2.12.2    |

1. **#1** — Enable chromatic dispersion on a glass prism.
   - **Expected:** R, G, B channels diverge across refraction path; spectrum visible on exit.
2. **#2** — Set dispersion == 0.
   - **Expected:** Channels recombine; no spectrum; matches non-dispersive reference.

### TC-2.12.3.1 Emissive HDR Bloom Contribution

| # | Requirement |
|---|-------------|
| 1 | R-2.12.3    |
| 2 | R-2.12.3    |

1. **#1** — Emissive material intensity 10.0; render and capture post-tonemap.
   - **Expected:** Emissive pixel bloom halo visible; bloom-only buffer non-zero.
2. **#2** — Animate emissive intensity over 60 frames.
   - **Expected:** Bloom halo tracks intensity per frame; no 1-frame lag.

### TC-2.12.4.1 Heightmap POM Steps

| # | Requirement |
|---|-------------|
| 1 | R-2.12.4    |
| 2 | R-2.12.4    |

1. **#1** — Render brick wall with POM at 32 steps, height 0.05 m.
   - **Expected:** Silhouette shows parallax depth; bricks appear recessed.
2. **#2** — Switch to tessellation technique.
   - **Expected:** True geometry displaces; silhouette deforms consistently.

### TC-2.12.5.1 Fabric Sheen BRDF

| # | Requirement |
|---|-------------|
| 1 | R-2.12.5    |
| 2 | R-2.12.5    |

1. **#1** — Evaluate fabric sheen at roughness 0.3, view grazing 80 deg.
   - **Expected:** Sheen peak > 0.8 relative intensity.
2. **#2** — Evaluate at view 0 deg.
   - **Expected:** Sheen close to 0; diffuse dominates.

### TC-2.12.6.1 Metal Anisotropy Direction

| # | Requirement |
|---|-------------|
| 1 | R-2.12.6    |
| 2 | R-2.12.6    |

1. **#1** — Anisotropic brushed metal with direction map; view along anisotropy axis.
   - **Expected:** Highlight elongates along anisotropy direction.
2. **#2** — Rotate view 90 deg.
   - **Expected:** Highlight elongation rotates; shape preserved.

### TC-2.12.7.1 Rubber Wax Soft Surface

| # | Requirement |
|---|-------------|
| 1 | R-2.12.7    |
| 2 | R-2.12.7    |

1. **#1** — Evaluate rubber material with roughness 0.9 under point light.
   - **Expected:** Diffuse-dominant shading; no strong specular peak.
2. **#2** — Wax material with subsurface thickness 2 mm.
   - **Expected:** Translucent glow from back-lit thin edges; matches reference.

### TC-2.12.8.1 Clearcoat Multi-Layer

| # | Requirement |
|---|-------------|
| 1 | R-2.12.8    |
| 2 | R-2.12.8    |

1. **#1** — Evaluate clearcoat stack: base metal + clearcoat 1.0 + clearcoat_ior 1.5.
   - **Expected:** Two-lobe specular visible; clearcoat lobe sharper, base lobe softer.
2. **#2** — Disable clearcoat.
   - **Expected:** Single specular lobe from base metal only.

### TC-2.12.9.1 Material Graph Compile To Shader

| # | Requirement |
|---|-------------|
| 1 | R-2.12.9    |
| 2 | R-2.12.9    |

1. **#1** — Build `MaterialGraph` with 10 nodes, 1 output; call `compile(PlatformTier::Desktop)`.
   - **Expected:** Returns `CompiledMaterial` with valid shader bytecode; no compile errors.
2. **#2** — Disconnect a node mid-graph.
   - **Expected:** `validate()` returns error listing the disconnected output.

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

<!-- THIN: design section lacks detail -->
### TC-2.11.1.I1 Outline Sobel Pipeline E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.1.1 |

1. **#1** — Enable Sobel outline pipeline; render 50 entity scene with 3 selected.
   - **Expected:** 3 outlines visible; outline color matches request; no artifacts on unselected.

<!-- THIN: design section lacks detail -->
### TC-2.11.1.I2 Outline Jump-Flood Pipeline E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.1.2 |

1. **#1** — Enable JFA pipeline with width 4; render scene.
   - **Expected:** Outline width 4 px; smooth contour; no gaps at entity corners.

<!-- THIN: design section lacks detail -->
### TC-2.11.1.I3 Outline Shell Extrusion Pipeline E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.1.3 |

1. **#1** — Enable shell extrusion; render a character with cel shading.
   - **Expected:** Extruded backface shell visible as outline; constant screen-space width.

<!-- THIN: design section lacks detail -->
### TC-2.11.2.I1 Highlight Gaussian Glow Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.11.2.1 |

1. **#1** — Apply Gaussian glow to selected object; render frame.
   - **Expected:** Glow blooms beyond silhouette; intensity modulated by highlight state.

<!-- THIN: design section lacks detail -->
### TC-2.11.2.I2 Highlight Fresnel Rim Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.11.2.2 |

1. **#1** — Apply fresnel rim to selected object; camera orbits.
   - **Expected:** Rim follows silhouette at grazing angles; absent at head-on view.

<!-- THIN: design section lacks detail -->
### TC-2.11.2.I3 Highlight Inner Glow Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.11.2.3 |

1. **#1** — Apply inner glow on stencil-masked mesh.
   - **Expected:** Glow concentrated inside silhouette; no spill outside.

<!-- THIN: design section lacks detail -->
### TC-2.11.3.I1 Toon Shading Band Ramp E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.3.1 |

1. **#1** — Render character with 3-band toon ramp and directional light.
   - **Expected:** Shading quantized into 3 discrete bands; output luminance histogram shows 3
     peaks.

<!-- THIN: design section lacks detail -->
### TC-2.11.3.I2 Toon Specular Shape E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.3.2 |

1. **#1** — Render character with `SpecularShape::Star`.
   - **Expected:** Specular highlight resembles star pattern.

<!-- THIN: design section lacks detail -->
### TC-2.11.3.I3 Toon Hatching Style E2E

| # | Requirement |
|---|-------------|
| 1 | US-2.11.3.3 |

1. **#1** — Enable hatching texture; render shaded sphere.
   - **Expected:** Crosshatch pattern appears in shaded regions; density varies with luminance.

<!-- THIN: design section lacks detail -->
### TC-2.11.4.I1 Cut-Through Roof Fade

| # | Requirement |
|---|-------------|
| 1 | US-2.11.4.1 |

1. **#1** — Camera moves under building roof; cut-through volume triggers.
   - **Expected:** Roof fades out smoothly over 0.3 s; restored when camera exits.

<!-- THIN: design section lacks detail -->
### TC-2.11.4.I2 Cut-Through Ray-Based

| # | Requirement |
|---|-------------|
| 1 | US-2.11.4.2 |

1. **#1** — Use ray-based fade mode; trace from camera to player.
   - **Expected:** Obstructing geometry fades; un-obstructed unchanged.

<!-- THIN: design section lacks detail -->
### TC-2.11.4.I3 Cut-Through Layer-Based

| # | Requirement |
|---|-------------|
| 1 | US-2.11.4.3 |

1. **#1** — Tag roof objects with fade layer; enable layer-based fade.
   - **Expected:** Only tagged objects fade when camera enters volume.

<!-- THIN: design section lacks detail -->
### TC-2.11.5.I1 X-Ray Silhouette Through Walls

| # | Requirement |
|---|-------------|
| 1 | US-2.11.5.1 |

1. **#1** — Render a teammate behind a wall with `XRayVisible`.
   - **Expected:** Teammate silhouette visible through wall; solid geometry culls normally.

<!-- THIN: design section lacks detail -->
### TC-2.11.5.I2 X-Ray Priority Filter

| # | Requirement |
|---|-------------|
| 1 | US-2.11.5.2 |

1. **#1** — Render two occluded entities with priorities 1 and 3; min priority filter 2.
   - **Expected:** Priority-3 visible; priority-1 hidden.

<!-- THIN: design section lacks detail -->
### TC-2.11.5.I3 X-Ray Style Dashed

| # | Requirement |
|---|-------------|
| 1 | US-2.11.5.3 |

1. **#1** — Use `XRayStyle::Dashed` on occluded entity.
   - **Expected:** Silhouette rendered in dashed pattern; matches style.

<!-- THIN: design section lacks detail -->
### TC-2.12.1.I1 Glass Crystal Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.1.1 |

1. **#1** — Render glass sphere over patterned floor.
   - **Expected:** Floor pattern refracts through sphere; matches Snell's law visually.

<!-- THIN: design section lacks detail -->
### TC-2.12.1.I2 Glass Dispersion Prism Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.1.2 |

1. **#1** — Render triangular prism with white light; enable dispersion.
   - **Expected:** Spectrum visible exiting prism; colors separated by wavelength.

<!-- THIN: design section lacks detail -->
### TC-2.12.1.I3 Glass Absorption Beer Lambert

| # | Requirement |
|---|-------------|
| 1 | US-2.12.1.3 |

1. **#1** — Thick tinted glass block; light passes through.
   - **Expected:** Transmitted light color matches `exp(-absorption * distance)` within 5%.

<!-- THIN: design section lacks detail -->
### TC-2.12.3.I1 Emissive Animated Sign

| # | Requirement |
|---|-------------|
| 1 | US-2.12.3.1 |

1. **#1** — Animated emissive sign with pulse intensity; render 60 frames.
   - **Expected:** Pulse visible; bloom halo tracks intensity per frame.

<!-- THIN: design section lacks detail -->
### TC-2.12.3.I2 Emissive HDR Bloom Contribution

| # | Requirement |
|---|-------------|
| 1 | US-2.12.3.2 |

1. **#1** — Emissive lava material in a cave scene; tonemap output.
   - **Expected:** Bloom halo around lava visible; emissive illuminates nearby geometry via IBL.

<!-- THIN: design section lacks detail -->
### TC-2.12.3.I3 Emissive Scrolling Texture

| # | Requirement |
|---|-------------|
| 1 | US-2.12.3.3 |

1. **#1** — Scrolling emissive texture over a surface; render.
   - **Expected:** Pattern scrolls at requested speed; UV offset animated consistently.

<!-- THIN: design section lacks detail -->
### TC-2.12.4.I1 Heightmap Tessellation Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.4.1 |

1. **#1** — Apply tessellated heightmap to terrain patch.
   - **Expected:** Geometry displaces per heightmap; silhouette shows deformation.

<!-- THIN: design section lacks detail -->
### TC-2.12.4.I2 Heightmap POM Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.4.2 |

1. **#1** — Apply POM to a stone wall.
   - **Expected:** Parallax depth visible without actual tessellation; silhouette flat.

<!-- THIN: design section lacks detail -->
### TC-2.12.4.I3 Heightmap LOD Transition

| # | Requirement |
|---|-------------|
| 1 | US-2.12.4.3 |

1. **#1** — Distance-based transition from tessellation near to POM far.
   - **Expected:** Transition smooth; no visual pop at LOD boundary.

<!-- THIN: design section lacks detail -->
### TC-2.12.5.I1 Fabric Velvet Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.5.1 |

1. **#1** — Render velvet sofa with fabric material under point light.
   - **Expected:** Sheen at grazing edges; diffuse at flat surfaces; no hard specular lobe.

<!-- THIN: design section lacks detail -->
### TC-2.12.5.I2 Fabric Silk Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.5.2 |

1. **#1** — Render silk fabric with subsurface transmission.
   - **Expected:** Backlit regions transmit light; front-lit regions reflect with sheen.

<!-- THIN: design section lacks detail -->
### TC-2.12.5.I3 Fabric Cotton Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.5.3 |

1. **#1** — Cotton with high fuzz intensity.
   - **Expected:** Matte appearance with visible fuzz silhouette; no shiny highlights.

<!-- THIN: design section lacks detail -->
### TC-2.12.6.I1 Metal Brushed Aluminum Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.6.1 |

1. **#1** — Brushed aluminum sheet with anisotropy 0.8; directional light overhead.
   - **Expected:** Elongated anisotropic highlight visible; matches direction map.

<!-- THIN: design section lacks detail -->
### TC-2.12.6.I2 Wood Weathering Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.6.2 |

1. **#1** — Aged wood plank with weathering map; render.
   - **Expected:** Weathering darkens grain recesses; bright top surfaces retain color.

<!-- THIN: design section lacks detail -->
### TC-2.12.6.I3 Stone Material Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.6.3 |

1. **#1** — Stone wall with micronormals; render.
   - **Expected:** Surface detail visible; highlights match surface bumps.

<!-- THIN: design section lacks detail -->
### TC-2.12.7.I1 Rubber Tire Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.7.1 |

1. **#1** — Car tire with rubber material.
   - **Expected:** Dark diffuse, minimal specular; matches physically plausible rubber.

<!-- THIN: design section lacks detail -->
### TC-2.12.7.I2 Wax Candle Full Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.7.2 |

1. **#1** — Lit candle with wax translucent material.
   - **Expected:** Upper wax glows from interior flame; lower wax opaque.

<!-- THIN: design section lacks detail -->
### TC-2.12.8.I1 Clearcoat Car Paint Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.8.1 |

1. **#1** — Render car hood with clearcoat over metallic base.
   - **Expected:** Double highlight visible: sharp clearcoat + soft base; matches reference.

<!-- THIN: design section lacks detail -->
### TC-2.12.8.I2 Clearcoat Lacquered Wood Scene

| # | Requirement |
|---|-------------|
| 1 | US-2.12.8.2 |

1. **#1** — Lacquered wood finish with clearcoat over wood texture.
   - **Expected:** Wood grain visible through clearcoat; top layer glossy.

<!-- THIN: design section lacks detail -->
### TC-2.12.8.I3 Clearcoat Multi-Layer Paint

| # | Requirement |
|---|-------------|
| 1 | US-2.12.8.3 |

1. **#1** — Render a 4-layer candy paint setup.
   - **Expected:** Layer cost < 50% baseline for equivalent single shader; visual matches LUT
     reference.

<!-- THIN: design section lacks detail -->
### TC-2.12.9.I1 Custom Material Graph Editor Save

| # | Requirement |
|---|-------------|
| 1 | US-2.12.9.1 |

1. **#1** — Build a custom graph with noise node + mul node; save to asset.
   - **Expected:** Asset serialized; reload produces identical graph and compiled shader.

<!-- THIN: design section lacks detail -->
### TC-2.12.9.I2 Custom Material Graph Validate

| # | Requirement |
|---|-------------|
| 1 | US-2.12.9.2 |

1. **#1** — Introduce cycle in graph connectivity; call `validate()`.
   - **Expected:** Returns `Err` indicating cycle.

<!-- THIN: design section lacks detail -->
### TC-2.12.9.I3 Custom Material Graph Hot Swap

| # | Requirement |
|---|-------------|
| 1 | US-2.12.9.3 |

1. **#1** — Edit graph at runtime; request recompile.
   - **Expected:** Recompile completes < 1 s incremental; active material refreshes.

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
