# R-2.5 -- Advanced Rendering (Ray Tracing and GI) Requirements

## Ray Tracing Infrastructure

| ID      | Derived From                                              |
|---------|-----------------------------------------------------------|
| R-2.5.1 | [F-2.5.1](../../features/rendering/advanced-rendering.md) |
| R-2.5.2 | [F-2.5.2](../../features/rendering/advanced-rendering.md) |
| R-2.5.3 | [F-2.5.3](../../features/rendering/advanced-rendering.md) |

1. **R-2.5.1** — The engine **SHALL** build bottom-level acceleration structures (BLAS) from meshlet
   geometry with post-build compaction to reduce memory, and rebuild or refit top-level acceleration
   structures (TLAS) each frame for dynamic scenes.
   - **Rationale:** Acceleration structures are the prerequisite for all hardware ray tracing
     features; BLAS compaction reduces VRAM waste and TLAS refit supports dynamic objects.
   - **Verification:** Build a BLAS from a mesh with 10,000 meshlets. Measure VRAM after compaction
     and verify at least 20% reduction compared to pre-compaction size. Refit the TLAS after moving
     50% of scene objects and verify ray intersection results remain correct for all moved objects.
     Measure TLAS refit completes in under 1ms for a scene with 10,000 instances.
2. **R-2.5.2** — The engine **SHALL** provide a hybrid reflection system combining screen-space ray
   marching (SSR) for nearby surfaces with hardware ray tracing for off-screen and rough
   reflections, using a roughness threshold to determine when RT rays supplement SSR, with temporal
   denoising for output stability.
   - **Rationale:** Hybrid reflections use cheap SSR where screen data is available and fall back to
     accurate RT for off-screen content and rough surfaces, balancing quality and performance.
   - **Verification:** Render a glossy floor reflecting an object. Verify SSR resolves the on-screen
     reflection. Move the reflected object off-screen and verify RT produces a correct reflection
     where SSR fails. Increase surface roughness past the threshold and verify RT rays activate.
     Confirm temporal denoising eliminates frame-to-frame noise without ghosting.
3. **R-2.5.3** — The engine **SHALL** trace secondary diffuse rays from surface positions to collect
   one-bounce indirect illumination, feeding the output into the global illumination system.
   - **Rationale:** One-bounce diffuse ray tracing captures indirect color bleeding that
     screen-space techniques miss, providing physically accurate indirect illumination input.
   - **Verification:** Render a Cornell box with colored walls. Verify color bleeding from the red
     and green walls onto the white surfaces. Compare against a path-traced reference and verify
     PSNR above 30 dB for the indirect-only component. Verify output integrates with the GI system
     without double-counting direct illumination.

## Global Illumination

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.5.4  | [F-2.5.4](../../features/rendering/advanced-rendering.md)  |
| R-2.5.7  | [F-2.5.7](../../features/rendering/advanced-rendering.md)  |
| R-2.5.8  | [F-2.5.8](../../features/rendering/advanced-rendering.md)  |
| R-2.5.14 | [F-2.5.14](../../features/rendering/advanced-rendering.md) |
| R-2.5.15 | [F-2.5.15](../../features/rendering/advanced-rendering.md) |

1. **R-2.5.4** — The engine **SHALL** implement Dynamic Diffuse Global Illumination using a probe
   grid where each probe casts rays via the TLAS, accumulates irradiance and visibility into
   octahedral atlas textures with temporal hysteresis, and surfaces sample nearby probes for diffuse
   indirect lighting producing color bleeding and ambient fill.
   - **Rationale:** DDGI provides dynamic indirect lighting that updates in real time with scene
     changes, unlike baked lightmaps.
   - **Verification:** Place a DDGI probe grid in a Cornell box. Verify color bleeding from colored
     walls appears on nearby surfaces within 2 seconds of convergence. Move a colored object and
     verify indirect lighting updates dynamically. Inspect probe atlas textures and verify
     octahedral encoding with correct irradiance and visibility data.
