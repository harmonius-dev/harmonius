# User Stories — 7.1 Navigation

## F-7.1.1 — Recast-Style NavMesh Generation

## US-7.1.1.1 Generate NavMesh from Level Geometry

**As a** designer (P-5), **I want to** generate a navigation mesh from static level geometry with
one click in the editor, **so that** AI agents can pathfind without manual navmesh authoring.

## US-7.1.1.2 Configure Agent Radius for NavMesh

**As a** designer (P-5), **I want to** set the agent radius and height used for NavMesh generation,
**so that** the mesh accounts for my character's physical size.

## US-7.1.1.3 Preview Walkable Areas in Editor

**As a** designer (P-5), **I want to** preview which surfaces are classified as walkable before
committing to a full NavMesh bake, **so that** I can adjust geometry or slope limits early.

## US-7.1.1.4 See AI Navigate Around Obstacles

**As a** player (P-23), **I want** enemies to navigate around furniture, rocks, and other obstacles
smoothly, **so that** AI movement feels natural and intelligent.

## US-7.1.1.5 Experience NPCs Using Appropriate Paths

**As a** player (P-23), **I want** NPCs to walk on roads and bridges instead of clipping through
terrain, **so that** the world feels believable.

## US-7.1.1.6 See Large Creatures Use Wide Paths

**As a** player (P-23), **I want** large creatures to avoid narrow doorways they cannot fit through,
**so that** their movement matches their visible size.

## US-7.1.1.7 Implement Voxel-Based NavMesh Generation

**As an** engine developer (P-26), **I want to** implement voxel-based NavMesh generation using
heightfield rasterization, **so that** complex geometry produces accurate walkable surfaces.

## US-7.1.1.8 Support Configurable Slope and Step Limits

**As an** engine developer (P-26), **I want to** expose slope angle and step-climb parameters to the
voxelization pipeline, **so that** different agent archetypes get appropriately filtered meshes.

## US-7.1.1.9 Run NavMesh Generation as Content Pipeline Step

**As an** engine developer (P-26), **I want to** integrate NavMesh baking into the offline content
pipeline, **so that** shipping builds include pre-baked meshes without runtime generation cost.

## US-7.1.1.10 Verify NavMesh Covers All Walkable Surfaces

**As an** engine tester (P-27), **I want to** verify that generated NavMesh covers 100% of walkable
surfaces on a reference test level, **so that** no valid paths are missing.

## US-7.1.1.11 Validate NavMesh Respects Slope Limits

**As an** engine tester (P-27), **I want to** confirm that surfaces exceeding the configured slope
limit are excluded from the NavMesh, **so that** agents never path up impassable inclines.

## US-7.1.1.12 Benchmark NavMesh Generation Time

**As an** engine tester (P-27), **I want to** measure NavMesh generation time on reference levels
across platforms, **so that** I can detect performance regressions in the bake pipeline.

---

## F-7.1.2 — NavMesh Tiling & Streaming

## US-7.1.2.1 Set NavMesh Tile Size in Editor

**As a** designer (P-5), **I want to** configure the NavMesh tile size to align with my world
streaming grid, **so that** navigation tiles load and unload with world chunks.

## US-7.1.2.2 Verify Seamless Navigation Across Tile Boundaries

**As a** designer (P-5), **I want to** test that agents path seamlessly across tile boundaries
without pausing, **so that** the tiled structure is invisible to players.

## US-7.1.2.3 Configure Tile Preload Radius

**As a** designer (P-5), **I want to** set how far ahead NavMesh tiles are preloaded around active
agents, **so that** pathfinding never waits on tile I/O.

## US-7.1.2.4 Experience Seamless Open-World Navigation

**As a** player (P-23), **I want** NPCs to navigate across the open world without stopping at
invisible boundaries, **so that** travel feels continuous.

## US-7.1.2.5 Not Notice Loading Hitches During NPC Movement

**As a** player (P-23), **I want** NPC navigation to be unaffected when new areas stream in,
**so that** pathfinding does not visibly stall during exploration.

## US-7.1.2.6 Walk Across Large Maps Without NPC Path Failures

**As a** player (P-23), **I want** AI companions to follow me across large maps without losing their
path, **so that** long journeys feel reliable.

## US-7.1.2.7 Implement Async Tile Loading and Unloading

**As an** engine developer (P-26), **I want to** load and unload NavMesh tiles asynchronously as the
simulation window moves, **so that** the full mesh is never required in memory.

## US-7.1.2.8 Align Tile Grid with World Streaming System

**As an** engine developer (P-26), **I want to** ensure NavMesh tile boundaries match or subdivide
the world chunk grid, **so that** streaming events are synchronized.

## US-7.1.2.9 Implement Tile Connectivity Graph

**As an** engine developer (P-26), **I want to** maintain a connectivity graph across tile
boundaries, **so that** cross-tile pathfinding produces continuous corridors.

## US-7.1.2.10 Test NavMesh Tile Streaming Under Memory Pressure

**As an** engine tester (P-27), **I want to** verify that NavMesh tiles load and unload correctly
under constrained memory conditions, **so that** no tiles leak or fail to load.

## US-7.1.2.11 Verify Agent Paths Across Tile Boundaries

