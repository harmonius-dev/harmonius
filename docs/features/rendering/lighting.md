# Lighting and Materials

## F-2.4.1 Forward+ Lighting (Tiled/Clustered)

Tiled or clustered light culling via a compute pass that assigns visible lights to screen-space
tiles. Fragment shading evaluates only the lights relevant to each tile, enabling hundreds of dynamic
lights.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven,
  [R-3.3.11](../../requirements/3-nonfunctional/3.3-data-constraints.md) lighting model selection

## F-2.4.2 Deferred Lighting (G-Buffer)

G-buffer rendering with albedo-metallic, normal-roughness, motion vectors, and depth targets. A
fullscreen compute pass accumulates all light contributions from the G-buffer. Suitable for scenes
with high geometric complexity and many lights.

- **Requirements:**
  [R-3.3.11](../../requirements/3-nonfunctional/3.3-data-constraints.md) lighting model selection

## F-2.4.3 PBR Materials (Cook-Torrance BRDF)

Physically-based rendering using Cook-Torrance microfacet BRDF with GGX normal distribution, Smith
geometry masking, and Schlick Fresnel approximation. Material textures (albedo, normal,
metallic-roughness, occlusion, emissive) are accessed via bindless indices.

- **Requirements:** [R-3.2.1](../../requirements/3-nonfunctional/3.2-hardware.md) bindless,
  [R-3.3.10](../../requirements/3-nonfunctional/3.3-data-constraints.md) glTF import

## F-2.4.4 Extended BSDF Materials

Extended material model adding subsurface scattering, clearcoat, anisotropy, and sheen layers on top
of the standard PBR base. Covers skin, fabric, car paint, and hair.

- **Requirements:** [R-3.2.1](../../requirements/3-nonfunctional/3.2-hardware.md) bindless

## F-2.4.5 Transparent Objects

Correct rendering of transparent geometry using CPU-side back-to-front distance sorting before
upload. Transparent objects are sorted per-object and drawn individually -- they are not batched by
material. Each transparent object produces its own draw call to preserve correct depth ordering.
Transparent objects participate in the standard lighting pipeline but are rendered after all opaque
geometry.

- **Requirements:** [R-3.3.1](../../requirements/3-nonfunctional/3.3-data-constraints.md) reverse-Z

## F-2.4.6 Material Instances

Parameterized overrides of a parent material allowing scalar, vector, and texture substitutions
without recompilation. Instances share the compiled shader and only upload a per-instance parameter
buffer, enabling thousands of material variants at minimal cost.

## F-2.4.7 Material Layers and Blending

Multi-layer material compositing where independent material layers (e.g., rust over metal, snow over
rock) are stacked with blend masks. Each layer is a self-contained material graph evaluated and
composited at the pixel level.

## F-2.4.8 Decal Rendering

Projected and mesh-based decals. Projected decals render as oriented boxes that write into a deferred
decal buffer (DBuffer) before lighting or directly into the G-buffer. Mesh decals wrap geometry for
proper normal-mapped contact with curved surfaces.

## F-2.4.9 Shading Model Variants

Dedicated per-pixel shading models beyond standard PBR, selected via material flag: hair (anisotropic
Marschner), cloth (fuzz layer with fiber scattering), eye (cornea refraction with iris parallax),
thin translucent (single-pass tinted glass), two-sided foliage (subsurface transmission),
preintegrated skin (low-cost SSS), and single-layer water (absorption, scattering, refraction).

## F-2.4.10 Stochastic Many-Light Sampling

Importance-sampled direct lighting for scenes with thousands of shadowed area lights. A fixed ray
budget per pixel is distributed across light sources proportional to estimated contribution
(luminance and solid angle), replacing per-light shadow map evaluation with a unified stochastic
pass and temporal accumulation denoiser.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven

## Shadows

## F-2.4.11 Cascaded Shadow Maps

Multi-cascade shadow mapping for directional lights with logarithmic/linear split blending. Supports
1-4 cascades with configurable resolution, depth bias, and temporal stabilization.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven,
  [R-3.1.9](../../requirements/3-nonfunctional/3.1-performance.md) quality tier fallbacks

## F-2.4.12 Soft Shadows (PCF / PCSS / RT)

Tiered soft shadow implementations: PCF (percentage closer filtering) as the baseline, PCSS
(percentage closer soft shadows) for light-size-aware penumbra, and ray-traced soft shadows on
RT-capable hardware.

- **Requirements:** [R-3.1.4](../../requirements/3-nonfunctional/3.1-performance.md) feature/budget
  gates,
  [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.4.13 Ambient Occlusion (SSAO / GTAO / RT AO)

Tiered ambient occlusion: SSAO at half resolution as the baseline, GTAO (ground truth AO) with bent
normals at mid tier, and ray-traced AO at high tier.

- **Requirements:** [R-3.1.4](../../requirements/3-nonfunctional/3.1-performance.md) feature/budget
  gates,
  [R-3.2.3](../../requirements/3-nonfunctional/3.2-hardware.md) RT soft-gated

## F-2.4.14 Virtual Shadow Maps

High-resolution clipmap-based shadow mapping providing consistent shadow detail equivalent to a
16k x 16k virtual shadow atlas. Pages are allocated on demand based on screen-space coverage, paired
with the meshlet pipeline for efficient geometry rendering into shadow pages.

- **Requirements:** [R-1.1.2](../../requirements/1-architecture/1.1-core-constraints.md) GPU-driven,
  [R-1.1.3](../../requirements/1-architecture/1.1-core-constraints.md) mesh shaders

## F-2.4.15 Contact Shadows

Per-pixel screen-space depth ray-marching evaluated per light. Short rays are traced from each pixel
along the light direction through the depth buffer to detect fine-scale shadow contact at geometry
edges and crevices.

## F-2.4.16 Distance Field Shadows

Shadows cast over long distances using mesh signed distance fields. A cone trace through the SDF
volume produces soft shadowing with natural penumbra falloff, extending shadow range far beyond
cascaded shadow map limits at minimal cost.

## F-2.4.17 Capsule Shadows

Lightweight soft shadow approximation for skeletal meshes using capsule physics representations. Each
capsule casts a cone-traced shadow through the SDF, providing indirect area shadowing in GI-lit
regions where traditional shadow maps have insufficient resolution.

## F-2.4.18 Order-Independent Transparency (OIT)

Moment-based order-independent transparency (MBOIT) rendering that correctly blends transparent
geometry, fog, water, and particle effects without requiring CPU-side depth sorting. Per-pixel
transmittance moments are accumulated in a multi-resolution pass, then resolved to produce correct
compositing order. Enables seamless blending of volumetric fog with transparent surfaces and
participating media.

## F-2.4.19 Volumetric Shadow Maps

Shadow maps dedicated to participating media (fog, clouds, volumetric effects) that enable any
volumetric element to cast and receive shadows from any light type. Integrated with the froxel-based
volumetric rendering system to produce colored volumetric shadowing, silver-lining cloud effects, and
light shaft occlusion through Fourier opacity mapping.
