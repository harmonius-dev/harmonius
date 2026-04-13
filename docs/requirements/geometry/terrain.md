# R-3.2 -- Terrain Requirements

## Heightfield Terrain

1. **R-3.2.1** -- The engine **SHALL** represent terrain as a tile-based heightfield streaming tiles
   via platform- native async I/O with low-LOD fallbacks for unloaded tiles.
   - **Rationale:** Tile streaming enables seamless traversal of open worlds without loading
     screens.
   - **Verification:** Traverse a world larger than VRAM. Assert tiles stream seamlessly. Assert
     fallback LODs display for unloaded tiles.

2. **R-3.2.2** -- The engine **SHALL** composite terrain materials using a virtual texture clipmap
   with GPU feedback-driven page loading, bounding VRAM regardless of world size.
   - **Rationale:** Virtual clipmaps maintain full detail near the camera while scaling to any world
     extent.
   - **Verification:** Pan across large terrain. Assert near-camera textures are full resolution.
     Assert VRAM stays bounded.

3. **R-3.2.3** -- The engine **SHALL** tessellate terrain using clipmap LOD rings with vertex
   morphing at boundaries, sizing rings so vertex density matches pixel resolution at every
   distance.
   - **Rationale:** Screen-space-error-driven rings maintain consistent quality across draw
     distances.
   - **Verification:** Render terrain at increasing distances. Assert no seams at LOD boundaries.

## Holes and Materials

4. **R-3.2.4** -- The engine **SHALL** support per-tile 1-bit-per-vertex hole masks that discard
   terrain triangles and mirror the same masks in collision.
   - **Rationale:** Holes enable caves, tunnels, and building entrances through the heightfield.
   - **Verification:** Paint holes. Assert mesh shader discards triangles. Assert physics raycasts
     pass through.

5. **R-3.2.5** -- The engine **SHALL** blend up to 16 PBR material layers per tile using weight
   splatmaps with height-weighted transitions.
   - **Rationale:** Height-weighted blending produces natural material transitions.
   - **Verification:** Configure 16 layers. Assert all render correctly. Assert transitions use
     height weighting.

## Terrain Physics

6. **R-3.2.6** -- The engine **SHALL** derive a collision representation from the heightfield with
   ray and shape query support, streaming with the tile grid and using collision LOD independent of
   visual LOD.
   - **Rationale:** Decoupled collision LOD ensures stable character movement on rough terrain.
   - **Verification:** Walk on steep terrain. Assert stable ground contact. Assert collision updates
     with tile streaming.

## Large World Coordinates

7. **R-3.2.7** -- The engine **SHALL** use 64-bit world coordinates on CPU with camera-relative
   32-bit rendering on GPU, eliminating jitter at distances exceeding 100 km.
   - **Rationale:** f32 drops below 1mm precision at ~16 km; f64 world coordinates are required for
     physics, spatial indices, and multiplayer servers.
   - **Verification:** Place objects at 100 km. Assert no jitter. Assert multiplayer position
     agreement.

## Indoor Environments

8. **R-3.2.8** -- The engine **SHALL** support room-based portal visibility for interiors,
   submitting draw calls only for rooms reachable through visible portal chains.
   - **Rationale:** Portal culling dramatically reduces GPU cost in complex interiors.
   - **Verification:** Build a multi-room interior. Assert only visible rooms receive draw calls.

## Voxel Terrain

9. **R-3.2.9** -- The engine **SHALL** provide a sparse octree voxel volume storing SDF value,
   material ID, and metadata per cell, with empty regions consuming no memory.
   - **Rationale:** Voxels enable caves and underground spaces impossible with heightmaps.
   - **Verification:** Create a cave. Assert correct rendering and material transitions. Assert
     empty regions consume no memory.

10. **R-3.2.10** -- The engine **SHALL** combine heightmap and voxel representations with automatic
    per-region selection, seamless boundary stitching, and continuous physics collision.
    - **Rationale:** Hybrid terrain provides heightmap efficiency with voxel flexibility where
      needed.
    - **Verification:** Walk across a heightmap-voxel boundary. Assert no seams or collision
      discontinuities.

11. **R-3.2.11** -- The engine **SHALL** represent planet surfaces as spherical voxel volumes with
    radial gravity and clipmap LOD scaling with camera distance.
    - **Rationale:** Planetary voxels enable seamless ground-to-orbit transitions for space games.
    - **Verification:** Generate a planet. Walk on the surface. Assert gravity points toward center.
      Fly to orbit. Assert LOD reduces distant terrain.

