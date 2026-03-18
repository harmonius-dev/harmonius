# R-13.2 — World Management Requirements

## Level Streaming

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-13.2.1 | [F-13.2.1](../../features/game-framework/world-management.md) |

1. **R-13.2.1** — The engine **SHALL** load and unload sub-levels asynchronously based on player
   proximity, quest phase, or server commands, using configurable load radii with hysteresis to
   prevent boundary thrashing, merging entities into the active ECS world without stalling the game
   thread, and prioritizing geometry and collision streaming before props and decoration.
   - **Rationale:** Async level streaming with priority ordering minimizes pop-in for fast-moving
     players and prevents frame hitches during zone transitions.
   - **Verification:** Integration test: place a player near a sub-level boundary, move across it,
     and verify the sub-level loads without frame stalls exceeding 1 ms on the game thread. Verify
     hysteresis prevents thrashing when oscillating at the boundary. Verify geometry loads before
     props by checking entity arrival order. Verify platform-native async I/O is used (no stdlib
     file I/O).

## World Partitioning

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-13.2.2 | [F-13.2.2](../../features/game-framework/world-management.md) |
| R-13.2.3 | [F-13.2.3](../../features/game-framework/world-management.md) |

1. **R-13.2.2** — The engine **SHALL** divide the open world into a regular grid of configurable
   cell size (e.g., 256 m or 512 m), where each cell is independently streamable and assignable to a
   server process, tracks its resident entities, static geometry references, and navigation mesh
   segment, and provides O(1) cell lookup from world position.
   - **Rationale:** Grid partitioning with O(1) lookup is critical for efficient interest management
     and server mesh assignment across thousands of concurrent players.
   - **Verification:** Unit test: create a world grid with 512 m cells, insert 10,000 entities at
     random positions, and verify each entity's cell lookup completes in O(1) time. Benchmark cell
     lookup for 100,000 queries and verify constant-time scaling regardless of entity count. Verify
     cell size is reconfigurable per world.
2. **R-13.2.3** — The engine **SHALL** supplement grid partitioning with an octree (3D) or quadtree
   (2D) for scenes with extreme vertical extent or non-uniform entity density, incrementally
   updating the tree as entities move and amortizing rebalancing costs across frames.
   - **Rationale:** Flat grids are too coarse for spatial queries in scenes with extreme vertical
     extent or density variation; hierarchical structures adapt to non-uniform distributions.
   - **Verification:** Unit test: insert 10,000 entities with non-uniform vertical distribution into
     an octree. Perform proximity and visibility queries and verify correct results. Move 1,000
     entities per frame and verify incremental rebalancing completes within 0.5 ms amortized per
     frame.

## Sub-Levels and Composition

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-13.2.4 | [F-13.2.4](../../features/game-framework/world-management.md) |
| R-13.2.5 | [F-13.2.5](../../features/game-framework/world-management.md) |

1. **R-13.2.4** — The engine **SHALL** organize world content into composable sub-levels (terrain,
   buildings, interiors, quest objects, seasonal decorations) authored independently and layered at
   runtime, with designer-assigned streaming groups sharing load/unload triggers, and conditional
   sub-levels that appear only when specific quest phases or world events are active.
   - **Rationale:** Composable sub-levels enable parallel content authoring and dynamic world
     changes (phased content, seasonal events) without rebuilding entire levels.
   - **Verification:** Integration test: create three sub-levels in a streaming group, trigger the
     group load, and verify all three merge into the active world. Define a conditional sub-level
     gated on a quest phase, advance the quest, and verify the sub-level appears. Revert the quest
     phase and verify the sub-level unloads.
2. **R-13.2.5** — The engine **SHALL** classify entities as persistent (surviving unload/reload with
   stable globally unique IDs, saved to the database) or transient (spawned from templates at load
   time, discarded on unload), with persistent identity surviving server restarts and shard
   migrations.
   - **Rationale:** Persistent identity ensures player housing, quest-giver state, and other durable
     objects survive server restarts, while transient actors avoid unnecessary database overhead.
   - **Verification:** Unit test: create a persistent entity with a globally unique ID, unload and
     reload the sub-level, and verify the entity retains its ID and component state. Create a
     transient entity, unload the sub-level, and verify the entity is discarded. Simulate a shard
     migration and verify persistent IDs remain stable.

## Time of Day and Weather

| ID       | Derived From                                                  |
|----------|---------------------------------------------------------------|
| R-13.2.6 | [F-13.2.6](../../features/game-framework/world-management.md) |
| R-13.2.7 | [F-13.2.7](../../features/game-framework/world-management.md) |

1. **R-13.2.6** — The engine **SHALL** drive a configurable time-of-day system with sun/moon
   position, sky color gradients, ambient light intensity, and shadow direction at an accelerated
   game-time ratio, producing smooth transitions between dawn, day, dusk, and night, with
   server-authoritative time synchronization ensuring all players in a zone see the same sky.
   - **Rationale:** A unified day/night cycle integrating lighting, fog, and post-processing
     provides visual consistency and enables time-gated gameplay content.
   - **Verification:** Integration test: run the day/night cycle and verify sun position, sky color,
     and ambient light intensity change smoothly across dawn, day, dusk, and night phases. Connect
     two clients to the same server zone and verify both see the same sky state within 100 ms of
     each other.
2. **R-13.2.7** — The engine **SHALL** define weather states (clear, overcast, rain, snow,
   sandstorm, fog) with blend transitions and regional overrides, feeding into particle emitters,
   post-processing (wet surfaces, fog density), audio ambience, and gameplay modifiers (movement
   speed, visibility), with support for server-driven weather events.
   - **Rationale:** Weather integration across rendering, audio, and gameplay systems creates
     immersive environments and enables weather-driven gameplay mechanics.
   - **Verification:** Integration test: trigger a weather transition from clear to rain and verify
     particle emitters activate, wet-surface post-processing enables, rain audio ambience plays, and
     movement speed modifier applies. Verify regional overrides allow one zone to have rain while an
     adjacent zone remains clear. Verify server-driven weather events propagate to all connected
     clients.

## Non-Functional Requirements

| ID         | Derived From |
|------------|--------------|
| R-13.2.NF1 | F-13.2.1     |
| R-13.2.NF2 | F-13.2.2     |

1. **R-13.2.NF1** — The engine **SHALL** complete sub-level entity merging into the active ECS world
   within 1 ms of game-thread time per streaming operation, with total async I/O and decompression
   happening off the game thread.
   - **Rationale:** Streaming stalls exceeding 1 ms cause perceptible frame hitches, especially at
     zone boundaries where multiple sub-levels may load simultaneously.
   - **Verification:** Stream a sub-level containing 1,000 entities while a player crosses a
     boundary at maximum movement speed. Measure game-thread stall per streaming merge and verify
     the 99th percentile stays under 1 ms.
2. **R-13.2.NF2** — The engine **SHALL** support a world grid of at least 4,096 x 4,096 cells (16.7
   million cells) with O(1) cell lookup and less than 100 MB of memory overhead for the grid
   metadata.
   - **Rationale:** Open-world games with large maps (e.g., 2048 km squared at 512 m cells) require
     millions of cells; the grid structure must scale without becoming a memory bottleneck.
   - **Verification:** Create a 4,096 x 4,096 grid. Perform 1 million random cell lookups and verify
     O(1) performance. Measure grid metadata memory and verify it stays under 100 MB.
