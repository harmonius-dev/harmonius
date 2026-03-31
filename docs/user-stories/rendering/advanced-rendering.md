# User Stories -- 2.5 Advanced Rendering

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.5.1.1  | engine developer (P-26)      |
| US-2.5.1.2  | technical artist (P-13)      |
| US-2.5.2.1  | environment artist (P-8)     |
| US-2.5.2.2  | technical artist (P-13)      |
| US-2.5.3.1  | environment artist (P-8)     |
| US-2.5.4.1  | environment artist (P-8)     |
| US-2.5.4.2  | engine developer (P-26)      |
| US-2.5.5.1  | technical artist (P-13)      |
| US-2.5.5.2  | environment artist (P-8)     |
| US-2.5.6.1  | environment artist (P-8)     |
| US-2.5.7.1  | engine developer (P-26)      |
| US-2.5.7.2  | environment artist (P-8)     |
| US-2.5.8.1  | engine developer (P-26)      |
| US-2.5.8.2  | environment artist (P-8)     |
| US-2.5.9.1  | technical artist (P-13)      |
| US-2.5.9.2  | engine developer (P-26)      |
| US-2.5.10.1 | engine developer (P-26)      |
| US-2.5.11.1 | engine developer (P-26)      |
| US-2.5.12.1 | technical artist (P-13)      |
| US-2.5.12.2 | engine developer (P-26)      |
| US-2.5.13.1 | effects artist (P-12)        |
| US-2.5.14.1 | environment artist (P-8)     |
| US-2.5.14.2 | technical artist (P-13)      |
| US-2.5.15.1 | engine developer (P-26)      |
| US-2.5.16.1 | technical artist (P-13)      |
| US-2.5.16.2 | environment artist (P-8)     |

1. **US-2.5.1.1** — **As a** engine developer (P-26), **I want** BLAS built from meshlet geometry
   with post-build compaction, **so that** ray tracing acceleration structures use minimal VRAM.

2. **US-2.5.1.2** — **As a** technical artist (P-13), **I want** the TLAS rebuilt or refit each
   frame for dynamic scenes, **so that** moving objects are correctly traced without manual
   structure updates.

3. **US-2.5.2.1** — **As a** environment artist (P-8), **I want** hybrid reflections combining SSR
   for nearby surfaces with RT for off-screen geometry, **so that** reflective floors and windows
   show complete surroundings.

4. **US-2.5.2.2** — **As a** technical artist (P-13), **I want** a roughness threshold controlling
   when RT rays supplement SSR, **so that** I can balance reflection quality and performance per
   scene.

5. **US-2.5.3.1** — **As a** environment artist (P-8), **I want** one-bounce indirect ray-traced
   illumination, **so that** colored light bleeds from nearby surfaces and fills shadowed areas
   naturally.

6. **US-2.5.4.1** — **As a** environment artist (P-8), **I want** DDGI probe grids providing dynamic
   diffuse GI, **so that** color bleeding and ambient fill update in real time when lights or
   objects move.

7. **US-2.5.4.2** — **As a** engine developer (P-26), **I want** DDGI probes with octahedral atlas
   textures and temporal hysteresis, **so that** irradiance accumulates stably without flickering.

8. **US-2.5.5.1** — **As a** technical artist (P-13), **I want** a progressive path tracer as a
   reference renderer, **so that** I can compare real-time output against ground-truth lighting.

9. **US-2.5.5.2** — **As a** environment artist (P-8), **I want** to render cinematic stills via the
   path tracer with configurable samples and bounces, **so that** I can produce offline-quality
   images for marketing.

10. **US-2.5.6.1** — **As a** environment artist (P-8), **I want** ray-traced subsurface
    transmission through skin and wax, **so that** backlit translucent objects glow realistically.

11. **US-2.5.7.1** — **As a** engine developer (P-26), **I want** surfel-based GI with no
    precomputation using clipmap probes and ray guiding, **so that** dynamic GI scales to arbitrary
    environment size.

12. **US-2.5.7.2** — **As a** environment artist (P-8), **I want** surfel GI providing indirect
    lighting that updates instantly when I change geometry, **so that** I can iterate on level
    layout without rebaking.

13. **US-2.5.8.1** — **As a** engine developer (P-26), **I want** a ReSTIR sampling framework for
    stochastic light selection with reservoir reuse, **so that** thousands of lights are sampled
    efficiently per pixel.

14. **US-2.5.8.2** — **As a** environment artist (P-8), **I want** thousands of shadowed lights
    evaluating soft shadows via ReSTIR, **so that** dense cityscapes are lit without manually
    budgeting shadow map count.

15. **US-2.5.9.1** — **As a** technical artist (P-13), **I want** tiered real-time path tracing from
    direct-RT through multi-bounce with rasterized GI fallback, **so that** I can choose quality vs
    performance per platform.

16. **US-2.5.9.2** — **As a** engine developer (P-26), **I want** the path tracer to fall back to
    rasterized GI after the final bounce instead of terminating to black, **so that** indirect
    lighting remains continuous.

17. **US-2.5.10.1** — **As a** engine developer (P-26), **I want** opacity micromaps classifying
    per-micro-triangle transparency to skip any-hit shaders, **so that** ray intersection for
    vegetation and hair is accelerated.

18. **US-2.5.11.1** — **As a** engine developer (P-26), **I want** shader execution reordering
    grouping similar shader invocations into coherent wavefronts, **so that** RT shader divergence
    is reduced and GPU occupancy improves.

19. **US-2.5.12.1** — **As a** technical artist (P-13), **I want** neural denoising producing
    temporally stable results from noisy RT output, **so that** low-sample-count path tracing looks
    clean at real-time frame rates.

20. **US-2.5.12.2** — **As a** engine developer (P-26), **I want** fallback to conventional NRD
    denoising when neural hardware is unavailable, **so that** denoising works on all RT-capable
    GPUs.

21. **US-2.5.13.1** — **As a** effects artist (P-12), **I want** ray-traced lens flares simulating
    real optical ghost and halo artifacts, **so that** bright sources produce physically accurate
    flare patterns.

22. **US-2.5.14.1** — **As a** environment artist (P-8), **I want** voxel-based GI providing
    indirect lighting without RT hardware, **so that** time-of-day transitions update lighting on
    rasterization-only platforms.

23. **US-2.5.14.2** — **As a** technical artist (P-13), **I want** the voxel cache re-lit for
    varying times of day without re-voxelization, **so that** day/night cycles are efficient on
    mid-tier hardware.

24. **US-2.5.15.1** — **As a** engine developer (P-26), **I want** a neural radiance cache trained
    at runtime to terminate path-traced paths early, **so that** noise is reduced without extra ray
    budget.

25. **US-2.5.16.1** — **As a** technical artist (P-13), **I want** stochastic SSR importance-sampled
    from the microfacet BRDF, **so that** specular elongation and spatially varying roughness are
    reproduced.

26. **US-2.5.16.2** — **As a** environment artist (P-8), **I want** screen-space reflections on
    glossy floors without RT hardware, **so that** reflections are available on all platforms at
    reduced quality.
