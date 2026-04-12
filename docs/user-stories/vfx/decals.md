# User Stories -- 11.2 Decals

## Stories

| ID          | Persona                 |
|-------------|-------------------------|
| US-11.2.1.1 | effects artist (P-12)   |
| US-11.2.1.2 | effects artist (P-12)   |
| US-11.2.1.3 | technical artist (P-13) |
| US-11.2.2.1 | effects artist (P-12)   |
| US-11.2.2.2 | technical artist (P-13) |
| US-11.2.3.1 | engine developer (P-26) |
| US-11.2.3.2 | engine developer (P-26) |
| US-11.2.4.1 | effects artist (P-12)   |
| US-11.2.4.2 | effects artist (P-12)   |
| US-11.2.4.3 | engine developer (P-26) |
| US-11.2.5.1 | effects artist (P-12)   |
| US-11.2.5.2 | game designer (P-5)     |
| US-11.2.5.3 | engine developer (P-26) |
| US-11.2.6.1 | effects artist (P-12)   |
| US-11.2.6.2 | effects artist (P-12)   |
| US-11.2.6.3 | game designer (P-5)     |

1. **US-11.2.1.1** — **As a** effects artist (P-12), **I want** deferred projected decals that
   independently modify albedo, normal, roughness, and metallic channels across multiple meshes,
   **so that** battle damage and signage conform to complex surfaces.

2. **US-11.2.1.2** — **As a** effects artist (P-12), **I want** triplanar decal projection with
   per-axis fade and angle-based attenuation, **so that** decals on angled walls and intersecting
   geometry look correct without stretching.

3. **US-11.2.1.3** — **As a** technical artist (P-13), **I want** the forward rendering path to fall
   back to mesh decals when deferred is disabled, **so that** decals work on all rendering
   pipelines.

4. **US-11.2.2.1** — **As a** effects artist (P-12), **I want** geometry-based mesh decals with
   correct tangent-space normals and independent UVs, **so that** permanent murals and signage have
   sub-pixel accuracy on curved surfaces.

5. **US-11.2.2.2** — **As a** technical artist (P-13), **I want** on-demand mesh decal generation
   disabled on mobile with fallback to deferred decals, **so that** mobile does not incur mesh
   generation overhead.

6. **US-11.2.3.1** — **As a** engine developer (P-26), **I want** a runtime decal texture atlas that
   packs textures into shared pages and batches decals into single indirect draws, **so that**
   hundreds of active decals render with minimal draw call overhead.

7. **US-11.2.3.2** — **As a** engine developer (P-26), **I want** LRU eviction of atlas entries
   under memory pressure, **so that** atlas memory stays bounded during large battles.

8. **US-11.2.4.1** — **As a** effects artist (P-12), **I want** priority-based decal stacking with
   per-channel blend modes (alpha, multiply, additive), **so that** overlapping blood, scorch marks,
   and footprints resolve without z-fighting.

9. **US-11.2.4.2** — **As a** effects artist (P-12), **I want** decal lifecycle with fade-in,
   sustain, and dissolve-out using noise-threshold breakup, **so that** decals disappear naturally
   over time rather than popping off.

10. **US-11.2.4.3** — **As a** engine developer (P-26), **I want** a global decal pool (64 on
    mobile, 256 on desktop) that reclaims the oldest low-priority decals first, **so that** decal
    memory is recycled per platform.

11. **US-11.2.5.1** — **As a** effects artist (P-12), **I want** procedural damage decals spawned
    from hit events with variation by weapon type, impact angle, and surface material, **so that**
    melee combat produces unique, non-repetitive splatter patterns.

12. **US-11.2.5.2** — **As a** game designer (P-5), **I want** blood decals disableable by platform
    content rating, **so that** the game passes certification in restricted regions.

13. **US-11.2.5.3** — **As a** engine developer (P-26), **I want** damage decals persisting on both
    static surfaces and character meshes, **so that** visual combat feedback is consistent across
    all geometry.

14. **US-11.2.6.1** — **As a** effects artist (P-12), **I want** surface-aware footprint decals
    where mud displaces, snow compresses, and sand scatters based on terrain material, **so that**
    locomotion feels grounded.

15. **US-11.2.6.2** — **As a** effects artist (P-12), **I want** ribbon-style tire track decals
    projected along vehicle paths with width matching the wheel contact patch, **so that** vehicles
    leave convincing marks on soft terrain.

16. **US-11.2.6.3** — **As a** game designer (P-5), **I want** mobile footprints spawned every 4th
    step (vs. every step on desktop), **so that** footprint generation cost scales per platform.

## Parent Stories

The 3-segment parent stories below are umbrella rollups for the refined 4-segment sub-stories listed
above. Each parent inherits the persona of its first sub-story and describes the umbrella capability
that the sub-stories refine.

| ID | Persona |
|----|---------|
| US-11.2.1 | effects artist (P-12) |
| US-11.2.2 | effects artist (P-12) |
| US-11.2.3 | engine developer (P-26) |
| US-11.2.4 | effects artist (P-12) |
| US-11.2.5 | effects artist (P-12) |
| US-11.2.6 | effects artist (P-12) |

1. **US-11.2.1** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.2.1.1 through US-11.2.1.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

2. **US-11.2.2** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.2.2.1 through US-11.2.2.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

3. **US-11.2.3** -- **As a** engine developer (P-26), **I want** the capabilities defined in
   sub-stories US-11.2.3.1 through US-11.2.3.2 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

4. **US-11.2.4** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.2.4.1 through US-11.2.4.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

5. **US-11.2.5** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.2.5.1 through US-11.2.5.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.

6. **US-11.2.6** -- **As a** effects artist (P-12), **I want** the capabilities defined in
   sub-stories US-11.2.6.1 through US-11.2.6.3 combined into a single umbrella feature, **so that**
   I have a coherent parent story covering the refined child stories.
