# 2.5 — Advanced Rendering (Ray Tracing and GI)

## Features

| ID       | Feature                                       |
|----------|-----------------------------------------------|
| F-2.5.1  | Acceleration Structure Management (BLAS/TLAS) |
| F-2.5.2  | Ray Traced Reflections (Hybrid SSR + RT)      |
| F-2.5.3  | Ray Traced Indirect Lighting                  |
| F-2.5.4  | Real-Time Global Illumination (DDGI)          |
| F-2.5.5  | Path Tracing (Reference Renderer)             |
| F-2.5.6  | Ray Traced Subsurface Transmission            |
| F-2.5.7  | Surfel-Based Global Illumination              |
| F-2.5.8  | ReSTIR Sampling Framework                     |
| F-2.5.9  | Real-Time Production Path Tracing             |
| F-2.5.10 | Opacity Micromaps                             |
| F-2.5.11 | Shader Execution Reordering                   |
| F-2.5.12 | Neural Denoising (Ray Reconstruction)         |
| F-2.5.13 | RT Lens Flare                                 |
| F-2.5.14 | Voxel-Based Global Illumination               |
| F-2.5.15 | Neural Radiance Cache                         |
| F-2.5.16 | Stochastic Screen-Space Reflections           |
| F-2.5.17 | Baked Lightmaps as GI Fallback                |

1. **F-2.5.1** — Bottom-level acceleration structure (BLAS) building from meshlet geometry with
   post-build compaction to reduce memory. Top-level acceleration structure (TLAS) rebuilt or refit
   each frame for dynamic scenes.
   - **Deps:** F-2.1.1
   - **Platform:** Mobile: disabled on most devices; Apple A17 Pro/M3+ support hardware RT via Metal
     but with limited BLAS budget. Switch (original): no RT hardware. Switch 2: Ampere RT cores
     available with limited BLAS budget. Desktop: full BLAS/TLAS with compaction. High-end:
     unlimited AS complexity.
2. **F-2.5.2** — Hybrid reflection system combining screen-space ray marching (SSR) for nearby
   surfaces with hardware ray tracing for off-screen and rough reflections. A roughness threshold
   determines when RT rays supplement SSR. Temporal denoising stabilizes the output.
   - **Deps:** F-2.5.1
   - **Platform:** Mobile: SSR only at half-res (no RT); disabled under budget pressure; reflection
     probes as fallback. Switch: SSR only, half-res. Desktop: hybrid SSR + RT at half-res, 0.5 spp.
     High-end: full hybrid at native res, 1 spp with temporal denoiser.
3. **F-2.5.3** — Secondary diffuse ray tracing from surface positions to collect one-bounce indirect
   illumination. Output feeds the global illumination system.
   - **Deps:** F-2.5.1
   - **Platform:** Mobile: disabled; uses baked lightmaps or voxel GI fallback. Switch: disabled;
     voxel GI or baked fallback. Desktop: enabled at 0.25 spp with temporal accumulation. High-end:
     1 spp with multi-bounce support.
4. **F-2.5.4** — Dynamic Diffuse Global Illumination using a probe grid. Each probe casts rays via
   the TLAS, accumulates irradiance and visibility into octahedral atlas textures with temporal
   hysteresis. Surfaces sample nearby probes for diffuse indirect lighting, producing color bleeding
   and ambient fill.
   - **Deps:** F-2.5.1, F-2.5.3
   - **Platform:** Mobile: disabled; requires RT hardware. Switch: disabled. Desktop: enabled with
     reduced probe density (8m spacing) and 64 rays/probe. High-end: full probe density (4m
     spacing), 128+ rays/probe.
5. **F-2.5.5** — Progressive unbiased hardware-accelerated renderer for offline-quality output.
   Traces full light transport paths (diffuse, specular, transmission) with configurable samples per
   pixel and max bounces. Supports all material types, volumetrics, and sky atmosphere. Used as a
   ground-truth reference and for cinematic rendering via the render queue.
   - **Deps:** F-2.5.1
   - **Platform:** Mobile: disabled; no real-time path tracing on mobile. Switch: disabled. Desktop:
     available as offline/editor-only tool; not real-time. High-end: progressive accumulation for
     cinematic stills; max bounces configurable.
