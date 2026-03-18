# User Stories -- 2.4 Lighting and Materials

## Stories

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-2.4.1.1  | environment artist (P-8) |          |              |
| US-2.4.1.2  | engine developer (P-26)  |          |              |
| US-2.4.2.1  | environment artist (P-8) |          |              |
| US-2.4.2.2  | engine tester (P-27)     |          |              |
| US-2.4.3.1  | environment artist (P-8) |          |              |
| US-2.4.3.2  | engine developer (P-26)  |          |              |
| US-2.4.4.1  | character artist (P-9)   |          |              |
| US-2.4.4.2  | engine tester (P-27)     |          |              |
| US-2.4.5.1  | effects artist (P-12)    |          |              |
| US-2.4.5.2  | engine developer (P-26)  |          |              |
| US-2.4.6.1  | technical artist (P-13)  |          |              |
| US-2.4.6.2  | engine tester (P-27)     |          |              |
| US-2.4.7.1  | environment artist (P-8) |          |              |
| US-2.4.7.2  | technical artist (P-13)  |          |              |
| US-2.4.8.1  | environment artist (P-8) |          |              |
| US-2.4.8.2  | engine tester (P-27)     |          |              |
| US-2.4.9.1  | character artist (P-9)   |          |              |
| US-2.4.9.2  | engine tester (P-27)     |          |              |
| US-2.4.10.1 | environment artist (P-8) |          |              |
| US-2.4.10.2 | engine developer (P-26)  |          |              |
| US-2.4.11.1 | technical artist (P-13)  |          |              |
| US-2.4.11.2 | engine tester (P-27)     |          |              |
| US-2.4.12.1 | player (P-23)            |          |              |
| US-2.4.12.2 | engine tester (P-27)     |          |              |
| US-2.4.13.1 | environment artist (P-8) |          |              |
| US-2.4.13.2 | engine tester (P-27)     |          |              |
| US-2.4.14.1 | environment artist (P-8) |          |              |
| US-2.4.14.2 | engine developer (P-26)  |          |              |
| US-2.4.15.1 | player (P-23)            |          |              |
| US-2.4.15.2 | engine developer (P-26)  |          |              |
| US-2.4.16.1 | environment artist (P-8) |          |              |
| US-2.4.16.2 | engine tester (P-27)     |          |              |
| US-2.4.17.1 | character artist (P-9)   |          |              |
| US-2.4.17.2 | engine tester (P-27)     |          |              |
| US-2.4.18.1 | effects artist (P-12)    |          |              |
| US-2.4.18.2 | engine tester (P-27)     |          |              |
| US-2.4.19.1 | environment artist (P-8) |          |              |
| US-2.4.19.2 | engine developer (P-26)  |          |              |
| US-2.4.20.1 | environment artist (P-8) |          |              |
| US-2.4.20.2 | engine tester (P-27)     |          |              |
| US-2.4.21.1 | environment artist (P-8) |          |              |
| US-2.4.21.2 | engine tester (P-27)     |          |              |
| US-2.4.22.1 | environment artist (P-8) |          |              |
| US-2.4.22.2 | engine tester (P-27)     |          |              |
| US-2.4.23.1 | environment artist (P-8) |          |              |
| US-2.4.23.2 | engine tester (P-27)     |          |              |

1. **US-2.4.1.1** — I want tiled/clustered forward+ light culling to automatically assign lights to
   screen-space tiles
   - **Acceptance:** I can place hundreds of dynamic lights in a marketplace scene without manually
     budgeting per-light cost
2. **US-2.4.1.2** — I want to query per-tile light list lengths and compute pass timing during
   clustered light culling
   - **Acceptance:** I can detect hotspots where too many lights overlap and tune cluster depth
     slice counts per platform
3. **US-2.4.2.1** — I want deferred lighting via G-buffer to accumulate all light contributions in a
   single fullscreen compute pass
   - **Acceptance:** dense interior scenes with high geometric complexity render efficiently without
     per-object light limits
4. **US-2.4.2.2** — I want to measure G-buffer external memory writes on mobile Vulkan and Metal and
   verify that on-chip tile storage is used for subpasses
   - **Acceptance:** deferred lighting does not blow the mobile bandwidth budget
5. **US-2.4.3.1** — I want Cook-Torrance BRDF evaluation with GGX, Smith geometry, and Schlick
   Fresnel in the editor viewport
   - **Acceptance:** material authoring matches the final in-game look without exporting to a
     separate preview tool
