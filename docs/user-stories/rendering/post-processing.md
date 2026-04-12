# User Stories -- 2.9 Post-Processing

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.9.1.1  | effects artist (P-12)        |
| US-2.9.1.2  | technical artist (P-13)      |
| US-2.9.2.1  | effects artist (P-12)        |
| US-2.9.2.2  | technical artist (P-13)      |
| US-2.9.3.1  | game developer (P-15)        |
| US-2.9.3.2  | technical artist (P-13)      |
| US-2.9.4.1  | environment artist (P-8)     |
| US-2.9.4.2  | technical artist (P-13)      |
| US-2.9.5.1  | environment artist (P-8)     |
| US-2.9.5.2  | technical artist (P-13)      |
| US-2.9.6.1  | effects artist (P-12)        |
| US-2.9.7.1  | effects artist (P-12)        |
| US-2.9.8.1  | effects artist (P-12)        |
| US-2.9.9.1  | effects artist (P-12)        |
| US-2.9.10.1 | technical artist (P-13)      |
| US-2.9.10.2 | effects artist (P-12)        |
| US-2.9.11.1 | environment artist (P-8)     |
| US-2.9.11.2 | technical artist (P-13)      |
| US-2.9.12.1 | game developer (P-15)        |
| US-2.9.13.1 | environment artist (P-8)     |
| US-2.9.13.2 | technical artist (P-13)      |
| US-2.9.14.1 | technical artist (P-13)      |
| US-2.9.14.2 | effects artist (P-12)        |
| US-2.9.15.1 | environment artist (P-8)     |

1. **US-2.9.1.1** — **As a** effects artist (P-12), **I want** bright-source bloom with configurable
   luminance threshold and blur iterations, **so that** neon signs and explosions glow naturally.

2. **US-2.9.1.2** — **As a** technical artist (P-13), **I want** bloom to use dual-kawase blur on
   mobile and multi-pass Gaussian on desktop, **so that** the effect fits within each platform's
   bandwidth budget.

3. **US-2.9.2.1** — **As a** effects artist (P-12), **I want** cinematic depth of field with
   configurable aperture, focal length, and bokeh shape, **so that** cutscenes achieve photographic
   focus effects.

4. **US-2.9.2.2** — **As a** technical artist (P-13), **I want** DOF to fall back to lightweight
   Gaussian blur on mobile, **so that** focus effects are available for cutscenes on all platforms.

5. **US-2.9.3.1** — **As a** game developer (P-15), **I want** per-pixel motion blur using the
   velocity buffer, **so that** fast-moving objects convey speed and the camera simulates shutter
   exposure.

6. **US-2.9.3.2** — **As a** technical artist (P-13), **I want** motion blur disabled on mobile and
   limited to camera-only on Switch, **so that** the effect respects per-platform bandwidth limits.

7. **US-2.9.4.1** — **As a** environment artist (P-8), **I want** auto exposure adapting scene
   brightness based on a luminance histogram, **so that** dark interiors and bright exteriors expose
   naturally.

8. **US-2.9.4.2** — **As a** technical artist (P-13), **I want** histogram resolution to scale from
   average luminance on mobile to 128-bin on high-end, **so that** exposure accuracy matches
   platform capability.

9. **US-2.9.5.1** — **As a** environment artist (P-8), **I want** ACES tonemapping with
   lift/gamma/gain color grading and LUT application, **so that** the final image matches my
   intended color palette.

10. **US-2.9.5.2** — **As a** technical artist (P-13), **I want** HDR10 and Dolby Vision output on
    desktop, **so that** games take advantage of HDR displays.

11. **US-2.9.6.1** — **As a** effects artist (P-12), **I want** procedural film grain with
    configurable intensity and luminance response, **so that** scenes achieve a cinematic
    photographic texture.

12. **US-2.9.7.1** — **As a** effects artist (P-12), **I want** chromatic aberration with
    configurable radial intensity, **so that** lens imperfection adds cinematic character to the
    image.

13. **US-2.9.8.1** — **As a** effects artist (P-12), **I want** image-based lens flare from bright
    sources with ghost and halo artifacts, **so that** strong light sources produce convincing
    optical reflections.

14. **US-2.9.9.1** — **As a** effects artist (P-12), **I want** configurable vignette darkening at
    frame edges, **so that** the image has natural optical falloff.

