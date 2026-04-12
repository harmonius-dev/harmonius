# User Stories -- 2.4 Lighting and Materials

## Stories

| ID           | Persona                      |
|--------------|------------------------------|
| US-2.4.1.1   | environment artist (P-8)     |
| US-2.4.1.2   | engine developer (P-26)      |
| US-2.4.2.1   | engine developer (P-26)      |
| US-2.4.2.2   | technical artist (P-13)      |
| US-2.4.3.1   | environment artist (P-8)     |
| US-2.4.3.2   | technical artist (P-13)      |
| US-2.4.4.1   | environment artist (P-8)     |
| US-2.4.4.2   | technical artist (P-13)      |
| US-2.4.5.1   | game developer (P-15)        |
| US-2.4.5.2   | engine developer (P-26)      |
| US-2.4.6.1   | environment artist (P-8)     |
| US-2.4.6.2   | technical artist (P-13)      |
| US-2.4.7.1   | environment artist (P-8)     |
| US-2.4.7.2   | technical artist (P-13)      |
| US-2.4.8.1   | environment artist (P-8)     |
| US-2.4.8.2   | effects artist (P-12)        |
| US-2.4.9.1   | technical artist (P-13)      |
| US-2.4.9.2   | environment artist (P-8)     |
| US-2.4.10.1  | engine developer (P-26)      |
| US-2.4.10.2  | environment artist (P-8)     |
| US-2.4.11.1  | environment artist (P-8)     |
| US-2.4.11.2  | technical artist (P-13)      |
| US-2.4.12.1  | environment artist (P-8)     |
| US-2.4.12.2  | technical artist (P-13)      |
| US-2.4.13.1  | environment artist (P-8)     |
| US-2.4.13.2  | technical artist (P-13)      |
| US-2.4.14.1  | engine developer (P-26)      |
| US-2.4.14.2  | technical artist (P-13)      |
| US-2.4.15.1  | environment artist (P-8)     |
| US-2.4.16.1  | environment artist (P-8)     |
| US-2.4.17.1  | technical artist (P-13)      |
| US-2.4.18.1  | effects artist (P-12)        |
| US-2.4.18.2  | game developer (P-15)        |
| US-2.4.19.1  | effects artist (P-12)        |
| US-2.4.20.1  | environment artist (P-8)     |
| US-2.4.20.2  | technical artist (P-13)      |
| US-2.4.21.1  | environment artist (P-8)     |
| US-2.4.22.1  | environment artist (P-8)     |
| US-2.4.23.1  | effects artist (P-12)        |
| US-2.4.23.2  | technical artist (P-13)      |

1. **US-2.4.1.1** — **As a** environment artist (P-8), **I want** tiled light culling enabling
   hundreds of dynamic lights per scene, **so that** I can light complex interiors without hitting
   per-fragment cost limits.

2. **US-2.4.1.2** — **As a** engine developer (P-26), **I want** a compute pass that assigns visible
   lights to screen-space tiles, **so that** fragment shading evaluates only relevant lights per
   tile.

3. **US-2.4.2.1** — **As a** engine developer (P-26), **I want** deferred lighting via a G-buffer
   with albedo, normal, roughness, motion vectors, and depth, **so that** high-geometry scenes
   decouple lighting cost from mesh count.

4. **US-2.4.2.2** — **As a** technical artist (P-13), **I want** the G-buffer to use on-chip tile
   storage on mobile, **so that** deferred lighting is feasible on bandwidth-constrained tile-based
   GPUs.

5. **US-2.4.3.1** — **As a** environment artist (P-8), **I want** physically-based Cook-Torrance
   materials with bindless textures, **so that** surfaces look consistent between the DCC tool and
   the engine viewport.

6. **US-2.4.3.2** — **As a** technical artist (P-13), **I want** half-precision BRDF evaluation and
   ASTC textures on mobile, **so that** PBR materials fit within mobile ALU and bandwidth budgets.

7. **US-2.4.4.1** — **As a** environment artist (P-8), **I want** extended BSDF layers for
   subsurface scattering, clearcoat, anisotropy, and sheen, **so that** I can render skin, fabric,
   car paint, and hair with a single material.

8. **US-2.4.4.2** — **As a** technical artist (P-13), **I want** extended lobes to degrade to fewer
   active layers on lower-tier hardware, **so that** materials look acceptable without manual
   per-platform authoring.

9. **US-2.4.5.1** — **As a** game developer (P-15), **I want** transparent objects sorted
   back-to-front and drawn individually, **so that** alpha-blended surfaces composite correctly
   without sorting artifacts.

10. **US-2.4.5.2** — **As a** engine developer (P-26), **I want** transparent objects drawn after
    all opaques with individual draw calls, **so that** depth ordering is preserved for correct
    transparency.

