# User Stories: World Building

## F-15.6.1 Terrain Sculpting Brushes

## US-15.6.1.1 Artist Sculpts with Brush Suite
**As an** artist (P-8), **I want** sculpting brushes (raise, lower, smooth, flatten, erode, noise)
with configurable radius, strength, falloff curve, and shape mask, **so that** I can shape terrain
heightmaps directly in the viewport with fine artistic control.

## US-15.6.1.2 Designer Sculpts at World Scale
**As a** designer (P-5), **I want** sculpting operations to stream to disk incrementally, **so
that** I can shape massive open-world terrains without loading the entire heightmap into memory.

## US-15.6.1.3 Tech Artist Customizes Brush Masks
**As a** tech artist (P-13), **I want** to import custom shape masks for sculpting brushes, **so
that** I can create terrain features matching specific reference shapes (e.g., crater stamps,
mountain silhouettes).

## US-15.6.1.4 Engine Tester Validates Streaming Sculpt
**As an** engine tester (P-27), **I want** to verify that incremental disk streaming during
sculpting does not cause visual artifacts or data loss, **so that** artists can sculpt large
terrains confidently.

## F-15.6.2 Terrain Erosion

## US-15.6.2.1 Artist Simulates Hydraulic Erosion
**As an** artist (P-8), **I want** to simulate hydraulic and thermal erosion on selected terrain
regions with adjustable parameters (rain amount, sediment capacity, thermal angle), **so that** I
can produce natural-looking valleys, ridges, and sediment deposits.

## US-15.6.2.2 Designer Previews Erosion in Real Time
**As a** designer (P-5), **I want** erosion results to preview in real time and commit on brush
release, **so that** I can iterate quickly on terrain weathering without waiting for offline
simulation.

## US-15.6.2.3 Engine Dev Validates GPU Compute Erosion
**As an** engine developer (P-26), **I want** the erosion simulation to run on GPU compute, **so
that** real-time feedback is achievable on regions up to 2048x2048 samples without frame drops.

## US-15.6.2.4 Creative Director Reviews Natural Terrain
**As a** creative director (P-2), **I want** erosion tools to produce geologically plausible terrain
formations, **so that** the game world looks convincingly natural rather than hand-sculpted.

## F-15.6.3 Terrain Material Painting

## US-15.6.3.1 Artist Paints Material Layers
**As an** artist (P-8), **I want** to paint material layers (grass, rock, dirt, sand, snow) onto
terrain tiles with per-layer weight maps, **so that** I can blend surface textures naturally across
the landscape.

## US-15.6.3.2 Designer Uses Auto-Paint Rules
**As a** designer (P-5), **I want** height-based and slope-based automatic painting rules with
triplanar projection for cliff faces, **so that** large terrain regions receive sensible material
coverage before manual refinement.

## US-15.6.3.3 Tech Artist Applies Macro Variation
**As a** tech artist (P-13), **I want** macro-variation textures to break tiling on large terrain
areas, **so that** repeated material patterns are not visible from a distance.

## US-15.6.3.4 Engine Tester Validates Layer Bounds
**As an** engine tester (P-27), **I want** to verify that the per-tile layer count stays within the
bounded limit for shader performance, **so that** painting cannot exceed limits that would cause
rendering failures.

## F-15.6.4 Water Body Placement

## US-15.6.4.1 Designer Places Rivers and Lakes
**As a** designer (P-5), **I want** to place water volumes (rivers, lakes, oceans) by defining
boundary splines and water surface elevation, **so that** I can add bodies of water with intuitive
spline-based editing.

## US-15.6.4.2 Artist Configures River Properties
**As an** artist (P-8), **I want** rivers to follow spline paths with configurable width, depth, and
flow speed, **so that** each waterway has unique visual character.

## US-15.6.4.3 Creative Director Reviews Water Rendering
**As a** creative director (P-2), **I want** water bodies to integrate with reflections, refraction,
caustics, and foam in the viewport preview, **so that** I can evaluate water quality during art
direction reviews.

## US-15.6.4.4 Engine Dev Validates Shoreline Masking
**As an** engine developer (P-26), **I want** lakes to fill to a specified altitude with automatic
shoreline masking, **so that** water boundaries match terrain geometry without manual edge painting.

