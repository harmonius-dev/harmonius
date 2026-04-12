# User Stories -- 2.12 Advanced Materials

## Stories

| ID          | Persona                      |
|-------------|------------------------------|
| US-2.12.1.1 | environment artist (P-8)     |
| US-2.12.1.2 | technical artist (P-13)      |
| US-2.12.2.1 | environment artist (P-8)     |
| US-2.12.2.2 | technical artist (P-13)      |
| US-2.12.3.1 | effects artist (P-12)        |
| US-2.12.3.2 | environment artist (P-8)     |
| US-2.12.4.1 | environment artist (P-8)     |
| US-2.12.4.2 | technical artist (P-13)      |
| US-2.12.5.1 | environment artist (P-8)     |
| US-2.12.5.2 | technical artist (P-13)      |
| US-2.12.6.1 | environment artist (P-8)     |
| US-2.12.6.2 | technical artist (P-13)      |
| US-2.12.7.1 | environment artist (P-8)     |
| US-2.12.7.2 | technical artist (P-13)      |
| US-2.12.8.1 | environment artist (P-8)     |
| US-2.12.8.2 | technical artist (P-13)      |
| US-2.12.9.1 | technical artist (P-13)      |
| US-2.12.9.2 | environment artist (P-8)     |
| US-2.12.9.3 | effects artist (P-12)        |

1. **US-2.12.1.1** — **As a** environment artist (P-8), **I want** physically-based glass, crystal,
   and ice materials with configurable index of refraction, **so that** transparent objects bend
   light realistically.

2. **US-2.12.1.2** — **As a** technical artist (P-13), **I want** screen-space refraction as a
   fallback when RT hardware is unavailable, **so that** transparent materials work on all
   platforms.

3. **US-2.12.2.1** — **As a** environment artist (P-8), **I want** ocean reflection and refraction
   compositing above-water planar reflections with underwater Fresnel visibility, **so that** large
   water surfaces look convincing from all angles.

4. **US-2.12.2.2** — **As a** technical artist (P-13), **I want** reflection quality to scale with
   screen coverage from low-res probes at distance to full-res planar near camera, **so that** ocean
   reflection cost is proportional.

5. **US-2.12.3.1** — **As a** effects artist (P-12), **I want** emissive materials with HDR
   intensity triggering bloom and animated emission via scrolling UV and pulsing, **so that** neon
   signs and magic effects glow dynamically.

6. **US-2.12.3.2** — **As a** environment artist (P-8), **I want** emissive surfaces contributing to
   indirect lighting when GI is active, **so that** glowing objects cast colored light onto nearby
   surfaces.

7. **US-2.12.4.1** — **As a** environment artist (P-8), **I want** heightmap-driven GPU tessellation
   adding geometric detail to cobblestone, brick, and carved stone, **so that** flat surfaces gain
   visible relief.

8. **US-2.12.4.2** — **As a** technical artist (P-13), **I want** parallax occlusion mapping as a
   fallback when tessellation is too expensive, **so that** surface detail is present on all
   platforms.

9. **US-2.12.5.1** — **As a** environment artist (P-8), **I want** fabric materials with sheen BRDF,
   thread direction maps, and thin-surface transmission, **so that** velvet, silk, and cotton render
   distinctly.

10. **US-2.12.5.2** — **As a** technical artist (P-13), **I want** fabric shading to degrade from
    full sheen on desktop to basic diffuse wrap on mobile, **so that** cloth materials are present
    on all platforms.

11. **US-2.12.6.1** — **As a** environment artist (P-8), **I want** enhanced PBR for metal, wood,
    and stone with anisotropic reflections, SSS, and a shared weathering system, **so that** natural
    surfaces show realistic age and exposure.

12. **US-2.12.6.2** — **As a** technical artist (P-13), **I want** weathering baked to textures on
    mobile and evaluated at runtime on desktop, **so that** environmental wear is present across all
    platforms.

13. **US-2.12.7.1** — **As a** environment artist (P-8), **I want** rubber, wax, and soft surface
    materials with subsurface scattering and deformation feedback, **so that** thin wax glows when
    backlit and rubber lightens when stretched.

14. **US-2.12.7.2** — **As a** technical artist (P-13), **I want** SSS and deformation feedback
    disabled on mobile with a high-roughness BRDF fallback, **so that** soft material appearance is
    acceptable everywhere.

15. **US-2.12.8.1** — **As a** environment artist (P-8), **I want** clearcoat materials with
    independent roughness, IOR, and separate normal maps for automotive paint and lacquered
    surfaces, **so that** multi-layer coatings are realistic.

16. **US-2.12.8.2** — **As a** technical artist (P-13), **I want** clearcoat to degrade from
    unlimited layers on high-end to 2 baked layers on mobile, **so that** multi-layer materials
    scale across platforms.

17. **US-2.12.9.1** — **As a** technical artist (P-13), **I want** fully custom material graphs with
    access to all rendering inputs and configurable output targets, **so that** I can implement any
    shading model without engine code changes.

18. **US-2.12.9.2** — **As a** environment artist (P-8), **I want** to create procedural materials
    like animated lava and holographic displays in the visual shader graph, **so that** unique
    effects require no programming.

19. **US-2.12.9.3** — **As a** effects artist (P-12), **I want** reusable material functions
    composing into complex materials, **so that** I build effects from shared building blocks.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-2.12.1 | environment artist (P-8) |
| US-2.12.2 | environment artist (P-8) |
| US-2.12.3 | effects artist (P-12) |
| US-2.12.4 | environment artist (P-8) |
| US-2.12.5 | environment artist (P-8) |
| US-2.12.6 | environment artist (P-8) |
| US-2.12.7 | environment artist (P-8) |
| US-2.12.8 | environment artist (P-8) |
| US-2.12.9 | technical artist (P-13) |

1. **US-2.12.1** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.1.1 through US-2.12.1.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

2. **US-2.12.2** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.2.1 through US-2.12.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-2.12.3** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-2.12.3.1 through US-2.12.3.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-2.12.4** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.4.1 through US-2.12.4.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-2.12.5** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.5.1 through US-2.12.5.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-2.12.6** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.6.1 through US-2.12.6.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

7. **US-2.12.7** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.7.1 through US-2.12.7.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

8. **US-2.12.8** -- **As a** environment artist (P-8), **I want** the capabilities defined in
   sub-stories US-2.12.8.1 through US-2.12.8.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

9. **US-2.12.9** -- **As a** technical artist (P-13), **I want** the capabilities defined in
   sub-stories US-2.12.9.1 through US-2.12.9.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.
