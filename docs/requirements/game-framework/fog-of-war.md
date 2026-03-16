# R-13.20 -- Fog of War Requirements

## R-13.20.1 Fog of War Grid System

The engine **SHALL** track per-cell, per-faction visibility in three states (unexplored, shrouded,
visible), compute visibility each frame from all active vision sources, generate a GPU fog texture
sampled by terrain, entity, and minimap shaders, and encode fog state at 2 bits per cell per faction
for efficient network transmission.

- **Derived from:** [F-13.20.1](../../features/game-framework/fog-of-war.md)
- **Rationale:** Three-state fog with GPU-rendered textures provides the standard RTS visibility
  model with minimal per-frame cost and compact multiplayer bandwidth.
- **Verification:** Place a unit on a fog grid with 256x256 cells. Verify cells within sight radius
  are visible, previously seen cells are shrouded, and unvisited cells are unexplored. Remove the
  unit; verify visible cells transition to shrouded. Confirm the fog texture renders correct
  darkening per state. Serialize fog state and verify the payload is exactly 2 *256* 256 / 8 =
  16,384 bytes per faction.

## R-13.20.2 Vision Source and Sight Radius

The engine **SHALL** reveal fog cells for each entity bearing a `VisionSource` component within its
configured sight radius and shape (circle or directional cone), apply elevation-based radius
bonuses, and block line-of-sight through walls, terrain, and tagged occluders using the shared
spatial index.

- **Derived from:** [F-13.20.2](../../features/game-framework/fog-of-war.md)
- **Rationale:** Per-entity vision with occlusion and elevation bonuses enables tactical depth where
  positioning and terrain advantage meaningfully affect information warfare.
- **Verification:** Place a unit with radius 10 on flat terrain; verify exactly the cells within
  radius 10 are revealed. Place a wall occluder between the unit and a target cell beyond the wall;
  verify the target cell remains hidden. Move the unit to an elevation tier granting +3 bonus;
  verify the effective radius increases to 13.

## R-13.20.3 Vision Modifier Volumes

The engine **SHALL** support world-placed trigger volumes that modify vision behavior, including
stealth zones (entities inside invisible to outsiders), vision-blocking volumes (block sight
through), high-ground zones (radius bonus), and darkness zones (radius reduction), with support for
dynamic modifiers that spawn and despawn at runtime.

- **Derived from:** [F-13.20.3](../../features/game-framework/fog-of-war.md)
- **Rationale:** Vision modifiers create tactical terrain features (bushes, smoke, high ground) that
  add strategic variety to fog-of-war gameplay.
- **Verification:** Place a unit inside a stealth zone; verify an enemy unit outside the zone cannot
  see it. Place a vision-blocking volume between two units; verify line-of-sight is blocked. Spawn a
  dynamic smoke volume at runtime; verify it blocks vision. Despawn it; verify vision restores
  within one frame.

## R-13.20.4 Fog of War Memory

The engine **SHALL** store per-cell last-seen snapshots in shrouded areas that display ghost images
of terrain, buildings, and resources at their last-observed state, updating the snapshot only when
the cell is re-revealed, with memory persisted per-faction through the save system.

- **Derived from:** [F-13.20.4](../../features/game-framework/fog-of-war.md)
- **Rationale:** Fog memory gives players useful contextual information about previously explored
  areas without leaking real-time state of hidden regions.
- **Verification:** Reveal an area containing a building, then remove vision. Verify the building
  ghost renders in the shrouded area. Destroy the building while shrouded; verify the ghost
  persists. Re-reveal the area; verify the ghost updates to reflect the destroyed state. Save and
  reload; verify fog memory snapshots persist correctly.

## Non-Functional Requirements

### NFR-13.20.1 Fog of War GPU Performance

Fog texture generation on the GPU **SHALL** complete within 0.5ms per frame for a 512x512 fog grid
with up to 128 active vision sources. CPU fallback on devices without compute **SHALL** complete
within 2ms for the same configuration.

- **Rationale:** Fog of war is computed every frame and must not consume a significant portion of
  the frame budget, especially in RTS games with many units.
- **Verification:** Configure a 512x512 grid with 128 vision sources. Measure GPU fog texture
  generation time. Verify it stays under 0.5ms. Disable compute and measure CPU fallback time.
  Verify it stays under 2ms.

### NFR-13.20.2 Fog Network Bandwidth

Fog state transmission **SHALL** consume no more than 2 bits per cell per faction. For a 512x512
grid with 4 factions, the total fog payload **SHALL NOT** exceed 262,144 bytes per full sync. Delta
updates **SHALL** transmit only changed cells, reducing bandwidth by at least 90% during
steady-state gameplay.

- **Rationale:** RTS games transmit fog state frequently. Compact encoding and delta updates
  minimize bandwidth consumption in multiplayer.
- **Verification:** Serialize a full 512x512 fog grid for 4 factions. Verify payload size equals 2 *
  512 *512* 4 / 8 = 262,144 bytes. Reveal 5% of cells and verify delta update size is at least 90%
  smaller than full sync.
