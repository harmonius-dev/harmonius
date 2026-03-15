# R-3.2 — Terrain

## R-3.2.1 Heightfield Terrain with Tile-Based Streaming

The engine **SHALL** represent terrain as a tile-based heightfield with 16-bit or 32-bit height grids,
streaming tiles in and out based on camera proximity via platform-native async I/O, with low-LOD fallbacks
for unloaded tiles.

- **Derived from:** [F-3.2.1](../../features/geometry-world/terrain.md)
- **Rationale:** Tile-based streaming with residency management enables seamless traversal of MMO-scale worlds
  without loading screens or visible holes.
- **Verification:** Traverse a world larger than VRAM capacity and confirm tiles stream seamlessly, fallback
  LODs are displayed for unloaded tiles, and async I/O uses the platform-native API (IOCP / GCD / io_uring).

## R-3.2.2 Virtual Texture Clipmap

The engine **SHALL** composite terrain surface materials using a virtual texture clipmap centered on the camera,
with nested resolution rings and GPU feedback-driven page loading.

- **Derived from:** [F-3.2.2](../../features/geometry-world/terrain.md)
- **Rationale:** Virtual texture clipmaps bound VRAM usage regardless of world size while maintaining full
  material detail near the camera.
- **Verification:** Pan across a large terrain and confirm near-camera textures render at full resolution,
  distant textures use coarser pages, VRAM usage stays bounded, and missing pages load asynchronously without
  visual corruption.

## R-3.2.3 CDLOD / Geometry Clipmap LOD

The engine **SHALL** tessellate terrain using clipmap-based LOD rings with vertex morphing at ring boundaries,
sizing rings so vertex density matches pixel resolution at every distance.

- **Derived from:** [F-3.2.3](../../features/geometry-world/terrain.md)
- **Rationale:** Screen-space-error-driven clipmap rings maintain consistent visual quality across draw
  distances of tens of kilometers while bounding vertex count.
- **Verification:** Render terrain at increasing distances and confirm no visible seams at LOD boundaries,
  vertex density decreases proportionally with distance, and screen-space error stays within threshold.

## R-3.2.4 Terrain Hole Masking

The engine **SHALL** support per-tile 1-bit-per-vertex hole masks that discard terrain triangles in marked
regions and mirror the same masks in the collision system.

- **Derived from:** [F-3.2.4](../../features/geometry-world/terrain.md)
- **Rationale:** Hole masks enable caves, tunnels, and building entrances to punch through the heightfield
  while keeping rendering and physics consistent.
- **Verification:** Paint holes on a terrain tile and confirm the mesh shader discards affected triangles,
  physics raycasts pass through the same regions, and no collision occurs in hole areas.

## R-3.2.5 Splatmap Material Blending

The engine **SHALL** blend up to 16 PBR material layers per tile using weight splatmaps with height-weighted
transitions at layer boundaries.

- **Derived from:** [F-3.2.5](../../features/geometry-world/terrain.md)
- **Rationale:** Height-weighted splatmap blending produces natural material transitions (e.g., rock through
  soil) that are visually superior to linear alpha blending.
- **Verification:** Configure a tile with 16 material layers and confirm all layers render correctly,
  transitions use height-weighted blending, and the runtime indirection table selects the active layer set.

## R-3.2.6 Terrain Physics Collision

The engine **SHALL** derive a collision representation from the heightfield that supports ray and shape queries,
streams with the tile grid, and uses a collision LOD independent of the visual LOD.

- **Derived from:** [F-3.2.6](../../features/geometry-world/terrain.md)
- **Rationale:** Decoupling collision LOD from visual LOD ensures stable character movement and vehicle
  physics on rough terrain without being constrained by rendering detail levels.
- **Verification:** Simulate a character walking on steep terrain and confirm stable ground contact, perform
  raycasts against loaded and streaming tiles, and verify collision mesh updates as tiles load/unload.

## R-3.2.7 Large World Coordinate Support

The engine **SHALL** use 64-bit floating-point world-space coordinates with camera-relative rendering, rebasing
positions to a 32-bit camera-origin frame before GPU submission.

- **Derived from:** [F-3.2.7](../../features/geometry-world/terrain.md)
- **Rationale:** 64-bit coordinates with camera-relative rendering eliminate floating-point jitter at large
  distances, supporting worlds spanning thousands of kilometers.
- **Verification:** Place objects at distances exceeding 10 km from the origin and confirm no vertex jitter,
  rendering artifacts, or precision loss in physics at those coordinates.

## R-3.2.8 Indoor Environments and Portal Visibility

The engine **SHALL** support room-based rendering for interior environments with portal openings connecting
adjacent rooms, traversing portals from the camera's room to frustum-cull and submit draw calls only for
visible rooms.

- **Derived from:** [F-3.2.8](../../features/geometry-world/terrain.md)
- **Rationale:** Portal-based visibility dramatically reduces GPU cost in complex interiors by limiting draw
  calls to rooms reachable through visible portal chains.
- **Verification:** Build a multi-room interior with portal connections and confirm only rooms visible through
  portal chains receive draw calls, rooms behind closed portals are fully culled, and independent lighting
  environments are maintained per room.

