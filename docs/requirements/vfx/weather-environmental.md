# R-11.4 -- Weather & Environmental FX Requirements

## Precipitation

1. **R-11.4.1** — The engine **SHALL** render multi-layered rain combining GPU particle streaks,
   screen-space droplets with gravity flow, merge, and evaporation, and ripple normal maps, with
   density scaling by weather intensity and sky exposure.
   - **Rationale:** Layered rain provides convincing precipitation from distant streaks to
     near-camera droplets, essential for immersive weather.
   - **Verification:** Enable rain and verify particle streaks, screen droplets, and ripple normals.
     Reduce intensity and confirm proportional density decrease. Move under shelter and verify
     droplets cease. Stop storm and confirm evaporation.

2. **R-11.4.2** — The engine **SHALL** form dynamic puddles from heightfield accumulation and
   terrain concavity, modifying roughness toward mirror-smooth, darkening albedo, adding ripple
   normals, and applying material-driven wet responses that drain after rain stops.
   - **Rationale:** Puddles and wet surfaces ground rainfall by showing material-appropriate
     moisture accumulation.
   - **Verification:** Enable rain and verify puddles in concave areas. Confirm stone darkens, metal
     reflects, dirt becomes mud. Verify ripple normals during rain. Stop rain and confirm drainage.

3. **R-11.4.3** — The engine **SHALL** render vertex-displacement snow accumulating on upward-facing
   surfaces using a world-space height texture, with character and vehicle deformation trails via
   depth stamps that reveal underlying surface and fade under continued snowfall.
   - **Rationale:** Interactive snow creates a living environment responding to weather and player
     activity.
   - **Verification:** Enable snowfall and verify accumulation. Walk a character and confirm
     deformation trails. Continue snowfall and verify trails fill in. Confirm walls remain
     snow-free.

## Atmospheric Effects

4. **R-11.4.4** — The engine **SHALL** render localized volumetric fog via oriented boxes or convex
   hulls with per-volume density, color, height falloff, and animation, injecting into the global
   froxel grid with temporal reprojection.
   - **Rationale:** Localized fog enables area-specific atmosphere without altering the global fog.
   - **Verification:** Place a box fog volume and verify density confined to bounds. Adjust
     parameters and confirm independence. Verify froxel injection without ghosting. Confirm areas
     outside unaffected.

5. **R-11.4.5** — The engine **SHALL** render procedural branching lightning using L-system
   subdivision with randomized angles and lengths, single-frame illumination with exponential decay,
   and distance-delayed audio cues, supporting multiple simultaneous bolts.
   - **Rationale:** Lightning provides dramatic visual punctuation requiring synchronized lighting
     and audio.
   - **Verification:** Trigger a bolt and verify branching geometry. Confirm single-frame
     illumination with decay. Trigger multiple bolts and verify independence. Verify thunder delay
     increases with distance.

6. **R-11.4.6** — The engine **SHALL** render GPU particle wind visualization (leaves, dust, debris)
   with velocity from the shared wind volume, and simulate dust storms by injecting scattering
   density into atmospheric fog and tinting the sky.
   - **Rationale:** Wind-driven particles make the wind field visually tangible and create dramatic
     weather transitions.
   - **Verification:** Enable wind and verify coherent particle motion with vegetation sway. Trigger
     a dust storm and confirm visibility reduction and sky color tinting. Disable wind and confirm
     particle motion ceases.

## Underwater Effects

7. **R-11.4.7** — The engine **SHALL** render underwater with animated caustics on submerged
   geometry, exponential wavelength-dependent depth fog, bubble streams, refraction distortion at
   the surface boundary, and screen-space god rays.
   - **Rationale:** Underwater rendering requires specialized optical effects simulating light
     through water.
   - **Verification:** Submerge camera and verify caustics, depth fog with red attenuation before
     blue, and bubbles. Cross the surface and confirm refraction distortion. Verify god rays from
     surface direction.
