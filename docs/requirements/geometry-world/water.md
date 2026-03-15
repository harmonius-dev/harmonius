# R-3.4 — Water Requirements

## R-3.4.1 FFT Ocean Wave Simulation

The engine **SHALL** compute open-ocean surface displacement via inverse FFT on a GPU compute shader
using multiple spectral cascades, producing tiled displacement, normal, and fold maps that
seamlessly repeat across an infinite ocean grid.

- **Derived from:** [F-3.4.1](../../features/geometry-world/water.md)
- **Rationale:** FFT-based ocean simulation is required to produce physically plausible wave motion
  across large draw distances with artist-controllable swell parameters.
- **Verification:** Integration test — initialize a Phillips spectrum with three cascades; dispatch
  the FFT compute shader for 60 frames at a fixed timestep; sample displacement at tile boundaries
  and assert continuity (delta < 0.001m); verify fold map Jacobian values correlate with wave
  steepness.

## R-3.4.2 Shoreline and Depth-Based Blending

The engine **SHALL** blend water surfaces with terrain at shorelines by fading opacity and wave
amplitude based on scene depth, and generate an animated shoreline foam mask from the depth
gradient.

- **Derived from:** [F-3.4.2](../../features/geometry-world/water.md)
- **Rationale:** Hard intersections between water and terrain break visual immersion; smooth
  depth-based blending and foam produce natural beach and rocky shore effects.
- **Verification:** Visual test — render water meeting terrain at varying slopes; verify opacity
  fades smoothly from full at depth > 2m to zero at depth 0m; verify foam mask is non-zero within
  the configured shoreline band and animates over time.

## R-3.4.3 Underwater Rendering and Volume Effects

The engine **SHALL** switch to an underwater rendering mode when the camera submerges, applying
depth-based fog with Beer-Lambert absorption, color shift toward the water body's absorption
spectrum, refracted surface view from below, and volumetric god-ray light shafts.

- **Derived from:** [F-3.4.3](../../features/geometry-world/water.md)
- **Rationale:** Underwater environments require distinct rendering to convey submersion through
  light attenuation, color shift, and volumetric effects.
- **Verification:** Integration test — place camera 5m below the water surface; assert fog density
  increases with depth, scene color shifts toward the configured absorption spectrum, a distorted
  above-surface refraction is visible, and light shaft intensity decreases with depth.

## R-3.4.4 Water Caustics Projection

The engine **SHALL** project caustic light patterns onto underwater surfaces and the seabed,
computed from the ocean normal map or approximated via animated tiling caustics maps, modulating
lighting contribution on receiving surfaces.

- **Derived from:** [F-3.4.4](../../features/geometry-world/water.md)
- **Rationale:** Caustics add critical visual richness to shallow water and underwater scenes,
  reinforcing the sense of light refracting through a water surface.
- **Verification:** Visual test — render a lit seabed 3m below the water surface; verify caustic
  patterns are visible, animate over time, and intensity scales with water depth and surface wave
  amplitude.

## R-3.4.5 Water Reflection and Refraction

The engine **SHALL** render water surfaces with Fresnel-weighted blending of reflection
(screen-space reflections for nearby objects, environment cubemap for distant sky, optional planar
reflection) and refraction (normal-map-driven distortion of the below-surface scene).

- **Derived from:** [F-3.4.5](../../features/geometry-world/water.md)
- **Rationale:** Physically motivated reflection and refraction blending is essential for visually
  convincing water across all viewing angles.
- **Verification:** Integration test — render water at grazing and steep view angles; assert
  reflection contribution increases at grazing angles per Fresnel equations; assert refraction
  distortion magnitude correlates with normal map amplitude; verify SSR, cubemap, and planar
  reflection sources activate under their respective conditions.

## R-3.4.6 Flow Map River Simulation

The engine **SHALL** animate river and stream surfaces using artist-painted flow maps that define
per-texel velocity direction and magnitude, driving UV animation of normal and foam textures with
flow-speed-modulated wave amplitude.

- **Derived from:** [F-3.4.6](../../features/geometry-world/water.md)
- **Rationale:** Rivers require directional flow distinct from ocean waves; flow maps provide
  artist-controllable per-channel velocity without runtime fluid simulation.
- **Verification:** Integration test — apply a flow map with a uniform rightward velocity; assert
  normal texture UV offset advances rightward each frame at the configured speed; verify foam
  intensity increases with flow speed; verify river splines connect seamlessly with the ocean system
  at estuary points.

## R-3.4.7 Dynamic Foam Generation

The engine **SHALL** generate a foam coverage map accumulated from wave folding Jacobian, shoreline
depth, flow map turbulence, and object wake interactions, decaying over time and modulating surface
albedo, roughness, and opacity in the water material.

- **Derived from:** [F-3.4.7](../../features/geometry-world/water.md)
- **Rationale:** Unified foam generation from multiple sources produces consistent whitecap, surf,
  and wake foam across all water body types.
- **Verification:** Integration test — enable all foam sources (FFT folds, shoreline, flow, wake);
  assert foam coverage map is non-zero at each source location, decays toward zero over the
  configured lifetime, and visibly modulates surface albedo and roughness in the final render.
