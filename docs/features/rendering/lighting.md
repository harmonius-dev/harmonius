# 2.4 — Lighting and Materials

## Features

| ID       | Feature                             |
|----------|-------------------------------------|
| F-2.4.1  | Forward+ Lighting (Tiled/Clustered) |
| F-2.4.2  | Deferred Lighting (G-Buffer)        |
| F-2.4.3  | PBR Materials (Cook-Torrance BRDF)  |
| F-2.4.4  | Extended BSDF Materials             |
| F-2.4.5  | Transparent Objects                 |
| F-2.4.6  | Material Instances                  |
| F-2.4.7  | Material Layers and Blending        |
| F-2.4.8  | Decal Rendering                     |
| F-2.4.9  | Shading Model Variants              |
| F-2.4.10 | Stochastic Many-Light Sampling      |

1. **F-2.4.1** — Tiled or clustered light culling via a compute pass that assigns visible lights to
   screen-space tiles. Fragment shading evaluates only the lights relevant to each tile, enabling
   hundreds of dynamic lights.
   - **Deps:** F-2.3.1
   - **Platform:** Mobile: preferred path; tiled forward fits tile-based GPU architecture naturally.
     Cluster depth slices reduced to 8 (vs 24 desktop). Switch: full clustered forward.
     Desktop/High-end: full quality, hundreds of lights.
2. **F-2.4.2** — G-buffer rendering with albedo-metallic, normal-roughness, motion vectors, and
   depth targets. A fullscreen compute pass accumulates all light contributions from the G-buffer.
   Suitable for scenes with high geometric complexity and many lights.
   - **Deps:** F-2.3.1
   - **Platform:** Mobile: disabled by default; G-buffer bandwidth is prohibitive on tile-based
     GPUs. If used, requires on-chip tile storage via subpass/local-read to avoid external memory
     writes. Switch: thin G-buffer (2 targets) with on-chip resolve. Desktop/High-end: full 4-target
     G-buffer.
3. **F-2.4.3** — Physically-based rendering using Cook-Torrance microfacet BRDF with GGX normal
   distribution, Smith geometry masking, and Schlick Fresnel approximation. Material textures
   (albedo, normal, metallic-roughness, occlusion, emissive) are accessed via bindless indices.
   - **Platform:** Mobile: max 4 texture samples per material; half-precision BRDF evaluation; ASTC
     textures. Switch: max 8 samples; full-precision BRDF. Desktop: full quality, BC7 textures,
     bindless heap. High-end: 8K texture support.
4. **F-2.4.4** — Extended material model adding subsurface scattering, clearcoat, anisotropy, and
   sheen layers on top of the standard PBR base. Covers skin, fabric, car paint, and hair.
   - **Deps:** F-2.4.3
   - **Platform:** Mobile: max 1 extra lobe (clearcoat or sheen, not both); SSS falls back to
     preintegrated LUT. Switch: max 2 lobes active simultaneously. Desktop: all lobes available.
     High-end: full multi-lobe with RT SSS integration.
5. **F-2.4.5** — Correct rendering of transparent geometry using CPU-side back-to-front distance
   sorting before upload. Transparent objects are sorted per-object and drawn individually -- they
   are not batched by material. Each transparent object produces its own draw call to preserve
   correct depth ordering. Transparent objects participate in the standard lighting pipeline but are
   rendered after all opaque geometry.
   - **Deps:** F-2.4.1
   - **Platform:** Mobile: max 16 transparent objects per view; simplified single-light shading per
     transparent object. Switch: max 32 transparent objects. Desktop/High-end: no limit; full
     multi-light evaluation per transparent draw.
6. **F-2.4.6** — Parameterized overrides of a parent material allowing scalar, vector, and texture
   substitutions without recompilation. Instances share the compiled shader and only upload a
   per-instance parameter buffer, enabling thousands of material variants at minimal cost.
   - **Deps:** F-2.4.3
   - **Platform:** All platforms: full quality, no degradation. Instance parameter buffers are
     lightweight across all tiers.
7. **F-2.4.7** — Multi-layer material compositing where independent material layers (e.g., rust over
   metal, snow over rock) are stacked with blend masks. Each layer is a self-contained material
   graph evaluated and composited at the pixel level.
   - **Deps:** F-2.4.3
   - **Platform:** Mobile: max 2 layers; blend mask baked to single texture at import. Switch: max 3
     layers. Desktop: max 4 layers with runtime blend masks. High-end: unlimited layers with
     per-pixel evaluation.