**As an** engine tester (P-27), **I want to** confirm that pathfinding corridors crossing tile
boundaries produce valid continuous paths, **so that** tile seams never break navigation.

## US-7.1.2.12 Measure Streaming Bandwidth of NavMesh Tiles

**As an** engine tester (P-27), **I want to** profile the I/O bandwidth consumed by NavMesh tile
streaming, **so that** it stays within the budget allocated by the streaming system.

---

## F-7.1.3 — A* Pathfinding on NavMesh

## US-7.1.3.1 Configure Area-Type Costs in Editor

**As a** designer (P-5), **I want to** assign traversal costs to area types like roads, swamps, and
lava, **so that** AI agents prefer contextually appropriate routes.

## US-7.1.3.2 Test Path Preferences on Mixed Terrain

**As a** designer (P-5), **I want to** place an agent on a map with roads and swamps and verify it
prefers the road, **so that** area-cost weights work as expected.

## US-7.1.3.3 Adjust Per-Tick Pathfinding Budget

**As a** designer (P-5), **I want to** configure the per-tick CPU budget for pathfinding queries,
**so that** I can balance path responsiveness against frame rate.

## US-7.1.3.4 See Enemies Take Smart Routes

**As a** player (P-23), **I want** enemies to take logical routes using roads when available and
avoiding hazards, **so that** AI movement looks purposeful.

## US-7.1.3.5 Not See AI Freeze While Computing Paths

**As a** player (P-23), **I want** AI agents to start moving quickly after receiving a new
destination, **so that** there is no visible delay.

## US-7.1.3.6 See NPCs Avoid Dangerous Terrain

**As a** player (P-23), **I want** NPCs to avoid walking through lava or toxic areas when a safe
path exists, **so that** their behavior appears survival-aware.

## US-7.1.3.7 Implement Hierarchical A* Over NavMesh Polygons

**As an** engine developer (P-26), **I want to** implement A* search over NavMesh polygon graphs
with configurable heuristics, **so that** pathfinding is efficient and extensible.

## US-7.1.3.8 Batch and Spread Queries Across Frames

**As an** engine developer (P-26), **I want to** batch pathfinding queries and spread them across
frames, **so that** CPU cost stays within the per-tick budget.

## US-7.1.3.9 Support Configurable Heuristic Functions

**As an** engine developer (P-26), **I want to** allow custom heuristic functions for A* search,
**so that** different game scenarios can tune path quality vs. search speed.

## US-7.1.3.10 Verify A* Returns Optimal Paths

**As an** engine tester (P-27), **I want to** verify that A* produces shortest-cost paths on known
reference graphs, **so that** the search algorithm is correct.

## US-7.1.3.11 Validate Per-Tick Budget Is Respected

**As an** engine tester (P-27), **I want to** confirm that pathfinding never exceeds the configured
per-tick CPU budget, **so that** frame rate remains stable under heavy query load.

## US-7.1.3.12 Stress Test Concurrent Path Queries

**As an** engine tester (P-27), **I want to** stress test with 128+ concurrent path queries on
desktop and verify correct results, **so that** the batching system handles high load.

---

## F-7.1.4 — String Pulling (Funnel Algorithm)

## US-7.1.4.1 See Tight Corner-Hugging Paths in Debug View

**As a** designer (P-5), **I want to** see agents follow tight, corner-hugging waypoints in the
debug overlay, **so that** I can confirm the funnel algorithm produces clean paths.

## US-7.1.4.2 Compare Funnel vs. Raw Corridor Paths

**As a** designer (P-5), **I want to** toggle between raw polygon corridor and funnel-smoothed paths
in the editor, **so that** I can visualize the improvement.

## US-7.1.4.3 Preview Funnel Waypoints in Editor

**As a** designer (P-5), **I want to** place start and end markers on a NavMesh in the editor and
see the funnel-generated waypoints displayed, **so that** I can iterate on level layout.

## US-7.1.4.4 See NPCs Cut Corners Naturally

**As a** player (P-23), **I want** NPCs to cut corners naturally instead of zigzagging between
polygon centers, **so that** movement looks human-like.

## US-7.1.4.5 Watch AI Navigate Tight Corridors Smoothly

**As a** player (P-23), **I want** AI agents to move smoothly through narrow corridors without
bouncing between walls, **so that** indoor navigation feels polished.

## US-7.1.4.6 See Smooth Transitions at Doorways

**As a** player (P-23), **I want** NPCs to pass through doorways in a straight line rather than
bouncing off door frames, **so that** entry and exit looks natural.

## US-7.1.4.7 Implement Funnel Algorithm for Waypoint Generation

**As an** engine developer (P-26), **I want to** implement the funnel algorithm to convert polygon
corridors into minimal waypoint sequences, **so that** path quality improves with low CPU cost.

## US-7.1.4.8 Ensure Funnel Output Stays Within NavMesh Bounds

**As an** engine developer (P-26), **I want to** guarantee that all funnel-generated waypoints lie
within the NavMesh polygon boundaries, **so that** agents never path off-mesh.

## US-7.1.4.9 Integrate Funnel Output with Steering System

**As an** engine developer (P-26), **I want to** feed funnel waypoints into the steering behavior
system, **so that** path following and local avoidance work together seamlessly.

