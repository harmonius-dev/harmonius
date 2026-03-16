# 13.20 — Fog of War

## F-13.20.1 Fog of War Grid System

Grid-based visibility tracking with three states per cell per player/faction: unexplored (fully
hidden), shrouded (previously seen, shows terrain but not entities), and visible (currently seen,
real-time display). The fog grid resolution is independent of the terrain grid. Fog state is
computed per-frame from all active vision sources belonging to the player's faction. The fog texture
is GPU-generated and sampled by terrain, entity, and minimap shaders to darken or hide content. Fog
data is compact (2 bits per cell per faction) for efficient networking in multiplayer. Fog state
integrates with the minimap (F-10.3.6) for progressive world map reveal.

- **Requirements:** R-13.20.1
- **Dependencies:** F-1.9.4 (Spatial Query), F-2.10.1 (ECS-to-Renderer Bridge), F-10.3.6 (Minimap)
- **Platform notes:** GPU compute generates the fog texture each frame. Fallback to CPU on devices
  without compute. Mobile uses lower fog grid resolution (half or quarter) to reduce per-frame
  compute cost.

### F-13.20.2 Vision Source and Sight Radius

Each entity with a `VisionSource` component reveals fog within its sight radius. Configuration per
entity type: radius (units can see further from high ground), shape (360-degree circle or
directional cone), and height advantage bonus (elevation increases effective radius). Line-of-sight
blocking uses the shared spatial index — walls, terrain, and tagged occluders block vision. Multiple
vision sources from the same faction union their revealed areas. Vision updates use a jump-flood or
ray-march algorithm for efficient circle-fill with occlusion.

- **Requirements:** R-13.20.2
- **Dependencies:** F-13.20.1, F-1.9.4 (Spatial Query)
- **Platform notes:** None

### F-13.20.3 Vision Modifier Volumes

World-placed volumes that modify vision behavior: tall grass patches (entities inside are invisible
to entities outside, MOBA-style bush mechanic), smoke clouds (block all vision through the volume),
high ground zones (units inside gain vision radius bonus), and darkness zones (reduce all vision
radii within). Modifiers are ECS entities with trigger volumes (F-4.2.8) and vision modifier
components. Modifiers can be dynamic — smoke grenades create temporary vision-blocking volumes that
dissipate over time.

- **Requirements:** R-13.20.3
- **Dependencies:** F-13.20.1, F-4.2.8 (Trigger Volumes)
- **Platform notes:** None

### F-13.20.4 Fog of War Memory

Shrouded areas display the last-seen state of terrain, buildings, and resource nodes as ghost
images. When a building is destroyed while shrouded, the fog memory retains the building's ghost
until the area is re-revealed, at which point the ghost updates to reflect current state. Memory
snapshots store per-cell: last-seen timestamp, building entity reference, and resource state. Memory
is per-faction and persisted through the save system. This prevents information leaks while giving
players useful contextual information about previously explored areas.

- **Requirements:** R-13.20.4
- **Dependencies:** F-13.20.1, F-13.3.1 (Save System)
- **Platform notes:** None
