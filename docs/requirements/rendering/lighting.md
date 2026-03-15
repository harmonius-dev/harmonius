# R-2.4 -- Lighting and Materials Requirements

## Lighting

### R-2.4.1 Forward+ Lighting (Tiled/Clustered)

The engine **SHALL** perform tiled or clustered light culling via a compute pass that assigns
visible lights to screen-space tiles, where fragment shading evaluates only the lights assigned to
each tile, supporting hundreds of dynamic lights per scene.

- **Derived from:** [F-2.4.1](../../features/rendering/lighting.md)
- **Rationale:** Tiled/clustered culling bounds per-fragment light evaluation cost, enabling
  hundreds of dynamic lights without quadratic fragment-light scaling.
- **Verification:** Render a scene with 500 point lights. Verify each fragment evaluates only lights
  assigned to its tile (inspect via GPU capture). Measure that frame time scales sub-linearly with
  light count compared to brute-force evaluation. Confirm no lighting artifacts at tile boundaries.

### R-2.4.2 Deferred Lighting (G-Buffer)

The engine **SHALL** support deferred lighting via a G-buffer containing albedo-metallic,
normal-roughness, motion vectors, and depth targets, with a fullscreen compute pass accumulating all
light contributions from the G-buffer data.

- **Derived from:** [F-2.4.2](../../features/rendering/lighting.md)
- **Rationale:** Deferred lighting decouples geometry complexity from lighting cost, making it
  efficient for scenes with high geometric complexity and many lights.
- **Verification:** Render a scene with deferred lighting. Inspect the G-buffer targets and verify
  each contains correct data (albedo-metallic, normal-roughness, motion vectors, depth). Verify the
  lighting compute pass produces output matching the forward path within a PSNR threshold of 40 dB
  for the same scene.

### R-2.4.3 PBR Materials (Cook-Torrance BRDF)

The engine **SHALL** implement physically-based rendering using Cook-Torrance microfacet BRDF with
GGX normal distribution, Smith geometry masking, and Schlick Fresnel approximation, accessing
material textures (albedo, normal, metallic-roughness, occlusion, emissive) via bindless indices.

- **Derived from:** [F-2.4.3](../../features/rendering/lighting.md)
- **Rationale:** Cook-Torrance with GGX/Smith/Schlick is the industry-standard PBR model providing
  energy-conserving physically plausible shading.
- **Verification:** Render a material test sphere grid varying roughness (0.0-1.0) and metallic
  (0.0-1.0). Verify energy conservation (no pixel brighter than incoming light). Compare output
  against a reference path tracer and verify PSNR above 35 dB. Confirm all material textures are
  accessed via bindless indices (no descriptor set switches per material).

### R-2.4.4 Extended BSDF Materials

The engine **SHALL** support an extended material model adding subsurface scattering, clearcoat,
anisotropy, and sheen layers on top of the standard PBR base, covering skin, fabric, car paint, and
hair rendering.

- **Derived from:** [F-2.4.4](../../features/rendering/lighting.md)
- **Rationale:** Real-world materials exhibit layered optical behavior (clearcoat on car paint, fuzz
  on fabric) that a single-layer BRDF cannot reproduce.
- **Verification:** Render test spheres with each extended layer enabled independently (clearcoat,
  anisotropy, sheen, SSS). Verify each layer produces visually distinct and physically plausible
  results compared to the base PBR material. Verify that enabling multiple layers simultaneously
  produces correct composited output.

### R-2.4.5 Transparent Objects

The engine **SHALL** render transparent geometry using CPU-side back-to-front distance sorting,
drawing each transparent object individually (one draw call per object) after all opaque geometry,
participating in the standard lighting pipeline without material batching.

- **Derived from:** [F-2.4.5](../../features/rendering/lighting.md)
- **Rationale:** Per-object sorted transparency ensures correct depth ordering for translucent
  surfaces at the cost of individual draw calls.
- **Verification:** Render 20 overlapping transparent planes at varying depths. Verify correct
  back-to-front compositing order with no sorting artifacts. Verify each transparent object
  generates exactly one draw call. Confirm transparent objects receive lighting from all active
  light sources.

### R-2.4.6 Material Instances

The engine **SHALL** support parameterized material instances that override scalar, vector, and
texture values of a parent material without shader recompilation, sharing the compiled shader and
uploading only a per-instance parameter buffer.

- **Derived from:** [F-2.4.6](../../features/rendering/lighting.md)
- **Rationale:** Material instances enable thousands of visual variants from a single compiled
  shader, reducing pipeline state object count and compilation cost.
- **Verification:** Create 1,000 material instances from a single parent material with varying color
  and roughness parameters. Verify all instances share the same compiled pipeline state object.
  Verify each instance renders with its unique parameter values. Measure that instance creation does
  not trigger any shader compilation.

