# User Stories — 7.1 Navigation

## F-7.1.1 — Recast-Style NavMesh Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.1.1 | designer (P-5) | I want to generate a navigation mesh from static level geometry with one click in the editor | AI agents can pathfind without manual navmesh authoring | F-7.1.1 | R-7.1.1 |
| US-7.1.1.2 | designer (P-5) | I want to set the agent radius and height used for NavMesh generation | the mesh accounts for my character's physical size | F-7.1.1 | R-7.1.1 |
| US-7.1.1.3 | designer (P-5) | I want to preview which surfaces are classified as walkable before committing to a full NavMesh bake | I can adjust geometry or slope limits early | F-7.1.1 | R-7.1.1 |
| US-7.1.1.4 | player (P-23) | I want enemies to navigate around furniture, rocks, and other obstacles smoothly | AI movement feels natural and intelligent | F-7.1.1 | R-7.1.1 |
| US-7.1.1.5 | player (P-23) | I want NPCs to walk on roads and bridges instead of clipping through terrain | the world feels believable | F-7.1.1 | R-7.1.1 |
| US-7.1.1.6 | player (P-23) | I want large creatures to avoid narrow doorways they cannot fit through | their movement matches their visible size | F-7.1.1 | R-7.1.1 |
| US-7.1.1.7 | engine developer (P-26) | I want to implement voxel-based NavMesh generation using heightfield rasterization | complex geometry produces accurate walkable surfaces | F-7.1.1 | R-7.1.1 |
| US-7.1.1.8 | engine developer (P-26) | I want to expose slope angle and step-climb parameters to the voxelization pipeline | different agent archetypes get appropriately filtered meshes | F-7.1.1 | R-7.1.1 |
| US-7.1.1.9 | engine developer (P-26) | I want to integrate NavMesh baking into the offline content pipeline | shipping builds include pre-baked meshes without runtime generation cost | F-7.1.1 | R-7.1.1 |
| US-7.1.1.10 | engine tester (P-27) | I want to verify that generated NavMesh covers 100% of walkable surfaces on a reference test level | no valid paths are missing | F-7.1.1 | R-7.1.1 |
| US-7.1.1.11 | engine tester (P-27) | I want to confirm that surfaces exceeding the configured slope limit are excluded from the NavMesh | agents never path up impassable inclines | F-7.1.1 | R-7.1.1 |
| US-7.1.1.12 | engine tester (P-27) | I want to measure NavMesh generation time on reference levels across platforms | I can detect performance regressions in the bake pipeline. --- | F-7.1.1 | R-7.1.1 |

## F-7.1.2 — NavMesh Tiling & Streaming

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.2.1 | designer (P-5) | I want to configure the NavMesh tile size to align with my world streaming grid | navigation tiles load and unload with world chunks | F-7.1.2 | R-7.1.2 |
| US-7.1.2.2 | designer (P-5) | I want to test that agents path seamlessly across tile boundaries without pausing | the tiled structure is invisible to players | F-7.1.2 | R-7.1.2 |
| US-7.1.2.3 | designer (P-5) | I want to set how far ahead NavMesh tiles are preloaded around active agents | pathfinding never waits on tile I/O | F-7.1.2 | R-7.1.2 |
| US-7.1.2.4 | player (P-23) | I want NPCs to navigate across the open world without stopping at invisible boundaries | travel feels continuous | F-7.1.2 | R-7.1.2 |
| US-7.1.2.5 | player (P-23) | I want NPC navigation to be unaffected when new areas stream in | pathfinding does not visibly stall during exploration | F-7.1.2 | R-7.1.2 |
| US-7.1.2.6 | player (P-23) | I want AI companions to follow me across large maps without losing their path | long journeys feel reliable | F-7.1.2 | R-7.1.2 |
| US-7.1.2.7 | engine developer (P-26) | I want to load and unload NavMesh tiles asynchronously as the simulation window moves | the full mesh is never required in memory | F-7.1.2 | R-7.1.2 |
| US-7.1.2.8 | engine developer (P-26) | I want to ensure NavMesh tile boundaries match or subdivide the world chunk grid | streaming events are synchronized | F-7.1.2 | R-7.1.2 |
| US-7.1.2.9 | engine developer (P-26) | I want to maintain a connectivity graph across tile boundaries | cross-tile pathfinding produces continuous corridors | F-7.1.2 | R-7.1.2 |
| US-7.1.2.10 | engine tester (P-27) | I want to verify that NavMesh tiles load and unload correctly under constrained memory conditions | no tiles leak or fail to load | F-7.1.2 | R-7.1.2 |
| US-7.1.2.11 | engine tester (P-27) | I want to confirm that pathfinding corridors crossing tile boundaries produce valid continuous paths | tile seams never break navigation | F-7.1.2 | R-7.1.2 |
| US-7.1.2.12 | engine tester (P-27) | I want to profile the I/O bandwidth consumed by NavMesh tile streaming | it stays within the budget allocated by the streaming system. --- | F-7.1.2 | R-7.1.2 |

