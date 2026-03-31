# R-13.2 -- World Management Requirements

## Level Streaming

1. **R-13.2.1** -- The engine **SHALL** load and unload sub-levels asynchronously based on proximity
   with configurable radii, hysteresis, and priority ordering, using platform-native async I/O (IOCP
   on Windows, GCD on macOS, io_uring on Linux) with no blocking calls.
   - **Rationale:** Non-blocking async I/O ensures streaming never stalls the game thread on any
     platform.
   - **Verification:** Cross a stream boundary and verify no frame spike. Verify geometry and
     collision load before props. Verify no duplicate entities appear after repeated crossings.

## World Grid

1. **R-13.2.2** -- The engine **SHALL** divide the world into a configurable grid with O(1) cell
   lookup from world position, per-cell entity tracking, and server mesh cell assignment.
   - **Rationale:** O(1) cell lookup is critical for efficient interest management across thousands
     of concurrent players.
   - **Verification:** Query the cell for one million random positions and verify O(1) performance.
     Move an entity across cell boundaries and verify it is tracked by exactly one cell.

2. **R-13.2.3** -- The engine **SHALL** supplement grid partitioning with octree (3D) or quadtree
   (2D) for scenes with extreme vertical extent, with incrementally amortized rebalancing.
   - **Rationale:** Hierarchical partitioning provides efficient spatial queries in non-uniform
     entity distributions.
   - **Verification:** Move 10,000 entities per frame and verify tree rebalancing cost stays under 1
     ms.

## Sub-Level Composition

1. **R-13.2.4** -- The engine **SHALL** organize world content into composable sub-levels with
   streaming groups, shared load/unload triggers, and conditional visibility based on quest phase or
   world events.
   - **Rationale:** Composable sub-levels enable independent content authoring and phased world
     changes.
   - **Verification:** Advance a quest that triggers a sub-level swap and verify the correct
     sub-level loads while the previous one unloads.

2. **R-13.2.5** -- The engine **SHALL** classify entities as persistent (saved to database with
   globally unique IDs surviving server restarts) or transient (spawned from templates at load,
   discarded on unload).
   - **Rationale:** Persistence classification ensures only identity- requiring entities incur
     storage cost.
   - **Verification:** Migrate a persistent entity between shards and verify its globally unique ID
     remains stable.

## Time of Day and Weather

1. **R-13.2.6** -- The engine **SHALL** drive a configurable day/night cycle with sun/moon position,
   sky color gradients, ambient light, and shadow direction at an accelerated real-to-game time
   ratio, with server-authoritative time synchronization.
   - **Rationale:** Server-authoritative time prevents visual desynchronization between players in
     the same zone.
   - **Verification:** Run the cycle at 30 and 120 FPS and verify identical sun positions at
     matching game times. Verify all players in a zone see the same sky.

2. **R-13.2.7** -- The engine **SHALL** define weather states with blend transitions and regional
   overrides, feeding into particle emitters, post-processing (wet surfaces, fog density), audio
   ambience, and gameplay modifiers (movement speed, visibility).
   - **Rationale:** Weather as a cross-system input enables immersive environmental storytelling and
     gameplay variation.
   - **Verification:** Activate each weather state and verify gameplay modifiers (movement speed on
     ice, reduced visibility in fog) apply correctly.