### R-2.4.7 Material Layers and Blending

The engine **SHALL** support multi-layer material compositing where independent material layers
(e.g., rust over metal, snow over rock) are stacked with blend masks and evaluated per pixel as
self-contained material graphs.

- **Derived from:** [F-2.4.7](../../features/rendering/lighting.md)
- **Rationale:** Layered materials enable complex surface appearances (weathering, damage,
  environmental accumulation) from reusable atomic material graphs.
- **Verification:** Create a two-layer material (rust over metal) with a procedural blend mask.
  Verify the output shows rust in masked areas and clean metal elsewhere with correct per-pixel
  blending at boundaries. Verify each layer's lighting is evaluated independently before
  compositing.

### R-2.4.8 Decal Rendering

The engine **SHALL** support projected decals rendered as oriented boxes writing into a deferred
decal buffer (DBuffer) before lighting, and mesh-based decals wrapping geometry with proper
normal-mapped contact on curved surfaces.

- **Derived from:** [F-2.4.8](../../features/rendering/lighting.md)
- **Rationale:** Decals add surface detail (bullet holes, footprints, signage) without modifying the
  underlying mesh or material.
- **Verification:** Place a projected decal on a flat surface and verify it appears in the DBuffer
  with correct orientation and bounds. Place a mesh decal on a curved surface and verify the normal
  map follows the surface curvature. Verify decals receive correct lighting from all active lights
  after DBuffer compositing.

### R-2.4.9 Shading Model Variants

The engine **SHALL** support per-pixel shading model selection via material flag for the following
dedicated models: hair (anisotropic Marschner), cloth (fuzz layer with fiber scattering), eye
(cornea refraction with iris parallax), thin translucent (single-pass tinted glass), two-sided
foliage (subsurface transmission), preintegrated skin (low-cost SSS), and single-layer water
(absorption, scattering, refraction).

- **Derived from:** [F-2.4.9](../../features/rendering/lighting.md)
- **Rationale:** Specialized shading models produce physically plausible results for materials whose
  optical behavior diverges significantly from the standard metallic/dielectric BRDF.
- **Verification:** Render a test object with each shading model variant. Verify each produces
  distinct, physically plausible visual characteristics (e.g., anisotropic highlights for hair,
  subsurface transmission for foliage). Verify the shading model selection is per-pixel (a single
  mesh can use different models on different faces via material assignment).

### R-2.4.10 Stochastic Many-Light Sampling

The engine **SHALL** provide importance-sampled direct lighting for scenes with thousands of
shadowed area lights, distributing a fixed ray budget per pixel across light sources proportional to
estimated contribution (luminance and solid angle) via a stochastic pass with temporal accumulation
denoising.

- **Derived from:** [F-2.4.10](../../features/rendering/lighting.md)
- **Rationale:** Stochastic sampling enables shadowed evaluation of thousands of lights within a
  fixed per-pixel cost budget, replacing per-light shadow map evaluation.
- **Verification:** Render a scene with 2,000 area lights. Verify all lights contribute to the final
  image after temporal convergence. Measure that per-frame cost remains constant regardless of light
  count (within 10% variance). Compare against a brute-force reference and verify converged output
  matches within PSNR 35 dB.

## Shadows

### R-2.4.11 Cascaded Shadow Maps

The engine **SHALL** support multi-cascade shadow mapping for directional lights with
logarithmic/linear split blending, configurable cascade count (1-4), per-cascade resolution, depth
bias, and temporal stabilization.

- **Derived from:** [F-2.4.11](../../features/rendering/lighting.md)
- **Rationale:** Cascaded shadow maps distribute shadow resolution across the view frustum,
  providing sharp nearby shadows and acceptable distant shadows for directional lights.
- **Verification:** Configure 4 cascades with logarithmic splits. Verify each cascade covers its
  expected depth range. Verify no shadow seams at cascade boundaries. Measure temporal stability by
  rotating the camera and verifying shadow edge movement is sub-pixel between frames.

### R-2.4.12 Soft Shadows (PCF / PCSS / RT)

The engine **SHALL** provide tiered soft shadow implementations: PCF (percentage closer filtering)
as the baseline, PCSS (percentage closer soft shadows) for light-size-aware penumbra, and ray-traced
soft shadows on RT-capable hardware.

- **Derived from:** [F-2.4.12](../../features/rendering/lighting.md)
- **Rationale:** Tiered shadow quality enables scalable visual fidelity from low-end (PCF) to
  high-end (RT) hardware with consistent shadow behavior.
- **Verification:** Render shadows with each tier. Verify PCF produces fixed-width soft edges, PCSS
  produces penumbra width proportional to light size and blocker distance, and RT shadows match a
  reference for area light penumbra shape. Verify runtime tier switching produces no artifacts.

### R-2.4.13 Ambient Occlusion (SSAO / GTAO / RT AO)

