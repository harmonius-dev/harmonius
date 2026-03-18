# R-2.12 — Advanced Materials Requirements

## Transparent and Refractive Materials

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.12.1 | [F-2.12.1](../../features/rendering/advanced-materials.md) |
| R-2.12.2 | [F-2.12.2](../../features/rendering/advanced-materials.md) |

1. **R-2.12.1** — The engine **SHALL** render transparent, translucent, and refractive materials
   (glass, crystal, gemstones, ice, liquids) with configurable index of refraction, chromatic
   dispersion, Fresnel reflectance, absorption tinting based on thickness, and caustic
   approximation, supporting both thin-surface mode (flat glass) and thick-surface mode (solid
   crystal with internal refraction paths).
   - **Rationale:** Physically-based refraction and absorption are required for convincing glass,
     crystal, and liquid rendering across many game genres.
   - **Verification:** Render a glass window (thin-surface) and a crystal gemstone (thick-surface)
     and verify IOR-based refraction distorts the background; enable chromatic dispersion and
     confirm visible spectral color splitting; verify Fresnel reflectance increases at grazing
     angles; apply colored absorption and confirm thickness-dependent tinting; verify RT refraction
     is used when available and screen-space fallback otherwise.
2. **R-2.12.2** — The engine **SHALL** render large water surfaces with above-water Fresnel-weighted
   reflections (planar, SSR, or RT) composited with underwater visibility, and below-water refracted
   above-water scenery distorted by wave normals with distance-based fog and god ray volumetrics,
   scaling reflection quality by screen coverage (probes at distance, full resolution near camera)
   and integrating with the FFT ocean system for wave-normal distortion and foam masking.
   - **Rationale:** Ocean rendering requires specialized reflection and refraction handling distinct
     from general-purpose transparent materials due to scale and wave interaction.
   - **Verification:** Render an ocean surface and verify above-water reflections are
     Fresnel-weighted; submerge the camera and verify refracted above-water scenery with wave
     distortion and depth fog; verify reflection quality scales with distance (low-res probes far,
     full-res near); confirm foam at wave crests masks reflection with diffuse white.

## Surface Detail

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.12.3 | [F-2.12.3](../../features/rendering/advanced-materials.md) |
| R-2.12.4 | [F-2.12.4](../../features/rendering/advanced-materials.md) |

1. **R-2.12.3** — The engine **SHALL** render per-pixel emissive contribution from emission map
   textures and emission intensity scalars, bypassing shadow and ambient occlusion, supporting HDR
   intensity values that trigger bloom (R-2.9.1), animated emission (scrolling UV, pulsing
   intensity, color cycling), and indirect lighting contribution when ray tracing or screen-space GI
   is active.
   - **Rationale:** Emissive materials are essential for neon signs, screens, magic effects, and
     other self-luminous surfaces that must contribute to the lighting environment.
   - **Verification:** Apply an emission map to a material and verify per-pixel emissive light
     appears in the lighting buffer independent of shadows and AO; set HDR emission intensity and
     confirm bloom is triggered; enable emission animation and verify scrolling, pulsing, and color
     cycling; verify emissive surfaces cast indirect light when GI is active.
2. **R-2.12.4** — The engine **SHALL** provide GPU tessellation driven by heightmap textures with
   adaptive tessellation factors based on camera distance and heightmap complexity, displacing
   vertices along surface normals, and a parallax occlusion mapping (POM) fallback with
   self-shadowing and silhouette correction for platforms or materials where tessellation is too
   expensive.
   - **Rationale:** Heightmap tessellation and POM add geometric detail to flat surfaces
     (cobblestone, brick, terrain) without requiring dense source geometry.
   - **Verification:** Apply a heightmap to a flat surface and verify visible vertex displacement
     with tessellation enabled; move the camera away and confirm tessellation factor decreases
     adaptively; switch to POM mode and verify ray-marched parallax depth with self-shadowing;
     confirm POM fallback works on all platforms while tessellation requires D3D12/Vulkan/Metal
     pipeline stages.

## Material Types

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.12.5 | [F-2.12.5](../../features/rendering/advanced-materials.md) |
| R-2.12.6 | [F-2.12.6](../../features/rendering/advanced-materials.md) |
| R-2.12.7 | [F-2.12.7](../../features/rendering/advanced-materials.md) |
| R-2.12.8 | [F-2.12.8](../../features/rendering/advanced-materials.md) |

1. **R-2.12.5** — The engine **SHALL** render fabric materials with a sheen BRDF layer
   (Charlie/Ashikhmin) for velvet, silk, and cotton highlights, thread direction maps for weave
   pattern simulation, thin-surface transmission for backlit fabrics, and a short-fiber scattering
   model for felt/fleece, integrating with the cloth simulation system for physically-driven
   deformation.
   - **Rationale:** Fabric requires specialized shading layers (sheen, transmission, fiber
     scattering) that standard PBR does not provide.
   - **Verification:** Render a velvet material and verify the sheen BRDF produces a soft fuzzy
     highlight; apply a thread direction map and confirm weave pattern is visible; place a thin
     fabric (curtain) backlit by sunlight and verify transmission; render fleece and verify
     short-fiber scattering; confirm deformation from cloth simulation updates the material
     response.
