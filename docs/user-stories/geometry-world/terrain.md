# User Stories -- 3.2 Terrain

## Heightfield Terrain

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.1.1 | level designer (P-6)     |
| US-3.2.1.2 | engine developer (P-26)  |
| US-3.2.1.3 | environment artist (P-8) |
| US-3.2.1.4 | technical artist (P-13)  |

1. **US-3.2.1.1** -- **As a** level designer (P-6), **I want** terrain tiles to stream in and out
   based on camera proximity with low-LOD fallbacks, **so that** I can traverse the open world
   without loading screens.
2. **US-3.2.1.2** -- **As a** engine developer (P-26), **I want** tile streaming to use Tokio async
   I/O, **so that** I/O performance matches each platform's capability.
3. **US-3.2.1.3** -- **As a** environment artist (P-8), **I want** each tile to store a 16-bit or
   32-bit height grid with metadata for materials and holes, **so that** terrain has sufficient
   vertical precision.
4. **US-3.2.1.4** -- **As a** technical artist (P-13), **I want** a residency manager tracking
   loaded, loading, and evicted tiles, **so that** streaming state is observable for debugging.

## Virtual Texturing

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.2.1 | environment artist (P-8) |
| US-3.2.2.2 | technical artist (P-13)  |
| US-3.2.2.3 | engine developer (P-26)  |

1. **US-3.2.2.1** -- **As a** environment artist (P-8), **I want** terrain materials to use a
   virtual texture clipmap with nested resolution rings, **so that** near-camera terrain has full
   detail while distant terrain uses coarser pages.
2. **US-3.2.2.2** -- **As a** technical artist (P-13), **I want** GPU feedback to identify visible
   virtual texture pages, **so that** missing pages load asynchronously without visual corruption.
3. **US-3.2.2.3** -- **As a** engine developer (P-26), **I want** VRAM usage bounded regardless of
   world size, **so that** the virtual texture system scales to any terrain extent.

## Terrain LOD

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.3.1 | technical artist (P-13)  |
| US-3.2.3.2 | level designer (P-6)     |
| US-3.2.3.3 | engine developer (P-26)  |

1. **US-3.2.3.1** -- **As a** technical artist (P-13), **I want** terrain meshes using clipmap LOD
   rings with vertex morphing at boundaries, **so that** there are no visible seams between LOD
   levels.
2. **US-3.2.3.2** -- **As a** level designer (P-6), **I want** vertex density to match pixel
   resolution at every distance, **so that** terrain looks sharp nearby and efficient at distance.
3. **US-3.2.3.3** -- **As a** engine developer (P-26), **I want** ring count and vertex density to
   scale per platform tier, **so that** mobile uses fewer rings with more aggressive LOD.

## Terrain Holes and Materials

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.4.1 | level designer (P-6)     |
| US-3.2.4.2 | engine developer (P-26)  |
| US-3.2.5.1 | environment artist (P-8) |
| US-3.2.5.2 | technical artist (P-13)  |

1. **US-3.2.4.1** -- **As a** level designer (P-6), **I want** per-tile hole masks that discard
   terrain triangles and mirror in collision, **so that** I can create cave entrances and building
   transitions.
2. **US-3.2.4.2** -- **As a** engine developer (P-26), **I want** hole masks stored as
   1-bit-per-vertex grids, **so that** the mask data is compact and consistent across rendering and
   physics.
3. **US-3.2.5.1** -- **As a** environment artist (P-8), **I want** up to 16 PBR material layers per
   tile with height-weighted transitions, **so that** terrain materials blend naturally at
   boundaries.
4. **US-3.2.5.2** -- **As a** technical artist (P-13), **I want** active layer count to scale per
   platform tier, **so that** mobile uses fewer layers without breaking the splatmap system.

## Terrain Collision

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.6.1 | level designer (P-6)     |
| US-3.2.6.2 | engine developer (P-26)  |

1. **US-3.2.6.1** -- **As a** level designer (P-6), **I want** terrain collision to use a dedicated
   representation independent of visual LOD, **so that** character movement and vehicle physics
   remain stable.
2. **US-3.2.6.2** -- **As a** engine developer (P-26), **I want** collision mesh updates to mirror
   the streaming tile grid, **so that** physics stays consistent as tiles load and unload.

## Large World Coordinates

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.7.1 | level designer (P-6)     |
| US-3.2.7.2 | engine developer (P-26)  |

