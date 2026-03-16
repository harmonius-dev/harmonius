# User Stories -- 2.4 Lighting and Materials

## US-2.4.1.1 Place Hundreds of Dynamic Lights Without Manual Culling

**As a** environment artist (P-8), **I want** tiled/clustered forward+ light culling to
automatically assign lights to screen-space tiles, **so that** I can place hundreds of dynamic
lights in a marketplace scene without manually budgeting per-light cost.

## US-2.4.1.2 Profile Clustered Light Assignment Overhead

**As an** engine developer (P-26), **I want** to query per-tile light list lengths and compute pass
timing during clustered light culling, **so that** I can detect hotspots where too many lights
overlap and tune cluster depth slice counts per platform.

## US-2.4.2.1 Light Complex Scenes With Many Overlapping Geometry Layers

**As a** environment artist (P-8), **I want** deferred lighting via G-buffer to accumulate all light
contributions in a single fullscreen compute pass, **so that** dense interior scenes with high
geometric complexity render efficiently without per-object light limits.

## US-2.4.2.2 Validate G-Buffer Bandwidth on Tile-Based GPUs

**As an** engine tester (P-27), **I want** to measure G-buffer external memory writes on mobile
Vulkan and Metal and verify that on-chip tile storage is used for subpasses, **so that** deferred
lighting does not blow the mobile bandwidth budget.

## US-2.4.3.1 Preview PBR Materials With Accurate BRDF in the Viewport

**As a** environment artist (P-8), **I want** Cook-Torrance BRDF evaluation with GGX, Smith
geometry, and Schlick Fresnel in the editor viewport, **so that** material authoring matches the
final in-game look without exporting to a separate preview tool.

## US-2.4.3.2 Verify Bindless Texture Access Across Backends

**As an** engine developer (P-26), **I want** to run a test with 1,000 unique materials using
bindless descriptor indices and confirm correct texture sampling on Metal, D3D12, and Vulkan,
**so that** bindless material access works reliably across all GPU backends.

## US-2.4.4.1 Create Skin and Fabric Materials With Extended BSDF Lobes

**As a** character artist (P-9), **I want** to add clearcoat, anisotropy, sheen, and SSS layers on
top of a standard PBR base material, **so that** I can create realistic skin, fabric, and car paint
without writing custom shaders.

## US-2.4.4.2 Test Extended BSDF Lobe Fallback on Switch

**As an** engine tester (P-27), **I want** to verify that only 2 simultaneous lobes are active on
Switch and that mobile falls back to a single extra lobe, **so that** extended materials degrade
gracefully without visual corruption on lower-tier platforms.

## US-2.4.5.1 Sort Transparent Objects Correctly in a Rainy Scene

**As a** effects artist (P-12), **I want** transparent objects (rain sheets, glass windows, water
puddles) to render in CPU-sorted back-to-front order after all opaque geometry, **so that**
overlapping transparencies composite correctly without depth artifacts.

## US-2.4.5.2 Verify Per-Object Transparent Draw Call Isolation

**As an** engine developer (P-26), **I want** to confirm that each transparent object produces its
own draw call and is not batched by material, **so that** depth ordering is preserved for correct
transparency compositing.

## US-2.4.6.1 Create Thousands of Material Variants Without Recompilation

**As a** technical artist (P-13), **I want** to create parameterized material instances that
override scalar, vector, and texture values on a parent material, **so that** I can generate
thousands of surface variations (damaged, weathered, tinted) from a single compiled shader.

## US-2.4.6.2 Verify Material Instance Parameter Buffer Upload

**As an** engine tester (P-27), **I want** to spawn 5,000 entities with unique material instances
and confirm that per-instance parameter buffers upload correctly, **so that** material instance
overhead remains minimal at scale.

## US-2.4.7.1 Stack Weathering Layers on Environment Props

**As a** environment artist (P-8), **I want** to stack material layers (rust over metal, snow over
rock, moss over stone) with blend masks, **so that** I can create varied surface wear without
authoring unique textures for every combination.

## US-2.4.7.2 Profile Multi-Layer Material Cost Per Platform

**As a** technical artist (P-13), **I want** to measure GPU cost per additional material layer and
verify platform caps (2 on mobile, 3 on Switch, 4 on desktop), **so that** I can set art production
guidelines that stay within each platform's layer budget.

## US-2.4.8.1 Place Projected Decals on Curved Fortress Walls

**As a** environment artist (P-8), **I want** projected decals that write into the DBuffer and wrap
across curved mesh surfaces with angle-based attenuation, **so that** battle damage marks and
graffiti conform to complex architecture without stretching.