## F-7.1.3 — A* Pathfinding on NavMesh

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.3.1 | designer (P-5) | I want to assign traversal costs to area types like roads, swamps, and lava | AI agents prefer contextually appropriate routes | F-7.1.3 | R-7.1.3 |
| US-7.1.3.2 | designer (P-5) | I want to place an agent on a map with roads and swamps and verify it prefers the road | area-cost weights work as expected | F-7.1.3 | R-7.1.3 |
| US-7.1.3.3 | designer (P-5) | I want to configure the per-tick CPU budget for pathfinding queries | I can balance path responsiveness against frame rate | F-7.1.3 | R-7.1.3 |
| US-7.1.3.4 | player (P-23) | I want enemies to take logical routes using roads when available and avoiding hazards | AI movement looks purposeful | F-7.1.3 | R-7.1.3 |
| US-7.1.3.5 | player (P-23) | I want AI agents to start moving quickly after receiving a new destination | there is no visible delay | F-7.1.3 | R-7.1.3 |
| US-7.1.3.6 | player (P-23) | I want NPCs to avoid walking through lava or toxic areas when a safe path exists | their behavior appears survival-aware | F-7.1.3 | R-7.1.3 |
| US-7.1.3.7 | engine developer (P-26) | I want to implement A* search over NavMesh polygon graphs with configurable heuristics | pathfinding is efficient and extensible | F-7.1.3 | R-7.1.3 |
| US-7.1.3.8 | engine developer (P-26) | I want to batch pathfinding queries and spread them across frames | CPU cost stays within the per-tick budget | F-7.1.3 | R-7.1.3 |
| US-7.1.3.9 | engine developer (P-26) | I want to allow custom heuristic functions for A* search | different game scenarios can tune path quality vs. search speed | F-7.1.3 | R-7.1.3 |
| US-7.1.3.10 | engine tester (P-27) | I want to verify that A* produces shortest-cost paths on known reference graphs | the search algorithm is correct | F-7.1.3 | R-7.1.3 |
| US-7.1.3.11 | engine tester (P-27) | I want to confirm that pathfinding never exceeds the configured per-tick CPU budget | frame rate remains stable under heavy query load | F-7.1.3 | R-7.1.3 |
| US-7.1.3.12 | engine tester (P-27) | I want to stress test with 128+ concurrent path queries on desktop and verify correct results | the batching system handles high load. --- | F-7.1.3 | R-7.1.3 |

## F-7.1.4 — String Pulling (Funnel Algorithm)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.4.1 | designer (P-5) | I want to see agents follow tight, corner-hugging waypoints in the debug overlay | I can confirm the funnel algorithm produces clean paths | F-7.1.4 | R-7.1.4 |
| US-7.1.4.2 | designer (P-5) | I want to toggle between raw polygon corridor and funnel-smoothed paths in the editor | I can visualize the improvement | F-7.1.4 | R-7.1.4 |
| US-7.1.4.3 | designer (P-5) | I want to place start and end markers on a NavMesh in the editor and see the funnel-generated waypoints displayed | I can iterate on level layout | F-7.1.4 | R-7.1.4 |
| US-7.1.4.4 | player (P-23) | I want NPCs to cut corners naturally instead of zigzagging between polygon centers | movement looks human-like | F-7.1.4 | R-7.1.4 |
| US-7.1.4.5 | player (P-23) | I want AI agents to move smoothly through narrow corridors without bouncing between walls | indoor navigation feels polished | F-7.1.4 | R-7.1.4 |
| US-7.1.4.6 | player (P-23) | I want NPCs to pass through doorways in a straight line rather than bouncing off door frames | entry and exit looks natural | F-7.1.4 | R-7.1.4 |
| US-7.1.4.7 | engine developer (P-26) | I want to implement the funnel algorithm to convert polygon corridors into minimal waypoint sequences | path quality improves with low CPU cost | F-7.1.4 | R-7.1.4 |
| US-7.1.4.8 | engine developer (P-26) | I want to guarantee that all funnel-generated waypoints lie within the NavMesh polygon boundaries | agents never path off-mesh | F-7.1.4 | R-7.1.4 |
| US-7.1.4.9 | engine developer (P-26) | I want to feed funnel waypoints into the steering behavior system | path following and local avoidance work together seamlessly | F-7.1.4 | R-7.1.4 |
| US-7.1.4.10 | engine tester (P-27) | I want to verify that the funnel algorithm produces fewer waypoints than the raw polygon count for reference corridors | paths are optimally short | F-7.1.4 | R-7.1.4 |
| US-7.1.4.11 | engine tester (P-27) | I want to confirm that no funnel-generated waypoint lies outside the NavMesh boundaries | agents never attempt invalid movement | F-7.1.4 | R-7.1.4 |
| US-7.1.4.12 | engine tester (P-27) | I want to measure funnel algorithm CPU time across varying corridor lengths | I can confirm it remains lightweight. --- | F-7.1.4 | R-7.1.4 |