11. **US-2.4.6.1** — **As a** environment artist (P-8), **I want** parameterized material instances
    that override parent properties without shader recompilation, **so that** I can create thousands
    of color and texture variants cheaply.

12. **US-2.4.6.2** — **As a** technical artist (P-13), **I want** material instances to share
    compiled shaders and upload only a per-instance parameter buffer, **so that** instance cost is
    minimal at scale.

13. **US-2.4.7.1** — **As a** environment artist (P-8), **I want** multi-layer material compositing
    with blend masks for rust over metal or snow over rock, **so that** I can add environmental wear
    without unique textures.

14. **US-2.4.7.2** — **As a** technical artist (P-13), **I want** per-pixel layer evaluation with
    configurable layer limits per platform, **so that** layered materials scale from 2 layers on
    mobile to unlimited on high-end.

15. **US-2.4.8.1** — **As a** environment artist (P-8), **I want** projected decals writing into a
    deferred decal buffer before lighting, **so that** bullet holes and paint splatters blend
    correctly with surface normals.

16. **US-2.4.8.2** — **As a** effects artist (P-12), **I want** mesh-based decals wrapping curved
    geometry, **so that** impact effects conform to round surfaces without stretching artifacts.

17. **US-2.4.9.1** — **As a** technical artist (P-13), **I want** dedicated shading model variants
    for hair, eye, cloth, foliage, and water selectable via material flag, **so that** specialized
    materials use optimized code paths.

18. **US-2.4.9.2** — **As a** environment artist (P-8), **I want** a two-sided foliage shading model
    with subsurface transmission, **so that** backlit leaves scatter light realistically.

19. **US-2.4.10.1** — **As a** engine developer (P-26), **I want** importance-sampled stochastic
    lighting for thousands of shadowed area lights, **so that** per-pixel ray budget replaces
    per-light shadow map evaluation.

20. **US-2.4.10.2** — **As a** environment artist (P-8), **I want** thousands of small area lights
    in a scene with soft shadows from every source, **so that** large cityscapes and interiors are
    lit realistically.

21. **US-2.4.11.1** — **As a** environment artist (P-8), **I want** cascaded shadow maps for
    directional lights with configurable cascade count and resolution, **so that** sun shadows cover
    the full scene with crisp near-field detail.

22. **US-2.4.11.2** — **As a** technical artist (P-13), **I want** per-platform cascade count and
    resolution caps, **so that** shadow quality scales from 1 cascade on mobile to 4 cascades at
    4096 on high-end desktop.

23. **US-2.4.12.1** — **As a** environment artist (P-8), **I want** tiered soft shadows from PCF
    through PCSS to ray-traced penumbra, **so that** shadow softness matches light source size on
    each hardware tier.

24. **US-2.4.12.2** — **As a** technical artist (P-13), **I want** the shadow technique to upgrade
    automatically based on GPU capabilities, **so that** I author content once and shadows improve
    on better hardware.

25. **US-2.4.13.1** — **As a** environment artist (P-8), **I want** ambient occlusion darkening in
    corners and under objects, **so that** scenes have spatial grounding without per-object shadow
    authoring.

26. **US-2.4.13.2** — **As a** technical artist (P-13), **I want** AO to scale from SSAO on mobile
    through GTAO on desktop to RT AO on high-end, **so that** occlusion quality matches the platform
    budget.

27. **US-2.4.14.1** — **As a** engine developer (P-26), **I want** clipmap-based virtual shadow maps
    providing consistent 16k-equivalent detail, **so that** shadow resolution is uniform across the
    visible range.

28. **US-2.4.14.2** — **As a** technical artist (P-13), **I want** VSM pages allocated on demand by
    screen-space coverage, **so that** VRAM is spent only on visible shadow detail.

29. **US-2.4.15.1** — **As a** environment artist (P-8), **I want** per-pixel contact shadows
    tracing the depth buffer along light direction, **so that** fine-scale shadow detail appears at
    geometry edges and crevices.

30. **US-2.4.16.1** — **As a** environment artist (P-8), **I want** distance field shadows extending
    shadow range far beyond cascade limits, **so that** distant mountains and buildings cast soft
    long-range shadows.

31. **US-2.4.17.1** — **As a** technical artist (P-13), **I want** capsule shadows for skeletal
    meshes approximating soft area shadowing in GI-lit regions, **so that** characters have grounded
    indirect shadows at low cost.

32. **US-2.4.18.1** — **As a** effects artist (P-12), **I want** order-independent transparency
    blending fog, water, and particles without CPU sorting, **so that** overlapping translucent
    volumes composite correctly.

