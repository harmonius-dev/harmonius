# User Stories -- 2.5 Advanced Rendering (Ray Tracing and GI)

## Stories

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-2.5.1.1  | engine developer (P-26)  |          |              |
| US-2.5.1.2  | engine tester (P-27)     |          |              |
| US-2.5.2.1  | player (P-23)            |          |              |
| US-2.5.2.2  | engine developer (P-26)  |          |              |
| US-2.5.2.3  | engine tester (P-27)     |          |              |
| US-2.5.3.1  | player (P-23)            |          |              |
| US-2.5.3.2  | engine developer (P-26)  |          |              |
| US-2.5.4.1  | environment artist (P-8) |          |              |
| US-2.5.4.2  | engine tester (P-27)     |          |              |
| US-2.5.5.1  | technical artist (P-13)  |          |              |
| US-2.5.5.2  | engine tester (P-27)     |          |              |
| US-2.5.6.1  | player (P-23)            |          |              |
| US-2.5.6.2  | engine tester (P-27)     |          |              |
| US-2.5.7.1  | environment artist (P-8) |          |              |
| US-2.5.7.2  | engine developer (P-26)  |          |              |
| US-2.5.8.1  | environment artist (P-8) |          |              |
| US-2.5.8.2  | engine developer (P-26)  |          |              |
| US-2.5.9.1  | player (P-23)            |          |              |
| US-2.5.9.2  | engine tester (P-27)     |          |              |
| US-2.5.10.1 | engine developer (P-26)  |          |              |
| US-2.5.10.2 | engine tester (P-27)     |          |              |
| US-2.5.11.1 | engine developer (P-26)  |          |              |
| US-2.5.11.2 | engine tester (P-27)     |          |              |
| US-2.5.12.1 | technical artist (P-13)  |          |              |
| US-2.5.12.2 | engine tester (P-27)     |          |              |
| US-2.5.13.1 | player (P-23)            |          |              |
| US-2.5.13.2 | engine tester (P-27)     |          |              |
| US-2.5.14.1 | environment artist (P-8) |          |              |
| US-2.5.14.2 | engine developer (P-26)  |          |              |
| US-2.5.15.1 | engine developer (P-26)  |          |              |
| US-2.5.15.2 | engine tester (P-27)     |          |              |
| US-2.5.16.1 | player (P-23)            |          |              |
| US-2.5.16.2 | engine tester (P-27)     |          |              |

1. **US-2.5.1.1** — I want to build BLAS from meshlet geometry with post-build compaction and
   rebuild TLAS each frame for dynamic objects
   - **Acceptance:** RT features have an up-to-date acceleration structure with minimal VRAM waste
2. **US-2.5.1.2** — I want to verify that BLAS construction respects per-platform memory budgets
   (limited on Apple A17+/M3+ and Switch 2 Ampere)
   - **Acceptance:** RT scenes do not exceed mobile or Switch VRAM allocations
3. **US-2.5.2.1** — I want hybrid reflections that combine screen-space rays with hardware RT for
   off-screen content
   - **Acceptance:** mirrors and polished floors show correct reflections even when the reflected
     object is not on screen
4. **US-2.5.2.2** — I want to measure the cost of RT reflection rays at varying roughness thresholds
   - **Acceptance:** I can tune the SSR-to-RT handoff point to balance quality and GPU budget per
     platform
5. **US-2.5.2.3** — I want to disable both SSR and RT and confirm that reflection probes provide
   correct fallback reflections on all surfaces
   - **Acceptance:** mobile devices without SSR still show plausible reflections
6. **US-2.5.3.1** — I want one-bounce indirect diffuse lighting that bleeds wall color onto adjacent
   surfaces
   - **Acceptance:** environments feel naturally lit with warm and cool tones rather than flat
     ambient
7. **US-2.5.3.2** — I want to verify that 0.25 spp RT indirect lighting accumulates over 8-16 frames
   to produce stable noise-free GI
   - **Acceptance:** indirect lighting converges quickly enough for real-time use
8. **US-2.5.4.1** — I want DDGI probes that recast rays and update irradiance when I reposition
   lights in the editor
   - **Acceptance:** indirect lighting responds to light changes in real time without rebaking
9. **US-2.5.4.2** — I want to verify that desktop uses 8m probe spacing with 64 rays/probe and
   high-end uses 4m/128+ rays
   - **Acceptance:** GI quality scales correctly with hardware capability
10. **US-2.5.5.1** — I want a progressive path tracer that renders offline- quality images with full
    light transport and all material types
    - **Acceptance:** I can compare real-time rendering against a reference to identify
      approximation artifacts
11. **US-2.5.5.2** — I want to render a white-furnace test scene with the path tracer and verify
    that energy is conserved (no darkening or brightening over bounces)
    - **Acceptance:** the reference renderer produces physically accurate results
