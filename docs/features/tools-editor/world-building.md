# 15.6 — World Building

## Terrain Sculpting

| ID | Feature |
| ---------- | --------------------------- |
| F-15.6.1 | Terrain Sculpting Brushes |
| F-15.6.2 | Terrain Erosion |

1. **F-15.6.1** — Provides a suite of sculpting brushes for shaping terrain heightmaps directly in
   the viewport: raise, lower, smooth, flatten, erode, and noise. Brushes support configurable
   radius, strength, falloff curve, and shape mask. Sculpting operations stream to disk
   incrementally so that artists can shape massive open-world terrains without loading the entire
   heightmap into memory.
   - **Deps:** F-3.2.1, F-15.1.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.6.2** — Simulates hydraulic and thermal erosion on selected terrain regions to produce
   natural-looking valleys, ridges, and sediment deposits. Erosion parameters (rain amount, sediment
   capacity, thermal angle) are exposed as brush settings for iterative artist control. Results
   preview in real time and commit on brush release. 2048x2048 samples.
   - **Deps:** F-15.6.1
   - **Platform:** Erosion simulation runs on GPU compute for real-time feedback on regions up to

## Terrain Painting and Water

| ID | Feature |
| ---------- | --------------------------- |
| F-15.6.3 | Terrain Material Painting |
| F-15.6.4 | Water Body Placement |

1. **F-15.6.3** — Paints material layers (grass, rock, dirt, sand, snow) onto terrain tiles with
   per-layer weight maps. Supports height-based and slope-based automatic painting rules, triplanar
   projection for cliff faces, and macro-variation textures to break tiling. Layer count per tile is
   bounded to maintain shader performance.
   - **Deps:** F-15.6.1, F-15.3.5
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.6.4** — Places water volumes (rivers, lakes, oceans) by defining boundary splines and
   water surface elevation. Rivers follow spline paths with configurable width, depth, and flow
   speed. Lakes fill to a specified altitude with automatic shoreline masking. Water bodies
   integrate with the rendering pipeline for reflections, refraction, caustics, and foam generation.
   - **Deps:** F-15.2.5, F-2.7.4
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Vegetation and Lighting

| ID | Feature |
| ---------- | ----------------------------------------------------- |
| F-15.6.5 | Vegetation Painting with Density Rules |
| F-15.6.6 | Lighting Setup (Light Probes and Reflection Probes) |

1. **F-15.6.5** — Paints vegetation instances across terrain using density brushes with per-species
   placement rules. Rules define slope limits, altitude bands, proximity exclusion radii, clustering
   behavior, and random scale/rotation ranges. A biome rule system allows defining vegetation
   distributions declaratively so that large regions can be auto-populated and then hand-refined.
   - **Deps:** F-3.3.1, F-15.2.7
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.6.6** — Provides placement and configuration tools for light probes and reflection probes
   used by the global illumination system. Light probes are placed on a tetrahedral grid or manually
   by the artist. Reflection probes define capture volumes with blend distances. Both probe types
   support baking, real-time update, and visualization overlays showing probe influence regions.
   - **Deps:** F-2.5.2, F-2.3.9
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Navigation and World Partition

| ID | Feature |
| ---------- | ------------------------------- |
| F-15.6.7 | Navmesh Preview |
| F-15.6.8 | World Partition Visualization |

1. **F-15.6.7** — Renders the navigation mesh as a translucent overlay in the viewport with
   color-coded walkable areas, slope limits, and agent radius offsets. Supports real-time navmesh
   regeneration for a selected region so designers can verify AI pathing after terrain or geometry
   edits. Displays pathfinding test results between user-placed start and goal markers.
   - **Deps:** F-11.1.1
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.6.8** — Displays the world partition grid as a 2D minimap and 3D viewport overlay showing
   cell boundaries, streaming states (loaded, pending, unloaded), and cell ownership in multiplayer
   editing. Enables designers to understand LOD streaming distances, identify cells exceeding entity
   or triangle budgets, and coordinate zone assignments across the team. Essential for managing
   MMO-scale worlds spanning hundreds of square kilometers.
   - **Deps:** F-3.2.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.