## US-7.1.4.10 Verify Waypoint Count Is Minimal

**As an** engine tester (P-27), **I want to** verify that the funnel algorithm produces fewer
waypoints than the raw polygon count for reference corridors, **so that** paths are optimally short.

## US-7.1.4.11 Validate No Waypoints Fall Outside NavMesh

**As an** engine tester (P-27), **I want to** confirm that no funnel-generated waypoint lies outside
the NavMesh boundaries, **so that** agents never attempt invalid movement.

## US-7.1.4.12 Benchmark Funnel Algorithm Performance

**As an** engine tester (P-27), **I want to** measure funnel algorithm CPU time across varying
corridor lengths, **so that** I can confirm it remains lightweight.

---

## F-7.1.5 — Path Smoothing

## US-7.1.5.1 Enable Curved Path Smoothing for Patrol Routes

**As a** designer (P-5), **I want to** enable Catmull-Rom or Bezier smoothing for specific patrol
routes, **so that** guard movements look natural and elegant.

## US-7.1.5.2 Choose Smoothing Mode Per Agent Archetype

**As a** designer (P-5), **I want to** select between linear, Catmull-Rom, and Bezier smoothing per
agent archetype, **so that** different NPCs have movement styles matching their character.

## US-7.1.5.3 See NPCs Walk in Smooth Curves

**As a** player (P-23), **I want** town patrol guards to walk in smooth curves around corners
instead of making sharp right-angle turns, **so that** movement feels lifelike.

## US-7.1.5.4 Watch AI Follow Natural Arcs During Chase

**As a** player (P-23), **I want** pursuing enemies to run in natural arcs rather than jagged
straight-line segments, **so that** chases feel cinematic.

## US-7.1.5.5 Not See NPCs Stutter at Waypoint Transitions

**As a** player (P-23), **I want** NPC movement to flow continuously between waypoints without
stuttering or speed changes, **so that** path transitions are invisible.

## US-7.1.5.6 Implement Raycast-Based Redundant Turn Removal

**As an** engine developer (P-26), **I want to** post-process waypoint paths with NavMesh raycasts
to remove redundant turns, **so that** paths are simplified before smoothing.

## US-7.1.5.7 Implement Catmull-Rom and Bezier Interpolation

**As an** engine developer (P-26), **I want to** apply Catmull-Rom or cubic Bezier interpolation to
waypoint paths, **so that** movement trajectories are smooth curves.

## US-7.1.5.8 Provide Linear Fallback for Mobile Platform

**As an** engine developer (P-26), **I want to** fall back to linear waypoint paths on mobile
platforms, **so that** per-path CPU cost stays within the tighter mobile budget.

## US-7.1.5.9 Verify Smoothed Paths Stay On NavMesh

**As an** engine tester (P-27), **I want to** verify that smoothed curved paths do not leave the
NavMesh boundary, **so that** agents never move through impassable geometry.

## US-7.1.5.10 Compare Visual Quality of Smoothing Modes

**As an** engine tester (P-27), **I want to** compare linear, Catmull-Rom, and Bezier paths on a
reference route and capture screenshots, **so that** visual quality differences are documented.

## US-7.1.5.11 Benchmark Path Smoothing Overhead

**As an** engine tester (P-27), **I want to** measure the CPU overhead of path smoothing for 100+
agents simultaneously, **so that** I can confirm it fits within the navigation budget.

## US-7.1.5.12 Validate Mobile Skips Bezier Smoothing

**As an** engine tester (P-27), **I want to** confirm that mobile platform builds use linear-only
paths and never invoke Bezier interpolation, **so that** the platform gate works.

---

## F-7.1.6 — Dynamic Obstacle Carving

## US-7.1.6.1 Mark Moving Objects as NavMesh Obstacles

**As a** designer (P-5), **I want to** tag dynamic objects (barricades, vehicles) as NavMesh
obstacles in the editor, **so that** agents automatically reroute around them.

## US-7.1.6.2 Test Agent Rerouting When Obstacle Appears

**As a** designer (P-5), **I want to** place a barricade in an agent's path at runtime and see it
reroute within seconds, **so that** I can verify dynamic carving responsiveness.

## US-7.1.6.3 Configure Carve Shape and Margin

**As a** designer (P-5), **I want to** configure the carve shape and clearance margin per obstacle
type, **so that** the blocked area matches the object's footprint.

## US-7.1.6.4 See AI Reroute Around a Fallen Tree

**As a** player (P-23), **I want** enemies to find an alternate route when a tree falls across their
path, **so that** the world feels dynamically responsive.

## US-7.1.6.5 Watch NPCs React to Placed Barricades

**As a** player (P-23), **I want** NPCs to walk around barricades I place instead of walking through
them, **so that** my tactical placements affect AI movement.

## US-7.1.6.6 See AI Recalculate When Vehicle Moves

**As a** player (P-23), **I want** AI agents to adjust paths when a vehicle parks in their way,
**so that** the world feels alive and reactive.

## US-7.1.6.7 Implement Tile-Local Re-Carving

**As an** engine developer (P-26), **I want to** implement tile-local NavMesh re-carving that
invalidates only affected polygons, **so that** carving cost is proportional to obstacle size.

## US-7.1.6.8 Trigger Localized Repath for Affected Agents