## F-7.1.5 — Path Smoothing

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.5.1 | designer (P-5) | I want to enable Catmull-Rom or Bezier smoothing for specific patrol routes | guard movements look natural and elegant | F-7.1.5 | R-7.1.5 |
| US-7.1.5.2 | designer (P-5) | I want to select between linear, Catmull-Rom, and Bezier smoothing per agent archetype | different NPCs have movement styles matching their character | F-7.1.5 | R-7.1.5 |
| US-7.1.5.3 | player (P-23) | I want town patrol guards to walk in smooth curves around corners instead of making sharp right-angle turns | movement feels lifelike | F-7.1.5 | R-7.1.5 |
| US-7.1.5.4 | player (P-23) | I want pursuing enemies to run in natural arcs rather than jagged straight-line segments | chases feel cinematic | F-7.1.5 | R-7.1.5 |
| US-7.1.5.5 | player (P-23) | I want NPC movement to flow continuously between waypoints without stuttering or speed changes | path transitions are invisible | F-7.1.5 | R-7.1.5 |
| US-7.1.5.6 | engine developer (P-26) | I want to post-process waypoint paths with NavMesh raycasts to remove redundant turns | paths are simplified before smoothing | F-7.1.5 | R-7.1.5 |
| US-7.1.5.7 | engine developer (P-26) | I want to apply Catmull-Rom or cubic Bezier interpolation to waypoint paths | movement trajectories are smooth curves | F-7.1.5 | R-7.1.5 |
| US-7.1.5.8 | engine developer (P-26) | I want to fall back to linear waypoint paths on mobile platforms | per-path CPU cost stays within the tighter mobile budget | F-7.1.5 | R-7.1.5 |
| US-7.1.5.9 | engine tester (P-27) | I want to verify that smoothed curved paths do not leave the NavMesh boundary | agents never move through impassable geometry | F-7.1.5 | R-7.1.5 |
| US-7.1.5.10 | engine tester (P-27) | I want to compare linear, Catmull-Rom, and Bezier paths on a reference route and capture screenshots | visual quality differences are documented | F-7.1.5 | R-7.1.5 |
| US-7.1.5.11 | engine tester (P-27) | I want to measure the CPU overhead of path smoothing for 100+ agents simultaneously | I can confirm it fits within the navigation budget | F-7.1.5 | R-7.1.5 |
| US-7.1.5.12 | engine tester (P-27) | I want to confirm that mobile platform builds use linear-only paths and never invoke Bezier interpolation | the platform gate works. --- | F-7.1.5 | R-7.1.5 |

## F-7.1.6 — Dynamic Obstacle Carving

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.6.1 | designer (P-5) | I want to tag dynamic objects (barricades, vehicles) as NavMesh obstacles in the editor | agents automatically reroute around them | F-7.1.6 | R-7.1.6 |
| US-7.1.6.2 | designer (P-5) | I want to place a barricade in an agent's path at runtime and see it reroute within seconds | I can verify dynamic carving responsiveness | F-7.1.6 | R-7.1.6 |
| US-7.1.6.3 | designer (P-5) | I want to configure the carve shape and clearance margin per obstacle type | the blocked area matches the object's footprint | F-7.1.6 | R-7.1.6 |
| US-7.1.6.4 | player (P-23) | I want enemies to find an alternate route when a tree falls across their path | the world feels dynamically responsive | F-7.1.6 | R-7.1.6 |
| US-7.1.6.5 | player (P-23) | I want NPCs to walk around barricades I place instead of walking through them | my tactical placements affect AI movement | F-7.1.6 | R-7.1.6 |
| US-7.1.6.6 | player (P-23) | I want AI agents to adjust paths when a vehicle parks in their way | the world feels alive and reactive | F-7.1.6 | R-7.1.6 |
| US-7.1.6.7 | engine developer (P-26) | I want to implement tile-local NavMesh re-carving that invalidates only affected polygons | carving cost is proportional to obstacle size | F-7.1.6 | R-7.1.6 |
| US-7.1.6.8 | engine developer (P-26) | I want to trigger repath requests only for agents whose corridors intersect modified polygons | unaffected agents are not re-queried | F-7.1.6 | R-7.1.6 |
| US-7.1.6.9 | engine developer (P-26) | I want to limit simultaneous carve operations to 4 per tick on mobile | carving does not exceed the platform CPU budget | F-7.1.6 | R-7.1.6 |
| US-7.1.6.10 | engine tester (P-27) | I want to verify that agents cannot path through carved obstacle regions | dynamic obstacles are correctly blocking | F-7.1.6 | R-7.1.6 |
| US-7.1.6.11 | engine tester (P-27) | I want to confirm that rapid obstacle placement does not cause a repath storm exceeding the CPU budget | frame rate remains stable | F-7.1.6 | R-7.1.6 |
| US-7.1.6.12 | engine tester (P-27) | I want to verify that removing a dynamic obstacle restores the original NavMesh polygons | no ghost obstacles persist. --- | F-7.1.6 | R-7.1.6 |

