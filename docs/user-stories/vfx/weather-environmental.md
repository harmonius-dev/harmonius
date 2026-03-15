# User Stories -- 11.4 Weather and Environmental FX

## US-11.4.1.1 See Multi-Layered Rain With Screen Droplets

**As a** player (P-23), **I want** multi-layered rain combining GPU particle streaks, screen-
space camera droplets that merge and evaporate, and ripple normals on wet surfaces, **so that**
rain feels immersive from world scale down to close-up camera effects.

## US-11.4.1.2 Author Rain Density Curves Tied to Weather Intensity

**As a** effects artist (P-12), **I want** particle density and screen droplet frequency scaled
by weather intensity values, **so that** light drizzle and heavy downpour look and feel
distinct.

## US-11.4.1.3 Validate Single-Layer Rain on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses a single particle layer
with no screen droplets, rain density at 25% of desktop, and half-res ripple normals, **so
that** rain rendering fits within mobile bandwidth and compute budgets.

## US-11.4.2.1 See Puddles Form in Terrain Concavities During Rain

**As a** player (P-23), **I want** dynamic puddles that form based on terrain heightfield
accumulation, modify surface roughness to mirror-smooth, darken albedo, and show animated
ripples during active rainfall, **so that** rain transforms the ground surface convincingly.

## US-11.4.2.2 Validate Pre-Placed Puddle Decal Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses pre-placed puddle decals
with albedo-darken only (no dynamic heightfield, no roughness modification), **so that** puddle
rendering is cheap on mobile.

## US-11.4.2.3 Author Material-Driven Wet Surface Responses

**As a** technical artist (P-13), **I want** per-material wet surface responses (stone darkens,
metal reflects, dirt becomes mud), **so that** each surface type reacts to rain in a physically
appropriate way.

## US-11.4.3.1 See Snow Accumulate on Upward-Facing Surfaces Over Time

**As a** player (P-23), **I want** vertex-displacement snow that accumulates on surfaces over
time based on weather state, with footprints and vehicle tracks carving deformation trails,
**so that** snowfall gradually transforms the landscape.

## US-11.4.3.2 Author Snow Accumulation Parameters Per Environment

**As a** environment artist (P-8), **I want** configurable snow accumulation rate, maximum depth,
and deformation trail fade speed per zone, **so that** tundra zones accumulate snow faster than
mountain passes.

## US-11.4.3.3 Validate Texture-Blend Snow Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses texture-blend snow with
decal-based deformation instead of vertex displacement, **so that** snow rendering stays within
mobile vertex processing budgets.

## US-11.4.4.1 Place Localized Fog Volumes for Swamp Haze and Dungeon Mist

**As a** environment artist (P-8), **I want** oriented box and convex hull fog volumes with
per-volume density, color, and height falloff that inject into the global froxel grid, **so
that** specific areas have localized atmospheric effects without affecting global fog.

## US-11.4.4.2 Validate Screen-Space Height Fog Fallback on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses screen-space height fog
instead of volumetric froxel injection, with limited concurrent fog volumes, **so that** fog
effects work on mobile without volumetric rendering.

## US-11.4.5.1 See Branching Lightning Bolts Illuminate the Landscape

**As a** player (P-23), **I want** procedural branching lightning bolts that emit light bursts
illuminating terrain and clouds for a single frame with exponential decay, **so that**
thunderstorms have dramatic, dynamic lighting events.

## US-11.4.5.2 Author Storm Sequences With Multiple Simultaneous Bolts

**As a** effects artist (P-12), **I want** configurable branching depth, segment length, and
simultaneous bolt count for storm sequences, **so that** I can build intensity from single bolts
to full electrical storms.

## US-11.4.5.3 Validate Lightning Branching Depth on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile limits branching depth to 2
with max 1 simultaneous bolt and uses a single directional light flash, **so that** lightning
rendering stays within mobile compute and lighting budgets.

## US-11.4.6.1 See Leaves and Debris Carried by Wind Across the Landscape

**As a** player (P-23), **I want** GPU particle-driven wind visualization with leaves, dust,
and debris carried by the global wind field coherently with vegetation sway, **so that** wind
feels like a unified physical force affecting all visual elements.

## US-11.4.6.2 Create Dust Storms That Reduce Visibility

**As a** effects artist (P-12), **I want** dust storms that inject scattering density into
atmospheric fog and tint the sky toward storm color, **so that** approaching dust storms
create dramatic visibility reduction and color shift across the world.

## US-11.4.6.3 Validate Wind Particle Count on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile uses 10% of desktop wind
particle count and dust storms use distance fog only without volumetric scattering, **so that**
wind visualization fits within mobile budgets.

## US-11.4.7.1 See Caustic Light Patterns on Submerged Geometry

**As a** player (P-23), **I want** animated caustic light patterns projected onto underwater
surfaces with wavelength-dependent depth fog (reds fade before blues) and bubble streams,
**so that** diving underwater feels atmospheric and visually rich.

## US-11.4.7.2 Author Underwater Environments With Refraction and God Rays

**As a** environment artist (P-8), **I want** a refraction distortion layer at the water surface
boundary and screen-space god rays from above, **so that** underwater environments have the
characteristic rippling interface and light shaft effects.

## US-11.4.7.3 Validate Simplified Underwater on Mobile

**As an** engine tester (P-27), **I want** to verify that mobile skips caustic projection and
god rays, using depth fog and simplified blue tint only with bubbles at 25% count, **so that**
underwater rendering stays within mobile GPU budgets.