2. **R-2.12.6** — The engine **SHALL** render enhanced PBR shading for natural and manufactured
   surfaces including anisotropic metal reflections with tangent-space direction maps and
   multi-layer coatings, subsurface-aware wood shading with grain anisotropy, porous stone
   subsurface scattering with wet/dry transitions, and a shared weathering system (age, exposure,
   damage) driving procedural detail overlay (rust, moss, cracks, stains) without unique textures.
   - **Rationale:** Natural material types have distinct optical properties (anisotropy, subsurface,
     porosity, weathering) that require specialized shading beyond standard PBR.
   - **Verification:** Render brushed steel and verify anisotropic reflection aligned to the
     direction map; render thin wood and verify subsurface translucency; render marble and verify
     porous SSS; toggle wet/dry state on stone and verify surface darkening; apply weathering
     parameters and confirm procedural overlay (rust, moss) appears without unique textures.
3. **R-2.12.7** — The engine **SHALL** render materials with strong subsurface scattering and
   deformation-dependent appearance: rubber with stretch-dependent color shift, wax with
   thickness-dependent transmission (thin wax glows when backlit, thick wax is opaque with
   translucent edges), and skin-like soft materials sharing the wax subsurface profile with adjusted
   scattering radius, with deformation feedback from the physics system modulating roughness and
   scattering parameters in real time.
   - **Rationale:** Soft deformable materials require coupling between physics-driven deformation
     and shading parameters for realistic appearance under interaction.
   - **Verification:** Render a rubber object, stretch it via physics deformation, and verify color
     lightens in stretched regions; render a thin wax surface backlit by a light and verify visible
     glow transmission; render a thick wax object and verify opaque body with translucent edges;
     apply physics deformation and confirm roughness and scattering respond in real time.
4. **R-2.12.8** — The engine **SHALL** render a clearcoat layer as a separate specular lobe with
   independent roughness, IOR, and tint on top of the base material's BRDF, with independent normal
   maps for base and clearcoat layers, and multi-layer material blending via height-based masking
   with smooth transitions, supporting runtime-driven layer parameter variation (rain wetting, rust
   spreading).
   - **Rationale:** Clearcoat and multi-layer materials are required for automotive paint, lacquered
     surfaces, wet surfaces, and weathered objects with layered appearance.
   - **Verification:** Render automotive paint with a clearcoat layer and verify a separate specular
     highlight from the clearcoat independent of the base layer; apply different normal maps to base
     and clearcoat and confirm both are visible; blend a rust mid-layer via height-based masking and
     verify smooth transitions; drive rain wetting at runtime and confirm the clearcoat parameters
     update dynamically.

## Custom Materials

| ID       | Derived From                                               |
|----------|------------------------------------------------------------|
| R-2.12.9 | [F-2.12.9](../../features/rendering/advanced-materials.md) |

1. **R-2.12.9** — The engine **SHALL** support materials authored entirely through the visual shader
   graph editor with access to all rendering inputs (surface attributes, lighting data, scene depth,
   camera vectors, custom textures, noise functions) and configurable output targets (albedo,
   normal, roughness, metallic, emissive, opacity, world position offset, custom data channels),
   compiled to HLSL and then to all GPU backends via DXC and Metal Shader Converter without engine
   source modification.
   - **Rationale:** Fully custom material graphs provide Unreal Engine Material Editor-level
     flexibility within the no-code visual authoring workflow, enabling any shading model.
   - **Verification:** Create a custom material graph reading scene depth and outputting a
     procedural pattern to albedo and normal channels; compile the graph to all GPU backends and
     verify correct rendering on each; confirm the material requires no engine source modification;
     verify access to all declared input categories and output targets.

## Non-Functional Requirements

| ID         |
|------------|
| NFR-2.12.1 |
| NFR-2.12.2 |
| NFR-2.12.3 |

1. **NFR-2.12.1** — A custom material graph with up to 100 nodes **SHALL** compile to all three GPU
   backends (Metal, D3D12, Vulkan) in under 5 seconds total, and incremental recompilation after a
   single node change **SHALL** complete in under 1 second.
   - **Rationale:** Material iteration in the editor must be interactive; long compile times break
     the authoring workflow.
   - **Verification:** Create a 100-node material graph and measure full compilation time across all
     backends. Verify it is under 5 seconds. Modify one node and verify recompilation under 1
     second.
2. **NFR-2.12.2** — Glass and crystal refraction **SHALL** produce output within 5 dB PSNR of a
   ray-traced reference when using the screen-space fallback path, and within 1 dB when using the RT
   path.
   - **Rationale:** Refraction quality must be physically plausible even on the fallback path to
     maintain visual consistency.
   - **Verification:** Render a glass sphere with both screen-space and RT refraction paths. Compare
     each against a ray-traced reference and verify PSNR thresholds.
3. **NFR-2.12.3** — Multi-layer material blending with up to 4 layers **SHALL** add no more than 50%
   to the base PBR evaluation cost per fragment.
   - **Rationale:** Layered materials are common for weathered surfaces; their cost must scale
     predictably with layer count.
   - **Verification:** Profile a 4-layer material (base + 3 overlay layers) and compare against
     single-layer PBR. Verify the cost increase is below 50%.
