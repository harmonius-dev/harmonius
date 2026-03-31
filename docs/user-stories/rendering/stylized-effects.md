# User Stories -- 2.11 Stylized and Gameplay Effects

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.11.1.1 | game developer (P-15)        |
| US-2.11.1.2 | effects artist (P-12)        |
| US-2.11.1.3 | technical artist (P-13)      |
| US-2.11.2.1 | game developer (P-15)        |
| US-2.11.2.2 | effects artist (P-12)        |
| US-2.11.3.1 | environment artist (P-8)     |
| US-2.11.3.2 | technical artist (P-13)      |
| US-2.11.4.1 | game developer (P-15)        |
| US-2.11.4.2 | environment artist (P-8)     |
| US-2.11.5.1 | game developer (P-15)        |
| US-2.11.5.2 | effects artist (P-12)        |

1. **US-2.11.1.1** — **As a** game developer (P-15), **I want** per-entity outlines with
   configurable color, width, and style, **so that** selected, hovered, and quest-target objects are
   visually distinct.

2. **US-2.11.1.2** — **As a** effects artist (P-12), **I want** jump-flood-based variable-width
   outlines with smooth falloff, **so that** outline quality scales independently of screen
   resolution.

3. **US-2.11.1.3** — **As a** technical artist (P-13), **I want** outline rendering to fall back
   from jump-flood to Sobel edge detection on devices without compute, **so that** outlines work on
   all platforms.

4. **US-2.11.2.1** — **As a** game developer (P-15), **I want** an emissive highlight overlay with
   pulsing, fresnel rim, and inner glow modes driven by an ECS component, **so that** gameplay
   systems control object highlighting directly.

5. **US-2.11.2.2** — **As a** effects artist (P-12), **I want** highlight glow with Gaussian blur
   and additive compositing, **so that** interactive objects are visually prominent with
   configurable intensity.

6. **US-2.11.3.1** — **As a** environment artist (P-8), **I want** a configurable cel-shading
   pipeline with artist-controlled band counts, ramp textures, and shaped specular highlights,
   **so that** toon-styled games have full creative control over lighting.

7. **US-2.11.3.2** — **As a** technical artist (P-13), **I want** toon shading to scale from 2-3
   bands on mobile to full band and hatching support on desktop, **so that** stylized rendering fits
   all platform budgets.

8. **US-2.11.4.1** — **As a** game developer (P-15), **I want** automatic roof and ceiling fading
   when the camera moves beneath occluding geometry, **so that** isometric and top-down games reveal
   interior spaces.

9. **US-2.11.4.2** — **As a** environment artist (P-8), **I want** configurable fade modes (volume,
   ray, layer) with dithered or smooth alpha dissolve, **so that** occlusion removal matches my
   game's visual style.

10. **US-2.11.5.1** — **As a** game developer (P-15), **I want** x-ray silhouettes rendering
    occluded entities through walls at configurable priority levels, **so that** allies and enemies
    are visible through cover.

11. **US-2.11.5.2** — **As a** effects artist (P-12), **I want** x-ray silhouettes with flat color,
    team tint, or fresnel outline modes, **so that** occluded entity visibility matches the game's
    visual language.