## F-7.1.7 — NavMesh Links (Off-Mesh Connections)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.7.1 | designer (P-5) | I want to place off-mesh links between disjoint NavMesh regions in the editor (jump points, ladder connections) | agents can traverse gaps | F-7.1.7 | R-7.1.7 |
| US-7.1.7.2 | designer (P-5) | I want to assign animation tags (jump, climb, swim) to each off-mesh link | agents play the correct traversal animation | F-7.1.7 | R-7.1.7 |
| US-7.1.7.3 | designer (P-5) | I want to set preconditions on off-mesh links (key required, ability unlocked) | only eligible agents use restricted connections | F-7.1.7 | R-7.1.7 |
| US-7.1.7.4 | player (P-23) | I want enemies to climb ladders and jump across gaps during pursuit | AI movement uses the full level geometry | F-7.1.7 | R-7.1.7 |
| US-7.1.7.5 | player (P-23) | I want NPCs to open doors along their path instead of stopping at closed doorways | they navigate the world as I do | F-7.1.7 | R-7.1.7 |
| US-7.1.7.6 | player (P-23) | I want guards to be unable to follow me through locked doors they cannot open | precondition gating feels fair and logical | F-7.1.7 | R-7.1.7 |
| US-7.1.7.7 | engine developer (P-26) | I want to implement off-mesh connections carrying cost, animation tag, and precondition data | the planner can evaluate link feasibility | F-7.1.7 | R-7.1.7 |
| US-7.1.7.8 | engine developer (P-26) | I want to incorporate off-mesh links into the A* polygon search with their associated costs | pathfinding considers multi-modal traversal | F-7.1.7 | R-7.1.7 |
| US-7.1.7.9 | engine developer (P-26) | I want to support both bidirectional and one-way off-mesh links | jump-downs and one-way drops are modeled correctly | F-7.1.7 | R-7.1.7 |
| US-7.1.7.10 | engine tester (P-27) | I want to verify that agents without the required ability cannot use precondition-gated links | link restrictions are enforced | F-7.1.7 | R-7.1.7 |
| US-7.1.7.11 | engine tester (P-27) | I want to confirm that the correct traversal animation plays when an agent uses each link type | movement matches the animation tag | F-7.1.7 | R-7.1.7 |
| US-7.1.7.12 | engine tester (P-27) | I want to verify that A* path costs correctly include off-mesh link traversal costs | the planner accurately compares link vs. walkable paths. --- | F-7.1.7 | R-7.1.7 |

## F-7.1.8 — Incremental Tile Rebuild

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.8.1 | designer (P-5) | I want to deform terrain in the editor and see the NavMesh update for only the affected tiles | I can iterate on destructible environments quickly | F-7.1.8 | R-7.1.8 |
| US-7.1.8.2 | designer (P-5) | I want to manually mark spatial regions as dirty to force a NavMesh rebuild | I can test incremental rebuild behavior in the editor | F-7.1.8 | R-7.1.8 |
| US-7.1.8.3 | designer (P-5) | I want to set rebuild priority so tiles near active gameplay rebuild first | player-facing navigation updates quickly | F-7.1.8 | R-7.1.8 |
| US-7.1.8.4 | player (P-23) | I want AI to find new paths through areas opened by building destruction | the world responds dynamically to environmental changes | F-7.1.8 | R-7.1.8 |
| US-7.1.8.5 | player (P-23) | I want AI to reroute around rubble created by a collapsed structure | destruction meaningfully changes navigation | F-7.1.8 | R-7.1.8 |
| US-7.1.8.6 | player (P-23) | I want AI movement to remain smooth while NavMesh tiles rebuild in the background | I do not notice any navigation hitches | F-7.1.8 | R-7.1.8 |
| US-7.1.8.7 | engine developer (P-26) | I want to use a `NavMeshDirtyRegion` ECS component to mark spatial extents for rebuild | the system processes them in priority order | F-7.1.8 | R-7.1.8 |
| US-7.1.8.8 | engine developer (P-26) | I want to revoxelize only the affected tile and its immediate neighbors | rebuild cost scales with damage area, not world size | F-7.1.8 | R-7.1.8 |
| US-7.1.8.9 | engine developer (P-26) | I want to cap concurrent tile rebuilds (1 on mobile, 4+ on desktop) | rebuild work fits within the platform's CPU budget | F-7.1.8 | R-7.1.8 |
| US-7.1.8.10 | engine tester (P-27) | I want to verify that only the dirty tiles and their neighbors are rebuilt while all other tiles remain unchanged | rebuild scope is minimal | F-7.1.8 | R-7.1.8 |
| US-7.1.8.11 | engine tester (P-27) | I want to compare incrementally rebuilt tiles against a full rebake and verify identical results | incremental and full builds are equivalent | F-7.1.8 | R-7.1.8 |
| US-7.1.8.12 | engine tester (P-27) | I want to measure the time savings of incremental tile rebuild vs. full rebake on reference levels | the optimization is validated. --- | F-7.1.8 | R-7.1.8 |

