# R-15.6 — World Building Requirements

## Terrain Sculpting

### R-15.6.1 Terrain Sculpting Brushes

The engine **SHALL** provide sculpting brushes (raise, lower, smooth, flatten, erode, noise) for
shaping terrain heightmaps in the viewport, with configurable radius, strength, falloff curve, and
shape mask, streaming operations to disk incrementally so that massive terrains do not require
loading the entire heightmap into memory.

- **Derived from:** [F-15.6.1](../../features/tools-editor/world-building.md)
- **Rationale:** Interactive terrain sculpting with incremental disk streaming allows artists to
  shape open-world terrains of arbitrary size without memory constraints.
- **Verification:** Sculpt a 16k x 16k heightmap using each brush type; verify the operation
  completes without loading the full heightmap (peak memory stays below 512 MB). Confirm each
  brush produces visually distinct terrain modifications and that undo restores prior state.

### R-15.6.2 Terrain Erosion

The engine **SHALL** simulate hydraulic and thermal erosion on selected terrain regions with
artist-configurable parameters (rain amount, sediment capacity, thermal angle), previewing
results in real time via GPU compute and committing on brush release.

- **Derived from:** [F-15.6.2](../../features/tools-editor/world-building.md)
- **Rationale:** Procedural erosion produces natural-looking terrain features that would be
  prohibitively time-consuming to sculpt by hand.
- **Verification:** Apply hydraulic erosion to a 2048x2048 sample region; verify the preview
  updates at interactive frame rates (above 15 FPS). Compare pre- and post-erosion heightmaps
  to confirm altitude changes are non-zero and concentrated along drainage paths.

## Terrain Painting and Water

### R-15.6.3 Terrain Material Painting

The engine **SHALL** paint material layers onto terrain tiles with per-layer weight maps, supporting
height-based and slope-based automatic painting rules, triplanar projection for cliff faces, and
macro-variation textures to break tiling, with a bounded layer count per tile to maintain shader
performance.

- **Derived from:** [F-15.6.3](../../features/tools-editor/world-building.md)
- **Rationale:** Multi-layer material painting with automatic rules enables rapid coverage of
  large terrains while maintaining artistic control over local details.
- **Verification:** Paint 4 material layers on a terrain tile; verify weight maps sum to 1.0 at
  every texel. Enable slope-based auto-painting and confirm rock material appears only on slopes
  exceeding the configured angle threshold.

### R-15.6.4 Water Body Placement

The engine **SHALL** place water volumes (rivers, lakes, oceans) by defining boundary splines and
surface elevation, with rivers following spline paths with configurable width, depth, and flow
speed, and lakes filling to a specified altitude with automatic shoreline masking, integrating with
the rendering pipeline for reflections, refraction, caustics, and foam.

- **Derived from:** [F-15.6.4](../../features/tools-editor/world-building.md)
- **Rationale:** Spline-driven water placement gives designers precise control over water body
  shape while automating shoreline blending and rendering integration.
- **Verification:** Place a river spline with 5 control points and a lake at a fixed altitude;
  verify the river follows the spline path and the lake fills to the specified elevation. Confirm
  reflections and refraction render correctly on the water surface.

## Vegetation and Lighting

### R-15.6.5 Vegetation Painting with Density Rules

The engine **SHALL** paint vegetation instances across terrain using density brushes with per-species
placement rules defining slope limits, altitude bands, proximity exclusion radii, clustering
behavior, and random scale/rotation ranges, with a biome rule system for declarative vegetation
distributions that can be auto-populated and then hand-refined.

- **Derived from:** [F-15.6.5](../../features/tools-editor/world-building.md)
- **Rationale:** Rule-based vegetation placement automates populating large regions while biome
  rules ensure ecological consistency across diverse terrain.
- **Verification:** Define a biome with 3 species and altitude/slope constraints; auto-populate a
  1 km^2 region and verify no instance violates its placement rules. Hand-paint additional
  instances and confirm they persist after re-running auto-population.

### R-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

The engine **SHALL** provide placement and configuration tools for light probes on tetrahedral grids
or manual positions and reflection probes with capture volumes and blend distances, supporting
baking, real-time update, and visualization overlays showing probe influence regions.

- **Derived from:** [F-15.6.6](../../features/tools-editor/world-building.md)
- **Rationale:** Probe-based indirect lighting requires precise placement tools so artists can
  ensure correct illumination and reflections across all scene regions.
- **Verification:** Place 10 light probes and 3 reflection probes; bake and verify indirect
  lighting changes are visible in the viewport. Toggle the influence overlay and confirm probe
  volumes render correctly. Move a probe and verify real-time update reflects the change.

## Non-Functional Requirements

### R-15.6.NF1 Terrain Sculpting Responsiveness

Terrain sculpting brush strokes **SHALL** produce visible heightmap changes within one frame (under
33ms) for brush radii up to 256 meters. GPU-accelerated erosion preview **SHALL** maintain above
15 FPS on regions up to 2048x2048 samples. Terrain material painting **SHALL** update the viewport
preview within one frame for brush sizes up to 128 meters. Sculpting on terrains exceeding 16k x
16k samples **SHALL** maintain interactive performance by streaming only the affected region.

- **Derived from:** F-15.6.1 through F-15.6.5 (terrain and vegetation features).
- **Rationale:** Real-time visual feedback during sculpting and painting is essential for artistic
  iteration. Delays between brush stroke and visible result break the direct-manipulation paradigm.
- **Verification:** Sculpt a 16k x 16k heightmap with a 256-meter brush. Measure time from input
  to visible heightmap change and assert under 33ms. Run erosion on a 2048x2048 region and assert
  frame rate stays above 15 FPS. Paint a terrain material with a 128-meter brush and assert the
  preview updates within one frame.

## Navigation and World Partition

### R-15.6.7 Navmesh Preview

The engine **SHALL** render the navigation mesh as a translucent viewport overlay with color-coded
walkable areas, slope limits, and agent radius offsets, supporting real-time navmesh regeneration
for selected regions and displaying pathfinding test results between user-placed start and goal
markers.

- **Derived from:** [F-15.6.7](../../features/tools-editor/world-building.md)
- **Rationale:** Navmesh preview with pathfinding tests lets designers verify AI navigation
  immediately after terrain or geometry edits without leaving the editor.
- **Verification:** Generate a navmesh for a terrain with slopes and obstacles; verify the overlay
  correctly marks steep slopes as non-walkable. Place start and goal markers and confirm the
  displayed path avoids obstacles and non-walkable regions.

### R-15.6.8 World Partition Visualization

The engine **SHALL** display the world partition grid as a 2D minimap and 3D viewport overlay
showing cell boundaries, streaming states (loaded, pending, unloaded), and cell ownership in
multiplayer editing, enabling designers to assess LOD streaming distances and identify cells
exceeding entity or triangle budgets.

- **Derived from:** [F-15.6.8](../../features/tools-editor/world-building.md)
- **Rationale:** World partition visualization is essential for managing large open worlds where
  streaming boundaries, budget overruns, and team coordination must be visible at a glance.
- **Verification:** Load a partitioned world with 20+ cells; verify the minimap and viewport
  overlay display all cells with correct streaming states. Exceed the triangle budget in one cell
  and confirm the visualization flags it with a warning indicator.
