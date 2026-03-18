# User Stories — 13.2 World Management

## F-13.2.1 Level Streaming

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.1.1 | level designer (P-6) | F-13.2.1 | R-13.2.1     |
| US-13.2.1.2 | level designer (P-6) | F-13.2.1 | R-13.2.1     |
| US-13.2.1.3 | player (P-23)        | F-13.2.1 | R-13.2.1     |
| US-13.2.1.4 | server admin (P-22)  | F-13.2.1 | R-13.2.1     |
| US-13.2.1.5 | engine tester (P-27) | F-13.2.1 | R-13.2.1     |
| US-13.2.1.6 | engine tester (P-27) | F-13.2.1 | R-13.2.1     |

1. **US-13.2.1.1** — I want to set per-zone load and unload radii with hysteresis margins in the
   visual editor so that sub-levels stream in and out smoothly without thrashing at boundaries
2. **US-13.2.1.2** — I want to prioritize streaming so geometry and collision load before props and
   decorations so that fast-moving players see terrain before detail objects
3. **US-13.2.1.3** — I want to move through the open world without visible loading screens or stalls
   so that exploration feels continuous and immersive
4. **US-13.2.1.4** — I want to view streaming I/O throughput and queue depth per server node so that
   I can identify streaming bottlenecks causing player-visible pop-in
5. **US-13.2.1.5** — I want to verify that level streaming uses IOCP on Windows, GCD on macOS, and
   io_uring on Linux with no blocking I/O calls so that streaming never stalls the game thread
6. **US-13.2.1.6** — I want to cross a stream boundary repeatedly and verify no duplicate entities
   appear in the ECS world so that streaming merges are idempotent

## F-13.2.2 Grid-Based World Partitioning

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.2.1 | level designer (P-6) | F-13.2.2 | R-13.2.2     |
| US-13.2.2.2 | level designer (P-6) | F-13.2.2 | R-13.2.2     |
| US-13.2.2.3 | player (P-23)        | F-13.2.2 | R-13.2.2     |
| US-13.2.2.4 | server admin (P-22)  | F-13.2.2 | R-13.2.2     |
| US-13.2.2.5 | engine tester (P-27) | F-13.2.2 | R-13.2.2     |
| US-13.2.2.6 | engine tester (P-27) | F-13.2.2 | R-13.2.2     |

1. **US-13.2.2.1** — I want to set the world partition grid cell size (e.g., 256m or 512m) per world
   in the editor so that cells match my zone layout and content density
2. **US-13.2.2.2** — I want to see grid cell boundaries overlaid on the world viewport so that I can
   place content aware of streaming boundaries
3. **US-13.2.2.3** — I want to see entities near cell borders without pop-in or disappearance so
   that partitioning is invisible during gameplay
4. **US-13.2.2.4** — I want to view and reassign grid cells to server processes in the server mesh
   dashboard so that I can balance load across nodes
5. **US-13.2.2.5** — I want to query the cell for one million random world positions and verify O(1)
   lookup performance so that interest management scales to thousands of concurrent players
6. **US-13.2.2.6** — I want to move entities across cell boundaries and verify that each entity is
   tracked by exactly one cell at all times so that no entity is lost or double-counted

## F-13.2.3 Hierarchical Spatial Partitioning

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.3.1 | level designer (P-6) | F-13.2.3 | R-13.2.3     |
| US-13.2.3.2 | player (P-23)        | F-13.2.3 | R-13.2.3     |
| US-13.2.3.3 | server admin (P-22)  | F-13.2.3 | R-13.2.3     |
| US-13.2.3.4 | engine tester (P-27) | F-13.2.3 | R-13.2.3     |

1. **US-13.2.3.1** — I want to select octree (3D) or quadtree (2D) partitioning per zone in the
   editor so that zones with extreme vertical extent use the appropriate data structure
2. **US-13.2.3.2** — I want to AoE abilities and proximity triggers to find affected entities
   efficiently so that gameplay targeting feels instant even in crowded areas
3. **US-13.2.3.3** — I want to view spatial query latency percentiles per tick so that I can detect
   degraded query performance before it affects gameplay
4. **US-13.2.3.4** — I want to move 10,000 entities per frame and verify that tree rebalancing cost
   is amortized below 1ms per frame so that updates do not cause frame spikes