8. **F-2.4.8** — Projected and mesh-based decals. Projected decals render as oriented boxes that
   write into a deferred decal buffer (DBuffer) before lighting or directly into the G-buffer. Mesh
   decals wrap geometry for proper normal-mapped contact with curved surfaces.
   - **Deps:** F-2.4.2
   - **Platform:** Mobile: forward-rendered decals only (no DBuffer); max 8 visible decals. Switch:
     DBuffer with 2 channels; max 16 decals. Desktop: full DBuffer with normal and roughness.
     High-end: unlimited decals with full DBuffer.
9. **F-2.4.9** — Dedicated per-pixel shading models beyond standard PBR, selected via material flag:
   hair (anisotropic Marschner), cloth (fuzz layer with fiber scattering), eye (cornea refraction
   with iris parallax), thin translucent (single-pass tinted glass), two-sided foliage (subsurface
   transmission), preintegrated skin (low-cost SSS), and single-layer water (absorption, scattering,
   refraction).
   - **Deps:** F-2.4.3
   - **Platform:** Mobile: simplified variants only -- preintegrated skin, single-layer water, basic
     cloth; eye refraction and Marschner hair disabled. Switch: all variants except Marschner hair
     (uses card-based fallback). Desktop/ High-end: all shading model variants available.
10. **F-2.4.10** — Importance-sampled direct lighting for scenes with thousands of shadowed area
    lights. A fixed ray budget per pixel is distributed across light sources proportional to
    estimated contribution (luminance and solid angle), replacing per-light shadow map evaluation
    with a unified stochastic pass and temporal accumulation denoiser.
    - **Deps:** F-2.4.1
    - **Platform:** Mobile: disabled; too expensive for mobile bandwidth and ALU budgets. Switch:
      disabled. Desktop: enabled with reduced ray budget (1-2 spp). High-end: full ray budget (4+
      spp) with temporal denoiser.

## Shadows

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-2.4.11 | Cascaded Shadow Maps                    |
| F-2.4.12 | Soft Shadows (PCF / PCSS / RT)          |
| F-2.4.13 | Ambient Occlusion (SSAO / GTAO / RT AO) |
| F-2.4.14 | Virtual Shadow Maps                     |
| F-2.4.15 | Contact Shadows                         |
| F-2.4.16 | Distance Field Shadows                  |
| F-2.4.17 | Capsule Shadows                         |
| F-2.4.18 | Order-Independent Transparency (OIT)    |
| F-2.4.19 | Volumetric Shadow Maps                  |
| F-2.4.20 | Area Lights (Rect/Sphere)               |
| F-2.4.21 | Sky Light (IBL)                         |
| F-2.4.22 | IES Light Profiles                      |
| F-2.4.23 | Light Functions                         |

1. **F-2.4.11** — Multi-cascade shadow mapping for directional lights with logarithmic/linear split
   blending. Supports 1-4 cascades with configurable resolution, depth bias, and temporal
   stabilization.
   - **Deps:** F-2.3.1
   - **Platform:** Mobile: 1-2 cascades at 512x512 each; reduced shadow distance. Switch: 2 cascades
     at 1024x1024 handheld, 3 cascades docked. Desktop: 3-4 cascades at 2048x2048. High-end: 4
     cascades at 4096x4096 with extended range.
2. **F-2.4.12** — Tiered soft shadow implementations: PCF (percentage closer filtering) as the
   baseline, PCSS (percentage closer soft shadows) for light-size-aware penumbra, and ray-traced
   soft shadows on RT-capable hardware.
   - **Deps:** F-2.4.11
   - **Platform:** Mobile: PCF only (4-tap). Switch: PCF (8-tap) handheld, PCSS docked. Desktop:
     PCSS by default; RT soft shadows optional. High-end: RT soft shadows with full penumbra from
     area light size.
3. **F-2.4.13** — Tiered ambient occlusion: SSAO at half resolution as the baseline, GTAO (ground
   truth AO) with bent normals at mid tier, and ray-traced AO at high tier.
   - **Deps:** F-2.4.2
   - **Platform:** Mobile: SSAO at quarter resolution (4 samples); disabled under budget pressure.
     Switch: SSAO half-res handheld; GTAO half-res docked. Desktop: GTAO at full resolution.
     High-end: RT AO with bent normals.
4. **F-2.4.14** — High-resolution clipmap-based shadow mapping providing consistent shadow detail
   equivalent to a 16k x 16k virtual shadow atlas. Pages are allocated on demand based on
   screen-space coverage, paired with the meshlet pipeline for efficient geometry rendering into
   shadow pages.
   - **Deps:** F-2.4.11
   - **Platform:** Mobile: disabled; VRAM cost and page management overhead too high for mobile.
     Switch: disabled; uses CSM fallback. Desktop: enabled with 8k virtual atlas; requires 8+ GB
     VRAM. High-end: full 16k atlas with aggressive page caching.
