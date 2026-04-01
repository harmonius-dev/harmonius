# 3.2 — Terrain

## Heightfield Terrain

| ID      | Feature |
|---------|----------------------------------------------- |
| F-3.2.1 | Heightfield Terrain with Tile-Based Streaming |

1. **F-3.2.1** — The terrain system uses a tile-based heightfield as its core representation. Each
   tile stores a 16-bit or 32-bit height grid at configurable resolution, plus metadata for material
   layers, holes, and physics. Tiles stream in and out based on camera proximity via Tokio async
   I/O. A residency manager tracks loaded, loading, and evicted tiles with placeholder low-LOD
   fallbacks, ensuring seamless transitions during MMO zone traversal.
   - **Platform:** Tokio handles platform I/O internally (IOCP on Windows, kqueue on macOS, epoll on
     Linux).

## Virtual Texturing

| ID      | Feature |
|---------|------------------------- |
| F-3.2.2 | Virtual Texture Clipmap |

1. **F-3.2.2** — Terrain surface materials use a virtual texture clipmap centered on the camera. The
   clipmap maintains nested resolution rings so near-camera terrain has full material detail while
   distant terrain uses coarser pages. A GPU feedback pass identifies which virtual texture pages
   are visible, and missing pages are transcoded and uploaded asynchronously. This bounds VRAM usage
   regardless of world size.
   - **Deps:** F-3.2.1
   - **Platform:** Requires sparse/virtual texture support (VK_EXT_sparse_residency, D3D12 tiled
     resources)

## Terrain LOD

| ID      | Feature |
|---------|------------------------------ |
| F-3.2.3 | CDLOD / Geometry Clipmap LOD |

1. **F-3.2.3** — Terrain meshes use a clipmap-based LOD scheme (CDLOD) where nested rectangular
   rings of decreasing vertex density surround the camera. Morphing zones at ring boundaries
   interpolate vertex heights to prevent visible seams. Screen-space error drives ring sizing so
   that vertex density matches pixel resolution at every distance, maintaining consistent visual
   quality across draw distances of tens of kilometers.
   - **Deps:** F-3.2.1
   - **Platform:** Ring count and vertex density scale per tier. Mobile uses fewer clipmap rings
     with more aggressive LOD; desktop uses more rings for longer draw distance.

## Terrain Holes and Materials

| ID      | Feature |
|---------|---------------------------- |
| F-3.2.4 | Terrain Hole Masking |
| F-3.2.5 | Splatmap Material Blending |

1. **F-3.2.4** — Per-tile bitmask layers mark terrain regions as holes, allowing caves, tunnels,
   building entrances, and dungeon transitions to punch through the heightfield. Hole masks are
   stored as 1-bit-per-vertex grids per tile. The terrain mesh shader discards triangles that
   overlap hole regions, and the collision system mirrors these masks so physics and navigation
   remain consistent.
   - **Deps:** F-3.2.1
   - **Platform:** Hole mask resolution is consistent across platforms (1-bit per vertex). No
     platform-specific scaling required.
2. **F-3.2.5** — Multi-layer terrain materials composited per pixel using weight splatmaps. Each
   material layer provides full PBR textures (albedo, normal, roughness, height). Blending uses
   height-weighted transitions for natural results at layer boundaries (e.g., rock emerging through
   soil). Up to 16 layers per tile are supported, with a runtime indirection table selecting the
   active layer set.
   - **Deps:** F-3.2.1, F-3.2.2
   - **Platform:** Active material layer count scales per tier: mobile 4-8 layers, Switch 8-12,
     desktop up to 16. Height-weighted blending on all tiers.

## Terrain Collision

| ID      | Feature |
|---------|--------------------------- |
| F-3.2.6 | Terrain Physics Collision |

1. **F-3.2.6** — A dedicated terrain collision representation derived from the heightfield that
   supports efficient ray and shape queries. The collision mesh mirrors the streaming tile grid and
   updates as tiles load. Collision LOD matches the physics simulation's needs rather than the
   visual LOD, ensuring stable character movement and vehicle physics on sloped or rough terrain
   across the entire MMO world.
   - **Deps:** F-3.2.1
   - **Platform:** Collision mesh LOD coarsened more aggressively on mobile. Tile load radius
     reduced on memory-constrained platforms.

## Large World Coordinates

| ID      | Feature |
|---------|-------------------------------- |
| F-3.2.7 | Large World Coordinate Support |

