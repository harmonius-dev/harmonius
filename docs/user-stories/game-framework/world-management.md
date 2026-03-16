# User Stories — 13.2 World Management

## F-13.2.1 Level Streaming

## US-13.2.1.1 Configure Streaming Load Radii Per Zone

**As a** level designer (P-6), **I want to** set per-zone load and unload radii with hysteresis
margins in the visual editor, **so that** sub-levels stream in and out smoothly without thrashing at
boundaries.

## US-13.2.1.2 Assign Streaming Priority Per Asset Type

**As a** level designer (P-6), **I want to** prioritize streaming so geometry and collision load
before props and decorations, **so that** fast-moving players see terrain before detail objects.

## US-13.2.1.3 Experience Seamless Zone Transitions

**As a** player (P-23), **I want to** move through the open world without visible loading screens or
stalls, **so that** exploration feels continuous and immersive.

## US-13.2.1.4 Monitor Streaming Throughput on Live Servers

**As a** server admin (P-22), **I want to** view streaming I/O throughput and queue depth per server
node, **so that** I can identify streaming bottlenecks causing player-visible pop-in.

## US-13.2.1.5 Verify Streaming Uses Platform-Native Async I/O

**As an** engine tester (P-27), **I want to** verify that level streaming uses IOCP on Windows, GCD
on macOS, and io_uring on Linux with no blocking I/O calls, **so that** streaming never stalls the
game thread.

## US-13.2.1.6 Verify No Entity Duplication Across Stream Boundaries

**As an** engine tester (P-27), **I want to** cross a stream boundary repeatedly and verify no
duplicate entities appear in the ECS world, **so that** streaming merges are idempotent.

## F-13.2.2 Grid-Based World Partitioning

## US-13.2.2.1 Configure World Grid Cell Size

**As a** level designer (P-6), **I want to** set the world partition grid cell size (e.g., 256m or
512m) per world in the editor, **so that** cells match my zone layout and content density.

## US-13.2.2.2 Visualize Grid Cell Boundaries in Editor

**As a** level designer (P-6), **I want to** see grid cell boundaries overlaid on the world
viewport, **so that** I can place content aware of streaming boundaries.

## US-13.2.2.3 Experience Consistent Entity Visibility Near Cell Borders

**As a** player (P-23), **I want to** see entities near cell borders without pop-in or
disappearance, **so that** partitioning is invisible during gameplay.

## US-13.2.2.4 Assign Grid Cells to Server Processes

**As a** server admin (P-22), **I want to** view and reassign grid cells to server processes in the
server mesh dashboard, **so that** I can balance load across nodes.

## US-13.2.2.5 Verify O(1) Cell Lookup From World Position

**As an** engine tester (P-27), **I want to** query the cell for one million random world positions
and verify O(1) lookup performance, **so that** interest management scales to thousands of
concurrent players.

## US-13.2.2.6 Verify Cell Entity Tracking Consistency

**As an** engine tester (P-27), **I want to** move entities across cell boundaries and verify that
each entity is tracked by exactly one cell at all times, **so that** no entity is lost or
double-counted.

## F-13.2.3 Hierarchical Spatial Partitioning

## US-13.2.3.1 Configure Spatial Tree Type Per Zone

**As a** level designer (P-6), **I want to** select octree (3D) or quadtree (2D) partitioning per
zone in the editor, **so that** zones with extreme vertical extent use the appropriate data
structure.

## US-13.2.3.2 Query Nearby Entities Within a Radius

**As a** player (P-23), **I want to** AoE abilities and proximity triggers to find affected entities
efficiently, **so that** gameplay targeting feels instant even in crowded areas.

## US-13.2.3.3 Monitor Spatial Query Latency on Live Servers

**As a** server admin (P-22), **I want to** view spatial query latency percentiles per tick,
**so that** I can detect degraded query performance before it affects gameplay.

## US-13.2.3.4 Verify Incremental Tree Update Amortization

**As an** engine tester (P-27), **I want to** move 10,000 entities per frame and verify that tree
rebalancing cost is amortized below 1ms per frame, **so that** updates do not cause frame spikes.

