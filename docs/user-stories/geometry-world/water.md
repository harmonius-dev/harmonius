# User Stories -- 3.4 Water

## Ocean Simulation

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.1.1 | environment artist (P-8) |
| US-3.4.1.2 | technical artist (P-13)  |
| US-3.4.1.3 | engine developer (P-26)  |
| US-3.4.1.4 | level designer (P-6)     |

1. **US-3.4.1.1** -- **As a** environment artist (P-8), **I want** open-ocean surface displacement
   computed via inverse FFT with multiple spectral cascades, **so that** waves range from large
   swells to capillary ripples.
2. **US-3.4.1.2** -- **As a** technical artist (P-13), **I want** analytical Gerstner waves
   layerable on top of FFT waves, **so that** I can art-direct swell behavior during storms or
   cinematics.
3. **US-3.4.1.3** -- **As a** engine developer (P-26), **I want** FFT resolution to scale per
   platform tier, **so that** mobile falls back to fewer cascades or Gerstner-only waves.
4. **US-3.4.1.4** -- **As a** level designer (P-6), **I want** the ocean surface to tile seamlessly
   across an infinite grid, **so that** naval traversal has no visible seams.

## Shoreline Blending

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.2.1 | environment artist (P-8) |
| US-3.4.2.2 | level designer (P-6)     |
| US-3.4.2.3 | technical artist (P-13)  |

1. **US-3.4.2.1** -- **As a** environment artist (P-8), **I want** water to blend smoothly with
   terrain at shorelines using depth-based opacity and wave amplitude adjustments, **so that**
   beaches look natural.
2. **US-3.4.2.2** -- **As a** level designer (P-6), **I want** a shoreline foam mask generated from
   the depth gradient and animated with scrolling noise, **so that** wave breaking and surf are
   visible.
3. **US-3.4.2.3** -- **As a** technical artist (P-13), **I want** foam mask resolution and blend
   width configurable per platform tier, **so that** mobile uses simpler shoreline effects.

## Underwater Rendering

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.3.1 | level designer (P-6)     |
| US-3.4.3.2 | environment artist (P-8) |
| US-3.4.3.3 | engine developer (P-26)  |

1. **US-3.4.3.1** -- **As a** level designer (P-6), **I want** the renderer to switch to underwater
   mode with depth fog and color shift when the camera submerges, **so that** underwater zones feel
   immersive.
2. **US-3.4.3.2** -- **As a** environment artist (P-8), **I want** volumetric god rays from the
   surface rendered as light shafts, **so that** underwater lighting has visual depth.
3. **US-3.4.3.3** -- **As a** engine developer (P-26), **I want** underwater effects to degrade
   gracefully on mobile with screen-space approximations, **so that** all platforms support
   submersion.

## Caustics

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.4.1 | environment artist (P-8) |
| US-3.4.4.2 | technical artist (P-13)  |
| US-3.4.4.3 | engine developer (P-26)  |

1. **US-3.4.4.1** -- **As a** environment artist (P-8), **I want** caustic light patterns projected
   onto underwater surfaces and the seabed, **so that** shallow water sparkles realistically.
2. **US-3.4.4.2** -- **As a** technical artist (P-13), **I want** real-time refracted caustics on
   desktop and pre-baked tiling caustics on mobile, **so that** visual quality scales with platform
   capability.
3. **US-3.4.4.3** -- **As a** engine developer (P-26), **I want** caustic textures projected in
   world space to modulate lighting on receiving surfaces, **so that** caustic integration is
   composable with the lighting pipeline.

## Water Reflection / Refraction

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.5.1 | environment artist (P-8) |
| US-3.4.5.2 | technical artist (P-13)  |
| US-3.4.5.3 | level designer (P-6)     |

1. **US-3.4.5.1** -- **As a** environment artist (P-8), **I want** water surfaces to combine
   reflection and refraction using Fresnel blending, **so that** water looks physically correct from
   all angles.
2. **US-3.4.5.2** -- **As a** technical artist (P-13), **I want** a hierarchical reflection approach
   (SSR, cubemap, optional planar pass) selectable per platform, **so that** reflection quality
   matches device capability.
3. **US-3.4.5.3** -- **As a** level designer (P-6), **I want** refraction to distort the
   below-surface scene using the water normal map, **so that** objects beneath the water appear to
   shimmer.

## River Flow

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.6.1 | environment artist (P-8) |
| US-3.4.6.2 | level designer (P-6)     |
| US-3.4.6.3 | technical artist (P-13)  |

1. **US-3.4.6.1** -- **As a** environment artist (P-8), **I want** rivers to use artist-painted flow
   maps that drive UV animation of normal and foam textures, **so that** water appears to flow
   directionally.
2. **US-3.4.6.2** -- **As a** level designer (P-6), **I want** river spline meshes to connect
   seamlessly with the ocean at estuary points, **so that** rivers and oceans form a unified water
   system.
3. **US-3.4.6.3** -- **As a** technical artist (P-13), **I want** flow speed to modulate wave
   amplitude and foam intensity, **so that** fast-flowing rapids look distinct from calm stretches.

## Foam Generation

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.7.1 | environment artist (P-8) |
| US-3.4.7.2 | technical artist (P-13)  |
| US-3.4.7.3 | engine developer (P-26)  |

1. **US-3.4.7.1** -- **As a** environment artist (P-8), **I want** foam generated dynamically from
   wave folding, shoreline depth, and object wakes, **so that** whitecaps and surf appear naturally.
2. **US-3.4.7.2** -- **As a** technical artist (P-13), **I want** the foam coverage map to modulate
   surface albedo, roughness, and opacity, **so that** foam integrates with the water material
   shader.
3. **US-3.4.7.3** -- **As a** engine developer (P-26), **I want** foam map resolution and feature
   set to scale per platform tier, **so that** mobile disables wake foam while desktop renders full
   dynamic foam.

## Clipmap Mesh Generation

| ID         | Persona                  |
|------------|--------------------------|
| US-3.4.8.1 | game developer (P-15)    |

1. **US-3.4.8.1** -- **As a** game developer (P-15), **I want** the ocean surface built as
   camera-centred clipmap rings with FFT displacement applied in the vertex shader, **so that**
   water maintains high tessellation near the camera and scales to infinite extent.
