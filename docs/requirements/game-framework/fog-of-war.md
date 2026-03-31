# R-13.20 -- Fog of War Requirements

## Visibility Grid

1. **R-13.20.1** -- The engine **SHALL** provide a grid-based visibility system with three states
   per cell per faction (unexplored, shrouded, visible), configurable resolution independent of
   terrain, GPU compute texture generation with CPU fallback, and compact 2-bit-per-cell networking.
   - **Rationale:** A compact per-faction grid enables efficient fog state replication and
     GPU-driven rendering.
   - **Verification:** Verify fog texture renders on GPU compute. Fall back to CPU on a device
     without compute and verify identical output. Verify network payload is 2 bits per cell per
     faction.

2. **R-13.20.2** -- The engine **SHALL** compute per-entity vision using configurable sight radius,
   shape (circle or cone), height advantage bonus, and line-of-sight blocking via the shared spatial
   index.
   - **Rationale:** LOS-blocked vision with configurable parameters enables per-unit-type tactical
     differentiation.
   - **Verification:** Place a wall between a unit and a cell and verify the cell is not revealed.
     Elevate a unit and verify increased effective radius.

## Vision Modifiers and Memory

1. **R-13.20.3** -- The engine **SHALL** support world-placed vision modifier volumes (concealment,
   occlusion, bonus, reduction) as ECS entities with trigger volumes, including dynamic temporary
   modifiers with configurable dissipation.
   - **Rationale:** Vision modifiers enable tactical terrain design and ability-created tactical
     cover.
   - **Verification:** Place a concealment volume and verify units inside are invisible to units
     outside. Create a temporary modifier and verify it dissipates after the configured duration.

2. **R-13.20.4** -- The engine **SHALL** store per-cell last-seen state as ghost images in shrouded
   areas, with per-faction persistence through the save system and update on re-reveal.
   - **Rationale:** Fog memory provides contextual information about previously explored areas
     without leaking real-time state.
   - **Verification:** Destroy a building while shrouded and verify the ghost persists until the
     area is re-revealed. Save and load and verify fog memory survives.