## F-15.6.5 Vegetation Painting with Density Rules

## US-15.6.5.1 Artist Paints Vegetation by Species
**As an** artist (P-8), **I want** to paint vegetation instances across terrain using density
brushes with per-species placement rules for slope limits, altitude bands, and clustering behavior,
**so that** environments look ecologically consistent.

## US-15.6.5.2 Designer Auto-Populates Biomes
**As a** designer (P-5), **I want** a biome rule system that defines vegetation distributions
declaratively so large regions auto-populate, **so that** I can cover square kilometers with
consistent vegetation and then hand-refine specific areas.

## US-15.6.5.3 Tech Artist Adjusts Exclusion Radii
**As a** tech artist (P-13), **I want** proximity exclusion radii and random scale/rotation ranges
per vegetation type, **so that** trees do not overlap and foliage distribution looks natural rather
than uniform.

## US-15.6.5.4 Engine Tester Validates Auto-Population
**As an** engine tester (P-27), **I want** to verify that biome auto-population produces consistent
results given the same seed, **so that** procedural vegetation is deterministic and reproducible
across builds.

## F-15.6.6 Lighting Setup (Light Probes and Reflection Probes)

## US-15.6.6.1 Artist Places Light Probes
**As an** artist (P-8), **I want** to place light probes on a tetrahedral grid or manually and
configure reflection probe capture volumes with blend distances, **so that** global illumination and
reflections are properly sampled throughout the environment.

## US-15.6.6.2 Designer Visualizes Probe Influence
**As a** designer (P-5), **I want** visualization overlays showing probe influence regions in the
viewport, **so that** I can verify that every playable area is covered by appropriate probes without
gaps.

## US-15.6.6.3 Tech Artist Bakes and Updates Probes
**As a** tech artist (P-13), **I want** both baked and real-time probe update modes, **so that** I
can choose between offline quality for static scenes and runtime updates for dynamic lighting
scenarios.

## US-15.6.6.4 Engine Tester Validates Probe Coverage
**As an** engine tester (P-27), **I want** to verify that probe placement produces correct indirect
lighting without dark spots or light bleeding, **so that** global illumination quality meets
production standards.

## F-15.6.7 Navmesh Preview

## US-15.6.7.1 Designer Previews Navigation Mesh
**As a** designer (P-5), **I want** the navigation mesh rendered as a translucent overlay with
color-coded walkable areas, slope limits, and agent radius offsets, **so that** I can verify AI
pathing after terrain or geometry edits.

## US-15.6.7.2 Designer Tests Pathfinding
**As a** designer (P-5), **I want** to place start and goal markers and see pathfinding test results
displayed in the viewport, **so that** I can verify that AI can navigate between key points in the
level.

## US-15.6.7.3 Developer Regenerates Navmesh Locally
**As a** developer (P-15), **I want** real-time navmesh regeneration for a selected region, **so
that** I can verify AI pathing changes immediately without rebuilding the full world navmesh.

## US-15.6.7.4 Engine Tester Validates Navmesh Accuracy
**As an** engine tester (P-27), **I want** to verify that the navmesh preview matches the actual
runtime navigation data, **so that** designers can trust the preview overlay during level design.

## F-15.6.8 World Partition Visualization

## US-15.6.8.1 Designer Views Partition Grid
**As a** designer (P-5), **I want** the world partition grid displayed as a 2D minimap and 3D
viewport overlay showing cell boundaries and streaming states (loaded, pending, unloaded), **so
that** I can understand the spatial organization of the world.

## US-15.6.8.2 DevOps Identifies Over-Budget Cells
**As a** DevOps engineer (P-16), **I want** cells exceeding entity or triangle budgets highlighted
in the partition view, **so that** I can flag performance-critical zones for optimization before
release.

## US-15.6.8.3 Server Admin Views Cell Ownership
**As a** server admin (P-22), **I want** cell ownership displayed in multiplayer editing mode, **so
that** I can coordinate zone assignments across the team and prevent editing conflicts.

## US-15.6.8.4 Engine Tester Validates Streaming Distances
**As an** engine tester (P-27), **I want** to verify that LOD streaming distances shown in the
partition view match runtime behavior, **so that** designers get accurate feedback about which
content is loaded at which distance.
