# User Stories -- 2.7 Environment and Atmosphere

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.7.1.1  | environment artist (P-8)     |
| US-2.7.1.2  | technical artist (P-13)      |
| US-2.7.2.1  | environment artist (P-8)     |
| US-2.7.2.2  | technical artist (P-13)      |
| US-2.7.3.1  | environment artist (P-8)     |
| US-2.7.3.2  | technical artist (P-13)      |
| US-2.7.4.1  | environment artist (P-8)     |
| US-2.7.5.1  | technical artist (P-13)      |
| US-2.7.5.2  | environment artist (P-8)     |
| US-2.7.6.1  | environment artist (P-8)     |
| US-2.7.6.2  | technical artist (P-13)      |
| US-2.7.7.1  | effects artist (P-12)        |
| US-2.7.7.2  | environment artist (P-8)     |
| US-2.7.8.1  | environment artist (P-8)     |
| US-2.7.8.2  | technical artist (P-13)      |
| US-2.7.9.1  | environment artist (P-8)     |
| US-2.7.10.1 | environment artist (P-8)     |
| US-2.7.10.2 | game developer (P-15)        |

1. **US-2.7.1.1** — **As a** environment artist (P-8), **I want** a physically-based sky atmosphere
   with time-of-day transitions, **so that** sunrises, sunsets, and daytime skies look natural.

2. **US-2.7.1.2** — **As a** technical artist (P-13), **I want** precomputed atmosphere LUTs with
   configurable resolution, **so that** sky rendering fits within mobile bandwidth without runtime
   recomputation.

3. **US-2.7.2.1** — **As a** environment artist (P-8), **I want** ray-marched volumetric fog using a
   froxel grid, **so that** shafts of light and atmospheric haze add depth to interior and exterior
   scenes.

4. **US-2.7.2.2** — **As a** technical artist (P-13), **I want** the froxel grid to scale from
   disabled on mobile to 160x90x128 on high-end, **so that** volumetric fog quality matches the
   platform budget.

5. **US-2.7.3.1** — **As a** environment artist (P-8), **I want** procedural volumetric clouds
   ray-marched through noise textures with temporal reprojection, **so that** realistic cloud
   formations enhance the sky.

6. **US-2.7.3.2** — **As a** technical artist (P-13), **I want** cloud ray march step count and
   resolution to scale per platform, **so that** clouds render within budget from mobile to
   high-end.

7. **US-2.7.4.1** — **As a** environment artist (P-8), **I want** god rays from bright occluded
   sources using screen-space or volumetric integration, **so that** sunlight streaming through
   windows is visible.

8. **US-2.7.5.1** — **As a** technical artist (P-13), **I want** analytical exponential fog as the
   primary fog method on mobile, **so that** atmospheric depth is present without volumetric froxel
   cost.

9. **US-2.7.5.2** — **As a** environment artist (P-8), **I want** distance and height fog with
   configurable falloff, **so that** distant objects fade naturally regardless of whether volumetric
   fog is active.

10. **US-2.7.6.1** — **As a** environment artist (P-8), **I want** an FFT ocean surface with
    reflections, foam, and underwater effects, **so that** large water bodies look and move
    realistically.

11. **US-2.7.6.2** — **As a** technical artist (P-13), **I want** the ocean to fall back to Gerstner
    wave sums on mobile, **so that** water is present on all platforms at reduced quality.

12. **US-2.7.7.1** — **As a** effects artist (P-12), **I want** volumetric rendering of fire, smoke,
    and explosions from OpenVDB data, **so that** imported Houdini volumes integrate with scene
    lighting.

13. **US-2.7.7.2** — **As a** environment artist (P-8), **I want** sparse volume textures
    ray-marched with emission, absorption, and scattering, **so that** environmental fog volumes
    have volumetric depth.

14. **US-2.7.8.1** — **As a** environment artist (P-8), **I want** voxel-based volumetric clouds
    with SDF acceleration for fly-through gameplay, **so that** high altitude scenes feature
    production-quality clouds.

15. **US-2.7.8.2** — **As a** technical artist (P-13), **I want** voxel clouds to fall back to
    noise-based ray marching on platforms without sufficient VRAM, **so that** clouds degrade
    gracefully.

16. **US-2.7.9.1** — **As a** environment artist (P-8), **I want** art-directable breaking waves
    with guide curves and Houdini-simulated deformation, **so that** shoreline wave shapes match my
    artistic intent.

17. **US-2.7.10.1** — **As a** environment artist (P-8), **I want** a weather state machine driving
    clouds, fog, precipitation, and lighting transitions, **so that** weather changes feel natural
    over configurable periods.

18. **US-2.7.10.2** — **As a** game developer (P-15), **I want** weather states to influence
    material wetness, puddle rendering, and vegetation animation, **so that** the world responds
    visually to rain and snow.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.7.1 | environment artist (P-8) |
| US-2.7.10 | environment artist (P-8) |
| US-2.7.2 | environment artist (P-8) |
| US-2.7.3 | environment artist (P-8) |
| US-2.7.4 | environment artist (P-8) |
| US-2.7.5 | technical artist (P-13) |
| US-2.7.6 | environment artist (P-8) |
| US-2.7.7 | effects artist (P-12) |
| US-2.7.8 | environment artist (P-8) |
| US-2.7.9 | environment artist (P-8) |

1. **US-2.7.1** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.1.1 through US-2.7.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-2.7.10** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.10.1 through US-2.7.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.7.2** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.2.1 through US-2.7.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-2.7.3** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.3.1 through US-2.7.3.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-2.7.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.4.1 through US-2.7.4.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-2.7.5** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.7.5.1 through US-2.7.5.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-2.7.6** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.6.1 through US-2.7.6.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-2.7.7** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-2.7.7.1 through US-2.7.7.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-2.7.8** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.7.8.1 through US-2.7.8.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-2.7.9** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.7.9.1 through US-2.7.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