## F-7.1.9 — Background NavMesh Generation

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.9.1 | designer (P-5) | I want the editor to remain fully responsive while NavMesh tiles generate in the background | I can continue working during long bakes | F-7.1.9 | R-7.1.9 |
| US-7.1.9.2 | designer (P-5) | I want to see a progress indicator showing how many tiles remain in the background generation queue | I know when the bake will finish | F-7.1.9 | R-7.1.9 |
| US-7.1.9.3 | designer (P-5) | I want to cancel an in-progress background NavMesh generation without corrupting the existing mesh | I can restart with new settings | F-7.1.9 | R-7.1.9 |
| US-7.1.9.4 | player (P-23) | I want gameplay to remain smooth while NavMesh tiles rebuild at runtime | world changes do not cause frame drops | F-7.1.9 | R-7.1.9 |
| US-7.1.9.5 | player (P-23) | I want AI agents to keep moving (using fallback navigation) while their tile is being rebuilt | NPCs never freeze in place | F-7.1.9 | R-7.1.9 |
| US-7.1.9.6 | player (P-23) | I want NPCs to transition seamlessly to newly built tiles without visible teleporting or path snapping | tile swaps are invisible | F-7.1.9 | R-7.1.9 |
| US-7.1.9.7 | engine developer (P-26) | I want to run NavMesh generation on background worker threads via the job system | the main simulation tick is never blocked | F-7.1.9 | R-7.1.9 |
| US-7.1.9.8 | engine developer (P-26) | I want to mark tiles under construction as `Pending` in the `NavMeshTileMap` ECS resource | agents receive fallback paths through pending areas | F-7.1.9 | R-7.1.9 |
| US-7.1.9.9 | engine developer (P-26) | I want to swap completed tiles into the live NavMesh atomically at the next sync point | agents never read a partially built tile | F-7.1.9 | R-7.1.9 |
| US-7.1.9.10 | engine tester (P-27) | I want to verify that NavMesh generation never blocks the main simulation thread | frame time is unaffected by background bakes | F-7.1.9 | R-7.1.9 |
| US-7.1.9.11 | engine tester (P-27) | I want to verify that agents navigating through pending tiles use valid fallback movement | no agent gets stuck or moves through walls | F-7.1.9 | R-7.1.9 |
| US-7.1.9.12 | engine tester (P-27) | I want to confirm that tile swaps are atomic and no agent ever reads partial tile data | data races are impossible. --- | F-7.1.9 | R-7.1.9 |