15. **US-2.9.10.1** — **As a** technical artist (P-13), **I want** custom post-process material
    graphs reading scene textures and outputting arbitrary color transforms, **so that** I can
    create unique stylization effects without engine code changes.

16. **US-2.9.10.2** — **As a** effects artist (P-12), **I want** to chain multiple custom
    post-process passes, **so that** complex screen effects compose from simple reusable building
    blocks.

17. **US-2.9.11.1** — **As a** environment artist (P-8), **I want** local exposure adjusting
    brightness per screen region, **so that** detail is preserved in high-contrast scenes with both
    bright and dark areas.

18. **US-2.9.11.2** — **As a** technical artist (P-13), **I want** the local exposure tile grid to
    scale from disabled on mobile to 32x32 on high-end, **so that** the effect cost matches platform
    capability.

19. **US-2.9.12.1** — **As a** game developer (P-15), **I want** panini projection correcting
    wide-FOV warping at screen edges, **so that** first-person views at 100+ degree FOV avoid
    distortion.

20. **US-2.9.13.1** — **As a** environment artist (P-8), **I want** screen-space cavity and
    curvature enhancement darkening concavities and brightening convexities, **so that**
    micro-surface detail is visible beyond standard lighting.

21. **US-2.9.13.2** — **As a** technical artist (P-13), **I want** curvature-only mode at half-res
    on mobile and full cavity plus curvature on desktop, **so that** the effect scales per platform.

22. **US-2.9.14.1** — **As a** technical artist (P-13), **I want** a node-based post-process graph
    editor to author custom effect chains visually, **so that** I can build image processing
    pipelines without shader code.

23. **US-2.9.14.2** — **As a** effects artist (P-12), **I want** the post-process graph to compile
    to optimized GPU compute dispatches, **so that** custom chains run efficiently within the
    post-processing pipeline.

24. **US-2.9.15.1** — **As an** environment artist (P-8), **I want** overlapping post-process
    volumes to blend their parameters by priority, weight, and shape (global, box, sphere) with
    configurable blend distances, **so that** transitions between areas with different grading,
    exposure, and effects are smooth and art-directed.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.9.1 | effects artist (P-12) |
| US-2.9.10 | technical artist (P-13) |
| US-2.9.11 | environment artist (P-8) |
| US-2.9.12 | game developer (P-15) |
| US-2.9.13 | environment artist (P-8) |
| US-2.9.14 | technical artist (P-13) |
| US-2.9.15 | environment artist (P-8) |
| US-2.9.2 | effects artist (P-12) |
| US-2.9.3 | game developer (P-15) |
| US-2.9.4 | environment artist (P-8) |
| US-2.9.5 | environment artist (P-8) |
| US-2.9.6 | effects artist (P-12) |
| US-2.9.7 | effects artist (P-12) |
| US-2.9.8 | effects artist (P-12) |
| US-2.9.9 | effects artist (P-12) |

1. **US-2.9.1** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-2.9.1.1 through US-2.9.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-2.9.10** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.9.10.1 through US-2.9.10.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.9.11** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.9.11.1 through US-2.9.11.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-2.9.12** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.9.12.1 through US-2.9.12.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-2.9.13** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.9.13.1 through US-2.9.13.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-2.9.14** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.9.14.1 through US-2.9.14.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-2.9.15** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.9.15.1 through US-2.9.15.1 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-2.9.2** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-2.9.2.1 through US-2.9.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-2.9.3** -- **As a** game developer (P-15), **I want** the capabilities defined in
   sub-stories US-2.9.3.1 through US-2.9.3.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

10. **US-2.9.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.9.4.1 through US-2.9.4.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

11. **US-2.9.5** -- **As a** environment artist (P-8), **I want** the capabilities defined in
    sub-stories
US-2.9.5.1 through US-2.9.5.2 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

12. **US-2.9.6** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.9.6.1 through US-2.9.6.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

13. **US-2.9.7** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.9.7.1 through US-2.9.7.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

14. **US-2.9.8** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.9.8.1 through US-2.9.8.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.

15. **US-2.9.9** -- **As a** effects artist (P-12), **I want** the capabilities defined in
    sub-stories
US-2.9.9.1 through US-2.9.9.1 combined into a single umbrella feature, **so that** I have a coherent
parent story covering the refined child stories.