## F-13.2.4 Sub-Level Composition

## US-13.2.4.1 Organize Content Into Composable Sub-Levels

**As a** level designer (P-6), **I want to** author terrain, buildings, interiors, quest objects,
and seasonal decorations as independent sub-levels, **so that** each content layer is edited without
affecting others.

## US-13.2.4.2 Assign Sub-Levels to Streaming Groups

**As a** level designer (P-6), **I want to** assign sub-levels to streaming groups with shared
load/unload triggers, **so that** related content loads and unloads together.

## US-13.2.4.3 Experience Quest-Phased Sub-Level Swaps

**As a** player (P-23), **I want to** see the world change based on my quest progress (e.g., a town
before and after destruction), **so that** my actions visibly affect the game world.

## US-13.2.4.4 Verify Conditional Sub-Levels Load Based on Quest State

**As an** engine tester (P-27), **I want to** advance a quest that triggers a sub-level swap and
verify the correct sub-level loads while the previous one unloads, **so that** phased content
transitions are correct.

## F-13.2.5 Persistent and Transient Actors

## US-13.2.5.1 Mark Entities as Persistent or Transient

**As a** level designer (P-6), **I want to** classify placed entities as persistent (saved to
database) or transient (spawned from templates at load), **so that** only entities requiring
identity survive unload/reload.

## US-13.2.5.2 Maintain Persistent Identity Across Server Restarts

**As a** player (P-23), **I want to** player housing objects, mailboxes, and quest-giver state to
survive server restarts, **so that** my placed objects and NPC interactions persist.

## US-13.2.5.3 Monitor Persistent Entity Database Size

**As a** server admin (P-22), **I want to** view the count and storage size of persistent entities
per shard, **so that** I can plan database capacity for growing player populations.

## US-13.2.5.4 Verify Persistent IDs Survive Shard Migration

**As an** engine tester (P-27), **I want to** migrate a persistent entity between shards and verify
its globally unique ID remains stable, **so that** identity is never lost during migrations.

## F-13.2.6 Day/Night Cycle

## US-13.2.6.1 Configure Time-of-Day Parameters

**As a** level designer (P-6), **I want to** configure the real-to-game time ratio, sun/moon
positions, sky color gradients, and ambient light intensity per zone, **so that** each zone has a
distinct day/night atmosphere.

## US-13.2.6.2 Experience Smooth Dawn-to-Night Transitions

**As a** player (P-23), **I want to** see smooth transitions between dawn, day, dusk, and night with
gradual lighting and sky changes, **so that** the time-of-day cycle feels natural.

## US-13.2.6.3 Synchronize Time Across All Players in a Zone

**As a** server admin (P-22), **I want to** all players in a zone to see the same sky and lighting
state, **so that** server-authoritative time synchronization prevents visual desynchronization.

## US-13.2.6.4 Verify Cycle Consistency Across Frame Rates

**As an** engine tester (P-27), **I want to** run the day/night cycle at 30 and 120 FPS and verify
identical sun positions at matching game times, **so that** the cycle is frame-rate independent.

## F-13.2.7 Weather System Integration

## US-13.2.7.1 Configure Weather States and Transitions

**As a** level designer (P-6), **I want to** define weather states (clear, rain, snow, sandstorm,
fog) with blend transitions and regional overrides, **so that** each zone has weather behavior
matching its biome.

## US-13.2.7.2 Experience Weather Effects During Gameplay

**As a** player (P-23), **I want to** see rain particles, hear thunder ambience, and notice wet
surface reflections during storms, **so that** weather feels immersive and multi-sensory.

## US-13.2.7.3 Trigger Server-Driven Weather Events

**As a** server admin (P-22), **I want to** trigger server-driven weather events (blizzard for world
boss, seasonal festival weather) via admin commands, **so that** live events include weather as a
gameplay element.

## US-13.2.7.4 Verify Weather Modifiers Affect Gameplay Stats

**As an** engine tester (P-27), **I want to** activate each weather state and verify that gameplay
modifiers (movement speed on ice, reduced visibility in fog) apply correctly, **so that** weather
has the intended gameplay impact.
