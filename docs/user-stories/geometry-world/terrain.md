# User Stories — 3.2 Terrain

## Heightfield Terrain

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.2.1 | player | I want terrain tiles to stream in and out based on camera proximity with low-LOD fallbacks | I can traverse the open world without loading screens or visible holes |  |  |
| US-3.2.2 | world artist | I want terrain materials to use a virtual texture clipmap that keeps near-camera detail at full resolution | VRAM usage stays bounded regardless of world size |  |  |
| US-3.2.3 | player | I want terrain vertex density to match pixel resolution at every distance with seamless morphing at ring boundaries | terrain looks sharp nearby and efficient at distance |  |  |
| US-3.2.4 | level designer | I want per-tile hole masks that discard terrain triangles and mirror in collision | I can create cave entrances, tunnels, and building transitions through the heightfield |  |  |
| US-3.2.5 | world artist | I want to blend up to 16 PBR material layers per tile with height-weighted transitions | terrain materials look natural at boundaries |  |  |
| US-3.2.6 | player | I want terrain collision to use a dedicated representation independent of visual LOD | character movement and vehicle physics remain stable on slopes |  |  |
| US-3.2.7 | player | I want 64-bit world coordinates with camera-relative rendering | I can explore worlds spanning thousands of kilometers without vertex jitter |  |  |

## Indoor Environments

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.2.8a | level designer | I want to place portal volumes at room openings and have the engine cull rooms not visible through portal chains | complex interiors render efficiently |  |  |
| US-3.2.8b | player | I want each room to support its own lighting environment connected through portals | a torch-lit dungeon connects naturally to a sunlit courtyard |  |  |

## Voxel Terrain

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-3.2.9 | level designer | I want a sparse voxel volume with SDF values and material IDs | I can create caves, overhangs, arches, and underground spaces impossible with heightmaps alone |  |  |
| US-3.2.10 | level designer | I want the engine to automatically choose heightmap or voxel representation per region with seamless boundary stitching | I get heightmap efficiency for most terrain with voxel flexibility where needed |  |  |
| US-3.2.11 | world artist | I want a spherical voxel volume with clipmap LOD and radial gravity | I can create planets with seamless ground-to-orbit transitions |  |  |
| US-3.2.12 | technical artist | I want to select from Marching Cubes, Dual Contouring, Surface Nets, or Transvoxel meshing with incremental re-meshing | I can match the meshing style to the art direction |  |  |
| US-3.2.13 | player | I want to dig tunnels, flatten ground, raise walls, and paint terrain materials with edits saved and replicated in multiplayer | I can shape the world during gameplay |  |  |
| US-3.2.14 | player | I want voxel octree nodes to stream on demand with compression and memory budget enforcement | planetary-scale voxel worlds are feasible on my hardware |  |  |