12. **R-3.2.12** -- The engine **SHALL** support at least 4 meshing algorithms (Marching Cubes, Dual
    Contouring, Surface Nets, Transvoxel) with incremental re-meshing and GPU compute meshing on
    desktop.
    - **Rationale:** Multiple algorithms serve different art styles; incremental meshing enables
      interactive editing.
    - **Verification:** Modify one voxel. Assert only the containing chunk re-meshes. Assert
      crack-free LOD transitions with Transvoxel.

13. **R-3.2.13** -- The engine **SHALL** support runtime voxel modification with incremental
    re-meshing, collision update, and NavMesh invalidation, serialized as compact delta logs with
    server-authoritative multiplayer validation.
    - **Rationale:** Runtime editing enables survival-style terrain manipulation.
    - **Verification:** Dig a tunnel. Assert mesh, collision, and NavMesh update. Save and reload.
      Assert persistence.

14. **R-3.2.14** -- The engine **SHALL** stream voxel octree nodes on demand with distance-based
    LOD, achieving at least 10:1 compression and enforcing memory budgets by evicting distant nodes.
    - **Rationale:** Streaming and compression make planetary voxel worlds feasible.
    - **Verification:** Load a planetary world. Assert memory stays within budget. Assert
      compression exceeds 10:1.

## Multi-Planet Coordinate System

15. **R-3.2.15** -- The engine **SHALL** support three coordinate spaces -- chunk-local,
    planet-local, and universe -- with each planet as a separate ECS world embedded in a
    universe-level Euclidean space, applying a one-time coordinate conversion on inter-planetary
    entity migration.
    - **Rationale:** Multi-planet games lose f32 precision at interstellar distances without a
      three-tier coordinate model.
    - **Verification:** Create two planet worlds in a universe. Migrate an entity between them.
      Assert the position transforms via Planet A to Universe to Planet B. Assert per-frame
      simulation uses only planet-local coordinates.
16. **R-3.2.16** -- The engine **SHALL** subdivide each planet into 6 cube faces, each a flat grid
    of chunks, with the vertex shader projecting chunk-local Cartesian to planet surface via
    normalized cube projection and handling cube-face seam vertices and normals to preserve
    continuity.
    - **Rationale:** Cube-sphere projection enables planetary terrain authoring as flat chunks while
      rendering a continuous sphere.
    - **Verification:** Build a planet with 6 cube faces. Assert the vertex shader curves chunk
      vertices onto the sphere. Assert shared edge vertices at face seams produce identical world
      positions and averaged normals.

## 2D Tilemap Geometry

17. **R-3.2.17** -- The engine **SHALL** store 2D tilemaps as fixed-size chunks of tiles (default
    16x16) organized into multiple layers, with per-chunk dirty flags and distance-based chunk
    streaming.
    - **Rationale:** Chunked storage supports infinite and procedural 2D levels with incremental
      updates rather than full-map rebuilds.
    - **Verification:** Create a multi-layer tilemap. Modify one tile. Assert only its chunk is
      flagged dirty. Move the camera beyond the streaming radius and assert distant chunks unload.
18. **R-3.2.18** -- The engine **SHALL** support auto-tiling that selects tile variants from
    neighbor connectivity via 4-bit cardinal or 8-bit cardinal-plus-diagonal bitmasks, compatible
    with Wang tile and blob tileset conventions.
    - **Rationale:** Designers need to paint terrain regions without manually selecting every
      transition tile.
    - **Verification:** Paint a terrain region. Assert border tiles receive the correct variant from
      the 8-bit bitmask lookup. Assert blob tileset conventions produce seamless borders.
19. **R-3.2.19** -- The engine **SHALL** generate per-chunk compound 2D colliders from tile flags
    (solid, slope, one-way platform), registered in the 2D physics BVH and rebuilt incrementally
    when a chunk's tiles change.
    - **Rationale:** Tilemap collision must stay in sync with visible tiles without separate
      collision authoring.
    - **Verification:** Paint solid and slope tiles. Assert the chunk compound collider contains
      rectangle and triangle colliders. Modify a tile and assert only the affected chunk's collider
      rebuilds.

## Non-Functional Requirements

20. **R-3.2.NF1** -- The engine **SHALL** decode a 257x257 LZ4-compressed heightfield tile within 1
    ms CPU time.
    - **Rationale:** Tile decode must not stall terrain streaming during fast camera movement.
    - **Verification:** Benchmark tile decode. Assert completion within 1 ms.

21. **R-3.2.NF2** -- The engine **SHALL** mesh a 16x16x16 voxel chunk within 5 ms CPU time using any
    supported meshing algorithm.
    - **Rationale:** Interactive voxel editing requires sub-frame mesh regeneration.
    - **Verification:** Modify a voxel and trigger re-mesh. Assert mesh regeneration completes
      within 5 ms.
