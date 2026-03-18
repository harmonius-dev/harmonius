# User Stories -- 2.12 Advanced Materials

## Stories

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-2.12.1.1 | environment artist (P-8) |          |              |
| US-2.12.1.2 | character artist (P-9)   |          |              |
| US-2.12.1.3 | engine tester (P-27)     |          |              |
| US-2.12.2.1 | player (P-23)            |          |              |
| US-2.12.2.2 | engine tester (P-27)     |          |              |
| US-2.12.3.1 | environment artist (P-8) |          |              |
| US-2.12.3.2 | effects artist (P-12)    |          |              |
| US-2.12.3.3 | engine developer (P-26)  |          |              |
| US-2.12.4.1 | environment artist (P-8) |          |              |
| US-2.12.4.2 | technical artist (P-13)  |          |              |
| US-2.12.4.3 | engine tester (P-27)     |          |              |
| US-2.12.5.1 | character artist (P-9)   |          |              |
| US-2.12.5.2 | environment artist (P-8) |          |              |
| US-2.12.5.3 | engine tester (P-27)     |          |              |
| US-2.12.6.1 | environment artist (P-8) |          |              |
| US-2.12.6.2 | technical artist (P-13)  |          |              |
| US-2.12.6.3 | engine tester (P-27)     |          |              |
| US-2.12.7.1 | environment artist (P-8) |          |              |
| US-2.12.7.2 | engine developer (P-26)  |          |              |
| US-2.12.8.1 | technical artist (P-13)  |          |              |
| US-2.12.8.2 | environment artist (P-8) |          |              |
| US-2.12.8.3 | engine tester (P-27)     |          |              |
| US-2.12.9.1 | technical artist (P-13)  |          |              |
| US-2.12.9.2 | environment artist (P-8) |          |              |
| US-2.12.9.3 | engine tester (P-27)     |          |              |

1. **US-2.12.1.1** — I want to create transparent glass materials with configurable IOR, chromatic
   dispersion, and Fresnel reflectance
   - **Acceptance:** stained glass windows and crystal chandeliers refract light realistically
     without custom shader work
2. **US-2.12.1.2** — I want to toggle between thin-surface mode (flat glass) and thick-surface mode
   (solid gemstones) per material
   - **Acceptance:** I can author jewelry and potion bottles with correct internal refraction paths
3. **US-2.12.1.3** — I want to disable RT hardware and verify that transparent glass falls back to
   screen-space refraction without visual corruption
   - **Acceptance:** glass rendering works on all hardware configurations
4. **US-2.12.2.1** — I want ocean water to show Fresnel-weighted reflections of the sky and
   shoreline with wave-normal distortion
   - **Acceptance:** the water surface looks believable from both above and below
5. **US-2.12.2.2** — I want to verify that distant ocean uses low-resolution reflection probes while
   near-camera water uses full-resolution planar or RT reflections
   - **Acceptance:** reflection quality scales appropriately with screen coverage
6. **US-2.12.3.1** — I want emissive materials with HDR intensity values that interact with the
   bloom pass
   - **Acceptance:** neon signs, screens, and magic runes glow brightly and bleed light into
     surrounding areas
7. **US-2.12.3.2** — I want to animate emission intensity with scrolling UV, pulsing curves, and
   color cycling driven by material parameters
   - **Acceptance:** I can create living surfaces like lava flows and enchanted weapons without
     per-frame texture swaps
8. **US-2.12.3.3** — I want to verify that emissive surfaces contribute indirect lighting onto
   nearby surfaces when RT GI is active
   - **Acceptance:** a glowing furnace casts colored light on surrounding walls as expected
9. **US-2.12.4.1** — I want GPU tessellation driven by heightmap textures to add real geometric
   detail to flat cobblestone and brick surfaces
   - **Acceptance:** close-up views show actual depth and parallax rather than a flat normal-mapped
     approximation
10. **US-2.12.4.2** — I want to choose between tessellation and POM per material based on platform
    budget
    - **Acceptance:** surfaces still show convincing depth on platforms where tessellation is too
      expensive
11. **US-2.12.4.3** — I want to confirm that tessellation density increases near the camera and
    drops to minimal subdivision at distance
    - **Acceptance:** tessellation cost is proportional to screen coverage
12. **US-2.12.5.1** — I want fabric materials with a sheen BRDF layer and thread direction maps
    - **Acceptance:** velvet cloaks and silk gowns show the soft fuzzy highlight and weave pattern
      that distinguishes them from hard surfaces
13. **US-2.12.5.2** — I want thin-surface transmission on fabric materials so that curtains and
    flags glow when backlit by sunlight
    - **Acceptance:** interior scenes have natural light filtering through window drapery
14. **US-2.12.5.3** — I want to verify that mobile uses basic diffuse wrap only with no sheen or
    fuzz layers
    - **Acceptance:** fabric rendering remains performant on low-end hardware without shader errors
15. **US-2.12.6.1** — I want metal materials with tangent-space anisotropic reflections and
    per-pixel roughness variation
    - **Acceptance:** brushed steel, machined aluminum, and weathered iron each show distinctive
      specular behavior
16. **US-2.12.6.2** — I want a shared weathering system that drives procedural rust, moss, cracks,
    and stains from age/exposure/damage parameters
    - **Acceptance:** I can weather any natural material without authoring unique detail textures
      per variant
17. **US-2.12.6.3** — I want to verify that stone SSS (marble, alabaster) renders with porous
    subsurface scattering on desktop and falls back to baked textures on mobile
    - **Acceptance:** quality scales correctly across platforms
18. **US-2.12.7.1** — I want wax and candle materials that glow when backlit with
    thickness-dependent SSS transmission
    - **Acceptance:** thin wax edges appear translucent while thick wax remains opaque
19. **US-2.12.7.2** — I want to verify that physics-driven vertex displacement on rubber materials
    modulates roughness and scattering parameters in real time
    - **Acceptance:** stretched rubber appears lighter and compressed rubber appears darker as
      specified
20. **US-2.12.8.1** — I want to apply a clearcoat layer with independent roughness, IOR, and tint on
    top of a base metallic material
    - **Acceptance:** automotive paint shows the characteristic deep gloss and orange-peel texture
      of real car finishes
21. **US-2.12.8.2** — I want to drive clearcoat wetness parameters at runtime via weather state
    - **Acceptance:** surfaces gradually become wet and reflective during rain and dry out afterward
22. **US-2.12.8.3** — I want to verify max 2 baked layers on mobile, 2 runtime layers on Switch, 4
    on desktop, and unlimited on high-end
    - **Acceptance:** multi-layer materials respect platform-specific layer budgets
23. **US-2.12.9.1** — I want to author materials through the visual shader graph editor with access
    to all rendering inputs and output targets
    - **Acceptance:** I can create procedural wood grain, animated lava, and energy shields without
      engine source modification
24. **US-2.12.9.2** — I want to build material functions as reusable sub-graphs and compose them
    into complex materials
    - **Acceptance:** I can share common operations (triplanar mapping, noise blending) across many
      materials without duplication
25. **US-2.12.9.3** — I want to confirm that mobile caps material graphs at 64 nodes and 4 texture
    reads, Switch at 128/8, and desktop has no limits
    - **Acceptance:** custom materials compile within per-platform instruction budgets