## F-13.2.4 Sub-Level Composition

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.4.1 | level designer (P-6) | F-13.2.4 | R-13.2.4     |
| US-13.2.4.2 | level designer (P-6) | F-13.2.4 | R-13.2.4     |
| US-13.2.4.3 | player (P-23)        | F-13.2.4 | R-13.2.4     |
| US-13.2.4.4 | engine tester (P-27) | F-13.2.4 | R-13.2.4     |

1. **US-13.2.4.1** — I want to author terrain, buildings, interiors, quest objects, and seasonal
   decorations as independent sub-levels so that each content layer is edited without affecting
   others
2. **US-13.2.4.2** — I want to assign sub-levels to streaming groups with shared load/unload
   triggers so that related content loads and unloads together
3. **US-13.2.4.3** — I want to see the world change based on my quest progress (e.g., a town before
   and after destruction) so that my actions visibly affect the game world
4. **US-13.2.4.4** — I want to advance a quest that triggers a sub-level swap and verify the correct
   sub-level loads while the previous one unloads so that phased content transitions are correct

## F-13.2.5 Persistent and Transient Actors

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.5.1 | level designer (P-6) | F-13.2.5 | R-13.2.5     |
| US-13.2.5.2 | player (P-23)        | F-13.2.5 | R-13.2.5     |
| US-13.2.5.3 | server admin (P-22)  | F-13.2.5 | R-13.2.5     |
| US-13.2.5.4 | engine tester (P-27) | F-13.2.5 | R-13.2.5     |

1. **US-13.2.5.1** — I want to classify placed entities as persistent (saved to database) or
   transient (spawned from templates at load) so that only entities requiring identity survive
   unload/reload
2. **US-13.2.5.2** — I want to player housing objects, mailboxes, and quest-giver state to survive
   server restarts so that my placed objects and NPC interactions persist
3. **US-13.2.5.3** — I want to view the count and storage size of persistent entities per shard so
   that I can plan database capacity for growing player populations
4. **US-13.2.5.4** — I want to migrate a persistent entity between shards and verify its globally
   unique ID remains stable so that identity is never lost during migrations

## F-13.2.6 Day/Night Cycle

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.6.1 | level designer (P-6) | F-13.2.6 | R-13.2.6     |
| US-13.2.6.2 | player (P-23)        | F-13.2.6 | R-13.2.6     |
| US-13.2.6.3 | server admin (P-22)  | F-13.2.6 | R-13.2.6     |
| US-13.2.6.4 | engine tester (P-27) | F-13.2.6 | R-13.2.6     |

1. **US-13.2.6.1** — I want to configure the real-to-game time ratio, sun/moon positions, sky color
   gradients, and ambient light intensity per zone so that each zone has a distinct day/night
   atmosphere
2. **US-13.2.6.2** — I want to see smooth transitions between dawn, day, dusk, and night with
   gradual lighting and sky changes so that the time-of-day cycle feels natural
3. **US-13.2.6.3** — I want to all players in a zone to see the same sky and lighting state so that
   server-authoritative time synchronization prevents visual desynchronization
4. **US-13.2.6.4** — I want to run the day/night cycle at 30 and 120 FPS and verify identical sun
   positions at matching game times so that the cycle is frame-rate independent

## F-13.2.7 Weather System Integration

| ID          | Persona              | Features | Requirements |
|-------------|----------------------|----------|--------------|
| US-13.2.7.1 | level designer (P-6) | F-13.2.7 | R-13.2.7     |
| US-13.2.7.2 | player (P-23)        | F-13.2.7 | R-13.2.7     |
| US-13.2.7.3 | server admin (P-22)  | F-13.2.7 | R-13.2.7     |
| US-13.2.7.4 | engine tester (P-27) | F-13.2.7 | R-13.2.7     |

1. **US-13.2.7.1** — I want to define weather states (clear, rain, snow, sandstorm, fog) with blend
   transitions and regional overrides so that each zone has weather behavior matching its biome
2. **US-13.2.7.2** — I want to see rain particles, hear thunder ambience, and notice wet surface
   reflections during storms so that weather feels immersive and multi-sensory
3. **US-13.2.7.3** — I want to trigger server-driven weather events (blizzard for world boss,
   seasonal festival weather) via admin commands so that live events include weather as a gameplay
   element
4. **US-13.2.7.4** — I want to activate each weather state and verify that gameplay modifiers
   (movement speed on ice, reduced visibility in fog) apply correctly so that weather has the
   intended gameplay impact
