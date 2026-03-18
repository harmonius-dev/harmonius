# User Stories — 3.2 Terrain

## Heightfield Terrain

| ID       | Persona        | Features | Requirements |
|----------|----------------|----------|--------------|
| US-3.2.1 | player         |          |              |
| US-3.2.2 | world artist   |          |              |
| US-3.2.3 | player         |          |              |
| US-3.2.4 | level designer |          |              |
| US-3.2.5 | world artist   |          |              |
| US-3.2.6 | player         |          |              |
| US-3.2.7 | player         |          |              |

1. **US-3.2.1** — I want terrain tiles to stream in and out based on camera proximity with low-LOD
   fallbacks
   - **Acceptance:** I can traverse the open world without loading screens or visible holes
2. **US-3.2.2** — I want terrain materials to use a virtual texture clipmap that keeps near-camera
   detail at full resolution
   - **Acceptance:** VRAM usage stays bounded regardless of world size
3. **US-3.2.3** — I want terrain vertex density to match pixel resolution at every distance with
   seamless morphing at ring boundaries
   - **Acceptance:** terrain looks sharp nearby and efficient at distance
4. **US-3.2.4** — I want per-tile hole masks that discard terrain triangles and mirror in collision
   - **Acceptance:** I can create cave entrances, tunnels, and building transitions through the
     heightfield
5. **US-3.2.5** — I want to blend up to 16 PBR material layers per tile with height-weighted
   transitions
   - **Acceptance:** terrain materials look natural at boundaries
6. **US-3.2.6** — I want terrain collision to use a dedicated representation independent of visual
   LOD
   - **Acceptance:** character movement and vehicle physics remain stable on slopes
7. **US-3.2.7** — I want 64-bit world coordinates with camera-relative rendering
   - **Acceptance:** I can explore worlds spanning thousands of kilometers without vertex jitter

## Indoor Environments

| ID        | Persona        | Features | Requirements |
|-----------|----------------|----------|--------------|
| US-3.2.8a | level designer |          |              |
| US-3.2.8b | player         |          |              |

1. **US-3.2.8a** — I want to place portal volumes at room openings and have the engine cull rooms
   not visible through portal chains
   - **Acceptance:** complex interiors render efficiently
2. **US-3.2.8b** — I want each room to support its own lighting environment connected through
   portals
   - **Acceptance:** a torch-lit dungeon connects naturally to a sunlit courtyard

## Voxel Terrain

| ID        | Persona          | Features | Requirements |
|-----------|------------------|----------|--------------|
| US-3.2.9  | level designer   |          |              |
| US-3.2.10 | level designer   |          |              |
| US-3.2.11 | world artist     |          |              |
| US-3.2.12 | technical artist |          |              |
| US-3.2.13 | player           |          |              |
| US-3.2.14 | player           |          |              |

1. **US-3.2.9** — I want a sparse voxel volume with SDF values and material IDs
   - **Acceptance:** I can create caves, overhangs, arches, and underground spaces impossible with
     heightmaps alone
2. **US-3.2.10** — I want the engine to automatically choose heightmap or voxel representation per
   region with seamless boundary stitching
   - **Acceptance:** I get heightmap efficiency for most terrain with voxel flexibility where needed
3. **US-3.2.11** — I want a spherical voxel volume with clipmap LOD and radial gravity
   - **Acceptance:** I can create planets with seamless ground-to-orbit transitions
4. **US-3.2.12** — I want to select from Marching Cubes, Dual Contouring, Surface Nets, or
   Transvoxel meshing with incremental re-meshing
   - **Acceptance:** I can match the meshing style to the art direction
5. **US-3.2.13** — I want to dig tunnels, flatten ground, raise walls, and paint terrain materials
   with edits saved and replicated in multiplayer
   - **Acceptance:** I can shape the world during gameplay
6. **US-3.2.14** — I want voxel octree nodes to stream on demand with compression and memory budget
   enforcement
   - **Acceptance:** planetary-scale voxel worlds are feasible on my hardware
