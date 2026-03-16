# User Stories: World Building

## F-15.6.1 Terrain Sculpting Brushes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.1.1 | artist (P-8) | sculpting brushes (raise, lower, smooth, flatten, erode, noise) with configurable radius, strength, falloff curve, and shape mask | I can shape terrain heightmaps directly in the viewport with fine artistic control |  |  |
| US-15.6.1.2 | designer (P-5) | sculpting operations to stream to disk incrementally | I can shape massive open-world terrains without loading the entire heightmap into memory |  |  |
| US-15.6.1.3 | tech artist (P-13) | to import custom shape masks for sculpting brushes | I can create terrain features matching specific reference shapes (e.g., crater stamps, mountain silhouettes) |  |  |
| US-15.6.1.4 | engine tester (P-27) | to verify that incremental disk streaming during sculpting does not cause visual artifacts or data loss | artists can sculpt large terrains confidently |  |  |

## F-15.6.2 Terrain Erosion

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.2.1 | artist (P-8) | to simulate hydraulic and thermal erosion on selected terrain regions with adjustable parameters (rain amount, sediment capacity, thermal angle) | I can produce natural-looking valleys, ridges, and sediment deposits |  |  |
| US-15.6.2.2 | designer (P-5) | erosion results to preview in real time and commit on brush release | I can iterate quickly on terrain weathering without waiting for offline simulation |  |  |
| US-15.6.2.3 | engine developer (P-26) | the erosion simulation to run on GPU compute | real-time feedback is achievable on regions up to 2048x2048 samples without frame drops |  |  |
| US-15.6.2.4 | creative director (P-2) | erosion tools to produce geologically plausible terrain formations | the game world looks convincingly natural rather than hand-sculpted |  |  |

## F-15.6.3 Terrain Material Painting

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.3.1 | artist (P-8) | to paint material layers (grass, rock, dirt, sand, snow) onto terrain tiles with per-layer weight maps | I can blend surface textures naturally across the landscape |  |  |
| US-15.6.3.2 | designer (P-5) | height-based and slope-based automatic painting rules with triplanar projection for cliff faces | large terrain regions receive sensible material coverage before manual refinement |  |  |
| US-15.6.3.3 | tech artist (P-13) | macro-variation textures to break tiling on large terrain areas | repeated material patterns are not visible from a distance |  |  |
| US-15.6.3.4 | engine tester (P-27) | to verify that the per-tile layer count stays within the bounded limit for shader performance | painting cannot exceed limits that would cause rendering failures |  |  |

## F-15.6.4 Water Body Placement

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.4.1 | designer (P-5) | to place water volumes (rivers, lakes, oceans) by defining boundary splines and water surface elevation | I can add bodies of water with intuitive spline-based editing |  |  |
| US-15.6.4.2 | artist (P-8) | rivers to follow spline paths with configurable width, depth, and flow speed | each waterway has unique visual character |  |  |
| US-15.6.4.3 | creative director (P-2) | water bodies to integrate with reflections, refraction, caustics, and foam in the viewport preview | I can evaluate water quality during art direction reviews |  |  |
| US-15.6.4.4 | engine developer (P-26) | lakes to fill to a specified altitude with automatic shoreline masking | water boundaries match terrain geometry without manual edge painting |  |  |

## F-15.6.5 Vegetation Painting with Density Rules

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.5.1 | artist (P-8) | to paint vegetation instances across terrain using density brushes with per-species placement rules for slope limits, altitude bands, and clustering behavior | environments look ecologically consistent |  |  |
| US-15.6.5.2 | designer (P-5) | a biome rule system that defines vegetation distributions declaratively so large regions auto-populate | I can cover square kilometers with consistent vegetation and then hand-refine specific areas |  |  |
| US-15.6.5.3 | tech artist (P-13) | proximity exclusion radii and random scale/rotation ranges per vegetation type | trees do not overlap and foliage distribution looks natural rather than uniform |  |  |
| US-15.6.5.4 | engine tester (P-27) | to verify that biome auto-population produces consistent results given the same seed | procedural vegetation is deterministic and reproducible across builds |  |  |

## F-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.6.1 | artist (P-8) | to place light probes on a tetrahedral grid or manually and configure reflection probe capture volumes with blend distances | global illumination and reflections are properly sampled throughout the environment |  |  |
| US-15.6.6.2 | designer (P-5) | visualization overlays showing probe influence regions in the viewport | I can verify that every playable area is covered by appropriate probes without gaps |  |  |
| US-15.6.6.3 | tech artist (P-13) | both baked and real-time probe update modes | I can choose between offline quality for static scenes and runtime updates for dynamic lighting scenarios |  |  |
| US-15.6.6.4 | engine tester (P-27) | to verify that probe placement produces correct indirect lighting without dark spots or light bleeding | global illumination quality meets production standards |  |  |

## F-15.6.7 Navmesh Preview

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.7.1 | designer (P-5) | the navigation mesh rendered as a translucent overlay with color-coded walkable areas, slope limits, and agent radius offsets | I can verify AI pathing after terrain or geometry edits |  |  |
| US-15.6.7.2 | designer (P-5) | to place start and goal markers and see pathfinding test results displayed in the viewport | I can verify that AI can navigate between key points in the level |  |  |
| US-15.6.7.3 | developer (P-15) | real-time navmesh regeneration for a selected region | I can verify AI pathing changes immediately without rebuilding the full world navmesh |  |  |
| US-15.6.7.4 | engine tester (P-27) | to verify that the navmesh preview matches the actual runtime navigation data | designers can trust the preview overlay during level design |  |  |

