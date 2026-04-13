# R-3.5 -- Sky & Atmosphere Requirements

## Procedural Sky

1. **R-3.5.1** -- The engine **SHALL** provide a physically-motivated analytical sky model computing
   sky luminance and chromaticity as a function of view angle, sun zenith, and atmospheric
   turbidity.
   - **Rationale:** An analytical sky serves as fallback when volumetric atmosphere exceeds budget.
   - **Verification:** Render the sky at various sun positions. Assert sky color matches reference
     (Preetham/Hosek-Wilkie) within perceptual threshold.

## Multi-Scattering Atmosphere

2. **R-3.5.2** -- The engine **SHALL** implement precomputed multi-scattering atmosphere LUTs
   (Rayleigh, Mie, ozone) with froxel-based aerial perspective applied to all scene geometry. LUTs
   **SHALL** recompute when atmosphere parameters change.
   - **Rationale:** Physically-based scattering with aerial perspective is critical for large draw
     distances.
   - **Verification:** Change atmosphere parameters. Assert LUTs rebuild. Assert distant terrain
     fades into haze matching the current atmosphere.

## Volumetric Clouds

3. **R-3.5.3** -- The engine **SHALL** render clouds by ray marching through noise-based volumes
   with temporal reprojection, configurable step count per platform tier, and weather-map-driven
   coverage.
   - **Rationale:** Ray-marched volumetric clouds provide depth and lighting not achievable with
     skybox textures.
   - **Verification:** Render clouds with temporal reprojection. Assert no visible ghosting. Assert
     step count matches the platform tier setting.

4. **R-3.5.4** -- The engine **SHALL** project cloud shadows onto terrain and scene geometry via a
   shadow map rendered from the sun's perspective.
   - **Rationale:** Cloud shadows reinforce dynamic weather and world scale at minimal cost.
   - **Verification:** Enable cloud shadows. Assert shadow patterns move with cloud coverage. Assert
     shadow map resolution matches the platform tier.

## Dynamic Time of Day

5. **R-3.5.5** -- The engine **SHALL** drive sun and moon positions along astronomically-derived
   arcs, continuously updating sky colors, atmosphere LUTs, ambient light, and shadow direction with
   a configurable time scale.
   - **Rationale:** Smooth time-of-day transitions are essential for day/night cycles in open-world
     games.
   - **Verification:** Accelerate time. Assert dawn through night transitions are smooth. Assert all
     dependent systems (sky, light, shadow) update in sync.

## Celestial Bodies

6. **R-3.5.6** -- The engine **SHALL** render sun, moon, stars, and planetary bodies as part of the
   sky dome with astronomically computed or artist-overridden positions.
   - **Rationale:** Celestial bodies complete the sky presentation at all times of day.
   - **Verification:** Render a night sky. Assert stars display with magnitude-based brightness.
     Assert moon phase matches the time-of-day state.

## Environment Cubemap Capture

7. **R-3.5.7** -- The engine **SHALL** capture sky, atmosphere, and clouds into a low-resolution
   cubemap on a round-robin schedule, providing ambient IBL for the scene.
   - **Rationale:** IBL from sky capture ensures reflections and ambient lighting match the current
     sky state.
   - **Verification:** Change the sky. Assert cubemap updates within the configured frame interval.
     Assert IBL reflections match the new sky.
