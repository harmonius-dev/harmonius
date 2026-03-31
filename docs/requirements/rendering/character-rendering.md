# R-2.8 -- Character Rendering Requirements

## Hair

1. **R-2.8.1** — The engine **SHALL** render individual hair strands with the Marschner BSDF (R, TT,
   TRT lobes) and self-shadowing via deep opacity maps, supporting hundreds of thousands of strands.
   - **Rationale:** Strand rendering with Marschner shading produces photorealistic hair for hero
     characters.
   - **Verification:** Render 500k strands. Verify R, TT, and TRT lobes match reference images.
     Verify deep opacity shadows with no light leaking.

2. **R-2.8.2** — The engine **SHALL** provide card-based hair rendering as a fallback with
   alpha-blended layered cards, and a multi-tier LOD system transitioning from strands to cards to
   mesh proxy with cross-fade dithering.
   - **Rationale:** Card hair and LOD ensure hair is represented at all distances and platforms.
   - **Verification:** Verify strand-to-card transition at configured distance. Verify mesh proxy at
     far distance. Verify cross-fade dithering with no popping.

3. **R-2.8.3** — The engine **SHALL** provide a compute software rasterizer for sub-pixel hair
   strands with froxel-based anti-aliased line drawing.
   - **Rationale:** Sub-pixel strands bypass hardware rasterization; compute rasterization ensures
     they are visible with anti-aliasing.
   - **Verification:** Render thin strands at distance. Verify compute-rasterized strands blend with
     hardware- rasterized strands with no visible transition.

## Eyes and Skin

4. **R-2.8.4** — The engine **SHALL** render eyes with layered cornea refraction, iris parallax
   depth, sclera SSS, and limbal ring darkening with analytical caustics.
   - **Rationale:** Layered eye shading produces photorealistic character eyes.
   - **Verification:** Render a close-up eye. Verify iris parallax. Verify cornea refraction. Verify
     sclera SSS. Verify flat iris fallback on mobile.

5. **R-2.8.5** — The engine **SHALL** provide screen-space SSS for skin using Burley normalized
   diffusion profiles, with preintegrated LUT fallback on mobile.
   - **Rationale:** Burley diffusion produces physically accurate skin translucency.
   - **Verification:** Render skin on desktop and mobile. Compare results. Verify LUT fallback is
     visually acceptable.

6. **R-2.8.6** — The engine **SHALL** render peach fuzz vellus hair as a screen-space effect driven
   by fuzz direction maps on character faces.
   - **Rationale:** Peach fuzz adds subtle realism to close- up character rendering without strand
     geometry.
   - **Verification:** Render a face close-up. Verify visible fuzz catching light at grazing angles.
     Verify disabled on mobile.

7. **R-2.8.7** — The engine **SHALL** provide biometric skin shading driven by melanin concentration
   and blood distribution maps producing accurate scattering for any skin tone.
   - **Rationale:** Biometric parameterization produces natural skin tone variation from biological
     data.
   - **Verification:** Render skin tones from light to dark using melanin maps. Verify correct
     scattering. Verify baked-to-diffuse fallback on mobile.

## Cloth

8. **R-2.8.8** — The engine **SHALL** provide a cloth shading model with a fiber fuzz layer and
   fabric-specific scatter, degrading to simplified wrap lighting on mobile.
   - **Rationale:** Cloth shading distinguishes fabric surfaces from smooth materials.
   - **Verification:** Render velvet fabric. Verify fuzz layer at grazing angles. Verify simplified
     fallback on mobile.