12. **US-2.5.6.1** — I want ray-traced subsurface transmission through thin body parts like ears,
    fingers, and nostrils when backlit
    - **Acceptance:** characters look lifelike in strong directional lighting
13. **US-2.5.6.2** — I want to disable RT and confirm that subsurface scattering falls back to
    preintegrated LUT on mobile and Switch without visual artifacts
    - **Acceptance:** skin rendering degrades gracefully
14. **US-2.5.7.1** — I want surfel-based GI that discretizes scene geometry from the G-buffer each
    frame without precomputation
    - **Acceptance:** fully dynamic environments with destructible objects receive correct indirect
      lighting
15. **US-2.5.7.2** — I want to measure per-frame GPU cost of surfel GI at 2 clipmap levels (desktop)
    versus 4+ levels (high-end)
    - **Acceptance:** I can validate that multi-scale surfel density meets performance targets per
      tier
16. **US-2.5.8.1** — I want ReSTIR DI to sample thousands of lights with physically correct soft
    shadows from every source
    - **Acceptance:** I can author dense light environments (concert halls, cityscapes) without
      per-light shadow cost
17. **US-2.5.8.2** — I want to measure per-pixel reservoir memory footprint during ReSTIR DI+GI and
    confirm it fits within a 2 GB render target budget on desktop
    - **Acceptance:** ReSTIR operates within VRAM constraints
18. **US-2.5.9.1** — I want multi-bounce real-time path tracing with neural denoising for
    reflections, AO, GI, and refractions
    - **Acceptance:** the game looks as close to cinematic quality as possible during live gameplay
19. **US-2.5.9.2** — I want to confirm that the path tracer falls back to rasterized GI after the
    final bounce rather than terminating to black
    - **Acceptance:** path-traced scenes maintain ambient fill in shadowed areas
20. **US-2.5.10.1** — I want opacity micromaps to annotate per-micro-triangle opacity for vegetation
    and hair
    - **Acceptance:** hardware skips any-hit shader invocations for classified triangles and RT
      performance improves by 20-40%
21. **US-2.5.10.2** — I want to run on Ada Lovelace and RDNA3+ GPUs and confirm that opacity
    micromaps are enabled automatically
    - **Acceptance:** supported hardware gets the RT performance benefit without manual
      configuration
22. **US-2.5.11.1** — I want shader execution reordering to group similar RT shader invocations into
    coherent wavefronts
    - **Acceptance:** divergent material shaders after ray intersection run with improved GPU
      occupancy
23. **US-2.5.11.2** — I want to verify that SER activates on Ada Lovelace+ via
    VK_NV_ray_tracing_invocation_reorder and is skipped on unsupported hardware
    - **Acceptance:** the feature is conditionally enabled without driver errors
24. **US-2.5.12.1** — I want a neural denoiser to replace hand-tuned spatiotemporal filters for RT
    and path-traced output
    - **Acceptance:** noisy 1-spp rays produce temporally stable, artifact-free images
25. **US-2.5.12.2** — I want to run on a GPU without Tensor Cores and confirm that the engine falls
    back to NRD bilateral/wavelet denoising
    - **Acceptance:** RT denoising works on all desktop GPUs regardless of neural hardware
26. **US-2.5.13.1** — I want ray-traced lens flare that simulates real lens element refraction to
    produce ghost and halo artifacts
    - **Acceptance:** looking toward the sun or bright explosion produces convincing optical effects
27. **US-2.5.13.2** — I want to disable RT and verify that lens flare falls back to the image-based
    approximation (F-2.9.8)
    - **Acceptance:** mobile and Switch still show lens flare effects
28. **US-2.5.14.1** — I want voxel-based GI that works without RT hardware by voxelizing the scene
    and relighting the voxel cache for time-of-day changes
    - **Acceptance:** Switch and older desktop GPUs still get convincing indirect lighting
29. **US-2.5.14.2** — I want to measure 3D texture VRAM for voxel GI at 64^3 (Switch), 128^3
    (desktop), and 256^3 (high-end) grid resolutions
    - **Acceptance:** I can verify each tier fits within its platform VRAM budget
30. **US-2.5.15.1** — I want a runtime-trained neural network that predicts remaining light
    transport after 2-3 bounces
    - **Acceptance:** path-traced output has less noise with fewer full-length paths required
31. **US-2.5.15.2** — I want to confirm that the neural radiance cache requires Tensor Core hardware
    and 12+ GB VRAM, and is disabled by default on all other configurations
    - **Acceptance:** the feature does not activate on unsupported hardware
32. **US-2.5.16.1** — I want stochastic screen-space reflections importance-sampled from the
    microfacet BRDF
    - **Acceptance:** rough metal surfaces show elongated, physically correct specular reflections
      rather than sharp mirror images
33. **US-2.5.16.2** — I want to compare stochastic SSR at quarter-res (Switch), half-res (desktop),
    and half-res with spatial+temporal denoiser (high-end)
    - **Acceptance:** each platform tier produces acceptable reflection quality