## US-2.4.8.2 Validate DBuffer Decal Channel Independence

**As an** engine tester (P-27), **I want** to project a decal that modifies only normals (not albedo
or roughness) and verify that unaffected channels remain untouched, **so that** decal channel
independence works as specified.

## US-2.4.9.1 Apply Specialized Shading Models Per Material Type

**As a** character artist (P-9), **I want** to select dedicated shading models (Marschner hair,
cloth fuzz, eye refraction, skin SSS) via a material flag, **so that** each character surface uses
the physically correct lighting model without manual shader switching.

## US-2.4.9.2 Test Shading Model Variant Fallbacks on Mobile

**As an** engine tester (P-27), **I want** to verify that Marschner hair and eye refraction are
disabled on mobile and replaced by preintegrated fallbacks, **so that** character rendering remains
functional on low-tier hardware without visual corruption.

## US-2.4.10.1 Light a Concert Hall With Thousands of Shadowed Area Lights

**As a** environment artist (P-8), **I want** importance-sampled stochastic many-light sampling to
distribute a fixed ray budget across thousands of area lights, **so that** I can author realistic
architectural lighting without shadow map explosion.

## US-2.4.10.2 Profile Stochastic Light Sampling Convergence

**As an** engine developer (P-26), **I want** to measure temporal accumulation convergence of the
stochastic lighting denoiser over 16 frames, **so that** I can verify noise settles to imperceptible
levels within the first second of camera stillness.

## US-2.4.11.1 Adjust Shadow Cascade Count and Resolution Per Platform

**As a** technical artist (P-13), **I want** to configure 1-4 shadow cascades with per-cascade
resolution and split blending, **so that** I can tune shadow quality versus cost independently for
mobile (1-2 cascades at 512), Switch, and desktop (3-4 at 2048+).

## US-2.4.11.2 Verify Cascade Temporal Stabilization Eliminates Shimmer

**As an** engine tester (P-27), **I want** to slowly rotate a directional light through a shadow
test scene and verify that temporal stabilization prevents texel shimmer at cascade boundaries,
**so that** shadow quality is stable during time-of-day transitions.

## US-2.4.12.1 See Realistic Soft Penumbra From Area Light Sources

**As a** player (P-23), **I want** soft shadows with penumbra size proportional to the light source,
**so that** shadows look natural and grounded rather than hard-edged and artificial.

## US-2.4.12.2 Validate PCF/PCSS/RT Shadow Tier Selection

**As an** engine tester (P-27), **I want** to verify that mobile uses 4-tap PCF, Switch uses PCSS in
docked mode, and desktop enables RT soft shadows when hardware is available, **so that** shadow
quality scales correctly across all platforms.

## US-2.4.13.1 Add Ambient Occlusion to Ground-Contact Areas

**As a** environment artist (P-8), **I want** GTAO with bent normals to darken crevices and contact
areas between objects, **so that** props feel grounded in the environment without baked AO textures.

## US-2.4.13.2 Compare AO Quality Across SSAO, GTAO, and RT AO Tiers

**As an** engine tester (P-27), **I want** to screenshot-compare ambient occlusion output from all
three AO tiers in a standardized test scene, **so that** I can verify each tier meets its quality
target and fallback transitions are seamless.

## US-2.4.14.1 Maintain Consistent Shadow Detail Across a Large Open World

**As a** environment artist (P-8), **I want** virtual shadow maps with a 16k virtual atlas to
provide consistent shadow detail based on screen-space coverage, **so that** distant trees and
buildings have sharp shadows without manually tweaking cascade distances.

## US-2.4.14.2 Verify VSM Page Allocation Under VRAM Pressure

**As an** engine developer (P-26), **I want** to monitor VSM page allocation and eviction when VRAM
is constrained to 8 GB, **so that** the page cache gracefully degrades without visual popping or
allocation failures.

## US-2.4.15.1 See Fine Shadow Contact at Geometry Edges

**As a** player (P-23), **I want** per-pixel contact shadows at geometry crevices and edges,
**so that** objects feel physically connected to surfaces rather than floating.

## US-2.4.15.2 Profile Contact Shadow Ray March Cost Per Light Type

**As an** engine developer (P-26), **I want** to measure the per-pixel cost of depth ray marching
for contact shadows on directional versus point lights, **so that** I can set appropriate step
counts per platform (8 on Switch, 16 desktop, 32 high-end).

## US-2.4.16.1 Cast Long-Distance Shadows Using SDF Volumes

