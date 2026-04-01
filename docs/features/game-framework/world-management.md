# 13.2 — World Management

## Level Streaming

| ID       | Feature         |
|----------|-----------------|
| F-13.2.1 | Level Streaming |

1. **F-13.2.1** — Loads and unloads sub-levels asynchronously based on player proximity, quest
   phase, or server commands. Streaming uses configurable load radii with hysteresis to prevent
   thrashing at boundaries. Loaded sub-levels merge their entities into the active ECS world without
   stalling the game thread. Supports priority ordering so that geometry and collision stream before
   props and decoration, minimizing pop-in for fast-moving players in open world zones.
   - **Deps:** F-1.1.10 (Multiple Worlds), F-12.3.1 (Asset Streaming)
   - **Platform:** Tokio handles platform I/O internally (IOCP on Windows, kqueue on macOS, epoll on
     Linux) for non-blocking level data reads.

## World Partitioning

| ID       | Feature                           |
|----------|-----------------------------------|
| F-13.2.2 | Grid-Based World Partitioning     |
| F-13.2.3 | Hierarchical Spatial Partitioning |

1. **F-13.2.2** — Divides the open world into a regular grid of cells, each independently streamable
   and assignable to a server process in the server mesh. Cell size is configurable per world (e.g.,
   256m or 512m tiles). Each cell tracks its resident entities, static geometry references, and
   navigation mesh segment. Grid partitioning provides O(1) cell lookup from world position, which
   is critical for efficient interest management across thousands of concurrent players.
   - **Deps:** F-13.2.1, F-8.7.3 (Dynamic Server Mesh)
2. **F-13.2.3** — Supplements grid partitioning with an octree (3D) or quadtree (2D) for scenes with
   extreme vertical extent or non-uniform entity density. Used for spatial queries (visibility,
   proximity, AoE targeting) where a flat grid would be too coarse. The tree is incrementally
   updated as entities move, amortizing rebalancing costs across frames.
   - **Deps:** F-13.2.2

## Sub-Levels and Composition

| ID       | Feature                         |
|----------|---------------------------------|
| F-13.2.4 | Sub-Level Composition           |
| F-13.2.5 | Persistent and Transient Actors |

1. **F-13.2.4** — Organizes world content into composable sub-levels (e.g., terrain, buildings,
   interiors, quest objects, seasonal decorations) that are authored independently and layered at
   runtime. Designers assign sub-levels to streaming groups with shared load/unload triggers.
   Supports conditional sub-levels that appear only when specific quest phases or world events are
   active, enabling phased content like post-destruction cityscapes or holiday-themed town
   decorations.
   - **Deps:** F-13.2.1, F-13.6.1 (Quest Graph)
2. **F-13.2.5** — Classifies entities as persistent (survive unload/reload with stable identity,
   saved to the database) or transient (spawned at load time from templates, discarded on unload).
   Persistent actors include player housing objects, mailboxes, auction NPCs, and quest-giver state.
   Transient actors include ambient wildlife, respawning resource nodes, and patrol-route NPCs.
   Persistence identity uses globally unique IDs that survive server restarts and shard migrations.
   - **Deps:** F-1.1.4 (Generational Indices), F-8.7.5 (Persistent World State)

## Time of Day and Weather

| ID       | Feature                    |
|----------|----------------------------|
| F-13.2.6 | Day/Night Cycle            |
| F-13.2.7 | Weather System Integration |

1. **F-13.2.6** — Drives a configurable time-of-day system with sun/moon position, sky color
   gradients, ambient light intensity, and shadow direction. Game time runs at an accelerated ratio
   to real time (e.g., 1 hour real = 24 hours in-game). The cycle integrates with the lighting, fog,
   and post-processing systems to produce smooth transitions between dawn, day, dusk, and night.
   Server-authoritative time synchronization ensures all players in a zone see the same sky.
   - **Deps:** F-2.3.1 (Lighting), F-2.6.1 (Sky/Atmosphere)
2. **F-13.2.7** — Defines weather states (clear, overcast, rain, snow, sandstorm, fog) with blend
   transitions and regional overrides. Weather state feeds into particle emitters, post-processing
   (wet surfaces, fog density), audio ambience, and gameplay modifiers (movement speed on ice,
   reduced visibility in fog). Supports server-driven weather events for world boss encounters and
   seasonal events.
   - **Deps:** F-13.2.6, F-11.1.1 (Particle Emitters), F-5.2.1 (Spatial Audio)
