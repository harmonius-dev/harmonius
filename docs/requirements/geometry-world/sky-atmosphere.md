# R-3.5 — Sky & Atmosphere Requirements

## R-3.5.1 Procedural Sky Model

The engine **SHALL** evaluate sky luminance and chromaticity analytically as a function of view
angle, sun zenith, and atmospheric turbidity using a physically-motivated model (Preetham or
Hosek-Wilkie), serving as a fallback when volumetric scattering is budget-limited.

- **Derived from:** [F-3.5.1](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** An analytical sky model provides a fast, low-cost sky representation for
  performance-constrained platforms and as a reference for baking sky lookup tables.
- **Verification:** Unit test — evaluate sky color for sun zenith angles 0 to 90 degrees at
  10-degree increments; assert luminance decreases monotonically toward the horizon opposite the
  sun; assert chromaticity shifts toward warm tones at low sun angles.

## R-3.5.2 Multi-Scattering Atmosphere with Aerial Perspective

The engine **SHALL** compute atmospheric scattering using precomputed transmittance, single-
scattering, and multi-scattering lookup tables incorporating Rayleigh, Mie, and ozone absorption,
and apply aerial perspective to all scene geometry via a froxel-based volume.

- **Derived from:** [F-3.5.2](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Physically-based atmosphere scattering with aerial perspective is critical for
  convincing long-range views spanning tens of kilometers.
- **Verification:** Integration test — render terrain at distances of 1km, 10km, and 50km; assert
  pixel color at 50km is closer to the sky horizon color than at 1km; verify LUTs recompute when
  atmosphere parameters change; verify Mie scattering produces a visible sun halo.

## R-3.5.3 Ray-Marched Volumetric Clouds

The engine **SHALL** render volumetric clouds by ray marching through a cloud volume defined by
layered noise fields, with density, coverage, and type controlled by a weather map texture, lit via
cone-sampled in-scattering, and amortized across frames via temporal reprojection.

- **Derived from:** [F-3.5.3](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Volumetric clouds are essential for dynamic sky visuals and weather systems, and
  temporal reprojection makes ray marching viable at real-time frame rates.
- **Verification:** Integration test — render clouds with a test weather map defining 50% coverage;
  assert cloud pixels are non-transparent where coverage > 0; verify temporal reprojection reduces
  per-frame ray-march sample count by at least 50% compared to single-frame rendering; verify
  lighting responds to sun direction changes.

## R-3.5.4 Cloud Shadow Map

The engine **SHALL** generate a shadow map by rendering cloud density from the sun's perspective and
use it to modulate direct lighting on terrain, foliage, and water surfaces.

- **Derived from:** [F-3.5.4](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Cloud shadows provide large-scale moving shadow patterns that reinforce dynamic
  weather and world scale.
- **Verification:** Integration test — render terrain with a cloud layer at 50% coverage; assert
  terrain pixels under dense cloud regions receive measurably less direct light than pixels under
  clear sky; verify shadow pattern moves when cloud coverage animates.

## R-3.5.5 Dynamic Time-of-Day System

The engine **SHALL** drive sun and moon positions along astronomically-derived arcs, continuously
updating sky colors, atmosphere LUTs, ambient light, shadow direction, and fog parameters, with a
configurable time scale parameter for gameplay-controlled day/night cycle speed.

- **Derived from:** [F-3.5.5](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Smooth day/night transitions driven by astronomical sun/moon positions are
  fundamental to open-world immersion and time-gated gameplay mechanics.
- **Verification:** Integration test — advance the time-of-day from dawn through night over 60
  seconds (accelerated); assert sun altitude follows a smooth arc, sky color transitions through
  warm dawn, neutral midday, warm dusk, and dark night; assert shadow direction rotates
  continuously; verify time scale parameter of 2x doubles the cycle speed.

## R-3.5.6 Celestial Body Rendering

The engine **SHALL** render sun, moon, stars, and planetary bodies on the sky dome with configurable
sun limb darkening, phase-accurate moon illumination, magnitude-based star brightness with twinkling
and atmospheric extinction, and astronomically computed or artist-overridden positions.

- **Derived from:** [F-3.5.6](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Celestial bodies complete the sky presentation and must match the time-of-day
  system for visual consistency across day/night transitions.
- **Verification:** Visual test — render the sky at midnight; assert stars are visible with
  brightness proportional to catalog magnitude; assert star visibility decreases near the horizon
  (atmospheric extinction); render at first quarter moon phase and assert exactly half the moon disc
  is illuminated.

## R-3.5.7 Environment Cubemap Capture

The engine **SHALL** capture the sky, atmosphere, and cloud systems into a low-resolution
environment cubemap on a round-robin schedule across faces, providing ambient diffuse and specular
lighting via image-based lighting that matches the current sky state.

- **Derived from:** [F-3.5.7](../../features/geometry-world/sky-atmosphere.md)
- **Rationale:** Scene-wide ambient and specular lighting must reflect the current sky and weather
  conditions; round-robin capture amortizes the cost across frames.
- **Verification:** Integration test — change sky from clear midday to overcast; assert the captured
  cubemap ambient color shifts within the configured update period; verify a reflective sphere in
  the scene shows sky reflections matching the current atmosphere.
