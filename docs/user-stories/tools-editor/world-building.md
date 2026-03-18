# User Stories: World Building

## F-15.6.1 Terrain Sculpting Brushes

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.6.1.1 | artist (P-8)         |          |              |
| US-15.6.1.2 | designer (P-5)       |          |              |
| US-15.6.1.3 | tech artist (P-13)   |          |              |
| US-15.6.1.4 | engine tester (P-27) |          |              |

1. **US-15.6.1.1** — sculpting brushes (raise, lower, smooth, flatten, erode, noise) with
   configurable radius, strength, falloff curve, and shape mask
   - **Acceptance:** I can shape terrain heightmaps directly in the viewport with fine artistic
     control
2. **US-15.6.1.2** — sculpting operations to stream to disk incrementally
   - **Acceptance:** I can shape massive open-world terrains without loading the entire heightmap
     into memory
3. **US-15.6.1.3** — to import custom shape masks for sculpting brushes
   - **Acceptance:** I can create terrain features matching specific reference shapes (e.g., crater
     stamps, mountain silhouettes)
4. **US-15.6.1.4** — to verify that incremental disk streaming during sculpting does not cause
   visual artifacts or data loss
   - **Acceptance:** artists can sculpt large terrains confidently

## F-15.6.2 Terrain Erosion

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.6.2.1 | artist (P-8)            |          |              |
| US-15.6.2.2 | designer (P-5)          |          |              |
| US-15.6.2.3 | engine developer (P-26) |          |              |
| US-15.6.2.4 | creative director (P-2) |          |              |

1. **US-15.6.2.1** — to simulate hydraulic and thermal erosion on selected terrain regions with
   adjustable parameters (rain amount, sediment capacity, thermal angle)
   - **Acceptance:** I can produce natural-looking valleys, ridges, and sediment deposits
2. **US-15.6.2.2** — erosion results to preview in real time and commit on brush release
   - **Acceptance:** I can iterate quickly on terrain weathering without waiting for offline
     simulation
3. **US-15.6.2.3** — the erosion simulation to run on GPU compute
   - **Acceptance:** real-time feedback is achievable on regions up to 2048x2048 samples without
     frame drops
4. **US-15.6.2.4** — erosion tools to produce geologically plausible terrain formations
   - **Acceptance:** the game world looks convincingly natural rather than hand-sculpted

## F-15.6.3 Terrain Material Painting

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.6.3.1 | artist (P-8)         |          |              |
| US-15.6.3.2 | designer (P-5)       |          |              |
| US-15.6.3.3 | tech artist (P-13)   |          |              |
| US-15.6.3.4 | engine tester (P-27) |          |              |

1. **US-15.6.3.1** — to paint material layers (grass, rock, dirt, sand, snow) onto terrain tiles
   with per-layer weight maps
   - **Acceptance:** I can blend surface textures naturally across the landscape
2. **US-15.6.3.2** — height-based and slope-based automatic painting rules with triplanar projection
   for cliff faces
   - **Acceptance:** large terrain regions receive sensible material coverage before manual
     refinement
3. **US-15.6.3.3** — macro-variation textures to break tiling on large terrain areas
   - **Acceptance:** repeated material patterns are not visible from a distance
4. **US-15.6.3.4** — to verify that the per-tile layer count stays within the bounded limit for
   shader performance
   - **Acceptance:** painting cannot exceed limits that would cause rendering failures

## F-15.6.4 Water Body Placement

| ID          | Persona                 | Features | Requirements |
|-------------|-------------------------|----------|--------------|
| US-15.6.4.1 | designer (P-5)          |          |              |
| US-15.6.4.2 | artist (P-8)            |          |              |
| US-15.6.4.3 | creative director (P-2) |          |              |
| US-15.6.4.4 | engine developer (P-26) |          |              |

1. **US-15.6.4.1** — to place water volumes (rivers, lakes, oceans) by defining boundary splines and
   water surface elevation
   - **Acceptance:** I can add bodies of water with intuitive spline-based editing