**As an** engine developer (P-26), **I want to** trigger repath requests only for agents whose
corridors intersect modified polygons, **so that** unaffected agents are not re-queried.

## US-7.1.6.9 Throttle Carve Operations on Mobile

**As an** engine developer (P-26), **I want to** limit simultaneous carve operations to 4 per tick
on mobile, **so that** carving does not exceed the platform CPU budget.

## US-7.1.6.10 Verify Carved Regions Block Pathfinding

**As an** engine tester (P-27), **I want to** verify that agents cannot path through carved obstacle
regions, **so that** dynamic obstacles are correctly blocking.

## US-7.1.6.11 Test Repath Storm Throttling

**As an** engine tester (P-27), **I want to** confirm that rapid obstacle placement does not cause a
repath storm exceeding the CPU budget, **so that** frame rate remains stable.

## US-7.1.6.12 Validate Carve Cleanup on Obstacle Removal

**As an** engine tester (P-27), **I want to** verify that removing a dynamic obstacle restores the
original NavMesh polygons, **so that** no ghost obstacles persist.

---

## F-7.1.7 — NavMesh Links (Off-Mesh Connections)

## US-7.1.7.1 Place Off-Mesh Links in Editor

**As a** designer (P-5), **I want to** place off-mesh links between disjoint NavMesh regions in the
editor (jump points, ladder connections), **so that** agents can traverse gaps.

## US-7.1.7.2 Assign Animation Tags to Links

**As a** designer (P-5), **I want to** assign animation tags (jump, climb, swim) to each off-mesh
link, **so that** agents play the correct traversal animation.

## US-7.1.7.3 Set Preconditions on Links

**As a** designer (P-5), **I want to** set preconditions on off-mesh links (key required, ability
unlocked), **so that** only eligible agents use restricted connections.

## US-7.1.7.4 See AI Climb Ladders and Jump Gaps

**As a** player (P-23), **I want** enemies to climb ladders and jump across gaps during pursuit,
**so that** AI movement uses the full level geometry.

## US-7.1.7.5 Watch NPCs Open Doors to Path Through

**As a** player (P-23), **I want** NPCs to open doors along their path instead of stopping at closed
doorways, **so that** they navigate the world as I do.

## US-7.1.7.6 See Guards Unable to Follow Through Locked Doors

**As a** player (P-23), **I want** guards to be unable to follow me through locked doors they cannot
open, **so that** precondition gating feels fair and logical.

## US-7.1.7.7 Implement Off-Mesh Link Data Structure

**As an** engine developer (P-26), **I want to** implement off-mesh connections carrying cost,
animation tag, and precondition data, **so that** the planner can evaluate link feasibility.

## US-7.1.7.8 Integrate Links into A* Search

**As an** engine developer (P-26), **I want to** incorporate off-mesh links into the A* polygon
search with their associated costs, **so that** pathfinding considers multi-modal traversal.

## US-7.1.7.9 Support Bidirectional and One-Way Links

**As an** engine developer (P-26), **I want to** support both bidirectional and one-way off-mesh
links, **so that** jump-downs and one-way drops are modeled correctly.

## US-7.1.7.10 Verify Precondition Gating Blocks Ineligible Agents

**As an** engine tester (P-27), **I want to** verify that agents without the required ability cannot
use precondition-gated links, **so that** link restrictions are enforced.

## US-7.1.7.11 Test Link Traversal Animation Playback

**As an** engine tester (P-27), **I want to** confirm that the correct traversal animation plays
when an agent uses each link type, **so that** movement matches the animation tag.

## US-7.1.7.12 Validate Path Cost Includes Link Costs

**As an** engine tester (P-27), **I want to** verify that A* path costs correctly include off-mesh
link traversal costs, **so that** the planner accurately compares link vs. walkable paths.

---

## F-7.1.8 — Incremental Tile Rebuild

## US-7.1.8.1 See NavMesh Update After Terrain Deformation

**As a** designer (P-5), **I want to** deform terrain in the editor and see the NavMesh update for
only the affected tiles, **so that** I can iterate on destructible environments quickly.

## US-7.1.8.2 Mark Dirty Regions for Rebuild

**As a** designer (P-5), **I want to** manually mark spatial regions as dirty to force a NavMesh
rebuild, **so that** I can test incremental rebuild behavior in the editor.

## US-7.1.8.3 Configure Rebuild Priority Order

**As a** designer (P-5), **I want to** set rebuild priority so tiles near active gameplay rebuild
first, **so that** player-facing navigation updates quickly.

## US-7.1.8.4 See AI Path Through Destroyed Buildings

**As a** player (P-23), **I want** AI to find new paths through areas opened by building
destruction, **so that** the world responds dynamically to environmental changes.

## US-7.1.8.5 Watch AI Avoid Rubble After Collapse

**As a** player (P-23), **I want** AI to reroute around rubble created by a collapsed structure,
**so that** destruction meaningfully changes navigation.

## US-7.1.8.6 Experience Smooth AI During World Changes

**As a** player (P-23), **I want** AI movement to remain smooth while NavMesh tiles rebuild in the
background, **so that** I do not notice any navigation hitches.

## US-7.1.8.7 Implement NavMeshDirtyRegion ECS Component