1. **F-3.2.7** — All world-space positions use 64-bit floating-point coordinates with
   camera-relative rendering. Before GPU submission, positions are rebased to a camera-origin
   coordinate frame using 32-bit floats, eliminating jitter at large distances from the world
   origin. The coordinate system supports worlds spanning thousands of kilometers, which is
   essential for seamless MMO continental maps and ocean traversal.
   - **Platform:** 64-bit world coordinates on all platforms. Camera-relative f32 rebasing on GPU is
     universal. No platform-specific scaling required.

## Indoor Environments

| ID      | Feature |
|---------|------------------------------------------- |
| F-3.2.8 | Indoor Environments and Portal Visibility |

1. **F-3.2.8** — Room-based rendering for interior environments (dungeons, buildings, space
   stations). Each room is a sealed convex or near-convex volume with portal openings (doors,
   windows, archways) connecting to adjacent rooms. The visibility system traverses portals from the
   camera's room, frustum-culling through each portal opening to determine which rooms and objects
   are visible. Only visible rooms submit draw calls, dramatically reducing GPU cost in complex
   interiors. Rooms support independent lighting environments (a torch-lit dungeon connects to a
   sunlit courtyard through a single portal). Room data is authored in the level editor as portal
   volumes placed at openings. The system integrates with the shared spatial index (F-1.9.7) for
   efficient room-object assignment. Setup)
   - **Deps:** F-3.2.1 (Heightfield Terrain), F-1.9.7 (Rendering Integration), F-2.10.4 (View
   - **Platform:** Portal traversal depth capped per tier: mobile 2-3 portals deep, desktop 6+.
     Reduces overdraw from deeply nested rooms on constrained GPUs.

## Voxel Terrain

| ID       | Feature |
|----------|--------------------------------------- |
| F-3.2.9  | Voxel Volume Representation |
| F-3.2.10 | Hybrid Heightmap-Voxel Terrain |
| F-3.2.11 | Planetary-Scale Voxel Sphere |
| F-3.2.12 | Voxel Meshing Pipeline |
| F-3.2.13 | Runtime Voxel Editing and Deformation |
| F-3.2.14 | Voxel LOD and Streaming |

1. **F-3.2.9** — A sparse voxel volume representation for terrain that supports arbitrary 3D
   geometry: caves, overhangs, arches, tunnels, floating islands, and underground spaces that
   heightmaps cannot represent. Voxels store a signed distance field (SDF) value, material ID, and
   optional metadata (hardness, moisture, temperature) per cell. The volume uses a sparse octree
   (empty regions consume no memory) with configurable resolution per octree node — surface regions
   subdivide to full detail while deep underground solid regions and empty air regions remain
   coarse. Multiple material layers per voxel cell enable smooth terrain material transitions
   (rock/dirt/grass blending at surface boundaries). The voxel system integrates with the heightmap
   terrain (F-3.2.1) via the hybrid mode (F-3.2.10) and with the planetary generation system
   (F-3.6.34) for planet-scale worlds.
   - **Deps:** F-1.7.6 (Memory Budgets)
   - **Platform:** Sparse octree depth is configurable per platform tier. Mobile uses shallower
     trees (lower resolution). Desktop uses full depth.
2. **F-3.2.10** — A dual-representation terrain system that combines heightmap efficiency with voxel
   flexibility. The heightmap (F-3.2.1) serves as the base terrain surface for the majority of the
   world — hills, valleys, mountains, and plains. Voxel volumes (F-3.2.9) overlay the heightmap in
   regions that require 3D geometry: cave systems, mine shafts, cliff overhangs, tunnels through
   mountains, and player-dug holes. The hybrid system automatically determines representation per
   region: pure heightmap where the surface is a simple elevation function, voxel where the surface
   has vertical complexity. The meshing pipeline stitches heightmap and voxel meshes at boundaries
   with seamless LOD transitions. Physics collision uses heightmap collision (F-3.2.6) for heightmap
   regions and voxel mesh collision for voxel regions, both registered in the shared spatial index
   (F-1.9.1). This is the default terrain mode for open-world games (Skyrim, 7 Days to Die, Valheim
   scale). Collision), F-1.9.1 (Shared BVH)
   - **Deps:** F-3.2.1 (Heightfield Terrain), F-3.2.9 (Voxel Volume), F-3.2.6 (Terrain
   - **Platform:** Mobile uses heightmap-only by default; voxel overlays limited to small regions.
     Desktop enables full hybrid across the world.
3. **F-3.2.11** — A voxel representation for entire planets where the terrain wraps around a sphere
   rather than lying on a flat plane. The planet surface is a spherical SDF voxel volume with octree
   subdivision adapted for spherical geometry (icosahedral or cube-sphere projection). Surface
   detail resolution scales with distance from the camera via clipmap-style LOD — near-camera
   terrain has meter-scale voxel resolution while the far side of the planet is represented at
   kilometer-scale. The voxel sphere integrates with the planetary terrain generator (F-3.6.34) for
   continent shapes, the geological simulation (F-3.6.42) for tectonic features, and the
   hydrological system (F-3.6.45) for ocean basins and river valleys. Gravity is radial (toward
   planet center) rather than uniform downward. This mode is used for space exploration games with
   seamless ground-to-orbit transitions. Tectonics), F-3.6.45 (Hydrology)
   - **Deps:** F-3.2.9 (Voxel Volume), F-3.6.34 (Planetary Terrain), F-3.6.42 (Plate
   - **Platform:** Planetary voxel sphere requires significant memory. LOD aggressiveness scales
     with platform tier.
4. **F-3.2.12** — Convert sparse voxel SDF data into renderable triangle meshes using configurable
   meshing algorithms: **Marching Cubes** (smooth isosurface extraction, good for organic terrain),
   **Dual Contouring** (preserves sharp edges for architectural/rocky terrain), **Surface Nets**
   (compromise between smooth and sharp with lower polygon count), and **Transvoxel** (seamless LOD
   transitions between octree levels without cracks or T-junctions). The meshing pipeline runs on
   GPU compute (F-3.6.32) for real-time terrain modification or on CPU worker threads for batch
   generation. Mesh output feeds into the meshlet pipeline (F-3.1.1) for GPU-driven rendering.
   Material assignment uses triplanar mapping with the voxel material IDs driving texture selection.
   Meshing is incremental — only modified chunks are re-meshed, not the entire volume.
   - **Deps:** F-3.2.9 (Voxel Volume), F-3.1.1 (Meshlet Pipeline), F-3.6.32 (GPU Compute)
   - **Platform:** GPU compute meshing on desktop; CPU fallback on mobile. Transvoxel LOD
     transitions are essential for all platforms.
5. **F-3.2.13** — Players and gameplay systems modify terrain voxels at runtime: dig tunnels, carve
   caves, flatten ground for building, raise walls, and sculpt terrain with brush tools. Editing
   operations apply SDF modifications (add/subtract/smooth/flatten/paint material) to the sparse
   voxel volume within a configurable radius and falloff. Modified chunks are re-meshed
   incrementally (F-3.2.12), physics collision is updated (F-3.2.6), and NavMesh tiles are
   invalidated for navigation rebuild (F-7.1.10). Edits are serialized for save/load (F-13.3.1) as a
   compact delta log (recording only modified voxels, not the full volume). In multiplayer, voxel
   edits are replicated as RPC operations (F-8.3.1) with server-authoritative validation (preventing
   players from editing restricted regions). Edit rate is throttled to maintain frame rate — large
   edits are spread across multiple frames. (NavMesh Rebuild), F-13.3.1 (Save System), F-8.3.1
   (RPCs)
   - **Deps:** F-3.2.9 (Voxel Volume), F-3.2.12 (Meshing), F-3.2.6 (Collision), F-7.1.10
   - **Platform:** Edit radius and resolution are scaled per platform. Mobile supports smaller
     edits.
6. **F-3.2.14** — Stream voxel terrain data using the same tile-based streaming infrastructure as
   heightmap terrain (F-3.2.1) but adapted for 3D octree volumes. The streaming system loads octree
   nodes on demand based on camera distance, with coarser LOD nodes loaded first and detail nodes
   streamed in as the player approaches. For planetary-scale voxels (F-3.2.11), only the hemisphere
   facing the camera loads at full resolution; the far hemisphere uses extremely coarse LOD. Voxel
   data is compressed using run-length encoding for material IDs and quantized SDF values, achieving
   10:1 to 50:1 compression ratios for typical terrain. Memory budget enforcement evicts distant
   voxel nodes when the budget is exceeded. Streaming integrates with the sparse cosmic storage
   (F-3.6.63) for planets generated by the universe pipeline. F-1.8.1 (Async I/O)
   - **Deps:** F-3.2.9, F-3.2.11 (Planetary Voxel), F-3.6.63 (Sparse Storage), F-12.5.1 (VFS),
   - **Platform:** Mobile uses aggressive LOD with fewer loaded nodes. Desktop streams more detail.
