# User Stories -- 2.8 Character Rendering

## Stories

| ID         | Persona                      |
|------------|------------------------------|
| US-2.8.1.1 | environment artist (P-8)     |
| US-2.8.1.2 | technical artist (P-13)      |
| US-2.8.2.1 | technical artist (P-13)      |
| US-2.8.2.2 | environment artist (P-8)     |
| US-2.8.3.1 | technical artist (P-13)      |
| US-2.8.4.1 | environment artist (P-8)     |
| US-2.8.4.2 | technical artist (P-13)      |
| US-2.8.5.1 | environment artist (P-8)     |
| US-2.8.5.2 | technical artist (P-13)      |
| US-2.8.6.1 | environment artist (P-8)     |
| US-2.8.6.2 | technical artist (P-13)      |
| US-2.8.7.1 | engine developer (P-26)      |
| US-2.8.7.2 | technical artist (P-13)      |
| US-2.8.8.1 | environment artist (P-8)     |
| US-2.8.9.1 | environment artist (P-8)     |
| US-2.8.9.2 | technical artist (P-13)      |

1. **US-2.8.1.1** — **As a** environment artist (P-8), **I want** individual hair strand rendering
   with the Marschner BSDF producing reflection, transmission, and double-reflection lobes,
   **so that** hero character hair looks photorealistic up close.

2. **US-2.8.1.2** — **As a** technical artist (P-13), **I want** strand self-shadowing via deep
   opacity maps, **so that** dense hair volumes have correct internal shadowing without manual
   shadow setup.

3. **US-2.8.2.1** — **As a** technical artist (P-13), **I want** card-based hair rendering as a
   fallback for mid-distance characters, **so that** hair is visible on all platforms without strand
   geometry cost.

4. **US-2.8.2.2** — **As a** environment artist (P-8), **I want** layered polygon card hair with
   alpha blending, **so that** characters at mid range have volumetric-looking hairstyles.

5. **US-2.8.3.1** — **As a** technical artist (P-13), **I want** multi-tier hair LOD transitioning
   from strands to cards to mesh proxy with cross-fade dithering, **so that** hair representation
   scales smoothly with distance.

6. **US-2.8.4.1** — **As a** environment artist (P-8), **I want** a layered eye model with cornea
   refraction, iris parallax, and sclera subsurface scattering, **so that** character eyes have
   photorealistic depth.

7. **US-2.8.4.2** — **As a** technical artist (P-13), **I want** the eye model to simplify to a flat
   iris texture on mobile, **so that** eyes render at acceptable quality on all platforms.

8. **US-2.8.5.1** — **As a** environment artist (P-8), **I want** a cloth shading model with a fiber
   fuzz layer, **so that** fabric surfaces scatter light differently from smooth materials.

9. **US-2.8.5.2** — **As a** technical artist (P-13), **I want** cloth fuzz limited to hero
   characters on Switch and disabled on mobile, **so that** the effect fits within per-platform ALU
   budgets.

10. **US-2.8.6.1** — **As a** environment artist (P-8), **I want** screen-space subsurface
    scattering with per-material scatter profiles for skin, **so that** characters have soft,
    lifelike skin rendering.

11. **US-2.8.6.2** — **As a** technical artist (P-13), **I want** the Burley diffusion model on
    desktop with a preintegrated LUT fallback on mobile, **so that** skin SSS scales across hardware
    tiers.

12. **US-2.8.7.1** — **As a** engine developer (P-26), **I want** a compute software rasterizer for
    sub-pixel hair strands, **so that** thin strands that bypass hardware rasterization are rendered
    with anti-aliasing.

13. **US-2.8.7.2** — **As a** technical artist (P-13), **I want** compute rasterized hair to blend
    seamlessly with hardware-rasterized strands, **so that** strand transitions are invisible to the
    viewer.

14. **US-2.8.8.1** — **As a** environment artist (P-8), **I want** peach fuzz vellus hair as a
    screen-space effect on character faces, **so that** close-up skin has subtle light-catching fuzz
    without strand geometry.

15. **US-2.8.9.1** — **As a** environment artist (P-8), **I want** biometric skin shading driven by
    melanin and blood distribution maps, **so that** any skin tone renders accurately from
    biological parameters.

16. **US-2.8.9.2** — **As a** technical artist (P-13), **I want** melanin and blood maps baked to
    diffuse color on mobile at import time, **so that** biometric skin is available without runtime
    pigment evaluation.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.8.1 | environment artist (P-8) |
| US-2.8.2 | technical artist (P-13) |
| US-2.8.3 | technical artist (P-13) |
| US-2.8.4 | environment artist (P-8) |
| US-2.8.5 | environment artist (P-8) |
| US-2.8.6 | environment artist (P-8) |
| US-2.8.7 | engine developer (P-26) |
| US-2.8.8 | environment artist (P-8) |
| US-2.8.9 | environment artist (P-8) |

1. **US-2.8.1** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.1.1 through US-2.8.1.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

2. **US-2.8.2** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.8.2.1 through US-2.8.2.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

3. **US-2.8.3** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.8.3.1 through US-2.8.3.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

4. **US-2.8.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.4.1 through US-2.8.4.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

5. **US-2.8.5** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.5.1 through US-2.8.5.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

6. **US-2.8.6** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.6.1 through US-2.8.6.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

7. **US-2.8.7** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-2.8.7.1 through US-2.8.7.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

8. **US-2.8.8** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.8.1 through US-2.8.8.1 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.

9. **US-2.8.9** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.8.9.1 through US-2.8.9.2 combined into a single umbrella feature, **so that** I
   have a coherent parent story covering the refined child stories.