1. **US-3.2.7.1** -- **As a** level designer (P-6), **I want** 64-bit world coordinates with
   camera-relative rendering, **so that** I can build worlds spanning thousands of kilometers
   without jitter.
2. **US-3.2.7.2** -- **As a** engine developer (P-26), **I want** positions rebased to a 32-bit
   camera-origin frame before GPU submission, **so that** GPU rendering has no precision loss at
   large distances.

## Indoor Environments

| ID         | Persona                  |
|------------|--------------------------|
| US-3.2.8.1 | level designer (P-6)     |
| US-3.2.8.2 | environment artist (P-8) |
| US-3.2.8.3 | engine developer (P-26)  |

1. **US-3.2.8.1** -- **As a** level designer (P-6), **I want** to place portal volumes at room
   openings and have the engine cull rooms not visible through portal chains, **so that** complex
   interiors render efficiently.
2. **US-3.2.8.2** -- **As a** environment artist (P-8), **I want** each room to support its own
   lighting environment connected through portals, **so that** a torch-lit dungeon connects
   naturally to a sunlit courtyard.
3. **US-3.2.8.3** -- **As a** engine developer (P-26), **I want** portal traversal depth capped per
   platform tier, **so that** mobile limits overdraw from deeply nested rooms.

## Voxel Terrain

| ID          | Persona                  |
|-------------|--------------------------|
| US-3.2.9.1  | level designer (P-6)     |
| US-3.2.9.2  | engine developer (P-26)  |
| US-3.2.10.1 | level designer (P-6)     |
| US-3.2.10.2 | engine developer (P-26)  |
| US-3.2.11.1 | environment artist (P-8) |
| US-3.2.11.2 | engine developer (P-26)  |
| US-3.2.12.1 | technical artist (P-13)  |
| US-3.2.12.2 | engine developer (P-26)  |
| US-3.2.13.1 | level designer (P-6)     |
| US-3.2.13.2 | engine developer (P-26)  |
| US-3.2.14.1 | engine developer (P-26)  |
| US-3.2.14.2 | level designer (P-6)     |

1. **US-3.2.9.1** -- **As a** level designer (P-6), **I want** a sparse voxel volume with SDF values
   and material IDs, **so that** I can create caves, overhangs, and underground spaces impossible
   with heightmaps.
2. **US-3.2.9.2** -- **As a** engine developer (P-26), **I want** the sparse octree to consume no
   memory for empty regions, **so that** voxel terrain scales to large worlds without excessive
   memory use.
3. **US-3.2.10.1** -- **As a** level designer (P-6), **I want** the engine to automatically choose
   heightmap or voxel representation per region, **so that** I get heightmap efficiency with voxel
   flexibility where needed.
4. **US-3.2.10.2** -- **As a** engine developer (P-26), **I want** seamless boundary stitching
   between heightmap and voxel regions, **so that** there are no visible cracks or physics
   discontinuities.
5. **US-3.2.11.1** -- **As a** environment artist (P-8), **I want** a spherical voxel volume with
   clipmap LOD and radial gravity, **so that** I can create planets with seamless ground-to-orbit
   transitions.
6. **US-3.2.11.2** -- **As a** engine developer (P-26), **I want** near-camera terrain at
   meter-scale resolution while the antipodal hemisphere uses kilometer-scale LOD, **so that**
   planetary terrain is memory-feasible.
7. **US-3.2.12.1** -- **As a** technical artist (P-13), **I want** selectable meshing algorithms
   (Marching Cubes, Dual Contouring, Surface Nets, Transvoxel), **so that** I can match the meshing
   style to the art direction.
8. **US-3.2.12.2** -- **As a** engine developer (P-26), **I want** incremental meshing that
   re-meshes only modified chunks, **so that** runtime editing has interactive performance.
9. **US-3.2.13.1** -- **As a** level designer (P-6), **I want** to dig tunnels, flatten ground, and
   paint materials at runtime with edits saved and replicated, **so that** I can shape the world
   during gameplay.
10. **US-3.2.13.2** -- **As a** engine developer (P-26), **I want** voxel edits serialized as
    compact delta logs with server-authoritative validation, **so that** multiplayer editing is
    secure and bandwidth-efficient.
11. **US-3.2.14.1** -- **As a** engine developer (P-26), **I want** voxel octree nodes to stream on
    demand with compression and memory budget enforcement, **so that** planetary-scale voxel worlds
    are feasible.
12. **US-3.2.14.2** -- **As a** level designer (P-6), **I want** distant voxel nodes to use coarse
    LOD that refines as I approach, **so that** terrain detail loads progressively.