## F-7.1.10 — Destruction-Triggered NavMesh Updates

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.10.1 | designer (P-5) | I want to destroy a building in the editor and see the NavMesh update to open new pathways | I can verify destruction integration works | F-7.1.10 | R-7.1.10 |
| US-7.1.10.2 | designer (P-5) | I want rebuild priority to scale with the number of active agents whose paths cross the destroyed region | high-traffic areas update first | F-7.1.10 | R-7.1.10 |
| US-7.1.10.3 | designer (P-5) | I want to verify that destroying a wall creates a usable NavMesh corridor through the breach | destruction has tactical pathfinding consequences | F-7.1.10 | R-7.1.10 |
| US-7.1.10.4 | player (P-23) | I want enemies to rush through a wall I just destroyed, finding new paths through the breach | destruction has tactical consequences | F-7.1.10 | R-7.1.10 |
| US-7.1.10.5 | player (P-23) | I want AI to treat rubble from collapsed structures as obstacles, routing around debris | destruction creates new navigation challenges | F-7.1.10 | R-7.1.10 |
| US-7.1.10.6 | player (P-23) | I want AI near a destruction event to repath within 1-2 seconds | their response feels immediate and aware | F-7.1.10 | R-7.1.10 |
| US-7.1.10.7 | engine developer (P-26) | I want the destruction system to emit `NavMeshInvalidation` ECS events with affected bounding regions | the rebuild system picks them up | F-7.1.10 | R-7.1.10 |
| US-7.1.10.8 | engine developer (P-26) | I want the `NavMeshRebuildSystem` to enqueue tile rebuilds with priority proportional to affected agent count | critical areas rebuild first | F-7.1.10 | R-7.1.10 |
| US-7.1.10.9 | engine developer (P-26) | I want to handle destruction events that affect tiles already in the rebuild queue by merging dirty regions | redundant rebuilds are avoided | F-7.1.10 | R-7.1.10 |
| US-7.1.10.10 | engine tester (P-27) | I want to verify that destroying a wall creates a passable NavMesh corridor through the breach | agents can path through openings | F-7.1.10 | R-7.1.10 |
| US-7.1.10.11 | engine tester (P-27) | I want to verify that rubble from destruction is represented as NavMesh obstacles | agents cannot walk through debris piles | F-7.1.10 | R-7.1.10 |
| US-7.1.10.12 | engine tester (P-27) | I want to confirm that mobile defers or skips rebuilds for distant destruction events | mobile CPU budget is respected. --- | F-7.1.10 | R-7.1.10 |

## F-7.1.11 — Player-Built Structure Integration

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.11.1 | designer (P-5) | I want to place a player-buildable structure in the editor and see the NavMesh update to reflect the new obstacle | I can verify building integration | F-7.1.11 | R-7.1.11 |
| US-7.1.11.2 | designer (P-5) | I want to set area type overrides on player structures (wall, ramp, platform) | the NavMesh correctly classifies the structure | F-7.1.11 | R-7.1.11 |
| US-7.1.11.3 | designer (P-5) | I want stairs and ladders on player structures to auto-generate off-mesh links | agents can traverse multi-level player buildings | F-7.1.11 | R-7.1.11 |
| US-7.1.11.4 | player (P-23) | I want enemies to navigate around the walls of my fort instead of walking through them | my building efforts affect AI pathing | F-7.1.11 | R-7.1.11 |
| US-7.1.11.5 | player (P-23) | I want AI allies to walk up ramps I build to reach elevated platforms | my constructions are usable by NPCs | F-7.1.11 | R-7.1.11 |
| US-7.1.11.6 | player (P-23) | I want AI to find new routes when I remove a wall section from my base | the NavMesh updates dynamically with my edits | F-7.1.11 | R-7.1.11 |
| US-7.1.11.7 | engine developer (P-26) | I want to register player structures as NavMesh obstacles via a `NavMeshObstacle` ECS component with shape data | tiles rebuild on placement | F-7.1.11 | R-7.1.11 |
| US-7.1.11.8 | engine developer (P-26) | I want to auto-generate off-mesh links for stairs and ladder attachment points on player structures | vertical traversal works automatically | F-7.1.11 | R-7.1.11 |
| US-7.1.11.9 | engine developer (P-26) | I want to queue incremental tile rebuilds when structures are placed or removed | the NavMesh reflects the latest player construction state | F-7.1.11 | R-7.1.11 |
| US-7.1.11.10 | engine tester (P-27) | I want to verify that placed structures marked as impassable walls correctly block agent pathfinding | walls function as barriers | F-7.1.11 | R-7.1.11 |
| US-7.1.11.11 | engine tester (P-27) | I want to verify that agents can traverse auto-generated links on player-built ramps and ladders | vertical movement works end-to-end | F-7.1.11 | R-7.1.11 |
| US-7.1.11.12 | engine tester (P-27) | I want to verify that removing a player structure restores the NavMesh to its pre-placement state | no ghost obstacles remain. --- | F-7.1.11 | R-7.1.11 |