**As an** engine developer (P-26), **I want to** use a `NavMeshDirtyRegion` ECS component to mark
spatial extents for rebuild, **so that** the system processes them in priority order.

## US-7.1.8.8 Revoxelize Only Affected Tiles and Neighbors

**As an** engine developer (P-26), **I want to** revoxelize only the affected tile and its immediate
neighbors, **so that** rebuild cost scales with damage area, not world size.

## US-7.1.8.9 Cap Concurrent Rebuilds Per Platform

**As an** engine developer (P-26), **I want to** cap concurrent tile rebuilds (1 on mobile, 4+ on
desktop), **so that** rebuild work fits within the platform's CPU budget.

## US-7.1.8.10 Verify Only Affected Tiles Rebuild

**As an** engine tester (P-27), **I want to** verify that only the dirty tiles and their neighbors
are rebuilt while all other tiles remain unchanged, **so that** rebuild scope is minimal.

## US-7.1.8.11 Test NavMesh Correctness After Incremental Rebuild

**As an** engine tester (P-27), **I want to** compare incrementally rebuilt tiles against a full
rebake and verify identical results, **so that** incremental and full builds are equivalent.

## US-7.1.8.12 Benchmark Incremental vs. Full Rebuild Time

**As an** engine tester (P-27), **I want to** measure the time savings of incremental tile rebuild
vs. full rebake on reference levels, **so that** the optimization is validated.

---

## F-7.1.9 — Background NavMesh Generation

## US-7.1.9.1 Verify Editor Remains Responsive During Bake

**As a** designer (P-5), **I want** the editor to remain fully responsive while NavMesh tiles
generate in the background, **so that** I can continue working during long bakes.

## US-7.1.9.2 See Progress Indicator for Background Generation

**As a** designer (P-5), **I want to** see a progress indicator showing how many tiles remain in the
background generation queue, **so that** I know when the bake will finish.

## US-7.1.9.3 Cancel Background Generation

**As a** designer (P-5), **I want to** cancel an in-progress background NavMesh generation without
corrupting the existing mesh, **so that** I can restart with new settings.

## US-7.1.9.4 Not Experience Gameplay Stutters During Rebuild

**As a** player (P-23), **I want** gameplay to remain smooth while NavMesh tiles rebuild at runtime,
**so that** world changes do not cause frame drops.

## US-7.1.9.5 See Agents Keep Moving During Tile Rebuild

**As a** player (P-23), **I want** AI agents to keep moving (using fallback navigation) while their
tile is being rebuilt, **so that** NPCs never freeze in place.

## US-7.1.9.6 Experience No Visible Glitches During Tile Swap

**As a** player (P-23), **I want** NPCs to transition seamlessly to newly built tiles without
visible teleporting or path snapping, **so that** tile swaps are invisible.

## US-7.1.9.7 Run Generation on Background Worker Threads

**As an** engine developer (P-26), **I want to** run NavMesh generation on background worker threads
via the job system, **so that** the main simulation tick is never blocked.

## US-7.1.9.8 Mark Pending Tiles in NavMeshTileMap

**As an** engine developer (P-26), **I want to** mark tiles under construction as `Pending` in the
`NavMeshTileMap` ECS resource, **so that** agents receive fallback paths through pending areas.

## US-7.1.9.9 Swap Completed Tiles Atomically

**As an** engine developer (P-26), **I want to** swap completed tiles into the live NavMesh
atomically at the next sync point, **so that** agents never read a partially built tile.

## US-7.1.9.10 Verify No Main-Thread Blocking

**As an** engine tester (P-27), **I want to** verify that NavMesh generation never blocks the main
simulation thread, **so that** frame time is unaffected by background bakes.

## US-7.1.9.11 Test Fallback Navigation During Pending State

**As an** engine tester (P-27), **I want to** verify that agents navigating through pending tiles
use valid fallback movement, **so that** no agent gets stuck or moves through walls.

## US-7.1.9.12 Validate Atomic Tile Swap Consistency

**As an** engine tester (P-27), **I want to** confirm that tile swaps are atomic and no agent ever
reads partial tile data, **so that** data races are impossible.

---

## F-7.1.10 — Destruction-Triggered NavMesh Updates

## US-7.1.10.1 See NavMesh Update After Building Destruction

**As a** designer (P-5), **I want to** destroy a building in the editor and see the NavMesh update
to open new pathways, **so that** I can verify destruction integration works.

## US-7.1.10.2 Configure Rebuild Priority by Agent Density

**As a** designer (P-5), **I want** rebuild priority to scale with the number of active agents whose
paths cross the destroyed region, **so that** high-traffic areas update first.

## US-7.1.10.3 Test Destruction Opens New Routes

**As a** designer (P-5), **I want to** verify that destroying a wall creates a usable NavMesh
corridor through the breach, **so that** destruction has tactical pathfinding consequences.

## US-7.1.10.4 See Enemies Rush Through Breached Walls

**As a** player (P-23), **I want** enemies to rush through a wall I just destroyed, finding new
paths through the breach, **so that** destruction has tactical consequences.

## US-7.1.10.5 Watch AI Avoid Fresh Rubble Piles

**As a** player (P-23), **I want** AI to treat rubble from collapsed structures as obstacles,
routing around debris, **so that** destruction creates new navigation challenges.