The engine **SHALL** provide tiered ambient occlusion: SSAO at half resolution as the baseline, GTAO
(ground truth AO) with bent normals at mid tier, and ray-traced AO at high tier.

- **Derived from:** [F-2.4.13](../../features/rendering/lighting.md)
- **Rationale:** Tiered AO provides scalable contact darkening from low-cost screen-space
  approximations to ground-truth ray-traced results.
- **Verification:** Render a scene with each AO tier. Verify SSAO runs at half resolution and
  produces darkening in concavities. Verify GTAO produces bent normal output in addition to
  occlusion. Verify RT AO matches a path-traced reference within PSNR 35 dB. Confirm runtime tier
  switching with no visual discontinuity.

### R-2.4.14 Virtual Shadow Maps

The engine **SHALL** provide clipmap-based virtual shadow mapping with on-demand page allocation
based on screen-space coverage, providing consistent shadow detail equivalent to a 16k x 16k virtual
shadow atlas, paired with the meshlet pipeline for efficient geometry rendering into shadow pages.

- **Derived from:** [F-2.4.14](../../features/rendering/lighting.md)
- **Rationale:** Virtual shadow maps provide uniformly high shadow resolution across the entire view
  without the per-cascade resolution compromise of CSM.
- **Verification:** Render a large scene with a directional light. Verify shadow texel density is
  approximately constant in screen space across near and far objects. Verify only pages with
  screen-space coverage are allocated (inspect page residency). Measure VRAM consumption is below
  the equivalent full 16k x 16k atlas.

### R-2.4.15 Contact Shadows

The engine **SHALL** evaluate per-pixel screen-space depth ray-marching per light, tracing short
rays from each pixel along the light direction through the depth buffer to detect fine-scale shadow
contact at geometry edges and crevices.

- **Derived from:** [F-2.4.15](../../features/rendering/lighting.md)
- **Rationale:** Contact shadows add fine-scale shadow detail at geometry contact points that shadow
  maps lack the resolution to capture.
- **Verification:** Render an object resting on a flat surface with a directional light. Verify
  contact shadow darkening appears at the base contact line where the object meets the surface.
  Verify the contact shadow ray length is configurable and that increasing it extends the shadow
  effect range.

### R-2.4.16 Distance Field Shadows

The engine **SHALL** provide long-distance shadows using mesh signed distance fields, casting
cone-traced shadows through the SDF volume with natural penumbra falloff extending shadow range
beyond cascaded shadow map limits.

- **Derived from:** [F-2.4.16](../../features/rendering/lighting.md)
- **Rationale:** SDF shadows provide soft shadowing at distances where shadow map resolution is
  insufficient, at a fraction of the cost of extending CSM range.
- **Verification:** Place a large object beyond the CSM far cascade. Verify SDF shadows are visible
  on the ground plane with soft penumbra. Verify penumbra width increases with distance from the
  shadow caster. Measure that SDF shadow cost is less than 20% of extending CSM to the same range.

### R-2.4.17 Capsule Shadows

The engine **SHALL** provide lightweight soft shadow approximation for skeletal meshes using capsule
physics representations, cone-tracing each capsule through the SDF to produce indirect area
shadowing in GI-lit regions.

- **Derived from:** [F-2.4.17](../../features/rendering/lighting.md)
- **Rationale:** Capsule shadows provide approximate indirect shadowing for animated characters at
  minimal cost compared to per-frame shadow map updates.
- **Verification:** Animate a skeletal character and verify capsule shadows update with the skeleton
  pose. Verify shadow softness is proportional to capsule radius. Confirm capsule shadows appear
  only in GI-lit regions (not under direct shadow maps). Measure per-character capsule shadow cost
  is below 0.1ms.

### R-2.4.18 Order-Independent Transparency (OIT)

The engine **SHALL** provide moment-based order-independent transparency (MBOIT) that correctly
blends transparent geometry, fog, water, and particle effects without requiring CPU-side depth
sorting, accumulating per-pixel transmittance moments and resolving to produce correct compositing
order.

- **Derived from:** [F-2.4.18](../../features/rendering/lighting.md)
- **Rationale:** OIT eliminates depth-sorting artifacts for overlapping transparent surfaces and
  enables correct blending of volumetric effects with transparent geometry.
- **Verification:** Render 10 overlapping transparent surfaces in random draw order. Verify the
  output matches a reference image sorted in correct depth order (within PSNR 30 dB). Verify
  volumetric fog blends correctly with transparent surfaces in the same region.

### R-2.4.19 Volumetric Shadow Maps

The engine **SHALL** provide shadow maps dedicated to participating media (fog, clouds, volumetric
effects) that enable volumetric elements to cast and receive shadows from any light type, producing
colored volumetric shadowing and light shaft occlusion through Fourier opacity mapping.