5. **F-2.4.15** — Per-pixel screen-space depth ray-marching evaluated per light. Short rays are
   traced from each pixel along the light direction through the depth buffer to detect fine-scale
   shadow contact at geometry edges and crevices.
   - **Deps:** F-2.4.2
   - **Platform:** Mobile: disabled; per-pixel ray march too costly for bandwidth. Switch:
     directional light only, 8 steps, half-res. Desktop: all lights, 16 steps. High-end: all lights,
     32 steps, full resolution.
6. **F-2.4.16** — Shadows cast over long distances using mesh signed distance fields. A cone trace
   through the SDF volume produces soft shadowing with natural penumbra falloff, extending shadow
   range far beyond cascaded shadow map limits at minimal cost.
   - **Platform:** Mobile: disabled; SDF volume memory too large for mobile VRAM. Switch: disabled.
     Desktop: enabled for directional light only; SDF resolution configurable. High-end: full SDF
     shadows for all light types.
7. **F-2.4.17** — Lightweight soft shadow approximation for skeletal meshes using capsule physics
   representations. Each capsule casts a cone-traced shadow through the SDF, providing indirect area
   shadowing in GI-lit regions where traditional shadow maps have insufficient resolution.
   - **Deps:** F-2.4.16
   - **Platform:** Mobile: disabled (depends on SDF which is disabled). Switch: disabled. Desktop:
     enabled for hero characters (max 4). High-end: enabled for all skeletal meshes.
8. **F-2.4.18** — Moment-based order-independent transparency (MBOIT) rendering that correctly
   blends transparent geometry, fog, water, and particle effects without requiring CPU-side depth
   sorting. Per-pixel transmittance moments are accumulated in a multi-resolution pass, then
   resolved to produce correct compositing order. Enables seamless blending of volumetric fog with
   transparent surfaces and participating media.
   - **Deps:** F-2.4.5
   - **Platform:** Mobile: disabled; moment buffers require too much bandwidth and VRAM; falls back
     to sorted transparency (F-2.4.5). Switch: disabled; uses sorted fallback. Desktop: enabled at
     half-resolution moments. High-end: full-resolution MBOIT with 4-moment precision.
9. **F-2.4.19** — Shadow maps dedicated to participating media (fog, clouds, volumetric effects)
   that enable any volumetric element to cast and receive shadows from any light type. Integrated
   with the froxel-based volumetric rendering system to produce colored volumetric shadowing,
   silver-lining cloud effects, and light shaft occlusion through Fourier opacity mapping.
   - **Deps:** F-2.4.11
   - **Platform:** Mobile: disabled; volumetric fog uses baked approximation instead. Switch:
     disabled; uses analytical fog fallback. Desktop: enabled for directional light. High-end:
     enabled for all volumetric-casting light types with Fourier opacity mapping.
10. **F-2.4.20** — Rectangular and spherical area light evaluation using linearly-transformed cosine
    (LTC) integration. Area lights produce soft reflections and natural falloff proportional to
    source size, enabling realistic window lighting and softbox illumination.
    - **Deps:** F-2.3.1
    - **Platform:** Mobile: disabled; area lights approximated as point/spot with adjusted falloff.
      Switch: LTC evaluation for rect lights only; max 4. Desktop: full LTC for rect and sphere;
      configurable count. High-end: unlimited area lights.
11. **F-2.4.21** — Image-based ambient lighting captured from the sky atmosphere or a provided
    cubemap. The sky contribution is pre-filtered into diffuse irradiance and split-sum specular
    lookup tables for efficient real-time evaluation.
    - **Deps:** F-2.3.9
    - **Platform:** Mobile: precomputed LUTs only; no runtime sky refiltering; max 64x64 irradiance
      cubemap. Switch: runtime refiltering on time-of-day change only. Desktop/High-end: full
      runtime refiltering at 256x256+ cubemap resolution.
12. **F-2.4.22** — Photometric light distribution profiles loaded from IES data files. Each profile
    defines intensity as a function of angle, applied to point and spot lights for physically
    accurate architectural and cinematic lighting.
    - **Deps:** F-2.3.1
    - **Platform:** Mobile: IES profiles baked to low-res 1D textures (64 texels). Switch: 1D
      profile textures at 128 texels. Desktop/High-end: full 2D IES profile textures at 256+
      resolution.
13. **F-2.4.23** — Material-driven intensity and color modulation applied to any light type. A
    lightweight material graph produces a scalar or color mask (e.g., window blinds, gobo patterns,
    animated flicker) evaluated per-pixel in the light's influence volume.
    - **Deps:** F-2.3.1, F-2.4.3
    - **Platform:** Mobile: disabled; bake light functions to static textures at import. Switch:
      scalar-only light functions (no color modulation). Desktop/High-end: full material-driven
      light functions with color and animation.