## US-7.1.10.6 See AI React Quickly to Nearby Destruction

**As a** player (P-23), **I want** AI near a destruction event to repath within 1-2 seconds,
**so that** their response feels immediate and aware.

## US-7.1.10.7 Observe NavMeshInvalidation Events from Destruction

**As an** engine developer (P-26), **I want** the destruction system to emit `NavMeshInvalidation`
ECS events with affected bounding regions, **so that** the rebuild system picks them up.

## US-7.1.10.8 Enqueue Rebuilds with Priority Scaling

**As an** engine developer (P-26), **I want** the `NavMeshRebuildSystem` to enqueue tile rebuilds
with priority proportional to affected agent count, **so that** critical areas rebuild first.

## US-7.1.10.9 Handle Destruction During Pending Rebuild

**As an** engine developer (P-26), **I want to** handle destruction events that affect tiles already
in the rebuild queue by merging dirty regions, **so that** redundant rebuilds are avoided.

## US-7.1.10.10 Verify New Pathways Open After Destruction

**As an** engine tester (P-27), **I want to** verify that destroying a wall creates a passable
NavMesh corridor through the breach, **so that** agents can path through openings.

## US-7.1.10.11 Verify Rubble Creates New Obstacles

**As an** engine tester (P-27), **I want to** verify that rubble from destruction is represented as
NavMesh obstacles, **so that** agents cannot walk through debris piles.

## US-7.1.10.12 Test Mobile Deferred Rebuild for Distant Destruction

**As an** engine tester (P-27), **I want to** confirm that mobile defers or skips rebuilds for
distant destruction events, **so that** mobile CPU budget is respected.

---

## F-7.1.11 — Player-Built Structure Integration

## US-7.1.11.1 Place Structures and See NavMesh Update

**As a** designer (P-5), **I want to** place a player-buildable structure in the editor and see the
NavMesh update to reflect the new obstacle, **so that** I can verify building integration.

## US-7.1.11.2 Configure Structure Area Type Override

**As a** designer (P-5), **I want to** set area type overrides on player structures (wall, ramp,
platform), **so that** the NavMesh correctly classifies the structure.

## US-7.1.11.3 See Auto-Generated Links on Stairs and Ladders

**As a** designer (P-5), **I want** stairs and ladders on player structures to auto-generate
off-mesh links, **so that** agents can traverse multi-level player buildings.

## US-7.1.11.4 Watch AI Navigate Around My Fort

**As a** player (P-23), **I want** enemies to navigate around the walls of my fort instead of
walking through them, **so that** my building efforts affect AI pathing.

## US-7.1.11.5 See AI Use Ramps I Build

**As a** player (P-23), **I want** AI allies to walk up ramps I build to reach elevated platforms,
**so that** my constructions are usable by NPCs.

## US-7.1.11.6 Watch Enemies Find New Routes When I Remove Walls

**As a** player (P-23), **I want** AI to find new routes when I remove a wall section from my base,
**so that** the NavMesh updates dynamically with my edits.

## US-7.1.11.7 Implement NavMeshObstacle ECS Component

**As an** engine developer (P-26), **I want to** register player structures as NavMesh obstacles via
a `NavMeshObstacle` ECS component with shape data, **so that** tiles rebuild on placement.

## US-7.1.11.8 Auto-Generate Links for Stairs and Ladders

**As an** engine developer (P-26), **I want to** auto-generate off-mesh links for stairs and ladder
attachment points on player structures, **so that** vertical traversal works automatically.

## US-7.1.11.9 Queue Tile Rebuild on Structure Add/Remove

**As an** engine developer (P-26), **I want to** queue incremental tile rebuilds when structures are
placed or removed, **so that** the NavMesh reflects the latest player construction state.

## US-7.1.11.10 Verify Structures Block Agent Paths

**As an** engine tester (P-27), **I want to** verify that placed structures marked as impassable
walls correctly block agent pathfinding, **so that** walls function as barriers.

## US-7.1.11.11 Test Ramp Traversal via Auto-Generated Links

**As an** engine tester (P-27), **I want to** verify that agents can traverse auto-generated links
on player-built ramps and ladders, **so that** vertical movement works end-to-end.

## US-7.1.11.12 Validate NavMesh Restores After Structure Removal

**As an** engine tester (P-27), **I want to** verify that removing a player structure restores the
NavMesh to its pre-placement state, **so that** no ghost obstacles remain.

---

## F-7.1.12 — Multi-Size Agent NavMeshes

## US-7.1.12.1 Define Agent Size Classes in Editor

**As a** designer (P-5), **I want to** define multiple agent size classes (humanoid, mounted, siege
golem) with radius, height, and slope parameters, **so that** each gets its own NavMesh.

## US-7.1.12.2 Assign Agents to NavMesh Layers

**As a** designer (P-5), **I want to** assign each agent entity to a NavMesh layer via a
`NavMeshAgent` ECS component, **so that** it paths on the mesh matching its size.

## US-7.1.12.3 Preview Multiple NavMesh Layers in Editor

**As a** designer (P-5), **I want to** toggle visibility of individual NavMesh layers in the editor
overlay, **so that** I can inspect each size class independently.

## US-7.1.12.4 See Large Creatures Avoid Narrow Passages

