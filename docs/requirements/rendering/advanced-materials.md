# R-2.12 -- Advanced Materials Requirements

## Transparency and Refraction

1. **R-2.12.1** — The engine **SHALL** render transparent, translucent, and refractive materials
   with configurable IOR, chromatic dispersion, Fresnel reflectance, absorption tinting, and caustic
   approximation, with screen-space refraction fallback when RT is unavailable.
   - **Rationale:** Physically-based refraction is required for glass, crystal, ice, and liquid
     materials.
   - **Verification:** Render a glass sphere. Verify refraction distortion. Verify Fresnel at
     grazing angles. Verify screen-space fallback without RT.

2. **R-2.12.2** — The engine **SHALL** composite ocean reflections and refractions with
   Fresnel-weighted above/ below-water visibility, scaling reflection quality with screen coverage.
   - **Rationale:** Water surfaces require combined reflection and refraction compositing.
   - **Verification:** Render a large ocean. Verify above- water planar reflection. Verify
     underwater refracted view. Verify low-res probes at distance.

## Surface Detail

3. **R-2.12.3** — The engine **SHALL** support emissive materials with HDR intensity triggering
   bloom, animated emission via UV scrolling and pulsing, and GI contribution when active.
   - **Rationale:** Emissive materials enable neon signs, magic effects, and light-casting surfaces.
   - **Verification:** Set emission intensity above bloom threshold. Verify bloom activation. Verify
     scrolling UV animation. Verify GI contribution with RT active.

4. **R-2.12.4** — The engine **SHALL** provide GPU tessellation driven by heightmap textures with
   adaptive factors based on camera distance, and parallax occlusion mapping as a fallback.
   - **Rationale:** Tessellation adds geometric detail to flat surfaces at close range; POM covers
     lower tiers.
   - **Verification:** Apply a heightmap to a flat surface. Verify displaced geometry. Verify
     adaptive density reduces at distance. Verify POM fallback with self- shadowing.

## Material Types

5. **R-2.12.5** — The engine **SHALL** provide fabric materials with sheen BRDF, thread direction
   maps, thin-surface transmission, and microfiber scattering, degrading to diffuse wrap on mobile.
   - **Rationale:** Fabric shading distinguishes woven and knitted materials from smooth surfaces.
   - **Verification:** Render velvet, silk, and denim. Verify distinct sheen responses. Verify
     transmission through thin fabric. Verify diffuse-only on mobile.

6. **R-2.12.6** — The engine **SHALL** provide enhanced PBR for metal, wood, and stone with
   anisotropic reflections, subsurface scattering, and a shared weathering system using procedural
   detail overlay.
   - **Rationale:** Natural material shading with weathering enables realistic environmental
     surfaces.
   - **Verification:** Apply weathering to a metal surface. Verify rust overlay. Verify anisotropic
     reflection on brushed steel. Verify runtime weathering on desktop and baked fallback on mobile.

7. **R-2.12.7** — The engine **SHALL** provide materials for rubber, wax, and soft surfaces with
   SSS, thickness- dependent transmission, and deformation-modulated scattering, falling back to
   high-roughness BRDF on mobile.
   - **Rationale:** Soft material shading enables wax glow, rubber stretch, and organic
     translucency.
   - **Verification:** Backlight a thin wax surface. Verify transmission glow. Verify
     stretch-dependent color shift on rubber. Verify BRDF fallback on mobile.

8. **R-2.12.8** — The engine **SHALL** provide clearcoat materials with independent roughness, IOR,
   tint, and separate normal maps, supporting arbitrary layer stacks with height-based blending.
   - **Rationale:** Clearcoat and multi-layer materials enable automotive paint, lacquer, and wet
     surfaces.
   - **Verification:** Create car paint with clearcoat. Verify separate specular lobes. Verify 4
     layers on desktop. Verify 2 baked layers on mobile.

## Custom Materials

9. **R-2.12.9** — The engine **SHALL** allow fully custom material graphs with access to all
   rendering inputs and configurable outputs, compiling to GLSL via `naga` to SPIR-V,
   with reusable material functions.
   - **Rationale:** Custom graphs provide full creative freedom without engine source modification.
   - **Verification:** Create a procedural lava material. Verify animated output. Verify material
     function reuse. Verify compiled output runs on all backends.
