# User Stories -- 11.4 Weather & Environmental FX

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-11.4.1.1 | effects artist (P-12)   |
| US-11.4.1.2 | effects artist (P-12)   |
| US-11.4.1.3 | engine developer (P-26) |
| US-11.4.2.1 | effects artist (P-12)   |
| US-11.4.2.2 | technical artist (P-13) |
| US-11.4.2.3 | engine developer (P-26) |
| US-11.4.3.1 | effects artist (P-12)   |
| US-11.4.3.2 | effects artist (P-12)   |
| US-11.4.3.3 | engine developer (P-26) |
| US-11.4.4.1 | effects artist (P-12)   |
| US-11.4.4.2 | game designer (P-5)     |
| US-11.4.4.3 | engine developer (P-26) |
| US-11.4.5.1 | effects artist (P-12)   |
| US-11.4.5.2 | effects artist (P-12)   |
| US-11.4.5.3 | engine developer (P-26) |
| US-11.4.6.1 | effects artist (P-12)   |
| US-11.4.6.2 | effects artist (P-12)   |
| US-11.4.6.3 | engine developer (P-26) |
| US-11.4.7.1 | effects artist (P-12)   |
| US-11.4.7.2 | effects artist (P-12)   |
| US-11.4.7.3 | engine developer (P-26) |

1. **US-11.4.1.1** — **As a** effects artist (P-12), **I want** multi-layered rain combining GPU
   particle streaks, screen-space camera droplets, and ripple normal maps on wet surfaces,
   **so that** rain feels immersive from world scale down to close-up.

2. **US-11.4.1.2** — **As a** effects artist (P-12), **I want** particle density and screen droplet
   frequency scaled by weather intensity, **so that** drizzle and downpour look and feel distinct.

3. **US-11.4.1.3** — **As a** engine developer (P-26), **I want** mobile to use a single particle
   layer with no screen droplets, 25% density, and half-res ripples, **so that** rain fits within
   mobile bandwidth budgets.

4. **US-11.4.2.1** — **As a** effects artist (P-12), **I want** dynamic puddles forming from
   heightfield accumulation with mirror-smooth roughness, darkened albedo, and animated ripples
   during rainfall, **so that** rain transforms the ground convincingly.

5. **US-11.4.2.2** — **As a** technical artist (P-13), **I want** per-material wet responses (stone
   darkens, metal reflects, dirt becomes mud), **so that** each surface reacts to rain in a
   physically appropriate way.

6. **US-11.4.2.3** — **As a** engine developer (P-26), **I want** mobile to use pre-placed puddle
   decals with albedo-darken only, **so that** puddle rendering is cheap on mobile.

7. **US-11.4.3.1** — **As a** effects artist (P-12), **I want** vertex-displacement snow
   accumulating on upward-facing surfaces over time with character and vehicle deformation trails,
   **so that** snowfall transforms the landscape.

8. **US-11.4.3.2** — **As a** effects artist (P-12), **I want** configurable accumulation rate,
   maximum depth, and deformation trail fade speed per zone, **so that** tundra zones accumulate
   faster than mountain passes.

9. **US-11.4.3.3** — **As a** engine developer (P-26), **I want** mobile to use texture-blend snow
   with decal-based deformation (no vertex displacement), **so that** snow stays within mobile
   vertex processing budgets.

10. **US-11.4.4.1** — **As a** effects artist (P-12), **I want** oriented box and convex hull fog
    volumes with per-volume density, color, height falloff, and animation, **so that** swamp haze
    and dungeon mist are localized without affecting global fog.

11. **US-11.4.4.2** — **As a** game designer (P-5), **I want** fog volumes to inject into the global
    froxel grid with temporal reprojection, **so that** volumetric fog is consistent with the global
    atmosphere.

12. **US-11.4.4.3** — **As a** engine developer (P-26), **I want** mobile to use screen-space height
    fog instead of froxel injection, **so that** fog effects work without volumetric rendering on
    mobile.

13. **US-11.4.5.1** — **As a** effects artist (P-12), **I want** procedural branching lightning
    bolts using L-system subdivision with single-frame illumination and exponential decay,
    **so that** storms have dramatic lighting events.

14. **US-11.4.5.2** — **As a** effects artist (P-12), **I want** configurable branching depth,
    segment length, and simultaneous bolt count with distance-delayed thunder, **so that** I can
    build intensity from single bolts to full electrical storms.

15. **US-11.4.5.3** — **As a** engine developer (P-26), **I want** mobile limited to branching depth
    2, max 1 bolt, and a single directional flash, **so that** lightning stays within mobile compute
    and lighting budgets.

16. **US-11.4.6.1** — **As a** effects artist (P-12), **I want** GPU particle-driven wind
    visualization of leaves, dust, and debris with velocity from the shared wind field, **so that**
    wind feels like a unified physical force.

17. **US-11.4.6.2** — **As a** effects artist (P-12), **I want** dust storms injecting scattering
    density into atmospheric fog with sky color tinting, **so that** approaching storms create
    dramatic visibility reduction.

18. **US-11.4.6.3** — **As a** engine developer (P-26), **I want** mobile to use 10% of desktop wind
    particles with distance fog only for dust storms, **so that** wind visualization fits within
    mobile budgets.

19. **US-11.4.7.1** — **As a** effects artist (P-12), **I want** animated caustic light patterns on
    submerged geometry with wavelength-dependent depth fog and bubble streams, **so that**
    underwater environments feel atmospheric.

20. **US-11.4.7.2** — **As a** effects artist (P-12), **I want** refraction distortion at the water
    surface boundary and screen-space god rays from above, **so that** underwater has the
    characteristic rippling interface.

21. **US-11.4.7.3** — **As a** engine developer (P-26), **I want** mobile to skip caustics and god
    rays, using depth fog and blue tint only with 25% bubble count, **so that** underwater rendering
    stays within mobile GPU budgets.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-11.4.1 | effects artist (P-12) |
| US-11.4.2 | effects artist (P-12) |
| US-11.4.3 | effects artist (P-12) |
| US-11.4.4 | effects artist (P-12) |
| US-11.4.5 | effects artist (P-12) |
| US-11.4.6 | effects artist (P-12) |
| US-11.4.7 | effects artist (P-12) |

1. **US-11.4.1** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.1.1 through US-11.4.1.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

2. **US-11.4.2** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.2.1 through US-11.4.2.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-11.4.3** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.3.1 through US-11.4.3.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-11.4.4** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.4.1 through US-11.4.4.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-11.4.5** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.5.1 through US-11.4.5.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-11.4.6** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.6.1 through US-11.4.6.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-11.4.7** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.4.7.1 through US-11.4.7.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.