6. **US-2.4.3.2** — I want to run a test with 1,000 unique materials using bindless descriptor
   indices and confirm correct texture sampling on Metal, D3D12, and Vulkan
   - **Acceptance:** bindless material access works reliably across all GPU backends
7. **US-2.4.4.1** — I want to add clearcoat, anisotropy, sheen, and SSS layers on top of a standard
   PBR base material
   - **Acceptance:** I can create realistic skin, fabric, and car paint without writing custom
     shaders
8. **US-2.4.4.2** — I want to verify that only 2 simultaneous lobes are active on Switch and that
   mobile falls back to a single extra lobe
   - **Acceptance:** extended materials degrade gracefully without visual corruption on lower-tier
     platforms
9. **US-2.4.5.1** — I want transparent objects (rain sheets, glass windows, water puddles) to render
   in CPU-sorted back-to-front order after all opaque geometry
   - **Acceptance:** overlapping transparencies composite correctly without depth artifacts
10. **US-2.4.5.2** — I want to confirm that each transparent object produces its own draw call and
    is not batched by material
    - **Acceptance:** depth ordering is preserved for correct transparency compositing
11. **US-2.4.6.1** — I want to create parameterized material instances that override scalar, vector,
    and texture values on a parent material
    - **Acceptance:** I can generate thousands of surface variations (damaged, weathered, tinted)
      from a single compiled shader
12. **US-2.4.6.2** — I want to spawn 5,000 entities with unique material instances and confirm that
    per-instance parameter buffers upload correctly
    - **Acceptance:** material instance overhead remains minimal at scale
13. **US-2.4.7.1** — I want to stack material layers (rust over metal, snow over rock, moss over
    stone) with blend masks
    - **Acceptance:** I can create varied surface wear without authoring unique textures for every
      combination
14. **US-2.4.7.2** — I want to measure GPU cost per additional material layer and verify platform
    caps (2 on mobile, 3 on Switch, 4 on desktop)
    - **Acceptance:** I can set art production guidelines that stay within each platform's layer
      budget
15. **US-2.4.8.1** — I want projected decals that write into the DBuffer and wrap across curved mesh
    surfaces with angle-based attenuation
    - **Acceptance:** battle damage marks and graffiti conform to complex architecture without
      stretching
16. **US-2.4.8.2** — I want to project a decal that modifies only normals (not albedo or roughness)
    and verify that unaffected channels remain untouched
    - **Acceptance:** decal channel independence works as specified
17. **US-2.4.9.1** — I want to select dedicated shading models (Marschner hair, cloth fuzz, eye
    refraction, skin SSS) via a material flag
    - **Acceptance:** each character surface uses the physically correct lighting model without
      manual shader switching
18. **US-2.4.9.2** — I want to verify that Marschner hair and eye refraction are disabled on mobile
    and replaced by preintegrated fallbacks
    - **Acceptance:** character rendering remains functional on low-tier hardware without visual
      corruption
19. **US-2.4.10.1** — I want importance-sampled stochastic many-light sampling to distribute a fixed
    ray budget across thousands of area lights
    - **Acceptance:** I can author realistic architectural lighting without shadow map explosion
20. **US-2.4.10.2** — I want to measure temporal accumulation convergence of the stochastic lighting
    denoiser over 16 frames
    - **Acceptance:** I can verify noise settles to imperceptible levels within the first second of
      camera stillness
21. **US-2.4.11.1** — I want to configure 1-4 shadow cascades with per-cascade resolution and split
    blending
    - **Acceptance:** I can tune shadow quality versus cost independently for mobile (1-2 cascades
      at 512), Switch, and desktop (3-4 at 2048+)
22. **US-2.4.11.2** — I want to slowly rotate a directional light through a shadow test scene and
    verify that temporal stabilization prevents texel shimmer at cascade boundaries
    - **Acceptance:** shadow quality is stable during time-of-day transitions
23. **US-2.4.12.1** — I want soft shadows with penumbra size proportional to the light source
    - **Acceptance:** shadows look natural and grounded rather than hard-edged and artificial
24. **US-2.4.12.2** — I want to verify that mobile uses 4-tap PCF, Switch uses PCSS in docked mode,
    and desktop enables RT soft shadows when hardware is available
    - **Acceptance:** shadow quality scales correctly across all platforms
25. **US-2.4.13.1** — I want GTAO with bent normals to darken crevices and contact areas between
    objects
    - **Acceptance:** props feel grounded in the environment without baked AO textures