**As a** environment artist (P-8), **I want** signed distance field shadows to extend shadow range
beyond cascaded shadow map limits, **so that** distant mountains and buildings cast soft shadows
across the landscape at minimal GPU cost.

## US-2.4.16.2 Validate SDF Shadow Consistency With CSM at Overlap Range

**As an** engine tester (P-27), **I want** to compare SDF shadows and CSM shadows in the overlap
distance range and confirm they blend seamlessly, **so that** players do not see a visible
transition boundary between shadow techniques.

## US-2.4.17.1 Add Soft Body Shadows to Animated Characters

**As a** character artist (P-9), **I want** capsule shadows on skeletal meshes to provide soft
indirect shadowing in GI-lit regions, **so that** characters cast visible ground shadows even where
traditional shadow maps lack resolution.

## US-2.4.17.2 Verify Capsule Shadow Limits Per Platform

**As an** engine tester (P-27), **I want** to spawn 10 characters with capsule shadows and verify
that desktop renders max 4 hero capsule shadows while mobile disables them entirely, **so that**
capsule shadow budgets are enforced per platform.

## US-2.4.18.1 Blend Transparent Surfaces Without Sorting Artifacts

**As a** effects artist (P-12), **I want** order-independent transparency using moment-based OIT to
correctly blend fog, water, and particle effects, **so that** overlapping transparent surfaces
composite correctly without CPU depth sorting.

## US-2.4.18.2 Validate OIT Fallback to Sorted Transparency on Switch

**As an** engine tester (P-27), **I want** to verify that OIT is disabled on Switch and mobile and
that the engine falls back to sorted transparency (F-2.4.5), **so that** transparency rendering
remains functional on platforms that cannot afford moment buffers.

## US-2.4.19.1 Cast Volumetric Shadows Through Clouds and Fog

**As a** environment artist (P-8), **I want** volumetric shadow maps to enable fog and clouds to
cast and receive shadows, **so that** light shafts and silver-lining effects appear naturally in
atmospheric scenes.

## US-2.4.19.2 Verify Fourier Opacity Mapping on High-End Tier

**As an** engine developer (P-26), **I want** to enable volumetric shadow maps with Fourier opacity
mapping on high-end hardware and confirm colored volumetric shadowing renders correctly, **so that**
the premium shadow path produces physically accurate results.

## US-2.4.20.1 Place Rectangular Area Lights for Realistic Window Illumination

**As a** environment artist (P-8), **I want** rectangular and spherical area lights with LTC
integration to produce soft reflections proportional to light source size, **so that** window
lighting and softbox illumination look photorealistic without faking it with many point lights.

## US-2.4.20.2 Validate Area Light Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that area lights degrade to point/spot
approximations on mobile with adjusted falloff, **so that** mobile scenes remain lit correctly
without LTC evaluation.

## US-2.4.21.1 Preview Sky-Driven Ambient Lighting During Time of Day

**As a** environment artist (P-8), **I want** image-based sky lighting that refilters irradiance and
specular LUTs when the sun position changes, **so that** ambient lighting updates dynamically during
time-of-day preview in the editor.

## US-2.4.21.2 Verify IBL LUT Precomputation on Mobile

**As an** engine tester (P-27), **I want** to confirm that mobile uses precomputed LUTs at 64x64
irradiance resolution with no runtime refiltering, **so that** sky lighting does not exceed the
mobile compute budget.

## US-2.4.22.1 Apply IES Light Profiles for Architectural Accuracy

**As a** environment artist (P-8), **I want** to load IES photometric data files and apply them to
point and spot lights, **so that** architectural and cinematic lighting matches real-world luminaire
distribution patterns.

## US-2.4.22.2 Test IES Profile Texture Resolution Per Platform

**As an** engine tester (P-27), **I want** to verify IES profiles bake to 64-texel 1D textures on
mobile and full 2D 256+ textures on desktop, **so that** profile resolution scales correctly across
hardware tiers.

## US-2.4.23.1 Animate Window Blind Shadows With Light Functions

**As a** environment artist (P-8), **I want** to author a material-driven light function that
projects a gobo pattern (window blinds, animated flicker) per-pixel in the light's volume,
**so that** I can create dynamic shadow patterns without extra shadow-casting geometry.

## US-2.4.23.2 Verify Light Function Baking on Mobile

**As an** engine tester (P-27), **I want** to confirm that light functions are baked to static
textures at import on mobile and that only scalar-only functions run on Switch, **so that** light
function complexity scales correctly per platform.
