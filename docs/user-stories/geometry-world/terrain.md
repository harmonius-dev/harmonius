# User Stories — 3.2 Terrain

## Heightfield Terrain

## US-3.2.1 Stream Terrain Tiles Seamlessly

**As a** player, **I want** terrain tiles to stream in and out based on camera proximity with
low-LOD fallbacks, **so that** I can traverse the open world without loading screens or visible
holes.

## US-3.2.2 Virtual Texture Terrain Materials

**As a** world artist, **I want** terrain materials to use a virtual texture clipmap that keeps
near-camera detail at full resolution, **so that** VRAM usage stays bounded regardless of world
size.

## US-3.2.3 Distance-Based Terrain LOD

**As a** player, **I want** terrain vertex density to match pixel resolution at every distance with
seamless morphing at ring boundaries, **so that** terrain looks sharp nearby and efficient at
distance.

## US-3.2.4 Punch Holes in Terrain

**As a** level designer, **I want** per-tile hole masks that discard terrain triangles and mirror in
collision, **so that** I can create cave entrances, tunnels, and building transitions through the
heightfield.

## US-3.2.5 Blend Terrain Material Layers

**As a** world artist, **I want** to blend up to 16 PBR material layers per tile with
height-weighted transitions, **so that** terrain materials look natural at boundaries.

## US-3.2.6 Stable Terrain Collision

**As a** player, **I want** terrain collision to use a dedicated representation independent of
visual LOD, **so that** character movement and vehicle physics remain stable on slopes.

## US-3.2.7 Explore Vast Worlds Without Jitter

**As a** player, **I want** 64-bit world coordinates with camera-relative rendering, **so that** I
can explore worlds spanning thousands of kilometers without vertex jitter.

## Indoor Environments

## US-3.2.8a Design Indoor Environments with Portals

**As a** level designer, **I want** to place portal volumes at room openings and have the engine
cull rooms not visible through portal chains, **so that** complex interiors render efficiently.

## US-3.2.8b Experience Distinct Interior Lighting

**As a** player, **I want** each room to support its own lighting environment connected through
portals, **so that** a torch-lit dungeon connects naturally to a sunlit courtyard.

## Voxel Terrain

## US-3.2.9 Represent 3D Terrain Geometry

**As a** level designer, **I want** a sparse voxel volume with SDF values and material IDs,
**so that** I can create caves, overhangs, arches, and underground spaces impossible with heightmaps
alone.

## US-3.2.10 Combine Heightmap and Voxel Terrain

**As a** level designer, **I want** the engine to automatically choose heightmap or voxel
representation per region with seamless boundary stitching, **so that** I get heightmap efficiency
for most terrain with voxel flexibility where needed.

## US-3.2.11 Build Entire Planets

**As a** world artist, **I want** a spherical voxel volume with clipmap LOD and radial gravity,
**so that** I can create planets with seamless ground-to-orbit transitions.

## US-3.2.12 Choose Meshing Algorithms

**As a** technical artist, **I want** to select from Marching Cubes, Dual Contouring, Surface Nets,
or Transvoxel meshing with incremental re-meshing, **so that** I can match the meshing style to the
art direction.

## US-3.2.13 Edit Terrain at Runtime

**As a** player, **I want** to dig tunnels, flatten ground, raise walls, and paint terrain materials
with edits saved and replicated in multiplayer, **so that** I can shape the world during gameplay.

## US-3.2.14 Stream Voxel Data Within Memory Budget

**As a** player, **I want** voxel octree nodes to stream on demand with compression and memory
budget enforcement, **so that** planetary-scale voxel worlds are feasible on my hardware.