2. **R-2.5.7** — The engine **SHALL** implement surfel-based GI that discretizes scene geometry into
   surface elements from the G-Buffer each frame, caches irradiance and depth in a clipmap probe
   system with octahedral representation, traces rays from surfels via hardware RT with ray guiding
   and ray binning, and applies multi-scale mean estimation for temporal hysteresis, scaling to
   environments of arbitrary size with predictable performance.
   - **Rationale:** Surfel-based GI requires no precomputation, adapts to fully dynamic scenes, and
     provides predictable performance independent of environment size.
   - **Verification:** Render a large open-world scene with surfel GI. Verify indirect lighting
     converges within 1 second and tracks dynamic light changes in real time. Measure that GPU cost
     remains within 10% variance when doubling the environment size. Verify separate character
     lighting probes produce correct indirect illumination on animated characters.
3. **R-2.5.8** — The engine **SHALL** provide a ReSTIR (reservoir-based spatiotemporal importance
   resampling) framework with ReSTIR DI for direct illumination from thousands of simultaneous
   lights with physically correct soft shadows, and ReSTIR GI for screen-space resampling of
   secondary surfaces for multi-bounce indirect lighting, maintaining per-pixel reservoirs across
   frames with spatial and temporal reuse.
   - **Rationale:** ReSTIR converges to correct illumination from massive light counts with a fixed
     per-pixel ray budget by resampling across space and time.
   - **Verification:** Render a scene with 5,000 lights using ReSTIR DI. Verify all lights
     contribute to the converged result. Measure that per-frame cost is independent of light count
     (within 10% variance). Enable ReSTIR GI and verify multi-bounce indirect illumination matches a
     path-traced reference within PSNR 30 dB after 60 frames of temporal accumulation.
4. **R-2.5.14** — The engine **SHALL** provide a rasterization-only GI system that voxelizes the
   world (albedo, normals, translucency) into a 3D voxel cache that can be re-lit for varying times
   of day without re-voxelization, combined with irradiance volumes and improved cubemap reflections
   for indirect lighting without hardware ray tracing.
   - **Rationale:** Voxel-based GI provides convincing indirect lighting on hardware without RT
     capability, using rasterization-only techniques with acceptable quality.
   - **Verification:** Render a scene with voxel GI on hardware without RT support. Verify indirect
     lighting (color bleeding, ambient fill) is visible. Change the directional light angle (time of
     day) and verify re-lighting without re-voxelization completes in under 2ms. Compare against a
     path-traced reference and verify PSNR above 25 dB for the indirect component.
5. **R-2.5.15** — The engine **SHALL** provide a runtime-trained neural network that estimates
   indirect radiance at arbitrary scene points, terminating the majority of path-traced paths early
   (after 2-3 bounces) and predicting remaining light transport, using a small percentage of
   full-length paths for ongoing training, supporting entirely dynamic scenes.
   - **Rationale:** Neural radiance caching reduces path tracing noise by replacing deep bounces
     with learned predictions, enabling higher quality at lower ray budgets for dynamic scenes.
   - **Verification:** Enable the radiance cache on a path-traced scene. Verify noise is visibly
     reduced compared to uncached path tracing at the same sample count. Move scene objects and
     verify the cache adapts within 2 seconds. Verify the cache uses full-length training paths
     (confirm via statistics that fewer than 5% of paths run to maximum depth).

## Path Tracing and Production Rendering

| ID      | Derived From                                              |
|---------|-----------------------------------------------------------|
| R-2.5.5 | [F-2.5.5](../../features/rendering/advanced-rendering.md) |
| R-2.5.6 | [F-2.5.6](../../features/rendering/advanced-rendering.md) |
| R-2.5.9 | [F-2.5.9](../../features/rendering/advanced-rendering.md) |

