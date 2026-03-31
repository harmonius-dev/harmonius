# 2.12 — Advanced Materials

## Transparency and Refraction

| ID       | Feature                                 |
|----------|-----------------------------------------|
| F-2.12.1 | Transparent Glass and Crystal Rendering |
| F-2.12.2 | Ocean Reflection and Refraction         |

1. **F-2.12.1** — Physically-based rendering of transparent, translucent, and refractive materials —
   glass windows, crystal objects, gemstones, ice, and liquids in containers. The system supports
   configurable index of refraction (IOR) per material, chromatic dispersion (wavelength-dependent
   refraction splitting white light into spectral colors), Fresnel reflectance (reflection intensity
   increases at grazing angles), absorption tinting (colored glass absorbs light traveling through
   the medium based on thickness), and caustic approximation (focused light patterns cast by
   refractive objects). Thin-surface mode handles flat glass (windows, bottles) without requiring
   solid mesh interiors. Thick-surface mode handles solid crystal and gemstone geometry with
   accurate internal refraction paths.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.10.5 (Material Parameter Binding)
   - **Platform:** Full ray-traced refraction available when RT hardware is present. Screen-space
     refraction used as fallback.
2. **F-2.12.2** — Specialized reflection and refraction for large water surfaces. Above-water
   rendering composites planar reflections (captured from a mirrored camera or screen-space
   reflection) with Fresnel-weighted underwater visibility through the surface. Below-water
   rendering shows refracted above-water scenery distorted by wave normals, with distance-based fog
   tint and god ray volumetrics. Reflection quality scales with the water surface's screen coverage
   — distant ocean uses low-resolution reflection probes while near-camera water uses
   full-resolution planar or ray-traced reflections. Integrates with the FFT ocean system (F-3.4.1)
   for wave-normal-driven distortion and foam masking at wave crests where reflection is replaced by
   diffuse white.
   - **Deps:** F-3.4.1 (FFT Ocean), F-2.5.3 (Ray-Traced Reflections)
   - **Platform:** Planar reflections require an additional render pass. RT reflections use the
     shared TLAS (F-2.5.1).

## Surface Detail

| ID       | Feature                              |
|----------|--------------------------------------|
| F-2.12.3 | Emission Maps and Emissive Materials |
| F-2.12.4 | Heightmap Tessellation and Parallax  |

1. **F-2.12.3** — Per-pixel emissive contribution controlled by an emission map (texture) and an
   emission intensity scalar. Emissive surfaces contribute directly to the lighting buffer,
   bypassing shadow and ambient occlusion. Emission intensity supports HDR values for bloom
   interaction — a neon sign emits enough light to trigger the bloom pass (F-2.9.2). Animated
   emission through scrolling UV, pulsing intensity curves, and color cycling driven by material
   parameters. Emissive materials participate in indirect lighting when ray tracing or screen-space
   GI is active, casting colored light onto nearby surfaces.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.9.2 (Bloom)
   - **Platform:** Mobile: static emission only (no animation); no GI contribution. Switch: animated
     emission; no GI contribution. Desktop: full emission with GI contribution when RT/screen-space
     GI active. High-end: same as desktop.
2. **F-2.12.4** — GPU tessellation driven by heightmap textures for adding geometric detail to flat
   surfaces — cobblestone streets, brick walls, terrain close-ups, and carved stone. The
   tessellation stage subdivides triangles based on camera distance and heightmap complexity, then
   displaces vertices along the surface normal by the heightmap value. Adaptive tessellation factors
   ensure dense tessellation only where needed (close to camera, high-detail areas) and minimal
   subdivision at distance. For surfaces where tessellation is too expensive, parallax occlusion
   mapping (POM) provides a ray-marched approximation with self-shadowing and silhouette correction.
   Material authors choose between tessellation and POM per material based on target platform
   budget.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.10.5 (Material Parameter Binding)
   - **Platform:** Hardware tessellation requires D3D12/Vulkan/Metal tessellation pipeline stages.
     POM fallback works on all platforms.

## Material Types

| ID       | Feature                                   |
|----------|-------------------------------------------|
| F-2.12.5 | Fabric and Cloth Materials                |
| F-2.12.6 | Metal, Wood, Stone, and Natural Materials |
| F-2.12.7 | Rubber, Wax, and Soft Surface Materials   |
| F-2.12.8 | Clearcoat and Multi-Layer Materials       |

1. **F-2.12.5** — Specialized shading models for woven and knitted fabrics. Fabric materials use a
   sheen BRDF layer (Charlie/Ashikhmin) for the soft, fuzzy highlight characteristic of velvet,
   silk, and cotton. Thread direction maps orient the sheen response to simulate weave patterns
   (satin, denim, tweed). Subsurface transmission through thin fabrics (curtains, flags backlit by
   sunlight) uses a thin-surface transmission approximation. Fuzz/microfiber rendering for materials
   like felt and fleece uses a short-fiber scattering model. Fabric materials integrate with the
   cloth simulation system (F-4.7.2) for physically-driven deformation.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-4.7.2 (Cloth Simulation)
   - **Platform:** Mobile: basic diffuse wrap only (no sheen/fuzz layer, no thread direction, no
     transmission). Switch: sheen layer on hero characters; no fuzz/microfiber. Desktop: full
     shading model with all layers. High-end: full model with subsurface transmission for thin
     fabrics.
