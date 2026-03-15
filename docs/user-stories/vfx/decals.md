# User Stories -- 11.2 Decals

## US-11.2.1.1 Project Decals Across Curved Multi-Mesh Surfaces

**As a** environment artist (P-8), **I want** deferred projected decals that modify albedo, normal,
roughness, and metallic channels independently across multiple meshes, **so that** battle damage,
graffiti, and signage conform to complex fortress walls and cave surfaces.

## US-11.2.1.2 Use Triplanar Projection on Complex Geometry

**As a** environment artist (P-8), **I want** triplanar decal projection for surfaces where standard
box projection would stretch, **so that** decals on angled walls and intersecting geometry look
correct.

## US-11.2.1.3 Validate Forward Path Fallback to Mesh Decals

**As an** engine tester (P-27), **I want** to enable forward rendering and verify that deferred
decals are replaced by mesh decals without visual errors, **so that** decals work on platforms where
deferred rendering is disabled.

## US-11.2.2.1 Place High-Quality Permanent Markings on Static Meshes

**As a** environment artist (P-8), **I want** geometry-based mesh decals with correct tangent- space
normals for permanent markings like murals, graffiti, and signage, **so that** persistent decals
have sub-pixel accuracy on curved surfaces.

## US-11.2.2.2 Validate On-Demand Mesh Decal Generation Disabled on Mobile

**As an** engine tester (P-27), **I want** to verify that on-demand mesh decal generation is
disabled on mobile due to CPU cost, falling back to deferred decals, **so that** mobile does not
incur mesh decal generation overhead.

## US-11.2.3.1 Batch Decals Sharing Atlas Pages Into Single Draw Calls

**As an** engine developer (P-26), **I want** a runtime decal texture atlas that packs decal
textures into shared pages to minimize texture binds and draw calls, **so that** hundreds of active
decals render with minimal submission overhead.

## US-11.2.3.2 Validate Atlas Eviction Under Memory Pressure During Sieges

**As an** engine tester (P-27), **I want** to spawn 500 decals during a siege battle and verify that
LRU eviction reclaims atlas entries when memory pressure rises, **so that** atlas memory stays
bounded without allocation failures.

## US-11.2.4.1 Stack Overlapping Decals by Priority Without Z-Fighting

**As a** environment artist (P-8), **I want** priority-based decal stacking with per-channel blend
modes (alpha, multiply, additive), **so that** overlapping blood splatter, scorch marks, and
footprints resolve cleanly without z-fighting.

## US-11.2.4.2 Fade Decals Out With Noise-Threshold Dissolve

**As a** effects artist (P-12), **I want** decal lifecycle with configurable fade-in, sustain, and
dissolve-out using noise-threshold breakup, **so that** decals disappear naturally over time rather
than popping off abruptly.

## US-11.2.4.3 Validate Decal Pool Budget Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile uses a global pool of 64 decals
with shorter lifecycle durations and desktop uses 256, **so that** decal memory is recycled fast
enough on each platform.

## US-11.2.5.1 Spawn Velocity-Directed Blood Splatter From Hit Events

**As a** effects artist (P-12), **I want** procedural blood decals spawned from hit events with
variation driven by weapon type, impact angle, and surface material, **so that** melee combat
produces unique, non-repetitive blood spatter patterns.

## US-11.2.5.2 Validate Blood Decal Content Rating Gating

**As an** engine tester (P-27), **I want** to verify that blood decals are disabled when platform
content rating requires it (region-specific), **so that** the game passes certification in
restricted regions.

## US-11.2.6.1 Leave Footprints That Match the Terrain Material

**As a** player (P-23), **I want** surface-aware footprint decals where mud displaces, snow
compresses, and sand scatters based on the terrain material layer, **so that** walking through
different surfaces feels tactile and grounded.

## US-11.2.6.2 Validate Footprint Spawn Frequency Per Platform

**As an** engine tester (P-27), **I want** to verify that mobile spawns footprints every 4th step
and desktop spawns every step, **so that** footprint generation cost scales per platform.

## US-11.2.6.3 Render Vehicle Tire Tracks Along the Drive Path

**As a** effects artist (P-12), **I want** ribbon-style tire track decals projected along the
vehicle path with width matching the wheel contact patch, **so that** vehicles leave convincing
track marks on soft terrain.
