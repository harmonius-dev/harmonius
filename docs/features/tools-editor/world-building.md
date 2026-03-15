# 15.6 — World Building

## Terrain Sculpting

### F-15.6.1 Terrain Sculpting Brushes

Provides a suite of sculpting brushes for shaping terrain heightmaps directly in the viewport:
raise, lower, smooth, flatten, erode, and noise. Brushes support configurable radius, strength,
falloff curve, and shape mask. Sculpting operations stream to disk incrementally so that artists can
shape massive open-world terrains without loading the entire heightmap into memory.

- **Requirements:** R-15.6.1
- **Dependencies:** F-3.2.1, F-15.1.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.6.2 Terrain Erosion

Simulates hydraulic and thermal erosion on selected terrain regions to produce natural-looking
valleys, ridges, and sediment deposits. Erosion parameters (rain amount, sediment capacity, thermal
angle) are exposed as brush settings for iterative artist control. Results preview in real time and
commit on brush release.

- **Requirements:** R-15.6.2
- **Dependencies:** F-15.6.1
- **Platform notes:** Erosion simulation runs on GPU compute for real-time feedback on regions up to
  2048x2048 samples.

## Terrain Painting and Water

### F-15.6.3 Terrain Material Painting

Paints material layers (grass, rock, dirt, sand, snow) onto terrain tiles with per-layer weight
maps. Supports height-based and slope-based automatic painting rules, triplanar projection for cliff
faces, and macro-variation textures to break tiling. Layer count per tile is bounded to maintain
shader performance.

- **Requirements:** R-15.6.3
- **Dependencies:** F-15.6.1, F-15.3.5
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.6.4 Water Body Placement

Places water volumes (rivers, lakes, oceans) by defining boundary splines and water surface
elevation. Rivers follow spline paths with configurable width, depth, and flow speed. Lakes fill to
a specified altitude with automatic shoreline masking. Water bodies integrate with the rendering
pipeline for reflections, refraction, caustics, and foam generation.

- **Requirements:** R-15.6.4
- **Dependencies:** F-15.2.5, F-2.7.4
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

## Vegetation and Lighting

### F-15.6.5 Vegetation Painting with Density Rules

Paints vegetation instances across terrain using density brushes with per-species placement rules.
Rules define slope limits, altitude bands, proximity exclusion radii, clustering behavior, and
random scale/rotation ranges. A biome rule system allows defining vegetation distributions
declaratively so that large regions can be auto-populated and then hand-refined.

- **Requirements:** R-15.6.5
- **Dependencies:** F-3.3.1, F-15.2.7
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

Provides placement and configuration tools for light probes and reflection probes used by the global
illumination system. Light probes are placed on a tetrahedral grid or manually by the artist.
Reflection probes define capture volumes with blend distances. Both probe types support baking,
real-time update, and visualization overlays showing probe influence regions.

- **Requirements:** R-15.6.6
- **Dependencies:** F-2.5.2, F-2.3.9
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

## Navigation and World Partition

### F-15.6.7 Navmesh Preview

Renders the navigation mesh as a translucent overlay in the viewport with color-coded walkable
areas, slope limits, and agent radius offsets. Supports real-time navmesh regeneration for a
selected region so designers can verify AI pathing after terrain or geometry edits. Displays
pathfinding test results between user-placed start and goal markers.

- **Requirements:** R-15.6.7
- **Dependencies:** F-11.1.1
- **Platform notes:** Desktop only. Not available on mobile or console runtime.

### F-15.6.8 World Partition Visualization

Displays the world partition grid as a 2D minimap and 3D viewport overlay showing cell boundaries,
streaming states (loaded, pending, unloaded), and cell ownership in multiplayer editing. Enables
designers to understand LOD streaming distances, identify cells exceeding entity or triangle
budgets, and coordinate zone assignments across the team. Essential for managing MMO-scale worlds
spanning hundreds of square kilometers.

- **Requirements:** R-15.6.8
- **Dependencies:** F-3.2.3
- **Platform notes:** Desktop only. Not available on mobile or console runtime.