1. **R-2.5.5** — The engine **SHALL** provide a progressive unbiased hardware-accelerated path
   tracer supporting full light transport (diffuse, specular, transmission) with configurable
   samples per pixel and maximum bounce count, supporting all material types, volumetrics, and sky
   atmosphere for ground-truth reference and cinematic rendering.
   - **Rationale:** A reference path tracer provides ground-truth images for validating real-time
     rendering accuracy and enables cinematic-quality offline rendering.
   - **Verification:** Render a Cornell box at 4096 spp. Verify output converges to an analytical
     reference within 1% relative error for diffuse surfaces. Verify specular, transmission, and
     volumetric paths produce physically correct results. Confirm the renderer is unbiased by
     verifying error decreases proportionally to 1/sqrt(spp).
2. **R-2.5.6** — The engine **SHALL** evaluate volumetric scattering computation through translucent
   media during shadow ray evaluation, computing light transport through the material's thickness
   using the subsurface profile when a shadow ray hits a subsurface material.
   - **Rationale:** Ray-traced subsurface transmission produces physically correct light penetration
     through thin geometry (ears, fingers, wax) that screen-space SSS cannot capture.
   - **Verification:** Render a thin translucent object (e.g., ear geometry) backlit by a strong
     light. Verify visible red/orange transmission through the thin regions. Vary material thickness
     and verify transmission attenuates exponentially. Compare against a volumetric reference and
     verify transmitted radiance matches within 15% relative error.
3. **R-2.5.9** — The engine **SHALL** provide tiered real-time path tracing for production game
   rendering with configurable quality tiers from direct-RT-only through multi-bounce full path
   tracing, falling back to rasterized GI rather than terminating to black after the final
   path-tracing bounce, paired with neural denoising and temporal upscaling for real-time frame
   rates.
   - **Rationale:** Tiered production path tracing enables scalable quality from basic RT effects to
     full path tracing, with rasterized GI fallback preventing energy loss at path termination.
   - **Verification:** Render a scene at each quality tier. Verify the lowest tier applies direct RT
     only and the highest tier produces multi-bounce results. Verify path termination falls back to
     rasterized GI (not black) by comparing terminated paths against a GI-only reference. Measure
     that the highest tier maintains at least 30 FPS at 1080p with denoising and upscaling enabled.

## RT Hardware Optimizations

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.5.10 | [F-2.5.10](../../features/rendering/advanced-rendering.md) |
| R-2.5.11 | [F-2.5.11](../../features/rendering/advanced-rendering.md) |
| R-2.5.12 | [F-2.5.12](../../features/rendering/advanced-rendering.md) |
| R-2.5.13 | [F-2.5.13](../../features/rendering/advanced-rendering.md) |

1. **R-2.5.10** — The engine **SHALL** annotate acceleration structures with per-micro-triangle
   opacity classification (opaque, transparent, or unknown) for alpha-tested geometry, enabling the
   hardware to skip any-hit shader invocations for micro-triangles classified as fully opaque or
   fully transparent.
   - **Rationale:** Opacity micromaps accelerate ray intersection testing for alpha-tested geometry
     (vegetation, hair) by eliminating unnecessary any-hit shader invocations.
   - **Verification:** Build opacity micromaps for vegetation geometry with alpha masks. Measure ray
     intersection throughput with and without micromaps and verify at least 30% improvement for
     vegetation-heavy scenes. Verify visual output is identical with and without micromaps enabled.
2. **R-2.5.11** — The engine **SHALL** support hardware-assisted reordering of ray tracing shader
   invocations to improve execution coherency, grouping similar shader invocations into coherent
   wavefronts after intersection testing scatters rays to diverse material shaders.
   - **Rationale:** SER reduces shader divergence caused by incoherent ray hits, improving GPU
     occupancy and throughput for material-diverse scenes.
   - **Verification:** Render a scene with 50+ distinct materials. Enable SER and measure GPU
     occupancy via hardware counters. Verify occupancy improves by at least 10% compared to SER
     disabled. Verify visual output is identical with and without SER enabled.