2. **F-2.12.6** — Enhanced PBR shading for common natural and manufactured surface types. Metal
   materials support anisotropic reflections (brushed steel, machined aluminum) with tangent-space
   direction maps, multi-layer coatings (oxidized/patina layers), and per-pixel roughness variation
   for weathering. Wood materials use subsurface-aware shading for the translucent quality of thin
   wood (lampshades, paper screens) and grain-aligned anisotropy. Stone materials support porous
   subsurface scattering (marble, alabaster) and wet/dry surface state transitions (rain darkening).
   All natural material types share a common weathering system — age, exposure, and damage
   parameters drive procedural detail overlay (rust, moss, cracks, stains) without requiring unique
   textures.
   - **Deps:** F-2.3.1 (Lighting Pipeline)
   - **Platform:** Mobile: standard PBR only (no anisotropy, SSS, or weathering system); weathering
     baked to textures at import. Switch: anisotropy on hero objects; no SSS; simplified weathering
     (2 parameters). Desktop: full shading with weathering system. High-end: full model with
     multi-layer coatings and runtime weathering.
3. **F-2.12.7** — Shading models for materials with strong subsurface scattering and
   deformation-dependent appearance. Rubber uses a high-roughness BRDF with stretch-dependent color
   shift (lighter when stretched). Wax and candle materials use subsurface scattering with
   thickness-dependent transmission — thin wax glows when backlit, thick wax is opaque with a soft
   translucent edge. Skin-like soft materials (fruit, soap, organic tissue) share the wax subsurface
   profile with adjusted scattering radius and albedo. Deformation feedback from the physics system
   (vertex displacement, compression) modulates roughness and scattering parameters in real time.
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.8.6 (Subsurface Scattering)
   - **Platform:** Mobile: high-roughness BRDF only; no SSS or thickness transmission; no
     deformation feedback. Switch: preintegrated SSS LUT; no deformation feedback. Desktop: full SSS
     with transmission. High-end: full model with real-time deformation feedback modulating
     scattering.
4. **F-2.12.8** — A clearcoat layer (automotive paint, lacquered wood, coated ceramics, wet
   surfaces) rendered as a separate specular lobe on top of the base material's BRDF. The clearcoat
   has independent roughness, IOR, and tint parameters. Normal maps can differ between the base
   layer (orange peel texture on car paint) and the clearcoat (smooth gloss). Multi-layer material
   blending extends clearcoat to arbitrary layer stacks — a corroded pipe has a metal base, rust
   mid-layer, and wet clearcoat. Each layer blends via height-based masking with smooth transitions.
   Layer parameters are driven by material instance properties for runtime variation (rain wetting a
   clearcoat, rust spreading over time).
   - **Deps:** F-2.3.1 (Lighting Pipeline), F-2.10.5 (Material Parameter Binding)
   - **Platform:** Mobile: clearcoat as single combined roughness (no separate normal map); max 2
     layers baked at import. Switch: clearcoat with separate roughness; max 2 runtime layers.
     Desktop: full clearcoat with separate normals; max 4 layers. High-end: unlimited runtime layers
     with height-based blending.

## Custom Materials

| ID       | Feature                      |
|----------|------------------------------|
| F-2.12.9 | Fully Custom Material Graphs |

1. **F-2.12.9** — Materials authored entirely through the visual shader graph editor (F-15.8.5) with
   access to all rendering inputs (surface attributes, lighting data, scene depth, camera vectors,
   custom textures, noise functions) and configurable output targets (albedo, normal, roughness,
   metallic, emissive, opacity, world position offset, custom data channels). Custom materials can
   implement any shading model — procedural wood grain, animated lava, holographic displays, energy
   shields, magic effects, terrain blend materials — without engine source modification. Material
   functions (reusable sub-graphs) compose into complex materials. The graph compiles to HLSL, which
   DXC compiles to DXIL and SPIR-V, and Metal Shader Converter translates DXIL to MSL (F-12.2.9).
   This provides Unreal Engine Material Editor-level flexibility within the no-code visual authoring
   workflow. F-2.10.5 (Material Parameter Binding)
   - **Deps:** F-15.8.5 (Shader and Material Graphs), F-12.2.9 (Shader Compilation Pipeline),
   - **Platform:** Mobile: graph complexity capped (max 64 nodes, 4 texture reads); compiled to
     SPIR-V (Vulkan) or MSL (Metal) with aggressive half-precision. Switch: max 128 nodes, 8 texture
     reads. Desktop/High-end: no node or texture limits; full-precision evaluation.