## F-7.1.12 — Multi-Size Agent NavMeshes

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.12.1 | designer (P-5) | I want to define multiple agent size classes (humanoid, mounted, siege golem) with radius, height, and slope parameters | each gets its own NavMesh | F-7.1.12 | R-7.1.12 |
| US-7.1.12.2 | designer (P-5) | I want to assign each agent entity to a NavMesh layer via a `NavMeshAgent` ECS component | it paths on the mesh matching its size | F-7.1.12 | R-7.1.12 |
| US-7.1.12.3 | designer (P-5) | I want to toggle visibility of individual NavMesh layers in the editor overlay | I can inspect each size class independently | F-7.1.12 | R-7.1.12 |
| US-7.1.12.4 | player (P-23) | I want large siege golems to take wide routes and avoid narrow alleys | their pathing matches their visible size | F-7.1.12 | R-7.1.12 |
| US-7.1.12.5 | player (P-23) | I want mounted riders to path along roads and open terrain rather than through tight doorways | mount navigation feels correct | F-7.1.12 | R-7.1.12 |
| US-7.1.12.6 | player (P-23) | I want small NPCs to take shortcuts through alleys that large NPCs cannot | size-based navigation differences are visible | F-7.1.12 | R-7.1.12 |
| US-7.1.12.7 | engine developer (P-26) | I want to maintain multiple NavMesh layers sharing the same spatial tile grid | streaming loads all layers for a region together | F-7.1.12 | R-7.1.12 |
| US-7.1.12.8 | engine developer (P-26) | I want to define `NavMeshAgentConfig` structs per size class with radius, height, step-climb, and max slope | each layer is generated correctly | F-7.1.12 | R-7.1.12 |
| US-7.1.12.9 | engine developer (P-26) | I want to limit NavMesh layers to 2 on mobile (humanoid + large) | memory and streaming bandwidth stay within mobile constraints | F-7.1.12 | R-7.1.12 |
| US-7.1.12.10 | engine tester (P-27) | I want to verify that each NavMesh layer produces correct walkable surfaces for its configured agent size | no layer has missing or extra coverage | F-7.1.12 | R-7.1.12 |
| US-7.1.12.11 | engine tester (P-27) | I want to verify that each agent queries only its assigned NavMesh layer and never paths on a wrong-sized mesh | layer assignment is enforced | F-7.1.12 | R-7.1.12 |
| US-7.1.12.12 | engine tester (P-27) | I want to measure the total memory consumed by multiple NavMesh layers | multi-layer overhead is within the allocated budget. --- | F-7.1.12 | R-7.1.12 |

## F-7.1.13 — Dynamic Area Cost Modification

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.13.1 | designer (P-5) | I want to modify NavMesh polygon area costs at runtime in the editor (e.g., flood a zone to raise water cost) | I can test dynamic routing | F-7.1.13 | R-7.1.13 |
| US-7.1.13.2 | designer (P-5) | I want to set per-faction area cost overrides so friendly territory is cheaper for allies | faction-aware routing is configurable | F-7.1.13 | R-7.1.13 |
| US-7.1.13.3 | designer (P-5) | I want to visualize a cost heatmap overlay on the NavMesh in the editor | I can see how area costs influence route selection | F-7.1.13 | R-7.1.13 |
| US-7.1.13.4 | player (P-23) | I want NPCs to prefer dry paths when a zone floods, avoiding the water unless no alternative exists | AI reacts to environmental changes | F-7.1.13 | R-7.1.13 |
| US-7.1.13.5 | player (P-23) | I want civilian NPCs to detour around active battlefields with danger area weighting | non-combatants flee to safety | F-7.1.13 | R-7.1.13 |
| US-7.1.13.6 | player (P-23) | I want allied troops to prefer roads in friendly territory while enemies take longer concealed routes | faction strategy is visible | F-7.1.13 | R-7.1.13 |
| US-7.1.13.7 | engine developer (P-26) | I want to store runtime area cost modifications in a `NavMeshAreaCosts` ECS resource applied during A* expansion | costs change without rebuilding geometry | F-7.1.13 | R-7.1.13 |
| US-7.1.13.8 | engine developer (P-26) | I want to support per-agent cost overrides for faction-specific routing | different factions evaluate the same area differently | F-7.1.13 | R-7.1.13 |
| US-7.1.13.9 | engine developer (P-26) | I want to ensure area cost modifications are pure data updates with no geometry rebuild | cost changes are instantaneous and lightweight | F-7.1.13 | R-7.1.13 |
| US-7.1.13.10 | engine tester (P-27) | I want to verify that increasing an area's cost causes agents to choose alternate routes when available | cost modifications affect pathfinding | F-7.1.13 | R-7.1.13 |
| US-7.1.13.11 | engine tester (P-27) | I want to place two agents from different factions and verify they take different routes based on faction cost overrides | faction-aware routing works | F-7.1.13 | R-7.1.13 |
| US-7.1.13.12 | engine tester (P-27) | I want to confirm that modifying area costs does not trigger any NavMesh tile rebuild | cost changes remain a zero-rebuild operation. --- | F-7.1.13 | R-7.1.13 |