3. **R-2.5.12** — The engine **SHALL** provide an AI-trained neural network denoiser for ray-traced
   and path-traced output operating on noisy per-pixel radiance with motion vectors and depth,
   producing temporally stable results with fallback to conventional NRD denoiser algorithms when
   neural hardware is unavailable.
   - **Rationale:** Neural denoising produces higher-quality temporally stable results from fewer
     samples than hand-tuned bilateral/wavelet filters, reducing the ray budget required for clean
     output.
   - **Verification:** Denoise a 1-spp path-traced image with the neural denoiser. Compare against a
     4096-spp reference and verify PSNR above 30 dB. Verify temporal stability across 120 frames of
     camera motion with no ghosting or flickering. Disable neural hardware and verify the NRD
     fallback activates and produces acceptable output (PSNR above 25 dB at 1 spp).
4. **R-2.5.13** — The engine **SHALL** provide ray-traced lens flare simulation where lens elements
   are modeled as refractive surfaces and rays from bright scene sources are traced through the lens
   system to produce physically accurate ghost and halo artifacts.
   - **Rationale:** Ray-traced lens flares match real optical behavior, producing physically
     grounded artifacts that image-based approximations cannot reproduce.
   - **Verification:** Position a bright light source in the scene. Verify lens ghosts and halos
     appear at physically correct positions based on the lens element configuration. Vary the number
     of lens elements and verify the ghost count changes accordingly. Compare ghost positions
     against an analytical thin-lens prediction and verify within 5% angular error.

## Screen-Space Reflections

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.5.16 | [F-2.5.16](../../features/rendering/advanced-rendering.md) |

1. **R-2.5.16** — The engine **SHALL** provide Monte Carlo importance-sampled screen-space
   reflections traced at half resolution with BRDF-driven importance sampling reproducing specular
   elongation and spatially-varying roughness, with adjacent pixel ray reuse and dedicated spatial
   and temporal denoising filters.
   - **Rationale:** Stochastic SSR produces physically correct rough reflections from screen-space
     data alone, providing a performant rasterization-only reflection solution.
   - **Verification:** Render a floor with varying roughness (0.0-1.0). Verify reflections elongate
     with increasing roughness matching the BRDF lobe shape. Verify execution at half resolution
     (the reflection buffer dimensions are half the render resolution). Measure denoising eliminates
     visible noise within 4 frames of temporal accumulation with no ghosting.

## Non-Functional Requirements

| ID        |
|-----------|
| NFR-2.5.1 |
| NFR-2.5.2 |
| NFR-2.5.3 |

1. **NFR-2.5.1** — Bottom-level acceleration structure building and compaction **SHALL** complete in
   under 2.0 ms per frame for incremental updates (10% of scene objects changed) and under 50 ms for
   full scene rebuild on target hardware.
   - **Rationale:** BLAS updates occur every frame for dynamic scenes; exceeding 2 ms would consume
     a significant portion of the frame budget.
   - **Verification:** Profile BLAS incremental update time with 10% of scene objects moved and
     verify it is below 2.0 ms. Profile full rebuild and verify it is below 50 ms.
2. **NFR-2.5.2** — The combined RT budget (reflections + GI + shadows + AO) **SHALL** not exceed 8
   ms at 1080p on RT-capable hardware at the default quality tier, allowing the remaining frame
   budget for rasterization, post-processing, and CPU work.
   - **Rationale:** RT features must share the GPU frame budget with rasterization and
     post-processing; unbounded RT cost would prevent achieving 60 FPS.
   - **Verification:** Profile all active RT passes in a representative scene at 1080p default
     quality and verify total RT GPU time is below 8 ms.
3. **NFR-2.5.3** — Neural and conventional denoisers **SHALL** produce output with PSNR above 30 dB
   compared to a 4096-spp reference at 1 sample per pixel input, with no visible temporal flickering
   or ghosting over 120 frames of camera motion.
   - **Rationale:** Denoisers must produce clean output from minimal samples to justify the reduced
     ray budget.
   - **Verification:** Measure PSNR of denoised 1-spp output against a 4096-spp reference. Verify
     PSNR exceeds 30 dB. Inspect 120 frames of camera motion for temporal artifacts.