## Voxel Terrain Editing

| ID | Feature |
| ----------- | ------------------------------- |
| F-15.6.9 | Voxel Sculpting Tools |
| F-15.6.10 | Destruction Authoring |
| F-15.6.11 | Voxel-Heightmap Hybrid Editor |
| F-15.6.12 | SDF Brush Library |

1. **F-15.6.9** — SDF-based sculpting brushes for editing voxel terrain volumes (F-3.2.9) directly
   in the editor viewport. Brush operations include add (deposit material), subtract (excavate),
   smooth (reduce surface noise), flatten (level to a reference plane), and paint material (assign
   voxel material ID without changing geometry). Each brush evaluates an SDF primitive (sphere,
   cube, cylinder, or custom shape from F-15.6.12) and applies it to the sparse octree volume.
   Real-time preview is powered by GPU compute meshing (F-3.2.12) — modified chunks are re-meshed
   and displayed within the same frame as the brush stroke. Brush parameters include radius,
   strength, falloff curve, and material ID. Sculpting operations support undo/redo via a delta log
   of modified voxels. In hybrid terrain mode (F-3.2.10), sculpting automatically converts heightmap
   regions to voxel when the edit introduces vertical complexity.
   - **Deps:** F-3.2.9, F-3.2.12, F-3.2.13, F-3.2.10, F-15.1.3
   - **Platform:** Desktop only. Not available on mobile or console runtime.
2. **F-15.6.10** — A visual fracture pattern editor integrated into the level editor viewport for
   authoring destruction behavior on destructible objects. The editor displays Voronoi fracture
   seeds as gizmos in the viewport, allowing the artist to add, remove, and reposition seeds
   interactively. A real-time fracture preview shows the resulting fragment geometry, connectivity
   graph, and joint placement overlaid on the object. The artist can simulate destruction by
   clicking to apply a test impact at any point on the object surface, watching fragments separate
   and fall in a physics preview sandbox without affecting the level state. Adjustable parameters
   include Voronoi seed count, break threshold per joint, fragment mass distribution, and debris
   lifetime. The authored fracture configuration is saved as a fracture asset (F-4.6.1) referenced
   by the `Destructible` component.
   - **Deps:** F-4.6.1, F-4.6.2, F-4.6.3, F-15.1.3, F-15.1.5
   - **Platform:** Desktop only. Not available on mobile or console runtime.
3. **F-15.6.11** — Seamless editing across heightmap and voxel regions in the same viewport using
   the hybrid terrain system (F-3.2.10). When the artist sculpts a heightmap region and introduces
   vertical complexity (overhangs, caves, tunnels), the editor automatically converts that region to
   voxel representation and continues sculpting in voxel mode. Conversely, flattening a voxel region
   back to a simple elevation surface offers to convert it back to heightmap for efficiency. The
   editor displays a visual indicator (outline or tint) showing which regions are heightmap vs.
   voxel. Boundary stitching between representations is previewed in real time so the artist can
   verify seamless transitions. Material painting works identically across both representations.
   - **Deps:** F-3.2.1, F-3.2.9, F-3.2.10, F-15.6.1, F-15.6.9
   - **Platform:** Desktop only. Not available on mobile or console runtime.
4. **F-15.6.12** — A library of preset SDF brush shapes for voxel sculpting (F-15.6.9): sphere,
   cube, cylinder, cone, torus, noise (Perlin, Worley, simplex), and custom shape from mesh. Each
   preset exposes brush-specific parameters — sphere has radius, cube has dimensions and corner
   rounding, noise has frequency/amplitude/octaves, and custom mesh brushes import an arbitrary mesh
   and convert it to an SDF for sculpting. Brush presets are saved as reusable assets in the project
   and appear in a categorized palette in the sculpting toolbar. Artists can create, name, and share
   custom brush presets with per-brush parameter defaults. The brush library integrates with the
   effect graph parameter system (F-11.6.3) so that brush operations can trigger VFX preview during
   sculpting.
   - **Deps:** F-15.6.9, F-3.2.9
   - **Platform:** Desktop only. Not available on mobile or console runtime.