2. **US-15.6.4.2** — rivers to follow spline paths with configurable width, depth, and flow speed
   - **Acceptance:** each waterway has unique visual character
3. **US-15.6.4.3** — water bodies to integrate with reflections, refraction, caustics, and foam in
   the viewport preview
   - **Acceptance:** I can evaluate water quality during art direction reviews
4. **US-15.6.4.4** — lakes to fill to a specified altitude with automatic shoreline masking
   - **Acceptance:** water boundaries match terrain geometry without manual edge painting

## F-15.6.5 Vegetation Painting with Density Rules

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.6.5.1 | artist (P-8)         |          |              |
| US-15.6.5.2 | designer (P-5)       |          |              |
| US-15.6.5.3 | tech artist (P-13)   |          |              |
| US-15.6.5.4 | engine tester (P-27) |          |              |

1. **US-15.6.5.1** — to paint vegetation instances across terrain using density brushes with
   per-species placement rules for slope limits, altitude bands, and clustering behavior
   - **Acceptance:** environments look ecologically consistent
2. **US-15.6.5.2** — a biome rule system that defines vegetation distributions declaratively so
   large regions auto-populate
   - **Acceptance:** I can cover square kilometers with consistent vegetation and then hand-refine
     specific areas
3. **US-15.6.5.3** — proximity exclusion radii and random scale/rotation ranges per vegetation type
   - **Acceptance:** trees do not overlap and foliage distribution looks natural rather than uniform
4. **US-15.6.5.4** — to verify that biome auto-population produces consistent results given the same
   seed
   - **Acceptance:** procedural vegetation is deterministic and reproducible across builds

## F-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.6.6.1 | artist (P-8)         |          |              |
| US-15.6.6.2 | designer (P-5)       |          |              |
| US-15.6.6.3 | tech artist (P-13)   |          |              |
| US-15.6.6.4 | engine tester (P-27) |          |              |

1. **US-15.6.6.1** — to place light probes on a tetrahedral grid or manually and configure
   reflection probe capture volumes with blend distances
   - **Acceptance:** global illumination and reflections are properly sampled throughout the
     environment
2. **US-15.6.6.2** — visualization overlays showing probe influence regions in the viewport
   - **Acceptance:** I can verify that every playable area is covered by appropriate probes without
     gaps
3. **US-15.6.6.3** — both baked and real-time probe update modes
   - **Acceptance:** I can choose between offline quality for static scenes and runtime updates for
     dynamic lighting scenarios
4. **US-15.6.6.4** — to verify that probe placement produces correct indirect lighting without dark
   spots or light bleeding
   - **Acceptance:** global illumination quality meets production standards

## F-15.6.7 Navmesh Preview

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-15.6.7.1 | designer (P-5)       |          |              |
| US-15.6.7.2 | designer (P-5)       |          |              |
| US-15.6.7.3 | developer (P-15)     |          |              |
| US-15.6.7.4 | engine tester (P-27) |          |              |

1. **US-15.6.7.1** — the navigation mesh rendered as a translucent overlay with color-coded walkable
   areas, slope limits, and agent radius offsets
   - **Acceptance:** I can verify AI pathing after terrain or geometry edits
2. **US-15.6.7.2** — to place start and goal markers and see pathfinding test results displayed in
   the viewport
   - **Acceptance:** I can verify that AI can navigate between key points in the level
3. **US-15.6.7.3** — real-time navmesh regeneration for a selected region
   - **Acceptance:** I can verify AI pathing changes immediately without rebuilding the full world
     navmesh
4. **US-15.6.7.4** — to verify that the navmesh preview matches the actual runtime navigation data
   - **Acceptance:** designers can trust the preview overlay during level design

## F-15.6.8 World Partition Visualization