33. **US-2.4.18.2** — **As a** game developer (P-15), **I want** OIT to fall back to sorted
    transparency on platforms where moment buffers are too expensive, **so that** transparency works
    everywhere.

34. **US-2.4.19.1** — **As a** effects artist (P-12), **I want** volumetric shadow maps enabling fog
    and clouds to cast and receive shadows from any light, **so that** volumetric elements integrate
    with the lighting system.

35. **US-2.4.20.1** — **As a** environment artist (P-8), **I want** rectangular and spherical area
    lights with LTC integration, **so that** windows and softboxes produce realistic soft
    illumination.

36. **US-2.4.20.2** — **As a** technical artist (P-13), **I want** area lights to fall back to
    point/spot approximation on mobile, **so that** the effect is present at reduced quality on all
    platforms.

37. **US-2.4.21.1** — **As a** environment artist (P-8), **I want** image-based sky lighting with
    pre-filtered diffuse irradiance and split-sum specular lookup, **so that** outdoor scenes have
    natural ambient fill.

38. **US-2.4.22.1** — **As a** environment artist (P-8), **I want** IES photometric profiles on
    point and spot lights, **so that** architectural and cinematic lighting matches real fixture
    distributions.

39. **US-2.4.23.1** — **As a** effects artist (P-12), **I want** material-driven light functions
    producing animated patterns like flickering or gobo projections, **so that** lights create
    dynamic atmosphere without custom code.

40. **US-2.4.23.2** — **As a** technical artist (P-13), **I want** light functions baked to static
    textures on mobile and evaluated at runtime on desktop, **so that** the effect scales across
    platforms.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.4.1 | environment artist (P-8) |
| US-2.4.10 | engine developer (P-26) |
| US-2.4.11 | environment artist (P-8) |
| US-2.4.12 | environment artist (P-8) |
| US-2.4.13 | environment artist (P-8) |
| US-2.4.14 | engine developer (P-26) |
| US-2.4.15 | environment artist (P-8) |
| US-2.4.16 | environment artist (P-8) |
| US-2.4.17 | technical artist (P-13) |
| US-2.4.18 | effects artist (P-12) |
| US-2.4.19 | effects artist (P-12) |
| US-2.4.2 | engine developer (P-26) |
| US-2.4.20 | environment artist (P-8) |
| US-2.4.21 | environment artist (P-8) |
| US-2.4.22 | environment artist (P-8) |
| US-2.4.23 | effects artist (P-12) |
| US-2.4.3 | environment artist (P-8) |
| US-2.4.4 | environment artist (P-8) |
| US-2.4.5 | game developer (P-15) |
| US-2.4.6 | environment artist (P-8) |
| US-2.4.7 | environment artist (P-8) |
| US-2.4.8 | environment artist (P-8) |
| US-2.4.9 | technical artist (P-13) |

1. **US-2.4.1** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.1.1 through US-2.4.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-2.4.10** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.4.10.1 through US-2.4.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.4.11** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.11.1 through US-2.4.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-2.4.12** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.12.1 through US-2.4.12.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-2.4.13** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.13.1 through US-2.4.13.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-2.4.14** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.4.14.1 through US-2.4.14.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-2.4.15** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.15.1 through US-2.4.15.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-2.4.16** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.4.16.1 through US-2.4.16.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-2.4.17** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.4.17.1 through US-2.4.17.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

10. **US-2.4.18** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.4.18.1 through US-2.4.18.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

11. **US-2.4.19** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.4.19.1 through US-2.4.19.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

12. **US-2.4.2** -- **As a** engine developer (P-26), **I want** the capabilities defined in
    sub-stories
US-2.4.2.1 through US-2.4.2.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-2.4.20** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.20.1 through US-2.4.20.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

14. **US-2.4.21** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.21.1 through US-2.4.21.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

15. **US-2.4.22** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.22.1 through US-2.4.22.1 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

16. **US-2.4.23** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.4.23.1 through US-2.4.23.2 combined into a single umbrella feature, **so that** I have a
coherent parent story covering the refined child stories.

17. **US-2.4.3** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.3.1 through US-2.4.3.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

18. **US-2.4.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.4.1 through US-2.4.4.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

19. **US-2.4.5** -- **As a** game developer (P-15), **I want** the capabilities defined in
    sub-stories
US-2.4.5.1 through US-2.4.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

20. **US-2.4.6** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.6.1 through US-2.4.6.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

21. **US-2.4.7** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.7.1 through US-2.4.7.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

22. **US-2.4.8** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.4.8.1 through US-2.4.8.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

23. **US-2.4.9** -- **As a** technical artist (P-13), **I want** the capabilities defined in
    sub-stories
US-2.4.9.1 through US-2.4.9.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
