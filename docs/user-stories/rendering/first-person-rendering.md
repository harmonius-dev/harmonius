# User Stories -- 2.13 First-Person Rendering

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.13.1.1 | game developer (P-15)        |
| US-2.13.1.2 | engine developer (P-26)      |
| US-2.13.2.1 | game developer (P-15)        |
| US-2.13.2.2 | technical artist (P-13)      |
| US-2.13.3.1 | engine developer (P-26)      |
| US-2.13.3.2 | game developer (P-15)        |
| US-2.13.4.1 | environment artist (P-8)     |
| US-2.13.4.2 | engine developer (P-26)      |
| US-2.13.5.1 | game developer (P-15)        |
| US-2.13.5.2 | effects artist (P-12)        |
| US-2.13.6.1 | game developer (P-15)        |
| US-2.13.7.1 | engine developer (P-26)      |
| US-2.13.7.2 | technical artist (P-13)      |
| US-2.13.8.1 | game developer (P-15)        |
| US-2.13.9.1 | game developer (P-15)        |
| US-2.13.9.2 | effects artist (P-12)        |

1. **US-2.13.1.1** — **As a** game developer (P-15), **I want** a dedicated viewmodel render layer
   for first- person arms and weapons, **so that** viewmodel geometry always renders in front of the
   world.

2. **US-2.13.1.2** — **As a** engine developer (P-26), **I want** the viewmodel pass scheduled after
   the main scene pass via the render graph, **so that** viewmodel depth is independent of world
   depth values.

3. **US-2.13.2.1** — **As a** game developer (P-15), **I want** viewmodel FOV independent of world
   camera FOV, **so that** weapons are not distorted at wide field-of-view settings.

4. **US-2.13.2.2** — **As a** technical artist (P-13), **I want** viewmodel near-clip adjustment on
   mobile where depth precision is 16-bit, **so that** arms render close to the camera without
   clipping artifacts.

5. **US-2.13.3.1** — **As a** engine developer (P-26), **I want** viewmodel depth composited into
   the final depth buffer at a fixed near value, **so that** DOF and SSAO treat the viewmodel as
   foreground.

6. **US-2.13.3.2** — **As a** game developer (P-15), **I want** transparent viewmodel surfaces like
   glass sights to blend correctly with the scene behind them, **so that** holographic sights are
   see-through.

7. **US-2.13.4.1** — **As a** environment artist (P-8), **I want** viewmodel surfaces to receive the
   same lighting, shadow maps, and reflection probes as world geometry, **so that** weapons match
   the scene lighting.

8. **US-2.13.4.2** — **As a** engine developer (P-26), **I want** dynamic lights near the camera to
   illuminate both viewmodel and world consistently, **so that** muzzle flash and flashlight light
   both arms and the environment.

9. **US-2.13.5.1** — **As a** game developer (P-15), **I want** viewmodel geometry to skip motion
   blur and depth of field via per-pixel stencil, **so that** weapons remain sharp during fast arm
   movement.

10. **US-2.13.5.2** — **As a** effects artist (P-12), **I want** viewmodel to still receive
    tonemapping, color grading, and film grain, **so that** the weapon matches the scene's visual
    treatment.

11. **US-2.13.6.1** — **As a** game developer (P-15), **I want** first-person arms and weapons to
    cast shadows into the world, **so that** the viewmodel casts realistic shadows on nearby
    surfaces.

12. **US-2.13.7.1** — **As a** engine developer (P-26), **I want** the viewmodel pass to write
    motion vectors using viewmodel projection deltas, **so that** TAA handles viewmodel edges
    correctly.

13. **US-2.13.7.2** — **As a** technical artist (P-13), **I want** correct viewmodel motion vectors
    for frame generation, **so that** weapon motion is reconstructed independently from world
    motion.

14. **US-2.13.8.1** — **As a** game developer (P-15), **I want** VR stereo viewmodel rendering with
    per-eye near-clip and hand tracking integration, **so that** weapons appear at the correct
    stereoscopic depth.

15. **US-2.13.9.1** — **As a** game developer (P-15), **I want** stencil-based scope lens masking
    rendering magnified scene inside a viewmodel cutout, **so that** sniper scopes show a zoomed
    view.

16. **US-2.13.9.2** — **As a** effects artist (P-12), **I want** muzzle flash and shell ejection VFX
    to write to the viewmodel stencil layer, **so that** effects occlude correctly against viewmodel
    geometry.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.13.1 | game developer (P-15) |
| US-2.13.2 | game developer (P-15) |
| US-2.13.3 | engine developer (P-26) |
| US-2.13.4 | environment artist (P-8) |
| US-2.13.5 | game developer (P-15) |
| US-2.13.6 | game developer (P-15) |
| US-2.13.7 | engine developer (P-26) |
| US-2.13.8 | game developer (P-15) |
| US-2.13.9 | game developer (P-15) |

1. **US-2.13.1** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.1.1 through US-2.13.1.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

2. **US-2.13.2** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.2.1 through US-2.13.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.13.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.13.3.1 through US-2.13.3.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-2.13.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.13.4.1 through US-2.13.4.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-2.13.5** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.5.1 through US-2.13.5.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-2.13.6** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.6.1 through US-2.13.6.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-2.13.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.13.7.1 through US-2.13.7.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-2.13.8** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.8.1 through US-2.13.8.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-2.13.9** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.13.9.1 through US-2.13.9.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.