- **Derived from:** [F-2.4.19](../../features/rendering/lighting.md)
- **Rationale:** Volumetric shadow maps enable physically correct light attenuation through
  participating media, producing effects like cloud shadows and colored fog absorption.
- **Verification:** Render a volumetric fog volume between a light and a surface. Verify the fog
  casts a visible shadow on the surface. Verify colored fog produces tinted shadow attenuation.
  Verify light shafts are correctly occluded by opaque geometry within the volume.

### R-2.4.20 Area Lights (Rect/Sphere)

The engine **SHALL** evaluate rectangular and spherical area lights using linearly-transformed
cosine (LTC) integration, producing soft reflections and natural falloff proportional to source
size.

- **Derived from:** [F-2.4.20](../../features/rendering/lighting.md)
- **Rationale:** Area lights produce physically correct soft illumination and specular reflections
  that point lights cannot approximate, essential for architectural and cinematic scenes.
- **Verification:** Render a glossy surface lit by a rectangular area light. Verify the specular
  reflection shape matches the rectangular source. Vary the area light size and verify reflection
  softness scales proportionally. Compare against a path-traced reference and verify PSNR above 35
  dB.

### R-2.4.21 Sky Light (IBL)

The engine **SHALL** provide image-based ambient lighting from the sky atmosphere or a provided
cubemap, pre-filtered into diffuse irradiance and split-sum specular lookup tables for real-time
evaluation.

- **Derived from:** [F-2.4.21](../../features/rendering/lighting.md)
- **Rationale:** IBL captures complex ambient lighting from the environment, providing realistic
  diffuse fill and specular reflections from the sky.
- **Verification:** Render a metallic sphere under a sky light. Verify the sphere reflects the
  environment cubemap. Vary roughness and verify reflections blur proportionally. Verify diffuse
  irradiance is consistent with the dominant sky color. Confirm pre-filtering is performed at
  initialization (not per-frame).

### R-2.4.22 IES Light Profiles

The engine **SHALL** support photometric light distribution profiles loaded from IES data files,
defining intensity as a function of angle and applicable to point and spot lights.

- **Derived from:** [F-2.4.22](../../features/rendering/lighting.md)
- **Rationale:** IES profiles enable physically accurate architectural and cinematic lighting
  matching real-world luminaire characteristics.
- **Verification:** Load an IES profile for a downlight fixture. Render a point light with the
  profile applied and verify the angular intensity distribution matches the IES data (sample at 8+
  angles and verify within 5% of the profile values). Verify the profile modulates the light
  independently of the base intensity and color.

### R-2.4.23 Light Functions

The engine **SHALL** support material-driven intensity and color modulation applied to any light
type, where a lightweight material graph produces a scalar or color mask evaluated per-pixel in the
light's influence volume.

- **Derived from:** [F-2.4.23](../../features/rendering/lighting.md)
- **Rationale:** Light functions enable dynamic projected patterns (gobos, window blinds, animated
  flicker) without dedicated shadow maps per effect.
- **Verification:** Assign a gobo pattern light function to a spot light. Verify the projected
  pattern appears on the illuminated surface matching the function output. Animate the function over
  60 frames and verify the pattern updates each frame. Verify light functions compose correctly with
  shadow maps (pattern visible only in lit regions).

## Non-Functional Requirements

### NFR-2.4.1 Light Count Scalability

The forward+ tiled/clustered lighting system **SHALL** support at least 500 dynamic lights per scene
with sub-linear frame time scaling relative to light count.

- **Rationale:** Many-light scenarios (cities, interiors with many fixtures) are common; the tiling
  system must prevent quadratic fragment-light cost.
- **Verification:** Render scenes with 100, 250, and 500 lights. Verify frame time scales
  sub-linearly and the 500-light scene completes within the frame budget.

### NFR-2.4.2 Shadow Map Memory Budget

The combined shadow map atlas (cascaded, spot, point) **SHALL** consume no more than 256 MB of VRAM
at default quality settings, with virtual shadow maps allocating pages on demand within this budget.

- **Rationale:** Shadow maps are a major VRAM consumer; budgeting prevents memory exhaustion in
  scenes with many shadow-casting lights.
- **Verification:** Profile VRAM usage of shadow maps in a scene with 4 CSM cascades and 20
  additional shadow-casting lights. Verify total shadow map VRAM is below 256 MB.

### NFR-2.4.3 PBR Material Evaluation Cost

The Cook-Torrance BRDF evaluation (GGX + Smith + Schlick) **SHALL** complete in under 0.1 ms per
million fragments at 1080p on target hardware.

- **Rationale:** Material evaluation runs per visible fragment and must not dominate frame time.
- **Verification:** Profile the BRDF evaluation pass on a 1080p scene with 2 million visible
  fragments and verify GPU time is below 0.2 ms.