## F-7.1.14 — Hierarchical Pathfinding (HPA*)

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.14.1 | designer (P-5) | I want to enable hierarchical pathfinding for paths that cross multiple NavMesh tiles | long-distance NPC travel is efficient | F-7.1.14 | R-7.1.14 |
| US-7.1.14.2 | designer (P-5) | I want to visualize the coarse hierarchical navigation graph in the editor | I can inspect cross-tile connectivity | F-7.1.14 | R-7.1.14 |
| US-7.1.14.3 | designer (P-5) | I want to command an NPC to travel across the entire world map and verify it arrives | long-distance pathfinding is reliable | F-7.1.14 | R-7.1.14 |
| US-7.1.14.4 | player (P-23) | I want NPCs to begin long-distance journeys immediately without visible planning delay | travel commands feel responsive | F-7.1.14 | R-7.1.14 |
| US-7.1.14.5 | player (P-23) | I want trade caravans to navigate across multiple regions on efficient routes | cross-world travel looks intentional and planned | F-7.1.14 | R-7.1.14 |
| US-7.1.14.6 | player (P-23) | I want my AI companion to follow me across the entire map without losing its way | long adventures feel seamless | F-7.1.14 | R-7.1.14 |
| US-7.1.14.7 | engine developer (P-26) | I want to build a coarse navigation graph from NavMesh tile connectivity | long-distance queries are O(1) per tile | F-7.1.14 | R-7.1.14 |
| US-7.1.14.8 | engine developer (P-26) | I want to refine hierarchical paths to full NavMesh resolution only for the tiles the agent is currently traversing | detail is computed just-in-time | F-7.1.14 | R-7.1.14 |
| US-7.1.14.9 | engine developer (P-26) | I want pathfinding cost to remain bounded regardless of world size | thousands of server-side NPCs can plan cross-continent paths | F-7.1.14 | R-7.1.14 |
| US-7.1.14.10 | engine tester (P-27) | I want to verify that HPA* produces valid navigable paths between any two reachable points on the world map | no routes are broken | F-7.1.14 | R-7.1.14 |
| US-7.1.14.11 | engine tester (P-27) | I want to compare HPA* paths against full A* paths on reference scenarios and measure cost deviation | hierarchical approximation is acceptable | F-7.1.14 | R-7.1.14 |
| US-7.1.14.12 | engine tester (P-27) | I want to benchmark HPA* with 1000+ concurrent long-distance queries | server-side NPC travel scales to MMO population levels. --- | F-7.1.14 | R-7.1.14 |

## F-7.1.15 — NavMesh Runtime Visualization

| ID | Persona | Story | Acceptance Criteria | Features | Requirements |
|----|---------|-------|---------------------|----------|--------------|
| US-7.1.15.1 | designer (P-5) | I want to toggle a debug overlay showing NavMesh polygons, tile boundaries, and area types in the editor | I can inspect navigation data visually | F-7.1.15 | R-7.1.15 |
| US-7.1.15.2 | designer (P-5) | I want different area types to be color-coded in the debug overlay (roads green, swamps brown, lava red) | I can identify zones at a glance | F-7.1.15 | R-7.1.15 |
| US-7.1.15.3 | designer (P-5) | I want to see agent path trails drawn as colored lines on the NavMesh overlay | I can diagnose why agents choose specific routes | F-7.1.15 | R-7.1.15 |
| US-7.1.15.4 | designer (P-5) | I want off-mesh links to appear as labeled arcs in the debug overlay | I can verify link placement and connectivity | F-7.1.15 | R-7.1.15 |
| US-7.1.15.5 | player (P-23) | I want no NavMesh debug overlays to appear in the shipping game | the visual experience is clean and immersive | F-7.1.15 | R-7.1.15 |
| US-7.1.15.6 | engine developer (P-26) | I want to control visualization through `NavMeshDebug` ECS components | debug rendering is opt-in and configurable per entity or region | F-7.1.15 | R-7.1.15 |
| US-7.1.15.7 | engine developer (P-26) | I want to integrate NavMesh visualization with the editor gizmo system (F-15.1.4) | overlays render within the existing debug infrastructure | F-7.1.15 | R-7.1.15 |
| US-7.1.15.8 | engine developer (P-26) | I want to strip all NavMesh debug visualization code from shipping builds | release binaries have no debug overhead | F-7.1.15 | R-7.1.15 |
| US-7.1.15.9 | designer (P-5) | I want tiles queued for rebuild to be highlighted in the overlay | I can see which regions are temporarily stale | F-7.1.15 | R-7.1.15 |
| US-7.1.15.10 | engine tester (P-27) | I want to verify that the debug overlay accurately represents the underlying NavMesh data | visualization is trustworthy for debugging | F-7.1.15 | R-7.1.15 |
| US-7.1.15.11 | engine tester (P-27) | I want to confirm that shipping builds contain no NavMesh debug rendering code or data | release builds are clean | F-7.1.15 | R-7.1.15 |
| US-7.1.15.12 | engine tester (P-27) | I want to measure overlay rendering performance with the entire NavMesh visible | debug visualization does not tank editor frame rate | F-7.1.15 | R-7.1.15 |