## F-15.6.8 World Partition Visualization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.8.1 | designer (P-5) | the world partition grid displayed as a 2D minimap and 3D viewport overlay showing cell boundaries and streaming states (loaded, pending, unloaded) | I can understand the spatial organization of the world |  |  |
| US-15.6.8.2 | DevOps engineer (P-16) | cells exceeding entity or triangle budgets highlighted in the partition view | I can flag performance-critical zones for optimization before release |  |  |
| US-15.6.8.3 | server admin (P-22) | cell ownership displayed in multiplayer editing mode | I can coordinate zone assignments across the team and prevent editing conflicts |  |  |
| US-15.6.8.4 | engine tester (P-27) | to verify that LOD streaming distances shown in the partition view match runtime behavior | designers get accurate feedback about which content is loaded at which distance |  |  |

## F-15.6.9 Voxel Sculpting Tools

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.9.1 | environment artist (P-8) | SDF sculpting brushes (add, subtract, smooth, flatten, paint material) for editing voxel terrain directly in the viewport | I can sculpt caves, overhangs, and tunnels with real-time visual feedback | F-15.6.9 | R-15.6.9 |
| US-15.6.9.2 | game designer (P-5) | voxel sculpting with configurable radius, strength, and falloff to shape terrain at any scale | I can author gameplay spaces like cave systems and cliff paths without programmer support | F-15.6.9 | R-15.6.9 |
| US-15.6.9.3 | effects artist (P-12) | sculpting brush operations to trigger VFX preview during editing | I can see particle effects (dust, chips) as I sculpt, matching what players will see at runtime | F-15.6.9 | R-15.6.9 |
| US-15.6.9.4 | game developer (P-15) | voxel sculpt operations to support undo/redo via a delta log of modified voxels | gameplay-critical terrain edits can be reverted without losing surrounding changes | F-15.6.9 | R-15.6.9 |
| US-15.6.9.5 | engine developer (P-26) | GPU compute meshing to re-mesh modified chunks within the same frame as the brush stroke | sculpting has zero-latency visual feedback at interactive frame rates | F-15.6.9 | R-15.6.9 |

## F-15.6.10 Destruction Authoring

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.10.1 | game designer (P-5) | a visual fracture pattern editor with Voronoi seed gizmos that I can add, remove, and reposition interactively | I can author destruction behavior on any object without writing code | F-15.6.10 | R-15.6.10 |
| US-15.6.10.2 | environment artist (P-8) | real-time fracture preview showing fragment geometry, connectivity graph, and joint placement overlaid on the object | I can verify that destruction patterns look correct before play-testing | F-15.6.10 | R-15.6.10 |
| US-15.6.10.3 | effects artist (P-12) | a physics preview sandbox where I can click to apply test impacts and watch fragments separate and fall | I can validate destruction VFX timing and fragment behavior in-editor without running the game | F-15.6.10 | R-15.6.10 |
| US-15.6.10.4 | game developer (P-15) | adjustable fracture parameters (seed count, break threshold, mass distribution, debris lifetime) saved as a fracture asset referenced by the Destructible component | destruction tuning is data-driven and does not require recompilation | F-15.6.10 | R-15.6.10 |
| US-15.6.10.5 | engine developer (P-26) | the destruction authoring tool to generate fracture assets compatible with the runtime fracture system (F-4.6.1) | authored destruction works identically in-editor and at runtime | F-15.6.10 | R-15.6.10 |

## F-15.6.11 Voxel-Heightmap Hybrid Editor

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.11.1 | environment artist (P-8) | seamless editing across heightmap and voxel regions in the same viewport | I can sculpt terrain without managing representation switches manually | F-15.6.11 | R-15.6.11 |
| US-15.6.11.2 | game designer (P-5) | automatic heightmap-to-voxel conversion when I introduce overhangs or caves | I can create 3D terrain features without understanding the underlying data representation | F-15.6.11 | R-15.6.11 |
| US-15.6.11.3 | effects artist (P-12) | visual indicators (outline or tint) showing which regions are heightmap vs. voxel | I can understand where VFX systems will use SDF collision vs. heightmap collision for particles | F-15.6.11 | R-15.6.11 |
| US-15.6.11.4 | game developer (P-15) | material painting to work identically across heightmap and voxel representations | gameplay systems receive consistent material data regardless of terrain representation | F-15.6.11 | R-15.6.11 |
| US-15.6.11.5 | engine developer (P-26) | boundary stitching between heightmap and voxel meshes previewed in real time | LOD transitions at representation boundaries are verified visually before shipping | F-15.6.11 | R-15.6.11 |

## F-15.6.12 SDF Brush Library

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-15.6.12.1 | environment artist (P-8) | a library of preset SDF brush shapes (sphere, cube, cylinder, cone, torus, noise) in a categorized palette | I can select the right brush shape quickly without creating custom shapes for common operations | F-15.6.12 | R-15.6.12 |
| US-15.6.12.2 | game designer (P-5) | custom mesh brushes that import an arbitrary mesh and convert it to an SDF for sculpting | I can stamp complex shapes (footprints, runes, architectural details) into voxel terrain | F-15.6.12 | R-15.6.12 |
| US-15.6.12.3 | effects artist (P-12) | brush operations to integrate with the effect graph parameter system so VFX preview triggers during sculpting | I can see material-matched particle effects as I apply each brush shape | F-15.6.12 | R-15.6.12 |
| US-15.6.12.4 | game developer (P-15) | brush presets saved as reusable project assets with per-brush parameter defaults | level generation scripts can apply the same SDF brush shapes programmatically at runtime | F-15.6.12 | R-15.6.12 |
| US-15.6.12.5 | engine developer (P-26) | noise brush shapes to expose frequency, amplitude, and octave parameters | procedural terrain detail is controllable and reproducible across builds | F-15.6.12 | R-15.6.12 |