## R-3.2.9 Voxel Volume Representation

The engine **SHALL** provide a sparse octree voxel volume storing SDF value, material ID, and
metadata per cell. Empty regions **SHALL** consume no memory. Surface regions **SHALL**
subdivide to configurable resolution (1m default on desktop, 2m on mobile). The volume
**SHALL** support multiple material layers per cell with smooth blending.

- **Derived from:** [F-3.2.9](../../features/geometry-world/terrain.md)
- **Rationale:** Voxels enable caves, overhangs, and underground spaces impossible with
  heightmaps alone.
- **Verification:** Create a cave in voxel terrain; verify it renders correctly with material
  transitions. Verify empty regions consume zero additional memory beyond the octree node.

## R-3.2.10 Hybrid Heightmap-Voxel Terrain

The engine **SHALL** combine heightmap and voxel representations in a single terrain system
with automatic representation selection per region. Boundary meshes **SHALL** stitch
seamlessly with no visible cracks or T-junctions. Physics collision **SHALL** work correctly
across both representations without discontinuities at boundaries.

- **Derived from:** [F-3.2.10](../../features/geometry-world/terrain.md)
- **Rationale:** Hybrid terrain provides heightmap efficiency for most of the world with voxel
  flexibility where needed, enabling open-world games with caves and tunnels.
- **Verification:** Place a cave entrance in heightmap terrain transitioning to a voxel cave;
  verify no seams at the boundary. Walk a character across the boundary; verify collision is
  continuous.

## R-3.2.11 Planetary-Scale Voxel Sphere

The engine **SHALL** represent entire planet surfaces as spherical voxel volumes with radial
gravity. Surface resolution **SHALL** scale with camera distance via clipmap LOD. The system
**SHALL** integrate with the planetary generation pipeline (F-3.6.34) for continent shapes
and geological features. Near-camera terrain **SHALL** have meter-scale resolution while the
antipodal hemisphere uses kilometer-scale LOD.

- **Derived from:** [F-3.2.11](../../features/geometry-world/terrain.md)
- **Rationale:** Planetary voxel spheres enable seamless ground-to-orbit transitions for space
  exploration games.
- **Verification:** Generate a planet; walk on the surface; verify gravity points toward center.
  Fly to orbit; verify LOD reduces distant terrain. Verify caves render on the curved surface.

## R-3.2.12 Voxel Meshing Pipeline

The engine **SHALL** support at least 4 meshing algorithms (Marching Cubes, Dual Contouring,
Surface Nets, Transvoxel). Meshing **SHALL** be incremental — only modified chunks are
re-meshed. Transvoxel **SHALL** produce crack-free LOD transitions. GPU compute meshing
**SHALL** be available on desktop with CPU fallback on mobile. Meshed output **SHALL** feed
into the meshlet pipeline (F-3.1.1). A 16x16x16 chunk **SHALL** re-mesh in under 2ms on
desktop GPU compute.

- **Derived from:** [F-3.2.12](../../features/geometry-world/terrain.md)
- **Rationale:** Multiple meshing algorithms serve different art styles; incremental meshing
  ensures interactive editing performance.
- **Verification:** Modify a single voxel; verify only the containing chunk re-meshes. Verify
  LOD transitions have no visible cracks with Transvoxel. Benchmark chunk re-mesh time.

## R-3.2.13 Runtime Voxel Editing and Deformation

The engine **SHALL** support runtime voxel modification (dig, build, smooth, flatten, paint
material) with incremental re-meshing, collision update, and NavMesh invalidation within the
same frame. Edits **SHALL** be serialized as compact delta logs for save/load (under 1KB per
edit operation). Multiplayer edits **SHALL** be server-authoritative with restricted-zone
validation. Edit rate **SHALL** be throttled to maintain the target frame rate.

- **Derived from:** [F-3.2.13](../../features/geometry-world/terrain.md)
- **Rationale:** Runtime terrain editing enables Minecraft/7-Days-to-Die style gameplay and
  survival base building.
- **Verification:** Dig a tunnel; verify mesh, collision, and NavMesh update within 1 frame.
  Save and reload; verify the tunnel persists. In multiplayer, verify server rejects edits
  in restricted zones.

## R-3.2.14 Voxel LOD and Streaming

The engine **SHALL** stream voxel octree nodes on demand with distance-based LOD. Compressed
voxel data **SHALL** achieve at least 10:1 compression ratio. Memory budget enforcement
**SHALL** evict distant nodes when the budget is exceeded. For planetary voxels, only the
camera-facing hemisphere **SHALL** load at full resolution. Streaming **SHALL** integrate
with the sparse cosmic storage (F-3.6.63) for universe-generated planets.

- **Derived from:** [F-3.2.14](../../features/geometry-world/terrain.md)
- **Rationale:** Streaming and LOD make planetary-scale voxel worlds feasible within memory
  constraints on all platform tiers.
- **Verification:** Load a planetary voxel world; verify memory stays within budget. Verify
  compression ratio exceeds 10:1. Verify the far hemisphere uses coarse LOD.