6. **F-2.5.6** — Volumetric scattering computation through translucent media during shadow ray
   evaluation. When a shadow ray hits a subsurface material, light transport through the material's
   thickness is evaluated using the subsurface profile, enabling realistic light transmission
   through skin, wax, and thin objects.
   - **Deps:** F-2.5.1
   - **Platform:** Mobile: disabled; uses preintegrated SSS fallback. Switch: disabled. Desktop:
     enabled for hero characters when RT active. High-end: enabled for all SSS materials during
     RT/path-tracing passes.
7. **F-2.5.7** — Dynamic surfel-based GI requiring no precomputation. Scene geometry is discretized
   into surface elements (surfels) from the G-Buffer each frame. A clipmap probe system with
   octahedral representation caches irradiance and depth. Hardware ray tracing traces rays from
   surfels to compute incoming radiance with ray guiding and ray binning for efficient integration.
   Multi-scale mean estimation provides temporal hysteresis. Separate ray-traced probes handle
   character lighting. Scales to environments of arbitrary size with predictable performance.
   - **Deps:** F-2.5.1, F-2.4.2
   - **Platform:** Mobile: disabled; requires RT and G-buffer. Switch: disabled. Desktop: enabled
     with reduced surfel density and 2-clipmap levels. High-end: full surfel density, 4+ clipmap
     levels, separate character probes.
8. **F-2.5.8** — Reservoir-based spatiotemporal importance resampling (ReSTIR) framework for
   efficient stochastic light sampling. ReSTIR DI handles direct illumination from thousands of
   lights simultaneously with physically correct soft shadows from every source. ReSTIR GI enables
   screen-space resampling of secondary surfaces for multi-bounce indirect lighting. Reservoirs are
   maintained per pixel across frames, with spatial and temporal reuse for rapid convergence.
   - **Deps:** F-2.5.1
   - **Platform:** Mobile: disabled; reservoir memory and RT cost prohibitive. Switch: disabled.
     Desktop: ReSTIR DI only with limited reservoir size. High-end: full ReSTIR DI + GI with spatial
     and temporal reuse.
9. **F-2.5.9** — Tiered real-time path tracing for production game rendering, distinct from the
   offline reference renderer. Supports configurable quality tiers from direct-RT-only through
   multi-bounce full path tracing with RT reflections, AO, GI, and refractions. After the final
   path-tracing bounce, the engine falls back to rasterized GI rather than terminating to black.
   Paired with neural denoising and temporal upscaling for real-time frame rates.
   - **Deps:** F-2.5.1, F-2.5.12
   - **Platform:** Mobile: disabled. Switch: disabled. Desktop: direct-RT-only tier (shadows +
     reflections) with rasterized GI fallback. High-end: multi-bounce path tracing (2-4 bounces)
     with neural denoiser and temporal upscaling.
10. **F-2.5.10** — Acceleration structure annotation encoding per-micro-triangle opacity
    classification (opaque, transparent, or unknown) for alpha-tested geometry. The hardware skips
    any-hit shader invocations for micro-triangles classified as fully opaque or fully transparent,
    significantly accelerating ray intersection testing for vegetation and hair.
    - **Deps:** F-2.5.1
    - **Platform:** Mobile: disabled; no OMM hardware support. Switch: disabled. Desktop: enabled on
      Ada Lovelace+ (NVIDIA) and RDNA3+ (AMD) GPUs. High-end: enabled with high-density
      micro-triangle classification.
11. **F-2.5.11** — Hardware-assisted reordering of ray tracing shader invocations to improve
    execution coherency. After intersection testing scatters rays to diverse material shaders, SER
    groups similar shader invocations into coherent wavefronts before execution, reducing divergence
    and improving GPU occupancy.
    - **Deps:** F-2.5.1
    - **Platform:** Mobile: disabled; no SER hardware. Switch: disabled. Desktop: enabled on Ada
      Lovelace+ (NVIDIA) via VK_NV_ray_tracing_invocation_reorder or D3D12 SER API. High-end:
      automatically enabled when RT is active.