**As a** player (P-23), **I want** large siege golems to take wide routes and avoid narrow alleys,
**so that** their pathing matches their visible size.

## US-7.1.12.5 See Mounted Riders Use Appropriate Paths

**As a** player (P-23), **I want** mounted riders to path along roads and open terrain rather than
through tight doorways, **so that** mount navigation feels correct.

## US-7.1.12.6 Watch Small and Large Agents Use Different Routes

**As a** player (P-23), **I want** small NPCs to take shortcuts through alleys that large NPCs
cannot, **so that** size-based navigation differences are visible.

## US-7.1.12.7 Maintain Multiple NavMesh Layers on Shared Grid

**As an** engine developer (P-26), **I want to** maintain multiple NavMesh layers sharing the same
spatial tile grid, **so that** streaming loads all layers for a region together.

## US-7.1.12.8 Configure NavMeshAgentConfig Per Size Class

**As an** engine developer (P-26), **I want to** define `NavMeshAgentConfig` structs per size class
with radius, height, step-climb, and max slope, **so that** each layer is generated correctly.

## US-7.1.12.9 Limit NavMesh Layers on Mobile

**As an** engine developer (P-26), **I want to** limit NavMesh layers to 2 on mobile (humanoid

+ large), **so that** memory and streaming bandwidth stay within mobile constraints.

## US-7.1.12.10 Verify Each Layer Is Independently Correct

**As an** engine tester (P-27), **I want to** verify that each NavMesh layer produces correct
walkable surfaces for its configured agent size, **so that** no layer has missing or extra coverage.

## US-7.1.12.11 Test Agents Path on Correct Layer

**As an** engine tester (P-27), **I want to** verify that each agent queries only its assigned
NavMesh layer and never paths on a wrong-sized mesh, **so that** layer assignment is enforced.

## US-7.1.12.12 Measure Multi-Layer Memory Overhead

**As an** engine tester (P-27), **I want to** measure the total memory consumed by multiple NavMesh
layers, **so that** multi-layer overhead is within the allocated budget.

---

## F-7.1.13 — Dynamic Area Cost Modification

## US-7.1.13.1 Modify Area Costs at Runtime in Editor

**As a** designer (P-5), **I want to** modify NavMesh polygon area costs at runtime in the editor
(e.g., flood a zone to raise water cost), **so that** I can test dynamic routing.

## US-7.1.13.2 Set Per-Faction Cost Overrides

**As a** designer (P-5), **I want to** set per-faction area cost overrides so friendly territory is
cheaper for allies, **so that** faction-aware routing is configurable.

## US-7.1.13.3 Preview Cost Heatmap in Editor

**As a** designer (P-5), **I want to** visualize a cost heatmap overlay on the NavMesh in the
editor, **so that** I can see how area costs influence route selection.

## US-7.1.13.4 See NPCs Avoid Flooded Areas

**As a** player (P-23), **I want** NPCs to prefer dry paths when a zone floods, avoiding the water
unless no alternative exists, **so that** AI reacts to environmental changes.

## US-7.1.13.5 Watch Enemies Avoid Battlefields

**As a** player (P-23), **I want** civilian NPCs to detour around active battlefields with danger
area weighting, **so that** non-combatants flee to safety.

## US-7.1.13.6 See Faction Troops Use Friendly Roads

**As a** player (P-23), **I want** allied troops to prefer roads in friendly territory while enemies
take longer concealed routes, **so that** faction strategy is visible.

## US-7.1.13.7 Implement NavMeshAreaCosts ECS Resource

**As an** engine developer (P-26), **I want to** store runtime area cost modifications in a
`NavMeshAreaCosts` ECS resource applied during A* expansion, **so that** costs change without
rebuilding geometry.

## US-7.1.13.8 Support Per-Agent Cost Overrides

**As an** engine developer (P-26), **I want to** support per-agent cost overrides for
faction-specific routing, **so that** different factions evaluate the same area differently.

## US-7.1.13.9 Ensure Cost Changes Are Data-Only

**As an** engine developer (P-26), **I want to** ensure area cost modifications are pure data
updates with no geometry rebuild, **so that** cost changes are instantaneous and lightweight.

## US-7.1.13.10 Verify Cost Changes Alter Agent Routes

**As an** engine tester (P-27), **I want to** verify that increasing an area's cost causes agents to
choose alternate routes when available, **so that** cost modifications affect pathfinding.

## US-7.1.13.11 Test Per-Faction Routing Divergence

**As an** engine tester (P-27), **I want to** place two agents from different factions and verify
they take different routes based on faction cost overrides, **so that** faction-aware routing works.

## US-7.1.13.12 Validate No Geometry Rebuild on Cost Change

**As an** engine tester (P-27), **I want to** confirm that modifying area costs does not trigger any
NavMesh tile rebuild, **so that** cost changes remain a zero-rebuild operation.

---

## F-7.1.14 — Hierarchical Pathfinding (HPA*)

## US-7.1.14.1 Enable HPA* for Cross-Region Paths

**As a** designer (P-5), **I want to** enable hierarchical pathfinding for paths that cross multiple
NavMesh tiles, **so that** long-distance NPC travel is efficient.

## US-7.1.14.2 Visualize Hierarchical Graph in Editor