26. **US-2.4.13.2** — I want to screenshot-compare ambient occlusion output from all three AO tiers
    in a standardized test scene
    - **Acceptance:** I can verify each tier meets its quality target and fallback transitions are
      seamless
27. **US-2.4.14.1** — I want virtual shadow maps with a 16k virtual atlas to provide consistent
    shadow detail based on screen-space coverage
    - **Acceptance:** distant trees and buildings have sharp shadows without manually tweaking
      cascade distances
28. **US-2.4.14.2** — I want to monitor VSM page allocation and eviction when VRAM is constrained to
    8 GB
    - **Acceptance:** the page cache gracefully degrades without visual popping or allocation
      failures
29. **US-2.4.15.1** — I want per-pixel contact shadows at geometry crevices and edges
    - **Acceptance:** objects feel physically connected to surfaces rather than floating
30. **US-2.4.15.2** — I want to measure the per-pixel cost of depth ray marching for contact shadows
    on directional versus point lights
    - **Acceptance:** I can set appropriate step counts per platform (8 on Switch, 16 desktop, 32
      high-end)
31. **US-2.4.16.1** — I want signed distance field shadows to extend shadow range beyond cascaded
    shadow map limits
    - **Acceptance:** distant mountains and buildings cast soft shadows across the landscape at
      minimal GPU cost
32. **US-2.4.16.2** — I want to compare SDF shadows and CSM shadows in the overlap distance range
    and confirm they blend seamlessly
    - **Acceptance:** players do not see a visible transition boundary between shadow techniques
33. **US-2.4.17.1** — I want capsule shadows on skeletal meshes to provide soft indirect shadowing
    in GI-lit regions
    - **Acceptance:** characters cast visible ground shadows even where traditional shadow maps lack
      resolution
34. **US-2.4.17.2** — I want to spawn 10 characters with capsule shadows and verify that desktop
    renders max 4 hero capsule shadows while mobile disables them entirely
    - **Acceptance:** capsule shadow budgets are enforced per platform
35. **US-2.4.18.1** — I want order-independent transparency using moment-based OIT to correctly
    blend fog, water, and particle effects
    - **Acceptance:** overlapping transparent surfaces composite correctly without CPU depth sorting
36. **US-2.4.18.2** — I want to verify that OIT is disabled on Switch and mobile and that the engine
    falls back to sorted transparency (F-2.4.5)
    - **Acceptance:** transparency rendering remains functional on platforms that cannot afford
      moment buffers
37. **US-2.4.19.1** — I want volumetric shadow maps to enable fog and clouds to cast and receive
    shadows
    - **Acceptance:** light shafts and silver-lining effects appear naturally in atmospheric scenes
38. **US-2.4.19.2** — I want to enable volumetric shadow maps with Fourier opacity mapping on
    high-end hardware and confirm colored volumetric shadowing renders correctly
    - **Acceptance:** the premium shadow path produces physically accurate results
39. **US-2.4.20.1** — I want rectangular and spherical area lights with LTC integration to produce
    soft reflections proportional to light source size
    - **Acceptance:** window lighting and softbox illumination look photorealistic without faking it
      with many point lights
40. **US-2.4.20.2** — I want to verify that area lights degrade to point/spot approximations on
    mobile with adjusted falloff
    - **Acceptance:** mobile scenes remain lit correctly without LTC evaluation
41. **US-2.4.21.1** — I want image-based sky lighting that refilters irradiance and specular LUTs
    when the sun position changes
    - **Acceptance:** ambient lighting updates dynamically during time-of-day preview in the editor
42. **US-2.4.21.2** — I want to confirm that mobile uses precomputed LUTs at 64x64 irradiance
    resolution with no runtime refiltering
    - **Acceptance:** sky lighting does not exceed the mobile compute budget
43. **US-2.4.22.1** — I want to load IES photometric data files and apply them to point and spot
    lights
    - **Acceptance:** architectural and cinematic lighting matches real-world luminaire distribution
      patterns
44. **US-2.4.22.2** — I want to verify IES profiles bake to 64-texel 1D textures on mobile and full
    2D 256+ textures on desktop
    - **Acceptance:** profile resolution scales correctly across hardware tiers
45. **US-2.4.23.1** — I want to author a material-driven light function that projects a gobo pattern
    (window blinds, animated flicker) per-pixel in the light's volume
    - **Acceptance:** I can create dynamic shadow patterns without extra shadow-casting geometry
46. **US-2.4.23.2** — I want to confirm that light functions are baked to static textures at import
    on mobile and that only scalar-only functions run on Switch
    - **Acceptance:** light function complexity scales correctly per platform