| ID          | Persona                | Features | Requirements |
|-------------|------------------------|----------|--------------|
| US-15.6.8.1 | designer (P-5)         |          |              |
| US-15.6.8.2 | DevOps engineer (P-16) |          |              |
| US-15.6.8.3 | server admin (P-22)    |          |              |
| US-15.6.8.4 | engine tester (P-27)   |          |              |

1. **US-15.6.8.1** — the world partition grid displayed as a 2D minimap and 3D viewport overlay
   showing cell boundaries and streaming states (loaded, pending, unloaded)
   - **Acceptance:** I can understand the spatial organization of the world
2. **US-15.6.8.2** — cells exceeding entity or triangle budgets highlighted in the partition view
   - **Acceptance:** I can flag performance-critical zones for optimization before release
3. **US-15.6.8.3** — cell ownership displayed in multiplayer editing mode
   - **Acceptance:** I can coordinate zone assignments across the team and prevent editing conflicts
4. **US-15.6.8.4** — to verify that LOD streaming distances shown in the partition view match
   runtime behavior
   - **Acceptance:** designers get accurate feedback about which content is loaded at which distance

## F-15.6.9 Voxel Sculpting Tools

| ID          | Persona                  | Features | Requirements |
|-------------|--------------------------|----------|--------------|
| US-15.6.9.1 | environment artist (P-8) | F-15.6.9 | R-15.6.9     |
| US-15.6.9.2 | game designer (P-5)      | F-15.6.9 | R-15.6.9     |
| US-15.6.9.3 | effects artist (P-12)    | F-15.6.9 | R-15.6.9     |
| US-15.6.9.4 | game developer (P-15)    | F-15.6.9 | R-15.6.9     |
| US-15.6.9.5 | engine developer (P-26)  | F-15.6.9 | R-15.6.9     |

1. **US-15.6.9.1** — SDF sculpting brushes (add, subtract, smooth, flatten, paint material) for
   editing voxel terrain directly in the viewport
   - **Acceptance:** I can sculpt caves, overhangs, and tunnels with real-time visual feedback
2. **US-15.6.9.2** — voxel sculpting with configurable radius, strength, and falloff to shape
   terrain at any scale
   - **Acceptance:** I can author gameplay spaces like cave systems and cliff paths without
     programmer support
3. **US-15.6.9.3** — sculpting brush operations to trigger VFX preview during editing
   - **Acceptance:** I can see particle effects (dust, chips) as I sculpt, matching what players
     will see at runtime
4. **US-15.6.9.4** — voxel sculpt operations to support undo/redo via a delta log of modified voxels
   - **Acceptance:** gameplay-critical terrain edits can be reverted without losing surrounding
     changes
5. **US-15.6.9.5** — GPU compute meshing to re-mesh modified chunks within the same frame as the
   brush stroke
   - **Acceptance:** sculpting has zero-latency visual feedback at interactive frame rates

## F-15.6.10 Destruction Authoring

| ID           | Persona                  | Features  | Requirements |
|--------------|--------------------------|-----------|--------------|
| US-15.6.10.1 | game designer (P-5)      | F-15.6.10 | R-15.6.10    |
| US-15.6.10.2 | environment artist (P-8) | F-15.6.10 | R-15.6.10    |
| US-15.6.10.3 | effects artist (P-12)    | F-15.6.10 | R-15.6.10    |
| US-15.6.10.4 | game developer (P-15)    | F-15.6.10 | R-15.6.10    |
| US-15.6.10.5 | engine developer (P-26)  | F-15.6.10 | R-15.6.10    |

1. **US-15.6.10.1** — a visual fracture pattern editor with Voronoi seed gizmos that I can add,
   remove, and reposition interactively
   - **Acceptance:** I can author destruction behavior on any object without writing code
2. **US-15.6.10.2** — real-time fracture preview showing fragment geometry, connectivity graph, and
   joint placement overlaid on the object
   - **Acceptance:** I can verify that destruction patterns look correct before play-testing