**As a** designer (P-5), **I want to** visualize the coarse hierarchical navigation graph in the
editor, **so that** I can inspect cross-tile connectivity.

## US-7.1.14.3 Test Cross-Continent NPC Travel

**As a** designer (P-5), **I want to** command an NPC to travel across the entire world map and
verify it arrives, **so that** long-distance pathfinding is reliable.

## US-7.1.14.4 See NPCs Travel Long Distances Without Hesitation

**As a** player (P-23), **I want** NPCs to begin long-distance journeys immediately without visible
planning delay, **so that** travel commands feel responsive.

## US-7.1.14.5 Watch Caravans Navigate Across Regions

**As a** player (P-23), **I want** trade caravans to navigate across multiple regions on efficient
routes, **so that** cross-world travel looks intentional and planned.

## US-7.1.14.6 See AI Companions Follow Across the Map

**As a** player (P-23), **I want** my AI companion to follow me across the entire map without losing
its way, **so that** long adventures feel seamless.

## US-7.1.14.7 Build Coarse Graph from Tile Connectivity

**As an** engine developer (P-26), **I want to** build a coarse navigation graph from NavMesh tile
connectivity, **so that** long-distance queries are O(1) per tile.

## US-7.1.14.8 Refine Hierarchical Paths to Full Resolution

**As an** engine developer (P-26), **I want to** refine hierarchical paths to full NavMesh
resolution only for the tiles the agent is currently traversing, **so that** detail is computed
just-in-time.

## US-7.1.14.9 Bound Pathfinding Cost Regardless of World Size

**As an** engine developer (P-26), **I want** pathfinding cost to remain bounded regardless of world
size, **so that** thousands of server-side NPCs can plan cross-continent paths.

## US-7.1.14.10 Verify HPA* Produces Valid End-to-End Paths

**As an** engine tester (P-27), **I want to** verify that HPA* produces valid navigable paths
between any two reachable points on the world map, **so that** no routes are broken.

## US-7.1.14.11 Compare HPA* vs. Full A* Path Quality

**As an** engine tester (P-27), **I want to** compare HPA* paths against full A* paths on reference
scenarios and measure cost deviation, **so that** hierarchical approximation is acceptable.

## US-7.1.14.12 Benchmark HPA* for 1000+ Concurrent Queries

**As an** engine tester (P-27), **I want to** benchmark HPA* with 1000+ concurrent long-distance
queries, **so that** server-side NPC travel scales to MMO population levels.

---

## F-7.1.15 — NavMesh Runtime Visualization

## US-7.1.15.1 Toggle NavMesh Debug Overlay in Editor

**As a** designer (P-5), **I want to** toggle a debug overlay showing NavMesh polygons, tile
boundaries, and area types in the editor, **so that** I can inspect navigation data visually.

## US-7.1.15.2 Color-Code Area Types in Overlay

**As a** designer (P-5), **I want** different area types to be color-coded in the debug overlay
(roads green, swamps brown, lava red), **so that** I can identify zones at a glance.

## US-7.1.15.3 Visualize Agent Path Trails

**As a** designer (P-5), **I want to** see agent path trails drawn as colored lines on the NavMesh
overlay, **so that** I can diagnose why agents choose specific routes.

## US-7.1.15.4 Show Off-Mesh Links in Debug View

**As a** designer (P-5), **I want** off-mesh links to appear as labeled arcs in the debug overlay,
**so that** I can verify link placement and connectivity.

## US-7.1.15.5 Not See Debug Overlays in Shipping Game

**As a** player (P-23), **I want** no NavMesh debug overlays to appear in the shipping game,
**so that** the visual experience is clean and immersive.

## US-7.1.15.6 Implement NavMeshDebug ECS Components

**As an** engine developer (P-26), **I want to** control visualization through `NavMeshDebug` ECS
components, **so that** debug rendering is opt-in and configurable per entity or region.

## US-7.1.15.7 Integrate with Editor Gizmo System

**As an** engine developer (P-26), **I want to** integrate NavMesh visualization with the editor
gizmo system (F-15.1.4), **so that** overlays render within the existing debug infrastructure.

## US-7.1.15.8 Strip Debug Visualization from Shipping Builds

**As an** engine developer (P-26), **I want to** strip all NavMesh debug visualization code from
shipping builds, **so that** release binaries have no debug overhead.

## US-7.1.15.9 Highlight Pending Rebuild Zones

**As a** designer (P-5), **I want** tiles queued for rebuild to be highlighted in the overlay,
**so that** I can see which regions are temporarily stale.

## US-7.1.15.10 Verify Debug Overlay Matches Actual NavMesh

**As an** engine tester (P-27), **I want to** verify that the debug overlay accurately represents
the underlying NavMesh data, **so that** visualization is trustworthy for debugging.

## US-7.1.15.11 Confirm Debug Code Is Stripped from Release

**As an** engine tester (P-27), **I want to** confirm that shipping builds contain no NavMesh debug
rendering code or data, **so that** release builds are clean.

## US-7.1.15.12 Test Overlay Performance with Full NavMesh Visible

**As an** engine tester (P-27), **I want to** measure overlay rendering performance with the entire
NavMesh visible, **so that** debug visualization does not tank editor frame rate.
