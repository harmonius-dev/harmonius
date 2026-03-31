# R-3.4 -- Water Requirements

## FFT Ocean

1. **R-3.4.1** -- The engine **SHALL** compute open-ocean surface displacement via inverse FFT on
   GPU compute with multiple spectral cascades, seamlessly tiling across an infinite grid, with
   resolution scaling per platform tier.
   - **Rationale:** FFT ocean with multiple cascades captures wave scales from swells to capillary
     ripples.
   - **Verification:** Render the ocean. Assert seamless tiling with no visible seams. Assert mobile
     uses fewer cascades or Gerstner fallback.

## Shoreline Blending

2. **R-3.4.2** -- The engine **SHALL** blend water surfaces with terrain at shorelines using
   depth-based opacity, wave amplitude adjustment, and an animated shoreline foam mask.
   - **Rationale:** Depth-based shoreline blending produces natural beach and shore effects.
   - **Verification:** View a shoreline. Assert soft intersection with terrain. Assert foam mask
     animates with scrolling noise.

## Underwater Rendering

3. **R-3.4.3** -- The engine **SHALL** switch to underwater mode when the camera submerges, applying
   depth fog, Beer-Lambert absorption, color shift, and optional volumetric god rays.
   - **Rationale:** Underwater mode is essential for immersive aquatic zones.
   - **Verification:** Submerge the camera. Assert fog and color shift activate. Assert god rays
     render on desktop. Assert mobile uses screen-space approximation.

## Water Caustics

4. **R-3.4.4** -- The engine **SHALL** project caustic light patterns onto underwater surfaces,
   using real-time refracted caustics on desktop and pre-baked tiling maps on mobile.
   - **Rationale:** Caustics add visual richness to shallow water environments.
   - **Verification:** View the seabed under shallow water. Assert caustic patterns are visible and
     animate with wave motion.

## Reflection and Refraction

5. **R-3.4.5** -- The engine **SHALL** combine reflection and refraction on water surfaces using
   Fresnel blending, with hierarchical reflection quality (SSR, cubemap, optional planar) scaling
   per platform tier.
   - **Rationale:** Fresnel-weighted blending produces physically correct water appearance.
   - **Verification:** View water at varying angles. Assert reflection dominates at grazing angles.
     Assert refraction distorts the below-surface scene.

## Flow Map Rivers

6. **R-3.4.6** -- The engine **SHALL** support flow-map- driven rivers with directional UV
   animation, flow-speed- modulated foam, and seamless connection to the ocean at estuary points.
   - **Rationale:** Flow maps produce convincing directional water movement along river channels.
   - **Verification:** Paint a flow map on a river. Assert normals animate in the flow direction.
     Assert the river connects to the ocean without visible seams.

## Dynamic Foam

7. **R-3.4.7** -- The engine **SHALL** generate foam dynamically from wave folding, shoreline depth,
   flow turbulence, and object wakes, modulating surface albedo and roughness via a foam coverage
   map.
   - **Rationale:** Dynamic foam produces realistic whitecaps and surf without manual placement.
   - **Verification:** Observe whitecaps in rough seas. Assert foam intensity correlates with wave
     folding. Assert foam coverage map resolution scales per tier.