3. **US-15.6.10.3** — a physics preview sandbox where I can click to apply test impacts and watch
   fragments separate and fall
   - **Acceptance:** I can validate destruction VFX timing and fragment behavior in-editor without
     running the game
4. **US-15.6.10.4** — adjustable fracture parameters (seed count, break threshold, mass
   distribution, debris lifetime) saved as a fracture asset referenced by the Destructible component
   - **Acceptance:** destruction tuning is data-driven and does not require recompilation
5. **US-15.6.10.5** — the destruction authoring tool to generate fracture assets compatible with the
   runtime fracture system (F-4.6.1)
   - **Acceptance:** authored destruction works identically in-editor and at runtime

## F-15.6.11 Voxel-Heightmap Hybrid Editor

| ID           | Persona                  | Features  | Requirements |
|--------------|--------------------------|-----------|--------------|
| US-15.6.11.1 | environment artist (P-8) | F-15.6.11 | R-15.6.11    |
| US-15.6.11.2 | game designer (P-5)      | F-15.6.11 | R-15.6.11    |
| US-15.6.11.3 | effects artist (P-12)    | F-15.6.11 | R-15.6.11    |
| US-15.6.11.4 | game developer (P-15)    | F-15.6.11 | R-15.6.11    |
| US-15.6.11.5 | engine developer (P-26)  | F-15.6.11 | R-15.6.11    |

1. **US-15.6.11.1** — seamless editing across heightmap and voxel regions in the same viewport
   - **Acceptance:** I can sculpt terrain without managing representation switches manually
2. **US-15.6.11.2** — automatic heightmap-to-voxel conversion when I introduce overhangs or caves
   - **Acceptance:** I can create 3D terrain features without understanding the underlying data
     representation
3. **US-15.6.11.3** — visual indicators (outline or tint) showing which regions are heightmap vs.
   voxel
   - **Acceptance:** I can understand where VFX systems will use SDF collision vs. heightmap
     collision for particles
4. **US-15.6.11.4** — material painting to work identically across heightmap and voxel
   representations
   - **Acceptance:** gameplay systems receive consistent material data regardless of terrain
     representation
5. **US-15.6.11.5** — boundary stitching between heightmap and voxel meshes previewed in real time
   - **Acceptance:** LOD transitions at representation boundaries are verified visually before
     shipping

## F-15.6.12 SDF Brush Library

| ID           | Persona                  | Features  | Requirements |
|--------------|--------------------------|-----------|--------------|
| US-15.6.12.1 | environment artist (P-8) | F-15.6.12 | R-15.6.12    |
| US-15.6.12.2 | game designer (P-5)      | F-15.6.12 | R-15.6.12    |
| US-15.6.12.3 | effects artist (P-12)    | F-15.6.12 | R-15.6.12    |
| US-15.6.12.4 | game developer (P-15)    | F-15.6.12 | R-15.6.12    |
| US-15.6.12.5 | engine developer (P-26)  | F-15.6.12 | R-15.6.12    |

1. **US-15.6.12.1** — a library of preset SDF brush shapes (sphere, cube, cylinder, cone, torus,
   noise) in a categorized palette
   - **Acceptance:** I can select the right brush shape quickly without creating custom shapes for
     common operations
2. **US-15.6.12.2** — custom mesh brushes that import an arbitrary mesh and convert it to an SDF for
   sculpting
   - **Acceptance:** I can stamp complex shapes (footprints, runes, architectural details) into
     voxel terrain
3. **US-15.6.12.3** — brush operations to integrate with the effect graph parameter system so VFX
   preview triggers during sculpting
   - **Acceptance:** I can see material-matched particle effects as I apply each brush shape
4. **US-15.6.12.4** — brush presets saved as reusable project assets with per-brush parameter
   defaults
   - **Acceptance:** level generation scripts can apply the same SDF brush shapes programmatically
     at runtime
5. **US-15.6.12.5** — noise brush shapes to expose frequency, amplitude, and octave parameters
   - **Acceptance:** procedural terrain detail is controllable and reproducible across builds
