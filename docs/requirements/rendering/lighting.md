# R-2.4 -- Lighting and Materials Requirements

## Lighting Paths

1. **R-2.4.1** — The engine **SHALL** perform tiled or clustered light culling via a compute pass,
   evaluating only per-tile lights during fragment shading, supporting hundreds of dynamic lights
   per scene.
   - **Rationale:** Tiled culling bounds per-fragment cost, enabling hundreds of lights without
     quadratic scaling.
   - **Verification:** Render 500 point lights. Verify each fragment evaluates only its tile's
     lights via GPU capture. Confirm sub-linear frame time scaling.

2. **R-2.4.2** — The engine **SHALL** support deferred lighting via a G-buffer with albedo-metallic,
   normal- roughness, motion vectors, and depth targets.
   - **Rationale:** Deferred lighting decouples geometry complexity from lighting cost.
   - **Verification:** Inspect G-buffer targets for correct data. Verify deferred output matches
     forward within 40 dB PSNR.

3. **R-2.4.3** — The engine **SHALL** implement Cook-Torrance PBR with GGX, Smith, and Schlick,
   accessing material textures via bindless indices.
   - **Rationale:** Cook-Torrance is the industry standard PBR model ensuring DCC-to-engine material
     consistency.
   - **Verification:** Render a material test sphere set. Compare against a reference renderer.
     Verify bindless indices produce correct texture reads.

4. **R-2.4.4** — The engine **SHALL** support extended BSDF layers for SSS, clearcoat, anisotropy,
   and sheen on top of the PBR base, degrading to fewer lobes on lower tiers.
   - **Rationale:** Extended layers cover skin, fabric, car paint, and hair without separate shading
     models.
   - **Verification:** Render each extended lobe. Verify visual correctness. Verify mobile uses max
     1 lobe and desktop enables all.

## Transparency and Material Instances

5. **R-2.4.5** — The engine **SHALL** render transparent objects sorted back-to-front with
   individual draw calls after all opaque geometry.
   - **Rationale:** Per-object sorting preserves correct transparency compositing order.
   - **Verification:** Place 10 overlapping transparent objects. Verify correct visual ordering from
     all angles.

6. **R-2.4.6** — The engine **SHALL** support parameterized material instances sharing compiled
   shaders with only a per-instance parameter buffer uploaded.
   - **Rationale:** Instances enable thousands of variants at minimal GPU cost.
   - **Verification:** Create 1,000 instances of one parent. Verify all share the same PSO. Verify
     parameter overrides are visible per instance.

7. **R-2.4.7** — The engine **SHALL** support multi-layer material compositing with blend masks
   evaluated per pixel, with configurable layer limits per platform.
   - **Rationale:** Layered materials enable environmental wear without unique textures.
   - **Verification:** Stack 4 layers on desktop. Verify correct blending. Verify mobile caps at 2
     layers.

8. **R-2.4.8** — The engine **SHALL** render projected and mesh-based decals, with projected decals
   writing into a deferred decal buffer before lighting.
   - **Rationale:** Decals add detail without modifying source geometry.
   - **Verification:** Place a projected decal on a curved surface. Verify correct normal-mapped
     contact. Verify DBuffer writes before lighting.

## Shading Variants and Stochastic Lighting

9. **R-2.4.9** — The engine **SHALL** provide dedicated shading model variants for hair, cloth, eye,
   foliage, thin translucent, preintegrated skin, and water, selectable via material flag.
   - **Rationale:** Specialized models optimize code paths for distinct material categories.
   - **Verification:** Render each variant. Verify visual correctness. Verify mobile disables eye
     refraction and Marschner hair.

10. **R-2.4.10** — The engine **SHALL** implement importance- sampled stochastic lighting for
    thousands of shadowed area lights with temporal accumulation denoising.
    - **Rationale:** Stochastic sampling replaces per-light shadow map evaluation at scale.
    - **Verification:** Render 2,000 area lights. Verify soft shadows converge within 8 frames.
      Verify frame time stays within budget.

## Shadows

11. **R-2.4.11** — The engine **SHALL** support cascaded shadow maps for directional lights with 1-4
    configurable cascades, depth bias, and temporal stabilization.
    - **Rationale:** CSM provides sun shadows with crisp near-field detail across the full scene
      range.
    - **Verification:** Configure 4 cascades. Verify no cascade seam artifacts. Verify temporal
      stabilization prevents shadow swimming.

12. **R-2.4.12** — The engine **SHALL** provide tiered soft shadows: PCF baseline, PCSS for
    penumbra, and RT soft shadows on capable hardware.
    - **Rationale:** Tiered shadows match quality to hardware capability.
    - **Verification:** Render with each tier. Verify PCSS penumbra matches light size. Verify RT
      shadows produce correct area light penumbra.

13. **R-2.4.13** — The engine **SHALL** provide tiered AO: SSAO at half-res baseline, GTAO with bent
    normals at mid tier, and RT AO at high tier.
    - **Rationale:** Tiered AO scales occlusion quality with hardware capability.
    - **Verification:** Render with each tier. Verify GTAO bent normals match reference. Verify RT
      AO matches ground truth within 2 dB PSNR.

14. **R-2.4.14** — The engine **SHALL** support virtual shadow maps using clipmap-based allocation
    providing consistent 16k-equivalent shadow detail.
    - **Rationale:** VSM provides uniform shadow resolution across the visible range.
    - **Verification:** Verify shadow resolution is uniform near and far. Verify page allocation is
      proportional to screen coverage.

15. **R-2.4.15** — The engine **SHALL** support contact shadows, distance field shadows, and capsule
    shadows for fine-scale, long-range, and character shadow detail.
    - **Rationale:** Complementary shadow techniques cover different spatial scales at low cost.
    - **Verification:** Verify contact shadows at geometry edges. Verify DF shadows extend beyond
      cascade range. Verify capsule shadows ground characters.

## Additional Lighting

16. **R-2.4.16** — The engine **SHALL** support order- independent transparency using moment-based
    OIT with fallback to sorted transparency on constrained platforms.
    - **Rationale:** OIT correctly composites overlapping translucent geometry without CPU sorting.
    - **Verification:** Render overlapping transparent objects. Verify OIT compositing matches
      reference. Verify sorted fallback activates on mobile.

17. **R-2.4.17** — The engine **SHALL** support rectangular and spherical area lights via LTC
    integration, IES photometric profiles, image-based sky lighting, light functions, and volumetric
    shadow maps.
    - **Rationale:** Area lights, IES profiles, IBL, and light functions provide production-quality
      lighting.
    - **Verification:** Render area lights and verify soft reflection matches LTC reference. Verify
      IES profiles match data files. Verify sky IBL updates with time.
