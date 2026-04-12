# R-2.7 -- Environment and Atmosphere Requirements

## Sky and Fog

1. **R-2.7.1** — The engine **SHALL** render a physically- based sky atmosphere using precomputed
   LUTs with time-of-day transitions and Rayleigh/Mie scattering.
   - **Rationale:** Precomputed atmosphere LUTs provide realistic sky rendering at low runtime cost.
   - **Verification:** Transition from noon to sunset. Verify smooth color changes. Compare against
     reference atmosphere model within 5 dB PSNR.

2. **R-2.7.2** — The engine **SHALL** provide ray-marched volumetric fog using a froxel grid with
   configurable resolution, and analytical distance/height fog as a fallback.
   - **Rationale:** Froxel fog provides volumetric depth; analytical fog covers constrained
     platforms.
   - **Verification:** Place a light in fog. Verify visible scattering in the froxel grid. Disable
     froxel fog and verify analytical fallback produces smooth distance fade.

3. **R-2.7.3** — The engine **SHALL** render procedural volumetric clouds via ray marching through
   noise textures with temporal reprojection, and voxel-based clouds with SDF acceleration for
   fly-through gameplay.
   - **Rationale:** Noise clouds provide base quality; voxel clouds enable production-quality
     fly-through.
   - **Verification:** Fly through voxel clouds. Verify no pop-in or temporal artifacts. Verify
     noise clouds render as fallback when voxel clouds are disabled.

## Atmospheric Effects

4. **R-2.7.4** — The engine **SHALL** render god rays from bright occluded sources via screen-space
   radial blur or volumetric froxel integration.
   - **Rationale:** God rays add visual depth to scenes with strong directional light sources.
   - **Verification:** Place the sun behind an occluder. Verify visible light shafts. Verify
     volumetric mode reuses the froxel grid.

## Water

5. **R-2.7.5** — The engine **SHALL** provide FFT-based ocean surface simulation with reflections,
   foam, and underwater effects, with Gerstner wave fallback on mobile.
   - **Rationale:** FFT ocean provides realistic large-scale water; Gerstner fallback covers
     constrained platforms.
   - **Verification:** Render a large ocean. Verify wave motion matches FFT spectrum. Verify
     Gerstner fallback produces smooth wave animation on mobile.

6. **R-2.7.6** — The engine **SHALL** support art-directable breaking waves with guide curves and
   Houdini-simulated deformation baked as 2D textures.
   - **Rationale:** Guide curves give artists full control over shoreline wave behavior.
   - **Verification:** Define 3 guide curves. Verify waves follow the curves. Verify deformation
     textures produce correct vertex buffer output.

## Volumes and Weather

7. **R-2.7.7** — The engine **SHALL** render volumetric fire, smoke, and explosions from sparse
   volume data with full lighting and shadow integration.
   - **Rationale:** OpenVDB volumes integrate DCC-authored effects with scene lighting.
   - **Verification:** Import an OpenVDB smoke volume. Verify correct emission, absorption, and
     shadow casting. Verify billboard fallback on Switch.

8. **R-2.7.8** — The engine **SHALL** provide a dynamic weather state machine driving clouds, fog,
   precipitation, lighting, material wetness, and puddle rendering with configurable transition
   periods.
   - **Rationale:** Dynamic weather creates immersive atmosphere responsive to gameplay.
   - **Verification:** Transition from clear to rain. Verify clouds, fog density, precipitation, and
     material wetness update over the configured period. Verify puddles appear on horizontal
     surfaces.

## Breaking Waves

9. **R-2.7.9** — The engine **SHALL** render art-directable breaking waves using Coons-surface
   deformation driven by guide curves.
   - **Rationale:** Guide-curve-driven breaking waves give artists full control over shoreline wave
     shape and timing beyond the base FFT ocean simulation.
   - **Verification:** Define 3 guide curves. Verify waves follow the curves with Coons-surface
     interpolation. Verify deformation textures produce correct vertex output.