12. **F-2.5.12** — AI-trained neural network denoiser that replaces hand-tuned spatiotemporal
    denoisers for ray-traced and path-traced output. Operates on noisy per-pixel radiance with
    motion vectors and depth, producing temporally stable results with fewer artifacts than
    traditional bilateral/wavelet filters. Supports fallback to conventional NRD denoiser algorithms
    when neural hardware is unavailable.
    - **Deps:** F-2.5.1
    - **Platform:** Mobile: disabled; no neural denoiser hardware. Switch: disabled. Desktop: NRD
      bilateral/wavelet fallback when Tensor Cores unavailable; neural denoiser on RTX 40+.
      High-end: full neural ray reconstruction via Tensor Cores.
13. **F-2.5.13** — Physically accurate ray-traced lens flare simulation. Lens elements are modeled
    as refractive surfaces; rays from bright scene sources are traced through the lens system to
    produce ghost and halo artifacts matching real optical behavior, replacing the image-based
    approximation with ground-truth results.
    - **Deps:** F-2.5.1
    - **Platform:** Mobile: disabled; uses image-based flare fallback (F-2.9.8). Switch: disabled.
      Desktop: optional when RT active. High-end: enabled with full multi-element lens model.
14. **F-2.5.14** — Rasterization-only GI system that voxelizes the world, storing albedo, normals,
    and translucency into a 3D voxel cache. The cache is re-lit for varying times of day without
    re-voxelization. Combined with irradiance volumes blended across time-of-day transitions and
    improved cubemap reflections, this system provides convincing indirect lighting entirely through
    rasterization without hardware ray tracing.
    - **Deps:** F-2.4.2
    - **Platform:** Mobile: disabled; 3D texture VRAM too high. Switch: enabled with 64^3 voxel
      grid, single cascade. Desktop: 128^3 grid, 2 cascades. High-end: 256^3 grid, 3+ cascades with
      per-voxel translucency.
15. **F-2.5.15** — Neural network trained at runtime to estimate indirect radiance at arbitrary
    scene points. The network terminates the majority of path-traced paths early (after 2-3 bounces)
    and predicts the remaining light transport, using a small percentage of full-length paths for
    ongoing training. Produces less noise than regular path tracing and supports entirely dynamic
    scenes.
    - **Deps:** F-2.5.1
    - **Platform:** Mobile: disabled. Switch: disabled. Desktop: disabled by default; requires
      Tensor Core hardware and 12+ GB VRAM. High-end: enabled with runtime training on RTX
      40+/RDNA4+ GPUs.
16. **F-2.5.16** — Monte Carlo importance-sampled screen-space reflections with spatiotemporal
    filtering. Rays are importance-sampled from the microfacet BRDF to reproduce specular elongation
    and spatially-varying roughness. Traced at half resolution with adjacent pixel ray reuse and
    dedicated spatial and temporal filters for denoising.
    - **Deps:** F-2.4.2
    - **Platform:** Mobile: disabled; uses reflection probes only. Switch: enabled at quarter-res
      with simplified sampling (no BRDF importance sampling). Desktop: half-res with full BRDF
      sampling and temporal filter. High-end: half-res with spatial + temporal denoiser and adjacent
      pixel reuse.
17. **F-2.5.17** — Baked lightmap loading and rendering as the lowest-tier indirect lighting
    fallback when no ray tracing hardware is present and voxel GI exceeds budget. Offline bakes
    store directional irradiance in texture atlases UV-mapped onto static geometry; dynamic objects
    sample irradiance volumes placed alongside the lightmaps. Lightmaps are selected automatically
    by the GI tier system when higher-tier techniques (RT GI, DDGI, surfel GI, voxel GI) are
    unavailable, ensuring all platforms receive plausible indirect lighting.
    - **Deps:** F-2.4.2
    - **Platform:** Mobile: primary GI path; compressed ASTC lightmap atlases with irradiance volume
      dynamic lighting. Switch: lightmap atlases with per-probe sampling. Desktop: used as fallback
      only when RT hardware absent. High-end: never selected; always superseded by higher-tier GI.
